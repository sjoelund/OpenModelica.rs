//! JavaScript bindings for a wasm build of the compiler (wasm-bindgen). Exposes
//! the same string-to-string command interface the interactive ZeroMQ server
//! uses: call [`omc_init`] once to start the runtime, then [`omc_eval`] to
//! evaluate each interactive command and get its reply. Counterpart of the
//! native C-ABI `omc_compiler_init`/`omc_compiler_eval` in `lib.rs`.

use std::panic::{AssertUnwindSafe, catch_unwind};

use arcstr::ArcStr;
use wasm_bindgen::prelude::*;

use openmodelica_backend_main::capi;

/// Seed an environment variable in the wasm in-process environment (there is no
/// OS environment on wasm). Call before [`omc_init`] to point the runtime at its
/// install dir, e.g. `omc_set_env("OPENMODELICAHOME", "/")`.
#[wasm_bindgen]
pub fn omc_set_env(name: &str, value: &str) {
    openmodelica_util::System::setEnv(ArcStr::from(name), ArcStr::from(value), true);
}

/// Initialise the compiler runtime. Returns `true` on success. Must be called
/// once before [`omc_eval`]. Mirrors `omc_compiler_init` (no command-line flags).
#[wasm_bindgen]
pub fn omc_init() -> bool {
    matches!(catch_unwind(AssertUnwindSafe(|| capi::init(&[]))), Ok(Ok(())))
}

/// Evaluate one interactive command and return its reply — the same string the
/// `--interactive=zmq` server returns for a request. Evaluation errors and
/// panics are returned as `"Error: …"` text rather than thrown, so a REPL can
/// keep running.
#[wasm_bindgen]
pub fn omc_eval(command: &str) -> String {
    match catch_unwind(AssertUnwindSafe(|| capi::eval(ArcStr::from(command)))) {
        Ok(Ok((_keep, reply))) => reply.to_string(),
        Ok(Err(e)) => format!("Error: {e}"),
        Err(_) => "Error: evaluation panicked".to_owned(),
    }
}
