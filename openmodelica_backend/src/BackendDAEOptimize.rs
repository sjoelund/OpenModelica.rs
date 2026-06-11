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

use crate::AdjacencyMatrix;
use crate::AvlSetInt;
use crate::BackendDAETransform;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendInline;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::Differentiate;
use crate::ExpressionSolve;
use crate::HpcOmTaskGraph;
use crate::Matching;
use crate::RewriteRules;
use crate::SynchronousFeatures;
use crate::Tearing;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_util::BackendDAEEXT;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend::HashSet;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend_base::Algorithm;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::HashTable2;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub(crate) fn simplifyAllExpressions(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut removedEqsList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    BackendDAEUtil::traverseBackendDAEExpsNoCopyWithUpdate(outDAE.clone(), (std::sync::Arc::new(ExpressionSimplify::simplify1TraverseHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), 0)?;
    shared = outDAE.shared.clone();
    for mut eq in &*BackendEquation::equationList(shared.removedEqs.clone())? {
        let mut eq = eq.clone();
        removedEqsList = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Nil }, .. } => removedEqsList.clone(),
        _ => metamodelica::cons(eq.clone(), removedEqsList.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    assign_field!(shared.removedEqs = BackendEquation::listEquation(metamodelica::Dangerous::listReverseInPlace(removedEqsList))?);
    assign_field!(outDAE.shared = shared);
    Ok(outDAE)
}

// =============================================================================
// simplifyInStream
//
// OM introduces $OMC$PositiveMax which can simplified using min or max attribute
// see Modelica spec for inStream
// author: Vitalij Ruge
// see. #3885, 4441, 5104
// =============================================================================
pub(crate) fn simplifyInStream(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut dae: Arc<BackendDAE::BackendDAE> = dae;
    let mut shared: Arc<BackendDAE::Shared> = dae.shared.clone();
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = dae.eqs.clone();
    let mut vars: Arc<metamodelica::List<BackendDAE::Variables>> = ({
        let mut __acc: Arc<metamodelica::List<BackendDAE::Variables>> = metamodelica::nil();
        for mut eq in (eqs.clone()).into_iter().cloned() {
            let __x = eq.orderedVars.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    vars = metamodelica::cons(shared.globalKnownVars.clone(), vars);
    vars = metamodelica::cons(shared.localKnownVars.clone(), vars);
    BackendDAEUtil::traverseBackendDAEExpsNoCopyWithUpdate(dae.clone(), (std::sync::Arc::new(simplifyInStreamWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Variables>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Variables>>)> + 'static>), vars)?;
    Ok(dae)
}

fn simplifyInStreamWork(mut inExp: Arc<DAE::Exp>, mut inVars: Arc<metamodelica::List<BackendDAE::Variables>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Variables>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outVars: Arc<metamodelica::List<BackendDAE::Variables>> = inVars.clone();
    (outExp, _) = Expression::traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(simplifyInStreamWork2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Variables>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Variables>>)> + 'static>), outVars.clone())?;
    if !(ExpressionBasics::expEqual(outExp.clone(), inExp)?) {
        (outExp, _) = ExpressionSimplify::simplify(outExp)?;
    }
    Ok((outExp, outVars))
}

fn simplifyInStreamWork2(mut inExp: Arc<DAE::Exp>, mut inVars: Arc<metamodelica::List<BackendDAE::Variables>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<BackendDAE::Variables>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outVars: Arc<metamodelica::List<BackendDAE::Variables>> = inVars.clone();
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$OMC$PositiveMax" }, expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Cons { head: expr, tail: Deref @ metamodelica::List::Nil } }, .. } => {
            let mut tp: Arc<DAE::Type>;
            let mut ret: Arc<DAE::Exp>;
            let mut eMin: Option<Arc<DAE::Exp>>;
            let mut eMax: Option<Arc<DAE::Exp>>;
            (eMin, eMax) = simplifyInStreamGetMinMaxAttributes(cr.clone(), outVars.clone());
            tp = ComponentReference::crefTypeFull(cr.clone())?;
            ret = if (Util::applyOptionOrDefault(eMax, (std::sync::Arc::new(Expression::isNegativeOrZero) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), false)?) {Expression::createZeroExpression(tp.clone())?} else if (Util::applyOptionOrDefault(eMin, (std::sync::Arc::new({ let __pe_b1 = expr.clone(); move |__pe_a0| Expression::isGreaterOrEqual(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), false)?) {e.clone()} else {Expression::makePureBuiltinCall((literal!("max")).clone(), list![e.clone(), expr.clone()], tp.clone())};
            ret
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$OMC$PositiveMax" }, expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, tail: Deref @ metamodelica::List::Cons { head: expr, tail: Deref @ metamodelica::List::Nil } }, .. } => {
            let mut ret: Arc<DAE::Exp>;
            let mut eMin: Option<Arc<DAE::Exp>>;
            let mut eMax: Option<Arc<DAE::Exp>>;
            (eMin, eMax) = simplifyInStreamGetMinMaxAttributes(cr.clone(), outVars.clone());
            ret = if (Util::applyOptionOrDefault(eMin, (std::sync::Arc::new(Expression::isPositiveOrZero) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), false)?) {Expression::createZeroExpression(tp.clone())?} else if (Util::applyOptionOrDefault(eMax, (std::sync::Arc::new({ let __pe_b0 = Expression::negate(expr.clone())?; move |__pe_a1| Expression::isGreaterOrEqual(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), false)?) {e.clone()} else {Expression::makePureBuiltinCall((literal!("max")).clone(), list![e.clone(), expr.clone()], tp.clone())};
            ret
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$OMC$PositiveMax" }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: expr, tail: Deref @ metamodelica::List::Nil } }, .. } => {
            Expression::makePureBuiltinCall((literal!("max")).clone(), list![e.clone(), expr.clone()], Expression::r#typeof(e.clone())?)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$OMC$inStreamDiv" }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: expr, tail: Deref @ metamodelica::List::Nil } }, .. } => {
            let mut ret: Arc<DAE::Exp>;
            let mut e = (*e).clone();
            (e, _) = ExpressionSimplify::simplify(e.clone())?;
            ret = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: a, operator: DAE::Operator::DIV { .. }, exp2: b } if (Expression::isZero(a.clone())? && Expression::isZero(b.clone())?) => {
            expr.clone()
        },
        _ => {
            e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ret
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outVars))
}

fn simplifyInStreamGetMinMaxAttributes(mut cr: Arc<DAE::ComponentRef>, mut inVars: Arc<metamodelica::List<BackendDAE::Variables>>) -> (Option<Arc<DAE::Exp>>, Option<Arc<DAE::Exp>>) {
    let mut outMin: Option<Arc<DAE::Exp>> = None;
    let mut outMax: Option<Arc<DAE::Exp>> = None;
    let mut v: BackendDAE::Var;
    '__loop0: for mut vars in &*inVars {
        let mut vars = vars.clone();
        if '__try1: {
            (v, _) = unwrap_break_err!(BackendVariable::getVarSingle(cr.clone(), vars.clone()), '__try1);
            (outMin, outMax) = BackendVariable::getMinMaxAttribute(v.clone());
            break '__loop0;
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    (outMin, outMax)
}

// =============================================================================
// simplify time independent function calls
//
// public functions:
//   - simplifyTimeIndepFuncCalls
// =============================================================================
pub(crate) fn simplifyTimeIndepFuncCalls(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE, (std::sync::Arc::new(fnptr!(simplifyTimeIndepFuncCalls0, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> + 'static>), false)?;
    outDAE = simplifyTimeIndepFuncCallsShared(outDAE)?;
    Ok(outDAE)
}

fn simplifyTimeIndepFuncCalls0(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inChanged: bool) -> (Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool) {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared>;
    let mut outChanged: bool;
    (osyst, outShared, outChanged) = 'mc: {
        let __mc_input = (isyst.clone(), inShared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst, shared) => {
                    let (_, (_, _, true)) = (BackendDAEUtil::traverseBackendDAEExpsEqns(syst.orderedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(traverserExpsimplifyTimeIndepFuncCalls, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))> + 'static>), (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false)))?) else { bail!("pattern mismatch") };
                    let (_, (_, _, true)) = (BackendDAEUtil::traverseBackendDAEExpsEqns(syst.removedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(traverserExpsimplifyTimeIndepFuncCalls, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))> + 'static>), (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false)))?) else { bail!("pattern mismatch") };
                    Ok((isyst.clone(), inShared.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), inShared.clone(), inChanged))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (osyst, outShared, outChanged)
}

fn traverserExpsimplifyTimeIndepFuncCalls(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, BackendDAE::Variables, bool)) -> (Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables, bool);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, tail: Deref @ metamodelica::List::Nil }, .. }, (globalKnownVars, aliasvars, _)) => {
                    let mut zero: Arc<DAE::Exp>;
                    let mut var: BackendDAE::Var;
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), globalKnownVars.clone())?;
                    let false = (BackendVariable::isVarOnTopLevelAndInput(var.clone())) else { bail!("pattern mismatch") };
                    (zero, _) = Expression::makeZeroExpression(Expression::arrayDimension(tp.clone()))?;
                    Ok((zero.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("pre") || idn.clone() == literal!("previous"))) { bail!("guard") }
                    let mut var: BackendDAE::Var;
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), globalKnownVars.clone())?;
                    let false = (BackendVariable::isInput(var.clone())) else { bail!("pattern mismatch") };
                    Ok((e.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("pre") || idn.clone() == literal!("previous"))) { bail!("guard") }
                    Ok((e.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::List::Cons { head: e @ Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. } }, tail: Deref @ metamodelica::List::Nil }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("pre") || idn.clone() == literal!("previous"))) { bail!("guard") }
                    Ok((e.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, tail: Deref @ metamodelica::List::Nil }, attr }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("pre") || idn.clone() == literal!("previous"))) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    let mut var: BackendDAE::Var;
                    let mut negate: bool;
                    let mut cr = (*cr).clone();
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), aliasvars.clone())?;
                    (cr, negate) = BackendVariable::getAlias(var.clone())?;
                    e = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
                    e = if (negate.clone()) {Expression::negate(e.clone())?} else {e.clone()};
                    (e, _) = ExpressionSimplify::simplify(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (idn.clone()).clone() }), expLst: list![e.clone()], attr: attr.clone() }))?;
                    (e, _) = Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(fnptr!(traverserExpsimplifyTimeIndepFuncCalls, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))> + 'static>), (globalKnownVars.clone(), aliasvars.clone(), false))?;
                    Ok((e.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, tail: Deref @ metamodelica::List::Nil }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("change") || idn.clone() == literal!("edge"))) { bail!("guard") }
                    let mut zero: Arc<DAE::Exp>;
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), globalKnownVars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    zero = Expression::arrayFill(Expression::arrayDimension(tp.clone()), Arc::new(DAE::Exp::BCONST { bool: false }))?;
                    Ok((zero.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, tail: Deref @ metamodelica::List::Nil }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("change") || idn.clone() == literal!("edge"))) { bail!("guard") }
                    let mut zero: Arc<DAE::Exp>;
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), aliasvars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    zero = Expression::arrayFill(Expression::arrayDimension(tp.clone()), Arc::new(DAE::Exp::BCONST { bool: false }))?;
                    Ok((zero.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("change") || idn.clone() == literal!("edge"))) { bail!("guard") }
                    Ok((Arc::new(DAE::Exp::BCONST { bool: false }), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: idn }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, tail: Deref @ metamodelica::List::Nil }, attr }, (globalKnownVars, aliasvars, _)) => {
                    if !((idn.clone() == literal!("change") || idn.clone() == literal!("edge"))) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    let mut var: BackendDAE::Var;
                    let mut negate: bool;
                    let mut cr = (*cr).clone();
                    (var, _) = BackendVariable::getVarSingle(cr.clone(), aliasvars.clone())?;
                    (cr, negate) = BackendVariable::getAlias(var.clone())?;
                    e = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: tp.clone() });
                    e = if (negate.clone()) {Expression::negate(e.clone())?} else {e.clone()};
                    (e, _) = ExpressionSimplify::simplify(Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (idn.clone()).clone() }), expLst: list![e.clone()], attr: attr.clone() }))?;
                    (e, _) = Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(fnptr!(traverserExpsimplifyTimeIndepFuncCalls, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))> + 'static>), (globalKnownVars.clone(), aliasvars.clone(), false))?;
                    Ok((e.clone(), (globalKnownVars.clone(), aliasvars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTpl)
}

fn simplifyTimeIndepFuncCallsShared(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut shared: Arc<BackendDAE::Shared>;
    shared = inDAE.shared.clone();
    BackendDAEUtil::traverseBackendDAEExpsVarsWithUpdate(shared.globalKnownVars.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(traverserExpsimplifyTimeIndepFuncCalls, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))> + 'static>), (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false)))?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(shared.initialEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(traverserExpsimplifyTimeIndepFuncCalls, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))> + 'static>), (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false)))?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(shared.removedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(traverserExpsimplifyTimeIndepFuncCalls, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))> + 'static>), (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false)))?;
    let (__asg0_0, _) = traverseEventInfoExps(shared.eventInfo.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(traverserExpsimplifyTimeIndepFuncCalls, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables, bool))> + 'static>), (shared.globalKnownVars.clone(), shared.aliasVars.clone(), false)))?;
    assign_field!(shared.eventInfo = __asg0_0.clone());
    outDAE = Arc::new(BackendDAE::BackendDAE { eqs: inDAE.eqs.clone(), shared: shared });
    Ok(outDAE)
}

fn traverseEventInfoExps<T: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut eventInfo: BackendDAE::EventInfo, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>, mut arg: T) -> Result<(BackendDAE::EventInfo, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>;

    let mut eventInfo: BackendDAE::EventInfo = eventInfo;
    let mut arg: T = arg;
    arg = DoubleEnded::mapFoldNoCopy(eventInfo.zeroCrossings.zc.clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| traverseZeroCrossingExps(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::ZeroCrossing, _) -> Result<_> + 'static>), arg)?;
    arg = DoubleEnded::mapFoldNoCopy(eventInfo.samples.zc.clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| traverseZeroCrossingExps(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::ZeroCrossing, _) -> Result<_> + 'static>), arg)?;
    arg = DoubleEnded::mapFoldNoCopy(eventInfo.relations.clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| traverseZeroCrossingExps(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::ZeroCrossing, _) -> Result<_> + 'static>), arg)?;
    Ok((eventInfo, arg))
}

fn traverseZeroCrossingExps<T: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut zc: BackendDAE::ZeroCrossing, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>, mut arg: T) -> Result<(BackendDAE::ZeroCrossing, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>;

    let mut zc: BackendDAE::ZeroCrossing = zc;
    let mut arg: T = arg;
    let mut relation: Arc<DAE::Exp>;
    (relation, arg) = Expression::traverseExpBottomUp(zc.relation_.clone(), func.clone(), arg)?;
    if !(referenceEq(&*(relation.clone()),&*(zc.relation_.clone()))) {
        zc.relation_ = relation;
    }
    Ok((zc, arg))
}

fn toplevelInputOrUnfixed(mut inVar: BackendDAE::Var) -> bool {
    let mut b: bool;
    b = BackendVariable::isVarOnTopLevelAndInput(inVar.clone()) || BackendVariable::isParam(inVar.clone()) && !(BackendVariable::varFixed(inVar));
    b
}

fn traversingTimeEqnsFinder(mut inExp: Arc<DAE::Exp>, mut inTpl: (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool)) -> (Arc<DAE::Exp>, bool, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp, inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", subscriptLst: Deref @ metamodelica::List::Nil, .. }, ty: _ }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (_, vars, globalKnownVars, b1, b2)) => {
                    let mut vlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), globalKnownVars.clone())?) {
                        (__pa0, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    vlst = __pa0.clone();
                    let false = (List::none(vlst.clone(), (std::sync::Arc::new(fnptr!(toplevelInputOrUnfixed, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, .. }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, (_, vars, globalKnownVars, b1, b2)) => {
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (_, vars, globalKnownVars, true, b2)) => {
                    let mut var: BackendDAE::Var;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), globalKnownVars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    var = __pa0.clone();
                    let DAE::INPUT { .. } = (BackendVariable::getVarDirection(var.clone())) else { bail!("pattern mismatch") };
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), true, b2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (_, vars, globalKnownVars, b1, true)) => {
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok((e.clone(), false, (true, vars.clone(), globalKnownVars.clone(), b1.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, (b, _, _, _, _)) => {
                    Ok((e.clone(), !(b.clone()), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outTpl)
}

pub(crate) fn countSimpleEquations(mut inDlow: Arc<BackendDAE::BackendDAE>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<i32> {
    let mut outSimpleEqns: i32;
    outSimpleEqns = (::match_deref::match_deref! { match &(inDlow) {
        dlow => {
            let mut n: i32;
            let (_, (_, __pa0)) = AdjacencyMatrix::traverseAdjacencyMatrix(inM.clone(), (std::sync::Arc::new(fnptr!(countSimpleEquationsFinder, Arc<metamodelica::List<i32>>, i32, (Arc<BackendDAE::BackendDAE>, i32))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, (Arc<BackendDAE::BackendDAE>, i32)) -> Result<(Arc<metamodelica::List<i32>>, (Arc<BackendDAE::BackendDAE>, i32))> + 'static>), (dlow.clone(), 0))?;
            n = __pa0.clone();
            n
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSimpleEqns)
}

fn countSimpleEquationsFinder(mut elem: Arc<metamodelica::List<i32>>, mut pos: i32, mut inTpl: (Arc<BackendDAE::BackendDAE>, i32)) -> (Arc<metamodelica::List<i32>>, (Arc<BackendDAE::BackendDAE>, i32)) {
    let mut outList: Arc<metamodelica::List<i32>>;
    let mut outTpl: (Arc<BackendDAE::BackendDAE>, i32);
    (outList, outTpl) = 'mc: {
        let __mc_input = inTpl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dae @ Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::List::Cons { head: syst, tail: Deref @ metamodelica::List::Nil }, shared }, n) => {
                    let mut l: i32;
                    let mut n_1: i32;
                    l = (elem.clone().len() as i32);
                    let true = (intLt(l.clone(), 3)) else { bail!("pattern mismatch") };
                    let true = (intGt(l.clone(), 0)) else { bail!("pattern mismatch") };
                    countsimpleEquation(elem.clone(), l.clone(), pos, syst.clone(), shared.clone())?;
                    n_1 = n.clone() + 1;
                    Ok((metamodelica::nil(), (dae.clone(), n_1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((metamodelica::nil(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outList, outTpl)
}

fn countsimpleEquation(mut elem: Arc<metamodelica::List<i32>>, mut length: i32, mut pos: i32, mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (elem, shared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: i, tail: Deref @ metamodelica::List::Nil }, Deref @ BackendDAE::Shared { backendDAEType: BackendDAE::BackendDAEType::JACOBIAN { .. }, .. }) => {
                    let mut cr: Arc<DAE::ComponentRef>;
                    let mut cre: Arc<DAE::Exp>;
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut var: BackendDAE::Var;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    vars = BackendVariable::daeVars(syst.clone());
                    var = BackendVariable::getVarAt(vars.clone(), intAbs(i.clone()))?;
                    let false = (BackendVariable::isStateorStateDerVar(var.clone())) else { bail!("pattern mismatch") };
                    eqns = BackendEquation::getEqnsFromEqSystem(syst.clone());
                    eqn = BackendEquation::get(eqns.clone(), pos)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    e2 = __pa1.clone();
                    globalKnownVars = BackendVariable::daeGlobalKnownVars(shared.clone());
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(fnptr!(traversingTimeEqnsFinder, Arc<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool)) -> Result<(Arc<DAE::Exp>, bool, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool))> + 'static>), (false, vars.clone(), globalKnownVars.clone(), true, false))?) {
                        (_, (false, _, _, _, _)) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e2.clone(), (std::sync::Arc::new(fnptr!(traversingTimeEqnsFinder, Arc<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool)) -> Result<(Arc<DAE::Exp>, bool, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool))> + 'static>), (false, vars.clone(), globalKnownVars.clone(), true, false))?) {
                        (_, (false, _, _, _, _)) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr = BackendVariable::varCref(var.clone())?;
                    cre = Expression::crefExp(cr.clone())?;
                    ::match_deref::match_deref! { match &(ExpressionSolve::solve(e1.clone(), e2.clone(), cre.clone(), None)?) {
                        (_, Deref @ metamodelica::List::Nil) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: i, tail: Deref @ metamodelica::List::Nil }, _) => {
                    let mut cr: Arc<DAE::ComponentRef>;
                    let mut cre: Arc<DAE::Exp>;
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut var: BackendDAE::Var;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    vars = BackendVariable::daeVars(syst.clone());
                    var = BackendVariable::getVarAt(vars.clone(), intAbs(i.clone()))?;
                    let false = (BackendVariable::isStateorStateDerVar(var.clone())) else { bail!("pattern mismatch") };
                    eqns = BackendEquation::getEqnsFromEqSystem(syst.clone());
                    eqn = BackendEquation::get(eqns.clone(), pos)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    e2 = __pa1.clone();
                    globalKnownVars = BackendVariable::daeGlobalKnownVars(shared.clone());
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(fnptr!(traversingTimeEqnsFinder, Arc<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool)) -> Result<(Arc<DAE::Exp>, bool, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool))> + 'static>), (false, vars.clone(), globalKnownVars.clone(), false, false))?) {
                        (_, (false, _, _, _, _)) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(Expression::traverseExpTopDown(e2.clone(), (std::sync::Arc::new(fnptr!(traversingTimeEqnsFinder, Arc<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool)) -> Result<(Arc<DAE::Exp>, bool, (bool, BackendDAE::Variables, BackendDAE::Variables, bool, bool))> + 'static>), (false, vars.clone(), globalKnownVars.clone(), false, false))?) {
                        (_, (false, _, _, _, _)) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    cr = BackendVariable::varCref(var.clone())?;
                    cre = Expression::crefExp(cr.clone())?;
                    ::match_deref::match_deref! { match &(ExpressionSolve::solve(e1.clone(), e2.clone(), cre.clone(), None)?) {
                        (_, Deref @ metamodelica::List::Nil) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut cr: Arc<DAE::ComponentRef>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    eqns = BackendEquation::getEqnsFromEqSystem(syst.clone());
                    eqn = BackendEquation::get(eqns.clone(), pos)?;
                    (cr, _, _, _, _) = BackendEquation::derivativeEquation(eqn.clone())?;
                    vars = BackendVariable::daeVars(syst.clone());
                    ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, _) => {
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    eqns = BackendEquation::getEqnsFromEqSystem(syst.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::get(eqns.clone(), pos)?) {
                        __pa0 @ Deref @ BackendDAE::Equation::EQUATION { .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqn = __pa0.clone();
                    BackendEquation::aliasEquation(eqn.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

// =============================================================================
// remove parameters stuff
//
// =============================================================================
pub(crate) fn removeParameters(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match &(inDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: systs, shared: shared @ Deref @ BackendDAE::Shared { globalKnownVars, .. } } => {
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut systs = (*systs).clone();
            let mut shared = (*shared).clone();
            let mut globalKnownVars = (*globalKnownVars).clone();
            repl = BackendVarTransform::emptyReplacements();
            (repl, _) = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), (std::sync::Arc::new(fnptr!(removeParametersFinder, BackendDAE::Var, (BackendVarTransform::VariableReplacements, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendVarTransform::VariableReplacements, BackendDAE::Variables)) -> Result<(BackendDAE::Var, (BackendVarTransform::VariableReplacements, BackendDAE::Variables))> + 'static>), (repl, globalKnownVars.clone()))?;
            (globalKnownVars, repl) = replaceFinalVars(1, globalKnownVars.clone(), repl)?;
            (globalKnownVars, repl) = replaceFinalVars(1, globalKnownVars.clone(), repl)?;
            if Flags::isSet(Flags::DUMP_PARAM_REPL.clone())? {
                BackendVarTransform::dumpReplacements(repl.clone())?;
            }
            systs = List::map1(systs.clone(), (std::sync::Arc::new(removeParameterswork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, BackendVarTransform::VariableReplacements) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), repl)?;
            assign_field!(shared.globalKnownVars = globalKnownVars.clone());
            Arc::new(BackendDAE::BackendDAE { eqs: systs.clone(), shared: shared.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDAE)
}

fn removeParameterswork(mut isyst: Arc<BackendDAE::EqSystem>, mut repl: BackendVarTransform::VariableReplacements) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    osyst = (::match_deref::match_deref! { match &(isyst) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. } => {
            let mut lsteqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut syst = (*syst).clone();
            let mut vars = (*vars).clone();
            (vars, _) = replaceFinalVars(1, vars.clone(), repl.clone())?;
            (lsteqns, _) = BackendVarTransform::replaceEquations(BackendEquation::equationList(eqns.clone())?, repl, None)?;
            assign_field!(
                syst.orderedVars = vars.clone(),
                syst.orderedEqs = BackendEquation::listEquation(lsteqns)?,
                syst.m = None,
                syst.mT = None
            );
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(osyst)
}

fn removeParametersFinder(mut inVar: BackendDAE::Var, mut inTpl: (BackendVarTransform::VariableReplacements, BackendDAE::Variables)) -> (BackendDAE::Var, (BackendVarTransform::VariableReplacements, BackendDAE::Variables)) {
    let mut outVar: BackendDAE::Var;
    let mut outTpl: (BackendVarTransform::VariableReplacements, BackendDAE::Variables);
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { varName, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(exp), .. }, (repl, vars)) => {
                    let mut repl_1: BackendVarTransform::VariableReplacements;
                    let mut exp1: Arc<DAE::Exp>;
                    (exp1, _) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(fnptr!(BackendDAEUtil::replaceCrefsWithValues, Arc<DAE::Exp>, (BackendDAE::Variables, Arc<DAE::ComponentRef>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<DAE::ComponentRef>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<DAE::ComponentRef>))> + 'static>), (vars.clone(), varName.clone()))?;
                    repl_1 = BackendVarTransform::addReplacement(repl.clone(), varName.clone(), exp1.clone(), None)?;
                    Ok((v.clone(), (repl_1.clone(), vars.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, outTpl)
}

fn replaceFinalVars(mut inNumRepl: i32, mut inVars: BackendDAE::Variables, mut inRepl: BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Variables, BackendVarTransform::VariableReplacements)> {
    let mut outVars: BackendDAE::Variables;
    let mut outRepl: BackendVarTransform::VariableReplacements;
    (outVars, outRepl) = 'mc: {
        let __mc_input = (inNumRepl, inVars, inRepl);
        if let Ok(__v) = (|| -> Result<_> {
            let (mut numrepl, mut globalKnownVars, mut repl) = __mc_input.clone() else { bail!("nomatch") };
            let true = (intEq(0, numrepl.clone())) else { bail!("pattern mismatch") };
            Ok((globalKnownVars.clone(), repl.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, mut globalKnownVars, mut repl) = __mc_input.clone() else { bail!("nomatch") };
            let mut numrepl: i32;
            let mut globalKnownVars1: BackendDAE::Variables;
            let mut globalKnownVars2: BackendDAE::Variables;
            let mut repl1: BackendVarTransform::VariableReplacements;
            let mut repl2: BackendVarTransform::VariableReplacements;
            let (__pa0, (__pa1, __pa2)) = BackendVariable::traverseBackendDAEVarsWithUpdate(globalKnownVars.clone(), (std::sync::Arc::new(replaceFinalVarTraverser) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendVarTransform::VariableReplacements, i32)) -> Result<(BackendDAE::Var, (BackendVarTransform::VariableReplacements, i32))> + 'static>), (repl.clone(), 0))?;
            globalKnownVars1 = __pa0.clone();
            repl1 = __pa1.clone();
            numrepl = __pa2.clone();
            (globalKnownVars2, repl2) = replaceFinalVars(numrepl.clone(), globalKnownVars1.clone(), repl1.clone())?;
            Ok((globalKnownVars2.clone(), repl2.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVars, outRepl))
}

fn replaceFinalVarTraverser(mut inVar: BackendDAE::Var, mut inTpl: (BackendVarTransform::VariableReplacements, i32)) -> Result<(BackendDAE::Var, (BackendVarTransform::VariableReplacements, i32))> {
    let mut outVar: BackendDAE::Var;
    let mut outTpl: (BackendVarTransform::VariableReplacements, i32);
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Var { bindExp: Some(Deref @ DAE::Exp::CALL { .. }), .. }, _) => {
                    Ok((inVar.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { varName: cr, bindExp: Some(e), values: attr, .. }, (repl, numrepl)) => {
                    let mut v1: BackendDAE::Var;
                    let mut repl_1: BackendVarTransform::VariableReplacements;
                    let mut e1: Arc<DAE::Exp>;
                    let mut attr = (*attr).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(e.clone(), repl.clone(), None)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    v1 = BackendVariable::setBindExp(v.clone(), Some(e1.clone()));
                    repl_1 = addConstExpReplacement(e1.clone(), cr.clone(), repl.clone());
                    (attr, repl_1) = BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), (std::sync::Arc::new(fnptr!(traverseExpVisitorWrapper, Arc<DAE::Exp>, BackendVarTransform::VariableReplacements)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendVarTransform::VariableReplacements) -> Result<(Arc<DAE::Exp>, BackendVarTransform::VariableReplacements)> + 'static>), repl_1.clone())?;
                    v1 = BackendVariable::setVarAttributes(v1.clone(), attr.clone());
                    Ok((v1.clone(), (repl_1.clone(), numrepl.clone() + 1)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { values: attr, .. }, (repl, numrepl)) => {
                    let mut v1: BackendDAE::Var;
                    let mut new_attr: Option<Arc<DAE::VariableAttributes>>;
                    let mut repl = (*repl).clone();
                    (new_attr, repl) = BackendDAEUtil::traverseBackendDAEVarAttr(attr.clone(), (std::sync::Arc::new(fnptr!(traverseExpVisitorWrapper, Arc<DAE::Exp>, BackendVarTransform::VariableReplacements)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendVarTransform::VariableReplacements) -> Result<(Arc<DAE::Exp>, BackendVarTransform::VariableReplacements)> + 'static>), repl.clone())?;
                    v1 = BackendVariable::setVarAttributes(v.clone(), new_attr.clone());
                    Ok((v1.clone(), (repl.clone(), numrepl.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outTpl))
}

fn addConstExpReplacement(mut inExp: Arc<DAE::Exp>, mut cr: Arc<DAE::ComponentRef>, mut inRepl: BackendVarTransform::VariableReplacements) -> BackendVarTransform::VariableReplacements {
    let mut outRepl: BackendVarTransform::VariableReplacements;
    outRepl = 'mc: {
        let __mc_input = inRepl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Expression::isConst(inExp.clone())?) else { bail!("pattern mismatch") };
            Ok(BackendVarTransform::addReplacement(inRepl.clone(), cr.clone(), inExp.clone(), None)?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(inRepl.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outRepl
}

fn traverseExpVisitorWrapper(mut inExp: Arc<DAE::Exp>, mut inRepl: BackendVarTransform::VariableReplacements) -> (Arc<DAE::Exp>, BackendVarTransform::VariableReplacements) {
    let mut exp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut repl: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
    (exp, repl) = (::match_deref::match_deref! { match &((inExp.clone(), inRepl.clone())) {
        (__esc_exp @ Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, __esc_repl) => {
            exp = (*__esc_exp).clone();
            repl = (*__esc_repl).clone();
            (exp, _) = BackendVarTransform::replaceExp(exp.clone(), repl.clone(), None);
            (exp.clone(), repl.clone())
        },
        _ => (inExp, inRepl),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, repl)
}

// =============================================================================
// remove protected parameters stuff
//
// =============================================================================
pub(crate) fn removeProtectedParameters(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match &(inDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: systs, shared: shared @ Deref @ BackendDAE::Shared { globalKnownVars, .. } } => {
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut systs = (*systs).clone();
            let mut shared = (*shared).clone();
            repl = BackendVarTransform::emptyReplacements();
            repl = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), (std::sync::Arc::new(fnptr!(protectedParametersFinder, BackendDAE::Var, BackendVarTransform::VariableReplacements)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> + 'static>), repl)?;
            if Flags::isSet(Flags::DUMP_PP_REPL.clone())? {
                BackendVarTransform::dumpReplacements(repl.clone())?;
            }
            systs = List::map1(systs.clone(), (std::sync::Arc::new(removeProtectedParameterswork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, BackendVarTransform::VariableReplacements) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), repl)?;
            assign_field!(shared.globalKnownVars = globalKnownVars.clone());
            Arc::new(BackendDAE::BackendDAE { eqs: systs.clone(), shared: shared.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDAE)
}

fn removeProtectedParameterswork(mut isyst: Arc<BackendDAE::EqSystem>, mut repl: BackendVarTransform::VariableReplacements) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    osyst = (::match_deref::match_deref! { match &(isyst) {
        syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, .. } => {
            let mut lsteqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut b: bool;
            let mut syst = (*syst).clone();
            lsteqns = BackendEquation::equationList(eqns.clone())?;
            (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns, repl, None)?;
            if b {
                assign_field!(syst.orderedEqs = BackendEquation::listEquation(lsteqns)?);
                syst = BackendDAEUtil::clearEqSyst(syst.clone())?;
            }
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(osyst)
}

fn protectedParametersFinder(mut inVar: BackendDAE::Var, mut inRepl: BackendVarTransform::VariableReplacements) -> (BackendDAE::Var, BackendVarTransform::VariableReplacements) {
    let mut outVar: BackendDAE::Var;
    let mut outRepl: BackendVarTransform::VariableReplacements;
    (outVar, outRepl) = 'mc: {
        let __mc_input = (inVar.clone(), inRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { varName, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(exp), values, .. }, repl) => {
                    let mut repl_1: BackendVarTransform::VariableReplacements;
                    let true = (DAEUtil::getProtectedAttr(values.clone())) else { bail!("pattern mismatch") };
                    repl_1 = BackendVarTransform::addReplacement(repl.clone(), varName.clone(), exp.clone(), None)?;
                    Ok((v.clone(), repl_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inRepl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, outRepl)
}

// =============================================================================
// remove equal function calls equations stuff
//
// =============================================================================
pub(crate) fn removeEqualRHS(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut odae: Arc<BackendDAE::BackendDAE>;
    odae = BackendDAEUtil::mapEqSystem(dae, (std::sync::Arc::new(removeEqualFunctionCallsWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>))?;
    Ok(odae)
}

fn removeEqualFunctionCallsWork(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    (osyst, oshared) = (::match_deref::match_deref! { match &(isyst) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. } => {
            let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut changed: Arc<metamodelica::List<i32>>;
            let mut isInitial: bool;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut syst = (*syst).clone();
            isInitial = BackendDAEUtil::isInitializationDAE(ishared.clone());
            funcs = BackendDAEUtil::getFunctions(ishared.clone())?;
            (syst, m, mT) = BackendDAEUtil::getAdjacencyMatrixfromOption(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(funcs), isInitial)?;
            let (__pa0, (__pa1, _, _, __pa2, _)) = AdjacencyMatrix::traverseAdjacencyMatrix(m.clone(), (std::sync::Arc::new(fnptr!(removeEqualFunctionCallFinder, Arc<metamodelica::List<i32>>, i32, (metamodelica::Array<Arc<metamodelica::List<i32>>>, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, (metamodelica::Array<Arc<metamodelica::List<i32>>>, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, bool)) -> Result<(Arc<metamodelica::List<i32>>, (metamodelica::Array<Arc<metamodelica::List<i32>>>, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, bool))> + 'static>), (mT.clone(), vars.clone(), eqns.clone(), metamodelica::nil(), isInitial))?;
            m = __pa0.clone();
            mT = __pa1.clone();
            changed = __pa2.clone();
            assign_field!(
                syst.m = Some(m.clone()),
                syst.mT = Some(mT.clone()),
                syst.matching = openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING()
            );
            syst = BackendDAEUtil::updateAdjacencyMatrix(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, changed, isInitial)?;
            (syst.clone(), ishared)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((osyst, oshared))
}

fn removeEqualFunctionCallFinder(mut elem: Arc<metamodelica::List<i32>>, mut pos: i32, mut inTpl: (metamodelica::Array<Arc<metamodelica::List<i32>>>, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, bool)) -> (Arc<metamodelica::List<i32>>, (metamodelica::Array<Arc<metamodelica::List<i32>>>, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, bool)) {
    let mut outList: Arc<metamodelica::List<i32>>;
    let mut outTpl: (metamodelica::Array<Arc<metamodelica::List<i32>>>, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, bool);
    (outList, outTpl) = 'mc: {
        let __mc_input = inTpl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (mT, vars, eqns, changed, isInitial) => {
                    let mut eqns1: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut ecr: Arc<DAE::Exp>;
                    let mut expvars: Arc<AvlSetInt::Tree>;
                    let mut controleqns: Arc<metamodelica::List<i32>>;
                    let mut expvars1: Arc<metamodelica::List<i32>>;
                    let mut expvarseqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut changed = (*changed).clone();
                    ::match_deref::match_deref! { match &(elem.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: _ } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendEquation::get(eqns.clone(), pos)?) {
                        Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    e2 = __pa1.clone();
                    (ecr, exp) = functionCallEqn(e1.clone(), e2.clone(), vars.clone())?;
                    expvars = BackendDAEUtil::adjacencyRowExp(exp.clone(), vars.clone(), crate::AvlSetInt::Tree::interned_EMPTY(), None, openmodelica_backend_types::BackendDAE::IndexType::NORMAL, isInitial.clone())?;
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(List::map2(AvlSetInt::listKeys(expvars.clone(), metamodelica::nil()), (std::sync::Arc::new(varEqns) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), pos, mT.clone())?) {
                        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    expvars1 = __pa2.clone();
                    expvarseqns = __pa3.clone();
                    controleqns = getControlEqns(expvars1.clone(), expvarseqns.clone())?;
                    (eqns1, changed) = removeEqualFunctionCall(controleqns.clone(), ecr.clone(), exp.clone(), eqns.clone(), changed.clone());
                    Ok((metamodelica::nil(), (mT.clone(), vars.clone(), eqns1.clone(), changed.clone(), isInitial.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((metamodelica::nil(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outList, outTpl)
}

fn functionCallEqn(mut ie1: Arc<DAE::Exp>, mut ie2: Arc<DAE::Exp>, mut inVars: BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>)> {
    let mut outECr: Arc<DAE::Exp>;
    let mut outExp: Arc<DAE::Exp>;
    (outECr, outExp) = (::match_deref::match_deref! { match &((ie1, ie2)) {
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
            bail!("fail")
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::CREF { .. }) => {
            bail!("fail")
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::CREF { .. }) => {
            bail!("fail")
        },
        (e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS { .. }, exp: e2 }) => {
            ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), inVars)?) {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (),
                _ => bail!("pattern mismatch"),
            } };
            (Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e1.clone() }), e2.clone())
        },
        (e1 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, e2) => {
            ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), inVars)?) {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (),
                _ => bail!("pattern mismatch"),
            } };
            (e1.clone(), e2.clone())
        },
        (Deref @ DAE::Exp::UNARY { operator: op @ DAE::Operator::UMINUS { .. }, exp: e1 }, e2 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
            ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), inVars)?) {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (),
                _ => bail!("pattern mismatch"),
            } };
            (Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e2.clone() }), e1.clone())
        },
        (e1, e2 @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
            ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), inVars)?) {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => (),
                _ => bail!("pattern mismatch"),
            } };
            (e2.clone(), e1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outECr, outExp))
}

fn varEqns(mut v: i32, mut pos: i32, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outVarEqns: Arc<metamodelica::List<i32>>;
    let mut vareqns: Arc<metamodelica::List<i32>>;
    let mut vareqns1: Arc<metamodelica::List<i32>>;
    vareqns = ({let __elt = mT.borrow()[(intAbs(v)-1) as usize].clone(); __elt});
    vareqns1 = List::map(vareqns, Arc::new(fnptr!(intAbs, i32)))?;
    outVarEqns = List::removeOnTrue(intAbs(pos), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), vareqns1)?;
    Ok(outVarEqns)
}

fn getControlEqns(mut inVarsEqn: Arc<metamodelica::List<i32>>, mut inVarsEqns: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<i32>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inVarsEqn, inVarsEqns)) {
        (a, Deref @ metamodelica::List::Nil) => {
            return Ok(a.clone())
        },
        (a, Deref @ metamodelica::List::Cons { head: b, tail: rest }) => {
            let mut c: Arc<metamodelica::List<i32>>;
            let mut d: Arc<metamodelica::List<i32>>;
            c = List::intersectionOnTrue(a.clone(), b.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            { (inVarsEqn, inVarsEqns) = (c, rest.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn removeEqualFunctionCall(mut inEqsLst: Arc<metamodelica::List<i32>>, mut inExp: Arc<DAE::Exp>, mut inECr: Arc<DAE::Exp>, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ichanged: Arc<metamodelica::List<i32>>) -> (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>) {
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outEqsLst: Arc<metamodelica::List<i32>>;
    (outEqns, outEqsLst) = 'mc: {
        let __mc_input = inEqsLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((inEqns.clone(), ichanged.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: pos, tail: rest } => {
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut eqn1: Arc<BackendDAE::Equation>;
                    let mut i: i32;
                    let mut changed: Arc<metamodelica::List<i32>>;
                    eqn = BackendEquation::get(inEqns.clone(), pos.clone())?;
                    let (__pa0, (_, _, __pa1)) = BackendDAETransform::traverseBackendDAEExpsEqnWithSymbolicOperation(eqn.clone(), (std::sync::Arc::new(replaceExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, (Arc<DAE::Exp>, Arc<DAE::Exp>, i32))) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, (Arc<DAE::Exp>, Arc<DAE::Exp>, i32)))> + 'static>), (inECr.clone(), inExp.clone(), 0))?;
                    eqn1 = __pa0.clone();
                    i = __pa1.clone();
                    let true = (intGt(i.clone(), 0)) else { bail!("pattern mismatch") };
                    eqns = BackendEquation::setAtIndex(inEqns.clone(), pos.clone(), eqn1.clone())?;
                    changed = List::consOnTrue(!(listMember(pos.clone(), ichanged.clone())), pos.clone(), ichanged.clone());
                    (eqns, changed) = removeEqualFunctionCall(rest.clone(), inExp.clone(), inECr.clone(), eqns.clone(), changed.clone());
                    Ok((eqns.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut changed: Arc<metamodelica::List<i32>>;
                    (eqns, changed) = removeEqualFunctionCall(rest.clone(), inExp.clone(), inECr.clone(), inEqns.clone(), ichanged.clone());
                    Ok((eqns.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outEqns, outEqsLst)
}

fn replaceExp(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, (Arc<DAE::Exp>, Arc<DAE::Exp>, i32))) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, (Arc<DAE::Exp>, Arc<DAE::Exp>, i32)))> {
    let mut e1: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, (Arc<DAE::Exp>, Arc<DAE::Exp>, i32));
    let mut e: Arc<DAE::Exp>;
    let mut se: Arc<DAE::Exp>;
    let mut te: Arc<DAE::Exp>;
    let mut i: i32;
    let mut j: i32;
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
    e = inExp;
    let (__pa0, (__pa1, __pa2, __pa3)) = inTpl;
    ops = __pa0.clone();
    se = __pa1.clone();
    te = __pa2.clone();
    i = __pa3.clone();
    (e1, j) = Expression::replaceExp(e.clone(), se.clone(), te.clone())?;
    ops = if (j > 0) {metamodelica::cons(Arc::new(DAE::SymbolicOperation::SUBSTITUTION { substitutions: list![e1.clone()], source: e }), ops)} else {ops};
    outTpl = (ops, (se, te, i + j));
    Ok((e1, outTpl))
}

// =============================================================================
// remove unused parameter
//
// =============================================================================
pub(crate) fn removeUnusedParameter(mut inDlow: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDlow: Arc<BackendDAE::BackendDAE>;
    outDlow = (::match_deref::match_deref! { match &(inDlow) {
        Deref @ BackendDAE::BackendDAE { eqs, shared } => {
            let mut globalKnownVars: BackendDAE::Variables;
            let mut globalKnownVars1: BackendDAE::Variables;
            let mut shared = (*shared).clone();
            globalKnownVars1 = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
            globalKnownVars = shared.globalKnownVars.clone();
            globalKnownVars1 = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), (std::sync::Arc::new(copyNonParamVariables) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> + 'static>), globalKnownVars1)?;
            (_, globalKnownVars1) = List::fold1(eqs.clone(), (std::sync::Arc::new(BackendDAEUtil::traverseBackendDAEExpsEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(checkUnusedVariables, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), (globalKnownVars.clone(), globalKnownVars1))?;
            (_, globalKnownVars1) = BackendDAEUtil::traverseBackendDAEExpsVars(globalKnownVars.clone(), (std::sync::Arc::new(fnptr!(checkUnusedParameter, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), (globalKnownVars.clone(), globalKnownVars1))?;
            (_, globalKnownVars1) = BackendDAEUtil::traverseBackendDAEExpsVars(shared.aliasVars.clone(), (std::sync::Arc::new(fnptr!(checkUnusedParameter, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), (globalKnownVars.clone(), globalKnownVars1))?;
            (_, globalKnownVars1) = BackendDAEUtil::traverseBackendDAEExpsEqns(shared.removedEqs.clone(), (std::sync::Arc::new(fnptr!(checkUnusedParameter, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), (globalKnownVars.clone(), globalKnownVars1))?;
            (_, globalKnownVars1) = BackendDAEUtil::traverseBackendDAEExpsEqns(shared.initialEqs.clone(), (std::sync::Arc::new(fnptr!(checkUnusedParameter, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), (globalKnownVars, globalKnownVars1))?;
            assign_field!(shared.globalKnownVars = globalKnownVars1);
            Arc::new(BackendDAE::BackendDAE { eqs: eqs.clone(), shared: shared.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDlow)
}

fn copyNonParamVariables(mut inVar: BackendDAE::Var, mut inVars: BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> {
    let mut outVar: BackendDAE::Var;
    let mut outVars: BackendDAE::Variables;
    (outVar, outVars) = (match inVar.clone() {
        mut v @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, .. } => {
            (v.clone(), inVars)
        },
        _ => {
            let mut vars1: BackendDAE::Variables;
            vars1 = BackendVariable::addVar(inVar.clone(), inVars)?;
            (inVar, vars1)
        },
    });
    Ok((outVar, outVars))
}

fn checkUnusedParameter(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, BackendDAE::Variables)) -> (Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables);
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, (vars, _)) => {
                    let mut vars1: BackendDAE::Variables;
                    let (_, (_, __pa0)) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(fnptr!(checkUnusedParameterExp, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), inTpl.clone())?;
                    vars1 = __pa0.clone();
                    Ok((exp.clone(), (vars.clone(), vars1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTpl)
}

fn checkUnusedParameterExp(mut inExp: Arc<DAE::Exp>, mut inTuple: (BackendDAE::Variables, BackendDAE::Variables)) -> (Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (BackendDAE::Variables, BackendDAE::Variables);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, _) => {
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: _ }, .. } }, tp) => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut tp = (*tp).clone();
                    expl = List::map1(varLst.clone(), (std::sync::Arc::new(Expression::generateCrefsExpFromExpVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), cr.clone())?;
                    (_, tp) = Expression::traverseExpList(expl.clone(), (std::sync::Arc::new(fnptr!(checkUnusedParameterExp, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), tp.clone())?;
                    Ok((e.clone(), tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, tp) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut tp = (*tp).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e.clone(), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    (_, tp) = Expression::traverseExpBottomUp(e1.clone(), (std::sync::Arc::new(fnptr!(checkUnusedParameterExp, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), tp.clone())?;
                    Ok((e.clone(), tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. }, .. }, _) => {
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (_, vars1)) => {
                    BackendVariable::getVar(cr.clone(), vars1.clone())?;
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, vars1)) => {
                    let mut var: BackendDAE::Var;
                    let mut vars1 = (*vars1).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    var = __pa0.clone();
                    vars1 = BackendVariable::addVar(var.clone(), vars1.clone())?;
                    Ok((e.clone(), (vars.clone(), vars1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTuple)
}

// =============================================================================
// remove unused variables
//
// =============================================================================
pub(crate) fn removeUnusedVariables(mut inDlow: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDlow: Arc<BackendDAE::BackendDAE>;
    outDlow = (::match_deref::match_deref! { match &(inDlow) {
        Deref @ BackendDAE::BackendDAE { eqs, shared } => {
            let mut globalKnownVars: BackendDAE::Variables;
            let mut globalKnownVars1: BackendDAE::Variables;
            let mut tpl: (BackendDAE::Variables, BackendDAE::Variables);
            let mut shared = (*shared).clone();
            globalKnownVars1 = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
            globalKnownVars = shared.globalKnownVars.clone();
            tpl = List::fold1(eqs.clone(), (std::sync::Arc::new(BackendDAEUtil::traverseBackendDAEExpsEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, _, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(checkUnusedVariables, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), (globalKnownVars.clone(), globalKnownVars1))?;
            tpl = BackendDAEUtil::traverseBackendDAEExpsVars(globalKnownVars, (std::sync::Arc::new(fnptr!(checkUnusedVariables, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), tpl)?;
            tpl = BackendDAEUtil::traverseBackendDAEExpsVars(shared.aliasVars.clone(), (std::sync::Arc::new(fnptr!(checkUnusedVariables, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), tpl)?;
            tpl = BackendDAEUtil::traverseBackendDAEExpsEqns(shared.removedEqs.clone(), (std::sync::Arc::new(fnptr!(checkUnusedVariables, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), tpl)?;
            (_, globalKnownVars1) = BackendDAEUtil::traverseBackendDAEExpsEqns(shared.initialEqs.clone(), (std::sync::Arc::new(fnptr!(checkUnusedVariables, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), tpl)?;
            assign_field!(shared.globalKnownVars = globalKnownVars1);
            Arc::new(BackendDAE::BackendDAE { eqs: eqs.clone(), shared: shared.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDlow)
}

fn checkUnusedVariables(mut inExp: Arc<DAE::Exp>, mut inTpl: (BackendDAE::Variables, BackendDAE::Variables)) -> (Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables);
    (outExp, outTpl) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                exp => {
                    let mut tpl: (BackendDAE::Variables, BackendDAE::Variables);
                    (_, tpl) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(fnptr!(checkUnusedVariablesExp, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), inTpl.clone())?;
                    Ok((exp.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTpl)
}

fn checkUnusedVariablesExp(mut inExp: Arc<DAE::Exp>, mut inTuple: (BackendDAE::Variables, BackendDAE::Variables)) -> (Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (BackendDAE::Variables, BackendDAE::Variables);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, .. }, tp) => {
                    Ok((e.clone(), tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: _ }, .. } }, tp) => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut tp = (*tp).clone();
                    expl = List::map1(varLst.clone(), (std::sync::Arc::new(Expression::generateCrefsExpFromExpVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), cr.clone())?;
                    (_, tp) = Expression::traverseExpList(expl.clone(), (std::sync::Arc::new(fnptr!(checkUnusedVariablesExp, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), tp.clone())?;
                    Ok((e.clone(), tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, tp) => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut tp = (*tp).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e.clone(), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    (_, tp) = Expression::traverseExpBottomUp(e1.clone(), (std::sync::Arc::new(fnptr!(checkUnusedVariablesExp, Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), tp.clone())?;
                    Ok((e.clone(), tp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. }, .. }, _) => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (_, vars1)) => {
                    BackendVariable::getVar(cr.clone(), vars1.clone())?;
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, vars1)) => {
                    let mut var: BackendDAE::Var;
                    let mut vars1 = (*vars1).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cr.clone(), vars.clone())?) {
                        (Deref @ metamodelica::List::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    var = __pa0.clone();
                    vars1 = BackendVariable::addVar(var.clone(), vars1.clone())?;
                    Ok((inExp.clone(), (vars.clone(), vars1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTuple)
}

// =============================================================================
// remove unused functions
//
// =============================================================================
pub(crate) fn removeUnusedFunctions(mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut inShared: Arc<BackendDAE::Shared>, mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inFunctionTree: Arc<AvlTreePathFunction::Tree>, mut inusedFunctions: Arc<AvlTreePathFunction::Tree>) -> Result<Arc<AvlTreePathFunction::Tree>> {
    type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>;

    let mut outFunctionTree: Arc<AvlTreePathFunction::Tree>;
    let mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>;
    let mut funcs: Arc<AvlTreePathFunction::Tree>;
    let mut usedfuncs: Arc<AvlTreePathFunction::Tree>;
    funcs = inFunctionTree;
    usedfuncs = inusedFunctions;
    func = (std::sync::Arc::new({ let __pe_b1 = funcs.clone(); move |__pe_a0, __pe_a2| checkUnusedFunctions(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>);
    usedfuncs = List::fold1(inEqs.clone(), (std::sync::Arc::new(BackendDAEUtil::traverseBackendDAEExpsEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, _, _) -> Result<_> + 'static>), func.clone(), usedfuncs)?;
    usedfuncs = List::fold1(inEqs, std::sync::Arc::new(fnptr!(BackendDAEUtil::traverseBackendDAEExpsEqSystemJacobians, Arc<BackendDAE::EqSystem>, _, _)), func.clone(), usedfuncs)?;
    usedfuncs = List::fold1(inEquationLst, (std::sync::Arc::new(BackendEquation::traverseExpsOfEquationList_WithoutChange) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, _, _) -> Result<_> + 'static>), func.clone(), usedfuncs)?;
    usedfuncs = BackendDAEUtil::traverseBackendDAEExpsVars(inShared.globalKnownVars.clone(), func.clone(), usedfuncs)?;
    usedfuncs = BackendDAEUtil::traverseBackendDAEExpsVars(inShared.externalObjects.clone(), func.clone(), usedfuncs)?;
    usedfuncs = BackendDAEUtil::traverseBackendDAEExpsVars(inShared.aliasVars.clone(), func.clone(), usedfuncs)?;
    usedfuncs = BackendDAEUtil::traverseBackendDAEExpsEqns(inShared.removedEqs.clone(), func.clone(), usedfuncs)?;
    usedfuncs = BackendDAEUtil::traverseBackendDAEExpsEqns(inShared.initialEqs.clone(), func.clone(), usedfuncs)?;
    usedfuncs = removeUnusedFunctionsSymJacs(inShared, funcs, usedfuncs)?;
    outFunctionTree = usedfuncs;
    Ok(outFunctionTree)
}

pub(crate) fn copyRecordConstructorAndExternalObjConstructorDestructor(mut inAllFunctionTree: Arc<AvlTreePathFunction::Tree>) -> Result<Arc<AvlTreePathFunction::Tree>> {
    let mut outUsedFunctionTree: Arc<AvlTreePathFunction::Tree>;
    let mut allfuncs_list: Arc<metamodelica::List<DAE::Function>>;
    outUsedFunctionTree = openmodelica_frontend_dump::AvlTreePathFunction::Tree::interned_EMPTY();
    allfuncs_list = DAEUtil::getFunctionList(inAllFunctionTree.clone(), false)?;
    for mut func in &*allfuncs_list {
        let mut func = func.clone();
        let () = (match func.clone() {
        DAE::Function::RECORD_CONSTRUCTOR { path: mut path, .. } => {
            let mut var_list: Arc<metamodelica::List<Arc<DAE::Var>>>;
            let mut obind: Option<Arc<DAE::Exp>>;
            let mut bind_exp: Arc<DAE::Exp>;
            outUsedFunctionTree = AvlTreePathFunction::add(outUsedFunctionTree.clone(), path.clone(), Some(func.clone()), (std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
            match '__try0: {
                let __pa1 = ::match_deref::match_deref! { match &(var_field!(func.type_, DAE::Function::RECORD_CONSTRUCTOR).clone()) {
                    Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_COMPLEX { varLst: __pa1, .. }, .. } => __pa1.clone(),
                    _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                var_list = __pa1.clone();
                Ok::<_, anyhow::Error>((var_list.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    var_list = __try0_o0;
                }
                Err(__try0_err) => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAEOptimize.copyRecordConstructorAndExternalObjConstructorDestructor")); __mm_s.push_str(&*literal!(" got unxpected record constructor structure for  ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone()], metamodelica::sourceInfo!("BackEnd/BackendDAEOptimize.mo"))?;
                    return Err(__try0_err);
                }
            }
            for mut var in &*var_list.clone() {
                let mut var = var.clone();
                obind = Types::getBindingExpOptional(var.clone());
                if isSome(obind.clone()) {
                    let __pa3 = ::match_deref::match_deref! { match &(obind.clone()) {
                        Some(__pa3) => __pa3.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    bind_exp = __pa3.clone();
                    (_, outUsedFunctionTree) = checkUnusedFunctions(bind_exp.clone(), inAllFunctionTree.clone(), outUsedFunctionTree.clone())?;
                }
            }
            ()
        },
        DAE::Function::FUNCTION { path: mut path, .. } => {
            if stringEq((AbsynUtil::pathLastIdent(path.clone())?).clone(), (literal!("constructor")).clone()) || stringEq((AbsynUtil::pathLastIdent(path.clone())?).clone(), (literal!("destructor")).clone()) {
                outUsedFunctionTree = AvlTreePathFunction::add(outUsedFunctionTree.clone(), path.clone(), Some(func.clone()), (std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
            }
            ()
        },
    });
    }
    Ok(outUsedFunctionTree)
}

fn removeUnusedFunctionsSymJacs(mut inShared: Arc<BackendDAE::Shared>, mut inFunctions: Arc<AvlTreePathFunction::Tree>, mut inUsedFunctions: Arc<AvlTreePathFunction::Tree>) -> Result<Arc<AvlTreePathFunction::Tree>> {
    let mut outUsedFunctions: Arc<AvlTreePathFunction::Tree> = inUsedFunctions.clone();
    let mut bdae: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut usedfuncs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    for mut sjac in &*inShared.symjacs.clone() {
        let mut sjac = sjac.clone();
        let () = (::match_deref::match_deref! { match &(sjac.clone()) {
        (Some((__esc_bdae, _, _, _, _, _)), _, _, _) => {
            bdae = (*__esc_bdae).clone();
            bdae = BackendDAEUtil::setFunctionTree(bdae.clone(), inFunctions.clone())?;
            shared = bdae.shared.clone();
            usedfuncs = removeUnusedFunctions(bdae.eqs.clone(), shared.clone(), metamodelica::nil(), shared.functionTree.clone(), inUsedFunctions.clone())?;
            outUsedFunctions = AvlTreePathFunction::join(outUsedFunctions.clone(), usedfuncs.clone(), (std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    let () = (::match_deref::match_deref! { match &(inShared.dataReconciliationData.clone()) {
        None => (),
        Some(BackendDAE::DataReconciliationData { symbolicJacobian: Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((__esc_bdae, _, _, _, _, _)), .. }, .. }) => {
            bdae = (*__esc_bdae).clone();
            bdae = BackendDAEUtil::setFunctionTree(bdae.clone(), inFunctions)?;
            shared = bdae.shared.clone();
            usedfuncs = removeUnusedFunctions(bdae.eqs.clone(), shared.clone(), metamodelica::nil(), shared.functionTree.clone(), inUsedFunctions)?;
            outUsedFunctions = AvlTreePathFunction::join(outUsedFunctions, usedfuncs, (std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
            ()
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAEOptimize.removeUnusedFunctionsSymJacs")); __mm_s.push_str(&*literal!(": Unexpected data reconciliation jacobian structure. ")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/BackendDAEOptimize.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outUsedFunctions)
}

fn checkUnusedFunctions(mut inExp: Arc<DAE::Exp>, mut inFunctions: Arc<AvlTreePathFunction::Tree>, mut inUsedFunctions: Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outUsedFunctions: Arc<AvlTreePathFunction::Tree>;
    (outExp, outUsedFunctions) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new({ let __pe_b1 = inFunctions; move |__pe_a0, __pe_a2| checkUnusedFunctionsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>), inUsedFunctions)?;
    Ok((outExp, outUsedFunctions))
}

fn checkUnusedFunctionsExp(mut inExp: Arc<DAE::Exp>, mut inFunctions: Arc<AvlTreePathFunction::Tree>, mut inUsedFunctions: Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outUsedFunctions: Arc<AvlTreePathFunction::Tree>;
    outUsedFunctions = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CALL { path, .. } => {
            addUnusedFunction(path.clone(), inFunctions, inUsedFunctions)?
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { path, .. } => {
            addUnusedFunction(path.clone(), inFunctions, inUsedFunctions)?
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { functionType: Deref @ DAE::Type::T_FUNCTION { path, .. }, .. }, .. } => {
            addUnusedFunction(path.clone(), inFunctions, inUsedFunctions)?
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut usedfuncs: Arc<AvlTreePathFunction::Tree>;
            (_, usedfuncs) = Expression::traverseExpCrefDims(cr.clone(), (std::sync::Arc::new({ let __pe_b1 = inFunctions; move |__pe_a0, __pe_a2| checkUnusedFunctions(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>), inUsedFunctions)?;
            usedfuncs
        },
        _ => {
            inUsedFunctions
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outUsedFunctions))
}

fn addUnusedFunction(mut inPath: Arc<Absyn::Path>, mut inFunctions: Arc<AvlTreePathFunction::Tree>, mut inUsedFunctions: Arc<AvlTreePathFunction::Tree>) -> Result<Arc<AvlTreePathFunction::Tree>> {
    let mut outUsedFunctions: Arc<AvlTreePathFunction::Tree> = inUsedFunctions.clone();
    let mut f: Option<DAE::Function>;
    let mut body: Arc<metamodelica::List<Arc<DAE::Element>>>;
    if '__try0: {
        unwrap_break_err!(AvlTreePathFunction::get(inUsedFunctions.clone(), inPath.clone()), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        (f, body) = getFunctionAndBody(inPath.clone(), inFunctions.clone());
        if isSome(f.clone()) {
            outUsedFunctions = AvlTreePathFunction::add(outUsedFunctions.clone(), inPath.clone(), f.clone(), (std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
            (_, outUsedFunctions) = DAEUtil::traverseDAEElementList(body.clone(), (std::sync::Arc::new({ let __pe_b1 = inFunctions.clone(); move |__pe_a0, __pe_a2| checkUnusedFunctions(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<DAE::Exp>, Arc<AvlTreePathFunction::Tree>)> + 'static>), outUsedFunctions.clone())?;
        }
    }
    Ok(outUsedFunctions)
}

fn getFunctionAndBody(mut inPath: Arc<Absyn::Path>, mut fns: Arc<AvlTreePathFunction::Tree>) -> (Option<DAE::Function>, Arc<metamodelica::List<Arc<DAE::Element>>>) {
    let mut outFn: Option<DAE::Function>;
    let mut outFnBody: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut r#fn: DAE::Function;
    match '__try0: {
        let (__pa2, __pa1) = ::match_deref::match_deref! { match &(unwrap_break_err!(AvlTreePathFunction::get(fns.clone(), inPath.clone()), '__try0)) {
            __pa2 @ Some(__pa1) => (__pa2.clone(), __pa1.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        r#fn = __pa1.clone();
        outFn = __pa2.clone();
        outFnBody = unwrap_break_err!(DAEUtil::getFunctionElements(r#fn.clone()), '__try0);
        Ok::<_, anyhow::Error>((outFn.clone(), outFnBody.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outFn = __try0_o0;
            outFnBody = __try0_o1;
        }
        Err(_) => {
            outFn = None;
            outFnBody = metamodelica::nil();
        }
    }
    (outFn, outFnBody)
}

// =============================================================================
// parallel back end stuff (TLM)
//
// =============================================================================
pub(crate) fn collapseIndependentBlocks(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut vars: BackendDAE::Variables;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    vars = BackendVariable::emptyVarsSized(((metamodelica::OrderedFloat((({
        let mut __acc: i32 = 0;
        for mut s in (systs.clone()).into_iter().cloned() {
            let __x = BackendVariable::varsSize(s.orderedVars.clone());
            __acc += __x;
        }
        __acc
    })) as f64) * metamodelica::OrderedFloat(1.4_f64)).0.floor() as i32));
    syst = List::fold(systs.reverse(), (std::sync::Arc::new(mergeIndependentBlocks) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), BackendDAEUtil::createEqSystem(vars, BackendEquation::emptyEqns(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns()))?;
    outDAE = Arc::new(BackendDAE::BackendDAE { eqs: list![syst], shared: shared });
    Ok(outDAE)
}

fn mergeIndependentBlocks(mut syst1: Arc<BackendDAE::EqSystem>, mut syst2: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut vars: BackendDAE::Variables;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut removedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>>;
    vars = BackendVariable::addVariables(syst1.orderedVars.clone(), syst2.orderedVars.clone())?;
    eqs = BackendEquation::addList(BackendEquation::equationList(syst1.orderedEqs.clone())?, syst2.orderedEqs.clone())?;
    removedEqs = BackendEquation::addList(BackendEquation::equationList(syst1.removedEqs.clone())?, syst2.removedEqs.clone())?;
    stateSets = listAppend(syst1.stateSets.clone(), syst2.stateSets.clone());
    syst = BackendDAEUtil::createEqSystem(vars, eqs, stateSets, openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, removedEqs);
    Ok(syst)
}

pub(crate) fn partitionIndependentBlocks(mut dlow: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDlow: Arc<BackendDAE::BackendDAE>;
    outDlow = (::match_deref::match_deref! { match &(dlow.clone()) {
        Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::List::Cons { head: syst, tail: Deref @ metamodelica::List::Nil }, shared } => {
            let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
            let mut shared = (*shared).clone();
            (systs, shared) = partitionIndependentBlocksHelper(syst.clone(), shared.clone(), Error::getNumErrorMessages(), false)?;
            Arc::new(BackendDAE::BackendDAE { eqs: systs, shared: shared.clone() })
        },
        _ => {
            let mut syst: Arc<BackendDAE::EqSystem>;
            let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
            let mut shared: Arc<BackendDAE::Shared>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(collapseIndependentBlocks(dlow)?) {
                Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            syst = __pa0.clone();
            shared = __pa1.clone();
            (systs, shared) = partitionIndependentBlocksHelper(syst.clone(), shared, Error::getNumErrorMessages(), false)?;
            Arc::new(BackendDAE::BackendDAE { eqs: systs, shared: shared.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDlow)
}

pub(crate) fn partitionIndependentBlocksHelper(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut numErrorMessages: i32, mut throwNoError: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<BackendDAE::Shared>)> {
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut oshared: Arc<BackendDAE::Shared>;
    (systs, oshared) = 'mc: {
        let __mc_input = (isyst, ishared.clone());
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst, shared) => {
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut rm: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut rmT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut eqPartMap: metamodelica::Array<i32>;
                    let mut varPartMap: metamodelica::Array<i32>;
                    let mut rixs: metamodelica::Array<i32>;
                    let mut vars: metamodelica::Array<bool>;
                    let mut rvars: metamodelica::Array<bool>;
                    let mut b: bool;
                    let mut isInitial: bool;
                    let mut i: i32;
                    let mut funcs: Arc<AvlTreePathFunction::Tree>;
                    let mut syst = (*syst).clone();
                    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = systs.clone();
                    isInitial = BackendDAEUtil::isInitializationDAE(ishared.clone());
                    funcs = BackendDAEUtil::getFunctions(ishared.clone())?;
                    (syst, m, mT) = BackendDAEUtil::getAdjacencyMatrixfromOption(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), isInitial.clone())?;
                    (rm, rmT) = BackendDAEUtil::removedAdjacencyMatrix(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), isInitial.clone())?;
                    eqPartMap = arrayCreate(metamodelica::arrayLength(m.clone()), 0);
                    varPartMap = arrayCreate(metamodelica::arrayLength(mT.clone()), 0);
                    rixs = arrayCreate(metamodelica::arrayLength(rm.clone()), 0);
                    vars = arrayCreate(metamodelica::arrayLength(mT.clone()), false);
                    rvars = arrayCreate(metamodelica::arrayLength(rmT.clone()), false);
                    i = SynchronousFeatures::partitionIndependentBlocks0(m.clone(), mT.clone(), rm.clone(), rmT.clone(), eqPartMap.clone(), varPartMap.clone(), rixs.clone(), vars.clone(), rvars.clone())?;
                    b = i.clone() > 1;
                    systs = if (b.clone()) {(SynchronousFeatures::partitionIndependentBlocksSplitBlocks(i.clone(), syst.clone(), eqPartMap.clone(), rixs.clone(), mT.clone(), rmT.clone(), throwNoError, funcs.clone(), isInitial.clone())?).0} else {list![syst.clone()]};
                    GCExt::free(eqPartMap.clone());
                    GCExt::free(varPartMap.clone());
                    GCExt::free(rixs.clone());
                    Ok(((systs.clone(), shared.clone()), systs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { systs = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::assertion(!(numErrorMessages == Error::getNumErrorMessages()), (literal!("BackendDAEOptimize.partitionIndependentBlocks failed without good error message")).clone(), Absyn::dummyInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((systs, oshared))
}

// =============================================================================
// residual stuff ... for whatever reason
//
// =============================================================================
pub(crate) fn residualForm(mut dlow: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut odlow: Arc<BackendDAE::BackendDAE>;
    odlow = BackendDAEUtil::mapEqSystem1(dlow, (std::sync::Arc::new(residualForm1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, i32, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>), 1)?;
    Ok(odlow)
}

fn residualForm1(mut syst: Arc<BackendDAE::EqSystem>, mut i: i32, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = syst.clone();
    let mut oshared: Arc<BackendDAE::Shared> = shared.clone();
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let __pa0 = ::match_deref::match_deref! { match &(syst) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqs = __pa0.clone();
    BackendEquation::traverseEquationArray_WithUpdate(eqs, (std::sync::Arc::new(fnptr!(residualForm2, Arc<BackendDAE::Equation>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, i32) -> Result<(Arc<BackendDAE::Equation>, i32)> + 'static>), 1)?;
    Ok((osyst, oshared))
}

fn residualForm2(mut inEq: Arc<BackendDAE::Equation>, mut ii: i32) -> (Arc<BackendDAE::Equation>, i32) {
    let mut outEq: Arc<BackendDAE::Equation>;
    let mut oi: i32;
    (outEq, oi) = 'mc: {
        let __mc_input = (inEq.clone(), ii);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr: eqAttr }, i) => {
                    let mut e: Arc<DAE::Exp>;
                    let mut source = (*source).clone();
                    ::match_deref::match_deref! { match &(Expression::r#typeof(e1.clone())?) {
                        Deref @ DAE::Type::T_REAL { .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    let false = (Expression::isZero(e1.clone())? || Expression::isZero(e2.clone())?) else { bail!("pattern mismatch") };
                    e = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e2.clone() });
                    (e, _) = ExpressionSimplify::simplify(e.clone())?;
                    source = ElementSource::addSymbolicTransformation(source.clone(), Arc::new(DAE::SymbolicOperation::OP_RESIDUAL { e1: e1.clone(), e2: e2.clone(), e: e.clone() }))?;
                    Ok((Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), scalar: e.clone(), source: source.clone(), attr: eqAttr.clone() }), i.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inEq.clone(), ii))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outEq, oi)
}

// =============================================================================
// countOperations
//
// =============================================================================
pub(crate) fn countOperations(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    if Flags::isSet(Flags::COUNT_OPERATIONS.clone())? {
        (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE, (std::sync::Arc::new(countOperations0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> + 'static>), false)?;
    } else {
        outDAE = inDAE;
    }
    Ok(outDAE)
}

fn countOperations0(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inChanged: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = isyst.clone();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outChanged: bool = inChanged;
    let mut compInfos: Arc<metamodelica::List<Arc<BackendDAE::CompInfo>>>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    compInfos = countOperationstraverseComps(comps, isyst, inShared, metamodelica::nil())?;
    Ok((osyst, outShared, outChanged))
}

pub(crate) fn countOperationstraverseComps(mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut compInfosIn: Arc<metamodelica::List<Arc<BackendDAE::CompInfo>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::CompInfo>>>> {
    let mut compInfosOut: Arc<metamodelica::List<Arc<BackendDAE::CompInfo>>>;
    compInfosOut = 'mc: {
        let __mc_input = (inComps.clone(), ishared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(compInfosIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    eqns = BackendEquation::getEqnsFromEqSystem(isyst.clone());
                    eqn = BackendEquation::get(eqns.clone(), eqIdx.clone())?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    compInfo = Arc::new(BackendDAE::CompInfo::COUNTER { comp: listHead(inComps.clone())?, numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    if Flags::isSet(Flags::COUNT_OPERATIONS.clone())? {
                        BackendDump::dumpCompInfo(compInfo.clone())?;
                    }
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), metamodelica::cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    eqn = BackendEquation::get(BackendEquation::getEqnsFromEqSystem(isyst.clone()), eqIdx.clone())?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    compInfo = Arc::new(BackendDAE::CompInfo::COUNTER { comp: listHead(inComps.clone())?, numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone() + 1, numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    if Flags::isSet(Flags::COUNT_OPERATIONS.clone())? {
                        BackendDump::dumpCompInfo(compInfo.clone())?;
                    }
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), metamodelica::cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqs, jac, jacType: BackendDAE::JacobianType::JAC_LINEAR { .. }, .. }, tail: rest }, _) => {
                    let mut size: i32;
                    let mut density: metamodelica::Real;
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    let mut allOps: Arc<BackendDAE::CompInfo>;
                    BackendDAETransform::getEquationAndSolvedVar(comp.clone(), BackendEquation::getEqnsFromEqSystem(isyst.clone()), BackendVariable::daeVars(isyst.clone()))?;
                    size = (eqs.clone().len() as i32);
                    density = realDiv(intReal(getNumJacEntries(jac.clone())), intReal(size.clone() * size.clone()));
                    allOps = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: 0, numMul: 0, numDiv: 0, numTrig: 0, numRelations: 0, numLog: 0, numOth: 0, funcCalls: 0 });
                    allOps = countOperationsJac(jac.clone(), ishared.clone(), allOps.clone())?;
                    compInfo = Arc::new(BackendDAE::CompInfo::SYSTEM { comp: comp.clone(), allOperations: allOps.clone(), size: size.clone(), density: density.clone() });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), metamodelica::cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jac, .. }, tail: rest }, _) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut size: i32;
                    let mut jacEntries: i32;
                    let mut density: metamodelica::Real;
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    let mut allOps: Arc<BackendDAE::CompInfo>;
                    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    (eqnlst, _, _) = BackendDAETransform::getEquationAndSolvedVar(comp.clone(), BackendEquation::getEqnsFromEqSystem(isyst.clone()), BackendVariable::daeVars(isyst.clone()))?;
                    size = (eqnlst.clone().len() as i32);
                    (numAdd, numMul, numDiv, numTrig, numRel, numLog, numOth, numFuncs) = BackendDAEUtil::traverseBackendDAEExpsEqns(BackendEquation::listEquation(eqnlst.clone())?, (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    allOps = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    jacEntries = getNumJacEntries(jac.clone());
                    if intEq(jacEntries.clone(), -1) {
                        jacEntries = size.clone() * size.clone();
                    }
                    density = realDiv(intReal(jacEntries.clone()), intReal(size.clone() * size.clone()));
                    compInfo = Arc::new(BackendDAE::CompInfo::SYSTEM { comp: comp.clone(), allOperations: allOps.clone(), size: size.clone(), density: density.clone() });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), metamodelica::cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    eqn = BackendEquation::get(BackendEquation::getEqnsFromEqSystem(isyst.clone()), eqIdx.clone())?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    compInfo = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), metamodelica::cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    eqn = BackendEquation::get(BackendEquation::getEqnsFromEqSystem(isyst.clone()), eqIdx.clone())?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    compInfo = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone() + 1, numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), metamodelica::cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    eqn = BackendEquation::get(BackendEquation::getEqnsFromEqSystem(isyst.clone()), eqIdx.clone())?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    compInfo = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone() + 1, numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), metamodelica::cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp @ Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: eqIdx, .. }, tail: rest }, _) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    eqn = BackendEquation::get(BackendEquation::getEqnsFromEqSystem(isyst.clone()), eqIdx.clone())?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    compInfo = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone() + 1, numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), metamodelica::cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: vlst, residualequations: tornEqs, innerEquations, .. }, linear: true, .. }, tail: rest }, Deref @ BackendDAE::Shared { functionTree: funcs, .. }) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut otherEqs: Arc<metamodelica::List<i32>>;
                    let mut comp: Arc<BackendDAE::StrongComponent>;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut vars: BackendDAE::Variables;
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    let mut torn: Arc<BackendDAE::CompInfo>;
                    let mut other: Arc<BackendDAE::CompInfo>;
                    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut vLstLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut vlst = (*vlst).clone();
                    comp = listHead(inComps.clone())?;
                    eqns = BackendEquation::getEqnsFromEqSystem(isyst.clone());
                    vars = BackendVariable::daeVars(isyst.clone());
                    eqnlst = BackendEquation::getList(tornEqs.clone(), eqns.clone())?;
                    varlst = List::map1(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    (explst, _) = BackendDAEUtil::getEqnSysRhs(BackendEquation::listEquation(eqnlst.clone())?, BackendVariable::listVar1(varlst.clone())?, Some(funcs.clone()))?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = Expression::traverseExpList(explst.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    torn = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    (otherEqs, vLstLst, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>))?;
                    vlst = List::flatten(vLstLst.clone())?;
                    eqnlst = BackendEquation::getList(otherEqs.clone(), eqns.clone())?;
                    varlst = List::map1(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    (explst, _) = BackendDAEUtil::getEqnSysRhs(BackendEquation::listEquation(eqnlst.clone())?, BackendVariable::listVar1(varlst.clone())?, Some(funcs.clone()))?;
                    let (_, (__pa8, __pa9, __pa10, __pa11, __pa12, __pa13, __pa14, __pa15)) = Expression::traverseExpList(explst.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa8.clone();
                    numMul = __pa9.clone();
                    numDiv = __pa10.clone();
                    numTrig = __pa11.clone();
                    numRel = __pa12.clone();
                    numLog = __pa13.clone();
                    numOth = __pa14.clone();
                    numFuncs = __pa15.clone();
                    other = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    compInfo = Arc::new(BackendDAE::CompInfo::TORN_ANALYSE { comp: comp.clone(), tornEqs: torn.clone(), otherEqs: other.clone(), tornSize: (tornEqs.clone().len() as i32) });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), metamodelica::cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: tornEqs, innerEquations, .. }, linear: false, .. }, tail: rest }, Deref @ BackendDAE::Shared { .. }) => {
                    let mut numAdd: i32;
                    let mut numMul: i32;
                    let mut numDiv: i32;
                    let mut numTrig: i32;
                    let mut numRel: i32;
                    let mut numOth: i32;
                    let mut numFuncs: i32;
                    let mut numLog: i32;
                    let mut otherEqs: Arc<metamodelica::List<i32>>;
                    let mut comp: Arc<BackendDAE::StrongComponent>;
                    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut compInfo: Arc<BackendDAE::CompInfo>;
                    let mut torn: Arc<BackendDAE::CompInfo>;
                    let mut other: Arc<BackendDAE::CompInfo>;
                    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    comp = listHead(inComps.clone())?;
                    eqns = BackendEquation::getEqnsFromEqSystem(isyst.clone());
                    eqnlst = BackendEquation::getList(tornEqs.clone(), eqns.clone())?;
                    explst = List::map(eqnlst.clone(), (std::sync::Arc::new(BackendEquation::getEquationRHS) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = Expression::traverseExpList(explst.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa0.clone();
                    numMul = __pa1.clone();
                    numDiv = __pa2.clone();
                    numTrig = __pa3.clone();
                    numRel = __pa4.clone();
                    numLog = __pa5.clone();
                    numOth = __pa6.clone();
                    numFuncs = __pa7.clone();
                    torn = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    (otherEqs, _, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>))?;
                    eqnlst = BackendEquation::getList(otherEqs.clone(), eqns.clone())?;
                    explst = List::map(eqnlst.clone(), (std::sync::Arc::new(BackendEquation::getEquationRHS) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    let (_, (__pa8, __pa9, __pa10, __pa11, __pa12, __pa13, __pa14, __pa15)) = Expression::traverseExpList(explst.clone(), (std::sync::Arc::new({ let __pe_b1 = ishared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
                    numAdd = __pa8.clone();
                    numMul = __pa9.clone();
                    numDiv = __pa10.clone();
                    numTrig = __pa11.clone();
                    numRel = __pa12.clone();
                    numLog = __pa13.clone();
                    numOth = __pa14.clone();
                    numFuncs = __pa15.clone();
                    other = Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() });
                    compInfo = Arc::new(BackendDAE::CompInfo::TORN_ANALYSE { comp: comp.clone(), tornEqs: torn.clone(), otherEqs: other.clone(), tornSize: (tornEqs.clone().len() as i32) });
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), metamodelica::cons(compInfo.clone(), compInfosIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: comp, tail: rest }, _) => {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("not supported component: ")); __mm_s.push_str(&*BackendDump::strongComponentString(comp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(countOperationstraverseComps(rest.clone(), isyst.clone(), ishared.clone(), compInfosIn.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(compInfosOut)
}

fn getNumJacEntries(mut inJac: Arc<BackendDAE::Jacobian>) -> i32 {
    let mut numEntries: i32;
    numEntries = (::match_deref::match_deref! { match &(inJac) {
        Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: None } => {
            -1
        },
        Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) } => {
            (jac.clone().len() as i32)
        },
        Deref @ BackendDAE::Jacobian::EMPTY_JACOBIAN { .. } => {
            -1
        },
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: None, .. } => {
            -1
        },
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some((_, _, vars1, vars2, _, _)), .. } if ((vars1.clone().len() as i32) == (vars2.clone().len() as i32)) => {
            (vars1.clone().len() as i32)
        },
        _ => {
            metamodelica::print((literal!("another JAC\n")).clone());
            -1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    numEntries
}

fn countOperationsJac(mut inJac: Arc<BackendDAE::Jacobian>, mut shared: Arc<BackendDAE::Shared>, mut compInfoIn: Arc<BackendDAE::CompInfo>) -> Result<Arc<BackendDAE::CompInfo>> {
    let mut compInfoOut: Arc<BackendDAE::CompInfo>;
    compInfoOut = (::match_deref::match_deref! { match &((inJac, compInfoIn.clone())) {
        (Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: None }, _) => {
            compInfoIn
        },
        (Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, Deref @ BackendDAE::CompInfo::COUNTER { comp, numAdds: numAdd, numMul, numDiv, numTrig, numRelations: numRel, numLog, numOth, funcCalls: numFuncs }) => {
            let mut numAdd = (*numAdd).clone();
            let mut numMul = (*numMul).clone();
            let mut numDiv = (*numDiv).clone();
            let mut numTrig = (*numTrig).clone();
            let mut numRel = (*numRel).clone();
            let mut numLog = (*numLog).clone();
            let mut numOth = (*numOth).clone();
            let mut numFuncs = (*numFuncs).clone();
            (numAdd, numMul, numDiv, numTrig, numRel, numLog, numOth, numFuncs) = List::fold(jac.clone(), (std::sync::Arc::new({ let __pe_b1 = shared; move |__pe_a0, __pe_a2| countOperationsJac1(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32, Arc<BackendDAE::Equation>), (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(i32, i32, i32, i32, i32, i32, i32, i32)> + 'static>), (numAdd.clone(), numMul.clone(), numDiv.clone(), numOth.clone(), numTrig.clone(), numRel.clone(), numLog.clone(), numFuncs.clone()))?;
            Arc::new(BackendDAE::CompInfo::COUNTER { comp: comp.clone(), numAdds: numAdd.clone(), numMul: numMul.clone(), numDiv: numDiv.clone(), numTrig: numTrig.clone(), numRelations: numRel.clone(), numLog: numLog.clone(), numOth: numOth.clone(), funcCalls: numFuncs.clone() })
        },
        (_, _) => {
            compInfoIn
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(compInfoOut)
}

fn countOperationsJac1(mut inJac: (i32, i32, Arc<BackendDAE::Equation>), mut shared: Arc<BackendDAE::Shared>, mut inTpl: (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(i32, i32, i32, i32, i32, i32, i32, i32)> {
    let mut outTpl: (i32, i32, i32, i32, i32, i32, i32, i32);
    (_, outTpl) = BackendEquation::traverseExpsOfEquation(Util::tuple33(inJac), (std::sync::Arc::new({ let __pe_b1 = shared; move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), inTpl)?;
    Ok(outTpl)
}

pub(crate) fn countOperationsExp(mut inExp: Arc<DAE::Exp>, mut shared: Arc<BackendDAE::Shared>, mut inTpl: (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (i32, i32, i32, i32, i32, i32, i32, i32);
    (outExp, outTpl) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new({ let __pe_b1 = shared; move |__pe_a0, __pe_a2| Ok(traversecountOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), inTpl)?;
    Ok((outExp, outTpl))
}

fn traversecountOperationsExp(mut inExp: Arc<DAE::Exp>, mut shared: Arc<BackendDAE::Shared>, mut inTuple: (i32, i32, i32, i32, i32, i32, i32, i32)) -> (Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (i32, i32, i32, i32, i32, i32, i32, i32);
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), inTuple);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { operator: op, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    tpl = countOperator(op.clone(), inTuple);
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNARY { operator: op, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    tpl = countOperator(op.clone(), inTuple);
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::LBINARY { operator: op, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    tpl = countOperator(op.clone(), inTuple);
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::LUNARY { operator: op, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    tpl = countOperator(op.clone(), inTuple);
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RELATION { operator: op, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    tpl = countOperator(op.clone(), inTuple);
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: exp1, expElse: exp2 }, _) => {
                    let mut i1: i32;
                    let mut i2: i32;
                    let mut i3: i32;
                    let mut i4: i32;
                    let mut i5: i32;
                    let mut i6: i32;
                    let mut i7: i32;
                    let mut i8: i32;
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = traversecountOperationsExp(exp1.clone(), shared.clone(), inTuple);
                    (_, tpl) = traversecountOperationsExp(exp2.clone(), shared.clone(), tpl.clone());
                    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = traversecountOperationsExp(cond.clone(), shared.clone(), tpl.clone());
                    i1 = __pa0.clone();
                    i2 = __pa1.clone();
                    i3 = __pa2.clone();
                    i4 = __pa3.clone();
                    i5 = __pa4.clone();
                    i6 = __pa5.clone();
                    i7 = __pa6.clone();
                    i8 = __pa7.clone();
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone() + 1, i7.clone(), i8.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RECORD { exps: expLst, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = Expression::traverseExpList(expLst.clone(), (std::sync::Arc::new({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), inTuple)?;
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ARRAY { array: expLst, .. }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = Expression::traverseExpList(expLst.clone(), (std::sync::Arc::new({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), inTuple)?;
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::TUPLE { PR: expLst }, _) => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = Expression::traverseExpList(expLst.clone(), (std::sync::Arc::new({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), inTuple)?;
                    Ok((e.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: opName }, .. }, (i1, i2, i3, i4, i5, i6, i7, i8)) => {
                    if !((stringEq((opName.clone()).clone(), (literal!("sin")).clone()) || stringEq((opName.clone()).clone(), (literal!("cos")).clone()) || stringEq((opName.clone()).clone(), (literal!("tan")).clone()))) { bail!("guard") }
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone() + 1, i5.clone(), i6.clone(), i7.clone(), i8.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, (i1, i2, i3, i4, i5, i6, i7, i8)) => {
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. }, (i1, i2, i3, i4, i5, i6, i7, i8)) => {
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone() + 1, i8.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, (i1, i2, i3, i4, i5, i6, i7, i8)) => {
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone() + 1)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, (i1, i2, i3, i4, i5, i6, i7, i8)) => {
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone() + 1)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path, .. }, _) => {
                    let mut func: DAE::Function;
                    let mut i1: i32;
                    let mut i2: i32;
                    let mut i3: i32;
                    let mut i4: i32;
                    let mut i5: i32;
                    let mut i6: i32;
                    let mut i7: i32;
                    let mut i8: i32;
                    let mut elemLst: Arc<metamodelica::List<Arc<DAE::Element>>>;
                    func = DAEUtil::getNamedFunction(path.clone(), BackendDAEUtil::getFunctions(shared.clone())?)?;
                    elemLst = DAEUtil::getFunctionElements(func.clone())?;
                    (i1, i2, i3, i4, i5, i6, i7, i8) = countOperationsInFunction(elemLst.clone(), shared.clone(), inTuple);
                    Ok((e.clone(), (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone() + 1)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTuple)
}

fn countOperationsInFunction(mut elemLst: Arc<metamodelica::List<Arc<DAE::Element>>>, mut shared: Arc<BackendDAE::Shared>, mut inTpl: (i32, i32, i32, i32, i32, i32, i32, i32)) -> (i32, i32, i32, i32, i32, i32, i32, i32) {
    let mut outTpl: (i32, i32, i32, i32, i32, i32, i32, i32);
    outTpl = 'mc: {
        let __mc_input = elemLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inTpl)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. }, tail: rest } => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = DAEUtil::traverseDAEEquationsStmts(stmts.clone(), (std::sync::Arc::new({ let __pe_b1 = shared.clone(); move |__pe_a0, __pe_a2| Ok(traversecountOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), inTpl)?;
                    Ok(countOperationsInFunction(rest.clone(), shared.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::EQUATION { exp: exp1, scalar: exp2, .. }, tail: rest } => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = traversecountOperationsExp(exp1.clone(), shared.clone(), inTpl);
                    (_, tpl) = traversecountOperationsExp(exp2.clone(), shared.clone(), tpl.clone());
                    Ok(countOperationsInFunction(rest.clone(), shared.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: exp1, rhs: exp2, .. }, tail: rest } => {
                    let mut tpl: (i32, i32, i32, i32, i32, i32, i32, i32);
                    (_, tpl) = traversecountOperationsExp(exp1.clone(), shared.clone(), inTpl);
                    (_, tpl) = traversecountOperationsExp(exp2.clone(), shared.clone(), tpl.clone());
                    Ok(countOperationsInFunction(rest.clone(), shared.clone(), tpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(countOperationsInFunction(rest.clone(), shared.clone(), inTpl))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outTpl
}

fn countOperator(mut op: DAE::Operator, mut inTpl: (i32, i32, i32, i32, i32, i32, i32, i32)) -> (i32, i32, i32, i32, i32, i32, i32, i32) {
    let mut outTpl: (i32, i32, i32, i32, i32, i32, i32, i32);
    outTpl = (match (op, inTpl) {
        (DAE::Operator::ADD { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone() + 1, i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::SUB { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone() + 1, i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::MUL { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone() + 1, i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::DIV { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone() + 1, i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::POW { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone() + 1, i8.clone())
        },
        (DAE::Operator::UMINUS { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone() + 1, i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::UMINUS_ARR { ty: ref tp }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            let mut i: i32;
            i = Expression::sizeOf(tp.clone());
            (i1.clone() + i, i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::ADD_ARR { ty: ref tp }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            let mut i: i32;
            i = Expression::sizeOf(tp.clone());
            (i1.clone() + i, i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::SUB_ARR { ty: ref tp }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            let mut i: i32;
            i = Expression::sizeOf(tp.clone());
            (i1.clone() + i, i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::MUL_ARR { ty: ref tp }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            let mut i: i32;
            i = Expression::sizeOf(tp.clone());
            (i1.clone(), i2.clone() + i, i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::DIV_ARR { ty: ref tp }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            let mut i: i32;
            i = Expression::sizeOf(tp.clone());
            (i1.clone(), i2.clone(), i3.clone() + i, i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::MUL_ARRAY_SCALAR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone() + 1, i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::ADD_ARRAY_SCALAR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone() + 1, i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::SUB_SCALAR_ARRAY { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone() + 1, i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::MUL_SCALAR_PRODUCT { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone() + 1, i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::MUL_MATRIX_PRODUCT { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone() + 1, i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::DIV_ARRAY_SCALAR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone() + 1, i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::DIV_SCALAR_ARRAY { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone() + 1, i4.clone(), i5.clone(), i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::POW_ARRAY_SCALAR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone() + 1, i8.clone())
        },
        (DAE::Operator::POW_SCALAR_ARRAY { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone() + 1, i8.clone())
        },
        (DAE::Operator::POW_ARR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone() + 1, i8.clone())
        },
        (DAE::Operator::POW_ARR2 { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone(), i7.clone() + 1, i8.clone())
        },
        (DAE::Operator::AND { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone() + 1, i7.clone(), i8.clone())
        },
        (DAE::Operator::OR { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone() + 1, i7.clone(), i8.clone())
        },
        (DAE::Operator::NOT { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone() + 1, i7.clone(), i8.clone())
        },
        (DAE::Operator::LESS { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone() + 1, i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::LESSEQ { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone() + 1, i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::GREATER { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone() + 1, i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::GREATEREQ { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone() + 1, i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::EQUAL { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone() + 1, i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::NEQUAL { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone() + 1, i6.clone(), i7.clone(), i8.clone())
        },
        (DAE::Operator::USERDEFINED { .. }, (mut i1, mut i2, mut i3, mut i4, mut i5, mut i6, mut i7, mut i8)) => {
            (i1.clone(), i2.clone(), i3.clone(), i4.clone(), i5.clone(), i6.clone() + 1, i7.clone(), i8.clone())
        },
        _ => {
            metamodelica::print((literal!("not supported operator\n")).clone());
            inTpl
        },
    });
    outTpl
}

// =============================================================================
// simplify if equations
//
// =============================================================================
pub(crate) fn simplifyIfEquations(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut odae: Arc<BackendDAE::BackendDAE>;
    odae = BackendDAEUtil::mapEqSystem(dae, (std::sync::Arc::new(fnptr!(simplifyIfEquationsWork, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>))?;
    Ok(odae)
}

fn simplifyIfEquationsWork(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>) -> (Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    (osyst, oshared) = 'mc: {
        let __mc_input = (isyst.clone(), ishared.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, .. }, shared @ Deref @ BackendDAE::Shared { globalKnownVars, initialEqs, .. }) => {
                    let mut eqnslst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut initial_asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut systChanged: bool;
                    let mut syst = (*syst).clone();
                    let mut shared = (*shared).clone();
                    eqnslst = BackendEquation::equationList(eqns.clone())?;
                    (eqnslst, asserts, systChanged) = List::fold31(eqnslst.clone().reverse(), (std::sync::Arc::new(simplifyIfEquationsFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> + 'static>), globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
                    assign_field!(syst.orderedEqs = BackendEquation::listEquation(eqnslst.clone())?);
                    eqnslst = BackendEquation::equationList(initialEqs.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::fold31(eqnslst.clone().reverse(), (std::sync::Arc::new(simplifyIfEquationsFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> + 'static>), globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), systChanged.clone())?) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqnslst = __pa0.clone();
                    initial_asserts = __pa1.clone();
                    assign_field!(shared.initialEqs = BackendEquation::listEquation(listAppend(initial_asserts.clone(), eqnslst.clone()))?);
                    syst = BackendDAEUtil::clearEqSyst(syst.clone())?;
                    syst = BackendEquation::requationsAddDAE(asserts.clone(), syst.clone())?;
                    Ok((syst.clone(), shared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), ishared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (osyst, oshared)
}

fn simplifyIfEquationsFinder(mut inElem: Arc<BackendDAE::Equation>, mut inConstArg: BackendDAE::Variables, mut acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut b: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> {
    let mut acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = acc;
    let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = asserts;
    let mut b: bool = b;
    (acc, asserts, b) = 'mc: {
        let __mc_input = (inElem, inConstArg);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::IF_EQUATION { conditions: explst, eqnstrue: eqnslstlst, eqnsfalse: eqnslst, source, attr }, globalKnownVars) => {
                    let mut asserts1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut explst = (*explst).clone();
                    let mut acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = acc.clone();
                    (explst, _) = Expression::traverseExpList(explst.clone(), (std::sync::Arc::new(fnptr!(simplifyEvaluatedParamter, Arc<DAE::Exp>, (BackendDAE::Variables, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool))> + 'static>), (globalKnownVars.clone(), false))?;
                    explst = ExpressionSimplify::simplifyList(explst.clone())?;
                    (acc, asserts1) = simplifyIfEquation(explst.clone(), eqnslstlst.clone(), eqnslst.clone(), metamodelica::nil(), metamodelica::nil(), source.clone(), globalKnownVars.clone(), acc.clone(), attr.clone())?;
                    asserts1 = listAppend(asserts.clone(), asserts1.clone());
                    Ok(((acc.clone(), asserts1.clone(), true), acc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { acc = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn, globalKnownVars) => {
                    let mut eqn = (*eqn).clone();
                    let mut b: bool = b.clone();
                    let (__pa0, (_, __pa1)) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new(fnptr!(simplifyIfExpevaluatedParamter, Arc<DAE::Exp>, (BackendDAE::Variables, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool))> + 'static>), (globalKnownVars.clone(), b))?;
                    eqn = __pa0.clone();
                    b = __pa1.clone();
                    Ok(((metamodelica::cons(eqn.clone(), acc.clone()), asserts.clone(), b), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { b = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((acc, asserts, b))
}

fn simplifyIfExpevaluatedParamter(mut inExp: Arc<DAE::Exp>, mut tpl1: (BackendDAE::Variables, bool)) -> (Arc<DAE::Exp>, (BackendDAE::Variables, bool)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut tpl2: (BackendDAE::Variables, bool);
    (outExp, tpl2) = 'mc: {
        let __mc_input = (inExp.clone(), tpl1.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1 @ Deref @ DAE::Exp::IFEXP { expCond: cond, expThen, expElse }, (globalKnownVars, b)) => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut b1: bool;
                    let mut cond = (*cond).clone();
                    let (__pa0, (_, __pa1)) = Expression::traverseExpBottomUp(cond.clone(), (std::sync::Arc::new(fnptr!(simplifyEvaluatedParamter, Arc<DAE::Exp>, (BackendDAE::Variables, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, bool))> + 'static>), (globalKnownVars.clone(), false))?;
                    cond = __pa0.clone();
                    b1 = __pa1.clone();
                    e2 = if (b1.clone()) {Arc::new(DAE::Exp::IFEXP { expCond: cond.clone(), expThen: expThen.clone(), expElse: expElse.clone() })} else {e1.clone()};
                    (e2, _) = ExpressionSimplify::condsimplify(b1.clone(), e2.clone())?;
                    Ok((e2.clone(), (globalKnownVars.clone(), b.clone() || b1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), tpl1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, tpl2)
}

fn simplifyEvaluatedParamter(mut inExp: Arc<DAE::Exp>, mut tpl1: (BackendDAE::Variables, bool)) -> (Arc<DAE::Exp>, (BackendDAE::Variables, bool)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut tpl2: (BackendDAE::Variables, bool);
    (outExp, tpl2) = 'mc: {
        let __mc_input = (inExp.clone(), tpl1.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (globalKnownVars, _)) => {
                    let mut v: BackendDAE::Var;
                    let mut e: Arc<DAE::Exp>;
                    (v, _) = BackendVariable::getVarSingle(cr.clone(), globalKnownVars.clone())?;
                    let true = (BackendVariable::hasVarEvaluateAnnotationTrue(v.clone()) || Flags::getConfigBool(Flags::EVALUATE_FINAL_PARAMS.clone())? && BackendVariable::isFinalVar(v.clone()) || Flags::getConfigBool(Flags::EVALUATE_PROTECTED_PARAMS.clone())? && BackendVariable::isProtectedVar(v.clone())) else { bail!("pattern mismatch") };
                    e = BackendVariable::varBindExpStartValue(v.clone())?;
                    Ok((e.clone(), (globalKnownVars.clone(), true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), tpl1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, tpl2)
}

fn simplifyIfEquation(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut elseenqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut conditions1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut source: Arc<DAE::ElementSource>, mut globalKnownVars: BackendDAE::Variables, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEqAttr: BackendDAE::EquationAttributes) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((conditions, theneqns, conditions1.clone(), theneqns1.clone())) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (eqns, asserts, _) = List::fold31(elseenqs.reverse(), (std::sync::Arc::new(simplifyIfEquationsFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> + 'static>), globalKnownVars, metamodelica::nil(), metamodelica::nil(), false)?;
            return Ok((listAppend(eqns.clone(), inEqns), asserts))
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _, _) => {
            let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut eqnslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut elseenqs1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            explst = conditions1.reverse();
            eqnslst = theneqns1.reverse();
            (elseenqs1, asserts, _) = List::fold31(elseenqs.reverse(), (std::sync::Arc::new(simplifyIfEquationsFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> + 'static>), globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
            elseenqs1 = listAppend(elseenqs1, asserts);
            (eqnslst, elseenqs1, asserts) = simplifyIfEquationAsserts(explst.clone(), eqnslst, elseenqs1, metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
            eqns = simplifyIfEquation1(explst.clone(), eqnslst.clone(), elseenqs1, source, globalKnownVars, inEqns, inEqAttr);
            return Ok((eqns.clone(), asserts))
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: true }, tail: _ }, Deref @ metamodelica::List::Cons { head: eqns, tail: _ }, Deref @ metamodelica::List::Nil, _) => {
            let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut eqns = (*eqns).clone();
            (eqns, asserts, _) = List::fold31(eqns.clone().reverse(), (std::sync::Arc::new(simplifyIfEquationsFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> + 'static>), globalKnownVars, metamodelica::nil(), metamodelica::nil(), false)?;
            return Ok((listAppend(eqns.clone(), inEqns), asserts))
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: true }, tail: _ }, Deref @ metamodelica::List::Cons { head: eqns, tail: _ }, _, _) => {
            let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut eqnslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>;
            let mut elseenqs1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut eqns = (*eqns).clone();
            explst = conditions1.reverse();
            eqnslst = theneqns1.reverse();
            (elseenqs1, asserts, _) = List::fold31(eqns.clone().reverse(), (std::sync::Arc::new(simplifyIfEquationsFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> + 'static>), globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
            elseenqs1 = listAppend(elseenqs1, asserts);
            (eqnslst, elseenqs1, asserts) = simplifyIfEquationAsserts(explst.clone(), eqnslst, elseenqs1, metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
            eqns = simplifyIfEquation1(explst.clone(), eqnslst.clone(), elseenqs1, source, globalKnownVars, inEqns, inEqAttr);
            return Ok((eqns.clone(), asserts))
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: false }, tail: explst }, Deref @ metamodelica::List::Cons { head: _, tail: eqnslst }, _, _) => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            { (conditions, theneqns, elseenqs, conditions1, theneqns1, source, globalKnownVars, inEqns, inEqAttr) = (explst.clone(), eqnslst.clone(), elseenqs, conditions1, theneqns1, source, globalKnownVars, inEqns, inEqAttr); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: explst }, Deref @ metamodelica::List::Cons { head: eqns, tail: eqnslst }, _, _) => {
            let mut asserts: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut eqns = (*eqns).clone();
            (eqns, asserts, _) = List::fold31(eqns.clone().reverse(), (std::sync::Arc::new(simplifyIfEquationsFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, bool)> + 'static>), globalKnownVars.clone(), metamodelica::nil(), metamodelica::nil(), false)?;
            eqns = listAppend(eqns.clone(), asserts);
            { (conditions, theneqns, elseenqs, conditions1, theneqns1, source, globalKnownVars, inEqns, inEqAttr) = (explst.clone(), eqnslst.clone(), elseenqs, metamodelica::cons(e.clone(), conditions1), metamodelica::cons(eqns.clone(), theneqns1), source, globalKnownVars, inEqns, inEqAttr); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn simplifyIfEquation1(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut elseenqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut source: Arc<DAE::ElementSource>, mut globalKnownVars: BackendDAE::Variables, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEqAttr: BackendDAE::EquationAttributes) -> Arc<metamodelica::List<Arc<BackendDAE::Equation>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    outEqns = 'mc: {
        let __mc_input = inEqAttr;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut crexplst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>;
            countEquationsInBranches(theneqns.clone(), elseenqs.clone(), source.clone())?;
            ht = HashTable2::emptyHashTable();
            ht = simplifySolvedIfEqnsElse(elseenqs.clone(), ht.clone())?;
            ht = simplifySolvedIfEqns(conditions.clone().reverse(), theneqns.clone().reverse(), ht.clone())?;
            crexplst = BaseHashTable::hashTableList(ht.clone())?;
            eqns = simplifySolvedIfEqns2(crexplst.clone(), inEqns.clone(), inEqAttr)?;
            Ok(eqns.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut fbsExp: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut tbsExp: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            countEquationsInBranches(theneqns.clone(), elseenqs.clone(), source.clone())?;
            fbsExp = makeEquationLstToResidualExpLst(elseenqs.clone())?;
            tbsExp = List::map(theneqns.clone(), (std::sync::Arc::new(makeEquationLstToResidualExpLst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?;
            eqns = makeEquationsFromResiduals(conditions.clone(), tbsExp.clone(), fbsExp.clone(), source.clone(), inEqAttr)?;
            Ok(listAppend(eqns.clone(), inEqns.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(metamodelica::cons(Arc::new(BackendDAE::Equation::IF_EQUATION { conditions: conditions.clone(), eqnstrue: theneqns.clone(), eqnsfalse: elseenqs.clone(), source: source.clone(), attr: inEqAttr }), inEqns.clone()))
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outEqns
}

fn simplifySolvedIfEqns2(mut crexplst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEqAttr: BackendDAE::EquationAttributes) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(crexplst) {
        Deref @ metamodelica::List::Nil => {
            return Ok(inEqns)
        },
        Deref @ metamodelica::List::Cons { head: (cr, e), tail: rest } => {
            let mut crexp: Arc<DAE::Exp>;
            crexp = Expression::crefExp(cr.clone())?;
            { (crexplst, inEqns, inEqAttr) = (rest.clone(), metamodelica::cons(Arc::new(BackendDAE::Equation::EQUATION { exp: crexp, scalar: e.clone(), source: DAE::emptyElementSource().clone(), attr: inEqAttr }), inEqns), inEqAttr); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn simplifySolvedIfEqns(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((conditions, theneqns)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok(iHt)
        },
        (Deref @ metamodelica::List::Cons { head: c, tail: explst }, Deref @ metamodelica::List::Cons { head: eqns, tail: rest }) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            ht = simplifySolvedIfEqns1(c.clone(), eqns.clone(), iHt, HashSet::emptyHashSet())?;
            { (conditions, theneqns, iHt) = (explst.clone(), rest.clone(), ht); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn simplifySolvedIfEqns1(mut condition: Arc<DAE::Exp>, mut brancheqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>)), mut iHs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(brancheqns) {
        Deref @ metamodelica::List::Nil => {
            return Ok(iHt)
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, scalar: e, .. }, tail: rest } => {
            let mut exp: Arc<DAE::Exp>;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let false = (Expression::expHasCref(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            hs = BaseHashSet::addUnique(cr.clone(), iHs)?;
            exp = BaseHashTable::get(cr.clone(), iHt.clone())?;
            exp = Arc::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e.clone(), expElse: exp });
            ht = BaseHashTable::add((cr.clone(), exp), iHt)?;
            { (condition, brancheqns, iHt, iHs) = (condition, rest.clone(), ht, hs); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, scalar: e, .. }, tail: rest } => {
            let mut exp: Arc<DAE::Exp>;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut e = (*e).clone();
            let false = (Expression::expHasCref(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            hs = BaseHashSet::addUnique(cr.clone(), iHs)?;
            exp = BaseHashTable::get(cr.clone(), iHt.clone())?;
            e = Expression::negate(e.clone())?;
            exp = Arc::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e.clone(), expElse: exp });
            ht = BaseHashTable::add((cr.clone(), exp), iHt)?;
            { (condition, brancheqns, iHt, iHs) = (condition, rest.clone(), ht, hs); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn simplifySolvedIfEqnsElse(mut elseenqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(elseenqs) {
        Deref @ metamodelica::List::Nil => {
            return Ok(iHt.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, scalar: e, .. }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), iHt.clone())?)) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let false = (Expression::expHasCref(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            ht = BaseHashTable::add((cr.clone(), e.clone()), iHt.clone())?;
            { (elseenqs, iHt) = (rest.clone(), ht); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } }, scalar: e, .. }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), iHt.clone())?)) => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
            let mut e = (*e).clone();
            let false = (Expression::expHasCref(e.clone(), cr.clone())?) else { bail!("pattern mismatch") };
            e = Expression::negate(e.clone())?;
            ht = BaseHashTable::add((cr.clone(), e.clone()), iHt.clone())?;
            { (elseenqs, iHt) = (rest.clone(), ht); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn simplifyIfEquationAsserts(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut elseenqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut conditions1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut theneqns1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((conditions, theneqns)) {
        (_, Deref @ metamodelica::List::Nil) => {
            let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            (beqns, eqns) = simplifyIfEquationAsserts1(elseenqs, None, conditions1, metamodelica::nil(), inEqns)?;
            return Ok((theneqns1.reverse(), beqns, eqns.clone()))
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: explst }, Deref @ metamodelica::List::Cons { head: eqns, tail: eqnslst }) => {
            let mut eqns1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut eqnslst1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>;
            let mut eqns = (*eqns).clone();
            (beqns, eqns) = simplifyIfEquationAsserts1(eqns.clone(), Some(e.clone()), conditions1.clone(), metamodelica::nil(), inEqns)?;
            { (conditions, theneqns, elseenqs, conditions1, theneqns1, inEqns) = (explst.clone(), eqnslst.clone(), elseenqs, metamodelica::cons(e.clone(), conditions1), metamodelica::cons(beqns, theneqns1), eqns.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn simplifyIfEquationAsserts1(mut brancheqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut condition: Option<Arc<DAE::Exp>>, mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut brancheqns1: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((brancheqns, condition.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((brancheqns1.reverse(), inEqns))
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSERT { cond, msg, level, source: source1 }, tail: Deref @ metamodelica::List::Nil } }, source, expand: crefExpand, attr: eqAttr }, tail: eqns }, None) => {
            let mut e: Arc<DAE::Exp>;
            let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut eqns = (*eqns).clone();
            e = List::fold(conditions.clone(), (std::sync::Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), cond.clone())?;
            { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_ASSERT { cond: e.clone(), msg: msg.clone(), level: level.clone(), source: source1.clone() })] }), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), inEqns)); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_ASSERT { cond, msg, level, source: source1 }, tail: Deref @ metamodelica::List::Nil } }, source, expand: crefExpand, attr: eqAttr }, tail: eqns }, Some(e)) => {
            let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut eqns = (*eqns).clone();
            let mut e = (*e).clone();
            e = Arc::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: cond.clone(), expElse: Arc::new(DAE::Exp::BCONST { bool: true }) });
            e = List::fold(conditions.clone(), (std::sync::Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e.clone())?;
            { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_ASSERT { cond: e.clone(), msg: msg.clone(), level: level.clone(), source: source1.clone() })] }), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), inEqns)); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_TERMINATE { msg, source: source1 }, tail: Deref @ metamodelica::List::Nil } }, source, expand: crefExpand, attr: eqAttr }, tail: eqns }, None) => {
            let mut e: Arc<DAE::Exp>;
            let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut eqns = (*eqns).clone();
            e = List::fold(conditions.clone(), (std::sync::Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), Arc::new(DAE::Exp::BCONST { bool: true }))?;
            { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: list![Arc::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source1.clone() })], else_: openmodelica_frontend_types::DAE::Else::interned_NOELSE(), source: source1.clone() })] }), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), inEqns)); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Statement::STMT_TERMINATE { msg, source: source1 }, tail: Deref @ metamodelica::List::Nil } }, source, expand: crefExpand, attr: eqAttr }, tail: eqns }, Some(e)) => {
            let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut eqns = (*eqns).clone();
            let mut e = (*e).clone();
            e = List::fold(conditions.clone(), (std::sync::Arc::new(fnptr!(makeIfExp, Arc<DAE::Exp>, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e.clone())?;
            { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: list![Arc::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source1.clone() })], else_: openmodelica_frontend_types::DAE::Else::interned_NOELSE(), source: source1.clone() })] }), source: source.clone(), expand: crefExpand.clone(), attr: eqAttr.clone() }), inEqns)); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: eqn, tail: eqns }, _) => {
            let mut beqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut eqns = (*eqns).clone();
            { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, metamodelica::cons(eqn.clone(), brancheqns1), inEqns); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn makeIfExp(mut cond: Arc<DAE::Exp>, mut else_: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut oExp: Arc<DAE::Exp>;
    oExp = Arc::new(DAE::Exp::IFEXP { expCond: cond, expThen: Arc::new(DAE::Exp::BCONST { bool: true }), expElse: else_ });
    oExp
}

fn countEquationsInBranches(mut trueBranches: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut falseBranch: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut source: Arc<DAE::ElementSource>) -> Result<i32> {
    let mut nrOfEquations: i32 = 0;
    nrOfEquations = 'mc: {
        let __mc_input = falseBranch.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut b: Arc<metamodelica::List<bool>>;
                    let mut nrOfEquationsBranches: Arc<metamodelica::List<i32>>;
                    let mut nrOfEquations: i32 = nrOfEquations.clone();
                    nrOfEquations = BackendEquation::equationLstSize(falseBranch.clone())?;
                    nrOfEquationsBranches = List::map(trueBranches.clone(), (std::sync::Arc::new(BackendEquation::equationLstSize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<i32> + 'static>))?;
                    b = List::map1(nrOfEquationsBranches.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), nrOfEquations)?;
                    let true = (List::reduce(b.clone(), (std::sync::Arc::new(fnptr!(boolAnd, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    Ok((nrOfEquations, nrOfEquations.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { nrOfEquations = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Error::addSourceMessage(Error::IF_EQUATION_MISSING_ELSE.clone(), metamodelica::nil(), ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: _ } => {
                    let mut strs: Arc<metamodelica::List<ArcStr>>;
                    let mut r#str: ArcStr;
                    let mut eqstr: ArcStr;
                    let mut nrOfEquationsBranches: Arc<metamodelica::List<i32>>;
                    let mut nrOfEquations: i32 = nrOfEquations.clone();
                    nrOfEquations = BackendEquation::equationLstSize(falseBranch.clone())?;
                    nrOfEquationsBranches = List::map(trueBranches.clone(), (std::sync::Arc::new(BackendEquation::equationLstSize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<i32> + 'static>))?;
                    eqstr = stringDelimitList(List::map(listAppend(trueBranches.clone(), list![falseBranch.clone()]), (std::sync::Arc::new(BackendDump::dumpEqnsStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<ArcStr> + 'static>))?, (literal!("\n")).clone());
                    strs = List::map(nrOfEquationsBranches.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
                    r#str = stringDelimitList(strs.clone(), (literal!(",")).clone());
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(nrOfEquations)); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
                    Error::addSourceMessage(Error::IF_EQUATION_UNBALANCED_2.clone(), list![(r#str.clone()).clone(), (eqstr.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    Ok((bail!("fail"), nrOfEquations.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { nrOfEquations = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(nrOfEquations)
}

fn makeEquationLstToResidualExpLst(mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut oExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    oExpLst = 'mc: {
        let __mc_input = eqLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: eq @ Deref @ BackendDAE::Equation::ALGORITHM { source, .. }, tail: rest } => {
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut r#str: ArcStr;
                    r#str = (BackendDump::equationString(eq.clone())?).clone();
                    r#str = (Util::stringReplaceChar((r#str.clone()).clone(), (literal!("\n")).clone(), (literal!("")).clone())?).clone();
                    Error::addSourceMessage(Error::IF_EQUATION_WARNING.clone(), list![(r#str.clone()).clone()], ElementSource::getElementSourceFileInfo(source.clone()))?;
                    exps = makeEquationLstToResidualExpLst(rest.clone())?;
                    Ok(exps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: eq, tail: rest } => {
                    let mut exps1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut exps2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    exps1 = makeEquationToResidualExpLst(eq.clone())?;
                    exps2 = makeEquationLstToResidualExpLst(rest.clone())?;
                    exps = listAppend(exps1.clone(), exps2.clone());
                    Ok(exps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oExpLst)
}

fn makeEquationToResidualExpLst(mut eq: Arc<BackendDAE::Equation>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut oExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    oExpLst = 'mc: {
        let __mc_input = eq;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::IF_EQUATION { conditions: conds, eqnstrue: tbs, eqnsfalse: fbs, .. } => {
                    let mut fbsExp: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut tbsExp: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
                    fbsExp = makeEquationLstToResidualExpLst(fbs.clone())?;
                    tbsExp = List::map(tbs.clone(), (std::sync::Arc::new(makeEquationLstToResidualExpLst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?;
                    exps = makeResidualIfExpLst(conds.clone(), tbsExp.clone(), fbsExp.clone())?;
                    Ok(exps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                elt => {
                    let mut exp: Arc<DAE::Exp>;
                    exp = makeEquationToResidualExp(elt.clone())?;
                    Ok(list![exp.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oExpLst)
}

fn makeResidualIfExpLst(mut inExp1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inExpLst2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inExpLst3: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    outExpLst = (::match_deref::match_deref! { match &((inExp1, inExpLst2, inExpLst3)) {
        (_, tbs, Deref @ metamodelica::List::Nil) => {
            let true = (List::all(tbs.clone(), std::sync::Arc::new(fnptr!(listEmpty, _)))?) else { bail!("pattern mismatch") };
            metamodelica::nil()
        },
        (conds, tbs, Deref @ metamodelica::List::Cons { head: fb, tail: fbs }) => {
            let mut tbsRest: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
            let mut tbsFirst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut rest_res: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut ifexp: Arc<DAE::Exp>;
            tbsRest = List::map(tbs.clone(), (std::sync::Arc::new(listRest) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
            rest_res = makeResidualIfExpLst(conds.clone(), tbsRest, fbs.clone())?;
            tbsFirst = List::map(tbs.clone(), (std::sync::Arc::new(listHead) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
            ifexp = Expression::makeNestedIf(conds.clone(), tbsFirst, fb.clone())?;
            metamodelica::cons(ifexp, rest_res)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExpLst)
}

pub(crate) fn makeEquationToResidualExp(mut eq: Arc<BackendDAE::Equation>) -> Result<Arc<DAE::Exp>> {
    let mut oExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    oExp = 'mc: {
        let __mc_input = eq.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. } => {
                    let mut ty: Arc<DAE::Type>;
                    let mut oExp: Arc<DAE::Exp> = oExp.clone();
                    ty = Expression::r#typeof(e1.clone())?;
                    let true = (Types::isIntegerOrRealOrSubTypeOfEither(ty.clone())) else { bail!("pattern mismatch") };
                    oExp = Expression::expSub(e1.clone(), e2.clone())?;
                    Ok((oExp.clone(), oExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { oExp = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. } => {
                    let mut oExp: Arc<DAE::Exp> = oExp.clone();
                    oExp = Expression::expSub(e1.clone(), e2.clone())?;
                    Ok((oExp.clone(), oExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { oExp = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr1, exp: e2, .. } => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut oExp: Arc<DAE::Exp> = oExp.clone();
                    e1 = Expression::crefExp(cr1.clone())?;
                    oExp = Expression::expSub(e1.clone(), e2.clone())?;
                    Ok((oExp.clone(), oExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { oExp = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: oExp, .. } => {
                    Ok(oExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: Deref @ DAE::Exp::TUPLE { PR: expl }, right: e2, .. } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut idx: i32;
                    let mut idxs: Arc<metamodelica::List<i32>>;
                    let mut oExp: Arc<DAE::Exp> = oExp.clone();
                    expl1 = metamodelica::nil();
                    idxs = metamodelica::nil();
                    idx = 1;
                    for mut elem in &*expl.clone() {
                        let mut elem = elem.clone();
                        if Expression::isNotWild(elem.clone()) {
                            idxs = metamodelica::cons(idx.clone(), idxs.clone());
                            expl1 = metamodelica::cons(elem.clone(), expl1.clone());
                        }
                        idx = idx.clone() + 1;
                    }
                    let __pa0 = ::match_deref::match_deref! { match &(expl1.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    let __pa2 = ::match_deref::match_deref! { match &(idxs.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    idx = __pa2.clone();
                    oExp = Expression::expSub(e.clone(), Arc::new(DAE::Exp::TSUB { exp: e2.clone(), ix: idx.clone(), ty: Expression::r#typeof(e.clone())? }))?;
                    Ok((oExp.clone(), oExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { oExp = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. } => {
                    let mut oExp: Arc<DAE::Exp> = oExp.clone();
                    oExp = Expression::expSub(e1.clone(), e2.clone())?;
                    Ok((oExp.clone(), oExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { oExp = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAEOptimize.makeEquationToResidualExp failed to transform equation: ")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); __mm_s.push_str(&*literal!(" to residual form!")); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oExp)
}

fn makeEquationsFromResiduals(mut inExp1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inExpLst2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inExpLst3: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inSource: Arc<DAE::ElementSource>, mut inEqAttr: BackendDAE::EquationAttributes) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    outExpLst = (::match_deref::match_deref! { match &(inExpLst3) {
        Deref @ metamodelica::List::Nil => {
            let true = (List::all(inExpLst2, std::sync::Arc::new(fnptr!(listEmpty, _)))?) else { bail!("pattern mismatch") };
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: fb, tail: fbs } => {
            let mut tbsRest: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
            let mut tbsFirst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut ifexp: Arc<DAE::Exp>;
            let mut eq: Arc<BackendDAE::Equation>;
            let mut rest_res: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut zeroExp: Arc<DAE::Exp>;
            let mut size: i32;
            size = Expression::sizeOf(Expression::r#typeof(fb.clone())?);
            tbsRest = List::map(inExpLst2.clone(), (std::sync::Arc::new(listRest) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
            rest_res = makeEquationsFromResiduals(inExp1.clone(), tbsRest, fbs.clone(), inSource.clone(), inEqAttr)?;
            tbsFirst = List::map(inExpLst2, (std::sync::Arc::new(listHead) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
            ifexp = Expression::makeNestedIf(inExp1, tbsFirst, fb.clone())?;
            if size == 1 {
                eq = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), scalar: ifexp, source: inSource, attr: inEqAttr });
            } else {
                zeroExp = Expression::createZeroExpression(Expression::r#typeof(fb.clone())?)?;
                eq = Arc::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size, left: zeroExp, right: ifexp, source: inSource, attr: inEqAttr });
            }
            metamodelica::cons(eq, rest_res)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExpLst)
}

// =============================================================================
// simplify semiLinear calls
//
// =============================================================================
pub(crate) fn simplifysemiLinear(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut odae: Arc<BackendDAE::BackendDAE>;
    odae = BackendDAEUtil::mapEqSystem(dae, (std::sync::Arc::new(fnptr!(simplifysemiLinearWork, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>))?;
    Ok(odae)
}

fn simplifysemiLinearWork(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>) -> (Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    (osyst, oshared) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqns, .. } => {
                    let mut eqnslst: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>;
                    let mut eqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
                    let mut syst = (*syst).clone();
                    let mut eqns = (*eqns).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendEquation::traverseEquationArray_WithUpdate(eqns.clone(), (std::sync::Arc::new(fnptr!(simplifysemiLinearFinder, Arc<BackendDAE::Equation>, (Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, i32, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, i32, bool)) -> Result<(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, i32, bool))> + 'static>), (metamodelica::nil(), 0, false))?) {
                        (__pa0, (__pa1, _, true)) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqns = __pa0.clone();
                    eqnslst = __pa1.clone();
                    eqnsarray = semiLinearSort(eqnslst.clone(), HashTableExpToIndex::emptyHashTable(), 1, arrayCreate(5, metamodelica::nil()))?;
                    eqnsarray = semiLinearSort1(Arc::new(eqnsarray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), 1, arrayCreate(5, metamodelica::nil()))?;
                    eqnslst = Array::fold(eqnsarray.clone(), (std::sync::Arc::new(fnptr!(semiLinearOptimize, Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>> + 'static>), metamodelica::nil())?;
                    assign_field!(syst.orderedEqs = List::fold(eqnslst.clone(), (std::sync::Arc::new(semiLinearReplaceEqns) as std::sync::Arc<dyn ::std::ops::Fn((Arc<BackendDAE::Equation>, i32), Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> + 'static>), eqns.clone())?);
                    Ok((BackendDAEUtil::clearEqSyst(syst.clone())?, ishared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), ishared.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (osyst, oshared)
}

fn semiLinearReplaceEqns(mut iTpl: (Arc<BackendDAE::Equation>, i32), mut iEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut oEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqn: Arc<BackendDAE::Equation>;
    let mut index: i32;
    (eqn, index) = iTpl;
    if Flags::isSet(Flags::SEMILINEAR.clone())? {
        BackendDump::debugStrEqnStr((literal!("Replace with ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
    }
    oEqns = BackendEquation::setAtIndex(iEqns, index + 1, eqn)?;
    Ok(oEqns)
}

fn semiLinearOptimize(mut eqnslst: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, mut iAcc: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>) -> Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>> {
    let mut oAcc: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>;
    oAcc = 'mc: {
        let __mc_input = eqnslst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => {
                    Ok(iAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                    let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                    let mut eqnsarray: metamodelica::Array<(Arc<BackendDAE::Equation>, i32)>;
                    let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    ht = HashTableExpToIndex::emptyHashTable();
                    ht1 = HashTableExpToIndex::emptyHashTable();
                    (ht, ht1) = semiLinearOptimize1(eqnslst.clone(), 1, ht.clone(), ht1.clone())?;
                    explst = List::fold1(BaseHashTable::hashTableKeyList(ht.clone())?, (std::sync::Arc::new(semiLinearGetSA) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), ht1.clone(), metamodelica::nil())?;
                    eqnsarray = metamodelica::arrayFromVec(eqnslst.clone().into_iter().cloned().collect());
                    Ok(semiLinearOptimize2(explst.clone(), ht.clone(), eqnsarray.clone(), iAcc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oAcc
}

fn semiLinearOptimize2(mut saLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), mut IEqnsarray: metamodelica::Array<(Arc<BackendDAE::Equation>, i32)>, mut iAcc: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>) -> Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>> {
    let mut oAcc: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>;
    oAcc = 'mc: {
        let __mc_input = saLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(iAcc.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: sa, tail: rest } => {
                    let mut sb: Arc<DAE::Exp>;
                    let mut s1: Arc<DAE::Exp>;
                    let mut y: Arc<DAE::Exp>;
                    let mut x: Arc<DAE::Exp>;
                    let mut explst: Arc<metamodelica::List<(Arc<DAE::Exp>, i32, Arc<DAE::ElementSource>)>>;
                    let mut acc: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut eqn1: Arc<BackendDAE::Equation>;
                    let mut i1: i32;
                    let mut index: i32;
                    let mut index1: i32;
                    let mut path: Arc<Absyn::Path>;
                    let mut attr: Arc<DAE::CallAttributes>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut source1: Arc<DAE::ElementSource>;
                    let mut eqAttr: BackendDAE::EquationAttributes;
                    i1 = BaseHashTable::get(sa.clone(), iHt.clone())?;
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(({let __elt = IEqnsarray.borrow()[(i1.clone()-1) as usize].clone(); __elt})) {
                        (Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: Deref @ DAE::Exp::CALL { path: __pa1, expLst: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil } } }, attr: __pa4 }, source: __pa5, attr: __pa6 }, __pa7) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    y = __pa0.clone();
                    path = __pa1.clone();
                    x = __pa2.clone();
                    s1 = __pa3.clone();
                    attr = __pa4.clone();
                    source = __pa5.clone();
                    eqAttr = __pa6.clone();
                    index = __pa7.clone();
                    (sb, source1, index1, explst) = semiLinearOptimize3(s1.clone(), source.clone(), index.clone(), iHt.clone(), IEqnsarray.clone(), metamodelica::nil());
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: s1.clone(), scalar: Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("noEvent")).clone() }), expLst: list![Arc::new(DAE::Exp::RELATION { exp1: x.clone(), operator: DAE::Operator::GREATEREQ { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), index: -1, optionExpisASUB: None })], attr: DAE::callAttrBuiltinBool().clone() }), expThen: sa.clone(), expElse: sb.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    eqn1 = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source1.clone(), attr: eqAttr.clone() });
                    acc = semiLinearOptimize4(explst.clone(), metamodelica::cons((eqn1.clone(), index1.clone()), iAcc.clone()), eqAttr.clone())?;
                    Ok(semiLinearOptimize2(rest.clone(), iHt.clone(), IEqnsarray.clone(), metamodelica::cons((eqn.clone(), index.clone()), acc.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(semiLinearOptimize2(rest.clone(), iHt.clone(), IEqnsarray.clone(), iAcc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oAcc
}

fn semiLinearOptimize4(mut explst: Arc<metamodelica::List<(Arc<DAE::Exp>, i32, Arc<DAE::ElementSource>)>>, mut iAcc: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, mut inEqAttr: BackendDAE::EquationAttributes) -> Result<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(explst) {
        Deref @ metamodelica::List::Nil => {
            return Ok(iAcc)
        },
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => {
            return Ok(iAcc)
        },
        Deref @ metamodelica::List::Cons { head: (s2, index, source), tail: rest @ Deref @ metamodelica::List::Cons { head: (s1, _, _), tail: _ } } => {
            let mut eqn: Arc<BackendDAE::Equation>;
            eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: s2.clone(), scalar: s1.clone(), source: source.clone(), attr: inEqAttr });
            { (explst, iAcc, inEqAttr) = (rest.clone(), metamodelica::cons((eqn, index.clone()), iAcc), inEqAttr); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn semiLinearOptimize3(mut exp: Arc<DAE::Exp>, mut isource: Arc<DAE::ElementSource>, mut iIndex: i32, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), mut IEqnsarray: metamodelica::Array<(Arc<BackendDAE::Equation>, i32)>, mut iAcc: Arc<metamodelica::List<(Arc<DAE::Exp>, i32, Arc<DAE::ElementSource>)>>) -> (Arc<DAE::Exp>, Arc<DAE::ElementSource>, i32, Arc<metamodelica::List<(Arc<DAE::Exp>, i32, Arc<DAE::ElementSource>)>>) {
    let mut slast: Arc<DAE::Exp>;
    let mut osource: Arc<DAE::ElementSource>;
    let mut oIndex: i32;
    let mut oAcc: Arc<metamodelica::List<(Arc<DAE::Exp>, i32, Arc<DAE::ElementSource>)>> = metamodelica::nil();
    (slast, osource, oIndex, oAcc) = 'mc: {
        let __mc_input = iAcc.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut sb: Arc<DAE::Exp>;
                    let mut i: i32;
                    let mut index: i32;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut oAcc: Arc<metamodelica::List<(Arc<DAE::Exp>, i32, Arc<DAE::ElementSource>)>> = oAcc.clone();
                    i = BaseHashTable::get(exp.clone(), iHt.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(({let __elt = IEqnsarray.borrow()[(i.clone()-1) as usize].clone(); __elt})) {
                        (Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } } }, .. }, source: __pa1, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    sb = __pa0.clone();
                    source = __pa1.clone();
                    index = __pa2.clone();
                    (sb, source, index, oAcc) = semiLinearOptimize3(sb.clone(), source.clone(), index.clone(), iHt.clone(), IEqnsarray.clone(), metamodelica::cons((exp.clone(), iIndex, source.clone()), iAcc.clone()));
                    Ok(((sb.clone(), source.clone(), index.clone(), oAcc.clone()), oAcc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { oAcc = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((exp.clone(), isource.clone(), iIndex, iAcc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (slast, osource, oIndex, oAcc)
}

fn semiLinearGetSA(mut key: Arc<DAE::Exp>, mut iHt1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), mut iAcc: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut oAcc: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    oAcc = if (BaseHashTable::hasKey(key.clone(), iHt1)?) {iAcc} else {metamodelica::cons(key, iAcc)};
    Ok(oAcc)
}

fn semiLinearOptimize1(mut eqnslst: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, mut i: i32, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), mut iHt1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(eqnslst) {
        Deref @ metamodelica::List::Nil => {
            return Ok((iHt, iHt1))
        },
        Deref @ metamodelica::List::Cons { head: (Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Nil } } }, .. }, .. }, _), tail: rest } => {
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
            let mut ht1: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
            ht = BaseHashTable::add((sa.clone(), i), iHt)?;
            ht1 = BaseHashTable::add((sb.clone(), i), iHt1)?;
            { (eqnslst, i, iHt, iHt1) = (rest.clone(), i + 1, ht, ht1); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn semiLinearSort(mut eqnslst: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), mut size: i32, mut iEqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>> {
    let mut oEqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
    oEqnsarray = 'mc: {
        let __mc_input = eqnslst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(iEqnsarray.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (eqn @ Deref @ BackendDAE::Equation::EQUATION { exp: y, .. }, index), tail: rest } => {
                    let mut i: i32;
                    let mut eqns: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>;
                    let mut eqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
                    i = BaseHashTable::get(y.clone(), iHt.clone())?;
                    eqns = ({let __elt = iEqnsarray.borrow()[(i.clone()-1) as usize].clone(); __elt});
                    eqnsarray = metamodelica::arrayUpdate(iEqnsarray.clone(), i.clone(), metamodelica::cons((eqn.clone(), index.clone()), eqns.clone()))?;
                    Ok(semiLinearSort(rest.clone(), iHt.clone(), size, eqnsarray.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (eqn @ Deref @ BackendDAE::Equation::EQUATION { exp: y, .. }, index), tail: rest } => {
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                    let mut eqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
                    ht = BaseHashTable::add((y.clone(), size), iHt.clone())?;
                    eqnsarray = if (intGt(size, metamodelica::arrayLength(iEqnsarray.clone()))) {Array::expand(5, iEqnsarray.clone(), metamodelica::nil())?} else {iEqnsarray.clone()};
                    eqnsarray = metamodelica::arrayUpdate(eqnsarray.clone(), size, list![(eqn.clone(), index.clone())])?;
                    Ok(semiLinearSort(rest.clone(), ht.clone(), size + 1, eqnsarray.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oEqnsarray)
}

fn semiLinearSort1(mut eqnslstlst: Arc<metamodelica::List<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>>, mut size: i32, mut iEqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(eqnslstlst) {
        Deref @ metamodelica::List::Nil => {
            return Ok(iEqnsarray.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: tpl, tail: Deref @ metamodelica::List::Nil }, tail: rest } => {
            let mut eqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
            eqnsarray = if (intGt(size, metamodelica::arrayLength(iEqnsarray.clone()))) {Array::expand(5, iEqnsarray.clone(), metamodelica::nil())?} else {iEqnsarray.clone()};
            eqnsarray = metamodelica::arrayUpdate(eqnsarray.clone(), size, list![tpl.clone()])?;
            { (eqnslstlst, size, iEqnsarray) = (rest.clone(), size + 1, eqnsarray.clone()); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: eqns, tail: rest } => {
            let mut size1: i32;
            let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
            let mut eqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
            ht = HashTableExpToIndex::emptyHashTable();
            (size1, eqnsarray) = semiLinearSort2(eqns.clone(), ht, size, iEqnsarray.clone())?;
            { (eqnslstlst, size, iEqnsarray) = (rest.clone(), size1, eqnsarray.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn semiLinearSort2(mut eqnslst: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), mut size: i32, mut iEqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>) -> Result<(i32, metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>)> {
    let mut osize: i32;
    let mut oEqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
    (osize, oEqnsarray) = 'mc: {
        let __mc_input = eqnslst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((size, iEqnsarray.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (eqn @ Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: x, tail: _ }, .. }, .. }, index), tail: rest } => {
                    let mut i: i32;
                    let mut eqns: Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>;
                    let mut eqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
                    i = BaseHashTable::get(x.clone(), iHt.clone())?;
                    eqns = ({let __elt = iEqnsarray.borrow()[(i.clone()-1) as usize].clone(); __elt});
                    eqnsarray = metamodelica::arrayUpdate(iEqnsarray.clone(), i.clone(), metamodelica::cons((eqn.clone(), index.clone()), eqns.clone()))?;
                    (i, eqnsarray) = semiLinearSort2(rest.clone(), iHt.clone(), size, eqnsarray.clone())?;
                    Ok((i.clone(), eqnsarray.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (eqn @ Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: x, tail: _ }, .. }, .. }, index), tail: rest } => {
                    let mut i: i32;
                    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr));
                    let mut eqnsarray: metamodelica::Array<Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>>;
                    ht = BaseHashTable::add((x.clone(), size), iHt.clone())?;
                    eqnsarray = if (intGt(size, metamodelica::arrayLength(iEqnsarray.clone()))) {Array::expand(5, iEqnsarray.clone(), metamodelica::nil())?} else {iEqnsarray.clone()};
                    eqnsarray = metamodelica::arrayUpdate(eqnsarray.clone(), size, list![(eqn.clone(), index.clone())])?;
                    (i, eqnsarray) = semiLinearSort2(rest.clone(), ht.clone(), size + 1, eqnsarray.clone())?;
                    Ok((i.clone(), eqnsarray.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osize, oEqnsarray))
}

fn simplifysemiLinearFinder(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, i32, bool)) -> (Arc<BackendDAE::Equation>, (Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, i32, bool)) {
    let mut outEq: Arc<BackendDAE::Equation>;
    let mut outTpl: (Arc<metamodelica::List<(Arc<BackendDAE::Equation>, i32)>>, i32, bool);
    (outEq, outTpl) = 'mc: {
        let __mc_input = (inEq, inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: y, scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::List::Cons { head: x, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Nil } } }, .. }, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let true = (Expression::isZero(y.clone())?) else { bail!("pattern mismatch") };
                    let true = (Expression::isZero(x.clone())?) else { bail!("pattern mismatch") };
                    Ok((Arc::new(BackendDAE::Equation::EQUATION { exp: sa.clone(), scalar: sb.clone(), source: source.clone(), attr: eqAttr.clone() }), (eqnslst.clone(), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::List::Cons { head: x, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Nil } } }, .. }, scalar: y, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let true = (Expression::isZero(y.clone())?) else { bail!("pattern mismatch") };
                    let true = (Expression::isZero(x.clone())?) else { bail!("pattern mismatch") };
                    Ok((Arc::new(BackendDAE::Equation::EQUATION { exp: sa.clone(), scalar: sb.clone(), source: source.clone(), attr: eqAttr.clone() }), (eqnslst.clone(), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: y, scalar: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::List::Cons { head: x, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Nil } } }, .. }, .. }, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let true = (Expression::isZero(y.clone())?) else { bail!("pattern mismatch") };
                    let true = (Expression::isZero(x.clone())?) else { bail!("pattern mismatch") };
                    Ok((Arc::new(BackendDAE::Equation::EQUATION { exp: sa.clone(), scalar: sb.clone(), source: source.clone(), attr: eqAttr.clone() }), (eqnslst.clone(), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::List::Cons { head: x, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Nil } } }, .. }, .. }, scalar: y, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let true = (Expression::isZero(y.clone())?) else { bail!("pattern mismatch") };
                    let true = (Expression::isZero(x.clone())?) else { bail!("pattern mismatch") };
                    Ok((Arc::new(BackendDAE::Equation::EQUATION { exp: sa.clone(), scalar: sb.clone(), source: source.clone(), attr: eqAttr.clone() }), (eqnslst.clone(), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: y, scalar: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Nil } } }, attr }, .. }, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation>;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::UNARY { exp: Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Nil } } }, attr }, .. }, scalar: y, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation>;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::UNARY { exp: y, .. }, scalar: Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Nil } } }, attr }, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation>;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Nil } } }, attr }, scalar: Deref @ DAE::Exp::UNARY { exp: y, .. }, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation>;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: y, scalar: Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Nil } } }, attr }, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut y = (*y).clone();
                    y = Expression::negate(y.clone())?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CALL { path: path @ Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { exp: x, .. }, tail: Deref @ metamodelica::List::Cons { head: sb, tail: Deref @ metamodelica::List::Cons { head: sa, tail: Deref @ metamodelica::List::Nil } } }, attr }, scalar: y, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut y = (*y).clone();
                    y = Expression::negate(y.clone())?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: list![x.clone(), sa.clone(), sb.clone()], attr: attr.clone() }), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, .. }, (eqnslst, index, _)) => {
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn @ Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, .. }, (eqnslst, index, _)) => {
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: y, scalar: Deref @ DAE::Exp::UNARY { exp: x @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, .. }, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut y = (*y).clone();
                    y = Expression::negate(y.clone())?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: x.clone(), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::UNARY { exp: x @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, .. }, .. }, scalar: y, source, attr: eqAttr }, (eqnslst, index, _)) => {
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut y = (*y).clone();
                    y = Expression::negate(y.clone())?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: y.clone(), scalar: x.clone(), source: source.clone(), attr: eqAttr.clone() });
                    if Flags::isSet(Flags::SEMILINEAR.clone())? {
                        BackendDump::debugStrEqnStr((literal!("Found semiLinear ")).clone(), eqn.clone(), (literal!("\n")).clone())?;
                    }
                    Ok((eqn.clone(), (metamodelica::cons((eqn.clone(), index.clone()), eqnslst.clone()), index.clone() + 1, true)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eqn, (eqnslst, index, b)) => {
                    Ok((eqn.clone(), (eqnslst.clone(), index.clone() + 1, b.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outEq, outTpl)
}

// =============================================================================
// remove constants stuff
//
// =============================================================================
pub(crate) fn removeConstants(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match &(inDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: systs, shared: shared @ Deref @ BackendDAE::Shared { globalKnownVars, .. } } => {
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut lsteqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut b: bool;
            let mut systs = (*systs).clone();
            let mut shared = (*shared).clone();
            let mut globalKnownVars = (*globalKnownVars).clone();
            repl = BackendVarTransform::emptyReplacements();
            repl = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), (std::sync::Arc::new(fnptr!(removeConstantsFinder, BackendDAE::Var, BackendVarTransform::VariableReplacements)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendVarTransform::VariableReplacements) -> Result<(BackendDAE::Var, BackendVarTransform::VariableReplacements)> + 'static>), repl)?;
            let (__pa0, (__pa1, _)) = BackendVariable::traverseBackendDAEVarsWithUpdate(globalKnownVars.clone(), (std::sync::Arc::new(replaceFinalVarTraverser) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendVarTransform::VariableReplacements, i32)) -> Result<(BackendDAE::Var, (BackendVarTransform::VariableReplacements, i32))> + 'static>), (repl, 0))?;
            globalKnownVars = __pa0.clone();
            repl = __pa1.clone();
            if Flags::isSet(Flags::DUMP_CONST_REPL.clone())? {
                BackendVarTransform::dumpReplacements(repl.clone())?;
            }
            lsteqns = BackendEquation::equationList(shared.initialEqs.clone())?;
            (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns, repl.clone(), None)?;
            assign_field!(shared.initialEqs = if (b) {BackendEquation::listEquation(lsteqns)?} else {shared.initialEqs.clone()});
            lsteqns = BackendEquation::equationList(shared.removedEqs.clone())?;
            (lsteqns, b) = BackendVarTransform::replaceEquations(lsteqns, repl.clone(), None)?;
            assign_field!(shared.removedEqs = if (b) {BackendEquation::listEquation(lsteqns)?} else {shared.removedEqs.clone()});
            systs = List::map1(systs.clone(), (std::sync::Arc::new(removeConstantsWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, BackendVarTransform::VariableReplacements) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), repl)?;
            Arc::new(BackendDAE::BackendDAE { eqs: systs.clone(), shared: shared.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDAE)
}

fn removeConstantsWork(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut repl: BackendVarTransform::VariableReplacements) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem>;
    outEqSystem = (::match_deref::match_deref! { match &(inEqSystem) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, .. } => {
            let mut lsteqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut b: bool;
            let mut syst = (*syst).clone();
            BackendVariable::traverseBackendDAEVarsWithUpdate(vars.clone(), (std::sync::Arc::new(replaceFinalVarTraverser) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendVarTransform::VariableReplacements, i32)) -> Result<(BackendDAE::Var, (BackendVarTransform::VariableReplacements, i32))> + 'static>), (repl.clone(), 0))?;
            (lsteqns, b) = BackendVarTransform::replaceEquations(BackendEquation::equationList(syst.orderedEqs.clone())?, repl.clone(), None)?;
            if b {
                assign_field!(syst.orderedEqs = BackendEquation::listEquation(lsteqns)?);
                syst = BackendDAEUtil::clearEqSyst(syst.clone())?;
            }
            (lsteqns, b) = BackendVarTransform::replaceEquations(BackendEquation::equationList(syst.removedEqs.clone())?, repl, None)?;
            if b {
                assign_field!(syst.removedEqs = BackendEquation::listEquation(lsteqns)?);
            }
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqSystem)
}

fn removeConstantsFinder(mut inVar: BackendDAE::Var, mut inRepl: BackendVarTransform::VariableReplacements) -> (BackendDAE::Var, BackendVarTransform::VariableReplacements) {
    let mut outVar: BackendDAE::Var;
    let mut outRepl: BackendVarTransform::VariableReplacements;
    (outVar, outRepl) = 'mc: {
        let __mc_input = (inVar.clone(), inRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ BackendDAE::Var { varName, varKind: BackendDAE::VarKind::CONST { .. }, bindExp: Some(exp), .. }, repl) => {
                    let mut repl_1: BackendVarTransform::VariableReplacements;
                    repl_1 = BackendVarTransform::addReplacement(repl.clone(), varName.clone(), exp.clone(), None)?;
                    Ok((v.clone(), repl_1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inRepl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, outRepl)
}

// =============================================================================
// reaplace edge and change with (b and not pre(b)) and (v <> pre(v))
//
// =============================================================================
pub(crate) fn replaceEdgeChange(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE, (std::sync::Arc::new(fnptr!(replaceEdgeChange0, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> + 'static>), false)?;
    outDAE = replaceEdgeChangeShared(outDAE)?;
    Ok(outDAE)
}

fn replaceEdgeChange0(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inChanged: bool) -> (Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool) {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outChanged: bool;
    (osyst, outChanged) = 'mc: {
        let __mc_input = isyst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { orderedEqs, removedEqs, .. } => {
                    BackendDAEUtil::traverseBackendDAEExpsEqns(orderedEqs.clone(), (std::sync::Arc::new(traverserreplaceEdgeChange) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
                    BackendDAEUtil::traverseBackendDAEExpsEqns(removedEqs.clone(), (std::sync::Arc::new(traverserreplaceEdgeChange) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
                    Ok((isyst.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), inChanged))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (osyst, outShared, outChanged)
}

fn traverserreplaceEdgeChange(mut e: Arc<DAE::Exp>, mut b: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut oe: Arc<DAE::Exp>;
    let mut ob: bool;
    (oe, ob) = Expression::traverseExpBottomUp(e, (std::sync::Arc::new(fnptr!(traverserExpreplaceEdgeChange, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), b)?;
    Ok((oe, ob))
}

fn traverserExpreplaceEdgeChange(mut inExp: Arc<DAE::Exp>, mut inB: bool) -> (Arc<DAE::Exp>, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outB: bool;
    (outExp, outB) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut ty: Arc<DAE::Type>;
                    ty = Expression::r#typeof(e.clone())?;
                    Ok((Arc::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::NEQUAL { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("pre")).clone() }), expLst: list![e.clone()], attr: Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) }), index: -1, optionExpisASUB: None }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut ty: Arc<DAE::Type>;
                    ty = Expression::r#typeof(e.clone())?;
                    Ok((Arc::new(DAE::Exp::LBINARY { exp1: e.clone(), operator: DAE::Operator::AND { ty: ty.clone() }, exp2: Arc::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: ty.clone() }, exp: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("pre")).clone() }), expLst: list![e.clone()], attr: Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) }) }) }), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inB))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outB)
}

fn replaceEdgeChangeShared(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match &(inDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: systs, shared: shared @ Deref @ BackendDAE::Shared { removedEqs: remeqns, .. } } => {
            BackendDAEUtil::traverseBackendDAEExpsEqns(remeqns.clone(), (std::sync::Arc::new(traverserreplaceEdgeChange) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
            Arc::new(BackendDAE::BackendDAE { eqs: systs.clone(), shared: shared.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outDAE)
}

// =============================================================================
// section for preOptModule >>removeLocalKnownVars<<
//
// =============================================================================
pub(crate) fn removeLocalKnownVars(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE, (std::sync::Arc::new(removeLocalKnownVars2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>))?;
    Ok(outDAE)
}

pub(crate) fn removeLocalKnownVars2(mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut syst: Arc<BackendDAE::EqSystem> = syst;
    let mut shared: Arc<BackendDAE::Shared> = shared;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut potentialLocalKnownVar: BackendDAE::Var;
    let mut potentialGlobalKnownEquation: Arc<BackendDAE::Equation>;
    let mut orderedVars: BackendDAE::Variables = syst.orderedVars.clone();
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = syst.orderedEqs.clone();
    let mut lhs: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut crefExp: Arc<DAE::Exp>;
    let mut binding: Arc<DAE::Exp>;
    let mut localKnownVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut localKnownEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eindex: i32 = 0;
    let mut vindex: i32;
    (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
    m = Array::map(m.clone(), (std::sync::Arc::new(fnptr!(Tearing::deleteNegativeEntries, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>))?;
    let __range0 = m.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut row in __range0 {
        eindex = eindex + 1;
        if (row.clone().len() as i32) == 1 {
            let __pa1 = ::match_deref::match_deref! { match &(row.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } => __pa1.clone(),
                _ => bail!("pattern mismatch"),
            } };
            vindex = __pa1.clone();
            potentialLocalKnownVar = BackendVariable::getVarAt(orderedVars.clone(), vindex)?;
            potentialGlobalKnownEquation = BackendEquation::get(orderedEqs.clone(), eindex)?;
            if '__try3: {
                let (__pa4, __pa5) = ::match_deref::match_deref! { match &(potentialGlobalKnownEquation.clone()) {
                    Deref @ BackendDAE::Equation::EQUATION { exp: __pa4, scalar: __pa5, .. } => (__pa4.clone(), __pa5.clone()),
                    _ => break '__try3 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                lhs = __pa4.clone();
                rhs = __pa5.clone();
                crefExp = unwrap_break_err!(BackendVariable::varExp(potentialLocalKnownVar.clone()), '__try3);
                (binding, _) = unwrap_break_err!(ExpressionSolve::solve(lhs.clone(), rhs.clone(), crefExp.clone(), None), '__try3);
                potentialLocalKnownVar = BackendVariable::setBindExp(potentialLocalKnownVar.clone(), Some(binding.clone()));
                localKnownVars = metamodelica::cons(vindex, localKnownVars.clone());
                localKnownEqns = metamodelica::cons(eindex, localKnownEqns.clone());
                assign_field!(shared.localKnownVars = unwrap_break_err!(BackendVariable::addVar(potentialLocalKnownVar.clone(), shared.localKnownVars.clone()), '__try3));
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
        }
    }
    localKnownVars = List::sort(localKnownVars, (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    localKnownEqns = localKnownEqns.reverse();
    for mut var in &*localKnownVars {
        let mut var = var.clone();
        (orderedVars, _) = BackendVariable::removeVar(var.clone(), orderedVars.clone())?;
    }
    for mut eqn in &*localKnownEqns {
        let mut eqn = eqn.clone();
        orderedEqs = BackendEquation::delete(eqn.clone(), orderedEqs.clone())?;
    }
    assign_field!(
        syst.m = None,
        syst.mT = None,
        syst.matching = openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING(),
        syst.orderedVars = BackendVariable::listVar(BackendVariable::varList(orderedVars)?)?,
        syst.orderedEqs = orderedEqs
    );
    Ok((syst, shared))
}

// =============================================================================
// section for postOptModule >>addInitialStmtsToAlgorithms<<
//
//   Real a[3];
// algorithm       -->  algorithm
//   a[1] := 1.0;         a[1] := $START.a[1];
//                        a[2] := $START.a[2];
//                        a[3] := $START.a[3];
//                        a[1] := 1.0;
// =============================================================================
pub(crate) fn addInitialStmtsToAlgorithms(mut inDAE: Arc<BackendDAE::BackendDAE>, mut isInitialSystem: bool) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem1(inDAE, (std::sync::Arc::new(addInitialStmtsToAlgorithms1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, bool, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>), isInitialSystem)?;
    Ok(outDAE)
}

fn addInitialStmtsToAlgorithms1(mut syst: Arc<BackendDAE::EqSystem>, mut isInitialSystem: bool, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = syst.clone();
    let mut oshared: Arc<BackendDAE::Shared> = shared.clone();
    let mut ordvars: BackendDAE::Variables;
    let mut ordeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(osyst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ordvars = __pa0.clone();
    ordeqns = __pa1.clone();
    BackendEquation::traverseEquationArray_WithUpdate(ordeqns, (std::sync::Arc::new(eaddInitialStmtsToAlgorithms1Helper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (BackendDAE::Variables, bool)) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, bool))> + 'static>), (ordvars, isInitialSystem))?;
    Ok((osyst, oshared))
}

fn eaddInitialStmtsToAlgorithms1Helper(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (BackendDAE::Variables, bool)) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, bool))> {
    let mut outEq: Arc<BackendDAE::Equation>;
    let mut outTpl: (BackendDAE::Variables, bool) = inTpl.clone();
    outEq = (::match_deref::match_deref! { match &((inEq.clone(), inTpl)) {
        (Deref @ BackendDAE::Equation::ALGORITHM { size, alg: alg @ Deref @ DAE::Algorithm { statementLst: statements }, source, expand: crExpand, attr }, (vars, isInitialEquations)) => {
            let mut outputs: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut statements = (*statements).clone();
            crlst = CheckModel::checkAndGetAlgorithmOutputs(alg.clone(), source.clone(), crExpand.clone())?;
            outputs = List::map(crlst, (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>))?;
            statements = expandAlgorithmStmts(statements.clone(), outputs, vars.clone(), isInitialEquations.clone())?;
            Arc::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: Arc::new(DAE::Algorithm { statementLst: statements.clone() }), source: source.clone(), expand: crExpand.clone(), attr: attr.clone() })
        },
        _ => {
            inEq
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEq, outTpl))
}

pub(crate) fn expandAlgorithmStmts(mut inAlg: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inOutputs: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inVars: BackendDAE::Variables, mut isInitialEquation: bool) -> Result<Arc<metamodelica::List<Arc<DAE::Statement>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inAlg, inOutputs)) {
        (statements, Deref @ metamodelica::List::Nil) => {
            return Ok(statements.clone())
        },
        (statements, Deref @ metamodelica::List::Cons { head: out, tail: rest }) => {
            let mut initExp: Arc<DAE::Exp>;
            let mut cref: Arc<DAE::ComponentRef>;
            let mut vars: Arc<metamodelica::List<BackendDAE::Var>>;
            let mut stmt: Arc<DAE::Statement>;
            let mut type_: Arc<DAE::Type>;
            let mut statements = (*statements).clone();
            cref = Expression::expCref(out.clone())?;
            (vars, _) = BackendVariable::getVar(cref, inVars.clone())?;
            for mut v in &*vars {
                let mut v = v.clone();
                type_ = v.varType.clone();
                if BackendVariable::isVarDiscrete(v.clone()) && !(isInitialEquation) {
                    initExp = Expression::makePureBuiltinCall((literal!("pre")).clone(), list![Expression::crefExp(v.varName.clone())?], type_.clone());
                } else {
                    initExp = Expression::crefExp(ComponentReference::crefPrefixStart(v.varName.clone()))?;
                }
                stmt = Algorithm::makeAssignment(Arc::new(DAE::Exp::CREF { componentRef: v.varName.clone(), ty: type_.clone() }), DAE::Properties::PROP { type_: type_.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, initExp.clone(), DAE::Properties::PROP { type_: type_.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, DAE::dummyAttrVar().clone(), openmodelica_frontend_types::SCode::Initial::NON_INITIAL, DAE::emptyElementSource().clone())?;
                statements = metamodelica::cons(stmt.clone(), statements.clone());
            }
            { (inAlg, inOutputs, inVars, isInitialEquation) = (statements.clone(), rest.clone(), inVars, isInitialEquation); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

// =============================================================================
// section for expandDerOperator
//
// =============================================================================
pub(crate) fn expandDerOperator(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE, (std::sync::Arc::new(expandDerOperatorWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>))?;
    Ok(outDAE)
}

fn expandDerOperatorWork(mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut syst: Arc<BackendDAE::EqSystem> = syst;
    let mut shared: Arc<BackendDAE::Shared> = shared;
    (syst, shared) = (::match_deref::match_deref! { match &((syst.clone(), shared.clone())) {
        (__esc_syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. }, Deref @ BackendDAE::Shared { initialEqs: inieqns, .. }) => {
            syst = (*__esc_syst).clone();
            let mut shared_arr: Mutable::Mutable<Arc<BackendDAE::Shared>>;
            let mut vars = (*vars).clone();
            shared_arr = Mutable::create(shared);
            (_, vars) = BackendEquation::traverseEquationArray_WithUpdate(eqns.clone(), (std::sync::Arc::new({ let __pe_b2 = shared_arr.clone(); move |__pe_a0, __pe_a1| traverserexpandDerEquation(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables) -> Result<(Arc<BackendDAE::Equation>, BackendDAE::Variables)> + 'static>), vars.clone())?;
            (_, vars) = BackendEquation::traverseEquationArray_WithUpdate(inieqns.clone(), (std::sync::Arc::new({ let __pe_b2 = shared_arr.clone(); move |__pe_a0, __pe_a1| traverserexpandDerEquation(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables) -> Result<(Arc<BackendDAE::Equation>, BackendDAE::Variables)> + 'static>), vars.clone())?;
            assign_field!(syst.orderedVars = vars.clone());
            (syst.clone(), Mutable::access(shared_arr))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((syst, shared))
}

fn traverserexpandDerEquation(mut eq: Arc<BackendDAE::Equation>, mut vars: BackendDAE::Variables, mut shared: Mutable::Mutable<Arc<BackendDAE::Shared>>) -> Result<(Arc<BackendDAE::Equation>, BackendDAE::Variables)> {
    let mut eq: Arc<BackendDAE::Equation> = eq;
    let mut vars: BackendDAE::Variables = vars;
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
    let (__pa0, (__pa1, __pa2)) = BackendEquation::traverseExpsOfEquation(eq, (std::sync::Arc::new({ let __pe_b2 = shared; move |__pe_a0, __pe_a1| traverserexpandDerExp(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>))> + 'static>), (vars, metamodelica::nil()))?;
    eq = __pa0.clone();
    vars = __pa1.clone();
    ops = __pa2.clone();
    eq = List::foldr(ops, (std::sync::Arc::new(BackendEquation::addOperation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<DAE::SymbolicOperation>) -> Result<Arc<BackendDAE::Equation>> + 'static>), eq)?;
    Ok((eq, vars))
}

fn traverserexpandDerExp(mut exp: Arc<DAE::Exp>, mut tpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>), mut shared: Mutable::Mutable<Arc<BackendDAE::Shared>>) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>))> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut tpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>) = tpl;
    let mut exp_1: Arc<DAE::Exp>;
    let mut vars1: BackendDAE::Variables;
    let mut vars2: BackendDAE::Variables;
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
    (vars1, ops) = tpl.clone();
    (exp_1, vars2) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new({ let __pe_b2 = shared; move |__pe_a0, __pe_a1| expandDerExp(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> + 'static>), vars1.clone())?;
    if !({ let __refeq_sl = &(vars1); let __refeq_sr = &(vars2.clone()); referenceEq(&*(__refeq_sl.crefIndices),&*(__refeq_sr.crefIndices)) && { let __refeq_sl = &(__refeq_sl.varArr); let __refeq_sr = &(__refeq_sr.varArr); ((__refeq_sl.numberOfElements) == (__refeq_sr.numberOfElements)) && referenceEq(&*(__refeq_sl.varOptArr),&*(__refeq_sr.varOptArr)) } && ((__refeq_sl.bucketSize) == (__refeq_sr.bucketSize)) && ((__refeq_sl.numberOfVars) == (__refeq_sr.numberOfVars)) } && referenceEq(&*(exp.clone()),&*(exp_1.clone()))) {
        ops = metamodelica::cons(Arc::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: DAE::crefTime().clone(), before: exp, after: exp_1.clone() }), ops);
        exp = exp_1;
        tpl = (vars2, ops);
    }
    Ok((exp, tpl))
}

fn expandDerExp(mut exp: Arc<DAE::Exp>, mut vars: BackendDAE::Variables, mut inShared: Mutable::Mutable<Arc<BackendDAE::Shared>>) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut vars: BackendDAE::Variables = vars;
    let mut failed: bool = false;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut r#str: ArcStr;
            r#str = (ComponentReference::crefStr(cr.clone())?).clone();
            r#str = stringAppendList(list![(literal!("The model includes derivatives of order > 1 for: ")).clone(), (r#str.clone()).clone(), (literal!(". That is not supported. Adding 'Real d")).clone(), (r#str.clone()).clone(), (literal!(" = der(")).clone(), (r#str).clone(), (literal!(");' *might* result in a solvable model")).clone()]);
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str).clone()])?;
            bail!("fail")
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, vars) = 'mc: {
        let __mc_input = exp.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e1 @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp> = exp.clone();
                    let mut vars: BackendDAE::Variables = vars.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e1.clone(), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2 = __pa0.clone();
                    (exp, vars) = Expression::traverseExpBottomUp(e2.clone(), (std::sync::Arc::new({ let __pe_b2 = inShared.clone(); move |__pe_a0, __pe_a1| expandDerExp(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> + 'static>), vars.clone())?;
                    Ok(((exp.clone(), vars.clone()), exp.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { exp = __wb0; vars = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e1 @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut exp: Arc<DAE::Exp> = exp.clone();
                    let mut vars: BackendDAE::Variables = vars.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::extendArrExp(e1.clone(), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2 = __pa0.clone();
                    (exp, vars) = Expression::traverseExpBottomUp(e2.clone(), (std::sync::Arc::new({ let __pe_b2 = inShared.clone(); move |__pe_a0, __pe_a1| expandDerExp(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> + 'static>), vars.clone())?;
                    Ok(((exp.clone(), vars.clone()), exp.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { exp = __wb0; vars = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e1 @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut v: BackendDAE::Var;
                    let mut e1 = (*e1).clone();
                    let mut failed: bool = failed.clone();
                    let mut vars: BackendDAE::Variables = vars.clone();
                    (v, _) = BackendVariable::getVarSingle(cr.clone(), vars.clone())?;
                    if '__try0: {
                        (vars, e1) = unwrap_break_err!(updateStatesVar(vars.clone(), v.clone(), e1.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_err() {
                        failed = true;
                    }
                    Ok(((e1.clone(), vars.clone()), failed.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { failed = __wb0; vars = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e1 @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut vars: BackendDAE::Variables = vars.clone();
                    (varlst, _) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    vars = updateStatesVars(vars.clone(), varlst.clone(), false)?;
                    Ok(((e1.clone(), vars.clone()), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { vars = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut shared: Arc<BackendDAE::Shared>;
                    let mut vars: BackendDAE::Variables = vars.clone();
                    (e2, shared) = Differentiate::differentiateExpTime(e1.clone(), vars.clone(), Mutable::access(inShared.clone()))?;
                    let false = (Expression::isZero(e2.clone())?) else { bail!("pattern mismatch") };
                    Mutable::update(inShared.clone(), shared.clone());
                    (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                    (_, vars) = Expression::traverseExpBottomUp(e2.clone(), (std::sync::Arc::new(fnptr!(derCrefsExp, Arc<DAE::Exp>, BackendDAE::Variables)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, BackendDAE::Variables) -> Result<(Arc<DAE::Exp>, BackendDAE::Variables)> + 'static>), vars.clone())?;
                    Ok(((e2.clone(), vars.clone()), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { vars = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((exp.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    if failed {
        bail!("fail");
    }
    Ok((exp, vars))
}

fn derCrefsExp(mut inExp: Arc<DAE::Exp>, mut inVars: BackendDAE::Variables) -> (Arc<DAE::Exp>, BackendDAE::Variables) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outVars: BackendDAE::Variables;
    (outExp, outVars) = 'mc: {
        let __mc_input = (inExp.clone(), inVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, vars) => {
                    let mut v: BackendDAE::Var;
                    let mut e = (*e).clone();
                    let mut vars = (*vars).clone();
                    (v, _) = BackendVariable::getVarSingle(cr.clone(), vars.clone())?;
                    (vars, e) = updateStatesVar(vars.clone(), v.clone(), e.clone())?;
                    Ok((e.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, vars) => {
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut vars = (*vars).clone();
                    (varlst, _) = BackendVariable::getVar(cr.clone(), vars.clone())?;
                    vars = updateStatesVars(vars.clone(), varlst.clone(), false)?;
                    Ok((e.clone(), vars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outVars)
}

fn updateStatesVar(mut inVars: BackendDAE::Variables, mut var: BackendDAE::Var, mut iExp: Arc<DAE::Exp>) -> Result<(BackendDAE::Variables, Arc<DAE::Exp>)> {
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut oExp: Arc<DAE::Exp> = iExp.clone();
    let mut var1: BackendDAE::Var;
    let mut arg: Arc<DAE::Exp>;
    if BackendVariable::isVarNonDifferentiable(var.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(iExp) {
            Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa0.clone();
        Error::addSourceMessageAndFail(Error::DER_OF_NONDIFFERENTIABLE_EXP.clone(), list![(ExpressionBasics::printExpStr(arg)?).clone()], var.source.info.clone())?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    } else if BackendVariable::isVarDiscrete(var.clone()) {
        oExp = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
    } else if !(BackendVariable::isStateVar(var.clone())) || BackendVariable::varStateSelectForced(var.clone()) {
        var1 = BackendVariable::setVarKind(var, BackendDAE::VarKind::STATE { index: 1, derName: None, natural: true })?;
        outVars = BackendVariable::addVar(var1, inVars)?;
        oExp = iExp;
    }
    Ok((outVars, oExp))
}

fn updateStatesVars(mut inVars: BackendDAE::Variables, mut inNewStates: Arc<metamodelica::List<BackendDAE::Var>>, mut noStateFound: bool) -> Result<BackendDAE::Variables> {
    let mut outVars: BackendDAE::Variables;
    outVars = 'mc: {
        let __mc_input = (inNewStates, noStateFound);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, true) => {
                    Ok(inVars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: var, tail: newStates }, _) => {
                    let mut vars: BackendDAE::Variables;
                    let mut var = (*var).clone();
                    let false = (BackendVariable::isVarDiscrete(var.clone())) else { bail!("pattern mismatch") };
                    let false = (BackendVariable::isStateVar(var.clone())) else { bail!("pattern mismatch") };
                    var = BackendVariable::setVarKind(var.clone(), BackendDAE::VarKind::STATE { index: 1, derName: None, natural: true })?;
                    vars = BackendVariable::addVar(var.clone(), inVars.clone())?;
                    vars = updateStatesVars(vars.clone(), newStates.clone(), true)?;
                    Ok(vars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: newStates }, _) => {
                    let mut vars: BackendDAE::Variables;
                    vars = updateStatesVars(inVars.clone(), newStates.clone(), noStateFound)?;
                    Ok(vars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVars)
}

// =============================================================================
// section for addedScaledVars
//
// =============================================================================
pub(crate) fn addedScaledVars_states(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut systlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut osystlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
    let mut lst_states: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut tmpv: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut norm: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut y_norm: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut y: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut lhs: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systlst = __pa0.clone();
    oshared = __pa1.clone();
    for mut syst in &*systlst {
        let mut syst = syst.clone();
        syst = (::match_deref::match_deref! { match &(syst.clone()) {
        syst1 @ Deref @ BackendDAE::EqSystem { orderedVars: __esc_vars, orderedEqs: __esc_eqns, .. } => {
            vars = (*__esc_vars).clone();
            eqns = (*__esc_eqns).clone();
            let mut syst1 = (*syst1).clone();
            lst_states = List::select(BackendVariable::varList(vars.clone())?, (std::sync::Arc::new(fnptr!(BackendVariable::isStateVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?;
            for mut v in &*lst_states.clone() {
                let mut v = v.clone();
                cref = BackendVariable::varCref(v.clone())?;
                tmpv = BackendVariable::createVar(cref.clone(), (literal!("__OMC$scaled_state")).clone())?;
                y = Expression::crefExp(cref.clone())?;
                norm = BackendVariable::getVarNominalValue(v.clone());
                y_norm = Expression::expDiv(y.clone(), norm.clone())?;
                (y_norm, _) = ExpressionSimplify::simplify(y_norm.clone())?;
                cref = BackendVariable::varCref(tmpv.clone())?;
                lhs = Expression::crefExp(cref.clone())?;
                eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: y_norm.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                vars = BackendVariable::addVar(tmpv.clone(), vars.clone())?;
            }
            assign_field!(
                syst1.orderedVars = vars.clone(),
                syst1.orderedEqs = eqns.clone()
            );
            BackendDAEUtil::clearEqSyst(syst1.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        osystlst = metamodelica::cons(syst.clone(), osystlst.clone());
    }
    outDAE = Arc::new(BackendDAE::BackendDAE { eqs: osystlst, shared: oshared });
    Ok(outDAE)
}

pub(crate) fn addedScaledVars_inputs(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut systlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut osystlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
    let mut kvarlst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut lst_inputs: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut tmpv: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut norm: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut y_norm: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut y: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut lhs: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systlst = __pa0.clone();
    oshared = __pa1.clone();
    kvarlst = BackendVariable::varList(oshared.globalKnownVars.clone())?;
    lst_inputs = List::select(kvarlst, (std::sync::Arc::new(fnptr!(BackendVariable::isVarOnTopLevelAndInputNoDerInput, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?;
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(systlst) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    syst = __pa2.clone();
    osystlst = __pa3.clone();
    syst = (::match_deref::match_deref! { match &(syst) {
        syst1 @ Deref @ BackendDAE::EqSystem { orderedEqs: __esc_eqns, orderedVars: __esc_vars, .. } => {
            eqns = (*__esc_eqns).clone();
            vars = (*__esc_vars).clone();
            let mut syst1 = (*syst1).clone();
            for mut v in &*lst_inputs {
                let mut v = v.clone();
                cref = BackendVariable::varCref(v.clone())?;
                tmpv = BackendVariable::createVar(cref.clone(), (literal!("__OMC$scaled_input")).clone())?;
                y = Expression::crefExp(cref.clone())?;
                norm = BackendVariable::getVarNominalValue(v.clone());
                y_norm = Expression::expDiv(y.clone(), norm.clone())?;
                (y_norm, _) = ExpressionSimplify::simplify(y_norm.clone())?;
                cref = BackendVariable::varCref(tmpv.clone())?;
                lhs = Expression::crefExp(cref.clone())?;
                eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: y_norm.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                vars = BackendVariable::addVar(tmpv.clone(), vars.clone())?;
            }
            assign_field!(
                syst1.orderedEqs = eqns.clone(),
                syst1.orderedVars = vars.clone()
            );
            BackendDAEUtil::clearEqSyst(syst1.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    osystlst = metamodelica::cons(syst, osystlst);
    outDAE = Arc::new(BackendDAE::BackendDAE { eqs: osystlst, shared: oshared });
    Ok(outDAE)
}

// =============================================================================
// section for sortEqnsVars
//
// author: Vitalij Ruge
// =============================================================================
pub(crate) fn sortEqnsVars(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut systlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut new_systlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut ne: i32 = 0;
    let mut nv: i32 = 0;
    let mut w_vars: metamodelica::Array<i32> = Default::default();
    let mut w_eqns: metamodelica::Array<i32> = Default::default();
    let mut functionTree: Arc<AvlTreePathFunction::Tree>;
    let mut tplIndexWeight: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut indexs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systlst = __pa0.clone();
    shared = __pa1.clone();
    let __pa2 = ::match_deref::match_deref! { match &(shared.clone()) {
        Deref @ BackendDAE::Shared { functionTree: __pa2, .. } => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    functionTree = __pa2.clone();
    for mut syst in &*systlst {
        let mut syst = syst.clone();
        syst = (::match_deref::match_deref! { match &(syst.clone()) {
        syst1 @ Deref @ BackendDAE::EqSystem { orderedVars: __esc_vars, orderedEqs: __esc_eqns, .. } => {
            vars = (*__esc_vars).clone();
            eqns = (*__esc_eqns).clone();
            let mut syst1 = (*syst1).clone();
            (_, m, mT) = BackendDAEUtil::getAdjacencyMatrix(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, Some(functionTree.clone()), BackendDAEUtil::isInitializationDAE(shared.clone()))?;
            if Flags::isSet(Flags::SORT_EQNS_AND_VARS.clone())? {
                BackendDump::dumpAdjacencyMatrix(m.clone())?;
                BackendDump::dumpAdjacencyMatrixT(mT.clone())?;
            }
            let BackendDAE::VARIABLES { varArr: BackendDAE::VARIABLE_ARRAY { numberOfElements: __pa0, .. }, .. } = (vars.clone()) else { bail!("pattern mismatch") };
            nv = __pa0.clone();
            ne = ExpandableArray::getNumberOfElements(eqns.clone());
            w_vars = arrayCreate(nv, -1);
            w_eqns = arrayCreate(ne, -1);
            sortEqnsVarsWeights(w_vars.clone(), nv, mT.clone());
            sortEqnsVarsWeights(w_eqns.clone(), ne, m.clone());
            tplIndexWeight = ({
        let mut __acc: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
        for mut i in (1..=nv).into_iter() {
            let __x = (i.clone(), ({let __elt = w_vars.borrow()[(i.clone()-1) as usize].clone(); __elt}));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            tplIndexWeight = List::sort(tplIndexWeight.clone(), std::sync::Arc::new(fnptr!(Util::compareTuple2IntLt, _, _)))?;
            indexs = sortEqnsVarsWorkTpl(tplIndexWeight.clone());
            var_lst = ({
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut i in (indexs.clone()).into_iter().cloned() {
            let __x = BackendVariable::getVarAt(vars.clone(), i.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            vars = BackendVariable::listVar1(var_lst.clone())?;
            tplIndexWeight = ({
        let mut __acc: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
        for mut i in (1..=ne).into_iter() {
            let __x = (i.clone(), ({let __elt = w_eqns.borrow()[(i.clone()-1) as usize].clone(); __elt}));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            tplIndexWeight = List::sort(tplIndexWeight.clone(), std::sync::Arc::new(fnptr!(Util::compareTuple2IntGt, _, _)))?;
            indexs = sortEqnsVarsWorkTpl(tplIndexWeight.clone());
            eqn_lst = ({
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
        for mut i in (indexs.clone()).into_iter().cloned() {
            let __x = BackendEquation::get(eqns.clone(), i.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            eqns = BackendEquation::listEquation(eqn_lst.clone())?;
            assign_field!(
                syst1.orderedEqs = eqns.clone(),
                syst1.orderedVars = vars.clone()
            );
            if Flags::isSet(Flags::SORT_EQNS_AND_VARS.clone())? {
                (_, m, mT) = BackendDAEUtil::getAdjacencyMatrix(syst1.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, Some(functionTree.clone()), BackendDAEUtil::isInitializationDAE(shared.clone()))?;
                BackendDump::dumpAdjacencyMatrix(m.clone())?;
                BackendDump::dumpAdjacencyMatrixT(mT.clone())?;
            }
            GCExt::free(w_vars.clone());
            GCExt::free(w_eqns.clone());
            BackendDAEUtil::clearEqSyst(syst1.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        new_systlst = metamodelica::cons(syst.clone(), new_systlst.clone());
    }
    outDAE = Arc::new(BackendDAE::BackendDAE { eqs: new_systlst, shared: shared });
    Ok(outDAE)
}

fn sortEqnsVarsWorkTpl(mut tplIndexWeight: Arc<metamodelica::List<(i32, i32)>>) -> Arc<metamodelica::List<i32>> {
    let mut outIndexs: Arc<metamodelica::List<i32>>;
    outIndexs = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut elem in (tplIndexWeight).into_iter().cloned() {
            let __x = Util::tuple21(elem.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outIndexs
}

fn sortEqnsVarsWeights(mut inW: metamodelica::Array<i32>, mut n: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> metamodelica::Array<i32> {
    let mut outW: metamodelica::Array<i32> = inW.clone();
    let mut i: i32 = 0;
    for mut i in 1..=n {
        {
            let __cell0 = (({let __elt = m.borrow()[(i-1) as usize].clone(); __elt}).len() as i32);
            let __idx0 = i;
            outW.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
        }
    }
    outW
}

// =============================================================================
// fix some bugs for complex function
//
// e.g. (a,-b) = f(.) -> (a,c) = f(.) with c = -b
//      (a,b) = (c,d) -> a=c and b = d
//      {a,b} = {c,d} -> a=c and b = d
//      (a,b) = f(a) fixed iterration var
// author: Vitalij Ruge
// =============================================================================
pub(crate) fn simplifyComplexFunction(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = simplifyComplexFunction1(inDAE, true)?;
    Ok(outDAE)
}

pub(crate) fn simplifyComplexFunction1(mut inDAE: Arc<BackendDAE::BackendDAE>, mut withTmpVars: bool) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut systlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut n: i32;
    let mut size: i32;
    let mut idx: i32 = 1;
    let mut m: i32;
    let mut j: i32;
    let mut eqn: Arc<BackendDAE::Equation>;
    let mut eqn1: Arc<BackendDAE::Equation>;
    let mut left: Arc<DAE::Exp>;
    let mut right: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut e2: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut e: Arc<DAE::Exp>;
    let mut e3: Arc<DAE::Exp>;
    let mut left_lst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut right_lst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut indRemove: Arc<metamodelica::List<i32>>;
    let mut source: Arc<DAE::ElementSource>;
    let mut attr: BackendDAE::EquationAttributes;
    let mut update: bool;
    let mut sc: bool;
    let mut path: Arc<Absyn::Path>;
    let mut arrayLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut arrayLst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut cattr: Arc<DAE::CallAttributes>;
    let mut cr: Arc<DAE::ComponentRef>;
    let mut tmpvar: BackendDAE::Var;
    let mut tmpVarPrefix: ArcStr;
    shared = inDAE.shared.clone();
    tmpVarPrefix = ((::match_deref::match_deref! { match &(shared) {
        Deref @ BackendDAE::Shared { backendDAEType: BackendDAE::BackendDAEType::SIMULATION { .. }, .. } => literal!("$OMC$CF$sim"),
        Deref @ BackendDAE::Shared { backendDAEType: BackendDAE::BackendDAEType::INITIALSYSTEM { .. }, .. } => literal!("$OMC$CF$init"),
        _ => literal!("$OMC$CF$unknown"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    for mut syst in &*inDAE.eqs.clone() {
        let mut syst = syst.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        vars = __pa0.clone();
        eqns = __pa1.clone();
        n = ExpandableArray::getNumberOfElements(eqns.clone());
        update = false;
        indRemove = metamodelica::nil();
        '__loop2: for mut i in 1..=n {
            if let Ok(__iflet3) = BackendEquation::get(eqns.clone(), i.clone()) {
                eqn = __iflet3;
            } else {
                continue '__loop2;
            }
            if BackendEquation::isComplexEquation(eqn.clone()) || BackendEquation::isArrayEquation(eqn.clone()) {
                if BackendEquation::isComplexEquation(eqn.clone()) {
                    let (__pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(eqn.clone()) {
                        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size: __pa4, left: __pa5, right: __pa6, attr: __pa7, source: __pa8 } => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    size = __pa4.clone();
                    left = __pa5.clone();
                    right = __pa6.clone();
                    attr = __pa7.clone();
                    source = __pa8.clone();
                } else {
                    let (__pa9, __pa10, __pa11, __pa12) = ::match_deref::match_deref! { match &(eqn.clone()) {
                        Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: __pa9, right: __pa10, attr: __pa11, source: __pa12, .. } => (__pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    left = __pa9.clone();
                    right = __pa10.clone();
                    attr = __pa11.clone();
                    source = __pa12.clone();
                }
                if Expression::isTuple(left.clone()) && Expression::isTuple(right.clone()) {
                    let __pa13 = ::match_deref::match_deref! { match &(left.clone()) {
                        Deref @ DAE::Exp::TUPLE { PR: __pa13 } => __pa13.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    left_lst = __pa13.clone();
                    let __pa14 = ::match_deref::match_deref! { match &(right.clone()) {
                        Deref @ DAE::Exp::TUPLE { PR: __pa14 } => __pa14.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    right_lst = __pa14.clone();
                    update = true;
                    indRemove = metamodelica::cons(i.clone(), indRemove.clone());
                    for mut e1 in &*left_lst.clone() {
                        let mut e1 = e1.clone();
                        let (__pa15, __pa16) = ::match_deref::match_deref! { match &(right_lst.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa15, tail: __pa16 } => (__pa15.clone(), __pa16.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        e2 = __pa15.clone();
                        right_lst = __pa16.clone();
                        if !(Expression::isWild(e1.clone())) {
                            if Expression::isScalar(e2.clone())? {
                                eqn1 = BackendEquation::generateEquation(e1.clone(), e2.clone(), source.clone(), attr)?;
                                eqns = BackendEquation::add(eqn1.clone(), eqns.clone())?;
                            } else {
                                expLst = simplifyComplexFunction2(e1.clone());
                                arrayLst = simplifyComplexFunction2(e2.clone());
                                for mut e_asub in &*arrayLst.clone() {
                                    let mut e_asub = e_asub.clone();
                                    let (__pa17, __pa18) = ::match_deref::match_deref! { match &(expLst.clone()) {
                                        Deref @ metamodelica::List::Cons { head: __pa17, tail: __pa18 } => (__pa17.clone(), __pa18.clone()),
                                        _ => bail!("pattern mismatch"),
                                    } };
                                    e3 = __pa17.clone();
                                    expLst = __pa18.clone();
                                    eqn1 = BackendEquation::generateEquation(e_asub.clone(), e3.clone(), source.clone(), attr)?;
                                    eqns = BackendEquation::add(eqn1.clone(), eqns.clone())?;
                                }
                            }
                        }
                    }
                } else if Expression::isArray(left.clone()) && Expression::isArray(right.clone()) {
                    match '__try19: {
                        left_lst = unwrap_break_err!(Expression::getArrayOrRangeContents(left.clone()), '__try19);
                        right_lst = unwrap_break_err!(Expression::getArrayOrRangeContents(right.clone()), '__try19);
                        update = true;
                        indRemove = metamodelica::cons(i.clone(), indRemove.clone());
                        for mut e1 in &*left_lst.clone() {
                            let mut e1 = e1.clone();
                            let (__pa20, __pa21) = ::match_deref::match_deref! { match &(right_lst.clone()) {
                                Deref @ metamodelica::List::Cons { head: __pa20, tail: __pa21 } => (__pa20.clone(), __pa21.clone()),
                                _ => break '__try19 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                            } };
                            e2 = __pa20.clone();
                            right_lst = __pa21.clone();
                            if !(Expression::isWild(e1.clone())) {
                                if unwrap_break_err!(Expression::isScalar(e2.clone()), '__try19) {
                                    eqn1 = unwrap_break_err!(BackendEquation::generateEquation(e1.clone(), e2.clone(), source.clone(), attr), '__try19);
                                    eqns = unwrap_break_err!(BackendEquation::add(eqn1.clone(), eqns.clone()), '__try19);
                                } else {
                                    expLst = simplifyComplexFunction2(e1.clone());
                                    arrayLst = simplifyComplexFunction2(e2.clone());
                                    for mut e_asub in &*arrayLst.clone() {
                                        let mut e_asub = e_asub.clone();
                                        let (__pa22, __pa23) = ::match_deref::match_deref! { match &(expLst.clone()) {
                                            Deref @ metamodelica::List::Cons { head: __pa22, tail: __pa23 } => (__pa22.clone(), __pa23.clone()),
                                            _ => break '__try19 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                                        } };
                                        e3 = __pa22.clone();
                                        expLst = __pa23.clone();
                                        eqn1 = unwrap_break_err!(BackendEquation::generateEquation(e_asub.clone(), e3.clone(), source.clone(), attr), '__try19);
                                        eqns = unwrap_break_err!(BackendEquation::add(eqn1.clone(), eqns.clone()), '__try19);
                                    }
                                }
                            }
                        }
                        Ok::<_, anyhow::Error>((indRemove.clone(), left_lst.clone(), right_lst.clone(), update.clone()))
                    } {
                        Ok((__try19_o0, __try19_o1, __try19_o2, __try19_o3)) => {
                            indRemove = __try19_o0;
                            left_lst = __try19_o1;
                            right_lst = __try19_o2;
                            update = __try19_o3;
                        }
                        Err(_) => {
                            continue '__loop2;
                        }
                    }
                } else if withTmpVars && Expression::isTuple(left.clone()) && Expression::isCall(right.clone()) {
                    let __pa24 = ::match_deref::match_deref! { match &(left.clone()) {
                        Deref @ DAE::Exp::TUPLE { PR: __pa24 } => __pa24.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    left_lst = __pa24.clone();
                    let (__pa25, __pa26, __pa27) = ::match_deref::match_deref! { match &(right.clone()) {
                        Deref @ DAE::Exp::CALL { path: __pa25, expLst: __pa26, attr: __pa27 } => (__pa25.clone(), __pa26.clone(), __pa27.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    path = __pa25.clone();
                    expLst = __pa26.clone();
                    cattr = __pa27.clone();
                    expLst = metamodelica::nil();
                    for mut e1 in &*left_lst.clone() {
                        let mut e1 = e1.clone();
                        if Expression::isCref(e1.clone()) {
                            let __pa28 = ::match_deref::match_deref! { match &(e1.clone()) {
                                Deref @ DAE::Exp::CREF { componentRef: __pa28, .. } => __pa28.clone(),
                                _ => bail!("pattern mismatch"),
                            } };
                            cr = __pa28.clone();
                            if Expression::expHasCrefNoPreOrStart(right.clone(), cr.clone())? {
                                update = true;
                                cr = ComponentReferenceBasics::makeCrefIdent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpVarPrefix.clone()); __mm_s.push_str(&*intString(idx)); ArcStr::from(__mm_s) }).clone(), Expression::r#typeof(e1.clone())?, metamodelica::nil());
                                idx = idx + 1;
                                e = Expression::crefExp(cr.clone())?;
                                tmpvar = BackendVariable::makeVar(cr.clone())?;
                                tmpvar = BackendVariable::setVarTS(tmpvar.clone(), Some(openmodelica_backend_types::BackendDAE::TearingSelect::AVOID));
                                vars = BackendVariable::addVar(tmpvar.clone(), vars.clone())?;
                                eqn1 = Arc::new(BackendDAE::Equation::EQUATION { exp: e.clone(), scalar: e1.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                                eqns = BackendEquation::add(eqn1.clone(), eqns.clone())?;
                            } else {
                                e = e1.clone();
                            }
                        } else if Expression::isUnaryCref(e1.clone()) {
                            update = true;
                            cr = ComponentReferenceBasics::makeCrefIdent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpVarPrefix.clone()); __mm_s.push_str(&*intString(idx)); ArcStr::from(__mm_s) }).clone(), Expression::r#typeof(e1.clone())?, metamodelica::nil());
                            idx = idx + 1;
                            e = Expression::crefExp(cr.clone())?;
                            tmpvar = BackendVariable::makeVar(cr.clone())?;
                            tmpvar = BackendVariable::setVarTS(tmpvar.clone(), Some(openmodelica_backend_types::BackendDAE::TearingSelect::AVOID));
                            vars = BackendVariable::addVar(tmpvar.clone(), vars.clone())?;
                            eqn1 = Arc::new(BackendDAE::Equation::EQUATION { exp: e.clone(), scalar: e1.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                            eqns = BackendEquation::add(eqn1.clone(), eqns.clone())?;
                        } else if Expression::isArray(e1.clone()) {
                            update = true;
                            let (__pa29, __pa30) = ::match_deref::match_deref! { match &(e1.clone()) {
                                Deref @ DAE::Exp::ARRAY { array: __pa29, scalar: __pa30, .. } => (__pa29.clone(), __pa30.clone()),
                                _ => bail!("pattern mismatch"),
                            } };
                            arrayLst = __pa29.clone();
                            sc = __pa30.clone();
                            m = (arrayLst.clone().len() as i32);
                            cr = ComponentReferenceBasics::makeCrefIdent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpVarPrefix.clone()); __mm_s.push_str(&*intString(idx)); ArcStr::from(__mm_s) }).clone(), Expression::r#typeof(e1.clone())?, metamodelica::nil());
                            idx = idx + 1;
                            e = Expression::crefExp(cr.clone())?;
                            tmpvar = BackendVariable::makeVar(cr.clone())?;
                            tmpvar = BackendVariable::setVarTS(tmpvar.clone(), Some(openmodelica_backend_types::BackendDAE::TearingSelect::AVOID));
                            tmpvar.arryDim = list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: m })];
                            arrayLst2 = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut k in (1..=m).into_iter() {
            let __x = Expression::makeAsubAddIndex(e.clone(), k.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                            j = 1;
                            for mut e2 in &*arrayLst2.clone() {
                                let mut e2 = e2.clone();
                                let (__pa31, __pa32) = ::match_deref::match_deref! { match &(arrayLst.clone()) {
                                    Deref @ metamodelica::List::Cons { head: __pa31, tail: __pa32 } => (__pa31.clone(), __pa32.clone()),
                                    _ => bail!("pattern mismatch"),
                                } };
                                e3 = __pa31.clone();
                                arrayLst = __pa32.clone();
                                eqn1 = Arc::new(BackendDAE::Equation::EQUATION { exp: e2.clone(), scalar: e3.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                                eqns = BackendEquation::add(eqn1.clone(), eqns.clone())?;
                                cr = ComponentReferenceBasics::makeCrefIdent(({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpVarPrefix.clone()); __mm_s.push_str(&*intString(idx - 1)); ArcStr::from(__mm_s) }).clone(), Expression::r#typeof(e1.clone())?, list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: j }) })]);
                                j = j + 1;
                                tmpvar.varName = cr.clone();
                                vars = BackendVariable::addVar(tmpvar.clone(), vars.clone())?;
                            }
                        } else {
                            e = e1.clone();
                        }
                        expLst = metamodelica::cons(e.clone(), expLst.clone());
                    }
                    left = Arc::new(DAE::Exp::TUPLE { PR: metamodelica::Dangerous::listReverseInPlace(expLst.clone()) });
                    eqn = BackendEquation::generateEquation(left.clone(), right.clone(), source.clone(), attr)?;
                    eqns = BackendEquation::setAtIndex(eqns.clone(), i.clone(), eqn.clone())?;
                }
            }
        }
        if update {
            for mut i in &*indRemove.clone().reverse() {
                let mut i = i.clone();
                eqns = BackendEquation::delete(i.clone(), eqns.clone())?;
            }
            eqns = BackendEquation::listEquation(BackendEquation::equationList(eqns.clone())?)?;
            systlst = metamodelica::cons(BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), syst.stateSets.clone(), syst.partitionKind.clone(), syst.removedEqs.clone()), systlst.clone());
        } else {
            systlst = metamodelica::cons(syst.clone(), systlst.clone());
        }
    }
    assign_field!(outDAE.eqs = systlst);
    Ok(outDAE)
}

pub(crate) fn simplifyComplexFunction2(mut e1: Arc<DAE::Exp>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut out_lst_e1: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut lst_e: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    if '__try0: {
        if Expression::isArray(e1.clone()) || Expression::isArrayType(unwrap_break_err!(Expression::r#typeof(e1.clone()), '__try0)) {
            lst_e = unwrap_break_err!(Expression::getArrayOrRangeContents(e1.clone()), '__try0);
            for mut e in &*lst_e.clone() {
                let mut e = e.clone();
                out_lst_e1 = listAppend(simplifyComplexFunction2(e.clone()), out_lst_e1.clone());
            }
        } else if Expression::isRecord(e1.clone()) {
            lst_e = unwrap_break_err!(Expression::splitRecord(e1.clone(), unwrap_break_err!(Expression::r#typeof(e1.clone()), '__try0)), '__try0);
            for mut e in &*lst_e.clone() {
                let mut e = e.clone();
                out_lst_e1 = listAppend(simplifyComplexFunction2(e.clone()), out_lst_e1.clone());
            }
            out_lst_e1 = list![e1.clone()];
        } else {
            out_lst_e1 = list![e1.clone()];
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        out_lst_e1 = list![e1.clone()];
    }
    out_lst_e1
}

// =============================================================================
// section for hets
//
// (h)euristic (e)quation (t)erms (s)ort
// heuristic sorting of terms for better numeric in equations(res, torn,...)
//
// author: Vitalij Ruge
// =============================================================================
pub(crate) fn hets(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    if Flags::getConfigString(Flags::HETS.clone())? != literal!("none") {
        outDAE = hetsWork(inDAE)?;
    } else {
        outDAE = inDAE;
    }
    Ok(outDAE)
}

fn hetsWork(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut matching: Arc<BackendDAE::Matching>;
    let mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>>;
    let mut partitionKind: BackendDAE::BaseClockPartitionKind;
    let mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>>;
    let mut innerEquation: BackendDAE::InnerEquation = <BackendDAE::InnerEquation as ::std::default::Default>::default();
    let mut i: i32 = 0;
    let mut j: i32;
    let mut eqn: Arc<BackendDAE::Equation>;
    let mut tvars: Arc<metamodelica::List<i32>>;
    let mut teqns: Arc<metamodelica::List<i32>>;
    let mut cr: Arc<DAE::ComponentRef>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    shared = outDAE.shared.clone();
    for mut syst in &*outDAE.eqs.clone() {
        let mut syst = syst.clone();
        let (__pa0, __pa1, __pa3, __pa2, __pa4, __pa5) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, matching: __pa3 @ Deref @ BackendDAE::Matching::MATCHING { comps: __pa2, .. }, stateSets: __pa4, partitionKind: __pa5, .. } => (__pa0.clone(), __pa1.clone(), __pa3.clone(), __pa2.clone(), __pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        vars = __pa0.clone();
        eqns = __pa1.clone();
        comps = __pa2.clone();
        matching = __pa3.clone();
        stateSets = __pa4.clone();
        partitionKind = __pa5.clone();
        for mut comp in &*comps.clone() {
            let mut comp = comp.clone();
            if BackendEquation::isTornSystem(comp.clone()) {
                let (__pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(comp.clone()) {
                    Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: __pa7, residualequations: __pa8, innerEquations: __pa9, .. }, .. } => (__pa7.clone(), __pa8.clone(), __pa9.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                tvars = __pa7.clone();
                teqns = __pa8.clone();
                innerEquations = __pa9.clone();
                for mut innerEquation in &*innerEquations.clone() {
                    let mut innerEquation = innerEquation.clone();
                    if '__try10: {
                        let (__pa11, __pa12) = ::match_deref::match_deref! { match &(unwrap_break_err!(BackendDAEUtil::getEqnAndVarsFromInnerEquation(innerEquation.clone()), '__try10)) {
                            (__pa11, Deref @ metamodelica::List::Cons { head: __pa12, tail: Deref @ metamodelica::List::Nil }, _) => (__pa11.clone(), __pa12.clone()),
                            _ => break '__try10 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                        } };
                        i = __pa11.clone();
                        j = __pa12.clone();
                        eqn = unwrap_break_err!(BackendEquation::get(eqns.clone(), i), '__try10);
                        let BackendDAE::VAR { varName: __pa14, .. } = (unwrap_break_err!(BackendVariable::getVarAt(vars.clone(), j), '__try10)) else { break '__try10 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        cr = __pa14.clone();
                        eqn = unwrap_break_err!(BackendEquation::solveEquation(eqn.clone(), unwrap_break_err!(Expression::crefExp(cr.clone()), '__try10), Some(shared.functionTree.clone())), '__try10);
                        eqn = unwrap_break_err!(hetsSplitRhs(eqn.clone()), '__try10);
                        eqns = unwrap_break_err!(BackendEquation::setAtIndex(eqns.clone(), i, eqn.clone()), '__try10);
                        Ok::<(), anyhow::Error>(())
                    }.is_err() {
                    }
                }
                for mut i in &*teqns.clone() {
                    let mut i = i.clone();
                    eqn = BackendEquation::get(eqns.clone(), i)?;
                    eqn = hetsSplitRes(eqn.clone())?;
                    eqns = BackendEquation::setAtIndex(eqns.clone(), i, eqn.clone())?;
                }
            } else if BackendEquation::isEquationsSystem(comp.clone()) {
                let __pa15 = ::match_deref::match_deref! { match &(comp.clone()) {
                    Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: __pa15, .. } => __pa15.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                teqns = __pa15.clone();
                for mut i in &*teqns.clone() {
                    let mut i = i.clone();
                    eqn = BackendEquation::get(eqns.clone(), i)?;
                    eqn = hetsSplitRes(eqn.clone())?;
                    eqns = BackendEquation::setAtIndex(eqns.clone(), i, eqn.clone())?;
                }
            }
        }
    }
    Ok(outDAE)
}

fn hetsSplitRes(mut iEqn: Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> {
    let mut oEqn: Arc<BackendDAE::Equation>;
    oEqn = (::match_deref::match_deref! { match &(iEqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr } => {
            let mut e: Arc<DAE::Exp>;
            e = Expression::createResidualExp(e1.clone(), e2.clone())?;
            e = hetsSplitExp(e)?;
            Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e.clone(), source: source.clone(), attr: attr.clone() })
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, source, attr } => {
            let mut e = (*e).clone();
            e = hetsSplitExp(e.clone())?;
            Arc::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e.clone(), source: source.clone(), attr: attr.clone() })
        },
        _ => {
            iEqn
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oEqn)
}

fn hetsSplitRhs(mut iEqn: Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> {
    let mut oEqn: Arc<BackendDAE::Equation>;
    oEqn = (::match_deref::match_deref! { match &(iEqn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr } => {
            let mut e2 = (*e2).clone();
            e2 = hetsSplitExp(e2.clone())?;
            Arc::new(BackendDAE::Equation::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: source.clone(), attr: attr.clone() })
        },
        _ => {
            iEqn
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oEqn)
}

fn hetsSplitExp(mut iExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut oExp: Arc<DAE::Exp>;
    oExp = (::match_deref::match_deref! { match &(iExp.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } if (Expression::isMulOrDiv(op.clone())) => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            e1 = hetsSplitExp(e1.clone())?;
            e2 = hetsSplitExp(e2.clone())?;
            Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() })
        },
        e @ Deref @ DAE::Exp::BINARY { exp1: _, operator: op, exp2: _ } if (Expression::isAddOrSub(op.clone())) => {
            let mut terms: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut termsDer: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            terms = Expression::terms(e.clone())?;
            terms = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut t in (terms).into_iter().cloned() {
            let __x = hetsSplitExp(t.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (termsDer, terms) = List::splitOnTrue(terms, (std::sync::Arc::new(Expression::expHasDer) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?;
            Expression::expAdd(Expression::makeSum1(terms, false)?, Expression::makeSum1(termsDer, false)?)?
        },
        _ => {
            iExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oExp)
}

// =============================================================================
// section inlineFunctionInLoops
// force inlining function of loop
// author: Vitalij Ruge
// motivation see #3997 library devs introduce annotation(Inline=true) for simplify loops
// =============================================================================
pub(crate) fn inlineFunctionInLoops(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut dae: Arc<BackendDAE::BackendDAE> = dae;
    dae = inlineFunctionInLoopsMain(dae)?;
    Ok(dae)
}

fn inlineFunctionInLoopsMain(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut functionTree: Arc<AvlTreePathFunction::Tree>;
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut _syst: Arc<BackendDAE::EqSystem>;
    shared = inDAE.shared.clone();
    functionTree = shared.functionTree.clone();
    eqs = metamodelica::nil();
    for mut syst in &*inDAE.eqs.clone() {
        let mut syst = syst.clone();
        (_syst, shared) = inlineFunctionInLoopsWork(syst.clone(), functionTree.clone(), shared.clone())?;
        eqs = metamodelica::cons(_syst.clone(), eqs.clone());
    }
    outDAE = Arc::new(BackendDAE::BackendDAE { eqs: eqs, shared: shared });
    Ok(outDAE)
}

fn inlineFunctionInLoopsWork(mut syst: Arc<BackendDAE::EqSystem>, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut syst: Arc<BackendDAE::EqSystem> = syst;
    let mut shared: Arc<BackendDAE::Shared> = shared;
    let mut vars: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut partitionKind: BackendDAE::BaseClockPartitionKind;
    let mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut matching: Arc<BackendDAE::Matching>;
    let mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>>;
    let mut fns: (Option<Arc<AvlTreePathFunction::Tree>>, Arc<metamodelica::List<DAE::InlineType>>) = (Some(functionTree.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE]);
    let mut inlined: bool;
    let mut eq: Arc<BackendDAE::Equation>;
    let mut eqNew: Arc<BackendDAE::Equation>;
    let mut tmpEqs: Arc<BackendDAE::EqSystem>;
    let mut tmpEqs1: Arc<BackendDAE::EqSystem>;
    let mut idEqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut inlined1: bool;
    let mut id: i32 = 0;
    inlined = false;
    inlined1 = false;
    tmpEqs1 = BackendDAEUtil::createEqSystem(BackendVariable::listVar(metamodelica::nil())?, BackendEquation::listEquation(metamodelica::nil())?, metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    let (__pa0, __pa1, __pa3, __pa2, __pa4, __pa5) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, matching: __pa3 @ Deref @ BackendDAE::Matching::MATCHING { comps: __pa2, .. }, stateSets: __pa4, partitionKind: __pa5, .. } => (__pa0.clone(), __pa1.clone(), __pa3.clone(), __pa2.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    vars = __pa0.clone();
    eqns = __pa1.clone();
    comps = __pa2.clone();
    matching = __pa3.clone();
    stateSets = __pa4.clone();
    partitionKind = __pa5.clone();
    for mut comp in &*comps {
        let mut comp = comp.clone();
        if BackendEquation::isEquationsSystem(comp.clone()) || BackendEquation::isTornSystem(comp.clone()) || (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }) {
            idEqns = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: __esc_idEqns, .. } => {
            idEqns = (*__esc_idEqns).clone();
            idEqns.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: __esc_id, .. } => {
            id = (*__esc_id).clone();
            list![id.clone()]
        },
        _ => bail!("match: no arm matched"),
    } });
            for mut id in &*idEqns.clone() {
                let mut id = id.clone();
                eq = BackendEquation::get(eqns.clone(), id)?;
                (eqNew, tmpEqs, inlined, shared) = BackendInline::inlineEqAppend_debug(eq.clone(), fns.clone(), shared.clone())?;
                if inlined || !(BackendEquation::equationEqual(eq.clone(), eqNew.clone())?) {
                    tmpEqs1 = BackendDAEUtil::mergeEqSystems(tmpEqs.clone(), tmpEqs1.clone())?;
                    eqns = BackendEquation::setAtIndexFirst(id, eqNew.clone(), eqns.clone())?;
                    inlined1 = true;
                }
            }
        }
    }
    assign_field!(syst.orderedEqs = eqns);
    if inlined1 {
        syst = BackendDAEUtil::clearEqSyst(syst)?;
        syst = BackendDAEUtil::mergeEqSystems(tmpEqs1, syst)?;
    }
    Ok((syst, shared))
}

// =============================================================================
// section for simplifyLoops
//
// simplify(hopful) loops for simulation/optimization
// author: Vitalij Ruge
// =============================================================================
pub(crate) fn simplifyLoops(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = if (Flags::getConfigInt(Flags::SIMPLIFY_LOOPS.clone())? > 0) {simplifyLoopsMain(inDAE)?} else {inDAE};
    Ok(outDAE)
}

fn simplifyLoopsMain(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut new_systlst: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut partitionKind: BackendDAE::BaseClockPartitionKind;
    let mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut matching: Arc<BackendDAE::Matching>;
    let mut functionTree: Arc<AvlTreePathFunction::Tree>;
    let mut update: bool;
    let mut index: i32 = 1;
    let mut ii: i32;
    let mut nSyst: Arc<BackendDAE::EqSystem>;
    let mut ass1: Arc<metamodelica::List<i32>>;
    let mut ass2: Arc<metamodelica::List<i32>>;
    let mut compOrders: Arc<metamodelica::List<i32>>;
    let mut ne: i32;
    let mut nv: i32;
    let mut simDAE: bool;
    shared = inDAE.shared.clone();
    functionTree = shared.functionTree.clone();
    simDAE = (::match_deref::match_deref! { match &(shared.clone()) {
        Deref @ BackendDAE::Shared { backendDAEType: BackendDAE::BackendDAEType::SIMULATION { .. }, .. } => true,
        Deref @ BackendDAE::Shared { backendDAEType: BackendDAE::BackendDAEType::INITIALSYSTEM { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
        metamodelica::print((literal!("START: simplifyLoops\n")).clone());
        if !(simDAE) {
            metamodelica::print((literal!("\n***noSIM***\n")).clone());
        }
    }
    for mut syst in &*inDAE.eqs.clone() {
        let mut syst = syst.clone();
        update = false;
        ass1 = metamodelica::nil();
        ass2 = metamodelica::nil();
        compOrders = metamodelica::nil();
        ii = 1;
        let (__pa0, __pa1, __pa3, __pa2, __pa4, __pa5) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, matching: __pa3 @ Deref @ BackendDAE::Matching::MATCHING { comps: __pa2, .. }, stateSets: __pa4, partitionKind: __pa5, .. } => (__pa0.clone(), __pa1.clone(), __pa3.clone(), __pa2.clone(), __pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        vars = __pa0.clone();
        eqns = __pa1.clone();
        comps = __pa2.clone();
        matching = __pa3.clone();
        stateSets = __pa4.clone();
        partitionKind = __pa5.clone();
        ne = ExpandableArray::getNumberOfElements(eqns.clone());
        let BackendDAE::VARIABLES { numberOfVars: __pa7, .. } = (vars.clone()) else { bail!("pattern mismatch") };
        nv = __pa7.clone();
        for mut comp in &*comps.clone() {
            let mut comp = comp.clone();
            if BackendEquation::isEquationsSystem(comp.clone()) || BackendEquation::isTornSystem(comp.clone()) {
                (index, vars, eqns, shared, update, ass1, ass2, compOrders) = simplifyLoopsWork(comp.clone(), index, vars.clone(), eqns.clone(), shared.clone(), update, ass1.clone(), ass2.clone(), simDAE, ii, compOrders.clone())?;
            }
            ii = ii + 1;
        }
        nSyst = if (update) {simplifyLoopsUpdateMatching(vars.clone(), eqns.clone(), syst.clone(), ass1.clone().reverse(), ass2.clone().reverse(), ne, nv, functionTree.clone(), compOrders.clone().reverse())?} else {syst.clone()};
        new_systlst = metamodelica::cons(nSyst.clone(), new_systlst.clone());
    }
    outDAE = Arc::new(BackendDAE::BackendDAE { eqs: new_systlst, shared: shared });
    if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
        metamodelica::print((literal!("END: simplifyLoops\n")).clone());
    }
    Ok(outDAE)
}

fn simplifyLoopsUpdateMatching(mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inSyst: Arc<BackendDAE::EqSystem>, mut ass1_: Arc<metamodelica::List<i32>>, mut ass2_: Arc<metamodelica::List<i32>>, mut nEqns: i32, mut nVars: i32, mut functionTree: Arc<AvlTreePathFunction::Tree>, mut compOrders: Arc<metamodelica::List<i32>>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSyst: Arc<BackendDAE::EqSystem> = inSyst.clone();
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut n1: i32;
    let mut n2: i32;
    let mut matching: Arc<BackendDAE::Matching>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    matching = inSyst.matching.clone();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(matching) {
        Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, ass1: __pa1, ass2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    ass1 = __pa1.clone();
    ass2 = __pa2.clone();
    n1 = (ass1_.clone().len() as i32);
    n2 = (ass2_.clone().len() as i32);
    ass1 = Array::expand(n1, ass1.clone(), -1)?;
    ass2 = Array::expand(n2, ass2.clone(), -1)?;
    ass1 = simplifyLoopsUpdateAss(ass1.clone(), ass1_.clone(), nVars);
    ass2 = simplifyLoopsUpdateAss(ass2.clone(), ass2_.clone(), nEqns);
    comps = simplifyLoopsUpdateComps(comps, ass1_, ass2_, compOrders)?;
    assign_field!(
        outSyst.matching = Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: comps }),
        outSyst.orderedEqs = inEqns,
        outSyst.orderedVars = inVars
    );
    outSyst = BackendDAEUtil::setEqSystMatrices(outSyst, None, None, None)?;
    Ok(outSyst)
}

fn simplifyLoopsUpdateAss(mut inAss: metamodelica::Array<i32>, mut new_ass: Arc<metamodelica::List<i32>>, mut n: i32) -> metamodelica::Array<i32> {
    let mut outAss: metamodelica::Array<i32> = inAss.clone();
    let mut i: i32 = 1;
    for mut a in &*new_ass {
        let mut a = a.clone();
        {
            let __cell0 = a.clone();
            let __idx0 = i + n;
            outAss.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
        }
        i = i + 1;
    }
    outAss
}

fn simplifyLoopsUpdateComps(mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut inAss1: Arc<metamodelica::List<i32>>, mut inAss2: Arc<metamodelica::List<i32>>, mut inCompOrders: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>> {
    let mut outComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = inComps.clone();
    let mut a1: i32 = 0;
    let mut a2: i32;
    let mut shift: i32 = 0;
    let mut o: i32;
    let mut comp: Arc<BackendDAE::StrongComponent>;
    let mut ass1: Arc<metamodelica::List<i32>> = inAss1.clone();
    let mut ass2: Arc<metamodelica::List<i32>> = inAss2.clone();
    let mut compOrders: Arc<metamodelica::List<i32>> = inCompOrders.clone();
    for mut a1 in &*ass1 {
        let mut a1 = a1.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(compOrders.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        o = __pa0.clone();
        compOrders = __pa1.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ass2.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        a2 = __pa2.clone();
        ass2 = __pa3.clone();
        comp = Arc::new(BackendDAE::StrongComponent::SINGLEEQUATION { eqn: a1, var: a2 });
        outComps = List::insert(outComps.clone(), o + shift, comp.clone())?;
        shift = shift + 1;
    }
    Ok(outComps)
}

fn simplifyLoopsWork(mut inComp: Arc<BackendDAE::StrongComponent>, mut inIndx: i32, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inShared: Arc<BackendDAE::Shared>, mut inUpdate: bool, mut ass1_: Arc<metamodelica::List<i32>>, mut ass2_: Arc<metamodelica::List<i32>>, mut simDAE: bool, mut ii: i32, mut inCompOrders: Arc<metamodelica::List<i32>>) -> Result<(i32, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut outIndx: i32 = inIndx;
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outUpdate: bool = inUpdate;
    let mut ass1: Arc<metamodelica::List<i32>> = ass1_.clone();
    let mut ass2: Arc<metamodelica::List<i32>> = ass2_.clone();
    let mut outCompOrders: Arc<metamodelica::List<i32>> = inCompOrders.clone();
    let mut eqns: Arc<metamodelica::List<i32>>;
    let mut vv: Arc<metamodelica::List<i32>>;
    let mut vars: Arc<metamodelica::List<i32>>;
    let mut var_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut cr: Arc<DAE::ComponentRef>;
    let mut eqn: Arc<BackendDAE::Equation>;
    let mut update: bool;
    let mut linear: bool;
    let mut i: i32 = 0;
    let mut k: i32;
    let mut innerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>>;
    let mut innerEquation: BackendDAE::InnerEquation = <BackendDAE::InnerEquation as ::std::default::Default>::default();
    if BackendEquation::isEquationsSystem(inComp.clone()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inComp.clone()) {
            Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: __pa0, vars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        eqns = __pa0.clone();
        vars = __pa1.clone();
        if BackendDAEUtil::isLinearEqSystemComp(inComp) {
            return Ok((outIndx.clone(), outVars.clone(), outEqns.clone(), outShared.clone(), outUpdate.clone(), ass1.clone(), ass2.clone(), outCompOrders.clone()));
        }
        if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
            metamodelica::print((literal!("------ EquationsSystem ------\n")).clone());
        }
    } else {
        let (__pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(inComp) {
            Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: __pa2, strictTearingSet: BackendDAE::TearingSet { tearingvars: __pa3, residualequations: __pa4, innerEquations: __pa5, .. }, .. } => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        linear = __pa2.clone();
        vars = __pa3.clone();
        eqns = __pa4.clone();
        innerEquations = __pa5.clone();
        if linear {
            return Ok((outIndx.clone(), outVars.clone(), outEqns.clone(), outShared.clone(), outUpdate.clone(), ass1.clone(), ass2.clone(), outCompOrders.clone()));
        }
        if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
            metamodelica::print((literal!("------ Tearing ------\n")).clone());
        }
        for mut innerEquation in &*innerEquations {
            let mut innerEquation = innerEquation.clone();
            (k, vv, _) = BackendDAEUtil::getEqnAndVarsFromInnerEquation(innerEquation.clone())?;
            eqns = metamodelica::cons(k, eqns.clone());
            vars = listAppend(vv.clone(), vars.clone());
        }
    }
    if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
        metamodelica::print((literal!("------ loop-vars ------\n")).clone());
    }
    for mut i in &*vars {
        let mut i = i.clone();
        let BackendDAE::VAR { varName: __pa6, .. } = (BackendVariable::getVarAt(outVars.clone(), i)?) else { bail!("pattern mismatch") };
        cr = __pa6.clone();
        var_lst = metamodelica::cons(cr.clone(), var_lst.clone());
        if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    if Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone())? {
        metamodelica::print((literal!("------------\n")).clone());
    }
    for mut i in &*eqns {
        let mut i = i.clone();
        if '__try7: {
            eqn = unwrap_break_err!(BackendEquation::get(outEqns.clone(), i), '__try7);
            if unwrap_break_err!(Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone()), '__try7) {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("update eqn[")); __mm_s.push_str(&*intString(i)); __mm_s.push_str(&*literal!("]\n")); ArcStr::from(__mm_s) }).clone());
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(BackendDump::equationString(eqn.clone()), '__try7)); __mm_s.push_str(&*literal!("--old--\n")); ArcStr::from(__mm_s) }).clone());
            }
            (outIndx, outVars, outEqns, outShared, update, eqn, ass1, ass2, outCompOrders) = unwrap_break_err!(simplifyLoopEqn(outIndx, outVars.clone(), outEqns.clone(), outShared.clone(), var_lst.clone(), eqn.clone(), ass1.clone(), ass2.clone(), simDAE, ii, outCompOrders.clone()), '__try7);
            outUpdate = outUpdate || update;
            outEqns = unwrap_break_err!(BackendEquation::setAtIndex(outEqns.clone(), i, eqn.clone()), '__try7);
            if unwrap_break_err!(Flags::isSet(Flags::DUMP_SIMPLIFY_LOOPS.clone()), '__try7) {
                metamodelica::print((literal!("=> ")).clone());
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(BackendDump::equationString(eqn.clone()), '__try7)); __mm_s.push_str(&*literal!("--new--\n")); ArcStr::from(__mm_s) }).clone());
            }
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    Ok((outIndx, outVars, outEqns, outShared, outUpdate, ass1, ass2, outCompOrders))
}

fn simplifyLoopEqn(mut inIndx: i32, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inShared: Arc<BackendDAE::Shared>, mut var_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inEqn: Arc<BackendDAE::Equation>, mut ass1_: Arc<metamodelica::List<i32>>, mut ass2_: Arc<metamodelica::List<i32>>, mut simDAE: bool, mut ii: i32, mut inCompOrders: Arc<metamodelica::List<i32>>) -> Result<(i32, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut outIndx: i32 = inIndx;
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outUpdate: bool = false;
    let mut outEqn: Arc<BackendDAE::Equation> = inEqn.clone();
    let mut ass1: Arc<metamodelica::List<i32>> = ass1_.clone();
    let mut ass2: Arc<metamodelica::List<i32>> = ass2_.clone();
    let mut outCompOrder: Arc<metamodelica::List<i32>> = inCompOrders.clone();
    let mut rhs: Arc<DAE::Exp>;
    let mut lhs: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    let mut update_lhs: bool;
    let mut update_rhs: bool;
    let mut loopTerms_lhs: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut noLoopTerms_lhs: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut loopTerms_rhs: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut noLoopTerms_rhs: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut useTmpVars: bool = Flags::getConfigInt(Flags::SIMPLIFY_LOOPS.clone())? > 1;
    if BackendEquation::isAlgorithm(outEqn.clone()) {
        return Ok((outIndx.clone(), outVars.clone(), outEqns.clone(), outShared.clone(), outUpdate.clone(), outEqn.clone(), ass1.clone(), ass2.clone(), outCompOrder.clone()));
    }
    lhs = BackendEquation::getEquationLHS(outEqn.clone())?;
    if !(Types::isIntegerOrRealOrSubTypeOfEither(Expression::r#typeof(lhs.clone())?)) {
        return Ok((outIndx.clone(), outVars.clone(), outEqns.clone(), outShared.clone(), outUpdate.clone(), outEqn.clone(), ass1.clone(), ass2.clone(), outCompOrder.clone()));
    }
    rhs = BackendEquation::getEquationRHS(outEqn.clone())?;
    (loopTerms_lhs, noLoopTerms_lhs) = simplifyLoops_SplitTerms(var_lst.clone(), lhs)?;
    (loopTerms_rhs, noLoopTerms_rhs) = simplifyLoops_SplitTerms(var_lst.clone(), rhs)?;
    if (loopTerms_lhs.clone().len() as i32) > (loopTerms_rhs.clone().len() as i32) {
        lhs = Expression::expSub(Expression::makeSum1(loopTerms_lhs, false)?, Expression::makeSum1(loopTerms_rhs, false)?)?;
        rhs = Expression::expSub(Expression::makeSum1(noLoopTerms_rhs, false)?, Expression::makeSum1(noLoopTerms_lhs, false)?)?;
    } else {
        lhs = Expression::expSub(Expression::makeSum1(loopTerms_rhs, false)?, Expression::makeSum1(loopTerms_lhs, false)?)?;
        rhs = Expression::expSub(Expression::makeSum1(noLoopTerms_lhs, false)?, Expression::makeSum1(noLoopTerms_rhs, false)?)?;
    }
    (lhs, rhs, _) = Expression::createResidualExp3(lhs, rhs);
    (lhs, e) = Expression::makeFraction(lhs)?;
    (lhs, _) = ExpressionSimplify::simplify(lhs)?;
    (e, _) = ExpressionSimplify::simplify(e)?;
    rhs = ExpressionSimplify::simplifySumOperatorExpression(rhs.clone(), DAE::Operator::MUL { ty: Expression::r#typeof(rhs)? }, e)?;
    (outIndx, outVars, outEqns, outShared, update_rhs, rhs, ass1, ass2, outCompOrder) = simplifyLoopExp(outIndx, outVars, outEqns, outShared, var_lst.clone(), rhs, ass1, ass2, simDAE, useTmpVars, ii, outCompOrder, (literal!("LOOP")).clone(), false)?;
    (outIndx, outVars, outEqns, outShared, update_lhs, lhs, ass1, ass2, outCompOrder) = simplifyLoopExp(outIndx, outVars, outEqns, outShared, var_lst, lhs, ass1, ass2, simDAE, useTmpVars, ii, outCompOrder, (literal!("LOOP")).clone(), false)?;
    outEqn = BackendEquation::setEquationLHS(outEqn, lhs)?;
    outEqn = BackendEquation::setEquationRHS(outEqn, rhs)?;
    outUpdate = outUpdate || update_rhs || update_lhs;
    Ok((outIndx, outVars, outEqns, outShared, outUpdate, outEqn, ass1, ass2, outCompOrder))
}

pub(crate) fn simplifyLoopExp(mut inIndx: i32, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inShared: Arc<BackendDAE::Shared>, mut var_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inExp: Arc<DAE::Exp>, mut ass1_: Arc<metamodelica::List<i32>>, mut ass2_: Arc<metamodelica::List<i32>>, mut simDAE: bool, mut useTmpVars: bool, mut ii: i32, mut inCompOrders: Arc<metamodelica::List<i32>>, mut tmpVarName: ArcStr, mut noPara: bool) -> Result<(i32, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, Arc<DAE::Exp>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut outIndx: i32 = inIndx;
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outUpdate: bool = false;
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut ass1: Arc<metamodelica::List<i32>> = ass1_.clone();
    let mut ass2: Arc<metamodelica::List<i32>> = ass2_.clone();
    let mut outCompOrder: Arc<metamodelica::List<i32>> = inCompOrders.clone();
    let mut loopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut noLoopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut loopFactors: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut noLoopFactors: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut loopTermsUpdatedFactors: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut loopFacotrsUpdatedTerms: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut res: Arc<DAE::Exp>;
    let mut noLoopTerm: Arc<DAE::Exp>;
    let mut noLoopFactor: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut update: bool;
    let mut op: DAE::Operator;
    let mut para: bool;
    (loopTerms, noLoopTerms) = simplifyLoops_SplitTerms(var_lst.clone(), outExp)?;
    (noLoopTerm, _) = ExpressionSimplify::simplify1(Expression::makeSum1(noLoopTerms, false)?)?;
    if useTmpVars && simDAE {
        (noLoopTerm, outEqns, outVars, outShared, update, para) = BackendEquation::makeTmpEqnForExp(noLoopTerm, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpVarName.clone()); __mm_s.push_str(&*literal!("T")); ArcStr::from(__mm_s) }).clone(), System::tmpTickIndex(Global::tmpVariableIndex.clone()), outEqns, outVars, outShared, false)?;
        (outUpdate, ass1, ass2, outIndx, outCompOrder) = simplifyLoopExpHelper(update, outUpdate, para, ass1, ass2, outVars.clone(), outEqns.clone(), outIndx, ii, outCompOrder)?;
    }
    loopTermsUpdatedFactors = metamodelica::nil();
    for mut factor in &*loopTerms {
        let mut factor = factor.clone();
        (loopFactors, noLoopFactors) = simplifyLoops_SplitFactors(var_lst.clone(), factor.clone())?;
        (noLoopFactor, _) = ExpressionSimplify::simplify1(Expression::makeProductLst(noLoopFactors.clone())?)?;
        if useTmpVars && simDAE {
            if (::match_deref::match_deref! { match &(noLoopFactor.clone()) {
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::DIV { .. }, .. } => true,
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::POW { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }) {
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(noLoopFactor.clone()) {
                    Deref @ DAE::Exp::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                e1 = __pa0.clone();
                op = __pa1.clone();
                e2 = __pa2.clone();
                (e1, outEqns, outVars, outShared, update, para) = BackendEquation::makeTmpEqnForExp(e1.clone(), (literal!("LOOPF")).clone(), if (simDAE) {outIndx} else {-(outIndx)}, outEqns.clone(), outVars.clone(), outShared.clone(), noPara)?;
                (outUpdate, ass1, ass2, outIndx, outCompOrder) = simplifyLoopExpHelper(update, outUpdate, para, ass1.clone(), ass2.clone(), outVars.clone(), outEqns.clone(), outIndx, ii, outCompOrder.clone())?;
                (e2, outEqns, outVars, outShared, update, para) = BackendEquation::makeTmpEqnForExp(e2.clone(), (literal!("LOOPF")).clone(), if (simDAE) {outIndx} else {-(outIndx)}, outEqns.clone(), outVars.clone(), outShared.clone(), noPara)?;
                (outUpdate, ass1, ass2, outIndx, outCompOrder) = simplifyLoopExpHelper(update, outUpdate, para, ass1.clone(), ass2.clone(), outVars.clone(), outEqns.clone(), outIndx, ii, outCompOrder.clone())?;
                noLoopFactor = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() });
            } else {
                (noLoopFactor, outEqns, outVars, outShared, update, para) = BackendEquation::makeTmpEqnForExp(noLoopFactor.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpVarName.clone()); __mm_s.push_str(&*literal!("F")); ArcStr::from(__mm_s) }).clone(), if (simDAE) {outIndx} else {-(outIndx)}, outEqns.clone(), outVars.clone(), outShared.clone(), false)?;
                (outUpdate, ass1, ass2, outIndx, outCompOrder) = simplifyLoopExpHelper(update, outUpdate, para, ass1.clone(), ass2.clone(), outVars.clone(), outEqns.clone(), outIndx, ii, outCompOrder.clone())?;
            }
        }
        loopFacotrsUpdatedTerms = metamodelica::nil();
        for mut term in &*loopFactors.clone() {
            let mut term = term.clone();
            res = term.clone();
            if Expression::isBinary(res.clone()) {
                let __pa3 = ::match_deref::match_deref! { match &(res.clone()) {
                    Deref @ DAE::Exp::BINARY { operator: __pa3, .. } => __pa3.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                op = __pa3.clone();
                if Expression::isAddOrSub(op.clone()) || Expression::isMulOrDiv(op.clone()) || Expression::isPow(op.clone()) {
                    if !(ExpressionBasics::expEqual(res.clone(), inExp.clone())?) {
                        if Expression::isDiv(op.clone()) || Expression::isPow(op.clone()) {
                            let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(res.clone()) {
                                Deref @ DAE::Exp::BINARY { exp1: __pa4, operator: __pa5, exp2: __pa6 } => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
                                _ => bail!("pattern mismatch"),
                            } };
                            e1 = __pa4.clone();
                            op = __pa5.clone();
                            e2 = __pa6.clone();
                            (outIndx, outVars, outEqns, outShared, update, e1, ass1, ass2, outCompOrder) = simplifyLoopExp(outIndx, outVars.clone(), outEqns.clone(), outShared.clone(), var_lst.clone(), e1.clone(), ass1.clone(), ass2.clone(), simDAE, useTmpVars, ii, outCompOrder.clone(), (literal!("LOOP")).clone(), false)?;
                            outUpdate = update || outUpdate;
                            (outIndx, outVars, outEqns, outShared, update, e2, ass1, ass2, outCompOrder) = simplifyLoopExp(outIndx, outVars.clone(), outEqns.clone(), outShared.clone(), var_lst.clone(), e2.clone(), ass1.clone(), ass2.clone(), simDAE, useTmpVars, ii, outCompOrder.clone(), (literal!("LOOP")).clone(), false)?;
                            outUpdate = update || outUpdate;
                            (e2, _) = ExpressionSimplify::simplify1(e2.clone())?;
                            res = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() });
                        } else {
                            (outIndx, outVars, outEqns, outShared, update, res, ass1, ass2, outCompOrder) = simplifyLoopExp(outIndx, outVars.clone(), outEqns.clone(), outShared.clone(), var_lst.clone(), res.clone(), ass1.clone(), ass2.clone(), simDAE, useTmpVars, ii, outCompOrder.clone(), (literal!("LOOP")).clone(), false)?;
                            outUpdate = update || outUpdate;
                        }
                    }
                }
            }
            loopFacotrsUpdatedTerms = metamodelica::cons(res.clone(), loopFacotrsUpdatedTerms.clone());
        }
        loopTermsUpdatedFactors = metamodelica::cons(Expression::makeProductLst(metamodelica::cons(noLoopFactor.clone(), loopFacotrsUpdatedTerms.clone()))?, loopTermsUpdatedFactors.clone());
    }
    (outExp, _) = ExpressionSimplify::simplify(Expression::makeSum1(metamodelica::cons(noLoopTerm, loopTermsUpdatedFactors), true)?)?;
    Ok((outIndx, outVars, outEqns, outShared, outUpdate, outExp, ass1, ass2, outCompOrder))
}

fn simplifyLoopExpHelper(mut update: bool, mut update_: bool, mut para: bool, mut ass1_: Arc<metamodelica::List<i32>>, mut ass2_: Arc<metamodelica::List<i32>>, mut inVars: BackendDAE::Variables, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inIndex: i32, mut ii: i32, mut inCompOrders: Arc<metamodelica::List<i32>>) -> Result<(bool, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, i32, Arc<metamodelica::List<i32>>)> {
    let mut outUpdate: bool = update_;
    let mut ass1: Arc<metamodelica::List<i32>> = ass1_.clone();
    let mut ass2: Arc<metamodelica::List<i32>> = ass2_.clone();
    let mut outIndx: i32 = inIndex;
    let mut outCompOrder: Arc<metamodelica::List<i32>> = inCompOrders.clone();
    let mut ne: i32;
    let mut nv: i32;
    if update {
        outIndx = outIndx + 1;
        outUpdate = update;
        if !(para) {
            ne = ExpandableArray::getNumberOfElements(inEqns);
            let BackendDAE::VARIABLES { numberOfVars: __pa0, .. } = (inVars) else { bail!("pattern mismatch") };
            nv = __pa0.clone();
            ass1 = metamodelica::cons(ne, ass1);
            ass2 = metamodelica::cons(nv, ass2);
            outCompOrder = metamodelica::cons(ii, outCompOrder);
        }
    }
    Ok((outUpdate, ass1, ass2, outIndx, outCompOrder))
}

pub(crate) fn simplifyLoops_SplitTerms(mut var_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inExp: Arc<DAE::Exp>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> {
    let mut loopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut noLoopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut tmp_loopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    noLoopTerms = Expression::terms(inExp)?;
    for mut cr in &*var_lst {
        let mut cr = cr.clone();
        if noLoopTerms.clone().is_empty() {
            break;
        } else {
            (tmp_loopTerms, noLoopTerms) = List::split1OnTrue(noLoopTerms.clone(), (std::sync::Arc::new(Expression::expHasCrefNoPreOrStart) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cr.clone())?;
            loopTerms = listAppend(tmp_loopTerms.clone(), loopTerms.clone());
        }
    }
    Ok((loopTerms, noLoopTerms))
}

fn simplifyLoops_SplitFactors(mut var_lst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inExp: Arc<DAE::Exp>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> {
    let mut loopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut noLoopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut tmp_loopTerms: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    noLoopTerms = Expression::factors(inExp)?;
    for mut cr in &*var_lst {
        let mut cr = cr.clone();
        if noLoopTerms.clone().is_empty() {
            break;
        } else {
            (tmp_loopTerms, noLoopTerms) = List::split1OnTrue(noLoopTerms.clone(), (std::sync::Arc::new(Expression::expHasCrefNoPreOrStart) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), cr.clone())?;
            loopTerms = listAppend(tmp_loopTerms.clone(), loopTerms.clone());
        }
    }
    Ok((loopTerms, noLoopTerms))
}

// =============================================================================
// section for introduceDerAlias
//
// =============================================================================
pub(crate) fn introduceDerAlias(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE, (std::sync::Arc::new(introduceDerAliasWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>))?;
    Ok(outDAE)
}

fn introduceDerAliasWork(mut inSyst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared> = shared.clone();
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
    let mut eqnsList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    osyst = (::match_deref::match_deref! { match &(inSyst) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: __esc_vars, orderedEqs: __esc_eqns, .. } => {
            vars = (*__esc_vars).clone();
            eqns = (*__esc_eqns).clone();
            let mut syst = (*syst).clone();
            let (__pa0, (__pa1, __pa2, _, _)) = BackendEquation::traverseEquationArray_WithUpdate(eqns.clone(), (std::sync::Arc::new(traverserintroduceDerAliasEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool)) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool))> + 'static>), (vars.clone(), metamodelica::nil(), shared, true))?;
            eqns = __pa0.clone();
            vars = __pa1.clone();
            eqnsList = __pa2.clone();
            eqns = BackendEquation::addList(eqnsList, eqns.clone())?;
            assign_field!(
                syst.orderedEqs = eqns.clone(),
                syst.orderedVars = vars.clone()
            );
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((osyst, oshared))
}

fn traverserintroduceDerAliasEquation(mut inEq: Arc<BackendDAE::Equation>, mut tpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool)) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool))> {
    let mut outEq: Arc<BackendDAE::Equation>;
    let mut outTpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool);
    let mut e: Arc<BackendDAE::Equation>;
    let mut vars: BackendDAE::Variables;
    let mut b: bool;
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut eqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    (vars, eqnLst, shared, b) = tpl;
    let (__pa0, (__pa1, __pa2, __pa3, __pa4, _)) = BackendEquation::traverseExpsOfEquation(inEq, (std::sync::Arc::new(traverserintroduceDerAliasExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, bool))> + 'static>), (vars, eqnLst, shared, metamodelica::nil(), b))?;
    e = __pa0.clone();
    vars = __pa1.clone();
    eqnLst = __pa2.clone();
    shared = __pa3.clone();
    ops = __pa4.clone();
    outEq = List::foldr(ops, (std::sync::Arc::new(BackendEquation::addOperation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<DAE::SymbolicOperation>) -> Result<Arc<BackendDAE::Equation>> + 'static>), e)?;
    outTpl = (vars, eqnLst, shared, b);
    Ok((outEq, outTpl))
}

fn traverserintroduceDerAliasExp(mut inExp: Arc<DAE::Exp>, mut tpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>, bool);
    let mut e: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp>;
    let mut ext_arg: (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, bool);
    let mut vars: BackendDAE::Variables;
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
    let mut b: bool;
    let mut addVars: bool;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut eqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    e = inExp;
    (vars, eqnLst, shared, ops, addVars) = tpl;
    ext_arg = (vars, eqnLst, shared, addVars, false);
    let (__pa0, (__pa1, __pa2, __pa3, _, __pa4)) = Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(fnptr!(introDerAlias, Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, bool)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, bool))> + 'static>), ext_arg)?;
    e1 = __pa0.clone();
    vars = __pa1.clone();
    eqnLst = __pa2.clone();
    shared = __pa3.clone();
    b = __pa4.clone();
    ops = List::consOnTrue(b, Arc::new(DAE::SymbolicOperation::SUBSTITUTION { substitutions: list![e1.clone()], source: e }), ops);
    outExp = e1;
    outTpl = (vars, eqnLst, shared, ops, addVars);
    Ok((outExp, outTpl))
}

fn introDerAlias(mut inExp: Arc<DAE::Exp>, mut itpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, bool)) -> (Arc<DAE::Exp>, (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, bool)) {
    let mut outExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut tpl: (BackendDAE::Variables, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<BackendDAE::Shared>, bool, bool);
    (outExp, tpl) = 'mc: {
        let __mc_input = (inExp.clone(), itpl.clone());
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty }, tail: Deref @ metamodelica::List::Nil }, .. }, (vars, eqnLst, shared, addVar, _)) => {
                    let mut cref: Arc<DAE::ComponentRef>;
                    let mut v: BackendDAE::Var;
                    let mut v1: BackendDAE::Var;
                    let mut numVars: i32;
                    let mut vars = (*vars).clone();
                    let mut eqnLst = (*eqnLst).clone();
                    let mut outExp: Arc<DAE::Exp> = outExp.clone();
                    (v, _) = BackendVariable::getVarSingle(cr.clone(), vars.clone())?;
                    cref = BackendVariable::varCref(v.clone())?;
                    v1 = BackendVariable::createAliasDerVar(cref.clone())?;
                    v1 = BackendVariable::mergeNominalAttribute(v.clone(), v1.clone(), false);
                    cref = BackendVariable::varCref(v1.clone())?;
                    outExp = Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ty.clone() });
                    if addVar.clone() {
                        numVars = BackendVariable::varsSize(vars.clone());
                        vars = BackendVariable::addVar(v1.clone(), vars.clone())?;
                        eqnLst = if (numVars.clone() < BackendVariable::varsSize(vars.clone())) {metamodelica::cons(Arc::new(BackendDAE::Equation::EQUATION { exp: inExp.clone(), scalar: outExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), eqnLst.clone())} else {eqnLst.clone()};
                    }
                    Ok(((outExp.clone(), (vars.clone(), eqnLst.clone(), shared.clone(), addVar.clone(), true)), outExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outExp = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _) => {
                    let mut r#str: ArcStr;
                    let mut r#str: ArcStr;
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAEOptimize.introduceDerAlias failed for: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), itpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, tpl)
}

// =============================================================================
// section for replaceDerCall
//
// =============================================================================
pub(crate) fn replaceDerCalls(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = BackendDAEUtil::mapEqSystem(inDAE, (std::sync::Arc::new(replaceDerCallWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>))?;
    Ok(outDAE)
}

fn replaceDerCallWork(mut inSyst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared> = shared.clone();
    osyst = (::match_deref::match_deref! { match &(inSyst) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. } => {
            let mut localKnowns: BackendDAE::Variables;
            let mut syst = (*syst).clone();
            let mut vars = (*vars).clone();
            let mut eqns = (*eqns).clone();
            (eqns, vars) = BackendEquation::traverseEquationArray_WithUpdate(eqns.clone(), (std::sync::Arc::new(traverserreplaceDerCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::Variables) -> Result<(Arc<BackendDAE::Equation>, BackendDAE::Variables)> + 'static>), vars.clone())?;
            (localKnowns, vars) = BackendVariable::traverseBackendDAEVars(vars.clone(), (std::sync::Arc::new(moveStatesVariables) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables))> + 'static>), (oshared.localKnownVars.clone(), vars.clone()))?;
            assign_field!(oshared.localKnownVars = localKnowns);
            assign_field!(
                syst.orderedEqs = eqns.clone(),
                syst.orderedVars = vars.clone()
            );
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((osyst, oshared))
}

fn traverserreplaceDerCall(mut inEq: Arc<BackendDAE::Equation>, mut inVars: BackendDAE::Variables) -> Result<(Arc<BackendDAE::Equation>, BackendDAE::Variables)> {
    let mut outEq: Arc<BackendDAE::Equation>;
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut e: Arc<BackendDAE::Equation>;
    let mut ops: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
    (e, ops) = BackendEquation::traverseExpsOfEquation(inEq, (std::sync::Arc::new(traverserreplaceDerCallExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>)> + 'static>), metamodelica::nil())?;
    outEq = List::foldr(ops, (std::sync::Arc::new(BackendEquation::addOperation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<DAE::SymbolicOperation>) -> Result<Arc<BackendDAE::Equation>> + 'static>), e)?;
    Ok((outEq, outVars))
}

fn traverserreplaceDerCallExp(mut inExp: Arc<DAE::Exp>, mut tpl: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: Arc<metamodelica::List<Arc<DAE::SymbolicOperation>>>;
    let mut e: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp>;
    let mut b: bool;
    e = inExp;
    (e1, b) = Expression::traverseExpBottomUp(e.clone(), (std::sync::Arc::new(fnptr!(replaceDerCall, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
    outTpl = List::consOnTrue(b, Arc::new(DAE::SymbolicOperation::SUBSTITUTION { substitutions: list![e1.clone()], source: e }), tpl);
    outExp = e1;
    Ok((outExp, outTpl))
}

fn replaceDerCall(mut inExp: Arc<DAE::Exp>, mut itpl: bool) -> (Arc<DAE::Exp>, bool) {
    let mut outExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut tpl: bool;
    (outExp, tpl) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty }, tail: Deref @ metamodelica::List::Nil }, .. } => {
                    let mut cref: Arc<DAE::ComponentRef>;
                    let mut outExp: Arc<DAE::Exp> = outExp.clone();
                    cref = ComponentReference::crefPrefixDer(cr.clone());
                    outExp = Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ty.clone() });
                    Ok(((outExp.clone(), true), outExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { outExp = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAEOptimize.replaceDerCall")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), itpl))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, tpl)
}

fn moveStatesVariables(mut inVar: BackendDAE::Var, mut inTpl: (BackendDAE::Variables, BackendDAE::Variables)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables))> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables) = inTpl.clone();
    let () = (match inVar.clone() {
        BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, varName: ref cref, .. } => {
            let mut newVar: BackendDAE::Var;
            let mut localKnowns: BackendDAE::Variables;
            let mut newVars: BackendDAE::Variables;
            let mut cref = cref.clone();
            (localKnowns, newVars) = inTpl;
            newVars = BackendVariable::deleteVar(cref.clone(), newVars)?;
            localKnowns = BackendVariable::addVar(inVar.clone(), localKnowns)?;
            cref = ComponentReference::crefPrefixDer(cref.clone());
            newVar = BackendVariable::copyVarNewName(cref.clone(), inVar);
            newVar = BackendVariable::setVarKind(newVar, openmodelica_backend_types::BackendDAE::VarKind::STATE_DER)?;
            newVars = BackendVariable::addVar(newVar, newVars)?;
            outTpl = (localKnowns, newVars);
            ()
        },
        _ => {
            ()
        },
    });
    Ok((outVar, outTpl))
}

// =============================================================================
// replace expression with rewritten expression
//
// =============================================================================
pub fn applyRewriteRulesBackend(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE, (std::sync::Arc::new(fnptr!(applyRewriteRulesBackend0, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool)> + 'static>), false)?;
    outDAE = applyRewriteRulesBackendShared(outDAE)?;
    Ok(outDAE)
}

fn applyRewriteRulesBackend0(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inChanged: bool) -> (Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool) {
    let mut osyst: Arc<BackendDAE::EqSystem> = isyst.clone();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outChanged: bool;
    match '__try0: {
        unwrap_break_err!(BackendDAEUtil::traverseBackendDAEExpsVarsWithUpdate(isyst.orderedVars.clone(), (std::sync::Arc::new(traverserapplyRewriteRulesBackend) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false), '__try0);
        unwrap_break_err!(BackendDAEUtil::traverseBackendDAEExpsEqns(isyst.orderedEqs.clone(), (std::sync::Arc::new(traverserapplyRewriteRulesBackend) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false), '__try0);
        unwrap_break_err!(BackendDAEUtil::traverseBackendDAEExpsEqns(isyst.removedEqs.clone(), (std::sync::Arc::new(traverserapplyRewriteRulesBackend) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false), '__try0);
        outChanged = true;
        Ok::<_, anyhow::Error>((outChanged.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outChanged = __try0_o0;
        }
        Err(_) => {
            outChanged = false;
        }
    }
    (osyst, outShared, outChanged)
}

fn traverserapplyRewriteRulesBackend(mut inExp: Arc<DAE::Exp>, mut inB: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outB: bool;
    (outExp, outB) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new(fnptr!(traverserExpapplyRewriteRulesBackend, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), inB)?;
    Ok((outExp, outB))
}

fn traverserExpapplyRewriteRulesBackend(mut inExp: Arc<DAE::Exp>, mut inB: bool) -> (Arc<DAE::Exp>, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outB: bool;
    (outExp, outB) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e => {
                    let mut e = (*e).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(RewriteRules::rewriteBackEnd(e.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    e = __pa0.clone();
                    Ok((e.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inB))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outB)
}

fn applyRewriteRulesBackendShared(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut shared: Arc<BackendDAE::Shared>;
    shared = inDAE.shared.clone();
    BackendDAEUtil::traverseBackendDAEExpsVarsWithUpdate(shared.globalKnownVars.clone(), (std::sync::Arc::new(traverserapplyRewriteRulesBackend) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(shared.initialEqs.clone(), (std::sync::Arc::new(traverserapplyRewriteRulesBackend) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(shared.removedEqs.clone(), (std::sync::Arc::new(traverserapplyRewriteRulesBackend) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
    outDAE = Arc::new(BackendDAE::BackendDAE { eqs: inDAE.eqs.clone(), shared: shared });
    Ok(outDAE)
}

// =============================================================================
// generates a list with all iteration variables
//
// =============================================================================
pub(crate) fn listAllIterationVariables(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<()> {
    let mut backendDAEType: BackendDAE::BackendDAEType;
    let mut warnings: Arc<metamodelica::List<ArcStr>>;
    let __pa0 = ::match_deref::match_deref! { match &(inBackendDAE.clone()) {
        Deref @ BackendDAE::BackendDAE { shared: Deref @ BackendDAE::Shared { backendDAEType: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    backendDAEType = __pa0.clone();
    (warnings, _) = listAllIterationVariables0(inBackendDAE.eqs.clone())?;
    Error::addCompilerNotification(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("List of all iteration variables (DAE kind: ")); __mm_s.push_str(&*BackendDump::printBackendDAEType2String(backendDAEType)?); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*stringDelimitList(warnings, (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

pub(crate) fn listAllIterationVariables0(mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outWarnings: Arc<metamodelica::List<ArcStr>>;
    let mut outComponentRef: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut warnings: Arc<metamodelica::List<ArcStr>>;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut warnings_accum: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut crefs_accum: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
    for mut eq in &*inEqs {
        let mut eq = eq.clone();
        (warnings, crefs) = listAllIterationVariables1(eq.clone())?;
        warnings_accum = metamodelica::cons(warnings.clone(), warnings_accum.clone());
        crefs_accum = metamodelica::cons(crefs.clone(), crefs_accum.clone());
    }
    outWarnings = List::flattenReverse(warnings_accum)?;
    outComponentRef = List::flattenReverse(crefs_accum)?;
    Ok((outWarnings, outComponentRef))
}

fn listAllIterationVariables1(mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outWarning: Arc<metamodelica::List<ArcStr>>;
    let mut outComponentRef: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut vars: BackendDAE::Variables;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inEqSystem) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa1, .. }, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    vars = __pa0.clone();
    comps = __pa1.clone();
    (outWarning, outComponentRef) = listAllIterationVariables2(comps, vars)?;
    Ok((outWarning, outComponentRef))
}

fn listAllIterationVariables2(mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut vars: BackendDAE::Variables) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut warnings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut componentRefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut var_idxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut var_idxs2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let NONLINEAR_SYSTEM: ArcStr = literal!("Iteration variables of nonlinear equation system:\n");
    let ANALYTIC_JACOBIAN: ArcStr = literal!("Iteration variables of equation system with analytic Jacobian:\n");
    let NO_ANALYTIC_JACOBIAN: ArcStr = literal!("Iteration variables of equation system without analytic Jacobian:\n");
    let TORN_LINEAR: ArcStr = literal!("Iteration variables of torn linear equation system:\n");
    let TORN_NONLINEAR: ArcStr = literal!("Iteration variables of torn nonlinear equation system:\n");
    for mut comp in &*comps.reverse() {
        let mut comp = comp.clone();
        (warnings, componentRefs) = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_NONLINEAR { .. }, .. } => listAllIterationVariables3(var_field!((*comp).vars, BackendDAE::StrongComponent::EQUATIONSYSTEM).clone(), vars.clone(), (NONLINEAR_SYSTEM.clone()).clone(), warnings.clone(), componentRefs.clone())?,
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_GENERIC { .. }, .. } => listAllIterationVariables3(var_field!((*comp).vars, BackendDAE::StrongComponent::EQUATIONSYSTEM).clone(), vars.clone(), (ANALYTIC_JACOBIAN.clone()).clone(), warnings.clone(), componentRefs.clone())?,
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jacType: BackendDAE::JacobianType::JAC_NO_ANALYTIC { .. }, .. } => listAllIterationVariables3(var_field!((*comp).vars, BackendDAE::StrongComponent::EQUATIONSYSTEM).clone(), vars.clone(), (NO_ANALYTIC_JACOBIAN.clone()).clone(), warnings.clone(), componentRefs.clone())?,
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: __esc_var_idxs, .. }, casualTearingSet: None, .. } => {
            var_idxs = (*__esc_var_idxs).clone();
            listAllIterationVariables3(var_idxs.clone(), vars.clone(), (if (var_field!((*comp).linear, BackendDAE::StrongComponent::TORNSYSTEM).clone()) {TORN_LINEAR.clone()} else {TORN_NONLINEAR.clone()}).clone(), warnings.clone(), componentRefs.clone())?
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: __esc_var_idxs, .. }, casualTearingSet: Some(BackendDAE::TearingSet { tearingvars: __esc_var_idxs2, .. }), .. } => {
            var_idxs = (*__esc_var_idxs).clone();
            var_idxs2 = (*__esc_var_idxs2).clone();
            listAllIterationVariables3(List::union(var_idxs.clone(), var_idxs2.clone()), vars.clone(), (if (var_field!((*comp).linear, BackendDAE::StrongComponent::TORNSYSTEM).clone()) {TORN_LINEAR.clone()} else {TORN_NONLINEAR.clone()}).clone(), warnings.clone(), componentRefs.clone())?
        },
        _ => (warnings.clone(), componentRefs.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((warnings, componentRefs))
}

fn listAllIterationVariables3(mut varIndices: Arc<metamodelica::List<i32>>, mut allVars: BackendDAE::Variables, mut message: ArcStr, mut warnings: Arc<metamodelica::List<ArcStr>>, mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut warnings: Arc<metamodelica::List<ArcStr>> = warnings;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = crefs;
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>>;
    if !(varIndices.clone().is_empty()) {
        vars = ({
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut v in (varIndices).into_iter().cloned() {
            let __x = BackendVariable::getVarAt(allVars.clone(), v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        crefs = List::append_reverse(({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut v in (vars.clone()).into_iter().cloned() {
            let __x = BackendVariable::varCref(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), crefs);
        warnings = metamodelica::cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*message); __mm_s.push_str(&*warnAboutVars(vars)?); ArcStr::from(__mm_s) }).clone(), warnings);
    }
    Ok((warnings, crefs))
}

fn warnAboutVars(mut vars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (vars).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*BackendDump::varString(v.clone())?); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!("\n")).clone());
    Ok(r#str)
}

pub(crate) fn addTimeAsState(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut eq: Arc<BackendDAE::EqSystem>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut orderedVars: BackendDAE::Variables;
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut var: BackendDAE::Var;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendDAEUtil::mapEqSystemAndFold(inDAE, (std::sync::Arc::new(fnptr!(addTimeAsState1, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> + 'static>), 0)?) {
        (Deref @ BackendDAE::BackendDAE { eqs: __pa0, shared: __pa1 }, _) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqs = __pa0.clone();
    shared = __pa1.clone();
    orderedVars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    var = BackendDAE::Var { varName: DAE::crefTimeState().clone(), varKind: BackendDAE::VarKind::STATE { index: 1, derName: None, natural: true }, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false };
    var = BackendVariable::setVarFixed(var, true)?;
    var = BackendVariable::setVarStartValue(var, Arc::new(DAE::Exp::CREF { componentRef: DAE::crefTime().clone(), ty: DAE::T_REAL_DEFAULT().clone() }))?;
    orderedVars = BackendVariable::addVar(var, orderedVars)?;
    orderedEqs = BackendEquation::emptyEqnsSized(1);
    orderedEqs = BackendEquation::add(Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: DAE::crefTimeState().clone(), ty: DAE::T_REAL_DEFAULT().clone() })], attr: DAE::callAttrBuiltinReal().clone() }), scalar: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), orderedEqs)?;
    eq = BackendDAEUtil::createEqSystem(orderedVars, orderedEqs, metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::CONTINUOUS_TIME_PARTITION, BackendEquation::emptyEqns());
    outDAE = Arc::new(BackendDAE::BackendDAE { eqs: metamodelica::cons(eq, eqs), shared: shared });
    Ok(outDAE)
}

fn addTimeAsState1(mut inSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inFoo: i32) -> (Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32) {
    let mut outSystem: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outFoo: i32 = inFoo;
    outSystem = 'mc: {
        let __mc_input = inSystem.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { orderedEqs, .. } => {
                    BackendEquation::traverseEquationArray_WithUpdate(orderedEqs.clone(), (std::sync::Arc::new(addTimeAsState2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, i32) -> Result<(Arc<BackendDAE::Equation>, i32)> + 'static>), inFoo)?;
                    Ok(syst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inSystem.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outSystem, outShared, outFoo)
}

fn addTimeAsState2(mut inEq: Arc<BackendDAE::Equation>, mut inFoo: i32) -> Result<(Arc<BackendDAE::Equation>, i32)> {
    let mut outEq: Arc<BackendDAE::Equation>;
    let mut outFoo: i32 = inFoo;
    (outEq, _) = BackendEquation::traverseExpsOfEquation(inEq, (std::sync::Arc::new(addTimeAsState3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, i32) -> Result<(Arc<DAE::Exp>, i32)> + 'static>), inFoo)?;
    Ok((outEq, outFoo))
}

fn addTimeAsState3(mut inExp: Arc<DAE::Exp>, mut inTuple: i32) -> Result<(Arc<DAE::Exp>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: i32;
    (outExp, outTuple) = Expression::traverseExpTopDown(inExp, (std::sync::Arc::new(fnptr!(addTimeAsState4, Arc<DAE::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, i32) -> Result<(Arc<DAE::Exp>, bool, i32)> + 'static>), inTuple)?;
    Ok((outExp, outTuple))
}

fn addTimeAsState4(mut inExp: Arc<DAE::Exp>, mut inTuple: i32) -> (Arc<DAE::Exp>, bool, i32) {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = true;
    let mut outTuple: i32 = inTuple;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }, ty } => {
            Arc::new(DAE::Exp::CREF { componentRef: DAE::crefTimeState().clone(), ty: ty.clone() })
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outTuple)
}

//-------------------------------------
//Evaluate Output Variables Only.
//-------------------------------------
pub(crate) fn evaluateOutputsOnly(mut daeIn: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut daeOut: Arc<BackendDAE::BackendDAE>;
    let mut size: i32;
    let mut nVars: i32;
    let mut nEqs: i32;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut varVisited: metamodelica::Array<i32>;
    let mut outputVarIndxs: Arc<metamodelica::List<i32>>;
    let mut stateIndxs: Arc<metamodelica::List<i32>>;
    let mut stateTasks: Arc<metamodelica::List<i32>>;
    let mut stateTasks1: Arc<metamodelica::List<i32>>;
    let mut outputTasks: Arc<metamodelica::List<i32>>;
    let mut predecessors: Arc<metamodelica::List<i32>>;
    let mut tasks: Arc<metamodelica::List<i32>>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut compsNew: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut addComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut comp: Arc<BackendDAE::StrongComponent> = Arc::new(<BackendDAE::StrongComponent as ::std::default::Default>::default());
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut systsNew: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut eq: Arc<BackendDAE::Equation>;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut matching: Arc<BackendDAE::Matching>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut vars: BackendDAE::Variables;
    let mut funcTree: Arc<AvlTreePathFunction::Tree>;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut eqLstNew: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut varLstNew: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut states: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskGraphData: HpcOmTaskGraph::TaskGraphMeta;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut mapEqnIncRow: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapIncRowEqn: metamodelica::Array<i32>;
    let mut systemNumber: i32 = 0;
    let mut numberOfSystems: i32;
    let mut eqIndLst: Arc<metamodelica::List<i32>>;
    let mut eqIndexLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut der_replacement: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Exp>>>;
    let mut derVar: BackendDAE::Var;
    let debug: bool = false;
    daeOut = daeIn.clone();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(daeIn) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    let __pa2 = ::match_deref::match_deref! { match &(shared.clone()) {
        Deref @ BackendDAE::Shared { functionTree: __pa2, .. } => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    funcTree = __pa2.clone();
    systsNew = metamodelica::nil();
    numberOfSystems = (systs.clone().len() as i32);
    for mut syst in &*systs {
        let mut syst = syst.clone();
        systemNumber = systemNumber + 1;
        let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedVars: __pa3, orderedEqs: __pa4, matching: __pa5, .. } => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        vars = __pa3.clone();
        eqs = __pa4.clone();
        matching = __pa5.clone();
        let (__pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(matching.clone()) {
            Deref @ BackendDAE::Matching::MATCHING { ass1: __pa6, ass2: __pa7, comps: __pa8 } => (__pa6.clone(), __pa7.clone(), __pa8.clone()),
            _ => bail!("pattern mismatch"),
        } };
        ass1 = __pa6.clone();
        ass2 = __pa7.clone();
        comps = __pa8.clone();
        (taskGraph, taskGraphData) = HpcOmTaskGraph::getEmptyTaskGraph(0, 0, 0);
        (taskGraph, taskGraphData, _) = HpcOmTaskGraph::createTaskGraph0(syst.clone(), shared.clone(), false, (taskGraph.clone(), taskGraphData.clone(), 1))?;
        let HpcOmTaskGraph::TASKGRAPHMETA { varCompMapping: __pa9, eqCompMapping: __pa10, .. } = (taskGraphData.clone()) else { bail!("pattern mismatch") };
        varCompMapping = __pa9.clone();
        eqCompMapping = __pa10.clone();
        size = metamodelica::arrayLength(taskGraph.clone());
        taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(taskGraph.clone(), size)?;
        let __pa11 = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedVars: __pa11, .. } => __pa11.clone(),
            _ => bail!("pattern mismatch"),
        } };
        vars = __pa11.clone();
        varLst = BackendVariable::varList(vars.clone())?;
        varLst = List::filterOnTrue(varLst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isVarOnTopLevelAndOutput, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?;
        if !(varLst.clone().is_empty()) {
            outputVarIndxs = BackendVariable::getVarIndexFromVars(varLst.clone(), vars.clone());
            outputTasks = List::map(List::map1(outputVarIndxs.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), varCompMapping.clone())?, std::sync::Arc::new(fnptr!(Util::tuple31, _)))?;
            if debug {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("outputTasks ")); __mm_s.push_str(&*stringDelimitList(List::map(outputTasks.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            predecessors = HpcOmTaskGraph::getAllSuccessors(outputTasks.clone(), taskGraphT.clone())?;
            predecessors = List::sort(predecessors.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            compsNew = List::map1(listAppend(outputTasks.clone(), predecessors.clone()), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), comps.clone())?;
            if debug {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("predecessors of outputs ")); __mm_s.push_str(&*stringDelimitList(List::map(predecessors.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            eqLstNew = BackendDAEUtil::getStrongComponentEquations(compsNew.clone(), eqs.clone(), vars.clone())?;
            stateTasks = metamodelica::nil();
            varVisited = arrayCreate(BackendVariable::varsSize(vars.clone()), -1);
            while !(eqLstNew.clone().is_empty()) {
                let (__pa12, __pa13) = ::match_deref::match_deref! { match &(eqLstNew.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa12, tail: __pa13 } => (__pa12.clone(), __pa13.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                eq = __pa12.clone();
                eqLstNew = __pa13.clone();
                if debug {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("eq: ")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
                crefs = BackendEquation::equationCrefs(eq.clone())?;
                crefs = List::filter1OnTrue(crefs.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isState, Arc<DAE::ComponentRef>, BackendDAE::Variables)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, BackendDAE::Variables) -> Result<bool> + 'static>), vars.clone())?;
                (states, stateIndxs) = BackendVariable::getVarLst(crefs.clone(), vars.clone());
                (stateIndxs, states) = List::filter1OnTrueSync(stateIndxs.clone(), (std::sync::Arc::new(stateVarIsNotVisited) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), varVisited.clone(), states.clone())?;
                if !(stateIndxs.clone().is_empty()) {
                    if debug {
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("states ")); __mm_s.push_str(&*stringDelimitList(List::map(states.clone(), (std::sync::Arc::new(BackendDump::varString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<ArcStr> + 'static>))?, (literal!("\n ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    List::map2_0(stateIndxs.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), 1, varVisited.clone())?;
                    stateTasks1 = List::map(List::map1(stateIndxs.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), varCompMapping.clone())?, std::sync::Arc::new(fnptr!(Util::tuple31, _)))?;
                    stateTasks = List::append_reverse(stateTasks1.clone(), stateTasks.clone());
                    predecessors = HpcOmTaskGraph::getAllSuccessors(stateTasks1.clone(), taskGraphT.clone())?;
                    addComps = List::map1(listAppend(stateTasks1.clone(), predecessors.clone()), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), comps.clone())?;
                    eqLstNew = listAppend(BackendDAEUtil::getStrongComponentEquations(addComps.clone(), eqs.clone(), vars.clone())?, eqLstNew.clone());
                }
            }
            stateTasks = Dangerous::listReverseInPlace(stateTasks.clone());
            predecessors = HpcOmTaskGraph::getAllSuccessors(listAppend(outputTasks.clone(), stateTasks.clone()), taskGraphT.clone())?;
            tasks = List::sort(listAppend(predecessors.clone(), listAppend(outputTasks.clone(), stateTasks.clone())), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            if debug {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("predecessors of outputs and states ")); __mm_s.push_str(&*stringDelimitList(List::map(tasks.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            compsNew = List::map1(tasks.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), comps.clone())?;
            compsNew = List::unique(compsNew.clone());
            if debug {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("There have been ")); __mm_s.push_str(&*intString((comps.clone().len() as i32))); __mm_s.push_str(&*literal!(" SCCs and now there are ")); __mm_s.push_str(&*intString((compsNew.clone().len() as i32))); __mm_s.push_str(&*literal!(" SCCs.\n")); ArcStr::from(__mm_s) }).clone());
            }
            eqLstNew = metamodelica::nil();
            varLstNew = metamodelica::nil();
            for mut comp in &*compsNew.clone() {
                let mut comp = comp.clone();
                (varLst, _, eqLst, eqIndLst) = BackendDAEUtil::getStrongComponentVarsAndEquations(comp.clone(), vars.clone(), eqs.clone())?;
                varLstNew = listAppend(varLst.clone(), varLstNew.clone());
                eqLstNew = listAppend(eqLst.clone(), eqLstNew.clone());
                eqIndexLst = listAppend(eqIndLst.clone(), eqIndexLst.clone());
            }
            assign_field!(
                syst.orderedVars = BackendVariable::listVar1(varLstNew.clone().reverse())?,
                syst.orderedEqs = BackendEquation::listEquation(eqLstNew.clone().reverse())?,
                syst.m = None,
                syst.mT = None,
                syst.matching = openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING()
            );
            (m, mT) = BackendDAEUtil::adjacencyMatrix(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
            assign_field!(
                syst.m = Some(m.clone()),
                syst.mT = Some(mT.clone())
            );
            nVars = (varLstNew.clone().len() as i32);
            nEqs = (eqLstNew.clone().len() as i32);
            ass1 = arrayCreate(nVars, -1);
            ass2 = arrayCreate(nEqs, -1);
            Matching::matchingExternalsetAdjacencyMatrix(nVars, nEqs, m.clone());
            BackendDAEEXT::matching(nVars, nEqs, 5, -1, metamodelica::OrderedFloat(0.0_f64), 1);
            BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
            matching = Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: compsNew.clone() });
            assign_field!(syst.matching = matching.clone());
            (syst, _, _, mapEqnIncRow, mapIncRowEqn) = BackendDAEUtil::getAdjacencyMatrixScalar(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(funcTree.clone()), BackendDAEUtil::isInitializationDAE(shared.clone()))?;
            (syst, _) = BackendDAETransform::strongComponentsScalar(syst.clone(), shared.clone(), mapEqnIncRow.clone(), mapIncRowEqn.clone())?;
            assign_field!(syst.removedEqs = BackendEquation::emptyEqns());
            systsNew = metamodelica::cons(syst.clone(), systsNew.clone());
            vars = BackendVariable::deleteVars(syst.orderedVars.clone(), vars.clone());
            eqs = BackendEquation::deleteList(eqs.clone(), eqIndexLst.clone())?;
        } else {
            if debug {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("No output variables in this system (")); __mm_s.push_str(&*intString(systemNumber)); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(numberOfSystems)); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
            }
        }
        der_replacement = UnorderedMap::new((std::sync::Arc::new(ComponentReferenceBasics::hashComponentRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), 1);
        for mut state in &*BackendVariable::varList(vars.clone())? {
            let mut state = state.clone();
            if BackendVariable::isStateVar(state.clone()) {
                derVar = BackendVariable::makeVar(ComponentReference::prependStringCref((literal!("$DER_REM_")).clone(), state.varName.clone())?)?;
                UnorderedMap::add(state.varName.clone(), Expression::crefExp(derVar.varName.clone())?, der_replacement.clone())?;
                vars = BackendVariable::addVar(derVar.clone(), vars.clone())?;
            }
        }
        (eqs, _) = BackendEquation::traverseEquationArray_WithUpdate(eqs.clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static> = (std::sync::Arc::new(replaceDerCallOutputsOnly) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Exp>>>) -> Result<(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Exp>>>)> + 'static>); move |__pe_a0, __pe_a2| BackendEquation::traverseExpsOfEquation(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, _) -> Result<_> + 'static>), der_replacement.clone())?;
        (vars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(vars.clone(), (std::sync::Arc::new(BackendVariable::makeParamOutputsOnly) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, bool) -> Result<(BackendDAE::Var, bool)> + 'static>), false)?;
        (eqs, _) = BackendEquation::traverseEquationArray_WithUpdate(eqs.clone(), (std::sync::Arc::new(BackendEquation::setEquationKind) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, BackendDAE::EquationKind) -> Result<(Arc<BackendDAE::Equation>, BackendDAE::EquationKind)> + 'static>), openmodelica_backend_types::BackendDAE::EquationKind::INITIAL_EQUATION)?;
        assign_field!(
            shared.globalKnownVars = BackendVariable::addVariables(vars.clone(), shared.globalKnownVars.clone())?,
            shared.initialEqs = BackendEquation::addList(BackendEquation::equationList(eqs.clone())?, shared.initialEqs.clone())?
        );
    }
    assign_field!(shared.aliasVars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()));
    daeOut = Arc::new(BackendDAE::BackendDAE { eqs: systsNew, shared: shared });
    Ok(daeOut)
}

fn stateVarIsNotVisited(mut idx: i32, mut varArr: metamodelica::Array<i32>) -> Result<bool> {
    let mut b: bool;
    b = intLt(metamodelica::arrayGet(varArr.clone(), idx)?, 0);
    Ok(b)
}

fn replaceDerCallOutputsOnly(mut exp: Arc<DAE::Exp>, mut der_replacement: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Exp>>>) -> Result<(Arc<DAE::Exp>, Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Exp>>>)> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut der_replacement: Arc<UnorderedMap::UnorderedMap<Arc<DAE::ComponentRef>, Arc<DAE::Exp>>> = der_replacement;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            UnorderedMap::getOrDefault(cr.clone(), der_replacement.clone(), exp)?
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, der_replacement))
}

// =============================================================================
// section for initOptModule >>inlineHomotopy<<
//
// =============================================================================
pub(crate) fn inlineHomotopy(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut foundHomotopy: bool;
    for mut syst in &*inDAE.eqs.clone() {
        let mut syst = syst.clone();
        orderedEqs = syst.orderedEqs.clone();
        (orderedEqs, foundHomotopy) = BackendEquation::traverseEquationArray_WithUpdate(orderedEqs.clone(), (std::sync::Arc::new(inlineHomotopy2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, bool) -> Result<(Arc<BackendDAE::Equation>, bool)> + 'static>), false)?;
        assign_field!(syst.orderedEqs = orderedEqs.clone());
    }
    Ok(outDAE)
}

fn inlineHomotopy2(mut inEq: Arc<BackendDAE::Equation>, mut inFoundHomotopy: bool) -> Result<(Arc<BackendDAE::Equation>, bool)> {
    let mut outEq: Arc<BackendDAE::Equation>;
    let mut outFoundHomotopy: bool = inFoundHomotopy;
    (outEq, outFoundHomotopy) = BackendEquation::traverseExpsOfEquation(inEq, (std::sync::Arc::new(inlineHomotopy3) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), inFoundHomotopy)?;
    Ok((outEq, outFoundHomotopy))
}

fn inlineHomotopy3(mut inExp: Arc<DAE::Exp>, mut inFoundHomotopy: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outFoundHomotopy: bool = inFoundHomotopy;
    (outExp, outFoundHomotopy) = Expression::traverseExpTopDown(inExp, (std::sync::Arc::new(replaceHomotopyWithLambdaExpression) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), inFoundHomotopy)?;
    Ok((outExp, outFoundHomotopy))
}

fn replaceHomotopyWithLambdaExpression(mut inExp: Arc<DAE::Exp>, mut inFoundHomotopy: bool) -> Result<(Arc<DAE::Exp>, bool, bool)> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut cont: bool = true;
    let mut outFoundHomotopy: bool;
    outFoundHomotopy = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, expLst: Deref @ metamodelica::List::Cons { head: actual, tail: Deref @ metamodelica::List::Cons { head: simplified, tail: Deref @ metamodelica::List::Nil } }, .. } => {
            let mut lambda: Arc<DAE::Exp>;
            lambda = Expression::crefExp(ComponentReferenceBasics::makeCrefIdent((arcstr::literal!(BackendDAE::homotopyLambda)).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil()))?;
            outExp = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: simplified.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: lambda.clone() }) }), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::BINARY { exp1: actual.clone(), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: lambda }) });
            true
        },
        _ => {
            inFoundHomotopy
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outFoundHomotopy))
}

// =============================================================================
// section for initOptModule >>generateHomotopyComponents<<
//
// =============================================================================
pub(crate) fn generateHomotopyComponents(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut newEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    if Config::adaptiveHomotopy()? {
        for mut syst in &*outDAE.eqs.clone() {
            let mut syst = syst.clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(syst.matching.clone()) {
                Deref @ BackendDAE::Matching::MATCHING { ass1: __pa0, ass2: __pa1, comps: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            ass1 = __pa0.clone();
            ass2 = __pa1.clone();
            comps = __pa2.clone();
            if Config::globalHomotopy()? {
                (comps, syst) = traverseStrongComponentsForHomotopyLoop(comps.clone(), syst.clone())?;
            } else {
                (comps, syst) = traverseStrongComponentsAddLambda(comps.clone(), syst.clone())?;
            }
            assign_field!(syst.matching = Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1.clone(), ass2: ass2.clone(), comps: comps.clone() }));
            newEqSystems = metamodelica::cons(syst.clone(), newEqSystems.clone());
        }
        assign_field!(outDAE.eqs = newEqSystems.reverse());
    } else {
        Error::addCompilerWarning((literal!("InitOptModule generateHomotopyComponents is activated for an equidistant homotopy method and will therefore be ignored.")).clone())?;
    }
    Ok(outDAE)
}

fn traverseStrongComponentsForHomotopyLoop(mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut system: Arc<BackendDAE::EqSystem>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<BackendDAE::EqSystem>)> {
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = comps;
    let mut system: Arc<BackendDAE::EqSystem> = system;
    let mut nComps: i32;
    let mut compIndex: i32 = 0;
    let mut homotopyLoopBeginning: i32 = 0;
    let mut homotopyLoopEnd: i32 = 0;
    let mut preHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut homotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut postHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut homotopyComponent: Arc<BackendDAE::StrongComponent>;
    let mut lambda: BackendDAE::Var;
    let mut lambdaIdx: i32;
    nComps = (comps.clone().len() as i32);
    for mut comp in &*comps.clone() {
        let mut comp = comp.clone();
        compIndex = compIndex + 1;
        let () = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqnIndex, .. } => {
            let mut eqn: Arc<BackendDAE::Equation>;
            let mut hasHomotopy: bool;
            eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone())?;
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
            if hasHomotopy {
                homotopyLoopEnd = compIndex;
                if homotopyLoopBeginning == 0 {
                    homotopyLoopBeginning = compIndex;
                }
            }
            ()
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqnIndexes, .. } => {
            let mut eqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut hasHomotopy: bool;
            if homotopyLoopBeginning == 0 {
                eqnLst = BackendEquation::getList(eqnIndexes.clone(), system.orderedEqs.clone())?;
                (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(eqnLst.clone(), (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
                if hasHomotopy {
                    homotopyLoopBeginning = compIndex;
                    homotopyLoopEnd = compIndex;
                }
            } else {
                homotopyLoopEnd = compIndex;
            }
            ()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: eqnIndex, .. } => {
            let mut eqn: Arc<BackendDAE::Equation>;
            let mut hasHomotopy: bool;
            eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone())?;
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
            if hasHomotopy {
                homotopyLoopEnd = compIndex;
                if homotopyLoopBeginning == 0 {
                    homotopyLoopBeginning = compIndex;
                }
            }
            ()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: eqnIndex, .. } => {
            let mut eqn: Arc<BackendDAE::Equation>;
            let mut hasHomotopy: bool;
            eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone())?;
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
            if hasHomotopy {
                homotopyLoopEnd = compIndex;
                if homotopyLoopBeginning == 0 {
                    homotopyLoopBeginning = compIndex;
                }
            }
            ()
        },
        Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: eqnIndex, .. } => {
            let mut eqn: Arc<BackendDAE::Equation>;
            let mut hasHomotopy: bool;
            eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone())?;
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
            if hasHomotopy {
                homotopyLoopEnd = compIndex;
                if homotopyLoopBeginning == 0 {
                    homotopyLoopBeginning = compIndex;
                }
            }
            ()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: eqnIndex, .. } => {
            let mut eqn: Arc<BackendDAE::Equation>;
            let mut hasHomotopy: bool;
            eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone())?;
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
            if hasHomotopy {
                homotopyLoopEnd = compIndex;
                if homotopyLoopBeginning == 0 {
                    homotopyLoopBeginning = compIndex;
                }
            }
            ()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: eqnIndex, .. } => {
            let mut eqn: Arc<BackendDAE::Equation>;
            let mut hasHomotopy: bool;
            eqn = BackendEquation::get(system.orderedEqs.clone(), eqnIndex.clone())?;
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquation(eqn.clone(), (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
            if hasHomotopy {
                homotopyLoopEnd = compIndex;
                if homotopyLoopBeginning == 0 {
                    homotopyLoopBeginning = compIndex;
                }
            }
            ()
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: resEqnIndexes, innerEquations, .. }, .. } => {
            let mut innerEqnIndexes: Arc<metamodelica::List<i32>>;
            let mut eqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut hasHomotopy: bool;
            if homotopyLoopBeginning == 0 {
                eqnLst = BackendEquation::getList(resEqnIndexes.clone(), system.orderedEqs.clone())?;
                (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(eqnLst.clone(), (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
                if !(hasHomotopy) {
                    (innerEqnIndexes, _, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>))?;
                    eqnLst = BackendEquation::getList(innerEqnIndexes.clone(), system.orderedEqs.clone())?;
                    (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(eqnLst.clone(), (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
                }
                if hasHomotopy {
                    homotopyLoopBeginning = compIndex;
                    homotopyLoopEnd = compIndex;
                }
            } else {
                homotopyLoopEnd = compIndex;
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    if homotopyLoopBeginning > 0 {
        lambda = BackendDAE::Var { varName: ComponentReferenceBasics::makeCrefIdent((arcstr::literal!(BackendDAE::homotopyLambda)).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil()), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false };
        assign_field!(system.orderedVars = BackendVariable::addVar(lambda, system.orderedVars.clone())?);
        lambdaIdx = BackendVariable::varsSize(system.orderedVars.clone());
        (preHomotopyComponents, homotopyComponents, postHomotopyComponents) = getHomotopyComponents(List::intRange(nComps), comps, homotopyLoopBeginning, homotopyLoopEnd, metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
        homotopyComponent = createOneHomotopyComponent(homotopyComponents, system.clone(), lambdaIdx)?;
        comps = metamodelica::cons(homotopyComponent, postHomotopyComponents);
        comps = listAppend(preHomotopyComponents, comps);
    }
    Ok((comps, system))
}

fn getHomotopyComponents(mut componentIndexes: Arc<metamodelica::List<i32>>, mut components: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut homotopyLoopBeginning: i32, mut homotopyLoopEnd: i32, mut outPreHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut outHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut outPostHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>)> {
    let mut outPreHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = outPreHomotopyComponents;
    let mut outHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = outHomotopyComponents;
    let mut outPostHomotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = outPostHomotopyComponents;
    (outPreHomotopyComponents, outHomotopyComponents, outPostHomotopyComponents) = (::match_deref::match_deref! { match &((componentIndexes, components)) {
        (Deref @ metamodelica::List::Cons { head: i, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: comp, tail: Deref @ metamodelica::List::Nil }) => {
            if intLt(i.clone(), homotopyLoopBeginning) {
                outPreHomotopyComponents = metamodelica::cons(comp.clone(), outPreHomotopyComponents);
            } else if intGt(i.clone(), homotopyLoopEnd) {
                outPostHomotopyComponents = metamodelica::cons(comp.clone(), outPostHomotopyComponents);
            } else {
                outHomotopyComponents = metamodelica::cons(comp.clone(), outHomotopyComponents);
            }
            (outPreHomotopyComponents.reverse(), outHomotopyComponents.reverse(), outPostHomotopyComponents.reverse())
        },
        (Deref @ metamodelica::List::Cons { head: i, tail: indexes }, Deref @ metamodelica::List::Cons { head: comp, tail: comps }) => {
            if intLt(i.clone(), homotopyLoopBeginning) {
                outPreHomotopyComponents = metamodelica::cons(comp.clone(), outPreHomotopyComponents);
            } else if intGt(i.clone(), homotopyLoopEnd) {
                outPostHomotopyComponents = metamodelica::cons(comp.clone(), outPostHomotopyComponents);
            } else {
                outHomotopyComponents = metamodelica::cons(comp.clone(), outHomotopyComponents);
            }
            getHomotopyComponents(indexes.clone(), comps.clone(), homotopyLoopBeginning, homotopyLoopEnd, outPreHomotopyComponents, outHomotopyComponents, outPostHomotopyComponents)?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outPreHomotopyComponents, outHomotopyComponents, outPostHomotopyComponents))
}

fn createOneHomotopyComponent(mut homotopyComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut inSystem: Arc<BackendDAE::EqSystem>, mut lambdaIdx: i32) -> Result<Arc<BackendDAE::StrongComponent>> {
    let mut outHomotopyComponent: Arc<BackendDAE::StrongComponent>;
    let mut newInnerEquations: Arc<metamodelica::List<BackendDAE::InnerEquation>> = metamodelica::nil();
    let mut newResEquations: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut newIterationVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut isMixed: bool = false;
    for mut comp in &*homotopyComponents {
        let mut comp = comp.clone();
        (newInnerEquations, newResEquations, newIterationVars) = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqnIndex, var: varIndex } => {
            let mut newInnerEquation: BackendDAE::InnerEquation;
            newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION { eqn: eqnIndex.clone(), vars: list![varIndex.clone()] };
            (metamodelica::cons(newInnerEquation.clone(), newInnerEquations.clone()), newResEquations.clone(), newIterationVars.clone())
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqnIndexes, vars: varIndexes, mixedSystem, .. } => {
            if mixedSystem.clone() {
                isMixed = true;
            }
            (newInnerEquations.clone(), listAppend(newResEquations.clone(), eqnIndexes.clone()), listAppend(newIterationVars.clone(), varIndexes.clone()))
        },
        Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: eqnIndex, vars: varIndexes } => {
            let mut newInnerEquation: BackendDAE::InnerEquation;
            newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION { eqn: eqnIndex.clone(), vars: varIndexes.clone() };
            (metamodelica::cons(newInnerEquation.clone(), newInnerEquations.clone()), newResEquations.clone(), newIterationVars.clone())
        },
        Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: eqnIndex, vars: varIndexes } => {
            let mut newInnerEquation: BackendDAE::InnerEquation;
            newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION { eqn: eqnIndex.clone(), vars: varIndexes.clone() };
            (metamodelica::cons(newInnerEquation.clone(), newInnerEquations.clone()), newResEquations.clone(), newIterationVars.clone())
        },
        Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: eqnIndex, vars: varIndexes } => {
            let mut newInnerEquation: BackendDAE::InnerEquation;
            newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION { eqn: eqnIndex.clone(), vars: varIndexes.clone() };
            (metamodelica::cons(newInnerEquation.clone(), newInnerEquations.clone()), newResEquations.clone(), newIterationVars.clone())
        },
        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: eqnIndex, vars: varIndexes } => {
            let mut newInnerEquation: BackendDAE::InnerEquation;
            newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION { eqn: eqnIndex.clone(), vars: varIndexes.clone() };
            (metamodelica::cons(newInnerEquation.clone(), newInnerEquations.clone()), newResEquations.clone(), newIterationVars.clone())
        },
        Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: eqnIndex, vars: varIndexes } => {
            let mut newInnerEquation: BackendDAE::InnerEquation;
            newInnerEquation = BackendDAE::InnerEquation::INNEREQUATION { eqn: eqnIndex.clone(), vars: varIndexes.clone() };
            (metamodelica::cons(newInnerEquation.clone(), newInnerEquations.clone()), newResEquations.clone(), newIterationVars.clone())
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: resEqnIndexes, tearingvars: tVarIndexes, innerEquations, .. }, mixedSystem, .. } => {
            if mixedSystem.clone() {
                isMixed = true;
            }
            for mut innerEquation in &*innerEquations.clone() {
                let mut innerEquation = innerEquation.clone();
                newInnerEquations = metamodelica::cons(innerEquation.clone(), newInnerEquations.clone());
            }
            (newInnerEquations.clone(), listAppend(newResEquations.clone(), resEqnIndexes.clone()), listAppend(newIterationVars.clone(), tVarIndexes.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    outHomotopyComponent = Arc::new(BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: listAppend(newIterationVars, list![lambdaIdx]), residualequations: newResEquations, innerEquations: newInnerEquations.reverse(), jac: openmodelica_backend_types::BackendDAE::Jacobian::interned_EMPTY_JACOBIAN() }, casualTearingSet: None, linear: false, mixedSystem: isMixed });
    Ok(outHomotopyComponent)
}

fn traverseStrongComponentsAddLambda(mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut system: Arc<BackendDAE::EqSystem>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<BackendDAE::EqSystem>)> {
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = comps;
    let mut system: Arc<BackendDAE::EqSystem> = system;
    let mut newComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut lambda: BackendDAE::Var;
    let mut lambdaIdx: i32;
    let mut hasAnyHomotopy: bool = false;
    lambdaIdx = BackendVariable::varsSize(system.orderedVars.clone()) + 1;
    for mut comp in &*comps {
        let mut comp = comp.clone();
        comp = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqnIndexes, vars: varIndexes, jac, jacType, mixedSystem } => {
            let mut eqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut hasHomotopy: bool;
            eqnLst = BackendEquation::getList(eqnIndexes.clone(), system.orderedEqs.clone())?;
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(eqnLst.clone(), (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
            if hasHomotopy {
                hasAnyHomotopy = true;
                comp = Arc::new(BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqnIndexes.clone(), vars: metamodelica::cons(lambdaIdx, varIndexes.clone()), jac: jac.clone(), jacType: jacType.clone(), mixedSystem: mixedSystem.clone() });
            }
            comp.clone()
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: resEqnIndexes, tearingvars: tVarIndexes, innerEquations, jac }, casualTearingSet, linear, mixedSystem } => {
            let mut innerEqnIndexes: Arc<metamodelica::List<i32>>;
            let mut eqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut hasHomotopy: bool;
            eqnLst = BackendEquation::getList(resEqnIndexes.clone(), system.orderedEqs.clone())?;
            (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(eqnLst.clone(), (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
            if !(hasHomotopy) {
                (innerEqnIndexes, _, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>))?;
                eqnLst = BackendEquation::getList(innerEqnIndexes.clone(), system.orderedEqs.clone())?;
                (_, hasHomotopy) = BackendEquation::traverseExpsOfEquationList(eqnLst.clone(), (std::sync::Arc::new(BackendDAEUtil::containsHomotopyCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
            }
            if hasHomotopy {
                hasAnyHomotopy = true;
                comp = Arc::new(BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: listAppend(tVarIndexes.clone(), list![lambdaIdx]), residualequations: resEqnIndexes.clone(), innerEquations: innerEquations.clone(), jac: jac.clone() }, casualTearingSet: casualTearingSet.clone(), linear: linear.clone(), mixedSystem: mixedSystem.clone() });
            }
            comp.clone()
        },
        _ => {
            comp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        newComps = metamodelica::cons(comp.clone(), newComps.clone());
    }
    if hasAnyHomotopy {
        lambda = BackendDAE::Var { varName: ComponentReferenceBasics::makeCrefIdent((arcstr::literal!(BackendDAE::homotopyLambda)).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil()), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: DAE::T_REAL_DEFAULT().clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: true, initNonlinear: false, encrypted: false };
        assign_field!(system.orderedVars = BackendVariable::addVar(lambda, system.orderedVars.clone())?);
    }
    comps = newComps.reverse();
    Ok((comps, system))
}

