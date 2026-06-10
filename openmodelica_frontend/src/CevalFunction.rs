// Auto-generated from MetaModelica source
/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF AGPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GNU AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium)
 * Public License (OSMC-PL) are obtained from OSMC, either from the above
 * address, from the URLs:
 * http://www.openmodelica.org or
 * https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica,
 * and in the OpenModelica distribution.
 *
 * GNU AGPL version 3 is obtained from:
 * https://www.gnu.org/licenses/licenses.html#GPL
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::Ceval;
use crate::FGraph;
use crate::FNode;
use crate::Lookup;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Graph;
use openmodelica_util::Lapack;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// Jump table for CevalFunction:
// [TYPE]  Types.
// [EVAL]  Constant evaluation functions.
// [EENV]  Environment extension functions (add variables).
// [MENV]  Environment manipulation functions (set and get variables).
// [DEPS]  Function variable dependency handling.
// [EOPT]  Expression optimization functions.
// public imports
// protected imports
// [TYPE]  Types
pub type FunctionVar = (Arc<DAE::Element>, Option<Arc<Values::Value>>);

// LoopControl is used to control the functions behaviour in different
// situations. All evaluation functions returns a LoopControl variable that
// tells the caller whether it should continue evaluating or not.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum LoopControl {
    /// Continue to the next statement.
    NEXT,
    /// Exit the current loop.
    BREAK,
    /// Exit the function.
    RETURN,
}
impl metamodelica::gc::MMTrace for LoopControl {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            LoopControl::NEXT => Ok(()),
            LoopControl::BREAK => Ok(()),
            LoopControl::RETURN => Ok(()),
        }
    }
}
impl Default for LoopControl {
    fn default() -> Self { Self::NEXT }
}
pub use self::LoopControl::{NEXT,BREAK,RETURN};

// [EVAL]  Constant evaluation functions.
pub fn evaluate(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFunction: DAE::Function, mut inFunctionArguments: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outResult: Arc<Values::Value>;
    (outCache, outResult) = 'mc: {
        let __mc_input = inFunction.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Function::FUNCTION { path: p, functions: Deref @ metamodelica::List::Cons { head: func, tail: _ }, type_: ty, partialPrefix: false, source: src, .. } => {
                    let mut result: Arc<Values::Value>;
                    let mut func_name: ArcStr;
                    let mut cache: FCore::Cache;
                    func_name = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
                    (cache, result) = evaluateFunctionDefinition(inCache.clone(), inEnv.clone(), (func_name.clone()).clone(), func.clone(), ty.clone(), inFunctionArguments.clone(), src.clone())?;
                    Ok((cache.clone(), result.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Function::FUNCTION { path: p, functions: Deref @ metamodelica::List::Cons { head: _, tail: _ }, partialPrefix, .. } => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CevalFunction.evaluate failed for function: ")); __mm_s.push_str(&*if (partialPrefix.clone()) {literal!("partial ")} else {literal!("")}); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outResult))
}

fn evaluateFunctionDefinition(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFuncName: ArcStr, mut inFunc: DAE::FunctionDefinition, mut inFuncType: Arc<DAE::Type>, mut inFuncArgs: Arc<metamodelica::List<Arc<Values::Value>>>, mut inSource: Arc<DAE::ElementSource>) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outResult: Arc<Values::Value>;
    (outCache, outResult) = 'mc: {
        let __mc_input = inFunc.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::FunctionDefinition::FUNCTION_DEF { body: mut body } = __mc_input.clone() else { bail!("nomatch") };
            let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut output_vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut func_params: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>;
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut return_values: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut return_value: Arc<Values::Value>;
            let mut body = body.clone();
            (vars, body) = List::splitOnFirstMatch(body.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isNotVar, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
            vars = List::map(vars.clone(), (std::sync::Arc::new(removeSelfReferentialDims) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<DAE::Element>> + 'static>))?;
            output_vars = List::filterOnTrue(vars.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isOutputVar, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
            func_params = pairFuncParamsWithArgs(vars.clone(), inFuncArgs.clone())?;
            func_params = sortFunctionVarsByDependency(func_params.clone(), inSource.clone())?;
            (cache, env) = setupFunctionEnvironment(inCache.clone(), inEnv.clone(), (inFuncName.clone()).clone(), func_params.clone())?;
            (cache, env, _) = evaluateElements(body.clone(), cache.clone(), env.clone(), crate::CevalFunction::LoopControl::NEXT)?;
            return_values = List::map1(output_vars.clone(), (std::sync::Arc::new(getFunctionReturnValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, FCore::Graph) -> Result<Arc<Values::Value>> + 'static>), env.clone())?;
            return_value = boxReturnValue(return_values.clone());
            Ok((cache.clone(), return_value.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::FunctionDefinition::FUNCTION_EXT { body: mut body, externalDecl: DAE::ExternalDecl { name: mut ext_fun_name, args: ref ext_fun_args, .. } } = __mc_input.clone() else { bail!("nomatch") };
            let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut output_vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut func_params: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>;
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut return_values: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut return_value: Arc<Values::Value>;
            (vars, _) = List::splitOnFirstMatch(body.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isNotVar, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
            vars = List::map(vars.clone(), (std::sync::Arc::new(removeSelfReferentialDims) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<DAE::Element>> + 'static>))?;
            output_vars = List::filterOnTrue(vars.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::isOutputVar, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
            func_params = pairFuncParamsWithArgs(vars.clone(), inFuncArgs.clone())?;
            func_params = sortFunctionVarsByDependency(func_params.clone(), inSource.clone())?;
            (cache, env) = setupFunctionEnvironment(inCache.clone(), inEnv.clone(), (inFuncName.clone()).clone(), func_params.clone())?;
            (cache, env) = evaluateExternalFunc((ext_fun_name.clone()).clone(), ext_fun_args.clone(), cache.clone(), env.clone())?;
            return_values = List::map1(output_vars.clone(), (std::sync::Arc::new(getFunctionReturnValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, FCore::Graph) -> Result<Arc<Values::Value>> + 'static>), env.clone())?;
            return_value = boxReturnValue(return_values.clone());
            Ok((cache.clone(), return_value.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- CevalFunction.evaluateFunction failed.\n")).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outResult))
}

fn pairFuncParamsWithArgs(mut inElements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>> {
    let mut outFunctionVars: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>;
    outFunctionVars = (::match_deref::match_deref! { match &((inElements.clone(), inValues.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT { .. }, .. }, tail: _ }, Deref @ metamodelica::List::Nil) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("- CevalFunction.pairFuncParamsWithArgs failed because of too few input arguments.\n")).clone())?;
            bail!("fail")
        },
        (Deref @ metamodelica::List::Cons { head: var @ Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT { .. }, .. }, tail: rest_vars }, Deref @ metamodelica::List::Cons { head: val, tail: rest_vals }) => {
            let mut params: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>;
            params = pairFuncParamsWithArgs(rest_vars.clone(), rest_vals.clone())?;
            metamodelica::cons((var.clone(), Some(val.clone())), params.clone())
        },
        (Deref @ metamodelica::List::Cons { head: var, tail: rest_vars }, _) => {
            let mut params: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>;
            params = pairFuncParamsWithArgs(rest_vars.clone(), inValues.clone())?;
            metamodelica::cons((var.clone(), None), params.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outFunctionVars)
}

fn removeSelfReferentialDims(mut inElement: Arc<DAE::Element>) -> Result<Arc<DAE::Element>> {
    let mut outElement: Arc<DAE::Element>;
    outElement = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { componentRef: cref @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, kind: vk, direction: vd, parallelism: vp, protection: vv, ty, binding: bind, dims, connectorType: ct, source: es, variableAttributesOption: va, comment: cmt, innerOuter: io, encrypted: e } => {
            let mut dims = (*dims).clone();
            dims = List::map1(dims.clone(), (std::sync::Arc::new(fnptr!(removeSelfReferentialDim, Arc<DAE::Dimension>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>, ArcStr) -> Result<Arc<DAE::Dimension>> + 'static>), (name.clone()).clone())?;
            Arc::new(DAE::Element::VAR { componentRef: cref.clone(), kind: vk.clone(), direction: vd.clone(), parallelism: vp.clone(), protection: vv.clone(), ty: ty.clone(), binding: bind.clone(), dims: dims.clone(), connectorType: ct.clone(), source: es.clone(), variableAttributesOption: va.clone(), comment: cmt.clone(), innerOuter: io.clone(), encrypted: e.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElement)
}

fn removeSelfReferentialDim(mut inDim: Arc<DAE::Dimension>, mut inName: ArcStr) -> Arc<DAE::Dimension> {
    let mut outDim: Arc<DAE::Dimension>;
    outDim = 'mc: {
        let __mc_input = inDim.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Dimension::DIM_EXP { exp } => {
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    crefs = Expression::extractCrefsFromExp(exp.clone())?;
                    let true = (List::isMemberOnTrue((inName.clone()).clone(), crefs.clone(), (std::sync::Arc::new(fnptr!(isCrefNamed, ArcStr, Arc<DAE::ComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    Ok(openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inDim.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outDim
}

fn isCrefNamed(mut inName: ArcStr, mut inCref: Arc<DAE::ComponentRef>) -> bool {
    let mut outIsNamed: bool;
    outIsNamed = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. } => {
            stringEq((inName.clone()).clone(), (name.clone()).clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsNamed
}

fn evaluateExtInputArg(mut inArgument: DAE::ExtArg, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(Arc<Values::Value>, FCore::Cache)> {
    let mut outValue: Arc<Values::Value>;
    let mut outCache: FCore::Cache;
    (outValue, outCache) = 'mc: {
        let __mc_input = (inArgument.clone(), inCache.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::ExtArg::EXTARG { componentRef: ref cref, type_: ref ty, .. }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut val: Arc<Values::Value>;
            val = getVariableValue(cref.clone(), ty.clone(), inEnv.clone())?;
            Ok((val.clone(), inCache.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::ExtArg::EXTARGEXP { exp: mut exp, .. }, mut cache) = __mc_input.clone() else { bail!("nomatch") };
            let mut val: Arc<Values::Value>;
            (cache, val) = cevalExp(exp.clone(), cache.clone(), inEnv.clone())?;
            Ok((val.clone(), cache.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::ExtArg::EXTARGSIZE { componentRef: ref cref, exp: mut exp, .. }, mut cache) = __mc_input.clone() else { bail!("nomatch") };
            let mut val: Arc<Values::Value>;
            let mut exp = exp.clone();
            exp = Arc::new(DAE::Exp::SIZE { exp: Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone() }), sz: Some(exp.clone()) });
            (cache, val) = cevalExp(exp.clone(), cache.clone(), inEnv.clone())?;
            Ok((val.clone(), cache.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut err_str: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            err_str = (DAEDump::dumpExtArgStr(inArgument.clone())?).clone();
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CevalFunction.evaluateExtInputArg failed on ")); __mm_s.push_str(&*err_str.clone()); ArcStr::from(__mm_s) }).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outValue, outCache))
}

fn evaluateExtIntArg(mut inArg: DAE::ExtArg, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(i32, FCore::Cache)> {
    let mut outValue: i32;
    let mut outCache: FCore::Cache;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(evaluateExtInputArg(inArg.clone(), inCache.clone(), inEnv.clone())?) {
        (Deref @ Values::Value::INTEGER { integer: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outValue = __pa0.clone();
    outCache = __pa1.clone();
    Ok((outValue, outCache))
}

fn evaluateExtRealArg(mut inArg: DAE::ExtArg, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(metamodelica::Real, FCore::Cache)> {
    let mut outValue: metamodelica::Real;
    let mut outCache: FCore::Cache;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(evaluateExtInputArg(inArg.clone(), inCache.clone(), inEnv.clone())?) {
        (Deref @ Values::Value::REAL { real: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outValue = __pa0.clone();
    outCache = __pa1.clone();
    Ok((outValue, outCache))
}

fn evaluateExtStringArg(mut inArg: DAE::ExtArg, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(ArcStr, FCore::Cache)> {
    let mut outValue: ArcStr;
    let mut outCache: FCore::Cache;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(evaluateExtInputArg(inArg.clone(), inCache.clone(), inEnv.clone())?) {
        (Deref @ Values::Value::STRING { string: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    outValue = __pa0.clone();
    outCache = __pa1.clone();
    Ok((outValue, outCache))
}

fn evaluateExtIntArrayArg(mut inArg: DAE::ExtArg, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(Arc<metamodelica::List<i32>>, FCore::Cache)> {
    let mut outValue: Arc<metamodelica::List<i32>>;
    let mut outCache: FCore::Cache;
    let mut val: Arc<Values::Value>;
    (val, outCache) = evaluateExtInputArg(inArg.clone(), inCache.clone(), inEnv.clone())?;
    outValue = ValuesUtil::arrayValueInts(val.clone())?;
    Ok((outValue, outCache))
}

fn evaluateExtRealArrayArg(mut inArg: DAE::ExtArg, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(Arc<metamodelica::List<metamodelica::Real>>, FCore::Cache)> {
    let mut outValue: Arc<metamodelica::List<metamodelica::Real>>;
    let mut outCache: FCore::Cache;
    let mut val: Arc<Values::Value>;
    (val, outCache) = evaluateExtInputArg(inArg.clone(), inCache.clone(), inEnv.clone())?;
    outValue = ValuesUtil::arrayValueReals(val.clone())?;
    Ok((outValue, outCache))
}

fn evaluateExtRealMatrixArg(mut inArg: DAE::ExtArg, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>, FCore::Cache)> {
    let mut outValue: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut outCache: FCore::Cache;
    let mut val: Arc<Values::Value>;
    (val, outCache) = evaluateExtInputArg(inArg.clone(), inCache.clone(), inEnv.clone())?;
    outValue = ValuesUtil::matrixValueReals(val.clone())?;
    Ok((outValue, outCache))
}

fn evaluateExtOutputArg(mut inArg: DAE::ExtArg) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef>;
    let DAE::EXTARG { componentRef: __pa0, .. } = (inArg.clone()) else { bail!("pattern mismatch") };
    outCref = __pa0.clone();
    Ok(outCref)
}

fn assignExtOutputs(mut inArgs: Arc<metamodelica::List<DAE::ExtArg>>, mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inArgs.clone(), inValues.clone(), inCache.clone(), inEnv.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
            return Ok((inCache.clone(), inEnv.clone()))
        },
        (Deref @ metamodelica::List::Cons { head: arg, tail: rest_args }, Deref @ metamodelica::List::Cons { head: val, tail: rest_vals }, cache, env) => {
            let mut cr: Arc<DAE::ComponentRef>;
            let mut val = (*val).clone();
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            cr = evaluateExtOutputArg(arg.clone())?;
            val = unliftExtOutputValue(cr.clone(), val.clone(), env.clone());
            (cache, env) = assignVariable(cr.clone(), val.clone(), cache.clone(), env.clone())?;
            { (inArgs, inValues, inCache, inEnv) = (rest_args.clone(), rest_vals.clone(), cache.clone(), env.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn unliftExtOutputValue(mut inCref: Arc<DAE::ComponentRef>, mut inValue: Arc<Values::Value>, mut inEnv: FCore::Graph) -> Arc<Values::Value> {
    let mut outValue: Arc<Values::Value>;
    outValue = 'mc: {
        let __mc_input = inValue.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: vals @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { .. }, tail: _ }, dimLst: Deref @ metamodelica::List::Cons { head: dim, tail: _ } } => {
                    let mut ty: Arc<DAE::Type>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut vals = (*vals).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getVariableTypeAndBinding(inCref.clone(), inEnv.clone())?) {
                        (Deref @ DAE::Type::T_ARRAY { ty: __pa0, dims: __pa1 }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    ty = __pa0.clone();
                    dims = __pa1.clone();
                    let false = (Types::isNonscalarArray(ty.clone(), dims.clone())) else { bail!("pattern mismatch") };
                    vals = List::map(vals.clone(), (std::sync::Arc::new(ValuesUtil::arrayScalar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<Values::Value>> + 'static>))?;
                    Ok(Arc::new(Values::Value::ARRAY { valueLst: vals.clone(), dimLst: list![dim.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inValue.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outValue
}

fn evaluateExternalFunc(mut inFuncName: ArcStr, mut inFuncArgs: Arc<metamodelica::List<DAE::ExtArg>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (::match_deref::match_deref! { match &((inFuncName.clone(), inFuncArgs.clone(), inCache.clone(), inEnv.clone())) {
        (Deref @ "dgeev", Deref @ metamodelica::List::Cons { head: arg_JOBVL, tail: Deref @ metamodelica::List::Cons { head: arg_JOBVR, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_WR, tail: Deref @ metamodelica::List::Cons { head: arg_WI, tail: Deref @ metamodelica::List::Cons { head: arg_VL, tail: Deref @ metamodelica::List::Cons { head: arg_LDVL, tail: Deref @ metamodelica::List::Cons { head: arg_VR, tail: Deref @ metamodelica::List::Cons { head: arg_LDVR, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_WI: Arc<Values::Value>;
            let mut val_WORK: Arc<Values::Value>;
            let mut val_WR: Arc<Values::Value>;
            let mut val_A: Arc<Values::Value>;
            let mut val_VL: Arc<Values::Value>;
            let mut val_VR: Arc<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDVL: i32;
            let mut LDVR: i32;
            let mut LWORK: i32;
            let mut N: i32;
            let mut JOBVL: ArcStr;
            let mut JOBVR: ArcStr;
            let mut WI: Arc<metamodelica::List<metamodelica::Real>>;
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
            let mut WR: Arc<metamodelica::List<metamodelica::Real>>;
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut VL: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut VR: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (JOBVL, cache) = evaluateExtStringArg(arg_JOBVL.clone(), cache.clone(), env.clone())?;
            (JOBVR, cache) = evaluateExtStringArg(arg_JOBVR.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (LDVL, cache) = evaluateExtIntArg(arg_LDVL.clone(), cache.clone(), env.clone())?;
            (LDVR, cache) = evaluateExtIntArg(arg_LDVR.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (A, WR, WI, VL, VR, WORK, INFO) = Lapack::dgeev((JOBVL.clone()).clone(), (JOBVR.clone()).clone(), N.clone(), A.clone(), LDA.clone(), LDVL.clone(), LDVR.clone(), WORK.clone(), LWORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_WR = ValuesMake::makeRealArray(WR.clone())?;
            val_WI = ValuesMake::makeRealArray(WI.clone())?;
            val_VL = ValuesMake::makeRealMatrix(VL.clone())?;
            val_VR = ValuesMake::makeRealMatrix(VR.clone())?;
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_WR.clone(), arg_WI.clone(), arg_VL.clone(), arg_VR.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_WR.clone(), val_WI.clone(), val_VL.clone(), val_VR.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgegv", Deref @ metamodelica::List::Cons { head: arg_JOBVL, tail: Deref @ metamodelica::List::Cons { head: arg_JOBVR, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_ALPHAR, tail: Deref @ metamodelica::List::Cons { head: arg_ALPHAI, tail: Deref @ metamodelica::List::Cons { head: arg_BETA, tail: Deref @ metamodelica::List::Cons { head: arg_VL, tail: Deref @ metamodelica::List::Cons { head: arg_LDVL, tail: Deref @ metamodelica::List::Cons { head: arg_VR, tail: Deref @ metamodelica::List::Cons { head: arg_LDVR, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_ALPHAI: Arc<Values::Value>;
            let mut val_ALPHAR: Arc<Values::Value>;
            let mut val_BETA: Arc<Values::Value>;
            let mut val_WORK: Arc<Values::Value>;
            let mut val_VL: Arc<Values::Value>;
            let mut val_VR: Arc<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut LDVL: i32;
            let mut LDVR: i32;
            let mut LWORK: i32;
            let mut N: i32;
            let mut JOBVL: ArcStr;
            let mut JOBVR: ArcStr;
            let mut ALPHAI: Arc<metamodelica::List<metamodelica::Real>>;
            let mut ALPHAR: Arc<metamodelica::List<metamodelica::Real>>;
            let mut BETA: Arc<metamodelica::List<metamodelica::Real>>;
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut VL: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut VR: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (JOBVL, cache) = evaluateExtStringArg(arg_JOBVL.clone(), cache.clone(), env.clone())?;
            (JOBVR, cache) = evaluateExtStringArg(arg_JOBVR.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (LDVL, cache) = evaluateExtIntArg(arg_LDVL.clone(), cache.clone(), env.clone())?;
            (LDVR, cache) = evaluateExtIntArg(arg_LDVR.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (ALPHAR, ALPHAI, BETA, VL, VR, WORK, INFO) = Lapack::dgegv((JOBVL.clone()).clone(), (JOBVR.clone()).clone(), N.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), LDVL.clone(), LDVR.clone(), WORK.clone(), LWORK.clone());
            val_ALPHAR = ValuesMake::makeRealArray(ALPHAR.clone())?;
            val_ALPHAI = ValuesMake::makeRealArray(ALPHAI.clone())?;
            val_BETA = ValuesMake::makeRealArray(BETA.clone())?;
            val_VL = ValuesMake::makeRealMatrix(VL.clone())?;
            val_VR = ValuesMake::makeRealMatrix(VR.clone())?;
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_ALPHAR.clone(), arg_ALPHAI.clone(), arg_BETA.clone(), arg_VL.clone(), arg_VR.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_ALPHAR.clone(), val_ALPHAI.clone(), val_BETA.clone(), val_VL.clone(), val_VR.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgels", Deref @ metamodelica::List::Cons { head: arg_TRANS, tail: Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_WORK: Arc<Values::Value>;
            let mut val_A: Arc<Values::Value>;
            let mut val_B: Arc<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut LWORK: i32;
            let mut M: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut TRANS: ArcStr;
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (TRANS, cache) = evaluateExtStringArg(arg_TRANS.clone(), cache.clone(), env.clone())?;
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (A, B, WORK, INFO) = Lapack::dgels((TRANS.clone()).clone(), M.clone(), N.clone(), NRHS.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), WORK.clone(), LWORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_B.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_B.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgelsx", Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_JPVT, tail: Deref @ metamodelica::List::Cons { head: arg_RCOND, tail: Deref @ metamodelica::List::Cons { head: arg_RANK, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_RANK: Arc<Values::Value>;
            let mut val_JPVT: Arc<Values::Value>;
            let mut val_A: Arc<Values::Value>;
            let mut val_B: Arc<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut M: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut RANK: i32;
            let mut RCOND: metamodelica::Real;
            let mut JPVT: Arc<metamodelica::List<i32>>;
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (JPVT, cache) = evaluateExtIntArrayArg(arg_JPVT.clone(), cache.clone(), env.clone())?;
            (RCOND, cache) = evaluateExtRealArg(arg_RCOND.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (A, B, JPVT, RANK, INFO) = Lapack::dgelsx(M.clone(), N.clone(), NRHS.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), JPVT.clone(), RCOND.clone(), WORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_JPVT = ValuesMake::makeIntArray(JPVT.clone())?;
            val_RANK = ValuesMake::makeInteger(RANK.clone());
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_B.clone(), arg_JPVT.clone(), arg_RANK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_B.clone(), val_JPVT.clone(), val_RANK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgelsx", Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_JPVT, tail: Deref @ metamodelica::List::Cons { head: arg_RCOND, tail: Deref @ metamodelica::List::Cons { head: arg_RANK, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_RANK: Arc<Values::Value>;
            let mut val_JPVT: Arc<Values::Value>;
            let mut val_A: Arc<Values::Value>;
            let mut val_B: Arc<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut M: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut RANK: i32;
            let mut RCOND: metamodelica::Real;
            let mut JPVT: Arc<metamodelica::List<i32>>;
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (JPVT, cache) = evaluateExtIntArrayArg(arg_JPVT.clone(), cache.clone(), env.clone())?;
            (RCOND, cache) = evaluateExtRealArg(arg_RCOND.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (A, B, JPVT, RANK, INFO) = Lapack::dgelsx(M.clone(), N.clone(), NRHS.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), JPVT.clone(), RCOND.clone(), WORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_JPVT = ValuesMake::makeIntArray(JPVT.clone())?;
            val_RANK = ValuesMake::makeInteger(RANK.clone());
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_B.clone(), arg_JPVT.clone(), arg_RANK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_B.clone(), val_JPVT.clone(), val_RANK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgelsy", Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_JPVT, tail: Deref @ metamodelica::List::Cons { head: arg_RCOND, tail: Deref @ metamodelica::List::Cons { head: arg_RANK, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_RANK: Arc<Values::Value>;
            let mut val_JPVT: Arc<Values::Value>;
            let mut val_WORK: Arc<Values::Value>;
            let mut val_A: Arc<Values::Value>;
            let mut val_B: Arc<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut LWORK: i32;
            let mut M: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut RANK: i32;
            let mut RCOND: metamodelica::Real;
            let mut JPVT: Arc<metamodelica::List<i32>>;
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (JPVT, cache) = evaluateExtIntArrayArg(arg_JPVT.clone(), cache.clone(), env.clone())?;
            (RCOND, cache) = evaluateExtRealArg(arg_RCOND.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (A, B, JPVT, RANK, WORK, INFO) = Lapack::dgelsy(M.clone(), N.clone(), NRHS.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), JPVT.clone(), RCOND.clone(), WORK.clone(), LWORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_JPVT = ValuesMake::makeIntArray(JPVT.clone())?;
            val_RANK = ValuesMake::makeInteger(RANK.clone());
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_B.clone(), arg_JPVT.clone(), arg_RANK.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_B.clone(), val_JPVT.clone(), val_RANK.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgesv", Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_IPIV, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_IPIV: Arc<Values::Value>;
            let mut val_A: Arc<Values::Value>;
            let mut val_B: Arc<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut IPIV: Arc<metamodelica::List<i32>>;
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (A, IPIV, B, INFO) = Lapack::dgesv(N.clone(), NRHS.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_IPIV = ValuesMake::makeIntArray(IPIV.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_IPIV.clone(), arg_B.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_IPIV.clone(), val_B.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgglse", Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_P, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_C, tail: Deref @ metamodelica::List::Cons { head: arg_D, tail: Deref @ metamodelica::List::Cons { head: arg_X, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_C: Arc<Values::Value>;
            let mut val_D: Arc<Values::Value>;
            let mut val_WORK: Arc<Values::Value>;
            let mut val_X: Arc<Values::Value>;
            let mut val_A: Arc<Values::Value>;
            let mut val_B: Arc<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut LWORK: i32;
            let mut M: i32;
            let mut N: i32;
            let mut P: i32;
            let mut C: Arc<metamodelica::List<metamodelica::Real>>;
            let mut D: Arc<metamodelica::List<metamodelica::Real>>;
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
            let mut X: Arc<metamodelica::List<metamodelica::Real>>;
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (P, cache) = evaluateExtIntArg(arg_P.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (C, cache) = evaluateExtRealArrayArg(arg_C.clone(), cache.clone(), env.clone())?;
            (D, cache) = evaluateExtRealArrayArg(arg_D.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (A, B, C, D, X, WORK, INFO) = Lapack::dgglse(M.clone(), N.clone(), P.clone(), A.clone(), LDA.clone(), B.clone(), LDB.clone(), C.clone(), D.clone(), WORK.clone(), LWORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_C = ValuesMake::makeRealArray(C.clone())?;
            val_D = ValuesMake::makeRealArray(D.clone())?;
            val_X = ValuesMake::makeRealArray(X.clone())?;
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_B.clone(), arg_C.clone(), arg_D.clone(), arg_X.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_B.clone(), val_C.clone(), val_D.clone(), val_X.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgtsv", Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_DL, tail: Deref @ metamodelica::List::Cons { head: arg_D, tail: Deref @ metamodelica::List::Cons { head: arg_DU, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_D: Arc<Values::Value>;
            let mut val_DL: Arc<Values::Value>;
            let mut val_DU: Arc<Values::Value>;
            let mut val_B: Arc<Values::Value>;
            let mut INFO: i32;
            let mut LDB: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut D: Arc<metamodelica::List<metamodelica::Real>>;
            let mut DL: Arc<metamodelica::List<metamodelica::Real>>;
            let mut DU: Arc<metamodelica::List<metamodelica::Real>>;
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (DL, cache) = evaluateExtRealArrayArg(arg_DL.clone(), cache.clone(), env.clone())?;
            (D, cache) = evaluateExtRealArrayArg(arg_D.clone(), cache.clone(), env.clone())?;
            (DU, cache) = evaluateExtRealArrayArg(arg_DU.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (DL, D, DU, B, INFO) = Lapack::dgtsv(N.clone(), NRHS.clone(), DL.clone(), D.clone(), DU.clone(), B.clone(), LDB.clone());
            val_DL = ValuesMake::makeRealArray(DL.clone())?;
            val_D = ValuesMake::makeRealArray(D.clone())?;
            val_DU = ValuesMake::makeRealArray(DU.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_DL.clone(), arg_D.clone(), arg_DU.clone(), arg_B.clone(), arg_INFO.clone()];
            val_out = list![val_DL.clone(), val_D.clone(), val_DU.clone(), val_B.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgbsv", Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_KL, tail: Deref @ metamodelica::List::Cons { head: arg_KU, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_AB, tail: Deref @ metamodelica::List::Cons { head: arg_LDAB, tail: Deref @ metamodelica::List::Cons { head: arg_IPIV, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_IPIV: Arc<Values::Value>;
            let mut val_AB: Arc<Values::Value>;
            let mut val_B: Arc<Values::Value>;
            let mut INFO: i32;
            let mut KL: i32;
            let mut KU: i32;
            let mut LDAB: i32;
            let mut LDB: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut IPIV: Arc<metamodelica::List<i32>>;
            let mut AB: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (KL, cache) = evaluateExtIntArg(arg_KL.clone(), cache.clone(), env.clone())?;
            (KU, cache) = evaluateExtIntArg(arg_KU.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (AB, cache) = evaluateExtRealMatrixArg(arg_AB.clone(), cache.clone(), env.clone())?;
            (LDAB, cache) = evaluateExtIntArg(arg_LDAB.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (AB, IPIV, B, INFO) = Lapack::dgbsv(N.clone(), KL.clone(), KU.clone(), NRHS.clone(), AB.clone(), LDAB.clone(), B.clone(), LDB.clone());
            val_AB = ValuesMake::makeRealMatrix(AB.clone())?;
            val_IPIV = ValuesMake::makeIntArray(IPIV.clone())?;
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_AB.clone(), arg_IPIV.clone(), arg_B.clone(), arg_INFO.clone()];
            val_out = list![val_AB.clone(), val_IPIV.clone(), val_B.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgesvd", Deref @ metamodelica::List::Cons { head: arg_JOBU, tail: Deref @ metamodelica::List::Cons { head: arg_JOBVT, tail: Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_S, tail: Deref @ metamodelica::List::Cons { head: arg_U, tail: Deref @ metamodelica::List::Cons { head: arg_LDU, tail: Deref @ metamodelica::List::Cons { head: arg_VT, tail: Deref @ metamodelica::List::Cons { head: arg_LDVT, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_WORK: Arc<Values::Value>;
            let mut val_A: Arc<Values::Value>;
            let mut val_S: Arc<Values::Value>;
            let mut val_U: Arc<Values::Value>;
            let mut val_VT: Arc<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDU: i32;
            let mut LDVT: i32;
            let mut LWORK: i32;
            let mut M: i32;
            let mut N: i32;
            let mut JOBU: ArcStr;
            let mut JOBVT: ArcStr;
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
            let mut S: Arc<metamodelica::List<metamodelica::Real>>;
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut U: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut VT: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (JOBU, cache) = evaluateExtStringArg(arg_JOBU.clone(), cache.clone(), env.clone())?;
            (JOBVT, cache) = evaluateExtStringArg(arg_JOBVT.clone(), cache.clone(), env.clone())?;
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (LDU, cache) = evaluateExtIntArg(arg_LDU.clone(), cache.clone(), env.clone())?;
            (LDVT, cache) = evaluateExtIntArg(arg_LDVT.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (A, S, U, VT, WORK, INFO) = Lapack::dgesvd((JOBU.clone()).clone(), (JOBVT.clone()).clone(), M.clone(), N.clone(), A.clone(), LDA.clone(), LDU.clone(), LDVT.clone(), WORK.clone(), LWORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_S = ValuesMake::makeRealArray(S.clone())?;
            val_U = ValuesMake::makeRealMatrix(U.clone())?;
            val_VT = ValuesMake::makeRealMatrix(VT.clone())?;
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_S.clone(), arg_U.clone(), arg_VT.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_S.clone(), val_U.clone(), val_VT.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgetrf", Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_IPIV, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_IPIV: Arc<Values::Value>;
            let mut val_A: Arc<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut M: i32;
            let mut N: i32;
            let mut IPIV: Arc<metamodelica::List<i32>>;
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (A, IPIV, INFO) = Lapack::dgetrf(M.clone(), N.clone(), A.clone(), LDA.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_IPIV = ValuesMake::makeIntArray(IPIV.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_IPIV.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_IPIV.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgetrs", Deref @ metamodelica::List::Cons { head: arg_TRANS, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_NRHS, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_IPIV, tail: Deref @ metamodelica::List::Cons { head: arg_B, tail: Deref @ metamodelica::List::Cons { head: arg_LDB, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_B: Arc<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut TRANS: ArcStr;
            let mut IPIV: Arc<metamodelica::List<i32>>;
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut B: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (TRANS, cache) = evaluateExtStringArg(arg_TRANS.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(arg_NRHS.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (IPIV, cache) = evaluateExtIntArrayArg(arg_IPIV.clone(), cache.clone(), env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(arg_B.clone(), cache.clone(), env.clone())?;
            (LDB, cache) = evaluateExtIntArg(arg_LDB.clone(), cache.clone(), env.clone())?;
            (B, INFO) = Lapack::dgetrs((TRANS.clone()).clone(), N.clone(), NRHS.clone(), A.clone(), LDA.clone(), IPIV.clone(), B.clone(), LDB.clone());
            val_B = ValuesMake::makeRealMatrix(B.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_B.clone(), arg_INFO.clone()];
            val_out = list![val_B.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgetri", Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_IPIV, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_WORK: Arc<Values::Value>;
            let mut val_A: Arc<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LWORK: i32;
            let mut N: i32;
            let mut IPIV: Arc<metamodelica::List<i32>>;
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (IPIV, cache) = evaluateExtIntArrayArg(arg_IPIV.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (A, WORK, INFO) = Lapack::dgetri(N.clone(), A.clone(), LDA.clone(), IPIV.clone(), WORK.clone(), LWORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dgeqpf", Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_JPVT, tail: Deref @ metamodelica::List::Cons { head: arg_TAU, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_JPVT: Arc<Values::Value>;
            let mut val_TAU: Arc<Values::Value>;
            let mut val_A: Arc<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut M: i32;
            let mut N: i32;
            let mut JPVT: Arc<metamodelica::List<i32>>;
            let mut TAU: Arc<metamodelica::List<metamodelica::Real>>;
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (JPVT, cache) = evaluateExtIntArrayArg(arg_JPVT.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (A, JPVT, TAU, INFO) = Lapack::dgeqpf(M.clone(), N.clone(), A.clone(), LDA.clone(), JPVT.clone(), WORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_JPVT = ValuesMake::makeIntArray(JPVT.clone())?;
            val_TAU = ValuesMake::makeRealArray(TAU.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_JPVT.clone(), arg_TAU.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_JPVT.clone(), val_TAU.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        (Deref @ "dorgqr", Deref @ metamodelica::List::Cons { head: arg_M, tail: Deref @ metamodelica::List::Cons { head: arg_N, tail: Deref @ metamodelica::List::Cons { head: arg_K, tail: Deref @ metamodelica::List::Cons { head: arg_A, tail: Deref @ metamodelica::List::Cons { head: arg_LDA, tail: Deref @ metamodelica::List::Cons { head: arg_TAU, tail: Deref @ metamodelica::List::Cons { head: arg_WORK, tail: Deref @ metamodelica::List::Cons { head: arg_LWORK, tail: Deref @ metamodelica::List::Cons { head: arg_INFO, tail: Deref @ metamodelica::List::Nil } } } } } } } } }, cache, env) => {
            let mut val_INFO: Arc<Values::Value>;
            let mut val_WORK: Arc<Values::Value>;
            let mut val_A: Arc<Values::Value>;
            let mut INFO: i32;
            let mut K: i32;
            let mut LDA: i32;
            let mut LWORK: i32;
            let mut M: i32;
            let mut N: i32;
            let mut TAU: Arc<metamodelica::List<metamodelica::Real>>;
            let mut WORK: Arc<metamodelica::List<metamodelica::Real>>;
            let mut A: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut arg_out: Arc<metamodelica::List<DAE::ExtArg>>;
            let mut val_out: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (M, cache) = evaluateExtIntArg(arg_M.clone(), cache.clone(), env.clone())?;
            (N, cache) = evaluateExtIntArg(arg_N.clone(), cache.clone(), env.clone())?;
            (K, cache) = evaluateExtIntArg(arg_K.clone(), cache.clone(), env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(arg_A.clone(), cache.clone(), env.clone())?;
            (LDA, cache) = evaluateExtIntArg(arg_LDA.clone(), cache.clone(), env.clone())?;
            (TAU, cache) = evaluateExtRealArrayArg(arg_TAU.clone(), cache.clone(), env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(arg_WORK.clone(), cache.clone(), env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(arg_LWORK.clone(), cache.clone(), env.clone())?;
            (A, WORK, INFO) = Lapack::dorgqr(M.clone(), N.clone(), K.clone(), A.clone(), LDA.clone(), TAU.clone(), WORK.clone(), LWORK.clone());
            val_A = ValuesMake::makeRealMatrix(A.clone())?;
            val_WORK = ValuesMake::makeRealArray(WORK.clone())?;
            val_INFO = ValuesMake::makeInteger(INFO.clone());
            arg_out = list![arg_A.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A.clone(), val_WORK.clone(), val_INFO.clone()];
            (cache, env) = assignExtOutputs(arg_out.clone(), val_out.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv))
}

fn evaluateElements(mut inElements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inLoopControl: LoopControl) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inElements.clone(), inLoopControl.clone())) {
        (_, LoopControl::RETURN { .. }) => {
            return Ok((inCache.clone(), inEnv.clone(), inLoopControl.clone()))
        },
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT))
        },
        (Deref @ metamodelica::List::Cons { head: elem, tail: rest_elems }, _) => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl;
            (cache, env, loop_ctrl) = evaluateElement(elem.clone(), inCache.clone(), inEnv.clone())?;
            { (inElements, inCache, inEnv, inLoopControl) = (rest_elems.clone(), cache.clone(), env.clone(), loop_ctrl.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn evaluateElement(mut inElement: Arc<DAE::Element>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl;
    (outCache, outEnv, outLoopControl) = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: sl }, .. } => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl;
            let mut sl = (*sl).clone();
            let (__pa0, (_, __pa1)) = DAEUtil::traverseDAEEquationsStmts(sl.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(optimizeExpTraverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, FCore::Graph) -> Result<(Arc<DAE::Exp>, FCore::Graph)> + 'static>), inEnv.clone()))?;
            sl = __pa0.clone();
            env = __pa1.clone();
            (cache, env, loop_ctrl) = evaluateStatements(sl.clone(), inCache.clone(), env.clone())?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateStatement(mut inStatement: Arc<DAE::Statement>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl = LoopControl::BREAK;
    (outCache, outEnv, outLoopControl) = (::match_deref::match_deref! { match &((inStatement.clone(), inCache.clone(), inEnv.clone())) {
        (Deref @ DAE::Statement::STMT_ASSIGN { exp1: lhs, exp: rhs, .. }, cache, env) => {
            let mut lhs_cref: Arc<DAE::ComponentRef>;
            let mut rhs_val: Arc<Values::Value>;
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (cache, rhs_val) = cevalExp(rhs.clone(), cache.clone(), env.clone())?;
            lhs_cref = extractLhsComponentRef(lhs.clone())?;
            (cache, env) = assignVariable(lhs_cref.clone(), rhs_val.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { .. }, _, _) => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            (cache, env) = evaluateTupleAssignStatement(inStatement.clone(), inCache.clone(), inEnv.clone())?;
            (cache.clone(), env.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        (Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs, exp: rhs, .. }, _, env) => {
            let mut cache: FCore::Cache;
            let mut lhs_cref: Arc<DAE::ComponentRef>;
            let mut rhs_val: Arc<Values::Value>;
            let mut env = (*env).clone();
            (cache, rhs_val) = cevalExp(rhs.clone(), inCache.clone(), env.clone())?;
            lhs_cref = extractLhsComponentRef(lhs.clone())?;
            (cache, env) = assignVariable(lhs_cref.clone(), rhs_val.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        (Deref @ DAE::Statement::STMT_IF { .. }, _, _) => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl;
            (cache, env, loop_ctrl) = evaluateIfStatement(inStatement.clone(), inCache.clone(), inEnv.clone())?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        (Deref @ DAE::Statement::STMT_FOR { .. }, _, _) => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl;
            (cache, env, loop_ctrl) = evaluateForStatement(inStatement.clone(), inCache.clone(), inEnv.clone())?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        (Deref @ DAE::Statement::STMT_WHILE { exp: condition, statementLst: statements, .. }, _, _) => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl;
            (cache, env, loop_ctrl) = evaluateWhileStatement(condition.clone(), statements.clone(), inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT)?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        (Deref @ DAE::Statement::STMT_ASSERT { cond: condition, .. }, _, _) => {
            let mut cache: FCore::Cache;
            let __pa0 = ::match_deref::match_deref! { match &(cevalExp(condition.clone(), inCache.clone(), inEnv.clone())?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: true }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            (cache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        (Deref @ DAE::Statement::STMT_ASSERT { cond: condition, .. }, _, _) => {
            let mut cache: FCore::Cache;
            let __pa0 = ::match_deref::match_deref! { match &(cevalExp(condition.clone(), inCache.clone(), inEnv.clone())?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: true }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            (cache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        (Deref @ DAE::Statement::STMT_NORETCALL { exp: rhs @ Deref @ DAE::Exp::CALL { expLst: exps, attr: Deref @ DAE::CallAttributes { tailCall, .. }, .. }, .. }, _, _) => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
            let mut v: Arc<Values::Value>;
            let mut vals: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut var: ArcStr = arcstr::literal!("");
            let mut vars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            (cache, vals) = cevalExpList(exps.clone(), inCache.clone(), inEnv.clone())?;
            (cache, v) = cevalExp(rhs.clone(), cache.clone(), inEnv.clone())?;
            (cache, env, outLoopControl) = (::match_deref::match_deref! { match &(tailCall.clone()) {
        DAE::TailCall::NO_TAIL { .. } => (cache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT),
        DAE::TailCall::TAIL { outVars: Deref @ metamodelica::List::Nil, .. } => (cache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::RETURN),
        DAE::TailCall::TAIL { outVars: Deref @ metamodelica::List::Cons { head: __esc_var, tail: Deref @ metamodelica::List::Nil }, .. } => {
            var = (*__esc_var).clone();
            (cache, env) = assignVariable(ComponentReference::makeUntypedCrefIdent((var.clone()).clone()), v.clone(), cache.clone(), inEnv.clone())?;
            (cache.clone(), env.clone(), crate::CevalFunction::LoopControl::RETURN)
        },
        DAE::TailCall::TAIL { outVars: __esc_vars, .. } => {
            vars = (*__esc_vars).clone();
            env = inEnv.clone();
            let __pa0 = ::match_deref::match_deref! { match &(v.clone()) {
                Deref @ Values::Value::TUPLE { valueLst: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            vals = __pa0.clone();
            for mut val in &*vals.clone() {
                let mut val = val.clone();
                let (__pa1, __pa2) = ::match_deref::match_deref! { match &(vars.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                var = __pa1.clone();
                vars = __pa2.clone();
                (cache, env) = assignVariable(ComponentReference::makeUntypedCrefIdent((var.clone()).clone()), val.clone(), cache.clone(), inEnv.clone())?;
            }
            (cache.clone(), env.clone(), crate::CevalFunction::LoopControl::RETURN)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (cache.clone(), env.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        (Deref @ DAE::Statement::STMT_RETURN { .. }, _, _) => {
            (inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::RETURN)
        },
        (Deref @ DAE::Statement::STMT_BREAK { .. }, _, _) => {
            (inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::BREAK)
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln((literal!("- CevalFunction.evaluateStatement failed for:")).clone())?;
            Debug::traceln((DAEDump::ppStatementStr(inStatement.clone())).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateStatements(mut inStatement: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl;
    (outCache, outEnv, outLoopControl) = evaluateStatements2(inStatement.clone(), inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT)?;
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateStatements2(mut inStatement: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inLoopControl: LoopControl) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inStatement.clone(), inLoopControl.clone())) {
        (_, LoopControl::BREAK { .. }) => {
            return Ok((inCache.clone(), inEnv.clone(), inLoopControl.clone()))
        },
        (_, LoopControl::RETURN { .. }) => {
            return Ok((inCache.clone(), inEnv.clone(), inLoopControl.clone()))
        },
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((inCache.clone(), inEnv.clone(), inLoopControl.clone()))
        },
        (Deref @ metamodelica::List::Cons { head: stmt, tail: rest_stmts }, LoopControl::NEXT { .. }) => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl;
            (cache, env, loop_ctrl) = evaluateStatement(stmt.clone(), inCache.clone(), inEnv.clone())?;
            { (inStatement, inCache, inEnv, inLoopControl) = (rest_stmts.clone(), cache.clone(), env.clone(), loop_ctrl.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn evaluateTupleAssignStatement(mut inStatement: Arc<DAE::Statement>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (::match_deref::match_deref! { match &((inStatement.clone(), inEnv.clone())) {
        (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: lhs_expl, exp: rhs, .. }, env) => {
            let mut rhs_vals: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut lhs_crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut cache: FCore::Cache;
            let mut env = (*env).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalExp(rhs.clone(), inCache.clone(), env.clone())?) {
                (__pa0, Deref @ Values::Value::TUPLE { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            rhs_vals = __pa1.clone();
            lhs_crefs = List::map(lhs_expl.clone(), (std::sync::Arc::new(extractLhsComponentRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
            (cache, env) = assignTuple(lhs_crefs.clone(), rhs_vals.clone(), cache.clone(), env.clone())?;
            (cache.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv))
}

fn evaluateIfStatement(mut inStatement: Arc<DAE::Statement>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl;
    (outCache, outEnv, outLoopControl) = (::match_deref::match_deref! { match &(inStatement.clone()) {
        Deref @ DAE::Statement::STMT_IF { exp: cond, statementLst: stmts, else_: else_branch, .. } => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut bool_cond: bool;
            let mut loop_ctrl: LoopControl;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalExp(cond.clone(), inCache.clone(), inEnv.clone())?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            bool_cond = __pa1.clone();
            (cache, env, loop_ctrl) = evaluateIfStatement2(bool_cond.clone(), stmts.clone(), else_branch.clone(), cache.clone(), inEnv.clone())?;
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateIfStatement2(mut inCondition: bool, mut inStatements: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inElse: Arc<DAE::Else>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCondition.clone(), inStatements.clone(), inElse.clone(), inEnv.clone())) {
        (true, statements, _, env) => {
            let mut cache: FCore::Cache;
            let mut loop_ctrl: LoopControl;
            let mut env = (*env).clone();
            return Ok(evaluateStatements(statements.clone(), inCache.clone(), env.clone())?)
        },
        (false, _, Deref @ DAE::Else::ELSE { statementLst: statements }, env) => {
            let mut cache: FCore::Cache;
            let mut loop_ctrl: LoopControl;
            let mut env = (*env).clone();
            return Ok(evaluateStatements(statements.clone(), inCache.clone(), env.clone())?)
        },
        (false, _, Deref @ DAE::Else::ELSEIF { exp: condition, statementLst: statements, else_: else_branch }, env) => {
            let mut cache: FCore::Cache;
            let mut bool_condition: bool;
            let mut loop_ctrl: LoopControl;
            let mut env = (*env).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalExp(condition.clone(), inCache.clone(), env.clone())?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            bool_condition = __pa1.clone();
            { (inCondition, inStatements, inElse, inCache, inEnv) = (bool_condition.clone(), statements.clone(), else_branch.clone(), cache.clone(), env.clone()); continue '__tco; }
        },
        (false, _, Deref @ DAE::Else::NOELSE { .. }, _) => {
            return Ok((inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn evaluateForStatement(mut inStatement: Arc<DAE::Statement>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl;
    (outCache, outEnv, outLoopControl) = 'mc: {
        let __mc_input = (inStatement.clone(), inEnv.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FOR { type_: ety, iter: iter_name, range, statementLst: statements, .. }, env) => {
                    let mut ty: Arc<DAE::Type>;
                    let mut range_vals: Arc<metamodelica::List<Arc<Values::Value>>>;
                    let mut cache: FCore::Cache;
                    let mut iter_cr: Arc<DAE::ComponentRef>;
                    let mut loop_ctrl: LoopControl;
                    let mut env = (*env).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalExp(range.clone(), inCache.clone(), env.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    range_vals = __pa1.clone();
                    (env, ty, iter_cr) = extendEnvWithForScope((iter_name.clone()).clone(), ety.clone(), env.clone())?;
                    (cache, env, loop_ctrl) = evaluateForLoopArray(cache.clone(), env.clone(), iter_cr.clone(), ty.clone(), range_vals.clone(), statements.clone(), crate::CevalFunction::LoopControl::NEXT)?;
                    Ok((cache.clone(), env.clone(), loop_ctrl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FOR { range, .. }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- evaluateForStatement not implemented for:")).clone())?;
                    Debug::traceln((ExpressionBasics::printExpStr(range.clone())?).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateForLoopArray(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inIter: Arc<DAE::ComponentRef>, mut inIterType: Arc<DAE::Type>, mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inStatements: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inLoopControl: LoopControl) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inEnv.clone(), inValues.clone(), inLoopControl.clone())) {
        (_, _, LoopControl::BREAK { .. }) => {
            return Ok((inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT))
        },
        (_, _, LoopControl::RETURN { .. }) => {
            return Ok((inCache.clone(), inEnv.clone(), inLoopControl.clone()))
        },
        (_, Deref @ metamodelica::List::Nil, _) => {
            return Ok((inCache.clone(), inEnv.clone(), inLoopControl.clone()))
        },
        (env, Deref @ metamodelica::List::Cons { head: value, tail: rest_vals }, LoopControl::NEXT { .. }) => {
            let mut cache: FCore::Cache;
            let mut loop_ctrl: LoopControl;
            let mut env = (*env).clone();
            env = updateVariableBinding(inIter.clone(), env.clone(), inIterType.clone(), value.clone())?;
            (cache, env, loop_ctrl) = evaluateStatements(inStatements.clone(), inCache.clone(), env.clone())?;
            { (inCache, inEnv, inIter, inIterType, inValues, inStatements, inLoopControl) = (cache.clone(), env.clone(), inIter.clone(), inIterType.clone(), rest_vals.clone(), inStatements.clone(), loop_ctrl.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn evaluateWhileStatement(mut inCondition: Arc<DAE::Exp>, mut inStatements: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inLoopControl: LoopControl) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl;
    (outCache, outEnv, outLoopControl) = (match inLoopControl.clone() {
        LoopControl::BREAK { .. } => {
            (inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT)
        },
        LoopControl::RETURN { .. } => {
            (inCache.clone(), inEnv.clone(), inLoopControl.clone())
        },
        _ => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl;
            let mut b: bool;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalExp(inCondition.clone(), inCache.clone(), inEnv.clone())?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            b = __pa1.clone();
            if b.clone() {
                (cache, env, loop_ctrl) = evaluateStatements(inStatements.clone(), cache.clone(), inEnv.clone())?;
                (cache, env, loop_ctrl) = evaluateWhileStatement(inCondition.clone(), inStatements.clone(), cache.clone(), env.clone(), loop_ctrl.clone())?;
            } else {
                loop_ctrl = crate::CevalFunction::LoopControl::NEXT;
                env = inEnv.clone();
            }
            (cache.clone(), env.clone(), loop_ctrl.clone())
        },
    });
    Ok((outCache, outEnv, outLoopControl))
}

fn extractLhsComponentRef(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cref, .. } => {
            cref.clone()
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CevalFunction.extractLhsComponentRef failed on ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCref)
}

fn cevalExp(mut inExp: Arc<DAE::Exp>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: Arc<Values::Value>;
    (outCache, outValue) = Ceval::ceval(inCache.clone(), inEnv.clone(), inExp.clone(), true, Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?;
    let false = (openmodelica_frontend_types::Values::Value::interned_META_FAIL() == outValue.clone()) else { bail!("pattern mismatch") };
    Ok((outCache, outValue))
}

fn cevalExpList(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<Values::Value>>>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: Arc<metamodelica::List<Arc<Values::Value>>>;
    (outCache, outValue) = Ceval::cevalList(inCache.clone(), inEnv.clone(), inExpLst.clone(), true, Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?;
    Ok((outCache, outValue))
}

// [EENV]  Environment extension functions (add variables).
fn setupFunctionEnvironment(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFuncName: ArcStr, mut inFuncParams: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    outEnv = FGraph::openScope(inEnv.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, (inFuncName.clone()).clone(), Some(openmodelica_frontend_dump::FCore::ScopeType::FUNCTION_SCOPE))?;
    (outCache, outEnv) = extendEnvWithFunctionVars(inCache.clone(), outEnv.clone(), inFuncParams.clone())?;
    Ok((outCache, outEnv))
}

fn extendEnvWithFunctionVars(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFuncParams: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>) -> Result<(FCore::Cache, FCore::Graph)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inFuncParams.clone())) {
        (_, _, Deref @ metamodelica::List::Nil) => {
            return Ok((inCache.clone(), inEnv.clone()))
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: param, tail: rest_params }) => {
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (cache, env) = extendEnvWithFunctionVar(cache.clone(), env.clone(), param.clone())?;
            { (inCache, inEnv, inFuncParams) = (cache.clone(), env.clone(), rest_params.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn extendEnvWithFunctionVar(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFuncParam: FunctionVar) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = 'mc: {
        let __mc_input = (inEnv.clone(), inFuncParam.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, (e, val @ Some(_))) => {
                    let mut cache: FCore::Cache;
                    let mut env = (*env).clone();
                    (cache, env) = extendEnvWithElement(e.clone(), val.clone(), inCache.clone(), env.clone())?;
                    Ok((cache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, (e @ Deref @ DAE::Element::VAR { binding: binding_exp, .. }, None)) => {
                    let mut val: Option<Arc<Values::Value>>;
                    let mut cache: FCore::Cache;
                    let mut env = (*env).clone();
                    (val, cache) = evaluateBinding(binding_exp.clone(), inCache.clone(), inEnv.clone())?;
                    (cache, env) = extendEnvWithElement(e.clone(), val.clone(), cache.clone(), env.clone())?;
                    Ok((cache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (e, _)) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("- CevalFunction.extendEnvWithFunctionVars failed for:")).clone())?;
                    Debug::traceln((DAEDump::dumpElementsStr(list![e.clone()])?).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv))
}

fn evaluateBinding(mut inBinding: Option<Arc<DAE::Exp>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(Option<Arc<Values::Value>>, FCore::Cache)> {
    let mut outValue: Option<Arc<Values::Value>>;
    let mut outCache: FCore::Cache;
    (outValue, outCache) = (::match_deref::match_deref! { match &(inBinding.clone()) {
        Some(binding_exp) => {
            let mut cache: FCore::Cache;
            let mut val: Arc<Values::Value>;
            (cache, val) = cevalExp(binding_exp.clone(), inCache.clone(), inEnv.clone())?;
            (Some(val.clone()), cache.clone())
        },
        None => {
            (None, inCache.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outValue, outCache))
}

fn extendEnvWithElement(mut inElement: Arc<DAE::Element>, mut inBindingValue: Option<Arc<Values::Value>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::VAR { componentRef: cr, ty, dims, .. } => {
            let mut name: ArcStr;
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            name = (ComponentReference::crefStr(cr.clone())?).clone();
            (cache, env) = extendEnvWithVar((name.clone()).clone(), ty.clone(), inBindingValue.clone(), dims.clone(), inCache.clone(), inEnv.clone())?;
            (cache.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv))
}

fn extendEnvWithVar(mut inName: ArcStr, mut inType: Arc<DAE::Type>, mut inOptValue: Option<Arc<Values::Value>>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = 'mc: {
        let __mc_input = inEnv.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut ty: Arc<DAE::Type>;
            let mut var: Arc<DAE::Var>;
            let mut binding: Arc<DAE::Binding>;
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut record_env: FCore::Graph;
            let true = (Types::isRecord(inType.clone())) else { bail!("pattern mismatch") };
            binding = makeBinding(inOptValue.clone());
            (cache, ty) = appendDimensions(inType.clone(), inOptValue.clone(), inDims.clone(), inCache.clone(), inEnv.clone())?;
            var = makeFunctionVariable((inName.clone()).clone(), ty.clone(), binding.clone());
            (cache, record_env) = makeRecordEnvironment(inType.clone(), inOptValue.clone(), cache.clone(), inEnv.clone())?;
            env = FGraph::mkComponentNode(inEnv.clone(), var.clone(), Arc::new(SCode::Element::COMPONENT { name: (inName.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), arrayDim: None }), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_dump::FCore::Status::VAR_TYPED, record_env.clone())?;
            Ok((cache.clone(), env.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut ty: Arc<DAE::Type>;
            let mut var: Arc<DAE::Var>;
            let mut binding: Arc<DAE::Binding>;
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            binding = makeBinding(inOptValue.clone());
            (cache, ty) = appendDimensions(inType.clone(), inOptValue.clone(), inDims.clone(), inCache.clone(), inEnv.clone())?;
            var = makeFunctionVariable((inName.clone()).clone(), ty.clone(), binding.clone());
            env = FGraph::mkComponentNode(inEnv.clone(), var.clone(), Arc::new(SCode::Element::COMPONENT { name: (inName.clone()).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: metamodelica::nil(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), arrayDim: None }), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_dump::FCore::Status::VAR_TYPED, FGraph::empty())?;
            Ok((cache.clone(), env.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv))
}

fn makeFunctionVariable(mut inName: ArcStr, mut inType: Arc<DAE::Type>, mut inBinding: Arc<DAE::Binding>) -> Arc<DAE::Var> {
    let mut outVar: Arc<DAE::Var>;
    outVar = Arc::new(DAE::Var { name: (inName.clone()).clone(), attributes: DAE::dummyAttrVar().clone(), ty: inType.clone(), binding: inBinding.clone(), bind_from_outside: false, constOfForIteratorRange: None });
    outVar
}

fn makeBinding(mut inBindingValue: Option<Arc<Values::Value>>) -> Arc<DAE::Binding> {
    let mut outBinding: Arc<DAE::Binding>;
    outBinding = (::match_deref::match_deref! { match &(inBindingValue.clone()) {
        Some(val) => {
            Arc::new(DAE::Binding::VALBOUND { valBound: val.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE })
        },
        None => {
            openmodelica_frontend_types::DAE::Binding::interned_UNBOUND()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBinding
}

fn makeRecordEnvironment(mut inRecordType: Arc<DAE::Type>, mut inOptValue: Option<Arc<Values::Value>>, mut inCache: FCore::Cache, mut inGraph: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outRecordEnv: FCore::Graph;
    (outCache, outRecordEnv) = (::match_deref::match_deref! { match &(inRecordType.clone()) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, varLst: var_lst, .. } => {
            let mut vals: Arc<metamodelica::List<Option<Arc<Values::Value>>>>;
            let mut cache: FCore::Cache;
            let mut graph: FCore::Graph;
            let mut parent: metamodelica::Array<FCore::Node>;
            let mut child: metamodelica::Array<FCore::Node>;
            let mut node: FCore::Node;
            parent = FGraph::lastScopeRef(inGraph.clone())?;
            (graph, node) = FGraph::node(inGraph.clone(), (arcstr::literal!(FNode::feNodeName)).clone(), list![parent.clone()], FCore::Data::ND { scopeType: None });
            child = FNode::toRef(node.clone());
            FNode::addChildRef(parent.clone(), (arcstr::literal!(FNode::feNodeName)).clone(), child.clone(), false)?;
            graph = FGraph::pushScopeRef(graph.clone(), child.clone())?;
            vals = getRecordValues(inOptValue.clone(), inRecordType.clone())?;
            (cache, graph) = List::threadFold(var_lst.clone(), vals.clone(), (std::sync::Arc::new(extendEnvWithRecordVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Option<Arc<Values::Value>>, (FCore::Cache, FCore::Graph)) -> Result<(FCore::Cache, FCore::Graph)> + 'static>), (inCache.clone(), graph.clone()))?;
            (cache.clone(), graph.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outRecordEnv))
}

fn getRecordValues(mut inOptValue: Option<Arc<Values::Value>>, mut inRecordType: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Option<Arc<Values::Value>>>>> {
    let mut outValues: Arc<metamodelica::List<Option<Arc<Values::Value>>>>;
    outValues = (::match_deref::match_deref! { match &((inOptValue.clone(), inRecordType.clone())) {
        (Some(Deref @ Values::Value::RECORD { orderd: vals, .. }), _) => {
            let mut opt_vals: Arc<metamodelica::List<Option<Arc<Values::Value>>>>;
            opt_vals = List::map(vals.clone(), std::sync::Arc::new(fnptr!(Util::makeOption, _)))?;
            opt_vals.clone()
        },
        (None, Deref @ DAE::Type::T_COMPLEX { varLst: vars, .. }) => {
            let mut opt_vals: Arc<metamodelica::List<Option<Arc<Values::Value>>>>;
            let mut n: i32;
            n = (vars.clone().len() as i32);
            opt_vals = List::fill(None, n.clone());
            opt_vals.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValues)
}

fn extendEnvWithRecordVar(mut inVar: Arc<DAE::Var>, mut inOptValue: Option<Arc<Values::Value>>, mut inEnv: (FCore::Cache, FCore::Graph)) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outEnv: (FCore::Cache, FCore::Graph) = (FCore::Cache::NO_CACHE, <FCore::Graph as ::std::default::Default>::default());
    outEnv = (::match_deref::match_deref! { match &((inVar.clone(), inEnv.clone())) {
        (Deref @ DAE::Var { name, ty, .. }, (cache, env)) => {
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (cache, env) = extendEnvWithVar((name.clone()).clone(), ty.clone(), inOptValue.clone(), metamodelica::nil(), cache.clone(), env.clone())?;
            outEnv = (cache.clone(), env.clone());
            outEnv.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEnv)
}

fn extendEnvWithForScope(mut inIterName: ArcStr, mut inIterType: Arc<DAE::Type>, mut inEnv: FCore::Graph) -> Result<(FCore::Graph, Arc<DAE::Type>, Arc<DAE::ComponentRef>)> {
    let mut outEnv: FCore::Graph;
    let mut outIterType: Arc<DAE::Type>;
    let mut outIterCref: Arc<DAE::ComponentRef>;
    outIterType = Types::expTypetoTypesType(inIterType.clone());
    outEnv = FGraph::addForIterator(inEnv.clone(), (inIterName.clone()).clone(), outIterType.clone(), openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), openmodelica_frontend_types::SCode::Variability::CONST, Some(openmodelica_frontend_types::DAE::Const::C_CONST))?;
    outIterCref = ComponentReferenceBasics::makeCrefIdent((inIterName.clone()).clone(), inIterType.clone(), metamodelica::nil());
    Ok((outEnv, outIterType, outIterCref))
}

fn appendDimensions(mut inType: Arc<DAE::Type>, mut inOptBinding: Option<Arc<Values::Value>>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<DAE::Type>)> {
    let mut outCache: FCore::Cache;
    let mut outType: Arc<DAE::Type>;
    let mut binding_dims: Arc<metamodelica::List<i32>>;
    binding_dims = ValuesUtil::valueDimensions(Util::getOptionOrDefault(inOptBinding.clone(), Arc::new(Values::Value::INTEGER { integer: 0 })));
    (outCache, outType) = appendDimensions2(inType.clone(), inDims.clone(), binding_dims.clone(), inCache.clone(), inEnv.clone())?;
    Ok((outCache, outType))
}

fn appendDimensions2(mut inType: Arc<DAE::Type>, mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inBindingDims: Arc<metamodelica::List<i32>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<DAE::Type>)> {
    let mut outCache: FCore::Cache;
    let mut outType: Arc<DAE::Type>;
    (outCache, outType) = 'mc: {
        let __mc_input = (inType.clone(), inDims.clone(), inBindingDims.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::List::Nil, _) => {
                    Ok((inCache.clone(), ty.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: rest_dims }, Deref @ metamodelica::List::Cons { head: dim_int, tail: bind_dims }) => {
                    let mut dim: Arc<DAE::Dimension>;
                    let mut cache: FCore::Cache;
                    let mut ty = (*ty).clone();
                    dim = Expression::intDimension(dim_int.clone());
                    (cache, ty) = appendDimensions2(ty.clone(), rest_dims.clone(), bind_dims.clone(), inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: rest_dims }, bind_dims) => {
                    let mut cache: FCore::Cache;
                    let mut ty = (*ty).clone();
                    (cache, ty) = appendDimensions2(ty.clone(), rest_dims.clone(), bind_dims.clone(), inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 0 })] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: dim_int }, tail: rest_dims }, bind_dims) => {
                    let mut dim: Arc<DAE::Dimension>;
                    let mut cache: FCore::Cache;
                    let mut ty = (*ty).clone();
                    let mut bind_dims = (*bind_dims).clone();
                    dim = Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim_int.clone() });
                    bind_dims = List::restOrEmpty(bind_dims.clone())?;
                    (cache, ty) = appendDimensions2(ty.clone(), rest_dims.clone(), bind_dims.clone(), inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_BOOLEAN { .. }, tail: rest_dims }, bind_dims) => {
                    let mut dim: Arc<DAE::Dimension>;
                    let mut cache: FCore::Cache;
                    let mut ty = (*ty).clone();
                    let mut bind_dims = (*bind_dims).clone();
                    dim = Arc::new(DAE::Dimension::DIM_INTEGER { integer: 2 });
                    bind_dims = List::restOrEmpty(bind_dims.clone())?;
                    (cache, ty) = appendDimensions2(ty.clone(), rest_dims.clone(), bind_dims.clone(), inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_ENUM { size: dim_int, .. }, tail: rest_dims }, bind_dims) => {
                    let mut dim: Arc<DAE::Dimension>;
                    let mut cache: FCore::Cache;
                    let mut ty = (*ty).clone();
                    let mut bind_dims = (*bind_dims).clone();
                    dim = Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim_int.clone() });
                    bind_dims = List::restOrEmpty(bind_dims.clone())?;
                    (cache, ty) = appendDimensions2(ty.clone(), rest_dims.clone(), bind_dims.clone(), inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_EXP { exp: dim_exp }, tail: rest_dims }, bind_dims) => {
                    let mut dim_val: Arc<Values::Value>;
                    let mut dim_int: i32;
                    let mut dim: Arc<DAE::Dimension>;
                    let mut cache: FCore::Cache;
                    let mut ty = (*ty).clone();
                    let mut bind_dims = (*bind_dims).clone();
                    (cache, dim_val) = cevalExp(dim_exp.clone(), inCache.clone(), inEnv.clone())?;
                    dim_int = ValuesUtil::valueInteger(dim_val.clone())?;
                    dim = Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim_int.clone() });
                    bind_dims = List::restOrEmpty(bind_dims.clone())?;
                    (cache, ty) = appendDimensions2(ty.clone(), rest_dims.clone(), bind_dims.clone(), inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- CevalFunction.appendDimensions2 failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outType))
}

// [MENV]  Environment manipulation functions (set and get variables).
fn assignVariable(mut inCref: Arc<DAE::ComponentRef>, mut inNewValue: Arc<Values::Value>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = 'mc: {
        let __mc_input = inCref.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::WILD { .. } => {
                    Ok((inCache.clone(), inEnv.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, subscriptLst: Deref @ metamodelica::List::Nil, identType: ety @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. } } => {
                    let mut cache: FCore::Cache;
                    let mut env: FCore::Graph;
                    let mut var: Arc<DAE::Var>;
                    let mut inst_status: FCore::Status;
                    (_, var, _, _, inst_status, env) = Lookup::lookupIdentLocal(inCache.clone(), inEnv.clone(), (id.clone()).clone())?;
                    (cache, env) = assignRecord(ety.clone(), inNewValue.clone(), inCache.clone(), env.clone())?;
                    var = updateRecordBinding(var.clone(), inNewValue.clone());
                    env = FGraph::updateComp(inEnv.clone(), var.clone(), inst_status.clone(), env.clone());
                    Ok((cache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cr @ Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::List::Nil, .. } => {
                    let mut env: FCore::Graph;
                    let mut ty: Arc<DAE::Type>;
                    ty = Types::unflattenArrayType(Expression::r#typeof(ValuesUtil::valueExp(inNewValue.clone(), None)?)?)?;
                    env = updateVariableBinding(cr.clone(), inEnv.clone(), ty.clone(), inNewValue.clone())?;
                    Ok((inCache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, .. } => {
                    let mut cr: Arc<DAE::ComponentRef>;
                    let mut cache: FCore::Cache;
                    let mut env: FCore::Graph;
                    let mut ty: Arc<DAE::Type>;
                    let mut val: Arc<Values::Value>;
                    cr = ComponentReference::crefStripSubs(inCref.clone())?;
                    (ty, val) = getVariableTypeAndValue(cr.clone(), inEnv.clone())?;
                    (cache, val) = assignVector(inNewValue.clone(), val.clone(), subs.clone(), inCache.clone(), inEnv.clone())?;
                    env = updateVariableBinding(cr.clone(), inEnv.clone(), ty.clone(), val.clone())?;
                    Ok((cache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, subscriptLst: Deref @ metamodelica::List::Nil, componentRef: cr_rest, .. } => {
                    let mut cache: FCore::Cache;
                    let mut env: FCore::Graph;
                    let mut var: Arc<DAE::Var>;
                    let mut inst_status: FCore::Status;
                    let mut comp_id: ArcStr;
                    (_, var, _, _, inst_status, env) = Lookup::lookupIdentLocal(inCache.clone(), inEnv.clone(), (id.clone()).clone())?;
                    (cache, env) = assignVariable(cr_rest.clone(), inNewValue.clone(), inCache.clone(), env.clone())?;
                    comp_id = (ComponentReferenceBasics::crefFirstIdent(cr_rest.clone())?).clone();
                    var = updateRecordComponentBinding(var.clone(), (comp_id.clone()).clone(), inNewValue.clone())?;
                    env = FGraph::updateComp(inEnv.clone(), var.clone(), inst_status.clone(), env.clone());
                    Ok((cache.clone(), env.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outEnv))
}

fn assignTuple(mut inLhsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inRhsValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inLhsCrefs.clone(), inRhsValues.clone(), inCache.clone(), inEnv.clone())) {
        (Deref @ metamodelica::List::Nil, _, cache, env) => {
            return Ok((cache.clone(), env.clone()))
        },
        (Deref @ metamodelica::List::Cons { head: cr, tail: rest_crefs }, Deref @ metamodelica::List::Cons { head: value, tail: rest_vals }, cache, env) => {
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (cache, env) = assignVariable(cr.clone(), value.clone(), cache.clone(), env.clone())?;
            { (inLhsCrefs, inRhsValues, inCache, inEnv) = (rest_crefs.clone(), rest_vals.clone(), cache.clone(), env.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn assignRecord(mut inType: Arc<DAE::Type>, mut inValue: Arc<Values::Value>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (::match_deref::match_deref! { match &((inType.clone(), inValue.clone())) {
        (Deref @ DAE::Type::T_COMPLEX { varLst: vars, .. }, Deref @ Values::Value::RECORD { orderd: values, .. }) => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            (cache, env) = assignRecordComponents(vars.clone(), values.clone(), inCache.clone(), inEnv.clone())?;
            (cache.clone(), env.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outEnv))
}

fn assignRecordComponents(mut inVars: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, FCore::Graph)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inVars.clone(), inValues.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok((inCache.clone(), inEnv.clone()))
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { name, ty, .. }, tail: rest_vars }, Deref @ metamodelica::List::Cons { head: val, tail: rest_vals }) => {
            let mut cr: Arc<DAE::ComponentRef>;
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            cr = ComponentReferenceBasics::makeCrefIdent((name.clone()).clone(), ty.clone(), metamodelica::nil());
            (cache, env) = assignVariable(cr.clone(), val.clone(), inCache.clone(), inEnv.clone())?;
            { (inVars, inValues, inCache, inEnv) = (rest_vars.clone(), rest_vals.clone(), cache.clone(), env.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn assignVector(mut inNewValue: Arc<Values::Value>, mut inOldValue: Arc<Values::Value>, mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outResult: Arc<Values::Value>;
    (outCache, outResult) = 'mc: {
        let __mc_input = (inNewValue.clone(), inOldValue.clone(), inSubscripts.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil) => {
                    Ok((inCache.clone(), inNewValue.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Values::Value::ARRAY { valueLst: values, dimLst: dims }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: e }, tail: rest_subs }) => {
                    let mut index: Arc<Values::Value>;
                    let mut val: Arc<Values::Value>;
                    let mut i: i32;
                    let mut cache: FCore::Cache;
                    let mut values = (*values).clone();
                    (cache, index) = cevalExp(e.clone(), inCache.clone(), inEnv.clone())?;
                    i = ValuesUtil::valueInteger(index.clone())?;
                    val = (values.clone()).get(i.clone())?;
                    (cache, val) = assignVector(inNewValue.clone(), val.clone(), rest_subs.clone(), cache.clone(), inEnv.clone())?;
                    values = List::replaceAt(val.clone(), i.clone(), values.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: values.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: values, .. }, Deref @ Values::Value::ARRAY { valueLst: old_values, dimLst: dims }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: e }, tail: rest_subs }) => {
                    let mut values2: Arc<metamodelica::List<Arc<Values::Value>>>;
                    let mut old_values2: Arc<metamodelica::List<Arc<Values::Value>>>;
                    let mut indices: Arc<metamodelica::List<Arc<Values::Value>>>;
                    let mut i: i32;
                    let mut cache: FCore::Cache;
                    let mut values = (*values).clone();
                    let mut old_values = (*old_values).clone();
                    let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(cevalExp(e.clone(), inCache.clone(), inEnv.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa2 @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: _ }, .. }) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cache = __pa0.clone();
                    i = __pa1.clone();
                    indices = __pa2.clone();
                    (old_values, old_values2) = List::splitr(old_values.clone(), i.clone() - 1)?;
                    (cache, values2) = assignSlice(values.clone(), old_values2.clone(), indices.clone(), rest_subs.clone(), i.clone(), cache.clone(), inEnv.clone())?;
                    values = List::append_reverse(old_values.clone(), values2.clone());
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: values.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: values, .. }, Deref @ Values::Value::ARRAY { valueLst: values2, dimLst: dims }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: rest_subs }) => {
                    let mut cache: FCore::Cache;
                    let mut values = (*values).clone();
                    (cache, values) = assignWholeDim(values.clone(), values2.clone(), rest_subs.clone(), inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), Arc::new(Values::Value::ARRAY { valueLst: values.clone(), dimLst: dims.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Cons { head: sub, tail: _ }) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    metamodelica::print((literal!("- CevalFunction.assignVector failed on: ")).clone());
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printSubscriptStr(sub.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outResult))
}

fn assignSlice(mut inNewValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inOldValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inIndices: Arc<metamodelica::List<Arc<Values::Value>>>, mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inIndex: i32, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<Values::Value>>>)> {
    let mut outCache: FCore::Cache;
    let mut outResult: Arc<metamodelica::List<Arc<Values::Value>>>;
    (outCache, outResult) = 'mc: {
        let __mc_input = (inNewValues.clone(), inOldValues.clone(), inIndices.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::List::Nil) => {
                    Ok((inCache.clone(), inOldValues.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vl1, Deref @ metamodelica::List::Cons { head: v2, tail: vl2 }, Deref @ metamodelica::List::Cons { head: index, tail: _ }) => {
                    let mut cache: FCore::Cache;
                    let mut vl1 = (*vl1).clone();
                    let true = (inIndex.clone() < ValuesUtil::valueInteger(index.clone())?) else { bail!("pattern mismatch") };
                    (cache, vl1) = assignSlice(vl1.clone(), vl2.clone(), inIndices.clone(), inSubscripts.clone(), inIndex.clone() + 1, inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), metamodelica::cons(v2.clone(), vl1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: v1, tail: vl1 }, Deref @ metamodelica::List::Cons { head: v2, tail: vl2 }, Deref @ metamodelica::List::Cons { head: _, tail: rest_indices }) => {
                    let mut cache: FCore::Cache;
                    let mut v1 = (*v1).clone();
                    let mut vl1 = (*vl1).clone();
                    (cache, v1) = assignVector(v1.clone(), v2.clone(), inSubscripts.clone(), inCache.clone(), inEnv.clone())?;
                    (cache, vl1) = assignSlice(vl1.clone(), vl2.clone(), rest_indices.clone(), inSubscripts.clone(), inIndex.clone() + 1, inCache.clone(), inEnv.clone())?;
                    Ok((cache.clone(), metamodelica::cons(v1.clone(), vl1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outResult))
}

fn assignWholeDim(mut inNewValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inOldValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inCache: FCore::Cache, mut inEnv: FCore::Graph) -> Result<(FCore::Cache, Arc<metamodelica::List<Arc<Values::Value>>>)> {
    let mut outCache: FCore::Cache;
    let mut outResult: Arc<metamodelica::List<Arc<Values::Value>>>;
    (outCache, outResult) = (::match_deref::match_deref! { match &((inNewValues.clone(), inOldValues.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            (inCache.clone(), metamodelica::nil())
        },
        (Deref @ metamodelica::List::Cons { head: v1, tail: vl1 }, Deref @ metamodelica::List::Cons { head: v2, tail: vl2 }) => {
            let mut cache: FCore::Cache;
            let mut v1 = (*v1).clone();
            let mut vl1 = (*vl1).clone();
            (cache, v1) = assignVector(v1.clone(), v2.clone(), inSubscripts.clone(), inCache.clone(), inEnv.clone())?;
            (cache, vl1) = assignWholeDim(vl1.clone(), vl2.clone(), inSubscripts.clone(), inCache.clone(), inEnv.clone())?;
            (cache.clone(), metamodelica::cons(v1.clone(), vl1.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCache, outResult))
}

fn updateVariableBinding(mut inVariableCref: Arc<DAE::ComponentRef>, mut inEnv: FCore::Graph, mut inType: Arc<DAE::Type>, mut inNewValue: Arc<Values::Value>) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    let mut var_name: ArcStr;
    let mut var: Arc<DAE::Var>;
    var_name = (ComponentReference::crefStr(inVariableCref.clone())?).clone();
    var = makeFunctionVariable((var_name.clone()).clone(), inType.clone(), Arc::new(DAE::Binding::VALBOUND { valBound: inNewValue.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }));
    outEnv = FGraph::updateComp(inEnv.clone(), var.clone(), openmodelica_frontend_dump::FCore::Status::VAR_TYPED, FGraph::empty());
    Ok(outEnv)
}

fn updateRecordBinding(mut inVar: Arc<DAE::Var>, mut inValue: Arc<Values::Value>) -> Arc<DAE::Var> {
    let mut outVar: Arc<DAE::Var>;
    outVar = inVar.clone();
    assign_field!(outVar.binding = Arc::new(DAE::Binding::VALBOUND { valBound: inValue.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }));
    outVar
}

fn updateRecordComponentBinding(mut inVar: Arc<DAE::Var>, mut inComponentId: ArcStr, mut inValue: Arc<Values::Value>) -> Result<Arc<DAE::Var>> {
    let mut outVar: Arc<DAE::Var>;
    let mut val: Arc<Values::Value>;
    outVar = inVar.clone();
    val = getBindingOrDefault(outVar.binding.clone(), outVar.ty.clone())?;
    val = updateRecordComponentValue((inComponentId.clone()).clone(), inValue.clone(), val.clone())?;
    assign_field!(outVar.binding = Arc::new(DAE::Binding::VALBOUND { valBound: val.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }));
    Ok(outVar)
}

fn updateRecordComponentValue(mut inComponentId: ArcStr, mut inComponentValue: Arc<Values::Value>, mut inRecordValue: Arc<Values::Value>) -> Result<Arc<Values::Value>> {
    let mut outRecordValue: Arc<Values::Value>;
    let mut name: Arc<Absyn::Path>;
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>>;
    let mut comps: Arc<metamodelica::List<ArcStr>>;
    let mut pos: i32;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inRecordValue.clone()) {
        Deref @ Values::Value::RECORD { record_: __pa0, orderd: __pa1, comp: __pa2, index: (-1) } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    vals = __pa1.clone();
    comps = __pa2.clone();
    pos = List::position((inComponentId.clone()).clone(), comps.clone())?;
    vals = List::replaceAt(inComponentValue.clone(), pos.clone(), vals.clone())?;
    outRecordValue = Arc::new(Values::Value::RECORD { record_: name.clone(), orderd: vals.clone(), comp: comps.clone(), index: -1 });
    Ok(outRecordValue)
}

fn getVariableTypeAndBinding(mut inCref: Arc<DAE::ComponentRef>, mut inEnv: FCore::Graph) -> Result<(Arc<DAE::Type>, Arc<DAE::Binding>)> {
    let mut outType: Arc<DAE::Type>;
    let mut outBinding: Arc<DAE::Binding>;
    (_, _, outType, outBinding, _, _, _, _, _) = Lookup::lookupVar(FCore::emptyCache(), inEnv.clone(), inCref.clone())?;
    Ok((outType, outBinding))
}

fn getVariableTypeAndValue(mut inCref: Arc<DAE::ComponentRef>, mut inEnv: FCore::Graph) -> Result<(Arc<DAE::Type>, Arc<Values::Value>)> {
    let mut outType: Arc<DAE::Type>;
    let mut outValue: Arc<Values::Value>;
    let mut binding: Arc<DAE::Binding>;
    (outType, binding) = getVariableTypeAndBinding(inCref.clone(), inEnv.clone())?;
    outValue = getBindingOrDefault(binding.clone(), outType.clone())?;
    Ok((outType, outValue))
}

fn getBindingValueOpt(mut inBinding: Arc<DAE::Binding>) -> Option<Arc<Values::Value>> {
    let mut outValue: Option<Arc<Values::Value>>;
    outValue = (::match_deref::match_deref! { match &(inBinding.clone()) {
        Deref @ DAE::Binding::VALBOUND { valBound: val, .. } => {
            Some(val.clone())
        },
        Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(val), .. } => {
            Some(val.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outValue
}

fn getBindingOrDefault(mut inBinding: Arc<DAE::Binding>, mut inType: Arc<DAE::Type>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value>;
    outValue = (::match_deref::match_deref! { match &(inBinding.clone()) {
        Deref @ DAE::Binding::VALBOUND { valBound: val, .. } => {
            val.clone()
        },
        Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(val), .. } => {
            val.clone()
        },
        _ => {
            generateDefaultBinding(inType.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValue)
}

fn generateDefaultBinding(mut inType: Arc<DAE::Type>) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value>;
    outValue = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { .. } => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { .. } => {
                    Ok(Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_STRING { .. } => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { .. } => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ENUMERATION { .. } => {
                    Ok(Arc::new(Values::Value::ENUM_LITERAL { name: Arc::new(Absyn::Path::IDENT { name: (literal!("")).clone() }), index: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil }, ty } => {
                    let mut int_dim: i32;
                    let mut dims: Arc<metamodelica::List<i32>>;
                    let mut values: Arc<metamodelica::List<Arc<Values::Value>>>;
                    let mut value: Arc<Values::Value>;
                    int_dim = Expression::dimensionSize(dim.clone())?;
                    value = generateDefaultBinding(ty.clone())?;
                    values = List::fill(value.clone(), int_dim.clone());
                    dims = ValuesUtil::valueDimensions(value.clone());
                    Ok(Arc::new(Values::Value::ARRAY { valueLst: values.clone(), dimLst: metamodelica::cons(int_dim.clone(), dims.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path }, varLst: vars, .. } => {
                    let mut values: Arc<metamodelica::List<Arc<Values::Value>>>;
                    let mut var_names: Arc<metamodelica::List<ArcStr>>;
                    (values, var_names) = List::map_2(vars.clone(), (std::sync::Arc::new(getRecordVarBindingAndName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<(Arc<Values::Value>, ArcStr)> + 'static>))?;
                    Ok(Arc::new(Values::Value::RECORD { record_: path.clone(), orderd: values.clone(), comp: var_names.clone(), index: -1 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- CevalFunction.generateDefaultBinding failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValue)
}

fn getRecordVarBindingAndName(mut inVar: Arc<DAE::Var>) -> Result<(Arc<Values::Value>, ArcStr)> {
    let mut outBinding: Arc<Values::Value>;
    let mut outName: ArcStr;
    (outBinding, outName) = 'mc: {
        let __mc_input = inVar.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { name, ty, binding, .. } => {
                    let mut val: Arc<Values::Value>;
                    val = getBindingOrDefault(binding.clone(), ty.clone())?;
                    Ok((val.clone(), name.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { name, .. } => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CevalFunction.getRecordVarBindingAndName failed on variable ")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outBinding, outName))
}

fn getFunctionReturnValue(mut inOutputVar: Arc<DAE::Element>, mut inEnv: FCore::Graph) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value>;
    outValue = (::match_deref::match_deref! { match &(inOutputVar.clone()) {
        Deref @ DAE::Element::VAR { componentRef: cr, ty, .. } => {
            let mut val: Arc<Values::Value>;
            val = getVariableValue(cr.clone(), ty.clone(), inEnv.clone())?;
            val.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValue)
}

fn getVariableValue(mut inCref: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>, mut inEnv: FCore::Graph) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value>;
    outValue = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. } => {
                    let mut val: Arc<Values::Value>;
                    let mut p: Arc<Absyn::Path>;
                    p = ComponentReference::crefToPath(inCref.clone())?;
                    val = getRecordValue(p.clone(), inType.clone(), inEnv.clone())?;
                    Ok(val.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut val: Arc<Values::Value>;
                    (_, val) = getVariableTypeAndValue(inCref.clone(), inEnv.clone())?;
                    Ok(val.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValue)
}

fn getRecordValue(mut inRecordName: Arc<Absyn::Path>, mut inType: Arc<DAE::Type>, mut inEnv: FCore::Graph) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value>;
    outValue = (::match_deref::match_deref! { match &((inRecordName.clone(), inType.clone())) {
        (Deref @ Absyn::Path::IDENT { name: id }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p }, varLst: vars, .. }) => {
            let mut vals: Arc<metamodelica::List<Arc<Values::Value>>>;
            let mut var_names: Arc<metamodelica::List<ArcStr>>;
            let mut env: FCore::Graph;
            (_, _, _, _, _, env) = Lookup::lookupIdentLocal(FCore::emptyCache(), inEnv.clone(), (id.clone()).clone())?;
            vals = List::map1(vars.clone(), (std::sync::Arc::new(getRecordComponentValue) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, FCore::Graph) -> Result<Arc<Values::Value>> + 'static>), env.clone())?;
            var_names = List::map(vars.clone(), (std::sync::Arc::new(TypesDump::getVarName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?;
            Arc::new(Values::Value::RECORD { record_: p.clone(), orderd: vals.clone(), comp: var_names.clone(), index: -1 })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValue)
}

fn getRecordComponentValue(mut inVars: Arc<DAE::Var>, mut inEnv: FCore::Graph) -> Result<Arc<Values::Value>> {
    let mut outValues: Arc<Values::Value>;
    outValues = (::match_deref::match_deref! { match &(inVars.clone()) {
        Deref @ DAE::Var { name: id, ty: ty @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, .. } => {
            let mut val: Arc<Values::Value>;
            val = getRecordValue(Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }), ty.clone(), inEnv.clone())?;
            val.clone()
        },
        Deref @ DAE::Var { name: id, ty, binding: tvbinding, .. } => {
            let mut val: Arc<Values::Value>;
            let mut oval: Option<Arc<Values::Value>>;
            let mut binding: Arc<DAE::Binding>;
            let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupIdentLocal(FCore::emptyCache(), inEnv.clone(), (id.clone()).clone())?) {
                (_, Deref @ DAE::Var { binding: __pa0, .. }, _, _, _, _) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            binding = __pa0.clone();
            oval = getBindingValueOpt(binding.clone());
            if isNone(oval.clone()) {
                oval = getBindingValueOpt(tvbinding.clone());
            }
            if isSome(oval.clone()) {
                let __pa1 = ::match_deref::match_deref! { match &(oval.clone()) {
                    Some(__pa1) => __pa1.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                val = __pa1.clone();
            } else {
                val = generateDefaultBinding(ty.clone())?;
            }
            val.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValues)
}

fn boxReturnValue(mut inReturnValues: Arc<metamodelica::List<Arc<Values::Value>>>) -> Arc<Values::Value> {
    let mut outValue: Arc<Values::Value>;
    outValue = (::match_deref::match_deref! { match &(inReturnValues.clone()) {
        Deref @ metamodelica::List::Nil => {
            openmodelica_frontend_types::Values::Value::interned_NORETCALL()
        },
        Deref @ metamodelica::List::Cons { head: val, tail: Deref @ metamodelica::List::Nil } => {
            val.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: _ } => {
            Arc::new(Values::Value::TUPLE { valueLst: inReturnValues.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outValue
}

// [DEPS]  Function variable dependency handling.
fn sortFunctionVarsByDependency(mut inFuncVars: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, mut inSource: Arc<DAE::ElementSource>) -> Result<Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>> {
    let mut outFuncVars: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>;
    let mut cycles: Arc<metamodelica::List<((Arc<DAE::Element>, Option<Arc<Values::Value>>), Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>)>>;
    (outFuncVars, cycles) = Graph::topologicalSort(Graph::buildGraph(inFuncVars.clone(), (std::sync::Arc::new(fnptr!(getElementDependencies, (Arc<DAE::Element>, Option<Arc<Values::Value>>), Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>)) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::Element>, Option<Arc<Values::Value>>), Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>) -> Result<Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>> + 'static>), inFuncVars.clone())?, (std::sync::Arc::new(isElementEqual) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::Element>, Option<Arc<Values::Value>>), (Arc<DAE::Element>, Option<Arc<Values::Value>>)) -> Result<bool> + 'static>))?;
    checkCyclicalComponents(cycles.clone(), inSource.clone())?;
    Ok(outFuncVars)
}

fn getElementDependencies(mut inElement: FunctionVar, mut inAllElements: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>) -> Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>> {
    pub type Arg = (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>);

    let mut outDependencies: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>;
    outDependencies = 'mc: {
        let __mc_input = inElement.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::VAR { binding: Some(bind_exp), dims, .. }, _) => {
                    let mut deps: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>;
                    let mut arg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>);
                    let (_, ref __pa1 @ (_, ref __pa0, _)) = Expression::traverseExpBidir(bind_exp.clone(), (std::sync::Arc::new(fnptr!(getElementDependenciesTraverserEnter, Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>))> + 'static>), (std::sync::Arc::new(getElementDependenciesTraverserExit) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>))> + 'static>), (inAllElements.clone(), metamodelica::nil(), metamodelica::nil()))?;
                    deps = __pa0.clone();
                    arg = __pa1.clone();
                    let (_, (_, __pa2, _)) = List::mapFold(dims.clone(), (std::sync::Arc::new(fnptr!(getElementDependenciesFromDims, Arc<DAE::Dimension>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>)) -> Result<(Arc<DAE::Dimension>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>))> + 'static>), arg.clone())?;
                    deps = __pa2.clone();
                    Ok(deps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::VAR { dims, .. }, _) => {
                    let mut deps: Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>;
                    let (_, (_, __pa0, _)) = List::mapFold(dims.clone(), (std::sync::Arc::new(fnptr!(getElementDependenciesFromDims, Arc<DAE::Dimension>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>)) -> Result<(Arc<DAE::Dimension>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>))> + 'static>), (inAllElements.clone(), metamodelica::nil(), metamodelica::nil()))?;
                    deps = __pa0.clone();
                    Ok(deps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outDependencies
}

fn getElementDependenciesFromDims(mut inDimension: Arc<DAE::Dimension>, mut inArg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>)) -> (Arc<DAE::Dimension>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>)) {
    pub type Arg = (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>);

    let mut outDimension: Arc<DAE::Dimension>;
    let mut outArg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>);
    (outDimension, outArg) = 'mc: {
        let __mc_input = inArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut arg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>);
                    let mut dim_exp: Arc<DAE::Exp>;
                    dim_exp = Expression::dimensionSizeExp(inDimension.clone())?;
                    (_, arg) = Expression::traverseExpBidir(dim_exp.clone(), (std::sync::Arc::new(fnptr!(getElementDependenciesTraverserEnter, Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>))> + 'static>), (std::sync::Arc::new(getElementDependenciesTraverserExit) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>))> + 'static>), inArg.clone())?;
                    Ok((inDimension.clone(), arg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inDimension.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outDimension, outArg)
}

fn getElementDependenciesTraverserEnter(mut inExp: Arc<DAE::Exp>, mut inArg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>)) -> (Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>)) {
    pub type Arg = (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>);

    let mut outExp: Arc<DAE::Exp>;
    let mut outArg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>);
    (outExp, outArg) = 'mc: {
        let __mc_input = (inExp.clone(), inArg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: iter, .. }, .. }, (all_el, accum_el, iters @ Deref @ metamodelica::List::Cons { head: _, tail: _ })) => {
                    let true = (List::isMemberOnTrue((iter.clone()).clone(), iters.clone(), (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    Ok((exp.clone(), (all_el.clone(), accum_el.clone(), iters.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { componentRef: cref, .. }, (all_el, accum_el, iters)) => {
                    let mut e: FunctionVar;
                    let mut all_el = (*all_el).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::deleteMemberOnTrue(cref.clone(), all_el.clone(), (std::sync::Arc::new(isElementNamed) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, (Arc<DAE::Element>, Option<Arc<Values::Value>>)) -> Result<bool> + 'static>))?) {
                        (__pa0, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    all_el = __pa0.clone();
                    e = __pa1.clone();
                    Ok((exp.clone(), (all_el.clone(), metamodelica::cons(e.clone(), accum_el.clone()), iters.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::REDUCTION { iterators: riters, .. }, (all_el, accum_el, iters)) => {
                    let mut iters = (*iters).clone();
                    iters = listAppend(List::map(riters.clone(), (std::sync::Arc::new(Expression::reductionIterName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ReductionIterator>) -> Result<ArcStr> + 'static>))?, iters.clone());
                    Ok((exp.clone(), (all_el.clone(), accum_el.clone(), iters.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inArg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outArg)
}

fn getElementDependenciesTraverserExit(mut inExp: Arc<DAE::Exp>, mut inArg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>))> {
    pub type Arg = (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>);

    let mut outExp: Arc<DAE::Exp>;
    let mut outArg: (Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>, Arc<metamodelica::List<ArcStr>>);
    (outExp, outArg) = (::match_deref::match_deref! { match &((inExp.clone(), inArg.clone())) {
        (exp @ Deref @ DAE::Exp::REDUCTION { iterators: riters, .. }, (all_el, accum_el, iters)) => {
            let mut iters = (*iters).clone();
            iters = compareIterators(riters.clone().reverse(), iters.clone())?;
            (exp.clone(), (all_el.clone(), accum_el.clone(), iters.clone()))
        },
        _ => {
            (inExp.clone(), inArg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outArg))
}

fn compareIterators(mut inRiters: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, mut inIters: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outIters: Arc<metamodelica::List<ArcStr>>;
    outIters = 'mc: {
        let __mc_input = (inRiters.clone(), inIters.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { id: id1, .. }, tail: riters }, Deref @ metamodelica::List::Cons { head: id2, tail: iters }) => {
                    let true = (stringEqual((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(compareIterators(riters.clone(), iters.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(inIters.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Different iterators in CevalFunction.compareIterators.")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outIters)
}

fn isElementNamed(mut inName: Arc<DAE::ComponentRef>, mut inElement: FunctionVar) -> Result<bool> {
    let mut isNamed: bool;
    let mut name: Arc<DAE::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &(inElement.clone()) {
        (Deref @ DAE::Element::VAR { componentRef: __pa0, .. }, _) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    isNamed = ComponentReferenceBasics::crefEqualWithoutSubs(name.clone(), inName.clone());
    Ok(isNamed)
}

fn isElementEqual(mut inElement1: FunctionVar, mut inElement2: FunctionVar) -> Result<bool> {
    let mut isEqual: bool;
    let mut cr1: Arc<DAE::ComponentRef>;
    let mut cr2: Arc<DAE::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &(inElement1.clone()) {
        (Deref @ DAE::Element::VAR { componentRef: __pa0, .. }, _) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cr1 = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(inElement2.clone()) {
        (Deref @ DAE::Element::VAR { componentRef: __pa1, .. }, _) => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cr2 = __pa1.clone();
    isEqual = ComponentReferenceBasics::crefEqualWithoutSubs(cr1.clone(), cr2.clone());
    Ok(isEqual)
}

fn checkCyclicalComponents(mut inCycles: Arc<metamodelica::List<((Arc<DAE::Element>, Option<Arc<Values::Value>>), Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>)>>, mut inSource: Arc<DAE::ElementSource>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inCycles.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        _ => {
            let mut cycles: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<DAE::Element>, Option<Arc<Values::Value>>)>>>>;
            let mut elements: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>;
            let mut crefs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>;
            let mut names: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>;
            let mut cycles_strs: Arc<metamodelica::List<ArcStr>>;
            let mut cycles_str: ArcStr;
            let mut scope_str: ArcStr;
            let mut info: SourceInfo;
            cycles = Graph::findCycles(inCycles.clone(), (std::sync::Arc::new(isElementEqual) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::Element>, Option<Arc<Values::Value>>), (Arc<DAE::Element>, Option<Arc<Values::Value>>)) -> Result<bool> + 'static>))?;
            elements = List::mapList(cycles.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)))?;
            crefs = List::mapList(elements.clone(), (std::sync::Arc::new(DAEUtil::varCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
            names = List::mapList(crefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))?;
            cycles_strs = List::map1(names.clone(), (std::sync::Arc::new(fnptr!(stringDelimitList, Arc<metamodelica::List<ArcStr>>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<ArcStr>>, ArcStr) -> Result<ArcStr> + 'static>), (literal!(",")).clone())?;
            cycles_str = stringDelimitList(cycles_strs.clone(), (literal!("}, {")).clone());
            cycles_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*cycles_str.clone()); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
            scope_str = (literal!("")).clone();
            info = ElementSource::getElementSourceFileInfo(inSource.clone());
            Error::addSourceMessage(Error::CIRCULAR_COMPONENTS.clone(), list![(scope_str.clone()).clone(), (cycles_str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

// [EOPT]  Expression optimization functions.
fn optimizeExpTraverser(mut inExp: Arc<DAE::Exp>, mut inEnv: FCore::Graph) -> Result<(Arc<DAE::Exp>, FCore::Graph)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outEnv: FCore::Graph;
    (outExp, outEnv) = (::match_deref::match_deref! { match &((inExp.clone(), inEnv.clone())) {
        (Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CREF { componentRef: cref, ty: ety }, sub: subs }, env) => {
            let mut exp: Arc<DAE::Exp>;
            let mut cref = (*cref).clone();
            cref = ComponentReference::subscriptCref(cref.clone(), subs.clone())?;
            exp = Expression::makeCrefExp(cref.clone(), ety.clone())?;
            (exp.clone(), env.clone())
        },
        (Deref @ DAE::Exp::TSUB { exp: Deref @ DAE::Exp::TUPLE { PR: Deref @ metamodelica::List::Cons { head: exp, tail: _ } }, ix: 1, .. }, env) => {
            (exp.clone(), env.clone())
        },
        _ => {
            (inExp.clone(), inEnv.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outEnv))
}

