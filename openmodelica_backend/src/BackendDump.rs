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

use crate::BackendDAEUtil;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::DumpHTML;
use crate::GraphvizDump;
use crate::HpcOmTaskGraph;
use crate::Initialization;
use crate::Matching;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_types::ZeroCrossings;
use openmodelica_codegen_graphml::GraphML;
use openmodelica_frontend::HashSet;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::ExpressionDumpTpl;
use openmodelica_frontend_types::DAE;
use openmodelica_tpl::Tpl;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::IOStream;
use openmodelica_util::MMath;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// section for all print* functions
//
// These are functions, that print directly to the standard-stream.
//   - printBackendDAE
//   - printEqSystem
//   - printEquation
//   - printEquationArray
//   - printEquationList
//   - printEquations
//   - printClassAttributes
//   - printShared
//   - printStateSets
//   - printVar
//   - printVariables
//   - printVarList
// =============================================================================
pub(crate) fn printBackendDAE(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<()> {
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inBackendDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqs = __pa0.clone();
    shared = __pa1.clone();
    List::map_0(eqs, (std::sync::Arc::new(printEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<()> + 'static>))?;
    metamodelica::print((literal!("\n")).clone());
    printShared(shared)?;
    Ok(())
}

pub(crate) fn printEqSystem(mut inSyst: Arc<BackendDAE::EqSystem>) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*partitionKindString(inSyst.partitionKind.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    dumpVariables(inSyst.orderedVars.clone(), (literal!("Variables")).clone())?;
    dumpEquationArray(inSyst.orderedEqs.clone(), (literal!("Equations")).clone())?;
    dumpEquationArray(inSyst.removedEqs.clone(), (literal!("Simple Equations")).clone())?;
    dumpStateSets(inSyst.stateSets.clone(), (literal!("State Sets")).clone())?;
    dumpOption(inSyst.m.clone(), (std::sync::Arc::new(dumpAdjacencyMatrix) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> + 'static>))?;
    dumpOption(inSyst.mT.clone(), (std::sync::Arc::new(dumpAdjacencyMatrixT) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> + 'static>))?;
    metamodelica::print((literal!("\n")).clone());
    dumpFullMatching(inSyst.matching.clone(), Some(inSyst))?;
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

pub(crate) fn printEquation(mut inEquation: Arc<BackendDAE::Equation>) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*equationString(inEquation)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn printEquationArray(mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<()> {
    List::fold(BackendEquation::equationList(eqns)?, (std::sync::Arc::new(printEquationList2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (i32, i32)) -> Result<(i32, i32)> + 'static>), (1, 1))?;
    Ok(())
}

pub(crate) fn printEquationList(mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<()> {
    List::fold(eqns, (std::sync::Arc::new(printEquationList2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (i32, i32)) -> Result<(i32, i32)> + 'static>), (1, 1))?;
    Ok(())
}

fn printEquationList2(mut inEquation: Arc<BackendDAE::Equation>, mut inInteger: (i32, i32)) -> Result<(i32, i32)> {
    let mut oInteger: (i32, i32);
    let mut iscalar: i32;
    let mut i: i32;
    let mut size: i32;
    let mut attr: BackendDAE::EquationAttributes;
    (i, iscalar) = inInteger;
    size = BackendEquation::equationSize(inEquation.clone())?;
    attr = BackendEquation::getEquationAttributes(inEquation.clone())?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(i)); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(iscalar)); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(size)); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*equationString(inEquation)?); __mm_s.push_str(&*literal!("   ")); __mm_s.push_str(&*equationAttrString(attr)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    oInteger = (i + 1, iscalar + size);
    Ok(oInteger)
}

pub(crate) fn equationListString(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut heading: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(heading.clone()) {
        Deref @ "" => {
            let mut buffer: ArcStr;
            (_, _, buffer) = List::fold(inEqns, (std::sync::Arc::new(equationList2String) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (i32, i32, ArcStr)) -> Result<(i32, i32, ArcStr)> + 'static>), (1, 1, literal!("")))?;
            buffer.clone()
        },
        _ => {
            let mut buffer: ArcStr;
            (_, _, buffer) = List::fold(inEqns, (std::sync::Arc::new(equationList2String) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (i32, i32, ArcStr)) -> Result<(i32, i32, ArcStr)> + 'static>), (1, 1, literal!("")))?;
            buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*buffer.clone()); ArcStr::from(__mm_s) }).clone();
            buffer.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn equationList2String(mut inEquation: Arc<BackendDAE::Equation>, mut inTuple: (i32, i32, ArcStr)) -> Result<(i32, i32, ArcStr)> {
    let mut outTuple: (i32, i32, ArcStr);
    let mut iscalar: i32;
    let mut i: i32;
    let mut size: i32;
    let mut buffer: ArcStr;
    (i, iscalar, buffer) = inTuple;
    size = BackendEquation::equationSize(inEquation.clone())?;
    buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*buffer); __mm_s.push_str(&*intString(i)); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(iscalar)); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(size)); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*equationString(inEquation)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    outTuple = (i + 1, iscalar + size, buffer);
    Ok(outTuple)
}

pub(crate) fn printEquations(mut inIntegerLst: Arc<metamodelica::List<i32>>, mut syst: Arc<BackendDAE::EqSystem>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inIntegerLst) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: n, tail: rest } => {
            printEquations(rest.clone(), syst.clone())?;
            printEquationNo(n.clone(), syst)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printEquationNo(mut inInteger: i32, mut syst: Arc<BackendDAE::EqSystem>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inInteger, syst)) {
        (eqno, Deref @ BackendDAE::EqSystem { orderedEqs: eqns, .. }) => {
            let mut eq: Arc<BackendDAE::Equation>;
            eq = BackendEquation::get(eqns.clone(), eqno.clone())?;
            printEquation(eq.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn printClassAttributes(mut optimicaFun: Arc<DAE::ClassAttributes>) -> Result<()> {
    let mut e1: Option<Arc<DAE::Exp>>;
    let mut e2: Option<Arc<DAE::Exp>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(optimicaFun) {
        Deref @ DAE::ClassAttributes { objetiveE: __pa0, objectiveIntegrandE: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    e2 = __pa1.clone();
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Mayer")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    metamodelica::print((ExpressionDump::printOptExpStr(e1)?).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Lagrange")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    metamodelica::print((ExpressionDump::printOptExpStr(e2)?).clone());
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

pub(crate) fn printShared(mut inShared: Arc<BackendDAE::Shared>) -> Result<()> {
    metamodelica::print((literal!("\nBackendDAEType: ")).clone());
    printBackendDAEType(inShared.backendDAEType.clone())?;
    metamodelica::print((literal!("\n\n")).clone());
    dumpVariables(inShared.globalKnownVars.clone(), (literal!("Known variables only depending on parameters and constants - globalKnownVars")).clone())?;
    dumpVariables(inShared.localKnownVars.clone(), (literal!("Known variables only depending on states and inputs - localKnownVars")).clone())?;
    dumpVariables(inShared.externalObjects.clone(), (literal!("External Objects")).clone())?;
    dumpExternalObjectClasses(inShared.extObjClasses.clone(), (literal!("Classes of External Objects")).clone())?;
    dumpVariables(inShared.aliasVars.clone(), (literal!("Alias Variables")).clone())?;
    dumpEquationArray(inShared.removedEqs.clone(), (literal!("Simple Shared Equations")).clone())?;
    dumpEquationArray(inShared.initialEqs.clone(), (literal!("Initial Equations")).clone())?;
    dumpZeroCrossingList(ZeroCrossings::toList(inShared.eventInfo.zeroCrossings.clone()), (literal!("Zero Crossings")).clone())?;
    dumpZeroCrossingList(DoubleEnded::toListNoCopyNoClear(inShared.eventInfo.relations.clone()), (literal!("Relations")).clone())?;
    if stringEqual((Config::simCodeTarget()?).clone(), (literal!("Cpp")).clone()) {
        dumpZeroCrossingList(ZeroCrossings::toList(inShared.eventInfo.samples.clone()), (literal!("Samples")).clone())?;
    } else {
        dumpTimeEvents(inShared.eventInfo.timeEvents.clone(), (literal!("Time Events")).clone())?;
    }
    dumpConstraintList(inShared.constraints.clone(), (literal!("Constraints")).clone())?;
    dumpBasePartitions(inShared.partitionsInfo.basePartitions.clone(), (literal!("Base partitions")).clone())?;
    dumpSubPartitions(inShared.partitionsInfo.subPartitions.clone(), (literal!("Sub partitions")).clone())?;
    if Flags::isSet(Flags::DUMP_FUNCTIONS.clone())? {
        DAEDump::dumpFunctionTree(inShared.functionTree.clone(), (literal!("Functions")).clone())?;
    }
    Ok(())
}

pub(crate) fn printBasePartitions(mut basePartitions: metamodelica::Array<BackendDAE::BasePartition>) -> Result<()> {
    let mut clkExpStr: ArcStr;
    let mut nSubClocksStr: ArcStr;
    for mut i in 1..=metamodelica::arrayLength(basePartitions.clone()) {
        clkExpStr = (Tpl::tplString2((std::sync::Arc::new(ExpressionDumpTpl::dumpClockKind) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<DAE::ClockKind>, ArcStr) -> Result<Tpl::Text> + 'static>), ({let __elt = basePartitions.borrow()[(i.clone()-1) as usize].clock.clone(); __elt}), (literal!("")).clone())?).clone();
        nSubClocksStr = (intString(({let __elt = basePartitions.borrow()[(i.clone()-1) as usize].nSubClocks.clone(); __elt}))).clone();
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*clkExpStr.clone()); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*nSubClocksStr.clone()); __mm_s.push_str(&*literal!("]")); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

pub(crate) fn printSubPartitions(mut subPartitions: metamodelica::Array<BackendDAE::SubPartition>) -> Result<()> {
    let mut subClockStr: ArcStr;
    let mut eventStr: ArcStr;
    for mut i in 1..=metamodelica::arrayLength(subPartitions.clone()) {
        subClockStr = (subClockString(({let __elt = subPartitions.borrow()[(i.clone()-1) as usize].clock.clone(); __elt}))?).clone();
        eventStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("event(")); __mm_s.push_str(&*boolString(({let __elt = subPartitions.borrow()[(i.clone()-1) as usize].holdEvents.clone(); __elt}))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*subClockStr.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*eventStr.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

pub(crate) fn subClockString(mut subClock: BackendDAE::SubClock) -> Result<ArcStr> {
    let mut subClockString: ArcStr = arcstr::literal!("");
    subClockString = ((match subClock.clone() {
        BackendDAE::SubClock::INFERED_SUBCLOCK { .. } => {
            literal!("INFERED_SUBCLOCK")
        },
        BackendDAE::SubClock::SUBCLOCK { factor: _, .. } => {
            let mut factorStr: ArcStr;
            let mut shiftStr: ArcStr;
            let mut solverStr: ArcStr;
            factorStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("factor(")); __mm_s.push_str(&*MMath::rationalString(var_field!(subClock.factor, BackendDAE::SubClock::SUBCLOCK).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            shiftStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("shift(")); __mm_s.push_str(&*MMath::rationalString(var_field!(subClock.shift, BackendDAE::SubClock::SUBCLOCK).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            solverStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("solver(")); __mm_s.push_str(&*optionString(var_field!(subClock.solver, BackendDAE::SubClock::SUBCLOCK).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            if ((solverStr.clone()).clone().len() as i32) > 8 {
                subClockString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*factorStr.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*shiftStr.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*solverStr.clone()); ArcStr::from(__mm_s) }).clone();
            } else {
                subClockString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*factorStr.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*shiftStr.clone()); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
            }
            subClockString
        },
    })).clone();
    Ok(subClockString)
}

pub(crate) fn optionString(mut option: Option<ArcStr>) -> ArcStr {
    let mut optionString: ArcStr;
    optionString = ((match option {
        Some(mut s) => {
            s.clone()
        },
        _ => {
            literal!("")
        },
    })).clone();
    optionString
}

pub(crate) fn printBackendDAEType(mut btp: BackendDAE::BackendDAEType) -> Result<()> {
    metamodelica::print((printBackendDAEType2String(btp)?).clone());
    Ok(())
}

pub(crate) fn printBackendDAEType2String(mut btp: BackendDAE::BackendDAEType) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match btp {
        BackendDAE::BackendDAEType::SIMULATION { .. } => literal!("simulation"),
        BackendDAE::BackendDAEType::JACOBIAN { .. } => literal!("jacobian"),
        BackendDAE::BackendDAEType::ALGEQSYSTEM { .. } => literal!("algebraic loop"),
        BackendDAE::BackendDAEType::ARRAYSYSTEM { .. } => literal!("multidim equation arrays"),
        BackendDAE::BackendDAEType::PARAMETERSYSTEM { .. } => literal!("parameter system"),
        BackendDAE::BackendDAEType::INITIALSYSTEM { .. } => literal!("initialization"),
        BackendDAE::BackendDAEType::INLINESYSTEM { .. } => literal!("inline system"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub(crate) fn printStateSets(mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>>) -> Result<()> {
    List::map_0(stateSets, (std::sync::Arc::new(printStateSet) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::StateSet) -> Result<()> + 'static>))?;
    Ok(())
}

fn printStateSet(mut inStateSet: BackendDAE::StateSet) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("StateSet \"")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(ComponentReferenceBasics::crefFirstCref(inStateSet.crA.clone())?)?); __mm_s.push_str(&*literal!("\" (rang ")); __mm_s.push_str(&*intString(inStateSet.rang.clone())); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    dumpVarList(inStateSet.statescandidates.clone(), (literal!("state candidates")).clone())?;
    dumpEquationList(inStateSet.eqns.clone(), (literal!("eqns")).clone())?;
    dumpVarList(inStateSet.ovars.clone(), (literal!("ovars")).clone())?;
    dumpEquationList(inStateSet.oeqns.clone(), (literal!("oeqns")).clone())?;
    dumpVarList(inStateSet.varA.clone(), (literal!("varA")).clone())?;
    dumpVarList(inStateSet.varJ.clone(), (literal!("varJ")).clone())?;
    Ok(())
}

pub(crate) fn printVar(mut inVar: BackendDAE::Var) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*varString(inVar)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn printVariables(mut vars: BackendDAE::Variables) -> Result<()> {
    List::fold(BackendVariable::varList(vars)?, (std::sync::Arc::new(printVars1) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, i32) -> Result<i32> + 'static>), 1)?;
    Ok(())
}

pub(crate) fn printVarList(mut vars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<()> {
    List::fold(vars, (std::sync::Arc::new(printVars1) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, i32) -> Result<i32> + 'static>), 1)?;
    Ok(())
}

fn printVars1(mut inVar: BackendDAE::Var, mut inVarNo: i32) -> Result<i32> {
    let mut outVarNo: i32;
    metamodelica::print((intString(inVarNo)).clone());
    metamodelica::print((literal!(": ")).clone());
    printVar(inVar)?;
    outVarNo = inVarNo + 1;
    Ok(outVarNo)
}

pub(crate) fn varListString(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut heading: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(heading.clone()) {
        Deref @ "" => {
            let mut buffer: ArcStr;
            (_, buffer) = List::fold(inVars, (std::sync::Arc::new(var1String) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, literal!("")))?;
            buffer.clone()
        },
        _ => {
            let mut buffer: ArcStr;
            (_, buffer) = List::fold(inVars, (std::sync::Arc::new(var1String) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, literal!("")))?;
            buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*buffer.clone()); ArcStr::from(__mm_s) }).clone();
            buffer.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn var1String(mut inVar: BackendDAE::Var, mut inTpl: (i32, ArcStr)) -> Result<(i32, ArcStr)> {
    let mut outTpl: (i32, ArcStr);
    let mut varNo: i32;
    let mut buffer: ArcStr;
    (varNo, buffer) = inTpl;
    buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*buffer); __mm_s.push_str(&*intString(varNo)); __mm_s.push_str(&*literal!(": ")); ArcStr::from(__mm_s) }).clone();
    buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*buffer); __mm_s.push_str(&*varString(inVar)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    outTpl = (varNo + 1, buffer);
    Ok(outTpl)
}

pub(crate) fn varListStringShort(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut heading: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(heading.clone()) {
        Deref @ "" => {
            let mut buffer: ArcStr;
            (_, buffer) = List::fold(inVars, (std::sync::Arc::new(varNameString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, literal!("")))?;
            buffer.clone()
        },
        _ => {
            let mut buffer: ArcStr;
            (_, buffer) = List::fold(inVars, (std::sync::Arc::new(varNameString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, literal!("")))?;
            buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*buffer.clone()); ArcStr::from(__mm_s) }).clone();
            buffer.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn varNameString(mut inVar: BackendDAE::Var, mut inTpl: (i32, ArcStr)) -> Result<(i32, ArcStr)> {
    let mut outTpl: (i32, ArcStr);
    let mut varNo: i32;
    let mut buffer: ArcStr;
    (varNo, buffer) = inTpl;
    buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*buffer); __mm_s.push_str(&*intString(varNo)); __mm_s.push_str(&*literal!(": ")); ArcStr::from(__mm_s) }).clone();
    buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*buffer); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inVar.varName.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    outTpl = (varNo + 1, buffer);
    Ok(outTpl)
}

pub(crate) fn varListStringIndented(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut heading: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(heading.clone()) {
        Deref @ "" => {
            let mut buffer: ArcStr;
            (_, buffer) = List::fold(inVars, (std::sync::Arc::new(var1StringIndented) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, literal!("")))?;
            buffer.clone()
        },
        _ => {
            let mut buffer: ArcStr;
            (_, buffer) = List::fold(inVars, (std::sync::Arc::new(var1StringIndented) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, literal!("")))?;
            buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*buffer.clone()); ArcStr::from(__mm_s) }).clone();
            buffer.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn var1StringIndented(mut inVar: BackendDAE::Var, mut inTpl: (i32, ArcStr)) -> Result<(i32, ArcStr)> {
    let mut outTpl: (i32, ArcStr);
    let mut varNo: i32;
    let mut buffer: ArcStr;
    (varNo, buffer) = inTpl;
    buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*buffer); __mm_s.push_str(&*literal!("   ")); __mm_s.push_str(&*intString(varNo)); __mm_s.push_str(&*literal!(": ")); ArcStr::from(__mm_s) }).clone();
    buffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*buffer); __mm_s.push_str(&*varString(inVar)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    outTpl = (varNo + 1, buffer);
    Ok(outTpl)
}

fn printExternalObjectClasses(mut cls: Arc<metamodelica::List<BackendDAE::ExternalObjectClass>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(cls) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: BackendDAE::ExternalObjectClass { path, source }, tail: _ } => {
            let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut paths_lst: Arc<metamodelica::List<ArcStr>>;
            let mut path_str: ArcStr;
            metamodelica::print((literal!("class ")).clone());
            metamodelica::print((AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone());
            metamodelica::print((literal!("\n  extends ExternalObject;")).clone());
            metamodelica::print((literal!("\n origin: ")).clone());
            paths = ElementSource::getElementSourceTypes(source.clone());
            paths_lst = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut p in (paths.clone()).into_iter().cloned() {
            let __x = AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            path_str = stringDelimitList(paths_lst.clone(), (literal!(", ")).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*path_str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            metamodelica::print((literal!("end ")).clone());
            metamodelica::print((AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn printSparsityPatternCrefs(mut inPattern: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>) -> Result<()> {
    for mut e in &*inPattern {
        let mut e = e.clone();
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(Util::tuple21(e.clone()))?); __mm_s.push_str(&*literal!(" affects the following (")); __mm_s.push_str(&*intString((Util::tuple22(e.clone()).len() as i32))); __mm_s.push_str(&*literal!(") outputs\n  ")); ArcStr::from(__mm_s) }).clone());
        ComponentReference::printComponentRefList(Util::tuple22(e.clone()))?;
    }
    Ok(())
}

// =============================================================================
// section for all graphviz* functions
//
// =============================================================================
pub(crate) fn graphvizBackendDAE(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inFileNameSuffix: ArcStr) -> Result<()> {
    let mut dae: Arc<BackendDAE::BackendDAE>;
    dae = setAdjacencyMatrix(inBackendDAE)?;
    Tpl::tplNoret2((std::sync::Arc::new(GraphvizDump::dumpBackendDAE) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<BackendDAE::BackendDAE>, ArcStr) -> Result<Tpl::Text> + 'static>), dae, (inFileNameSuffix).clone())?;
    Ok(())
}

pub(crate) fn graphvizAdjacencyMatrix(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inFileNameSuffix: ArcStr) -> Result<()> {
    let mut dae: Arc<BackendDAE::BackendDAE>;
    dae = setAdjacencyMatrix(inBackendDAE)?;
    Tpl::tplNoret2((std::sync::Arc::new(GraphvizDump::dumpAdjacencyMatrix) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<BackendDAE::BackendDAE>, ArcStr) -> Result<Tpl::Text> + 'static>), dae, (inFileNameSuffix).clone())?;
    Ok(())
}

fn setAdjacencyMatrix(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: Arc<BackendDAE::BackendDAE>;
    let mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inBackendDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqSystems = __pa0.clone();
    shared = __pa1.clone();
    eqSystems = List::map1(eqSystems, (std::sync::Arc::new(setAdjacencyMatrix1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, bool) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), BackendDAEUtil::isInitializationDAE(shared.clone()))?;
    outBackendDAE = Arc::new(BackendDAE::BackendDAE { eqs: eqSystems, shared: shared });
    Ok(outBackendDAE)
}

fn setAdjacencyMatrix1(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut isInitial: bool) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem>;
    (outEqSystem, _, _) = BackendDAEUtil::getAdjacencyMatrix(inEqSystem, openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, isInitial)?;
    Ok(outEqSystem)
}

// =============================================================================
// section for all dump* functions
//
// These are functions, that print directly to the standard-stream and separates
// there output (e.g. with some kind of headings).
//   - dumpBackendDAE
//   - dumpBackendDAEEqnList
//   - dumpBackendDAEVarList
//   - dumpComponent
//   - dumpComponents
//   - dumpComponentsAdvanced
//   - dumpEqnsSolved
//   - dumpEqSystem
//   - dumpEqSystems
//   - dumpEquationArray
//   - dumpEquationList
//   - dumpHashSet
//   - dumpSparsityPattern
//   - dumpTearing
//   - dumpVariables
//   - dumpVarList
// =============================================================================
pub(crate) const BORDER: &'static str = "########################################";

pub(crate) const UNDERLINE: &'static str = "========================================";

pub(crate) fn dumpDAE(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    dumpBackendDAE(inDAE, (literal!("dumpDAE")).clone())?;
    Ok(outDAE)
}

pub(crate) fn dumpBackendDAE(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut heading: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    printBackendDAE(inBackendDAE)?;
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

pub(crate) fn dumpEqSystem(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut heading: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    printEqSystem(inEqSystem)?;
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

pub(crate) fn dumpEqSystemShort(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut heading: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*partitionKindString(inEqSystem.partitionKind.clone())?); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    dumpVariables(inEqSystem.orderedVars.clone(), (literal!("Variables")).clone())?;
    dumpEquationArray(inEqSystem.orderedEqs.clone(), (literal!("Equations")).clone())?;
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

pub(crate) fn dumpEqSystems(mut inEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut heading: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((inEqSystems.clone().len() as i32))); __mm_s.push_str(&*literal!(" partitions)\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    List::map_0(inEqSystems, (std::sync::Arc::new(printEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<()> + 'static>))?;
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

pub(crate) fn dumpBasePartitions(mut basePartitions: metamodelica::Array<BackendDAE::BasePartition>, mut heading: ArcStr) -> Result<()> {
    if metamodelica::arrayLength(basePartitions.clone()) > 0 {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(metamodelica::arrayLength(basePartitions.clone()))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        printBasePartitions(basePartitions.clone())?;
        metamodelica::print((literal!("\n")).clone());
    }
    Ok(())
}

pub(crate) fn dumpSubPartitions(mut subPartitions: metamodelica::Array<BackendDAE::SubPartition>, mut heading: ArcStr) -> Result<()> {
    if metamodelica::arrayLength(subPartitions.clone()) > 0 {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(metamodelica::arrayLength(subPartitions.clone()))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        printSubPartitions(subPartitions.clone())?;
        metamodelica::print((literal!("\n")).clone());
    }
    Ok(())
}

pub fn dumpVariables(mut inVars: BackendDAE::Variables, mut heading: ArcStr) -> Result<()> {
    if BackendVariable::varsSize(inVars.clone()) > 0 {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BackendVariable::varsSize(inVars.clone()))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        printVariables(inVars)?;
        metamodelica::print((literal!("\n")).clone());
    }
    Ok(())
}

pub fn dumpVarList(mut inVars: Arc<metamodelica::List<BackendDAE::Var>>, mut heading: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((inVars.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    printVarList(inVars)?;
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

pub fn dumpEquationArray(mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut heading: ArcStr) -> Result<()> {
    if BackendEquation::getNumberOfEquations(inEqns.clone()) + BackendEquation::equationArraySize(inEqns.clone())? > 0 {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BackendEquation::getNumberOfEquations(inEqns.clone()))); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(BackendEquation::equationArraySize(inEqns.clone())?)); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        printEquationArray(inEqns)?;
        metamodelica::print((literal!("\n")).clone());
    }
    Ok(())
}

pub fn dumpEquationList(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut heading: ArcStr) -> Result<()> {
    if !(inEqns.clone().is_empty()) {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((inEqns.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        printEquationList(inEqns)?;
        metamodelica::print((literal!("\n")).clone());
    }
    Ok(())
}

fn dumpExternalObjectClasses(mut inEOC: Arc<metamodelica::List<BackendDAE::ExternalObjectClass>>, mut heading: ArcStr) -> Result<()> {
    if !(inEOC.clone().is_empty()) {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((inEOC.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        printExternalObjectClasses(inEOC)?;
        metamodelica::print((literal!("\n")).clone());
    }
    Ok(())
}

pub(crate) fn dumpStateSets(mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>>, mut heading: ArcStr) -> Result<()> {
    if !(stateSets.clone().is_empty()) {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        printStateSets(stateSets)?;
        metamodelica::print((literal!("\n")).clone());
    }
    Ok(())
}

pub(crate) fn dumpZeroCrossingList(mut inZeroCrossingList: Arc<metamodelica::List<BackendDAE::ZeroCrossing>>, mut heading: ArcStr) -> Result<()> {
    let mut zeroCrossing: BackendDAE::ZeroCrossing = <BackendDAE::ZeroCrossing as ::std::default::Default>::default();
    if !(inZeroCrossingList.clone().is_empty()) {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((inZeroCrossingList.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        for mut zeroCrossing in &*inZeroCrossingList {
            let mut zeroCrossing = zeroCrossing.clone();
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*zeroCrossingString(zeroCrossing.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        metamodelica::print((literal!("\n")).clone());
    }
    Ok(())
}

pub(crate) fn dumpTimeEvents(mut inTimeEvents: Arc<metamodelica::List<BackendDAE::TimeEvent>>, mut heading: ArcStr) -> Result<()> {
    let mut timeEvent: BackendDAE::TimeEvent = BackendDAE::TimeEvent::SIMPLE_TIME_EVENT;
    if !(inTimeEvents.clone().is_empty()) {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((inTimeEvents.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        for mut timeEvent in &*inTimeEvents {
            let mut timeEvent = timeEvent.clone();
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*timeEventString(timeEvent.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        metamodelica::print((literal!("\n")).clone());
    }
    Ok(())
}

fn dumpConstraintList(mut inConstraintArray: Arc<metamodelica::List<Arc<DAE::Constraint>>>, mut heading: ArcStr) -> Result<()> {
    if !(inConstraintArray.clone().is_empty()) {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((inConstraintArray.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        dumpConstraints(inConstraintArray, 0)?;
        metamodelica::print((literal!("\n")).clone());
    }
    Ok(())
}

pub(crate) fn dumpHashSet(mut hashSet: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut heading: ArcStr) -> Result<()> {
    let mut size: i32;
    size = BaseHashSet::currentSize(hashSet.clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(size)); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    BaseHashSet::printHashSet(hashSet)?;
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

pub(crate) fn dumpSparsityPattern(mut inPattern: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), mut heading: ArcStr) -> Result<()> {
    let mut pattern: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>;
    let mut patternT: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>;
    let mut diffVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut diffedVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut nnz: i32;
    let (__pa0, __pa1, (__pa2, __pa3), __pa4) = inPattern;
    pattern = __pa0.clone();
    patternT = __pa1.clone();
    diffVars = __pa2.clone();
    diffedVars = __pa3.clone();
    nnz = __pa4.clone();
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of non zero elements: ")); __mm_s.push_str(&*intString(nnz)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Independents [or inputs] (")); __mm_s.push_str(&*intString((diffVars.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    ComponentReference::printComponentRefList(diffVars)?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Dependents [or outputs] (")); __mm_s.push_str(&*intString((diffedVars.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    ComponentReference::printComponentRefList(diffedVars)?;
    printSparsityPatternCrefs(pattern)?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("Transposed pattern")); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    printSparsityPatternCrefs(patternT)?;
    Ok(())
}

pub(crate) fn dumpSparseColoring(mut inColoring: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, mut heading: ArcStr) -> Result<()> {
    let mut i: i32 = 0;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of colors: ")); __mm_s.push_str(&*intString((inColoring.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    for mut crList in &*inColoring {
        let mut crList = crList.clone();
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following (")); __mm_s.push_str(&*intString((crList.clone().len() as i32))); __mm_s.push_str(&*literal!(") independents belong to one color\n")); __mm_s.push_str(&*intString(i)); __mm_s.push_str(&*literal!(": ")); ArcStr::from(__mm_s) }).clone());
        ComponentReference::printComponentRefList(crList.clone())?;
        i = i + 1;
    }
    Ok(())
}

pub(crate) fn dumpTearing(mut inResEqn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inTearVar: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inResEqn, inTearVar)) {
        (Deref @ metamodelica::List::Cons { head: residualeqns, tail: r }, Deref @ metamodelica::List::Cons { head: tearingvars, tail: t }) => {
            let mut str_r: Arc<metamodelica::List<ArcStr>>;
            let mut str_t: Arc<metamodelica::List<ArcStr>>;
            let mut str_r_f: ArcStr;
            let mut str_r_1: ArcStr;
            let mut str_t_f: ArcStr;
            let mut str_t_1: ArcStr;
            let mut r#str: ArcStr;
            let mut sr: ArcStr;
            let mut st: ArcStr;
            str_r = List::map(residualeqns.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            str_r_f = stringDelimitList(str_r.clone(), (literal!(", ")).clone());
            str_r_1 = (stringAppend((str_r_f.clone()).clone(), (literal!("\n")).clone())).clone();
            sr = (stringAppend((literal!("ResidualEqns: ")).clone(), (str_r_1.clone()).clone())).clone();
            str_t = List::map(tearingvars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            str_t_f = stringDelimitList(str_t.clone(), (literal!(", ")).clone());
            str_t_1 = (stringAppend((str_t_f.clone()).clone(), (literal!("\n")).clone())).clone();
            st = (stringAppend((literal!("TearingVars: ")).clone(), (str_t_1.clone()).clone())).clone();
            r#str = (stringAppend((sr.clone()).clone(), (st.clone()).clone())).clone();
            metamodelica::print((r#str.clone()).clone());
            metamodelica::print((literal!("\n")).clone());
            dumpTearing(r.clone(), t.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn dumpBackendDAEEqnList(mut inBackendDAEEqnList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut header: ArcStr, mut printExpTree: bool) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*header); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    dumpBackendDAEEqnList2(inBackendDAEEqnList, printExpTree)?;
    metamodelica::print((literal!("===================\n")).clone());
    Ok(())
}

fn dumpBackendDAEEqnList2(mut inBackendDAEEqnList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut printExpTree: bool) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inBackendDAEEqnList;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, .. }, tail: res } => {
                    let mut r#str: ArcStr;
                    r#str = (literal!("EQUATION: ")).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" = ")); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                    metamodelica::print((r#str.clone()).clone());
                    r#str = (literal!("LHS:\n")).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionDump::dumpExpStr(e1.clone(), 0)?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("RHS:\n")); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionDump::dumpExpStr(e2.clone(), 0)?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    r#str = (if (printExpTree) {r#str.clone()} else {literal!("")}).clone();
                    metamodelica::print((r#str.clone()).clone());
                    dumpBackendDAEEqnList2(res.clone(), printExpTree)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, .. }, tail: res } => {
                    let mut r#str: ArcStr;
                    r#str = (literal!("COMPLEX_EQUATION: ")).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" = ")); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                    metamodelica::print((r#str.clone()).clone());
                    r#str = (literal!("LHS:\n")).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionDump::dumpExpStr(e1.clone(), 0)?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("RHS:\n")); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionDump::dumpExpStr(e2.clone(), 0)?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    r#str = (if (printExpTree) {r#str.clone()} else {literal!("")}).clone();
                    metamodelica::print((r#str.clone()).clone());
                    dumpBackendDAEEqnList2(res.clone(), printExpTree)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e, attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, .. }, tail: res } => {
                    let mut r#str: ArcStr;
                    metamodelica::print((literal!("SOLVED_EQUATION: ")).clone());
                    r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
                    metamodelica::print((r#str.clone()).clone());
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
                    r#str = (ExpressionDump::dumpExpStr(e.clone(), 0)?).clone();
                    r#str = (if (printExpTree) {r#str.clone()} else {literal!("")}).clone();
                    metamodelica::print((r#str.clone()).clone());
                    metamodelica::print((literal!("\n")).clone());
                    dumpBackendDAEEqnList2(res.clone(), printExpTree)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, .. }, tail: res } => {
                    let mut r#str: ArcStr;
                    r#str = (literal!("RESIDUAL_EQUATION: ")).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                    metamodelica::print((r#str.clone()).clone());
                    r#str = (ExpressionDump::dumpExpStr(e.clone(), 0)?).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    r#str = (if (printExpTree) {r#str.clone()} else {literal!("")}).clone();
                    metamodelica::print((r#str.clone()).clone());
                    dumpBackendDAEEqnList2(res.clone(), printExpTree)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, .. }, tail: res } => {
                    let mut r#str: ArcStr;
                    metamodelica::print((literal!("ARRAY_EQUATION: ")).clone());
                    r#str = (ExpressionBasics::printExpStr(e1.clone())?).clone();
                    metamodelica::print((r#str.clone()).clone());
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                    r#str = (ExpressionDump::dumpExpStr(e1.clone(), 0)?).clone();
                    r#str = (if (printExpTree) {r#str.clone()} else {literal!("")}).clone();
                    metamodelica::print((r#str.clone()).clone());
                    metamodelica::print((literal!("\n")).clone());
                    dumpBackendDAEEqnList2(res.clone(), printExpTree)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::ALGORITHM { alg, attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, .. }, tail: res } => {
                    metamodelica::print((literal!("ALGORITHM: ")).clone());
                    dumpAlgorithms(list![alg.clone()], 0)?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
                    dumpBackendDAEEqnList2(res.clone(), printExpTree)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: weqn, attr: BackendDAE::EquationAttributes { kind: eqKind, .. }, .. }, tail: _ } => {
                    let mut e: Arc<DAE::Exp>;
                    let mut r#str: ArcStr;
                    metamodelica::print((literal!("WHEN_EQUATION: ")).clone());
                    r#str = (whenEquationString(weqn.clone(), true)?).clone();
                    metamodelica::print((r#str.clone()).clone());
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*equationKindString(eqKind.clone())?); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone();
                    e = weqn.condition.clone();
                    r#str = (ExpressionDump::dumpExpStr(e.clone(), 0)?).clone();
                    r#str = (if (printExpTree) {r#str.clone()} else {literal!("")}).clone();
                    metamodelica::print((r#str.clone()).clone());
                    metamodelica::print((literal!("\n")).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: res } => {
                    metamodelica::print((literal!("SKIPED EQUATION\n")).clone());
                    dumpBackendDAEEqnList2(res.clone(), printExpTree)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub(crate) fn dumpBackendDAEVarList(mut inBackendDAEVarList: Arc<metamodelica::List<BackendDAE::Var>>, mut header: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*header); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    printVarList(inBackendDAEVarList)?;
    metamodelica::print((literal!("===================\n")).clone());
    Ok(())
}

pub(crate) fn dumpEqnsSolved(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut heading: ArcStr) -> Result<()> {
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    let __pa0 = ::match_deref::match_deref! { match &(inBackendDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqs = __pa0.clone();
    List::map_0(eqs, (std::sync::Arc::new(dumpEqnsSolved1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<()> + 'static>))?;
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

fn dumpEqnsSolved1(mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inEqSystem) {
        Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, matching: Deref @ BackendDAE::Matching::MATCHING { comps, .. }, .. } => {
            dumpEqnsSolved2(comps.clone(), eqns.clone(), vars.clone());
            ()
        },
        _ => {
            metamodelica::print((literal!("No Matching\n")).clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpEqnsSolved2(mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut vars: BackendDAE::Variables) -> () {
    let () = 'mc: {
        let __mc_input = inComps;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: e, var: v }, tail: rest } => {
                    let mut var: BackendDAE::Var;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SingleEquation: ")); __mm_s.push_str(&*intString(e.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    var = BackendVariable::getVarAt(vars.clone(), v.clone())?;
                    printVarList(list![var.clone()])?;
                    eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                    printEquationList(list![eqn.clone()])?;
                    metamodelica::print((literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: elst, vars: vlst, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: jac }, jacType, .. }, tail: rest } => {
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Equationsystem ")); __mm_s.push_str(&*jacobianTypeStr(jacType.clone())?); __mm_s.push_str(&*literal!(":\n")); ArcStr::from(__mm_s) }).clone());
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    printVarList(varlst.clone())?;
                    eqnlst = BackendEquation::getList(elst.clone(), eqns.clone())?;
                    printEquationList(eqnlst.clone())?;
                    metamodelica::print((literal!("\n")).clone());
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Jac:\n")); __mm_s.push_str(&*dumpJacobianStr(jac.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    metamodelica::print((literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: e, vars: vlst }, tail: rest } => {
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    metamodelica::print((literal!("ArrayEquation:\n")).clone());
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    printVarList(varlst.clone())?;
                    eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                    printEquationList(list![eqn.clone()])?;
                    metamodelica::print((literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: e, vars: vlst }, tail: rest } => {
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    metamodelica::print((literal!("IfEquation:\n")).clone());
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    printVarList(varlst.clone())?;
                    eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                    printEquationList(list![eqn.clone()])?;
                    metamodelica::print((literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: e, vars: vlst }, tail: rest } => {
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    metamodelica::print((literal!("Algorithm:\n")).clone());
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    printVarList(varlst.clone())?;
                    eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                    printEquationList(list![eqn.clone()])?;
                    metamodelica::print((literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: e, vars: vlst }, tail: rest } => {
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    metamodelica::print((literal!("ComplexEquation:\n")).clone());
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    printVarList(varlst.clone())?;
                    eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                    printEquationList(list![eqn.clone()])?;
                    metamodelica::print((literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: e, vars: vlst }, tail: rest } => {
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    metamodelica::print((literal!("WhenEquation:\n")).clone());
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    printVarList(varlst.clone())?;
                    eqn = BackendEquation::get(eqns.clone(), e.clone())?;
                    printEquationList(list![eqn.clone()])?;
                    metamodelica::print((literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: vlst, residualequations: elst, innerEquations, .. }, casualTearingSet: None, linear: b, .. }, tail: rest } => {
                    let mut vlst1: Arc<metamodelica::List<i32>>;
                    let mut elst1: Arc<metamodelica::List<i32>>;
                    let mut vlst1Lst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut s: ArcStr;
                    s = (if (b.clone()) {literal!("linear")} else {literal!("nonlinear")}).clone();
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("torn ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" Equationsystem:\n")); ArcStr::from(__mm_s) }).clone());
                    (elst1, vlst1Lst, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>))?;
                    vlst1 = List::flatten(vlst1Lst.clone())?;
                    varlst = List::map1r(vlst1.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\ninternal vars (")); __mm_s.push_str(&*intString((varlst.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
                    printVarList(varlst.clone())?;
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nresidual vars (")); __mm_s.push_str(&*intString((varlst.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
                    printVarList(varlst.clone())?;
                    eqnlst = BackendEquation::getList(elst1.clone(), eqns.clone())?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\ninternal equations (")); __mm_s.push_str(&*intString((eqnlst.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
                    printEquationList(eqnlst.clone())?;
                    eqnlst = BackendEquation::getList(elst.clone(), eqns.clone())?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nresidual equations (")); __mm_s.push_str(&*intString((eqnlst.clone().len() as i32))); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
                    printEquationList(eqnlst.clone())?;
                    metamodelica::print((literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: vlst, residualequations: elst, innerEquations, .. }, casualTearingSet: Some(BackendDAE::TearingSet { tearingvars: vlst2, residualequations: elst2, innerEquations: innerEquations2, .. }), linear: b, .. }, tail: rest } => {
                    let mut vlst1: Arc<metamodelica::List<i32>>;
                    let mut elst1: Arc<metamodelica::List<i32>>;
                    let mut vlst1Lst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut varlst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut s: ArcStr;
                    s = (if (b.clone()) {literal!("linear")} else {literal!("nonlinear")}).clone();
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Strict torn ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" Equationsystem:\n")); ArcStr::from(__mm_s) }).clone());
                    (elst1, vlst1Lst, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>))?;
                    vlst1 = List::flatten(vlst1Lst.clone())?;
                    varlst = List::map1r(vlst1.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    printVarList(varlst.clone())?;
                    varlst = List::map1r(vlst.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    printVarList(varlst.clone())?;
                    metamodelica::print((literal!("\n")).clone());
                    eqnlst = BackendEquation::getList(elst1.clone(), eqns.clone())?;
                    printEquationList(eqnlst.clone())?;
                    metamodelica::print((literal!("\n")).clone());
                    eqnlst = BackendEquation::getList(elst.clone(), eqns.clone())?;
                    printEquationList(eqnlst.clone())?;
                    metamodelica::print((literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone());
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Casual torn ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" Equationsystem:\n")); ArcStr::from(__mm_s) }).clone());
                    (elst1, vlst1Lst, _) = List::map_3(innerEquations2.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>))?;
                    vlst1 = List::flatten(vlst1Lst.clone())?;
                    varlst = List::map1r(vlst1.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    printVarList(varlst.clone())?;
                    varlst = List::map1r(vlst2.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    printVarList(varlst.clone())?;
                    metamodelica::print((literal!("\n")).clone());
                    eqnlst = BackendEquation::getList(elst1.clone(), eqns.clone())?;
                    printEquationList(eqnlst.clone())?;
                    metamodelica::print((literal!("\n")).clone());
                    eqnlst = BackendEquation::getList(elst2.clone(), eqns.clone())?;
                    printEquationList(eqnlst.clone())?;
                    metamodelica::print((literal!("\n")).clone());
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::traceln((literal!("BackendDump.dumpEqnsSolved2 failed!")).clone())?;
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    dumpEqnsSolved2(rest.clone(), eqns.clone(), vars.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

pub(crate) fn dumpLoops(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut isyst: i32 = 1;
    let mut firstComp: bool = true;
    for mut syst in &*inDAE.eqs.clone() {
        let mut syst = syst.clone();
        firstComp = true;
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(syst.clone()) {
            Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa2, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        vars = __pa0.clone();
        eqns = __pa1.clone();
        comps = __pa2.clone();
        for mut comp in &*comps.clone() {
            let mut comp = comp.clone();
            if BackendEquation::isEquationsSystem(comp.clone()) || BackendEquation::isTornSystem(comp.clone()) {
                if firstComp {
                    firstComp = false;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nsystem ")); __mm_s.push_str(&*intString(isyst)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n dumpLoops: SORTED COMPONENT \n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                dumpEqnsSolved2(list![comp.clone()], eqns.clone(), vars.clone());
                if Flags::isSet(Flags::DUMP_LOOPS_VERBOSE.clone())? {
                    printComponentAdjacencyMatrixEnhanced(comp.clone(), eqns.clone(), vars.clone(), outDAE.shared.clone())?;
                }
            }
        }
        isyst = isyst + 1;
    }
    Ok(outDAE)
}

pub(crate) fn printComponentAdjacencyMatrixEnhanced(mut comp: Arc<BackendDAE::StrongComponent>, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut vars: BackendDAE::Variables, mut shared: Arc<BackendDAE::Shared>) -> Result<()> {
    let mut compEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut compVarLst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut compEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut compVars: BackendDAE::Variables;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    (compVarLst, _, compEqnLst, _) = BackendDAEUtil::getStrongComponentVarsAndEquations(comp, vars, eqns)?;
    compEqns = BackendEquation::listEquation(compEqnLst)?;
    compVars = BackendVariable::listVar(compVarLst)?;
    syst = BackendDAEUtil::createEqSystem(compVars.clone(), compEqns.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    (m, mT, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(syst, shared, false)?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n dumpLoopsVerbose: UNSORTED COMPONENT WITH ENHANCED ADJACENCY MATRIX \n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    dumpVariables(compVars, (literal!("component variables")).clone())?;
    dumpEquationArray(compEqns, (literal!("component equations")).clone())?;
    dumpAdjacencyMatrixEnhanced(m.clone())?;
    metamodelica::print((literal!("\n\n")).clone());
    dumpAdjacencyMatrixTEnhanced(mT.clone())?;
    Ok(())
}

pub(crate) fn dumpComponentsAdvanced(mut l: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut v2: metamodelica::Array<i32>, mut syst: Arc<BackendDAE::EqSystem>) -> Result<()> {
    let mut vars: BackendDAE::Variables;
    metamodelica::print((literal!("Blocks\n")).clone());
    metamodelica::print((literal!("=======\n")).clone());
    vars = BackendVariable::daeVars(syst);
    dumpComponentsAdvanced2(l, 1, v2.clone(), vars)?;
    Ok(())
}

fn dumpComponentsAdvanced2(mut inIntegerLstLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inInteger: i32, mut v2: metamodelica::Array<i32>, mut vars: BackendDAE::Variables) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inIntegerLstLst, inInteger)) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: l, tail: lst }, i) => {
            let mut i_1: i32;
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            metamodelica::print((literal!("{")).clone());
            ls = List::map(l.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            metamodelica::print((s.clone()).clone());
            metamodelica::print((literal!("} ")).clone());
            dumpComponentsAdvanced3(l.clone(), v2.clone(), vars.clone())?;
            metamodelica::print((literal!("\n")).clone());
            i_1 = i.clone() + 1;
            dumpComponentsAdvanced2(lst.clone(), i_1.clone(), v2.clone(), vars)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpComponentsAdvanced3(mut inIntegerLst: Arc<metamodelica::List<i32>>, mut v2: metamodelica::Array<i32>, mut vars: BackendDAE::Variables) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inIntegerLst) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: i, tail: Deref @ metamodelica::List::Nil } => {
            let mut v: i32;
            let mut s: ArcStr;
            let mut c: Arc<DAE::ComponentRef>;
            let mut var: BackendDAE::Var;
            let mut b: bool;
            v = ({let __elt = v2.borrow()[(i.clone()-1) as usize].clone(); __elt});
            var = BackendVariable::getVarAt(vars, v.clone())?;
            c = BackendVariable::varCref(var.clone())?;
            b = BackendVariable::isStateVar(var.clone());
            s = (if (b.clone()) {literal!("der(")} else {literal!("")}).clone();
            metamodelica::print((s.clone()).clone());
            s = (ComponentReferenceBasics::printComponentRefStr(c.clone())?).clone();
            metamodelica::print((s.clone()).clone());
            s = (if (b.clone()) {literal!(") ")} else {literal!(" ")}).clone();
            metamodelica::print((s.clone()).clone());
            ()
        },
        Deref @ metamodelica::List::Cons { head: i, tail: l } => {
            let mut v: i32;
            let mut s: ArcStr;
            let mut c: Arc<DAE::ComponentRef>;
            let mut var: BackendDAE::Var;
            let mut b: bool;
            v = ({let __elt = v2.borrow()[(i.clone()-1) as usize].clone(); __elt});
            var = BackendVariable::getVarAt(vars.clone(), v.clone())?;
            c = BackendVariable::varCref(var.clone())?;
            b = BackendVariable::isStateVar(var.clone());
            s = (if (b.clone()) {literal!("der(")} else {literal!("")}).clone();
            metamodelica::print((s.clone()).clone());
            s = (ComponentReferenceBasics::printComponentRefStr(c.clone())?).clone();
            metamodelica::print((s.clone()).clone());
            s = (if (b.clone()) {literal!(") ")} else {literal!(" ")}).clone();
            metamodelica::print((s.clone()).clone());
            dumpComponentsAdvanced3(l.clone(), v2.clone(), vars)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn dumpComponents(mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut inSyst: Option<Arc<BackendDAE::EqSystem>>) -> Result<()> {
    metamodelica::print((literal!("StrongComponents\n")).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    List::map1(inComps, (std::sync::Arc::new(dumpComponent) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, Option<Arc<BackendDAE::EqSystem>>) -> Result<()> + 'static>), inSyst)?;
    Ok(())
}

pub(crate) fn dumpComponent(mut inComp: Arc<BackendDAE::StrongComponent>, mut inSyst: Option<Arc<BackendDAE::EqSystem>>) -> Result<()> {
    metamodelica::print((printComponent(inComp, inSyst)?).clone());
    Ok(())
}

pub(crate) fn printComponent(mut inComp: Arc<BackendDAE::StrongComponent>, mut inSyst: Option<Arc<BackendDAE::EqSystem>>) -> Result<ArcStr> {
    let mut oString: ArcStr;
    let mut tmpStr: ArcStr = arcstr::literal!("");
    let mut tmpStr2: ArcStr = arcstr::literal!("");
    oString = ((::match_deref::match_deref! { match &(inComp) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: i, var: v } => {
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*intString(v.clone())); __mm_s.push_str(&*literal!("}\n")); ArcStr::from(__mm_s) }).clone();
            tmpStr
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: ilst, vars: vlst, jacType, .. } => {
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            ls = List::map(ilst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s2 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("} Size: ")); __mm_s.push_str(&*intString((vlst.clone().len() as i32))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*jacobianTypeStr(jacType.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            tmpStr
        },
        Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: i, vars: vlst } => {
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Array ")); __mm_s.push_str(&*literal!(" {{")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}}\n")); ArcStr::from(__mm_s) }).clone();
            tmpStr
        },
        Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: i, vars: vlst } => {
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("IfEquation ")); __mm_s.push_str(&*literal!(" {{")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}}\n")); ArcStr::from(__mm_s) }).clone();
            tmpStr
        },
        Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: i, vars: vlst } => {
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Algorithm ")); __mm_s.push_str(&*literal!(" {{")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}}\n")); ArcStr::from(__mm_s) }).clone();
            tmpStr
        },
        Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: i, vars: vlst } => {
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ComplexEquation ")); __mm_s.push_str(&*literal!(" {")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}\n")); ArcStr::from(__mm_s) }).clone();
            tmpStr
        },
        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: i, vars: vlst } => {
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("WhenEquation ")); __mm_s.push_str(&*literal!(" {")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}\n")); ArcStr::from(__mm_s) }).clone();
            tmpStr
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: ilst, tearingvars: vlst, innerEquations, .. }, casualTearingSet: None, linear: b, .. } => {
            let mut innerEqLst: Arc<metamodelica::List<i32>>;
            let mut innerVarLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut s4: ArcStr;
            let mut eSys: Arc<BackendDAE::EqSystem>;
            ls = List::map(innerEquations.clone(), (std::sync::Arc::new(innerEquationString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(ilst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s2 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s3 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            s4 = (if (b.clone()) {literal!("linear")} else {literal!("nonlinear")}).clone();
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{{")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}\n,{")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("}} Size: ")); __mm_s.push_str(&*intString((vlst.clone().len() as i32))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*s4.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            if isSome(inSyst.clone()) {
                if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                    let __pa0 = ::match_deref::match_deref! { match &(inSyst) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    eSys = __pa0.clone();
                    (innerEqLst, innerVarLst, _) = BackendDAEUtil::getEqnAndVarsFromInnerEquationLst(innerEquations.clone())?;
                    tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpStr); __mm_s.push_str(&*literal!("\nTearing Variables:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedVars(eSys.clone(), vlst.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("Residual Equations:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedEqns(eSys.clone(), ilst.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("Inner Variables:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedVarsLsts(eSys.clone(), innerVarLst.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("InnerEquations:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedEqns(eSys.clone(), innerEqLst.clone())?); ArcStr::from(__mm_s) }).clone();
                } else {
                    tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpStr); __mm_s.push_str(&*literal!("For more information please use \"-d=tearingdump\".\n")); ArcStr::from(__mm_s) }).clone();
                }
            }
            tmpStr
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: ilst, tearingvars: vlst, innerEquations, .. }, casualTearingSet: Some(BackendDAE::TearingSet { residualequations: ilst2, tearingvars: vlst2, innerEquations: innerEquations2, .. }), linear: b, .. } => {
            let mut innerEqLst: Arc<metamodelica::List<i32>>;
            let mut innerVarLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut s4: ArcStr;
            let mut eSys: Arc<BackendDAE::EqSystem>;
            ls = List::map(innerEquations.clone(), (std::sync::Arc::new(innerEquationString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(ilst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s2 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s3 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            s4 = (if (b.clone()) {literal!("linear")} else {literal!("nonlinear")}).clone();
            tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{{")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}\n,{")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("}} Size: ")); __mm_s.push_str(&*intString((vlst.clone().len() as i32))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*s4.clone()); __mm_s.push_str(&*literal!(" (strict tearing set)\n")); ArcStr::from(__mm_s) }).clone();
            if isSome(inSyst.clone()) {
                if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                    let __pa0 = ::match_deref::match_deref! { match &(inSyst.clone()) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    eSys = __pa0.clone();
                    (innerEqLst, innerVarLst, _) = BackendDAEUtil::getEqnAndVarsFromInnerEquationLst(innerEquations.clone())?;
                    tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpStr); __mm_s.push_str(&*literal!("\nTearing Variables:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedVars(eSys.clone(), vlst.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("Residual Equations:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedEqns(eSys.clone(), ilst.clone())?); __mm_s.push_str(&*literal!("Inner Variables:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedVarsLsts(eSys.clone(), innerVarLst.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("InnerEquations:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedEqns(eSys.clone(), innerEqLst.clone())?); ArcStr::from(__mm_s) }).clone();
                } else {
                    tmpStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpStr); __mm_s.push_str(&*literal!("For more information please use \"-d=tearingdump\".\n")); ArcStr::from(__mm_s) }).clone();
                }
            }
            ls = List::map(innerEquations2.clone(), (std::sync::Arc::new(innerEquationString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(ilst2.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s2 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(vlst2.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s3 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            s4 = (if (b.clone()) {literal!("linear")} else {literal!("nonlinear")}).clone();
            tmpStr2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{{")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("}\n,{")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("}} Size: ")); __mm_s.push_str(&*intString((vlst2.clone().len() as i32))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*s4.clone()); __mm_s.push_str(&*literal!(" (casual tearing set)\n")); ArcStr::from(__mm_s) }).clone();
            if isSome(inSyst.clone()) {
                if Flags::isSet(Flags::TEARING_DUMP.clone())? || Flags::isSet(Flags::TEARING_DUMPVERBOSE.clone())? {
                    let __pa1 = ::match_deref::match_deref! { match &(inSyst) {
                        Some(__pa1) => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    eSys = __pa1.clone();
                    (innerEqLst, innerVarLst, _) = BackendDAEUtil::getEqnAndVarsFromInnerEquationLst(innerEquations2.clone())?;
                    tmpStr2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpStr2); __mm_s.push_str(&*literal!("\nTearing Variables:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedVars(eSys.clone(), vlst2.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("Residual Equations:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedEqns(eSys.clone(), ilst2.clone())?); __mm_s.push_str(&*literal!("Inner Variables:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedVarsLsts(eSys.clone(), innerVarLst.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("InnerEquations:\n-------------------------------------\n")); __mm_s.push_str(&*dumpMarkedEqns(eSys.clone(), innerEqLst.clone())?); ArcStr::from(__mm_s) }).clone();
                } else {
                    tmpStr2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpStr2); __mm_s.push_str(&*literal!("For more information please use \"-d=tearingdump\".\n")); ArcStr::from(__mm_s) }).clone();
                }
            }
            { let mut __mm_s = String::new(); __mm_s.push_str(&*tmpStr); __mm_s.push_str(&*tmpStr2); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(oString)
}

pub(crate) fn dumpListList(mut lstLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut heading: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*heading); __mm_s.push_str(&*literal!(":\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*stringDelimitList(List::map(lstLst, (std::sync::Arc::new(intListStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))?, (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

// =============================================================================
// section for all *String functions
//
// These are functions, that return their output with a String.
//   - equationString
//   - strongComponentString
// =============================================================================
pub(crate) fn strongComponentString(mut inComp: Arc<BackendDAE::StrongComponent>) -> Result<ArcStr> {
    let mut outS: ArcStr;
    outS = ((::match_deref::match_deref! { match &(inComp) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: i, var: v } => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            s = (intString(i.clone())).clone();
            s1 = (intString(v.clone())).clone();
            s = stringAppendList(list![(literal!("{")).clone(), (s.clone()).clone(), (literal!(":")).clone(), (s1.clone()).clone(), (literal!("}")).clone()]);
            s.clone()
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: ilst, vars: vlst, jacType, .. } => {
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut ls1: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut sl: ArcStr;
            let mut sj: ArcStr;
            ls = List::map(ilst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls1 = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s1 = stringDelimitList(ls1.clone(), (literal!(", ")).clone());
            sl = (intString((ilst.clone().len() as i32))).clone();
            sj = (jacobianTypeStr(jacType.clone())?).clone();
            s2 = stringAppendList(list![(literal!("{")).clone(), (s.clone()).clone(), (literal!(":")).clone(), (s1.clone()).clone(), (literal!("} Size: ")).clone(), (sl.clone()).clone(), (literal!(" ")).clone(), (sj.clone()).clone()]);
            s2.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: i, vars: vlst } => {
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            let mut sl: ArcStr;
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            sl = (intString(i.clone())).clone();
            s2 = stringAppendList(list![(literal!("Array ")).clone(), (sl.clone()).clone(), (literal!(" {")).clone(), (s.clone()).clone(), (literal!("}")).clone()]);
            s2.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: i, vars: vlst } => {
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            let mut sl: ArcStr;
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            sl = (intString(i.clone())).clone();
            s2 = stringAppendList(list![(literal!("Array ")).clone(), (sl.clone()).clone(), (literal!(" {")).clone(), (s.clone()).clone(), (literal!("}")).clone()]);
            s2.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: i, vars: vlst } => {
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            let mut sl: ArcStr;
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            sl = (intString(i.clone())).clone();
            s2 = stringAppendList(list![(literal!("Algorithm ")).clone(), (sl.clone()).clone(), (literal!(" {")).clone(), (s.clone()).clone(), (literal!("}")).clone()]);
            s2.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: i, vars: vlst } => {
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            let mut sl: ArcStr;
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            sl = (intString(i.clone())).clone();
            s2 = stringAppendList(list![(literal!("ComplexEquation ")).clone(), (sl.clone()).clone(), (literal!(" {")).clone(), (s.clone()).clone(), (literal!("}")).clone()]);
            s2.clone()
        },
        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: i, vars: vlst } => {
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            let mut sl: ArcStr;
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            sl = (intString(i.clone())).clone();
            s2 = stringAppendList(list![(literal!("WhenEquation ")).clone(), (sl.clone()).clone(), (literal!(" {")).clone(), (s.clone()).clone(), (literal!("}")).clone()]);
            s2.clone()
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: ilst, tearingvars: vlst, innerEquations, .. }, linear: b, .. } => {
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut sl: ArcStr;
            let mut sj: ArcStr;
            ls = List::map(innerEquations.clone(), (std::sync::Arc::new(innerEquationString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(ilst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s1 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            ls = List::map(vlst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s2 = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            sj = (intString((vlst.clone().len() as i32))).clone();
            sl = (if (b.clone()) {literal!("linear")} else {literal!("nonlinear")}).clone();
            s2 = stringAppendList(list![(literal!("torn ")).clone(), (sl.clone()).clone(), (literal!(" Equationsystem")).clone(), (literal!("{{")).clone(), (s.clone()).clone(), (literal!("},\n{")).clone(), (s1.clone()).clone(), (literal!(":")).clone(), (s2.clone()).clone(), (literal!("} Size: ")).clone(), (sj.clone()).clone()]);
            s2.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outS)
}

pub fn whenEquationString(mut inWhenEqn: Arc<BackendDAE::WhenEquation>, mut inStart: bool) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut conditionStr: ArcStr;
    let mut whenStmtStr: ArcStr;
    let mut elseWhenStr: ArcStr;
    let mut cond: Arc<DAE::Exp>;
    let mut weqn: Arc<BackendDAE::WhenEquation>;
    let mut oweqn: Option<Arc<BackendDAE::WhenEquation>>;
    let mut whenStmtLst: Arc<metamodelica::List<BackendDAE::WhenOperator>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inWhenEqn) {
        Deref @ BackendDAE::WhenEquation { condition: __pa0, whenStmtLst: __pa1, elsewhenPart: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    cond = __pa0.clone();
    whenStmtLst = __pa1.clone();
    oweqn = __pa2.clone();
    conditionStr = (ExpressionBasics::printExpStr(cond)?).clone();
    whenStmtStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(whenStmtLst, (std::sync::Arc::new(dumpWhenOperatorStr) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::WhenOperator) -> Result<ArcStr> + 'static>))?, (literal!(";\n  ")).clone())); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone();
    if isSome(oweqn.clone()) {
        let __pa3 = ::match_deref::match_deref! { match &(oweqn) {
            Some(__pa3) => __pa3.clone(),
            _ => bail!("pattern mismatch"),
        } };
        weqn = __pa3.clone();
        elseWhenStr = (whenEquationString(weqn, false)?).clone();
    } else {
        elseWhenStr = (literal!("")).clone();
    }
    if inStart {
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("when ")); __mm_s.push_str(&*conditionStr); __mm_s.push_str(&*literal!(" then\n  ")); __mm_s.push_str(&*whenStmtStr); __mm_s.push_str(&*elseWhenStr); __mm_s.push_str(&*literal!("end when;")); ArcStr::from(__mm_s) }).clone();
    } else {
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("elsewhen ")); __mm_s.push_str(&*conditionStr); __mm_s.push_str(&*literal!(" then\n  ")); __mm_s.push_str(&*whenStmtStr); __mm_s.push_str(&*elseWhenStr); ArcStr::from(__mm_s) }).clone();
    }
    Ok(outString)
}

pub(crate) fn equationString(mut inEquation: Arc<BackendDAE::Equation>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inEquation) {
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" = ")).clone(), (s2.clone()).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr, exp: e2, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            s2 = (ExpressionBasics::printExpStr(e2.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!(" := ")).clone(), (s2.clone()).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: weqn, .. } => {
            let mut res: ArcStr;
            res = (whenEquationString(weqn.clone(), true)?).clone();
            res.clone()
        },
        Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. } => {
            let mut s1: ArcStr;
            let mut res: ArcStr;
            s1 = (ExpressionBasics::printExpStr(e.clone())?).clone();
            res = stringAppendList(list![(s1.clone()).clone(), (literal!("= 0")).clone()]);
            res.clone()
        },
        Deref @ BackendDAE::Equation::ALGORITHM { alg, source, .. } => {
            let mut res: ArcStr;
            res = (DAEDump::dumpAlgorithmsStr(list![Arc::new(DAE::Element::ALGORITHM { algorithm_: alg.clone(), source: source.clone() })])?).clone();
            res.clone()
        },
        Deref @ BackendDAE::Equation::IF_EQUATION { conditions: Deref @ metamodelica::List::Cons { head: e1, tail: expl }, eqnstrue: Deref @ metamodelica::List::Cons { head: eqns, tail: eqnstrue }, eqnsfalse, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut res: ArcStr;
            s1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            s2 = stringDelimitList(List::map(eqns.clone(), (std::sync::Arc::new(equationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>))?, (literal!("\n  ")).clone());
            s3 = stringAppendList(list![(literal!("if ")).clone(), (s1.clone()).clone(), (literal!(" then\n  ")).clone(), (s2.clone()).clone()]);
            res = (ifequationString(expl.clone(), eqnstrue.clone(), eqnsfalse.clone(), (s3.clone()).clone())?).clone();
            res.clone()
        },
        Deref @ BackendDAE::Equation::FOR_EQUATION { iter, start, stop, body: eqn, .. } => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(iter.clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(start.clone())?); __mm_s.push_str(&*literal!(" : ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(stop.clone())?); ArcStr::from(__mm_s) }).clone();
            s2 = (equationString(eqn.clone())?).clone();
            res = stringAppendList(list![(literal!("for ")).clone(), (s1.clone()).clone(), (literal!(" loop\n    ")).clone(), (s2.clone()).clone(), (literal!("; end for; ")).clone()]);
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn zeroCrossingString(mut inZeroCrossing: BackendDAE::ZeroCrossing) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inZeroCrossing) {
        BackendDAE::ZeroCrossing { relation_: e @ Deref @ DAE::Exp::RELATION { index: index_, .. }, occurEquLst: eq, .. } => {
            let mut eq_s_list: Arc<metamodelica::List<ArcStr>>;
            let mut eq_s: ArcStr;
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            let mut str_index: ArcStr;
            eq_s_list = List::map(eq.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            eq_s = stringDelimitList(eq_s_list.clone(), (literal!(",")).clone());
            r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
            str_index = (intString(index_.clone())).clone();
            str2 = stringAppendList(list![(r#str.clone()).clone(), (literal!(" with index = ")).clone(), (str_index.clone()).clone(), (literal!(" in equations [")).clone(), (eq_s.clone()).clone(), (literal!("]")).clone()]);
            str2.clone()
        },
        BackendDAE::ZeroCrossing { relation_: e @ Deref @ DAE::Exp::LBINARY { .. }, occurEquLst: eq, .. } => {
            let mut eq_s_list: Arc<metamodelica::List<ArcStr>>;
            let mut eq_s: ArcStr;
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            eq_s_list = List::map(eq.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            eq_s = stringDelimitList(eq_s_list.clone(), (literal!(",")).clone());
            r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
            str2 = stringAppendList(list![(r#str.clone()).clone(), (literal!(" in equations [")).clone(), (eq_s.clone()).clone(), (literal!("]")).clone()]);
            str2.clone()
        },
        BackendDAE::ZeroCrossing { relation_: e @ Deref @ DAE::Exp::LUNARY { .. }, occurEquLst: eq, .. } => {
            let mut eq_s_list: Arc<metamodelica::List<ArcStr>>;
            let mut eq_s: ArcStr;
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            eq_s_list = List::map(eq.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            eq_s = stringDelimitList(eq_s_list.clone(), (literal!(",")).clone());
            r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
            str2 = stringAppendList(list![(r#str.clone()).clone(), (literal!(" in equations [")).clone(), (eq_s.clone()).clone(), (literal!("]")).clone()]);
            str2.clone()
        },
        BackendDAE::ZeroCrossing { relation_: e @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { .. }, .. }, occurEquLst: eq, .. } => {
            let mut eq_s_list: Arc<metamodelica::List<ArcStr>>;
            let mut eq_s: ArcStr;
            let mut r#str: ArcStr;
            let mut str2: ArcStr;
            eq_s_list = List::map(eq.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            eq_s = stringDelimitList(eq_s_list.clone(), (literal!(",")).clone());
            r#str = (ExpressionBasics::printExpStr(e.clone())?).clone();
            str2 = stringAppendList(list![(r#str.clone()).clone(), (literal!(" in equations [")).clone(), (eq_s.clone()).clone(), (literal!("]")).clone()]);
            str2.clone()
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn timeEventString(mut inTimeEvent: BackendDAE::TimeEvent) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inTimeEvent.clone() {
        BackendDAE::TimeEvent::SIMPLE_TIME_EVENT { .. } => literal!("SIMPLE_TIME_EVENT"),
        BackendDAE::TimeEvent::SAMPLE_TIME_EVENT { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(var_field!(inTimeEvent.index, BackendDAE::TimeEvent::SAMPLE_TIME_EVENT).clone())); __mm_s.push_str(&*literal!(": sample(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(var_field!(inTimeEvent.startExp, BackendDAE::TimeEvent::SAMPLE_TIME_EVENT).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(var_field!(inTimeEvent.intervalExp, BackendDAE::TimeEvent::SAMPLE_TIME_EVENT).clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        _ => literal!("unknown time event"),
    })).clone();
    Ok(outString)
}

pub(crate) fn simIteratorString(mut iter: BackendDAE::SimIterator) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match iter.clone() {
        BackendDAE::SimIterator::SIM_ITERATOR_RANGE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var_field!(iter.name, BackendDAE::SimIterator::SIM_ITERATOR_RANGE).clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(var_field!(iter.start, BackendDAE::SimIterator::SIM_ITERATOR_RANGE).clone())?); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*ExpressionBasics::printExpStr(var_field!(iter.step, BackendDAE::SimIterator::SIM_ITERATOR_RANGE).clone())?); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*ExpressionBasics::printExpStr(var_field!(iter.stop, BackendDAE::SimIterator::SIM_ITERATOR_RANGE).clone())?); ArcStr::from(__mm_s) },
        BackendDAE::SimIterator::SIM_ITERATOR_LIST { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var_field!(iter.name, BackendDAE::SimIterator::SIM_ITERATOR_LIST).clone())?); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*List::toString(var_field!(iter.lst, BackendDAE::SimIterator::SIM_ITERATOR_LIST).clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 10)?); ArcStr::from(__mm_s) },
    })).clone();
    Ok(r#str)
}

// =============================================================================
// section for all debug* functions
//
// description: ???
// =============================================================================
pub(crate) fn debugStrCrefLstStr(mut a: ArcStr, mut b: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut c: ArcStr, mut d: ArcStr) -> Result<()> {
    metamodelica::print((a).clone());
    debuglst(b, (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>), (c).clone(), (d).clone())?;
    Ok(())
}

pub(crate) fn debugCrefStr(mut a: Arc<DAE::ComponentRef>, mut b: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(a)?); __mm_s.push_str(&*b); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugStrIntStr(mut a: ArcStr, mut b: i32, mut c: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*a); __mm_s.push_str(&*intString(b)); __mm_s.push_str(&*c); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugStrIntStrIntStr(mut a: ArcStr, mut b: i32, mut c: ArcStr, mut d: i32, mut e: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*a); __mm_s.push_str(&*intString(b)); __mm_s.push_str(&*c); __mm_s.push_str(&*intString(d)); __mm_s.push_str(&*e); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugCrefStrIntStr(mut a: Arc<DAE::ComponentRef>, mut b: ArcStr, mut c: i32, mut d: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(a)?); __mm_s.push_str(&*b); __mm_s.push_str(&*intString(c)); __mm_s.push_str(&*d); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugStrCrefStr(mut a: ArcStr, mut b: Arc<DAE::ComponentRef>, mut c: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*a); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(b)?); __mm_s.push_str(&*c); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugStrCrefStrIntStr(mut a: ArcStr, mut b: Arc<DAE::ComponentRef>, mut c: ArcStr, mut d: i32, mut e: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*a); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(b)?); __mm_s.push_str(&*c); __mm_s.push_str(&*intString(d)); __mm_s.push_str(&*e); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugStrCrefStrRealStrRealStrRealStr(mut a: ArcStr, mut b: Arc<DAE::ComponentRef>, mut c: ArcStr, mut d: metamodelica::Real, mut e: ArcStr, mut f: metamodelica::Real, mut g: ArcStr, mut h: metamodelica::Real, mut i: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*a); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(b)?); __mm_s.push_str(&*c); __mm_s.push_str(&*realString(d)); __mm_s.push_str(&*e); __mm_s.push_str(&*realString(f)); __mm_s.push_str(&*g); __mm_s.push_str(&*realString(h)); __mm_s.push_str(&*i); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugStrRealStrRealStrRealStrRealStr(mut a: ArcStr, mut b: metamodelica::Real, mut c: ArcStr, mut d: metamodelica::Real, mut e: ArcStr, mut f: metamodelica::Real, mut g: ArcStr, mut h: metamodelica::Real, mut i: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*a); __mm_s.push_str(&*realString(b)); __mm_s.push_str(&*c); __mm_s.push_str(&*realString(d)); __mm_s.push_str(&*e); __mm_s.push_str(&*realString(f)); __mm_s.push_str(&*g); __mm_s.push_str(&*realString(h)); __mm_s.push_str(&*i); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugStrCrefStrExpStr(mut a: ArcStr, mut b: Arc<DAE::ComponentRef>, mut c: ArcStr, mut d: Arc<DAE::Exp>, mut e: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*a); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(b)?); __mm_s.push_str(&*c); __mm_s.push_str(&*ExpressionBasics::printExpStr(d)?); __mm_s.push_str(&*e); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugStrCrefStrCrefStr(mut a: ArcStr, mut b: Arc<DAE::ComponentRef>, mut c: ArcStr, mut d: Arc<DAE::ComponentRef>, mut e: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*a); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(b)?); __mm_s.push_str(&*c); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(d)?); __mm_s.push_str(&*e); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugExpStr(mut a: Arc<DAE::Exp>, mut b: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(a)?); __mm_s.push_str(&*b); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugStrExpStr(mut a: ArcStr, mut b: Arc<DAE::Exp>, mut c: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*a); __mm_s.push_str(&*ExpressionBasics::printExpStr(b)?); __mm_s.push_str(&*c); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugStrExpLstStr(mut a: ArcStr, mut b: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut c: ArcStr, mut d: ArcStr) -> Result<()> {
    metamodelica::print((a).clone());
    debuglst(b, (std::sync::Arc::new(ExpressionBasics::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>), (c).clone(), (d).clone())?;
    Ok(())
}

pub(crate) fn debugStrExpStrCrefStr(mut a: ArcStr, mut b: Arc<DAE::Exp>, mut c: ArcStr, mut d: Arc<DAE::ComponentRef>, mut e: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*a); __mm_s.push_str(&*ExpressionBasics::printExpStr(b)?); __mm_s.push_str(&*c); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(d)?); __mm_s.push_str(&*e); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugStrExpStrExpStr(mut a: ArcStr, mut b: Arc<DAE::Exp>, mut c: ArcStr, mut d: Arc<DAE::Exp>, mut e: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*a); __mm_s.push_str(&*ExpressionBasics::printExpStr(b)?); __mm_s.push_str(&*c); __mm_s.push_str(&*ExpressionBasics::printExpStr(d)?); __mm_s.push_str(&*e); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugExpStrExpStrExpStr(mut a: Arc<DAE::Exp>, mut b: ArcStr, mut c: Arc<DAE::Exp>, mut d: ArcStr, mut e: Arc<DAE::Exp>, mut f: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printExpStr(a)?); __mm_s.push_str(&*b); __mm_s.push_str(&*ExpressionBasics::printExpStr(c)?); __mm_s.push_str(&*d); __mm_s.push_str(&*ExpressionBasics::printExpStr(e)?); __mm_s.push_str(&*f); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugStrExpStrExpStrExpStr(mut a: ArcStr, mut b: Arc<DAE::Exp>, mut c: ArcStr, mut d: Arc<DAE::Exp>, mut e: ArcStr, mut f: Arc<DAE::Exp>, mut g: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*a); __mm_s.push_str(&*ExpressionBasics::printExpStr(b)?); __mm_s.push_str(&*c); __mm_s.push_str(&*ExpressionBasics::printExpStr(d)?); __mm_s.push_str(&*e); __mm_s.push_str(&*ExpressionBasics::printExpStr(f)?); __mm_s.push_str(&*g); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugStrEqnStr(mut a: ArcStr, mut b: Arc<BackendDAE::Equation>, mut c: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*a); __mm_s.push_str(&*equationString(b)?); __mm_s.push_str(&*c); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debugStrEqnStrEqnStr(mut a: ArcStr, mut b: Arc<BackendDAE::Equation>, mut c: ArcStr, mut d: Arc<BackendDAE::Equation>, mut e: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*a); __mm_s.push_str(&*equationString(b)?); __mm_s.push_str(&*c); __mm_s.push_str(&*equationString(d)?); __mm_s.push_str(&*e); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn debuglst<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(mut lst: Arc<metamodelica::List<Type_a>>, mut f: Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>, mut c: ArcStr, mut se: ArcStr) -> Result<()> {
    pub type FuncTypeType_aToStr<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>;

    let () = (::match_deref::match_deref! { match &(lst) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::print((se).clone());
            ()
        },
        Deref @ metamodelica::List::Cons { head: a, tail: Deref @ metamodelica::List::Nil } => {
            metamodelica::print((f(a.clone())?).clone());
            metamodelica::print((se).clone());
            ()
        },
        Deref @ metamodelica::List::Cons { head: a, tail: rest } => {
            metamodelica::print((f(a.clone())?).clone());
            metamodelica::print((c.clone()).clone());
            debuglst(rest.clone(), f.clone(), (c).clone(), (se).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

// =============================================================================
// unsorted section
//
// These section should be empty. Feel free to sort these functions into one of
// the upper sections.
// =============================================================================
pub(crate) fn printCallFunction2StrDIVISION<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(mut inExp: Arc<DAE::Exp>, mut stringDelimiter: ArcStr, mut opcreffunc: Option<(Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<ArcStr> + 'static>, Type_a)>) -> Result<ArcStr> {
    pub type strongComponentStringRefStrFunc<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<ArcStr> + 'static>;

    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::SCONST { string: _ }, tail: Deref @ metamodelica::List::Nil } } }, attr: Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut s: ArcStr;
            s = (ExpressionDump::printExp2Str(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV { ty: ty.clone() }, exp2: e2.clone() }), (stringDelimiter).clone(), opcreffunc, Some((std::sync::Arc::new(printCallFunction2StrDIVISION) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _) -> Result<ArcStr> + 'static>)))).clone();
            s.clone()
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION_ARRAY_SCALAR" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::SCONST { string: _ }, tail: Deref @ metamodelica::List::Nil } } }, attr: Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut s: ArcStr;
            s = (ExpressionDump::printExp2Str(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV_ARRAY_SCALAR { ty: ty.clone() }, exp2: e2.clone() }), (stringDelimiter).clone(), opcreffunc, Some((std::sync::Arc::new(printCallFunction2StrDIVISION) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _) -> Result<ArcStr> + 'static>)))).clone();
            s.clone()
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION_SCALAR_ARRAY" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::SCONST { string: _ }, tail: Deref @ metamodelica::List::Nil } } }, attr: Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut s: ArcStr;
            s = (ExpressionDump::printExp2Str(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::DIV_SCALAR_ARRAY { ty: ty.clone() }, exp2: e2.clone() }), (stringDelimiter).clone(), opcreffunc, Some((std::sync::Arc::new(printCallFunction2StrDIVISION) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _) -> Result<ArcStr> + 'static>)))).clone();
            s.clone()
        },
        Deref @ DAE::Exp::CALL { path: fcn, expLst: args, .. } => {
            let mut s: ArcStr;
            let mut s_1: ArcStr;
            let mut s_2: ArcStr;
            let mut fs: ArcStr;
            let mut argstr: ArcStr;
            fs = (AbsynUtil::pathString(fcn.clone(), (literal!(".")).clone(), true, false)?).clone();
            argstr = stringDelimitList(List::map3(args.clone(), std::sync::Arc::new(fnptr!(ExpressionDump::printExp2Str, Arc<DAE::Exp>, ArcStr, _, _)), (stringDelimiter).clone(), opcreffunc, Some((std::sync::Arc::new(printCallFunction2StrDIVISION) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr, _) -> Result<ArcStr> + 'static>)))?, (literal!(",")).clone());
            s = (stringAppend((fs.clone()).clone(), (literal!("(")).clone())).clone();
            s_1 = (stringAppend((s.clone()).clone(), (argstr.clone()).clone())).clone();
            s_2 = (stringAppend((s_1.clone()).clone(), (literal!(")")).clone())).clone();
            s_2.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

// protected function printVarsStatistics "author: PA
//
//   Prints statistics on variables, etc.
// "
//   input BackendDAE.Variables inVariables1;
//   input BackendDAE.Variables inVariables2;
// algorithm
//   _:=
//   matchcontinue (inVariables1,inVariables2)
//     local
//       String lenstr,bstr;
//       BackendDAE.VariableArray v1,v2;
//       Integer bsize1,n1,bsize2,n2;
//     case (BackendDAE.VARIABLES(varArr = v1,bucketSize = bsize1,numberOfVars = n1),BackendDAE.VARIABLES(varArr = v2,bucketSize = bsize2,numberOfVars = n2))
//       equation
//         print("Variable Statistics\n");
//         print("===================\n");
//         print("Number of variables: ");
//         lenstr = intString(n1);
//         print(lenstr);
//         print("\n");
//         print("Bucket size for variables: ");
//         bstr = intString(bsize1);
//         print(bstr);
//         print("\n");
//         print("Number of known variables: ");
//         lenstr = intString(n2);
//         print(lenstr);
//         print("\n");
//         print("Bucket size for known variables: ");
//         bstr = intString(bsize1);
//         print(bstr);
//         print("\n");
//       then
//         ();
//   end matchcontinue;
// end printVarsStatistics;
pub(crate) fn dumpWhenOperatorStr(mut inWhenOperator: BackendDAE::WhenOperator) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inWhenOperator {
        BackendDAE::WhenOperator::ASSIGN { left: ref e1, right: ref e, .. } => {
            let mut scr: ArcStr;
            let mut se: ArcStr;
            let mut r#str: ArcStr;
            scr = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            se = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str = stringAppendList(list![(scr.clone()).clone(), (literal!(" := ")).clone(), (se.clone()).clone()]);
            r#str.clone()
        },
        BackendDAE::WhenOperator::REINIT { stateVar: ref cr, value: ref e, .. } => {
            let mut scr: ArcStr;
            let mut se: ArcStr;
            let mut r#str: ArcStr;
            scr = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            se = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str = stringAppendList(list![(literal!("reinit(")).clone(), (scr.clone()).clone(), (literal!(",")).clone(), (se.clone()).clone(), (literal!(")")).clone()]);
            r#str.clone()
        },
        BackendDAE::WhenOperator::ASSERT { condition: ref e, message: ref e1, .. } => {
            let mut se: ArcStr;
            let mut se1: ArcStr;
            let mut r#str: ArcStr;
            se = (ExpressionBasics::printExpStr(e.clone())?).clone();
            se1 = (ExpressionBasics::printExpStr(e1.clone())?).clone();
            r#str = stringAppendList(list![(literal!("assert(")).clone(), (se.clone()).clone(), (literal!(",")).clone(), (se1.clone()).clone(), (literal!(")")).clone()]);
            r#str.clone()
        },
        BackendDAE::WhenOperator::TERMINATE { message: ref e, .. } => {
            let mut se: ArcStr;
            let mut r#str: ArcStr;
            se = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str = stringAppendList(list![(literal!("terminate(")).clone(), (se.clone()).clone(), (literal!(")")).clone()]);
            r#str.clone()
        },
        BackendDAE::WhenOperator::NORETCALL { exp: ref e, .. } => {
            ExpressionBasics::printExpStr(e.clone())?
        },
    })).clone();
    Ok(outString)
}

pub(crate) fn dumpOption<Type_A: Clone + 'static + metamodelica::gc::MMTrace>(mut inType: Option<Type_A>, mut infunc: Arc<dyn ::std::ops::Fn(Type_A) -> Result<()> + 'static>) -> Result<()> {
    pub type printType_A<Type_A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Type_A) -> Result<()> + 'static>;

    let () = (match inType {
        Some(mut a) => {
            infunc(a.clone())?;
            ()
        },
        _ => {
            ()
        },
    });
    Ok(())
}

pub(crate) fn dumpAlgorithms(mut ialgs: Arc<metamodelica::List<Arc<DAE::Algorithm>>>, mut indx: i32) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(ialgs) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Algorithm { statementLst: stmts }, tail: algs } => {
            let mut myStream: IOStream::IOStream;
            let mut is: ArcStr;
            is = (intString(indx)).clone();
            myStream = IOStream::create((literal!("")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = IOStream::append(myStream.clone(), (stringAppend((is.clone()).clone(), (literal!(". ")).clone())).clone())?;
            myStream = DAEDump::dumpAlgorithmStream(Arc::new(DAE::Element::ALGORITHM { algorithm_: Arc::new(DAE::Algorithm { statementLst: stmts.clone() }), source: DAE::emptyElementSource().clone() }), myStream.clone());
            IOStream::print(myStream.clone(), IOStream::stdOutput.clone())?;
            dumpAlgorithms(algs.clone(), indx + 1)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn dumpConstraints(mut ionstrs: Arc<metamodelica::List<Arc<DAE::Constraint>>>, mut indx: i32) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(ionstrs) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst: exps }, tail: constrs } => {
            let mut myStream: IOStream::IOStream;
            let mut is: ArcStr;
            is = (intString(indx)).clone();
            myStream = IOStream::create((literal!("")).clone(), openmodelica_util::IOStream::IOStreamType::LIST)?;
            myStream = IOStream::append(myStream.clone(), (stringAppend((is.clone()).clone(), (literal!(". ")).clone())).clone())?;
            myStream = DAEDump::dumpConstraintStream(list![Arc::new(DAE::Element::CONSTRAINT { constraints: Arc::new(DAE::Constraint::CONSTRAINT_EXPS { constraintLst: exps.clone() }), source: DAE::emptyElementSource().clone() })], myStream.clone())?;
            IOStream::print(myStream.clone(), IOStream::stdOutput.clone())?;
            dumpConstraints(constrs.clone(), indx + 1)?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn dumpSparsePatternArray(mut inSparsePatter: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Print sparse pattern: ")); __mm_s.push_str(&*intString(metamodelica::arrayLength(inSparsePatter.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    dumpSparsePattern2(Arc::new(inSparsePatter.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), 1)?;
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

pub(crate) fn dumpSparsePattern(mut inSparsePatter: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Print sparse pattern: ")); __mm_s.push_str(&*intString((inSparsePatter.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    dumpSparsePattern2(inSparsePatter, 1)?;
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

pub(crate) fn dumpSparsePattern2(mut inSparsePatter: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inInteger: i32) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inSparsePatter) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: elem, tail: rest } => {
            let mut sparsepatternStr: ArcStr;
            sparsepatternStr = (List::toString(elem.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Row[")); __mm_s.push_str(&*intString(inInteger)); __mm_s.push_str(&*literal!("] = ")); ArcStr::from(__mm_s) }).clone(), (literal!("{")).clone(), (literal!(";")).clone(), (literal!("}")).clone(), true, 0)?).clone();
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*sparsepatternStr.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            dumpSparsePattern2(rest.clone(), inInteger + 1)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn dumpJacobianStr(mut inTplIntegerIntegerEquationLstOption: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inTplIntegerIntegerEquationLstOption) {
        Some(eqns) => {
            let mut res: Arc<metamodelica::List<ArcStr>>;
            let mut res_1: ArcStr;
            res = dumpJacobianStr2(eqns.clone())?;
            res_1 = stringDelimitList(res.clone(), (literal!(",\n")).clone());
            res_1.clone()
        },
        None => {
            literal!("No analytic jacobian available\n")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn dumpJacobianStr2(mut inTplIntegerIntegerEquationLst: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>>;
    outStringLst = (::match_deref::match_deref! { match &(inTplIntegerIntegerEquationLst) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: (row, col, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. }), tail: eqns } => {
            let mut estr: ArcStr;
            let mut rowstr: ArcStr;
            let mut colstr: ArcStr;
            let mut r#str: ArcStr;
            let mut strs: Arc<metamodelica::List<ArcStr>>;
            estr = (ExpressionBasics::printExpStr(e.clone())?).clone();
            rowstr = (intString(row.clone())).clone();
            colstr = (intString(col.clone())).clone();
            r#str = stringAppendList(list![(literal!("{")).clone(), (rowstr.clone()).clone(), (literal!(",")).clone(), (colstr.clone()).clone(), (literal!("}:")).clone(), (estr.clone()).clone()]);
            strs = dumpJacobianStr2(eqns.clone())?;
            metamodelica::cons((r#str.clone()).clone(), strs.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStringLst)
}

pub(crate) fn jacobianTypeStr(mut inJacobianType: BackendDAE::JacobianType) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inJacobianType {
        BackendDAE::JacobianType::JAC_CONSTANT { .. } => literal!("Jacobian Constant"),
        BackendDAE::JacobianType::JAC_LINEAR { .. } => literal!("Jacobian Linear"),
        BackendDAE::JacobianType::JAC_NONLINEAR { .. } => literal!("Jacobian Nonlinear"),
        BackendDAE::JacobianType::JAC_GENERIC { .. } => literal!("Generic Jacobian via directional derivatives"),
        BackendDAE::JacobianType::JAC_NO_ANALYTIC { .. } => literal!("No analytic jacobian"),
    })).clone();
    Ok(outString)
}

pub(crate) fn dumpJacobianString(mut jacIn: Arc<BackendDAE::Jacobian>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(jacIn) {
        Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: fJac } => {
            let mut s: ArcStr;
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("###############\n")); __mm_s.push_str(&*literal!(" FULL_JACOBIAN \n")); __mm_s.push_str(&*literal!("###############\n\n")); __mm_s.push_str(&*dumpJacobianStr(fJac.clone())?); ArcStr::from(__mm_s) }).clone();
            metamodelica::print((s.clone()).clone());
            literal!("")
        },
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: Some(sJac), sparsePattern, .. } => {
            let mut dae: Arc<BackendDAE::BackendDAE>;
            (dae, _, _, _, _, _) = sJac.clone();
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("##################\n")); __mm_s.push_str(&*literal!(" GENERIC_JACOBIAN \n")); __mm_s.push_str(&*literal!("##################\n\n")); ArcStr::from(__mm_s) }).clone());
            dumpBackendDAE(dae.clone(), (literal!("Directional Derivatives System")).clone())?;
            dumpSparsityPattern(sparsePattern.clone(), (literal!("Sparse Pattern")).clone())?;
            literal!("")
        },
        Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: None, sparsePattern, .. } => {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("##################\n")); __mm_s.push_str(&*literal!(" GENERIC_JACOBIAN \n")); __mm_s.push_str(&*literal!("##################\n\n")); ArcStr::from(__mm_s) }).clone());
            dumpSparsityPattern(sparsePattern.clone(), (literal!("Sparse Pattern")).clone())?;
            literal!("")
        },
        Deref @ BackendDAE::Jacobian::EMPTY_JACOBIAN { .. } => {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("################\n")); __mm_s.push_str(&*literal!(" EMPTY_JACOBIAN \n")); __mm_s.push_str(&*literal!("################\n\n")); ArcStr::from(__mm_s) }).clone());
            literal!("")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn symJacString(mut jacIn: (Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>)) -> Result<ArcStr> {
    let mut sOut: ArcStr;
    sOut = ((::match_deref::match_deref! { match &(jacIn) {
        (Some(sJac), sparsePattern, _) => {
            let mut dae: Arc<BackendDAE::BackendDAE>;
            let mut s: ArcStr;
            (dae, _, _, _, _, _) = sJac.clone();
            s = (literal!("GENERIC JACOBIAN:\n")).clone();
            dumpBackendDAE(dae.clone(), (literal!("Directional Derivatives System")).clone())?;
            dumpSparsityPattern(sparsePattern.clone(), (literal!("Sparse Pattern")).clone())?;
            s.clone()
        },
        (None, sparsePattern, _) => {
            let mut s: ArcStr;
            s = (literal!("GENERIC JACOBIAN:\n")).clone();
            dumpSparsityPattern(sparsePattern.clone(), (literal!("Sparse Pattern")).clone())?;
            s.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(sOut)
}

pub(crate) fn dumpEqnsStr(mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(dumpEqnsStr2(eqns, 1, metamodelica::nil())?, (literal!("\n")).clone());
    Ok(r#str)
}

fn dumpEqnsStr2(mut inEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inInteger: i32, mut inAcc: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inEquationLst, inInteger, inAcc)) {
        (Deref @ metamodelica::List::Nil, _, acc) => {
            return Ok(acc.clone().reverse())
        },
        (Deref @ metamodelica::List::Cons { head: eqn, tail: eqns }, index, acc) => {
            let mut es: ArcStr;
            let mut is: ArcStr;
            let mut r#str: ArcStr;
            let mut index_1: i32;
            let mut acc = (*acc).clone();
            es = (equationString(eqn.clone())?).clone();
            is = (intString(index.clone())).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*is.clone()); __mm_s.push_str(&*literal!(" : ")); __mm_s.push_str(&*es.clone()); ArcStr::from(__mm_s) }).clone();
            index_1 = index.clone() + 1;
            acc = metamodelica::cons((r#str.clone()).clone(), acc.clone());
            { (inEquationLst, inInteger, inAcc) = (eqns.clone(), index_1.clone(), acc.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn ifequationString(mut conditions: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut eqnstrue: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>>, mut eqnsfalse: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut iString: ArcStr) -> Result<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((conditions, eqnstrue, eqnsfalse.clone())) {
        (Deref @ metamodelica::List::Nil, _, Deref @ metamodelica::List::Nil) => {
            let mut s: ArcStr;
            return Ok(stringAppendList(list![(iString).clone(), (literal!("\nend if")).clone()]))
        },
        (Deref @ metamodelica::List::Nil, _, _) => {
            let mut seqns: ArcStr;
            let mut s: ArcStr;
            seqns = stringDelimitList(List::map(eqnsfalse, (std::sync::Arc::new(equationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>))?, (literal!("\n  ")).clone());
            return Ok(stringAppendList(list![(iString).clone(), (literal!("\nelse\n  ")).clone(), (seqns.clone()).clone(), (literal!("\nend if")).clone()]))
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: elst }, Deref @ metamodelica::List::Cons { head: eqns, tail: eqnslst }, _) => {
            let mut seqns: ArcStr;
            let mut s: ArcStr;
            let mut se: ArcStr;
            se = (ExpressionBasics::printExpStr(e.clone())?).clone();
            seqns = stringDelimitList(List::map(eqns.clone(), (std::sync::Arc::new(equationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>))?, (literal!("\n  ")).clone());
            s = stringAppendList(list![(iString).clone(), (literal!("\nelseif ")).clone(), (se.clone()).clone(), (literal!(" then\n  ")).clone(), (seqns.clone()).clone()]);
            { (conditions, eqnstrue, eqnsfalse, iString) = (elst.clone(), eqnslst.clone(), eqnsfalse, (s.clone()).clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn varString(mut inVar: BackendDAE::Var) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>>;
    let mut paths_lst: Arc<metamodelica::List<ArcStr>>;
    let mut unreplaceableStr: ArcStr;
    let mut dimensions: ArcStr;
    paths = ElementSource::getElementSourceTypes(inVar.source.clone());
    paths_lst = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut p in (paths).into_iter().cloned() {
            let __x = AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    unreplaceableStr = (if (inVar.unreplaceable.clone()) {literal!(" unreplaceable")} else {literal!("")}).clone();
    dimensions = (ExpressionBasics::dimensionsString(inVar.arryDim.clone())?).clone();
    dimensions = (if (dimensions.clone() != literal!("")) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" [")); __mm_s.push_str(&*dimensions); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }} else {literal!("")}).clone();
    outStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*DAEDump::dumpDirectionStr(inVar.varDirection.clone())?); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inVar.varName.clone())?); __mm_s.push_str(&*if (isSome(inVar.tplExp.clone())) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(Util::getOption(inVar.tplExp.clone())?)?); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*kindString(inVar.varKind.clone())?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*connectorTypeString(inVar.connectorType.clone())); __mm_s.push_str(&*attributesString(inVar.values.clone())?); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*optExpressionString(inVar.bindExp.clone(), (literal!("")).clone())?); __mm_s.push_str(&*DAEDumpTypes::dumpCommentAnnotationStr(inVar.comment.clone())); __mm_s.push_str(&*stringDelimitList(paths_lst, (literal!(", ")).clone())); __mm_s.push_str(&*literal!(" type: ")); __mm_s.push_str(&*DAEDump::daeTypeStr(inVar.varType.clone())?); __mm_s.push_str(&*dimensions); __mm_s.push_str(&*unreplaceableStr); ArcStr::from(__mm_s) }).clone();
    Ok(outStr)
}

pub(crate) fn varStringShort(mut inVar: BackendDAE::Var) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = (ComponentReferenceBasics::printComponentRefStr(inVar.varName.clone())?).clone();
    Ok(outStr)
}

pub(crate) fn dumpKind(mut inVarKind: BackendDAE::VarKind) -> Result<()> {
    metamodelica::print((kindString(inVarKind)?).clone());
    Ok(())
}

pub(crate) fn kindString(mut inVarKind: BackendDAE::VarKind) -> Result<ArcStr> {
    let mut kindStr: ArcStr;
    kindStr = ((::match_deref::match_deref! { match &(inVarKind) {
        BackendDAE::VarKind::VARIABLE { .. } => {
            literal!("VARIABLE")
        },
        BackendDAE::VarKind::STATE { index: i, derName: None, .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("STATE(")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        BackendDAE::VarKind::STATE { index: i, derName: Some(dcr), .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("STATE(")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(dcr.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        BackendDAE::VarKind::STATE_DER { .. } => {
            literal!("STATE_DER")
        },
        BackendDAE::VarKind::DUMMY_DER { .. } => {
            literal!("DUMMY_DER")
        },
        BackendDAE::VarKind::DUMMY_STATE { .. } => {
            literal!("DUMMY_STATE")
        },
        BackendDAE::VarKind::CLOCKED_STATE { .. } => {
            literal!("CLOCKED_STATE")
        },
        BackendDAE::VarKind::DISCRETE { .. } => {
            literal!("DISCRETE")
        },
        BackendDAE::VarKind::PARAM { .. } => {
            literal!("PARAM")
        },
        BackendDAE::VarKind::CONST { .. } => {
            literal!("CONST")
        },
        BackendDAE::VarKind::EXTOBJ { fullClassName: path } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("EXTOBJ: ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }
        },
        BackendDAE::VarKind::JAC_VAR { .. } => {
            literal!("JACOBIAN_VAR")
        },
        BackendDAE::VarKind::JAC_TMP_VAR { .. } => {
            literal!("JACOBIAN_TMP_VAR")
        },
        BackendDAE::VarKind::OPT_CONSTR { .. } => {
            literal!("OPT_CONSTR")
        },
        BackendDAE::VarKind::OPT_FCONSTR { .. } => {
            literal!("OPT_FCONSTR")
        },
        BackendDAE::VarKind::OPT_INPUT_WITH_DER { .. } => {
            literal!("OPT_INPUT_WITH_DER")
        },
        BackendDAE::VarKind::OPT_INPUT_DER { .. } => {
            literal!("OPT_INPUT_DER")
        },
        BackendDAE::VarKind::OPT_TGRID { .. } => {
            literal!("OPT_TGRID")
        },
        BackendDAE::VarKind::OPT_LOOP_INPUT { .. } => {
            literal!("OPT_LOOP_INPUT")
        },
        BackendDAE::VarKind::ALG_STATE { .. } => {
            literal!("ALG_STATE")
        },
        BackendDAE::VarKind::ALG_STATE_OLD { .. } => {
            literal!("ALG_STATE_OLD")
        },
        BackendDAE::VarKind::DAE_RESIDUAL_VAR { .. } => {
            literal!("DAE_RESIDUAL_VAR")
        },
        BackendDAE::VarKind::DAE_AUX_VAR { .. } => {
            literal!("DAE_AUX_VAR")
        },
        BackendDAE::VarKind::LOOP_ITERATION { .. } => {
            literal!("LOOP_ITERATION")
        },
        BackendDAE::VarKind::LOOP_SOLVED { .. } => {
            literal!("LOOP_SOLVED")
        },
        _ => {
            literal!("ERROR: BackendDump.kindString varKind not implemented")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(kindStr)
}

pub(crate) fn dumpConnectorType(mut inConnectorType: Arc<DAE::ConnectorType>) -> Result<()> {
    metamodelica::print((connectorTypeString(inConnectorType)).clone());
    Ok(())
}

pub(crate) fn connectorTypeString(mut inConnectorType: Arc<DAE::ConnectorType>) -> ArcStr {
    let mut connectorTypeStr: ArcStr;
    connectorTypeStr = ((::match_deref::match_deref! { match &(inConnectorType) {
        Deref @ DAE::ConnectorType::FLOW { .. } => literal!("flow=true "),
        Deref @ DAE::ConnectorType::POTENTIAL { .. } => literal!("flow=false "),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    connectorTypeStr
}

pub(crate) fn dumpAttributes(mut inAttr: Option<Arc<DAE::VariableAttributes>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inAttr) {
        None => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { min: None, max: None, start: None, fixed: None, nominal: None, stateSelectOption: None, isProtected: None, finalPrefix: None, distributionOption: None, .. }) => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { min, max, start, fixed, nominal, stateSelectOption, isProtected, finalPrefix, distributionOption: dist, .. }) => {
            dumpOptExpression(min.clone(), (literal!("min")).clone())?;
            dumpOptExpression(max.clone(), (literal!("max")).clone())?;
            dumpOptExpression(start.clone(), (literal!("start")).clone())?;
            dumpOptExpression(fixed.clone(), (literal!("fixed")).clone())?;
            dumpOptExpression(nominal.clone(), (literal!("nominal")).clone())?;
            dumpOptStateSelection(stateSelectOption.clone())?;
            dumpOptBoolean(isProtected.clone(), (literal!("protected")).clone())?;
            dumpOptBoolean(finalPrefix.clone(), (literal!("final")).clone())?;
            dumpOptDistribution(dist.clone())?;
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { min: None, max: None, start: None, fixed: None, isProtected: None, finalPrefix: None, distributionOption: None, .. }) => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { min, max, start, fixed, isProtected, finalPrefix, distributionOption: dist, .. }) => {
            dumpOptExpression(min.clone(), (literal!("min")).clone())?;
            dumpOptExpression(max.clone(), (literal!("max")).clone())?;
            dumpOptExpression(start.clone(), (literal!("start")).clone())?;
            dumpOptExpression(fixed.clone(), (literal!("fixed")).clone())?;
            dumpOptBoolean(isProtected.clone(), (literal!("protected")).clone())?;
            dumpOptBoolean(finalPrefix.clone(), (literal!("final")).clone())?;
            dumpOptDistribution(dist.clone())?;
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start: None, fixed: None, isProtected: None, finalPrefix: None, .. }) => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start, fixed, isProtected, finalPrefix, .. }) => {
            dumpOptExpression(start.clone(), (literal!("start")).clone())?;
            dumpOptExpression(fixed.clone(), (literal!("fixed")).clone())?;
            dumpOptBoolean(isProtected.clone(), (literal!("protected")).clone())?;
            dumpOptBoolean(finalPrefix.clone(), (literal!("final")).clone())?;
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start: None, isProtected: None, finalPrefix: None, .. }) => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start, isProtected, finalPrefix, .. }) => {
            dumpOptExpression(start.clone(), (literal!("start")).clone())?;
            dumpOptBoolean(isProtected.clone(), (literal!("protected")).clone())?;
            dumpOptBoolean(finalPrefix.clone(), (literal!("final")).clone())?;
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { min: None, max: None, start: None, fixed: None, isProtected: None, finalPrefix: None, .. }) => {
            ()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { min, max, start, fixed, isProtected, finalPrefix, .. }) => {
            dumpOptExpression(min.clone(), (literal!("min")).clone())?;
            dumpOptExpression(max.clone(), (literal!("max")).clone())?;
            dumpOptExpression(start.clone(), (literal!("start")).clone())?;
            dumpOptExpression(fixed.clone(), (literal!("fixed")).clone())?;
            dumpOptBoolean(isProtected.clone(), (literal!("protected")).clone())?;
            dumpOptBoolean(finalPrefix.clone(), (literal!("final")).clone())?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpOptDistribution(mut dist: Option<Arc<DAE::Distribution>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(dist) {
        None => {
            ()
        },
        Some(Deref @ DAE::Distribution { name: e1, params: e2, paramNames: e3 }) => {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("distribution = Distribution(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e3.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn dumpOptStateSelection(mut ss: Option<DAE::StateSelect>) -> Result<()> {
    let () = (match ss {
        Some(DAE::StateSelect::NEVER { .. }) => {
            metamodelica::print((literal!("stateSelect=StateSelect.never ")).clone());
            ()
        },
        Some(DAE::StateSelect::AVOID { .. }) => {
            metamodelica::print((literal!("stateSelect=StateSelect.avoid ")).clone());
            ()
        },
        Some(DAE::StateSelect::DEFAULT { .. }) => (),
        Some(DAE::StateSelect::PREFER { .. }) => {
            metamodelica::print((literal!("stateSelect=StateSelect.prefer ")).clone());
            ()
        },
        Some(DAE::StateSelect::ALWAYS { .. }) => {
            metamodelica::print((literal!("stateSelect=StateSelect.alwas ")).clone());
            ()
        },
        _ => (),
    });
    Ok(())
}

fn dumpOptExpression(mut inExp: Option<Arc<DAE::Exp>>, mut inString: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inExp, inString)) {
        (Some(e), s) => {
            let mut se: ArcStr;
            let mut r#str: ArcStr;
            se = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str = stringAppendList(list![(s.clone()).clone(), (literal!(" = ")).clone(), (se.clone()).clone(), (literal!(" ")).clone()]);
            metamodelica::print((r#str.clone()).clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn dumpOptBoolean(mut inExp: Option<bool>, mut inString: ArcStr) -> Result<()> {
    let () = (match (inExp, inString) {
        (Some(true), mut s) => {
            let mut r#str: ArcStr;
            r#str = stringAppendList(list![(s.clone()).clone(), (literal!(" = true ")).clone()]);
            metamodelica::print((r#str.clone()).clone());
            ()
        },
        _ => {
            ()
        },
    });
    Ok(())
}

pub(crate) fn attributesString(mut inAttr: Option<Arc<DAE::VariableAttributes>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inAttr) {
        None => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { min: None, max: None, start: None, unit: None, fixed: None, nominal: None, stateSelectOption: None, isProtected: None, finalPrefix: None, distributionOption: None, uncertainOption: None, .. }) => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { min, max, start, unit, fixed, nominal, stateSelectOption, isProtected, finalPrefix, distributionOption: dist, uncertainOption: uncertainopt, .. }) => {
            let mut r#str: ArcStr;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*optExpressionString(min.clone(), (literal!("min")).clone())?); __mm_s.push_str(&*optExpressionString(max.clone(), (literal!("max")).clone())?); __mm_s.push_str(&*optExpressionString(start.clone(), (literal!("start")).clone())?); __mm_s.push_str(&*optExpressionString(unit.clone(), (literal!("unit")).clone())?); __mm_s.push_str(&*optExpressionString(fixed.clone(), (literal!("fixed")).clone())?); __mm_s.push_str(&*optExpressionString(nominal.clone(), (literal!("nominal")).clone())?); __mm_s.push_str(&*optStateSelectionString(stateSelectOption.clone())); __mm_s.push_str(&*optBooleanString(isProtected.clone(), (literal!("protected")).clone())); __mm_s.push_str(&*optBooleanString(finalPrefix.clone(), (literal!("final")).clone())); __mm_s.push_str(&*optDistributionString(dist.clone())?); __mm_s.push_str(&*optUncertainty(uncertainopt.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { min: None, max: None, start: None, fixed: None, isProtected: None, finalPrefix: None, distributionOption: None, uncertainOption: None, .. }) => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { min, max, start, fixed, isProtected, finalPrefix, uncertainOption: uncertainopt, .. }) => {
            let mut r#str: ArcStr;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*optExpressionString(min.clone(), (literal!("min")).clone())?); __mm_s.push_str(&*optExpressionString(max.clone(), (literal!("max")).clone())?); __mm_s.push_str(&*optExpressionString(start.clone(), (literal!("start")).clone())?); __mm_s.push_str(&*optExpressionString(fixed.clone(), (literal!("fixed")).clone())?); __mm_s.push_str(&*optBooleanString(isProtected.clone(), (literal!("protected")).clone())); __mm_s.push_str(&*optBooleanString(finalPrefix.clone(), (literal!("final")).clone())); __mm_s.push_str(&*optUncertainty(uncertainopt.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start: None, fixed: None, isProtected: None, finalPrefix: None, .. }) => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { start, fixed, isProtected, finalPrefix, .. }) => {
            let mut r#str: ArcStr;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*optExpressionString(start.clone(), (literal!("start")).clone())?); __mm_s.push_str(&*optExpressionString(fixed.clone(), (literal!("fixed")).clone())?); __mm_s.push_str(&*optBooleanString(isProtected.clone(), (literal!("protected")).clone())); __mm_s.push_str(&*optBooleanString(finalPrefix.clone(), (literal!("final")).clone())); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start: None, isProtected: None, finalPrefix: None, .. }) => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { start, isProtected, finalPrefix, .. }) => {
            let mut r#str: ArcStr;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*optExpressionString(start.clone(), (literal!("start")).clone())?); __mm_s.push_str(&*optBooleanString(isProtected.clone(), (literal!("protected")).clone())); __mm_s.push_str(&*optBooleanString(finalPrefix.clone(), (literal!("final")).clone())); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { min: None, max: None, start: None, fixed: None, isProtected: None, finalPrefix: None, .. }) => {
            literal!("")
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { min, max, start, fixed, isProtected, finalPrefix, .. }) => {
            let mut r#str: ArcStr;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*optExpressionString(min.clone(), (literal!("min")).clone())?); __mm_s.push_str(&*optExpressionString(max.clone(), (literal!("max")).clone())?); __mm_s.push_str(&*optExpressionString(start.clone(), (literal!("start")).clone())?); __mm_s.push_str(&*optExpressionString(fixed.clone(), (literal!("fixed")).clone())?); __mm_s.push_str(&*optBooleanString(isProtected.clone(), (literal!("protected")).clone())); __mm_s.push_str(&*optBooleanString(finalPrefix.clone(), (literal!("final")).clone())); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn optDistributionString(mut dist: Option<Arc<DAE::Distribution>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(dist) {
        None => {
            literal!("")
        },
        Some(Deref @ DAE::Distribution { name: e1, params: e2, paramNames: e3 }) => {
            let mut r#str: ArcStr;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("distribution = Distribution(")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e3.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn optUncertainty(mut uncertainty: Option<DAE::Uncertainty>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match uncertainty {
        None => literal!(""),
        Some(DAE::Uncertainty::GIVEN { .. }) => literal!("uncertain=Uncertainty.given"),
        Some(DAE::Uncertainty::SOUGHT { .. }) => literal!("uncertain=Uncertainty.sought"),
        Some(DAE::Uncertainty::REFINE { .. }) => literal!("uncertain=Uncertainty.refine"),
        Some(DAE::Uncertainty::PROPAGATE { .. }) => literal!("uncertain=Uncertainty.propagate"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn optStateSelectionString(mut ss: Option<DAE::StateSelect>) -> ArcStr {
    let mut outString: ArcStr;
    outString = ((match ss {
        Some(DAE::StateSelect::NEVER { .. }) => literal!("stateSelect=StateSelect.never "),
        Some(DAE::StateSelect::AVOID { .. }) => literal!("stateSelect=StateSelect.avoid "),
        Some(DAE::StateSelect::DEFAULT { .. }) => literal!(""),
        Some(DAE::StateSelect::PREFER { .. }) => literal!("stateSelect=StateSelect.prefer "),
        Some(DAE::StateSelect::ALWAYS { .. }) => literal!("stateSelect=StateSelect.always "),
        _ => literal!(""),
    })).clone();
    outString
}

pub(crate) fn partitionKindString(mut inPartitionKind: BackendDAE::BaseClockPartitionKind) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inPartitionKind {
        BackendDAE::BaseClockPartitionKind::CLOCKED_PARTITION { subPartIdx: mut idx } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("clocked partition(")); __mm_s.push_str(&*intString(idx.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        BackendDAE::BaseClockPartitionKind::CONTINUOUS_TIME_PARTITION { .. } => {
            literal!("continuous time partition")
        },
        BackendDAE::BaseClockPartitionKind::UNSPECIFIED_PARTITION { .. } => {
            literal!("unspecified partition")
        },
        BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION { .. } => {
            literal!("unknown partition")
        },
        _ => {
            Error::addInternalError((literal!("function partitionKindString failed")).clone(), metamodelica::sourceInfo!("BackEnd/BackendDump.mo"))?;
            bail!("fail")
        },
    })).clone();
    Ok(outString)
}

fn equationAttrString(mut inEqAttr: BackendDAE::EquationAttributes) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut kind: BackendDAE::EquationKind;
    let mut evalStages: BackendDAE::EvaluationStages;
    let BackendDAE::EQUATION_ATTRIBUTES { kind: __pa0, evalStages: __pa1, .. } = (inEqAttr) else { bail!("pattern mismatch") };
    kind = __pa0.clone();
    evalStages = __pa1.clone();
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*equationKindString(kind)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*equationEvaluationStageString(evalStages)); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
    Ok(outString)
}

fn equationKindString(mut inEqKind: BackendDAE::EquationKind) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inEqKind {
        BackendDAE::EquationKind::BINDING_EQUATION { .. } => {
            literal!("binding")
        },
        BackendDAE::EquationKind::DYNAMIC_EQUATION { .. } => {
            literal!("dynamic")
        },
        BackendDAE::EquationKind::INITIAL_EQUATION { .. } => {
            literal!("initial")
        },
        BackendDAE::EquationKind::AUX_EQUATION { .. } => {
            literal!("auxiliary")
        },
        BackendDAE::EquationKind::DISCRETE_EQUATION { .. } => {
            literal!("discrete")
        },
        BackendDAE::EquationKind::UNKNOWN_EQUATION_KIND { .. } => {
            literal!("unknown")
        },
        BackendDAE::EquationKind::CLOCKED_EQUATION { clk: mut i } => {
            let mut cr: Arc<DAE::ComponentRef>;
            cr = Arc::new(DAE::ComponentRef::CREF_IDENT { ident: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BackendDAE::WHENCLK_PRREFIX)); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), identType: DAE::T_CLOCK_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("clocked(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        _ => {
            Error::addInternalError((literal!("function equationKindString failed")).clone(), metamodelica::sourceInfo!("BackEnd/BackendDump.mo"))?;
            bail!("fail")
        },
    })).clone();
    Ok(outString)
}

fn equationEvaluationStageString(mut inEqEvalStage: BackendDAE::EvaluationStages) -> ArcStr {
    let mut outString: ArcStr = literal!("|");
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outString); __mm_s.push_str(&*if (inEqEvalStage.dynamicEval.clone()) {literal!("1|")} else {literal!("0|")}); ArcStr::from(__mm_s) }).clone();
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outString); __mm_s.push_str(&*if (inEqEvalStage.algebraicEval.clone()) {literal!("1|")} else {literal!("0|")}); ArcStr::from(__mm_s) }).clone();
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outString); __mm_s.push_str(&*if (inEqEvalStage.zerocrossEval.clone()) {literal!("1|")} else {literal!("0|")}); ArcStr::from(__mm_s) }).clone();
    outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outString); __mm_s.push_str(&*if (inEqEvalStage.discreteEval.clone()) {literal!("1|")} else {literal!("0|")}); ArcStr::from(__mm_s) }).clone();
    outString
}

fn optExpressionString(mut inExp: Option<Arc<DAE::Exp>>, mut inString: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inExp) {
        Some(e) => {
            let mut se: ArcStr;
            let mut r#str: ArcStr;
            se = (ExpressionBasics::printExpStr(e.clone())?).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inString); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*se.clone()); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn optBooleanString(mut inExp: Option<bool>, mut inString: ArcStr) -> ArcStr {
    let mut outString: ArcStr;
    outString = ((match inExp {
        Some(true) => {
            let mut r#str: ArcStr;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inString); __mm_s.push_str(&*literal!(" = true ")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => {
            literal!("")
        },
    })).clone();
    outString
}

pub(crate) fn dumpAdjacencyMatrix(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut rowIndex: i32 = 0;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nAdjacency Matrix (row: equation)\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("number of rows: ")); __mm_s.push_str(&*intString(metamodelica::arrayLength(m.clone()))); ArcStr::from(__mm_s) }).clone());
    let __range0 = m.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut row in __range0 {
        rowIndex = rowIndex + 1;
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*intString(rowIndex)); __mm_s.push_str(&*literal!(":")); ArcStr::from(__mm_s) }).clone());
        for mut i in &*row.clone() {
            let mut i = i.clone();
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone());
        }
    }
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

pub(crate) fn dumpAdjacencyMatrixT(mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut rowIndex: i32 = 0;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nTransposed Adjacency Matrix (row: variable)\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("number of rows: ")); __mm_s.push_str(&*intString(metamodelica::arrayLength(mT.clone()))); ArcStr::from(__mm_s) }).clone());
    let __range0 = mT.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut row in __range0 {
        rowIndex = rowIndex + 1;
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*intString(rowIndex)); __mm_s.push_str(&*literal!(":")); ArcStr::from(__mm_s) }).clone());
        for mut i in &*row.clone() {
            let mut i = i.clone();
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone());
        }
    }
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

pub(crate) fn dumpAdjacencyRow(mut inIntegerLst: Arc<metamodelica::List<i32>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inIntegerLst) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::print((literal!("\n")).clone());
            ()
        },
        Deref @ metamodelica::List::Cons { head: x, tail: xs } => {
            let mut s: ArcStr;
            s = (intString(x.clone())).clone();
            metamodelica::print((s.clone()).clone());
            metamodelica::print((literal!(" ")).clone());
            dumpAdjacencyRow(xs.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn dumpAdjacencyMatrixEnhanced(mut m: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<()> {
    let mut mlen: i32;
    let mut mlen_str: ArcStr;
    let mut m_1: Arc<metamodelica::List<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>>;
    metamodelica::print((literal!("Adjacency Matrix Enhanced (row == equation)\n")).clone());
    metamodelica::print((literal!("====================================\n")).clone());
    mlen = metamodelica::arrayLength(m.clone());
    mlen_str = (intString(mlen)).clone();
    metamodelica::print((literal!("number of rows: ")).clone());
    metamodelica::print((mlen_str).clone());
    metamodelica::print((literal!("\n")).clone());
    m_1 = Arc::new(m.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    dumpAdjacencyMatrixEnhanced2(m_1, 1)?;
    Ok(())
}

pub(crate) fn dumpAdjacencyMatrixTEnhanced(mut m: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<()> {
    let mut mlen: i32;
    let mut mlen_str: ArcStr;
    let mut m_1: Arc<metamodelica::List<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>>;
    metamodelica::print((literal!("Transpose Adjacency Matrix Enhanced (row == var)\n")).clone());
    metamodelica::print((literal!("=====================================\n")).clone());
    mlen = metamodelica::arrayLength(m.clone());
    mlen_str = (intString(mlen)).clone();
    metamodelica::print((literal!("number of rows: ")).clone());
    metamodelica::print((mlen_str).clone());
    metamodelica::print((literal!("\n")).clone());
    m_1 = Arc::new(m.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    dumpAdjacencyMatrixEnhanced2(m_1, 1)?;
    Ok(())
}

fn dumpAdjacencyMatrixEnhanced2(mut inRows: Arc<metamodelica::List<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>>, mut rowIndex: i32) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inRows) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: row, tail: rows } => {
            metamodelica::print((intString(rowIndex)).clone());
            metamodelica::print((literal!(":")).clone());
            dumpAdjacencyRowEnhanced(row.clone())?;
            dumpAdjacencyMatrixEnhanced2(rows.clone(), rowIndex + 1)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn dumpAdjacencyRowEnhanced(mut inRow: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inRow) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::print((literal!("\n")).clone());
            ()
        },
        Deref @ metamodelica::List::Cons { head: (x, solva, Deref @ metamodelica::List::Nil), tail: xs } => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            s = (intString(x.clone())).clone();
            s1 = (dumpSolvability(solva.clone())?).clone();
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone());
            metamodelica::print((literal!(" ")).clone());
            dumpAdjacencyRowEnhanced(xs.clone())?;
            ()
        },
        Deref @ metamodelica::List::Cons { head: (x, solva, cons), tail: xs } => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s = (intString(x.clone())).clone();
            s1 = (dumpSolvability(solva.clone())?).clone();
            s2 = (ExpressionDump::constraintDTlistToString(cons.clone(), (literal!(",")).clone())?).clone();
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone());
            metamodelica::print((literal!(" ")).clone());
            dumpAdjacencyRowEnhanced(xs.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn dumpSolvability(mut solva: BackendDAE::Solvability) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = ((match solva {
        BackendDAE::Solvability::SOLVABILITY_SOLVED { .. } => {
            literal!("solved")
        },
        BackendDAE::Solvability::SOLVABILITY_CONSTONE { .. } => {
            literal!("constone")
        },
        BackendDAE::Solvability::SOLVABILITY_CONST { b: mut b } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("const(")); __mm_s.push_str(&*boolString(b.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        BackendDAE::Solvability::SOLVABILITY_PARAMETER { b: mut b } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("param(")); __mm_s.push_str(&*boolString(b.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        BackendDAE::Solvability::SOLVABILITY_LINEAR { b: mut b } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("variable(")); __mm_s.push_str(&*boolString(b.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
        BackendDAE::Solvability::SOLVABILITY_NONLINEAR { .. } => {
            literal!("nonlinear")
        },
        BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE { .. } => {
            literal!("unsolvable")
        },
        BackendDAE::Solvability::SOLVABILITY_SOLVABLE { .. } => {
            literal!("solvable")
        },
    })).clone();
    Ok(s)
}

pub(crate) fn dumpFullMatching(mut inMatch: Arc<BackendDAE::Matching>, mut inSyst: Option<Arc<BackendDAE::EqSystem>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inMatch) {
        Deref @ BackendDAE::Matching::NO_MATCHING { .. } => {
            metamodelica::print((literal!("no matching\n")).clone());
            ()
        },
        Deref @ BackendDAE::Matching::MATCHING { ass1, ass2: _, comps } => {
            dumpMatching(ass1.clone())?;
            metamodelica::print((literal!("\n\n")).clone());
            dumpComponents(comps.clone(), inSyst)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn dumpMatching(mut v: metamodelica::Array<i32>) -> Result<()> {
    let mut len: i32;
    let mut len_str: ArcStr;
    metamodelica::print((literal!("Matching\n")).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    len = metamodelica::arrayLength(v.clone());
    len_str = (intString(len)).clone();
    metamodelica::print((len_str).clone());
    metamodelica::print((literal!(" variables and equations\n")).clone());
    dumpMatching2(v.clone(), 1, len);
    Ok(())
}

fn dumpMatching2(mut v: metamodelica::Array<i32>, mut i: i32, mut len: i32) -> () {
    let () = 'mc: {
        let __mc_input = len;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut eqn: i32;
            let mut s: ArcStr;
            let mut s2: ArcStr;
            let true = (intLe(i, len)) else { bail!("pattern mismatch") };
            s = (intString(i)).clone();
            eqn = ({let __elt = v.borrow()[(i-1) as usize].clone(); __elt});
            s2 = (intString(eqn.clone())).clone();
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("var ")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" is solved in eqn ")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            dumpMatching2(v.clone(), i + 1, len);
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

pub(crate) fn dumpMatchingVars(mut ass1: metamodelica::Array<i32>) -> Result<()> {
    let mut varIndex: i32 = 0;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nMatching\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(metamodelica::arrayLength(ass1.clone()))); __mm_s.push_str(&*literal!(" variables\n")); ArcStr::from(__mm_s) }).clone());
    let __range0 = ass1.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        varIndex = varIndex + 1;
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("var ")); __mm_s.push_str(&*intString(varIndex)); __mm_s.push_str(&*literal!(" is solved in eqn ")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

pub(crate) fn dumpMatchingEqns(mut ass2: metamodelica::Array<i32>) -> Result<()> {
    let mut eqnIndex: i32 = 0;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nMatching\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(metamodelica::arrayLength(ass2.clone()))); __mm_s.push_str(&*literal!(" equations\n")); ArcStr::from(__mm_s) }).clone());
    let __range0 = ass2.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut i in __range0 {
        eqnIndex = eqnIndex + 1;
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("eqn ")); __mm_s.push_str(&*intString(eqnIndex)); __mm_s.push_str(&*literal!(" is solved for var ")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

pub(crate) fn dumpMarkedEqns(mut syst: Arc<BackendDAE::EqSystem>, mut inIntegerLst: Arc<metamodelica::List<i32>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut slst: Arc<metamodelica::List<ArcStr>>;
    let __pa0 = ::match_deref::match_deref! { match &(syst) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqns = __pa0.clone();
    slst = List::map1(inIntegerLst, (std::sync::Arc::new(dumpMarkedEqns1) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<ArcStr> + 'static>), eqns)?;
    outString = stringDelimitList(slst, (literal!("\n")).clone());
    Ok(outString)
}

fn dumpMarkedEqns1(mut index: i32, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<ArcStr> {
    let mut outS: ArcStr;
    let mut eqn: Arc<BackendDAE::Equation>;
    eqn = BackendEquation::get(eqns, index)?;
    outS = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*intString(index)); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*equationString(eqn)?); ArcStr::from(__mm_s) }).clone();
    Ok(outS)
}

pub(crate) fn dumpMarkedVarsLsts(mut syst: Arc<BackendDAE::EqSystem>, mut inIntegerLstLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = literal!("");
    for mut inIntegerLst in &*inIntegerLstLst {
        let mut inIntegerLst = inIntegerLst.clone();
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outString.clone()); __mm_s.push_str(&*dumpMarkedVars(syst.clone(), inIntegerLst.clone())?); __mm_s.push_str(&*literal!(",")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(outString)
}

pub(crate) fn dumpMarkedVars(mut syst: Arc<BackendDAE::EqSystem>, mut inIntegerLst: Arc<metamodelica::List<i32>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut vars: BackendDAE::Variables;
    let mut slst: Arc<metamodelica::List<ArcStr>>;
    let __pa0 = ::match_deref::match_deref! { match &(syst) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    vars = __pa0.clone();
    slst = List::map1(inIntegerLst, (std::sync::Arc::new(dumpMarkedVars1) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables) -> Result<ArcStr> + 'static>), vars)?;
    outString = stringDelimitList(slst, (literal!("\n")).clone());
    Ok(outString)
}

fn dumpMarkedVars1(mut index: i32, mut vars: BackendDAE::Variables) -> Result<ArcStr> {
    let mut outS: ArcStr;
    let mut var: BackendDAE::Var;
    var = BackendVariable::getVarAt(vars, index)?;
    outS = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*intString(index)); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*varString(var)?); ArcStr::from(__mm_s) }).clone();
    Ok(outS)
}

pub(crate) fn dumpMarkedVarList(mut varList: Arc<metamodelica::List<BackendDAE::Var>>, mut selList: Arc<metamodelica::List<i32>>) -> Result<ArcStr> {
    let mut outString: ArcStr = literal!("");
    let mut var: BackendDAE::Var;
    for mut sel in &*selList {
        let mut sel = sel.clone();
        if let Ok(__iflet0) = (varList.clone()).get(sel.clone()) {
            var = __iflet0;
        } else {
            Error::addInternalError((literal!("function dumpMarkedVarList failed")).clone(), metamodelica::sourceInfo!("BackEnd/BackendDump.mo"))?;
            Error::addCompilerNotification(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Could not get variable ")); __mm_s.push_str(&*intString(sel.clone())); __mm_s.push_str(&*literal!(" from varList \n")); __mm_s.push_str(&*varListString(varList.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail");
        }
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*outString.clone()); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*varString(var.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(outString)
}

pub(crate) fn dumpComponentsGraphStr(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut n: i32;
    let mut lst: Arc<metamodelica::List<ArcStr>>;
    let mut s: ArcStr;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let (__pa4, __pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::List::Cons { head: __pa4 @ Deref @ BackendDAE::EqSystem { m: Some(__pa0), mT: Some(__pa1), matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa2, ass2: __pa3, .. }, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => (__pa4.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    m = __pa0.clone();
    mT = __pa1.clone();
    ass1 = __pa2.clone();
    ass2 = __pa3.clone();
    syst = __pa4.clone();
    n = BackendDAEUtil::systemSize(syst)?;
    lst = dumpComponentsGraphStr2(1, n, m.clone(), mT.clone(), ass1.clone(), ass2.clone())?;
    s = stringDelimitList(lst, (literal!(",")).clone());
    s = stringAppendList(list![(literal!("{")).clone(), (s).clone(), (literal!("}")).clone()]);
    metamodelica::print((s).clone());
    outDAE = inDAE;
    Ok(outDAE)
}

fn dumpComponentsGraphStr2(mut i: i32, mut n: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut ass1: metamodelica::Array<i32>, mut ass2: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut llst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut eqns: Arc<metamodelica::List<i32>>;
    let mut strLst: Arc<metamodelica::List<ArcStr>>;
    let mut slst: Arc<metamodelica::List<ArcStr>>;
    let mut r#str: ArcStr;
    if i <= n {
        eqns = Matching::reachableEquations(i, mT.clone(), ass2.clone());
        llst = List::map(eqns, std::sync::Arc::new(fnptr!(List::create, _)))?;
        llst = List::map1(llst, std::sync::Arc::new(fnptr!(List::consr, _, _)), i)?;
        slst = List::map(llst, (std::sync::Arc::new(intListStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))?;
        r#str = stringDelimitList(slst, (literal!(",")).clone());
        r#str = stringAppendList(list![(literal!("{")).clone(), (r#str).clone(), (literal!("}")).clone()]);
        strLst = dumpComponentsGraphStr2(i + 1, n, m.clone(), mT.clone(), ass1.clone(), ass2.clone())?;
        lst = metamodelica::cons((r#str).clone(), strLst);
    }
    Ok(lst)
}

pub(crate) fn dumpList(mut l: Arc<metamodelica::List<i32>>, mut r#str: ArcStr) -> Result<()> {
    let mut s: Arc<metamodelica::List<ArcStr>>;
    let mut sl: ArcStr;
    s = List::map(l, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
    sl = stringDelimitList(s, (literal!(", ")).clone());
    metamodelica::print((r#str).clone());
    metamodelica::print((sl).clone());
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

pub(crate) fn dumpComponentsOLD(mut l: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<()> {
    metamodelica::print((literal!("Blocks\n")).clone());
    metamodelica::print((literal!("=======\n")).clone());
    dumpComponents2(l, 1)?;
    Ok(())
}

fn dumpComponents2(mut inIntegerLstLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inInteger: i32) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((inIntegerLstLst, inInteger)) {
        (Deref @ metamodelica::List::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::List::Cons { head: l, tail: lst }, i) => {
            let mut i_1: i32;
            let mut ls: Arc<metamodelica::List<ArcStr>>;
            let mut s: ArcStr;
            metamodelica::print((literal!("{")).clone());
            ls = List::map(List::sort(l.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
            s = stringDelimitList(ls.clone(), (literal!(", ")).clone());
            metamodelica::print((s.clone()).clone());
            metamodelica::print((literal!("}\n")).clone());
            i_1 = i.clone() + 1;
            dumpComponents2(lst.clone(), i_1.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn intListStr(mut lst: Arc<metamodelica::List<i32>>) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = stringDelimitList(List::map(lst, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone());
    res = stringAppendList(list![(literal!("{")).clone(), (res).clone(), (literal!("}")).clone()]);
    Ok(res)
}

// protected function dumpAliasVariable
// "author: Frenkel TUD 2010-11"
//  input tuple<BackendDAE.Var,list<Integer>> inTpl;
//  output tuple<BackendDAE.Var,list<Integer>> outTpl;
// algorithm
//   outTpl:=
//   matchcontinue (inTpl)
//     local
//       BackendDAE.Var v;
//       DAE.ComponentRef cr;
//       DAE.Exp e;
//       String s,scr,se;
//     case ((v,_))
//       equation
//         cr = BackendVariable.varCref(v);
//         e = BackendVariable.varBindExp(v);
//         //print("### dump var : " +  ComponentReferenceBasics.printComponentRefStr(cr) + "\n");
//         scr = ComponentReferenceBasics.printComponentRefStr(cr);
//         se = ExpressionBasics.printExpStr(e);
//         s = stringAppendList({scr," = ",se,"\n"});
//         print(s);
//       then ((v,{}));
//     else inTpl;
//   end matchcontinue;
// end dumpAliasVariable;
pub(crate) fn dumpStateVariables(mut inVars: BackendDAE::Variables) -> Result<()> {
    metamodelica::print((literal!("States Variables\n")).clone());
    metamodelica::print((literal!("=================\n")).clone());
    BackendVariable::traverseBackendDAEVars(inVars, (std::sync::Arc::new(fnptr!(dumpStateVariable, BackendDAE::Var, i32)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, i32) -> Result<(BackendDAE::Var, i32)> + 'static>), 1)?;
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

fn dumpStateVariable(mut inVar: BackendDAE::Var, mut inPos: i32) -> (BackendDAE::Var, i32) {
    let mut v: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut pos: i32 = 0;
    (v, pos) = 'mc: {
        let __mc_input = (inVar.clone(), inPos);
        if let Ok(__v) = (|| -> Result<_> {
            let (mut v, mut pos) = __mc_input.clone() else { bail!("nomatch") };
            let mut cr: Arc<DAE::ComponentRef>;
            let mut scr: ArcStr;
            let true = (BackendVariable::isStateVar(v.clone())) else { bail!("pattern mismatch") };
            cr = BackendVariable::varCref(v.clone())?;
            scr = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
            metamodelica::print((intString(pos)).clone());
            metamodelica::print((literal!(": ")).clone());
            metamodelica::print((scr.clone()).clone());
            metamodelica::print((literal!("\n")).clone());
            Ok((v.clone(), pos + 1))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((inVar.clone(), inPos))
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (v, pos)
}

pub(crate) fn bltdump(mut headerline: ArcStr, mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inDAE.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    let mut strlow: ArcStr;
                    let Flags::STRING_FLAG { data: __pa0 } = (Flags::getConfigValue(Flags::DUMP_TARGET.clone())?) else { bail!("pattern mismatch") };
                    r#str = __pa0.clone();
                    strlow = (System::tolower((r#str.clone()).clone())).clone();
                    let true = (intGt(System::stringFind((strlow.clone()).clone(), (literal!(".html")).clone())?, 0)) else { bail!("pattern mismatch") };
                    DumpHTML::dumpDAE(inDAE.clone(), (headerline.clone()).clone(), (r#str.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::BackendDAE { eqs, shared } => {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*headerline.clone()); __mm_s.push_str(&*literal!(":\n")); ArcStr::from(__mm_s) }).clone());
                    List::map_0(eqs.clone(), (std::sync::Arc::new(printEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<()> + 'static>))?;
                    metamodelica::print((literal!("\n")).clone());
                    printShared(shared.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub(crate) fn innerEquationString(mut innerEquation: BackendDAE::InnerEquation) -> Result<ArcStr> {
    let mut s: ArcStr;
    let mut e: i32;
    let mut v: Arc<metamodelica::List<i32>>;
    (e, v, _) = BackendDAEUtil::getEqnAndVarsFromInnerEquation(innerEquation)?;
    s = stringDelimitList(List::map(v, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone());
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*intString(e)); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    Ok(s)
}

pub type DumpCompShortSystemsTpl = (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);

pub type DumpCompShortMixedTpl = (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);

pub type DumpCompShortTornTpl = (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);

pub(crate) fn dumpCompShort(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<()> {
    let mut sys: i32;
    let mut inp: i32;
    let mut st: i32;
    let mut dvar: i32;
    let mut dst: i32;
    let mut seq: i32;
    let mut salg: i32;
    let mut sarr: i32;
    let mut sce: i32;
    let mut swe: i32;
    let mut sie: i32;
    let mut eqsys: i32;
    let mut meqsys: i32;
    let mut teqsys: i32;
    let mut teqsys2: i32;
    let mut strcomps: i32;
    let mut e_jc: Arc<metamodelica::List<i32>>;
    let mut e_jn: Arc<metamodelica::List<i32>>;
    let mut e_nj: Arc<metamodelica::List<i32>>;
    let mut te_l: Arc<metamodelica::List<(i32, i32, i32)>>;
    let mut te_l2: Arc<metamodelica::List<(i32, i32, i32)>>;
    let mut te_nl: Arc<metamodelica::List<(i32, i32)>>;
    let mut te_nl2: Arc<metamodelica::List<(i32, i32)>>;
    let mut m_se: Arc<metamodelica::List<i32>>;
    let mut m_salg: Arc<metamodelica::List<i32>>;
    let mut m_sarr: Arc<metamodelica::List<i32>>;
    let mut m_sec: Arc<metamodelica::List<i32>>;
    let mut me_jc: Arc<metamodelica::List<(i32, i32)>>;
    let mut e_jt: Arc<metamodelica::List<(i32, i32)>>;
    let mut me_jt: Arc<metamodelica::List<(i32, i32)>>;
    let mut me_jn: Arc<metamodelica::List<(i32, i32)>>;
    let mut me_nj: Arc<metamodelica::List<(i32, i32)>>;
    let mut me_lt: Arc<metamodelica::List<(i32, i32)>>;
    let mut me_nt: Arc<metamodelica::List<(i32, i32)>>;
    let mut states: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut discvars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut discstates: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut clockedstates: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut HS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut removedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut sysStr: ArcStr;
    let mut stStr: ArcStr;
    let mut dvarStr: ArcStr;
    let mut dstStr: ArcStr;
    let mut clckStr: ArcStr;
    let mut statesStr: ArcStr;
    let mut discvarsStr: ArcStr;
    let mut discstatesStr: ArcStr;
    let mut clockedstatesStr: ArcStr;
    let mut inpStr: ArcStr;
    let mut strcompsStr: ArcStr;
    let mut seqStr: ArcStr;
    let mut sarrStr: ArcStr;
    let mut salgStr: ArcStr;
    let mut sceStr: ArcStr;
    let mut sweStr: ArcStr;
    let mut sieStr: ArcStr;
    let mut eqsysStr: ArcStr;
    let mut teqsysStr: ArcStr;
    let mut meqsysStr: ArcStr;
    let mut daeType: ArcStr;
    let mut msgs: Arc<metamodelica::List<ArcStr>>;
    let mut systemsTpl: DumpCompShortSystemsTpl;
    let mut mixedTpl: DumpCompShortMixedTpl;
    let mut tornTpl: DumpCompShortTornTpl;
    let mut tornTpl2: DumpCompShortTornTpl;
    let mut backendDAEType: BackendDAE::BackendDAEType;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, shared: Deref @ BackendDAE::Shared { backendDAEType: __pa1, .. } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    backendDAEType = __pa1.clone();
    removedEqs = BackendDAEUtil::collapseRemovedEqs(inDAE.clone())?;
    daeType = (printBackendDAEType2String(backendDAEType)?).clone();
    HS = HashSet::emptyHashSet();
    HS = List::fold(systs.clone(), (std::sync::Arc::new(Initialization::collectPreVariablesEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> + 'static>), HS)?;
    (_, HS) = BackendDAEUtil::traverseBackendDAEExpsEqns(removedEqs, (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(Initialization::collectPreVariablesTraverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> + 'static>), HS))?;
    discstates = BaseHashSet::hashSetList(HS)?;
    dst = (discstates.clone().len() as i32);
    for mut syst in &*systs {
        let mut syst = syst.clone();
        clockedstates = BackendVariable::filterCrefs(syst.orderedVars.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isVarClockedState, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>), clockedstates.clone())?;
    }
    (sys, inp, st, states, dvar, discvars, seq, salg, sarr, sce, swe, sie, systemsTpl, mixedTpl, tornTpl, tornTpl2) = BackendDAEUtil::foldEqSystem(inDAE, (std::sync::Arc::new(dumpCompShort1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (i32, i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))) -> Result<(i32, i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))> + 'static>), (0, 0, 0, metamodelica::nil(), 0, metamodelica::nil(), 0, 0, 0, 0, 0, 0, (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), (metamodelica::nil(), metamodelica::nil()), (metamodelica::nil(), metamodelica::nil())))?;
    (e_jc, e_jt, e_jn, e_nj) = systemsTpl.clone();
    (m_se, m_salg, m_sarr, m_sec, me_jc, me_jt, me_jn, me_nj, me_lt, me_nt) = mixedTpl.clone();
    (te_l, te_nl) = tornTpl.clone();
    (te_l2, te_nl2) = tornTpl2.clone();
    eqsys = (e_jc.len() as i32) + (e_jt.len() as i32) + (e_jn.len() as i32) + (e_nj.len() as i32);
    meqsys = (m_se.len() as i32) + (m_sarr.len() as i32) + (m_salg.len() as i32) + (m_sec.len() as i32) + (me_jc.len() as i32) + (me_jt.len() as i32) + (me_jn.len() as i32) + (me_nj.len() as i32) + (me_lt.len() as i32) + (me_nt.len() as i32);
    teqsys = (te_l.len() as i32) + (te_nl.len() as i32);
    teqsys2 = (te_l2.len() as i32) + (te_nl2.len() as i32);
    strcomps = seq + eqsys + meqsys + sarr + salg + sce + swe + sie + teqsys;
    sysStr = (intString(sys)).clone();
    stStr = (intString(st)).clone();
    dvarStr = (intString(dvar)).clone();
    dstStr = (intString(dst)).clone();
    clckStr = (intString((clockedstates.clone().len() as i32))).clone();
    statesStr = (if (Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())?) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*stringDelimitList(List::map(states, (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }} else {literal!(" ('-d=stateselection' for list of states)")}).clone();
    discvarsStr = (if (Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())?) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*stringDelimitList(List::map(discvars, (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }} else {literal!(" ('-d=discreteinfo' for list of discrete vars)")}).clone();
    discstatesStr = (if (Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())?) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*stringDelimitList(List::map(discstates, (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }} else {literal!(" ('-d=discreteinfo' for list of discrete states)")}).clone();
    clockedstatesStr = (if (Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())?) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*stringDelimitList(List::map(clockedstates, (std::sync::Arc::new(ComponentReferenceBasics::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }} else {literal!(" ('-d=discreteinfo' for list of clocked states)")}).clone();
    stStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stStr); __mm_s.push_str(&*statesStr); ArcStr::from(__mm_s) }).clone();
    dvarStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*dvarStr); __mm_s.push_str(&*discvarsStr); ArcStr::from(__mm_s) }).clone();
    dstStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*dstStr); __mm_s.push_str(&*discstatesStr); ArcStr::from(__mm_s) }).clone();
    clckStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*clckStr); __mm_s.push_str(&*clockedstatesStr); ArcStr::from(__mm_s) }).clone();
    inpStr = (intString(inp)).clone();
    msgs = list![(daeType.clone()).clone(), (sysStr).clone(), (stStr).clone(), (dvarStr).clone(), (dstStr).clone(), (clckStr).clone(), (inpStr).clone()];
    Error::addMessage(Error::BACKENDDAEINFO_STATISTICS.clone(), msgs)?;
    strcompsStr = (intString(strcomps)).clone();
    seqStr = (intString(seq)).clone();
    sarrStr = (intString(sarr)).clone();
    salgStr = (intString(salg)).clone();
    sceStr = (intString(sce)).clone();
    sweStr = (intString(swe)).clone();
    sieStr = (intString(sie)).clone();
    eqsysStr = (intString(eqsys)).clone();
    teqsysStr = (intString(teqsys)).clone();
    meqsysStr = (intString(meqsys)).clone();
    msgs = list![(daeType).clone(), (strcompsStr).clone(), (seqStr).clone(), (sarrStr).clone(), (salgStr).clone(), (sceStr).clone(), (sweStr).clone(), (sieStr).clone(), (eqsysStr).clone(), (teqsysStr).clone(), (meqsysStr).clone()];
    Error::addMessage(Error::BACKENDDAEINFO_STRONGCOMPONENT_STATISTICS.clone(), msgs)?;
    if intGt(eqsys, 0) {
        dumpCompSystems(systemsTpl)?;
    }
    if intGt(meqsys, 0) {
        dumpCompMixed(mixedTpl)?;
    }
    if intGt(teqsys, 0) {
        dumpCompTorn(tornTpl, (literal!("strict")).clone())?;
    }
    if intGt(teqsys2, 0) && !(stringEqual((Config::dynamicTearing()?).clone(), (literal!("false")).clone())) {
        dumpCompTorn(tornTpl2, (literal!("casual")).clone())?;
    }
    Ok(())
}

fn dumpCompSystems(mut systemsTpl: DumpCompShortSystemsTpl) -> Result<()> {
    let mut e_jc: Arc<metamodelica::List<i32>>;
    let mut e_jn: Arc<metamodelica::List<i32>>;
    let mut e_nj: Arc<metamodelica::List<i32>>;
    let mut e_jt: Arc<metamodelica::List<(i32, i32)>>;
    let mut s_jc: ArcStr;
    let mut s_jn: ArcStr;
    let mut s_nj: ArcStr;
    let mut s_jt: ArcStr;
    (e_jc, e_jt, e_jn, e_nj) = systemsTpl;
    s_jc = (equationSizesStr(e_jc, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?).clone();
    s_jt = (equationSizesStr(e_jt, (std::sync::Arc::new(sizeNumNonZeroTplString) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))?).clone();
    s_jn = (equationSizesStr(e_jn, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?).clone();
    s_nj = (equationSizesStr(e_nj, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?).clone();
    Error::addMessage(Error::BACKENDDAEINFO_SYSTEMS.clone(), list![(s_jc).clone(), (s_jt).clone(), (s_jn).clone(), (s_nj).clone()])?;
    Ok(())
}

fn dumpCompTorn(mut systemsTpl: DumpCompShortTornTpl, mut whichset: ArcStr) -> Result<()> {
    let mut te_l: Arc<metamodelica::List<(i32, i32, i32)>>;
    let mut te_nl: Arc<metamodelica::List<(i32, i32)>>;
    let mut s_l: ArcStr;
    let mut s_nl: ArcStr;
    (te_l, te_nl) = systemsTpl;
    s_l = (equationSizesStr(te_l, (std::sync::Arc::new(sizeNumNonZeroTornTplString) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32, i32)) -> Result<ArcStr> + 'static>))?).clone();
    s_nl = (equationSizesStr(te_nl, (std::sync::Arc::new(fnptr!(intTplString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))?).clone();
    Error::addMessage(Error::BACKENDDAEINFO_TORN.clone(), list![(whichset).clone(), (s_l).clone(), (s_nl).clone()])?;
    Ok(())
}

fn dumpCompMixed(mut mixedTpl: DumpCompShortMixedTpl) -> Result<()> {
    let mut m_se: Arc<metamodelica::List<i32>>;
    let mut m_salg: Arc<metamodelica::List<i32>>;
    let mut m_sarr: Arc<metamodelica::List<i32>>;
    let mut m_sec: Arc<metamodelica::List<i32>>;
    let mut me_jc: Arc<metamodelica::List<(i32, i32)>>;
    let mut me_jt: Arc<metamodelica::List<(i32, i32)>>;
    let mut me_jn: Arc<metamodelica::List<(i32, i32)>>;
    let mut me_nj: Arc<metamodelica::List<(i32, i32)>>;
    let mut me_lt: Arc<metamodelica::List<(i32, i32)>>;
    let mut me_nt: Arc<metamodelica::List<(i32, i32)>>;
    let mut s_se: ArcStr;
    let mut s_salg: ArcStr;
    let mut s_sarr: ArcStr;
    let mut s_sec: ArcStr;
    let mut s_jc: ArcStr;
    let mut s_jt: ArcStr;
    let mut s_jn: ArcStr;
    let mut s_nj: ArcStr;
    let mut s_lt: ArcStr;
    let mut s_nt: ArcStr;
    (m_se, m_salg, m_sarr, m_sec, me_jc, me_jt, me_jn, me_nj, me_lt, me_nt) = mixedTpl;
    s_se = (equationSizesStr(m_se, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?).clone();
    s_salg = (equationSizesStr(m_salg, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?).clone();
    s_sarr = (equationSizesStr(m_sarr, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?).clone();
    s_sec = (equationSizesStr(m_sec, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?).clone();
    s_jc = (equationSizesStr(me_jc, (std::sync::Arc::new(fnptr!(intTplString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))?).clone();
    s_jt = (equationSizesStr(me_jt, (std::sync::Arc::new(fnptr!(intTplString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))?).clone();
    s_jn = (equationSizesStr(me_jn, (std::sync::Arc::new(fnptr!(intTplString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))?).clone();
    s_nj = (equationSizesStr(me_nj, (std::sync::Arc::new(fnptr!(intTplString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))?).clone();
    s_lt = (equationSizesStr(me_lt, (std::sync::Arc::new(fnptr!(intTplString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))?).clone();
    s_nt = (equationSizesStr(me_nt, (std::sync::Arc::new(fnptr!(intTplString, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))?).clone();
    Error::addMessage(Error::BACKENDDAEINFO_MIXED.clone(), list![(s_se).clone(), (s_salg).clone(), (s_sarr).clone(), (s_sec).clone(), (s_jc).clone(), (s_jt).clone(), (s_jn).clone(), (s_nj).clone(), (s_lt).clone(), (s_nt).clone()])?;
    Ok(())
}

fn equationSizesStr<A: Clone + 'static + metamodelica::gc::MMTrace>(mut eqs: Arc<metamodelica::List<A>>, mut r#fn: Arc<dyn ::std::ops::Fn(A) -> Result<ArcStr> + 'static>) -> Result<ArcStr> {
    pub type AToStr<A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(A) -> Result<ArcStr> + 'static>;

    let mut r#str: ArcStr;
    let mut len: i32;
    len = (eqs.clone().len() as i32);
    r#str = (if (len == 1) {literal!("1 system")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(len)); __mm_s.push_str(&*literal!(" systems")); ArcStr::from(__mm_s) }}).clone();
    r#str = (if (len == 0) {r#str} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n   {")); __mm_s.push_str(&*stringDelimitList(List::map(eqs, r#fn.clone())?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }}).clone();
    Ok(r#str)
}

fn sizeNumNonZeroTplString(mut inTpl: (i32, i32)) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut sz: i32;
    let mut nnz: i32;
    let mut density: metamodelica::Real;
    (sz, nnz) = inTpl;
    density = realDiv((metamodelica::OrderedFloat(100.0_f64)) * (intReal(nnz)), (intReal(sz)) * (intReal(sz)));
    r#str = (System::snprintff((literal!("%.1f")).clone(), 20, density)?).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(sz)); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("%)")); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

fn sizeNumNonZeroTornTplString(mut inTpl: (i32, i32, i32)) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut sz: i32;
    let mut nnz: i32;
    let mut others: i32;
    let mut density: metamodelica::Real;
    (sz, others, nnz) = inTpl;
    density = if (nnz == 0) {metamodelica::OrderedFloat(0.0_f64)} else {realDiv((metamodelica::OrderedFloat(100.0_f64)) * (intReal(nnz)), (intReal(sz)) * (intReal(sz)))};
    r#str = (System::snprintff((literal!("%.1f")).clone(), 20, density)?).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(sz)); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(others)); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("%)")); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

fn intTplString(mut inTpl: (i32, i32)) -> ArcStr {
    let mut outStr: ArcStr;
    let mut e: i32;
    let mut d: i32;
    (d, e) = inTpl;
    outStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(d)); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(e)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    outStr
}

fn dumpCompShort1(mut inSyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inTpl: (i32, i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))) -> Result<(i32, i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))> {
    let mut outTpl: (i32, i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>));
    let mut vars: BackendDAE::Variables;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut sys: i32;
    let mut inp: i32;
    let mut st: i32;
    let mut dvar: i32;
    let mut seq: i32;
    let mut salg: i32;
    let mut sarr: i32;
    let mut sce: i32;
    let mut swe: i32;
    let mut sie: i32;
    let mut inp1: i32;
    let mut st1: i32;
    let mut dvar1: i32;
    let mut seq1: i32;
    let mut salg1: i32;
    let mut sarr1: i32;
    let mut sce1: i32;
    let mut swe1: i32;
    let mut sie1: i32;
    let mut eqsys: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    let mut eqsys1: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    let mut meqsys: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);
    let mut meqsys1: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);
    let mut teqsys: (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);
    let mut teqsys1: (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);
    let mut teqsys_2: (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);
    let mut teqsys1_2: (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>);
    let mut states: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut states1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut discvars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut discvars1: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let __pa0 = ::match_deref::match_deref! { match &(inSyst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    vars = __pa0.clone();
    (sys, inp, st, states, dvar, discvars, seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys_2) = inTpl;
    (inp1, st1, states1, dvar1, discvars1) = BackendVariable::traverseBackendDAEVars(vars, (std::sync::Arc::new(fnptr!(traversingisStateTopInputVarFinder, BackendDAE::Var, (i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> Result<(BackendDAE::Var, (i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>))> + 'static>), (inp, st, states, dvar, discvars))?;
    comps = BackendDAEUtil::getStrongComponents(inSyst);
    (seq1, salg1, sarr1, sce1, swe1, sie1, eqsys1, meqsys1, teqsys1, teqsys1_2) = List::fold(comps, (std::sync::Arc::new(dumpCompShort2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, (i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))) -> Result<(i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))> + 'static>), (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys_2))?;
    outTpl = (sys + 1, inp1, st1, states1, dvar1, discvars1, seq1, salg1, sarr1, sce1, swe1, sie1, eqsys1, meqsys1, teqsys1, teqsys1_2);
    Ok(outTpl)
}

fn traversingisStateTopInputVarFinder(mut inVar: BackendDAE::Var, mut inTpl: (i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) -> (BackendDAE::Var, (i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)) {
    let mut outVar: BackendDAE::Var;
    let mut outTpl: (i32, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, i32, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>);
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, (inp, st, states, dvar, discvars)) => {
                    let mut cr: Arc<DAE::ComponentRef>;
                    let true = (BackendVariable::isStateVar(v.clone())) else { bail!("pattern mismatch") };
                    cr = BackendVariable::varCref(v.clone())?;
                    Ok((v.clone(), (inp.clone(), st.clone() + 1, metamodelica::cons(cr.clone(), states.clone()), dvar.clone(), discvars.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, (inp, st, states, dvar, discvars)) => {
                    let mut cr: Arc<DAE::ComponentRef>;
                    let true = (BackendVariable::isVarDiscrete(v.clone())) else { bail!("pattern mismatch") };
                    cr = BackendVariable::varCref(v.clone())?;
                    Ok((v.clone(), (inp.clone(), st.clone(), states.clone(), dvar.clone() + 1, metamodelica::cons(cr.clone(), discvars.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, (inp, st, states, dvar, discvars)) => {
                    let true = (BackendVariable::isVarOnTopLevelAndInput(v.clone())) else { bail!("pattern mismatch") };
                    Ok((v.clone(), (inp.clone() + 1, st.clone(), states.clone(), dvar.clone(), discvars.clone())))
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

fn dumpCompShort2(mut inComp: Arc<BackendDAE::StrongComponent>, mut inTpl: (i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))) -> Result<(i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>))> {
    let mut outTpl: (i32, i32, i32, i32, i32, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), (Arc<metamodelica::List<(i32, i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>));
    outTpl = (::match_deref::match_deref! { match &((inComp.clone(), inTpl)) {
        (Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone() + 1, salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEARRAY { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone(), salg.clone(), sarr.clone() + 1, sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone() + 1, eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone(), salg.clone() + 1, sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone() + 1, swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, teqsys, teqsys2)) => {
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone() + 1, sie.clone(), eqsys.clone(), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: ilst, jacType: BackendDAE::JacobianType::JAC_CONSTANT { .. }, .. }, (seq, salg, sarr, sce, swe, sie, (e_jc, e_jt, e_jn, e_nj), meqsys, teqsys, teqsys2)) => {
            let mut e: i32;
            e = (ilst.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), (metamodelica::cons(e.clone(), e_jc.clone()), e_jt.clone(), e_jn.clone(), e_nj.clone()), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: ilst, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, jacType: BackendDAE::JacobianType::JAC_LINEAR { .. }, .. }, (seq, salg, sarr, sce, swe, sie, (e_jc, e_jt, e_jn, e_nj), meqsys, teqsys, teqsys2)) => {
            let mut e: i32;
            let mut nnz: i32;
            e = (ilst.clone().len() as i32);
            nnz = (jac.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), (e_jc.clone(), metamodelica::cons((e.clone(), nnz.clone()), e_jt.clone()), e_jn.clone(), e_nj.clone()), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: ilst, jacType: BackendDAE::JacobianType::JAC_NONLINEAR { .. }, .. }, (seq, salg, sarr, sce, swe, sie, (e_jc, e_jt, e_jn, e_nj), meqsys, teqsys, teqsys2)) => {
            let mut e: i32;
            e = (ilst.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), (e_jc.clone(), e_jt.clone(), metamodelica::cons(e.clone(), e_jn.clone()), e_nj.clone()), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: ilst, jacType: BackendDAE::JacobianType::JAC_GENERIC { .. }, .. }, (seq, salg, sarr, sce, swe, sie, (e_jc, e_jt, e_jn, e_nj), meqsys, teqsys, teqsys2)) => {
            let mut e: i32;
            e = (ilst.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), (e_jc.clone(), e_jt.clone(), metamodelica::cons(e.clone(), e_jn.clone()), e_nj.clone()), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: ilst, jacType: BackendDAE::JacobianType::JAC_NO_ANALYTIC { .. }, .. }, (seq, salg, sarr, sce, swe, sie, (e_jc, e_jt, e_jn, e_nj), meqsys, teqsys, teqsys2)) => {
            let mut e: i32;
            e = (ilst.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), (e_jc.clone(), e_jt.clone(), e_jn.clone(), metamodelica::cons(e.clone(), e_nj.clone())), meqsys.clone(), teqsys.clone(), teqsys2.clone())
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: ilst, innerEquations, jac: Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: _, sparsePattern: (_, _, _, nnz), coloring: _, .. }, .. }, casualTearingSet: None, linear: true, .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, (te_l, te_nl), (te_l2, te_nl2))) => {
            let mut e: i32;
            let mut d: i32;
            d = (ilst.clone().len() as i32);
            e = (innerEquations.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), (metamodelica::cons((d.clone(), e.clone(), nnz.clone()), te_l.clone()), te_nl.clone()), (metamodelica::cons((0, 0, 0), te_l2.clone()), te_nl2.clone()))
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: ilst, innerEquations, .. }, casualTearingSet: None, linear: false, .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, (te_l, te_nl), (te_l2, te_nl2))) => {
            let mut e: i32;
            let mut d: i32;
            d = (ilst.clone().len() as i32);
            e = (innerEquations.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), (te_l.clone(), metamodelica::cons((d.clone(), e.clone()), te_nl.clone())), (te_l2.clone(), metamodelica::cons((0, 0), te_nl2.clone())))
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: ilst, innerEquations, jac: Deref @ BackendDAE::Jacobian::EMPTY_JACOBIAN { .. }, .. }, casualTearingSet: None, linear: true, .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, (te_l, te_nl), (te_l2, te_nl2))) => {
            let mut e: i32;
            let mut d: i32;
            d = (ilst.clone().len() as i32);
            e = (innerEquations.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), (metamodelica::cons((d.clone(), e.clone(), 0), te_l.clone()), te_nl.clone()), (metamodelica::cons((0, 0, 0), te_l2.clone()), te_nl2.clone()))
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: ilst, innerEquations, jac: Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: _, sparsePattern: (_, _, _, nnz), coloring: _, .. }, .. }, casualTearingSet: Some(BackendDAE::TearingSet { tearingvars: ilst2, innerEquations: innerEquations2, jac: Deref @ BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: _, sparsePattern: (_, _, _, nnz2), coloring: _, .. }, .. }), linear: true, .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, (te_l, te_nl), (te_l2, te_nl2))) => {
            let mut e: i32;
            let mut d: i32;
            let mut e2: i32;
            let mut d2: i32;
            d = (ilst.clone().len() as i32);
            e = (innerEquations.clone().len() as i32);
            d2 = (ilst2.clone().len() as i32);
            e2 = (innerEquations2.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), (metamodelica::cons((d.clone(), e.clone(), nnz.clone()), te_l.clone()), te_nl.clone()), (metamodelica::cons((d2.clone(), e2.clone(), nnz2.clone()), te_l2.clone()), te_nl2.clone()))
        },
        (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: ilst, innerEquations, .. }, casualTearingSet: Some(BackendDAE::TearingSet { tearingvars: ilst2, innerEquations: innerEquations2, .. }), linear: false, .. }, (seq, salg, sarr, sce, swe, sie, eqsys, meqsys, (te_l, te_nl), (te_l2, te_nl2))) => {
            let mut e: i32;
            let mut d: i32;
            let mut e2: i32;
            let mut d2: i32;
            d = (ilst.clone().len() as i32);
            e = (innerEquations.clone().len() as i32);
            d2 = (ilst2.clone().len() as i32);
            e2 = (innerEquations2.clone().len() as i32);
            (seq.clone(), salg.clone(), sarr.clone(), sce.clone(), swe.clone(), sie.clone(), eqsys.clone(), meqsys.clone(), (te_l.clone(), metamodelica::cons((d.clone(), e.clone()), te_nl.clone())), (te_l2.clone(), metamodelica::cons((d2.clone(), e2.clone()), te_nl2.clone())))
        },
        _ => {
            metamodelica::print((literal!("dumpCompShort2 failed with:\n")).clone());
            dumpComponent(inComp, None)?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTpl)
}

pub(crate) fn dumpNrOfEquations(mut inDAE: Arc<BackendDAE::BackendDAE>, mut preStr: ArcStr) -> Result<()> {
    let mut nlst: Arc<metamodelica::List<i32>>;
    let mut n: i32;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let __pa0 = ::match_deref::match_deref! { match &(inDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    nlst = List::map(systs, (std::sync::Arc::new(BackendDAEUtil::systemSize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<i32> + 'static>))?;
    n = List::fold(nlst, (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 0)?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*preStr); __mm_s.push_str(&*literal!(" NrOfEquations: ")); __mm_s.push_str(&*intString(n)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

pub(crate) fn dumpCompInfo(mut compInfo: Arc<BackendDAE::CompInfo>) -> Result<()> {
    metamodelica::print((printCompInfo(compInfo)).clone());
    Ok(())
}

fn printCompInfo(mut compInfo: Arc<BackendDAE::CompInfo>) -> ArcStr {
    let mut sOut: ArcStr;
    sOut = ('mc: {
        let __mc_input = compInfo;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::COUNTER { comp, numAdds, numMul, numDiv, numTrig, numRelations: numRel, numLog, numOth, funcCalls: numFuncs } => {
                    let mut s: ArcStr;
                    s = (literal!("")).clone();
                    if BackendDAEUtil::isSingleEquationComp(comp.clone()) {
                        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SE ")); __mm_s.push_str(&*printComponent(comp.clone(), None)?); ArcStr::from(__mm_s) }).clone();
                    } else if BackendDAEUtil::isWhenComp(comp.clone()) {
                        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("WE ")); __mm_s.push_str(&*printComponent(comp.clone(), None)?); ArcStr::from(__mm_s) }).clone();
                    } else if BackendDAEUtil::isArrayComp(comp.clone()) {
                        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AE ")); __mm_s.push_str(&*printComponent(comp.clone(), None)?); ArcStr::from(__mm_s) }).clone();
                    }
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\tadd|")); __mm_s.push_str(&*intString(numAdds.clone())); __mm_s.push_str(&*literal!("\tmul|")); __mm_s.push_str(&*intString(numMul.clone())); __mm_s.push_str(&*literal!("\tdiv|")); __mm_s.push_str(&*intString(numDiv.clone())); __mm_s.push_str(&*literal!("\ttrig|")); __mm_s.push_str(&*intString(numTrig.clone())); __mm_s.push_str(&*literal!("\trel|")); __mm_s.push_str(&*intString(numRel.clone())); __mm_s.push_str(&*literal!("\tlog|")); __mm_s.push_str(&*intString(numLog.clone())); __mm_s.push_str(&*literal!("\toth|")); __mm_s.push_str(&*intString(numOth.clone())); __mm_s.push_str(&*literal!("\tfuncs|")); __mm_s.push_str(&*intString(numFuncs.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::SYSTEM { allOperations: allOps, comp, size, density: dens } => {
                    let mut s: ArcStr;
                    s = (literal!("")).clone();
                    if BackendDAEUtil::isLinearEqSystemComp(comp.clone()) {
                        s = (literal!("LSYS")).clone();
                    } else {
                        s = (literal!("NLSYS")).clone();
                    }
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*printComponent(comp.clone(), None)?); __mm_s.push_str(&*literal!("\tsize|")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!("\tdens|")); __mm_s.push_str(&*intString(((dens.clone() * metamodelica::OrderedFloat(100.0_f64)).0.floor() as i32))); __mm_s.push_str(&*printCompInfo(allOps.clone())); ArcStr::from(__mm_s) }).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::TORN_ANALYSE { tornEqs, otherEqs, comp, tornSize: size } => {
                    let mut s: ArcStr;
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TS ")); __mm_s.push_str(&*printComponent(comp.clone(), None)?); __mm_s.push_str(&*literal!("\tsize|")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\tthe torn eqs:\t")); __mm_s.push_str(&*printCompInfo(tornEqs.clone())); ArcStr::from(__mm_s) }).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\tthe other eqs:\t")); __mm_s.push_str(&*printCompInfo(otherEqs.clone())); ArcStr::from(__mm_s) }).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::CompInfo::NO_COMP { numAdds, numMul, numDiv, numTrig, numRelations: numRel, numLog, numOth, funcCalls: numFuncs } => {
                    let mut s: ArcStr;
                    s = (literal!("NC")).clone();
                    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\tadd|")); __mm_s.push_str(&*intString(numAdds.clone())); __mm_s.push_str(&*literal!("\tmul|")); __mm_s.push_str(&*intString(numMul.clone())); __mm_s.push_str(&*literal!("\tdiv|")); __mm_s.push_str(&*intString(numDiv.clone())); __mm_s.push_str(&*literal!("\ttrig|")); __mm_s.push_str(&*intString(numTrig.clone())); __mm_s.push_str(&*literal!("\trel|")); __mm_s.push_str(&*intString(numRel.clone())); __mm_s.push_str(&*literal!("\tlog|")); __mm_s.push_str(&*intString(numLog.clone())); __mm_s.push_str(&*literal!("\toth|")); __mm_s.push_str(&*intString(numOth.clone())); __mm_s.push_str(&*literal!("\tfuncs|")); __mm_s.push_str(&*intString(numFuncs.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("Dont know this compInfo\n"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    }).clone();
    sOut
}

// =============================================================================
// section for all html-dumping functions
//
// =============================================================================
pub(crate) fn dumpEqSystemMatrixHTML(mut sys: Arc<BackendDAE::EqSystem>) -> Result<()> {
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    if isSome(sys.m.clone()) {
        m = Util::getOption(sys.m.clone())?;
    } else {
        (_, m, _) = BackendDAEUtil::getAdjacencyMatrix(sys.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, false)?;
    }
    dumpEqSystem(sys.clone(), (literal!("SYS")).clone())?;
    dumpMatrixHTML(m.clone(), List::map(List::intRange(BackendDAEUtil::systemSize(sys.clone())?), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, List::map(BackendVariable::varList(sys.orderedVars.clone())?, (std::sync::Arc::new(varStringShort) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<ArcStr> + 'static>))?, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("MATRIX_")); __mm_s.push_str(&*intString(BackendDAEUtil::systemSize(sys)?)); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

pub(crate) fn dumpEqSystemBLTmatrixHTML(mut sys: Arc<BackendDAE::EqSystem>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = sys.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqs, m: _, mT: _, mapping: _, matching: Deref @ BackendDAE::Matching::MATCHING { comps, .. }, stateSets: _, partitionKind: _, removedEqs: _ } => {
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut vIdxs: Arc<metamodelica::List<i32>>;
                    let mut eIdxs: Arc<metamodelica::List<i32>>;
                    let mut vars = (*vars).clone();
                    let mut eqs = (*eqs).clone();
                    (varLst, vIdxs, eqLst, eIdxs) = BackendDAEUtil::getStrongComponentsVarsAndEquations(comps.clone(), vars.clone(), eqs.clone())?;
                    eqs = BackendEquation::listEquation(eqLst.clone())?;
                    vars = BackendVariable::listVar1(varLst.clone())?;
                    (m, _) = BackendDAEUtil::adjacencyMatrixDispatch(vars.clone(), eqs.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, false)?;
                    dumpMatrixHTML(m.clone(), List::map(eIdxs.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, List::map(vIdxs.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BLT_MATRIX_")); __mm_s.push_str(&*intString(BackendDAEUtil::systemSize(sys.clone())?)); ArcStr::from(__mm_s) }).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("dumpEqSystemBLTmatrixHTML does not output anything since there is no BLT sorting.")).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub(crate) fn dumpMatrixHTML(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowNames: Arc<metamodelica::List<ArcStr>>, mut columNames: Arc<metamodelica::List<ArcStr>>, mut fileName: ArcStr) -> Result<()> {
    let mut size: i32;
    size = metamodelica::arrayLength(m.clone());
    if (rowNames.clone().len() as i32) == size && (columNames.clone().len() as i32) == size {
        DumpHTML::dumpMatrixHTML(m.clone(), rowNames, columNames, (fileName).clone())?;
    } else {
        DumpHTML::dumpMatrixHTML(m.clone(), List::fill((literal!("?")).clone(), size), List::fill((literal!("?")).clone(), size), (fileName).clone())?;
    }
    Ok(())
}

// =============================================================================
// section for all graphML dumping functions
//
// =============================================================================
pub(crate) fn dumpBipartiteGraphDAE(mut dae: Arc<BackendDAE::BackendDAE>, mut fileName: ArcStr) -> Result<()> {
    let mut vars: BackendDAE::Variables;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut varAtts: Arc<metamodelica::List<(bool, ArcStr)>>;
    let mut eqAtts: Arc<metamodelica::List<(bool, ArcStr)>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dae) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqSysts = __pa0.clone();
    shared = __pa1.clone();
    eqLst = List::flatten(List::mapMap(eqSysts.clone(), (std::sync::Arc::new(fnptr!(BackendEquation::getEqnsFromEqSystem, Arc<BackendDAE::EqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> + 'static>), (std::sync::Arc::new(BackendEquation::equationList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> + 'static>))?)?;
    varLst = List::flatten(List::mapMap(eqSysts, (std::sync::Arc::new(fnptr!(BackendVariable::daeVars, Arc<BackendDAE::EqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<BackendDAE::Variables> + 'static>), (std::sync::Arc::new(BackendVariable::varList) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables) -> Result<Arc<metamodelica::List<BackendDAE::Var>>> + 'static>))?)?;
    vars = BackendVariable::listVar1(varLst.clone())?;
    eqs = BackendEquation::listEquation(eqLst.clone())?;
    (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(Arc::new(BackendDAE::EqSystem { orderedVars: vars.clone(), orderedEqs: eqs.clone(), m: None, mT: None, mapping: None, matching: openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING(), stateSets: metamodelica::nil(), partitionKind: openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, removedEqs: BackendEquation::emptyEqns() }), openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(BackendDAEUtil::getFunctions(shared.clone())?), BackendDAEUtil::isInitializationDAE(shared))?;
    varAtts = List::threadMap(List::fill(false, (varLst.clone().len() as i32)), List::fill((literal!("")).clone(), (varLst.len() as i32)), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
    eqAtts = List::threadMap(List::fill(false, (eqLst.clone().len() as i32)), List::fill((literal!("")).clone(), (eqLst.len() as i32)), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
    dumpBipartiteGraphStrongComponent2(vars, eqs, m.clone(), varAtts, eqAtts, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BipartiteGraph_")); __mm_s.push_str(&*fileName); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

pub(crate) fn dumpBipartiteGraphEqSystem(mut syst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut fileName: ArcStr) -> Result<()> {
    let mut vars: BackendDAE::Variables;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mO: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>;
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut varAtts: Arc<metamodelica::List<(bool, ArcStr)>>;
    let mut eqAtts: Arc<metamodelica::List<(bool, ArcStr)>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(syst.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, m: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    vars = __pa0.clone();
    eqs = __pa1.clone();
    mO = __pa2.clone();
    varLst = BackendVariable::varList(vars.clone())?;
    varAtts = List::threadMap(List::fill(false, (varLst.clone().len() as i32)), List::fill((literal!("")).clone(), (varLst.len() as i32)), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
    eqAtts = List::threadMap(List::fill(false, BackendEquation::equationArraySize(eqs.clone())?), List::fill((literal!("")).clone(), BackendEquation::equationArraySize(eqs.clone())?), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
    if isSome(mO.clone()) {
        dumpBipartiteGraphStrongComponent2(vars, eqs, Util::getOption(mO)?, varAtts, eqAtts, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BipartiteGraph_")); __mm_s.push_str(&*fileName); ArcStr::from(__mm_s) }).clone())?;
    } else {
        (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(syst, openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(BackendDAEUtil::getFunctions(shared.clone())?), BackendDAEUtil::isInitializationDAE(shared))?;
        dumpBipartiteGraphStrongComponent2(vars, eqs, m.clone(), varAtts, eqAtts, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BipartiteGraph2_")); __mm_s.push_str(&*fileName); ArcStr::from(__mm_s) }).clone())?;
    }
    Ok(())
}

pub(crate) fn dumpBipartiteGraphStrongComponent(mut inComp: Arc<BackendDAE::StrongComponent>, mut eqSys: Arc<BackendDAE::EqSystem>, mut funcs: Option<Arc<AvlTreePathFunction::Tree>>, mut name: ArcStr) -> Result<()> {
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqSys) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    vars = __pa0.clone();
    eqs = __pa1.clone();
    varLst = BackendVariable::varList(vars)?;
    eqLst = BackendEquation::equationList(eqs)?;
    dumpBipartiteGraphStrongComponent1(inComp, eqLst, varLst, funcs, (name).clone())?;
    Ok(())
}

pub(crate) fn dumpBipartiteGraphStrongComponent1(mut inComp: Arc<BackendDAE::StrongComponent>, mut eqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut varsIn: Arc<metamodelica::List<BackendDAE::Var>>, mut funcs: Option<Arc<AvlTreePathFunction::Tree>>, mut graphName: ArcStr) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inComp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqIdcs, vars: varIdcs, .. } => {
                    let mut numEqs: i32;
                    let mut numVars: i32;
                    let mut varAtts: Arc<metamodelica::List<(bool, ArcStr)>>;
                    let mut eqAtts: Arc<metamodelica::List<(bool, ArcStr)>>;
                    let mut compEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut compVars: BackendDAE::Variables;
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut compEqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut compVarLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    compEqLst = List::map1(eqIdcs.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), eqsIn.clone())?;
                    compVarLst = List::map1(varIdcs.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), varsIn.clone())?;
                    compVars = BackendVariable::listVar1(compVarLst.clone())?;
                    compEqs = BackendEquation::listEquation(compEqLst.clone())?;
                    numEqs = (compEqLst.clone().len() as i32);
                    numVars = (compVarLst.clone().len() as i32);
                    (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(Arc::new(BackendDAE::EqSystem { orderedVars: compVars.clone(), orderedEqs: compEqs.clone(), m: None, mT: None, mapping: None, matching: openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING(), stateSets: metamodelica::nil(), partitionKind: openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, removedEqs: BackendEquation::emptyEqns() }), openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, funcs.clone(), false)?;
                    varAtts = List::threadMap(List::fill(false, numVars.clone()), List::fill((literal!("")).clone(), numVars.clone()), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
                    eqAtts = List::threadMap(List::fill(false, numEqs.clone()), List::fill((literal!("")).clone(), numEqs.clone()), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
                    dumpBipartiteGraphStrongComponent2(compVars.clone(), compEqs.clone(), m.clone(), varAtts.clone(), eqAtts.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rL_eqSys_")); __mm_s.push_str(&*graphName.clone()); ArcStr::from(__mm_s) }).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: rEqIdcs, tearingvars: tVarIdcs, innerEquations, .. }, .. } => {
                    let mut numEqs: i32;
                    let mut numVars: i32;
                    let mut tornInfo: Arc<metamodelica::List<bool>>;
                    let mut addInfo: Arc<metamodelica::List<ArcStr>>;
                    let mut eqIdcs: Arc<metamodelica::List<i32>>;
                    let mut varIdcs: Arc<metamodelica::List<i32>>;
                    let mut tVarIdcsNew: Arc<metamodelica::List<i32>>;
                    let mut rEqIdcsNew: Arc<metamodelica::List<i32>>;
                    let mut varIdcsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut varAtts: Arc<metamodelica::List<(bool, ArcStr)>>;
                    let mut eqAtts: Arc<metamodelica::List<(bool, ArcStr)>>;
                    let mut compEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut compVars: BackendDAE::Variables;
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut compEqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut compVarLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    (eqIdcs, varIdcsLst, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>))?;
                    varIdcs = List::flatten(varIdcsLst.clone())?;
                    eqIdcs = listAppend(eqIdcs.clone(), rEqIdcs.clone());
                    varIdcs = listAppend(varIdcs.clone(), tVarIdcs.clone());
                    compEqLst = List::map1(eqIdcs.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), eqsIn.clone())?;
                    compVarLst = List::map1(varIdcs.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), varsIn.clone())?;
                    compVars = BackendVariable::listVar1(compVarLst.clone())?;
                    compEqs = BackendEquation::listEquation(compEqLst.clone())?;
                    numEqs = (compEqLst.clone().len() as i32);
                    numVars = (compVarLst.clone().len() as i32);
                    (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(Arc::new(BackendDAE::EqSystem { orderedVars: compVars.clone(), orderedEqs: compEqs.clone(), m: None, mT: None, mapping: None, matching: openmodelica_backend_types::BackendDAE::Matching::interned_NO_MATCHING(), stateSets: metamodelica::nil(), partitionKind: openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, removedEqs: BackendEquation::emptyEqns() }), openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, funcs.clone(), false)?;
                    addInfo = List::map(varIdcs.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
                    tornInfo = List::fill(true, numVars.clone());
                    tVarIdcsNew = List::intRange(numVars.clone() - (tVarIdcs.clone().len() as i32));
                    tornInfo = List::fold1(tVarIdcsNew.clone(), (std::sync::Arc::new(List::replaceAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<_> + 'static>), false, tornInfo.clone())?;
                    varAtts = List::threadMap(tornInfo.clone(), addInfo.clone(), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
                    addInfo = List::map(eqIdcs.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?;
                    tornInfo = List::fill(true, numEqs.clone());
                    rEqIdcsNew = List::intRange(numEqs.clone() - (rEqIdcs.clone().len() as i32));
                    tornInfo = List::fold1(rEqIdcsNew.clone(), (std::sync::Arc::new(List::replaceAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<_> + 'static>), false, tornInfo.clone())?;
                    eqAtts = List::threadMap(tornInfo.clone(), addInfo.clone(), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
                    dumpBipartiteGraphStrongComponent2(compVars.clone(), compEqs.clone(), m.clone(), varAtts.clone(), eqAtts.clone(), (graphName.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("dumpTornSystemBipartiteGraphML1 failed\n")).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub(crate) fn dumpBipartiteGraphStrongComponent2(mut varsIn: BackendDAE::Variables, mut eqsIn: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut varAtts: Arc<metamodelica::List<(bool, ArcStr)>>, mut eqAtts: Arc<metamodelica::List<(bool, ArcStr)>>, mut name: ArcStr) -> Result<()> {
    let mut nameAttIdx: i32;
    let mut typeAttIdx: i32;
    let mut idxAttIdx: i32;
    let mut numVars: i32;
    let mut numEqs: i32;
    let mut varRange: Arc<metamodelica::List<i32>>;
    let mut eqRange: Arc<metamodelica::List<i32>>;
    let mut graphInfo: GraphML::GraphInfo;
    let mut graphIdx: i32;
    numEqs = BackendEquation::equationArraySize(eqsIn.clone())?;
    numVars = BackendVariable::varsSize(varsIn.clone());
    varRange = List::intRange(numVars);
    eqRange = List::intRange(numEqs);
    graphInfo = GraphML::createGraphInfo();
    let (__pa0, (_, __pa1)) = GraphML::addGraph((literal!("EqSystemGraph")).clone(), true, graphInfo)?;
    graphInfo = __pa0.clone();
    graphIdx = __pa1.clone();
    let (__pa2, (_, __pa3)) = GraphML::addAttribute((literal!("")).clone(), (literal!("type")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
    graphInfo = __pa2.clone();
    typeAttIdx = __pa3.clone();
    let (__pa4, (_, __pa5)) = GraphML::addAttribute((literal!("")).clone(), (literal!("name")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
    graphInfo = __pa4.clone();
    nameAttIdx = __pa5.clone();
    let (__pa6, (_, __pa7)) = GraphML::addAttribute((literal!("")).clone(), (literal!("systIdx")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
    graphInfo = __pa6.clone();
    idxAttIdx = __pa7.clone();
    (graphInfo, graphIdx) = addEqNodesToGraph(eqsIn, eqAtts, list![nameAttIdx, typeAttIdx, idxAttIdx], (graphInfo, graphIdx))?;
    (graphInfo, graphIdx) = List::fold3(varRange, (std::sync::Arc::new(addVarNodeToGraph) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables, Arc<metamodelica::List<(bool, ArcStr)>>, Arc<metamodelica::List<i32>>, (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> + 'static>), varsIn, varAtts, list![nameAttIdx, typeAttIdx, idxAttIdx], (graphInfo, graphIdx))?;
    graphInfo = List::fold1(eqRange, (std::sync::Arc::new(addEdgeToGraph) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, GraphML::GraphInfo) -> Result<GraphML::GraphInfo> + 'static>), mIn.clone(), graphInfo)?;
    GraphML::dumpGraph(graphInfo, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(".graphml")); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

fn addEqNodesToGraph(mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut attsIn: Arc<metamodelica::List<(bool, ArcStr)>>, mut attributeIdcs: Arc<metamodelica::List<i32>>, mut graphInfoIn: (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> {
    let mut graphInfoOut: (GraphML::GraphInfo, i32);
    let mut eq: Arc<BackendDAE::Equation>;
    let mut isResEq: bool;
    let mut nameAttrIdx: i32;
    let mut typeAttrIdx: i32;
    let mut idxAttrIdx: i32;
    let mut graphIdx: i32;
    let mut size: i32;
    let mut numEqs: i32;
    let mut e: i32;
    let mut eAbs: i32;
    let mut nextE: i32;
    let mut eqString: ArcStr;
    let mut eqNodeId: ArcStr;
    let mut idxString: ArcStr;
    let mut typeStr: ArcStr;
    let mut daeIdxStr: ArcStr;
    let mut graphInfo: GraphML::GraphInfo;
    let mut nodeLabel: GraphML::NodeLabel;
    nameAttrIdx = (attributeIdcs.clone()).get(1)?;
    typeAttrIdx = (attributeIdcs.clone()).get(2)?;
    idxAttrIdx = (attributeIdcs.clone()).get(3)?;
    (graphInfo, graphIdx) = graphInfoIn;
    numEqs = BackendEquation::getNumberOfEquations(eqs.clone());
    e = 1;
    eAbs = 1;
    size = 1;
    while e <= numEqs {
        eq = BackendEquation::get(eqs.clone(), e)?;
        size = BackendEquation::equationSize(eq.clone())?;
        nextE = eAbs + size;
        while nextE > eAbs {
            nameAttrIdx = (attributeIdcs.clone()).get(1)?;
            typeAttrIdx = (attributeIdcs.clone()).get(2)?;
            idxAttrIdx = (attributeIdcs.clone()).get(3)?;
            isResEq = Util::tuple21((attsIn.clone()).get(e)?);
            daeIdxStr = (Util::tuple22((attsIn.clone()).get(e)?)).clone();
            typeStr = (if (isResEq) {literal!("residualEq")} else {literal!("otherEq")}).clone();
            let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::getList(list![e], eqs.clone())?) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            eq = __pa0.clone();
            eqString = (equationString(eq.clone())?).clone();
            eqNodeId = (getEqNodeIdx(eAbs)).clone();
            idxString = (intString(eAbs)).clone();
            nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (idxString.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
            (graphInfo, _) = GraphML::addNode((eqNodeId.clone()).clone(), (arcstr::literal!(GraphML::COLOR_GREEN2)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![nodeLabel.clone()], openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE, Some((eqString.clone()).clone()), list![(nameAttrIdx, eqString.clone()), (typeAttrIdx, typeStr.clone()), (idxAttrIdx, daeIdxStr.clone())], graphIdx, graphInfo.clone())?;
            eAbs = eAbs + 1;
            size = size - 1;
        }
        e = e + 1;
    }
    graphInfoOut = (graphInfo, graphIdx);
    Ok(graphInfoOut)
}

pub(crate) fn dumpDAGStrongComponent(mut graphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut metaIn: HpcOmTaskGraph::TaskGraphMeta, mut fileName: ArcStr) -> Result<()> {
    let mut graphIdx: i32;
    let mut nameAttIdx: i32;
    let mut graphInfo: GraphML::GraphInfo;
    graphInfo = GraphML::createGraphInfo();
    let (__pa0, (_, __pa1)) = GraphML::addGraph((literal!("TornSystemGraph")).clone(), true, graphInfo)?;
    graphInfo = __pa0.clone();
    graphIdx = __pa1.clone();
    let (__pa2, (_, __pa3)) = GraphML::addAttribute((literal!("")).clone(), (literal!("Name")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
    graphInfo = __pa2.clone();
    nameAttIdx = __pa3.clone();
    graphInfo = buildGraphInfoDAG(graphIn.clone(), metaIn, graphInfo, graphIdx, list![nameAttIdx])?;
    GraphML::dumpGraph(graphInfo, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fileName); __mm_s.push_str(&*literal!(".graphml")); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

fn buildGraphInfoDAG(mut graphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut metaIn: HpcOmTaskGraph::TaskGraphMeta, mut graphInfoIn: GraphML::GraphInfo, mut graphIdx: i32, mut attIdcs: Arc<metamodelica::List<i32>>) -> Result<GraphML::GraphInfo> {
    let mut graphInfoOut: GraphML::GraphInfo;
    let mut nodeIdcs: Arc<metamodelica::List<i32>>;
    let mut nodes: Arc<metamodelica::List<GraphML::Node>>;
    let mut nameAttIdx: i32;
    nameAttIdx = listHead(attIdcs)?;
    nodeIdcs = List::intRange(metamodelica::arrayLength(graphIn.clone()));
    graphInfoOut = List::fold4(nodeIdcs, (std::sync::Arc::new(addNodeToDAG) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, i32, Arc<metamodelica::List<i32>>, GraphML::GraphInfo) -> Result<GraphML::GraphInfo> + 'static>), graphIn.clone(), metaIn, graphIdx, list![nameAttIdx], graphInfoIn)?;
    let GraphML::GRAPHINFO { nodes: __pa0, .. } = (graphInfoOut.clone()) else { bail!("pattern mismatch") };
    nodes = __pa0.clone();
    Ok(graphInfoOut)
}

fn addNodeToDAG(mut nodeIdx: i32, mut graphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut metaIn: HpcOmTaskGraph::TaskGraphMeta, mut graphIdx: i32, mut atts: Arc<metamodelica::List<i32>>, mut graphInfoIn: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut graphInfoOut: GraphML::GraphInfo;
    let mut tmpGraph: GraphML::GraphInfo;
    let mut nameAttIdx: i32;
    let mut childNodes: Arc<metamodelica::List<i32>>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeLabel: GraphML::NodeLabel;
    let mut nodeString: ArcStr;
    let mut nodeDesc: ArcStr;
    let mut compName: ArcStr;
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, compDescs: __pa1, .. } = (metaIn) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    compDescs = __pa1.clone();
    nodeDesc = (metamodelica::arrayGet(compDescs.clone(), nodeIdx)?).clone();
    nodeString = (intString(nodeIdx)).clone();
    compName = stringDelimitList(List::map(metamodelica::arrayGet(inComps.clone(), nodeIdx)?, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone());
    nameAttIdx = (atts).get(1)?;
    nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (nodeString).clone(), backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
    (tmpGraph, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(nodeIdx)); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_ORANGE)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![nodeLabel], openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE, Some((nodeDesc).clone()), list![(nameAttIdx, compName)], graphIdx, graphInfoIn)?;
    childNodes = metamodelica::arrayGet(graphIn.clone(), nodeIdx)?;
    graphInfoOut = List::fold1(childNodes, (std::sync::Arc::new(addDirectedEdge) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, GraphML::GraphInfo) -> Result<GraphML::GraphInfo> + 'static>), nodeIdx, tmpGraph)?;
    Ok(graphInfoOut)
}

fn addDirectedEdge(mut child: i32, mut parent: i32, mut graphInfoIn: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut graphInfoOut: GraphML::GraphInfo;
    (graphInfoOut, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Edge")); __mm_s.push_str(&*intString(parent)); __mm_s.push_str(&*intString(child)); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(child)); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(parent)); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_BLACK)).clone(), openmodelica_codegen_graphml::GraphML::LineType::LINE, GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE, openmodelica_codegen_graphml::GraphML::ArrowType::ARROWSTANDART), metamodelica::nil(), graphInfoIn)?;
    Ok(graphInfoOut)
}

fn addVarNodeToGraph(mut indx: i32, mut vars: BackendDAE::Variables, mut attsIn: Arc<metamodelica::List<(bool, ArcStr)>>, mut attributeIdcs: Arc<metamodelica::List<i32>>, mut graphInfoIn: (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> {
    let mut graphInfoOut: (GraphML::GraphInfo, i32);
    let mut var: BackendDAE::Var;
    let mut isTearVar: bool;
    let mut nameAttrIdx: i32;
    let mut typeAttIdx: i32;
    let mut idxAttrIdx: i32;
    let mut graphIdx: i32;
    let mut varString: ArcStr;
    let mut varNodeId: ArcStr;
    let mut idxString: ArcStr;
    let mut typeStr: ArcStr;
    let mut daeIdxStr: ArcStr;
    let mut graphInfo: GraphML::GraphInfo;
    let mut nodeLabel: GraphML::NodeLabel;
    (graphInfo, graphIdx) = graphInfoIn;
    nameAttrIdx = (attributeIdcs.clone()).get(1)?;
    typeAttIdx = (attributeIdcs.clone()).get(2)?;
    idxAttrIdx = (attributeIdcs).get(3)?;
    isTearVar = Util::tuple21((attsIn.clone()).get(indx)?);
    daeIdxStr = (Util::tuple22((attsIn).get(indx)?)).clone();
    typeStr = (if (isTearVar) {literal!("tearingVar")} else {literal!("otherVar")}).clone();
    var = BackendVariable::getVarAt(vars, indx)?;
    varString = (self::varString(var)?).clone();
    varNodeId = (getVarNodeIdx(indx)).clone();
    idxString = (intString(indx)).clone();
    nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (idxString).clone(), backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
    (graphInfo, _) = GraphML::addNode((varNodeId).clone(), (arcstr::literal!(GraphML::COLOR_ORANGE2)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![nodeLabel], openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE, Some((varString.clone()).clone()), list![(nameAttrIdx, varString), (typeAttIdx, typeStr), (idxAttrIdx, daeIdxStr)], graphIdx, graphInfo)?;
    graphInfoOut = (graphInfo, graphIdx);
    Ok(graphInfoOut)
}

fn addEqNodeToGraph(mut indx: i32, mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut attsIn: Arc<metamodelica::List<(bool, ArcStr)>>, mut attributeIdcs: Arc<metamodelica::List<i32>>, mut graphInfoIn: (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> {
    let mut graphInfoOut: (GraphML::GraphInfo, i32);
    let mut eq: Arc<BackendDAE::Equation>;
    let mut isResEq: bool;
    let mut nameAttrIdx: i32;
    let mut typeAttrIdx: i32;
    let mut idxAttrIdx: i32;
    let mut graphIdx: i32;
    let mut eqString: ArcStr;
    let mut eqNodeId: ArcStr;
    let mut idxString: ArcStr;
    let mut typeStr: ArcStr;
    let mut daeIdxStr: ArcStr;
    let mut graphInfo: GraphML::GraphInfo;
    let mut nodeLabel: GraphML::NodeLabel;
    (graphInfo, graphIdx) = graphInfoIn;
    nameAttrIdx = (attributeIdcs.clone()).get(1)?;
    typeAttrIdx = (attributeIdcs.clone()).get(2)?;
    idxAttrIdx = (attributeIdcs).get(3)?;
    isResEq = Util::tuple21((attsIn.clone()).get(indx)?);
    daeIdxStr = (Util::tuple22((attsIn).get(indx)?)).clone();
    typeStr = (if (isResEq) {literal!("residualEq")} else {literal!("otherEq")}).clone();
    let __pa0 = ::match_deref::match_deref! { match &(BackendEquation::getList(list![indx], eqs)?) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eq = __pa0.clone();
    eqString = (equationString(eq)?).clone();
    eqNodeId = (getEqNodeIdx(indx)).clone();
    idxString = (intString(indx)).clone();
    nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (idxString).clone(), backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN };
    (graphInfo, _) = GraphML::addNode((eqNodeId).clone(), (arcstr::literal!(GraphML::COLOR_GREEN2)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![nodeLabel], openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE, Some((eqString.clone()).clone()), list![(nameAttrIdx, eqString), (typeAttrIdx, typeStr), (idxAttrIdx, daeIdxStr)], graphIdx, graphInfo)?;
    graphInfoOut = (graphInfo, graphIdx);
    Ok(graphInfoOut)
}

fn addEdgeToGraph(mut eqIdx: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut graphInfoIn: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut graphInfoOut: GraphML::GraphInfo;
    let mut varLst: Arc<metamodelica::List<i32>>;
    varLst = metamodelica::arrayGet(m.clone(), eqIdx)?;
    graphInfoOut = List::fold1(varLst, (std::sync::Arc::new(addEdgeToGraph2) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, GraphML::GraphInfo) -> Result<GraphML::GraphInfo> + 'static>), eqIdx, graphInfoIn)?;
    Ok(graphInfoOut)
}

fn addEdgeToGraph2(mut varIdxIn: i32, mut eqIdx: i32, mut graphInfoIn: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut graphInfoOut: GraphML::GraphInfo;
    let mut varIdx: i32;
    let mut eqNodeId: ArcStr;
    let mut varNodeId: ArcStr;
    let mut lt: GraphML::LineType;
    if varIdxIn <= 0 {
        lt = openmodelica_codegen_graphml::GraphML::LineType::DASHED;
    } else {
        lt = openmodelica_codegen_graphml::GraphML::LineType::LINE;
    }
    varIdx = intAbs(varIdxIn);
    eqNodeId = (getEqNodeIdx(eqIdx)).clone();
    varNodeId = (getVarNodeIdx(varIdx)).clone();
    (graphInfoOut, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Edge_")); __mm_s.push_str(&*intString(varIdx)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(eqIdx)); ArcStr::from(__mm_s) }).clone(), (varNodeId).clone(), (eqNodeId).clone(), (arcstr::literal!(GraphML::COLOR_BLACK)).clone(), lt, GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE, openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE), metamodelica::nil(), graphInfoIn)?;
    Ok(graphInfoOut)
}

fn getVarNodeIdx(mut idx: i32) -> ArcStr {
    let mut varString: ArcStr;
    varString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("varNode")); __mm_s.push_str(&*intString(intAbs(idx))); ArcStr::from(__mm_s) }).clone();
    varString
}

fn getEqNodeIdx(mut idx: i32) -> ArcStr {
    let mut eqString: ArcStr;
    eqString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("eqNode")); __mm_s.push_str(&*intString(intAbs(idx))); ArcStr::from(__mm_s) }).clone();
    eqString
}

pub fn dumpBackendDAEBipartiteGraph(mut dae: Arc<BackendDAE::BackendDAE>, mut filename: ArcStr) -> Result<()> {
    let mut graphIdx: i32;
    let mut sysIdx: i32;
    let mut varIdx: i32 = 0;
    let mut eqIdx: i32 = 0;
    let mut order: i32;
    let mut nameAttIdx: i32;
    let mut varAttIdx: i32;
    let mut eqAttIdx: i32;
    let mut sysAttIdx: i32;
    let mut tearAttIdx: i32;
    let mut compAttIdx: i32;
    let mut orderAttIdx: i32;
    let mut tearInfo: ArcStr;
    let mut nodeColor: ArcStr;
    let mut graphInfo: GraphML::GraphInfo;
    let mut shapeType: GraphML::ShapeType;
    let mut lineType: GraphML::LineType;
    let mut lineWidth: metamodelica::Real;
    let mut borderWidth: metamodelica::Real;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut eqIdxs: Arc<metamodelica::List<i32>>;
    let mut varIdxs: Arc<metamodelica::List<i32>>;
    let mut vars: BackendDAE::Variables;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut ass2: metamodelica::Array<i32>;
    graphInfo = GraphML::createGraphInfo();
    let (__pa0, (_, __pa1)) = GraphML::addGraph((literal!("TaskGraph")).clone(), true, graphInfo)?;
    graphInfo = __pa0.clone();
    graphIdx = __pa1.clone();
    let (__pa2, (_, __pa3)) = GraphML::addAttribute((literal!("")).clone(), (literal!("Name")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
    graphInfo = __pa2.clone();
    nameAttIdx = __pa3.clone();
    let (__pa4, (_, __pa5)) = GraphML::addAttribute((literal!("")).clone(), (literal!("VarIdx")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
    graphInfo = __pa4.clone();
    varAttIdx = __pa5.clone();
    let (__pa6, (_, __pa7)) = GraphML::addAttribute((literal!("")).clone(), (literal!("EqIdx")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
    graphInfo = __pa6.clone();
    eqAttIdx = __pa7.clone();
    let (__pa8, (_, __pa9)) = GraphML::addAttribute((literal!("")).clone(), (literal!("SysIdx")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
    graphInfo = __pa8.clone();
    sysAttIdx = __pa9.clone();
    let (__pa10, (_, __pa11)) = GraphML::addAttribute((literal!("")).clone(), (literal!("Tearing")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
    graphInfo = __pa10.clone();
    tearAttIdx = __pa11.clone();
    let (__pa12, (_, __pa13)) = GraphML::addAttribute((literal!("")).clone(), (literal!("SCC")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
    graphInfo = __pa12.clone();
    compAttIdx = __pa13.clone();
    let (__pa14, (_, __pa15)) = GraphML::addAttribute((literal!("")).clone(), (literal!("executionOrder")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
    graphInfo = __pa14.clone();
    orderAttIdx = __pa15.clone();
    let (__pa16, __pa17) = ::match_deref::match_deref! { match &(dae) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa16, shared: __pa17 } => (__pa16.clone(), __pa17.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa16.clone();
    shared = __pa17.clone();
    sysIdx = 1;
    for mut sys in &*systs {
        let mut sys = sys.clone();
        let (__pa18, __pa19, __pa20, __pa21) = ::match_deref::match_deref! { match &(sys.clone()) {
            Deref @ BackendDAE::EqSystem { orderedVars: __pa18, orderedEqs: __pa19, matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa20, ass2: __pa21, .. }, .. } => (__pa18.clone(), __pa19.clone(), __pa20.clone(), __pa21.clone()),
            _ => bail!("pattern mismatch"),
        } };
        vars = __pa18.clone();
        eqs = __pa19.clone();
        comps = __pa20.clone();
        ass2 = __pa21.clone();
        (m, mT) = BackendDAEUtil::adjacencyMatrix(sys.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(BackendDAEUtil::getFunctions(shared.clone())?), BackendDAEUtil::isInitializationDAE(shared.clone()))?;
        order = 1;
        for mut comp in &*comps.clone() {
            let mut comp = comp.clone();
            (varLst, varIdxs, eqLst, eqIdxs) = BackendDAEUtil::getStrongComponentsVarsAndEquations(list![comp.clone()], vars.clone(), eqs.clone())?;
            for mut varIdx in &*varIdxs.clone() {
                let mut varIdx = varIdx.clone();
                nodeColor = (if (isAlgLoop(comp.clone())) {arcstr::literal!(GraphML::COLOR_RED2)} else {arcstr::literal!(GraphML::COLOR_GREEN2)}).clone();
                borderWidth = if (BackendVariable::isStateVar(BackendVariable::getVarAt(vars.clone(), varIdx)?)) {GraphML::BORDERWIDTH_BOLD.clone()} else {GraphML::BORDERWIDTH_STANDARD.clone()};
                if isTearingVar(varIdx, comp.clone())? {
                    shapeType = openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE;
                    tearInfo = (literal!("TearingVar")).clone();
                    nodeColor = (arcstr::literal!(GraphML::COLOR_RED)).clone();
                } else {
                    shapeType = openmodelica_codegen_graphml::GraphML::ShapeType::ELLIPSE;
                    tearInfo = (literal!("AlgebraicVar")).clone();
                }
                (graphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("V_")); __mm_s.push_str(&*intString(sysIdx)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(varIdx)); ArcStr::from(__mm_s) }).clone(), (nodeColor.clone()).clone(), borderWidth, list![GraphML::NodeLabel::NODELABEL_INTERNAL { text: (intString(varIdx)).clone(), backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN }], shapeType.clone(), Some((varString(BackendVariable::getVarAt(vars.clone(), varIdx)?)?).clone()), list![(nameAttIdx, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("V_")); __mm_s.push_str(&*intString(sysIdx)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(varIdx)); ArcStr::from(__mm_s) }), (varAttIdx, intString(varIdx)), (eqAttIdx, literal!("-")), (compAttIdx, printComponent(comp.clone(), None)?), (sysAttIdx, intString(sysIdx)), (tearAttIdx, tearInfo.clone()), (orderAttIdx, intString(order))], graphIdx, graphInfo.clone())?;
            }
            for mut eqIdx in &*eqIdxs.clone() {
                let mut eqIdx = eqIdx.clone();
                nodeColor = (if (isAlgLoop(comp.clone())) {arcstr::literal!(GraphML::COLOR_RED2)} else {arcstr::literal!(GraphML::COLOR_GREEN2)}).clone();
                if isResidualEq(eqIdx, comp.clone())? {
                    shapeType = openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE;
                    tearInfo = (literal!("ResidualEq")).clone();
                    nodeColor = (arcstr::literal!(GraphML::COLOR_RED)).clone();
                } else {
                    shapeType = openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE;
                    tearInfo = (literal!("AlgebraicEq")).clone();
                }
                (graphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("E_")); __mm_s.push_str(&*intString(sysIdx)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(eqIdx)); ArcStr::from(__mm_s) }).clone(), (nodeColor.clone()).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![GraphML::NodeLabel::NODELABEL_INTERNAL { text: (intString(eqIdx)).clone(), backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN }], shapeType.clone(), Some((equationString(BackendEquation::get(eqs.clone(), eqIdx)?)?).clone()), list![(nameAttIdx, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("E_")); __mm_s.push_str(&*intString(sysIdx)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(eqIdx)); ArcStr::from(__mm_s) }), (varAttIdx, literal!("-")), (compAttIdx, printComponent(comp.clone(), None)?), (eqAttIdx, intString(eqIdx)), (sysAttIdx, intString(sysIdx)), (tearAttIdx, tearInfo.clone()), (orderAttIdx, intString(order))], graphIdx, graphInfo.clone())?;
            }
            order = order + 1;
        }
        for mut eqIdx in 1..=metamodelica::arrayLength(m.clone()) {
            for mut varIdx in &*metamodelica::arrayGet(m.clone(), eqIdx)? {
                let mut varIdx = varIdx.clone();
                if intLe(varIdx, 0) {
                    lineType = openmodelica_codegen_graphml::GraphML::LineType::DASHED;
                } else {
                    lineType = openmodelica_codegen_graphml::GraphML::LineType::LINE;
                }
                varIdx = intAbs(varIdx);
                lineWidth = if (intEq(varIdx, ({let __elt = ass2.borrow()[(eqIdx-1) as usize].clone(); __elt}))) {GraphML::LINEWIDTH_BOLD.clone()} else {GraphML::LINEWIDTH_STANDARD.clone()};
                (graphInfo, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Edge_")); __mm_s.push_str(&*intString(sysIdx)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(eqIdx)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(varIdx)); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("V_")); __mm_s.push_str(&*intString(sysIdx)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(varIdx)); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("E_")); __mm_s.push_str(&*intString(sysIdx)); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(eqIdx)); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_BLACK)).clone(), lineType.clone(), lineWidth, false, metamodelica::nil(), (openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE, openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE), metamodelica::nil(), graphInfo.clone())?;
            }
        }
        sysIdx = sysIdx + 1;
    }
    GraphML::dumpGraph(graphInfo, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*filename); __mm_s.push_str(&*literal!(".graphml")); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

fn isTearingVar(mut varIdx: i32, mut comp: Arc<BackendDAE::StrongComponent>) -> Result<bool> {
    let mut isTear: bool;
    isTear = (::match_deref::match_deref! { match &(comp) {
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: tVars, .. }, .. } => {
            List::exist1(tVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), varIdx)?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isTear)
}

fn isAlgLoop(mut comp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut isLoop: bool;
    isLoop = (::match_deref::match_deref! { match &(comp) {
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: _, .. } => true,
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: _, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isLoop
}

fn isResidualEq(mut eqIdx: i32, mut comp: Arc<BackendDAE::StrongComponent>) -> Result<bool> {
    let mut isRes: bool;
    isRes = (::match_deref::match_deref! { match &(comp) {
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: resEqs, .. }, .. } => {
            List::exist1(resEqs.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), eqIdx)?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isRes)
}

pub(crate) fn SSSHandlerArgString(mut arg: Option<(BackendDAE::StateOrder, metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)>) -> Result<()> {
    let mut stateorder: BackendDAE::StateOrder;
    let mut constraints: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>;
    let mut eqs2EqIdxs: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut eqIdx2Eq: metamodelica::Array<i32>;
    let mut numEqs: i32;
    if isSome(arg.clone()) {
        let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(arg) {
            Some((__pa0, __pa1, __pa2, __pa3, __pa4)) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        stateorder = __pa0.clone();
        constraints = __pa1.clone();
        eqs2EqIdxs = __pa2.clone();
        eqIdx2Eq = __pa3.clone();
        numEqs = __pa4.clone();
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(numEqs)); __mm_s.push_str(&*literal!("eqs before IR\n")); ArcStr::from(__mm_s) }).clone());
        dumpStateOrder(stateorder)?;
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Constraints:\n")); __mm_s.push_str(&*constraintEquationString(constraints.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    } else {
        metamodelica::print((literal!("Empty StructurallySingularSystemHandlerArg\n")).clone());
    }
    Ok(())
}

pub(crate) fn constraintEquationString(mut constraints: metamodelica::Array<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>>) -> Result<ArcStr> {
    let mut s: ArcStr = literal!("");
    let mut i: i32 = 0;
    let mut s1: ArcStr;
    for mut i in 1..=metamodelica::arrayLength(constraints.clone()) {
        s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(metamodelica::arrayGet(constraints.clone(), i)?, (std::sync::Arc::new(equationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>))?, (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n------------------\n")); ArcStr::from(__mm_s) }).clone();
        if metamodelica::arrayGet(constraints.clone(), i)?.is_empty() {
            s1 = (literal!("empty Constraints\n")).clone();
        }
        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("eq ")); __mm_s.push_str(&*intString(i)); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone();
    }
    Ok(s)
}

pub(crate) fn dumpStateOrder(mut inStateOrder: BackendDAE::StateOrder) -> Result<()> {
    let () = (match inStateOrder {
        BackendDAE::StateOrder::STATEORDER { hashTable: mut ht, invHashTable: _ } => {
            let mut r#str: ArcStr;
            let mut len_str: ArcStr;
            let mut len: i32;
            let mut tplLst: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)>>;
            tplLst = BaseHashTable::hashTableList(ht.clone())?;
            if !(tplLst.clone().is_empty()) {
                metamodelica::print((literal!("State Order: (")).clone());
                r#str = stringDelimitList(List::map(tplLst.clone(), (std::sync::Arc::new(printStateOrderStr) as std::sync::Arc<dyn ::std::ops::Fn((Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)) -> Result<ArcStr> + 'static>))?, (literal!("\n")).clone());
                len = (tplLst.clone().len() as i32);
                len_str = (intString(len.clone())).clone();
                metamodelica::print((len_str.clone()).clone());
                metamodelica::print((literal!(")\n")).clone());
                metamodelica::print((literal!("=============\n")).clone());
                metamodelica::print((r#str.clone()).clone());
                metamodelica::print((literal!("\n\n")).clone());
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printStateOrderStr(mut tpl: (Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>)) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(Util::tuple21(tpl.clone()))?); __mm_s.push_str(&*literal!(" ---d/dt---> ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(Util::tuple22(tpl))?); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

pub(crate) fn dumpBackendDAEModeData(mut inDAEmodeData: BackendDAE::BackendDAEModeData) -> Result<()> {
    let mut modelVars: BackendDAE::Variables;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*arcstr::literal!(BORDER)); __mm_s.push_str(&*literal!("\nDAEMode\n")); __mm_s.push_str(&*arcstr::literal!(UNDERLINE)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    if isSome(inDAEmodeData.modelVars.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(inDAEmodeData.modelVars.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        modelVars = __pa0.clone();
        dumpVariables(modelVars, (literal!("ModelVariables")).clone())?;
    } else {
        metamodelica::print((literal!("No ModelVariables\n")).clone());
    }
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("DAEmode System:\n ")); __mm_s.push_str(&*intString(inDAEmodeData.numResVars.clone())); __mm_s.push_str(&*literal!(" residual variables\n ")); __mm_s.push_str(&*intString((inDAEmodeData.stateVars.clone().len() as i32))); __mm_s.push_str(&*literal!(" state variables\n ")); __mm_s.push_str(&*intString((inDAEmodeData.algStateVars.clone().len() as i32))); __mm_s.push_str(&*literal!(" algebraic state variables\n")); ArcStr::from(__mm_s) }).clone());
    dumpVarList(inDAEmodeData.stateVars.clone(), (literal!("State Variables")).clone())?;
    dumpVarList(inDAEmodeData.algStateVars.clone(), (literal!("Algebraic State Variables")).clone())?;
    Ok(())
}

