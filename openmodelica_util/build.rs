// Link the system LAPACK (and its BLAS backend) so the FFI declarations in
// `src/Lapack.rs` resolve. The reference `liblapack.so` provides the
// `d*_` Fortran-ABI routines; `libblas` satisfies its transitive symbols.
fn main() {
    println!("cargo:rustc-link-lib=dylib=lapack");
    println!("cargo:rustc-link-lib=dylib=blas");
}
