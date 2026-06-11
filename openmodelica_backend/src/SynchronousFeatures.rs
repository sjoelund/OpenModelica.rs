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

use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_error::ErrorTypes;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::MMath;
use openmodelica_util::StringUtil;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// clock partitioning
//
// =============================================================================
pub(crate) fn clockPartitioning(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::List::Cons { head: syst, tail: Deref @ metamodelica::List::Nil }, shared } => {
            clockPartitioning1(syst.clone(), shared.clone())?
        },
        _ => {
            let mut syst: Arc<BackendDAE::EqSystem>;
            let mut shared: Arc<BackendDAE::Shared>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendDAEOptimize::collapseIndependentBlocks(inDAE)?) {
                Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            syst = __pa0.clone();
            shared = __pa1.clone();
            clockPartitioning1(syst.clone(), shared.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDAE)
}

pub(crate) fn synchronousFeatures(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut contSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut clockedSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    (clockedSysts, contSysts) = List::splitOnTrue(inDAE.eqs.clone(), (std::sync::Arc::new(fnptr!(BackendDAEUtil::isClockedSyst, Arc<BackendDAE::EqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<bool> + 'static>))?;
    if !(clockedSysts.clone().is_empty()) {
        shared = inDAE.shared.clone();
        (clockedSysts, shared) = treatClockedStates(clockedSysts, shared)?;
        systs = listAppend(contSysts, clockedSysts);
        outDAE = Arc::new(BackendDAE::BackendDAE { eqs: systs.clone(), shared: shared.clone() });
        if Flags::isSet(Flags::DUMP_SYNCHRONOUS.clone())? {
            metamodelica::print((literal!("synchronous features post-phase: synchronousFeatures\n\n")).clone());
            BackendDump::dumpEqSystems(systs, (literal!("clock partitioning")).clone())?;
            BackendDump::dumpBasePartitions(shared.partitionsInfo.basePartitions.clone(), (literal!("Base clocks")).clone())?;
            BackendDump::dumpSubPartitions(shared.partitionsInfo.subPartitions.clone(), (literal!("Sub clocks")).clone())?;
        }
    } else {
        outDAE = inDAE;
    }
    Ok(outDAE)
}

pub(crate) fn contPartitioning(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut clockedSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut clockedSysts1: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut unpartRemEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    (clockedSysts, systs) = List::splitOnTrue(inDAE.eqs.clone(), (std::sync::Arc::new(fnptr!(BackendDAEUtil::isClockedSyst, Arc<BackendDAE::EqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<bool> + 'static>))?;
    shared = inDAE.shared.clone();
    if !(systs.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendDAEOptimize::collapseIndependentBlocks(Arc::new(BackendDAE::BackendDAE { eqs: systs, shared: shared }))?) {
            Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        syst = __pa0.clone();
        shared = __pa1.clone();
        (systs, clockedSysts1, unpartRemEqs) = baseClockPartitioning(syst, shared.clone())?;
        assert!(clockedSysts1.is_empty(), "{}", &*(literal!("Get clocked system in SynchronousFeatures.addContVarsEqs")).clone());
        assign_field!(shared.removedEqs = BackendEquation::addList(unpartRemEqs, shared.removedEqs.clone())?);
    }
    outDAE = Arc::new(BackendDAE::BackendDAE { eqs: listAppend(systs, clockedSysts), shared: shared });
    Ok(outDAE)
}

fn clockPartitioning1(mut inSyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut contSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut clockedSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut shared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut holdComps: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut unpartRemEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    syst = substitutePartitionOpExps(inSyst, inShared)?;
    (contSysts, clockedSysts, unpartRemEqs) = baseClockPartitioning(syst, shared.clone())?;
    (contSysts, holdComps) = removeHoldExpsSyst(contSysts)?;
    (clockedSysts, shared) = subClockPartitioning1(clockedSysts, shared, holdComps)?;
    unpartRemEqs = createBoolClockWhenClauses(shared.clone(), unpartRemEqs);
    assign_field!(shared.removedEqs = BackendEquation::addList(unpartRemEqs, shared.removedEqs.clone())?);
    systs = listAppend(contSysts, clockedSysts.clone());
    outDAE = Arc::new(BackendDAE::BackendDAE { eqs: systs.clone(), shared: shared.clone() });
    if !(clockedSysts.is_empty()) {
        if Flags::isSet(Flags::DUMP_SYNCHRONOUS.clone())? {
            metamodelica::print((literal!("synchronous features pre-phase: synchronousFeatures\n\n")).clone());
            BackendDump::dumpEqSystems(systs, (literal!("clock partitioning")).clone())?;
            BackendDump::dumpBasePartitions(shared.partitionsInfo.basePartitions.clone(), (literal!("Base clocks")).clone())?;
            BackendDump::dumpSubPartitions(shared.partitionsInfo.subPartitions.clone(), (literal!("Sub clocks")).clone())?;
        }
    }
    Ok(outDAE)
}

fn createBoolClockWhenClauses(mut inShared: Arc<BackendDAE::Shared>, mut inRemovedEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Arc<metamodelica::List<Arc<BackendDAE::Equation>>> {
    let mut outRemovedEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inRemovedEqs.clone();
    let mut basePartition: BackendDAE::BasePartition;
    for mut i in 1..=metamodelica::arrayLength(inShared.partitionsInfo.basePartitions.clone()) {
        basePartition = ({let __elt = inShared.partitionsInfo.basePartitions.borrow()[(i.clone()-1) as usize].clone(); __elt});
        outRemovedEqs = (::match_deref::match_deref! { match &(basePartition.clock.clone()) {
        Deref @ DAE::ClockKind::EVENT_CLOCK { condition: c, startInterval: _ } => {
            let mut e: Arc<DAE::Exp>;
            let mut whenEq: Arc<BackendDAE::WhenEquation>;
            let mut eq: Arc<BackendDAE::Equation>;
            e = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$_clkfire")).clone() }), expLst: list![Arc::new(DAE::Exp::ICONST { integer: i.clone() })], attr: DAE::callAttrBuiltinOther().clone() });
            whenEq = Arc::new(BackendDAE::WhenEquation { condition: c.clone(), whenStmtLst: list![BackendDAE::WhenOperator::NORETCALL { exp: e.clone(), source: DAE::emptyElementSource().clone() }], elsewhenPart: None });
            eq = Arc::new(BackendDAE::Equation::WHEN_EQUATION { size: 0, whenEquation: whenEq.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
            metamodelica::cons(eq.clone(), outRemovedEqs.clone())
        },
        _ => {
            outRemovedEqs.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outRemovedEqs
}

pub(crate) fn getBoolClockWhenClauses(mut eq: Arc<BackendDAE::Equation>, mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> (Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) {
    let mut eq: Arc<BackendDAE::Equation> = eq;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = eqLst;
    if hasBoolClockWhenClause(eq.clone()) {
        eqLst = metamodelica::cons(eq.clone(), eqLst);
    }
    (eq, eqLst)
}

fn hasBoolClockWhenClause(mut eqn: Arc<BackendDAE::Equation>) -> bool {
    let mut hasBool: bool = false;
    let () = (::match_deref::match_deref! { match &(eqn) {
        Deref @ BackendDAE::Equation::WHEN_EQUATION { size: 0, whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$_clkfire" }, .. }, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => {
            hasBool = true;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasBool
}

fn treatClockedStates(mut inSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<BackendDAE::Shared>)> {
    let mut outSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = inShared.clone();
    outSysts = ({
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
        for mut syst in (inSysts).into_iter().cloned() {
            let __x = ({
        let mut lstEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
        let mut derVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: eqs, .. } => {
            let mut idx: i32;
            let mut subPartition: BackendDAE::SubPartition;
            let mut solverMethod: ArcStr;
            let mut eq: Arc<BackendDAE::Equation>;
            let mut var: BackendDAE::Var;
            let mut exp: Arc<DAE::Exp>;
            let mut exp2: Arc<DAE::Exp>;
            let mut ty: Arc<DAE::Type>;
            let mut eqs = (*eqs).clone();
            let BackendDAE::CLOCKED_PARTITION { subPartIdx: __pa0 } = (syst.partitionKind.clone()) else { bail!("pattern mismatch") };
            idx = __pa0.clone();
            subPartition = ({let __elt = shared.partitionsInfo.subPartitions.borrow()[(idx-1) as usize].clone(); __elt});
            solverMethod = (BackendDump::optionString(getSubClockSolverOpt(subPartition.clock.clone()))).clone();
            if StringUtil::startsWith((solverMethod.clone()).clone(), (literal!("Explicit")).clone()) {
                if solverMethod.clone() != literal!("ExplicitEuler") {
                    Error::addMessage(Error::CLOCK_SOLVERMETHOD.clone(), list![(literal!("ExplicitEuler")).clone(), (solverMethod.clone()).clone()])?;
                    solverMethod = (literal!("ExplicitEuler")).clone();
                }
            } else if ((solverMethod.clone()).clone().len() as i32) > 0 && solverMethod.clone() != literal!("ImplicitEuler") && solverMethod.clone() != literal!("SemiImplicitEuler") && solverMethod.clone() != literal!("ImplicitTrapezoid") {
                Error::addMessage(Error::CLOCK_SOLVERMETHOD.clone(), list![(literal!("ImplicitEuler")).clone(), (solverMethod.clone()).clone()])?;
                solverMethod = (literal!("ImplicitEuler")).clone();
            }
            for mut i in 1..=BackendEquation::getNumberOfEquations(eqs.clone()) {
                eq = BackendEquation::get(eqs.clone(), i.clone())?;
                let (__pa1, (__pa2, _)) = BackendEquation::traverseExpsOfEquation(eq.clone(), (std::sync::Arc::new(getDerVars1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>))> + 'static>), (derVars.clone(), BackendEquation::getForEquationIterIdent(eq.clone())))?;
                eq = __pa1.clone();
                derVars = __pa2.clone();
                lstEqs = metamodelica::cons(eq.clone(), lstEqs.clone());
            }
            for mut derVar in &*derVars.clone() {
                let mut derVar = derVar.clone();
                var = ((BackendVariable::getVar(derVar.clone(), syst.orderedVars.clone())?).0).get(1)?;
                var = BackendDAE::Var { varName: ComponentReference::crefPrefixDer(derVar.clone()), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: var.varType.clone(), bindExp: None, tplExp: None, arryDim: var.arryDim.clone(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
                assign_field!(syst.orderedVars = BackendVariable::addVar(var.clone(), syst.orderedVars.clone())?);
            }
            for mut derVar in &*derVars.clone() {
                let mut derVar = derVar.clone();
                var = ((BackendVariable::getVar(derVar.clone(), syst.orderedVars.clone())?).0).get(1)?;
                ty = var.varType.clone();
                derVar = (::match_deref::match_deref! { match &(var.varType.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty: __esc_ty, .. } => {
            ty = (*__esc_ty).clone();
            ComponentReference::crefApplySubs(derVar.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("i")).clone(), identType: DAE::T_INTEGER_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_INTEGER_DEFAULT().clone() }) })])?
        },
        _ => derVar.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                exp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: derVar.clone(), ty: ty.clone() })], attr: DAE::callAttrBuiltinImpureReal().clone() });
                (exp, _) = substituteFiniteDifference(exp.clone(), metamodelica::nil());
                exp2 = Arc::new(DAE::Exp::CREF { componentRef: ComponentReference::crefPrefixDer(derVar.clone()), ty: ty.clone() });
                if solverMethod.clone() == literal!("ExplicitEuler") {
                    exp2 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![exp2.clone()], attr: DAE::callAttrBuiltinImpureReal().clone() });
                } else if solverMethod.clone() == literal!("ImplicitTrapezoid") {
                    exp2 = Arc::new(DAE::Exp::BINARY { exp1: exp2.clone(), operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![exp2.clone()], attr: DAE::callAttrBuiltinImpureReal().clone() }) });
                    exp2 = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.5_f64) }), operator: DAE::Operator::MUL { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: exp2.clone() });
                }
                exp2 = Arc::new(DAE::Exp::IFEXP { expCond: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("firstTick")).clone() }), expLst: metamodelica::nil(), attr: DAE::callAttrBuiltinImpureBool().clone() }), expThen: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) }), expElse: exp2.clone() });
                eq = (::match_deref::match_deref! { match &(var.varType.clone()) {
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: dim, tail: Deref @ metamodelica::List::Nil }, .. } => {
            Arc::new(BackendDAE::Equation::FOR_EQUATION { iter: Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("i")).clone(), identType: DAE::T_INTEGER_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_INTEGER_DEFAULT().clone() }), start: Arc::new(DAE::Exp::ICONST { integer: 1 }), stop: DAEUtil::dimExp(dim.clone())?, body: Arc::new(BackendDAE::Equation::EQUATION { exp: exp.clone(), scalar: exp2.clone(), source: var.source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), source: var.source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() })
        },
        _ => {
            Arc::new(BackendDAE::Equation::EQUATION { exp: exp.clone(), scalar: exp2.clone(), source: var.source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                lstEqs = metamodelica::cons(eq.clone(), lstEqs.clone());
            }
            assign_field!(syst.orderedEqs = BackendEquation::listEquation(lstEqs.clone().reverse())?);
            if solverMethod.clone() == literal!("SemiImplicitEuler") {
                for mut i in 1..=BackendEquation::getNumberOfEquations(eqs.clone()) {
                    eq = BackendEquation::get(eqs.clone(), i.clone())?;
                    (eq, _) = BackendEquation::traverseExpsOfEquation(eq.clone(), (std::sync::Arc::new(shiftDerVars1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), derVars.clone())?;
                    eqs = BackendEquation::setAtIndex(eqs.clone(), i.clone(), eq.clone())?;
                }
            }
            shared = markClockedStates(syst.clone(), shared.clone(), derVars.clone())?;
            BackendDAEUtil::clearEqSyst(syst.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((outSysts, shared))
}

fn getDerVars1(mut inExp: Arc<DAE::Exp>, mut inDerVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outDerVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>);
    (outExp, outDerVars) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new(getDerVars) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>))> + 'static>), inDerVars)?;
    Ok((outExp, outDerVars))
}

fn getDerVars(mut inExp: Arc<DAE::Exp>, mut inDerVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outDerVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>) = inDerVars.clone();
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: x, ty }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut derVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut optForIter: Option<ArcStr>;
            let mut forIter: ArcStr = arcstr::literal!("");
            let mut der_x: Arc<DAE::Exp>;
            let mut x = (*x).clone();
            der_x = Arc::new(DAE::Exp::CREF { componentRef: ComponentReference::crefPrefixDer(x.clone()), ty: ty.clone() });
            (derVars, optForIter) = inDerVars;
            let () = (match optForIter.clone() {
        Some(mut __esc_forIter) => {
            forIter = __esc_forIter.clone();
            x = ComponentReference::crefStripIterSub(x.clone(), (forIter.clone()).clone());
            ()
        },
        _ => (),
    });
            if !(ComponentReferenceBasics::crefInLst(x.clone(), derVars.clone())?) {
                derVars = metamodelica::cons(x.clone(), derVars);
            }
            outDerVars = (derVars, optForIter);
            der_x
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outDerVars))
}

fn shiftDerVars1(mut inExp: Arc<DAE::Exp>, mut inDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    (outExp, outDerVars) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new(shiftDerVars) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), inDerVars)?;
    Ok((outExp, outDerVars))
}

fn shiftDerVars(mut inExp: Arc<DAE::Exp>, mut inDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = inDerVars.clone();
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: x, .. } if (ComponentReferenceBasics::crefInLst(x.clone(), inDerVars.clone())?) => {
            let mut exp: Arc<DAE::Exp>;
            exp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![inExp], attr: DAE::callAttrBuiltinImpureReal().clone() });
            exp
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst, .. }, tail: Deref @ metamodelica::List::Nil }, attr: attr @ Deref @ DAE::CallAttributes { .. } } => {
            let mut exp: Arc<DAE::Exp>;
            exp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: expLst.clone(), attr: attr.clone() });
            exp
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst, .. }, tail: Deref @ metamodelica::List::Nil }, attr: attr @ Deref @ DAE::CallAttributes { .. } } => {
            let mut exp: Arc<DAE::Exp>;
            exp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: expLst.clone(), attr: attr.clone() });
            exp
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outDerVars))
}

fn substituteFiniteDifference1(mut inExp: Arc<DAE::Exp>, mut inDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    (outExp, outDerVars) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new(fnptr!(substituteFiniteDifference, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), inDerVars)?;
    Ok((outExp, outDerVars))
}

fn substituteFiniteDifference(mut inExp: Arc<DAE::Exp>, mut inDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> (Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outDerVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    (outExp, outDerVars) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: expLst @ Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: x, .. }, tail: Deref @ metamodelica::List::Nil }, attr: attr @ Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut exp: Arc<DAE::Exp>;
            exp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: expLst.clone(), attr: attr.clone() });
            exp = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::CREF { componentRef: x.clone(), ty: ty.clone() }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: exp });
            exp = Arc::new(DAE::Exp::BINARY { exp1: exp, operator: DAE::Operator::DIV { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("interval")).clone() }), expLst: metamodelica::nil(), attr: DAE::callAttrBuiltinImpureReal().clone() }) });
            (exp, metamodelica::cons(x.clone(), inDerVars))
        },
        _ => {
            (inExp, inDerVars)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outDerVars)
}

fn markClockedStates(mut inSyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut derVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<BackendDAE::Shared>> {
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut eq: Arc<BackendDAE::Equation>;
    let mut prevVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut isPrevVarArr: metamodelica::Array<bool>;
    let mut isDerVarArr: metamodelica::Array<bool>;
    let mut varIxs: Arc<metamodelica::List<i32>>;
    let mut var: BackendDAE::Var;
    let mut idx: i32;
    let mut subPartition: BackendDAE::SubPartition;
    let BackendDAE::CLOCKED_PARTITION { subPartIdx: __pa0 } = (inSyst.partitionKind.clone()) else { bail!("pattern mismatch") };
    idx = __pa0.clone();
    subPartition = ({let __elt = outShared.partitionsInfo.subPartitions.borrow()[(idx-1) as usize].clone(); __elt});
    isPrevVarArr = arrayCreate(BackendVariable::varsSize(inSyst.orderedVars.clone()), false);
    isDerVarArr = arrayCreate(BackendVariable::varsSize(inSyst.orderedVars.clone()), false);
    for mut cr in &*derVars {
        let mut cr = cr.clone();
        varIxs = getVarIxs(cr.clone(), inSyst.orderedVars.clone());
        for mut idx in &*varIxs.clone() {
            let mut idx = idx.clone();
            metamodelica::arrayUpdate(isDerVarArr.clone(), idx, true)?;
        }
    }
    for mut i in 1..=BackendEquation::getNumberOfEquations(inSyst.orderedEqs.clone()) {
        eq = BackendEquation::get(inSyst.orderedEqs.clone(), i.clone())?;
        let (_, (__pa1, _)) = BackendEquation::traverseExpsOfEquation(eq.clone(), (std::sync::Arc::new(collectPrevVars) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>))> + 'static>), (prevVars.clone(), BackendEquation::getForEquationIterIdent(eq.clone())))?;
        prevVars = __pa1.clone();
    }
    for mut i in 1..=BackendEquation::getNumberOfEquations(inSyst.removedEqs.clone()) {
        eq = BackendEquation::get(inSyst.removedEqs.clone(), i.clone())?;
        let (_, (__pa2, _)) = BackendEquation::traverseExpsOfEquation(eq.clone(), (std::sync::Arc::new(collectPrevVars) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>))> + 'static>), (prevVars.clone(), BackendEquation::getForEquationIterIdent(eq.clone())))?;
        prevVars = __pa2.clone();
    }
    if !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) {
        prevVars = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut cr in (prevVars).into_iter().cloned() {
            let __x = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    }
    for mut cr in &*prevVars {
        let mut cr = cr.clone();
        varIxs = getVarIxs(cr.clone(), inSyst.orderedVars.clone());
        for mut idx in &*varIxs.clone() {
            let mut idx = idx.clone();
            metamodelica::arrayUpdate(isPrevVarArr.clone(), idx, true)?;
        }
    }
    prevVars = metamodelica::nil();
    for mut i in 1..=metamodelica::arrayLength(isPrevVarArr.clone()) {
        if ({let __elt = isPrevVarArr.borrow()[(i.clone()-1) as usize].clone(); __elt}) {
            var = BackendVariable::getVarAt(inSyst.orderedVars.clone(), i.clone())?;
            var = BackendVariable::setVarKind(var.clone(), BackendDAE::VarKind::CLOCKED_STATE { previousName: ComponentReference::crefPrefixPrevious(var.varName.clone()), isStartFixed: ({let __elt = isDerVarArr.borrow()[(i.clone()-1) as usize].clone(); __elt}) })?;
            var = BackendVariable::setVarFixed(var.clone(), true)?;
            BackendVariable::setVarAt(inSyst.orderedVars.clone(), i.clone(), var.clone())?;
            prevVars = metamodelica::cons(var.varName.clone(), prevVars.clone());
        }
    }
    subPartition.prevVars = prevVars;
    metamodelica::arrayUpdate(outShared.partitionsInfo.subPartitions.clone(), idx, subPartition)?;
    Ok(outShared)
}

fn collectPrevVars(mut inExp: Arc<DAE::Exp>, mut inPrevVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outPrevVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>);
    (outExp, outPrevVars) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new(fnptr!(collectPrevVars1, Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>))> + 'static>), inPrevVars)?;
    Ok((outExp, outPrevVars))
}

fn collectPrevVars1(mut inExp: Arc<DAE::Exp>, mut inPrevVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) -> (Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>)) {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outPrevVars: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Option<ArcStr>);
    outPrevVars = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut inPrevCompRefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut inForIter: Option<ArcStr>;
            let mut forIter: ArcStr = arcstr::literal!("");
            let mut cr = (*cr).clone();
            (inPrevCompRefs, inForIter) = inPrevVars;
            let () = (match inForIter.clone() {
        Some(mut __esc_forIter) => {
            forIter = __esc_forIter.clone();
            cr = ComponentReference::crefStripIterSub(cr.clone(), (forIter.clone()).clone());
            ()
        },
        _ => (),
    });
            (metamodelica::cons(cr.clone(), inPrevCompRefs), inForIter)
        },
        _ => {
            inPrevVars
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outPrevVars)
}

fn subClockPartitioning1(mut inSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut inShared: Arc<BackendDAE::Shared>, mut inHoldComps: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<BackendDAE::Shared>)> {
    let mut outSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut baseClock: Arc<DAE::ClockKind>;
    let mut varsPartition: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    let mut i: i32;
    let mut j: i32;
    let mut n: i32;
    let mut nBaseClocks: i32;
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut hasHoldOperator: metamodelica::Array<bool>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut lstSubClocks1: Arc<metamodelica::List<BackendDAE::SubClock>>;
    let mut lstSubClocks: Arc<metamodelica::List<BackendDAE::SubClock>> = metamodelica::nil();
    let mut partitionsInfo: BackendDAE::PartitionsInfo;
    let mut basePartitions: metamodelica::Array<BackendDAE::BasePartition>;
    let mut subPartitions: metamodelica::Array<BackendDAE::SubPartition>;
    nBaseClocks = (inSysts.clone().len() as i32);
    basePartitions = arrayCreate(nBaseClocks, BackendDAE::BasePartition { clock: openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK(), nSubClocks: 0 });
    varsPartition = HashTable::emptyHashTable();
    i = 0;
    j = 1;
    for mut syst in &*inSysts {
        let mut syst = syst.clone();
        (systs, baseClock, lstSubClocks1) = subClockPartitioning(syst.clone(), outShared.clone(), i)?;
        n = (systs.clone().len() as i32);
        metamodelica::arrayUpdate(basePartitions.clone(), j, BackendDAE::BasePartition { clock: baseClock.clone(), nSubClocks: n })?;
        outSysts = List::append_reverse(systs.clone(), outSysts.clone());
        lstSubClocks = List::append_reverse(lstSubClocks1.clone(), lstSubClocks.clone());
        i = i + n;
        j = j + 1;
    }
    outSysts = metamodelica::Dangerous::listReverseInPlace(outSysts);
    lstSubClocks = metamodelica::Dangerous::listReverseInPlace(lstSubClocks);
    hasHoldOperator = arrayCreate((lstSubClocks.clone().len() as i32), false);
    i = 1;
    for mut syst in &*outSysts.clone() {
        let mut syst = syst.clone();
        for mut j in 1..=BackendVariable::varsSize(syst.orderedVars.clone()) {
            let BackendDAE::VAR { varName: __pa0, .. } = (BackendVariable::getVarAt(syst.orderedVars.clone(), j)?) else { bail!("pattern mismatch") };
            cr = __pa0.clone();
            varsPartition = BaseHashTable::add((cr.clone(), i), varsPartition.clone())?;
        }
        i = i + 1;
    }
    for mut cr in &*inHoldComps {
        let mut cr = cr.clone();
        i = BaseHashTable::get(cr.clone(), varsPartition.clone())?;
        metamodelica::arrayUpdate(hasHoldOperator.clone(), i, true)?;
    }
    i = 1;
    subPartitions = arrayCreate((lstSubClocks.clone().len() as i32), BackendDAE::SubPartition { clock: BackendDAE::DEFAULT_SUBCLOCK.clone(), holdEvents: false, prevVars: metamodelica::nil() });
    for mut subclock in &*lstSubClocks {
        let mut subclock = subclock.clone();
        metamodelica::arrayUpdate(subPartitions.clone(), i, BackendDAE::SubPartition { clock: subclock.clone(), holdEvents: ({let __elt = hasHoldOperator.borrow()[(i-1) as usize].clone(); __elt}), prevVars: metamodelica::nil() })?;
        i = i + 1;
    }
    partitionsInfo = outShared.partitionsInfo.clone();
    partitionsInfo.basePartitions = basePartitions.clone();
    partitionsInfo.subPartitions = subPartitions.clone();
    assign_field!(outShared.partitionsInfo = partitionsInfo);
    Ok((outSysts, outShared))
}

fn removeHoldExpsSyst(mut inSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut outHoldComps: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    for mut syst1 in &*inSysts {
        let mut syst1 = syst1.clone();
        syst1 = (::match_deref::match_deref! { match &(syst1.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqs, .. } => {
            let mut lstEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut i: i32 = 0;
            let mut eq: Arc<BackendDAE::Equation>;
            let mut syst = (*syst).clone();
            lstEqs = metamodelica::nil();
            for mut i in 1..=BackendEquation::getNumberOfEquations(eqs.clone()) {
                eq = BackendEquation::get(eqs.clone(), i)?;
                (eq, outHoldComps) = BackendEquation::traverseExpsOfEquation(eq.clone(), (std::sync::Arc::new(removeHoldExp1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), outHoldComps.clone())?;
                lstEqs = metamodelica::cons(eq.clone(), lstEqs.clone());
            }
            assign_field!(syst.orderedEqs = BackendEquation::listEquation(lstEqs.clone().reverse())?);
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outSysts = metamodelica::cons(BackendDAEUtil::clearEqSyst(syst1.clone())?, outSysts.clone());
    }
    Ok((outSysts, outHoldComps))
}

fn removeHoldExp1(mut inExp: Arc<DAE::Exp>, mut inComps: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outComps: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    (outExp, outComps) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new(removeHoldExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), inComps)?;
    Ok((outExp, outComps))
}

fn removeHoldExp(mut inExp: Arc<DAE::Exp>, mut inComps: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outComps: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    (outExp, outComps) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "hold" }, expLst: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, attr: _ } => {
            let mut cr: Arc<DAE::ComponentRef>;
            let __pa0 = ::match_deref::match_deref! { match &(e.clone()) {
                Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            (substGetPartition(e.clone())?, metamodelica::cons(cr, inComps))
        },
        _ => {
            (inExp, inComps)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outComps))
}

fn getSubPartitionAdjacency(mut numPartitions: i32, mut baseClockEq: i32, mut subPartitionInterfaceEqs: Arc<metamodelica::List<i32>>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>, mut clockedVarsMask: metamodelica::Array<bool>, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut vars: BackendDAE::Variables) -> Result<(metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::SubClock)>>>, metamodelica::Array<i32>)> {
    let mut partAdjacency: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::SubClock)>>>;
    let mut order: metamodelica::Array<i32>;
    let mut infered: bool;
    let mut part: i32;
    let mut part1: i32;
    let mut part2: i32;
    let mut var1: i32;
    let mut var2: i32;
    let mut partLst: Arc<metamodelica::List<i32>>;
    let mut orderLst: Arc<metamodelica::List<i32>>;
    let mut subClk1: BackendDAE::SubClock;
    let mut subClk2: BackendDAE::SubClock;
    let mut partitionParents: metamodelica::Array<i32>;
    let mut partitionParentsVisited: metamodelica::Array<bool>;
    let mut partitionInterfacesClockVars: metamodelica::Array<bool>;
    partAdjacency = arrayCreate(numPartitions, metamodelica::nil());
    partitionParents = arrayCreate(numPartitions, -1);
    partitionInterfacesClockVars = arrayCreate(numPartitions, false);
    for mut subPartEq in &*subPartitionInterfaceEqs {
        let mut subPartEq = subPartEq.clone();
        (infered, part1, var1, subClk1, part2, var2, subClk2) = getConnectedSubPartitions(BackendEquation::get(eqs.clone(), subPartEq.clone())?, varPartMap.clone(), vars.clone())?;
        if part1 != 0 && part2 != 0 {
            addPartAdjacencyEdge(part1, subClk1.clone(), part2, subClk2.clone(), partAdjacency.clone())?;
        }
        if ({let __elt = partitionParents.borrow()[(part2-1) as usize].clone(); __elt}) == part1 && ({let __elt = partitionInterfacesClockVars.borrow()[(part2-1) as usize].clone(); __elt}) {
            {
                let __cell0 = -1;
                let __idx0 = part2;
                partitionParents.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
            }
        }
        {
            let __cell1 = !(({let __elt = clockedVarsMask.borrow()[(var1-1) as usize].clone(); __elt}) && ({let __elt = clockedVarsMask.borrow()[(var2-1) as usize].clone(); __elt}));
            let __idx1 = part1;
            partitionInterfacesClockVars.clone().borrow_mut()[(__idx1-1) as usize] = __cell1;
        }
        if ({let __elt = partitionParents.borrow()[(part2-1) as usize].clone(); __elt}) != part1 {
            {
                let __cell2 = part2;
                let __idx2 = part1;
                partitionParents.clone().borrow_mut()[(__idx2-1) as usize] = __cell2;
            }
        }
    }
    partLst = List::intRange(numPartitions);
    partitionParentsVisited = arrayCreate(numPartitions, false);
    orderLst = metamodelica::nil();
    while !(partLst.clone().is_empty()) {
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(partLst.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        part = __pa3.clone();
        partLst = __pa4.clone();
        if !(({let __elt = partitionParentsVisited.borrow()[(part-1) as usize].clone(); __elt})) {
            if ({let __elt = partitionParents.borrow()[(part-1) as usize].clone(); __elt}) == -1 || ({let __elt = partitionParents.borrow()[(part-1) as usize].clone(); __elt}) == part {
                orderLst = metamodelica::cons(part, orderLst.clone());
                {
                    let __cell5 = true;
                    let __idx5 = part;
                    partitionParentsVisited.clone().borrow_mut()[(__idx5-1) as usize] = __cell5;
                }
            } else if ({let __elt = partitionParentsVisited.borrow()[(({let __elt = partitionParents.borrow()[(part-1) as usize].clone(); __elt})-1) as usize].clone(); __elt}) {
                orderLst = metamodelica::cons(part, orderLst.clone());
                {
                    let __cell6 = true;
                    let __idx6 = part;
                    partitionParentsVisited.clone().borrow_mut()[(__idx6-1) as usize] = __cell6;
                }
            } else {
                partLst = metamodelica::cons(part, partLst.clone());
                partLst = metamodelica::cons(({let __elt = partitionParents.borrow()[(part-1) as usize].clone(); __elt}), partLst.clone());
            }
        }
    }
    order = metamodelica::arrayFromVec(orderLst.reverse().into_iter().cloned().collect());
    Ok((partAdjacency, order))
}

fn getSubClockForClkConstructor(mut refClock: Arc<DAE::ClockKind>, mut clk: Arc<DAE::ClockKind>) -> Result<BackendDAE::SubClock> {
    let mut subClk: BackendDAE::SubClock;
    subClk = (::match_deref::match_deref! { match &((refClock, clk)) {
        (Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: Deref @ DAE::Exp::ICONST { integer: i1 }, resolution: Deref @ DAE::Exp::ICONST { integer: i2 } }, Deref @ DAE::ClockKind::INFERRED_CLOCK { .. }) => {
            BackendDAE::SubClock::SUBCLOCK { factor: MMath::Rational { nom: i2.clone(), denom: i1.clone() }, shift: MMath::RAT0.clone(), solver: None }
        },
        (Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: Deref @ DAE::Exp::ICONST { integer: i1 }, resolution: Deref @ DAE::Exp::ICONST { integer: i2 } }, Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: Deref @ DAE::Exp::ICONST { integer: i3 }, resolution: Deref @ DAE::Exp::ICONST { integer: i4 } }) => {
            BackendDAE::SubClock::SUBCLOCK { factor: MMath::divRational(MMath::Rational { nom: i2.clone(), denom: i1.clone() }, MMath::Rational { nom: i4.clone(), denom: i3.clone() })?, shift: MMath::RAT0.clone(), solver: None }
        },
        (Deref @ DAE::ClockKind::REAL_CLOCK { interval: Deref @ DAE::Exp::RCONST { real: r1 } }, Deref @ DAE::ClockKind::INFERRED_CLOCK { .. }) => {
            BackendDAE::SubClock::SUBCLOCK { factor: MMath::Rational { nom: 1, denom: ((metamodelica::OrderedFloat(1.0_f64) / r1.clone()).0.floor() as i32) }, shift: MMath::RAT0.clone(), solver: None }
        },
        (Deref @ DAE::ClockKind::REAL_CLOCK { interval: Deref @ DAE::Exp::RCONST { real: r1 } }, Deref @ DAE::ClockKind::REAL_CLOCK { interval: Deref @ DAE::Exp::RCONST { real: r2 } }) => {
            BackendDAE::SubClock::SUBCLOCK { factor: MMath::divRational(MMath::Rational { nom: 1, denom: ((metamodelica::OrderedFloat(1.0_f64) / r1.clone()).0.floor() as i32) }, MMath::Rational { nom: 1, denom: ((metamodelica::OrderedFloat(1.0_f64) / r2.clone()).0.floor() as i32) })?, shift: MMath::RAT0.clone(), solver: None }
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SynchronousFeatures.getSubClockForClkConstructor")); __mm_s.push_str(&*literal!(" failed.\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/SynchronousFeatures.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(subClk)
}

fn setSolverSubClock(mut baseClkIn: Arc<DAE::ClockKind>, mut inSubClock: BackendDAE::SubClock) -> (Arc<DAE::ClockKind>, BackendDAE::SubClock) {
    let mut baseClkOut: Arc<DAE::ClockKind>;
    let mut outSubClock: BackendDAE::SubClock = BackendDAE::SubClock::INFERED_SUBCLOCK;
    (baseClkOut, outSubClock) = (::match_deref::match_deref! { match &(baseClkIn.clone()) {
        Deref @ DAE::ClockKind::SOLVER_CLOCK { c: Deref @ DAE::Exp::CLKCONST { clk }, solverMethod: Deref @ DAE::Exp::SCONST { string: solver } } => {
            outSubClock = setSubClockSolver(inSubClock, if (solver.clone() == literal!("")) {None} else {Some((solver.clone()).clone())});
            (clk.clone(), outSubClock)
        },
        _ => {
            (baseClkIn, inSubClock)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (baseClkOut, outSubClock)
}

fn findSubClocks(mut numPartitions: i32, mut baseClockEq: i32, mut baseClk: Arc<DAE::ClockKind>, mut baseClockConstructors: Arc<metamodelica::List<i32>>, mut subPartitionInterfaceEqs: Arc<metamodelica::List<i32>>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut partAdjacency: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::SubClock)>>>) -> Result<(Arc<DAE::ClockKind>, metamodelica::Array<BackendDAE::SubClock>)> {
    let mut baseClkOut: Arc<DAE::ClockKind>;
    let mut outSubClocks: metamodelica::Array<BackendDAE::SubClock>;
    let mut part1: i32;
    let mut part2: i32;
    let mut partLst: Arc<metamodelica::List<i32>>;
    let mut subClk1: BackendDAE::SubClock;
    let mut subClk2: BackendDAE::SubClock;
    let mut clk: Arc<DAE::ClockKind>;
    let mut partIsAssigned: metamodelica::Array<bool>;
    let mut adjParts: Arc<metamodelica::List<(i32, BackendDAE::SubClock)>>;
    outSubClocks = arrayCreate(numPartitions, BackendDAE::DEFAULT_SUBCLOCK.clone());
    partIsAssigned = arrayCreate(numPartitions, false);
    for mut clockEq in &*baseClockConstructors {
        let mut clockEq = clockEq.clone();
        if !(intEq(baseClockEq, clockEq.clone())) && !(intEq(baseClockEq, -1)) {
            part1 = metamodelica::arrayGet(eqPartMap.clone(), clockEq.clone())?;
            clk = getBaseClock(BackendEquation::get(eqs.clone(), clockEq.clone())?);
            if !(isInferedBaseClock(clk.clone())) {
                subClk1 = getSubClockForClkConstructor(baseClk.clone(), clk.clone())?;
                metamodelica::arrayUpdate(outSubClocks.clone(), part1, subClk1.clone())?;
                metamodelica::arrayUpdate(partIsAssigned.clone(), part1, true)?;
            }
        }
    }
    if isInferedBaseClock(baseClk.clone()) {
        baseClkOut = baseClk;
        partLst = List::intRange(numPartitions);
    } else {
        part1 = metamodelica::arrayGet(eqPartMap.clone(), baseClockEq)?;
        partLst = metamodelica::cons(part1, List::intRange(numPartitions));
        (baseClkOut, subClk1) = setSolverSubClock(baseClk, ({let __elt = outSubClocks.borrow()[(part1-1) as usize].clone(); __elt}));
        metamodelica::arrayUpdate(outSubClocks.clone(), part1, subClk1.clone())?;
        metamodelica::arrayUpdate(partIsAssigned.clone(), part1, true)?;
    }
    while !(partLst.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(partLst.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        part1 = __pa0.clone();
        partLst = __pa1.clone();
        adjParts = metamodelica::arrayGet(partAdjacency.clone(), part1)?;
        for mut adjPart in &*adjParts.clone() {
            let mut adjPart = adjPart.clone();
            part2 = Util::tuple21(adjPart.clone());
            if !(metamodelica::arrayGet(partIsAssigned.clone(), part2)?) {
                subClk1 = metamodelica::arrayGet(outSubClocks.clone(), part1)?;
                subClk2 = Util::tuple22(adjPart.clone());
                subClk2 = computeAbsoluteSubClock(subClk1.clone(), subClk2.clone())?;
                if !(isInferedSubClock(subClk2.clone())) {
                    metamodelica::arrayUpdate(outSubClocks.clone(), part2, subClk2.clone())?;
                    metamodelica::arrayUpdate(partIsAssigned.clone(), part2, true)?;
                    partLst = metamodelica::cons(part2, partLst.clone());
                }
            }
        }
    }
    Ok((baseClkOut, outSubClocks))
}

fn computeAbsoluteSubClock(mut preClock: BackendDAE::SubClock, mut subSeqClock: BackendDAE::SubClock) -> Result<BackendDAE::SubClock> {
    let mut subClk: BackendDAE::SubClock = BackendDAE::DEFAULT_SUBCLOCK.clone();
    subClk = (match (preClock, subSeqClock.clone()) {
        (BackendDAE::SubClock::SUBCLOCK { factor: mut f1, shift: mut s1, solver: mut solver1 }, BackendDAE::SubClock::SUBCLOCK { factor: mut f2, shift: mut s2, solver: mut solver2 }) => {
            solver1 = mergeSolver(solver1.clone(), solver2.clone())?;
            BackendDAE::SubClock::SUBCLOCK { factor: MMath::divRational(f1.clone(), f2.clone())?, shift: MMath::addRational(MMath::multRational(s1.clone(), f2.clone())?, s2.clone())?, solver: solver1.clone() }
        },
        (BackendDAE::SubClock::SUBCLOCK { factor: _, shift: _, solver: _ }, BackendDAE::SubClock::INFERED_SUBCLOCK { .. }) => {
            subSeqClock
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SynchronousFeatures.computeAbsoluteSubClock")); __mm_s.push_str(&*literal!(" failed.\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/SynchronousFeatures.mo"))?;
            bail!("fail")
        },
    });
    Ok(subClk)
}

fn mergeSolver(mut solver1: Option<ArcStr>, mut solver2: Option<ArcStr>) -> Result<Option<ArcStr>> {
    let mut sOut: Option<ArcStr>;
    sOut = (match (solver1, solver2) {
        (None, Some(mut s2)) => {
            Some((s2.clone()).clone())
        },
        (Some(mut s1), None) => {
            Some((s1.clone()).clone())
        },
        (Some(mut s1), Some(mut s2)) => {
            if !(stringEq((s1.clone()).clone(), (s2.clone()).clone())) {
                Error::addCompilerNotification(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Infered sub clock partitions have different solvers:")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(" <->")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone())?;
            }
            Some((s1.clone()).clone())
        },
        _ => {
            None
        },
    });
    Ok(sOut)
}

fn addPartAdjacencyEdge(mut part1: i32, mut sub1: BackendDAE::SubClock, mut part2: i32, mut sub2: BackendDAE::SubClock, mut partAdjacency: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::SubClock)>>>) -> Result<()> {
    let mut partEdges: Arc<metamodelica::List<(i32, BackendDAE::SubClock)>>;
    if intGt(part1, 0) && intGt(part2, 0) {
        partEdges = metamodelica::arrayGet(partAdjacency.clone(), part1)?;
        for mut edge in &*partEdges.clone() {
            let mut edge = edge.clone();
            if intEq(Util::tuple21(edge.clone()), part2) {
            }
        }
        metamodelica::arrayUpdate(partAdjacency.clone(), part1, metamodelica::cons((part2, sub1), partEdges))?;
        partEdges = metamodelica::arrayGet(partAdjacency.clone(), part2)?;
        metamodelica::arrayUpdate(partAdjacency.clone(), part2, metamodelica::cons((part1, sub2), partEdges))?;
    }
    Ok(())
}

fn setSubClockFactor(mut subClk: BackendDAE::SubClock, mut factor: MMath::Rational) -> BackendDAE::SubClock {
    let mut subClkOut: BackendDAE::SubClock;
    subClkOut = (match subClk.clone() {
        BackendDAE::SubClock::SUBCLOCK { factor: _, shift: mut shift, solver: mut solver } => {
            BackendDAE::SubClock::SUBCLOCK { factor: factor, shift: shift.clone(), solver: solver.clone() }
        },
        _ => {
            subClk
        },
    });
    subClkOut
}

fn getSubClockFactor(mut subClk: BackendDAE::SubClock) -> MMath::Rational {
    let mut factor: MMath::Rational = <MMath::Rational as ::std::default::Default>::default();
    factor = (match subClk {
        BackendDAE::SubClock::SUBCLOCK { factor: mut __esc_factor, shift: _, solver: _ } => {
            factor = __esc_factor.clone();
            factor
        },
        _ => MMath::RAT1.clone(),
    });
    factor
}

fn getSubClockShift(mut subClk: BackendDAE::SubClock) -> MMath::Rational {
    let mut shift: MMath::Rational = <MMath::Rational as ::std::default::Default>::default();
    shift = (match subClk {
        BackendDAE::SubClock::SUBCLOCK { factor: _, shift: mut __esc_shift, solver: _ } => {
            shift = __esc_shift.clone();
            shift
        },
        _ => MMath::RAT0.clone(),
    });
    shift
}

fn getSubClockSolverOpt(mut subClk: BackendDAE::SubClock) -> Option<ArcStr> {
    let mut solver: Option<ArcStr> = None;
    solver = (match subClk {
        BackendDAE::SubClock::SUBCLOCK { factor: _, shift: _, solver: mut __esc_solver } => {
            solver = __esc_solver.clone();
            solver.clone()
        },
        _ => None,
    });
    solver
}

fn setSubClockShift(mut subClk: BackendDAE::SubClock, mut shift: MMath::Rational) -> BackendDAE::SubClock {
    let mut subClkOut: BackendDAE::SubClock;
    subClkOut = (match subClk.clone() {
        BackendDAE::SubClock::SUBCLOCK { factor: mut factor, shift: _, solver: mut solver } => {
            BackendDAE::SubClock::SUBCLOCK { factor: factor.clone(), shift: shift, solver: solver.clone() }
        },
        _ => {
            subClk
        },
    });
    subClkOut
}

fn setSubClockSolver(mut subClk: BackendDAE::SubClock, mut solver: Option<ArcStr>) -> BackendDAE::SubClock {
    let mut subClkOut: BackendDAE::SubClock;
    subClkOut = (match subClk.clone() {
        BackendDAE::SubClock::SUBCLOCK { factor: mut factor, shift: mut shift, solver: _ } => {
            BackendDAE::SubClock::SUBCLOCK { factor: factor.clone(), shift: shift.clone(), solver: solver }
        },
        _ => {
            subClk
        },
    });
    subClkOut
}

fn getConnectedSubPartitions(mut eq: Arc<BackendDAE::Equation>, mut varPartMap: metamodelica::Array<i32>, mut vars: BackendDAE::Variables) -> Result<(bool, i32, i32, BackendDAE::SubClock, i32, i32, BackendDAE::SubClock)> {
    let mut infered: bool = false;
    let mut part1: i32;
    let mut var1: i32 = -1;
    let mut sub1: BackendDAE::SubClock;
    let mut part2: i32;
    let mut var2: i32 = -1;
    let mut sub2: BackendDAE::SubClock;
    sub1 = BackendDAE::DEFAULT_SUBCLOCK.clone();
    sub2 = BackendDAE::DEFAULT_SUBCLOCK.clone();
    (part1, var1, part2, var2) = (::match_deref::match_deref! { match &(eq) {
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: factor }, tail: Deref @ metamodelica::List::Nil } }, .. }, .. } => {
            let mut v1: i32;
            let mut v2: i32;
            let mut p1: i32;
            let mut p2: i32;
            infered = intEq(factor.clone(), 0);
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v1 = __pa0.clone();
            p1 = ({let __elt = varPartMap.borrow()[(v1-1) as usize].clone(); __elt});
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref2.clone(), vars)?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v2 = __pa2.clone();
            p2 = ({let __elt = varPartMap.borrow()[(v2-1) as usize].clone(); __elt});
            if infered {
                sub1 = openmodelica_backend_types::BackendDAE::SubClock::INFERED_SUBCLOCK;
                sub2 = openmodelica_backend_types::BackendDAE::SubClock::INFERED_SUBCLOCK;
            } else {
                sub1 = setSubClockFactor(sub1, MMath::divRational(MMath::RAT1.clone(), MMath::Rational { nom: factor.clone(), denom: 1 })?);
                sub2 = setSubClockFactor(sub2, MMath::Rational { nom: factor.clone(), denom: 1 });
            }
            (p1, v1, p2, v2)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: factor }, tail: Deref @ metamodelica::List::Nil } }, .. }, .. } => {
            let mut v1: i32;
            let mut v2: i32;
            let mut p1: i32;
            let mut p2: i32;
            infered = intEq(factor.clone(), 0);
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v1 = __pa0.clone();
            p1 = ({let __elt = varPartMap.borrow()[(v1-1) as usize].clone(); __elt});
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref2.clone(), vars)?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v2 = __pa2.clone();
            p2 = ({let __elt = varPartMap.borrow()[(v2-1) as usize].clone(); __elt});
            if infered {
                sub1 = openmodelica_backend_types::BackendDAE::SubClock::INFERED_SUBCLOCK;
                sub2 = openmodelica_backend_types::BackendDAE::SubClock::INFERED_SUBCLOCK;
            } else {
                sub1 = setSubClockFactor(sub1, MMath::Rational { nom: factor.clone(), denom: 1 });
                sub2 = setSubClockFactor(sub2, MMath::divRational(MMath::RAT1.clone(), MMath::Rational { nom: factor.clone(), denom: 1 })?);
            }
            (p1, v1, p2, v2)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: counter }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: resolution }, tail: Deref @ metamodelica::List::Nil } } }, .. }, .. } => {
            let mut v1: i32;
            let mut v2: i32;
            let mut p1: i32;
            let mut p2: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v1 = __pa0.clone();
            p1 = ({let __elt = varPartMap.borrow()[(v1-1) as usize].clone(); __elt});
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref2.clone(), vars)?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v2 = __pa2.clone();
            p2 = ({let __elt = varPartMap.borrow()[(v2-1) as usize].clone(); __elt});
            sub1 = setSubClockShift(sub1, MMath::subRational(MMath::RAT0.clone(), MMath::Rational { nom: counter.clone(), denom: resolution.clone() })?);
            sub2 = setSubClockShift(sub2, MMath::Rational { nom: counter.clone(), denom: resolution.clone() });
            (p1, v1, p2, v2)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: counter }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: resolution }, tail: Deref @ metamodelica::List::Nil } } }, .. }, .. } => {
            let mut v1: i32;
            let mut v2: i32;
            let mut p1: i32;
            let mut p2: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v1 = __pa0.clone();
            p1 = ({let __elt = varPartMap.borrow()[(v1-1) as usize].clone(); __elt});
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref2.clone(), vars)?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v2 = __pa2.clone();
            p2 = ({let __elt = varPartMap.borrow()[(v2-1) as usize].clone(); __elt});
            sub1 = setSubClockShift(sub1, MMath::Rational { nom: counter.clone(), denom: resolution.clone() });
            sub2 = setSubClockShift(sub2, MMath::subRational(MMath::RAT0.clone(), MMath::Rational { nom: counter.clone(), denom: resolution.clone() })?);
            (p1, v1, p2, v2)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::SOLVER_CLOCK { c: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, solverMethod: Deref @ DAE::Exp::SCONST { string: solver } } }, .. } => {
            let mut v1: i32;
            let mut v2: i32;
            let mut p1: i32;
            let mut p2: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars.clone())?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v1 = __pa0.clone();
            p1 = ({let __elt = varPartMap.borrow()[(v1-1) as usize].clone(); __elt});
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref2.clone(), vars)?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            v2 = __pa2.clone();
            p2 = ({let __elt = varPartMap.borrow()[(v2-1) as usize].clone(); __elt});
            sub1 = setSubClockSolver(sub1, Some((solver.clone()).clone()));
            sub2 = setSubClockSolver(sub2, Some((solver.clone()).clone()));
            (p1, v1, p2, v2)
        },
        _ => {
            (-1, -1, -1, -1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((infered, part1, var1, sub1, part2, var2, sub2))
}

fn chooseBaseClock(mut clockEqs: Arc<metamodelica::List<i32>>, mut numPartitions: i32, mut eqPartMap: metamodelica::Array<i32>, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<(Arc<DAE::ClockKind>, i32)> {
    let mut outBaseClock: Arc<DAE::ClockKind> = openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK();
    let mut baseClockEqIdx: i32 = -1;
    let mut subClkPartMap: metamodelica::Array<BackendDAE::SubClock>;
    let mut eq: Arc<BackendDAE::Equation>;
    subClkPartMap = arrayCreate(numPartitions, BackendDAE::DEFAULT_SUBCLOCK.clone());
    for mut clockEq in &*clockEqs {
        let mut clockEq = clockEq.clone();
        eq = BackendEquation::get(eqs.clone(), clockEq.clone())?;
        if isBaseClockEq(eq.clone()) {
            outBaseClock = getBaseClock(eq.clone());
            baseClockEqIdx = clockEq.clone();
        }
    }
    Ok((outBaseClock, baseClockEqIdx))
}

fn isBaseClockEq(mut eq: Arc<BackendDAE::Equation>) -> bool {
    let mut isBaseClock: bool;
    isBaseClock = (::match_deref::match_deref! { match &(eq) {
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { .. }, scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::INFERRED_CLOCK { .. } }, .. } => {
            false
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { .. }, scalar: Deref @ DAE::Exp::CLKCONST { .. }, .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBaseClock
}

fn getBaseClock(mut eq: Arc<BackendDAE::Equation>) -> Arc<DAE::ClockKind> {
    let mut baseClk: Arc<DAE::ClockKind>;
    baseClk = (::match_deref::match_deref! { match &(eq) {
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { .. }, scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::INFERRED_CLOCK { .. } }, .. } => {
            openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK()
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { .. }, scalar: Deref @ DAE::Exp::CLKCONST { clk }, .. } => {
            clk.clone()
        },
        _ => {
            openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    baseClk
}

fn removeEdge(mut eq: i32, mut var: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut row: Arc<metamodelica::List<i32>>;
    row = metamodelica::arrayGet(m.clone(), eq)?;
    (row, _) = List::deleteMemberOnTrue(var, row, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    metamodelica::arrayUpdate(m.clone(), eq, row)?;
    row = metamodelica::arrayGet(mT.clone(), var)?;
    (row, _) = List::deleteMemberOnTrue(eq, row, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    metamodelica::arrayUpdate(mT.clone(), var, row)?;
    Ok(())
}

fn findBaseClockInterfaces(mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut vars: BackendDAE::Variables, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut clockEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut subClockInterfaceEqIdxs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut subClockInterfaceEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eqIdx: i32 = 0;
    let mut eq: Arc<BackendDAE::Equation>;
    for mut eqIdx in 1..=BackendEquation::getNumberOfEquations(eqs.clone()) {
        eq = BackendEquation::get(eqs.clone(), eqIdx)?;
        (clockEqs, subClockInterfaceEqIdxs, subClockInterfaceEqs) = findBaseClockInterfaces1(eq.clone(), eqIdx, eqs.clone(), vars.clone(), m.clone(), mT.clone(), clockEqs.clone(), subClockInterfaceEqIdxs.clone(), subClockInterfaceEqs.clone())?;
    }
    Ok((clockEqs, subClockInterfaceEqIdxs, subClockInterfaceEqs))
}

fn findBaseClockInterfaces1(mut eq: Arc<BackendDAE::Equation>, mut eqIdx: i32, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut vars: BackendDAE::Variables, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut clockEqsIn: Arc<metamodelica::List<i32>>, mut subClockInterfaceEqIdxsIn: Arc<metamodelica::List<i32>>, mut subClockInterfaceEqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut clockEqsOut: Arc<metamodelica::List<i32>>;
    let mut subClockInterfaceEqIdxsOut: Arc<metamodelica::List<i32>>;
    let mut subClockInterfaceEqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    (clockEqsOut, subClockInterfaceEqIdxsOut, subClockInterfaceEqsOut) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::INFERRED_CLOCK { .. } }, .. } => {
            (metamodelica::cons(eqIdx, clockEqsIn), subClockInterfaceEqIdxsIn, subClockInterfaceEqsIn)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: _, .. } }, .. } => {
            (metamodelica::cons(eqIdx, clockEqsIn), subClockInterfaceEqIdxsIn, subClockInterfaceEqsIn)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::REAL_CLOCK { interval: _ } }, .. } => {
            (metamodelica::cons(eqIdx, clockEqsIn), subClockInterfaceEqIdxsIn, subClockInterfaceEqsIn)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::EVENT_CLOCK { condition: _, .. } }, .. } => {
            (metamodelica::cons(eqIdx, clockEqsIn), subClockInterfaceEqIdxsIn, subClockInterfaceEqsIn)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::SOLVER_CLOCK { c: Deref @ DAE::Exp::CREF { componentRef: _, .. }, solverMethod: _ } }, .. } => {
            (clockEqsIn, metamodelica::cons(eqIdx, subClockInterfaceEqIdxsIn), metamodelica::cons(eq, subClockInterfaceEqsIn))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::SOLVER_CLOCK { c: Deref @ DAE::Exp::CLKCONST { clk: _ }, solverMethod: _ } }, .. } => {
            (metamodelica::cons(eqIdx, clockEqsIn), subClockInterfaceEqIdxsIn, subClockInterfaceEqsIn)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, .. } => {
            let mut varIdx: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            varIdx = __pa0.clone();
            removeEdge(eqIdx, varIdx, m.clone(), mT.clone())?;
            (clockEqsIn, metamodelica::cons(eqIdx, subClockInterfaceEqIdxsIn), metamodelica::cons(eq, subClockInterfaceEqsIn))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, .. } => {
            let mut varIdx: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            varIdx = __pa0.clone();
            removeEdge(eqIdx, varIdx, m.clone(), mT.clone())?;
            (clockEqsIn, metamodelica::cons(eqIdx, subClockInterfaceEqIdxsIn), metamodelica::cons(eq, subClockInterfaceEqsIn))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, .. }, .. } => {
            let mut varIdx: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            varIdx = __pa0.clone();
            removeEdge(eqIdx, varIdx, m.clone(), mT.clone())?;
            (clockEqsIn, metamodelica::cons(eqIdx, subClockInterfaceEqIdxsIn), metamodelica::cons(eq, subClockInterfaceEqsIn))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, .. } => {
            let mut varIdx: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            varIdx = __pa0.clone();
            removeEdge(eqIdx, varIdx, m.clone(), mT.clone())?;
            (clockEqsIn, metamodelica::cons(eqIdx, subClockInterfaceEqIdxsIn), metamodelica::cons(eq, subClockInterfaceEqsIn))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, .. }, .. } => {
            let mut varIdx: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            varIdx = __pa0.clone();
            removeEdge(eqIdx, varIdx, m.clone(), mT.clone())?;
            (clockEqsIn, metamodelica::cons(eqIdx, subClockInterfaceEqIdxsIn), metamodelica::cons(eq, subClockInterfaceEqsIn))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }, .. }, .. } => {
            let mut varIdx: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            varIdx = __pa0.clone();
            removeEdge(eqIdx, varIdx, m.clone(), mT.clone())?;
            (clockEqsIn, metamodelica::cons(eqIdx, subClockInterfaceEqIdxsIn), metamodelica::cons(eq, subClockInterfaceEqsIn))
        },
        Deref @ BackendDAE::Equation::EQUATION { .. } => {
            (clockEqsIn, subClockInterfaceEqIdxsIn, subClockInterfaceEqsIn)
        },
        _ => {
            (clockEqsIn, subClockInterfaceEqIdxsIn, subClockInterfaceEqsIn)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((clockEqsOut, subClockInterfaceEqIdxsOut, subClockInterfaceEqsOut))
}

fn findHighestWhenPrefixIdx(mut inVar: BackendDAE::Var, mut idxIn: i32) -> Result<(BackendDAE::Var, i32)> {
    let mut outVar: BackendDAE::Var = inVar.clone();
    let mut idxOut: i32 = idxIn;
    let mut name: Arc<DAE::ComponentRef>;
    let mut chars: Arc<metamodelica::List<ArcStr>>;
    let mut chars1: Arc<metamodelica::List<ArcStr>>;
    let mut chars2: Arc<metamodelica::List<ArcStr>>;
    name = inVar.varName.clone();
    chars = stringListStringChar((ComponentReference::crefStr(name)?).clone());
    if intGt((chars.clone().len() as i32), 9) {
        (chars1, chars2) = List::split(chars, 8)?;
        if stringEq(stringDelimitList(chars1, (literal!("")).clone()), (arcstr::literal!(BackendDAE::WHENCLK_PRREFIX)).clone()) {
            idxOut = intMax(idxIn, stringInt(stringDelimitList(chars2, (literal!("")).clone()))?);
        }
    }
    Ok((outVar, idxOut))
}

fn replaceSampledClocks(mut eqsIn: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut varsIn: BackendDAE::Variables) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables)> {
    let mut eqsOut: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut varsOut: BackendDAE::Variables;
    let mut prefIdx: i32;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut newEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut newVars: Arc<metamodelica::List<BackendDAE::Var>>;
    prefIdx = BackendVariable::traverseBackendDAEVars(varsIn.clone(), (std::sync::Arc::new(findHighestWhenPrefixIdx) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, i32) -> Result<(BackendDAE::Var, i32)> + 'static>), 1)?;
    let (__pa0, (_, _, __pa1, __pa2)) = BackendEquation::traverseEquationArray_WithUpdate(eqsIn, (std::sync::Arc::new(replaceSampledClocks1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (BackendDAE::Variables, i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>))> + 'static>), (varsIn.clone(), prefIdx + 1, metamodelica::nil(), metamodelica::nil()))?;
    eqs = __pa0.clone();
    newEqs = __pa1.clone();
    newVars = __pa2.clone();
    eqsOut = BackendEquation::addList(newEqs, eqs)?;
    varsOut = BackendVariable::addVars(newVars, varsIn)?;
    Ok((eqsOut, varsOut))
}

fn replaceSampledClocks1(mut eqIn: Arc<BackendDAE::Equation>, mut tplIn: (BackendDAE::Variables, i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)) -> Result<(Arc<BackendDAE::Equation>, (BackendDAE::Variables, i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>))> {
    let mut eqOut: Arc<BackendDAE::Equation>;
    let mut tplOut: (BackendDAE::Variables, i32, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>);
    (eqOut, tplOut) = (::match_deref::match_deref! { match &((eqIn.clone(), tplIn.clone())) {
        (Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr: BackendDAE::EquationAttributes { kind: BackendDAE::EquationKind::DYNAMIC_EQUATION { .. }, .. } }, (vars, suffixIdx0, newEqs, newVars)) => {
            let mut suffixIdx: i32;
            let mut attr: BackendDAE::EquationAttributes;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            let mut newEqs = (*newEqs).clone();
            let mut newVars = (*newVars).clone();
            let (__pa0, (__pa1, __pa2, __pa3)) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(replaceSampledClocks2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32))> + 'static>), (newEqs.clone(), newVars.clone(), suffixIdx0.clone()))?;
            e1 = __pa0.clone();
            newEqs = __pa1.clone();
            newVars = __pa2.clone();
            suffixIdx = __pa3.clone();
            let (__pa4, (__pa5, __pa6, __pa7)) = Expression::traverseExpTopDown(e2.clone(), (std::sync::Arc::new(replaceSampledClocks2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32))> + 'static>), (newEqs.clone(), newVars.clone(), suffixIdx))?;
            e2 = __pa4.clone();
            newEqs = __pa5.clone();
            newVars = __pa6.clone();
            suffixIdx = __pa7.clone();
            if intEq(suffixIdx - suffixIdx0.clone(), 1) {
                attr = BackendEquation::defaultClockedEqAttr(suffixIdx0.clone());
            } else {
                attr = BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone();
            }
            (Arc::new(BackendDAE::Equation::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: source.clone(), attr: attr }), (vars.clone(), suffixIdx, newEqs.clone(), newVars.clone()))
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize, left: e1, right: e2, source, attr: BackendDAE::EquationAttributes { kind: BackendDAE::EquationKind::DYNAMIC_EQUATION { .. }, .. }, recordSize }, (vars, suffixIdx0, newEqs, newVars)) => {
            let mut suffixIdx: i32;
            let mut attr: BackendDAE::EquationAttributes;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            let mut newEqs = (*newEqs).clone();
            let mut newVars = (*newVars).clone();
            let (__pa0, (__pa1, __pa2, __pa3)) = Expression::traverseExpTopDown(e1.clone(), (std::sync::Arc::new(replaceSampledClocks2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32))> + 'static>), (newEqs.clone(), newVars.clone(), suffixIdx0.clone()))?;
            e1 = __pa0.clone();
            newEqs = __pa1.clone();
            newVars = __pa2.clone();
            suffixIdx = __pa3.clone();
            let (__pa4, (__pa5, __pa6, __pa7)) = Expression::traverseExpTopDown(e2.clone(), (std::sync::Arc::new(replaceSampledClocks2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32))> + 'static>), (newEqs.clone(), newVars.clone(), suffixIdx))?;
            e2 = __pa4.clone();
            newEqs = __pa5.clone();
            newVars = __pa6.clone();
            suffixIdx = __pa7.clone();
            if intEq(suffixIdx - suffixIdx0.clone(), 1) {
                attr = BackendEquation::defaultClockedEqAttr(suffixIdx0.clone());
            } else {
                attr = BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone();
            }
            (Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimSize.clone(), left: e1.clone(), right: e2.clone(), source: source.clone(), attr: attr, recordSize: recordSize.clone() }), (vars.clone(), suffixIdx, newEqs.clone(), newVars.clone()))
        },
        _ => {
            (eqIn, tplIn)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqOut, tplOut))
}

fn replaceSampledClocks2(mut inExp: Arc<DAE::Exp>, mut tplIn: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut tplOut: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32);
    (outExp, cont, tplOut) = (::match_deref::match_deref! { match &((inExp.clone(), tplIn.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, expLst: Deref @ metamodelica::List::Cons { head: varExp @ Deref @ DAE::Exp::CREF { componentRef: _, .. }, tail: Deref @ metamodelica::List::Cons { head: clk @ Deref @ DAE::Exp::CLKCONST { clk: _ }, tail: Deref @ metamodelica::List::Nil } }, .. }, (newEqs, newVars, suffixIdx)) => {
            let mut cr: Arc<DAE::ComponentRef>;
            let mut addEq: Arc<BackendDAE::Equation>;
            let mut addVar: BackendDAE::Var;
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BackendDAE::WHENCLK_PRREFIX)); __mm_s.push_str(&*intString(suffixIdx.clone())); ArcStr::from(__mm_s) }).clone(), identType: DAE::T_CLOCK_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            addVar = BackendVariable::makeVar(cr.clone())?;
            addEq = Arc::new(BackendDAE::Equation::EQUATION { exp: Expression::crefToExp(cr)?, scalar: clk.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
            (substGetPartition(varExp.clone())?, false, (metamodelica::cons(addEq, newEqs.clone()), metamodelica::cons(addVar, newVars.clone()), suffixIdx.clone() + 1))
        },
        _ => {
            (inExp, true, tplIn)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, tplOut))
}

fn subClockPartitioning(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut off: i32) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<DAE::ClockKind>, Arc<metamodelica::List<BackendDAE::SubClock>>)> {
    let mut outSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut outBaseClock: Arc<DAE::ClockKind>;
    let mut outSubClocks: Arc<metamodelica::List<BackendDAE::SubClock>>;
    let mut funcs: Arc<AvlTreePathFunction::Tree>;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut remEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut clockEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut clockVars: BackendDAE::Variables;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut rm: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut rmT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut partitionsCnt: i32;
    let mut remEqPartMap: metamodelica::Array<i32>;
    let mut newClockEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut newClockVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut contPartitions: metamodelica::Array<Option<bool>>;
    let mut subclksCnt: metamodelica::Array<i32>;
    let mut order: metamodelica::Array<i32>;
    let mut subclocks: metamodelica::Array<BackendDAE::SubClock>;
    let mut clockedEqsMask: metamodelica::Array<bool>;
    let mut clockedVarsMask: metamodelica::Array<bool>;
    let mut usedVars: metamodelica::Array<bool>;
    let mut usedRemovedVars: metamodelica::Array<bool>;
    let mut baseClockEqIdx: i32;
    let mut eqIdx: i32 = 0;
    let mut varIdx: i32 = 0;
    let mut baseClockEquations: Arc<metamodelica::List<i32>>;
    let mut subClockInterfaceEqIdxs: Arc<metamodelica::List<i32>>;
    let mut subClockInterfaceEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut varPartMap: metamodelica::Array<i32>;
    let mut eqPartMap: metamodelica::Array<i32>;
    let mut partAdjacency: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::SubClock)>>>;
    let mut sys: Arc<BackendDAE::EqSystem>;
    funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inEqSystem.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, removedEqs: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    vars = __pa0.clone();
    eqs = __pa1.clone();
    remEqs = __pa2.clone();
    (eqs, vars) = replaceSampledClocks(eqs, vars)?;
    sys = BackendDAEUtil::setEqSystVars(inEqSystem.clone(), vars.clone())?;
    sys = BackendDAEUtil::setEqSystEqs(sys, eqs.clone());
    (sys, m, mT) = BackendDAEUtil::getAdjacencyMatrix(sys, openmodelica_backend_types::BackendDAE::IndexType::SUBCLOCK_IDX, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
    (baseClockEquations, subClockInterfaceEqIdxs, subClockInterfaceEqs) = findBaseClockInterfaces(eqs.clone(), vars.clone(), m.clone(), mT.clone())?;
    (clockEqs, clockedEqsMask) = splitClockEqs(eqs.clone())?;
    (clockVars, clockedVarsMask) = splitClockVars(vars.clone())?;
    (rm, rmT) = BackendDAEUtil::removedAdjacencyMatrix(sys, openmodelica_backend_types::BackendDAE::IndexType::SUBCLOCK_IDX, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
    remEqPartMap = arrayCreate(metamodelica::arrayLength(rm.clone()), 0);
    eqPartMap = arrayCreate(metamodelica::arrayLength(m.clone()), 0);
    varPartMap = arrayCreate(metamodelica::arrayLength(mT.clone()), 0);
    usedRemovedVars = arrayCreate(metamodelica::arrayLength(rmT.clone()), false);
    usedVars = arrayCreate(metamodelica::arrayLength(mT.clone()), false);
    partitionsCnt = partitionIndependentBlocksMasked(m.clone(), mT.clone(), rm.clone(), rmT.clone(), arrayCreate(BackendEquation::getNumberOfEquations(eqs.clone()), true), eqPartMap.clone(), varPartMap.clone(), remEqPartMap.clone(), usedVars.clone(), usedRemovedVars.clone())?;
    (outBaseClock, baseClockEqIdx) = chooseBaseClock(baseClockEquations.clone(), partitionsCnt, eqPartMap.clone(), eqs.clone())?;
    (partAdjacency, order) = getSubPartitionAdjacency(partitionsCnt, baseClockEqIdx, subClockInterfaceEqIdxs.clone(), eqPartMap.clone(), varPartMap.clone(), clockedVarsMask.clone(), eqs.clone(), vars.clone())?;
    (m, mT) = BackendDAEUtil::adjacencyMatrixMasked(inEqSystem.clone(), openmodelica_backend_types::BackendDAE::IndexType::SUBCLOCK_IDX, clockedEqsMask.clone(), Some(funcs), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
    (newClockEqs, newClockVars, contPartitions, subclksCnt) = collectSubclkInfo(eqs.clone(), inEqSystem.removedEqs.clone(), partitionsCnt, eqPartMap.clone(), remEqPartMap.clone(), vars.clone(), mT.clone())?;
    (outBaseClock, subclocks) = findSubClocks(partitionsCnt, baseClockEqIdx, outBaseClock, baseClockEquations, subClockInterfaceEqIdxs, eqPartMap.clone(), varPartMap.clone(), eqs.clone(), partAdjacency.clone())?;
    for mut eqIdx in 1..=metamodelica::arrayLength(clockedEqsMask.clone()) {
        if !(metamodelica::arrayGet(clockedEqsMask.clone(), eqIdx)?) {
            metamodelica::arrayUpdate(eqPartMap.clone(), eqIdx, 0)?;
        }
    }
    for mut varIdx in 1..=metamodelica::arrayLength(clockedVarsMask.clone()) {
        if !(metamodelica::arrayGet(clockedVarsMask.clone(), varIdx)?) {
            metamodelica::arrayUpdate(varPartMap.clone(), varIdx, 0)?;
        }
    }
    (outSysts, outSubClocks) = orderSubPartitions(partitionsCnt, subclocks.clone(), order.clone(), eqPartMap.clone(), varPartMap.clone(), remEqPartMap.clone(), eqs, vars, remEqs, inShared, off)?;
    Ok((outSysts, outBaseClock, outSubClocks))
}

fn orderSubPartitions(mut numParts: i32, mut subclocks: metamodelica::Array<BackendDAE::SubClock>, mut order: metamodelica::Array<i32>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>, mut remEqPartMap: metamodelica::Array<i32>, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut vars: BackendDAE::Variables, mut remEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut shared: Arc<BackendDAE::Shared>, mut partitionOffset: i32) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<metamodelica::List<BackendDAE::SubClock>>)> {
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut subClksOut: Arc<metamodelica::List<BackendDAE::SubClock>> = metamodelica::nil();
    let mut considerRemovedEqs: bool;
    let mut part: i32 = 0;
    let mut mergedParts: Arc<metamodelica::List<i32>>;
    let mut partVarMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut partEqMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut partRemEqMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut sys: Arc<BackendDAE::EqSystem>;
    let mut clk: BackendDAE::SubClock;
    let mut clk2: BackendDAE::SubClock;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut remEqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut mergedOrder: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    considerRemovedEqs = intGe(metamodelica::arrayLength(remEqPartMap.clone()), 1);
    partVarMap = arrayCreate(numParts, metamodelica::nil());
    for mut varIdx in 1..=metamodelica::arrayLength(varPartMap.clone()) {
        part = metamodelica::arrayGet(varPartMap.clone(), varIdx.clone())?;
        if part > 0 {
            metamodelica::arrayUpdate(partVarMap.clone(), part, listAppend(({let __elt = partVarMap.borrow()[(part-1) as usize].clone(); __elt}), list![varIdx.clone()]))?;
        }
    }
    partEqMap = arrayCreate(numParts, metamodelica::nil());
    for mut eqIdx in 1..=metamodelica::arrayLength(eqPartMap.clone()) {
        part = metamodelica::arrayGet(eqPartMap.clone(), eqIdx.clone())?;
        if part > 0 {
            metamodelica::arrayUpdate(partEqMap.clone(), part, listAppend(({let __elt = partEqMap.borrow()[(part-1) as usize].clone(); __elt}), list![eqIdx.clone()]))?;
        }
    }
    partRemEqMap = arrayCreate(numParts, metamodelica::nil());
    if considerRemovedEqs {
        for mut reqIdx in 1..=metamodelica::arrayLength(partRemEqMap.clone()) {
            part = metamodelica::arrayGet(remEqPartMap.clone(), reqIdx.clone())?;
            if part > 0 {
                metamodelica::arrayUpdate(partRemEqMap.clone(), part, listAppend(({let __elt = partRemEqMap.borrow()[(part-1) as usize].clone(); __elt}), list![reqIdx.clone()]))?;
            }
        }
    }
    mergedOrder = metamodelica::nil();
    mergedParts = metamodelica::nil();
    clk = metamodelica::arrayGet(subclocks.clone(), ({let __elt = order.borrow()[(1-1) as usize].clone(); __elt}))?;
    let __range0 = order.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut part in __range0 {
        clk2 = metamodelica::arrayGet(subclocks.clone(), part)?;
        if subClkEqual(clk.clone(), clk2.clone())? {
            mergedParts = metamodelica::cons(part, mergedParts.clone());
        } else {
            mergedOrder = metamodelica::cons(mergedParts.clone().reverse(), mergedOrder.clone());
            mergedParts = list![part];
            clk = metamodelica::arrayGet(subclocks.clone(), part)?;
        }
    }
    mergedOrder = metamodelica::cons(mergedParts.reverse(), mergedOrder);
    mergedOrder = mergedOrder.reverse();
    part = 1;
    for mut mergedParts in &*mergedOrder {
        let mut mergedParts = mergedParts.clone();
        eqLst = metamodelica::nil();
        varLst = metamodelica::nil();
        remEqLst = metamodelica::nil();
        for mut partIdx in &*mergedParts.clone() {
            let mut partIdx = partIdx.clone();
            for mut e in &*metamodelica::arrayGet(partEqMap.clone(), partIdx.clone())? {
                let mut e = e.clone();
                eqLst = metamodelica::cons(BackendEquation::get(eqs.clone(), e.clone())?, eqLst.clone());
            }
            for mut v in &*metamodelica::arrayGet(partVarMap.clone(), partIdx.clone())? {
                let mut v = v.clone();
                varLst = metamodelica::cons(BackendVariable::getVarAt(vars.clone(), v.clone())?, varLst.clone());
            }
            for mut r in &*metamodelica::arrayGet(partRemEqMap.clone(), partIdx.clone())? {
                let mut r = r.clone();
                remEqLst = metamodelica::cons(BackendEquation::get(remEqs.clone(), r.clone())?, remEqLst.clone());
            }
            clk = metamodelica::arrayGet(subclocks.clone(), partIdx.clone())?;
        }
        if !(eqLst.clone().is_empty()) || !(remEqLst.clone().is_empty()) {
            (sys, _) = createEqSystem(eqLst.clone().reverse(), varLst.clone().reverse(), remEqLst.clone(), (true, true))?;
            assign_field!(sys.partitionKind = BackendDAE::BaseClockPartitionKind::CLOCKED_PARTITION { subPartIdx: partitionOffset + part });
            subClksOut = metamodelica::cons(clk.clone(), subClksOut.clone());
            systs = metamodelica::cons(sys.clone(), systs.clone());
            part = part + 1;
        }
    }
    systs = systs.reverse();
    subClksOut = subClksOut.reverse();
    Ok((systs, subClksOut))
}

fn isInferedSubClock(mut subClk: BackendDAE::SubClock) -> bool {
    let mut isInfered: bool;
    isInfered = (match subClk {
        BackendDAE::SubClock::INFERED_SUBCLOCK { .. } => true,
        _ => false,
    });
    isInfered
}

fn isInferedBaseClock(mut subClk: Arc<DAE::ClockKind>) -> bool {
    let mut isInfered: bool;
    isInfered = (::match_deref::match_deref! { match &(subClk) {
        Deref @ DAE::ClockKind::INFERRED_CLOCK { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isInfered
}

fn setFactor(mut oldVal: MMath::Rational, mut newVal: MMath::Rational) -> Result<MMath::Rational> {
    let mut outVal: MMath::Rational;
    outVal = (match (oldVal, newVal) {
        (MMath::Rational { nom: 1, denom: 1 }, _) => newVal,
        (_, MMath::Rational { nom: 1, denom: 1 }) => oldVal,
        _ => {
            if !(MMath::equals(oldVal, newVal)?) {
                Error::addMessage(Error::SUBCLOCK_CONFLICT.clone(), list![(literal!("factor")).clone(), (MMath::rationalString(oldVal)?).clone(), (MMath::rationalString(newVal)?).clone()])?;
                bail!("fail");
            }
            newVal
        },
    });
    Ok(outVal)
}

fn setShift(mut oldVal: MMath::Rational, mut newVal: MMath::Rational) -> Result<MMath::Rational> {
    let mut outVal: MMath::Rational;
    outVal = (match (oldVal, newVal) {
        (MMath::Rational { nom: 0, denom: _ }, _) => newVal,
        (_, MMath::Rational { nom: 0, denom: _ }) => oldVal,
        _ => {
            if !(MMath::equals(oldVal, newVal)?) {
                Error::addMessage(Error::SUBCLOCK_CONFLICT.clone(), list![(literal!("shift")).clone(), (MMath::rationalString(oldVal)?).clone(), (MMath::rationalString(newVal)?).clone()])?;
                bail!("fail");
            }
            newVal
        },
    });
    Ok(outVal)
}

fn collectSubclkInfoExp(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>);
    let mut newEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut newVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut contPartitions: metamodelica::Array<Option<bool>>;
    let mut partitionIdx: i32;
    let mut partitions: metamodelica::Array<i32>;
    let mut vars: BackendDAE::Variables;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut path: Arc<Absyn::Path> = Arc::new(<Absyn::Path as ::std::default::Default>::default());
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut attr: Arc<DAE::CallAttributes> = Arc::new(<DAE::CallAttributes as ::std::default::Default>::default());
    let mut clksCnt: metamodelica::Array<i32>;
    let mut clkCnt: i32;
    let mut source: SourceInfo;
    (newEqs, newVars, contPartitions, source, clksCnt, partitionIdx, partitions, vars, mT) = inTpl;
    clkCnt = metamodelica::arrayGet(clksCnt.clone(), partitionIdx)?;
    (outExp, newEqs, newVars, clkCnt) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: __esc_path, expLst: __esc_expLst, attr: __esc_attr } => {
            path = (*__esc_path).clone();
            expLst = (*__esc_expLst).clone();
            attr = (*__esc_attr).clone();
            collectSubclkInfoCall(path.clone(), expLst.clone(), attr.clone(), newEqs, newVars, contPartitions.clone(), partitionIdx, clkCnt, partitions.clone(), vars.clone(), mT.clone(), source.clone())?
        },
        _ => (inExp, newEqs, newVars, clkCnt),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    metamodelica::arrayUpdate(clksCnt.clone(), partitionIdx, clkCnt)?;
    outTpl = (newEqs, newVars, contPartitions.clone(), source, clksCnt.clone(), partitionIdx, partitions.clone(), vars, mT.clone());
    Ok((outExp, outTpl))
}

fn createSubClockVar(mut inPartitionIdx: i32, mut inClkCnt: i32, mut inPath: Arc<Absyn::Path>, mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inAttr: Arc<DAE::CallAttributes>, mut inPartitions: metamodelica::Array<i32>, mut inVars: BackendDAE::Variables, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(BackendDAE::Var, Arc<BackendDAE::Equation>)> {
    let mut outVar: BackendDAE::Var;
    let mut outEq: Arc<BackendDAE::Equation>;
    let mut cr: Arc<DAE::ComponentRef>;
    let mut varIxs: Arc<metamodelica::List<i32>>;
    let mut i: i32;
    let mut e: Arc<DAE::Exp>;
    let mut subclk: Arc<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(listHead(inExpLst.clone())?) {
        Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cr = __pa0.clone();
    (_, varIxs) = BackendVariable::getVar(cr, inVars)?;
    i = listHead(varIxs)?;
    i = listHead(metamodelica::arrayGet(mT.clone(), i)?)?;
    i = metamodelica::arrayGet(inPartitions.clone(), i)?;
    subclk = Arc::new(DAE::Exp::CREF { componentRef: getSubClkName(i, 1, DAE::T_CLOCK_DEFAULT().clone()), ty: DAE::T_CLOCK_DEFAULT().clone() });
    e = Arc::new(DAE::Exp::CALL { path: inPath, expLst: metamodelica::cons(subclk, listRest(inExpLst)?), attr: inAttr });
    (outVar, outEq) = createSubClock(inPartitionIdx, inClkCnt, e)?;
    Ok((outVar, outEq))
}

fn setContClockedPartition(mut inIsContClockedPartition: bool, mut inPartitionIdx: i32, mut inContPartitions: metamodelica::Array<Option<bool>>, mut source: SourceInfo) -> Result<()> {
    let mut isContClockedPartition: Option<bool>;
    let mut isContClockedPrevPartition: bool = false;
    isContClockedPartition = metamodelica::arrayGet(inContPartitions.clone(), inPartitionIdx)?;
    isContClockedPartition = (match isContClockedPartition {
        None => Some(inIsContClockedPartition),
        Some(mut __esc_isContClockedPrevPartition) => {
            isContClockedPrevPartition = __esc_isContClockedPrevPartition.clone();
            Some(inIsContClockedPartition || isContClockedPrevPartition)
        },
    });
    metamodelica::arrayUpdate(inContPartitions.clone(), inPartitionIdx, isContClockedPartition)?;
    Ok(())
}

fn collectSubclkInfoCall(mut inPath: Arc<Absyn::Path>, mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inAttr: Arc<DAE::CallAttributes>, mut inNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inNewVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inContPartitions: metamodelica::Array<Option<bool>>, mut inPartitionIdx: i32, mut inClkCnt: i32, mut inPartitions: metamodelica::Array<i32>, mut inVars: BackendDAE::Variables, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut source: SourceInfo) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut outNewVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut outClkCnt: i32;
    (outExp, outNewEqs, outNewVars, outClkCnt) = (::match_deref::match_deref! { match &((inPath.clone(), (inExpLst.clone().len() as i32))) {
        (Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (Arc::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (Arc::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (Arc::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (Arc::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "terminal" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (Arc::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (Arc::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, 3) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (Arc::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (Arc::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (Arc::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (Arc::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "reinit" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (Arc::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, _) => {
            setContClockedPartition(false, inPartitionIdx, inContPartitions.clone(), source)?;
            (Arc::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "firstTick" }, _) => {
            setContClockedPartition(false, inPartitionIdx, inContPartitions.clone(), source)?;
            (Arc::new(DAE::Exp::CALL { path: inPath, expLst: metamodelica::nil(), attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "interval" }, _) => {
            setContClockedPartition(false, inPartitionIdx, inContPartitions.clone(), source)?;
            (Arc::new(DAE::Exp::CALL { path: inPath, expLst: metamodelica::nil(), attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, 2) => {
            let mut var: BackendDAE::Var;
            let mut eq: Arc<BackendDAE::Equation>;
            (var, eq) = createSubClock(inPartitionIdx, inClkCnt, (inExpLst.clone()).get(2)?)?;
            (substGetPartition((inExpLst).get(1)?)?, metamodelica::cons(eq, inNewEqs), metamodelica::cons(var, inNewVars), inClkCnt + 1)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, 2) => {
            (substGetPartition((inExpLst).get(1)?)?, inNewEqs, inNewVars, inClkCnt + 1)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, 2) => {
            (substGetPartition((inExpLst).get(1)?)?, inNewEqs, inNewVars, inClkCnt + 1)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, 3) => {
            (substGetPartition((inExpLst).get(1)?)?, inNewEqs, inNewVars, inClkCnt + 1)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, 3) => {
            (substGetPartition((inExpLst).get(1)?)?, inNewEqs, inNewVars, inClkCnt + 1)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "noClock" }, 1) => {
            (substGetPartition((inExpLst).get(1)?)?, inNewEqs, inNewVars, inClkCnt)
        },
        _ => {
            (Arc::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outNewEqs, outNewVars, outClkCnt))
}

fn createSubClockVarFactor(mut inPartitionIdx: i32, mut inClkCnt: i32, mut inPath: Arc<Absyn::Path>, mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inAttr: Arc<DAE::CallAttributes>, mut inPartitions: metamodelica::Array<i32>, mut inVars: BackendDAE::Variables, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inNewVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inNewEqs.clone();
    let mut outNewVars: Arc<metamodelica::List<BackendDAE::Var>> = inNewVars.clone();
    let mut outClkCnt: i32 = inClkCnt;
    outExp = substGetPartition(listHead(inExpLst)?)?;
    Ok((outExp, outNewEqs, outNewVars, outClkCnt))
}

fn substGetPartition(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut attrs: Arc<DAE::CallAttributes>;
    attrs = Arc::new(DAE::CallAttributes { ty: Expression::r#typeof(inExp.clone())?, tuple_: false, builtin: true, isImpure: true, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL });
    outExp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$getPart")).clone() }), expLst: list![inExp], attr: attrs });
    Ok(outExp)
}

fn getSubClkName(mut inPartitionIdx: i32, mut inClkIdx: i32, mut inTy: Arc<DAE::Type>) -> Arc<DAE::ComponentRef> {
    let mut outRef: Arc<DAE::ComponentRef>;
    let mut name: ArcStr;
    name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$subclk")); __mm_s.push_str(&*intString(inPartitionIdx)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(inClkIdx)); ArcStr::from(__mm_s) }).clone();
    outRef = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name).clone(), identType: inTy, subscriptLst: metamodelica::nil() });
    outRef
}

fn createSubClock(mut inPartitionIdx: i32, mut inCnt: i32, mut inExp: Arc<DAE::Exp>) -> Result<(BackendDAE::Var, Arc<BackendDAE::Equation>)> {
    let mut outVar: BackendDAE::Var;
    let mut outEq: Arc<BackendDAE::Equation>;
    let mut ty: Arc<DAE::Type>;
    let mut cr: Arc<DAE::ComponentRef>;
    ty = DAE::T_CLOCK_DEFAULT().clone();
    cr = getSubClkName(inPartitionIdx, inCnt, ty.clone());
    (outVar, outEq) = createEqVarPair(cr, ty, inExp)?;
    Ok((outVar, outEq))
}

fn collectSubclkInfo(mut inEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inRemovedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inPartitionCnt: i32, mut inPartitions: metamodelica::Array<i32>, mut inReqsPartitions: metamodelica::Array<i32>, mut inVars: BackendDAE::Variables, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, metamodelica::Array<i32>)> {
    let mut outNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut outNewVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut outContPartitions: metamodelica::Array<Option<bool>>;
    let mut oClksCnt: metamodelica::Array<i32>;
    let mut eq: Arc<BackendDAE::Equation>;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cnt: i32;
    let mut eq: Arc<BackendDAE::Equation>;
    let mut cr: Arc<DAE::ComponentRef>;
    let mut var: BackendDAE::Var;
    let mut partitionsWhenClocks: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    outContPartitions = arrayCreate(inPartitionCnt, None);
    partitionsWhenClocks = arrayCreate(inPartitionCnt, metamodelica::nil());
    oClksCnt = arrayCreate(inPartitionCnt, 1);
    (outNewEqs, outNewVars) = collectEquationArrayClocks(inEqs, inPartitionCnt, inPartitions.clone(), partitionsWhenClocks.clone(), oClksCnt.clone(), outContPartitions.clone(), inVars.clone(), mT.clone(), metamodelica::nil(), metamodelica::nil())?;
    (outNewEqs, outNewVars) = collectEquationArrayClocks(inRemovedEqs, inPartitionCnt, inReqsPartitions.clone(), partitionsWhenClocks.clone(), oClksCnt.clone(), outContPartitions.clone(), inVars, mT.clone(), outNewEqs, outNewVars)?;
    for mut i in 1..=inPartitionCnt {
        for mut j in &*metamodelica::arrayGet(partitionsWhenClocks.clone(), i)? {
            let mut j = j.clone();
            cnt = metamodelica::arrayGet(oClksCnt.clone(), i)?;
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BackendDAE::WHENCLK_PRREFIX)); __mm_s.push_str(&*intString(j)); ArcStr::from(__mm_s) }).clone(), identType: DAE::T_CLOCK_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            (var, eq) = createSubClock(i, cnt, Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: DAE::T_CLOCK_DEFAULT().clone() }))?;
            outNewEqs = metamodelica::cons(eq.clone(), outNewEqs.clone());
            outNewVars = metamodelica::cons(var.clone(), outNewVars.clone());
            metamodelica::arrayUpdate(oClksCnt.clone(), i, cnt + 1)?;
        }
        if metamodelica::arrayGet(oClksCnt.clone(), i)? == 1 {
            (var, eq) = createSubClock(i, 1, Arc::new(DAE::Exp::CLKCONST { clk: openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK() }))?;
            outNewEqs = metamodelica::cons(eq.clone(), outNewEqs.clone());
            outNewVars = metamodelica::cons(var.clone(), outNewVars.clone());
            metamodelica::arrayUpdate(oClksCnt.clone(), i, 2)?;
        }
    }
    Ok((outNewEqs, outNewVars, outContPartitions, oClksCnt))
}

fn collectEquationArrayClocks(mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut partitionsCnt: i32, mut partitions: metamodelica::Array<i32>, mut partitionsWhenClocks: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut clksCnt: metamodelica::Array<i32>, mut contPartitions: metamodelica::Array<Option<bool>>, mut inVars: BackendDAE::Variables, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inNewVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut outNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inNewEqs.clone();
    let mut outNewVars: Arc<metamodelica::List<BackendDAE::Var>> = inNewVars.clone();
    let mut eq: Arc<BackendDAE::Equation>;
    let mut eqAttr: BackendDAE::EquationAttributes;
    let mut partitionIdx: i32;
    let mut source: SourceInfo;
    for mut i in 1..=BackendEquation::getNumberOfEquations(eqs.clone()) {
        eq = BackendEquation::get(eqs.clone(), i.clone())?;
        partitionIdx = metamodelica::arrayGet(partitions.clone(), i.clone())?;
        let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::equationSource(eq.clone())?) {
            Deref @ DAE::ElementSource { info: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        source = __pa0.clone();
        if partitionIdx != 0 {
            eqAttr = BackendEquation::getEquationAttributes(eq.clone())?;
            eqAttr = (match eqAttr {
        BackendDAE::EquationAttributes { kind: BackendDAE::EquationKind::CLOCKED_EQUATION { clk: mut whenIdx }, .. } => {
            let mut partitionsWhenClocksLst: Arc<metamodelica::List<i32>>;
            partitionsWhenClocksLst = ({let __elt = partitionsWhenClocks.borrow()[(partitionIdx-1) as usize].clone(); __elt});
            if whenIdx.clone() != 0 && List::notMember(whenIdx.clone(), partitionsWhenClocksLst.clone()) {
                metamodelica::arrayUpdate(partitionsWhenClocks.clone(), partitionIdx, metamodelica::cons(whenIdx.clone(), partitionsWhenClocksLst.clone()))?;
            }
            eqAttr.kind = openmodelica_backend_types::BackendDAE::EquationKind::DYNAMIC_EQUATION;
            eqAttr
        },
        _ => {
            eqAttr
        },
    });
            eq = BackendEquation::setEquationAttributes(eq.clone(), eqAttr)?;
            let (__pa1, (__pa2, __pa3, _, _, _, _, _, _, _)) = BackendEquation::traverseExpsOfEquation(eq.clone(), (std::sync::Arc::new(collectSubclkInfoExp1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>))> + 'static>), (outNewEqs.clone(), outNewVars.clone(), contPartitions.clone(), source.clone(), clksCnt.clone(), partitionIdx, partitions.clone(), inVars.clone(), mT.clone()))?;
            eq = __pa1.clone();
            outNewEqs = __pa2.clone();
            outNewVars = __pa3.clone();
            BackendEquation::setAtIndex(eqs.clone(), i.clone(), eq.clone())?;
        }
    }
    Ok((outNewEqs, outNewVars))
}

fn collectSubclkInfoExp1(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>);
    (outExp, outTpl) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new(collectSubclkInfoExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<Option<bool>>, SourceInfo, metamodelica::Array<i32>, i32, metamodelica::Array<i32>, BackendDAE::Variables, metamodelica::Array<Arc<metamodelica::List<i32>>>))> + 'static>), inTpl)?;
    Ok((outExp, outTpl))
}

fn splitClockEqs(mut inEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<bool>)> {
    let mut outClockEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut outClockEqsMask: metamodelica::Array<bool>;
    let mut clockEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eq: Arc<BackendDAE::Equation>;
    let mut i: i32 = 0;
    outClockEqsMask = arrayCreate(BackendEquation::getNumberOfEquations(inEqs.clone()), true);
    for mut i in 1..=BackendEquation::getNumberOfEquations(inEqs.clone()) {
        eq = BackendEquation::get(inEqs.clone(), i)?;
        if isClockEquation(eq.clone())? {
            clockEqs = metamodelica::cons(eq.clone(), clockEqs.clone());
            metamodelica::arrayUpdate(outClockEqsMask.clone(), i, false)?;
        }
    }
    outClockEqs = BackendEquation::listEquation(clockEqs)?;
    Ok((outClockEqs, outClockEqsMask))
}

fn splitClockVars(mut inVars: BackendDAE::Variables) -> Result<(BackendDAE::Variables, metamodelica::Array<bool>)> {
    let mut outClockVars: BackendDAE::Variables;
    let mut outClockVarsMask: metamodelica::Array<bool>;
    let mut clockVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut var: BackendDAE::Var;
    outClockVarsMask = arrayCreate(BackendVariable::varsSize(inVars.clone()), true);
    for mut i in 1..=BackendVariable::varsSize(inVars.clone()) {
        var = BackendVariable::getVarAt(inVars.clone(), i.clone())?;
        if Types::isClockOrSubTypeClock(var.varType.clone()) {
            clockVars = metamodelica::cons(var.clone(), clockVars.clone());
            metamodelica::arrayUpdate(outClockVarsMask.clone(), i.clone(), false)?;
        }
    }
    outClockVars = BackendVariable::listVar(clockVars)?;
    Ok((outClockVars, outClockVarsMask))
}

fn substitutePartitionOpExps(mut inSyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSyst: Arc<BackendDAE::EqSystem> = inSyst.clone();
    let mut newEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut newVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut cnt: i32 = 1;
    for mut eq in &*BackendEquation::equationList(inSyst.orderedEqs.clone())? {
        let mut eq = eq.clone();
        let (__pa0, (__pa1, __pa2, __pa3, _)) = BackendEquation::traverseExpsOfEquation(eq.clone(), (std::sync::Arc::new(substitutePartitionOpExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>))> + 'static>), (newEqs.clone(), newVars.clone(), cnt, inShared.clone()))?;
        eq = __pa0.clone();
        newEqs = __pa1.clone();
        newVars = __pa2.clone();
        cnt = __pa3.clone();
        newEqs = metamodelica::cons(eq.clone(), newEqs.clone());
    }
    assign_field!(
        outSyst.orderedEqs = BackendEquation::listEquation(newEqs.reverse())?,
        outSyst.orderedVars = BackendVariable::addVars(newVars, inSyst.orderedVars.clone())?
    );
    outSyst = BackendDAEUtil::clearEqSyst(outSyst)?;
    Ok(outSyst)
}

fn substitutePartitionOpExp(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>);
    (outExp, outTpl) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new(substitutePartitionOpExp1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>))> + 'static>), inTpl)?;
    Ok((outExp, outTpl))
}

fn substitutePartitionOpExp1(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>)) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>);
    let mut path: Arc<Absyn::Path> = Arc::new(<Absyn::Path as ::std::default::Default>::default());
    let mut shared: Arc<BackendDAE::Shared>;
    let mut attr: Arc<DAE::CallAttributes> = Arc::new(<DAE::CallAttributes as ::std::default::Default>::default());
    let mut clk: Arc<DAE::ClockKind> = Arc::new(DAE::ClockKind::INFERRED_CLOCK);
    let mut cnt: i32;
    let mut newEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut newVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    (newEqs, newVars, cnt, shared) = inTpl.clone();
    (outExp, outTpl) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CLKCONST { clk: __esc_clk } => {
            clk = (*__esc_clk).clone();
            (clk, newEqs, newVars, cnt) = substClock(clk.clone(), newEqs, newVars, cnt, shared.clone())?;
            (Arc::new(DAE::Exp::CLKCONST { clk: clk.clone() }), (newEqs, newVars, cnt, shared))
        },
        Deref @ DAE::Exp::CALL { path: __esc_path, expLst: __esc_exps, attr: __esc_attr } => {
            path = (*__esc_path).clone();
            exps = (*__esc_exps).clone();
            attr = (*__esc_attr).clone();
            substituteExpsCall(path.clone(), exps.clone(), attr.clone(), newEqs, newVars, cnt, shared)?
        },
        _ => (inExp, inTpl),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTpl))
}

fn substClock(mut inClk: Arc<DAE::ClockKind>, mut inNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inNewVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inCnt: i32, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<DAE::ClockKind>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)> {
    let mut outClk: Arc<DAE::ClockKind>;
    let mut outNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut outNewVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut outCnt: i32;
    let mut e: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut i: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut f: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut cnt: i32 = 0;
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    (outClk, outNewEqs, outNewVars, outCnt) = (::match_deref::match_deref! { match &(inClk.clone()) {
        Deref @ DAE::ClockKind::EVENT_CLOCK { condition: __esc_e, startInterval: __esc_f } => {
            e = (*__esc_e).clone();
            f = (*__esc_f).clone();
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(substExp(list![e.clone()], inNewEqs, inNewVars, inCnt)?) {
                (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            eqs = __pa1.clone();
            vars = __pa2.clone();
            cnt = __pa3.clone();
            (Arc::new(DAE::ClockKind::EVENT_CLOCK { condition: e.clone(), startInterval: f.clone() }), eqs, vars, cnt)
        },
        Deref @ DAE::ClockKind::REAL_CLOCK { interval: __esc_e } => {
            e = (*__esc_e).clone();
            (e, eqs, vars, cnt) = substClockExp(e.clone(), inNewEqs, inNewVars, inCnt, inShared)?;
            (Arc::new(DAE::ClockKind::REAL_CLOCK { interval: e.clone() }), eqs, vars, cnt)
        },
        Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: __esc_e, resolution: __esc_i } => {
            e = (*__esc_e).clone();
            i = (*__esc_i).clone();
            (e, eqs, vars, cnt) = substClockExp(e.clone(), inNewEqs, inNewVars, inCnt, inShared)?;
            (Arc::new(DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: e.clone(), resolution: i.clone() }), eqs, vars, cnt)
        },
        _ => (inClk, inNewEqs, inNewVars, inCnt),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClk, outNewEqs, outNewVars, outCnt))
}

fn isKnownOrConstantExp(mut inExp: Arc<DAE::Exp>, mut inKnownVars: BackendDAE::Variables) -> Result<bool> {
    let mut outKnown: bool;
    let (_, (__pa0, _)) = Expression::traverseExpTopDown(inExp, (std::sync::Arc::new(fnptr!(isKnownOrConstantExp_traverser, Arc<DAE::Exp>, (bool, BackendDAE::Variables))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (bool, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, bool, (bool, BackendDAE::Variables))> + 'static>), (true, inKnownVars))?;
    outKnown = __pa0.clone();
    Ok(outKnown)
}

fn isKnownOrConstantExp_traverser(mut inExp: Arc<DAE::Exp>, mut inTpl: (bool, BackendDAE::Variables)) -> (Arc<DAE::Exp>, bool, (bool, BackendDAE::Variables)) {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outContinue: bool;
    let mut outTpl: (bool, BackendDAE::Variables);
    let mut globalKnownVars: BackendDAE::Variables;
    let mut isKnown: bool;
    (isKnown, globalKnownVars) = inTpl;
    isKnown = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CALL { .. } => {
            false
        },
        Deref @ DAE::Exp::CREF { componentRef, .. } => {
            BackendVariable::containsCref(componentRef.clone(), globalKnownVars.clone())
        },
        _ => {
            isKnown
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTpl = (isKnown, globalKnownVars);
    outContinue = isKnown;
    (outExp, outContinue, outTpl)
}

fn substClockExp(mut inExp: Arc<DAE::Exp>, mut inNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inNewVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inCnt: i32, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outNewEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut outNewVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut outCnt: i32;
    if isKnownOrConstantExp(inExp.clone(), inShared.globalKnownVars.clone())? {
        outExp = inExp;
        outNewEqs = inNewEqs;
        outNewVars = inNewVars;
        outCnt = inCnt;
    } else {
        let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(substExp(list![inExp], inNewEqs, inNewVars, inCnt)?) {
            (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, __pa1, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        outExp = __pa0.clone();
        outNewEqs = __pa1.clone();
        outNewVars = __pa2.clone();
        outCnt = __pa3.clone();
    }
    Ok((outExp, outNewEqs, outNewVars, outCnt))
}

fn substituteExpsCall(mut inPath: Arc<Absyn::Path>, mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inAttr: Arc<DAE::CallAttributes>, mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inCnt: i32, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32, Arc<BackendDAE::Shared>);
    let mut replace: bool;
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut cnt: i32;
    replace = (::match_deref::match_deref! { match &((inPath.clone(), (inExps.clone().len() as i32))) {
        (Deref @ Absyn::Path::IDENT { name: Deref @ "hold" }, 1) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, 2) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, 2) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, 2) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, 3) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, 3) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "noClock" }, 1) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exps, eqs, vars, cnt) = if (replace) {substExp(inExps, inEqs, inVars, inCnt)?} else {(inExps, inEqs, inVars, inCnt)};
    outExp = Arc::new(DAE::Exp::CALL { path: inPath, expLst: exps, attr: inAttr });
    outTpl = (eqs, vars, cnt, inShared);
    Ok((outExp, outTpl))
}

fn createVar(mut inComp: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>) -> Result<BackendDAE::Var> {
    let mut outVar: BackendDAE::Var;
    outVar = BackendDAE::Var { varName: inComp, varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: inType.clone(), bindExp: None, tplExp: None, arryDim: metamodelica::nil(), source: DAE::emptyElementSource().clone(), values: DAEUtil::setProtectedAttr(DAEUtil::getEmptyVarAttr(inType), true)?, tearingSelectOption: Some(openmodelica_backend_types::BackendDAE::TearingSelect::DEFAULT), hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
    Ok(outVar)
}

fn createEqVarPair(mut inComp: Arc<DAE::ComponentRef>, mut inType: Arc<DAE::Type>, mut inExp: Arc<DAE::Exp>) -> Result<(BackendDAE::Var, Arc<BackendDAE::Equation>)> {
    let mut outVar: BackendDAE::Var;
    let mut outEq: Arc<BackendDAE::Equation>;
    outVar = createVar(inComp.clone(), inType.clone())?;
    outEq = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: inComp, ty: inType }), scalar: inExp, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
    Ok((outVar, outEq))
}

fn substExp(mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut inCnt: i32) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32)> {
    let mut outTpl: (Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>, i32);
    let mut create: bool;
    let mut e: Arc<DAE::Exp>;
    e = listHead(inExps.clone())?;
    create = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CREF { .. } => false,
        Deref @ DAE::Exp::RCONST { .. } => false,
        Deref @ DAE::Exp::SCONST { .. } => false,
        Deref @ DAE::Exp::BCONST { .. } => false,
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => false,
        Deref @ DAE::Exp::CLKCONST { .. } => true,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTpl = (match create {
        true => {
            let mut cr: Arc<DAE::ComponentRef>;
            let mut ty: Arc<DAE::Type>;
            let mut eq: Arc<BackendDAE::Equation>;
            let mut var: BackendDAE::Var;
            ty = Expression::r#typeof(e.clone())?;
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$var")); __mm_s.push_str(&*intString(inCnt)); ArcStr::from(__mm_s) }).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() });
            (var, eq) = createEqVarPair(cr.clone(), ty.clone(), e)?;
            (metamodelica::cons(Arc::new(DAE::Exp::CREF { componentRef: cr, ty: ty }), listRest(inExps)?), metamodelica::cons(eq, inEqs), metamodelica::cons(var, inVars), inCnt + 1)
        },
        false => {
            (inExps, inEqs, inVars, inCnt)
        },
    });
    Ok(outTpl)
}

fn getVarIxs(mut inComp: Arc<DAE::ComponentRef>, mut inVariables: BackendDAE::Variables) -> Arc<metamodelica::List<i32>> {
    let mut outIntegerLst: Arc<metamodelica::List<i32>>;
    outIntegerLst = 'mc: {
        let __mc_input = inComp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ixs: Arc<metamodelica::List<i32>>;
                    (_, ixs) = BackendVariable::getVar(inComp.clone(), inVariables.clone())?;
                    Ok(ixs.clone())
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
    outIntegerLst
}

fn baseClockPartitioning(mut inSyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outContSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut outClockedSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut outUnpartRemEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut funcs: Arc<AvlTreePathFunction::Tree>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut rm: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut rmT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut partitionCnt: i32;
    let mut i: i32 = 0;
    let mut j: i32;
    let mut cr: Arc<DAE::ComponentRef>;
    let mut varIxs: Arc<metamodelica::List<i32>>;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut eqPartMap: metamodelica::Array<i32>;
    let mut varPartMap: metamodelica::Array<i32>;
    let mut reqsPartition: metamodelica::Array<i32>;
    let mut varsPartition: metamodelica::Array<bool>;
    let mut rvarsPartition: metamodelica::Array<bool>;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut refsInfo: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>;
    let mut refInfo: (Arc<DAE::ComponentRef>, bool) = (Arc::new(DAE::ComponentRef::WILD), false);
    let mut partitionType: Option<bool>;
    let mut isClocked: bool;
    let mut isInitial: bool;
    let mut clockedEqs: metamodelica::Array<Option<bool>>;
    let mut clockedVars: metamodelica::Array<Option<bool>>;
    let mut clockedPartitions: metamodelica::Array<Option<bool>>;
    let mut info: SourceInfo;
    funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
    isInitial = BackendDAEUtil::isInitializationDAE(inShared.clone());
    (syst, m, mT) = BackendDAEUtil::getAdjacencyMatrixfromOption(inSyst.clone(), openmodelica_backend_types::BackendDAE::IndexType::BASECLOCK_IDX, Some(funcs.clone()), isInitial)?;
    (rm, rmT) = BackendDAEUtil::removedAdjacencyMatrix(inSyst, openmodelica_backend_types::BackendDAE::IndexType::BASECLOCK_IDX, Some(funcs.clone()), isInitial)?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    vars = __pa0.clone();
    eqs = __pa1.clone();
    eqPartMap = arrayCreate(metamodelica::arrayLength(m.clone()), 0);
    varPartMap = arrayCreate(metamodelica::arrayLength(mT.clone()), 0);
    reqsPartition = arrayCreate(metamodelica::arrayLength(rm.clone()), 0);
    varsPartition = arrayCreate(metamodelica::arrayLength(mT.clone()), false);
    rvarsPartition = arrayCreate(metamodelica::arrayLength(rmT.clone()), false);
    partitionCnt = partitionIndependentBlocks0(m.clone(), mT.clone(), rm.clone(), rmT.clone(), eqPartMap.clone(), varPartMap.clone(), reqsPartition.clone(), varsPartition.clone(), rvarsPartition.clone())?;
    if partitionCnt > 1 {
        (systs, outUnpartRemEqs, _) = partitionIndependentBlocksSplitBlocks(partitionCnt, syst, eqPartMap.clone(), reqsPartition.clone(), mT.clone(), rmT.clone(), false, funcs, BackendDAEUtil::isInitializationDAE(inShared))?;
    } else {
        (systs, outUnpartRemEqs) = (list![syst], metamodelica::nil());
    }
    clockedEqs = arrayCreate(BackendEquation::getNumberOfEquations(eqs.clone()), None);
    clockedVars = arrayCreate(BackendVariable::varsSize(vars.clone()), None);
    clockedPartitions = arrayCreate(if (partitionCnt > 0) {partitionCnt} else {1}, None);
    j = 0;
    for mut eq in &*BackendEquation::equationList(eqs.clone())? {
        let mut eq = eq.clone();
        j = j + 1;
        (partitionType, refsInfo) = detectEqPartition(eq.clone())?;
        info = BackendEquation::equationInfo(eq.clone())?;
        metamodelica::arrayUpdate(clockedEqs.clone(), j, setClockedPartition(partitionType.clone(), metamodelica::arrayGet(clockedEqs.clone(), j)?, None, info.clone())?)?;
        for mut refInfo in &*refsInfo.clone() {
            let mut refInfo = refInfo.clone();
            (cr, isClocked) = refInfo.clone();
            varIxs = getVarIxs(cr.clone(), vars.clone());
            for mut i in &*varIxs.clone() {
                let mut i = i.clone();
                metamodelica::arrayUpdate(clockedVars.clone(), i, setClockedPartition(Some(isClocked), metamodelica::arrayGet(clockedVars.clone(), i)?, Some(cr.clone()), info.clone())?)?;
            }
        }
    }
    for mut i in 1..=metamodelica::arrayLength(clockedVars.clone()) {
        partitionType = metamodelica::arrayGet(clockedVars.clone(), i)?;
        cr = BackendVariable::varCref(BackendVariable::getVarAt(vars.clone(), i)?)?;
        for mut j in &*metamodelica::arrayGet(mT.clone(), i)? {
            let mut j = j.clone();
            info = BackendEquation::equationInfo(BackendEquation::get(eqs.clone(), j)?)?;
            metamodelica::arrayUpdate(clockedEqs.clone(), j, setClockedPartition(partitionType.clone(), metamodelica::arrayGet(clockedEqs.clone(), j)?, Some(cr.clone()), info.clone())?)?;
        }
    }
    for mut i in 1..=metamodelica::arrayLength(clockedEqs.clone()) {
        partitionType = metamodelica::arrayGet(clockedEqs.clone(), i)?;
        info = BackendEquation::equationInfo(BackendEquation::get(eqs.clone(), i)?)?;
        j = metamodelica::arrayGet(eqPartMap.clone(), i)?;
        metamodelica::arrayUpdate(clockedPartitions.clone(), j, setClockedPartition(partitionType.clone(), metamodelica::arrayGet(clockedPartitions.clone(), j)?, None, info.clone())?)?;
    }
    i = 1;
    for mut syst in &*systs {
        let mut syst = syst.clone();
        (outContSysts, outClockedSysts) = (match metamodelica::arrayGet(clockedPartitions.clone(), i)? {
        Some(false) => (metamodelica::cons(setSystPartition(syst.clone(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::CONTINUOUS_TIME_PARTITION)?, outContSysts.clone()), outClockedSysts.clone()),
        None => (metamodelica::cons(setSystPartition(syst.clone(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNSPECIFIED_PARTITION)?, outContSysts.clone()), outClockedSysts.clone()),
        Some(true) => (outContSysts.clone(), metamodelica::cons(syst.clone(), outClockedSysts.clone())),
        _ => bail!("match: no arm matched"),
    });
        i = i + 1;
    }
    Ok((outContSysts, outClockedSysts, outUnpartRemEqs))
}

fn isClockExp(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut out: bool;
    out = Types::isClockOrSubTypeClock(Expression::r#typeof(inExp)?);
    Ok(out)
}

fn isClockEquation(mut inEq: Arc<BackendDAE::Equation>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: e, .. } => {
            return Ok(isClockExp(e.clone())?)
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e, .. } => {
            return Ok(isClockExp(e.clone())?)
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { body: eq, .. } => {
            { inEq = eq.clone(); continue '__tco; }
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e, .. } => {
            return Ok(isClockExp(e.clone())?)
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. } => {
            return Ok(isClockExp(e.clone())?)
        },
        Deref @ BackendDAE::Equation::ALGORITHM { .. } => {
            return Ok(false)
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: e, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => {
            let mut info: SourceInfo;
            if isClockExp(e.clone())? {
                let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::equationSource(inEq)?) {
                    Deref @ DAE::ElementSource { info: __pa0, .. } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                info = __pa0.clone();
                Error::addSourceMessageAndFail(Error::INVALID_CLOCK_EQUATION.clone(), metamodelica::nil(), info)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            return Ok(false)
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::REINIT { value: e, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => {
            let mut info: SourceInfo;
            if isClockExp(e.clone())? {
                let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::equationSource(inEq)?) {
                    Deref @ DAE::ElementSource { info: __pa0, .. } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                info = __pa0.clone();
                Error::addSourceMessageAndFail(Error::INVALID_CLOCK_EQUATION.clone(), metamodelica::nil(), info)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            return Ok(false)
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e, .. } => {
            return Ok(isClockExp(e.clone())?)
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { eqnstrue: trueEqs, eqnsfalse: falseEqs, .. } => {
            let mut listEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut info: SourceInfo;
            for mut listEqs in &*trueEqs.clone() {
                let mut listEqs = listEqs.clone();
                for mut eq in &*listEqs.clone() {
                    let mut eq = eq.clone();
                    if isClockEquation(eq.clone())? {
                        let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::equationSource(eq.clone())?) {
                            Deref @ DAE::ElementSource { info: __pa0, .. } => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        info = __pa0.clone();
                        Error::addSourceMessageAndFail(Error::INVALID_CLOCK_EQUATION.clone(), metamodelica::nil(), info.clone())?;
                        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    }
                }
            }
            for mut eq in &*falseEqs.clone() {
                let mut eq = eq.clone();
                if isClockEquation(eq.clone())? {
                    let __pa1 = ::match_deref::match_deref! { match &(BackendEquation::equationSource(eq.clone())?) {
                        Deref @ DAE::ElementSource { info: __pa1, .. } => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    info = __pa1.clone();
                    Error::addSourceMessageAndFail(Error::INVALID_CLOCK_EQUATION.clone(), metamodelica::nil(), info.clone())?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                }
            }
            return Ok(false)
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SynchronousFeatures.isClockEquation")); __mm_s.push_str(&*literal!(" failed.\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/SynchronousFeatures.mo"))?;
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn detectEqPartition(mut inEq: Arc<BackendDAE::Equation>) -> Result<(Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>)> {
    let mut outPartitionType: Option<bool>;
    let mut refsInfo: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>;
    let mut partitionType: Option<bool>;
    let mut isClockEq: bool;
    let mut info: SourceInfo;
    partitionType = (match BackendEquation::getEquationAttributes(inEq.clone())? {
        BackendDAE::EquationAttributes { kind: BackendDAE::EquationKind::CLOCKED_EQUATION { .. }, .. } => Some(true),
        _ => None,
    });
    info = BackendEquation::equationInfo(inEq.clone())?;
    let (_, (__pa0, __pa1, _)) = BackendEquation::traverseExpsOfEquation(inEq.clone(), (std::sync::Arc::new(detectEqPartitionExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo))> + 'static>), (partitionType, metamodelica::nil(), info.clone()))?;
    partitionType = __pa0.clone();
    refsInfo = __pa1.clone();
    isClockEq = isClockEquation(inEq)?;
    outPartitionType = if (isClockEq) {setClockedPartition(Some(true), partitionType, None, info)?} else {partitionType};
    Ok((outPartitionType, refsInfo))
}

fn printPartitionType(mut isClockedPartition: Option<bool>) -> ArcStr {
    let mut out: ArcStr;
    out = ((match isClockedPartition {
        Some(false) => literal!("CONT_PARTITION"),
        Some(true) => literal!("CLOCKED_PARTITION"),
        _ => literal!("UNSPECIFIED_PARTITION"),
    })).clone();
    out
}

fn detectEqPartitionExp(mut inExp: Arc<DAE::Exp>, mut inTpl: (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo);
    (outExp, outTpl) = Expression::traverseExpTopDown(inExp, (std::sync::Arc::new(detectEqPartitionExp1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo))> + 'static>), inTpl)?;
    Ok((outExp, outTpl))
}

fn detectEqPartitionExp1(mut inExp: Arc<DAE::Exp>, mut inTpl: (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo)) -> Result<(Arc<DAE::Exp>, bool, (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo))> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut cont: bool;
    let mut outTpl: (Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, SourceInfo);
    let mut refs: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>;
    let mut partition: Option<bool>;
    let mut info: SourceInfo;
    (partition, refs, info) = inTpl;
    (partition, refs, cont) = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::EVENT_CLOCK { condition: e, startInterval: _ } } => {
            let mut cr: Arc<DAE::ComponentRef>;
            let __pa0 = ::match_deref::match_deref! { match &(e.clone()) {
                Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            (partition, metamodelica::cons((cr, false), refs), false)
        },
        Deref @ DAE::Exp::CALL { path, expLst: exps, .. } => {
            detectEqPartitionCall(path.clone(), exps.clone(), refs, partition, info.clone())?
        },
        _ => {
            (partition, refs, true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTpl = (partition, refs, info);
    Ok((outExp, cont, outTpl))
}

fn detectEqPartitionCall(mut inPath: Arc<Absyn::Path>, mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inRefs: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, mut inPartition: Option<bool>, mut info: SourceInfo) -> Result<(Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, bool)> {
    let mut outPartition: Option<bool>;
    let mut outRefs: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>;
    let mut cont: bool;
    (outPartition, outRefs, cont) = (::match_deref::match_deref! { match &((inPath, inExps)) {
        (Deref @ Absyn::Path::IDENT { name: Deref @ "hold" }, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }) => {
            detectEqPartitionCall1(false, true, inPartition, e.clone(), inRefs, info)?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }) => {
            detectEqPartitionCall1(true, false, inPartition, e.clone(), inRefs, info)?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }) => {
            detectEqPartitionCall1(true, true, inPartition, e.clone(), inRefs, info)?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }) => {
            detectEqPartitionCall1(true, true, inPartition, e.clone(), inRefs, info)?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }) => {
            detectEqPartitionCall1(true, true, inPartition, e.clone(), inRefs, info)?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }) => {
            detectEqPartitionCall1(true, true, inPartition, e.clone(), inRefs, info)?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "noClock" }, Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }) => {
            detectEqPartitionCall1(true, true, inPartition, e.clone(), inRefs, info)?
        },
        _ => {
            (inPartition, inRefs, true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outPartition, outRefs, cont))
}

fn detectEqPartitionCall1(mut expClocked: bool, mut refClocked: bool, mut inPartition: Option<bool>, mut inExp: Arc<DAE::Exp>, mut inRefs: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, mut info: SourceInfo) -> Result<(Option<bool>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>, bool)> {
    let mut outPartition: Option<bool>;
    let mut outRefs: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, bool)>>;
    let mut cont: bool = false;
    (outPartition, outRefs) = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ } => {
            (setClockedPartition(Some(expClocked), inPartition, None, info)?, metamodelica::cons((cr.clone(), refClocked), inRefs))
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SynchronousFeatures.detectEqPartitionCall1")); __mm_s.push_str(&*literal!(" failed.\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/SynchronousFeatures.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outPartition, outRefs, cont))
}

fn setSystPartition(mut inSyst: Arc<BackendDAE::EqSystem>, mut inPartitionKind: BackendDAE::BaseClockPartitionKind) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outSyst: Arc<BackendDAE::EqSystem>;
    outSyst = (::match_deref::match_deref! { match &(inSyst) {
        syst @ Deref @ BackendDAE::EqSystem { .. } => {
            let mut syst = (*syst).clone();
            assign_field!(syst.partitionKind = inPartitionKind);
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSyst)
}

fn getPartitionConflictError(mut inComp: Option<Arc<DAE::ComponentRef>>) -> Result<(ErrorTypes::Message, Arc<metamodelica::List<ArcStr>>)> {
    let mut msg: ErrorTypes::Message;
    let mut tokens: Arc<metamodelica::List<ArcStr>>;
    (msg, tokens) = (::match_deref::match_deref! { match &(inComp) {
        Some(cr) => {
            (Error::CONT_CLOCKED_PARTITION_CONFLICT_VAR.clone(), list![(ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone()])
        },
        _ => {
            (Error::CONT_CLOCKED_PARTITION_CONFLICT_EQ.clone(), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((msg, tokens))
}

fn setClockedPartition(mut inNewPartitionType: Option<bool>, mut inOldPartitionType: Option<bool>, mut inComp: Option<Arc<DAE::ComponentRef>>, mut info: SourceInfo) -> Result<Option<bool>> {
    let mut outPartitionType: Option<bool>;
    outPartitionType = (match (inOldPartitionType.clone(), inNewPartitionType.clone()) {
        (None, _) => {
            inNewPartitionType
        },
        (_, None) => {
            inOldPartitionType
        },
        (Some(mut oldVal), Some(mut newVal)) if (oldVal.clone() == newVal.clone()) => {
            inNewPartitionType
        },
        _ => {
            let mut msg: ErrorTypes::Message;
            let mut tokens: Arc<metamodelica::List<ArcStr>>;
            (msg, tokens) = getPartitionConflictError(inComp)?;
            Error::addSourceMessage(msg, tokens, info)?;
            bail!("fail")
        },
    });
    Ok(outPartitionType)
}

pub(crate) fn partitionIndependentBlocks0(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rm: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rmT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>, mut rixs: metamodelica::Array<i32>, mut vars: metamodelica::Array<bool>, mut rvars: metamodelica::Array<bool>) -> Result<i32> {
    let mut on: i32 = 0;
    for mut i in ({let __s=metamodelica::arrayLength(m.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        on = if (partitionIndependentBlocksWork(i.clone(), false, on + 1, m.clone(), mT.clone(), rm.clone(), rmT.clone(), eqPartMap.clone(), varPartMap.clone(), rixs.clone(), vars.clone(), rvars.clone())?) {on + 1} else {on};
    }
    for mut i in ({let __s=metamodelica::arrayLength(rm.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        on = if (partitionIndependentBlocksWork(i.clone(), true, on + 1, m.clone(), mT.clone(), rm.clone(), rmT.clone(), eqPartMap.clone(), varPartMap.clone(), rixs.clone(), vars.clone(), rvars.clone())?) {on + 1} else {on};
    }
    Ok(on)
}

fn partitionIndependentBlocks(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>) -> Result<i32> {
    let mut on: i32 = 0;
    for mut eq in ({let __s=metamodelica::arrayLength(m.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("check eq ")); __mm_s.push_str(&*intString(eq.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        if !(intEq(metamodelica::arrayGet(eqPartMap.clone(), eq.clone())?, -2)) {
            on = if (partitionIndependentBlocks2(eq.clone(), on + 1, m.clone(), mT.clone(), eqPartMap.clone(), varPartMap.clone())?) {on + 1} else {on};
        }
    }
    Ok(on)
}

fn partitionIndependentBlocks2(mut eqIdx: i32, mut partIdx: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>) -> Result<bool> {
    let mut ochange: bool;
    ochange = metamodelica::arrayGet(eqPartMap.clone(), eqIdx)? == -1;
    if ochange {
        metamodelica::arrayUpdate(eqPartMap.clone(), eqIdx, partIdx)?;
        for mut var in &*metamodelica::arrayGet(m.clone(), eqIdx)? {
            let mut var = var.clone();
            if !(intGt(metamodelica::arrayGet(varPartMap.clone(), intAbs(var.clone()))?, 0)) {
                metamodelica::arrayUpdate(varPartMap.clone(), intAbs(var.clone()), partIdx)?;
                for mut newEq in &*metamodelica::arrayGet(mT.clone(), intAbs(var.clone()))? {
                    let mut newEq = newEq.clone();
                    partitionIndependentBlocks2(intAbs(newEq.clone()), partIdx, m.clone(), mT.clone(), eqPartMap.clone(), varPartMap.clone())?;
                }
            }
        }
    }
    Ok(ochange)
}

fn partitionIndependentBlocksMasked(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rm: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rmT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mask: metamodelica::Array<bool>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>, mut remEqPartMap: metamodelica::Array<i32>, mut vars: metamodelica::Array<bool>, mut rvars: metamodelica::Array<bool>) -> Result<i32> {
    let mut on: i32;
    on = 0;
    for mut i in ({let __s=metamodelica::arrayLength(m.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        if ({let __elt = mask.borrow()[(i.clone()-1) as usize].clone(); __elt}) {
            if partitionIndependentBlocksWork(i.clone(), false, on + 1, m.clone(), mT.clone(), rm.clone(), rmT.clone(), eqPartMap.clone(), varPartMap.clone(), remEqPartMap.clone(), vars.clone(), rvars.clone())? {
                on = on + 1;
            }
        }
    }
    for mut i in ({let __s=metamodelica::arrayLength(rm.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        if partitionIndependentBlocksWork(i.clone(), true, on + 1, m.clone(), mT.clone(), rm.clone(), rmT.clone(), eqPartMap.clone(), varPartMap.clone(), remEqPartMap.clone(), vars.clone(), rvars.clone())? {
            on = on + 1;
        }
    }
    Ok(on)
}

fn partitionIndependentBlocksWork(mut idx: i32, mut isRemovedIdx: bool, mut partIdx: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rm: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rmT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqPartMap: metamodelica::Array<i32>, mut varPartMap: metamodelica::Array<i32>, mut rixs: metamodelica::Array<i32>, mut vars: metamodelica::Array<bool>, mut rvars: metamodelica::Array<bool>) -> Result<bool> {
    let mut ochange: bool;
    let mut eqIdx: i32;
    let mut rmIdx: i32;
    let mut workListEq: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut workListRm: Arc<metamodelica::List<i32>> = metamodelica::nil();
    ochange = false;
    if isRemovedIdx {
        if metamodelica::arrayGet(rixs.clone(), idx)? == 0 {
            metamodelica::arrayUpdate(rixs.clone(), idx, partIdx)?;
            workListRm = list![idx];
            ochange = true;
        }
    } else {
        if metamodelica::arrayGet(eqPartMap.clone(), idx)? == 0 {
            metamodelica::arrayUpdate(eqPartMap.clone(), idx, partIdx)?;
            workListEq = list![idx];
            ochange = true;
        }
    }
    if !(ochange) {
        return Ok(ochange.clone());
    }
    while !(workListEq.clone().is_empty() && workListRm.clone().is_empty()) {
        if !(workListEq.clone().is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(workListEq.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            eqIdx = __pa0.clone();
            workListEq = __pa1.clone();
            for mut varIdx in &*metamodelica::arrayGet(m.clone(), eqIdx)? {
                let mut varIdx = varIdx.clone();
                if !(metamodelica::arrayGet(vars.clone(), intAbs(varIdx.clone()))?) {
                    metamodelica::arrayUpdate(vars.clone(), intAbs(varIdx.clone()), true)?;
                    metamodelica::arrayUpdate(varPartMap.clone(), intAbs(varIdx.clone()), partIdx)?;
                    for mut nextEqIdx in &*metamodelica::arrayGet(mT.clone(), intAbs(varIdx.clone()))? {
                        let mut nextEqIdx = nextEqIdx.clone();
                        if metamodelica::arrayGet(eqPartMap.clone(), intAbs(nextEqIdx.clone()))? == 0 {
                            workListEq = metamodelica::cons(intAbs(nextEqIdx.clone()), workListEq.clone());
                            metamodelica::arrayUpdate(eqPartMap.clone(), intAbs(nextEqIdx.clone()), partIdx)?;
                        }
                    }
                    for mut nextEqIdx in &*metamodelica::arrayGet(rmT.clone(), intAbs(varIdx.clone()))? {
                        let mut nextEqIdx = nextEqIdx.clone();
                        if metamodelica::arrayGet(rixs.clone(), intAbs(nextEqIdx.clone()))? == 0 {
                            workListRm = metamodelica::cons(intAbs(nextEqIdx.clone()), workListRm.clone());
                            metamodelica::arrayUpdate(rixs.clone(), intAbs(nextEqIdx.clone()), partIdx)?;
                        }
                    }
                }
            }
        } else {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(workListRm.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            rmIdx = __pa2.clone();
            workListRm = __pa3.clone();
            for mut varIdx in &*metamodelica::arrayGet(rm.clone(), rmIdx)? {
                let mut varIdx = varIdx.clone();
                if !(metamodelica::arrayGet(rvars.clone(), intAbs(varIdx.clone()))?) {
                    metamodelica::arrayUpdate(rvars.clone(), intAbs(varIdx.clone()), true)?;
                    for mut nextEqIdx in &*metamodelica::arrayGet(mT.clone(), intAbs(varIdx.clone()))? {
                        let mut nextEqIdx = nextEqIdx.clone();
                        if metamodelica::arrayGet(eqPartMap.clone(), intAbs(nextEqIdx.clone()))? == 0 {
                            workListEq = metamodelica::cons(intAbs(nextEqIdx.clone()), workListEq.clone());
                            metamodelica::arrayUpdate(eqPartMap.clone(), intAbs(nextEqIdx.clone()), partIdx)?;
                        }
                    }
                    for mut nextEqIdx in &*metamodelica::arrayGet(rmT.clone(), intAbs(varIdx.clone()))? {
                        let mut nextEqIdx = nextEqIdx.clone();
                        if metamodelica::arrayGet(rixs.clone(), intAbs(nextEqIdx.clone()))? == 0 {
                            workListRm = metamodelica::cons(intAbs(nextEqIdx.clone()), workListRm.clone());
                            metamodelica::arrayUpdate(rixs.clone(), intAbs(nextEqIdx.clone()), partIdx)?;
                        }
                    }
                }
            }
        }
    }
    Ok(ochange)
}

pub(crate) fn partitionIndependentBlocksSplitBlocks(mut n: i32, mut inSyst: Arc<BackendDAE::EqSystem>, mut ixs: metamodelica::Array<i32>, mut rixs: metamodelica::Array<i32>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rmT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut throwNoError: bool, mut funcs: Arc<AvlTreePathFunction::Tree>, mut isInitial: bool) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>)> {
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut unpartRemovedEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut varPartMap: metamodelica::Array<i32>;
    let mut ea: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
    let mut rea: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
    let mut va: metamodelica::Array<Arc<metamodelica::List<BackendDAE::Var>>>;
    let mut i1: i32;
    let mut i2: i32;
    let mut b: bool;
    let mut b1: bool = true;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut varsPartition: metamodelica::Array<i32>;
    let mut lstVars: Arc<metamodelica::List<BackendDAE::Var>>;
    ea = arrayCreate(n, metamodelica::nil());
    rea = arrayCreate(n, metamodelica::nil());
    va = arrayCreate(n, metamodelica::nil());
    varPartMap = arrayCreate(n, -1);
    i1 = BackendEquation::equationArraySize(inSyst.orderedEqs.clone())?;
    i2 = BackendVariable::varsSize(inSyst.orderedVars.clone());
    if i1 != i2 && !(throwNoError) {
        Error::addSourceMessage(if (i1 > i2) {Error::OVERDET_EQN_SYSTEM.clone()} else {Error::UNDERDET_EQN_SYSTEM.clone()}, list![ArcStr::from(::std::format!("{}", i1)), ArcStr::from(::std::format!("{}", i2))], Absyn::dummyInfo.clone())?;
        BackendDAEUtil::checkAdjacencyMatrixSolvability(inSyst.clone(), funcs, isInitial)?;
        bail!("fail");
    }
    partitionEquations(inSyst.orderedEqs.clone(), ixs.clone(), ea.clone())?;
    unpartRemovedEqs = partitionEquations(inSyst.removedEqs.clone(), rixs.clone(), rea.clone())?;
    varsPartition = arrayCreate(BackendVariable::varsSize(inSyst.orderedVars.clone()), 0);
    for mut i in 1..=BackendVariable::varsSize(inSyst.orderedVars.clone()) {
        setVarPartition(varsPartition.clone(), i.clone(), ({let __elt = mT.borrow()[(i.clone()-1) as usize].clone(); __elt}), ixs.clone())?;
        setVarPartition(varsPartition.clone(), i.clone(), ({let __elt = rmT.borrow()[(i.clone()-1) as usize].clone(); __elt}), rixs.clone())?;
    }
    for mut i in ({let __s=metamodelica::arrayLength(varsPartition.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        if ({let __elt = varsPartition.borrow()[(i.clone()-1) as usize].clone(); __elt}) != 0 {
            lstVars = ({let __elt = va.borrow()[(({let __elt = varsPartition.borrow()[(i.clone()-1) as usize].clone(); __elt})-1) as usize].clone(); __elt});
            metamodelica::arrayUpdate(va.clone(), ({let __elt = varsPartition.borrow()[(i.clone()-1) as usize].clone(); __elt}), metamodelica::cons(BackendVariable::getVarAt(inSyst.orderedVars.clone(), i.clone())?, lstVars.clone()))?;
        }
    }
    for mut i in 1..=n {
        let (__pa0, (__pa1, _)) = createEqSystem(({let __elt = ea.borrow()[(i.clone()-1) as usize].clone(); __elt}), ({let __elt = va.borrow()[(i.clone()-1) as usize].clone(); __elt}), ({let __elt = rea.borrow()[(i.clone()-1) as usize].clone(); __elt}), (true, throwNoError))?;
        syst = __pa0.clone();
        b = __pa1.clone();
        systs = metamodelica::cons(syst.clone(), systs.clone());
        b1 = b1 && b;
    }
    let true = (throwNoError || b1) else { bail!("pattern mismatch") };
    systs = systs.reverse();
    Ok((systs, unpartRemovedEqs, varPartMap))
}

fn setVarPartition(mut varsPartition: metamodelica::Array<i32>, mut i: i32, mut eqsIxs: Arc<metamodelica::List<i32>>, mut eqsPartitions: metamodelica::Array<i32>) -> Result<()> {
    let mut partitionIdx: i32;
    for mut eq in &*eqsIxs {
        let mut eq = eq.clone();
        partitionIdx = ({let __elt = eqsPartitions.borrow()[(eq.clone()-1) as usize].clone(); __elt});
        if partitionIdx != 0 {
            assert!(({let __elt = varsPartition.borrow()[(i-1) as usize].clone(); __elt}) == 0 || ({let __elt = varsPartition.borrow()[(i-1) as usize].clone(); __elt}) == partitionIdx, "{}", &*(literal!("SynchronousFeatures.setVarPartition failed")).clone());
            metamodelica::arrayUpdate(varsPartition.clone(), i, partitionIdx)?;
        }
    }
    Ok(())
}

fn createEqSystem(mut el: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut vl: Arc<metamodelica::List<BackendDAE::Var>>, mut rel: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iTpl: (bool, bool)) -> Result<(Arc<BackendDAE::EqSystem>, (bool, bool))> {
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut oTpl: (bool, bool);
    let mut arr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut remArr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut i1: i32;
    let mut i2: i32;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    let mut s3: ArcStr;
    let mut s4: ArcStr;
    let mut crs: Arc<metamodelica::List<ArcStr>>;
    let mut success: bool;
    let mut throwNoError: bool;
    (success, throwNoError) = iTpl;
    vars = BackendVariable::listVar1(vl.clone())?;
    arr = BackendEquation::listEquation(el.clone())?;
    remArr = BackendEquation::listEquation(rel)?;
    i1 = BackendEquation::equationArraySize(arr.clone())?;
    i2 = BackendVariable::varsSize(vars.clone());
    if i1 != i2 && !(throwNoError) {
        s1 = (intString(i1)).clone();
        s2 = (intString(i2)).clone();
        crs = List::mapMap(vl, (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>), (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))?;
        s3 = stringDelimitList(crs, (literal!("\n")).clone());
        s4 = (BackendDump::dumpEqnsStr(el)?).clone();
        Error::addSourceMessage(Error::IMBALANCED_EQUATIONS.clone(), list![(s1).clone(), (s2).clone(), (s3).clone(), (s4).clone()], Absyn::dummyInfo.clone())?;
        bail!("fail");
    }
    syst = BackendDAEUtil::createEqSystem(vars, arr, metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, remArr);
    success = success && i1 == i2;
    oTpl = (success, throwNoError);
    Ok((syst, oTpl))
}

fn partitionEquations(mut arr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ixs: metamodelica::Array<i32>, mut ea: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut restEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut ix: i32;
    let mut lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut eq: Arc<BackendDAE::Equation>;
    for mut i in ({let __s=BackendEquation::getNumberOfEquations(arr.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        ix = ({let __elt = ixs.borrow()[(i.clone()-1) as usize].clone(); __elt});
        eq = BackendEquation::get(arr.clone(), i.clone())?;
        if ix == 0 {
            restEqs = metamodelica::cons(eq.clone(), restEqs.clone());
        } else {
            lst = ({let __elt = ea.borrow()[(ix-1) as usize].clone(); __elt});
            lst = metamodelica::cons(eq.clone(), lst.clone());
            metamodelica::arrayUpdate(ea.clone(), ix, lst.clone())?;
        }
    }
    Ok(restEqs)
}

fn subClkEqual(mut sc1: BackendDAE::SubClock, mut sc2: BackendDAE::SubClock) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = (match (sc1.clone(), sc2.clone()) {
        (BackendDAE::SubClock::INFERED_SUBCLOCK { .. }, BackendDAE::SubClock::INFERED_SUBCLOCK { .. }) => true,
        (BackendDAE::SubClock::SUBCLOCK { .. }, BackendDAE::SubClock::SUBCLOCK { .. }) => MMath::equals(var_field!(sc1.factor, BackendDAE::SubClock::SUBCLOCK).clone(), var_field!(sc2.factor, BackendDAE::SubClock::SUBCLOCK).clone())? && MMath::equals(var_field!(sc1.shift, BackendDAE::SubClock::SUBCLOCK).clone(), var_field!(sc2.shift, BackendDAE::SubClock::SUBCLOCK).clone())? && Util::optionEqual(var_field!(sc1.solver, BackendDAE::SubClock::SUBCLOCK).clone(), var_field!(sc2.solver, BackendDAE::SubClock::SUBCLOCK).clone(), (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?,
        _ => false,
    });
    Ok(isEqual)
}

fn subClockTreeString(mut treeIn: metamodelica::Array<(BackendDAE::SubClock, i32)>) -> Result<ArcStr> {
    let mut sOut: ArcStr = literal!("");
    let mut tpl: (BackendDAE::SubClock, i32) = (BackendDAE::SubClock::INFERED_SUBCLOCK, 0);
    let mut subClock: BackendDAE::SubClock;
    let mut i: i32;
    let mut idx: i32 = 1;
    let __range0 = treeIn.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut tpl in __range0 {
        (subClock, i) = tpl.clone();
        sOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(idx)); __mm_s.push_str(&*literal!(": [")); __mm_s.push_str(&*intString(i)); __mm_s.push_str(&*literal!("]:  ")); __mm_s.push_str(&*BackendDump::subClockString(subClock.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*sOut.clone()); ArcStr::from(__mm_s) }).clone();
        idx = idx + 1;
    }
    Ok(sOut)
}

