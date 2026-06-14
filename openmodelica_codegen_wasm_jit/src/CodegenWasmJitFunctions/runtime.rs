// The execute half of the `wasm-jit` target: load a module produced by
// `super::translateFunctions`, JIT it with `wasmtime` and call its `main`
// export, marshalling `Values.Value`s in and out. Counterpart of
// `DynLoadExt::executeFunction` for the C/dlopen target — far simpler here
// because the calling convention is just scalar wasm params/results plus the
// `.wasm.sig` sidecar, with no MMC heap to build.

use std::sync::Arc;

use anyhow::{Result, anyhow, bail};
use metamodelica::List;

use openmodelica_frontend_types::Values;

use super::SigTy;

/// wasmtime errors carry their own (re-exported) `anyhow`, which does not unify
/// with ours under the feature set we build with; flatten via the message.
fn wt<T>(r: std::result::Result<T, wasmtime::Error>) -> Result<T> {
    r.map_err(|e| anyhow!("{e:?}"))
}

/// Parsed `.wasm.sig` sidecar: scalar types of the main function's inputs and
/// outputs.
struct Sig {
    inputs: Vec<SigTy>,
    outputs: Vec<SigTy>,
}

fn read_sig(path: &str) -> Result<Sig> {
    let text = std::fs::read_to_string(path)?;
    let mut lines = text.lines();
    let parse = |line: Option<&str>| -> Result<Vec<SigTy>> {
        line.unwrap_or("").chars().map(SigTy::from_code).collect()
    };
    let inputs = parse(lines.next())?;
    let outputs = parse(lines.next())?;
    Ok(Sig { inputs, outputs })
}

/// Register the host-imported math builtins (module `"env"`), matching
/// `super::BUILTINS` one-for-one.
fn add_host_builtins(linker: &mut wasmtime::Linker<()>) -> Result<()> {
    macro_rules! f1 {
        ($name:literal, $f:expr) => {
            wt(linker.func_wrap("env", $name, |x: f64| -> f64 { ($f)(x) }))?;
        };
    }
    macro_rules! f2 {
        ($name:literal, $f:expr) => {
            wt(linker.func_wrap("env", $name, |x: f64, y: f64| -> f64 { ($f)(x, y) }))?;
        };
    }
    f2!("pow", f64::powf);
    f2!("atan2", f64::atan2);
    f1!("sin", f64::sin);
    f1!("cos", f64::cos);
    f1!("tan", f64::tan);
    f1!("asin", f64::asin);
    f1!("acos", f64::acos);
    f1!("atan", f64::atan);
    f1!("sinh", f64::sinh);
    f1!("cosh", f64::cosh);
    f1!("tanh", f64::tanh);
    f1!("exp", f64::exp);
    f1!("log", f64::ln);
    f1!("log10", f64::log10);
    Ok(())
}

/// Extract a numeric argument as an `f64`, accepting any scalar `Values.Value`.
fn value_as_f64(v: &Values::Value) -> Result<f64> {
    Ok(match v {
        Values::Value::REAL { real } => real.into_inner(),
        Values::Value::INTEGER { integer } => *integer as f64,
        Values::Value::BOOL { boolean } => *boolean as i64 as f64,
        Values::Value::ENUM_LITERAL { index, .. } => *index as f64,
        other => bail!("CodegenWasmer: cannot pass {other:?} to a wasm function"),
    })
}

/// Extract a numeric argument as an `i32`, accepting any scalar `Values.Value`.
fn value_as_i32(v: &Values::Value) -> Result<i32> {
    Ok(match v {
        Values::Value::INTEGER { integer } => *integer,
        Values::Value::BOOL { boolean } => *boolean as i32,
        Values::Value::ENUM_LITERAL { index, .. } => *index,
        Values::Value::REAL { real } => real.into_inner() as i32,
        other => bail!("CodegenWasmer: cannot pass {other:?} to a wasm function"),
    })
}

pub(super) fn load_and_execute(
    file_name: &str,
    _name: &str,
    args: &Arc<List<Arc<Values::Value>>>,
) -> Result<Arc<Values::Value>> {
    let wasm_path = format!("{file_name}.wasm");
    let sig = read_sig(&format!("{file_name}.wasm.sig"))?;
    let bytes = std::fs::read(&wasm_path)?;

    let engine = wasmtime::Engine::default();
    let module = wt(wasmtime::Module::new(&engine, &bytes))?;
    let mut linker = wasmtime::Linker::new(&engine);
    add_host_builtins(&mut linker)?;
    let mut store = wasmtime::Store::new(&engine, ());
    let instance = wt(linker.instantiate(&mut store, &module))?;
    let func = instance
        .get_func(&mut store, "main")
        .ok_or_else(|| anyhow!("CodegenWasmer: module has no `main` export"))?;

    // Marshal the arguments according to the input signature.
    let argv: Vec<&Arc<Values::Value>> = (&**args).into_iter().collect();
    if argv.len() != sig.inputs.len() {
        bail!("CodegenWasmer: function expects {} arguments, got {}", sig.inputs.len(), argv.len());
    }
    let mut params: Vec<wasmtime::Val> = Vec::with_capacity(argv.len());
    for (a, ty) in argv.iter().zip(sig.inputs.iter()) {
        params.push(match ty {
            SigTy::Real => wasmtime::Val::F64(value_as_f64(a)?.to_bits()),
            SigTy::Int | SigTy::Bool => wasmtime::Val::I32(value_as_i32(a)?),
        });
    }

    // Result buffer sized to the function's declared results.
    let n_results = func.ty(&store).results().len();
    let mut results = vec![wasmtime::Val::I32(0); n_results];
    wt(func.call(&mut store, &params, &mut results))?;

    if results.len() != sig.outputs.len() {
        bail!("CodegenWasmer: wasm returned {} values but signature has {}", results.len(), sig.outputs.len());
    }

    let mut out: Vec<Arc<Values::Value>> = Vec::with_capacity(results.len());
    for (val, ty) in results.iter().zip(sig.outputs.iter()) {
        out.push(Arc::new(match ty {
            SigTy::Int => Values::Value::INTEGER {
                integer: val.i32().ok_or_else(|| anyhow!("CodegenWasmer: expected i32 result"))?,
            },
            SigTy::Bool => Values::Value::BOOL {
                boolean: val.i32().ok_or_else(|| anyhow!("CodegenWasmer: expected i32 result"))? != 0,
            },
            SigTy::Real => Values::Value::REAL {
                real: metamodelica::Real::from(val.f64().ok_or_else(|| anyhow!("CodegenWasmer: expected f64 result"))?),
            },
        }));
    }

    Ok(match out.len() {
        0 => Arc::new(Values::Value::NORETCALL),
        1 => out.pop().unwrap(),
        _ => Arc::new(Values::Value::TUPLE { valueLst: Arc::new(List::from_iter(out)) }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_encoder as we;

    /// Encode a one-function module exporting `main` with the given signature
    /// and body, write it plus its sidecar under a temp basename, and return
    /// the basename for `load_and_execute`.
    fn emit(base: &str, params: &[we::ValType], results: &[we::ValType], sig: &str, body: &[we::Instruction]) -> String {
        let mut m = we::Module::new();
        let mut types = we::TypeSection::new();
        types.ty().function(params.iter().copied(), results.iter().copied());
        m.section(&types);
        let mut funcs = we::FunctionSection::new();
        funcs.function(0);
        m.section(&funcs);
        let mut exports = we::ExportSection::new();
        exports.export("main", we::ExportKind::Func, 0);
        m.section(&exports);
        let mut code = we::CodeSection::new();
        let mut f = we::Function::new([]);
        for i in body {
            f.instruction(i);
        }
        code.function(&f);
        m.section(&code);
        let path = std::env::temp_dir().join(base);
        let path = path.to_str().unwrap().to_string();
        std::fs::write(format!("{path}.wasm"), m.finish()).unwrap();
        std::fs::write(format!("{path}.wasm.sig"), sig).unwrap();
        path
    }

    fn ival(v: &Values::Value) -> i32 {
        match v {
            Values::Value::INTEGER { integer } => *integer,
            other => panic!("expected INTEGER, got {other:?}"),
        }
    }
    fn rval(v: &Values::Value) -> f64 {
        match v {
            Values::Value::REAL { real } => real.into_inner(),
            other => panic!("expected REAL, got {other:?}"),
        }
    }

    #[test]
    fn integer_add() {
        let base = emit(
            "wjit_iadd",
            &[we::ValType::I32, we::ValType::I32],
            &[we::ValType::I32],
            "II\nI\n",
            &[we::Instruction::LocalGet(0), we::Instruction::LocalGet(1), we::Instruction::I32Add, we::Instruction::End],
        );
        let args = Arc::new(List::from_iter([
            Arc::new(Values::Value::INTEGER { integer: 3 }),
            Arc::new(Values::Value::INTEGER { integer: 4 }),
        ]));
        let r = load_and_execute(&base, "main", &args).unwrap();
        assert_eq!(ival(&r), 7);
    }

    #[test]
    fn real_scale() {
        // main(x) = x * 2.0
        let base = emit(
            "wjit_rscale",
            &[we::ValType::F64],
            &[we::ValType::F64],
            "R\nR\n",
            &[
                we::Instruction::LocalGet(0),
                we::Instruction::F64Const(2.0.into()),
                we::Instruction::F64Mul,
                we::Instruction::End,
            ],
        );
        let args = Arc::new(List::from_iter([Arc::new(Values::Value::REAL { real: metamodelica::Real::from(21.0) })]));
        let r = load_and_execute(&base, "main", &args).unwrap();
        assert_eq!(rval(&r), 42.0);
    }

    #[test]
    fn multi_output_tuple() {
        // main(x) = (x, x+1)
        let base = emit(
            "wjit_tuple",
            &[we::ValType::I32],
            &[we::ValType::I32, we::ValType::I32],
            "I\nII\n",
            &[
                we::Instruction::LocalGet(0),
                we::Instruction::LocalGet(0),
                we::Instruction::I32Const(1),
                we::Instruction::I32Add,
                we::Instruction::End,
            ],
        );
        let args = Arc::new(List::from_iter([Arc::new(Values::Value::INTEGER { integer: 41 })]));
        let r = load_and_execute(&base, "main", &args).unwrap();
        match &*r {
            Values::Value::TUPLE { valueLst } => {
                let v: Vec<_> = (&**valueLst).into_iter().collect();
                assert_eq!(v.len(), 2);
                assert_eq!(ival(&v[0]), 41);
                assert_eq!(ival(&v[1]), 42);
            }
            other => panic!("expected TUPLE, got {other:?}"),
        }
    }

    #[test]
    fn host_builtin_sin() {
        // main(x) = sin(x), importing the host builtin "sin" (index 2 in env).
        let mut m = we::Module::new();
        let mut types = we::TypeSection::new();
        types.ty().function([we::ValType::F64], [we::ValType::F64]); // type 0: sin
        types.ty().function([we::ValType::F64], [we::ValType::F64]); // type 1: main
        m.section(&types);
        let mut imports = we::ImportSection::new();
        imports.import("env", "sin", we::EntityType::Function(0));
        m.section(&imports);
        let mut funcs = we::FunctionSection::new();
        funcs.function(1);
        m.section(&funcs);
        let mut exports = we::ExportSection::new();
        exports.export("main", we::ExportKind::Func, 1); // func 0 is the import
        m.section(&exports);
        let mut code = we::CodeSection::new();
        let mut f = we::Function::new([]);
        f.instruction(&we::Instruction::LocalGet(0));
        f.instruction(&we::Instruction::Call(0));
        f.instruction(&we::Instruction::End);
        code.function(&f);
        m.section(&code);
        let path = std::env::temp_dir().join("wjit_sin");
        let path = path.to_str().unwrap().to_string();
        std::fs::write(format!("{path}.wasm"), m.finish()).unwrap();
        std::fs::write(format!("{path}.wasm.sig"), "R\nR\n").unwrap();

        let args = Arc::new(List::from_iter([Arc::new(Values::Value::REAL {
            real: metamodelica::Real::from(std::f64::consts::FRAC_PI_2),
        })]));
        let r = load_and_execute(&path, "main", &args).unwrap();
        assert!((rval(&r) - 1.0).abs() < 1e-12);
    }
}
