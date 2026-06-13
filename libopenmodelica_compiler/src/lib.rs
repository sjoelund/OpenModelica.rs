//! `libOpenModelicaCompiler.so` — the C ABI OMEdit links against to drive the
//! Rust omc in-process.
//!
//! This exposes the same interactive command/response protocol used over
//! ZeroMQ, but through direct function calls: initialise the runtime once with
//! [`omc_compiler_init`], then evaluate command strings with
//! [`omc_compiler_eval`]. The implementation is a thin C wrapper over the safe
//! [`openmodelica_backend_main::capi`] embedding API.
//!
//! ## Contract
//! * Call [`omc_compiler_init`] exactly once, before any [`omc_compiler_eval`].
//! * All calls must come from the **same thread** (the compiler keeps
//!   per-thread state), and that thread should have a large stack (several MiB)
//!   — see the note in [`openmodelica_backend_main::capi`].
//! * Strings returned by [`omc_compiler_eval`] are owned by the caller and must
//!   be released with [`omc_compiler_free_string`].
//! * No Rust panic is allowed to cross the FFI boundary; every entry point
//!   traps unwinding and reports failure instead.
//!
//! The typed `OMCInterface` (generated `OpenModelicaScriptingAPIQt.cpp`) that
//! OMEdit also uses is a future layer built on top of this string interface;
//! see the design notes accompanying this crate.

use arcstr::ArcStr;
use openmodelica_backend_main::capi;
use std::ffi::{CStr, CString, c_char, c_int};
use std::panic::{AssertUnwindSafe, catch_unwind};

// Re-export the generated typed OMEdit interface ABI (the `extern "C"` wrappers
// behind OpenModelicaScriptingAPIQt). The `pub use` makes the `#[no_mangle]`
// symbols reachable from this cdylib's crate root so the linker keeps them in
// `libOpenModelicaCompiler.so` (an rlib's `#[no_mangle]` items are otherwise
// liable to be dropped if nothing references them).
pub use openmodelica_backend_main::scripting_api_qt::*;

/// Initialise the compiler runtime on the calling thread.
///
/// Returns `0` on success and `-1` on failure (initialisation error or a
/// panic). The installation directory is taken from the `OPENMODELICAHOME`
/// environment variable, as in a normal omc startup. No command-line flags are
/// passed; a richer init taking `argc`/`argv` can be added when OMEdit needs to
/// forward flags.
#[unsafe(no_mangle)]
pub extern "C" fn omc_compiler_init() -> c_int {
    match catch_unwind(|| capi::init(&[])) {
        Ok(Ok(())) => 0,
        Ok(Err(_)) | Err(_) => -1,
    }
}

/// Evaluate one interactive command string and return its reply.
///
/// `command` must be a non-null, NUL-terminated UTF-8 string. The returned
/// pointer is a newly-allocated NUL-terminated C string owned by the caller
/// (free it with [`omc_compiler_free_string`]). Returns null only if `command`
/// is null or evaluation traps; a normal evaluation error is reported through
/// the reply string itself (the same way the interactive server does).
#[unsafe(no_mangle)]
pub extern "C" fn omc_compiler_eval(command: *const c_char) -> *mut c_char {
    if command.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: the contract requires a valid NUL-terminated string.
    let cmd_bytes = unsafe { CStr::from_ptr(command) }.to_bytes();
    let cmd = ArcStr::from(String::from_utf8_lossy(cmd_bytes));

    let reply = match catch_unwind(AssertUnwindSafe(|| capi::eval(cmd))) {
        Ok(Ok((_keep_running, reply))) => reply,
        // Evaluation failure: surface the error text rather than a bare null so
        // the embedder gets a diagnostic, matching omc's interactive behaviour.
        Ok(Err(e)) => ArcStr::from(format!("Error: {e}")),
        Err(_) => return std::ptr::null_mut(),
    };

    // NUL bytes cannot appear in a C string; replace any (there should be none
    // in a textual reply) so construction cannot fail.
    match CString::new(reply.as_bytes()) {
        Ok(c) => c.into_raw(),
        Err(_) => {
            let sanitized: Vec<u8> = reply.as_bytes().iter().copied().filter(|&b| b != 0).collect();
            CString::new(sanitized).unwrap().into_raw()
        }
    }
}

/// Free a string previously returned by [`omc_compiler_eval`].
#[unsafe(no_mangle)]
pub extern "C" fn omc_compiler_free_string(s: *mut c_char) {
    if !s.is_null() {
        // SAFETY: `s` was produced by `CString::into_raw` in this module.
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}
