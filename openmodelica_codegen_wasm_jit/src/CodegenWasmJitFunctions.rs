// Manually written file (the `CodegenWasmJitFunctions` MetaModelica package is a
// placeholder; see HANDWRITTEN_TOP_PACKAGES in mmtorust/src/codegen.rs).
//
// The `wasm-jit` simCodeTarget, function half. Counterpart of
// `CodegenCFunctions` for the C target and of `DynLoad`/`DynLoadExt` for the
// execute side: instead of generating C, building a shared object and
// `dlopen`ing it, the `-d=gen` functions are lowered to a WebAssembly module
// that is JIT-compiled and run in-process with `wasmtime`. This skips the
// gcc/clang invocation, which dominates the latency of interactive function
// evaluation.
//
// `translateFunctions` lowers the `SimCodeFunction.FunctionCode` to a `.wasm`
// module (via the `wasm-encoder` crate) plus a small `.wasm.sig` sidecar that
// records the input/output scalar types (the wasm value types alone cannot tell
// Integer from Boolean). `loadAndExecute` reads them back, instantiates the
// module and calls the exported entry `main`, marshalling `Values.Value`s in
// and out.
//
// SCOPE (first step): scalar functions over Integer / Real / Boolean (and
// Enumeration literals, treated as their Integer index). Arithmetic,
// comparisons, `if`/`while`/`for`, calls to other generated functions and a
// curated set of math builtins are supported. String and structured
// (list/record/array) values are NOT yet handled: lowering a function that
// needs them fails cleanly (the module is not written and `loadAndExecute`
// returns `Values.META_FAIL`), exactly as if constant evaluation had failed —
// the caller can fall back to the C target. Strings need a linear-memory
// representation plus host-imported runtime builtins and are the next step.

// The two entry points keep their MetaModelica camelCase names so the generated
// `CevalScript` caller resolves them; the rest of the module is idiomatic Rust.
#![allow(non_snake_case)]

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Result, bail};
use arcstr::ArcStr;
use metamodelica::List;

use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::{DAE, Values};
use openmodelica_simcode_types::SimCodeFunction;

use wasm_encoder as we;

mod runtime;

/// A wasm value type. MetaModelica `Integer` is the port's `i32`
/// ([[funcbuiltin-i32-intmaxlit]]); `Boolean` and `Enumeration` indices also
/// live in an `i32`; `Real` is an `f64`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum WTy {
    I32,
    F64,
}

impl WTy {
    fn val(self) -> we::ValType {
        match self {
            WTy::I32 => we::ValType::I32,
            WTy::F64 => we::ValType::F64,
        }
    }
}

/// One scalar Modelica type, as recorded in the `.wasm.sig` sidecar so
/// `loadAndExecute` can map wasm scalars back to the right `Values.Value`
/// constructor (an `i32` result is ambiguous between Integer and Boolean).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SigTy {
    Int,
    Real,
    Bool,
}

impl SigTy {
    fn code(self) -> char {
        match self {
            SigTy::Int => 'I',
            SigTy::Real => 'R',
            SigTy::Bool => 'B',
        }
    }
    fn from_code(c: char) -> Result<SigTy> {
        Ok(match c {
            'I' => SigTy::Int,
            'R' => SigTy::Real,
            'B' => SigTy::Bool,
            other => bail!("CodegenWasmJit: unknown signature type code {other:?}"),
        })
    }
    fn wty(self) -> WTy {
        match self {
            SigTy::Int | SigTy::Bool => WTy::I32,
            SigTy::Real => WTy::F64,
        }
    }
}

/// Host-imported math builtins, in a fixed order so their wasm function indices
/// are stable: index `i` is `BUILTINS[i]`. Every generated module imports all
/// of them from module `"env"` (the runtime `Linker` provides them all); unused
/// imports cost nothing at runtime. Builtins implementable with a single wasm
/// instruction (`sqrt`, `abs`, `floor`, `ceil`, `min`, `max`, …) are emitted
/// inline instead and are not in this table.
const BUILTINS: &[(&str, &[WTy], WTy)] = &[
    ("pow", &[WTy::F64, WTy::F64], WTy::F64),
    ("atan2", &[WTy::F64, WTy::F64], WTy::F64),
    ("sin", &[WTy::F64], WTy::F64),
    ("cos", &[WTy::F64], WTy::F64),
    ("tan", &[WTy::F64], WTy::F64),
    ("asin", &[WTy::F64], WTy::F64),
    ("acos", &[WTy::F64], WTy::F64),
    ("atan", &[WTy::F64], WTy::F64),
    ("sinh", &[WTy::F64], WTy::F64),
    ("cosh", &[WTy::F64], WTy::F64),
    ("tanh", &[WTy::F64], WTy::F64),
    ("exp", &[WTy::F64], WTy::F64),
    ("log", &[WTy::F64], WTy::F64),
    ("log10", &[WTy::F64], WTy::F64),
];

fn builtin_index(name: &str) -> Option<u32> {
    BUILTINS.iter().position(|(n, _, _)| *n == name).map(|i| i as u32)
}

/// `_`-mangled name of a function path, matching `CevalScript`'s
/// `generateFunctionName` (`AbsynUtil.pathStringUnquoteReplaceDot(path, "_")`).
/// Used as the key that resolves a `CALL` to one of the generated functions.
fn mangle(path: &Absyn::Path) -> Result<String> {
    Ok(AbsynUtil::pathStringUnquoteReplaceDot(Arc::new(path.clone()), arcstr::literal!("_"))?.to_string())
}

// -------------------------------------------------------------------------
// Module assembly
// -------------------------------------------------------------------------

/// Signature of a generated wasm function: parameter and result value types.
#[derive(Clone)]
struct FnSig {
    params: Vec<WTy>,
    results: Vec<WTy>,
}

/// Everything the second pass needs to resolve a `CALL` to another generated
/// function: its final wasm function index and signature.
struct FnInfo {
    index: u32,
    sig: FnSig,
}

/// Build the wasm module for `fnCode`. Returns the encoded module bytes and the
/// input/output `SigTy`s of the main function (for the sidecar).
fn build_module(fn_code: &SimCodeFunction::FunctionCode) -> Result<(Vec<u8>, Vec<SigTy>, Vec<SigTy>)> {
    // Collect the functions: the main function first (wasm index BUILTINS.len()),
    // then the dependencies.
    let mut funcs: Vec<&SimCodeFunction::Function::Function> = Vec::new();
    let Some(main) = &fn_code.mainFunction else {
        bail!("CodegenWasmJit: function code has no main function");
    };
    funcs.push(&**main);
    for f in &*fn_code.functions {
        funcs.push(&**f);
    }

    let base = BUILTINS.len() as u32;
    // Map mangled function name -> (local id, signature) so CALLs can resolve.
    let mut by_name: HashMap<String, FnInfo> = HashMap::new();
    let mut sigs: Vec<FnSig> = Vec::with_capacity(funcs.len());
    for (id, f) in funcs.iter().enumerate() {
        let (name, sig) = function_signature(f)?;
        by_name.insert(name, FnInfo { index: base + id as u32, sig: sig.clone() });
        sigs.push(sig);
    }

    // Type section: one type per builtin, then one per generated function.
    let mut types = we::TypeSection::new();
    for (_, params, result) in BUILTINS {
        types.ty().function(params.iter().map(|w| w.val()), [result.val()]);
    }
    for sig in &sigs {
        types.ty().function(sig.params.iter().map(|w| w.val()), sig.results.iter().map(|w| w.val()));
    }

    // Import section: all builtins from module "env", using type index i.
    let mut imports = we::ImportSection::new();
    for (i, (name, _, _)) in BUILTINS.iter().enumerate() {
        imports.import("env", *name, we::EntityType::Function(i as u32));
    }

    // Function + code sections.
    let mut functions = we::FunctionSection::new();
    let mut code = we::CodeSection::new();
    for (id, f) in funcs.iter().enumerate() {
        functions.function(base + id as u32); // type index = base + id
        let body = compile_function(f, &by_name)?;
        code.function(&body);
    }

    // Export the main function as "main".
    let mut exports = we::ExportSection::new();
    exports.export("main", we::ExportKind::Func, base);

    let mut module = we::Module::new();
    module.section(&types);
    module.section(&imports);
    module.section(&functions);
    module.section(&exports);
    module.section(&code);
    let bytes = module.finish();

    // Signature types of the main function for the sidecar.
    let (in_sig, out_sig) = main_sig_types(main)?;
    Ok((bytes, in_sig, out_sig))
}

/// The mangled name and wasm signature of a generated function.
fn function_signature(f: &SimCodeFunction::Function::Function) -> Result<(String, FnSig)> {
    let SimCodeFunction::Function::Function::FUNCTION { name, outVars, functionArguments, .. } = f else {
        bail!("CodegenWasmJit: only plain Modelica/MetaModelica FUNCTIONs are supported (no external/record-constructor)");
    };
    let params = var_wtys(functionArguments)?;
    let results = var_wtys(outVars)?;
    Ok((mangle(name)?, FnSig { params, results }))
}

/// The input/output scalar `SigTy`s of the main function, for the sidecar.
fn main_sig_types(f: &SimCodeFunction::Function::Function) -> Result<(Vec<SigTy>, Vec<SigTy>)> {
    let SimCodeFunction::Function::Function::FUNCTION { outVars, functionArguments, .. } = f else {
        bail!("CodegenWasmJit: only plain FUNCTIONs are supported");
    };
    Ok((var_sigtys(functionArguments)?, var_sigtys(outVars)?))
}

fn var_wtys(vars: &Arc<List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Vec<WTy>> {
    var_sigtys(vars).map(|v| v.into_iter().map(|s| s.wty()).collect())
}

fn var_sigtys(vars: &Arc<List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Vec<SigTy>> {
    let mut out = Vec::new();
    for v in &**vars {
        let SimCodeFunction::Variable::Variable::VARIABLE { ty, .. } = &**v else {
            bail!("CodegenWasmJit: unsupported variable kind (function pointer)");
        };
        out.push(sig_ty(ty)?);
    }
    Ok(out)
}

/// Map a scalar `DAE.Type` to a `SigTy`, or fail for non-scalar types.
fn sig_ty(ty: &DAE::Type) -> Result<SigTy> {
    Ok(match ty {
        DAE::Type::T_INTEGER { .. } => SigTy::Int,
        DAE::Type::T_REAL { .. } => SigTy::Real,
        DAE::Type::T_BOOL { .. } => SigTy::Bool,
        // An enumeration value is its 1-based Integer index.
        DAE::Type::T_ENUMERATION { .. } => SigTy::Int,
        DAE::Type::T_SUBTYPE_BASIC { .. } => bail!("CodegenWasmJit: subtype-basic types not yet supported"),
        DAE::Type::T_STRING { .. } => bail!("CodegenWasmJit: String not yet supported"),
        other => bail!("CodegenWasmJit: non-scalar type not supported: {other:?}"),
    })
}

// -------------------------------------------------------------------------
// Function-body compilation
// -------------------------------------------------------------------------

/// Per-function compilation state.
struct FnCtx<'a> {
    /// ident -> (local index, type). Inputs are the wasm params (indices
    /// `0..n_in`); outputs and locals follow.
    locals: HashMap<String, (u32, WTy)>,
    /// Wasm types of every non-parameter local (for `Function::new`), in index
    /// order starting at `n_params`.
    extra_locals: Vec<we::ValType>,
    n_params: u32,
    /// Output local indices, pushed (in order) before every `return`.
    outputs: Vec<(u32, WTy)>,
    /// Resolves a `CALL` to another generated function.
    by_name: &'a HashMap<String, FnInfo>,
    instrs: Vec<we::Instruction<'static>>,
}

impl<'a> FnCtx<'a> {
    fn emit(&mut self, i: we::Instruction<'static>) {
        self.instrs.push(i);
    }
    /// Allocate a fresh scratch local of the given type and return its index.
    /// Never reused, so transient uses inside one expression never clobber.
    fn alloc_temp(&mut self, wty: WTy) -> u32 {
        let idx = self.n_params + self.extra_locals.len() as u32;
        self.extra_locals.push(wty.val());
        idx
    }
}

fn compile_function(
    f: &SimCodeFunction::Function::Function,
    by_name: &HashMap<String, FnInfo>,
) -> Result<we::Function> {
    let SimCodeFunction::Function::Function::FUNCTION { outVars, functionArguments, variableDeclarations, body, .. } = f
    else {
        bail!("CodegenWasmJit: only plain FUNCTIONs are supported");
    };

    let mut locals: HashMap<String, (u32, WTy)> = HashMap::new();
    let mut idx: u32 = 0;
    // Parameters first (wasm locals 0..n_in).
    for v in &**functionArguments {
        let (name, wty) = var_name_ty(v)?;
        locals.insert(name, (idx, wty));
        idx += 1;
    }
    let n_params = idx;
    let mut extra_locals: Vec<we::ValType> = Vec::new();
    let mut outputs: Vec<(u32, WTy)> = Vec::new();
    // Outputs next, then local declarations. An output is often also listed in
    // `variableDeclarations` (the function body assigns to it through the same
    // name); it must map to a single local, so a name already allocated as an
    // input or output is reused rather than given a fresh slot.
    for v in &**outVars {
        let (name, wty) = var_name_ty(v)?;
        let slot = *locals.entry(name).or_insert_with(|| {
            let s = (idx, wty);
            extra_locals.push(wty.val());
            idx += 1;
            s
        });
        outputs.push(slot);
    }
    for v in &**variableDeclarations {
        let (name, wty) = var_name_ty(v)?;
        locals.entry(name).or_insert_with(|| {
            let s = (idx, wty);
            extra_locals.push(wty.val());
            idx += 1;
            s
        });
    }

    let mut ctx = FnCtx { locals, extra_locals, n_params, outputs, by_name, instrs: Vec::new() };
    compile_stmts(&mut ctx, body)?;
    // Fall-through return: push the output locals and end.
    push_outputs(&mut ctx);
    ctx.emit(we::Instruction::End);

    let FnCtx { extra_locals, instrs, .. } = ctx;
    let mut func = we::Function::new(extra_locals.into_iter().map(|t| (1u32, t)));
    for i in &instrs {
        func.instruction(i);
    }
    Ok(func)
}

fn push_outputs(ctx: &mut FnCtx) {
    for (idx, _) in ctx.outputs.clone() {
        ctx.emit(we::Instruction::LocalGet(idx));
    }
}

/// Name and wasm type of a `VARIABLE`. Only `CREF_IDENT` scalars are supported.
fn var_name_ty(v: &SimCodeFunction::Variable::Variable) -> Result<(String, WTy)> {
    let SimCodeFunction::Variable::Variable::VARIABLE { name, ty, .. } = v else {
        bail!("CodegenWasmJit: function-pointer variables not supported");
    };
    Ok((cref_ident(name)?, sig_ty(ty)?.wty()))
}

/// The identifier of a scalar `CREF_IDENT` component reference (no subscripts /
/// qualification, which only arise for arrays / records).
fn cref_ident(cr: &DAE::ComponentRef) -> Result<String> {
    match cr {
        DAE::ComponentRef::CREF_IDENT { ident, subscriptLst, .. } => {
            if !subscriptLst.is_empty() {
                bail!("CodegenWasmJit: subscripted component reference (arrays not supported)");
            }
            Ok(ident.to_string())
        }
        DAE::ComponentRef::CREF_QUAL { .. } => bail!("CodegenWasmJit: qualified component reference (records not supported)"),
        other => bail!("CodegenWasmJit: unsupported component reference {other:?}"),
    }
}

fn compile_stmts(ctx: &mut FnCtx, stmts: &Arc<List<Arc<DAE::Statement>>>) -> Result<()> {
    for s in &**stmts {
        compile_stmt(ctx, s)?;
    }
    Ok(())
}

fn compile_stmt(ctx: &mut FnCtx, stmt: &DAE::Statement) -> Result<()> {
    use DAE::Statement as S;
    match stmt {
        S::STMT_ASSIGN { exp1, exp, .. } => {
            // The lhs must be a scalar local.
            let DAE::Exp::CREF { componentRef, .. } = &**exp1 else {
                bail!("CodegenWasmJit: assignment to non-cref lhs not supported");
            };
            let ident = cref_ident(componentRef)?;
            let (idx, dst_wty) = *ctx
                .locals
                .get(&ident)
                .ok_or_else(|| anyhow::anyhow!("CodegenWasmJit: assignment to unknown variable `{ident}`"))?;
            let src_wty = compile_exp(ctx, exp)?;
            coerce(ctx, src_wty, dst_wty);
            ctx.emit(we::Instruction::LocalSet(idx));
            Ok(())
        }
        S::STMT_IF { exp, statementLst, else_, .. } => {
            let c = compile_exp(ctx, exp)?;
            coerce(ctx, c, WTy::I32);
            ctx.emit(we::Instruction::If(we::BlockType::Empty));
            compile_stmts(ctx, statementLst)?;
            compile_else(ctx, else_)?;
            ctx.emit(we::Instruction::End);
            Ok(())
        }
        S::STMT_WHILE { exp, statementLst, .. } => {
            // block { loop { <cond>; i32.eqz; br_if 1; <body>; br 0 } }
            ctx.emit(we::Instruction::Block(we::BlockType::Empty));
            ctx.emit(we::Instruction::Loop(we::BlockType::Empty));
            let c = compile_exp(ctx, exp)?;
            coerce(ctx, c, WTy::I32);
            ctx.emit(we::Instruction::I32Eqz);
            ctx.emit(we::Instruction::BrIf(1));
            compile_stmts(ctx, statementLst)?;
            ctx.emit(we::Instruction::Br(0));
            ctx.emit(we::Instruction::End); // loop
            ctx.emit(we::Instruction::End); // block
            Ok(())
        }
        S::STMT_RETURN { .. } => {
            push_outputs(ctx);
            ctx.emit(we::Instruction::Return);
            Ok(())
        }
        S::STMT_NORETCALL { exp, .. } => {
            // Evaluate for side effects and drop any results.
            let n = compile_call_drop(ctx, exp)?;
            for _ in 0..n {
                ctx.emit(we::Instruction::Drop);
            }
            Ok(())
        }
        S::STMT_ASSERT { cond, .. } => {
            // Trap on a failed assertion: the wasm call fails and loadAndExecute
            // returns META_FAIL, matching a runtime assertion failure. The
            // message/level are not yet propagated (they need strings).
            let c = compile_exp(ctx, cond)?;
            coerce(ctx, c, WTy::I32);
            ctx.emit(we::Instruction::I32Eqz);
            ctx.emit(we::Instruction::If(we::BlockType::Empty));
            ctx.emit(we::Instruction::Unreachable);
            ctx.emit(we::Instruction::End);
            Ok(())
        }
        S::STMT_FOR { iter, range, statementLst, type_, .. } => compile_for(ctx, iter, range, statementLst, type_),
        other => bail!("CodegenWasmJit: statement not yet supported: {other:?}"),
    }
}

fn compile_else(ctx: &mut FnCtx, e: &DAE::Else) -> Result<()> {
    match e {
        DAE::Else::NOELSE => Ok(()),
        DAE::Else::ELSE { statementLst } => {
            ctx.emit(we::Instruction::Else);
            compile_stmts(ctx, statementLst)
        }
        DAE::Else::ELSEIF { exp, statementLst, else_ } => {
            ctx.emit(we::Instruction::Else);
            let c = compile_exp(ctx, exp)?;
            coerce(ctx, c, WTy::I32);
            ctx.emit(we::Instruction::If(we::BlockType::Empty));
            compile_stmts(ctx, statementLst)?;
            compile_else(ctx, else_)?;
            ctx.emit(we::Instruction::End);
            Ok(())
        }
    }
}

/// Lower `for iter in start:stop loop ...` (unit step) and `start:step:stop`
/// over an Integer range. Other ranges (Real, arrays) are not yet supported.
fn compile_for(
    ctx: &mut FnCtx,
    iter: &ArcStr,
    range: &DAE::Exp,
    body: &Arc<List<Arc<DAE::Statement>>>,
    _ty: &DAE::Type,
) -> Result<()> {
    let DAE::Exp::RANGE { start, step, stop, .. } = range else {
        bail!("CodegenWasmJit: for-loop over non-range expression not supported");
    };
    // Allocate the iterator local and stop/step locals.
    let it = ctx.alloc_temp(WTy::I32);
    ctx.locals.insert(iter.to_string(), (it, WTy::I32));
    let stop_l = ctx.alloc_temp(WTy::I32);
    let step_l = ctx.alloc_temp(WTy::I32);

    let sw = compile_exp(ctx, start)?;
    coerce(ctx, sw, WTy::I32);
    ctx.emit(we::Instruction::LocalSet(it));
    match step {
        Some(e) => {
            let w = compile_exp(ctx, e)?;
            coerce(ctx, w, WTy::I32);
        }
        None => ctx.emit(we::Instruction::I32Const(1)),
    }
    ctx.emit(we::Instruction::LocalSet(step_l));
    let pw = compile_exp(ctx, stop)?;
    coerce(ctx, pw, WTy::I32);
    ctx.emit(we::Instruction::LocalSet(stop_l));

    // block { loop { (it>stop) -> br 1; body; it+=step; br 0 } }
    // Assumes a positive step (the common case for generated loops).
    ctx.emit(we::Instruction::Block(we::BlockType::Empty));
    ctx.emit(we::Instruction::Loop(we::BlockType::Empty));
    ctx.emit(we::Instruction::LocalGet(it));
    ctx.emit(we::Instruction::LocalGet(stop_l));
    ctx.emit(we::Instruction::I32GtS);
    ctx.emit(we::Instruction::BrIf(1));
    compile_stmts(ctx, body)?;
    ctx.emit(we::Instruction::LocalGet(it));
    ctx.emit(we::Instruction::LocalGet(step_l));
    ctx.emit(we::Instruction::I32Add);
    ctx.emit(we::Instruction::LocalSet(it));
    ctx.emit(we::Instruction::Br(0));
    ctx.emit(we::Instruction::End); // loop
    ctx.emit(we::Instruction::End); // block
    Ok(())
}

// -------------------------------------------------------------------------
// Expression compilation
// -------------------------------------------------------------------------

/// Compile an expression, leaving exactly one value on the wasm stack; returns
/// its type.
fn compile_exp(ctx: &mut FnCtx, exp: &DAE::Exp) -> Result<WTy> {
    use DAE::Exp as E;
    match exp {
        E::ICONST { integer } => {
            ctx.emit(we::Instruction::I32Const(*integer));
            Ok(WTy::I32)
        }
        E::BCONST { bool } => {
            ctx.emit(we::Instruction::I32Const(*bool as i32));
            Ok(WTy::I32)
        }
        E::RCONST { real } => {
            ctx.emit(we::Instruction::F64Const(real.into_inner().into()));
            Ok(WTy::F64)
        }
        E::ENUM_LITERAL { index, .. } => {
            ctx.emit(we::Instruction::I32Const(*index));
            Ok(WTy::I32)
        }
        E::CREF { componentRef, .. } => {
            let ident = cref_ident(componentRef)?;
            let (idx, wty) = *ctx
                .locals
                .get(&ident)
                .ok_or_else(|| anyhow::anyhow!("CodegenWasmJit: reference to unknown variable `{ident}`"))?;
            ctx.emit(we::Instruction::LocalGet(idx));
            Ok(wty)
        }
        E::CAST { ty, exp } => {
            let from = compile_exp(ctx, exp)?;
            let to = sig_ty(ty)?.wty();
            coerce(ctx, from, to);
            Ok(to)
        }
        E::UNARY { operator, exp } => compile_unary(ctx, operator, exp),
        E::LUNARY { operator, exp } => {
            // `not` — the only logical unary.
            let DAE::Operator::NOT { .. } = operator else {
                bail!("CodegenWasmJit: unsupported logical unary operator {operator:?}");
            };
            let w = compile_exp(ctx, exp)?;
            coerce(ctx, w, WTy::I32);
            ctx.emit(we::Instruction::I32Eqz);
            Ok(WTy::I32)
        }
        E::BINARY { exp1, operator, exp2 } => compile_binary(ctx, exp1, operator, exp2),
        E::LBINARY { exp1, operator, exp2 } => {
            let a = compile_exp(ctx, exp1)?;
            coerce(ctx, a, WTy::I32);
            let b = compile_exp(ctx, exp2)?;
            coerce(ctx, b, WTy::I32);
            match operator {
                DAE::Operator::AND { .. } => ctx.emit(we::Instruction::I32And),
                DAE::Operator::OR { .. } => ctx.emit(we::Instruction::I32Or),
                other => bail!("CodegenWasmJit: unsupported logical binary operator {other:?}"),
            }
            Ok(WTy::I32)
        }
        E::RELATION { exp1, operator, exp2, .. } => compile_relation(ctx, exp1, operator, exp2),
        E::IFEXP { expCond, expThen, expElse } => {
            let c = compile_exp(ctx, expCond)?;
            coerce(ctx, c, WTy::I32);
            // Determine the result type from the then-branch; both branches are
            // coerced to it.
            let result_wty = exp_wty_hint(ctx, expThen)?;
            ctx.emit(we::Instruction::If(we::BlockType::Result(result_wty.val())));
            let t = compile_exp(ctx, expThen)?;
            coerce(ctx, t, result_wty);
            ctx.emit(we::Instruction::Else);
            let e = compile_exp(ctx, expElse)?;
            coerce(ctx, e, result_wty);
            ctx.emit(we::Instruction::End);
            Ok(result_wty)
        }
        E::CALL { path, expLst, attr } => {
            let results = compile_call(ctx, path, expLst, attr)?;
            match results.len() {
                1 => Ok(results[0]),
                0 => bail!("CodegenWasmJit: call to {} used in expression position returns no value", mangle(path)?),
                _ => bail!("CodegenWasmJit: call to {} returns multiple values; not usable in expression position", mangle(path)?),
            }
        }
        other => bail!("CodegenWasmJit: expression not yet supported: {other:?}"),
    }
}

/// A cheap static guess of an expression's wasm type, used to pick the result
/// type of an `if`-expression block before compiling the branches.
fn exp_wty_hint(ctx: &FnCtx, exp: &DAE::Exp) -> Result<WTy> {
    use DAE::Exp as E;
    Ok(match exp {
        E::RCONST { .. } => WTy::F64,
        E::ICONST { .. } | E::BCONST { .. } | E::ENUM_LITERAL { .. } | E::RELATION { .. } | E::LBINARY { .. } | E::LUNARY { .. } => WTy::I32,
        E::CAST { ty, .. } => sig_ty(ty)?.wty(),
        E::CREF { componentRef, .. } => {
            let ident = cref_ident(componentRef)?;
            ctx.locals.get(&ident).map(|(_, w)| *w).unwrap_or(WTy::F64)
        }
        E::BINARY { operator, .. } => operator_wty(operator)?,
        E::UNARY { operator, .. } => operator_wty(operator)?,
        E::IFEXP { expThen, .. } => exp_wty_hint(ctx, expThen)?,
        E::CALL { attr, .. } => sig_ty(&attr.ty)?.wty(),
        _ => WTy::F64,
    })
}

fn operator_wty(op: &DAE::Operator) -> Result<WTy> {
    use DAE::Operator as O;
    let ty = match op {
        O::ADD { ty } | O::SUB { ty } | O::MUL { ty } | O::DIV { ty } | O::POW { ty } | O::UMINUS { ty } => ty,
        other => bail!("CodegenWasmJit: cannot determine type of operator {other:?}"),
    };
    Ok(sig_ty(ty)?.wty())
}

fn compile_unary(ctx: &mut FnCtx, op: &DAE::Operator, exp: &DAE::Exp) -> Result<WTy> {
    let DAE::Operator::UMINUS { ty } = op else {
        bail!("CodegenWasmJit: unsupported unary operator {op:?}");
    };
    let wty = sig_ty(ty)?.wty();
    let w = compile_exp(ctx, exp)?;
    coerce(ctx, w, wty);
    match wty {
        WTy::F64 => ctx.emit(we::Instruction::F64Neg),
        WTy::I32 => {
            // 0 - x: reorder via a temp so the constant 0 is below x.
            let t = ctx.alloc_temp(WTy::I32);
            ctx.emit(we::Instruction::LocalSet(t));
            ctx.emit(we::Instruction::I32Const(0));
            ctx.emit(we::Instruction::LocalGet(t));
            ctx.emit(we::Instruction::I32Sub);
        }
    }
    Ok(wty)
}

fn compile_binary(ctx: &mut FnCtx, e1: &DAE::Exp, op: &DAE::Operator, e2: &DAE::Exp) -> Result<WTy> {
    use DAE::Operator as O;
    let wty = operator_wty(op)?;
    // POW has no wasm instruction: route to the host `pow` import.
    if matches!(op, O::POW { .. }) {
        let a = compile_exp(ctx, e1)?;
        coerce(ctx, a, WTy::F64);
        let b = compile_exp(ctx, e2)?;
        coerce(ctx, b, WTy::F64);
        ctx.emit(we::Instruction::Call(builtin_index("pow").unwrap()));
        // Integer power keeps Integer type in Modelica: truncate back.
        if wty == WTy::I32 {
            ctx.emit(we::Instruction::I32TruncF64S);
            return Ok(WTy::I32);
        }
        return Ok(WTy::F64);
    }
    let a = compile_exp(ctx, e1)?;
    coerce(ctx, a, wty);
    let b = compile_exp(ctx, e2)?;
    coerce(ctx, b, wty);
    match (op, wty) {
        (O::ADD { .. }, WTy::F64) => ctx.emit(we::Instruction::F64Add),
        (O::ADD { .. }, WTy::I32) => ctx.emit(we::Instruction::I32Add),
        (O::SUB { .. }, WTy::F64) => ctx.emit(we::Instruction::F64Sub),
        (O::SUB { .. }, WTy::I32) => ctx.emit(we::Instruction::I32Sub),
        (O::MUL { .. }, WTy::F64) => ctx.emit(we::Instruction::F64Mul),
        (O::MUL { .. }, WTy::I32) => ctx.emit(we::Instruction::I32Mul),
        (O::DIV { .. }, WTy::F64) => ctx.emit(we::Instruction::F64Div),
        (O::DIV { .. }, WTy::I32) => ctx.emit(we::Instruction::I32DivS),
        (other, _) => bail!("CodegenWasmJit: unsupported binary operator {other:?}"),
    }
    Ok(wty)
}

fn compile_relation(ctx: &mut FnCtx, e1: &DAE::Exp, op: &DAE::Operator, e2: &DAE::Exp) -> Result<WTy> {
    use DAE::Operator as O;
    let operand_wty = operand_type_of_relation(op)?;
    let a = compile_exp(ctx, e1)?;
    coerce(ctx, a, operand_wty);
    let b = compile_exp(ctx, e2)?;
    coerce(ctx, b, operand_wty);
    let instr = match (op, operand_wty) {
        (O::LESS { .. }, WTy::F64) => we::Instruction::F64Lt,
        (O::LESS { .. }, WTy::I32) => we::Instruction::I32LtS,
        (O::LESSEQ { .. }, WTy::F64) => we::Instruction::F64Le,
        (O::LESSEQ { .. }, WTy::I32) => we::Instruction::I32LeS,
        (O::GREATER { .. }, WTy::F64) => we::Instruction::F64Gt,
        (O::GREATER { .. }, WTy::I32) => we::Instruction::I32GtS,
        (O::GREATEREQ { .. }, WTy::F64) => we::Instruction::F64Ge,
        (O::GREATEREQ { .. }, WTy::I32) => we::Instruction::I32GeS,
        (O::EQUAL { .. }, WTy::F64) => we::Instruction::F64Eq,
        (O::EQUAL { .. }, WTy::I32) => we::Instruction::I32Eq,
        (O::NEQUAL { .. }, WTy::F64) => we::Instruction::F64Ne,
        (O::NEQUAL { .. }, WTy::I32) => we::Instruction::I32Ne,
        (other, _) => bail!("CodegenWasmJit: unsupported relational operator {other:?}"),
    };
    ctx.emit(instr);
    Ok(WTy::I32)
}

fn operand_type_of_relation(op: &DAE::Operator) -> Result<WTy> {
    use DAE::Operator as O;
    let ty = match op {
        O::LESS { ty } | O::LESSEQ { ty } | O::GREATER { ty } | O::GREATEREQ { ty } | O::EQUAL { ty } | O::NEQUAL { ty } => ty,
        other => bail!("CodegenWasmJit: not a relational operator: {other:?}"),
    };
    Ok(sig_ty(ty)?.wty())
}

/// Compile a `CALL`, leaving its result value(s) on the stack; returns their
/// types. Resolves to another generated function, an inline math builtin, or a
/// host-imported builtin.
fn compile_call(
    ctx: &mut FnCtx,
    path: &Absyn::Path,
    args: &Arc<List<Arc<DAE::Exp>>>,
    attr: &DAE::CallAttributes,
) -> Result<Vec<WTy>> {
    let mangled = mangle(path)?;
    // A call to another generated function.
    if let Some(info) = ctx.by_name.get(&mangled) {
        let params = info.sig.params.clone();
        let results = info.sig.results.clone();
        let index = info.index;
        let argv: Vec<&Arc<DAE::Exp>> = (&**args).into_iter().collect();
        if argv.len() != params.len() {
            bail!("CodegenWasmJit: call to {mangled} expects {} args, got {}", params.len(), argv.len());
        }
        for (a, p) in argv.iter().zip(params.iter()) {
            let w = compile_exp(ctx, a)?;
            coerce(ctx, w, *p);
        }
        ctx.emit(we::Instruction::Call(index));
        return Ok(results);
    }
    // Otherwise it must be a (builtin) math function.
    let name = AbsynUtil::pathLastIdent(Arc::new(path.clone()))?.to_string();
    compile_math_builtin(ctx, &name, args, attr).map(|w| vec![w])
}

/// Like [`compile_call`] but for statement position; returns the number of
/// result values left on the stack (to be dropped).
fn compile_call_drop(ctx: &mut FnCtx, exp: &DAE::Exp) -> Result<usize> {
    let DAE::Exp::CALL { path, expLst, attr } = exp else {
        bail!("CodegenWasmJit: no-return statement is not a call: {exp:?}");
    };
    Ok(compile_call(ctx, path, expLst, attr)?.len())
}

/// Lower a scalar math builtin. Single-instruction builtins are emitted inline;
/// transcendental ones go through the host imports in [`BUILTINS`].
fn compile_math_builtin(
    ctx: &mut FnCtx,
    name: &str,
    args: &Arc<List<Arc<DAE::Exp>>>,
    attr: &DAE::CallAttributes,
) -> Result<WTy> {
    let argv: Vec<&Arc<DAE::Exp>> = (&**args).into_iter().collect();
    let result_wty = sig_ty(&attr.ty).unwrap_or(SigTy::Real).wty();

    // Host-imported transcendentals (all operate on f64).
    if let Some(bi) = builtin_index(name) {
        let (_, params, result) = BUILTINS[bi as usize];
        if argv.len() != params.len() {
            bail!("CodegenWasmJit: builtin {name} expects {} args", params.len());
        }
        for (a, p) in argv.iter().zip(params.iter()) {
            let w = compile_exp(ctx, a)?;
            coerce(ctx, w, *p);
        }
        ctx.emit(we::Instruction::Call(bi));
        return Ok(result);
    }

    match name {
        "sqrt" => {
            unary_f64(ctx, &argv, we::Instruction::F64Sqrt)?;
            Ok(WTy::F64)
        }
        "floor" => {
            unary_f64(ctx, &argv, we::Instruction::F64Floor)?;
            Ok(WTy::F64)
        }
        "ceil" => {
            unary_f64(ctx, &argv, we::Instruction::F64Ceil)?;
            Ok(WTy::F64)
        }
        // integer(r): largest Integer <= r.
        "integer" => {
            unary_f64(ctx, &argv, we::Instruction::F64Floor)?;
            ctx.emit(we::Instruction::I32TruncF64S);
            Ok(WTy::I32)
        }
        "abs" => {
            need_args(&argv, 1, name)?;
            if result_wty == WTy::F64 {
                let w = compile_exp(ctx, argv[0])?;
                coerce(ctx, w, WTy::F64);
                ctx.emit(we::Instruction::F64Abs);
                Ok(WTy::F64)
            } else {
                let w = compile_exp(ctx, argv[0])?;
                coerce(ctx, w, WTy::I32);
                let t = ctx.alloc_temp(WTy::I32);
                ctx.emit(we::Instruction::LocalSet(t));
                // select(-x, x, x<0)
                ctx.emit(we::Instruction::I32Const(0));
                ctx.emit(we::Instruction::LocalGet(t));
                ctx.emit(we::Instruction::I32Sub); // -x
                ctx.emit(we::Instruction::LocalGet(t)); // x
                ctx.emit(we::Instruction::LocalGet(t));
                ctx.emit(we::Instruction::I32Const(0));
                ctx.emit(we::Instruction::I32LtS); // x<0
                ctx.emit(we::Instruction::Select);
                Ok(WTy::I32)
            }
        }
        "max" | "min" => {
            need_args(&argv, 2, name)?;
            if result_wty == WTy::F64 {
                let a = compile_exp(ctx, argv[0])?;
                coerce(ctx, a, WTy::F64);
                let b = compile_exp(ctx, argv[1])?;
                coerce(ctx, b, WTy::F64);
                ctx.emit(if name == "max" { we::Instruction::F64Max } else { we::Instruction::F64Min });
                Ok(WTy::F64)
            } else {
                let a = compile_exp(ctx, argv[0])?;
                coerce(ctx, a, WTy::I32);
                let b = compile_exp(ctx, argv[1])?;
                coerce(ctx, b, WTy::I32);
                let tb = ctx.alloc_temp(WTy::I32);
                let ta = ctx.alloc_temp(WTy::I32);
                ctx.emit(we::Instruction::LocalSet(tb));
                ctx.emit(we::Instruction::LocalSet(ta));
                ctx.emit(we::Instruction::LocalGet(ta));
                ctx.emit(we::Instruction::LocalGet(tb));
                ctx.emit(we::Instruction::LocalGet(ta));
                ctx.emit(we::Instruction::LocalGet(tb));
                ctx.emit(if name == "max" { we::Instruction::I32GtS } else { we::Instruction::I32LtS });
                ctx.emit(we::Instruction::Select);
                Ok(WTy::I32)
            }
        }
        // div(a,b): integer division truncating toward zero.
        "div" if result_wty == WTy::I32 => {
            need_args(&argv, 2, name)?;
            let a = compile_exp(ctx, argv[0])?;
            coerce(ctx, a, WTy::I32);
            let b = compile_exp(ctx, argv[1])?;
            coerce(ctx, b, WTy::I32);
            ctx.emit(we::Instruction::I32DivS);
            Ok(WTy::I32)
        }
        // rem(a,b): integer remainder truncating toward zero.
        "rem" if result_wty == WTy::I32 => {
            need_args(&argv, 2, name)?;
            let a = compile_exp(ctx, argv[0])?;
            coerce(ctx, a, WTy::I32);
            let b = compile_exp(ctx, argv[1])?;
            coerce(ctx, b, WTy::I32);
            ctx.emit(we::Instruction::I32RemS);
            Ok(WTy::I32)
        }
        other => bail!("CodegenWasmJit: builtin function `{other}` not yet supported"),
    }
}

fn unary_f64(ctx: &mut FnCtx, argv: &[&Arc<DAE::Exp>], instr: we::Instruction<'static>) -> Result<()> {
    need_args(argv, 1, "<f64 builtin>")?;
    let w = compile_exp(ctx, argv[0])?;
    coerce(ctx, w, WTy::F64);
    ctx.emit(instr);
    Ok(())
}

fn need_args(argv: &[&Arc<DAE::Exp>], n: usize, name: &str) -> Result<()> {
    if argv.len() != n {
        bail!("CodegenWasmJit: builtin {name} expects {n} args, got {}", argv.len());
    }
    Ok(())
}

/// Emit a numeric conversion if the value on the stack is not already the
/// wanted type. Integer/Boolean both live in `i32`, so I32<->I32 is a no-op.
fn coerce(ctx: &mut FnCtx, from: WTy, to: WTy) {
    match (from, to) {
        (WTy::I32, WTy::F64) => ctx.emit(we::Instruction::F64ConvertI32S),
        (WTy::F64, WTy::I32) => ctx.emit(we::Instruction::I32TruncF64S),
        _ => {}
    }
}

// -------------------------------------------------------------------------
// MetaModelica entry points (called from CevalScript)
// -------------------------------------------------------------------------

/// `CodegenWasmJitFunctions.translateFunctions`: lower `fnCode` to a wasm module
/// written to `<name>.wasm` (+ `<name>.wasm.sig`). Infallible at the call site
/// (the MetaModelica declaration cannot fail); on a lowering error nothing is
/// written and the subsequent `loadAndExecute` returns `Values.META_FAIL`, so
/// an unsupported function fails just like a failed constant evaluation. The
/// error is logged to stderr so it is not silently lost.
pub fn translateFunctions(fnCode: SimCodeFunction::FunctionCode) {
    if let Err(e) = translate_functions_inner(&fnCode) {
        eprintln!("CodegenWasmJit: cannot JIT function `{}`: {e:#}", fnCode.name);
        // Remove any stale artefacts from a previous target so loadAndExecute
        // does not pick up an unrelated module.
        let _ = std::fs::remove_file(format!("{}.wasm", fnCode.name));
        let _ = std::fs::remove_file(format!("{}.wasm.sig", fnCode.name));
    }
}

fn translate_functions_inner(fn_code: &SimCodeFunction::FunctionCode) -> Result<()> {
    let (bytes, in_sig, out_sig) = build_module(fn_code)?;
    let base = fn_code.name.to_string();
    std::fs::write(format!("{base}.wasm"), &bytes)?;
    // Sidecar: line 1 = input type codes, line 2 = output type codes.
    let in_codes: String = in_sig.iter().map(|s| s.code()).collect();
    let out_codes: String = out_sig.iter().map(|s| s.code()).collect();
    std::fs::write(format!("{base}.wasm.sig"), format!("{in_codes}\n{out_codes}\n"))?;
    Ok(())
}

/// `CodegenWasmJitFunctions.loadAndExecute`: instantiate `<fileName>.wasm` and
/// call the exported `main`, marshalling `args` in and the result out. Returns
/// `Values.META_FAIL` on any failure (missing/invalid module, a wasm trap from
/// a failed assertion or division by zero, …), mirroring `DynLoad.executeFunction`.
pub fn loadAndExecute(fileName: ArcStr, name: ArcStr, args: Arc<List<Arc<Values::Value>>>) -> Arc<Values::Value> {
    match runtime::load_and_execute(&fileName, &name, &args) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("CodegenWasmJit: execution of `{name}` failed: {e:#}");
            Arc::new(Values::Value::META_FAIL)
        }
    }
}
