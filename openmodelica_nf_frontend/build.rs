// Compiles the C++ exception barrier used by FFI.callFunction — see the
// comments in src/ffi_catch.cpp and src/FFI.rs.
fn main() {
    println!("cargo:rerun-if-changed=src/ffi_catch.cpp");
    cc::Build::new()
        .cpp(true)
        .file("src/ffi_catch.cpp")
        .compile("omrs_ffi_catch");
}
