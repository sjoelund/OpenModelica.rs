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

// MMC object headers (`MMC_STRUCTHDR(slots, ctor) = (slots << 10) | (ctor << 2)`)
// for the `RML_STYLE_TAGPTR` representation the runtime is built with: a value
// is an immediate integer when bit 0 is clear (`i << 1`), otherwise a pointer
// tagged `+3`. A struct has `hdr & 3 == 0`; a boxed string has `hdr & 7 == 5`;
// anything else boxed is a real.
const MMC_NILHDR: usize = 0; // STRUCTHDR(0, 0): {}
const MMC_CONSHDR: usize = (2 << 10) | (1 << 2); // STRUCTHDR(2, 1): cons
const MMC_NONEHDR: usize = 1 << 2; // STRUCTHDR(0, 1): NONE()
const MMC_SOMEHDR: usize = (1 << 10) | (1 << 2); // STRUCTHDR(1, 1): SOME(x)
const MMC_REALHDR: usize = (1 << 10) | 9; // boxed double
const MMC_SIZE_INT: usize = 8;

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
        // `modelica_string` is itself a boxed MMC string metatype.
        TD_STRING => decode_metatype(d.d0 as usize),
        TD_MMC => decode_metatype(d.d0 as usize),
        other => bail!("DynLoad.executeFunction: unsupported result type_description tag {other}"),
    }
}

/// Read MMC slot `i` (a machine word) of an untagged object at `base`.
#[inline]
unsafe fn slot(base: usize, i: usize) -> usize {
    unsafe { *((base + i * std::mem::size_of::<usize>()) as *const usize) }
}

/// Decode a boxed MMC value (`modelica_metatype`) into a `Values.Value`. Handles
/// the representations a `-d=gen` function can hand back across the boundary:
/// immediate integers, boxed reals/strings, lists (`cons`/`{}`), options
/// (`SOME`/`NONE`) and MetaModelica tuples. Records/uniontypes are not decoded
/// yet (they need the generated `record_description` for field names).
fn decode_metatype(m: usize) -> Result<Arc<Values::Value>> {
    // Immediate integer: bit 0 clear, value is an arithmetic right shift.
    if m & 1 == 0 {
        return Ok(Arc::new(Values::Value::INTEGER { integer: ((m as isize) >> 1) as i32 }));
    }
    let base = m - 3; // untag the pointer
    let hdr = unsafe { *(base as *const usize) };
    if hdr & 3 != 0 {
        // Not a struct: either a boxed string (`hdr & 7 == 5`) or a boxed real.
        if hdr & 7 == 5 {
            let len = (hdr >> 3) - MMC_SIZE_INT;
            let data = (base + std::mem::size_of::<usize>()) as *const u8;
            let bytes = unsafe { std::slice::from_raw_parts(data, len) };
            return Ok(Arc::new(Values::Value::STRING { string: arcstr::ArcStr::from(String::from_utf8_lossy(bytes)) }));
        }
        let val = f64::from_bits(unsafe { slot(base, 1) } as u64);
        return Ok(Arc::new(Values::Value::REAL { real: metamodelica::Real::from(val) }));
    }
    // Struct: distinguish by constructor / slot count.
    let ctor = (hdr >> 2) & 0xff;
    let slots = hdr >> 10;
    match (ctor, slots) {
        _ if hdr == MMC_NILHDR => Ok(Arc::new(Values::Value::LIST { valueLst: metamodelica::nil() })),
        _ if hdr == MMC_CONSHDR => {
            // Walk the cons spine, decoding each element.
            let mut items: Vec<Arc<Values::Value>> = Vec::new();
            let mut cur = m;
            loop {
                let b = cur - 3;
                let h = unsafe { *(b as *const usize) };
                if h == MMC_NILHDR {
                    break;
                }
                if h != MMC_CONSHDR {
                    bail!("DynLoad.executeFunction: malformed list");
                }
                items.push(decode_metatype(unsafe { slot(b, 1) })?);
                cur = unsafe { slot(b, 2) };
            }
            Ok(Arc::new(Values::Value::LIST { valueLst: Arc::new(List::from_iter(items)) }))
        }
        _ if hdr == MMC_NONEHDR => Ok(Arc::new(Values::Value::OPTION { some: None })),
        _ if hdr == MMC_SOMEHDR => {
            Ok(Arc::new(Values::Value::OPTION { some: Some(decode_metatype(unsafe { slot(base, 1) })?) }))
        }
        // Constructor 0 with at least one field is a MetaModelica tuple.
        (0, n) if n >= 1 => {
            let mut items: Vec<Arc<Values::Value>> = Vec::with_capacity(n);
            for i in 1..=n {
                items.push(decode_metatype(unsafe { slot(base, i) })?);
            }
            Ok(Arc::new(Values::Value::META_TUPLE { valueLst: Arc::new(List::from_iter(items)) }))
        }
        (c, _) if c >= 3 => {
            bail!("DynLoad.executeFunction: record/uniontype results not yet supported")
        }
        _ => bail!("DynLoad.executeFunction: unsupported MMC header {hdr:#x}"),
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

    // The generated function prints side effects through the C runtime's
    // `stdout`, a different buffer from the port's own output. Flush ours first
    // so anything already produced precedes the function's output.
    {
        use std::io::Write;
        let _ = std::io::stdout().flush();
    }

    // SAFETY: `addr` is the resolved `in_<name>` entry; its ABI is fixed by the
    // code generator. `args` outlives the call; `in_*` reads at most one element
    // per declared input.
    let func: InFn = unsafe { std::mem::transmute(addr) };
    let rc = func(thread_data, args.as_mut_ptr(), &mut out);

    // Flush the C runtime's streams: on failure the generated `in_*` wrapper
    // returns through `MMC_CATCH_TOP` before its own trailing `fflush`, so a
    // function's `print` side effects would otherwise surface out of order
    // (after the caller has already printed this call's result).
    if let Some(fflush_addr) = dynload::runtime_symbol("fflush") {
        let fflush: extern "C" fn(*mut c_void) -> i32 = unsafe { std::mem::transmute(fflush_addr) };
        fflush(std::ptr::null_mut());
    }

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
