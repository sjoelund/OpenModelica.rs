// The execute half of the `wasm-jit` target: load a module produced by
// `super::translateFunctions`, JIT it with `wasmtime` and call its `main`
// export, marshalling `Values.Value`s in and out. Counterpart of
// `DynLoadExt::executeFunction` for the C/dlopen target — far simpler here
// because the calling convention is just scalar wasm params/results plus the
// `.wasm.sig` sidecar, with no MMC heap to build.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use anyhow::{Result, anyhow, bail};
use arcstr::ArcStr;
use metamodelica::List;

use openmodelica_frontend_types::Values;

use super::SigTy;

/// wasmtime errors carry their own (re-exported) `anyhow`, which does not unify
/// with ours under the feature set we build with; flatten via the message.
fn wt<T>(r: std::result::Result<T, wasmtime::Error>) -> Result<T> {
    r.map_err(|e| anyhow!("{e:?}"))
}

/// The static linear-memory runtime, precompiled from
/// `openmodelica_codegen_wasm_jit_runtime` to `wasm32-unknown-unknown` (rebuild
/// with that crate's `build-runtime.sh`). Instantiated alongside every generated
/// module, which imports its `memory` and `rt_*` exports — so the allocator,
/// reference counting and string ops are shared precompiled code, not re-emitted
/// per module.
pub(super) static RUNTIME_WASM: &[u8] = include_bytes!("../runtime.wasm");

/// Process-wide JIT cache shared across all `load_and_execute` calls.
///
/// Building a `wasmtime::Engine` and (especially) compiling a module with
/// Cranelift are the expensive steps; both are independent of the call
/// arguments, so we do them once and reuse the results. Only the `Store` and
/// `Instance` are rebuilt per call — they are cheap and a fresh instance keeps
/// each evaluation isolated (no leftover globals/linear memory between calls).
///
/// Compiled modules are keyed by their wasm *bytes*, not by file name: a
/// function redefined in an interactive session is re-translated to different
/// bytes and so compiles afresh, while repeated evaluation of the same function
/// hits the cache.
struct JitCache {
    engine: wasmtime::Engine,
    /// Host-imported math builtins (module `"env"`); identical for every module,
    /// so built once and reused for every instantiation.
    env_linker: wasmtime::Linker<()>,
    /// The static linear-memory runtime ([`RUNTIME_WASM`]), compiled once. A
    /// fresh instance is created per call to give each evaluation its own heap.
    runtime_module: wasmtime::Module,
    modules: Mutex<HashMap<Vec<u8>, wasmtime::Module>>,
}

fn jit_cache() -> &'static JitCache {
    static CACHE: OnceLock<JitCache> = OnceLock::new();
    CACHE.get_or_init(|| {
        let engine = wasmtime::Engine::default();
        let mut env_linker = wasmtime::Linker::new(&engine);
        // The builtin set is fixed and cannot collide; a failure here is a
        // programming error in `add_host_builtins`, not a runtime condition.
        add_host_builtins(&mut env_linker).expect("register wasm-jit host builtins");
        let runtime_module = wasmtime::Module::new(&engine, RUNTIME_WASM).expect("compile wasm-jit runtime");
        JitCache { engine, env_linker, runtime_module, modules: Mutex::new(HashMap::new()) }
    })
}

/// Return the compiled module for `bytes`, compiling and caching it on a miss.
///
/// Compilation runs outside the lock so that concurrent compiles of *different*
/// modules do not serialise; if two threads race to compile the *same* module
/// the first insert wins and the duplicate result is discarded (both are
/// equivalent).
fn get_or_compile_module(cache: &JitCache, bytes: &[u8]) -> Result<wasmtime::Module> {
    if let Some(module) = cache.modules.lock().unwrap().get(bytes) {
        return Ok(module.clone());
    }
    let module = wt(wasmtime::Module::new(&cache.engine, bytes))?;
    Ok(cache
        .modules
        .lock()
        .unwrap()
        .entry(bytes.to_vec())
        .or_insert(module)
        .clone())
}

/// Parsed `.wasm.sig` sidecar: scalar types of the main function's inputs and
/// outputs.
struct Sig {
    inputs: Vec<SigTy>,
    outputs: Vec<SigTy>,
}

fn read_sig(path: &str) -> Result<Sig> {
    let text = std::fs::read_to_string(path)?;
    let mut lines = text.lines();
    let parse = |line: Option<&str>| -> Result<Vec<SigTy>> {
        super::parse_sig_types(line.unwrap_or(""))
    };
    let inputs = parse(lines.next())?;
    let outputs = parse(lines.next())?;
    Ok(Sig { inputs, outputs })
}

/// Register the host-imported math builtins (module `"env"`), matching
/// `super::BUILTINS` one-for-one.
fn add_host_builtins(linker: &mut wasmtime::Linker<()>) -> Result<()> {
    macro_rules! f1 {
        ($name:literal, $f:expr) => {
            wt(linker.func_wrap("env", $name, |x: f64| -> f64 { ($f)(x) }))?;
        };
    }
    macro_rules! f2 {
        ($name:literal, $f:expr) => {
            wt(linker.func_wrap("env", $name, |x: f64, y: f64| -> f64 { ($f)(x, y) }))?;
        };
    }
    f2!("pow", f64::powf);
    f2!("atan2", f64::atan2);
    f1!("sin", f64::sin);
    f1!("cos", f64::cos);
    f1!("tan", f64::tan);
    f1!("asin", f64::asin);
    f1!("acos", f64::acos);
    f1!("atan", f64::atan);
    f1!("sinh", f64::sinh);
    f1!("cosh", f64::cosh);
    f1!("tanh", f64::tanh);
    f1!("exp", f64::exp);
    f1!("log", f64::ln);
    f1!("log10", f64::log10);
    Ok(())
}

/// Extract a numeric argument as an `f64`, accepting any scalar `Values.Value`.
fn value_as_f64(v: &Values::Value) -> Result<f64> {
    Ok(match v {
        Values::Value::REAL { real } => real.into_inner(),
        Values::Value::INTEGER { integer } => *integer as f64,
        Values::Value::BOOL { boolean } => *boolean as i64 as f64,
        Values::Value::ENUM_LITERAL { index, .. } => *index as f64,
        other => bail!("CodegenWasmJit: cannot pass {other:?} to a wasm function"),
    })
}

/// Extract a numeric argument as an `i32`, accepting any scalar `Values.Value`.
fn value_as_i32(v: &Values::Value) -> Result<i32> {
    Ok(match v {
        Values::Value::INTEGER { integer } => *integer,
        Values::Value::BOOL { boolean } => *boolean as i32,
        Values::Value::ENUM_LITERAL { index, .. } => *index,
        Values::Value::REAL { real } => real.into_inner() as i32,
        other => bail!("CodegenWasmJit: cannot pass {other:?} to a wasm function"),
    })
}

pub(super) fn load_and_execute(
    file_name: &str,
    _name: &str,
    args: &Arc<List<Arc<Values::Value>>>,
) -> Result<Arc<Values::Value>> {
    let wasm_path = format!("{file_name}.wasm");
    let sig = read_sig(&format!("{file_name}.wasm.sig"))?;
    let bytes = std::fs::read(&wasm_path)?;

    // Reuse the shared engine/env-linker and the per-content compiled module.
    // Each call gets a fresh runtime instance (its own heap/linear memory); the
    // generated module imports the runtime's `memory` and `rt_*` exports under
    // module name "rt", plus the `env` math builtins.
    let cache = jit_cache();
    let module = get_or_compile_module(cache, &bytes)?;
    let mut store = wasmtime::Store::new(&cache.engine, ());
    let rt_inst = wt(cache.env_linker.instantiate(&mut store, &cache.runtime_module))?;
    let mut linker = cache.env_linker.clone();
    wt(linker.instance(&mut store, "rt", rt_inst))?;
    let instance = wt(linker.instantiate(&mut store, &module))?;

    // Runtime entry points needed to marshal heap values (strings, arrays) in
    // and out of the shared heap.
    let memory = rt_inst
        .get_memory(&mut store, "memory")
        .ok_or_else(|| anyhow!("CodegenWasmJit: runtime has no `memory` export"))?;
    let rt = RtFns {
        mem: memory,
        str_new: wt(rt_inst.get_typed_func(&mut store, "rt_str_new"))?,
        str_len: wt(rt_inst.get_typed_func(&mut store, "rt_str_len"))?,
        str_data: wt(rt_inst.get_typed_func(&mut store, "rt_str_data"))?,
        arr_new: wt(rt_inst.get_typed_func(&mut store, "rt_array_new"))?,
        arr_set_dim: wt(rt_inst.get_typed_func(&mut store, "rt_array_set_dim"))?,
        arr_ndims: wt(rt_inst.get_typed_func(&mut store, "rt_array_ndims"))?,
        arr_total: wt(rt_inst.get_typed_func(&mut store, "rt_array_total"))?,
        arr_dim: wt(rt_inst.get_typed_func(&mut store, "rt_array_dim"))?,
        arr_elem_ptr: wt(rt_inst.get_typed_func(&mut store, "rt_array_elem_ptr"))?,
    };

    let func = instance
        .get_func(&mut store, "main")
        .ok_or_else(|| anyhow!("CodegenWasmJit: module has no `main` export"))?;

    // Marshal the arguments according to the input signature.
    let argv: Vec<&Arc<Values::Value>> = (&**args).into_iter().collect();
    if argv.len() != sig.inputs.len() {
        bail!("CodegenWasmJit: function expects {} arguments, got {}", sig.inputs.len(), argv.len());
    }
    let mut params: Vec<wasmtime::Val> = Vec::with_capacity(argv.len());
    for (a, ty) in argv.iter().zip(sig.inputs.iter()) {
        params.push(marshal_in(&mut store, &rt, ty, a)?);
    }

    // Result buffer sized to the function's declared results.
    let n_results = func.ty(&store).results().len();
    let mut results = vec![wasmtime::Val::I32(0); n_results];
    wt(func.call(&mut store, &params, &mut results))?;

    if results.len() != sig.outputs.len() {
        bail!("CodegenWasmJit: wasm returned {} values but signature has {}", results.len(), sig.outputs.len());
    }

    let mut out: Vec<Arc<Values::Value>> = Vec::with_capacity(results.len());
    for (val, ty) in results.iter().zip(sig.outputs.iter()) {
        out.push(Arc::new(marshal_out(&mut store, &rt, ty, val)?));
    }

    Ok(match out.len() {
        0 => Arc::new(Values::Value::NORETCALL),
        1 => out.pop().unwrap(),
        _ => Arc::new(Values::Value::TUPLE { valueLst: Arc::new(List::from_iter(out)) }),
    })
}

/// The runtime entry points (and shared memory) the host needs to build/read
/// heap values. `wasmtime::TypedFunc` and `Memory` are cheap handles, so this is
/// passed by reference alongside `&mut Store`.
struct RtFns {
    mem: wasmtime::Memory,
    str_new: wasmtime::TypedFunc<i32, i32>,
    str_len: wasmtime::TypedFunc<i32, i32>,
    str_data: wasmtime::TypedFunc<i32, i32>,
    arr_new: wasmtime::TypedFunc<(i32, i32, i32), i32>,
    arr_set_dim: wasmtime::TypedFunc<(i32, i32, i32), ()>,
    arr_ndims: wasmtime::TypedFunc<i32, i32>,
    arr_total: wasmtime::TypedFunc<i32, i32>,
    arr_dim: wasmtime::TypedFunc<(i32, i32), i32>,
    arr_elem_ptr: wasmtime::TypedFunc<(i32, i32), i32>,
}

type Store = wasmtime::Store<()>;

/// Build a `wasmtime::Val` (scalar, or an `i32` handle into the heap) for the
/// argument value `v` of the given Modelica type. Heap values (strings, arrays)
/// are materialized in the runtime heap; the generated callee owns/consumes
/// them, so the host does not release them afterwards.
fn marshal_in(store: &mut Store, rt: &RtFns, ty: &SigTy, v: &Values::Value) -> Result<wasmtime::Val> {
    Ok(match ty {
        SigTy::Real => wasmtime::Val::F64(value_as_f64(v)?.to_bits()),
        SigTy::Int | SigTy::Bool => wasmtime::Val::I32(value_as_i32(v)?),
        SigTy::Str => wasmtime::Val::I32(str_to_handle(store, rt, v)?),
        SigTy::Array { elem, rank } => wasmtime::Val::I32(array_to_handle(store, rt, elem, *rank, v)?),
    })
}

/// Materialize a `Values.STRING` into a fresh runtime string, returning its handle.
fn str_to_handle(store: &mut Store, rt: &RtFns, v: &Values::Value) -> Result<i32> {
    let Values::Value::STRING { string } = v else {
        bail!("CodegenWasmJit: expected a String argument, got {v:?}");
    };
    let b = string.as_bytes();
    let h = wt(rt.str_new.call(&mut *store, b.len() as i32))?;
    let d = wt(rt.str_data.call(&mut *store, h))? as usize;
    rt.mem.write(&mut *store, d, b).map_err(|e| anyhow!("CodegenWasmJit: memory write: {e}"))?;
    Ok(h)
}

/// Materialize a (nested) `Values.ARRAY` into a flat row-major runtime array of
/// `elem`/`rank`, returning its handle. The `Values.ARRAY` nests one level per
/// dimension; the leaves are flattened row-major and written into the object.
fn array_to_handle(store: &mut Store, rt: &RtFns, elem: &SigTy, rank: u32, v: &Values::Value) -> Result<i32> {
    let Values::Value::ARRAY { dimLst, .. } = v else {
        bail!("CodegenWasmJit: expected an array argument, got {v:?}");
    };
    let dims: Vec<i32> = (&**dimLst).into_iter().copied().collect();
    if dims.len() as u32 != rank {
        bail!("CodegenWasmJit: array argument has {} dimensions, expected rank {rank}", dims.len());
    }
    let total: i32 = dims.iter().product();
    let obj = wt(rt.arr_new.call(&mut *store, (elem.elem_kind() as i32, rank as i32, total)))?;
    for (axis, d) in dims.iter().enumerate() {
        wt(rt.arr_set_dim.call(&mut *store, (obj, axis as i32, *d)))?;
    }
    // Flatten the nested value into row-major scalar leaves and write each.
    let mut leaves = Vec::new();
    flatten_values(v, &mut leaves);
    if leaves.len() as i32 != total {
        bail!("CodegenWasmJit: array argument has {} elements but dimensions imply {total}", leaves.len());
    }
    for (k, leaf) in leaves.iter().enumerate() {
        let addr = wt(rt.arr_elem_ptr.call(&mut *store, (obj, k as i32 + 1)))? as usize;
        write_elem(store, rt, elem, addr, leaf)?;
    }
    Ok(obj)
}

/// Collect the scalar leaf values of a (possibly nested) `Values.ARRAY` in
/// row-major order.
fn flatten_values<'a>(v: &'a Values::Value, out: &mut Vec<&'a Values::Value>) {
    match v {
        Values::Value::ARRAY { valueLst, .. } => {
            for e in &**valueLst {
                flatten_values(e, out);
            }
        }
        other => out.push(other),
    }
}

/// Write one array element of type `elem` at byte address `addr`.
fn write_elem(store: &mut Store, rt: &RtFns, elem: &SigTy, addr: usize, v: &Values::Value) -> Result<()> {
    match elem {
        SigTy::Real => write_bytes(store, rt, addr, &value_as_f64(v)?.to_le_bytes())?,
        SigTy::Int | SigTy::Bool => write_bytes(store, rt, addr, &value_as_i32(v)?.to_le_bytes())?,
        SigTy::Str => {
            let h = str_to_handle(store, rt, v)?;
            write_bytes(store, rt, addr, &h.to_le_bytes())?;
        }
        SigTy::Array { elem, rank } => {
            let h = array_to_handle(store, rt, elem, *rank, v)?;
            write_bytes(store, rt, addr, &h.to_le_bytes())?;
        }
    }
    Ok(())
}

fn write_bytes(store: &mut Store, rt: &RtFns, addr: usize, bytes: &[u8]) -> Result<()> {
    rt.mem.write(&mut *store, addr, bytes).map_err(|e| anyhow!("CodegenWasmJit: memory write: {e}"))
}

/// Build a `Values.Value` from a wasm result of the given Modelica type.
fn marshal_out(store: &mut Store, rt: &RtFns, ty: &SigTy, val: &wasmtime::Val) -> Result<Values::Value> {
    Ok(match ty {
        SigTy::Int => Values::Value::INTEGER {
            integer: val.i32().ok_or_else(|| anyhow!("CodegenWasmJit: expected i32 result"))?,
        },
        SigTy::Bool => Values::Value::BOOL {
            boolean: val.i32().ok_or_else(|| anyhow!("CodegenWasmJit: expected i32 result"))? != 0,
        },
        SigTy::Real => Values::Value::REAL {
            real: metamodelica::Real::from(val.f64().ok_or_else(|| anyhow!("CodegenWasmJit: expected f64 result"))?),
        },
        SigTy::Str => {
            let h = val.i32().ok_or_else(|| anyhow!("CodegenWasmJit: expected i32 string handle result"))?;
            Values::Value::STRING { string: ArcStr::from(read_string(store, rt, h)?.as_str()) }
        }
        SigTy::Array { elem, .. } => {
            let h = val.i32().ok_or_else(|| anyhow!("CodegenWasmJit: expected i32 array handle result"))?;
            read_array(store, rt, elem, h)?
        }
    })
}

/// Read a runtime string handle's bytes into a `String`.
fn read_string(store: &mut Store, rt: &RtFns, h: i32) -> Result<String> {
    let len = wt(rt.str_len.call(&mut *store, h))? as usize;
    let d = wt(rt.str_data.call(&mut *store, h))? as usize;
    let mut buf = vec![0u8; len];
    rt.mem.read(&*store, d, &mut buf).map_err(|e| anyhow!("CodegenWasmJit: memory read: {e}"))?;
    String::from_utf8(buf).map_err(|e| anyhow!("CodegenWasmJit: non-utf8 result string: {e}"))
}

/// Read a runtime array handle into a (nested) `Values.ARRAY` of element type
/// `elem`. The flat row-major elements are folded into the nested
/// dimension-by-dimension structure the rest of the compiler uses.
fn read_array(store: &mut Store, rt: &RtFns, elem: &SigTy, h: i32) -> Result<Values::Value> {
    let ndims = wt(rt.arr_ndims.call(&mut *store, h))?;
    let total = wt(rt.arr_total.call(&mut *store, h))?;
    let mut dims = Vec::with_capacity(ndims as usize);
    for axis in 1..=ndims {
        dims.push(wt(rt.arr_dim.call(&mut *store, (h, axis)))?);
    }
    // Read every scalar leaf row-major.
    let mut flat = Vec::with_capacity(total as usize);
    for k in 0..total {
        let addr = wt(rt.arr_elem_ptr.call(&mut *store, (h, k + 1)))? as usize;
        flat.push(read_elem(store, rt, elem, addr)?);
    }
    Ok(nest_values(&dims, &flat))
}

/// Read one array element of type `elem` at byte address `addr`.
fn read_elem(store: &mut Store, rt: &RtFns, elem: &SigTy, addr: usize) -> Result<Values::Value> {
    Ok(match elem {
        SigTy::Real => Values::Value::REAL { real: metamodelica::Real::from(f64::from_le_bytes(read_bytes::<8>(store, rt, addr)?)) },
        SigTy::Int => Values::Value::INTEGER { integer: i32::from_le_bytes(read_bytes::<4>(store, rt, addr)?) },
        SigTy::Bool => Values::Value::BOOL { boolean: i32::from_le_bytes(read_bytes::<4>(store, rt, addr)?) != 0 },
        SigTy::Str => {
            let h = i32::from_le_bytes(read_bytes::<4>(store, rt, addr)?);
            Values::Value::STRING { string: ArcStr::from(read_string(store, rt, h)?.as_str()) }
        }
        SigTy::Array { elem, .. } => {
            let h = i32::from_le_bytes(read_bytes::<4>(store, rt, addr)?);
            read_array(store, rt, elem, h)?
        }
    })
}

fn read_bytes<const N: usize>(store: &mut Store, rt: &RtFns, addr: usize) -> Result<[u8; N]> {
    let mut buf = [0u8; N];
    rt.mem.read(&*store, addr, &mut buf).map_err(|e| anyhow!("CodegenWasmJit: memory read: {e}"))?;
    Ok(buf)
}

// `elem_kind` reuses `super::SigTy::elem_kind` so the host and codegen share one
// definition of the element-kind tags.

/// Fold a flat row-major element list into the nested `Values.ARRAY` structure:
/// one nesting level per dimension, each level's `dimLst` being the dimensions
/// at and below it.
fn nest_values(dims: &[i32], flat: &[Values::Value]) -> Values::Value {
    let d = dims[0];
    let values: Vec<Arc<Values::Value>> = if dims.len() == 1 {
        flat.iter().cloned().map(Arc::new).collect()
    } else {
        let chunk = flat.len() / d.max(1) as usize;
        (0..d as usize)
            .map(|i| Arc::new(nest_values(&dims[1..], &flat[i * chunk..(i + 1) * chunk])))
            .collect()
    };
    Values::Value::ARRAY {
        valueLst: Arc::new(List::from_iter(values)),
        dimLst: Arc::new(List::from_iter(dims.iter().copied())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_encoder as we;

    /// Read the bytes of a runtime string handle out of an instance's memory.
    fn read_rt_string(
        store: &mut wasmtime::Store<()>,
        inst: &wasmtime::Instance,
        mem: wasmtime::Memory,
        handle: i32,
    ) -> String {
        let len = inst.get_typed_func::<i32, i32>(&mut *store, "rt_str_len").unwrap().call(&mut *store, handle).unwrap();
        let data = inst.get_typed_func::<i32, i32>(&mut *store, "rt_str_data").unwrap().call(&mut *store, handle).unwrap();
        let mut buf = vec![0u8; len as usize];
        mem.read(&*store, data as usize, &mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }

    /// The precompiled runtime instantiates and its `rt_*` string ABI works,
    /// including `rt_real_string` matching the canonical `metamodelica::realString`
    /// byte-for-byte (so `String(Real)` stays identical to the C target).
    #[test]
    fn precompiled_runtime_string_abi() {
        let engine = wasmtime::Engine::default();
        let module = wasmtime::Module::new(&engine, RUNTIME_WASM).unwrap();
        let mut store = wasmtime::Store::new(&engine, ());
        let inst = wasmtime::Linker::new(&engine).instantiate(&mut store, &module).unwrap();
        let mem = inst.get_memory(&mut store, "memory").unwrap();

        let int_string = inst.get_typed_func::<i32, i32>(&mut store, "rt_int_string").unwrap();
        let real_string = inst.get_typed_func::<f64, i32>(&mut store, "rt_real_string").unwrap();
        let bool_string = inst.get_typed_func::<i32, i32>(&mut store, "rt_bool_string").unwrap();
        let concat = inst.get_typed_func::<(i32, i32), i32>(&mut store, "rt_concat").unwrap();
        let streq = inst.get_typed_func::<(i32, i32), i32>(&mut store, "rt_streq").unwrap();
        let substring = inst.get_typed_func::<(i32, i32, i32), i32>(&mut store, "rt_substring").unwrap();
        let retain = inst.get_typed_func::<i32, ()>(&mut store, "rt_retain").unwrap();
        let release = inst.get_typed_func::<i32, ()>(&mut store, "rt_release").unwrap();

        let h42 = int_string.call(&mut store, 42).unwrap();
        assert_eq!(read_rt_string(&mut store, &inst, mem, h42), "42");
        let htrue = bool_string.call(&mut store, 1).unwrap();
        assert_eq!(read_rt_string(&mut store, &inst, mem, htrue), "true");

        // Concatenation: "12" + "3" via int formatting.
        let a = int_string.call(&mut store, 12).unwrap();
        let b = int_string.call(&mut store, 3).unwrap();
        let ab = concat.call(&mut store, (a, b)).unwrap();
        assert_eq!(read_rt_string(&mut store, &inst, mem, ab), "123");

        // substring("123", 2, 3) = "23".
        let sub = substring.call(&mut store, (ab, 2, 3)).unwrap();
        assert_eq!(read_rt_string(&mut store, &inst, mem, sub), "23");

        // Equality.
        let a2 = int_string.call(&mut store, 12).unwrap();
        assert_eq!(streq.call(&mut store, (a, a2)).unwrap(), 1);
        assert_eq!(streq.call(&mut store, (a, b)).unwrap(), 0);

        // realString must match the canonical formatter for a spread of values.
        for v in [0.0, 1.5, -2.0, 1.0 / 3.0, 1e-7, 1234567.0, 6.022e23, std::f64::consts::PI] {
            let h = real_string.call(&mut store, v).unwrap();
            let got = read_rt_string(&mut store, &inst, mem, h);
            let want = metamodelica::realString(metamodelica::Real::from(v)).to_string();
            assert_eq!(got, want, "realString({v})");
        }

        // Refcount: retain then two releases frees without trapping; the freed
        // slot is reused by the next allocation (allocator actually frees).
        let r = int_string.call(&mut store, 7).unwrap();
        retain.call(&mut store, r).unwrap();
        release.call(&mut store, r).unwrap();
        release.call(&mut store, r).unwrap();
    }

    /// The N-D array ABI: header bookkeeping (ndims/total/dims), row-major
    /// element addressing, and `rt_array_release` freeing nested heap (String)
    /// elements without trapping.
    #[test]
    fn precompiled_runtime_array_abi() {
        let engine = wasmtime::Engine::default();
        let module = wasmtime::Module::new(&engine, RUNTIME_WASM).unwrap();
        let mut store = wasmtime::Store::new(&engine, ());
        let inst = wasmtime::Linker::new(&engine).instantiate(&mut store, &module).unwrap();
        let mem = inst.get_memory(&mut store, "memory").unwrap();

        let arr_new = inst.get_typed_func::<(i32, i32, i32), i32>(&mut store, "rt_array_new").unwrap();
        let set_dim = inst.get_typed_func::<(i32, i32, i32), ()>(&mut store, "rt_array_set_dim").unwrap();
        let ndims = inst.get_typed_func::<i32, i32>(&mut store, "rt_array_ndims").unwrap();
        let total = inst.get_typed_func::<i32, i32>(&mut store, "rt_array_total").unwrap();
        let dim = inst.get_typed_func::<(i32, i32), i32>(&mut store, "rt_array_dim").unwrap();
        let elem_ptr = inst.get_typed_func::<(i32, i32), i32>(&mut store, "rt_array_elem_ptr").unwrap();
        let arr_release = inst.get_typed_func::<i32, ()>(&mut store, "rt_array_release").unwrap();
        let int_string = inst.get_typed_func::<i32, i32>(&mut store, "rt_int_string").unwrap();

        // A 2x3 Real array (elem_kind 1). Header reports rank/total/dims.
        let a = arr_new.call(&mut store, (1, 2, 6)).unwrap();
        set_dim.call(&mut store, (a, 0, 2)).unwrap();
        set_dim.call(&mut store, (a, 1, 3)).unwrap();
        assert_eq!(ndims.call(&mut store, a).unwrap(), 2);
        assert_eq!(total.call(&mut store, a).unwrap(), 6);
        assert_eq!(dim.call(&mut store, (a, 1)).unwrap(), 2);
        assert_eq!(dim.call(&mut store, (a, 2)).unwrap(), 3);

        // Round-trip f64 elements through the row-major linear addresses.
        for k in 1..=6i32 {
            let addr = elem_ptr.call(&mut store, (a, k)).unwrap() as usize;
            mem.write(&mut store, addr, &(k as f64 * 1.5).to_le_bytes()).unwrap();
        }
        for k in 1..=6i32 {
            let addr = elem_ptr.call(&mut store, (a, k)).unwrap() as usize;
            let mut buf = [0u8; 8];
            mem.read(&store, addr, &mut buf).unwrap();
            assert_eq!(f64::from_le_bytes(buf), k as f64 * 1.5);
        }
        arr_release.call(&mut store, a).unwrap();

        // An array of two Strings (elem_kind 3): releasing the array must release
        // the contained string handles (no trap, double release is the symptom).
        let s = arr_new.call(&mut store, (3, 1, 2)).unwrap();
        set_dim.call(&mut store, (s, 0, 2)).unwrap();
        for (k, n) in [(1, 11), (2, 22)] {
            let h = int_string.call(&mut store, n).unwrap();
            let addr = elem_ptr.call(&mut store, (s, k)).unwrap() as usize;
            mem.write(&mut store, addr, &h.to_le_bytes()).unwrap();
        }
        arr_release.call(&mut store, s).unwrap();

        // rt_array_copy gives independent storage (value semantics): mutating the
        // copy must not change the original.
        let arr_copy = inst.get_typed_func::<i32, i32>(&mut store, "rt_array_copy").unwrap();
        let orig = arr_new.call(&mut store, (1, 1, 3)).unwrap();
        set_dim.call(&mut store, (orig, 0, 3)).unwrap();
        for k in 1..=3i32 {
            let addr = elem_ptr.call(&mut store, (orig, k)).unwrap() as usize;
            mem.write(&mut store, addr, &(k as f64).to_le_bytes()).unwrap();
        }
        let dup = arr_copy.call(&mut store, orig).unwrap();
        assert_ne!(dup, orig);
        // Mutate the copy's first element.
        let addr = elem_ptr.call(&mut store, (dup, 1)).unwrap() as usize;
        mem.write(&mut store, addr, &(99.0f64).to_le_bytes()).unwrap();
        // Original is unchanged.
        let addr = elem_ptr.call(&mut store, (orig, 1)).unwrap() as usize;
        let mut buf = [0u8; 8];
        mem.read(&store, addr, &mut buf).unwrap();
        assert_eq!(f64::from_le_bytes(buf), 1.0);
        arr_release.call(&mut store, dup).unwrap();
        arr_release.call(&mut store, orig).unwrap();
    }

    /// `rt_real_format` (`String(Real, significantDigits, minimumLength,
    /// leftJustified)`) must match the canonical `printf`-`%g` formatter that
    /// `Ceval.cevalBuiltinString` / the C target use, and `rt_str_pad`
    /// (`String(Integer/Boolean/String, minimumLength, leftJustified)`) must
    /// match `ExpressionSimplify.cevalBuiltinStringFormat`'s space-padding.
    #[test]
    fn precompiled_runtime_real_format_and_pad_abi() {
        use openmodelica_util::System;

        let engine = wasmtime::Engine::default();
        let module = wasmtime::Module::new(&engine, RUNTIME_WASM).unwrap();
        let mut store = wasmtime::Store::new(&engine, ());
        let inst = wasmtime::Linker::new(&engine).instantiate(&mut store, &module).unwrap();
        let mem = inst.get_memory(&mut store, "memory").unwrap();

        let real_format =
            inst.get_typed_func::<(f64, i32, i32, i32), i32>(&mut store, "rt_real_format").unwrap();
        let str_new = inst.get_typed_func::<i32, i32>(&mut store, "rt_str_new").unwrap();
        let str_data = inst.get_typed_func::<i32, i32>(&mut store, "rt_str_data").unwrap();
        let str_pad = inst.get_typed_func::<(i32, i32, i32), i32>(&mut store, "rt_str_pad").unwrap();

        // String(Real, sig, minlen, leftjust): mirror Ceval's format string
        // `"%[-]{minlen}.{sig}g"` evaluated with `System.snprintff` (the canonical
        // C-printf port) and compare byte-for-byte.
        let values = [0.0, 1.5, -2.0, 1.0 / 3.0, 1e-7, 1234567.0, 6.022e23, std::f64::consts::PI, -0.000123456];
        for &v in &values {
            for &(sig, minlen, leftjust) in &[(6, 0, 1), (3, 0, 1), (6, 12, 1), (6, 12, 0), (2, 0, 1)] {
                let h = real_format.call(&mut store, (v, sig, minlen, leftjust)).unwrap();
                let got = read_rt_string(&mut store, &inst, mem, h);
                let dash = if leftjust != 0 { "-" } else { "" };
                let fmt = format!("%{dash}{minlen}.{sig}g");
                let want = System::snprintff(
                    arcstr::ArcStr::from(fmt),
                    minlen + 20,
                    metamodelica::Real::from(v),
                )
                .unwrap()
                .to_string();
                assert_eq!(got, want, "String({v}, sig={sig}, minlen={minlen}, leftjust={leftjust})");
            }
        }

        // rt_str_pad: write a string into a fresh runtime object, then pad.
        let make = |store: &mut wasmtime::Store<()>, s: &str| -> i32 {
            let h = str_new.call(&mut *store, s.len() as i32).unwrap();
            let d = str_data.call(&mut *store, h).unwrap() as usize;
            mem.write(&mut *store, d, s.as_bytes()).unwrap();
            h
        };
        // Shorter than minlen: pad with spaces (trailing when leftjust, else leading).
        let h = make(&mut store, "hi");
        let p = str_pad.call(&mut store, (h, 6, 1)).unwrap();
        assert_eq!(read_rt_string(&mut store, &inst, mem, p), "hi    ");
        let h = make(&mut store, "hi");
        let p = str_pad.call(&mut store, (h, 6, 0)).unwrap();
        assert_eq!(read_rt_string(&mut store, &inst, mem, p), "    hi");
        // Already at least minlen: returned unchanged.
        let h = make(&mut store, "already long");
        let p = str_pad.call(&mut store, (h, 4, 0)).unwrap();
        assert_eq!(read_rt_string(&mut store, &inst, mem, p), "already long");
    }

    /// Encode a one-function module exporting `main` with the given signature
    /// and body, write it plus its sidecar under a temp basename, and return
    /// the basename for `load_and_execute`.
    fn emit(base: &str, params: &[we::ValType], results: &[we::ValType], sig: &str, body: &[we::Instruction]) -> String {
        let mut m = we::Module::new();
        let mut types = we::TypeSection::new();
        types.ty().function(params.iter().copied(), results.iter().copied());
        m.section(&types);
        let mut funcs = we::FunctionSection::new();
        funcs.function(0);
        m.section(&funcs);
        let mut exports = we::ExportSection::new();
        exports.export("main", we::ExportKind::Func, 0);
        m.section(&exports);
        let mut code = we::CodeSection::new();
        let mut f = we::Function::new([]);
        for i in body {
            f.instruction(i);
        }
        code.function(&f);
        m.section(&code);
        let path = std::env::temp_dir().join(base);
        let path = path.to_str().unwrap().to_string();
        std::fs::write(format!("{path}.wasm"), m.finish()).unwrap();
        std::fs::write(format!("{path}.wasm.sig"), sig).unwrap();
        path
    }

    fn ival(v: &Values::Value) -> i32 {
        match v {
            Values::Value::INTEGER { integer } => *integer,
            other => panic!("expected INTEGER, got {other:?}"),
        }
    }
    fn rval(v: &Values::Value) -> f64 {
        match v {
            Values::Value::REAL { real } => real.into_inner(),
            other => panic!("expected REAL, got {other:?}"),
        }
    }

    #[test]
    fn integer_add() {
        let base = emit(
            "wjit_iadd",
            &[we::ValType::I32, we::ValType::I32],
            &[we::ValType::I32],
            "II\nI\n",
            &[we::Instruction::LocalGet(0), we::Instruction::LocalGet(1), we::Instruction::I32Add, we::Instruction::End],
        );
        let args = Arc::new(List::from_iter([
            Arc::new(Values::Value::INTEGER { integer: 3 }),
            Arc::new(Values::Value::INTEGER { integer: 4 }),
        ]));
        let r = load_and_execute(&base, "main", &args).unwrap();
        assert_eq!(ival(&r), 7);
    }

    #[test]
    fn real_scale() {
        // main(x) = x * 2.0
        let base = emit(
            "wjit_rscale",
            &[we::ValType::F64],
            &[we::ValType::F64],
            "R\nR\n",
            &[
                we::Instruction::LocalGet(0),
                we::Instruction::F64Const(2.0.into()),
                we::Instruction::F64Mul,
                we::Instruction::End,
            ],
        );
        let args = Arc::new(List::from_iter([Arc::new(Values::Value::REAL { real: metamodelica::Real::from(21.0) })]));
        let r = load_and_execute(&base, "main", &args).unwrap();
        assert_eq!(rval(&r), 42.0);
    }

    #[test]
    fn multi_output_tuple() {
        // main(x) = (x, x+1)
        let base = emit(
            "wjit_tuple",
            &[we::ValType::I32],
            &[we::ValType::I32, we::ValType::I32],
            "I\nII\n",
            &[
                we::Instruction::LocalGet(0),
                we::Instruction::LocalGet(0),
                we::Instruction::I32Const(1),
                we::Instruction::I32Add,
                we::Instruction::End,
            ],
        );
        let args = Arc::new(List::from_iter([Arc::new(Values::Value::INTEGER { integer: 41 })]));
        let r = load_and_execute(&base, "main", &args).unwrap();
        match &*r {
            Values::Value::TUPLE { valueLst } => {
                let v: Vec<_> = (&**valueLst).into_iter().collect();
                assert_eq!(v.len(), 2);
                assert_eq!(ival(&v[0]), 41);
                assert_eq!(ival(&v[1]), 42);
            }
            other => panic!("expected TUPLE, got {other:?}"),
        }
    }

    #[test]
    fn cache_reuse_and_redefinition() {
        // First definition of `main(a,b) = a + b`. Running it twice must give
        // the same answer (the second call hits the compiled-module cache).
        let base = emit(
            "wjit_cache",
            &[we::ValType::I32, we::ValType::I32],
            &[we::ValType::I32],
            "II\nI\n",
            &[we::Instruction::LocalGet(0), we::Instruction::LocalGet(1), we::Instruction::I32Add, we::Instruction::End],
        );
        let args = Arc::new(List::from_iter([
            Arc::new(Values::Value::INTEGER { integer: 5 }),
            Arc::new(Values::Value::INTEGER { integer: 7 }),
        ]));
        assert_eq!(ival(&load_and_execute(&base, "main", &args).unwrap()), 12);
        assert_eq!(ival(&load_and_execute(&base, "main", &args).unwrap()), 12);

        // Redefine the same basename to `main(a,b) = a * b` (different bytes).
        // The cache is keyed by content, so this must recompile rather than
        // reuse the stale `+` module.
        emit(
            "wjit_cache",
            &[we::ValType::I32, we::ValType::I32],
            &[we::ValType::I32],
            "II\nI\n",
            &[we::Instruction::LocalGet(0), we::Instruction::LocalGet(1), we::Instruction::I32Mul, we::Instruction::End],
        );
        assert_eq!(ival(&load_and_execute(&base, "main", &args).unwrap()), 35);
    }

    #[test]
    fn host_builtin_sin() {
        // main(x) = sin(x), importing the host builtin "sin" (index 2 in env).
        let mut m = we::Module::new();
        let mut types = we::TypeSection::new();
        types.ty().function([we::ValType::F64], [we::ValType::F64]); // type 0: sin
        types.ty().function([we::ValType::F64], [we::ValType::F64]); // type 1: main
        m.section(&types);
        let mut imports = we::ImportSection::new();
        imports.import("env", "sin", we::EntityType::Function(0));
        m.section(&imports);
        let mut funcs = we::FunctionSection::new();
        funcs.function(1);
        m.section(&funcs);
        let mut exports = we::ExportSection::new();
        exports.export("main", we::ExportKind::Func, 1); // func 0 is the import
        m.section(&exports);
        let mut code = we::CodeSection::new();
        let mut f = we::Function::new([]);
        f.instruction(&we::Instruction::LocalGet(0));
        f.instruction(&we::Instruction::Call(0));
        f.instruction(&we::Instruction::End);
        code.function(&f);
        m.section(&code);
        let path = std::env::temp_dir().join("wjit_sin");
        let path = path.to_str().unwrap().to_string();
        std::fs::write(format!("{path}.wasm"), m.finish()).unwrap();
        std::fs::write(format!("{path}.wasm.sig"), "R\nR\n").unwrap();

        let args = Arc::new(List::from_iter([Arc::new(Values::Value::REAL {
            real: metamodelica::Real::from(std::f64::consts::FRAC_PI_2),
        })]));
        let r = load_and_execute(&path, "main", &args).unwrap();
        assert!((rval(&r) - 1.0).abs() < 1e-12);
    }
}
