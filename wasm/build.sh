#!/usr/bin/env bash
#
# Build OpenModelica as a wasm library + JavaScript (wasm-bindgen) bindings.
#
#   ./wasm/build.sh [debug|release]      (default: debug)
#
# Produces wasm/pkg/ — the wasm-bindgen Node package exporting omc_init() and
# omc_eval(command) (see wasm/omc-cli.js). Requires the wasm32 rust target and a
# wasm-bindgen-cli matching the pinned wasm-bindgen (0.2.100):
#   rustup target add wasm32-unknown-unknown
#   cargo install wasm-bindgen-cli --version 0.2.100
set -euo pipefail

# wasm-bindgen-cli installs into the cargo bin dir, which isn't always on PATH.
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"   # .../OpenModelica.rs/wasm
ROOT="$(dirname "$HERE")"                              # .../OpenModelica.rs
cd "$ROOT"

MODE="${1:-debug}"
TARGET=wasm32-unknown-unknown
CRATE=libopenmodelica_compiler
WASM_NAME=OpenModelicaCompiler
OUTDIR="$HERE/pkg"

# wasmtime has no wasm backend, so the wasm-jit engine must be wasmer (`js`).
COMMON=(--target "$TARGET" -p "$CRATE" --no-default-features --features engine-wasmer)

case "$MODE" in
  debug)
    # The workspace dev profile selects the cranelift *rustc* backend (fast
    # native builds); it cannot target wasm32, so force LLVM for codegen here.
    cargo build "${COMMON[@]}" --config 'profile.dev.codegen-backend="llvm"'
    WASM="target/$TARGET/debug/$WASM_NAME.wasm"
    ;;
  release)
    cargo build --release "${COMMON[@]}"
    WASM="target/$TARGET/release/$WASM_NAME.wasm"
    ;;
  *)
    echo "usage: $0 [debug|release]" >&2
    exit 1
    ;;
esac

echo "==> wasm-bindgen ($MODE) -> $OUTDIR"
rm -rf "$OUTDIR"
wasm-bindgen "$WASM" --out-dir "$OUTDIR" --target nodejs

# Optional size optimisation for release if wasm-opt (binaryen) is available.
if [ "$MODE" = release ] && command -v wasm-opt >/dev/null 2>&1; then
  echo "==> wasm-opt -Oz"
  wasm-opt -Oz "$OUTDIR/${WASM_NAME}_bg.wasm" -o "$OUTDIR/${WASM_NAME}_bg.wasm"
fi

echo "==> built ($MODE):"
ls -la "$OUTDIR"
echo "Try:  node wasm/omc-cli.js 'getVersion()'   or   node wasm/omc-cli.js  (REPL)"
