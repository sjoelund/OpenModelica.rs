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

use crate::BackendDAECreate;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendUtil;
use crate::BackendVariable;
use crate::SymbolicJacobian::DAE_CJ;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::AvlSetPath;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend::Ceval;
use openmodelica_frontend_base::Algorithm;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTpl;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_tpl::Tpl;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub(crate) const defaultMaxIter: i32 = 20;

// =============================================================================
// differentiation interfaces:
//  - createDifferentiatedCrefName
//  - createSeedCrefName
//  - differentiateEquation
//  - differentiateEquationTime
//  - differentiateExpCrefFullJacobian
//  - differentiateExpSolve
//  - differentiateExpTime
// =============================================================================
pub(crate) fn differentiateEquationTime(mut inEquation: Arc<BackendDAE::Equation>, mut inVariables: BackendDAE::Variables, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Option<Arc<BackendDAE::Equation>>, Arc<BackendDAE::Shared>)> {
    let mut outEquation: Option<Arc<BackendDAE::Equation>>;
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut diffData: BackendDAE::DifferentiateInputData;
    let mut eqn: Arc<BackendDAE::Equation>;
    let mut knvars: BackendDAE::Variables;
    let mut source: Arc<DAE::ElementSource>;
    let mut funcs: Arc<AvlTreePathFunction::Tree>;
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrEqnStr((literal!("### differentiateEquationTime\n")).clone(), inEquation.clone(), (literal!(" w.r.t. time\n")).clone()), '__try0);
        }
        funcs = unwrap_break_err!(BackendDAEUtil::getFunctions(inShared.clone()), '__try0);
        knvars = unwrap_break_err!(BackendDAEUtil::getGlobalKnownVarsFromShared(inShared.clone()), '__try0);
        diffData = BackendDAE::emptyInputData().clone();
        diffData.dependenentVars = Some(inVariables.clone());
        diffData.knownVars = Some(knvars.clone());
        diffData.allVars = Some(inVariables.clone());
        (eqn, funcs) = unwrap_break_err!(differentiateEquation(inEquation.clone(), DAE::crefTime().clone(), diffData.clone(), openmodelica_backend_types::BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, funcs.clone()), '__try0);
        outEquation = Some(eqn.clone());
        outShared = unwrap_break_err!(BackendDAEUtil::setSharedFunctionTree(inShared.clone(), funcs.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrEqnStr((literal!("### Result of differentiateEquationTime\n --> ")).clone(), eqn.clone(), (literal!("\n")).clone()), '__try0);
        }
        Ok::<_, anyhow::Error>((outEquation.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outEquation = __try0_o0;
        }
        Err(_) => {
            source = BackendEquation::equationSource(inEquation.clone())?;
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nDifferentiate.differentiateEquationTime failed for ")); __mm_s.push_str(&*BackendDump::equationString(inEquation.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
            outEquation = None;
        }
    }
    Ok((outEquation, outShared))
}

pub(crate) fn differentiateExpTime(mut inExp: Arc<DAE::Exp>, mut inVariables: BackendDAE::Variables, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<DAE::Exp>, Arc<BackendDAE::Shared>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outShared: Arc<BackendDAE::Shared>;
    let mut dexp: Arc<DAE::Exp>;
    let mut funcs: Arc<AvlTreePathFunction::Tree>;
    let mut diffData: BackendDAE::DifferentiateInputData;
    let mut knvars: BackendDAE::Variables;
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrExpStr((literal!("### differentiateExpTime\n ")).clone(), inExp.clone(), (literal!(" w.r.t. time\n")).clone()), '__try0);
        }
        funcs = unwrap_break_err!(BackendDAEUtil::getFunctions(inShared.clone()), '__try0);
        knvars = unwrap_break_err!(BackendDAEUtil::getGlobalKnownVarsFromShared(inShared.clone()), '__try0);
        diffData = BackendDAE::emptyInputData().clone();
        diffData.dependenentVars = Some(inVariables.clone());
        diffData.knownVars = Some(knvars.clone());
        (dexp, funcs) = unwrap_break_err!(differentiateExp(inExp.clone(), DAE::crefTime().clone(), diffData.clone(), openmodelica_backend_types::BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, funcs.clone(), defaultMaxIter.clone()), '__try0);
        (outExp, _) = unwrap_break_err!(ExpressionSimplify::simplify(dexp.clone()), '__try0);
        outShared = unwrap_break_err!(BackendDAEUtil::setSharedFunctionTree(inShared.clone(), funcs.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrExpStr((literal!("### Result of differentiateExpTime\n --> ")).clone(), outExp.clone(), (literal!("n")).clone()), '__try0);
        }
        Ok::<_, anyhow::Error>((dexp.clone(), diffData.clone(), funcs.clone(), knvars.clone(), outExp.clone(), outShared.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            dexp = __try0_o0;
            diffData = __try0_o1;
            funcs = __try0_o2;
            knvars = __try0_o3;
            outExp = __try0_o4;
            outShared = __try0_o5;
        }
        Err(__try0_err) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addSourceMessage(Error::NON_EXISTING_DERIVATIVE.clone(), list![(ExpressionBasics::printExpStr(inExp.clone())?).clone(), (literal!("time")).clone()], metamodelica::sourceInfo!("BackEnd/Differentiate.mo"))?;
            }
            return Err(__try0_err);
        }
    }
    Ok((outExp, outShared))
}

pub(crate) fn differentiateExpSolve(mut inExp: Arc<DAE::Exp>, mut inCref: Arc<DAE::ComponentRef>, mut functions: Option<Arc<AvlTreePathFunction::Tree>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut fac: Arc<metamodelica::List<Arc<DAE::Exp>>> = Expression::factors(inExp.clone())?;
    let mut dexp: Arc<DAE::Exp>;
    let mut fun: Arc<AvlTreePathFunction::Tree>;
    ::match_deref::match_deref! { match &(List::split1OnTrue(fac, (std::sync::Arc::new(Expression::expHasCrefInIf) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), inCref.clone())?) {
        (Deref @ metamodelica::List::Nil, _) => (),
        _ => bail!("pattern mismatch"),
    } };
    match '__try0: {
        fun = (::match_deref::match_deref! { match &(functions.clone()) {
        Some(fun_) => {
            fun_.clone()
        },
        _ => {
            openmodelica_frontend_dump::AvlTreePathFunction::Tree::interned_EMPTY()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrExpStrCrefStr((literal!("### differentiateExpSolve\n ")).clone(), inExp.clone(), (literal!(" w.r.t. ")).clone(), inCref.clone(), (literal!("\n")).clone()), '__try0);
        }
        (dexp, _) = unwrap_break_err!(differentiateExp(inExp.clone(), inCref.clone(), BackendDAE::emptyInputData().clone(), openmodelica_backend_types::BackendDAE::DifferentiationType::SIMPLE_DIFFERENTIATION, fun.clone(), defaultMaxIter.clone()), '__try0);
        (outExp, _) = unwrap_break_err!(ExpressionSimplify::simplify(dexp.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone()), '__try0) {
            unwrap_break_err!(BackendDump::debugStrExpStr((literal!("### Result of differentiateExpSolve\n --> ")).clone(), outExp.clone(), (literal!("\n")).clone()), '__try0);
        }
        Ok::<_, anyhow::Error>((dexp.clone(), fun.clone(), outExp.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            dexp = __try0_o0;
            fun = __try0_o1;
            outExp = __try0_o2;
        }
        Err(__try0_err) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addSourceMessage(Error::NON_EXISTING_DERIVATIVE.clone(), list![(ExpressionBasics::printExpStr(inExp.clone())?).clone(), (ComponentReference::crefStr(inCref.clone())?).clone()], metamodelica::sourceInfo!("BackEnd/Differentiate.mo"))?;
            }
            return Err(__try0_err);
        }
    }
    Ok(outExp)
}

pub(crate) fn differentiateExpCrefFullJacobian(mut inExp: Arc<DAE::Exp>, mut inCref: Arc<DAE::ComponentRef>, mut inVariables: BackendDAE::Variables, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<DAE::Exp>, Arc<BackendDAE::Shared>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outShared: Arc<BackendDAE::Shared>;
    let mut dexp: Arc<DAE::Exp>;
    let mut funcs: Arc<AvlTreePathFunction::Tree>;
    let mut diffData: BackendDAE::DifferentiateInputData;
    let mut knvars: BackendDAE::Variables;
    match '__try0: {
        funcs = unwrap_break_err!(BackendDAEUtil::getFunctions(inShared.clone()), '__try0);
        knvars = unwrap_break_err!(BackendDAEUtil::getGlobalKnownVarsFromShared(inShared.clone()), '__try0);
        diffData = BackendDAE::emptyInputData().clone();
        diffData.dependenentVars = Some(inVariables.clone());
        diffData.knownVars = Some(knvars.clone());
        (dexp, funcs) = unwrap_break_err!(differentiateExp(inExp.clone(), inCref.clone(), diffData.clone(), openmodelica_backend_types::BackendDAE::DifferentiationType::DIFF_FULL_JACOBIAN, funcs.clone(), defaultMaxIter.clone()), '__try0);
        (outExp, _) = unwrap_break_err!(ExpressionSimplify::simplify(dexp.clone()), '__try0);
        outShared = unwrap_break_err!(BackendDAEUtil::setSharedFunctionTree(inShared.clone(), funcs.clone()), '__try0);
        Ok::<_, anyhow::Error>((dexp.clone(), diffData.clone(), funcs.clone(), knvars.clone(), outExp.clone(), outShared.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            dexp = __try0_o0;
            diffData = __try0_o1;
            funcs = __try0_o2;
            knvars = __try0_o3;
            outExp = __try0_o4;
            outShared = __try0_o5;
        }
        Err(__try0_err) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addSourceMessage(Error::NON_EXISTING_DERIVATIVE.clone(), list![(ExpressionBasics::printExpStr(inExp.clone())?).clone(), (ComponentReference::crefStr(inCref.clone())?).clone()], metamodelica::sourceInfo!("BackEnd/Differentiate.mo"))?;
            }
            return Err(__try0_err);
        }
    }
    Ok((outExp, outShared))
}

// =============================================================================
// further interface functions to differentiation
//  - differentiateEquation
//  - differentiateBackendDAE
//
// =============================================================================
pub(crate) fn differentiateEquation(mut inEquation: Arc<BackendDAE::Equation>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<BackendDAE::Equation>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outEquation: Arc<BackendDAE::Equation>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree>;
    if let Ok((__pa0, __pa1)) = differentiateEquationFragile(inEquation.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone()) {
        outEquation = __pa0.clone();
        outFunctionTree = __pa1.clone();
    } else {
        Error::addSourceMessage(Error::NON_EXISTING_DERIVATIVE.clone(), list![(BackendDump::equationString(inEquation.clone())?).clone(), (ComponentReference::crefStr(inDiffwrtCref.clone())?).clone()], metamodelica::sourceInfo!("BackEnd/Differentiate.mo"))?;
        bail!("fail");
    }
    Ok((outEquation, outFunctionTree))
}

pub(crate) fn differentiateEquationFragile(mut inEquation: Arc<BackendDAE::Equation>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<BackendDAE::Equation>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outEquation: Arc<BackendDAE::Equation>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree>;
    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
        BackendDump::debugStrEqnStr((literal!("### differentiateEquation\n ")).clone(), inEquation.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" w.r.t. ")); __mm_s.push_str(&*ComponentReference::crefStr(inDiffwrtCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
    }
    (outEquation, outFunctionTree) = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr: eqAttr } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut op1: Arc<DAE::SymbolicOperation>;
            let mut op2: Arc<DAE::SymbolicOperation>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut source = (*source).clone();
            (e1_1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, defaultMaxIter.clone())?;
            (e1_1, _) = ExpressionSimplify::simplify(e1_1)?;
            (e2_1, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData, inDiffType, funcs, defaultMaxIter.clone())?;
            (e2_1, _) = ExpressionSimplify::simplify(e2_1)?;
            op1 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e1.clone(), after: e1_1.clone() });
            op2 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref, before: e2.clone(), after: e2_1.clone() });
            source = List::foldr(list![op1, op2], (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone())?;
            (Arc::new(BackendDAE::Equation::EQUATION { exp: e1_1, scalar: e2_1, source: source.clone(), attr: eqAttr.clone() }), funcs)
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref, exp: e2, source, attr: eqAttr } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut op1: Arc<DAE::SymbolicOperation>;
            let mut op2: Arc<DAE::SymbolicOperation>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut source = (*source).clone();
            e1 = Expression::crefExp(cref.clone())?;
            (e1_1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, defaultMaxIter.clone())?;
            (e1_1, _) = ExpressionSimplify::simplify(e1_1)?;
            (e2_1, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData, inDiffType, funcs, defaultMaxIter.clone())?;
            (e2_1, _) = ExpressionSimplify::simplify(e2_1)?;
            op1 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e1.clone(), after: e1_1.clone() });
            op2 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref, before: e2.clone(), after: e2_1.clone() });
            source = List::foldr(list![op1, op2], (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone())?;
            (Arc::new(BackendDAE::Equation::EQUATION { exp: e1_1, scalar: e2_1, source: source.clone(), attr: eqAttr.clone() }), funcs)
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, source, attr: eqAttr } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut op1: Arc<DAE::SymbolicOperation>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut source = (*source).clone();
            (e1_1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData, inDiffType, inFunctionTree, defaultMaxIter.clone())?;
            (e1_1, _) = ExpressionSimplify::simplify(e1_1)?;
            op1 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref, before: e1.clone(), after: e1_1.clone() });
            source = List::foldr(list![op1], (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone())?;
            (Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1_1, source: source.clone(), attr: eqAttr.clone() }), funcs)
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size, left: e1, right: e2, source, attr: eqAttr } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut op1: Arc<DAE::SymbolicOperation>;
            let mut op2: Arc<DAE::SymbolicOperation>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut source = (*source).clone();
            (e1_1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, defaultMaxIter.clone())?;
            (e1_1, _) = ExpressionSimplify::simplify(e1_1)?;
            (e2_1, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData, inDiffType, funcs, defaultMaxIter.clone())?;
            (e2_1, _) = ExpressionSimplify::simplify(e2_1)?;
            op1 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e1.clone(), after: e1_1.clone() });
            op2 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref, before: e2.clone(), after: e2_1.clone() });
            source = List::foldr(list![op1, op2], (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone())?;
            (Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: e1_1, right: e2_1, source: source.clone(), attr: eqAttr.clone() }), funcs)
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize, left: e1, right: e2, source, attr: eqAttr, recordSize } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut op1: Arc<DAE::SymbolicOperation>;
            let mut op2: Arc<DAE::SymbolicOperation>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut source = (*source).clone();
            (e1_1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, defaultMaxIter.clone())?;
            (e1_1, _) = ExpressionSimplify::simplify(e1_1)?;
            (e2_1, funcs) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData, inDiffType, funcs, defaultMaxIter.clone())?;
            (e2_1, _) = ExpressionSimplify::simplify(e2_1)?;
            op1 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref.clone(), before: e1.clone(), after: e1_1.clone() });
            op2 = Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: inDiffwrtCref, before: e2.clone(), after: e2_1.clone() });
            source = List::foldr(list![op1, op2], (std::sync::Arc::new(ElementSource::addSymbolicTransformation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ElementSource>, Arc<DAE::SymbolicOperation>) -> Result<Arc<DAE::ElementSource>> + 'static>), source.clone())?;
            (Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimSize.clone(), left: e1_1, right: e2_1, source: source.clone(), attr: eqAttr.clone(), recordSize: recordSize.clone() }), funcs)
        },
        Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst }, source, expand, attr: eqAttr } => {
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut alg: Arc<DAE::Algorithm>;
            let mut statementLst = (*statementLst).clone();
            (statementLst, funcs) = differentiateStatements(statementLst.clone(), inDiffwrtCref, inInputData, inDiffType, metamodelica::nil(), inFunctionTree, defaultMaxIter.clone())?;
            alg = Arc::new(DAE::Algorithm { statementLst: statementLst.clone() });
            (Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg, source: source.clone(), expand: expand.clone(), attr: eqAttr.clone() }), funcs)
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { conditions: expExpLst, eqnstrue: eqnslst, eqnsfalse: eqns, source, attr: eqAttr } => {
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut eqnslst = (*eqnslst).clone();
            let mut eqns = (*eqns).clone();
            (eqnslst, funcs) = differentiateEquationsLst(eqnslst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree)?;
            (eqns, funcs) = differentiateEquations(eqns.clone(), inDiffwrtCref, inInputData, inDiffType, metamodelica::nil(), funcs)?;
            (Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: expExpLst.clone(), eqnstrue: eqnslst.clone(), eqnsfalse: eqns.clone(), source: source.clone(), attr: eqAttr.clone() }), funcs)
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { size, whenEquation: whenEqn, source, attr: eqAttr } => {
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut whenEqn = (*whenEqn).clone();
            (whenEqn, funcs) = differentiateWhenEquations(whenEqn.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree)?;
            (Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEqn.clone(), source: source.clone(), attr: eqAttr.clone() }), funcs)
        },
        _ => {
            Error::addSourceMessage(Error::NON_EXISTING_DERIVATIVE.clone(), list![(BackendDump::equationString(inEquation)?).clone(), (ComponentReference::crefStr(inDiffwrtCref)?).clone()], metamodelica::sourceInfo!("BackEnd/Differentiate.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
        BackendDump::debugStrEqnStr((literal!("### Result of differentiateEquation\n --> ")).clone(), outEquation.clone(), (literal!("\n")).clone())?;
    }
    Ok((outEquation, outFunctionTree))
}

fn differentiateEquations(mut inEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inEquationsAccum: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree>;
    (outEquations, outFunctionTree) = 'mc: {
        let __mc_input = inEquations;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((inEquationsAccum.clone().reverse(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: eqn, tail: rest } => {
                    let mut funcs: Arc<AvlTreePathFunction::Tree>;
                    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut eqn = (*eqn).clone();
                    (eqn, funcs) = differentiateEquation(eqn.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone())?;
                    eqns = metamodelica::cons(eqn.clone(), inEquationsAccum.clone());
                    (eqns, funcs) = differentiateEquations(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), eqns.clone(), funcs.clone())?;
                    Ok((eqns.clone(), funcs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: eqn, tail: _ } => {
                    Error::addSourceMessage(Error::NON_EXISTING_DERIVATIVE.clone(), list![(BackendDump::equationString(eqn.clone())?).clone(), (ComponentReference::crefStr(inDiffwrtCref.clone())?).clone()], metamodelica::sourceInfo!("BackEnd/Differentiate.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEquations, outFunctionTree))
}

fn differentiateEquationsLst(mut inEquationsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inEquationsLstAccum: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outEquationsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree>;
    (outEquationsLst, outFunctionTree) = 'mc: {
        let __mc_input = inEquationsLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((inEquationsLstAccum.clone().reverse(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: eqns, tail: rest } => {
                    let mut funcs: Arc<AvlTreePathFunction::Tree>;
                    let mut eqnsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>;
                    let mut eqns = (*eqns).clone();
                    (eqns, funcs) = differentiateEquations(eqns.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone())?;
                    eqnsLst = metamodelica::cons(eqns.clone(), inEquationsLstAccum.clone());
                    (eqnsLst, funcs) = differentiateEquationsLst(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), eqnsLst.clone(), funcs.clone())?;
                    Ok((eqnsLst.clone(), funcs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: eqns, tail: _ } => {
                    Error::addSourceMessage(Error::NON_EXISTING_DERIVATIVE.clone(), list![(BackendDump::equationListString(eqns.clone(), (literal!("equation list")).clone())?).clone(), (ComponentReference::crefStr(inDiffwrtCref.clone())?).clone()], metamodelica::sourceInfo!("BackEnd/Differentiate.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEquationsLst, outFunctionTree))
}

fn differentiateWhenEquations(mut inWhenEquations: Arc<BackendDAE::WhenEquation>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<BackendDAE::WhenEquation>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outWhenEquations: Arc<BackendDAE::WhenEquation>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree>;
    let mut elsewhenPart: Arc<BackendDAE::WhenEquation>;
    let mut delsewhenPart: Arc<BackendDAE::WhenEquation>;
    let mut oelsepart: Option<Arc<BackendDAE::WhenEquation>>;
    let mut whenStmtLst: Arc<metamodelica::List<BackendDAE::WhenOperator>>;
    let mut stmtLst: Arc<metamodelica::List<BackendDAE::WhenOperator>>;
    let mut funcs: Arc<AvlTreePathFunction::Tree>;
    let mut condition: Arc<DAE::Exp>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inWhenEquations) {
        Deref @ BackendDAE::WhenEquation { condition: __pa0, whenStmtLst: __pa1, elsewhenPart: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    condition = __pa0.clone();
    whenStmtLst = __pa1.clone();
    oelsepart = __pa2.clone();
    funcs = inFunctionTree;
    stmtLst = metamodelica::nil();
    for mut rs in &*whenStmtLst {
        let mut rs = rs.clone();
        rs = (match rs.clone() {
        BackendDAE::WhenOperator::ASSIGN { left: ref eleft, right: mut right, source: ref src } => {
            let mut dright: Arc<DAE::Exp>;
            let mut dleft: Arc<DAE::Exp>;
            (dleft, funcs) = differentiateExp(eleft.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), defaultMaxIter.clone())?;
            (dright, funcs) = differentiateExp(right.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs.clone(), defaultMaxIter.clone())?;
            BackendDAE::WhenOperator::ASSIGN { left: dleft.clone(), right: dright.clone(), source: src.clone() }
        },
        _ => {
            rs.clone()
        },
    });
        stmtLst = metamodelica::cons(rs.clone(), stmtLst.clone());
    }
    if isSome(oelsepart.clone()) {
        let __pa3 = ::match_deref::match_deref! { match &(oelsepart) {
            Some(__pa3) => __pa3.clone(),
            _ => bail!("pattern mismatch"),
        } };
        elsewhenPart = __pa3.clone();
        (delsewhenPart, funcs) = differentiateWhenEquations(elsewhenPart, inDiffwrtCref, inInputData, inDiffType, funcs)?;
        oelsepart = Some(delsewhenPart);
    } else {
        oelsepart = None;
    }
    outWhenEquations = Arc::new(BackendDAE::WhenEquation { condition: condition, whenStmtLst: stmtLst, elsewhenPart: oelsepart });
    outFunctionTree = funcs;
    Ok((outWhenEquations, outFunctionTree))
}

// =============================================================================
// main differentiation functions
//  - differentiateExp
//  - differentiateStatements
//
// =============================================================================
fn differentiateExp(mut inExp: Arc<DAE::Exp>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree>;
    let debug: bool = false;
    if debug {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nDifferentiate Exp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" w.r.t. ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    (outDiffedExp, outFunctionTree) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::SCONST { .. } => {
            (inExp, inFunctionTree)
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            (inExp, inFunctionTree)
        },
        Deref @ DAE::Exp::CLKCONST { .. } => {
            (inExp, inFunctionTree)
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            (inExp, inFunctionTree)
        },
        Deref @ DAE::Exp::ICONST { .. } => {
            (Arc::new(DAE::Exp::ICONST { integer: 0 }), inFunctionTree)
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            (Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), inFunctionTree)
        },
        Deref @ DAE::Exp::CREF { componentRef: cref, ty: tp } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree>;
            if ComponentReference::isStartCref(cref.clone()) {
                res = Expression::makeConstZero(tp.clone());
                functionTree = inFunctionTree;
            } else {
                (res, functionTree) = differentiateCrefs(inExp, inDiffwrtCref.clone(), inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            }
            (res, functionTree)
        },
        Deref @ DAE::Exp::BINARY { .. } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree>;
            (res, functionTree) = differentiateBinary(inExp, inDiffwrtCref.clone(), inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            res = ExpressionSimplify::simplifyBinaryExp(res)?;
            (res, functionTree)
        },
        Deref @ DAE::Exp::UNARY { operator: op, exp: e1 } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree>;
            (res, functionTree) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            res = Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: res });
            res = ExpressionSimplify::simplifyUnaryExp(res);
            (res, functionTree)
        },
        Deref @ DAE::Exp::LBINARY { .. } => {
            (inExp, inFunctionTree)
        },
        Deref @ DAE::Exp::LUNARY { .. } => {
            (inExp, inFunctionTree)
        },
        Deref @ DAE::Exp::RELATION { .. } => {
            (inExp, inFunctionTree)
        },
        Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 } => {
            let mut res: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree>;
            (res1, functionTree) = differentiateExp(e2.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter - 1)?;
            (res2, functionTree) = differentiateExp(e3.clone(), inDiffwrtCref.clone(), inInputData, inDiffType, functionTree, maxIter - 1)?;
            res = Arc::new(DAE::Exp::IFEXP { expCond: e1.clone(), expThen: res1, expElse: res2 });
            (res, _) = ExpressionSimplify::simplify1(res)?;
            (res, functionTree)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, expLst: Deref @ metamodelica::List::Cons { head: actual, tail: Deref @ metamodelica::List::Cons { head: simplified, tail: Deref @ metamodelica::List::Nil } }, .. } => {
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut e3: Arc<DAE::Exp>;
            let mut lambda: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree>;
            lambda = Expression::crefExp(ComponentReferenceBasics::makeCrefIdent((arcstr::literal!(BackendDAE::homotopyLambda)).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil()))?;
            (e1, functionTree) = differentiateExp(actual.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (e2, functionTree) = differentiateExp(simplified.clone(), inDiffwrtCref.clone(), inInputData, inDiffType, functionTree, maxIter)?;
            e3 = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: lambda.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e1.clone() }), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: lambda }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e2.clone() }) });
            (e3.clone(), functionTree)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: e3, tail: Deref @ metamodelica::List::Nil } } }, .. } if (Expression::expHasCref(e2.clone(), inDiffwrtCref.clone())? || Expression::expHasCref(e3.clone(), inDiffwrtCref.clone())?) => {
            bail!("fail")
        },
        Deref @ DAE::Exp::CALL { .. } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree>;
            (res, functionTree) = differentiateCalls(inExp, inDiffwrtCref.clone(), inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            (res, _) = ExpressionSimplify::simplify1(res)?;
            (res, functionTree)
        },
        Deref @ DAE::Exp::RECORD { path: p, exps: expl, comp: strLst, ty: tp } => {
            let mut e1: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree>;
            let mut sub: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            sub = metamodelica::nil();
            functionTree = inFunctionTree;
            for mut e in &*expl.clone() {
                let mut e = e.clone();
                (e1, functionTree) = differentiateExp(e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functionTree.clone(), maxIter)?;
                sub = metamodelica::cons(e1.clone(), sub.clone());
            }
            (Arc::new(DAE::Exp::RECORD { path: p.clone(), exps: sub.reverse(), comp: strLst.clone(), ty: tp.clone() }), functionTree)
        },
        Deref @ DAE::Exp::ARRAY { ty: tp, scalar: b, array: expl } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree>;
            let mut expl = (*expl).clone();
            (expl, functionTree) = List::map3Fold(expl.clone(), (std::sync::Arc::new({ let __pe_b5 = maxIter - 1; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, BackendDAE::DifferentiateInputData, BackendDAE::DifferentiationType, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>), inDiffwrtCref.clone(), inInputData, inDiffType, inFunctionTree)?;
            res = Arc::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: b.clone(), array: expl.clone() });
            (res, _) = ExpressionSimplify::simplify1(res)?;
            (res, functionTree)
        },
        Deref @ DAE::Exp::MATRIX { ty: tp, integer: i, matrix } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree>;
            let mut dmatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
            (dmatrix, functionTree) = List::mapFoldList(matrix.clone(), (std::sync::Arc::new({ let __pe_b1 = inDiffwrtCref.clone(); let __pe_b2 = inInputData; let __pe_b3 = inDiffType; let __pe_b5 = maxIter - 1; move |__pe_a0, __pe_a4| differentiateExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4, __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>), inFunctionTree)?;
            res = Arc::new(DAE::Exp::MATRIX { ty: tp.clone(), integer: i.clone(), matrix: dmatrix });
            (res, _) = ExpressionSimplify::simplify1(res)?;
            (res, functionTree)
        },
        Deref @ DAE::Exp::RANGE { .. } => {
            (inExp, inFunctionTree)
        },
        Deref @ DAE::Exp::TUPLE { PR: expl } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree>;
            let mut expl = (*expl).clone();
            (expl, functionTree) = List::map3Fold(expl.clone(), (std::sync::Arc::new({ let __pe_b5 = maxIter - 1; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, BackendDAE::DifferentiateInputData, BackendDAE::DifferentiationType, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>), inDiffwrtCref.clone(), inInputData, inDiffType, inFunctionTree)?;
            res = Arc::new(DAE::Exp::TUPLE { PR: expl.clone() });
            (res, _) = ExpressionSimplify::simplify1(res)?;
            (res, functionTree)
        },
        Deref @ DAE::Exp::CAST { ty: tp, exp: e1 } => {
            let mut res: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree>;
            (res, functionTree) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            (res, _) = ExpressionSimplify::simplify1(res)?;
            (Arc::new(DAE::Exp::CAST { ty: tp.clone(), exp: res }), functionTree)
        },
        Deref @ DAE::Exp::ASUB { exp: e1, sub: subs } => {
            let mut res: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree>;
            (res1, functionTree) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            res = Expression::makeASUB(res1, ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = Expression::getSubscriptExp(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
            (res, _) = ExpressionSimplify::simplify1(res)?;
            (res, functionTree)
        },
        Deref @ DAE::Exp::TSUB { exp: e1, ix: i, ty: tp } => {
            let mut res: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree>;
            (res1, functionTree) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            if !(referenceEq(&*(e1.clone()),&*(res1.clone()))) {
                res = Arc::new(DAE::Exp::TSUB { exp: res1, ix: i.clone(), ty: tp.clone() });
                (res, _) = ExpressionSimplify::simplify1(res)?;
            } else {
                res = inExp;
            }
            (res, functionTree)
        },
        e1 @ Deref @ DAE::Exp::RSUB { .. } => {
            let mut p1: Arc<Absyn::Path> = Arc::new(<Absyn::Path as ::std::default::Default>::default());
            let mut p2: Arc<Absyn::Path> = Arc::new(<Absyn::Path as ::std::default::Default>::default());
            let mut b: bool;
            let mut res: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree>;
            let mut strLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut varLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut e1 = (*e1).clone();
            (res, b) = ExpressionSimplify::simplify(e1.clone())?;
            if b {
                (res, functionTree) = differentiateExp(res, inDiffwrtCref.clone(), inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            } else {
                (res1, functionTree) = differentiateExp(var_field!((*e1).exp, DAE::Exp::RSUB).clone(), inDiffwrtCref.clone(), inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
                if !(referenceEq(&*(var_field!((*e1).exp, DAE::Exp::RSUB).clone()),&*(res1.clone()))) {
                    match '__try0: {
                        (expl, strLst) = (::match_deref::match_deref! { match &(res1.clone()) {
        Deref @ DAE::Exp::RECORD { exps: __esc_expl, comp: __esc_strLst, .. } => {
            expl = (*__esc_expl).clone();
            strLst = (*__esc_strLst).clone();
            (expl.clone(), strLst.clone())
        },
        Deref @ DAE::Exp::CALL { path: p1, expLst: __esc_expl, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p2 }, varLst: __esc_varLst, .. }, .. } } if (AbsynUtil::pathEqual(p1.clone(), p2.clone())) => {
            expl = (*__esc_expl).clone();
            varLst = (*__esc_varLst).clone();
            (expl.clone(), ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (varLst.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))
        },
        _ => bail!("match: no arm matched"),
    } });
                        res = unwrap_break_err!((expl.clone()).get(unwrap_break_err!(List::position1OnTrue(strLst.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (var_field!((*e1).fieldName, DAE::Exp::RSUB).clone()).clone()), '__try0)), '__try0);
                        Ok::<_, anyhow::Error>((res.clone(),))
                    } {
                        Ok((__try0_o0,)) => {
                            res = __try0_o0;
                        }
                        Err(_) => {
                            assign_variant_field!(e1 => DAE::Exp::RSUB; exp = res1.clone());
                            (res, _) = ExpressionSimplify::simplify1(e1.clone())?;
                        }
                    }
                }
            }
            (res, functionTree)
        },
        Deref @ DAE::Exp::SIZE { .. } => {
            (inExp, inFunctionTree)
        },
        Deref @ DAE::Exp::REDUCTION { .. } => {
            let mut res: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut functionTree: Arc<AvlTreePathFunction::Tree>;
            (res1, functionTree) = differentiateExp(var_field!((*inExp).expr, DAE::Exp::REDUCTION).clone(), inDiffwrtCref.clone(), inInputData, inDiffType, inFunctionTree, maxIter - 1)?;
            if !(referenceEq(&*(var_field!((*inExp).expr, DAE::Exp::REDUCTION).clone()),&*(res1.clone()))) {
                res = Arc::new(DAE::Exp::REDUCTION { reductionInfo: var_field!((*inExp).reductionInfo, DAE::Exp::REDUCTION).clone(), expr: res1, iterators: var_field!((*inExp).iterators, DAE::Exp::REDUCTION).clone() });
                (res, _) = ExpressionSimplify::simplify1(res)?;
            } else {
                res = inExp;
            }
            (res, functionTree)
        },
        _ => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut stp: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            s1 = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
            s2 = (ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref.clone())?).clone();
            stp = (TypesDump::printTypeStr(Expression::r#typeof(inExp)?)).clone();
            Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- differentiateExp ")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(" type: ")); __mm_s.push_str(&*stp); __mm_s.push_str(&*literal!(" w.r.t ")); __mm_s.push_str(&*s2); __mm_s.push_str(&*literal!(" failed\n")); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if debug {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Differentiate-Exp-result: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(outDiffedExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((outDiffedExp, outFunctionTree))
}

fn differentiateStatements(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inStmtsAccum: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree>;
    (outDiffedStmts, outFunctionTree) = 'mc: {
        let __mc_input = inStmts.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((inStmtsAccum.clone().reverse(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_ASSIGN { type_, exp1: lhs, exp: rhs, source }, tail: restStatements } => {
                    let mut derivedLHS: Arc<DAE::Exp>;
                    let mut derivedRHS: Arc<DAE::Exp>;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    (derivedLHS, functions) = differentiateExp(lhs.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter)?;
                    (derivedRHS, functions) = differentiateExp(rhs.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functions.clone(), maxIter)?;
                    (derivedRHS, _) = ExpressionSimplify::simplify(derivedRHS.clone())?;
                    if Expression::isZero(derivedLHS.clone())? {
                        derivedStatements1 = list![currStatement.clone()];
                    } else {
                        derivedStatements1 = list![Arc::new(DAE::Statement::STMT_ASSIGN { type_: type_.clone(), exp1: derivedLHS.clone(), exp: derivedRHS.clone(), source: source.clone() }), currStatement.clone()];
                    }
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: expLst, exp: rhs, source, .. }, tail: restStatements } => {
                    let mut derivedRHS: Arc<DAE::Exp>;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut dexpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut expLstRHS: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut exptl: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<DAE::Exp>)>>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut optDerivedStatements1: Arc<metamodelica::List<Option<Arc<DAE::Statement>>>>;
                    (dexpLst, functions) = List::map3Fold(expLst.clone(), (std::sync::Arc::new({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, BackendDAE::DifferentiateInputData, BackendDAE::DifferentiationType, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone())?;
                    let (__pa1, __pa0, __pa2) = ::match_deref::match_deref! { match &(differentiateExp(rhs.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functions.clone(), maxIter)?) {
                        (__pa1 @ Deref @ DAE::Exp::TUPLE { PR: __pa0 }, __pa2) => (__pa1.clone(), __pa0.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    expLstRHS = __pa0.clone();
                    derivedRHS = __pa1.clone();
                    functions = __pa2.clone();
                    let __pa3 = ::match_deref::match_deref! { match &(ExpressionSimplify::simplify(derivedRHS.clone())?) {
                        (Deref @ DAE::Exp::TUPLE { PR: __pa3 }, _) => __pa3.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    expLstRHS = __pa3.clone();
                    exptl = List::zip(dexpLst.clone(), expLstRHS.clone());
                    optDerivedStatements1 = List::map2(exptl.clone(), (std::sync::Arc::new(makeAssignmentfromTuple) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::Exp>, Arc<DAE::Exp>), Arc<DAE::ElementSource>, Arc<AvlTreePathFunction::Tree>) -> Result<Option<Arc<DAE::Statement>>> + 'static>), source.clone(), inFunctionTree.clone())?;
                    derivedStatements1 = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
        for mut s in (optDerivedStatements1.clone()).into_iter().cloned() {
                    if !(isSome(s.clone())) { continue; }
                    let __x = Util::getOption(s.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    derivedStatements2 = listAppend(derivedStatements1.clone(), list![currStatement.clone()]);
                    derivedStatements1 = listAppend(derivedStatements2.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements1.clone(), functions.clone(), maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: expLst, exp: rhs @ Deref @ DAE::Exp::CALL { .. }, type_, source }, tail: restStatements } => {
                    let mut derivedRHS: Arc<DAE::Exp>;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut dexpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut optDerivedStatements1: Arc<metamodelica::List<Option<Arc<DAE::Statement>>>>;
                    let mut type_ = (*type_).clone();
                    (dexpLst, functions) = List::map3Fold(expLst.clone(), (std::sync::Arc::new({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, BackendDAE::DifferentiateInputData, BackendDAE::DifferentiationType, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone())?;
                    let (__pa1, __pa0, __pa2) = ::match_deref::match_deref! { match &(differentiateExp(rhs.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functions.clone(), maxIter)?) {
                        (__pa1 @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: __pa0, .. }, .. }, __pa2) => (__pa1.clone(), __pa0.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    type_ = __pa0.clone();
                    derivedRHS = __pa1.clone();
                    functions = __pa2.clone();
                    optDerivedStatements1 = list![Some(Arc::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: type_.clone(), expExpLst: dexpLst.clone(), exp: derivedRHS.clone(), source: source.clone() }))];
                    derivedStatements1 = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
        for mut s in (optDerivedStatements1.clone()).into_iter().cloned() {
                    if !(isSome(s.clone())) { continue; }
                    let __x = Util::getOption(s.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    derivedStatements2 = listAppend(derivedStatements1.clone(), list![currStatement.clone()]);
                    derivedStatements1 = listAppend(derivedStatements2.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements1.clone(), functions.clone(), maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs, exp: rhs, type_, source }, tail: restStatements } => {
                    let mut derivedLHS: Arc<DAE::Exp>;
                    let mut derivedRHS: Arc<DAE::Exp>;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    (derivedLHS, functions) = differentiateExp(lhs.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter)?;
                    (derivedRHS, functions) = differentiateExp(rhs.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functions.clone(), maxIter)?;
                    (derivedRHS, _) = ExpressionSimplify::simplify(derivedRHS.clone())?;
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_ASSIGN_ARR { type_: type_.clone(), lhs: derivedLHS.clone(), exp: derivedRHS.clone(), source: source.clone() }), currStatement.clone()];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_FOR { type_, iterIsArray, iter: ident, range: exp, statementLst, source }, tail: restStatements } => {
                    let mut cref: Arc<DAE::ComponentRef>;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut inputData: BackendDAE::DifferentiateInputData;
                    let mut controlVar: BackendDAE::Var;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    cref = ComponentReferenceBasics::makeCrefIdent((ident.clone()).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil());
                    controlVar = BackendDAE::Var { varName: cref.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::DISCRETE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
                    inputData = addGlobalVars(list![controlVar.clone()], inInputData.clone())?;
                    (derivedStatements1, functions) = differentiateStatements(statementLst.clone(), inDiffwrtCref.clone(), inputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone(), maxIter)?;
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_FOR { type_: type_.clone(), iterIsArray: iterIsArray.clone(), iter: (ident.clone()).clone(), range: exp.clone(), statementLst: derivedStatements1.clone(), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_IF { exp, statementLst, else_: Deref @ DAE::Else::NOELSE { .. }, source }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    (derivedStatements1, functions) = differentiateStatements(statementLst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone(), maxIter)?;
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: derivedStatements1.clone(), else_: openmodelica_frontend_types::DAE::Else::interned_NOELSE(), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_IF { exp, statementLst, else_: Deref @ DAE::Else::ELSEIF { exp: elseif_exp, statementLst: elseif_statementLst, else_: elseif_else_ }, source }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    (derivedStatements1, functions) = differentiateStatements(statementLst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone(), maxIter)?;
                    (derivedStatements2, functions) = differentiateStatements(list![Arc::new(DAE::Statement::STMT_IF { exp: elseif_exp.clone(), statementLst: elseif_statementLst.clone(), else_: elseif_else_.clone(), source: source.clone() })], inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), functions.clone(), maxIter)?;
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: derivedStatements1.clone(), else_: Arc::new(DAE::Else::ELSE { statementLst: derivedStatements2.clone() }), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_IF { exp, statementLst, else_: Deref @ DAE::Else::ELSE { statementLst: else_statementLst }, source }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    (derivedStatements1, functions) = differentiateStatements(statementLst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone(), maxIter)?;
                    (derivedStatements2, functions) = differentiateStatements(else_statementLst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), functions.clone(), maxIter)?;
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: derivedStatements1.clone(), else_: Arc::new(DAE::Else::ELSE { statementLst: derivedStatements2.clone() }), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHILE { exp, statementLst, source }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    (derivedStatements1, functions) = differentiateStatements(statementLst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone(), maxIter)?;
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_WHILE { exp: exp.clone(), statementLst: derivedStatements1.clone(), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHEN { exp, initialCall, statementLst, elseWhen: None, source, .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    (derivedStatements1, functions) = differentiateStatements(statementLst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone(), maxIter)?;
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_WHEN { exp: exp.clone(), conditions: metamodelica::nil(), initialCall: initialCall.clone(), statementLst: derivedStatements1.clone(), elseWhen: None, source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_WHEN { exp, initialCall, statementLst, elseWhen: Some(stmt), source, .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut dstmt: Arc<DAE::Statement>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    (derivedStatements1, functions) = differentiateStatements(statementLst.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), inFunctionTree.clone(), maxIter)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(differentiateStatements(list![stmt.clone()], inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), metamodelica::nil(), functions.clone(), maxIter)?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    dstmt = __pa0.clone();
                    functions = __pa1.clone();
                    derivedStatements1 = list![Arc::new(DAE::Statement::STMT_WHEN { exp: exp.clone(), conditions: metamodelica::nil(), initialCall: initialCall.clone(), statementLst: derivedStatements1.clone(), elseWhen: Some(dstmt.clone()), source: source.clone() })];
                    derivedStatements2 = listAppend(derivedStatements1.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements2.clone(), functions.clone(), maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSERT { .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    (derivedStatements1, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inStmtsAccum.clone(), inFunctionTree.clone(), maxIter)?;
                    Ok((derivedStatements1.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_TERMINATE { .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    let mut derivedStatements2: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    derivedStatements1 = metamodelica::cons(currStatement.clone(), inStmtsAccum.clone());
                    (derivedStatements2, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements1.clone(), inFunctionTree.clone(), maxIter)?;
                    Ok((derivedStatements2.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_REINIT { .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    derivedStatements1 = metamodelica::cons(currStatement.clone(), inStmtsAccum.clone());
                    (derivedStatements1, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements1.clone(), inFunctionTree.clone(), maxIter)?;
                    Ok((derivedStatements1.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_NORETCALL { .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    derivedStatements1 = metamodelica::cons(currStatement.clone(), inStmtsAccum.clone());
                    (derivedStatements1, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements1.clone(), inFunctionTree.clone(), maxIter)?;
                    Ok((derivedStatements1.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_RETURN { .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    derivedStatements1 = metamodelica::cons(currStatement.clone(), inStmtsAccum.clone());
                    (derivedStatements1, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements1.clone(), inFunctionTree.clone(), maxIter)?;
                    Ok((derivedStatements1.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currStatement @ Deref @ DAE::Statement::STMT_BREAK { .. }, tail: restStatements } => {
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut derivedStatements1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
                    derivedStatements1 = metamodelica::cons(currStatement.clone(), inStmtsAccum.clone());
                    (derivedStatements1, functions) = differentiateStatements(restStatements.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), derivedStatements1.clone(), inFunctionTree.clone(), maxIter)?;
                    Ok((derivedStatements1.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut currStatement: Arc<DAE::Statement>;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        let __pa0 = ::match_deref::match_deref! { match &(inStmts.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        currStatement = __pa0.clone();
                        s1 = (DAEDump::ppStatementStr(currStatement.clone())).clone();
                        s2 = (ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref.clone())?).clone();
                        Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- differentiateStatements ")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" w.r.t: ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(" failed\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outDiffedStmts, outFunctionTree))
}

fn isDiscreteAssignStatment(mut inStmt: Arc<DAE::Statement>) -> bool {
    let mut out: bool;
    out = (::match_deref::match_deref! { match &(inStmt) {
        Deref @ DAE::Statement::STMT_ASSIGN { type_: tp, .. } => {
            Types::isDiscreteType(tp.clone())
        },
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { type_: tp, .. } => {
            Types::isDiscreteType(tp.clone())
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp, .. } => {
            Types::isDiscreteType(tp.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

fn makeAssignmentfromTuple(mut inTpl: (Arc<DAE::Exp>, Arc<DAE::Exp>), mut source: Arc<DAE::ElementSource>, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>) -> Result<Option<Arc<DAE::Statement>>> {
    let mut outStmt: Option<Arc<DAE::Statement>>;
    outStmt = (::match_deref::match_deref! { match &(inTpl) {
        (e1 @ Deref @ DAE::Exp::CREF { ty: tp, .. }, e2) => {
            Some(Arc::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: e1.clone(), exp: e2.clone(), source: source }))
        },
        (e1 @ Deref @ DAE::Exp::CALL { .. }, e2) if (Expression::isRecordCall(e1.clone(), inFunctionTree.clone())?) => {
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(e1.clone())?;
            Some(Arc::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: e1.clone(), exp: e2.clone(), source: source }))
        },
        (e1, e2) if (Expression::isZero(e1.clone())?) => {
            None
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStmt)
}

// =============================================================================
// help functions for differentiation
//  - differentiateCrefs
//  - differentiateCalls
//  - differentiateBinary (e.g.: ADD, SUB, MUL, DIV, POW, ...
//
// =============================================================================
fn differentiateCrefs(mut inExp: Arc<DAE::Exp>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let debug: bool = false;
    if debug {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nDifferentiate Exp-Cref: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" w.r.t. ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    (outDiffedExp, outFunctionTree) = ({
        let mut diffed_exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        'mc: {
        let __mc_input = (inExp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. } }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION { .. }) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    cr = ComponentReference::prependStringCref((arcstr::literal!(BackendDAE::functionDerivativeNamePrefix)).clone(), cr.clone())?;
                    cr = ComponentReference::prependStringCref((matrixName.clone()).clone(), cr.clone())?;
                    res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp @ Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path }, .. } }, _, _, _) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = outFunctionTree.clone();
                    expl = List::map1(varLst.clone(), (std::sync::Arc::new(Expression::generateCrefsExpFromExpVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), cr.clone())?;
                    (expl_1, outFunctionTree) = List::map3Fold(expl.clone(), (std::sync::Arc::new({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, BackendDAE::DifferentiateInputData, BackendDAE::DifferentiationType, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone())?;
                    res = Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expl_1.clone(), attr: Arc::new(DAE::CallAttributes { ty: tp.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) });
                    Ok(((res.clone(), outFunctionTree.clone()), outFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outFunctionTree = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp @ Deref @ DAE::Type::T_ARRAY { .. } }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION { .. }) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    cr = ComponentReference::prependStringCref((arcstr::literal!(BackendDAE::functionDerivativeNamePrefix)).clone(), cr.clone())?;
                    cr = ComponentReference::prependStringCref((matrixName.clone()).clone(), cr.clone())?;
                    res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, _, _, diffType) => {
                    if !(((match diffType.clone() {
        BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. } => false,
        _ => true,
    }))) { bail!("guard") }
                    let mut e1: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = outFunctionTree.clone();
                    let true = (Flags::isSet(Flags::NF_SCALARIZE.clone())?) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e.clone(), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    (res, outFunctionTree) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter)?;
                    Ok(((res.clone(), outFunctionTree.clone()), outFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outFunctionTree = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, _, _, _) => {
                    Ok((e.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, _, _) => {
                    let mut one: Arc<DAE::Exp>;
                    let true = (ComponentReferenceBasics::crefEqual(cr.clone(), inDiffwrtCref.clone())?) else { bail!("pattern mismatch") };
                    (one, _) = Expression::makeOneExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((one.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, .. }, _, _, BackendDAE::DifferentiationType::SIMPLE_DIFFERENTIATION { .. }) => {
                    let mut zero: Arc<DAE::Exp>;
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, .. }, _, _, BackendDAE::DifferentiationType::DIFF_FULL_JACOBIAN { .. }) => {
                    let mut zero: Arc<DAE::Exp>;
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { knownVars: Some(knvars), .. }, _) => {
                    let mut var: BackendDAE::Var;
                    let mut zero: Arc<DAE::Exp>;
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), knvars.clone())?;
                    let false = (BackendVariable::isVarOnTopLevelAndInput(var.clone())) else { bail!("pattern mismatch") };
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { allVars: Some(timevars), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
                    let mut kind: BackendDAE::VarKind;
                    let mut zero: Arc<DAE::Exp>;
                    let (BackendDAE::VAR { varKind: __pa0, .. }, _) = (BackendVariable::getVarSingle(cr.clone(), timevars.clone())?) else { bail!("pattern mismatch") };
                    kind = __pa0.clone();
                    let true = (listMember(kind.clone(), list![openmodelica_backend_types::BackendDAE::VarKind::DISCRETE]) || !(Types::isReal(tp.clone()))) else { bail!("pattern mismatch") };
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { dependenentVars: Some(timevars), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
                    let mut var: BackendDAE::Var;
                    let mut res: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), timevars.clone())?;
                    let true = (BackendVariable::isDummyStateVar(var.clone())) else { bail!("pattern mismatch") };
                    cr = ComponentReference::crefPrefixDer(cr.clone());
                    res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { dependenentVars: Some(timevars), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
                    let mut res: Arc<DAE::Exp>;
                    BackendVariable::getVarSingle(cr.clone(), timevars.clone())?;
                    res = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![e.clone()], attr: Arc::new(DAE::CallAttributes { ty: tp.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) });
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { dependenentVars: Some(timevars), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION { .. }) => {
                    let mut zero: Arc<DAE::Exp>;
                    let mut cr1: Arc<DAE::ComponentRef>;
                    cr1 = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
                    BackendVariable::getVar(cr1.clone(), timevars.clone())?;
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { dependenentVars: Some(timevars), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION { .. }) => {
                    let mut zero: Arc<DAE::Exp>;
                    BackendVariable::getVar(cr.clone(), timevars.clone())?;
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION { .. }) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    cr = ComponentReference::prependStringCref((arcstr::literal!(BackendDAE::functionDerivativeNamePrefix)).clone(), cr.clone())?;
                    cr = ComponentReference::prependStringCref((matrixName.clone()).clone(), cr.clone())?;
                    res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "$", .. }, _, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }) => {
                    let mut res: Arc<DAE::Exp>;
                    (res, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { independenentVars: Some(timevars), matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }) => {
                    let mut scalarLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut arrayType: Arc<DAE::Type>;
                    let mut res: Arc<DAE::Exp>;
                    let mut res1: Arc<DAE::Exp>;
                    let mut scalarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut cr = (*cr).clone();
                    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = outFunctionTree.clone();
                    (scalarLst, _) = BackendVariable::getVar(cr.clone(), timevars.clone())?;
                    arrayType = ComponentReference::crefTypeFull(cr.clone())?;
                    if !(scalarLst.clone().is_empty()) && (scalarLst.clone().len() as i32) != Types::getDimensionProduct(arrayType.clone())? {
                        scalarCrefs = ComponentReference::expandCref(cr.clone(), true)?;
                        outFunctionTree = inFunctionTree.clone();
                        for mut cref in &*scalarCrefs.clone() {
                            let mut cref = cref.clone();
                            (res1, outFunctionTree) = differentiateCrefs(Expression::crefExp(cref.clone())?, inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), outFunctionTree.clone(), maxIter)?;
                            diffed_exps = metamodelica::cons(res1.clone(), diffed_exps.clone());
                        }
                        res = Expression::listToArray(diffed_exps.clone().reverse(), TypesDump::getDimensions(arrayType.clone()))?;
                    } else {
                        cr = createSeedCrefName(cr.clone(), (matrixName.clone()).clone())?;
                        res = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
                    }
                    Ok(((res.clone(), inFunctionTree.clone()), outFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outFunctionTree = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { allVars: Some(timevars), matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }) => {
                    let mut var: BackendDAE::Var;
                    let mut res: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), timevars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    var = __pa0.clone();
                    let false = (BackendVariable::isStateVar(var.clone())) else { bail!("pattern mismatch") };
                    cr = ComponentReference::createDifferentiatedCrefName(cr.clone(), inDiffwrtCref.clone(), (matrixName.clone()).clone())?;
                    res = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, _, BackendDAE::DifferentiateInputData { dependenentVars: Some(timevars), matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }) => {
                    let mut var: BackendDAE::Var;
                    let mut res: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), timevars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    var = __pa0.clone();
                    let false = (BackendVariable::isStateVar(var.clone())) else { bail!("pattern mismatch") };
                    cr = ComponentReference::createDifferentiatedCrefName(cr.clone(), inDiffwrtCref.clone(), (matrixName.clone()).clone())?;
                    res = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
                    Ok((res.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, .. }, _, _, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }) => {
                    let mut zero: Arc<DAE::Exp>;
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: tp, .. }, _, _, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
                    let mut zero: Arc<DAE::Exp>;
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut serr: ArcStr;
                    let mut se1: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    s1 = (ExpressionBasics::printExpStr(inExp.clone())?).clone();
                    se1 = (TypesDump::printTypeStr(Expression::r#typeof(inExp.clone())?)).clone();
                    s2 = (ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref.clone())?).clone();
                    serr = stringAppendList(list![(literal!("\n- differentiateCrefs ")).clone(), (s1.clone()).clone(), (literal!(" type:")).clone(), (se1.clone()).clone(), (literal!(" w.r.t: ")).clone(), (s2.clone()).clone(), (literal!(" failed\n")).clone()]);
                    Debug::trace((serr.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }
    });
    if debug {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Differentiate-ExpCref-result: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(outDiffedExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((outDiffedExp, outFunctionTree))
}

pub(crate) fn createDiffedCrefName(mut inCref: Arc<DAE::ComponentRef>, mut inMatrixName: ArcStr) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef>;
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    subs = ComponentReference::crefLastSubs(inCref.clone())?;
    outCref = ComponentReferenceBasics::crefStripLastSubs(inCref.clone())?;
    outCref = ComponentReference::prependStringCref((arcstr::literal!(BackendDAE::functionDerivativeNamePrefix)).clone(), outCref)?;
    outCref = ComponentReference::prependStringCref((inMatrixName).clone(), outCref)?;
    outCref = ComponentReference::crefSetLastSubs(outCref, subs)?;
    outCref = ComponentReference::crefSetLastType(outCref, ComponentReference::crefLastType(inCref)?)?;
    Ok(outCref)
}

pub(crate) fn createSeedCrefName(mut inCref: Arc<DAE::ComponentRef>, mut inMatrixName: ArcStr) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef>;
    let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    let debug: bool = false;
    if debug {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("inCref: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    if debug {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("after full type  ")); __mm_s.push_str(&*TypesDump::printTypeStr(ComponentReference::crefTypeConsiderSubs(inCref.clone())?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    subs = ComponentReference::crefLastSubs(inCref.clone())?;
    outCref = ComponentReferenceBasics::crefStripLastSubs(inCref.clone())?;
    outCref = ComponentReference::crefSetLastType(outCref, DAE::T_UNKNOWN_DEFAULT().clone())?;
    outCref = ComponentReference::joinCrefs(outCref, ComponentReferenceBasics::makeCrefIdent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Seed")); __mm_s.push_str(&*inMatrixName); ArcStr::from(__mm_s) }).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
    if debug {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("after join: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefListStr(ComponentReference::expandCref(outCref.clone(), true)?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    outCref = ComponentReference::crefSetLastSubs(outCref, subs)?;
    outCref = ComponentReference::crefSetLastType(outCref, ComponentReference::crefLastType(inCref)?)?;
    if debug {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("outCref: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(outCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(outCref)
}

pub(crate) fn isSeedCref(mut cr: Arc<DAE::ComponentRef>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(cr.clone()) {
        Deref @ DAE::ComponentRef::CREF_IDENT { .. } => return StringUtil::startsWith((var_field!((*cr).ident, DAE::ComponentRef::CREF_IDENT).clone()).clone(), (literal!("Seed")).clone()),
        Deref @ DAE::ComponentRef::CREF_QUAL { .. } => { cr = var_field!((*cr).componentRef, DAE::ComponentRef::CREF_QUAL).clone(); continue '__tco; },
        _ => return false,
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn differentiateCalls(mut inExp: Arc<DAE::Exp>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree>;
    let debug: bool = false;
    if debug {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nDifferentiate Exp-Call: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" w.r.t. ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    (outDiffedExp, outFunctionTree) = (::match_deref::match_deref! { match &((inExp.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, expLst: Deref @ metamodelica::List::Cons { head: actual, tail: Deref @ metamodelica::List::Cons { head: simplified, tail: Deref @ metamodelica::List::Nil } }, .. }, _, _, _) => {
            let mut e1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (e1, funcs) = differentiateExp(actual.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter)?;
            (_, funcs) = differentiateExp(simplified.clone(), inDiffwrtCref, inInputData, inDiffType, funcs, maxIter)?;
            (e1.clone(), funcs)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, tail: Deref @ metamodelica::List::Nil }, .. }, _, BackendDAE::DifferentiateInputData { independenentVars: Some(timevars), matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::GENERIC_GRADIENT { .. }) => {
            let mut res: Arc<DAE::Exp>;
            let mut cr = (*cr).clone();
            cr = ComponentReferenceBasics::makeCrefQual((arcstr::literal!(DAE::previousNamePrefix)).clone(), tp.clone(), metamodelica::nil(), cr.clone());
            ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), timevars.clone())?) {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => (),
                _ => bail!("pattern mismatch"),
            } };
            cr = createSeedCrefName(cr.clone(), (matrixName.clone()).clone())?;
            res = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
            (res, inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, attr }, _, _, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
            (Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![e.clone(), Arc::new(DAE::Exp::ICONST { integer: 2 })], attr: attr.clone() }), inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::List::Nil } }, attr }, _, _, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
            let mut i = (*i).clone();
            i = i.clone() + 1;
            (Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![e.clone(), Arc::new(DAE::Exp::ICONST { integer: i.clone() })], attr: attr.clone() }), inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, BackendDAE::DifferentiationType::GENERIC_GRADIENT { daeMode: true }) => {
            let mut cr: Arc<DAE::ComponentRef>;
            let mut cj: Arc<DAE::ComponentRef>;
            let mut res: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type>;
            cj = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (arcstr::literal!(DAE_CJ)).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            cr = Expression::expCref(e.clone())?;
            tp = Expression::r#typeof(e.clone())?;
            cr = createSeedCrefName(cr, (matrixName.clone()).clone())?;
            res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
            res = Arc::new(DAE::Exp::BINARY { exp1: Expression::makeCrefExp(cj, DAE::T_REAL_DEFAULT().clone())?, operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: res });
            (res, inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }, _, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }, _) => {
            let mut cr: Arc<DAE::ComponentRef>;
            let mut res: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type>;
            cr = Expression::expCref(e.clone())?;
            tp = Expression::r#typeof(e.clone())?;
            cr = ComponentReference::crefPrefixDer(cr);
            cr = ComponentReference::createDifferentiatedCrefName(cr, inDiffwrtCref.clone(), (matrixName.clone()).clone())?;
            res = Expression::makeCrefExp(cr.clone(), tp.clone())?;
            if ComponentReferenceBasics::crefEqual(Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("$")).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), inDiffwrtCref)? {
                (res, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            }
            (res, inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "$", .. }, _, _) => {
            let mut zero: Arc<DAE::Exp>;
            (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(Expression::r#typeof(e.clone())?))?;
            (zero, inFunctionTree.clone())
        },
        (e @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, builtin: false, .. }, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "$", .. }, _, _) if (!(Expression::isRecordCall(e.clone(), inFunctionTree.clone())?)) => {
            let mut zero: Arc<DAE::Exp>;
            (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (zero, inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil } }, _, _, _) => {
            let mut res: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (res, funcs) = differentiateCallExp1Arg((name.clone()).clone(), e.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree.clone(), maxIter)?;
            (res, funcs)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e1 @ Deref @ DAE::Exp::RCONST { real: __rlit_0 }, tail: Deref @ metamodelica::List::Nil } } }, _, _, _) if __rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            (e1.clone(), inFunctionTree.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, attr: attr @ Deref @ DAE::CallAttributes { builtin: true, .. }, expLst: expl @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } }, _, _, _) => {
            let mut res: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (res, funcs) = differentiateCallExpNArg((name.clone()).clone(), expl.clone(), attr.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree.clone(), maxIter)?;
            (res, funcs)
        },
        (e @ Deref @ DAE::Exp::CALL { .. }, _, _, _) => {
            let mut e1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (e1, funcs) = differentiateFunctionCall(e.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree.clone(), maxIter)?;
            (e1, _, _, _) = Inline::inlineExp(e1, (Some(funcs.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE]), DAE::emptyElementSource().clone());
            (e1.clone(), funcs)
        },
        _ => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut serr: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            s1 = (ExpressionBasics::printExpStr(inExp)?).clone();
            s2 = (ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref)?).clone();
            serr = stringAppendList(list![(literal!("\n- Function differentiateCalls failed. differentiateExp ")).clone(), (s1).clone(), (literal!(" w.r.t: ")).clone(), (s2).clone(), (literal!(" failed\n")).clone()]);
            Debug::trace((serr).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if debug {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Differentiate-ExpCall-result: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(outDiffedExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((outDiffedExp, outFunctionTree))
}

fn differentiateCallExp1Arg(mut name: ArcStr, mut exp: Arc<DAE::Exp>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFuncs: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree>;
    (outDiffedExp, outFunctionTree) = (::match_deref::match_deref! { match &((name, exp.clone())) {
        (Deref @ "pre", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp)?;
            (exp_1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (exp_1, inFuncs)
        },
        (Deref @ "previous", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp)?;
            (exp_1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (exp_1, inFuncs)
        },
        (Deref @ "$getPart", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (exp_1, funcs) = differentiateExp(exp, inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            (exp_1, funcs)
        },
        (Deref @ "firstTick", _) => {
            (exp, inFuncs)
        },
        (Deref @ "interval", _) => {
            (exp, inFuncs)
        },
        (Deref @ "sin", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall((literal!("cos")).clone(), list![exp], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_2, operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_1 }), funcs)
        },
        (Deref @ "cos", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall((literal!("sin")).clone(), list![exp], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp.clone() }, exp: exp_2 }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_1 }), funcs)
        },
        (Deref @ "tan", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall((literal!("cos")).clone(), list![Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp })], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_1 }), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: exp_2, operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }) }) }), funcs)
        },
        (Deref @ "asin", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: exp.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp }) })], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: exp_2 }), funcs)
        },
        (Deref @ "acos", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: exp.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp }) })], tp.clone());
            (Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp.clone() }, exp: Arc::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: exp_2 }) }), funcs)
        },
        (Deref @ "atan", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: exp.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp }) }) }), funcs)
        },
        (Deref @ "sinh", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall((literal!("cosh")).clone(), list![exp], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_2 }), funcs)
        },
        (Deref @ "cosh", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall((literal!("sinh")).clone(), list![exp], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_2 }), funcs)
        },
        (Deref @ "tanh", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall((literal!("cosh")).clone(), list![exp], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: exp_2, operator: DAE::Operator::POW { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }) }) }), funcs)
        },
        (Deref @ "exp", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall((literal!("exp")).clone(), list![exp], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_2, operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_1 }), funcs)
        },
        (Deref @ "log", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: exp }), funcs)
        },
        (Deref @ "log10", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall((literal!("log")).clone(), list![Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(10.0_f64) })], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: exp, operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_2 }) }), funcs)
        },
        (Deref @ "sqrt", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![exp], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_1, operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_2 }) }), funcs)
        },
        (Deref @ "abs", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp.clone(), inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall((literal!("sign")).clone(), list![exp], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: exp_2, operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: exp_1 }), funcs)
        },
        (Deref @ "sign", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp)?;
            (exp_1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (exp_1, inFuncs)
        },
        (Deref @ "transpose", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp, inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall((literal!("transpose")).clone(), list![exp_1], tp.clone());
            (exp_2, funcs)
        },
        (Deref @ "sum", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp.clone())?;
            (exp_1, funcs) = differentiateExp(exp, inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            exp_2 = Expression::makePureBuiltinCall((literal!("sum")).clone(), list![exp_1], tp.clone());
            (exp_2, funcs)
        },
        (Deref @ "max", Deref @ DAE::Exp::ARRAY { array: expl, ty: tp, .. }) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp = (*tp).clone();
            tp = Types::arrayElementType(tp.clone());
            exp_1 = createFromNCall2ArgsCall((literal!("max")).clone(), expl.clone(), tp.clone())?;
            (exp_2, funcs) = differentiateExp(exp_1, inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            (exp_2, funcs)
        },
        (Deref @ "min", Deref @ DAE::Exp::ARRAY { array: expl, ty: tp, .. }) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut exp_2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp = (*tp).clone();
            tp = Types::arrayElementType(tp.clone());
            exp_1 = createFromNCall2ArgsCall((literal!("min")).clone(), expl.clone(), tp.clone())?;
            (exp_2, funcs) = differentiateExp(exp_1, inDiffwrtCref, inInputData, inDiffType, inFuncs, maxIter)?;
            (exp_2, funcs)
        },
        (Deref @ "floor", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp)?;
            (exp_1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (exp_1, inFuncs)
        },
        (Deref @ "ceil", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp)?;
            (exp_1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (exp_1, inFuncs)
        },
        (Deref @ "integer", _) => {
            let mut exp_1: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type>;
            tp = Expression::r#typeof(exp)?;
            (exp_1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (exp_1, inFuncs)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outDiffedExp, outFunctionTree))
}

fn createFromNCall2ArgsCall(mut funcName: ArcStr, mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut tp: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut result: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut rest: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(expl) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    e2 = __pa1.clone();
    rest = __pa2.clone();
    result = Expression::makePureBuiltinCall((funcName.clone()).clone(), list![e1, e2], tp.clone());
    for mut elem in &*rest {
        let mut elem = elem.clone();
        result = Expression::makePureBuiltinCall((funcName.clone()).clone(), list![result.clone(), elem.clone()], tp.clone());
    }
    Ok(result)
}

fn differentiateCallExpNArg(mut name: ArcStr, mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inAttr: Arc<DAE::CallAttributes>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree>;
    (outDiffedExp, outFunctionTree) = (::match_deref::match_deref! { match &((name.clone(), inExpl, inAttr.clone())) {
        (Deref @ "smooth", Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut e1: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (res1, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter)?;
            e1 = Arc::new(DAE::Exp::ICONST { integer: i.clone() - 1 });
            res2 = if (intGe(i.clone(), 1)) {Expression::makePureBuiltinCall((literal!("smooth")).clone(), list![e1.clone(), res1], tp.clone())} else {res1};
            (res2, funcs)
        },
        (Deref @ "noEvent", Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter)?;
            res1 = Expression::makePureBuiltinCall((literal!("noEvent")).clone(), list![res1], tp.clone());
            (res1, funcs)
        },
        (Deref @ "atan2", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut e2: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            e2 = Expression::makeDiv(e.clone(), e1.clone())?;
            (res1, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter)?;
            res2 = Expression::addNoEventToRelations(Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::EQUAL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None }), expThen: e1.clone(), expElse: Arc::new(DAE::Exp::BINARY { exp1: res1, operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() }) }) }) }))?;
            (res2, funcs)
        },
        (Deref @ "semiLinear", Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (res, funcs) = differentiateExp(e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), funcs, maxIter)?;
            (res2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, funcs, maxIter)?;
            res1 = Expression::expAdd(Expression::expMul(res1, e.clone())?, Expression::expMul(e1.clone(), res.clone())?)?;
            res2 = Expression::expAdd(Expression::expMul(res2, e.clone())?, Expression::expMul(e2.clone(), res)?)?;
            (res, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            res = Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::GREATEREQ { ty: tp.clone() }, exp2: res, index: -1, optionExpisASUB: None });
            (Arc::new(DAE::Exp::IFEXP { expCond: res, expThen: res1, expElse: res2 }), funcs)
        },
        (Deref @ "cross", Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (res2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, funcs, maxIter)?;
            res2 = Expression::makePureBuiltinCall((literal!("cross")).clone(), list![e1.clone(), res2], tp.clone());
            res1 = Expression::makePureBuiltinCall((literal!("cross")).clone(), list![res1, e2.clone()], tp.clone());
            (Arc::new(DAE::Exp::BINARY { exp1: res2, operator: DAE::Operator::ADD_ARR { ty: tp.clone() }, exp2: res1 }), funcs)
        },
        (Deref @ "max", Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (res2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("noEvent")).clone() }), expLst: list![Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::GREATER { ty: tp.clone() }, exp2: e2.clone(), index: -1, optionExpisASUB: None })], attr: DAE::callAttrBuiltinBool().clone() }), expThen: res1, expElse: res2 }), funcs)
        },
        (Deref @ "min", Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (res2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("noEvent")).clone() }), expLst: list![Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::LESS { ty: tp.clone() }, exp2: e2.clone(), index: -1, optionExpisASUB: None })], attr: DAE::callAttrBuiltinBool().clone() }), expThen: res1, expElse: res2 }), funcs)
        },
        (Deref @ "div", Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            (res1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (res1, inFunctionTree)
        },
        (Deref @ "mod", Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut e: Arc<DAE::Exp>;
            let mut etmp: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            etmp = Expression::makePureBuiltinCall((literal!("floor")).clone(), list![Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: e2.clone() })], tp.clone());
            e = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: etmp }) });
            (res1, funcs) = differentiateExp(e.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter)?;
            (res1, funcs)
        },
        (Deref @ "rem", Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (res1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData, inDiffType, inFunctionTree, maxIter)?;
            (res1, funcs)
        },
        (Deref @ "delay", Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: e3, tail: Deref @ metamodelica::List::Cons { head: e4, tail: Deref @ metamodelica::List::Nil } } } }, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut e: Arc<DAE::Exp>;
            let mut res: Arc<DAE::Exp>;
            let mut res1: Arc<DAE::Exp>;
            let mut res2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            res1 = (match inDiffType.clone() {
        BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. } => Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }),
        _ => Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }),
    });
            (res2, funcs) = differentiateExp(e3.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType, inFunctionTree, maxIter)?;
            res2 = Arc::new(DAE::Exp::BINARY { exp1: res1, operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: res2 });
            (res2, _) = ExpressionSimplify::simplify(res2)?;
            if Expression::isZero(res2.clone())? {
                (res, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            } else {
                (e, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData, openmodelica_backend_types::BackendDAE::DifferentiationType::DIFFERENTIATION_TIME, funcs, maxIter)?;
                e = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (name).clone() }), expLst: list![Arc::new(DAE::Exp::ICONST { integer: -1 }), e, e3.clone(), e4.clone()], attr: inAttr });
                res = Arc::new(DAE::Exp::BINARY { exp1: res2, operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e.clone() });
                (res, _) = ExpressionSimplify::simplify(res)?;
            }
            (res, funcs)
        },
        (Deref @ "sample", _, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            (res1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (res1, inFunctionTree)
        },
        (Deref @ "floor", _, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            (res1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (res1, inFunctionTree)
        },
        (Deref @ "ceil", _, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            (res1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (res1, inFunctionTree)
        },
        (Deref @ "integer", _, Deref @ DAE::CallAttributes { ty: tp, .. }) => {
            let mut res1: Arc<DAE::Exp>;
            (res1, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (res1, inFunctionTree)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outDiffedExp, outFunctionTree))
}

fn differentiateBinary(mut inExp: Arc<DAE::Exp>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree>;
    (outDiffedExp, outFunctionTree) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { ty: tp }, exp2: e2 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: de2 }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD_ARR { ty: tp }, exp2: e2 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::ADD_ARR { ty: tp.clone() }, exp2: de2 }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD_ARRAY_SCALAR { ty: tp }, exp2: e2 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::ADD_ARRAY_SCALAR { ty: tp.clone() }, exp2: de2 }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { ty: tp }, exp2: e2 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: de2 }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB_ARR { ty: tp }, exp2: e2 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::SUB_ARR { ty: tp.clone() }, exp2: de2 }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB_SCALAR_ARRAY { ty: tp }, exp2: e2 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::SUB_SCALAR_ARRAY { ty: tp.clone() }, exp2: de2 }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: tp }, exp2: e2 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de2 }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_ARR { ty: tp }, exp2: e2 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: de2 }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp }, exp2: e2 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: de2 }), operator: DAE::Operator::ADD_ARR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_SCALAR_PRODUCT { ty: tp }, exp2: e2 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_SCALAR_PRODUCT { ty: tp.clone() }, exp2: de2 }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL_SCALAR_PRODUCT { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_MATRIX_PRODUCT { ty: tp }, exp2: e2 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_MATRIX_PRODUCT { ty: tp.clone() }, exp2: de2 }), operator: DAE::Operator::ADD_ARR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL_MATRIX_PRODUCT { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: tp }, exp2: e2 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() }), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de2 }) }), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_ARR { ty: tp }, exp2: e2 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: e2.clone() }), operator: DAE::Operator::SUB_ARR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: de2 }) }), operator: DAE::Operator::DIV_ARR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_ARRAY_SCALAR { ty: tp }, exp2: e2 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut tp1: Arc<DAE::Type>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, funcs, maxIter)?;
            tp1 = Expression::r#typeof(e2.clone())?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: e2.clone() }), operator: DAE::Operator::SUB_ARR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: de2 }) }), operator: DAE::Operator::DIV_ARRAY_SCALAR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp1 }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_SCALAR_ARRAY { ty: tp }, exp2: e2 } => {
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree, maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, funcs, maxIter)?;
            (Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: de1, operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: e2.clone() }), operator: DAE::Operator::SUB_ARR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }, exp2: de2 }) }), operator: DAE::Operator::DIV_ARR { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL_ARR { ty: tp.clone() }, exp2: e2.clone() }) }), funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: tp }, exp2: e2 @ Deref @ DAE::Exp::RCONST { real: r } } => {
            let mut e: Arc<DAE::Exp>;
            let mut de1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut r = (*r).clone();
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, inFunctionTree, maxIter)?;
            r = r.clone() - metamodelica::OrderedFloat(1.0_f64);
            e = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: r.clone() }) }) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de1 });
            (e, funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: tp }, exp2: e2 @ Deref @ DAE::Exp::ICONST { integer: i } } => {
            let mut e: Arc<DAE::Exp>;
            let mut de1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut i = (*i).clone();
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, inFunctionTree, maxIter)?;
            i = i.clone() - 1;
            e = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) }) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de1 });
            (e, funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: __rlit_1 }, operator: DAE::Operator::POW { ty: tp }, .. } if __rlit_1.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            let mut zero: Arc<DAE::Exp>;
            (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
            (zero, inFunctionTree)
        },
        e0 @ Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: r }, operator: DAE::Operator::POW { ty: tp }, exp2: e1 } => {
            let mut e: Arc<DAE::Exp>;
            let mut de1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut r = (*r).clone();
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, inFunctionTree, maxIter)?;
            r = (r.clone()).ln();
            e = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e0.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: r.clone() }) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de1 });
            (e, funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: tp }, exp2: e2 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. } } if (isParamOrConstant(cr.clone(), inInputData.clone())?) => {
            let mut e: Arc<DAE::Exp>;
            let mut etmp: Arc<DAE::Exp>;
            let mut de1: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            etmp = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: 1 }) }),
        _ => Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, inFunctionTree, maxIter)?;
            e = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: etmp }) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de1 });
            (e, funcs)
        },
        e0 @ Deref @ DAE::Exp::BINARY { exp1: e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, operator: DAE::Operator::POW { ty: tp }, exp2: e2 } if (isParamOrConstant(cr.clone(), inInputData.clone())?) => {
            let mut e: Arc<DAE::Exp>;
            let mut etmp: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, inFunctionTree, maxIter)?;
            etmp = Expression::makePureBuiltinCall((literal!("log")).clone(), list![e1.clone()], tp.clone());
            e = Expression::addNoEventToRelations(Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::EQUAL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None }), expThen: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), expElse: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e0.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: etmp }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de2 }) }))?;
            (e, funcs)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: tp }, exp2: e2 } => {
            let mut e: Arc<DAE::Exp>;
            let mut etmp: Arc<DAE::Exp>;
            let mut de1: Arc<DAE::Exp>;
            let mut de2: Arc<DAE::Exp>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            (de1, funcs) = differentiateExp(e1.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter)?;
            (de2, funcs) = differentiateExp(e2.clone(), inDiffwrtCref, inInputData.clone(), inDiffType, inFunctionTree, maxIter)?;
            etmp = Expression::makePureBuiltinCall((literal!("log")).clone(), list![e1.clone()], tp.clone());
            e = Expression::addNoEventToRelations(Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: DAE::Operator::EQUAL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None }), expThen: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), expElse: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }) }) }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: etmp }), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de2 }), operator: DAE::Operator::ADD { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: de1 }) }) }) }))?;
            (e, funcs)
        },
        _ => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut serr: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            s1 = (ExpressionBasics::printExpStr(inExp)?).clone();
            s2 = (ComponentReferenceBasics::printComponentRefStr(inDiffwrtCref)?).clone();
            serr = stringAppendList(list![(literal!("\n- Function differentiateBinary failed. differentiateExp ")).clone(), (s1).clone(), (literal!(" w.r.t: ")).clone(), (s2).clone(), (literal!(" failed\n")).clone()]);
            Debug::trace((serr).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outDiffedExp, outFunctionTree))
}

// =============================================================================
// functions to generate derivative of a function
// =============================================================================
fn differentiateFunctionCall(mut inExp: Arc<DAE::Exp>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    (outDiffedExp, outFunctionTree) = 'mc: {
        let __mc_input = (inExp.clone(), inDiffType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, BackendDAE::DifferentiationType::SIMPLE_DIFFERENTIATION { .. }) => {
                    if !((!(Expression::expHasCref(inExp.clone(), inDiffwrtCref.clone())?))) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    (e, _) = Expression::makeZeroExpression(Expression::arrayDimension(ComponentReference::crefTypeFull(inDiffwrtCref.clone())?))?;
                    Ok((e.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path, expLst: expl, attr: Deref @ DAE::CallAttributes { tuple_: b, builtin: c, isImpure, ty, tailCall: tc, .. } }, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
                    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut dexpl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut dpath: Arc<Absyn::Path>;
                    let mut dinl: DAE::InlineType;
                    let mut mapper: DAE::FunctionDefinition;
                    let mut tp: Arc<DAE::Type>;
                    let mut dtp: Arc<DAE::Type>;
                    let mut blst: Arc<metamodelica::List<bool>>;
                    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree> = outFunctionTree.clone();
                    (mapper, tp) = getFunctionMapper(path.clone(), inFunctionTree.clone())?;
                    (dpath, blst) = differentiateFunction1(path.clone(), mapper.clone(), tp.clone(), expl.clone(), (inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone()))?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(inFunctionTree.clone(), dpath.clone())?) {
                        Some(DAE::Function::FUNCTION { type_: __pa0, inlineType: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    dtp = __pa0.clone();
                    dinl = __pa1.clone();
                    ::match_deref::match_deref! { match &(checkDerivativeFunctionInputs(blst.clone(), tp.clone(), dtp.clone())?) {
                        (true, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (expl1, _) = List::splitOnBoolList(expl.clone(), blst.clone())?;
                    (dexpl, outFunctionTree) = List::map3Fold(expl1.clone(), (std::sync::Arc::new({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, BackendDAE::DifferentiateInputData, BackendDAE::DifferentiationType, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone())?;
                    expl1 = listAppend(expl.clone(), dexpl.clone());
                    Ok(((Arc::new(DAE::Exp::CALL { path: dpath.clone(), expLst: expl1.clone(), attr: Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: b.clone(), builtin: c.clone(), isImpure: isImpure.clone(), isFunctionPointerCall: false, inlineType: dinl.clone(), tailCall: tc.clone() }) }), outFunctionTree.clone()), outFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outFunctionTree = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path, expLst: expl, .. }, BackendDAE::DifferentiationType::DIFFERENTIATION_TIME { .. }) => {
                    let mut dpath: Arc<Absyn::Path>;
                    let mut mapper: DAE::FunctionDefinition;
                    let mut tp: Arc<DAE::Type>;
                    let mut dtp: Arc<DAE::Type>;
                    let mut blst: Arc<metamodelica::List<bool>>;
                    let mut tlst: Arc<metamodelica::List<Arc<DAE::Type>>>;
                    let mut typstring: ArcStr;
                    let mut dastring: ArcStr;
                    let mut typlststring: Arc<metamodelica::List<ArcStr>>;
                    (mapper, tp) = getFunctionMapper(path.clone(), inFunctionTree.clone())?;
                    (dpath, blst) = differentiateFunction1(path.clone(), mapper.clone(), tp.clone(), expl.clone(), (inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone()))?;
                    let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(inFunctionTree.clone(), dpath.clone())?) {
                        Some(DAE::Function::FUNCTION { type_: __pa0, .. }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    dtp = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(checkDerivativeFunctionInputs(blst.clone(), tp.clone(), dtp.clone())?) {
                        (false, __pa1) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    tlst = __pa1.clone();
                    typlststring = List::map(tlst.clone(), (std::sync::Arc::new(TypesDump::unparseType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>))?;
                    typstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(typlststring.clone(), (literal!(";\n")).clone())); ArcStr::from(__mm_s) }).clone();
                    dastring = (AbsynUtil::pathString(dpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    metamodelica::print((literal!("Input warnings for function mapper2\n")).clone());
                    Error::addMessage(Error::UNEXPECTED_FUNCTION_INPUTS_WARNING.clone(), list![(dastring.clone()).clone(), (typstring.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: false, .. }, .. }, _) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    if '__try0: {
                        let BackendDAE::DIFF_FULL_JACOBIAN { .. } = (inDiffType.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    let __pa1 = ::match_deref::match_deref! { match &(Inline::forceInlineExp(inExp.clone(), (Some(inFunctionTree.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE]), DAE::emptyElementSource().clone(), (std::sync::Arc::new(Ceval::cevalSimpleWithFunctionTreeReturnExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>) -> Result<Arc<DAE::Exp>> + 'static>))?) {
                        (__pa1, _, true) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa1.clone();
                    (e, functions) = differentiateExp(e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter)?;
                    Ok((e.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path, expLst: expl, attr }, _) => {
                    if !((Expression::isRecordCall(e.clone(), inFunctionTree.clone())?)) { bail!("guard") }
                    let mut dexpl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    (dexpl, functions) = List::map3Fold(expl.clone(), (std::sync::Arc::new({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, BackendDAE::DifferentiateInputData, BackendDAE::DifferentiationType, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone())?;
                    Ok((Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: dexpl.clone(), attr: attr.clone() }), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _) => {
                    let mut de: Arc<DAE::Exp>;
                    let mut b: bool;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut e = (*e).clone();
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
                        BackendDump::debugStrExpStr((literal!("### Differentiate call\n ")).clone(), e.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" w.r.t. ")); __mm_s.push_str(&*ComponentReference::crefStr(inDiffwrtCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (de, functions) = differentiateFunctionCallPartial(e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter)?;
                    (e, _, b) = Inline::forceInlineExp(de.clone(), (Some(functions.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE]), DAE::emptyElementSource().clone(), (std::sync::Arc::new(Ceval::cevalSimpleWithFunctionTreeReturnExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    if b.clone() {
                        de = e.clone();
                    }
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
                        BackendDump::debugStrExpStr((literal!("### result output :\n")).clone(), de.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" w.r.t. ")); __mm_s.push_str(&*ComponentReference::crefStr(inDiffwrtCref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok((de.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut zero: Arc<DAE::Exp>;
                    let mut tp: Arc<DAE::Type>;
                    let false = (Expression::expContains(inExp.clone(), Expression::crefExp(inDiffwrtCref.clone())?)?) else { bail!("pattern mismatch") };
                    tp = Expression::r#typeof(inExp.clone())?;
                    zero = Expression::createZeroExpression(tp.clone())?;
                    Ok((zero.clone(), inFunctionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Differentiate.differentiateFunctionCall")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outDiffedExp, outFunctionTree))
}

fn differentiateFunctionCallPartial(mut inExp: Arc<DAE::Exp>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outDiffedExp: Arc<DAE::Exp>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree>;
    (outDiffedExp, outFunctionTree) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path, expLst: expl, attr: Deref @ DAE::CallAttributes { tuple_: b, builtin: c, isImpure, ty, tailCall: tc, .. } } => {
                    let mut diffFuncData: BackendDAE::DifferentiateInputData;
                    let mut e: Arc<DAE::Exp>;
                    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut dexpl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut dexplZero: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut dpath: Arc<Absyn::Path>;
                    let mut dinl: DAE::InlineType;
                    let mut dtp: Arc<DAE::Type>;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut mapper: DAE::FunctionDefinition;
                    let mut tp: Arc<DAE::Type>;
                    let mut dtp: Arc<DAE::Type>;
                    let mut blst: Arc<metamodelica::List<bool>>;
                    let mut funcname: ArcStr;
                    (mapper, tp) = getFunctionMapper(path.clone(), inFunctionTree.clone())?;
                    (dpath, blst) = differentiateFunction1(path.clone(), mapper.clone(), tp.clone(), expl.clone(), (inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone()))?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(inFunctionTree.clone(), dpath.clone())?) {
                        Some(DAE::Function::FUNCTION { type_: __pa0, inlineType: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    dtp = __pa0.clone();
                    dinl = __pa1.clone();
                    ::match_deref::match_deref! { match &(checkDerivativeFunctionInputs(blst.clone(), tp.clone(), dtp.clone())?) {
                        (true, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (expl1, _) = List::splitOnBoolList(expl.clone(), blst.clone())?;
                    (dexpl, functions) = List::map3Fold(expl1.clone(), (std::sync::Arc::new({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, BackendDAE::DifferentiateInputData, BackendDAE::DifferentiationType, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone())?;
                    funcname = (BackendUtil::modelicaStringToCStr((AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone(), false)?).clone();
                    diffFuncData = BackendDAE::emptyInputData().clone();
                    diffFuncData.matrixName = Some((funcname.clone()).clone());
                    (dexplZero, functions) = List::map3Fold(expl1.clone(), (std::sync::Arc::new({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, BackendDAE::DifferentiateInputData, BackendDAE::DifferentiationType, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>), Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("$")).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), diffFuncData.clone(), BackendDAE::DifferentiationType::GENERIC_GRADIENT { daeMode: false }, functions.clone())?;
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
                        metamodelica::print((literal!("### differentiated argument list:\n")).clone());
                        metamodelica::print((literal!("Diffed ExpList: \n")).clone());
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(dexpl.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    e = Arc::new(DAE::Exp::CALL { path: dpath.clone(), expLst: expl1.clone(), attr: Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: b.clone(), builtin: c.clone(), isImpure: isImpure.clone(), isFunctionPointerCall: false, inlineType: dinl.clone(), tailCall: tc.clone() }) });
                    e = createPartialArguments(ty.clone(), dexpl.clone(), dexplZero.clone(), expl.clone(), e.clone())?;
                    Ok((e.clone(), functions.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path, expLst: expl, .. } => {
                    let mut dpath: Arc<Absyn::Path>;
                    let mut dtp: Arc<DAE::Type>;
                    let mut mapper: DAE::FunctionDefinition;
                    let mut tp: Arc<DAE::Type>;
                    let mut dtp: Arc<DAE::Type>;
                    let mut blst: Arc<metamodelica::List<bool>>;
                    let mut tlst: Arc<metamodelica::List<Arc<DAE::Type>>>;
                    let mut typstring: ArcStr;
                    let mut dastring: ArcStr;
                    let mut typlststring: Arc<metamodelica::List<ArcStr>>;
                    (mapper, tp) = getFunctionMapper(path.clone(), inFunctionTree.clone())?;
                    (dpath, blst) = differentiateFunction1(path.clone(), mapper.clone(), tp.clone(), expl.clone(), (inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone()))?;
                    let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(inFunctionTree.clone(), dpath.clone())?) {
                        Some(DAE::Function::FUNCTION { type_: __pa0, .. }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    dtp = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(checkDerivativeFunctionInputs(blst.clone(), tp.clone(), dtp.clone())?) {
                        (false, __pa1) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    tlst = __pa1.clone();
                    typlststring = List::map(tlst.clone(), (std::sync::Arc::new(TypesDump::unparseType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>))?;
                    typstring = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(typlststring.clone(), (literal!(";\n")).clone())); ArcStr::from(__mm_s) }).clone();
                    dastring = (AbsynUtil::pathString(dpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    metamodelica::print((literal!("Input warnings for function mapper2\n")).clone());
                    Error::addMessage(Error::UNEXPECTED_FUNCTION_INPUTS_WARNING.clone(), list![(dastring.clone()).clone(), (typstring.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e @ Deref @ DAE::Exp::CALL { path, expLst: expl, attr: Deref @ DAE::CallAttributes { tuple_: b, builtin: false, isImpure, ty, tailCall: tc, .. } } => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut dexpl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut dexplZero: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut dpath: Arc<Absyn::Path>;
                    let mut dtp: Arc<DAE::Type>;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut dtp: Arc<DAE::Type>;
                    let mut blst: Arc<metamodelica::List<bool>>;
                    let mut expBoolLst: Arc<metamodelica::List<(Arc<DAE::Exp>, bool)>>;
                    let mut funstring: ArcStr;
                    let mut inputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut outputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut func: DAE::Function;
                    let mut dfunc: DAE::Function;
                    let mut success: bool;
                    let mut e = (*e).clone();
                    let mut inInputData: BackendDAE::DifferentiateInputData = inInputData.clone();
                    if '__try0: {
                        let BackendDAE::SIMPLE_DIFFERENTIATION { .. } = (inDiffType.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    if '__try1: {
                        let BackendDAE::DIFF_FULL_JACOBIAN { .. } = (inDiffType.clone()) else { break '__try1 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    let __pa2 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(inFunctionTree.clone(), path.clone())?) {
                        Some(__pa2) => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    func = __pa2.clone();
                    if !(AvlSetPath::hasKey(inInputData.diffedFunctions.clone(), path.clone())?) {
                        inInputData.diffedFunctions = AvlSetPath::add(inInputData.diffedFunctions.clone(), path.clone())?;
                        (dfunc, functions, blst) = differentiatePartialFunction(func.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter)?;
                        dpath = DAEUtil::functionName(dfunc.clone())?;
                        let __pa3 = ::match_deref::match_deref! { match &(DAEUtil::getFunctionType(dfunc.clone())?) {
                            Deref @ DAE::Type::T_FUNCTION { funcResultType: __pa3, .. } => __pa3.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        dtp = __pa3.clone();
                        if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                            funstring = (Tpl::tplString((std::sync::Arc::new(DAEDumpTpl::dumpFunction) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, DAE::Function) -> Result<Tpl::Text> + 'static>), dfunc.clone())?).clone();
                            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Differentiate function: \n")); __mm_s.push_str(&*funstring.clone()); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                        }
                        functions = AvlTreePathFunction::addDaeFunction(list![dfunc.clone()], functions.clone())?;
                        func = DAEUtil::addFunctionDefinition(func.clone(), DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivedFunction: path.clone(), derivativeFunction: dpath.clone(), derivativeOrder: 1, conditionRefs: metamodelica::nil(), defaultDerivative: None, lowerOrderDerivatives: metamodelica::nil() });
                        functions = AvlTreePathFunction::add(functions.clone(), path.clone(), Some(func.clone()), (std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
                    } else {
                        (functions, inputVarsDer, _, outputVarsDer, _, blst) = getFunctionInOutVars(func.clone(), inFunctionTree.clone(), inDiffwrtCref.clone(), maxIter)?;
                        (dpath, dtp) = getDiffedTypeandName(func.clone(), inputVarsDer.clone(), outputVarsDer.clone(), blst.clone())?;
                        let __pa4 = ::match_deref::match_deref! { match &(dtp.clone()) {
                            Deref @ DAE::Type::T_FUNCTION { funcResultType: __pa4, .. } => __pa4.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        dtp = __pa4.clone();
                    }
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                        metamodelica::print((literal!("### Detailed arguments list: \n")).clone());
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(expl.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        metamodelica::print((literal!("### and argument types: \n")).clone());
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::mapMap(expl.clone(), (std::sync::Arc::new(Expression::r#typeof) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Type>> + 'static>), (std::sync::Arc::new(fnptr!(TypesDump::printTypeStr, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<ArcStr> + 'static>))?, (literal!(" | ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### and output type: \n")); __mm_s.push_str(&*TypesDump::printTypeStr(dtp.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    expBoolLst = List::zip(expl.clone(), blst.clone());
                    expBoolLst = List::filterOnTrue(expBoolLst.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)))?;
                    expl1 = List::map(expBoolLst.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)))?;
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                        metamodelica::print((literal!("### Selected Arguments: \n")).clone());
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(expl1.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    (dexpl, functions) = List::map3Fold(expl1.clone(), (std::sync::Arc::new({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, BackendDAE::DifferentiateInputData, BackendDAE::DifferentiationType, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functions.clone())?;
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                        metamodelica::print((literal!("### Diffed ExpList: \n")).clone());
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(dexpl.clone(), (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    (dexplZero, functions, success) = tryZeroDiff(expl1.clone(), functions.clone(), maxIter);
                    if success.clone() {
                        e = Arc::new(DAE::Exp::CALL { path: dpath.clone(), expLst: dexpl.clone(), attr: Arc::new(DAE::CallAttributes { ty: dtp.clone(), tuple_: b.clone(), builtin: false, isImpure: isImpure.clone(), isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: tc.clone() }) });
                        exp = createPartialArguments(ty.clone(), dexpl.clone(), dexplZero.clone(), expl.clone(), e.clone())?;
                    } else {
                        exp = Arc::new(DAE::Exp::CALL { path: dpath.clone(), expLst: listAppend(expl.clone(), dexpl.clone()), attr: Arc::new(DAE::CallAttributes { ty: dtp.clone(), tuple_: b.clone(), builtin: false, isImpure: isImpure.clone(), isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: tc.clone() }) });
                    }
                    if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                        metamodelica::print((literal!("### differentiated result CALL :\n")).clone());
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok(((exp.clone(), functions.clone()), inInputData.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { inInputData = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Differentiate.differentiateFunctionCallPartial failed for ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    Debug::trace((r#str.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outDiffedExp, outFunctionTree))
}

fn addFunctionConstantsAndParameters(mut knownVars_opt: Option<BackendDAE::Variables>, mut func: DAE::Function) -> Result<Option<BackendDAE::Variables>> {
    let mut knownVars_opt: Option<BackendDAE::Variables> = knownVars_opt;
    knownVars_opt = ({
        let mut body_knowns: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(func) {
        DAE::Function::FUNCTION { functions: Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body }, tail: _ }, .. } => {
            let mut var_opt: Option<BackendDAE::Var>;
            for mut element in &*body.clone() {
                let mut element = element.clone();
                var_opt = BackendDAECreate::lowerKnownVarSingle(element.clone())?;
                if isSome(var_opt.clone()) {
                    body_knowns = metamodelica::cons(Util::getOption(var_opt.clone())?, body_knowns.clone());
                }
            }
            if body_knowns.clone().is_empty() {
                knownVars_opt = knownVars_opt;
            } else if isSome(knownVars_opt.clone()) {
                knownVars_opt = Some(BackendVariable::addVars(body_knowns, Util::getOption(knownVars_opt)?)?);
            } else {
                knownVars_opt = Some(BackendVariable::listVar(body_knowns)?);
            }
            knownVars_opt
        },
        _ => {
            knownVars_opt
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok(knownVars_opt)
}

fn tryZeroDiff(mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut functions: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> (Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<AvlTreePathFunction::Tree>, bool) {
    let mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>> = explist;
    let mut functions: Arc<AvlTreePathFunction::Tree> = functions;
    let mut success: bool;
    match '__try0: {
        (explist, functions) = unwrap_break_err!(List::map3Fold(explist.clone(), (std::sync::Arc::new({ let __pe_b5 = maxIter; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4| differentiateExp(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>, BackendDAE::DifferentiateInputData, BackendDAE::DifferentiationType, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>), Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("$")).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), BackendDAE::emptyInputData().clone(), BackendDAE::DifferentiationType::GENERIC_GRADIENT { daeMode: false }, functions.clone()), '__try0);
        success = true;
        Ok::<_, anyhow::Error>((explist.clone(), success.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            explist = __try0_o0;
            success = __try0_o1;
        }
        Err(_) => {
            explist = metamodelica::nil();
            success = false;
        }
    }
    (explist, functions, success)
}

fn createPartialArguments(mut outputType: Arc<DAE::Type>, mut inArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffedArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inOrginalExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCall: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (outputType.clone(), inCall.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: rPath }, varLst, .. }, Deref @ DAE::Exp::CALL { path, .. }) => {
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>>;
                    let mut varNames: Arc<metamodelica::List<ArcStr>>;
                    tys = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut v in (varLst.clone()).into_iter().cloned() {
                    let __x = DAEUtil::varType(v.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    varNames = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (varLst.clone()).into_iter().cloned() {
                    let __x = DAEUtil::typeVarIdent(v.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    expLst = createPartialArgumentsRecord(tys.clone(), varNames.clone(), inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), inCall.clone())?;
                    Ok(Arc::new(DAE::Exp::RECORD { path: rPath.clone(), exps: expLst.clone(), comp: varNames.clone(), ty: outputType.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, Deref @ DAE::Exp::TSUB { exp: Deref @ DAE::Exp::CALL { path, attr, .. }, .. }) => {
                    Ok(Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: listAppend(inOrginalExpl.clone(), inArgs.clone()), attr: attr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_TUPLE { types: tys, .. }, _) => {
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    expLst = createPartialArgumentsTuple(tys.clone(), inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), inCall.clone())?;
                    Ok(Arc::new(DAE::Exp::TUPLE { PR: expLst.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut ezero: Arc<DAE::Exp>;
                    let mut e: Arc<DAE::Exp>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    dims = Expression::arrayDimension(outputType.clone());
                    (ezero, _) = Expression::makeZeroExpression(dims.clone())?;
                    e = createPartialDifferentiatedExp(inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), inCall.clone(), 1, ezero.clone())?;
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CALL { path, attr, .. }) => {
                    Ok(Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: listAppend(inOrginalExpl.clone(), inArgs.clone()), attr: attr.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn createPartialArgumentsTuple(mut inTypesLst: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffedArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inOrginalExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCall: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    outExpLst = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        let __thr_src0 = inTypesLst.clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let mut __thr_it1 = (1..=(inTypesLst.len() as i32)).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(tp), Some(number)) => {
                    let __x = createPartialArguments(tp.clone(), inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), Arc::new(DAE::Exp::TSUB { exp: inCall.clone(), ix: number.clone(), ty: tp.clone() }))?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => bail!("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(outExpLst)
}

fn createPartialArgumentsRecord(mut inTypesLst: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inVarNames: Arc<metamodelica::List<ArcStr>>, mut inArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffedArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inOrginalExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCall: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    outExpLst = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        let __thr_src0 = inTypesLst;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = inVarNames;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(tp), Some(name)) => {
                    let __x = createPartialArguments(tp.clone(), inArgs.clone(), inDiffedArgs.clone(), inOrginalExpl.clone(), Arc::new(DAE::Exp::RSUB { exp: inCall.clone(), ix: -1, fieldName: (name.clone()).clone(), ty: tp.clone() }))?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => bail!("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(outExpLst)
}

fn createPartialDifferentiatedExp(mut inDiffExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffExplZero: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inOrginalExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCall: Arc<DAE::Exp>, mut currentLstElement: i32, mut inAccum: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp> = inAccum.clone();
    let mut i: i32 = currentLstElement;
    for mut de in &*inDiffExpl {
        let mut de = de.clone();
        outExp = (::match_deref::match_deref! { match &((de.clone(), inCall.clone())) {
        (_, Deref @ DAE::Exp::CALL { path, attr, .. }) if (Types::isRecord(Expression::r#typeof(de.clone())?)) => {
            let mut e: Arc<DAE::Exp>;
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut dexpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            dexpLst = List::set(inDiffExplZero.clone(), i, de.clone())?;
            expLst = listAppend(inOrginalExpl.clone(), dexpLst.clone());
            e = Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: attr.clone() });
            e.clone()
        },
        (Deref @ DAE::Exp::ARRAY { ty: tp, scalar: b, array: expl }, _) => {
            let mut e: Arc<DAE::Exp>;
            let mut eArray: Arc<DAE::Exp>;
            let mut arrayArgs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut dexpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            eArray = (inDiffExplZero.clone()).get(i)?;
            dexpLst = Expression::arrayElements(eArray.clone())?;
            arrayArgs = prepareArgumentsExplArray(expl.clone(), dexpLst.clone(), 1, metamodelica::nil())?;
            expLst = List::map2(arrayArgs.clone(), (std::sync::Arc::new(fnptr!(Expression::makeArray, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool) -> Result<Arc<DAE::Exp>> + 'static>), tp.clone(), b.clone())?;
            arrayArgs = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
        for mut exp in (expLst.clone()).into_iter().cloned() {
            let __x = List::set(inDiffExplZero.clone(), i, exp.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            arrayArgs = List::map1r(arrayArgs.clone(), Arc::new(fnptr!(listAppend, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)), inOrginalExpl.clone())?;
            e = createPartialSum(arrayArgs.clone(), expl.clone(), inCall.clone(), outExp.clone())?;
            e.clone()
        },
        _ => {
            let mut e: Arc<DAE::Exp>;
            let mut eone: Arc<DAE::Exp>;
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut dexpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut tp: Arc<DAE::Type>;
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
            tp = Expression::r#typeof(de.clone())?;
            dims = Expression::arrayDimension(tp.clone());
            (eone, _) = Expression::makeOneExpression(dims.clone())?;
            dexpLst = List::set(inDiffExplZero.clone(), i, eone.clone())?;
            expLst = listAppend(inOrginalExpl.clone(), dexpLst.clone());
            e = createPartialSum(list![expLst.clone()], list![de.clone()], inCall.clone(), outExp.clone())?;
            e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        i = i + 1;
    }
    Ok(outExp)
}

fn createPartialSum(mut inArgsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inDiff: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCall: Arc<DAE::Exp>, mut inAccum: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp> = inAccum.clone();
    let mut restDiff: Arc<metamodelica::List<Arc<DAE::Exp>>> = inDiff.clone();
    let mut de: Arc<DAE::Exp>;
    let mut res: Arc<DAE::Exp>;
    for mut expLst in &*inArgsLst {
        let mut expLst = expLst.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(restDiff.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        de = __pa0.clone();
        restDiff = __pa1.clone();
        if !(Expression::isZero(de.clone())?) {
            res = (::match_deref::match_deref! { match &(inCall.clone()) {
        Deref @ DAE::Exp::RSUB { exp: Deref @ DAE::Exp::CALL { path, attr, .. }, ix, fieldName: name, ty } => {
            Arc::new(DAE::Exp::RSUB { exp: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: attr.clone() }), ix: ix.clone(), fieldName: (name.clone()).clone(), ty: ty.clone() })
        },
        Deref @ DAE::Exp::TSUB { exp: Deref @ DAE::Exp::CALL { path, attr, .. }, ix, ty } => {
            Arc::new(DAE::Exp::TSUB { exp: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: attr.clone() }), ix: ix.clone(), ty: ty.clone() })
        },
        Deref @ DAE::Exp::CALL { path, attr, .. } => {
            Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: attr.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
            res = Expression::expMul(de.clone(), res.clone())?;
            outExp = Expression::expAdd(outExp.clone(), res.clone())?;
        }
    }
    Ok(outExp)
}

fn prepareArgumentsExplArray(mut inWorkLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inArgs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inCurrentArg: i32, mut inAccum: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inWorkLst) {
        Deref @ metamodelica::List::Nil => {
            return Ok(inAccum.reverse())
        },
        Deref @ metamodelica::List::Cons { head: e, tail: rest } => {
            let mut args: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut eone: Arc<DAE::Exp>;
            let mut tp: Arc<DAE::Type>;
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
            tp = Expression::r#typeof(e.clone())?;
            dims = Expression::arrayDimension(tp);
            (eone, _) = Expression::makeOneExpression(dims)?;
            args = List::set(inArgs.clone(), inCurrentArg, eone)?;
            { (inWorkLst, inArgs, inCurrentArg, inAccum) = (rest.clone(), inArgs, inCurrentArg + 1, metamodelica::cons(args, inAccum)); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn differentiatePartialFunction(mut inFunction: DAE::Function, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut maxIter: i32) -> Result<(DAE::Function, Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<bool>>)> {
    let mut outDerFunction: DAE::Function;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree>;
    let mut outBooleanlst: Arc<metamodelica::List<bool>>;
    (outDerFunction, outFunctionTree, outBooleanlst) = 'mc: {
        let __mc_input = inFunction.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut func = __mc_input.clone() else { bail!("nomatch") };
            let mut inputData: BackendDAE::DifferentiateInputData;
            let mut diffFuncData: BackendDAE::DifferentiateInputData;
            let mut path: Arc<Absyn::Path>;
            let mut dpath: Arc<Absyn::Path>;
            let mut isImpure: bool;
            let mut dinl: DAE::InlineType;
            let mut functions: Arc<AvlTreePathFunction::Tree>;
            let mut dtp: Arc<DAE::Type>;
            let mut funcbodyDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut inputVars: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut inputVarsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut inputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut outputVars: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut outputVarsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut outputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut protectedVars: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut protectedVarsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut protectedVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut newProtectedVars: Arc<metamodelica::List<Arc<DAE::Element>>>;
            let mut bodyStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>;
            let mut derbodyStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>;
            let mut dfunc: DAE::Function;
            let mut funcname: ArcStr;
            let mut funstring: ArcStr;
            let mut blst: Arc<metamodelica::List<bool>>;
            let mut visibility: SCode::Visibility;
            if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                funstring = (Tpl::tplString((std::sync::Arc::new(DAEDumpTpl::dumpFunction) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, DAE::Function) -> Result<Tpl::Text> + 'static>), func.clone())?).clone();
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Differentiate differentiateFunctionCallPartial: \n")); __mm_s.push_str(&*funstring.clone()); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            inputVars = DAEUtil::getFunctionInputVars(func.clone())?;
            outputVars = DAEUtil::getFunctionOutputVars(func.clone())?;
            protectedVars = DAEUtil::getFunctionProtectedVars(func.clone())?;
            bodyStmts = DAEUtil::getFunctionAlgorithmStmts(func.clone())?;
            visibility = DAEUtil::getFunctionVisibility(func.clone());
            (functions, inputVarsDer, inputVarsNoDer, outputVarsDer, outputVarsNoDer, blst) = getFunctionInOutVars(func.clone(), inFunctionTree.clone(), inDiffwrtCref.clone(), maxIter)?;
            path = DAEUtil::functionName(func.clone())?;
            funcname = (BackendUtil::modelicaStringToCStr((AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone(), false)?).clone();
            diffFuncData = BackendDAE::emptyInputData().clone();
            diffFuncData.matrixName = Some((funcname.clone()).clone());
            diffFuncData.diffedFunctions = inInputData.diffedFunctions.clone();
            (inputData, _) = addElementVars2Dep(inputVarsNoDer.clone(), functions.clone(), diffFuncData.clone())?;
            (inputData, _) = addElementVars2Dep(outputVarsNoDer.clone(), functions.clone(), inputData.clone())?;
            (protectedVarsDer, functions, protectedVarsNoDer, _) = differentiateElementVars(protectedVars.clone(), inDiffwrtCref.clone(), inputData.clone(), openmodelica_backend_types::BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION, functions.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), maxIter, false)?;
            (inputData, _) = addElementVars2Dep(protectedVarsNoDer.clone(), functions.clone(), inputData.clone())?;
            if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                dumpInputData(inputData.clone())?;
            }
            inputData.knownVars = addFunctionConstantsAndParameters(inputData.knownVars.clone(), func.clone())?;
            (derbodyStmts, functions) = differentiateStatements(bodyStmts.clone().reverse(), inDiffwrtCref.clone(), inputData.clone(), openmodelica_backend_types::BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION, metamodelica::nil(), functions.clone(), maxIter)?;
            if Flags::isSet(Flags::DEBUG_DIFFERENTIATION_VERBOSE.clone())? {
                funstring = (DAEDump::ppStmtListStr(derbodyStmts.clone(), 0)?).clone();
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Differentiate differentiateFunctionCallPartial stmts: \n")); __mm_s.push_str(&*funstring.clone()); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            (dpath, dtp) = getDiffedTypeandName(func.clone(), inputVarsDer.clone(), outputVarsDer.clone(), blst.clone())?;
            newProtectedVars = List::map1(outputVars.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::setElementVarVisibility, Arc<DAE::Element>, DAE::VarVisibility)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, DAE::VarVisibility) -> Result<Arc<DAE::Element>> + 'static>), openmodelica_frontend_types::DAE::VarVisibility::PROTECTED)?;
            newProtectedVars = List::map1(newProtectedVars.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::setElementVarDirection, Arc<DAE::Element>, DAE::VarDirection)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, DAE::VarDirection) -> Result<Arc<DAE::Element>> + 'static>), openmodelica_frontend_types::DAE::VarDirection::BIDIR)?;
            funcbodyDer = listAppend(newProtectedVars.clone(), list![Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: derbodyStmts.clone() }), source: DAE::emptyElementSource().clone() })]);
            funcbodyDer = listAppend(protectedVarsDer.clone(), funcbodyDer.clone());
            funcbodyDer = listAppend(protectedVars.clone(), funcbodyDer.clone());
            funcbodyDer = listAppend(outputVarsDer.clone(), funcbodyDer.clone());
            funcbodyDer = listAppend(inputVarsDer.clone(), funcbodyDer.clone());
            funcbodyDer = listAppend(inputVars.clone(), funcbodyDer.clone());
            isImpure = DAEUtil::getFunctionImpureAttribute(func.clone())?;
            dinl = DAEUtil::getFunctionInlineType(func.clone())?;
            dfunc = DAE::Function::FUNCTION { path: dpath.clone(), functions: list![DAE::FunctionDefinition::FUNCTION_DEF { body: funcbodyDer.clone() }], type_: dtp.clone(), visibility: visibility.clone(), partialPrefix: false, isImpure: isImpure.clone(), inlineType: dinl.clone(), unusedInputs: metamodelica::nil(), source: DAE::emptyElementSource().clone(), comment: None };
            Ok((dfunc.clone(), functions.clone(), blst.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut path: Arc<Absyn::Path>;
            let mut r#str: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            path = DAEUtil::functionName(inFunction.clone())?;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nDifferentiate.differentiatePartialFunction failed for function: ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            Debug::trace((r#str.clone()).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outDerFunction, outFunctionTree, outBooleanlst))
}

fn getDiffedTypeandName(mut inFunction: DAE::Function, mut inputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>>, mut outputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>>, mut blst: Arc<metamodelica::List<bool>>) -> Result<(Arc<Absyn::Path>, Arc<DAE::Type>)> {
    let mut diffedName: Arc<Absyn::Path>;
    let mut diffedType: Arc<DAE::Type>;
    diffedType = Types::extendsFunctionTypeArgs(DAEUtil::getFunctionType(inFunction.clone())?, inputVarsDer, outputVarsDer, blst)?;
    diffedName = AbsynUtil::stringPath(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$DER")); __mm_s.push_str(&*BackendUtil::modelicaStringToCStr((AbsynUtil::pathString(DAEUtil::functionName(inFunction)?, (literal!(".")).clone(), true, false)?).clone(), false)?); ArcStr::from(__mm_s) }).clone())?;
    Ok((diffedName, diffedType))
}

fn getFunctionInOutVars(mut inFunction: DAE::Function, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut maxIter: i32) -> Result<(Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<bool>>)> {
    let mut functions: Arc<AvlTreePathFunction::Tree> = inFunctionTree.clone();
    let mut inputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut inputVarsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut outputVarsDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut outputVarsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut blst: Arc<metamodelica::List<bool>>;
    let mut inputVars: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut outputVars: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut diffData: BackendDAE::DifferentiateInputData;
    inputVars = DAEUtil::getFunctionInputVars(inFunction.clone())?;
    outputVars = DAEUtil::getFunctionOutputVars(inFunction.clone())?;
    diffData = BackendDAE::emptyInputData().clone();
    diffData.matrixName = Some((BackendUtil::modelicaStringToCStr((AbsynUtil::pathString(DAEUtil::functionName(inFunction)?, (literal!(".")).clone(), true, false)?).clone(), false)?).clone());
    (inputVarsDer, functions, inputVarsNoDer, blst) = differentiateElementVars(inputVars, inDiffwrtCref.clone(), diffData.clone(), openmodelica_backend_types::BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION, functions, metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), maxIter, true)?;
    (outputVarsDer, functions, outputVarsNoDer, _) = differentiateElementVars(outputVars, inDiffwrtCref, diffData, openmodelica_backend_types::BackendDAE::DifferentiationType::DIFFERENTIATION_FUNCTION, functions, metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), maxIter, false)?;
    Ok((functions, inputVarsDer, inputVarsNoDer, outputVarsDer, outputVarsNoDer, blst))
}

fn differentiateElementVars(mut inElements: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inDiffwrtCref: Arc<DAE::ComponentRef>, mut inInputData: BackendDAE::DifferentiateInputData, mut inDiffType: BackendDAE::DifferentiationType, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut inElementsDer: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inElementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inBooleanLst: Arc<metamodelica::List<bool>>, mut maxIter: i32, mut elementListInputs: bool) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<AvlTreePathFunction::Tree>, Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<bool>>)> {
    let mut outElements: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree>;
    let mut outElementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut outBooleanLst: Arc<metamodelica::List<bool>>;
    (outElements, outFunctionTree, outElementsNoDer, outBooleanLst) = 'mc: {
        let __mc_input = (inElements, inInputData.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok((metamodelica::Dangerous::listReverseInPlace(inElementsDer.clone()), inFunctionTree.clone(), metamodelica::Dangerous::listReverseInPlace(inElementsNoDer.clone()), metamodelica::Dangerous::listReverseInPlace(inBooleanLst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var1 @ Deref @ DAE::Element::VAR { componentRef: cref, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, binding: Some(binding), .. }, tail: rest }, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut elementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut var: Arc<DAE::Element>;
                    let mut dcref: Arc<DAE::ComponentRef>;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut dbinding: Arc<DAE::Exp>;
                    let mut blst: Arc<metamodelica::List<bool>>;
                    dcref = createDiffedCrefName(cref.clone(), (matrixName.clone()).clone())?;
                    var = DAEUtil::replaceCrefInVar(dcref.clone(), var1.clone())?;
                    (dbinding, _) = differentiateExp(binding.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter)?;
                    var = DAEUtil::replaceBindungInVar(dbinding.clone(), var.clone())?;
                    vars = metamodelica::cons(var.clone(), inElementsDer.clone());
                    blst = metamodelica::cons(true, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), vars.clone(), inElementsNoDer.clone(), blst.clone(), maxIter, elementListInputs)?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var1 @ Deref @ DAE::Element::VAR { componentRef: cref, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, .. }, tail: rest }, BackendDAE::DifferentiateInputData { matrixName: Some(matrixName), .. }) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut elementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut var: Arc<DAE::Element>;
                    let mut dcref: Arc<DAE::ComponentRef>;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut blst: Arc<metamodelica::List<bool>>;
                    dcref = createDiffedCrefName(cref.clone(), (matrixName.clone()).clone())?;
                    var = DAEUtil::replaceCrefInVar(dcref.clone(), var1.clone())?;
                    vars = metamodelica::cons(var.clone(), inElementsDer.clone());
                    blst = metamodelica::cons(true, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), vars.clone(), inElementsNoDer.clone(), blst.clone(), maxIter, elementListInputs)?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var @ Deref @ DAE::Element::VAR { binding: Some(binding), .. }, tail: rest }, BackendDAE::DifferentiateInputData { independenentVars: Some(timevars), .. }) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut elementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut blst: Arc<metamodelica::List<bool>>;
                    crefLst = Expression::extractCrefsFromExp(binding.clone())?;
                    ::match_deref::match_deref! { match &(BackendVariable::getVarLst(crefLst.clone(), timevars.clone())) {
                        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    vars = metamodelica::cons(var.clone(), inElementsNoDer.clone());
                    blst = metamodelica::cons(false, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), inElementsDer.clone(), vars.clone(), blst.clone(), maxIter, elementListInputs)?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var1 @ Deref @ DAE::Element::VAR { componentRef: cref, ty: tp, binding: Some(binding), .. }, tail: rest }, _) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut elementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut var: Arc<DAE::Element>;
                    let mut dcref: Arc<DAE::ComponentRef>;
                    let mut e: Arc<DAE::Exp>;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut dbinding: Arc<DAE::Exp>;
                    let mut blst: Arc<metamodelica::List<bool>>;
                    if elementListInputs {
                        let true = (Types::isRealOrSubTypeReal(tp.clone())) else { bail!("pattern mismatch") };
                    }
                    e = Expression::crefExp(cref.clone())?;
                    (e, functions) = differentiateCrefs(e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter)?;
                    dcref = Expression::expCref(e.clone())?;
                    var = DAEUtil::replaceCrefInVar(dcref.clone(), var1.clone())?;
                    (dbinding, functions) = differentiateExp(binding.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter)?;
                    var = DAEUtil::replaceBindungInVar(dbinding.clone(), var.clone())?;
                    vars = metamodelica::cons(var.clone(), inElementsDer.clone());
                    blst = metamodelica::cons(true, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functions.clone(), vars.clone(), inElementsNoDer.clone(), blst.clone(), maxIter, elementListInputs)?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var1 @ Deref @ DAE::Element::VAR { componentRef: cref, ty: tp, .. }, tail: rest }, _) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut elementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut var: Arc<DAE::Element>;
                    let mut dcref: Arc<DAE::ComponentRef>;
                    let mut e: Arc<DAE::Exp>;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut blst: Arc<metamodelica::List<bool>>;
                    if elementListInputs {
                        let true = (Types::isRealOrSubTypeReal(tp.clone())) else { bail!("pattern mismatch") };
                    }
                    e = Expression::crefExp(cref.clone())?;
                    (e, functions) = differentiateCrefs(e.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), maxIter)?;
                    dcref = Expression::expCref(e.clone())?;
                    var = DAEUtil::replaceCrefInVar(dcref.clone(), var1.clone())?;
                    vars = metamodelica::cons(var.clone(), inElementsDer.clone());
                    blst = metamodelica::cons(true, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), functions.clone(), vars.clone(), inElementsNoDer.clone(), blst.clone(), maxIter, elementListInputs)?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var @ Deref @ DAE::Element::VAR { .. }, tail: rest }, _) => {
                    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut elementsNoDer: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    let mut functions: Arc<AvlTreePathFunction::Tree>;
                    let mut blst: Arc<metamodelica::List<bool>>;
                    elementsNoDer = metamodelica::cons(var.clone(), inElementsNoDer.clone());
                    blst = metamodelica::cons(false, inBooleanLst.clone());
                    (vars, functions, elementsNoDer, blst) = differentiateElementVars(rest.clone(), inDiffwrtCref.clone(), inInputData.clone(), inDiffType.clone(), inFunctionTree.clone(), inElementsDer.clone(), elementsNoDer.clone(), blst.clone(), maxIter, elementListInputs)?;
                    Ok((vars.clone(), functions.clone(), elementsNoDer.clone(), blst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outElements, outFunctionTree, outElementsNoDer, outBooleanLst))
}

fn differentiateFunction1(mut inFuncName: Arc<Absyn::Path>, mut inMapper: DAE::FunctionDefinition, mut inTp: Arc<DAE::Type>, mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffArgs: (Arc<DAE::ComponentRef>, BackendDAE::DifferentiateInputData, BackendDAE::DifferentiationType, Arc<AvlTreePathFunction::Tree>)) -> Result<(Arc<Absyn::Path>, Arc<metamodelica::List<bool>>)> {
    let mut outFuncName: Arc<Absyn::Path>;
    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
    (outFuncName, blst) = 'mc: {
        let __mc_input = (inMapper, inTp, inDiffArgs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivativeFunction: inDFuncName, derivativeOrder, conditionRefs: cr, .. }, Deref @ DAE::Type::T_FUNCTION { funcArg, .. }, _) => {
                    if !((intEq(1, derivativeOrder.clone()))) { bail!("guard") }
                    let mut tplst: Arc<metamodelica::List<Arc<DAE::Type>>>;
                    let mut bl: Arc<metamodelica::List<bool>>;
                    let mut ba: metamodelica::Array<bool>;
                    tplst = List::map(funcArg.clone(), (std::sync::Arc::new(Types::funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>))?;
                    ba = Array::mapList(tplst.clone(), (std::sync::Arc::new(fnptr!(diffableTypes, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<bool> + 'static>))?;
                    bl = checkDerFunctionConds(ba.clone(), cr.clone(), expl.clone(), inDiffArgs.clone())?;
                    Ok((inDFuncName.clone(), bl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivativeFunction: inDFuncName, derivativeOrder, conditionRefs: cr, .. }, tp, (_, _, _, functions)) => {
                    if !((!(intEq(1, derivativeOrder.clone())))) { bail!("guard") }
                    let mut fname: Arc<Absyn::Path>;
                    let mut bl: Arc<metamodelica::List<bool>>;
                    let mut mapper: DAE::FunctionDefinition;
                    let mut ba: metamodelica::Array<bool>;
                    let mut tp = (*tp).clone();
                    let mut blst: Arc<metamodelica::List<bool>> = blst.clone();
                    fname = getlowerOrderDerivative(inFuncName.clone(), functions.clone())?;
                    (mapper, tp) = getFunctionMapper(fname.clone(), functions.clone())?;
                    (_, blst) = differentiateFunction1(fname.clone(), mapper.clone(), tp.clone(), expl.clone(), inDiffArgs.clone())?;
                    (bl, _) = List::split1OnTrue(blst.clone(), std::sync::Arc::new(fnptr!(valueEq, _, _)), true)?;
                    ba = metamodelica::arrayAppend(arrayCreate((blst.clone().len() as i32), false), metamodelica::arrayFromVec(bl.clone().into_iter().cloned().collect()));
                    bl = checkDerFunctionConds(ba.clone(), cr.clone(), expl.clone(), inDiffArgs.clone())?;
                    Ok(((inDFuncName.clone(), bl.clone()), blst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { blst = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivedFunction: fname, derivativeOrder, defaultDerivative: Some(default), lowerOrderDerivatives, .. }, tp, _) => {
                    let mut da: Arc<Absyn::Path>;
                    let mut bl: Arc<metamodelica::List<bool>>;
                    (da, bl) = differentiateFunction1(inFuncName.clone(), DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivedFunction: fname.clone(), derivativeFunction: default.clone(), derivativeOrder: derivativeOrder.clone(), conditionRefs: metamodelica::nil(), defaultDerivative: Some(default.clone()), lowerOrderDerivatives: lowerOrderDerivatives.clone() }, tp.clone(), expl.clone(), inDiffArgs.clone())?;
                    Ok((da.clone(), bl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outFuncName, blst))
}

fn checkDerivativeFunctionInputs(mut blst: Arc<metamodelica::List<bool>>, mut tp: Arc<DAE::Type>, mut dtp: Arc<DAE::Type>) -> Result<(bool, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outBoolean: bool;
    let mut outExpectedTypeLst: Arc<metamodelica::List<Arc<DAE::Type>>>;
    (outBoolean, outExpectedTypeLst) = 'mc: {
        let __mc_input = (tp, dtp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_FUNCTION { funcArg: falst, .. }, Deref @ DAE::Type::T_FUNCTION { funcArg: dfalst, .. }) => {
                    let mut falst1: Arc<metamodelica::List<Arc<DAE::FuncArg>>>;
                    let mut falst2: Arc<metamodelica::List<Arc<DAE::FuncArg>>>;
                    let mut tlst: Arc<metamodelica::List<Arc<DAE::Type>>>;
                    let mut dtlst: Arc<metamodelica::List<Arc<DAE::Type>>>;
                    let mut ret: bool;
                    (falst1, _) = List::splitOnBoolList(falst.clone(), blst.clone())?;
                    falst2 = listAppend(falst.clone(), falst1.clone());
                    tlst = List::map(falst2.clone(), (std::sync::Arc::new(Types::funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>))?;
                    dtlst = List::map(dfalst.clone(), (std::sync::Arc::new(Types::funcArgType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::FuncArg>) -> Result<Arc<DAE::Type>> + 'static>))?;
                    ret = List::isEqualOnTrue(tlst.clone(), dtlst.clone(), (std::sync::Arc::new(fnptr!(Types::equivtypes, Arc<DAE::Type>, Arc<DAE::Type>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>, Arc<DAE::Type>) -> Result<bool> + 'static>))?;
                    Ok((ret.clone(), tlst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-Differentiate.checkDerivativeFunctionInputs failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outBoolean, outExpectedTypeLst))
}

fn checkDerFunctionConds(mut inbarr: metamodelica::Array<bool>, mut icrlst: Arc<metamodelica::List<(i32, DAE::derivativeCond)>>, mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inDiffArgs: (Arc<DAE::ComponentRef>, BackendDAE::DifferentiateInputData, BackendDAE::DifferentiationType, Arc<AvlTreePathFunction::Tree>)) -> Result<Arc<metamodelica::List<bool>>> {
    let mut outblst: Arc<metamodelica::List<bool>>;
    let mut i: i32;
    let mut dc: DAE::derivativeCond;
    let mut e: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut p1: Arc<Absyn::Path> = Arc::new(<Absyn::Path as ::std::default::Default>::default());
    let mut p2: Arc<Absyn::Path> = Arc::new(<Absyn::Path as ::std::default::Default>::default());
    let mut ba: metamodelica::Array<bool> = inbarr.clone();
    let mut diffwrtCref: Arc<DAE::ComponentRef>;
    let mut inputData: BackendDAE::DifferentiateInputData;
    let mut diffType: BackendDAE::DifferentiationType;
    let mut functionTree: Arc<AvlTreePathFunction::Tree>;
    (diffwrtCref, inputData, diffType, functionTree) = inDiffArgs;
    for mut tpl in &*icrlst {
        let mut tpl = tpl.clone();
        (i, dc) = tpl.clone();
        let () = 'mc: {
        let __mc_input = dc.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::derivativeCond::ZERO_DERIVATIVE { .. } => {
                    let mut e: Arc<DAE::Exp> = e.clone();
                    let mut functionTree: Arc<AvlTreePathFunction::Tree> = functionTree.clone();
                    e = (expl.clone()).get(i)?;
                    (e, functionTree) = differentiateExp(e.clone(), diffwrtCref.clone(), inputData.clone(), diffType.clone(), functionTree.clone(), defaultMaxIter.clone())?;
                    let true = (Expression::isZero(e.clone())?) else { bail!("pattern mismatch") };
                    Ok(((), e.clone(), functionTree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { e = __wb0; functionTree = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::derivativeCond::NO_DERIVATIVE { binding: Deref @ DAE::Exp::CALL { path: p1, .. } } => {
                    let mut p2: Arc<Absyn::Path> = p2.clone();
                    let __pa0 = ::match_deref::match_deref! { match &((expl.clone()).get(i)?) {
                        Deref @ DAE::Exp::CALL { path: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    p2 = __pa0.clone();
                    let true = (AbsynUtil::pathEqual(p1.clone(), p2.clone())) else { bail!("pattern mismatch") };
                    Ok(((), p2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { p2 = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::derivativeCond::NO_DERIVATIVE { binding: Deref @ DAE::Exp::ICONST { .. } } => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("-Differentiate.checkDerFunctionConds failed")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        metamodelica::arrayUpdate(ba.clone(), i, false)?;
    }
    outblst = Arc::new(ba.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    Ok(outblst)
}

fn getlowerOrderDerivative(mut fname: Arc<Absyn::Path>, mut functions: Arc<AvlTreePathFunction::Tree>) -> Result<Arc<Absyn::Path>> {
    let mut outFName: Arc<Absyn::Path>;
    outFName = (::match_deref::match_deref! { match &(functions.clone()) {
        _ => {
            let mut flst: Arc<metamodelica::List<DAE::FunctionDefinition>>;
            let mut lowerOrderDerivatives: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut name: Arc<Absyn::Path>;
            let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(functions, fname)?) {
                Some(DAE::Function::FUNCTION { functions: __pa0, .. }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            flst = __pa0.clone();
            let DAE::FUNCTION_DER_MAPPER { lowerOrderDerivatives: __pa1, .. } = (getFunctionMapper1(flst)?) else { bail!("pattern mismatch") };
            lowerOrderDerivatives = __pa1.clone();
            name = List::last(lowerOrderDerivatives)?;
            name
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outFName)
}

pub(crate) fn getFunctionMapper(mut fname: Arc<Absyn::Path>, mut functions: Arc<AvlTreePathFunction::Tree>) -> Result<(DAE::FunctionDefinition, Arc<DAE::Type>)> {
    let mut mapper: DAE::FunctionDefinition;
    let mut tp: Arc<DAE::Type>;
    (mapper, tp) = 'mc: {
        let __mc_input = functions.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut flst: Arc<metamodelica::List<DAE::FunctionDefinition>>;
                    let mut t: Arc<DAE::Type>;
                    let mut m: DAE::FunctionDefinition;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(functions.clone(), fname.clone())?) {
                        Some(DAE::Function::FUNCTION { functions: __pa0, type_: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    flst = __pa0.clone();
                    t = __pa1.clone();
                    m = getFunctionMapper1(flst.clone())?;
                    Ok((m.clone(), t.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    s = (AbsynUtil::pathString(fname.clone(), (literal!(".")).clone(), true, false)?).clone();
                    s = (stringAppend((literal!("-Differentiate.getFunctionMapper failed for function ")).clone(), (s.clone()).clone())).clone();
                    Debug::traceln((s.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((mapper, tp))
}

fn getFunctionMapper1(mut inFuncDefs: Arc<metamodelica::List<DAE::FunctionDefinition>>) -> Result<DAE::FunctionDefinition> {
    let mut mapper: DAE::FunctionDefinition;
    mapper = 'mc: {
        let __mc_input = inFuncDefs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: m @ DAE::FunctionDefinition::FUNCTION_DER_MAPPER { .. }, tail: _ } => {
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: funcDefs } => {
                    let mut m: DAE::FunctionDefinition;
                    m = getFunctionMapper1(funcDefs.clone())?;
                    Ok(m.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-Differentiate.getFunctionMapper1 failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(mapper)
}

fn diffableTypes(mut inType: Arc<DAE::Type>) -> bool {
    let mut out: bool = Types::isRealOrSubTypeReal(inType.clone()) || Types::isRecord(inType.clone());
    out
}

//
// util functions for Types: DifferentiateInputData, DifferentiateInputArguments, DifferentiationType
//
fn addDependentVars(mut inVarsLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inDiffData: BackendDAE::DifferentiateInputData) -> Result<BackendDAE::DifferentiateInputData> {
    let mut outDiffData: BackendDAE::DifferentiateInputData = inDiffData.clone();
    let mut depVars: BackendDAE::Variables;
    if isSome(outDiffData.dependenentVars.clone()) {
        depVars = BackendVariable::addVars(inVarsLst, Util::getOption(outDiffData.dependenentVars.clone())?)?;
    } else {
        depVars = BackendVariable::listVar(inVarsLst)?;
    }
    outDiffData.dependenentVars = Some(depVars);
    Ok(outDiffData)
}

fn addAllVars(mut inVarsLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inDiffData: BackendDAE::DifferentiateInputData) -> Result<BackendDAE::DifferentiateInputData> {
    let mut outDiffData: BackendDAE::DifferentiateInputData = inDiffData.clone();
    let mut allVars: BackendDAE::Variables;
    if isSome(outDiffData.allVars.clone()) {
        allVars = BackendVariable::addVars(inVarsLst, Util::getOption(outDiffData.allVars.clone())?)?;
    } else {
        allVars = BackendVariable::listVar(inVarsLst)?;
    }
    outDiffData.allVars = Some(allVars);
    Ok(outDiffData)
}

fn addGlobalVars(mut inVarsLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inDiffData: BackendDAE::DifferentiateInputData) -> Result<BackendDAE::DifferentiateInputData> {
    let mut outDiffData: BackendDAE::DifferentiateInputData = inDiffData.clone();
    let mut glVars: BackendDAE::Variables;
    if isSome(outDiffData.knownVars.clone()) {
        glVars = BackendVariable::addVars(inVarsLst, Util::getOption(outDiffData.knownVars.clone())?)?;
    } else {
        glVars = BackendVariable::listVar(inVarsLst)?;
    }
    outDiffData.knownVars = Some(glVars);
    Ok(outDiffData)
}

fn lowerVarsElementVars(mut inElementLstVars: Arc<metamodelica::List<Arc<DAE::Element>>>, mut functions: Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut varsLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut reqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut knvars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut exvars: Arc<metamodelica::List<BackendDAE::Var>>;
    if '__try0: {
        (vars, knvars, exvars, eqnsLst, reqnsLst) = unwrap_break_err!(BackendDAECreate::lowerVars(inElementLstVars.clone(), functions.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), '__try0);
        varsLst = listAppend(exvars.clone(), listAppend(vars.clone(), knvars.clone()));
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
        Debug::traceln((literal!("- Differentiate.lowerVarsElementVars failed.")).clone())?;
    }
    Ok((varsLst, eqnsLst, reqnsLst))
}

fn addElementVars2Dep(mut inElementLstVars: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inFunctions: Arc<AvlTreePathFunction::Tree>, mut inDiffData: BackendDAE::DifferentiateInputData) -> Result<(BackendDAE::DifferentiateInputData, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outDiffData: BackendDAE::DifferentiateInputData;
    let mut outEqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut varsLst: Arc<metamodelica::List<BackendDAE::Var>>;
    match '__try0: {
        (varsLst, outEqnsLst, _) = unwrap_break_err!(lowerVarsElementVars(inElementLstVars.clone(), inFunctions.clone()), '__try0);
        outDiffData = unwrap_break_err!(addDependentVars(varsLst.clone(), inDiffData.clone()), '__try0);
        Ok::<_, anyhow::Error>((outDiffData.clone(), outEqnsLst.clone(), varsLst.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            outDiffData = __try0_o0;
            outEqnsLst = __try0_o1;
            varsLst = __try0_o2;
        }
        Err(__try0_err) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Differentiate.addElementVars2Dep failed")).clone()])?;
            return Err(__try0_err);
        }
    }
    Ok((outDiffData, outEqnsLst))
}

fn dumpInputData(mut inDiffData: BackendDAE::DifferentiateInputData) -> Result<()> {
    let mut independenentVars: Option<BackendDAE::Variables>;
    let mut dependenentVars: Option<BackendDAE::Variables>;
    let mut knownVars: Option<BackendDAE::Variables>;
    let mut allVars: Option<BackendDAE::Variables>;
    let mut controlVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut diffCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut matrixName: Option<ArcStr>;
    metamodelica::print((literal!("### dumpInputData ###\n")).clone());
    if isSome(inDiffData.matrixName.clone()) {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### for ")); __mm_s.push_str(&*Util::getOption(inDiffData.matrixName.clone())?); __mm_s.push_str(&*literal!(" ###\n")); ArcStr::from(__mm_s) }).clone());
    }
    if isSome(inDiffData.independenentVars.clone()) {
        metamodelica::print((literal!("independentVars:\n")).clone());
        BackendDump::printVariables(Util::getOption(inDiffData.independenentVars.clone())?)?;
    }
    if isSome(inDiffData.dependenentVars.clone()) {
        metamodelica::print((literal!("dependenentVars:\n")).clone());
        BackendDump::printVariables(Util::getOption(inDiffData.dependenentVars.clone())?)?;
    }
    if isSome(inDiffData.knownVars.clone()) {
        metamodelica::print((literal!("knownVars:\n")).clone());
        BackendDump::printVariables(Util::getOption(inDiffData.knownVars.clone())?)?;
    }
    if isSome(inDiffData.allVars.clone()) {
        metamodelica::print((literal!("allVars:\n")).clone());
        BackendDump::printVariables(Util::getOption(inDiffData.allVars.clone())?)?;
    }
    if !(inDiffData.controlVars.clone().is_empty()) {
        metamodelica::print((literal!("controlVars:\n")).clone());
        BackendDump::printVarList(inDiffData.controlVars.clone())?;
    }
    if !(inDiffData.diffCrefs.clone().is_empty()) {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("diffCrefs:\n")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefListStr(inDiffData.diffCrefs.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

fn isParamOrConstant(mut cref: Arc<DAE::ComponentRef>, mut diffData: BackendDAE::DifferentiateInputData) -> Result<bool> {
    let mut b: bool = false;
    b = (match diffData {
        BackendDAE::DifferentiateInputData { knownVars: Some(mut knownVars), .. } => {
            let mut var_lst: Option<Arc<metamodelica::List<BackendDAE::Var>>>;
            let mut var: BackendDAE::Var;
            var_lst = BackendVariable::getVarTryHard(cref, knownVars.clone());
            if isSome(var_lst.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(Util::getOption(var_lst)?) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                var = __pa0.clone();
                b = BackendVariable::isParamOrConstant(var);
            } else {
                b = false;
            }
            b
        },
        _ => {
            false
        },
    });
    Ok(b)
}

