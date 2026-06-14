// Manually written file (the `CodegenWasmJit` MetaModelica package is a
// placeholder; see HANDWRITTEN_TOP_PACKAGES in mmtorust/src/codegen.rs).
//
// Simulation half of the `wasm-jit` target — the counterpart of `CodegenC` for
// the C target. Instead of generating ~25 C files + `_init.xml` + a makefile,
// building an executable and running it to write a `.mat`, this lowers the
// SimCode equation systems to a single WebAssembly *model module* (the
// numerical right-hand sides) and runs the simulation in-process with wasmtime.
//
// Two design departures from the C runtime, per the project steer:
//   * No XML/JSON serialization of model metadata. The host (this Rust code)
//     holds the SimCode-derived data (variable names, start values, parameter
//     values, simulation settings) in memory and feeds it to the run / to the
//     `.mat` writer directly — the "expose SimCode data through host functions"
//     approach.
//   * The forward-Euler integrator loop runs *in wasm* (the precompiled runtime
//     primitives `rt_euler_step` / `rt_sim_store_row` plus an emitted `simulate`
//     loop), so a whole run is a single host->wasm call with no per-step
//     boundary crossing. A second, host-driven driver (the Euler loop in native
//     Rust, one wasm call per step) is provided for benchmarking — selected with
//     `OMC_WASM_SIM_DRIVER=host`.
//
// ## SimData memory layout
//
// All model state lives in one `SimData` block (allocated with the runtime's
// `rt_alloc`) of contiguous little-endian slots:
//
//   [ time:f64 | realVars:f64[2*nStates + nAlgs] | realParams:f64[nRP]
//     | intVars:i32[nIA] | intParams:i32[nIP] | boolVars:i32[nBA] | boolParams:i32[nBP] ]
//
// `realVars` is ordered `[states | derivatives | algebraics]`, matching the C
// runtime's `realVars` ordering. Every model variable therefore has a
// compile-time-constant byte offset; the generated equation functions take the
// `SimData` pointer as their single parameter and access a variable with one
// `f64.load`/`f64.store` (or `i32.*`) at that offset. A result-buffer row is the
// time-variant prefix `[time | realVars]` (`n_reals = 1 + 2*nStates + nAlgs`
// f64), so emitting a row is a copy of the first `n_reals` slots of `SimData`.

#![allow(non_snake_case)]

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::sync::Arc;

use anyhow::{Result, anyhow, bail};
use arcstr::ArcStr;
use metamodelica::List;
use wasm_encoder as we;

use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;

use crate::CodegenWasmJitFunctions::{
    BUILTINS, ENV_EXTRA, FnCtx, FnInfo, RT_BUILTINS, SimCtx, SimSlot, WTy, compile_function,
    external_known, function_signature, rt_index, sim_cref_key,
};

mod sim_runtime;

/// Iterate a MetaModelica `List` (which is `IntoIterator` by reference, not via
/// an `.iter()` method).
fn lst<T: Clone>(l: &Arc<List<T>>) -> impl Iterator<Item = &T> {
    (&**l).into_iter()
}

// ===========================================================================
// SimData layout
// ===========================================================================

/// Byte offset of `time` within `SimData`.
const TIME_OFF: u32 = 0;
/// Byte offset of the first real variable (`realVars[0]`, a state).
const REAL_OFF: u32 = 8;

/// Fully-resolved layout of one model's `SimData` block. All offsets are byte
/// offsets within the block; all are compile-time constants baked into the
/// generated module.
#[derive(Clone)]
struct SimLayout {
    n_states: u32,
    /// `algVars ++ discreteAlgVars` (the real algebraic variables emitted as
    /// time-variant result signals after the states and derivatives).
    n_real_alg: u32,
    rparam_off: u32,
    int_off: u32,
    iparam_off: u32,
    bool_off: u32,
    bparam_off: u32,
    total: u32,
}

impl SimLayout {
    fn new(
        n_states: u32,
        n_real_alg: u32,
        n_real_param: u32,
        n_int_alg: u32,
        n_int_param: u32,
        n_bool_alg: u32,
        n_bool_param: u32,
    ) -> Self {
        let n_real = 2 * n_states + n_real_alg; // states | ders | algs
        let rparam_off = REAL_OFF + n_real * 8;
        let int_off = rparam_off + n_real_param * 8;
        let iparam_off = int_off + n_int_alg * 4;
        let bool_off = iparam_off + n_int_param * 4;
        let bparam_off = bool_off + n_bool_alg * 4;
        let total = bparam_off + n_bool_param * 4;
        SimLayout { n_states, n_real_alg, rparam_off, int_off, iparam_off, bool_off, bparam_off, total }
    }

    /// Number of f64 in a result row: `time` + all real variables.
    fn n_reals_row(&self) -> u32 {
        1 + 2 * self.n_states + self.n_real_alg
    }
}

// ===========================================================================
// Result-variable metadata (held by the host, written into the `.mat`)
// ===========================================================================

/// How a result signal is stored in the `.mat` (which matrix + value source).
#[derive(Clone)]
enum ResultKind {
    /// The independent variable (`time`): data_2 row 1, channel 0.
    Time,
    /// A time-variant signal: data_2 at the given 1-based row.
    TimeVariant { row: u32 },
    /// A time-invariant parameter: data_1 at the given 1-based row; `value` is
    /// read from `SimData` after initialization.
    Param { off: u32, wty: WTy },
}

/// One signal in the result file (in C-compatible order: time, states,
/// derivatives, algebraics, then parameters).
#[derive(Clone)]
struct ResultVar {
    name: String,
    comment: String,
    kind: ResultKind,
}

/// The prepared, ready-to-run artifact for one model, stashed in-process by
/// [`translateModel`] and consumed by [`runSimulation`] (keyed by file-name
/// prefix). This is the in-memory replacement for the C target's `_init.xml`
/// + `_info.json` + the built executable.
struct SimModel {
    wasm: Vec<u8>,
    layout: SimLayout,
    result_vars: Vec<ResultVar>,
    model_name: String,
    start_time: f64,
    stop_time: f64,
    n_intervals: u32,
    output_format: String,
}

/// Process-wide table of prepared models, keyed by file-name prefix. Populated
/// by `translateModel` (during `callTargetTemplates`) and read by
/// `runSimulation` (during `simulate`) in the same process.
fn sim_models() -> &'static Mutex<HashMap<String, Arc<SimModel>>> {
    static MODELS: OnceLock<Mutex<HashMap<String, Arc<SimModel>>>> = OnceLock::new();
    MODELS.get_or_init(|| Mutex::new(HashMap::new()))
}

// ===========================================================================
// Public entry points (called from the MetaModelica sources after regen)
// ===========================================================================

/// `CodegenWasmJit.translateModel`: lower `simCode` to a model wasm module,
/// write `<prefix>.wasm`, and stash the prepared [`SimModel`] for the later
/// `runSimulation`. Counterpart of `CodegenC.translateModel` + the makefile/XML
/// machinery for the C target. Errors are fatal (a panic naming the reason),
/// matching `CodegenWasmJitFunctions.translateFunctions`.
pub fn translateModel(simCode: SimCode::SimCode) {
    let prefix = simCode.fileNamePrefix.to_string();
    let _ = std::fs::remove_file(format!("{prefix}.wasm"));
    match build_sim_model(&simCode) {
        Ok(model) => {
            // Write the module for inspection/debugging (mirrors the function
            // half writing `<name>.wasm`); the run itself uses the stashed bytes.
            if let Err(e) = std::fs::write(format!("{prefix}.wasm"), &model.wasm) {
                panic!("CodegenWasmJit: cannot write {prefix}.wasm: {e:#}");
            }
            sim_models().lock().unwrap().insert(prefix, Arc::new(model));
        }
        Err(e) => panic!("CodegenWasmJit: cannot build simulation module for `{prefix}`: {e:#}"),
    }
}

/// `CodegenWasmJit.runSimulation`: run the prepared model in-process and write
/// the result file. Returns 0 on success, 1 on failure (matching the exit code
/// the C target's executable would return, which `simulate` checks).
pub fn runSimulation(fileNamePrefix: ArcStr, resultFile: ArcStr, simflags: ArcStr) -> i32 {
    let res = run_simulation_inner(&fileNamePrefix, &resultFile, &simflags);
    // The simulate scripting flow reads `<prefix>.log` after a run (the C target's
    // executable writes one); write it here so the success path is taken. On
    // failure the log carries the error so it surfaces in the result `messages`.
    let log = match &res {
        Ok(()) => "LOG_SUCCESS       | info    | The initialization finished successfully without homotopy method.\n\
                    LOG_SUCCESS       | info    | The simulation finished successfully.\n"
            .to_string(),
        Err(e) => format!("LOG_ERROR         | error   | wasm-jit simulation failed: {e:#}\n"),
    };
    let _ = std::fs::write(format!("{fileNamePrefix}.log"), log);
    match res {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("CodegenWasmJit: simulation of `{fileNamePrefix}` failed: {e:#}");
            1
        }
    }
}

fn run_simulation_inner(prefix: &str, result_file: &str, _simflags: &str) -> Result<()> {
    let model = sim_models()
        .lock()
        .unwrap()
        .get(prefix)
        .cloned()
        .ok_or_else(|| anyhow!("no prepared wasm-jit model for `{prefix}` (translateModel not run?)"))?;
    if model.output_format != "mat" {
        bail!("CodegenWasmJit: only the `mat` output format is supported (got `{}`)", model.output_format);
    }
    let driver = std::env::var("OMC_WASM_SIM_DRIVER").unwrap_or_default();
    let run = sim_runtime::run(&model, driver == "host")?;
    write_mat4(&model, result_file, &run.rows, run.n_reals, &run.params)
}

// ===========================================================================
// Building the variable->slot map and the result-variable list
// ===========================================================================

/// The data the equation-function lowering needs to resolve component
/// references: the cref->slot map and the per-variable start expressions.
struct SimVarMap {
    vars: HashMap<String, SimSlot>,
    starts: HashMap<String, Option<Arc<DAE::Exp>>>,
}

/// Display name of a model variable's component reference (OMC `.`-separated
/// form, e.g. `body.r[1]`).
fn cref_display(cr: &Arc<DAE::ComponentRef>) -> Result<String> {
    Ok(ComponentReferenceBasics::printComponentRefStr(cr.clone())?.to_string())
}

/// Build the cref->slot map and the result-variable list from the model's
/// `SimVars`. The slot offsets follow [`SimLayout`]; the result order matches
/// the C runtime (time, states, state derivatives, real algebraics, then
/// parameters) so the `.mat` reads back identically.
fn build_var_map(
    vars: &SimCodeVar::SimVars,
    layout: &SimLayout,
) -> Result<(SimVarMap, Vec<ResultVar>)> {
    let mut map = SimVarMap { vars: HashMap::new(), starts: HashMap::new() };
    let mut result_vars: Vec<ResultVar> = Vec::new();

    // time — result signal 0.
    result_vars.push(ResultVar {
        name: "time".to_string(),
        comment: "Simulation time [s]".to_string(),
        kind: ResultKind::Time,
    });

    let states: Vec<&SimCodeVar::SimVar> = lst(&vars.stateVars).collect();
    let ders: Vec<&SimCodeVar::SimVar> = lst(&vars.derivativeVars).collect();

    // States: realVars[0..nStates].
    let mut next_tv_row: u32 = 2; // data_2 row 1 is time; signals start at row 2
    for (i, sv) in states.iter().enumerate() {
        let off = REAL_OFF + (i as u32) * 8;
        insert_var(&mut map, sv, off, WTy::F64)?;
        result_vars.push(ResultVar {
            name: cref_display(&sv.name)?,
            comment: sv.comment.to_string(),
            kind: ResultKind::TimeVariant { row: next_tv_row },
        });
        next_tv_row += 1;
    }
    // Derivatives: realVars[nStates..2*nStates], paired with states by index.
    for (i, sv) in ders.iter().enumerate() {
        let off = REAL_OFF + (layout.n_states + i as u32) * 8;
        insert_var(&mut map, sv, off, WTy::F64)?;
        // der(x) is displayed as `der(<state name>)`.
        let dname = match states.get(i) {
            Some(s) => format!("der({})", cref_display(&s.name)?),
            None => cref_display(&sv.name)?,
        };
        result_vars.push(ResultVar {
            name: dname,
            comment: sv.comment.to_string(),
            kind: ResultKind::TimeVariant { row: next_tv_row },
        });
        next_tv_row += 1;
    }
    // Real algebraics: algVars ++ discreteAlgVars.
    let real_algs: Vec<&SimCodeVar::SimVar> =
        lst(&vars.algVars).chain(lst(&vars.discreteAlgVars)).collect();
    for (j, sv) in real_algs.iter().enumerate() {
        let off = REAL_OFF + (2 * layout.n_states + j as u32) * 8;
        insert_var(&mut map, sv, off, WTy::F64)?;
        result_vars.push(ResultVar {
            name: cref_display(&sv.name)?,
            comment: sv.comment.to_string(),
            kind: ResultKind::TimeVariant { row: next_tv_row },
        });
        next_tv_row += 1;
    }

    // Real parameters (time-invariant; data_1).
    for (k, sv) in lst(&vars.paramVars).enumerate() {
        let off = layout.rparam_off + (k as u32) * 8;
        insert_var(&mut map, sv, off, WTy::F64)?;
        result_vars.push(ResultVar {
            name: cref_display(&sv.name)?,
            comment: sv.comment.to_string(),
            kind: ResultKind::Param { off, wty: WTy::F64 },
        });
    }
    // Integer / Boolean algebraics and parameters: slots only (not yet emitted
    // as result signals — mechanical models are Real-dominated). They resolve in
    // equations; surfacing them as result columns is future work.
    for (i, sv) in lst(&vars.intAlgVars).enumerate() {
        insert_var(&mut map, sv, layout.int_off + (i as u32) * 4, WTy::I32)?;
    }
    for (k, sv) in lst(&vars.intParamVars).enumerate() {
        insert_var(&mut map, sv, layout.iparam_off + (k as u32) * 4, WTy::I32)?;
    }
    for (i, sv) in lst(&vars.boolAlgVars).enumerate() {
        insert_var(&mut map, sv, layout.bool_off + (i as u32) * 4, WTy::I32)?;
    }
    for (k, sv) in lst(&vars.boolParamVars).enumerate() {
        insert_var(&mut map, sv, layout.bparam_off + (k as u32) * 4, WTy::I32)?;
    }

    // Aliases: resolve to the slot of the target variable (with negation). This
    // lets equations and `$START` of an alias read the aliased value. Alias
    // result columns are future work; the simulation itself is unaffected.
    for av in lst(&vars.aliasVars).chain(lst(&vars.intAliasVars)).chain(lst(&vars.boolAliasVars)) {
        let (target, negate) = match &av.aliasvar {
            SimCodeVar::AliasVariable::ALIAS { varName } => (varName.clone(), false),
            SimCodeVar::AliasVariable::NEGATEDALIAS { varName } => (varName.clone(), true),
            SimCodeVar::AliasVariable::NOALIAS => continue,
        };
        let tkey = sim_cref_key(&target)?;
        if let Some(tslot) = map.vars.get(&tkey).copied() {
            let key = sim_cref_key(&av.name)?;
            map.vars.insert(
                key,
                SimSlot { off: tslot.off, wty: tslot.wty, negate: tslot.negate ^ negate },
            );
        }
        // An alias whose target is not (yet) mapped is silently skipped; a
        // reference to it then fails loudly at lowering with a clear message.
    }

    Ok((map, result_vars))
}

/// Register one variable's slot (by canonical cref key) and its start value.
fn insert_var(map: &mut SimVarMap, sv: &SimCodeVar::SimVar, off: u32, wty: WTy) -> Result<()> {
    let key = sim_cref_key(&sv.name)?;
    map.vars.insert(key.clone(), SimSlot { off, wty, negate: false });
    map.starts.insert(key, sv.initialValue.clone());
    Ok(())
}

// ===========================================================================
// Module assembly
// ===========================================================================

/// Wasm function indices of the generated equation functions (after the
/// imports and the model's Modelica functions).
struct EqFnIdx {
    parameters: u32,
    initial: u32,
    ode: u32,
    algebraics: u32,
}

fn build_sim_model(sim_code: &SimCode::SimCode) -> Result<SimModel> {
    let mi = &sim_code.modelInfo;
    let vi = &mi.varInfo;
    let vars = &mi.vars;

    let n_states = vi.numStateVars.max(0) as u32;
    let n_real_alg = (count(&vars.algVars) + count(&vars.discreteAlgVars)) as u32;
    let n_real_param = count(&vars.paramVars) as u32;
    let layout = SimLayout::new(
        n_states,
        n_real_alg,
        n_real_param,
        count(&vars.intAlgVars) as u32,
        count(&vars.intParamVars) as u32,
        count(&vars.boolAlgVars) as u32,
        count(&vars.boolParamVars) as u32,
    );

    let (var_map, result_vars) = build_var_map(vars, &layout)?;

    // Index -> equation map (for SES_ALIAS, which re-runs another equation by
    // index). `allEquations` holds every scalar equation.
    let mut eq_index: HashMap<i32, Arc<SimCode::SimEqSystem>> = HashMap::new();
    for e in lst(&sim_code.allEquations) {
        eq_index.insert(eq_index_of(e), e.clone());
    }

    // --- Collect the model's Modelica functions (callable from equations). ---
    let model_fns: Vec<&SimCodeFunction::Function::Function> = lst(&mi.functions)
        .map(|f| &**f)
        .filter(|f| {
            matches!(f, SimCodeFunction::Function::Function::FUNCTION { .. }) || external_known(f)
        })
        .collect();

    // Function index space: imports (env builtins, rt runtime, env-extra), then
    // the model's Modelica functions, then the generated equation functions.
    let import_base = (BUILTINS.len() + RT_BUILTINS.len() + ENV_EXTRA.len()) as u32;
    let mut by_name: HashMap<String, FnInfo> = HashMap::new();
    for (id, f) in model_fns.iter().enumerate() {
        let (name, sig) = function_signature(f)?;
        by_name.insert(name, FnInfo { index: import_base + id as u32, sig });
    }
    let eq_base = import_base + model_fns.len() as u32;
    let eqfn = EqFnIdx {
        parameters: eq_base,
        initial: eq_base + 1,
        ode: eq_base + 2,
        algebraics: eq_base + 3,
    };
    let simulate_idx = eq_base + 4;

    // --- Type section: one type per import, per model function, per equation
    // function (all take one i32 `SimData` ptr, no result), then `simulate`
    // (f64,f64,f64,i32 -> i32). ---
    let mut types = we::TypeSection::new();
    for (_, params, result) in BUILTINS {
        types.ty().function(params.iter().map(|w| w.val()), [result.val()]);
    }
    for (_, params, results) in RT_BUILTINS {
        types.ty().function(params.iter().map(|w| w.val()), results.iter().map(|w| w.val()));
    }
    for (_, params, results) in ENV_EXTRA {
        types.ty().function(params.iter().map(|w| w.val()), results.iter().map(|w| w.val()));
    }
    let mut model_fn_type: Vec<u32> = Vec::with_capacity(model_fns.len());
    for f in &model_fns {
        let (_, sig) = function_signature(f)?;
        let ti = types.len();
        types.ty().function(
            sig.params.iter().map(|s| s.wty().val()),
            sig.results.iter().map(|s| s.wty().val()),
        );
        model_fn_type.push(ti);
    }
    // Equation function type: (i32) -> ().
    let eqfn_type = types.len();
    types.ty().function([we::ValType::I32], []);
    // simulate type: (i32 simdata, f64 start, f64 stop, i32 nsteps) -> i32 buf.
    let simulate_type = types.len();
    types.ty().function(
        [we::ValType::I32, we::ValType::F64, we::ValType::F64, we::ValType::I32],
        [we::ValType::I32],
    );

    // --- Import section. ---
    let mut imports = we::ImportSection::new();
    imports.import(
        "rt",
        "memory",
        we::MemoryType { minimum: 0, maximum: None, memory64: false, shared: false, page_size_log2: None },
    );
    for (i, (name, _, _)) in BUILTINS.iter().enumerate() {
        imports.import("env", *name, we::EntityType::Function(i as u32));
    }
    for (j, (name, _, _)) in RT_BUILTINS.iter().enumerate() {
        imports.import("rt", *name, we::EntityType::Function((BUILTINS.len() + j) as u32));
    }
    for (k, (name, _, _)) in ENV_EXTRA.iter().enumerate() {
        imports.import("env", *name, we::EntityType::Function((BUILTINS.len() + RT_BUILTINS.len() + k) as u32));
    }

    // --- Compile bodies (collecting String literals into the module pool). ---
    let mut literals: Vec<Vec<u8>> = Vec::new();
    let mut bodies: Vec<we::Function> = Vec::new();
    // Model functions first, in index order.
    for f in &model_fns {
        bodies.push(compile_function(f, &by_name, &mut literals)?);
    }
    // Parameter bindings (`parameter Real c = 0.5`) are not in
    // `parameterEquations` for constant bindings — the C target reads them from
    // `_init.xml`. Initialize every parameter from its binding expression
    // (`SimVar.initialValue`) in declaration order (the backend sorts dependent
    // parameters so a binding only references earlier ones), then run
    // `parameterEquations` for any computed parameters.
    let param_bindings = collect_param_bindings(vars);

    // Equation functions.
    bodies.push(build_eq_fn_with_prelude("parameterEquations", &param_bindings, flatten_eqs(&sim_code.parameterEquations), &var_map, &eq_index, &by_name, &mut literals)?);
    bodies.push(build_eq_fn("initialEquations", flatten_eqs(&sim_code.initialEquations), &var_map, &eq_index, &by_name, &mut literals)?);
    bodies.push(build_eq_fn("odeEquations", flatten_eqs_ll(&sim_code.odeEquations), &var_map, &eq_index, &by_name, &mut literals)?);
    bodies.push(build_eq_fn("algebraicEquations", flatten_eqs_ll(&sim_code.algebraicEquations), &var_map, &eq_index, &by_name, &mut literals)?);
    // The integrator loop.
    bodies.push(build_simulate(&layout, &eqfn));

    // --- Function section (type index per body, in body order). ---
    let mut functions = we::FunctionSection::new();
    for ti in &model_fn_type {
        functions.function(*ti);
    }
    for _ in 0..4 {
        functions.function(eqfn_type);
    }
    functions.function(simulate_type);

    // --- Code section. ---
    let mut code = we::CodeSection::new();
    for body in &bodies {
        code.function(body);
    }

    // --- Exports: the equation functions (for the host-driven driver) and
    // `simulate` (for the in-wasm driver). ---
    let mut exports = we::ExportSection::new();
    exports.export("functionParameters", we::ExportKind::Func, eqfn.parameters);
    exports.export("functionInitialEquations", we::ExportKind::Func, eqfn.initial);
    exports.export("functionODE", we::ExportKind::Func, eqfn.ode);
    exports.export("functionAlgebraics", we::ExportKind::Func, eqfn.algebraics);
    exports.export("simulate", we::ExportKind::Func, simulate_idx);

    let mut module = we::Module::new();
    module.section(&types);
    module.section(&imports);
    module.section(&functions);
    module.section(&exports);
    if !literals.is_empty() {
        module.section(&we::DataCountSection { count: literals.len() as u32 });
    }
    module.section(&code);
    if !literals.is_empty() {
        let mut data = we::DataSection::new();
        for lit in &literals {
            data.passive(lit.iter().copied());
        }
        module.section(&data);
    }
    let wasm = module.finish();

    let settings = sim_code
        .simulationSettingsOpt
        .as_ref()
        .ok_or_else(|| anyhow!("CodegenWasmJit: model has no simulation settings"))?;

    Ok(SimModel {
        wasm,
        layout,
        result_vars,
        model_name: openmodelica_frontend_dump::AbsynUtil::pathString(
            mi.name.clone(),
            arcstr::literal!("."),
            true,
            false,
        )?
        .to_string(),
        start_time: settings.startTime.into_inner(),
        stop_time: settings.stopTime.into_inner(),
        n_intervals: settings.numberOfIntervals.max(0) as u32,
        output_format: settings.outputFormat.to_string(),
    })
}

/// A fresh `T_REAL` type for synthesizing the lhs `CREF` expression of a simple
/// assignment (the type is not consulted on the simulation cref path).
fn t_real() -> Arc<DAE::Type> {
    Arc::new(DAE::Type::T_REAL { varLst: metamodelica::nil() })
}

fn count<T: Clone>(list: &Arc<List<T>>) -> usize {
    lst(list).count()
}

/// Flatten a `list<SimEqSystem>` to a Vec of references.
fn flatten_eqs(eqs: &Arc<List<Arc<SimCode::SimEqSystem>>>) -> Vec<Arc<SimCode::SimEqSystem>> {
    lst(eqs).cloned().collect()
}

/// Flatten a `list<list<SimEqSystem>>` (partitioned equations) to a flat Vec.
fn flatten_eqs_ll(
    eqs: &Arc<List<Arc<List<Arc<SimCode::SimEqSystem>>>>>,
) -> Vec<Arc<SimCode::SimEqSystem>> {
    let mut out = Vec::new();
    for part in lst(eqs) {
        for e in lst(part) {
            out.push(e.clone());
        }
    }
    out
}

/// Build one equation function (`SimData* -> ()`), lowering each equation in
/// order. Unsupported equation kinds (systems, array assigns) fail loudly so a
/// model that needs them is rejected rather than silently mis-simulated.
/// Collect parameter binding assignments (`cref := initialValue`) from all
/// parameter `SimVar`s that have a binding, in declaration order.
fn collect_param_bindings(vars: &SimCodeVar::SimVars) -> Vec<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)> {
    let mut out = Vec::new();
    for p in lst(&vars.paramVars)
        .chain(lst(&vars.intParamVars))
        .chain(lst(&vars.boolParamVars))
    {
        if let Some(v) = &p.initialValue {
            out.push((p.name.clone(), v.clone()));
        }
    }
    out
}

fn build_eq_fn(
    which: &str,
    eqs: Vec<Arc<SimCode::SimEqSystem>>,
    var_map: &SimVarMap,
    eq_index: &HashMap<i32, Arc<SimCode::SimEqSystem>>,
    by_name: &HashMap<String, FnInfo>,
    literals: &mut Vec<Vec<u8>>,
) -> Result<we::Function> {
    build_eq_fn_with_prelude(which, &[], eqs, var_map, eq_index, by_name, literals)
}

fn build_eq_fn_with_prelude(
    which: &str,
    prelude: &[(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)],
    eqs: Vec<Arc<SimCode::SimEqSystem>>,
    var_map: &SimVarMap,
    eq_index: &HashMap<i32, Arc<SimCode::SimEqSystem>>,
    by_name: &HashMap<String, FnInfo>,
    literals: &mut Vec<Vec<u8>>,
) -> Result<we::Function> {
    let sim = SimCtx {
        data_local: 0,
        vars: var_map.vars.clone(),
        starts: var_map.starts.clone(),
    };
    let mut ctx = FnCtx::new_sim(sim, by_name, literals);
    for (cref, exp) in prelude {
        let lhs = DAE::Exp::CREF { componentRef: cref.clone(), ty: t_real() };
        ctx.sim_assign(&lhs, exp).map_err(|e| anyhow!("in {which} binding: {e}"))?;
    }
    for eq in &eqs {
        lower_equation(&mut ctx, eq, eq_index)
            .map_err(|e| anyhow!("in {which}: {e}"))?;
    }
    let (locals, instrs) = ctx.finish_sim();
    let mut func = we::Function::new(locals.into_iter().map(|t| (1u32, t)));
    for i in &instrs {
        func.instruction(i);
    }
    Ok(func)
}

/// Lower a single `SimEqSystem` into the current equation function.
fn lower_equation(
    ctx: &mut FnCtx,
    eq: &SimCode::SimEqSystem,
    eq_index: &HashMap<i32, Arc<SimCode::SimEqSystem>>,
) -> Result<()> {
    use SimCode::SimEqSystem as E;
    match eq {
        E::SES_SIMPLE_ASSIGN { cref, exp, .. } => {
            let lhs = DAE::Exp::CREF { componentRef: cref.clone(), ty: t_real() };
            ctx.sim_assign(&lhs, exp)
        }
        E::SES_ALGORITHM { statements, .. } => ctx.sim_stmts(statements),
        // An alias equation re-runs another equation (by index): inline it.
        E::SES_ALIAS { aliasOf, .. } => {
            let target = eq_index
                .get(aliasOf)
                .ok_or_else(|| anyhow!("SES_ALIAS references unknown equation index {aliasOf}"))?
                .clone();
            lower_equation(ctx, &target, eq_index)
        }
        other => bail!(
            "CodegenWasmJit: unsupported equation kind in simulation (only simple assignments and \
             algorithms are handled so far): {} (index {})",
            eq_kind_name(other),
            eq_index_of(other),
        ),
    }
}

fn eq_kind_name(eq: &SimCode::SimEqSystem) -> &'static str {
    use SimCode::SimEqSystem as E;
    match eq {
        E::SES_RESIDUAL { .. } => "SES_RESIDUAL",
        E::SES_FOR_RESIDUAL { .. } => "SES_FOR_RESIDUAL",
        E::SES_GENERIC_RESIDUAL { .. } => "SES_GENERIC_RESIDUAL",
        E::SES_SIMPLE_ASSIGN { .. } => "SES_SIMPLE_ASSIGN",
        E::SES_SIMPLE_ASSIGN_CONSTRAINTS { .. } => "SES_SIMPLE_ASSIGN_CONSTRAINTS",
        E::SES_ARRAY_CALL_ASSIGN { .. } => "SES_ARRAY_CALL_ASSIGN",
        E::SES_LINEAR { .. } => "SES_LINEAR",
        E::SES_NONLINEAR { .. } => "SES_NONLINEAR",
        E::SES_MIXED { .. } => "SES_MIXED",
        E::SES_WHEN { .. } => "SES_WHEN",
        E::SES_IFEQUATION { .. } => "SES_IFEQUATION",
        E::SES_ALGORITHM { .. } => "SES_ALGORITHM",
        E::SES_INVERSE_ALGORITHM { .. } => "SES_INVERSE_ALGORITHM",
        E::SES_RESIZABLE_ASSIGN { .. } => "SES_RESIZABLE_ASSIGN",
        E::SES_GENERIC_ASSIGN { .. } => "SES_GENERIC_ASSIGN",
        E::SES_ENTWINED_ASSIGN { .. } => "SES_ENTWINED_ASSIGN",
        E::SES_FOR_LOOP { .. } => "SES_FOR_LOOP",
        E::SES_FOR_EQUATION { .. } => "SES_FOR_EQUATION",
        E::SES_ALIAS { .. } => "SES_ALIAS",
        E::SES_ALGEBRAIC_SYSTEM { .. } => "SES_ALGEBRAIC_SYSTEM",
    }
}

/// The `index` of a `SimEqSystem` (best-effort; systems without a top-level
/// index report -1).
fn eq_index_of(eq: &SimCode::SimEqSystem) -> i32 {
    use SimCode::SimEqSystem as E;
    match eq {
        E::SES_RESIDUAL { index, .. }
        | E::SES_FOR_RESIDUAL { index, .. }
        | E::SES_GENERIC_RESIDUAL { index, .. }
        | E::SES_SIMPLE_ASSIGN { index, .. }
        | E::SES_SIMPLE_ASSIGN_CONSTRAINTS { index, .. }
        | E::SES_ARRAY_CALL_ASSIGN { index, .. }
        | E::SES_RESIZABLE_ASSIGN { index, .. }
        | E::SES_GENERIC_ASSIGN { index, .. }
        | E::SES_ENTWINED_ASSIGN { index, .. }
        | E::SES_IFEQUATION { index, .. }
        | E::SES_ALGORITHM { index, .. }
        | E::SES_INVERSE_ALGORITHM { index, .. }
        | E::SES_MIXED { index, .. }
        | E::SES_WHEN { index, .. }
        | E::SES_FOR_LOOP { index, .. } => *index,
        _ => -1,
    }
}

/// Emit the in-wasm forward-Euler integrator loop:
/// `simulate(sim_data, start, stop, n_steps) -> result_buffer`.
fn build_simulate(layout: &SimLayout, eqfn: &EqFnIdx) -> we::Function {
    // Params: 0 sim_data(i32), 1 start(f64), 2 stop(f64), 3 n_steps(i32).
    // Locals: 4 buf(i32), 5 h(f64), 6 row(i32).
    const SIM_DATA: u32 = 0;
    const START: u32 = 1;
    const STOP: u32 = 2;
    const N_STEPS: u32 = 3;
    const BUF: u32 = 4;
    const H: u32 = 5;
    const ROW: u32 = 6;

    let n_reals = layout.n_reals_row();
    let n_states = layout.n_states;
    let mut f = we::Function::new([(1, we::ValType::I32), (1, we::ValType::F64), (1, we::ValType::I32)]);
    use we::Instruction as I;

    // functionParameters(sim_data); functionInitialEquations(sim_data)
    f.instruction(&I::LocalGet(SIM_DATA));
    f.instruction(&I::Call(eqfn.parameters));
    f.instruction(&I::LocalGet(SIM_DATA));
    f.instruction(&I::Call(eqfn.initial));

    // buf = rt_alloc((n_steps + 1) * n_reals * 8)
    f.instruction(&I::LocalGet(N_STEPS));
    f.instruction(&I::I32Const(1));
    f.instruction(&I::I32Add);
    f.instruction(&I::I32Const((n_reals * 8) as i32));
    f.instruction(&I::I32Mul);
    f.instruction(&I::Call(rt_index("rt_alloc")));
    f.instruction(&I::LocalSet(BUF));

    // h = (stop - start) / n_steps   (n_steps converted to f64)
    f.instruction(&I::LocalGet(STOP));
    f.instruction(&I::LocalGet(START));
    f.instruction(&I::F64Sub);
    f.instruction(&I::LocalGet(N_STEPS));
    f.instruction(&I::F64ConvertI32S);
    f.instruction(&I::F64Div);
    f.instruction(&I::LocalSet(H));

    // row = 0
    f.instruction(&I::I32Const(0));
    f.instruction(&I::LocalSet(ROW));

    // block { loop {
    f.instruction(&I::Block(we::BlockType::Empty));
    f.instruction(&I::Loop(we::BlockType::Empty));

    // time = start + row * h
    f.instruction(&I::LocalGet(SIM_DATA));
    f.instruction(&I::LocalGet(START));
    f.instruction(&I::LocalGet(ROW));
    f.instruction(&I::F64ConvertI32S);
    f.instruction(&I::LocalGet(H));
    f.instruction(&I::F64Mul);
    f.instruction(&I::F64Add);
    f.instruction(&I::F64Store(crate::CodegenWasmJitFunctions::mem_arg(TIME_OFF, 3)));

    // functionODE(sim_data); functionAlgebraics(sim_data)
    f.instruction(&I::LocalGet(SIM_DATA));
    f.instruction(&I::Call(eqfn.ode));
    f.instruction(&I::LocalGet(SIM_DATA));
    f.instruction(&I::Call(eqfn.algebraics));

    // rt_sim_store_row(buf, row, sim_data, n_reals)
    f.instruction(&I::LocalGet(BUF));
    f.instruction(&I::LocalGet(ROW));
    f.instruction(&I::LocalGet(SIM_DATA));
    f.instruction(&I::I32Const(n_reals as i32));
    f.instruction(&I::Call(rt_index("rt_sim_store_row")));

    // if row >= n_steps: break (exit the block)
    f.instruction(&I::LocalGet(ROW));
    f.instruction(&I::LocalGet(N_STEPS));
    f.instruction(&I::I32GeS);
    f.instruction(&I::BrIf(1)); // branch out of the loop to the block end

    // rt_euler_step(sim_data, n_states, h)
    f.instruction(&I::LocalGet(SIM_DATA));
    f.instruction(&I::I32Const(n_states as i32));
    f.instruction(&I::LocalGet(H));
    f.instruction(&I::Call(rt_index("rt_euler_step")));

    // row += 1; continue
    f.instruction(&I::LocalGet(ROW));
    f.instruction(&I::I32Const(1));
    f.instruction(&I::I32Add);
    f.instruction(&I::LocalSet(ROW));
    f.instruction(&I::Br(0));

    f.instruction(&I::End); // loop
    f.instruction(&I::End); // block

    // return buf
    f.instruction(&I::LocalGet(BUF));
    f.instruction(&I::End); // function
    f
}

// ===========================================================================
// MATLAB v4 result-file writer
// ===========================================================================

/// Write the simulation result as an OpenModelica MATLAB v4 (`.mat`) file.
/// `rows` is the row-major result buffer (`n_rows * n_reals` f64: per row,
/// `[time, realVars...]`); `params` come from the [`SimModel`] result vars.
fn write_mat4(model: &SimModel, path: &str, rows: &[f64], n_reals: u32, params: &[f64]) -> Result<()> {
    let n_reals = n_reals as usize;
    let n_rows = if n_reals == 0 { 0 } else { rows.len() / n_reals };
    let signals = &model.result_vars;

    // Names and descriptions, column-major char matrices.
    let names: Vec<&str> = signals.iter().map(|v| v.name.as_str()).collect();
    let descs: Vec<&str> = signals.iter().map(|v| v.comment.as_str()).collect();

    let mut out: Vec<u8> = Vec::new();

    // Aclass (4 x 11 char), rows: "Atrajectory","1.1","","binTrans".
    let aclass_rows = ["Atrajectory", "1.1", "", "binTrans"];
    write_char_matrix_rows(&mut out, "Aclass", &aclass_rows, 11);

    // name / description: each signal occupies one column.
    write_char_matrix_cols(&mut out, "name", &names);
    write_char_matrix_cols(&mut out, "description", &descs);

    // dataInfo (4 x nSignals int32, column-major): [channel, index, interp, extrap].
    let mut data_info: Vec<i32> = Vec::with_capacity(signals.len() * 4);
    // Parameters occupy data_1 rows starting at 2 (row 1 is the reserved
    // start/stop row), in result order.
    let mut next_param_row: i32 = 2;
    for v in signals {
        let info = match &v.kind {
            ResultKind::Time => [0, 1, 0, -1],
            ResultKind::TimeVariant { row } => [2, *row as i32, 0, 0],
            ResultKind::Param { .. } => {
                let r = next_param_row;
                next_param_row += 1;
                [1, r, 0, 0]
            }
        };
        data_info.extend_from_slice(&info);
    }
    write_int_matrix(&mut out, "dataInfo", 4, signals.len(), &data_info);

    // data_1 (nData1 x 2 double, column-major): row 1 = [start, stop]; then one
    // row per parameter (same value in both columns). Parameter values were read
    // from `SimData` by the runner (in result/`Param` order).
    let param_vals: &[f64] = params;
    let n_data1 = 1 + param_vals.len();
    let mut data_1: Vec<f64> = vec![0.0; n_data1 * 2];
    // Column 0 (start values), column 1 (stop values), column-major.
    data_1[0] = model.start_time;
    data_1[n_data1] = model.stop_time;
    for (i, val) in param_vals.iter().enumerate() {
        data_1[1 + i] = *val;
        data_1[n_data1 + 1 + i] = *val;
    }
    write_double_matrix(&mut out, "data_1", n_data1, 2, &data_1);

    // data_2 (n_reals x n_rows double, column-major) — identical byte layout to
    // the row-major result buffer (variable=row, timestep=column).
    write_double_matrix(&mut out, "data_2", n_reals, n_rows, rows);

    std::fs::write(path, &out).map_err(|e| anyhow!("CodegenWasmJit: cannot write {path}: {e}"))?;
    let _ = &model.model_name; // (kept for diagnostics)
    Ok(())
}

/// MATLAB v4 matrix type code: `1000*M + 100*O + 10*P + T`. M=0 (little-endian
/// IEEE), O=0; P selects the element type (0 double, 2 int32, 5 uint8); T=1 for
/// a text (char) matrix, 0 for numeric.
fn mat_type(p: i32, text: bool) -> i32 {
    10 * p + if text { 1 } else { 0 }
}

fn write_mat_header(out: &mut Vec<u8>, name: &str, ty: i32, mrows: usize, ncols: usize) {
    out.extend_from_slice(&ty.to_le_bytes());
    out.extend_from_slice(&(mrows as i32).to_le_bytes());
    out.extend_from_slice(&(ncols as i32).to_le_bytes());
    out.extend_from_slice(&0i32.to_le_bytes()); // imagf
    out.extend_from_slice(&((name.len() + 1) as i32).to_le_bytes());
    out.extend_from_slice(name.as_bytes());
    out.push(0);
}

fn write_double_matrix(out: &mut Vec<u8>, name: &str, mrows: usize, ncols: usize, data: &[f64]) {
    write_mat_header(out, name, mat_type(0, false), mrows, ncols);
    for v in data {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

fn write_int_matrix(out: &mut Vec<u8>, name: &str, mrows: usize, ncols: usize, data: &[i32]) {
    write_mat_header(out, name, mat_type(2, false), mrows, ncols);
    for v in data {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

/// Write a char matrix whose columns are `cols` (each string null-padded to the
/// longest length + 1). Column-major storage: element (r,c) at `c*mrows + r`.
fn write_char_matrix_cols(out: &mut Vec<u8>, name: &str, cols: &[&str]) {
    let mrows = cols.iter().map(|s| s.len()).max().unwrap_or(0) + 1;
    let ncols = cols.len();
    write_mat_header(out, name, mat_type(5, true), mrows, ncols);
    for c in cols {
        let bytes = c.as_bytes();
        for r in 0..mrows {
            out.push(if r < bytes.len() { bytes[r] } else { 0 });
        }
    }
}

/// Write a char matrix from explicit rows (each padded to `ncols`). Column-major
/// storage: element (r,c) at `c*mrows + r`.
fn write_char_matrix_rows(out: &mut Vec<u8>, name: &str, rows: &[&str], ncols: usize) {
    let mrows = rows.len();
    write_mat_header(out, name, mat_type(5, true), mrows, ncols);
    for c in 0..ncols {
        for r in rows {
            let bytes = r.as_bytes();
            out.push(if c < bytes.len() { bytes[c] } else { 0 });
        }
    }
}
