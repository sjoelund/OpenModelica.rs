// Host side of the `wasm-jit` simulation target: JIT the precompiled runtime
// module and the generated model module (sharing one linear memory), then run
// the integration in one of two ways and return the result trajectory.
//
//   * In-wasm driver (default): a single call to the model's `simulate` export,
//     whose emitted loop calls `functionODE`/`functionAlgebraics` and the
//     runtime's `rt_euler_step`/`rt_sim_store_row` with no host boundary
//     crossing per step. Returns a result buffer the host reads out.
//   * Host-driven driver (`OMC_WASM_SIM_DRIVER=host`, for benchmarking): the
//     forward-Euler loop runs in native Rust, calling `functionODE`/
//     `functionAlgebraics` once per step (a wasm boundary crossing each step)
//     and reading/writing `SimData` through the wasm memory.
//
// Both drivers share the same generated model module and the same `SimData`
// layout; only the loop location differs.

use anyhow::{Result, anyhow};
use std::time::Instant;

use super::{REAL_OFF, ResultKind, SimModel, TIME_OFF};
use crate::CodegenWasmJitFunctions::WTy;
use crate::CodegenWasmJitFunctions::runtime::add_host_builtins;

/// The runtime module, embedded the same way the function half embeds it.
static RUNTIME_WASM: &[u8] = include_bytes!("../runtime.wasm");

/// Result of a simulation run.
pub(super) struct RunResult {
    /// Row-major trajectory: `n_rows * n_reals` f64, each row `[time, realVars…]`.
    pub(super) rows: Vec<f64>,
    /// Values per row (`1 + 2*nStates + nAlgs`).
    pub(super) n_reals: u32,
    /// Parameter values (in result `Param` order), read from `SimData` after the run.
    pub(super) params: Vec<f64>,
}

type Store = wasmtime::Store<()>;

fn wt<T>(r: std::result::Result<T, wasmtime::Error>) -> Result<T> {
    r.map_err(|e| anyhow!("{e:?}"))
}

/// Read one little-endian f64 from wasm linear memory at byte address `addr`.
fn read_f64(mem: &wasmtime::Memory, store: &Store, addr: u32) -> Result<f64> {
    let mut b = [0u8; 8];
    mem.read(store, addr as usize, &mut b).map_err(|e| anyhow!("CodegenWasmJit: mem read: {e}"))?;
    Ok(f64::from_le_bytes(b))
}

fn write_f64(mem: &wasmtime::Memory, store: &mut Store, addr: u32, v: f64) -> Result<()> {
    mem.write(store, addr as usize, &v.to_le_bytes()).map_err(|e| anyhow!("CodegenWasmJit: mem write: {e}"))?;
    Ok(())
}

pub(super) fn run(model: &SimModel, host_driven: bool) -> Result<RunResult> {
    let engine = wasmtime::Engine::default();
    let mut linker = wasmtime::Linker::new(&engine);
    add_host_builtins(&mut linker)?;

    let runtime_module = wt(wasmtime::Module::new(&engine, RUNTIME_WASM))?;
    let model_module = wt(wasmtime::Module::new(&engine, &model.wasm))?;

    let mut store = wasmtime::Store::new(&engine, ());
    let rt_inst = wt(linker.instantiate(&mut store, &runtime_module))?;
    // The generated module imports the runtime's exports under module name "rt".
    wt(linker.instance(&mut store, "rt", rt_inst))?;
    let instance = wt(linker.instantiate(&mut store, &model_module))?;

    let memory = rt_inst
        .get_memory(&mut store, "memory")
        .ok_or_else(|| anyhow!("CodegenWasmJit: runtime has no `memory` export"))?;
    let rt_alloc = wt(rt_inst.get_typed_func::<u32, u32>(&mut store, "rt_alloc"))?;

    let layout = &model.layout;
    let n_reals = layout.n_reals_row();
    let n_steps = model.n_intervals;
    let n_rows = n_steps + 1;

    // Allocate the shared SimData block.
    let sim_data = wt(rt_alloc.call(&mut store, layout.total))?;

    let start = model.start_time;
    let stop = model.stop_time;

    let t0 = Instant::now();
    let rows: Vec<f64> = if host_driven {
        run_host(&mut store, &instance, &memory, model, sim_data, n_reals, n_rows, start, stop)?
    } else {
        run_wasm(&mut store, &instance, &memory, sim_data, n_reals, n_rows, start, stop)?
    };
    let elapsed = t0.elapsed();
    if std::env::var("OMC_WASM_SIM_BENCH").is_ok() {
        eprintln!(
            "wasm-jit sim [{}] {} steps in {:?} ({:.2} us/step)",
            if host_driven { "host" } else { "wasm" },
            n_steps,
            elapsed,
            elapsed.as_secs_f64() * 1e6 / (n_rows.max(1) as f64),
        );
    }

    // Read parameter values from SimData (result `Param` order).
    let mut params = Vec::new();
    for v in &model.result_vars {
        if let ResultKind::Param { off, wty } = &v.kind {
            let val = match wty {
                WTy::F64 => read_f64(&memory, &store, sim_data + off)?,
                WTy::I32 => {
                    let mut b = [0u8; 4];
                    memory.read(&store, (sim_data + off) as usize, &mut b).map_err(|e| anyhow!("{e}"))?;
                    i32::from_le_bytes(b) as f64
                }
            };
            params.push(val);
        }
    }

    Ok(RunResult { rows, n_reals, params })
}

/// In-wasm driver: one call to `simulate`, then read the result buffer.
fn run_wasm(
    store: &mut Store,
    instance: &wasmtime::Instance,
    memory: &wasmtime::Memory,
    sim_data: u32,
    n_reals: u32,
    n_rows: u32,
    start: f64,
    stop: f64,
) -> Result<Vec<f64>> {
    let simulate = wt(instance.get_typed_func::<(u32, f64, f64, u32), u32>(&mut *store, "simulate"))?;
    let buf = wt(simulate.call(&mut *store, (sim_data, start, stop, n_rows - 1)))?;
    let count = (n_rows * n_reals) as usize;
    let mut bytes = vec![0u8; count * 8];
    memory.read(&*store, buf as usize, &mut bytes).map_err(|e| anyhow!("CodegenWasmJit: result read: {e}"))?;
    Ok(bytes.chunks_exact(8).map(|c| f64::from_le_bytes(c.try_into().unwrap())).collect())
}

/// Host-driven driver: the forward-Euler loop in native Rust.
fn run_host(
    store: &mut Store,
    instance: &wasmtime::Instance,
    memory: &wasmtime::Memory,
    model: &SimModel,
    sim_data: u32,
    n_reals: u32,
    n_rows: u32,
    start: f64,
    stop: f64,
) -> Result<Vec<f64>> {
    let f_params = wt(instance.get_typed_func::<u32, ()>(&mut *store, "functionParameters"))?;
    let f_init = wt(instance.get_typed_func::<u32, ()>(&mut *store, "functionInitialEquations"))?;
    let f_ode = wt(instance.get_typed_func::<u32, ()>(&mut *store, "functionODE"))?;
    let f_alg = wt(instance.get_typed_func::<u32, ()>(&mut *store, "functionAlgebraics"))?;

    wt(f_params.call(&mut *store, sim_data))?;
    wt(f_init.call(&mut *store, sim_data))?;

    let n_states = model.layout.n_states;
    let n_steps = n_rows - 1;
    let h = if n_steps == 0 { 0.0 } else { (stop - start) / n_steps as f64 };
    let states_base = sim_data + REAL_OFF;
    let ders_base = states_base + n_states * 8;

    let mut rows: Vec<f64> = Vec::with_capacity((n_rows * n_reals) as usize);
    for row in 0..n_rows {
        let time = start + row as f64 * h;
        write_f64(memory, store, sim_data + TIME_OFF, time)?;
        wt(f_ode.call(&mut *store, sim_data))?;
        wt(f_alg.call(&mut *store, sim_data))?;
        // Store the row: [time, realVars…] = first n_reals f64 of SimData.
        for i in 0..n_reals {
            rows.push(read_f64(memory, store, sim_data + i * 8)?);
        }
        if row == n_steps {
            break;
        }
        // Forward-Euler update of the states.
        for i in 0..n_states {
            let s = read_f64(memory, store, states_base + i * 8)?;
            let d = read_f64(memory, store, ders_base + i * 8)?;
            write_f64(memory, store, states_base + i * 8, s + h * d)?;
        }
    }
    Ok(rows)
}
