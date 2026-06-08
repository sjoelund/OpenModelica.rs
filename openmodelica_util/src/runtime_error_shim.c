/*
 * Runtime error interception shim for evaluated external C functions.
 *
 * Mirror of `OpenModelica_ErrorModule_Modelica{,V}FormatError` in
 * `Compiler/runtime/errorext.cpp`: when the compiler evaluates an external C
 * function (the `-d=gen` dlopen path), a `ModelicaError` / `ModelicaFormatError`
 * call inside it must append a RUNTIME/error message to the compiler's error
 * buffer (so `getErrorString` reports it) instead of only streaming to the
 * simulation log and throwing.
 *
 * The reference compiler achieves this in `Error_registerModelicaFormatError`
 * by rebinding the runtime's `OpenModelica_Modelica{,V}FormatError` function
 * pointers. The Rust port does the same from `dynload::ensure_runtime`, but the
 * `va_list` formatting cannot be expressed in stable Rust — hence this C shim.
 *
 * `omrs_register_modelica_error` stores the runtime's original throwing
 * functions; each shim formats/forwards the message to the Rust error buffer
 * (`omrs_add_runtime_error`) and then calls the original to perform the throw
 * (`MMC_THROW` via the runtime's longjmp), exactly as the C ErrorModule path
 * adds the message and then `MMC_THROW`s.
 */
#include <stdarg.h>
#include <stdio.h>

/* Defined in Rust (ErrorExt.rs): append a RUNTIME/error message to the buffer. */
extern void omrs_add_runtime_error(const char *msg);

typedef void (*omrs_err_fn)(const char *);
typedef void (*omrs_verr_fn)(const char *, va_list);

static omrs_err_fn  omrs_orig_error = 0;
static omrs_verr_fn omrs_orig_vformat_error = 0;

/* Replacement for OpenModelica_ModelicaError: report, then throw. */
static void omrs_modelica_error(const char *msg) {
  omrs_add_runtime_error(msg);
  if (omrs_orig_error) {
    omrs_orig_error(msg); /* longjmp/throw — does not return */
  }
}

/* Replacement for OpenModelica_ModelicaVFormatError: format, report, throw. */
static void omrs_modelica_vformat_error(const char *fmt, va_list ap) {
  char buf[8192];
  va_list ap2;
  va_copy(ap2, ap);
  vsnprintf(buf, sizeof(buf), fmt, ap2);
  va_end(ap2);
  omrs_add_runtime_error(buf);
  if (omrs_orig_vformat_error) {
    omrs_orig_vformat_error(fmt, ap); /* longjmp/throw — does not return */
  }
}

/*
 * Install the interception. `err_slot`/`verr_slot` are the addresses of the
 * runtime's `OpenModelica_ModelicaError` / `OpenModelica_ModelicaVFormatError`
 * function-pointer variables (resolved by dlsym in dynload::ensure_runtime).
 * The originals are saved for the throw, then the slots are repointed at the
 * shims. Keeping all function-pointer typing in C avoids expressing `va_list`
 * in Rust. Idempotent only if called once; the caller guards that.
 */
void omrs_install_modelica_error(omrs_err_fn *err_slot, omrs_verr_fn *verr_slot) {
  if (err_slot) {
    omrs_orig_error = *err_slot;
    *err_slot = omrs_modelica_error;
  }
  if (verr_slot) {
    omrs_orig_vformat_error = *verr_slot;
    *verr_slot = omrs_modelica_vformat_error;
  }
}
