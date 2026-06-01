use openmodelica_backend_main::Main;
use std::env::args;
use arcstr::ArcStr;
use std::sync::Arc;

fn main() -> () {
    if let Err(e) = Main::main(Arc::new(args().map(|e| ArcStr::from(e)).collect())) {
        println!("{:?}", e);
    }
}
