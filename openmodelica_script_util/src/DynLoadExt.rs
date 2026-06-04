// Manually written file.
//
// `Values` marshalling for the `-d=gen` dynamic-load pipeline — the
// implementation behind `DynLoad.executeFunction`'s `external "C"
// DynLoad_executeFunction`.
//
// A `-d=gen` function is compiled to C with an entry point
//
//     int in_<name>(threadData_t *threadData,
//                   type_description *inArgs, type_description *outVar);
//
// `in_*` reads its arguments out of the `inArgs` array with the runtime's
// `read_modelica_*` helpers (each advances the cursor by one element) and writes
// its results into `outVar`, accumulating several outputs into a `TYPE_DESC_TUPLE`.
// `type_description` is a small tagged union; we build/read it directly here
// (mirroring the C struct layout) rather than going through the runtime's
// boxed-`Values` marshalling, so scalar calls need no GC allocation at all.
//
// `crate::dynload` (in `openmodelica_util`) owns the loaded libraries, resolves
// the `in_*` address and provides the initialised `threadData`. `mmtorust`
// routes `DynLoad_executeFunction` here via `external_c_impl_path`.

use std::ffi::c_void;
use std::sync::Arc;

use anyhow::{Result, bail};
use metamodelica::List;

use openmodelica_frontend_types::Values;
use openmodelica_util::dynload;

// `enum type_desc_e` tags from `openmodelica.h`.
const TD_NONE: i32 = 0;
const TD_REAL: i32 = 1;
const TD_INT: i32 = 3;
const TD_BOOL: i32 = 5;
const TD_STRING: i32 = 7;
const TD_TUPLE: i32 = 9;
const TD_MMC: i32 = 13;
const TD_NORETCALL: i32 = 14;

/// `struct type_desc_s` from `openmodelica.h` (40 bytes): an `enum` tag, a
/// 1-bit `retval` flag (its storage int), then an 8-byte-aligned union. The
/// union members we touch all start at offset 8; `d0..d3` cover its 32 bytes.
/// For a scalar the value lives in `d0`; for a tuple `d0` is the element count
/// (`size_t`) and `d1` the element pointer.
#[repr(C)]
#[derive(Clone, Copy)]
struct TypeDesc {
    tag: i32,
    retval: i32,
    d0: u64,
    d1: u64,
    d2: u64,
    d3: u64,
}

impl TypeDesc {
    const fn none() -> Self {
        TypeDesc { tag: TD_NONE, retval: 0, d0: 0, d1: 0, d2: 0, d3: 0 }
    }
    const fn scalar(tag: i32, d0: u64) -> Self {
        TypeDesc { tag, retval: 0, d0, d1: 0, d2: 0, d3: 0 }
    }
}

type InFn = extern "C" fn(*mut c_void, *mut TypeDesc, *mut TypeDesc) -> i32;

/// Marshal one argument `Value` into a `type_description`. The runtime's
/// `read_modelica_metatype` boxes the scalar tags on demand, so an `Integer`/
/// `Real`/`Boolean` argument needs no allocation whether the parameter is a
/// builtin scalar or a `MetaModelica` value.
fn value_to_desc(v: &Values::Value) -> Result<TypeDesc> {
    match v {
        Values::Value::INTEGER { integer } => Ok(TypeDesc::scalar(TD_INT, *integer as i64 as u64)),
        Values::Value::REAL { real } => Ok(TypeDesc::scalar(TD_REAL, real.into_inner().to_bits())),
        Values::Value::BOOL { boolean } => Ok(TypeDesc::scalar(TD_BOOL, *boolean as u64)),
        other => bail!("DynLoad.executeFunction: marshalling argument {other:?} not yet supported"),
    }
}

/// Read a `type_description` produced by `in_*` back into a `Value`. Multiple
/// function outputs arrive as a `TYPE_DESC_TUPLE` and become a `Values.TUPLE`.
fn desc_to_value(d: &TypeDesc) -> Result<Arc<Values::Value>> {
    match d.tag {
        TD_INT => Ok(Arc::new(Values::Value::INTEGER { integer: d.d0 as i64 as i32 })),
        TD_REAL => Ok(Arc::new(Values::Value::REAL { real: metamodelica::Real::from(f64::from_bits(d.d0)) })),
        TD_BOOL => Ok(Arc::new(Values::Value::BOOL { boolean: (d.d0 as i32) != 0 })),
        TD_NORETCALL => Ok(Arc::new(Values::Value::NORETCALL)),
        TD_TUPLE => {
            let n = d.d0 as usize;
            let elems = d.d1 as *const TypeDesc;
            if n != 0 && elems.is_null() {
                bail!("DynLoad.executeFunction: malformed result tuple");
            }
            let mut vals: Vec<Arc<Values::Value>> = Vec::with_capacity(n);
            for i in 0..n {
                let e = unsafe { &*elems.add(i) };
                vals.push(desc_to_value(e)?);
            }
            Ok(Arc::new(Values::Value::TUPLE { valueLst: Arc::new(List::from_iter(vals)) }))
        }
        TD_STRING => bail!("DynLoad.executeFunction: string results not yet supported"),
        TD_MMC => bail!("DynLoad.executeFunction: MetaModelica results not yet supported"),
        other => bail!("DynLoad.executeFunction: unsupported result type_description tag {other}"),
    }
}

/// Body of `DynLoad.executeFunction`: call the dynamically loaded `in_*` entry
/// point identified by `handle`, marshalling `values` in and the result out. A
/// non-zero return from `in_*` means the generated function failed (`MMC_THROW`);
/// the C runtime returns `Values.META_FAIL` for that, so we do too.
pub fn executeFunction(handle: i32, values: Arc<List<Arc<Values::Value>>>, _debug: bool) -> Result<Arc<Values::Value>> {
    let addr = dynload::function_addr(handle)?;
    let thread_data = dynload::thread_data()? as *mut c_void;

    let mut args: Vec<TypeDesc> = Vec::new();
    for v in &*values {
        args.push(value_to_desc(v)?);
    }

    let mut out = TypeDesc::none();
    out.retval = 1; // request owned (malloc'd) array/string results we can free

    // SAFETY: `addr` is the resolved `in_<name>` entry; its ABI is fixed by the
    // code generator. `args` outlives the call; `in_*` reads at most one element
    // per declared input.
    let func: InFn = unsafe { std::mem::transmute(addr) };
    let rc = func(thread_data, args.as_mut_ptr(), &mut out);
    if rc != 0 {
        return Ok(Arc::new(Values::Value::META_FAIL));
    }

    let result = desc_to_value(&out);

    // Release any heap the runtime allocated for the result (the tuple element
    // array, owned arrays/strings). Scalars own nothing, so this is a no-op for
    // them; best-effort if the symbol is unavailable.
    if let Some(free_addr) = dynload::runtime_symbol("free_type_description") {
        let free_fn: extern "C" fn(*mut TypeDesc) = unsafe { std::mem::transmute(free_addr) };
        free_fn(&mut out);
    }

    result
}
