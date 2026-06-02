#!/bin/bash
RUST=$PWD
/usr/bin/omc listMSL.mos
mkdir -p /projects/OpenModelica/OMCompiler/Compiler/boot/parser
cd /projects/OpenModelica/OMCompiler/Compiler/boot/parser
omc -g=MetaModelica $RUST/listMetaModelicaFiles.mos
P=`realpath /projects/OpenModelica/OMCompiler/Compiler`
sed -i "s,$P,/projects/OpenModelica/OMCompiler/Compiler," compilerSources.txt
cp compilerSources.txt $RUST
