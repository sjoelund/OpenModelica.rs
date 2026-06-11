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

use crate::AvlSetInt;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::ExpressionSolve;
use crate::HpcOmTaskGraph;
use crate::Tearing;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub(crate) fn resolveLoops(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut eqSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    (eqSysts, shared, _) = List::mapFold2(inDAE.eqs.clone(), (std::sync::Arc::new(fnptr!(resolveLoops_main, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> + 'static>), inDAE.shared.clone(), 1)?;
    outDAE = Arc::new(BackendDAE::BackendDAE { eqs: eqSysts, shared: shared });
    Ok(outDAE)
}

fn resolveLoops_main(mut inEqSys: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inSysIdx: i32) -> (Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32) {
    let mut outEqSys: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outSysIdx: i32;
    (outEqSys, outSysIdx) = 'mc: {
        let __mc_input = inEqSys.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqs, .. } => {
                    let mut numSimpEqs: i32;
                    let mut numVars: i32;
                    let mut eqMapArr: metamodelica::Array<i32>;
                    let mut varMapArr: metamodelica::Array<i32>;
                    let mut nonLoopEqMark: metamodelica::Array<i32>;
                    let mut markLinEqVars: metamodelica::Array<i32>;
                    let mut eqMapping: Arc<metamodelica::List<i32>>;
                    let mut partitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut varAtts: Arc<metamodelica::List<(bool, ArcStr)>>;
                    let mut eqAtts: Arc<metamodelica::List<(bool, ArcStr)>>;
                    let mut simpVars: BackendDAE::Variables;
                    let mut simpEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut m_cut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut mT_cut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut m_after: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut simpEqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut simpVarLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut syst = (*syst).clone();
                    let mut eqs = (*eqs).clone();
                    (m, _) = BackendDAEUtil::adjacencyMatrix(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
                    if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? {
                        BackendDump::dumpBipartiteGraphEqSystem(syst.clone(), inShared.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("whole System_")); __mm_s.push_str(&*intString(inSysIdx)); ArcStr::from(__mm_s) }).clone())?;
                    }
                    markLinEqVars = arrayCreate(BackendVariable::varsSize(vars.clone()), -1);
                    (simpEqLst, eqMapping, _, _, markLinEqVars, _) = BackendEquation::traverseEquationArray(eqs.clone(), (std::sync::Arc::new(getSimpleEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, i32, BackendDAE::Variables, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, i32, BackendDAE::Variables, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>))> + 'static>), (metamodelica::nil(), metamodelica::nil(), 1, vars.clone(), markLinEqVars.clone(), m.clone()))?;
                    eqMapArr = metamodelica::arrayFromVec(eqMapping.clone().into_iter().cloned().collect());
                    (simpVarLst, varMapArr) = getSimpleEquationVariables(markLinEqVars.clone(), vars.clone())?;
                    simpEqs = BackendEquation::listEquation(simpEqLst.clone())?;
                    simpVars = BackendVariable::listVar1(simpVarLst.clone())?;
                    numSimpEqs = (simpEqLst.clone().len() as i32);
                    numVars = (simpVarLst.clone().len() as i32);
                    (m, mT) = BackendDAEUtil::adjacencyMatrixDispatch(simpVars.clone(), simpEqs.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
                    if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? {
                        varAtts = List::threadMap(List::fill(false, numVars.clone()), List::fill((literal!("")).clone(), numVars.clone()), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
                        eqAtts = List::threadMap(List::fill(false, numSimpEqs.clone()), List::fill((literal!("")).clone(), numSimpEqs.clone()), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
                        BackendDump::dumpBipartiteGraphStrongComponent2(simpVars.clone(), simpEqs.clone(), m.clone(), varAtts.clone(), eqAtts.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rL_simpEqs_")); __mm_s.push_str(&*intString(inSysIdx)); ArcStr::from(__mm_s) }).clone())?;
                    }
                    partitions = partitionBipartiteGraph(m.clone(), mT.clone())?;
                    partitions = List::filterOnTrue(partitions.clone(), std::sync::Arc::new(fnptr!(List::hasSeveralElements, _)))?;
                    m_cut = metamodelica::arrayFromVec(m.clone().borrow().clone());
                    mT_cut = metamodelica::arrayFromVec(mT.clone().borrow().clone());
                    (_, nonLoopEqMark) = resolveLoops_cutNodes(m_cut.clone(), mT_cut.clone())?;
                    if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? {
                        varAtts = List::threadMap(List::fill(false, numVars.clone()), List::fill((literal!("")).clone(), numVars.clone()), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
                        eqAtts = List::threadMap(List::fill(false, numSimpEqs.clone()), List::fill((literal!("")).clone(), numSimpEqs.clone()), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
                        BackendDump::dumpBipartiteGraphStrongComponent2(simpVars.clone(), simpEqs.clone(), m_cut.clone(), varAtts.clone(), eqAtts.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rL_loops_")); __mm_s.push_str(&*intString(inSysIdx)); ArcStr::from(__mm_s) }).clone())?;
                    }
                    eqs = resolveLoops_resolvePartitions(partitions.clone(), m_cut.clone(), mT_cut.clone(), m.clone(), mT.clone(), eqMapArr.clone(), varMapArr.clone(), eqs.clone(), vars.clone(), nonLoopEqMark.clone())?;
                    assign_field!(syst.orderedEqs = eqs.clone());
                    if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? {
                        simpEqLst = BackendEquation::getList(eqMapping.clone(), eqs.clone())?;
                        simpEqs = BackendEquation::listEquation(simpEqLst.clone())?;
                        numSimpEqs = (simpEqLst.clone().len() as i32);
                        numVars = (simpVarLst.clone().len() as i32);
                        (m_after, _) = BackendDAEUtil::adjacencyMatrixDispatch(simpVars.clone(), simpEqs.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
                        varAtts = List::threadMap(List::fill(false, numVars.clone()), List::fill((literal!("")).clone(), numVars.clone()), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
                        eqAtts = List::threadMap(List::fill(false, numSimpEqs.clone()), List::fill((literal!("")).clone(), numSimpEqs.clone()), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
                        BackendDump::dumpBipartiteGraphStrongComponent2(simpVars.clone(), simpEqs.clone(), m_after.clone(), varAtts.clone(), eqAtts.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rL_after_")); __mm_s.push_str(&*intString(inSysIdx)); ArcStr::from(__mm_s) }).clone())?;
                    }
                    syst = BackendDAEUtil::clearEqSyst(syst.clone())?;
                    Ok((syst.clone(), inSysIdx + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inEqSys.clone(), inSysIdx + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outEqSys, outShared, outSysIdx)
}

fn resolveLoops_resolvePartitions(mut partitionsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m_uncut: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT_uncut: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqMap: metamodelica::Array<i32>, mut varMap: metamodelica::Array<i32>, mut daeEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut daeVars: BackendDAE::Variables, mut nonLoopEqMark: metamodelica::Array<i32>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut daeEqsOut: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    daeEqsOut = (::match_deref::match_deref! { match &(partitionsIn) {
        Deref @ metamodelica::List::Cons { head: partition, tail: rest } => {
            let mut optStructureMapping: Option<(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)>;
            let mut eqCrossLst: Arc<metamodelica::List<i32>>;
            let mut varCrossLst: Arc<metamodelica::List<i32>>;
            let mut mapIndices: Arc<metamodelica::List<i32>>;
            let mut loops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut map: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut partition = (*partition).clone();
            partition = List::filter1OnTrue(partition.clone(), (std::sync::Arc::new(fnptr!(arrayIsZeroAt, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), nonLoopEqMark.clone())?;
            if partition.clone().is_empty() {
                eqs = resolveLoops_resolvePartitions(rest.clone(), mIn.clone(), mTIn.clone(), m_uncut.clone(), mT_uncut.clone(), eqMap.clone(), varMap.clone(), daeEqs, daeVars, nonLoopEqMark.clone())?;
            } else {
                (loops, eqCrossLst, varCrossLst, optStructureMapping) = resolveLoops_findLoops(list![partition.clone()], mIn.clone(), mTIn.clone(), false);
                if isSome(optStructureMapping.clone()) {
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(optStructureMapping) {
                        Some((__pa0, __pa1, __pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    mapIndices = __pa0.clone();
                    map = __pa1.clone();
                    loops = __pa2.clone();
                    loops = List::filter1OnTrueAndUpdate(loops, (std::sync::Arc::new(evaluateTripleLoop) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<bool> + 'static>), (std::sync::Arc::new(updateTripleLoop) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<Arc<metamodelica::List<i32>>> + 'static>), (m_uncut.clone(), mapIndices, map.clone()))?;
                } else {
                    loops = List::filterOnFalse(loops, std::sync::Arc::new(fnptr!(listEmpty, _)))?;
                    loops = List::filter1OnTrue(loops, (std::sync::Arc::new(evaluateLoop) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>)) -> Result<bool> + 'static>), (m_uncut.clone(), mT_uncut.clone(), eqCrossLst.clone()))?;
                }
                (eqs, _) = resolveLoops_resolveAndReplace(loops, eqCrossLst, varCrossLst, mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqs, daeVars.clone(), metamodelica::nil())?;
                eqs = resolveLoops_resolvePartitions(rest.clone(), mIn.clone(), mTIn.clone(), m_uncut.clone(), mT_uncut.clone(), eqMap.clone(), varMap.clone(), eqs, daeVars, nonLoopEqMark.clone())?;
            }
            eqs
        },
        Deref @ metamodelica::List::Nil => {
            daeEqs
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(daeEqsOut)
}

fn resolveLoops_cutNodes(mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut deadEndVarsMark: metamodelica::Array<i32> = Default::default();
    let mut deadEndEqsMark: metamodelica::Array<i32> = Default::default();
    (deadEndVarsMark, deadEndEqsMark) = 'mc: {
        let __mc_input = mTIn.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut numVars: i32;
            let mut numEqs: i32;
            let mut idx: i32 = 0;
            let mut loopVars: Arc<metamodelica::List<i32>>;
            let mut loopEqs: Arc<metamodelica::List<i32>>;
            let mut nonLoopVars: Arc<metamodelica::List<i32>>;
            let mut deadEndEqsMark: metamodelica::Array<i32> = deadEndEqsMark.clone();
            let mut deadEndVarsMark: metamodelica::Array<i32> = deadEndVarsMark.clone();
            numVars = metamodelica::arrayLength(mTIn.clone());
            numEqs = metamodelica::arrayLength(mIn.clone());
            nonLoopVars = List::filter2OnTrue(List::intRange(numVars.clone()), (std::sync::Arc::new(arrayEntryLengthIs) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, i32) -> Result<bool> + 'static>), mTIn.clone(), 1)?;
            deadEndVarsMark = arrayCreate(numVars.clone(), 0);
            deadEndEqsMark = arrayCreate(numVars.clone(), 0);
            for mut idx in &*nonLoopVars.clone() {
                let mut idx = idx.clone();
                metamodelica::arrayUpdate(deadEndVarsMark.clone(), idx.clone(), 1)?;
            }
            for mut idx in &*nonLoopVars.clone() {
                let mut idx = idx.clone();
                markDeadEndsInBipartiteGraph(idx.clone(), mIn.clone(), mTIn.clone(), deadEndEqsMark.clone(), deadEndVarsMark.clone())?;
            }
            idx = 1;
            while idx.clone() <= numVars.clone() {
                if metamodelica::arrayGet(deadEndVarsMark.clone(), idx.clone())? == 1 {
                    metamodelica::arrayUpdate(mTIn.clone(), idx.clone(), metamodelica::nil())?;
                } else {
                    loopEqs = metamodelica::arrayGet(mTIn.clone(), idx.clone())?;
                    loopEqs = List::filter1OnTrue(loopEqs.clone(), (std::sync::Arc::new(fnptr!(arrayIsZeroAt, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), deadEndEqsMark.clone())?;
                    metamodelica::arrayUpdate(mTIn.clone(), idx.clone(), loopEqs.clone())?;
                }
                idx = idx.clone() + 1;
            }
            idx = 1;
            while idx.clone() <= numEqs.clone() {
                if metamodelica::arrayGet(deadEndEqsMark.clone(), idx.clone())? == 1 {
                    metamodelica::arrayUpdate(mIn.clone(), idx.clone(), metamodelica::nil())?;
                } else {
                    loopVars = metamodelica::arrayGet(mIn.clone(), idx.clone())?;
                    loopVars = List::filter1OnTrue(loopVars.clone(), (std::sync::Arc::new(fnptr!(arrayIsZeroAt, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), deadEndVarsMark.clone())?;
                    metamodelica::arrayUpdate(mIn.clone(), idx.clone(), loopVars.clone())?;
                }
                idx = idx.clone() + 1;
            }
            Ok(((deadEndVarsMark.clone(), deadEndEqsMark.clone()), deadEndEqsMark.clone(), deadEndVarsMark.clone()))
        })() { deadEndEqsMark = __wb0; deadEndVarsMark = __wb1; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addInternalError((literal!("function resolveLoops_cutNodes failed")).clone(), metamodelica::sourceInfo!("BackEnd/ResolveLoops.mo"))?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((deadEndVarsMark, deadEndEqsMark))
}

fn arrayEntryLengthIs(mut idx: i32, mut arr: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut len: i32) -> Result<bool> {
    let mut eqLen: bool;
    let mut entry: Arc<metamodelica::List<i32>>;
    let mut len1: i32;
    entry = metamodelica::arrayGet(arr.clone(), idx)?;
    len1 = (entry.len() as i32);
    eqLen = intEq(len, len1);
    Ok(eqLen)
}

fn getSimpleEquations(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, i32, BackendDAE::Variables, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, i32, BackendDAE::Variables, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>))> {
    let mut outEq: Arc<BackendDAE::Equation> = inEq.clone();
    let mut outTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, i32, BackendDAE::Variables, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>);
    let mut isSimple: bool;
    let mut idx: i32;
    let mut eq: Arc<BackendDAE::Equation>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut vars: BackendDAE::Variables;
    let mut markLinEqVars: metamodelica::Array<i32>;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut idxMap: Arc<metamodelica::List<i32>>;
    (eqLst, idxMap, idx, vars, markLinEqVars, m) = inTpl;
    if BackendEquation::isEquation(inEq.clone()) && !(eqIsConst(inEq.clone())) {
        let (__pa0, (__pa1, _)) = BackendEquation::traverseExpsOfEquation(inEq, (std::sync::Arc::new(isAddOrSubExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (bool, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (bool, BackendDAE::Variables))> + 'static>), (true, vars.clone()))?;
        eq = __pa0.clone();
        isSimple = __pa1.clone();
        if isSimple {
            eqLst = metamodelica::cons(eq, eqLst);
            idxMap = metamodelica::cons(idx, idxMap);
            let __range2 = &*({let __elt = m.borrow()[(idx-1) as usize].clone(); __elt});
            for mut varIdx in __range2 {
                let mut varIdx = varIdx.clone();
                metamodelica::arrayUpdate(markLinEqVars.clone(), intAbs(varIdx.clone()), 1)?;
            }
        }
    }
    outTpl = (eqLst, idxMap, idx + 1, vars, markLinEqVars.clone(), m.clone());
    Ok((outEq, outTpl))
}

fn getSimpleEquationVariables(mut markLinEqVars: metamodelica::Array<i32>, mut vars: BackendDAE::Variables) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<i32>)> {
    let mut simpVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut varMapArr: metamodelica::Array<i32>;
    let mut varIdx: i32 = 0;
    let mut varMap: Arc<metamodelica::List<i32>>;
    varMap = metamodelica::nil();
    for mut varIdx in 1..=metamodelica::arrayLength(markLinEqVars.clone()) {
        if ({let __elt = markLinEqVars.borrow()[(varIdx-1) as usize].clone(); __elt}) > 0 {
            varMap = metamodelica::cons(varIdx, varMap.clone());
            simpVars = metamodelica::cons(BackendVariable::getVarAt(vars.clone(), varIdx)?, simpVars.clone());
        }
    }
    varMapArr = metamodelica::arrayFromVec(varMap.into_iter().cloned().collect());
    Ok((simpVars, varMapArr))
}

pub(crate) fn resolveLoops_findLoops(mut partitionsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut findExactlyOneLoop: bool) -> (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Option<(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)>) {
    let mut loopsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut crossEqsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut crossVarsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut optStructureMapping: Option<(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> = None;
    let mut loops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut eqVars: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut eqCrossLst: Arc<metamodelica::List<i32>>;
    let mut varCrossLst: Arc<metamodelica::List<i32>>;
    let mut partitionVars: Arc<metamodelica::List<i32>>;
    let mut set: Arc<AvlSetInt::Tree>;
    for mut partition in &*partitionsIn {
        let mut partition = partition.clone();
        match '__try0: {
            eqVars = unwrap_break_err!(List::map1(partition.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), mIn.clone()), '__try0);
            set = crate::AvlSetInt::Tree::interned_EMPTY();
            for mut vars in &*eqVars.clone() {
                let mut vars = vars.clone();
                set = unwrap_break_err!(AvlSetInt::addList(set.clone(), vars.clone()), '__try0);
            }
            partitionVars = AvlSetInt::listKeys(set.clone(), metamodelica::nil());
            eqCrossLst = unwrap_break_err!(List::fold2(partition.clone(), (std::sync::Arc::new(gatherCrossNodes) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), mIn.clone(), mTIn.clone(), metamodelica::nil()), '__try0);
            varCrossLst = unwrap_break_err!(List::fold2(partitionVars.clone(), (std::sync::Arc::new(gatherCrossNodes) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), mTIn.clone(), mIn.clone(), metamodelica::nil()), '__try0);
            (loops, optStructureMapping) = unwrap_break_err!(resolveLoops_findLoops2(partition.clone(), eqCrossLst.clone(), varCrossLst.clone(), mIn.clone(), mTIn.clone(), findExactlyOneLoop), '__try0);
            if if (findExactlyOneLoop) {!(loops.clone().is_empty()) && !(loopsOut.clone().is_empty())} else {false} {
                break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
            }
            loopsOut = listAppend(loops.clone(), loopsOut.clone());
            if true /* isPresent not implemented in Rust */ {
                crossEqsOut = listAppend(eqCrossLst.clone(), crossEqsOut.clone());
            }
            if true /* isPresent not implemented in Rust */ {
                crossVarsOut = listAppend(varCrossLst.clone(), crossVarsOut.clone());
            }
            Ok::<_, anyhow::Error>((eqCrossLst.clone(), eqVars.clone(), loops.clone(), loopsOut.clone(), optStructureMapping.clone(), partitionVars.clone(), set.clone(), varCrossLst.clone()))
        } {
            Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7)) => {
                eqCrossLst = __try0_o0;
                eqVars = __try0_o1;
                loops = __try0_o2;
                loopsOut = __try0_o3;
                optStructureMapping = __try0_o4;
                partitionVars = __try0_o5;
                set = __try0_o6;
                varCrossLst = __try0_o7;
            }
            Err(_) => {
                return (loopsOut.clone(), crossEqsOut.clone(), crossVarsOut.clone(), optStructureMapping.clone());
            }
        }
    }
    (loopsOut, crossEqsOut, crossVarsOut, optStructureMapping)
}

fn resolveLoops_findLoops2(mut eqsIn: Arc<metamodelica::List<i32>>, mut eqCrossLstIn: Arc<metamodelica::List<i32>>, mut varCrossLstIn: Arc<metamodelica::List<i32>>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut findExactlyOneLoop: bool) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Option<(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)>)> {
    let mut loopsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut structureMapping: Option<(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)>;
    (loopsOut, structureMapping) = (::match_deref::match_deref! { match &((eqCrossLstIn.clone(), varCrossLstIn.clone())) {
        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Nil) => {
            let mut isNoSingleLoop: bool;
            let mut eqCrossLst: Arc<metamodelica::List<i32>>;
            let mut subLoop: Arc<metamodelica::List<i32>>;
            let mut mapIndices: Arc<metamodelica::List<i32>>;
            let mut paths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut allPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut simpleLoops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut tripleLoops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut paths0: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut paths1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut loopConnectors: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut connectedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut minAdj: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut map: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut mapping: (Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>);
            let mut optTripleMapping: Option<(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)>;
            allPaths = getPathTillNextCrossEq(eqCrossLstIn.clone(), mIn.clone(), mTIn.clone(), eqCrossLstIn.clone(), metamodelica::nil(), metamodelica::nil())?;
            allPaths = List::sort(allPaths, (std::sync::Arc::new(List::listIsLonger) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
            paths1 = List::fold1(allPaths.clone(), (std::sync::Arc::new(fnptr!(getReverseDoubles, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> + 'static>), allPaths, metamodelica::nil())?;
            simpleLoops = getDoubles(paths1.clone(), metamodelica::nil());
            (_, paths, _) = List::intersection1OnTrue(paths1.clone(), simpleLoops.clone(), (std::sync::Arc::new(intLstIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>))?;
            if simpleLoops.clone().is_empty() {
                (eqCrossLst, paths1, mapping, minAdj) = findEqualPathStructure(eqCrossLstIn, paths1)?;
                (mapIndices, map) = mapping;
                (tripleLoops, paths0) = getTriples(eqCrossLst, minAdj.clone())?;
                optTripleMapping = Some((mapIndices, map.clone(), tripleLoops));
            } else {
                optTripleMapping = None;
                paths0 = List::sort(paths, (std::sync::Arc::new(List::listIsLonger) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
                (connectedPaths, loopConnectors) = connect2PathsToLoops(paths0, metamodelica::nil(), metamodelica::nil())?;
                loopConnectors = List::filter1OnTrue(loopConnectors, (std::sync::Arc::new(connectsLoops) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<bool> + 'static>), simpleLoops.clone())?;
                simpleLoops = listAppend(simpleLoops, loopConnectors);
                subLoop = connectPathsToOneLoop(simpleLoops.clone(), metamodelica::nil());
                isNoSingleLoop = subLoop.clone().is_empty();
                simpleLoops = if (isNoSingleLoop) {simpleLoops} else {list![subLoop]};
                paths0 = listAppend(simpleLoops, connectedPaths);
                paths0 = sortPathsAsChain(paths0);
                if findExactlyOneLoop {
                    if !(paths0.clone().is_empty()) {
                        ::match_deref::match_deref! { match &(paths0.clone()) {
                            Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => (),
                            _ => bail!("pattern mismatch"),
                        } };
                    }
                }
            }
            (paths0, optTripleMapping)
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
            let mut paths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut paths0: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut paths1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut closedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            paths = getPathTillNextCrossEq(varCrossLstIn.clone(), mTIn.clone(), mIn.clone(), varCrossLstIn, metamodelica::nil(), metamodelica::nil())?;
            paths = List::sort(paths, (std::sync::Arc::new(List::listIsLonger) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
            paths = paths.reverse();
            (paths0, paths1) = List::extract1OnTrue(paths.clone(), (std::sync::Arc::new(fnptr!(listLengthIs, Arc<metamodelica::List<i32>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), (List::last(paths)?.len() as i32))?;
            paths1 = if (paths1.clone().is_empty()) {paths0.clone()} else {paths1};
            closedPaths = List::map1(paths1, (std::sync::Arc::new(closePathDirectly) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), paths0)?;
            closedPaths = List::fold1(closedPaths.clone(), (std::sync::Arc::new(fnptr!(getReverseDoubles, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> + 'static>), closedPaths, metamodelica::nil())?;
            closedPaths = List::map(closedPaths, std::sync::Arc::new(fnptr!(List::unique, _)))?;
            closedPaths = List::map1(closedPaths, (std::sync::Arc::new(getEqNodesForVarLoop) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), mTIn.clone())?;
            if findExactlyOneLoop {
                if !(closedPaths.clone().is_empty()) {
                    ::match_deref::match_deref! { match &(closedPaths.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                }
            }
            (closedPaths, None)
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            let mut subLoop: Arc<metamodelica::List<i32>>;
            subLoop = eqsIn.clone();
            for mut e in &*eqsIn {
                let mut e = e.clone();
                if ({let __elt = mIn.borrow()[(e.clone()-1) as usize].clone(); __elt}).is_empty() {
                    subLoop = metamodelica::nil();
                    break;
                }
            }
            (list![subLoop], None)
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
            let mut paths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut eqCrossSet: Arc<AvlSetInt::Tree>;
            for mut i in 1..=metamodelica::arrayLength(mIn.clone()) {
                metamodelica::arrayUpdate(mIn.clone(), i.clone(), List::heapSortIntList(({let __elt = mIn.borrow()[(i.clone()-1) as usize].clone(); __elt})))?;
            }
            for mut i in 1..=metamodelica::arrayLength(mTIn.clone()) {
                metamodelica::arrayUpdate(mTIn.clone(), i.clone(), List::heapSortIntList(({let __elt = mTIn.borrow()[(i.clone()-1) as usize].clone(); __elt})))?;
            }
            eqCrossSet = AvlSetInt::addList(crate::AvlSetInt::Tree::interned_EMPTY(), eqCrossLstIn)?;
            paths = getShortPathsBetweenEqCrossNodes(AvlSetInt::listKeysReverse(eqCrossSet.clone(), metamodelica::nil()), eqCrossSet, mIn.clone(), mTIn.clone(), metamodelica::nil(), findExactlyOneLoop)?;
            (paths, None)
        },
        _ => {
            Error::addInternalError((literal!("function resolveLoops_findLoops2 failed")).clone(), metamodelica::sourceInfo!("BackEnd/ResolveLoops.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((loopsOut, structureMapping))
}

fn findEqualPathStructure(mut crossNodes: Arc<metamodelica::List<i32>>, mut uniquePaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, (Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>), metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut crossNodes: Arc<metamodelica::List<i32>> = crossNodes;
    let mut uniquePaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = uniquePaths;
    let mut mapping: (Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>);
    let mut minAdj: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapIndices: Arc<metamodelica::List<i32>>;
    let mut map: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    minAdj = getMinimalAdjacencyMatrix(crossNodes.clone(), uniquePaths.clone())?;
    (minAdj, uniquePaths, mapIndices, map, crossNodes) = removeEqualPaths(crossNodes.clone(), minAdj.clone(), uniquePaths, metamodelica::nil(), arrayCreate(({
        let mut __acc: Option<i32> = None;
        for mut cn in (crossNodes).into_iter().cloned() {
            let __x = cn.clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.unwrap_or((-i32::MAX))
    }), metamodelica::nil()), metamodelica::nil())?;
    mapping = (mapIndices, map.clone());
    Ok((crossNodes, uniquePaths, mapping, minAdj))
}

fn getMinimalAdjacencyMatrix(mut crossNodes: Arc<metamodelica::List<i32>>, mut uniquePaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut minAdj: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    minAdj = arrayCreate(({
        let mut __acc: Option<i32> = None;
        for mut cn in (crossNodes.clone()).into_iter().cloned() {
            let __x = cn.clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.unwrap_or((-i32::MAX))
    }), metamodelica::nil());
    for mut path in &*uniquePaths {
        let mut path = path.clone();
        let _ = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ metamodelica::List::Cons { head: a, tail: Deref @ metamodelica::List::Cons { head: b, tail: Deref @ metamodelica::List::Nil } } => {
            minAdj = Array::consToElement(a.clone(), b.clone(), minAdj.clone())?;
            minAdj = Array::consToElement(b.clone(), a.clone(), minAdj.clone())?;
            0
        },
        _ => {
            1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    for mut cn in &*crossNodes {
        let mut cn = cn.clone();
        metamodelica::arrayUpdate(minAdj.clone(), cn.clone(), List::sort(metamodelica::arrayGet(minAdj.clone(), cn.clone())?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?)?;
    }
    Ok(minAdj)
}

fn removeEqualPaths(mut crossNodes: Arc<metamodelica::List<i32>>, mut minAdj: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut uniquePaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut mapIndices: Arc<metamodelica::List<i32>>, mut map: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut accCrossNodes: Arc<metamodelica::List<i32>>) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>)> {
    let mut minAdj: metamodelica::Array<Arc<metamodelica::List<i32>>> = minAdj;
    let mut uniquePaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = uniquePaths;
    let mut mapIndices: Arc<metamodelica::List<i32>> = mapIndices;
    let mut map: metamodelica::Array<Arc<metamodelica::List<i32>>> = map;
    let mut accCrossNodes: Arc<metamodelica::List<i32>> = accCrossNodes;
    (minAdj, uniquePaths, mapIndices, map, accCrossNodes) = ({
        let mut assigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut unassigned: Arc<metamodelica::List<i32>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(crossNodes) {
        Deref @ metamodelica::List::Cons { head: cn1, tail: rest } => {
            if !(listMember(cn1.clone(), accCrossNodes.clone())) {
                accCrossNodes = metamodelica::cons(cn1.clone(), accCrossNodes);
            }
            for mut cn2 in &*rest.clone() {
                let mut cn2 = cn2.clone();
                if HpcOmTaskGraph::equalLists(metamodelica::arrayGet(minAdj.clone(), cn1.clone())?, metamodelica::arrayGet(minAdj.clone(), cn2.clone())?) {
                    assigned = metamodelica::cons(cn2.clone(), assigned.clone());
                    metamodelica::arrayUpdate(minAdj.clone(), cn2.clone(), metamodelica::nil())?;
                    uniquePaths = removeNode(cn2.clone(), uniquePaths.clone(), metamodelica::nil());
                } else {
                    unassigned = metamodelica::cons(cn2.clone(), unassigned.clone());
                    if !(listMember(cn2.clone(), accCrossNodes.clone())) {
                        accCrossNodes = metamodelica::cons(cn2.clone(), accCrossNodes.clone());
                    }
                }
            }
            if !(assigned.clone().is_empty()) {
                mapIndices = metamodelica::cons(cn1.clone(), mapIndices);
                map = Array::appendToElement(cn1.clone(), assigned, map.clone())?;
            }
            removeEqualPaths(unassigned, minAdj.clone(), uniquePaths, mapIndices, map.clone(), accCrossNodes)?
        },
        _ => {
            (minAdj.clone(), uniquePaths, mapIndices, map.clone(), accCrossNodes)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok((minAdj, uniquePaths, mapIndices, map, accCrossNodes))
}

fn removeNode(mut node: i32, mut inPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut accPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut accPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = accPaths;
    accPaths = (::match_deref::match_deref! { match &(inPaths) {
        Deref @ metamodelica::List::Cons { head: path, tail: rest } => {
            let mut acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            if !(pathContainsNode(node, path.clone())) {
                acc = metamodelica::cons(path.clone(), accPaths);
            } else {
                acc = accPaths;
            }
            removeNode(node, rest.clone(), acc)
        },
        Deref @ metamodelica::List::Nil => {
            accPaths
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    accPaths
}

fn pathContainsNode(mut node: i32, mut inPath: Arc<metamodelica::List<i32>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inPath) {
        Deref @ metamodelica::List::Cons { head: n, tail: _ } if (intEq(n.clone(), node)) => {
            return true
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            { (node, inPath) = (node, rest.clone()); continue '__tco; }
        },
        Deref @ metamodelica::List::Nil => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn listContains(mut lst: Arc<metamodelica::List<i32>>, mut int: i32) -> bool {
    let mut res: bool = false;
    for mut i in &*lst {
        let mut i = i.clone();
        if intEq(i.clone(), int) {
            res = true;
            return res.clone();
        }
    }
    res
}

fn hasSameIntSortedExcept(mut inList1: Arc<metamodelica::List<i32>>, mut inList2: Arc<metamodelica::List<i32>>, mut excl: i32) -> Result<bool> {
    let mut rv: bool = false;
    let mut i1: i32;
    let mut i2: i32;
    let mut l1: Arc<metamodelica::List<i32>> = inList1.clone();
    let mut l2: Arc<metamodelica::List<i32>> = inList2.clone();
    if inList1.is_empty() || inList2.is_empty() {
        return Ok(rv.clone());
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(l1) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    i1 = __pa0.clone();
    l1 = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(l2) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    i2 = __pa2.clone();
    l2 = __pa3.clone();
    loop {
        if i1 > i2 {
            if l2.clone().is_empty() {
                return Ok(rv.clone());
            }
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(l2.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                _ => bail!("pattern mismatch"),
            } };
            i2 = __pa4.clone();
            l2 = __pa5.clone();
        } else if i1 < i2 {
            if l1.clone().is_empty() {
                return Ok(rv.clone());
            }
            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(l1.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
                _ => bail!("pattern mismatch"),
            } };
            i1 = __pa6.clone();
            l1 = __pa7.clone();
        } else {
            if i1 != excl {
                rv = true;
                return Ok(rv.clone());
            }
            if l1.clone().is_empty() || l2.clone().is_empty() {
                return Ok(rv.clone());
            }
            let (__pa8, __pa9) = ::match_deref::match_deref! { match &(l1.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa8, tail: __pa9 } => (__pa8.clone(), __pa9.clone()),
                _ => bail!("pattern mismatch"),
            } };
            i1 = __pa8.clone();
            l1 = __pa9.clone();
            let (__pa10, __pa11) = ::match_deref::match_deref! { match &(l2.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa10, tail: __pa11 } => (__pa10.clone(), __pa11.clone()),
                _ => bail!("pattern mismatch"),
            } };
            i2 = __pa10.clone();
            l2 = __pa11.clone();
        }
    }
    Ok(rv)
}

fn getShortPathsBetweenEqCrossNodes(mut eqCrossLstIn: Arc<metamodelica::List<i32>>, mut eqCrossSet: Arc<AvlSetInt::Tree>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut pathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut findExactlyOneLoop: bool) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    '__tco: loop {
        ({
        let mut paths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        ::match_deref::match_deref! { match &(eqCrossLstIn) {
        Deref @ metamodelica::List::Cons { head: crossEq, tail: rest } => {
            let mut adjVar: i32 = 0;
            let mut adjEq: i32 = 0;
            let mut adjVars: Arc<metamodelica::List<i32>>;
            let mut newPath: Arc<metamodelica::List<i32>>;
            adjVars = metamodelica::arrayGet(mIn.clone(), crossEq.clone())?;
            for mut adjVar in &*adjVars.clone() {
                let mut adjVar = adjVar.clone();
                for mut adjEq in &*metamodelica::arrayGet(mTIn.clone(), adjVar)? {
                    let mut adjEq = adjEq.clone();
                    if if (adjEq > crossEq.clone()) {!(AvlSetInt::hasKey(eqCrossSet.clone(), adjEq)?)} else {true} {
                        continue;
                    }
                    if hasSameIntSortedExcept(adjVars.clone(), metamodelica::arrayGet(mIn.clone(), adjEq)?, adjVar)? {
                        newPath = metamodelica::cons(adjEq, list![crossEq.clone()]);
                        paths = List::unionElt(newPath.clone(), paths.clone());
                        if if (findExactlyOneLoop) {!(pathsIn.clone().is_empty())} else {false} {
                            bail!("fail");
                        }
                    }
                }
            }
            { (eqCrossLstIn, eqCrossSet, mIn, mTIn, pathsIn, findExactlyOneLoop) = (rest.clone(), eqCrossSet, mIn.clone(), mTIn.clone(), listAppend(paths, pathsIn), findExactlyOneLoop); continue '__tco; }
        },
        Deref @ metamodelica::List::Nil => {
            return Ok(pathsIn)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    })
    }
}

fn connectsLoops(mut path: Arc<metamodelica::List<i32>>, mut allLoops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<bool> {
    let mut connected: bool;
    let mut b1: bool;
    let mut b2: bool;
    let mut startNode: i32;
    let mut endNode: i32;
    let mut loops1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut loops2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    startNode = listHead(path.clone())?;
    endNode = List::last(path)?;
    loops1 = List::filter1OnTrue(allLoops.clone(), (std::sync::Arc::new(firstInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), startNode)?;
    loops2 = List::filter1OnTrue(allLoops.clone(), (std::sync::Arc::new(lastInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), startNode)?;
    b1 = !(loops1.is_empty()) || !(loops2.is_empty());
    loops1 = List::filter1OnTrue(allLoops.clone(), (std::sync::Arc::new(firstInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), endNode)?;
    loops2 = List::filter1OnTrue(allLoops, (std::sync::Arc::new(lastInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), endNode)?;
    b2 = !(loops1.is_empty()) || !(loops2.is_empty());
    connected = b1 && b2;
    Ok(connected)
}

fn connectPathsToOneLoop(mut allPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut loopIn: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut loopOut: Arc<metamodelica::List<i32>>;
    loopOut = 'mc: {
        let __mc_input = (allPathsIn.clone(), loopIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: startNode, tail: path }) => {
                    let mut endNode: i32;
                    endNode = List::last(path.clone())?;
                    let true = (intEq(startNode.clone(), endNode.clone())) else { bail!("pattern mismatch") };
                    Ok(path.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: startNode, tail: _ }) => {
                    let mut path: Arc<metamodelica::List<i32>>;
                    let mut nextPath: Arc<metamodelica::List<i32>>;
                    let mut rest: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut nextPaths1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut nextPaths2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    nextPaths1 = List::filter1OnTrue(allPathsIn.clone(), (std::sync::Arc::new(firstInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), startNode.clone())?;
                    nextPaths2 = List::filter1OnTrue(allPathsIn.clone(), (std::sync::Arc::new(lastInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), startNode.clone())?;
                    nextPaths2 = listAppend(nextPaths1.clone(), nextPaths2.clone());
                    nextPath = listHead(nextPaths2.clone())?;
                    (rest, _) = List::deleteMemberOnTrue(nextPath.clone(), allPathsIn.clone(), (std::sync::Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
                    (nextPath, _) = List::deleteMemberOnTrue(startNode.clone(), nextPath.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    path = listAppend(nextPath.clone(), loopIn.clone());
                    path = connectPathsToOneLoop(rest.clone(), path.clone());
                    Ok(path.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: path, tail: rest }, Deref @ metamodelica::List::Nil) => {
                    let mut startNode: i32;
                    let mut nextPath: Arc<metamodelica::List<i32>>;
                    let mut restPath: Arc<metamodelica::List<i32>>;
                    let mut nextPaths1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut nextPaths2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut path = (*path).clone();
                    let mut rest = (*rest).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(path.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    startNode = __pa0.clone();
                    restPath = __pa1.clone();
                    nextPaths1 = List::filter1OnTrue(rest.clone(), (std::sync::Arc::new(firstInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), startNode.clone())?;
                    nextPaths2 = List::filter1OnTrue(rest.clone(), (std::sync::Arc::new(lastInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), startNode.clone())?;
                    nextPaths2 = listAppend(nextPaths1.clone(), nextPaths2.clone());
                    nextPath = listHead(nextPaths2.clone())?;
                    (rest, _) = List::deleteMemberOnTrue(nextPath.clone(), rest.clone(), (std::sync::Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
                    path = listAppend(nextPath.clone(), restPath.clone());
                    path = connectPathsToOneLoop(rest.clone(), path.clone());
                    Ok(path.clone())
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
    loopOut
}

fn resolveLoops_resolveAndReplace(mut loopsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut eqCrossLstIn: Arc<metamodelica::List<i32>>, mut varCrossLstIn: Arc<metamodelica::List<i32>>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqMap: metamodelica::Array<i32>, mut varMap: metamodelica::Array<i32>, mut daeEqsIn: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut daeVarsIn: BackendDAE::Variables, mut replEqsIn: Arc<metamodelica::List<i32>>) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((loopsIn, eqCrossLstIn.clone(), varCrossLstIn.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            return Ok((daeEqsIn, replEqsIn))
        },
        (Deref @ metamodelica::List::Cons { head: loop1, tail: rest }, Deref @ metamodelica::List::Cons { head: _, tail: crossEqs }, Deref @ metamodelica::List::Nil) => {
            let mut pos: i32;
            let mut eqs: Arc<metamodelica::List<i32>>;
            let mut vars: Arc<metamodelica::List<i32>>;
            let mut replEqs: Arc<metamodelica::List<i32>>;
            let mut loopVars: Arc<metamodelica::List<i32>>;
            let mut adjVars: Arc<metamodelica::List<i32>>;
            let mut m_row: Arc<metamodelica::List<i32>>;
            let mut eqVars: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut resolvedEq: Arc<BackendDAE::Equation>;
            let mut daeEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut loop1 = (*loop1).clone();
            let mut rest = (*rest).clone();
            let mut crossEqs = (*crossEqs).clone();
            loop1 = List::unique(loop1.clone());
            (resolvedEq, m_row) = resolveClosedLoop(loop1.clone(), mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqsIn.clone(), daeVarsIn.clone())?;
            (crossEqs, eqs, _) = List::intersection1OnTrue(loop1.clone(), eqCrossLstIn.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            replEqs = List::intersectionOnTrue(replEqsIn.clone(), loop1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            if !(eqs.clone().is_empty()) {
                pos = listHead(eqs)?;
            } else if !(replEqs.clone().is_empty()) {
                pos = listHead(replEqs)?;
            } else if !(crossEqs.clone().is_empty()) {
                pos = listHead(crossEqs.clone())?;
            } else {
                pos = -1;
            }
            (eqs, _) = List::deleteMemberOnTrue(pos, loop1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            eqVars = List::map1(loop1.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), mIn.clone())?;
            vars = List::flatten(eqVars)?;
            loopVars = doubleEntriesInLst(vars.clone());
            (_, adjVars, _) = List::intersection1OnTrue(vars, loopVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            List::map2_0(loopVars, (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), metamodelica::nil(), mTIn.clone())?;
            List::map2_0(adjVars.clone(), (std::sync::Arc::new(arrayGetDeleteInLst) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> + 'static>), loop1.clone(), mTIn.clone())?;
            List::map2_0(adjVars.clone(), (std::sync::Arc::new(arrayGetAppendLst) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> + 'static>), list![pos], mTIn.clone())?;
            List::map2_0(loop1.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), metamodelica::nil(), mIn.clone())?;
            metamodelica::arrayUpdate(mIn.clone(), pos, adjVars)?;
            rest = List::map2(rest.clone(), (std::sync::Arc::new(replaceContractedNodes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), pos, eqs)?;
            rest = List::unique(rest.clone());
            replEqs = metamodelica::cons(pos, replEqsIn);
            metamodelica::arrayUpdate(mIn.clone(), pos, m_row)?;
            pos = metamodelica::arrayGet(eqMap.clone(), pos)?;
            daeEqs = BackendEquation::setAtIndex(daeEqsIn, pos, resolvedEq)?;
            { (loopsIn, eqCrossLstIn, varCrossLstIn, mIn, mTIn, eqMap, varMap, daeEqsIn, daeVarsIn, replEqsIn) = (rest.clone(), eqCrossLstIn, varCrossLstIn, mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqs, daeVarsIn, replEqs); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: loop1, tail: rest }, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: _, tail: crossVars }) => {
            let mut pos: i32;
            let mut eqs: Arc<metamodelica::List<i32>>;
            let mut vars: Arc<metamodelica::List<i32>>;
            let mut replEqs: Arc<metamodelica::List<i32>>;
            let mut loopVars: Arc<metamodelica::List<i32>>;
            let mut adjVars: Arc<metamodelica::List<i32>>;
            let mut m_row: Arc<metamodelica::List<i32>>;
            let mut eqVars: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut resolvedEq: Arc<BackendDAE::Equation>;
            let mut daeEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut loop1 = (*loop1).clone();
            let mut rest = (*rest).clone();
            let mut crossVars = (*crossVars).clone();
            loop1 = List::unique(loop1.clone());
            (resolvedEq, m_row) = resolveClosedLoop(loop1.clone(), mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqsIn.clone(), daeVarsIn.clone())?;
            (replEqs, _, eqs) = List::intersection1OnTrue(replEqsIn.clone(), loop1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            eqs = priorizeEqsWithVarCrosses(eqs, mIn.clone(), varCrossLstIn.clone())?;
            pos = if (!(replEqs.clone().is_empty())) {listHead(replEqs)?} else {-1};
            pos = if (!(eqs.clone().is_empty())) {listHead(eqs)?} else {pos};
            (eqs, _) = List::deleteMemberOnTrue(pos, loop1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            eqVars = List::map1(loop1.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), mIn.clone())?;
            vars = List::flatten(eqVars)?;
            loopVars = doubleEntriesInLst(vars.clone());
            (crossVars, loopVars, _) = List::intersection1OnTrue(loopVars, varCrossLstIn.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            (_, adjVars, _) = List::intersection1OnTrue(vars, loopVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            adjVars = listAppend(crossVars.clone(), adjVars);
            adjVars = List::unique(adjVars);
            List::map2_0(loopVars, (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), metamodelica::nil(), mTIn.clone())?;
            List::map2_0(adjVars.clone(), (std::sync::Arc::new(arrayGetDeleteInLst) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> + 'static>), loop1.clone(), mTIn.clone())?;
            List::map2_0(adjVars.clone(), (std::sync::Arc::new(arrayGetAppendLst) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> + 'static>), list![pos], mTIn.clone())?;
            List::map2_0(loop1.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), metamodelica::nil(), mIn.clone())?;
            metamodelica::arrayUpdate(mIn.clone(), pos, adjVars)?;
            rest = List::map2(rest.clone(), (std::sync::Arc::new(replaceContractedNodes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), pos, eqs)?;
            rest = List::unique(rest.clone());
            replEqs = metamodelica::cons(pos, replEqsIn);
            metamodelica::arrayUpdate(mIn.clone(), pos, m_row)?;
            pos = metamodelica::arrayGet(eqMap.clone(), pos)?;
            daeEqs = BackendEquation::setAtIndex(daeEqsIn, pos, resolvedEq)?;
            { (loopsIn, eqCrossLstIn, varCrossLstIn, mIn, mTIn, eqMap, varMap, daeEqsIn, daeVarsIn, replEqsIn) = (rest.clone(), eqCrossLstIn, varCrossLstIn, mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqs, daeVarsIn, replEqs); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: loop1, tail: rest }, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            let mut pos: i32;
            let mut vars: Arc<metamodelica::List<i32>>;
            let mut crossEqs: Arc<metamodelica::List<i32>>;
            let mut replEqs: Arc<metamodelica::List<i32>>;
            let mut m_row: Arc<metamodelica::List<i32>>;
            let mut eqVars: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut resolvedEq: Arc<BackendDAE::Equation>;
            let mut daeEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut loop1 = (*loop1).clone();
            loop1 = List::unique(loop1.clone());
            (resolvedEq, m_row) = resolveClosedLoop(loop1.clone(), mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqsIn.clone(), daeVarsIn.clone())?;
            (_, crossEqs, _) = List::intersection1OnTrue(loop1.clone(), replEqsIn.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            let __pa0 = ::match_deref::match_deref! { match &(crossEqs.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            pos = __pa0.clone();
            eqVars = List::map1(loop1.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), mIn.clone())?;
            vars = List::flatten(eqVars)?;
            List::map2_0(loop1.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), metamodelica::nil(), mIn.clone())?;
            List::map2_0(vars, (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), metamodelica::nil(), mTIn.clone())?;
            replEqs = metamodelica::cons(pos, replEqsIn);
            metamodelica::arrayUpdate(mIn.clone(), pos, m_row)?;
            pos = metamodelica::arrayGet(eqMap.clone(), pos)?;
            daeEqs = BackendEquation::setAtIndex(daeEqsIn, pos, resolvedEq)?;
            { (loopsIn, eqCrossLstIn, varCrossLstIn, mIn, mTIn, eqMap, varMap, daeEqsIn, daeVarsIn, replEqsIn) = (rest.clone(), eqCrossLstIn, varCrossLstIn, mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqs, daeVarsIn, replEqs); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: loop1, tail: rest }, Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
            let mut pos: i32;
            let mut eq1: i32;
            let mut eq2: i32;
            let mut replEqs: Arc<metamodelica::List<i32>>;
            let mut m_row: Arc<metamodelica::List<i32>>;
            let mut resolvedEq: Arc<BackendDAE::Equation>;
            let mut daeEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut loop1 = (*loop1).clone();
            loop1 = List::unique(loop1.clone());
            let true = ((loop1.clone().len() as i32) == 2) else { bail!("pattern mismatch") };
            (resolvedEq, m_row) = resolveClosedLoop(loop1.clone(), mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqsIn.clone(), daeVarsIn.clone())?;
            if eqIsConst(resolvedEq.clone()) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(loop1.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                eq1 = __pa0.clone();
                eq2 = __pa1.clone();
                if (BackendEquation::equationVars(BackendEquation::get(daeEqsIn.clone(), metamodelica::arrayGet(eqMap.clone(), eq1)?)?, daeVarsIn.clone())?.len() as i32) >= (BackendEquation::equationVars(BackendEquation::get(daeEqsIn.clone(), metamodelica::arrayGet(eqMap.clone(), eq2)?)?, daeVarsIn.clone())?.len() as i32) {
                    pos = eq1;
                } else {
                    pos = eq2;
                }
                replEqs = metamodelica::cons(pos, replEqsIn);
                metamodelica::arrayUpdate(mIn.clone(), pos, m_row)?;
                pos = metamodelica::arrayGet(eqMap.clone(), pos)?;
                daeEqs = BackendEquation::setAtIndex(daeEqsIn, pos, resolvedEq)?;
            } else {
                replEqs = replEqsIn;
                daeEqs = daeEqsIn;
            }
            { (loopsIn, eqCrossLstIn, varCrossLstIn, mIn, mTIn, eqMap, varMap, daeEqsIn, daeVarsIn, replEqsIn) = (rest.clone(), eqCrossLstIn, varCrossLstIn, mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqs, daeVarsIn, replEqs); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn eqIsConst(mut eq: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(eq) {
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::RCONST { .. }, scalar: Deref @ DAE::Exp::CREF { .. }, .. } => true,
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { .. }, scalar: Deref @ DAE::Exp::RCONST { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn arrayIsZeroAt(mut pos: i32, mut arr: metamodelica::Array<i32>) -> bool {
    let mut isZero: bool;
    isZero = intEq(0, ({let __elt = arr.borrow()[(pos-1) as usize].clone(); __elt}));
    isZero
}

fn markDeadEndsInBipartiteGraph(mut varIdx: i32, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut deadEndEqs: metamodelica::Array<i32>, mut deadEndVars: metamodelica::Array<i32>) -> Result<()> {
    let mut eqIdx: i32;
    let mut nextVarIdx: i32;
    let mut adjEqs: Arc<metamodelica::List<i32>>;
    let mut adjVars: Arc<metamodelica::List<i32>>;
    adjEqs = metamodelica::arrayGet(mTIn.clone(), varIdx)?;
    adjEqs = List::filter1OnTrue(adjEqs, (std::sync::Arc::new(fnptr!(arrayIsZeroAt, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), deadEndEqs.clone())?;
    if (adjEqs.clone().len() as i32) == 1 {
        eqIdx = listHead(adjEqs)?;
        metamodelica::arrayUpdate(deadEndVars.clone(), varIdx, 1)?;
        adjVars = metamodelica::arrayGet(mIn.clone(), eqIdx)?;
        adjVars = List::filter1OnTrue(adjVars, (std::sync::Arc::new(fnptr!(arrayIsZeroAt, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), deadEndVars.clone())?;
        if (adjVars.clone().len() as i32) == 1 {
            nextVarIdx = listHead(adjVars)?;
            metamodelica::arrayUpdate(deadEndEqs.clone(), eqIdx, 1)?;
            markDeadEndsInBipartiteGraph(nextVarIdx, mIn.clone(), mTIn.clone(), deadEndEqs.clone(), deadEndVars.clone())?;
        }
    }
    Ok(())
}

fn arrayGetDeleteInLst(mut idx: i32, mut delEntries: Arc<metamodelica::List<i32>>, mut arrIn: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut entry: Arc<metamodelica::List<i32>>;
    entry = metamodelica::arrayGet(arrIn.clone(), idx)?;
    (_, entry, _) = List::intersection1OnTrue(entry, delEntries, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    metamodelica::arrayUpdate(arrIn.clone(), idx, entry)?;
    Ok(())
}

fn arrayGetAppendLst(mut idx: i32, mut appLst: Arc<metamodelica::List<i32>>, mut arrIn: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut entry: Arc<metamodelica::List<i32>>;
    entry = metamodelica::arrayGet(arrIn.clone(), idx)?;
    metamodelica::arrayUpdate(arrIn.clone(), idx, listAppend(entry, appLst))?;
    Ok(())
}

fn getReverseDoubles(mut elem: Arc<metamodelica::List<i32>>, mut elemLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut foldLstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut foldLstOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    foldLstOut = 'mc: {
        let __mc_input = foldLstIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut elemR: Arc<metamodelica::List<i32>>;
                    let mut foldLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    elemR = elem.clone().reverse();
                    elemR = List::getMember(elemR.clone(), elemLst.clone())?;
                    (foldLst, _) = List::deleteMemberOnTrue(elem.clone(), foldLstIn.clone(), (std::sync::Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
                    Ok(metamodelica::cons(elemR.clone(), foldLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(foldLstIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    foldLstOut
}

fn getDoubles(mut elemLstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut lstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut lstOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    lstOut = (::match_deref::match_deref! { match &(elemLstIn) {
        Deref @ metamodelica::List::Nil => {
            lstIn
        },
        Deref @ metamodelica::List::Cons { head: elem, tail: elemLst } => {
            let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            if listMember(elem.clone(), elemLst.clone()) {
                lst = getDoubles(elemLst.clone(), metamodelica::cons(elem.clone(), lstIn));
            } else {
                lst = getDoubles(elemLst.clone(), lstIn);
            }
            lst
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    lstOut
}

fn getTriples(mut crossNodes: Arc<metamodelica::List<i32>>, mut minAdj: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut tripleLoops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut allPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut path1: Arc<metamodelica::List<i32>>;
    let mut path2: Arc<metamodelica::List<i32>>;
    let mut path3: Arc<metamodelica::List<i32>>;
    for mut c0 in &*crossNodes {
        let mut c0 = c0.clone();
        path1 = metamodelica::arrayGet(minAdj.clone(), c0.clone())?;
        for mut c1 in &*path1.clone() {
            let mut c1 = c1.clone();
            if intGt(c1.clone(), c0.clone()) {
                path2 = metamodelica::arrayGet(minAdj.clone(), c1.clone())?;
                for mut c2 in &*path2.clone() {
                    let mut c2 = c2.clone();
                    if intGt(c2.clone(), c1.clone()) {
                        path3 = metamodelica::arrayGet(minAdj.clone(), c2.clone())?;
                        if listContains(path3.clone(), c0.clone()) {
                            tripleLoops = metamodelica::cons(list![c0.clone(), c1.clone(), c2.clone()], tripleLoops.clone());
                            allPaths = metamodelica::cons(list![c1.clone(), c2.clone()], allPaths.clone());
                            allPaths = metamodelica::cons(list![c0.clone(), c2.clone()], allPaths.clone());
                            allPaths = metamodelica::cons(list![c0.clone(), c1.clone()], allPaths.clone());
                        }
                    }
                }
            }
        }
    }
    Ok((tripleLoops, allPaths))
}

fn getEqNodesForVarLoop(mut varIdcs: Arc<metamodelica::List<i32>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut eqIdcs: Arc<metamodelica::List<i32>>;
    let mut varEqLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut eqLst: Arc<metamodelica::List<i32>>;
    varEqLst = List::map1(varIdcs, (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), mTIn.clone())?;
    eqLst = List::flatten(varEqLst)?;
    eqIdcs = doubleEntriesInLst(eqLst);
    Ok(eqIdcs)
}

fn resolveClosedLoop(mut loopIn: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqMap: metamodelica::Array<i32>, mut varMap: metamodelica::Array<i32>, mut daeEqsIn: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut daeVarsIn: BackendDAE::Variables) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)> {
    let mut eqOut: Arc<BackendDAE::Equation>;
    let mut m_row: Arc<metamodelica::List<i32>>;
    let mut startEqIdx: i32;
    let mut startEqDaeIdx: i32;
    let mut loop1: Arc<metamodelica::List<i32>>;
    let mut restLoop: Arc<metamodelica::List<i32>>;
    let mut eq: Arc<BackendDAE::Equation>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(loopIn) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    startEqIdx = __pa0.clone();
    restLoop = __pa1.clone();
    startEqDaeIdx = metamodelica::arrayGet(eqMap.clone(), startEqIdx)?;
    loop1 = sortLoop(restLoop, m.clone(), mT.clone(), list![startEqIdx])?;
    if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? && (loop1.clone().len() as i32) > 1 {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("solve the loop: ")); __mm_s.push_str(&*List::toString(loop1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    eq = BackendEquation::get(daeEqsIn.clone(), startEqDaeIdx)?;
    (eqOut, m_row) = resolveClosedLoop2(eq, loop1, m.clone(), metamodelica::arrayGet(m.clone(), startEqIdx)?, eqMap.clone(), varMap.clone(), daeEqsIn, daeVarsIn)?;
    Ok((eqOut, m_row))
}

fn resolveClosedLoop2(mut eq: Arc<BackendDAE::Equation>, mut loopIn: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m_row: Arc<metamodelica::List<i32>>, mut eqMap: metamodelica::Array<i32>, mut varMap: metamodelica::Array<i32>, mut daeEqsIn: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut daeVarsIn: BackendDAE::Variables) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)> {
    let mut eq: Arc<BackendDAE::Equation> = eq;
    let mut m_row: Arc<metamodelica::List<i32>> = m_row;
    (eq, m_row) = (::match_deref::match_deref! { match &(loopIn) {
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => {
            (eq, m_row)
        },
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: eqIdx2, tail: restLoop } } => {
            let mut algSign: bool;
            let mut adjVars: Arc<metamodelica::List<i32>>;
            let mut adjVars1: Arc<metamodelica::List<i32>>;
            let mut adjVars2: Arc<metamodelica::List<i32>>;
            let mut posVars: Arc<metamodelica::List<i32>>;
            let mut negVars: Arc<metamodelica::List<i32>>;
            let mut nonUnitVars: Arc<metamodelica::List<i32>>;
            let mut adjCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut eq2: Arc<BackendDAE::Equation>;
            let mut eq3: Arc<BackendDAE::Equation>;
            let mut resolvedEq: Arc<BackendDAE::Equation>;
            let mut replacements: BackendVarTransform::VariableReplacements;
            eq2 = BackendEquation::get(daeEqsIn.clone(), metamodelica::arrayGet(eqMap.clone(), eqIdx2.clone())?)?;
            adjVars1 = m_row;
            adjVars2 = metamodelica::arrayGet(m.clone(), eqIdx2.clone())?;
            (adjVars, adjVars1, adjVars2) = List::intersection1OnTrue(adjVars1, adjVars2, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            (adjVars, nonUnitVars) = List::splitOnTrue(adjVars, (std::sync::Arc::new({ let __pe_b1 = varMap.clone(); let __pe_b2 = daeVarsIn.clone(); let __pe_b3 = eq.clone(); let __pe_b4 = eq2.clone(); move |__pe_a0| varIsUnitCoeff(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            (posVars, negVars) = List::splitOnTrue(adjVars, (std::sync::Arc::new({ let __pe_b1 = varMap.clone(); let __pe_b2 = daeVarsIn.clone(); let __pe_b3 = eq.clone(); let __pe_b4 = eq2.clone(); move |__pe_a0| varSign(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            algSign = (posVars.clone().len() as i32) > (negVars.clone().len() as i32);
            adjCrefs = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut idx in (if (algSign) {posVars.clone()} else {negVars.clone()}).into_iter().cloned() {
            let __x = crefFromIndex(idx.clone(), varMap.clone(), daeVarsIn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            m_row = List::flatten(list![adjVars1, adjVars2, nonUnitVars, if (algSign) {negVars} else {posVars}])?;
            replacements = BackendVarTransform::emptyReplacementsSized((adjCrefs.clone().len() as i32));
            replacements = BackendVarTransform::addReplacements(replacements, adjCrefs.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut c in (adjCrefs).into_iter().cloned() {
            let __x = Expression::createZeroExpression(ComponentReference::crefTypeFull(c.clone())?)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), None)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendVarTransform::replaceEquations(list![eq.clone(), eq2.clone()], replacements, None)?) {
                (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } }, _) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            resolvedEq = __pa0.clone();
            eq3 = __pa1.clone();
            resolvedEq = sumUp2Equations(algSign, resolvedEq, eq3)?;
            if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("From eqs \n")); __mm_s.push_str(&*BackendDump::equationString(eq)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*BackendDump::equationString(eq2)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("resolved the eq \n")); __mm_s.push_str(&*BackendDump::equationString(resolvedEq.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            resolveClosedLoop2(resolvedEq, metamodelica::cons(eqIdx2.clone(), restLoop.clone()), m.clone(), m_row, eqMap.clone(), varMap.clone(), daeEqsIn, daeVarsIn)?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((eq, m_row))
}

fn crefFromIndex(mut varIdx: i32, mut varMap: metamodelica::Array<i32>, mut daeVarsIn: BackendDAE::Variables) -> Result<Arc<DAE::ComponentRef>> {
    let mut cref: Arc<DAE::ComponentRef>;
    let mut daeVarIdx: i32;
    let mut var: BackendDAE::Var;
    daeVarIdx = metamodelica::arrayGet(varMap.clone(), varIdx)?;
    var = BackendVariable::getVarAt(daeVarsIn, daeVarIdx)?;
    cref = BackendVariable::varCref(var)?;
    Ok(cref)
}

fn varSign(mut index: i32, mut varMap: metamodelica::Array<i32>, mut daeVarsIn: BackendDAE::Variables, mut eq1: Arc<BackendDAE::Equation>, mut eq2: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut algSign: bool;
    let mut cref: Arc<DAE::ComponentRef> = crefFromIndex(index, varMap.clone(), daeVarsIn.clone())?;
    algSign = CRefIsPosOnRHS(cref.clone(), eq1)? != CRefIsPosOnRHS(cref, eq2)?;
    Ok(algSign)
}

fn varIsUnitCoeff(mut index: i32, mut varMap: metamodelica::Array<i32>, mut daeVarsIn: BackendDAE::Variables, mut eq1: Arc<BackendDAE::Equation>, mut eq2: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut isUnit: bool;
    let mut cref: Arc<DAE::ComponentRef> = crefFromIndex(index, varMap.clone(), daeVarsIn.clone())?;
    isUnit = crefHasUnitCoeff(cref.clone(), eq1)? && crefHasUnitCoeff(cref, eq2)?;
    Ok(isUnit)
}

fn crefHasUnitCoeff(mut cref: Arc<DAE::ComponentRef>, mut eq: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut isUnit: bool;
    isUnit = (::match_deref::match_deref! { match &(eq) {
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. } => {
            crefUnitCoeffInExp(e1.clone(), cref.clone())? && crefUnitCoeffInExp(e2.clone(), cref)?
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isUnit)
}

fn crefUnitCoeffInExp(mut exp: Arc<DAE::Exp>, mut cref: Arc<DAE::ComponentRef>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp) {
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } => {
            return Ok(crefUnitCoeffInExp(e1.clone(), cref.clone())? && crefUnitCoeffInExp(e2.clone(), cref)?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
            return Ok(crefUnitCoeffInExp(e1.clone(), cref.clone())? && crefUnitCoeffInExp(e2.clone(), cref)?)
        },
        Deref @ DAE::Exp::UNARY { exp: e1, .. } => {
            { (exp, cref) = (e1.clone(), cref); continue '__tco; }
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { componentRef: c, .. }, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
            return Ok(!(ComponentReferenceBasics::crefEqualNoStringCompare(cref, c.clone())?) || Expression::isOne(e2.clone()) || Expression::isConstMinusOne(e2.clone()))
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CREF { componentRef: c, .. } } => {
            return Ok(!(ComponentReferenceBasics::crefEqualNoStringCompare(cref, c.clone())?) || Expression::isOne(e1.clone()) || Expression::isConstMinusOne(e1.clone()))
        },
        _ => {
            return Ok(true)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn sortLoop(mut loopIn: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut sortLoopIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((loopIn.clone(), sortLoopIn.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok(sortLoopIn.reverse())
        },
        (_, Deref @ metamodelica::List::Cons { head: start, tail: _ }) => {
            let mut next: i32;
            let mut rest: Arc<metamodelica::List<i32>>;
            let mut vars: Arc<metamodelica::List<i32>>;
            let mut eqs: Arc<metamodelica::List<i32>>;
            let mut varEqs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            vars = metamodelica::arrayGet(m.clone(), start.clone())?;
            varEqs = List::map1(vars, (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), mT.clone())?;
            eqs = List::flatten(varEqs)?;
            eqs = List::unique(eqs);
            eqs = List::intersectionOnTrue(eqs, loopIn.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            if eqs.clone().is_empty() {
                next = listHead(loopIn.clone())?;
            } else {
                next = listHead(eqs)?;
            }
            (rest, _) = List::deleteMemberOnTrue(next, loopIn, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            { (loopIn, m, mT, sortLoopIn) = (rest, m.clone(), mT.clone(), metamodelica::cons(next, sortLoopIn)); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn closePathDirectly(mut pathIn: Arc<metamodelica::List<i32>>, mut pathLstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut pathOut: Arc<metamodelica::List<i32>>;
    pathOut = 'mc: {
        let __mc_input = pathLstIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut startNode: i32;
                    let mut endNode: i32;
                    startNode = listHead(pathIn.clone())?;
                    endNode = List::last(pathIn.clone())?;
                    let true = (intEq(startNode.clone(), endNode.clone())) else { bail!("pattern mismatch") };
                    Ok(pathIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut closed: bool;
                    let mut startNode: i32;
                    let mut endNode: i32;
                    let mut path: Arc<metamodelica::List<i32>>;
                    let __pa0 = ::match_deref::match_deref! { match &(pathIn.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    startNode = __pa0.clone();
                    endNode = List::last(pathIn.clone())?;
                    path = findPathByEnds(pathLstIn.clone(), startNode.clone(), endNode.clone())?;
                    closed = !(path.clone().is_empty());
                    path = if (closed.clone()) {path.clone()} else {metamodelica::nil()};
                    path = listAppend(pathIn.clone(), path.clone());
                    path = List::unique(path.clone());
                    Ok(path.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function ResolveLoops.closePathDirectly failed")).clone(), metamodelica::sourceInfo!("BackEnd/ResolveLoops.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(pathOut)
}

fn findPathByEnds(mut pathLstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut startNodeIn: i32, mut endNodeIn: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut pathOut: Arc<metamodelica::List<i32>>;
    pathOut = 'mc: {
        let __mc_input = pathLstIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: path, tail: pathLst } => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut startNode: i32;
                    let mut endNode: i32;
                    let mut path = (*path).clone();
                    startNode = listHead(path.clone())?;
                    b1 = intEq(startNode.clone(), endNodeIn);
                    endNode = List::last(path.clone())?;
                    b2 = intEq(endNode.clone(), startNodeIn);
                    path = if (!(b1.clone() && b2.clone())) {findPathByEnds(pathLst.clone(), startNodeIn, endNodeIn)?} else {path.clone()};
                    Ok(path.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
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
                _ => {
                    Error::addInternalError((literal!("function ResolveLoops.findPathByEnds failed")).clone(), metamodelica::sourceInfo!("BackEnd/ResolveLoops.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(pathOut)
}

fn countDoubleEntriesInLst(mut lstIn: Arc<metamodelica::List<i32>>, mut checkLst: Arc<metamodelica::List<i32>>, mut dupLst: Arc<metamodelica::List<i32>>) -> (i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut num: i32 = 0;
    let mut checkLst: Arc<metamodelica::List<i32>> = checkLst;
    let mut dupLst: Arc<metamodelica::List<i32>> = dupLst;
    for mut elem in &*lstIn {
        let mut elem = elem.clone();
        if listMember(elem.clone(), checkLst.clone()) {
            num = num + 1;
            if !(listMember(elem.clone(), dupLst.clone())) {
                dupLst = metamodelica::cons(elem.clone(), dupLst.clone());
            }
        } else {
            checkLst = metamodelica::cons(elem.clone(), checkLst.clone());
        }
    }
    (num, checkLst, dupLst)
}

fn countDoubleEntriesInLstLst(mut lstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut checkLst: Arc<metamodelica::List<i32>>, mut dupLst: Arc<metamodelica::List<i32>>) -> (i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut num: i32 = 0;
    let mut checkLst: Arc<metamodelica::List<i32>> = checkLst;
    let mut dupLst: Arc<metamodelica::List<i32>> = dupLst;
    for mut lst in &*lstIn {
        let mut lst = lst.clone();
        for mut elem in &*lst.clone() {
            let mut elem = elem.clone();
            if listMember(elem.clone(), checkLst.clone()) {
                num = num + 1;
                if !(listMember(elem.clone(), dupLst.clone())) {
                    dupLst = metamodelica::cons(elem.clone(), dupLst.clone());
                }
            } else {
                checkLst = metamodelica::cons(elem.clone(), checkLst.clone());
            }
        }
    }
    (num, checkLst, dupLst)
}

fn doubleEntriesInLst(mut lstIn: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut doubleLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut checkLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut i in &*lstIn {
        let mut i = i.clone();
        if listMember(i.clone(), checkLst.clone()) {
            doubleLst = metamodelica::cons(i.clone(), doubleLst.clone());
        } else {
            checkLst = metamodelica::cons(i.clone(), checkLst.clone());
        }
    }
    doubleLst
}

fn getPathTillNextCrossEq(mut checkEqCrossNodes: Arc<metamodelica::List<i32>>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut allEqCrossNodes: Arc<metamodelica::List<i32>>, mut unfinPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut eqPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut eqPathsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    eqPathsOut = 'mc: {
        let __mc_input = (checkEqCrossNodes.clone(), unfinPathsIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: crossEq, tail: restCrossNodes }, Deref @ metamodelica::List::Nil) => {
                    let mut adjVars: Arc<metamodelica::List<i32>>;
                    let mut nextEqs: Arc<metamodelica::List<i32>>;
                    let mut endEqs: Arc<metamodelica::List<i32>>;
                    let mut unfinEqs: Arc<metamodelica::List<i32>>;
                    let mut paths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut adjEqs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut unfinPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    adjVars = metamodelica::arrayGet(mIn.clone(), crossEq.clone())?;
                    adjEqs = List::map1(adjVars.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), mTIn.clone())?;
                    adjEqs = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut eq in (adjEqs.clone()).into_iter().cloned() {
                    let __x = (List::deleteMemberOnTrue(crossEq.clone(), eq.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?).0;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    adjEqs = List::filterOnFalse(adjEqs.clone(), std::sync::Arc::new(fnptr!(listEmpty, _)))?;
                    nextEqs = List::flatten(adjEqs.clone())?;
                    (endEqs, unfinEqs, _) = List::intersection1OnTrue(nextEqs.clone(), allEqCrossNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    paths = List::map1(endEqs.clone(), (std::sync::Arc::new(fnptr!(cons1, i32, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), list![crossEq.clone()])?;
                    paths = listAppend(paths.clone(), eqPathsIn.clone());
                    unfinPaths = List::map1(unfinEqs.clone(), (std::sync::Arc::new(fnptr!(cons1, i32, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), list![crossEq.clone()])?;
                    unfinPaths = listAppend(unfinPaths.clone(), unfinPathsIn.clone());
                    paths = getPathTillNextCrossEq(restCrossNodes.clone(), mIn.clone(), mTIn.clone(), allEqCrossNodes.clone(), unfinPaths.clone(), paths.clone())?;
                    Ok(paths.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: pathStart, tail: restUnfinPaths }) => {
                    let mut lastEq: i32;
                    let mut prevEq: i32;
                    let mut adjVars: Arc<metamodelica::List<i32>>;
                    let mut nextEqs: Arc<metamodelica::List<i32>>;
                    let mut endEqs: Arc<metamodelica::List<i32>>;
                    let mut unfinEqs: Arc<metamodelica::List<i32>>;
                    let mut paths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut adjEqs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut unfinPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    lastEq = listHead(pathStart.clone())?;
                    prevEq = List::second(pathStart.clone())?;
                    adjVars = metamodelica::arrayGet(mIn.clone(), lastEq.clone())?;
                    adjEqs = List::map1(adjVars.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), mTIn.clone())?;
                    adjEqs = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut eq in (adjEqs.clone()).into_iter().cloned() {
                    let __x = (List::deleteMemberOnTrue(lastEq.clone(), eq.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?).0;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    adjEqs = List::filterOnFalse(adjEqs.clone(), std::sync::Arc::new(fnptr!(listEmpty, _)))?;
                    nextEqs = List::map(adjEqs.clone(), (std::sync::Arc::new(listHead) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
                    (nextEqs, _) = List::deleteMemberOnTrue(prevEq.clone(), nextEqs.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    (endEqs, unfinEqs, _) = List::intersection1OnTrue(nextEqs.clone(), allEqCrossNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    paths = List::map1(endEqs.clone(), (std::sync::Arc::new(fnptr!(cons1, i32, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), pathStart.clone())?;
                    paths = listAppend(paths.clone(), eqPathsIn.clone());
                    unfinPaths = List::map1(unfinEqs.clone(), (std::sync::Arc::new(fnptr!(cons1, i32, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), pathStart.clone())?;
                    unfinPaths = listAppend(unfinPaths.clone(), restUnfinPaths.clone());
                    paths = getPathTillNextCrossEq(checkEqCrossNodes.clone(), mIn.clone(), mTIn.clone(), allEqCrossNodes.clone(), unfinPaths.clone(), paths.clone())?;
                    Ok(paths.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(eqPathsIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError((literal!("function ResolveLoops.getPathTillNextCrossEq failed")).clone(), metamodelica::sourceInfo!("BackEnd/ResolveLoops.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(eqPathsOut)
}

fn cons1(mut elem: i32, mut lst: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut outLst: Arc<metamodelica::List<i32>>;
    outLst = metamodelica::cons(elem, lst);
    outLst
}

fn replaceContractedNodes(mut pathIn: Arc<metamodelica::List<i32>>, mut nodeIn: i32, mut replNodes: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut pathOut: Arc<metamodelica::List<i32>>;
    pathOut = List::map2(pathIn, (std::sync::Arc::new(replaceContractedNodes2) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, Arc<metamodelica::List<i32>>) -> Result<i32> + 'static>), nodeIn, replNodes)?;
    Ok(pathOut)
}

fn replaceContractedNodes2(mut entryIn: i32, mut nodeIn: i32, mut replNodes: Arc<metamodelica::List<i32>>) -> Result<i32> {
    let mut entryOut: i32;
    let mut repl: bool;
    repl = List::isMemberOnTrue(entryIn, replNodes, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    entryOut = if (repl) {nodeIn} else {entryIn};
    Ok(entryOut)
}

fn priorizeEqsWithVarCrosses(mut eqsIn: Arc<metamodelica::List<i32>>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut varCrossLst: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut eqsOut: Arc<metamodelica::List<i32>>;
    let mut priorities: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    priorities = arrayCreate(3, metamodelica::nil());
    for mut eq in &*eqsIn {
        let mut eq = eq.clone();
        priorizeEqsWithVarCrosses2(eq.clone(), mIn.clone(), varCrossLst.clone(), priorities.clone())?;
    }
    eqsOut = List::flatten(Arc::new(priorities.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()))?;
    Ok(eqsOut)
}

fn priorizeEqsWithVarCrosses2(mut eq: i32, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut varCrossLst: Arc<metamodelica::List<i32>>, mut priorities: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut eqVars: Arc<metamodelica::List<i32>>;
    let mut crossVars: Arc<metamodelica::List<i32>>;
    eqVars = metamodelica::arrayGet(mIn.clone(), eq)?;
    crossVars = List::intersectionOnTrue(eqVars, varCrossLst, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if crossVars.clone().is_empty() {
        arrayGetAppendLst(1, list![eq], priorities.clone())?;
    } else if List::hasOneElement(crossVars) {
        arrayGetAppendLst(2, list![eq], priorities.clone())?;
    } else {
        arrayGetAppendLst(3, list![eq], priorities.clone())?;
    }
    Ok(())
}

fn evaluateLoop(mut loopIn: Arc<metamodelica::List<i32>>, mut tplIn: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>)) -> Result<bool> {
    let mut resolve: bool = true;
    let mut r1: bool;
    let mut r2: bool;
    let mut numInLoop: i32;
    let mut numOutLoop: i32;
    let mut eqCrossLst: Arc<metamodelica::List<i32>>;
    let mut chk: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut dup: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqVars: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    if !(intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 3)) {
        (m, _, eqCrossLst) = tplIn;
        eqVars = List::map1(loopIn, (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), m.clone())?;
        (numInLoop, chk, dup) = countDoubleEntriesInLstLst(eqVars, chk, dup);
        numOutLoop = (chk.len() as i32) - (dup.len() as i32);
        r1 = intGe(numInLoop, numOutLoop - 1) && intLe(numInLoop, 6);
        r2 = intGe(numInLoop, numOutLoop - 2);
        r1 = if (intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 1)) {r1} else {false};
        resolve = if (intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 2)) {r2} else {r1};
    }
    Ok(resolve)
}

fn evaluateTripleLoop(mut loopIn: Arc<metamodelica::List<i32>>, mut tplIn: (metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<bool> {
    let mut resolve: bool = true;
    let mut r1: bool;
    let mut r2: bool;
    let mut n: i32;
    let mut numInLoop: i32 = 0;
    let mut numOutLoop: i32 = 0;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut map: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapIndices: Arc<metamodelica::List<i32>>;
    let mut chk: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut dup: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if !(intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 3)) {
        (m, mapIndices, map) = tplIn;
        for mut j in &*loopIn {
            let mut j = j.clone();
            (n, chk, dup) = countDoubleEntriesInLst(metamodelica::arrayGet(m.clone(), j.clone())?, chk.clone(), dup.clone());
            numInLoop = numInLoop + n;
        }
        for mut i in &*mapIndices {
            let mut i = i.clone();
            for mut j in &*metamodelica::arrayGet(map.clone(), i.clone())? {
                let mut j = j.clone();
                (n, chk, dup) = countDoubleEntriesInLst(metamodelica::arrayGet(m.clone(), j.clone())?, chk.clone(), dup.clone());
                numInLoop = numInLoop + n;
            }
        }
        numOutLoop = (chk.len() as i32) - (dup.len() as i32);
        r1 = intGe(numInLoop, numOutLoop - 1) && intLe(numInLoop, 10);
        r2 = intGe(numInLoop, numOutLoop - 2);
        r1 = if (intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 1)) {r1} else {false};
        resolve = if (intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 2)) {r2} else {r1};
    }
    Ok(resolve)
}

fn updateTripleLoop(mut loopFull: Arc<metamodelica::List<i32>>, mut tplIn: (metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<Arc<metamodelica::List<i32>>> {
    let mut loopFull: Arc<metamodelica::List<i32>> = loopFull;
    let mut map: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mapIndices: Arc<metamodelica::List<i32>>;
    (_, mapIndices, map) = tplIn;
    for mut i in &*mapIndices {
        let mut i = i.clone();
        loopFull = listAppend(metamodelica::arrayGet(map.clone(), i.clone())?, loopFull.clone());
    }
    Ok(loopFull)
}

fn sumUp2Equations(mut sumUp: bool, mut eq1: Arc<BackendDAE::Equation>, mut eq2: Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> {
    let mut eqOut: Arc<BackendDAE::Equation>;
    let mut exp1: Arc<DAE::Exp>;
    let mut exp2: Arc<DAE::Exp>;
    let mut exp3: Arc<DAE::Exp>;
    let mut exp4: Arc<DAE::Exp>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eq1) {
        Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp1 = __pa0.clone();
    exp2 = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(eq2) {
        Deref @ BackendDAE::Equation::EQUATION { exp: __pa2, scalar: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp3 = __pa2.clone();
    exp4 = __pa3.clone();
    exp1 = sumUp2Expressions(sumUp, exp1, exp3)?;
    exp2 = sumUp2Expressions(sumUp, exp2, exp4)?;
    exp2 = sumUp2Expressions(false, exp2, exp1)?;
    (exp2, _) = ExpressionSimplify::simplify(exp2)?;
    exp1 = Expression::createZeroExpression(Expression::r#typeof(exp2.clone())?)?;
    eqOut = Arc::new(BackendDAE::Equation::EQUATION { exp: exp1, scalar: exp2, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() });
    eqOut = simplifyZeroAssignment(eqOut);
    Ok(eqOut)
}

fn simplifyZeroAssignment(mut eIn: Arc<BackendDAE::Equation>) -> Arc<BackendDAE::Equation> {
    let mut eOut: Arc<BackendDAE::Equation>;
    eOut = (::match_deref::match_deref! { match &(eIn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::RCONST { real: __rlit_0 }, scalar: Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: _ }, operator: DAE::Operator::MUL { .. }, exp2: e @ Deref @ DAE::Exp::CREF { .. } }, source, attr } if __rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), scalar: e.clone(), source: source.clone(), attr: attr.clone() })
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::RCONST { real: __rlit_1 }, exp: Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: _ }, operator: DAE::Operator::MUL { .. }, exp2: e @ Deref @ DAE::Exp::CREF { .. } }, source, attr } if __rlit_1.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), scalar: e.clone(), source: source.clone(), attr: attr.clone() })
        },
        _ => {
            eIn
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    eOut
}

fn CRefIsPosOnRHS(mut crefIn: Arc<DAE::ComponentRef>, mut eqIn: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut isPos: bool;
    isPos = 'mc: {
        let __mc_input = eqIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. } => {
                    let mut exists1: bool;
                    let mut sign1: bool;
                    let mut sign2: bool;
                    (exists1, sign1) = expIsCref(e1.clone(), crefIn.clone())?;
                    (_, sign2) = expIsCref(e2.clone(), crefIn.clone())?;
                    sign1 = if (exists1.clone()) {!(sign1.clone())} else {sign2.clone()};
                    Ok(sign1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("add a case to CRefIsPosOnRHS")); __mm_s.push_str(&*BackendDump::equationString(eqIn.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(isPos)
}

fn expIsCref(mut expIn: Arc<DAE::Exp>, mut crefIn: Arc<DAE::ComponentRef>) -> Result<(bool, bool)> {
    let mut isInExp: bool;
    let mut algSign: bool;
    (isInExp, algSign) = (::match_deref::match_deref! { match &(expIn.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cref, .. } => {
            let mut sameCref: bool;
            sameCref = ComponentReferenceBasics::crefEqualNoStringCompare(crefIn, cref.clone())?;
            (sameCref, true)
        },
        Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::SUB { .. }, exp2 } => {
            let mut sign: bool;
            let mut sign1: bool;
            let mut sign2: bool;
            let mut exists: bool;
            let mut exists1: bool;
            let mut exists2: bool;
            (exists1, sign1) = expIsCref(exp1.clone(), crefIn.clone())?;
            (exists2, sign2) = expIsCref(exp2.clone(), crefIn)?;
            sign2 = boolNot(sign2);
            exists = boolOr(exists1, exists2);
            sign = exists1 && sign1;
            sign = if (exists2) {sign2} else {sign};
            (exists, sign)
        },
        Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD { .. }, exp2 } => {
            let mut sign: bool;
            let mut sign1: bool;
            let mut sign2: bool;
            let mut exists: bool;
            let mut exists1: bool;
            let mut exists2: bool;
            (exists1, sign1) = expIsCref(exp1.clone(), crefIn.clone())?;
            (exists2, sign2) = expIsCref(exp2.clone(), crefIn)?;
            exists = boolOr(exists1, exists2);
            sign = exists1 && sign1;
            sign = if (exists2) {sign2} else {sign};
            (exists, sign)
        },
        Deref @ DAE::Exp::BINARY { exp1: exp1 @ Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::RCONST { real: r } } => {
            let mut sign: bool;
            let mut exists: bool;
            (exists, _) = expIsCref(exp1.clone(), crefIn)?;
            sign = r.clone() > metamodelica::OrderedFloat((0) as f64);
            (exists, sign)
        },
        Deref @ DAE::Exp::BINARY { exp1: exp1 @ Deref @ DAE::Exp::RCONST { real: r }, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CREF { .. } } => {
            let mut sign: bool;
            let mut exists: bool;
            (exists, _) = expIsCref(exp1.clone(), crefIn)?;
            sign = r.clone() > metamodelica::OrderedFloat((0) as f64);
            (exists, sign)
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: exp1 } => {
            let mut sign: bool;
            let mut exists: bool;
            (exists, sign) = expIsCref(exp1.clone(), crefIn)?;
            sign = boolNot(sign);
            (exists, sign)
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            (false, false)
        },
        Deref @ DAE::Exp::ICONST { .. } => {
            (false, false)
        },
        _ => {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("add a case to expIsCref:")); __mm_s.push_str(&*ExpressionBasics::printExpStr(expIn)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            (false, false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((isInExp, algSign))
}

fn listLengthIs(mut lst: Arc<metamodelica::List<i32>>, mut value: i32) -> bool {
    let mut bOut: bool;
    bOut = intEq((lst.len() as i32), value);
    bOut
}

pub(crate) fn partitionBipartiteGraph(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut partitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut numEqs: i32;
    let mut numVars: i32;
    let mut markEqs: metamodelica::Array<i32>;
    let mut markVars: metamodelica::Array<i32>;
    numEqs = metamodelica::arrayLength(m.clone());
    numVars = metamodelica::arrayLength(mT.clone());
    if numEqs == 0 || numVars == 0 {
        partitions = list![metamodelica::nil()];
    } else {
        markEqs = arrayCreate(numEqs, -1);
        markVars = arrayCreate(numVars, -1);
        (_, partitions) = colorNodePartitions(m.clone(), mT.clone(), list![1], markEqs.clone(), markVars.clone(), 1, metamodelica::nil(), 1)?;
    }
    Ok(partitions)
}

fn colorNodePartitions(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut checkNextIn: Arc<metamodelica::List<i32>>, mut markEqs: metamodelica::Array<i32>, mut markVars: metamodelica::Array<i32>, mut currNumberIn: i32, mut partitionsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut nextIndex: i32) -> Result<(i32, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    '__tco: loop {
        let mut eq: i32 = 0;
        let mut next_index: i32 = 0;
        let mut rest: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut eqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut part: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut restPart: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        let mut partitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        ::match_deref::match_deref! { match &(checkNextIn) {
        Deref @ metamodelica::List::Cons { head: 0, tail: Deref @ metamodelica::List::Nil } => return Ok((currNumberIn - 1, partitionsIn)),
        Deref @ metamodelica::List::Cons { head: __esc_eq, tail: __esc_rest } => {
            eq = (*__esc_eq).clone();
            rest = (*__esc_rest).clone();
            if arrayGetIsNotPositive(eq.clone(), markEqs.clone())? {
                metamodelica::arrayUpdate(markEqs.clone(), eq.clone(), currNumberIn)?;
                if partitionsIn.clone().is_empty() {
                    partitions = list![list![eq.clone()]];
                } else {
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(partitionsIn) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    part = __pa0.clone();
                    restPart = __pa1.clone();
                    part = metamodelica::cons(eq.clone(), part);
                    partitions = metamodelica::cons(part, restPart);
                }
                vars = metamodelica::arrayGet(m.clone(), eq.clone())?;
                let true = (!(vars.clone().is_empty())) else { bail!("pattern mismatch") };
                vars = List::filter1OnTrue(vars, (std::sync::Arc::new(arrayGetIsNotPositive) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), markVars.clone())?;
                List::map2_0(vars.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), currNumberIn, markVars.clone())?;
                eqs = List::fold1(vars, (std::sync::Arc::new(getArrayEntryAndAppend) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), mT.clone(), metamodelica::nil())?;
                eqs = List::filter1OnTrue(eqs, (std::sync::Arc::new(arrayGetIsNegative) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), markEqs.clone())?;
                List::map2_0(eqs.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), 0, markEqs.clone())?;
                rest = listAppend(rest.clone(), eqs);
            } else {
                partitions = partitionsIn;
            }
            { (m, mT, checkNextIn, markEqs, markVars, currNumberIn, partitionsIn, nextIndex) = (m.clone(), mT.clone(), rest.clone(), markEqs.clone(), markVars.clone(), currNumberIn, partitions, nextIndex); continue '__tco; }
        },
        Deref @ metamodelica::List::Nil => {
            eq = 0;
            next_index = nextIndex;
            for mut i in nextIndex..=metamodelica::arrayLength(markEqs.clone()) {
                if ({let __elt = markEqs.borrow()[(i.clone()-1) as usize].clone(); __elt}) == -1 {
                    eq = i.clone();
                    next_index = i.clone() + 1;
                    break;
                }
            }
            { (m, mT, checkNextIn, markEqs, markVars, currNumberIn, partitionsIn, nextIndex) = (m.clone(), mT.clone(), list![eq], markEqs.clone(), markVars.clone(), currNumberIn + 1, metamodelica::cons(metamodelica::nil(), partitionsIn), next_index); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn arrayGetIsNotPositive(mut idx: i32, mut arrayIn: metamodelica::Array<i32>) -> Result<bool> {
    let mut isNonZero: bool;
    isNonZero = metamodelica::arrayGet(arrayIn.clone(), idx)? <= 0;
    Ok(isNonZero)
}

fn arrayGetIsNegative(mut idx: i32, mut arrayIn: metamodelica::Array<i32>) -> Result<bool> {
    let mut isNonZero: bool;
    isNonZero = metamodelica::arrayGet(arrayIn.clone(), idx)? < 0;
    Ok(isNonZero)
}

fn getArrayEntryAndAppend(mut entry: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut lstIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut lstOut: Arc<metamodelica::List<i32>>;
    let mut lst: Arc<metamodelica::List<i32>>;
    lst = metamodelica::arrayGet(m.clone(), entry)?;
    lstOut = listAppend(lst, lstIn);
    Ok(lstOut)
}

fn gatherCrossNodes(mut idx: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut lstIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut lstOut: Arc<metamodelica::List<i32>>;
    let mut isCross: bool;
    let mut num: i32;
    let mut row: Arc<metamodelica::List<i32>>;
    row = metamodelica::arrayGet(m.clone(), idx)?;
    num = (row.len() as i32);
    isCross = intGt(num, 2);
    lstOut = if (isCross) {metamodelica::cons(idx, lstIn)} else {lstIn};
    Ok(lstOut)
}

fn isAddOrSubExp(mut inExp: Arc<DAE::Exp>, mut inTuple: (bool, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (bool, BackendDAE::Variables))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTuple: (bool, BackendDAE::Variables);
    (outExp, outTuple) = (::match_deref::match_deref! { match &((inExp.clone(), inTuple.clone())) {
        (Deref @ DAE::Exp::CREF { .. }, (true, vars)) => {
            (inExp, (true, vars.clone()))
        },
        (Deref @ DAE::Exp::UNARY { exp: exp1, .. }, (true, vars)) => {
            let mut b: bool;
            let (_, (__pa0, _)) = isAddOrSubExp(exp1.clone(), (true, vars.clone()))?;
            b = __pa0.clone();
            (inExp, (b, vars.clone()))
        },
        (Deref @ DAE::Exp::RCONST { .. }, (true, vars)) => {
            (inExp, (true, vars.clone()))
        },
        (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD { .. }, exp2 }, (true, vars)) => {
            let mut b: bool;
            let (_, (__pa0, _)) = isAddOrSubExp(exp1.clone(), (true, vars.clone()))?;
            b = __pa0.clone();
            let (_, (__pa1, _)) = isAddOrSubExp(exp2.clone(), (b, vars.clone()))?;
            b = __pa1.clone();
            (inExp, (b, vars.clone()))
        },
        (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::SUB { .. }, exp2 }, (true, vars)) => {
            let mut b: bool;
            let (_, (__pa0, _)) = isAddOrSubExp(exp1.clone(), (true, vars.clone()))?;
            b = __pa0.clone();
            let (_, (__pa1, _)) = isAddOrSubExp(exp2.clone(), (b, vars.clone()))?;
            b = __pa1.clone();
            (inExp, (b, vars.clone()))
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { componentRef: cref, .. }, operator: DAE::Operator::MUL { .. }, exp2 }, (true, vars)) => {
            let mut b: bool;
            b = BackendVariable::isState(cref.clone(), vars.clone()) && Expression::isConst(exp2.clone())?;
            (inExp, (b, vars.clone()))
        },
        (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CREF { componentRef: cref, .. } }, (true, vars)) => {
            let mut b: bool;
            b = Expression::isConst(exp1.clone())? && BackendVariable::isState(cref.clone(), vars.clone());
            (inExp, (b, vars.clone()))
        },
        _ => {
            (inExp, (false, Util::tuple22(inTuple)))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTuple))
}

fn sumUp2Expressions(mut sumUp: bool, mut exp1: Arc<DAE::Exp>, mut exp2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut expOut: Arc<DAE::Exp>;
    let mut op: DAE::Operator;
    let mut ty: Arc<DAE::Type>;
    ty = DAE::T_REAL_DEFAULT().clone();
    op = if (sumUp) {DAE::Operator::ADD { ty: ty }} else {DAE::Operator::SUB { ty: ty }};
    expOut = Arc::new(DAE::Exp::BINARY { exp1: exp1, operator: op, exp2: exp2 });
    (expOut, _) = ExpressionSimplify::simplify(expOut)?;
    Ok(expOut)
}

fn intLstIsEqual(mut lst1: Arc<metamodelica::List<i32>>, mut lst2: Arc<metamodelica::List<i32>>) -> Result<bool> {
    let mut bOut: bool;
    bOut = List::isEqualOnTrue(lst1, lst2, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    Ok(bOut)
}

fn sortPathsAsChain(mut pathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut pathsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    pathsOut = 'mc: {
        let __mc_input = pathsIn.clone();
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
                _ => {
                    let mut pathLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    pathLst = sortPathsAsChain1(pathsIn.clone(), 0, 0, metamodelica::nil())?;
                    Ok(pathLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(pathsIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    pathsOut
}

fn sortPathsAsChain1(mut pathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut firstNode: i32, mut lastNode: i32, mut sortedPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut sortedPathsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    sortedPathsOut = 'mc: {
        let __mc_input = (pathsIn.clone(), firstNode, lastNode, sortedPathsIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok(sortedPathsIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (-1), (-1), _) => {
                    Ok(sortedPathsIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: path, tail: rest }, _, _, Deref @ metamodelica::List::Nil) => {
                    let mut startNode: i32;
                    let mut endNode: i32;
                    let mut sortedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    startNode = listHead(path.clone())?;
                    endNode = List::last(path.clone())?;
                    sortedPaths = sortPathsAsChain1(rest.clone(), startNode.clone(), endNode.clone(), list![path.clone()])?;
                    Ok(sortedPaths.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut endNode: i32;
                    let mut path: Arc<metamodelica::List<i32>>;
                    let mut rest: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut paths1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut paths2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut allPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut sortedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    paths1 = List::filter1OnTrue(pathsIn.clone(), (std::sync::Arc::new(firstInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), lastNode)?;
                    paths2 = List::filter1OnTrue(pathsIn.clone(), (std::sync::Arc::new(lastInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), lastNode)?;
                    allPaths = listAppend(paths1.clone(), paths2.clone());
                    let false = (allPaths.clone().is_empty()) else { bail!("pattern mismatch") };
                    path = listHead(allPaths.clone())?;
                    endNode = if (!(allPaths.clone().is_empty())) {List::last(path.clone())?} else {-1};
                    endNode = if (!(paths2.clone().is_empty())) {listHead(path.clone())?} else {-1};
                    (rest, _) = List::deleteMemberOnTrue(path.clone(), pathsIn.clone(), (std::sync::Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
                    sortedPaths = listAppend(sortedPathsIn.clone(), list![path.clone()]);
                    sortedPaths = sortPathsAsChain1(rest.clone(), firstNode, endNode.clone(), sortedPaths.clone())?;
                    Ok(sortedPaths.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut startNode: i32;
                    let mut path: Arc<metamodelica::List<i32>>;
                    let mut rest: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut paths1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut paths2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut allPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut sortedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    paths1 = List::filter1OnTrue(pathsIn.clone(), (std::sync::Arc::new(firstInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), firstNode)?;
                    paths2 = List::filter1OnTrue(pathsIn.clone(), (std::sync::Arc::new(lastInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), firstNode)?;
                    allPaths = listAppend(paths1.clone(), paths2.clone());
                    let false = (allPaths.clone().is_empty()) else { bail!("pattern mismatch") };
                    path = listHead(allPaths.clone())?;
                    startNode = if (!(allPaths.clone().is_empty())) {List::last(path.clone())?} else {-1};
                    startNode = if (!(paths2.clone().is_empty())) {listHead(path.clone())?} else {-1};
                    (rest, _) = List::deleteMemberOnTrue(path.clone(), pathsIn.clone(), (std::sync::Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
                    sortedPaths = metamodelica::cons(path.clone(), sortedPathsIn.clone());
                    sortedPaths = sortPathsAsChain1(rest.clone(), startNode.clone(), lastNode, sortedPaths.clone())?;
                    Ok(sortedPaths.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut startNode: i32;
                    let mut path: Arc<metamodelica::List<i32>>;
                    let mut rest: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut sortedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(pathsIn.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    path = __pa0.clone();
                    rest = __pa1.clone();
                    sortedPaths = metamodelica::cons(path.clone(), sortedPathsIn.clone());
                    startNode = listHead(path.clone())?;
                    sortedPaths = sortPathsAsChain1(rest.clone(), startNode.clone(), lastNode, sortedPaths.clone())?;
                    Ok(sortedPaths.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(sortedPathsOut)
}

fn firstInListIsEqual(mut lstIn: Arc<metamodelica::List<i32>>, mut value: i32) -> Result<bool> {
    let mut isEq: bool;
    let mut first: i32;
    first = listHead(lstIn)?;
    isEq = intEq(first, value);
    Ok(isEq)
}

fn lastInListIsEqual(mut lstIn: Arc<metamodelica::List<i32>>, mut value: i32) -> Result<bool> {
    let mut isEq: bool;
    let mut last: i32;
    last = List::last(lstIn)?;
    isEq = intEq(last, value);
    Ok(isEq)
}

fn connect2PathsToLoops(mut pathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut loopsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut restPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut pathsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut restPathsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    (pathsOut, restPathsOut) = 'mc: {
        let __mc_input = pathsIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: path, tail: Deref @ metamodelica::List::Nil } => {
                    let mut closedALoop: bool;
                    let mut startNode: i32;
                    let mut endNode: i32;
                    let mut loops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut restPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    startNode = listHead(path.clone())?;
                    endNode = List::last(path.clone())?;
                    closedALoop = intEq(startNode.clone(), endNode.clone());
                    loops = if (closedALoop.clone()) {metamodelica::cons(path.clone(), loopsIn.clone())} else {loopsIn.clone()};
                    restPaths = if (closedALoop.clone()) {restPathsIn.clone()} else {metamodelica::cons(path.clone(), restPathsIn.clone())};
                    Ok((loops.clone(), restPaths.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: path, tail: rest } => {
                    let mut startNode: i32;
                    let mut endNode: i32;
                    let mut loops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut restPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    startNode = listHead(path.clone())?;
                    endNode = List::last(path.clone())?;
                    let true = (intEq(startNode.clone(), endNode.clone())) else { bail!("pattern mismatch") };
                    loops = metamodelica::cons(path.clone(), loopsIn.clone());
                    (loops, restPaths) = connect2PathsToLoops(rest.clone(), loops.clone(), restPathsIn.clone())?;
                    Ok((loops.clone(), restPaths.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: path, tail: rest } => {
                    let mut closedALoop: bool;
                    let mut startNode: i32;
                    let mut endNode: i32;
                    let mut endPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut startPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut newLoops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut loops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut restPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    startNode = listHead(path.clone())?;
                    endNode = List::last(path.clone())?;
                    startPaths = List::filter1OnTrue(rest.clone(), (std::sync::Arc::new(firstInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), startNode.clone())?;
                    startPaths = List::filter1OnTrue(startPaths.clone(), (std::sync::Arc::new(lastInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), endNode.clone())?;
                    endPaths = List::filter1OnTrue(rest.clone(), (std::sync::Arc::new(firstInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), endNode.clone())?;
                    endPaths = List::filter1OnTrue(endPaths.clone(), (std::sync::Arc::new(lastInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), startNode.clone())?;
                    endPaths = listAppend(startPaths.clone(), endPaths.clone());
                    closedALoop = !(endPaths.clone().is_empty());
                    newLoops = if (closedALoop.clone()) {connectPaths(path.clone(), endPaths.clone())?} else {metamodelica::nil()};
                    restPaths = if (closedALoop.clone()) {restPathsIn.clone()} else {metamodelica::cons(path.clone(), restPathsIn.clone())};
                    loops = listAppend(newLoops.clone(), loopsIn.clone());
                    (loops, restPaths) = connect2PathsToLoops(rest.clone(), loops.clone(), restPaths.clone())?;
                    Ok((loops.clone(), restPaths.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("connect2PathsToLoops failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((pathsOut, restPathsOut))
}

fn connectPaths(mut pathIn: Arc<metamodelica::List<i32>>, mut closingPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut loopsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut path: Arc<metamodelica::List<i32>>;
    let __pa0 = ::match_deref::match_deref! { match &(pathIn) {
        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    path = __pa0.clone();
    path = List::stripLast(path)?;
    loopsOut = List::map1(closingPaths, Arc::new(fnptr!(listAppend, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)), path)?;
    Ok(loopsOut)
}

//____________________________________________________
//reshuffle systems of equations, not yet finished
//____________________________________________________
pub(crate) fn reshuffling_post(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    if Flags::isSet(Flags::RESHUFFLE_POST.clone())? {
        eqSystems = List::map1(inDAE.eqs.clone(), (std::sync::Arc::new(reshuffling_post0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), inDAE.shared.clone())?;
        outDAE = Arc::new(BackendDAE::BackendDAE { eqs: eqSystems, shared: inDAE.shared.clone() });
    } else {
        outDAE = inDAE;
    }
    Ok(outDAE)
}

fn reshuffling_post0(mut isyst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    osyst = List::fold1(comps, (std::sync::Arc::new(fnptr!(reshuffling_post1, Arc<BackendDAE::StrongComponent>, Arc<BackendDAE::Shared>, Arc<BackendDAE::EqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, Arc<BackendDAE::Shared>, Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), shared, isyst)?;
    Ok(osyst)
}

fn reshuffling_post1(mut compIn: Arc<BackendDAE::StrongComponent>, mut shared: Arc<BackendDAE::Shared>, mut systIn: Arc<BackendDAE::EqSystem>) -> Arc<BackendDAE::EqSystem> {
    let mut systOut: Arc<BackendDAE::EqSystem>;
    systOut = 'mc: {
        let __mc_input = compIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqIdcs, vars: vIdcs, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: ojac }, jacType: jacType @ BackendDAE::JacobianType::JAC_LINEAR { .. }, .. } => {
                    let mut eqSys: Arc<BackendDAE::EqSystem>;
                    (eqSys, _) = reshuffling_post2(eqIdcs.clone(), vIdcs.clone(), systIn.clone(), shared.clone(), ojac.clone(), jacType.clone())?;
                    Ok(eqSys.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(systIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    systOut
}

fn reshuffling_post2(mut eqIdcs: Arc<metamodelica::List<i32>>, mut varIdcs: Arc<metamodelica::List<i32>>, mut dae: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut ojac: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>>, mut jacType: BackendDAE::JacobianType) -> Result<(Arc<BackendDAE::EqSystem>, bool)> {
    let mut daeOut: Arc<BackendDAE::EqSystem>;
    let mut outRunMatching: bool;
    let mut size: i32;
    let mut resEqs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut ass1Sys: metamodelica::Array<i32>;
    let mut ass2Sys: metamodelica::Array<i32>;
    let mut varAtts: Arc<metamodelica::List<(bool, ArcStr)>>;
    let mut eqAtts: Arc<metamodelica::List<(bool, ArcStr)>>;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut daeEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut daeVars: BackendDAE::Variables;
    let mut subSys: Arc<BackendDAE::EqSystem>;
    let mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut funcs: Arc<AvlTreePathFunction::Tree>;
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut eqsInLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
    size = (varIdcs.clone().len() as i32);
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(dae.clone()) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, matching: Deref @ BackendDAE::Matching::MATCHING { ass1: __pa2, ass2: __pa3, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    daeVars = __pa0.clone();
    daeEqs = __pa1.clone();
    ass1Sys = __pa2.clone();
    ass2Sys = __pa3.clone();
    funcs = BackendDAEUtil::getFunctions(shared.clone())?;
    eqLst = BackendEquation::getList(eqIdcs.clone(), daeEqs.clone())?;
    eqs = BackendEquation::listEquation(eqLst.clone())?;
    varLst = List::map1r(varIdcs.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), daeVars)?;
    vars = BackendVariable::listVar1(varLst.clone())?;
    subSys = BackendDAEUtil::createEqSystem(vars.clone(), eqs.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    (me, meT, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(subSys.clone(), shared.clone(), false)?;
    (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(subSys, openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(BackendDAEUtil::getFunctions(shared.clone())?), BackendDAEUtil::isInitializationDAE(shared.clone()))?;
    ass1 = arrayCreate(size, -1);
    ass2 = arrayCreate(size, -1);
    varAtts = List::threadMap(List::fill(false, (varLst.clone().len() as i32)), List::map(eqIdcs.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
    eqAtts = List::threadMap(List::fill(false, (eqLst.clone().len() as i32)), List::map(varIdcs, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
    BackendDump::dumpBipartiteGraphStrongComponent2(vars, eqs, m.clone(), varAtts, eqAtts, (literal!("shuffle_pre")).clone())?;
    resEqs = reshuffling_post3_selectShuffleEqs(me.clone(), meT.clone());
    eqsInLst = reshuffling_post4_resolveAndReplace(resEqs, eqLst, varLst, me.clone(), meT.clone())?;
    daeEqs = List::threadFold(eqIdcs, eqsInLst, (std::sync::Arc::new(BackendEquation::setAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<BackendDAE::Equation>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> + 'static>), daeEqs)?;
    daeOut = BackendDAEUtil::setEqSystEqs(dae, daeEqs);
    daeOut = BackendDAEUtil::setEqSystMatching(daeOut, Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1Sys.clone(), ass2: ass2Sys.clone(), comps: metamodelica::nil() }))?;
    (daeOut, _, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(daeOut, openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(funcs), BackendDAEUtil::isInitializationDAE(shared))?;
    outRunMatching = true;
    Ok((daeOut, outRunMatching))
}

fn reshuffling_post3_selectShuffleEqs(mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut resolveEqs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    resolveEqs = 'mc: {
        let __mc_input = meT.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut bArr: metamodelica::Array<bool>;
            let mut suitableEqs: Arc<metamodelica::List<i32>>;
            let mut eqPairs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            bArr = Array::map1(me.clone(), (std::sync::Arc::new(chooseEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<bool> + 'static>), meT.clone())?;
            (_, suitableEqs) = List::filter1OnTrueSync(Arc::new(bArr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), (std::sync::Arc::new(fnptr!(boolEq, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), true, List::intRange(metamodelica::arrayLength(me.clone())))?;
            eqPairs = List::map2(suitableEqs.clone(), (std::sync::Arc::new(getEqPairs) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), me.clone(), meT.clone())?;
            eqPairs = List::filterOnTrue(eqPairs.clone(), std::sync::Arc::new(fnptr!(List::hasSeveralElements, _)))?;
            Ok(eqPairs.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("reshuffling_post3_selectShuffleEqs failed!\n")).clone());
            Ok(metamodelica::nil())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    resolveEqs
}

fn reshuffling_post4_resolveAndReplace(mut resolveEqLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut unassEqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut unassVarsIn: Arc<metamodelica::List<BackendDAE::Var>>, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut unassEqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    unassEqsOut = 'mc: {
        let __mc_input = resolveEqLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(unassEqsIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: resolveEqs, tail: rest } => {
                    let mut maxNum: i32;
                    let mut replEqIdx: i32;
                    let mut numOfAdjVars: Arc<metamodelica::List<i32>>;
                    let mut unassEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut resolvedEq: Arc<BackendDAE::Equation>;
                    resolvedEq = resolveEquations(None, resolveEqs.clone(), me.clone(), meT.clone(), unassEqsIn.clone(), unassVarsIn.clone())?;
                    numOfAdjVars = List::map(List::map1(resolveEqs.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), me.clone())?, std::sync::Arc::new(fnptr!(listLength, _)))?;
                    maxNum = List::fold(numOfAdjVars.clone(), (std::sync::Arc::new(fnptr!(intMax, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), listHead(numOfAdjVars.clone())?)?;
                    replEqIdx = (resolveEqs.clone()).get(List::position(maxNum.clone(), numOfAdjVars.clone())?)?;
                    unassEqs = List::replaceAt(resolvedEq.clone(), replEqIdx.clone(), unassEqsIn.clone())?;
                    Ok(reshuffling_post4_resolveAndReplace(rest.clone(), unassEqs.clone(), unassVarsIn.clone(), me.clone(), meT.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("reshuffling_post4_resolveAndReplace failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(unassEqsOut)
}

fn getEqPairs(mut eq: i32, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut lstOut: Arc<metamodelica::List<i32>>;
    let mut vars: Arc<metamodelica::List<i32>>;
    let mut eqs: Arc<metamodelica::List<i32>>;
    vars = List::map(metamodelica::arrayGet(me.clone(), eq)?, std::sync::Arc::new(fnptr!(Util::tuple31, _)))?;
    eqs = List::map(List::flatten(List::map1(vars, (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), meT.clone())?)?, std::sync::Arc::new(fnptr!(Util::tuple31, _)))?;
    eqs = getDoublicates(eqs)?;
    lstOut = List::consOnTrue(!(listMember(eq, eqs.clone())), eq, eqs);
    Ok(lstOut)
}

fn chooseEquation(mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<bool> {
    let mut chooseThis: bool;
    let mut b1: bool;
    let mut b2: bool;
    let mut b3: bool;
    let mut vars: Arc<metamodelica::List<i32>>;
    let mut eqs: Arc<metamodelica::List<i32>>;
    let mut numEqs: Arc<metamodelica::List<i32>>;
    let mut eqLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    vars = List::map(row.clone(), std::sync::Arc::new(fnptr!(Util::tuple31, _)))?;
    b1 = intEq((row.clone().len() as i32), 2);
    eqLst = List::mapList(List::map1(vars, (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), meT.clone())?, std::sync::Arc::new(fnptr!(Util::tuple31, _)))?;
    numEqs = List::map(eqLst.clone(), std::sync::Arc::new(fnptr!(listLength, _)))?;
    b3 = List::applyAndFold1(numEqs, (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 2, false)?;
    eqs = List::flatten(eqLst)?;
    b2 = intEq((eqs.clone().len() as i32), (List::unique(eqs).len() as i32) + 2);
    b1 = b1 && b2 && b3;
    chooseThis = b1 && List::applyAndFold(row, (std::sync::Arc::new(fnptr!(boolAnd, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(isSolvable) as std::sync::Arc<dyn ::std::ops::Fn((i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> + 'static>), true)?;
    Ok(chooseThis)
}

fn getDoublicates(mut lstIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut lstOut: Arc<metamodelica::List<i32>>;
    let mut max: i32;
    let mut arr: metamodelica::Array<i32>;
    max = List::fold(lstIn.clone(), (std::sync::Arc::new(fnptr!(intMax, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), listHead(lstIn.clone())?)?;
    arr = arrayCreate(max, -1);
    List::map1_0(lstIn, (std::sync::Arc::new(getDoublicates2) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<()> + 'static>), arr.clone())?;
    (_, lstOut) = List::filter1OnTrueSync(Arc::new(arr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), (std::sync::Arc::new(fnptr!(intGe, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 1, List::intRange(metamodelica::arrayLength(arr.clone())))?;
    Ok(lstOut)
}

fn getDoublicates2(mut idx: i32, mut arr: metamodelica::Array<i32>) -> Result<()> {
    let mut entry: i32;
    entry = metamodelica::arrayGet(arr.clone(), idx)?;
    metamodelica::arrayUpdate(arr.clone(), idx, entry + 1)?;
    Ok(())
}

fn isSolvable(mut entry: (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> {
    let mut solvable: bool;
    solvable = !(Tearing::unsolvable(list![entry])?);
    Ok(solvable)
}

pub(crate) fn resolveEquations(mut eq: Option<Arc<BackendDAE::Equation>>, mut loopIn: Arc<metamodelica::List<i32>>, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut eqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut varsIn: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<BackendDAE::Equation>> {
    let mut eqOut: Arc<BackendDAE::Equation>;
    eqOut = 'mc: {
        let __mc_input = (eq, loopIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(eq1), Deref @ metamodelica::List::Nil) => {
                    Ok(eq1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, Deref @ metamodelica::List::Cons { head: startEq, tail: rest }) => {
                    let mut nextEq: i32;
                    let mut sharedVar: i32;
                    let mut vars1: Arc<metamodelica::List<i32>>;
                    let mut vars2: Arc<metamodelica::List<i32>>;
                    let mut numEqs: Arc<metamodelica::List<i32>>;
                    let mut eq1: Arc<BackendDAE::Equation>;
                    let mut eq2: Arc<BackendDAE::Equation>;
                    let mut var: BackendDAE::Var;
                    let mut attr: BackendDAE::EquationAttributes;
                    let mut lhs1: Arc<DAE::Exp>;
                    let mut lhs2: Arc<DAE::Exp>;
                    let mut rhs1: Arc<DAE::Exp>;
                    let mut rhs2: Arc<DAE::Exp>;
                    let mut varExp: Arc<DAE::Exp>;
                    let mut eqExp: Arc<DAE::Exp>;
                    let mut source: Arc<DAE::ElementSource>;
                    let mut rest = (*rest).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    nextEq = __pa0.clone();
                    rest = __pa1.clone();
                    vars1 = List::map(metamodelica::arrayGet(me.clone(), startEq.clone())?, std::sync::Arc::new(fnptr!(Util::tuple31, _)))?;
                    vars2 = List::map(metamodelica::arrayGet(me.clone(), nextEq.clone())?, std::sync::Arc::new(fnptr!(Util::tuple31, _)))?;
                    vars1 = List::intersectionOnTrue(vars1.clone(), vars2.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    numEqs = List::map(List::map1(vars1.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), meT.clone())?, std::sync::Arc::new(fnptr!(listLength, _)))?;
                    (_, vars1) = List::filter1OnTrueSync(numEqs.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 2, vars1.clone())?;
                    sharedVar = listHead(vars1.clone())?;
                    eq1 = (eqsIn.clone()).get(startEq.clone())?;
                    eq2 = (eqsIn.clone()).get(nextEq.clone())?;
                    var = (varsIn.clone()).get(sharedVar.clone())?;
                    varExp = Expression::crefExp(BackendVariable::varCref(var.clone())?)?;
                    let (__pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(eq1.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { exp: __pa2, scalar: __pa3, source: __pa4, attr: __pa5 } => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    lhs1 = __pa2.clone();
                    rhs1 = __pa3.clone();
                    source = __pa4.clone();
                    attr = __pa5.clone();
                    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(eq2.clone()) {
                        Deref @ BackendDAE::Equation::EQUATION { exp: __pa6, scalar: __pa7, .. } => (__pa6.clone(), __pa7.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    lhs2 = __pa6.clone();
                    rhs2 = __pa7.clone();
                    (eqExp, _) = ExpressionSolve::solve(lhs1.clone(), rhs1.clone(), varExp.clone(), None)?;
                    (lhs2, _) = Expression::replaceExp(lhs2.clone(), varExp.clone(), eqExp.clone())?;
                    (rhs2, _) = Expression::replaceExp(rhs2.clone(), varExp.clone(), eqExp.clone())?;
                    (lhs2, _) = ExpressionSimplify::simplify(lhs2.clone())?;
                    (rhs2, _) = ExpressionSimplify::simplify(rhs2.clone())?;
                    eq2 = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs2.clone(), scalar: rhs2.clone(), source: source.clone(), attr: attr.clone() });
                    Ok(resolveEquations(Some(eq2.clone()), rest.clone(), me.clone(), meT.clone(), eqsIn.clone(), varsIn.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("resolveEquations failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(eqOut)
}

// =============================================================================
// section for postOptModule >solveLinearSystem<<
//
// solve linear system of equations (A x = b)
// =============================================================================
pub(crate) fn solveLinearSystem(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut maxSize: i32 = Flags::getConfigInt(Flags::MAX_SIZE_FOR_SOLVE_LINIEAR_SYSTEM.clone())?;
    let mut b: bool = 1 < maxSize;
    if b {
        (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE, (std::sync::Arc::new(solveLinearSystem0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (bool, i32, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (bool, i32, i32))> + 'static>), (false, 1, maxSize))?;
    } else {
        outDAE = inDAE;
    }
    Ok(outDAE)
}

fn solveLinearSystem0(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inTpl: (bool, i32, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (bool, i32, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared>;
    let mut outTpl: (bool, i32, i32);
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    (osyst, outShared, outTpl) = solveLinearSystem1(isyst, inShared, comps, inTpl)?;
    Ok((osyst, outShared, outTpl))
}

fn solveLinearSystem1(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut inTpl: (bool, i32, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (bool, i32, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem> = isyst.clone();
    let mut oshared: Arc<BackendDAE::Shared> = ishared.clone();
    let mut outTpl: (bool, i32, i32);
    let mut b: bool;
    let mut runMatching: bool;
    let mut ii: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut offset: i32;
    let mut maxSize: i32;
    (runMatching, offset, maxSize) = inTpl;
    for mut comp in &*inComps {
        let mut comp = comp.clone();
        (osyst, oshared, b, ii, offset) = solveLinearSystem2(osyst.clone(), oshared.clone(), comp.clone(), ii.clone(), offset, maxSize);
        runMatching = runMatching || b;
    }
    outTpl = (runMatching, offset, maxSize);
    if runMatching {
        osyst = (::match_deref::match_deref! { match &(osyst) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. } => {
            let mut syst = (*syst).clone();
            let mut eqns = (*eqns).clone();
            eqns = List::fold(ii, (std::sync::Arc::new(BackendEquation::delete) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> + 'static>), eqns.clone())?;
            assign_field!(
                syst.orderedVars = BackendVariable::listVar1(BackendVariable::varList(vars.clone())?)?,
                syst.orderedEqs = BackendEquation::listEquation(BackendEquation::equationList(eqns.clone())?)?
            );
            BackendDAEUtil::clearEqSyst(syst.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((osyst, oshared, outTpl))
}

fn solveLinearSystem2(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut comp: Arc<BackendDAE::StrongComponent>, mut ii: Arc<metamodelica::List<i32>>, mut offset: i32, mut maxSize: i32) -> (Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, Arc<metamodelica::List<i32>>, i32) {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut outRunMatching: bool;
    let mut oi: Arc<metamodelica::List<i32>>;
    let mut offset_: i32;
    (osyst, oshared, outRunMatching, oi, offset_) = 'mc: {
        let __mc_input = (isyst.clone(), ishared.clone(), comp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. }, shared, Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eindex, vars: vindx, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, jacType: BackendDAE::JacobianType::JAC_LINEAR { .. }, .. }) => {
                    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
                    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut toffset: i32;
                    let mut syst = (*syst).clone();
                    let mut shared = (*shared).clone();
                    eqn_lst = BackendEquation::getList(eindex.clone(), eqns.clone())?;
                    var_lst = List::map1r(vindx.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    let true = ((var_lst.clone().len() as i32) <= maxSize) else { bail!("pattern mismatch") };
                    ::match_deref::match_deref! { match &(List::splitOnTrue(var_lst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isStateVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?) {
                        (Deref @ metamodelica::List::Nil, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (syst, shared, toffset) = solveLinearSystem3(syst.clone(), shared.clone(), eqn_lst.clone(), eindex.clone(), var_lst.clone(), vindx.clone(), jac.clone(), offset)?;
                    Ok((syst.clone(), shared.clone(), true, listAppend(eindex.clone(), ii.clone()), toffset.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), ishared.clone(), false, ii.clone(), offset))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (osyst, oshared, outRunMatching, oi, offset_)
}

fn solveLinearSystem3(mut inSyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut eqn_indxs: Arc<metamodelica::List<i32>>, mut var_lst: Arc<metamodelica::List<BackendDAE::Var>>, mut var_indxs: Arc<metamodelica::List<i32>>, mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut offset: i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> {
    let mut osyst: Arc<BackendDAE::EqSystem>;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut offset_: i32;
    (osyst, oshared, offset_) = (::match_deref::match_deref! { match &((inSyst, ishared)) {
        (syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. }, shared @ Deref @ BackendDAE::Shared { functionTree: funcs, .. }) => {
            let mut beqs: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut names: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut n: i32;
            let mut syst = (*syst).clone();
            let mut vars = (*vars).clone();
            let mut eqns = (*eqns).clone();
            let mut shared = (*shared).clone();
            (beqs, _) = BackendDAEUtil::getEqnSysRhs(BackendEquation::listEquation(eqn_lst)?, BackendVariable::listVar1(var_lst.clone())?, Some(funcs.clone()))?;
            beqs = beqs.reverse();
            n = (beqs.clone().len() as i32);
            names = List::map(var_lst.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
            (eqns, vars, n, shared) = solveLinearSystem4(beqs, jac, names, var_lst, n, eqns.clone(), vars.clone(), offset, shared.clone())?;
            assign_field!(
                syst.orderedVars = vars.clone(),
                syst.orderedEqs = eqns.clone()
            );
            syst = BackendDAEUtil::setEqSystMatrices(syst.clone(), None, None, None)?;
            (syst.clone(), shared.clone(), n)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((osyst, oshared, offset_))
}

fn solveLinearSystem4(mut b_lst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut cr_x: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut var_lst: Arc<metamodelica::List<BackendDAE::Var>>, mut n: i32, mut ieqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ivars: BackendDAE::Variables, mut offset: i32, mut ishared: Arc<BackendDAE::Shared>) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, i32, Arc<BackendDAE::Shared>)> {
    let mut oeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = ieqns.clone();
    let mut ovars: BackendDAE::Variables = ivars.clone();
    let mut offset_: i32 = offset + 1;
    let mut oshared: Arc<BackendDAE::Shared> = ishared.clone();
    let mut R: metamodelica::Array<Arc<DAE::Exp>>;
    let mut Qb: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut b: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut A: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n * n, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut ax: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut scaled_x: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut scaleA: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut a: Arc<DAE::Exp>;
    let mut m: i32;
    let mut ii: i32;
    let mut jj: i32;
    let mut mm: i32;
    let mut x_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = List::map(cr_x.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>))?;
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = var_lst.clone();
    let mut var: BackendDAE::Var;
    let mut eqn: Arc<BackendDAE::Equation>;
    let mut jac_: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>> = jac.clone();
    mm = (jac.len() as i32);
    for mut i in 1..=mm {
        let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(jac_.clone()) {
            Deref @ metamodelica::List::Cons { head: (__pa0, __pa1, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: __pa2, .. }), tail: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        jj = __pa0.clone();
        ii = __pa1.clone();
        a = __pa2.clone();
        jac_ = __pa3.clone();
        m = ii + (jj - 1) * n;
        (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(a.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$A$")); __mm_s.push_str(&*intString(m)); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        metamodelica::arrayUpdate(A.clone(), m, a.clone())?;
    }
    for mut i in 1..=n {
        m = (i.clone() - 1) * n;
        a = Expression::makeSum1(({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut j in (1..=n).into_iter() {
            let __x = Expression::makeAbs(metamodelica::arrayGet(A.clone(), m + j.clone())?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), false)?;
        metamodelica::arrayUpdate(scaleA.clone(), i.clone(), a.clone())?;
    }
    for mut i in 1..=n {
        m = (i.clone() - 1) * n;
        for mut j in 1..=n {
            a = metamodelica::arrayGet(A.clone(), j.clone() + m)?;
            if !(Expression::isZero(a.clone())?) {
                a = Expression::expDiv(a.clone(), metamodelica::arrayGet(scaleA.clone(), j.clone())?)?;
                (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(a.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$sA$")); __mm_s.push_str(&*intString(i.clone() + (j.clone() - 1) * n)); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
                metamodelica::arrayUpdate(A.clone(), j.clone() + m, a.clone())?;
            }
        }
    }
    m = 1;
    for mut b_ in &*b_lst {
        let mut b_ = b_.clone();
        (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(b_.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$b$")); __mm_s.push_str(&*intString(m)); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        metamodelica::arrayUpdate(b.clone(), m, a.clone())?;
        m = m + 1;
    }
    m = 1;
    for mut xx in &*x_lst {
        let mut xx = xx.clone();
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(vars.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        var = __pa4.clone();
        vars = __pa5.clone();
        if BackendVariable::isStateVar(var.clone()) {
            metamodelica::arrayUpdate(ax.clone(), m, Expression::expDer(xx.clone()))?;
        } else {
            metamodelica::arrayUpdate(ax.clone(), m, xx.clone())?;
        }
        m = m + 1;
    }
    for mut i in 1..=n {
        a = Expression::expMul(metamodelica::arrayGet(ax.clone(), i.clone())?, metamodelica::arrayGet(scaleA.clone(), i.clone())?)?;
        (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(a.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$sx$")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        metamodelica::arrayUpdate(scaled_x.clone(), i.clone(), a.clone())?;
    }
    (R, Qb, oeqns, ovars, oshared) = qrDecompositionHouseholder(A.clone(), n, b.clone(), oeqns, ovars, offset, oshared)?;
    for mut i in ({let __s=n; let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        m = (i.clone() - 1) * n;
        a = Expression::makeSum1(({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut j in (i.clone()..=n).into_iter() {
            let __x = Expression::expMul(metamodelica::arrayGet(R.clone(), m + j.clone())?, metamodelica::arrayGet(scaled_x.clone(), j.clone())?)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), false)?;
        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: a.clone(), scalar: metamodelica::arrayGet(Qb.clone(), i.clone())?, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() });
        eqn = BackendEquation::solveEquation(eqn.clone(), metamodelica::arrayGet(scaled_x.clone(), i.clone())?, None)?;
        oeqns = BackendEquation::add(eqn.clone(), oeqns.clone())?;
    }
    Ok((oeqns, ovars, offset_, oshared))
}

fn qrDecompositionHouseholder(mut A: metamodelica::Array<Arc<DAE::Exp>>, mut n: i32, mut ib: metamodelica::Array<Arc<DAE::Exp>>, mut ieqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ivars: BackendDAE::Variables, mut offset: i32, mut ishared: Arc<BackendDAE::Shared>) -> Result<(metamodelica::Array<Arc<DAE::Exp>>, metamodelica::Array<Arc<DAE::Exp>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, Arc<BackendDAE::Shared>)> {
    let mut R: metamodelica::Array<Arc<DAE::Exp>> = A.clone();
    let mut b: metamodelica::Array<Arc<DAE::Exp>> = ib.clone();
    let mut oeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = ieqns.clone();
    let mut ovars: BackendDAE::Variables = ivars.clone();
    let mut oshared: Arc<BackendDAE::Shared> = ishared.clone();
    let mut cA: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut v: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut alpha: Arc<DAE::Exp>;
    let mut y1: Arc<DAE::Exp>;
    let mut h: Arc<DAE::Exp>;
    let mut h2: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp>;
    let mut e2: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp>;
    let mut m: i32;
    let mut nn: i32 = n - 1;
    let mut idxVars: i32 = 1;
    let mut shift: i32;
    for mut iter in 1..=nn {
        m = n - iter.clone() + 1;
        qrGet_cA(A.clone(), iter.clone(), 1, n, v.clone())?;
        y1 = metamodelica::arrayGet(v.clone(), 1)?;
        alpha = qrCalc_alpha(v.clone(), y1.clone(), m)?;
        (alpha, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(alpha.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$a$")); __mm_s.push_str(&*intString(iter.clone())); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        e = Expression::expAdd(y1.clone(), alpha.clone())?;
        (e, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(e.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$y1$")); __mm_s.push_str(&*intString(iter.clone())); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        metamodelica::arrayUpdate(v.clone(), 1, e.clone())?;
        h = Expression::expAdd(y1.clone(), alpha.clone())?;
        h = Expression::expMul(alpha.clone(), h.clone())?;
        h = Expression::negate(h.clone())?;
        (h, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(h.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$h$")); __mm_s.push_str(&*intString(iter.clone())); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        shift = (iter.clone() - 1) * n + iter.clone();
        metamodelica::arrayUpdate(R.clone(), shift, Expression::negate(alpha.clone())?)?;
        for mut j in 2..=m {
            metamodelica::arrayUpdate(R.clone(), shift + (j.clone() - 1) * n, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
        }
        for mut col in 2..=m {
            qrGet_cA(A.clone(), iter.clone(), col.clone(), n, cA.clone())?;
            h2 = Expression::makeScalarProduct(v.clone(), cA.clone())?;
            h2 = Expression::expDiv(h2.clone(), h.clone())?;
            (h2, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(h2.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$h2$")); __mm_s.push_str(&*intString(idxVars)); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
            idxVars = idxVars + 1;
            for mut j in 1..=m {
                e1 = metamodelica::arrayGet(cA.clone(), j.clone())?;
                e2 = metamodelica::arrayGet(v.clone(), j.clone())?;
                e = Expression::expAdd(e1.clone(), Expression::expMul(h2.clone(), e2.clone())?)?;
                (e, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(e.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$R$")); __mm_s.push_str(&*intString(idxVars)); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
                idxVars = idxVars + 1;
                metamodelica::arrayUpdate(A.clone(), shift + (j.clone() - 1) * n + col.clone() - 1, e.clone())?;
            }
        }
        for mut j in 1..=m {
            metamodelica::arrayUpdate(cA.clone(), j.clone(), metamodelica::arrayGet(b.clone(), iter.clone() - 1 + j.clone())?)?;
        }
        h2 = Expression::makeScalarProduct(v.clone(), cA.clone())?;
        h2 = Expression::expDiv(h2.clone(), h.clone())?;
        (h2, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(h2.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$b_$")); __mm_s.push_str(&*intString(idxVars)); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        idxVars = idxVars + 1;
        for mut j in 1..=m {
            e1 = metamodelica::arrayGet(cA.clone(), j.clone())?;
            e2 = metamodelica::arrayGet(v.clone(), j.clone())?;
            e = Expression::expAdd(e1.clone(), Expression::expMul(h2.clone(), e2.clone())?)?;
            e = Expression::expand(e.clone())?;
            e = ExpressionSimplify::simplify2(e.clone(), true, true)?;
            (e, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(e.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$b$")); __mm_s.push_str(&*intString(idxVars)); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
            idxVars = idxVars + 1;
            metamodelica::arrayUpdate(b.clone(), iter.clone() - 1 + j.clone(), e.clone())?;
        }
    }
    Ok((R, b, oeqns, ovars, oshared))
}

fn qrGet_cA(mut A: metamodelica::Array<Arc<DAE::Exp>>, mut iter: i32, mut j: i32, mut n: i32, mut cA: metamodelica::Array<Arc<DAE::Exp>>) -> Result<()> {
    let mut shift: i32 = (iter - 1) * n + iter + j - 1;
    let mut m: i32 = n - iter + 1;
    for mut i in 1..=m {
        metamodelica::arrayUpdate(cA.clone(), i.clone(), metamodelica::arrayGet(A.clone(), shift + (i.clone() - 1) * n)?)?;
    }
    for mut i in m + 1..=n {
        metamodelica::arrayUpdate(cA.clone(), i.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))?;
    }
    Ok(())
}

fn qrCalc_alpha(mut y: metamodelica::Array<Arc<DAE::Exp>>, mut y1: Arc<DAE::Exp>, mut m: i32) -> Result<Arc<DAE::Exp>> {
    let mut alpha: Arc<DAE::Exp>;
    let mut sgn_y1: Arc<DAE::Exp> = Expression::makeSign(y1.clone());
    let mut norm_y: Arc<DAE::Exp> = Expression::lenVec(y.clone())?;
    norm_y = Expression::makeSum1(({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut j in (1..=m).into_iter() {
            let __x = Expression::expPow(metamodelica::arrayGet(y.clone(), j.clone())?, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), false)?;
    norm_y = Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![norm_y], DAE::T_REAL_DEFAULT().clone());
    alpha = Expression::expMul(sgn_y1, norm_y)?;
    Ok(alpha)
}

fn qrDecomposition(mut A: metamodelica::Array<Arc<DAE::Exp>>, mut n: i32, mut ib: metamodelica::Array<Arc<DAE::Exp>>, mut ieqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ivars: BackendDAE::Variables, mut offset: i32, mut ishared: Arc<BackendDAE::Shared>) -> Result<(metamodelica::Array<Arc<DAE::Exp>>, metamodelica::Array<Arc<DAE::Exp>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, Arc<BackendDAE::Shared>)> {
    let mut R: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n * n, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut b: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut oeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = ieqns.clone();
    let mut ovars: BackendDAE::Variables = ivars.clone();
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut Q: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n * n, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut v: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut u: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut x: metamodelica::Array<Arc<DAE::Exp>>;
    let mut y: metamodelica::Array<Arc<DAE::Exp>>;
    let mut a: Arc<DAE::Exp>;
    let mut kk: i32 = 1;
    let mut m: i32 = n - 1;
    let mut nn: i32;
    v = qrDecomposition1(A.clone(), n, kk)?;
    (u, oeqns, ovars, oshared) = BackendEquation::normalizationVec(v.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$NOM$")); __mm_s.push_str(&*intString(kk)); ArcStr::from(__mm_s) }).clone(), offset, oeqns, ovars, ishared)?;
    for mut j in 1..=n {
        (a, _) = ExpressionSimplify::simplify(metamodelica::arrayGet(u.clone(), j.clone())?)?;
        (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(a.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$Q$")); __mm_s.push_str(&*intString(kk + (j.clone() - 1) * n)); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        metamodelica::arrayUpdate(Q.clone(), kk + (j.clone() - 1) * n, a.clone())?;
    }
    for mut k in 1..=m {
        v = qrDecomposition1(A.clone(), n, k.clone() + 1)?;
        for mut j in 1..=k.clone() {
            u = qrDecomposition1(Q.clone(), n, j.clone())?;
            (v, oeqns, ovars, oshared) = gramSchmidtProcessHelper(v.clone(), u.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$W$")); __mm_s.push_str(&*intString(kk)); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*intString(kk)); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone())?;
            kk = kk + 1;
        }
        (u, oeqns, ovars, oshared) = BackendEquation::normalizationVec(v.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$NOM$")); __mm_s.push_str(&*intString(k.clone() + 1)); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone())?;
        for mut j in 1..=n {
            nn = k.clone() + 1 + (j.clone() - 1) * n;
            (a, _) = ExpressionSimplify::simplify(metamodelica::arrayGet(u.clone(), j.clone())?)?;
            (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(a.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$Q$")); __mm_s.push_str(&*intString(nn)); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
            metamodelica::arrayUpdate(Q.clone(), nn, a.clone())?;
        }
    }
    for mut i in 1..=n {
        x = qrDecomposition1(Q.clone(), n, i.clone())?;
        m = (i.clone() - 1) * n;
        for mut j in i.clone()..=n {
            y = qrDecomposition1(A.clone(), n, j.clone())?;
            a = Expression::makeScalarProduct(x.clone(), y.clone())?;
            (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(a.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$R$")); __mm_s.push_str(&*intString(m + j.clone())); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
            metamodelica::arrayUpdate(R.clone(), m + j.clone(), a.clone())?;
        }
    }
    for mut i in 1..=n {
        x = qrDecomposition1(Q.clone(), n, i.clone())?;
        a = Expression::makeScalarProduct(x.clone(), ib.clone())?;
        (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(a.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$Qb$")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        metamodelica::arrayUpdate(b.clone(), i.clone(), a.clone())?;
    }
    Ok((R, b, oeqns, ovars, oshared))
}

fn qrDecomposition1(mut A: metamodelica::Array<Arc<DAE::Exp>>, mut sizeA: i32, mut i: i32) -> Result<metamodelica::Array<Arc<DAE::Exp>>> {
    let mut column: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(sizeA, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    for mut j in 1..=sizeA {
        metamodelica::arrayUpdate(column.clone(), j.clone(), metamodelica::arrayGet(A.clone(), i + (j.clone() - 1) * sizeA)?)?;
    }
    Ok(column)
}

fn qrDecomposition2(mut A: metamodelica::Array<Arc<DAE::Exp>>, mut sizeA: i32, mut i: i32) -> Result<metamodelica::Array<Arc<DAE::Exp>>> {
    let mut row: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(sizeA, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut k: i32 = i - 1;
    for mut j in 1..=sizeA {
        metamodelica::arrayUpdate(row.clone(), j.clone(), metamodelica::arrayGet(A.clone(), j.clone() + k * sizeA)?)?;
    }
    Ok(row)
}

fn qrDecomposition3(mut A: metamodelica::Array<Arc<DAE::Exp>>, mut sizeA: i32, mut isMat: bool, mut s: ArcStr) -> Result<()> {
    let mut n: i32 = sizeA;
    let mut m: i32 = if (isMat) {sizeA} else {1};
    metamodelica::print((literal!("\n")).clone());
    for mut i in 1..=n {
        metamodelica::print((literal!("\n")).clone());
        for mut j in 1..=m {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(j.clone())); __mm_s.push_str(&*literal!(") = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(metamodelica::arrayGet(A.clone(), (i.clone() - 1) * m + j.clone())?)?); __mm_s.push_str(&*literal!("\t")); ArcStr::from(__mm_s) }).clone());
        }
    }
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

fn gramSchmidtProcessHelper(mut w: metamodelica::Array<Arc<DAE::Exp>>, mut u: metamodelica::Array<Arc<DAE::Exp>>, mut name: ArcStr, mut offset: i32, mut ieqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ivars: BackendDAE::Variables, mut ishared: Arc<BackendDAE::Shared>) -> Result<(metamodelica::Array<Arc<DAE::Exp>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, Arc<BackendDAE::Shared>)> {
    let mut v: metamodelica::Array<Arc<DAE::Exp>>;
    let mut oeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut ovars: BackendDAE::Variables;
    let mut oshared: Arc<BackendDAE::Shared>;
    let mut h: Arc<DAE::Exp> = Expression::makeScalarProduct(w.clone(), u.clone())?;
    let mut n: i32 = metamodelica::arrayLength(w.clone());
    (h, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(h, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("_h")); ArcStr::from(__mm_s) }).clone(), offset, ieqns, ivars, ishared, false)?;
    v = Array::map1(u.clone(), (std::sync::Arc::new(Expression::expMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), h.clone())?;
    v = Expression::subVec(w.clone(), v.clone())?;
    for mut i in 1..=n {
        (h, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(metamodelica::arrayGet(v.clone(), i.clone())?, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), offset, oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        metamodelica::arrayUpdate(v.clone(), i.clone(), h.clone())?;
    }
    Ok((v, oeqns, ovars, oshared))
}

