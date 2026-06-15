//! JavaScript bindings for a wasm build of the compiler (wasm-bindgen). Exposes
//! the same string-to-string command interface the interactive ZeroMQ server
//! uses: call [`omc_init`] once to start the runtime, then [`omc_eval`] to
//! evaluate each interactive command and get its reply. Counterpart of the
//! native C-ABI `omc_compiler_init`/`omc_compiler_eval` in `lib.rs`.

use std::cell::RefCell;
use std::panic::{AssertUnwindSafe, catch_unwind};

use arcstr::ArcStr;
use wasm_bindgen::prelude::*;

use openmodelica_backend_main::capi;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error(s: &str);
}

// The compiler emits stdout/stderr in fragments (a `print` call need not end on
// a line boundary), but `console.log`/`console.error` each render one line. Hold
// a per-stream buffer and flush only complete lines, so multi-call output lands
// on one console line instead of many.
thread_local! {
    static OUT_BUF: RefCell<String> = const { RefCell::new(String::new()) };
    static ERR_BUF: RefCell<String> = const { RefCell::new(String::new()) };
}

fn buffer_lines(buf: &'static std::thread::LocalKey<RefCell<String>>, s: &str, emit: fn(&str)) {
    buf.with(|b| {
        let mut b = b.borrow_mut();
        b.push_str(s);
        while let Some(i) = b.find('\n') {
            // Emit the line without its trailing '\n' (console adds one).
            emit(&b[..i]);
            b.drain(..=i);
        }
    });
}

fn stdout_sink(s: &str) {
    buffer_lines(&OUT_BUF, s, console_log);
}

fn stderr_sink(s: &str) {
    buffer_lines(&ERR_BUF, s, console_error);
}

/// Seed an environment variable in the wasm in-process environment (there is no
/// OS environment on wasm). Call before [`omc_init`] to point the runtime at its
/// install dir, e.g. `omc_set_env("OPENMODELICAHOME", "/")`.
#[wasm_bindgen]
pub fn omc_set_env(name: &str, value: &str) {
    openmodelica_util::System::setEnv(ArcStr::from(name), ArcStr::from(value), true);
}

/// Initialise the compiler runtime. Returns `true` on success. Must be called
/// once before [`omc_eval`]. Mirrors `omc_compiler_init`, but additionally:
///   * routes the compiler's stdout/stderr (and Rust panics) to the JS console,
///   * defaults the code-generation target to `wasm-jit` — the only simCode
///     target usable in-browser (the C/Cpp/FMU targets need an external
///     toolchain and are unavailable here).
#[wasm_bindgen]
pub fn omc_init() -> bool {
    // Panics → console.error (instead of the default unwinding into a wasm trap
    // with no message). Installed once; the hook is process-global.
    std::panic::set_hook(Box::new(|info| {
        console_error(&format!("{info}"));
    }));
    // stdout/stderr → console. First binding wins, so this is a no-op if a
    // previous omc_init already bound them.
    metamodelica::setStdoutHook(stdout_sink);
    metamodelica::setStderrHook(stderr_sink);

    let args = [ArcStr::from("--simCodeTarget=wasm-jit")];
    matches!(catch_unwind(AssertUnwindSafe(|| capi::init(&args))), Ok(Ok(())))
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
