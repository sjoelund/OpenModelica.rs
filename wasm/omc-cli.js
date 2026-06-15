#!/usr/bin/env node
//
// Minimal OpenModelica wasm CLI. Loads the wasm-bindgen package built by
// wasm/build.sh, initialises the compiler runtime, then evaluates interactive
// commands as strings and prints the replies — the same string-to-string
// protocol the interactive ZeroMQ server speaks.
//
//   node wasm/omc-cli.js 'getVersion()'     # one-shot
//   node wasm/omc-cli.js                     # REPL (Ctrl-D or quit() to exit)
//
'use strict';

const path = require('node:path');
const readline = require('node:readline');

const omc = require(path.join(__dirname, 'pkg', 'OpenModelicaCompiler.js'));

// Seed the install dir (no OS environment inside wasm). Note: the wasm build has
// no filesystem yet, so commands that read library files will still fail — but
// self-contained commands (getVersion, arithmetic, …) work.
omc.omc_set_env('OPENMODELICAHOME', process.env.OPENMODELICAHOME || '/usr');

if (!omc.omc_init()) {
  // The wasm32-unknown-unknown build has no filesystem / OPENMODELICAHOME yet,
  // so full initialisation (loading the Modelica library, etc.) may not
  // complete; simple, self-contained commands can still work.
  console.error('warning: omc_init() reported failure (no filesystem/OPENMODELICAHOME in this wasm build)');
}

const oneShot = process.argv.slice(2).join(' ').trim();
if (oneShot) {
  process.stdout.write(omc.omc_eval(oneShot));
  process.stdout.write('\n');
  process.exit(0);
}

const rl = readline.createInterface({ input: process.stdin, output: process.stdout, prompt: '>>> ' });
rl.prompt();
rl.on('line', (line) => {
  const cmd = line.trim();
  if (cmd === 'quit()' || cmd === 'exit') { rl.close(); return; }
  if (cmd) process.stdout.write(omc.omc_eval(cmd) + '\n');
  rl.prompt();
});
rl.on('close', () => process.exit(0));
