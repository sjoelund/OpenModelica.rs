// Auto-generated lib file
#![recursion_limit = "1024"]
#[cfg(not(target_arch = "wasm32"))]
pub mod Curl;
#[cfg(target_arch = "wasm32")]
#[path = "Curl_wasm.rs"]
pub mod Curl;
pub mod DynLoad;
pub mod GlobalScriptUtil;
pub mod PackageManagement;
pub mod SimulationResults;
pub mod UnitParserExt;
pub mod Unzip;
pub mod DynLoadExt;
