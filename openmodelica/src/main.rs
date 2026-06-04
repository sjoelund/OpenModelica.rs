use arcstr::ArcStr;
use openmodelica_backend_main::Main;
use std::env::args;
use std::io::Write;
use std::sync::Arc;

fn main() -> std::process::ExitCode {
    if Main::main(Arc::new(args().skip(1).map(|e| ArcStr::from(e)).collect())).is_err() {
        // Mirror `rml_execution_failed()` in the generated C wrapper
        // (CodegenCFunctions.tpl): flush pending output, report on stderr
        // and exit with status 1. The MetaModelica exception carries no
        // payload worth printing — diagnostics were already emitted via
        // the Error buffer before `fail()` reached us.
        let _ = std::io::stdout().flush();
        eprintln!("Execution failed!");
        return std::process::ExitCode::FAILURE;
    }
    std::process::ExitCode::SUCCESS
}
