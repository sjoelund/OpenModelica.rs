#!/bin/bash
#
# Build OMEdit against the Rust omc port (libOpenModelicaCompiler.so) in-process.
#
# Builds the cdylib, installs the Rust-targeting OMC interface + the embedding
# header + the .so into the OpenModelica build tree, then builds OMEdit with the
# OMC_RUST_ABI compile-time switch enabled. See omedit-rust-embedding notes.
#
# Run from the OpenModelica.rs checkout:  ./build-omedit.sh
# Override the build tree with: OMBUILDDIR=/path/to/build ./build-omedit.sh
#
set -euo pipefail

RUST="$(cd "$(dirname "$0")" && pwd)"
OMBUILDDIR="${OMBUILDDIR:-/projects/OpenModelica/build}"
OMEDIT="${OMEDIT:-/projects/OpenModelica/OMEdit}"
HOST_SHORT="${HOST_SHORT:-$(gcc -dumpmachine)}"
JOBS="${JOBS:-12}"

INC="$OMBUILDDIR/include/omc/scripting-API"
LIB="$OMBUILDDIR/lib/$HOST_SHORT/omc"

echo ">>> 1/4 building libOpenModelicaCompiler.so (release)"
cd "$RUST"
cargo build -j"$JOBS" --release -p libopenmodelica_compiler

echo ">>> 2/4 installing the Rust OMC interface + embedding header into $INC"
mkdir -p "$INC" "$LIB"
# Back up the C-omc versions once, so the stock build can be restored.
for f in "$LIB/libOpenModelicaCompiler.so" \
         "$INC/OpenModelicaScriptingAPIQt.cpp" "$INC/OpenModelicaScriptingAPIQt.h"; do
  [ -f "$f" ] && [ ! -f "$f.cbak" ] && cp -a "$f" "$f.cbak" && echo "    backed up $(basename "$f") -> $(basename "$f").cbak"
done
cp "$RUST/openmodelica_scripting_qt/qt/OpenModelicaScriptingAPIQt.cpp" "$INC/"
cp "$RUST/openmodelica_scripting_qt/qt/OpenModelicaScriptingAPIQt.h"   "$INC/"
cp "$RUST/openmodelica_scripting_qt/qt/OpenModelicaScriptingAPIQtABI.h" "$INC/"
cp "$RUST/libopenmodelica_compiler/include/omc_rust_embedding.h"        "$INC/"
cp "$RUST/target/release/libOpenModelicaCompiler.so" "$LIB/libOpenModelicaCompiler.so"

echo ">>> 3/4 qmake (OMEDIT_RUST_OMC=1 enables the OMC_RUST_ABI switch)"
cd "$OMEDIT"
OMEDIT_RUST_OMC=1 qmake6 -r

echo ">>> 4/4 building OMEdit"
OMEDIT_RUST_OMC=1 make -j"$JOBS"

cat <<EOF

OMEdit built: $OMEDIT/bin/OMEdit
Run it with the build tree's libraries on the path, e.g.:

  OPENMODELICAHOME=$OMBUILDDIR \\
  LD_LIBRARY_PATH=$LIB:$OMBUILDDIR/lib:$OMBUILDDIR/lib/$HOST_SHORT \\
  $OMEDIT/bin/OMEdit

(add QT_QPA_PLATFORM=offscreen for a headless smoke test).
To restore the stock C build, copy the *.cbak files back over the installed ones.
EOF
