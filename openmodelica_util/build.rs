// Link the system LAPACK (and its BLAS backend) so the FFI declarations in
// `src/Lapack.rs` resolve. The reference `liblapack.so` provides the
// `d*_` Fortran-ABI routines; `libblas` satisfies its transitive symbols.
fn main() {
    println!("cargo:rustc-link-lib=dylib=lapack");
    println!("cargo:rustc-link-lib=dylib=blas");

    // Runtime error interception shim for evaluated external C functions
    // (see src/runtime_error_shim.c and the rebinding in dynload::ensure_runtime).
    // The `va_list` formatting it performs cannot be written in stable Rust.
    println!("cargo:rerun-if-changed=src/runtime_error_shim.c");
    cc::Build::new()
        .file("src/runtime_error_shim.c")
        .compile("omrs_runtime_error_shim");
}
