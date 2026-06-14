//! Static linear-memory runtime for the `wasm-jit` simCodeTarget.
//!
//! Compiled once to `wasm32-unknown-unknown` and shared (its `memory` plus the
//! `rt_*` exports) by every JIT-compiled function / simulation-RHS module. This
//! is the *static* half the user asked to precompile: the allocator, reference
//! counting and string operations live here as one optimized native artifact
//! instead of being re-emitted into each generated module.
//!
//! ## Heap object ABI
//!
//! Every heap object is reference counted. `rt_alloc` returns a pointer to the
//! object; the first 4 bytes (`obj[0..4]`) are the `u32` reference count, the
//! rest is payload. A hidden size word precedes the object so `rt_free` can hand
//! the block back to `dlmalloc`. `rt_retain` / `rt_release` adjust the count;
//! `rt_release` frees the object (via `dlmalloc`) when it reaches zero. Handle
//! `0` is the null object: retain/release on it are no-ops, so a heap local can
//! start zero-initialized.
//!
//! ## String layout
//!
//! A `String` object is `[refcount:u32][len:u32][utf8 bytes...]`; the byte data
//! starts at `obj + 8` (`rt_str_data`). Strings are immutable: every operation
//! that "modifies" a string returns a freshly allocated one. Formatting
//! (`rt_int_string` / `rt_real_string` / `rt_bool_string`) reuses the exact same
//! algorithm as the rest of the compiler so `String(x)` is byte-identical to the
//! C target (see `ryu_to_hr`, ported from `metamodelica`/the C `om_format.c`).
//!
//! ## Arrays
//!
//! 1-D arrays reuse `rt_alloc`/`rt_retain` and add a self-describing object (see
//! the Arrays section) with its own `rt_array_release` that releases contained
//! heap elements before freeing. Records will follow the same pattern with an
//! `rt_record_release` when they land.

#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use core::alloc::{GlobalAlloc, Layout};

#[global_allocator]
static GLOBAL: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

/// A wasm trap on panic (e.g. allocation failure or a bad substring range),
/// which the host surfaces as `Values.META_FAIL` exactly like a runtime error.
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

// ---------------------------------------------------------------------------
// Raw little-endian memory access (all pointers are byte offsets into the one
// shared linear memory).
// ---------------------------------------------------------------------------

#[inline]
unsafe fn load_u32(addr: u32) -> u32 {
    unsafe { core::ptr::read_unaligned(addr as *const u32) }
}

#[inline]
unsafe fn store_u32(addr: u32, v: u32) {
    unsafe { core::ptr::write_unaligned(addr as *mut u32, v) }
}

// ---------------------------------------------------------------------------
// Allocator + reference counting
// ---------------------------------------------------------------------------

/// Bytes reserved before every object for the allocation size (used by
/// `rt_free`). 8 rather than 4 so the returned object is 8-byte aligned, which
/// keeps `f64` array/record elements naturally aligned.
const HEADER: usize = 8;
const ALIGN: usize = 8;

/// Allocate an object of `size` payload bytes (including its 4-byte refcount),
/// returning its pointer. The reference count is left zero — the typed
/// constructors below set it to 1.
#[unsafe(no_mangle)]
pub extern "C" fn rt_alloc(size: u32) -> u32 {
    let total = HEADER + size as usize;
    let layout = Layout::from_size_align(total, ALIGN).expect("bad layout");
    let raw = unsafe { GLOBAL.alloc(layout) } as u32;
    if raw == 0 {
        // Out of memory: trap.
        core::arch::wasm32::unreachable();
    }
    unsafe { store_u32(raw, total as u32) };
    raw + HEADER as u32
}

/// Free an object previously returned by `rt_alloc`.
#[unsafe(no_mangle)]
pub extern "C" fn rt_free(obj: u32) {
    if obj == 0 {
        return;
    }
    let raw = obj - HEADER as u32;
    let total = unsafe { load_u32(raw) } as usize;
    let layout = Layout::from_size_align(total, ALIGN).expect("bad layout");
    unsafe { GLOBAL.dealloc(raw as *mut u8, layout) };
}

/// Increment an object's reference count (no-op on the null handle).
#[unsafe(no_mangle)]
pub extern "C" fn rt_retain(obj: u32) {
    if obj == 0 {
        return;
    }
    unsafe { store_u32(obj, load_u32(obj) + 1) };
}

/// Decrement an object's reference count, freeing it at zero (no-op on null).
///
/// Only valid for objects with no contained heap references (currently every
/// `String`). Arrays/records of heap elements will get typed release entry
/// points that release their children first.
#[unsafe(no_mangle)]
pub extern "C" fn rt_release(obj: u32) {
    if obj == 0 {
        return;
    }
    let rc = unsafe { load_u32(obj) } - 1;
    unsafe { store_u32(obj, rc) };
    if rc == 0 {
        rt_free(obj);
    }
}

// ---------------------------------------------------------------------------
// Arrays (N-dimensional, flat row-major)
// ---------------------------------------------------------------------------
//
// An array object is
//   `[refcount:u32][elem_kind:u32][ndims:u32][total:u32][dim_0:u32 .. dim_{n-1}:u32]
//    [pad to 8][elements row-major ...]`.
// `total` is the product of the dims (the number of scalar elements). The
// element area is 8-byte aligned (so `f64` elements are aligned) and the
// elements are packed by kind — 4 bytes for Integer/Boolean and for a heap
// *handle* (String / nested array), 8 bytes for Real — stored row-major (last
// subscript varies fastest), matching the C runtime's `base_array` and the
// nested `Values.ARRAY` the rest of the compiler uses.
//
// The `elem_kind`/`ndims`/`dim_*` words make the object self-describing, so the
// generic `rt_array_release` frees contained heap elements (recursing into
// nested arrays) and the host marshals it out without a per-element sidecar.
//
// The element-kind tags MUST match `SigTy::elem_kind` in the codegen.
const EK_INT: u32 = 0;
const EK_REAL: u32 = 1;
const EK_BOOL: u32 = 2;
const EK_STR: u32 = 3;
const EK_ARRAY: u32 = 4;

const ARR_KIND_OFF: u32 = 4;
const ARR_NDIMS_OFF: u32 = 8;
const ARR_TOTAL_OFF: u32 = 12;
const ARR_DIMS_OFF: u32 = 16;

/// Byte stride of one element of the given kind: 8 for Real, 4 for Integer /
/// Boolean and for a heap handle (String / nested array).
fn elem_stride(kind: u32) -> u32 {
    match kind {
        EK_REAL => 8,
        EK_INT | EK_BOOL | EK_STR | EK_ARRAY => 4,
        _ => 4,
    }
}

/// Round `n` up to the next multiple of 8 (element-area alignment).
fn align8(n: u32) -> u32 {
    (n + 7) & !7
}

/// Byte offset from the object base to the first element, given `ndims`.
fn arr_data_off(ndims: u32) -> u32 {
    align8(ARR_DIMS_OFF + ndims * 4)
}

/// Byte address of the first element.
fn arr_data(obj: u32) -> u32 {
    obj + arr_data_off(unsafe { load_u32(obj + ARR_NDIMS_OFF) })
}

/// Allocate a zero-initialized array object: `ndims` dimensions, `total` scalar
/// elements of `elem_kind` (refcount 1, dims left 0 — set with `rt_array_set_dim`).
/// Zeroing means handle elements start as the null handle (0), so releasing a
/// partially filled array is safe.
#[unsafe(no_mangle)]
pub extern "C" fn rt_array_new(elem_kind: u32, ndims: u32, total: u32) -> u32 {
    let data_off = arr_data_off(ndims);
    let obj = rt_alloc(data_off + total * elem_stride(elem_kind));
    unsafe {
        store_u32(obj, 1); // refcount
        store_u32(obj + ARR_KIND_OFF, elem_kind);
        store_u32(obj + ARR_NDIMS_OFF, ndims);
        store_u32(obj + ARR_TOTAL_OFF, total);
        // Zero the dim words and the element area (rt_alloc does not zero).
        for off in (ARR_DIMS_OFF..data_off + total * elem_stride(elem_kind)).step_by(4) {
            store_u32(obj + off, 0);
        }
    }
    obj
}

/// Set the size of dimension `axis` (0-based) of an array.
#[unsafe(no_mangle)]
pub extern "C" fn rt_array_set_dim(obj: u32, axis: u32, size: u32) {
    unsafe { store_u32(obj + ARR_DIMS_OFF + axis * 4, size) };
}

/// Number of dimensions.
#[unsafe(no_mangle)]
pub extern "C" fn rt_array_ndims(obj: u32) -> u32 {
    unsafe { load_u32(obj + ARR_NDIMS_OFF) }
}

/// Total number of scalar elements (product of the dimensions).
#[unsafe(no_mangle)]
pub extern "C" fn rt_array_total(obj: u32) -> u32 {
    unsafe { load_u32(obj + ARR_TOTAL_OFF) }
}

/// Size of dimension `axis` (1-based, as Modelica `size(a, axis)` is). Traps on
/// an out-of-range axis.
#[unsafe(no_mangle)]
pub extern "C" fn rt_array_dim(obj: u32, axis: i32) -> u32 {
    let ndims = rt_array_ndims(obj) as i32;
    if axis < 1 || axis > ndims {
        core::arch::wasm32::unreachable();
    }
    unsafe { load_u32(obj + ARR_DIMS_OFF + (axis as u32 - 1) * 4) }
}

/// Byte address of the element at row-major linear position `index` (1-based).
/// Traps on an out-of-range index (→ META_FAIL), matching the bounds check the
/// C target performs. The generated code computes `index` from the per-axis
/// subscripts and `rt_array_dim`, then loads/stores through this address with
/// the element's natural wasm type.
#[unsafe(no_mangle)]
pub extern "C" fn rt_array_elem_ptr(obj: u32, index: i32) -> u32 {
    let total = rt_array_total(obj) as i32;
    if index < 1 || index > total {
        core::arch::wasm32::unreachable();
    }
    let kind = unsafe { load_u32(obj + ARR_KIND_OFF) };
    arr_data(obj) + (index as u32 - 1) * elem_stride(kind)
}

/// Decrement an array's reference count, freeing it at zero (no-op on null).
/// Before freeing, any contained heap elements are released according to the
/// element kind, so nested strings / arrays are not leaked. Recurses through
/// nested arrays (each carries its own `elem_kind`).
#[unsafe(no_mangle)]
pub extern "C" fn rt_array_release(obj: u32) {
    if obj == 0 {
        return;
    }
    let rc = unsafe { load_u32(obj) } - 1;
    unsafe { store_u32(obj, rc) };
    if rc != 0 {
        return;
    }
    let kind = unsafe { load_u32(obj + ARR_KIND_OFF) };
    // Heap element kinds hold handles that must be released first.
    if kind == EK_STR || kind == EK_ARRAY {
        let data = arr_data(obj);
        for i in 0..rt_array_total(obj) {
            let handle = unsafe { load_u32(data + i * 4) };
            if kind == EK_STR {
                rt_release(handle);
            } else {
                rt_array_release(handle);
            }
        }
    }
    rt_free(obj);
}

// ---------------------------------------------------------------------------
// Strings
// ---------------------------------------------------------------------------

const STR_LEN_OFF: u32 = 4;
const STR_DATA_OFF: u32 = 8;

/// Allocate an uninitialized `String` object of `len` bytes (refcount 1, length
/// set). The caller fills `rt_str_data(obj)..+len` with the bytes.
#[unsafe(no_mangle)]
pub extern "C" fn rt_str_new(len: u32) -> u32 {
    let obj = rt_alloc(STR_DATA_OFF + len);
    unsafe {
        store_u32(obj, 1); // refcount
        store_u32(obj + STR_LEN_OFF, len);
    }
    obj
}

/// Byte length of a string.
#[unsafe(no_mangle)]
pub extern "C" fn rt_str_len(obj: u32) -> u32 {
    unsafe { load_u32(obj + STR_LEN_OFF) }
}

/// Pointer to a string's UTF-8 bytes (lets generated/host code load/store
/// directly without per-byte calls).
#[unsafe(no_mangle)]
pub extern "C" fn rt_str_data(obj: u32) -> u32 {
    obj + STR_DATA_OFF
}

/// View a string object's bytes as a slice.
unsafe fn str_bytes<'a>(obj: u32) -> &'a [u8] {
    let len = rt_str_len(obj) as usize;
    unsafe { core::slice::from_raw_parts((obj + STR_DATA_OFF) as *const u8, len) }
}

/// Allocate a `String` object holding `s` and return its pointer.
fn new_str_from(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let obj = rt_str_new(bytes.len() as u32);
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), rt_str_data(obj) as *mut u8, bytes.len());
    }
    obj
}

/// `stringAppend(a, b)` — concatenate two strings into a fresh one.
#[unsafe(no_mangle)]
pub extern "C" fn rt_concat(a: u32, b: u32) -> u32 {
    let la = rt_str_len(a);
    let lb = rt_str_len(b);
    let obj = rt_str_new(la + lb);
    unsafe {
        let dst = rt_str_data(obj) as *mut u8;
        core::ptr::copy_nonoverlapping(rt_str_data(a) as *const u8, dst, la as usize);
        core::ptr::copy_nonoverlapping(rt_str_data(b) as *const u8, dst.add(la as usize), lb as usize);
    }
    obj
}

/// `stringEqual(a, b)` → 1 / 0.
#[unsafe(no_mangle)]
pub extern "C" fn rt_streq(a: u32, b: u32) -> i32 {
    (unsafe { str_bytes(a) == str_bytes(b) }) as i32
}

/// `stringCompare(a, b)` → -1 / 0 / 1 (lexicographic over bytes).
#[unsafe(no_mangle)]
pub extern "C" fn rt_strcmp(a: u32, b: u32) -> i32 {
    use core::cmp::Ordering::*;
    match unsafe { str_bytes(a).cmp(str_bytes(b)) } {
        Less => -1,
        Equal => 0,
        Greater => 1,
    }
}

/// `substring(s, i, j)` — 1-based inclusive `[i, j]`. A bad range traps (→
/// META_FAIL), matching the bounds check in the canonical builtin.
#[unsafe(no_mangle)]
pub extern "C" fn rt_substring(obj: u32, i: i32, j: i32) -> u32 {
    let len = rt_str_len(obj) as i32;
    if i < 1 || j > len || i > j + 1 {
        core::arch::wasm32::unreachable();
    }
    let start = (i - 1) as usize;
    let count = (j - i + 1) as usize;
    let out = rt_str_new(count as u32);
    unsafe {
        core::ptr::copy_nonoverlapping(
            (rt_str_data(obj) as *const u8).add(start),
            rt_str_data(out) as *mut u8,
            count,
        );
    }
    out
}

/// `String(i)` for an Integer.
#[unsafe(no_mangle)]
pub extern "C" fn rt_int_string(i: i32) -> u32 {
    new_str_from(&format!("{i}"))
}

/// `String(b)` for a Boolean.
#[unsafe(no_mangle)]
pub extern "C" fn rt_bool_string(b: i32) -> u32 {
    new_str_from(if b != 0 { "true" } else { "false" })
}

/// `String(r)` for a Real — byte-identical to `metamodelica::realString` and the
/// C target (`ryu_to_hr` below is the same algorithm).
#[unsafe(no_mangle)]
pub extern "C" fn rt_real_string(r: f64) -> u32 {
    if r.is_infinite() {
        return new_str_from(if r < 0.0 { "-inf" } else { "inf" });
    }
    if r.is_nan() {
        return new_str_from("NaN");
    }
    new_str_from(&ryu_to_hr(&format!("{r:e}"), true))
}

/// `String(r, significantDigits, minimumLength, leftJustified)` for a Real.
///
/// The frontend always expands `String(Real)` to this 4-argument form (defaults
/// `significantDigits = 6`, `minimumLength = 0`, `leftJustified = true`), and
/// `Ceval.cevalBuiltinString` evaluates it as the C `printf` conversion
/// `"%[-]{minimumLength}.{significantDigits}g"`. This must therefore match C's
/// `%g` (NOT the shortest-round-trip `rt_real_string`/`realString`). The `%g`
/// rendering and the trailing-zero trimming below mirror
/// `openmodelica_util::System::c_format_double` exactly so the wasm-jit target,
/// the Ceval interpreter and the C target all agree byte-for-byte.
#[unsafe(no_mangle)]
pub extern "C" fn rt_real_format(r: f64, sig: i32, min_len: i32, left_just: i32) -> u32 {
    // C `%g` treats a precision of 0 as 1; the default is 6. `significantDigits`
    // is always supplied here, so a non-positive value is the only edge case.
    let p = if sig < 1 { 1 } else { sig as usize };
    let body = format_g(r, p);
    new_str_from(&pad(&body, min_len, left_just != 0))
}

/// Pad a string to `min_len` bytes with spaces (`leftJustified` → trailing,
/// otherwise leading), used by `String(Integer/Boolean/Enumeration, minLength,
/// leftJustified)`. Mirrors `ExpressionSimplify.cevalBuiltinStringFormat`:
/// strings already at least `min_len` long are returned unchanged.
#[unsafe(no_mangle)]
pub extern "C" fn rt_str_pad(obj: u32, min_len: i32, left_just: i32) -> u32 {
    let s = core::str::from_utf8(unsafe { str_bytes(obj) }).unwrap_or("");
    new_str_from(&pad(s, min_len, left_just != 0))
}

/// Space-pad `s` to `min_len` bytes (no-op if already long enough). Shared by
/// `rt_real_format` (printf width field) and `rt_str_pad`.
fn pad(s: &str, min_len: i32, left_just: bool) -> String {
    let len = s.len() as i32;
    if len >= min_len {
        return String::from(s);
    }
    let fill = (min_len - len) as usize;
    let spaces = " ".repeat(fill);
    if left_just { format!("{s}{spaces}") } else { format!("{spaces}{s}") }
}

/// C `%.{p}g` of `val` (`p` significant digits, `p >= 1`), with trailing zeros
/// trimmed. Faithful port of the `'g'` arm of
/// `openmodelica_util::System::c_format_double` (kept identical so all targets
/// agree): the `%e`-style decimal exponent `x` selects the presentation — `%f`
/// when `-4 <= x < p`, otherwise `%e` — then a trailing-zero/point trim.
fn format_g(val: f64, p: usize) -> String {
    let x = decimal_exponent(val, p);
    let s = if x >= -4 && (x as i64) < p as i64 {
        let prec = (p as i32 - 1 - x).max(0) as usize;
        format!("{val:.prec$}")
    } else {
        format_c_exp(val, p - 1)
    };
    trim_g_significand(s)
}

/// Decimal exponent `X` that C's `%e` conversion of `val` would use at `sig`
/// significant digits (rounding can carry into a higher exponent, e.g. 9.99 at
/// 2 sig digits → "1.0e1", X = 1). Mirrors `System::decimal_exponent`.
fn decimal_exponent(val: f64, sig: usize) -> i32 {
    if val == 0.0 || !val.is_finite() {
        return 0;
    }
    let raw = format!("{:.*e}", sig.saturating_sub(1), val);
    raw.split_once('e').and_then(|(_, e)| e.parse().ok()).unwrap_or(0)
}

/// C `%e` rendering: mantissa, `e`, explicit sign, at-least-two-digit exponent
/// (Rust's `{:e}` omits the sign and zero-padding). Mirrors `System::format_c_exp`.
fn format_c_exp(val: f64, precision: usize) -> String {
    let raw = format!("{:.*e}", precision, val);
    let (mant, exp) = raw.split_once('e').unwrap_or((raw.as_str(), "0"));
    let exp_num: i32 = exp.parse().unwrap_or(0);
    let sign = if exp_num < 0 { '-' } else { '+' };
    format!("{mant}e{sign}{:02}", exp_num.abs())
}

/// Strip trailing zeros (and a dangling decimal point) from a `%g` significand,
/// leaving any exponent suffix intact. Mirrors `System::trim_g_significand`.
fn trim_g_significand(s: String) -> String {
    let (mant, exp) = match s.find(['e', 'E']) {
        Some(idx) => (&s[..idx], &s[idx..]),
        None => (s.as_str(), ""),
    };
    let mant = if mant.contains('.') {
        mant.trim_end_matches('0').trim_end_matches('.')
    } else {
        mant
    };
    format!("{mant}{exp}")
}

/// Port of `ryu_to_hr` from `3rdParty/ryu/ryu/om_format.c` (and
/// `metamodelica::ryu_to_hr`): convert a shortest-form scientific representation
/// (`8.13e2`) to the minimal decimal / exponential rendering omc uses for Reals.
/// `real_output` adds a trailing `.0` to round values. Kept identical to the
/// `metamodelica` copy so `String(Real)` matches everywhere.
fn ryu_to_hr(d2s_str: &str, real_output: bool) -> String {
    let Some(epos) = d2s_str.find(['e', 'E']) else {
        return d2s_str.replace('E', "e");
    };
    let mant_str = &d2s_str[..epos];
    let mut exp: i32 = d2s_str[epos + 1..].parse().unwrap_or(0);
    let (neg, mut digits) = match mant_str.strip_prefix('-') {
        Some(m) => (true, String::from(m)),
        None => (false, String::from(mant_str)),
    };
    let mut ndec: i32 = if digits.contains('.') { digits.len() as i32 - 2 } else { 0 };
    let mut exp_repr: String = d2s_str.replace('E', "e");

    if ndec > 12 && !real_output {
        let mant: f64 = digits.parse().unwrap_or(0.0);
        let mut rounded = format!("{mant:.12}");
        if rounded == "10.000000000000" {
            rounded = String::from("1.000000000000");
            exp += 1;
        }
        let mut nz = 0;
        while rounded.ends_with('0') {
            rounded.pop();
            nz += 1;
        }
        if rounded.ends_with('.') {
            rounded.pop();
        }
        if nz > 3 {
            digits = rounded;
            ndec = if digits.contains('.') { digits.len() as i32 - 2 } else { 0 };
            exp_repr = format!("{}{digits}e{exp}", if neg { "-" } else { "" });
        }
    }

    if !(-3..=5).contains(&exp) || (exp > 0 && exp - ndec > 3) {
        return exp_repr;
    }

    let digs: alloc::vec::Vec<char> = digits.chars().filter(|c| *c != '.').collect();
    let mut out = String::with_capacity(24);
    if neg {
        out.push('-');
    }
    if exp == 0 {
        out.push_str(&digits);
    } else if exp > 0 {
        out.push(digs[0]);
        let take = ndec.min(exp) as usize;
        out.extend(&digs[1..1 + take]);
        if exp > ndec {
            for _ in 0..(exp - ndec) {
                out.push('0');
            }
        } else if exp < ndec {
            out.push('.');
            out.extend(&digs[1 + take..]);
        }
    } else {
        out.push_str("0.");
        for _ in 0..(-exp - 1) {
            out.push('0');
        }
        out.extend(&digs);
    }
    if exp >= ndec && real_output {
        out.push_str(".0");
    }
    out
}
