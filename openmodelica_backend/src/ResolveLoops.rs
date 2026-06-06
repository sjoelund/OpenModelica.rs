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

pub fn resolveLoops(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut eqSysts: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    (eqSysts, shared, _) = List::mapFold2(inDAE.eqs.clone(), (std::sync::Arc::new(resolveLoops_main) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> + 'static>), inDAE.shared.clone(), 1)?;
    outDAE = Arc::new(BackendDAE::BackendDAE { eqs: eqSysts.clone(), shared: shared.clone() });
    Ok(outDAE)
}

fn resolveLoops_main(mut inEqSys: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inSysIdx: i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> {
    let mut outEqSys: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut outSysIdx: i32 = 0;
    (outEqSys, outSysIdx) = 'mc: {
        let __mc_input = inEqSys.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqs, .. } => {
                    let mut numSimpEqs: i32 = 0;
                    let mut numVars: i32 = 0;
                    let mut eqMapArr: metamodelica::Array<i32> = Default::default();
                    let mut varMapArr: metamodelica::Array<i32> = Default::default();
                    let mut nonLoopEqMark: metamodelica::Array<i32> = Default::default();
                    let mut markLinEqVars: metamodelica::Array<i32> = Default::default();
                    let mut eqMapping: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut partitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut varAtts: Arc<metamodelica::List<(bool, ArcStr)>> = metamodelica::nil();
                    let mut eqAtts: Arc<metamodelica::List<(bool, ArcStr)>> = metamodelica::nil();
                    let mut simpVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
                    let mut simpEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
                    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
                    let mut m_cut: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
                    let mut mT_cut: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
                    let mut m_after: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
                    let mut simpEqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut simpVarLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut syst = (*syst).clone();
                    let mut eqs = (*eqs).clone();
                    (m, _) = BackendDAEUtil::adjacencyMatrix(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
                    if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? {
                        BackendDump::dumpBipartiteGraphEqSystem(syst.clone(), inShared.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("whole System_")); __mm_s.push_str(&*intString(inSysIdx.clone())); ArcStr::from(__mm_s) }).clone())?;
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
                        BackendDump::dumpBipartiteGraphStrongComponent2(simpVars.clone(), simpEqs.clone(), m.clone(), varAtts.clone(), eqAtts.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rL_simpEqs_")); __mm_s.push_str(&*intString(inSysIdx.clone())); ArcStr::from(__mm_s) }).clone())?;
                    }
                    partitions = partitionBipartiteGraph(m.clone(), mT.clone())?;
                    partitions = List::filterOnTrue(partitions.clone(), std::sync::Arc::new(fnptr!(List::hasSeveralElements, _)))?;
                    m_cut = metamodelica::arrayFromVec(m.clone().borrow().clone());
                    mT_cut = metamodelica::arrayFromVec(mT.clone().borrow().clone());
                    (_, nonLoopEqMark) = resolveLoops_cutNodes(m_cut.clone(), mT_cut.clone())?;
                    if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? {
                        varAtts = List::threadMap(List::fill(false, numVars.clone()), List::fill((literal!("")).clone(), numVars.clone()), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
                        eqAtts = List::threadMap(List::fill(false, numSimpEqs.clone()), List::fill((literal!("")).clone(), numSimpEqs.clone()), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
                        BackendDump::dumpBipartiteGraphStrongComponent2(simpVars.clone(), simpEqs.clone(), m_cut.clone(), varAtts.clone(), eqAtts.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rL_loops_")); __mm_s.push_str(&*intString(inSysIdx.clone())); ArcStr::from(__mm_s) }).clone())?;
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
                        BackendDump::dumpBipartiteGraphStrongComponent2(simpVars.clone(), simpEqs.clone(), m_after.clone(), varAtts.clone(), eqAtts.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rL_after_")); __mm_s.push_str(&*intString(inSysIdx.clone())); ArcStr::from(__mm_s) }).clone())?;
                    }
                    syst = BackendDAEUtil::clearEqSyst(syst.clone())?;
                    Ok((syst.clone(), inSysIdx.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inEqSys.clone(), inSysIdx.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outEqSys, outShared, outSysIdx))
}

fn resolveLoops_resolvePartitions(mut partitionsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m_uncut: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT_uncut: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqMap: metamodelica::Array<i32>, mut varMap: metamodelica::Array<i32>, mut daeEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut daeVars: BackendDAE::Variables, mut nonLoopEqMark: metamodelica::Array<i32>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut daeEqsOut: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
    daeEqsOut = (::match_deref::match_deref! { match &(partitionsIn.clone()) {
        Deref @ metamodelica::List::Cons { head: partition, tail: rest } => {
            let mut optStructureMapping: Option<(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> = None;
            let mut eqCrossLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut varCrossLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut mapIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut loops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
            let mut map: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
            let mut partition = (*partition).clone();
            partition = List::filter1OnTrue(partition.clone(), (std::sync::Arc::new(fnptr!(arrayIsZeroAt, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), nonLoopEqMark.clone())?;
            if partition.clone().is_empty() {
                eqs = resolveLoops_resolvePartitions(rest.clone(), mIn.clone(), mTIn.clone(), m_uncut.clone(), mT_uncut.clone(), eqMap.clone(), varMap.clone(), daeEqs.clone(), daeVars.clone(), nonLoopEqMark.clone())?;
            } else {
                (loops, eqCrossLst, varCrossLst, optStructureMapping) = resolveLoops_findLoops(list![partition.clone()], mIn.clone(), mTIn.clone(), false);
                if isSome(optStructureMapping.clone()) {
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(optStructureMapping.clone()) {
                        Some((__pa0, __pa1, __pa2)) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    mapIndices = __pa0.clone();
                    map = __pa1.clone();
                    loops = __pa2.clone();
                    loops = List::filter1OnTrueAndUpdate(loops.clone(), (std::sync::Arc::new(evaluateTripleLoop) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<bool> + 'static>), (std::sync::Arc::new(updateTripleLoop) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<Arc<metamodelica::List<i32>>> + 'static>), (m_uncut.clone(), mapIndices.clone(), map.clone()))?;
                } else {
                    loops = List::filterOnFalse(loops.clone(), std::sync::Arc::new(fnptr!(listEmpty, _)))?;
                    loops = List::filter1OnTrue(loops.clone(), (std::sync::Arc::new(evaluateLoop) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>)) -> Result<bool> + 'static>), (m_uncut.clone(), mT_uncut.clone(), eqCrossLst.clone()))?;
                }
                (eqs, _) = resolveLoops_resolveAndReplace(loops.clone(), eqCrossLst.clone(), varCrossLst.clone(), mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqs.clone(), daeVars.clone(), metamodelica::nil())?;
                eqs = resolveLoops_resolvePartitions(rest.clone(), mIn.clone(), mTIn.clone(), m_uncut.clone(), mT_uncut.clone(), eqMap.clone(), varMap.clone(), eqs.clone(), daeVars.clone(), nonLoopEqMark.clone())?;
            }
            eqs.clone()
        },
        Deref @ metamodelica::List::Nil => {
            daeEqs.clone()
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
            let mut numVars: i32 = 0;
            let mut numEqs: i32 = 0;
            let mut idx: i32 = 0;
            let mut loopVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut loopEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut nonLoopVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut deadEndEqsMark: metamodelica::Array<i32> = deadEndEqsMark.clone();
            let mut deadEndVarsMark: metamodelica::Array<i32> = deadEndVarsMark.clone();
            numVars = metamodelica::arrayLength(mTIn.clone());
            numEqs = metamodelica::arrayLength(mIn.clone());
            nonLoopVars = List::filter2OnTrue(List::intRange(numVars.clone()), (std::sync::Arc::new(arrayEntryLengthIs) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, i32) -> Result<bool> + 'static>), mTIn.clone(), 1)?;
            deadEndVarsMark = arrayCreate(numVars.clone(), 0);
            deadEndEqsMark = arrayCreate(numVars.clone(), 0);
            for mut idx in &*nonLoopVars.clone() {
                let mut idx = idx.clone();
                {let _arr = deadEndVarsMark.clone(); let _idx = idx.clone(); let _val = 1; _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
            }
            for mut idx in &*nonLoopVars.clone() {
                let mut idx = idx.clone();
                markDeadEndsInBipartiteGraph(idx.clone(), mIn.clone(), mTIn.clone(), deadEndEqsMark.clone(), deadEndVarsMark.clone())?;
            }
            idx = 1;
            while idx.clone() <= numVars.clone() {
                if metamodelica::arrayGet(deadEndVarsMark.clone(), idx.clone())? == 1 {
                    {let _arr = mTIn.clone(); let _idx = idx.clone(); let _val = metamodelica::nil(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
                } else {
                    loopEqs = metamodelica::arrayGet(mTIn.clone(), idx.clone())?;
                    loopEqs = List::filter1OnTrue(loopEqs.clone(), (std::sync::Arc::new(fnptr!(arrayIsZeroAt, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), deadEndEqsMark.clone())?;
                    {let _arr = mTIn.clone(); let _idx = idx.clone(); let _val = loopEqs.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
                }
                idx = idx.clone() + 1;
            }
            idx = 1;
            while idx.clone() <= numEqs.clone() {
                if metamodelica::arrayGet(deadEndEqsMark.clone(), idx.clone())? == 1 {
                    {let _arr = mIn.clone(); let _idx = idx.clone(); let _val = metamodelica::nil(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
                } else {
                    loopVars = metamodelica::arrayGet(mIn.clone(), idx.clone())?;
                    loopVars = List::filter1OnTrue(loopVars.clone(), (std::sync::Arc::new(fnptr!(arrayIsZeroAt, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), deadEndVarsMark.clone())?;
                    {let _arr = mIn.clone(); let _idx = idx.clone(); let _val = loopVars.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
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
    let mut eqLen: bool = false;
    let mut entry: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut len1: i32 = 0;
    entry = metamodelica::arrayGet(arr.clone(), idx.clone())?;
    len1 = (entry.clone().len() as i32);
    eqLen = intEq(len.clone(), len1.clone());
    Ok(eqLen)
}

fn getSimpleEquations(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, i32, BackendDAE::Variables, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, i32, BackendDAE::Variables, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>))> {
    let mut outEq: Arc<BackendDAE::Equation> = inEq.clone();
    let mut outTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>, i32, BackendDAE::Variables, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>) = (metamodelica::nil(), metamodelica::nil(), 0, <BackendDAE::Variables as ::std::default::Default>::default(), Default::default(), Default::default());
    let mut isSimple: bool = false;
    let mut idx: i32 = 0;
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut markLinEqVars: metamodelica::Array<i32> = Default::default();
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut idxMap: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (eqLst, idxMap, idx, vars, markLinEqVars, m) = inTpl.clone();
    if BackendEquation::isEquation(inEq.clone()) && !(eqIsConst(inEq.clone())) {
        let (__pa0, (__pa1, _)) = BackendEquation::traverseExpsOfEquation(inEq.clone(), (std::sync::Arc::new(isAddOrSubExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (bool, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (bool, BackendDAE::Variables))> + 'static>), (true, vars.clone()))?;
        eq = __pa0.clone();
        isSimple = __pa1.clone();
        if isSimple.clone() {
            eqLst = metamodelica::cons(eq.clone(), eqLst.clone());
            idxMap = metamodelica::cons(idx.clone(), idxMap.clone());
            let __range2 = &*({let __elt = m.borrow()[(idx.clone()-1) as usize].clone(); __elt});
            for mut varIdx in __range2 {
                let mut varIdx = varIdx.clone();
                {let _arr = markLinEqVars.clone(); let _idx = intAbs(varIdx.clone()); let _val = 1; _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
            }
        }
    }
    outTpl = (eqLst.clone(), idxMap.clone(), idx.clone() + 1, vars.clone(), markLinEqVars.clone(), m.clone());
    Ok((outEq, outTpl))
}

fn getSimpleEquationVariables(mut markLinEqVars: metamodelica::Array<i32>, mut vars: BackendDAE::Variables) -> Result<(Arc<metamodelica::List<BackendDAE::Var>>, metamodelica::Array<i32>)> {
    let mut simpVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut varMapArr: metamodelica::Array<i32> = Default::default();
    let mut varIdx: i32 = 0;
    let mut varMap: Arc<metamodelica::List<i32>> = metamodelica::nil();
    varMap = metamodelica::nil();
    for mut varIdx in 1..=metamodelica::arrayLength(markLinEqVars.clone()) {
        if ({let __elt = markLinEqVars.borrow()[(varIdx.clone()-1) as usize].clone(); __elt}) > 0 {
            varMap = metamodelica::cons(varIdx.clone(), varMap.clone());
            simpVars = metamodelica::cons(BackendVariable::getVarAt(vars.clone(), varIdx.clone())?, simpVars.clone());
        }
    }
    varMapArr = metamodelica::arrayFromVec(varMap.clone().into_iter().cloned().collect());
    Ok((simpVars, varMapArr))
}

pub fn resolveLoops_findLoops(mut partitionsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut findExactlyOneLoop: bool) -> (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Option<(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)>) {
    let mut loopsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut crossEqsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut crossVarsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut optStructureMapping: Option<(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> = None;
    let mut loops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut eqVars: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut eqCrossLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varCrossLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut partitionVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut set: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
    for mut partition in &*partitionsIn.clone() {
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
            (loops, optStructureMapping) = unwrap_break_err!(resolveLoops_findLoops2(partition.clone(), eqCrossLst.clone(), varCrossLst.clone(), mIn.clone(), mTIn.clone(), findExactlyOneLoop.clone()), '__try0);
            if if (findExactlyOneLoop.clone()) {!(loops.clone().is_empty()) && !(loopsOut.clone().is_empty())} else {false} {
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
    let mut loopsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut structureMapping: Option<(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> = None;
    (loopsOut, structureMapping) = (::match_deref::match_deref! { match &((eqCrossLstIn.clone(), varCrossLstIn.clone())) {
        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Nil) => {
            let mut isNoSingleLoop: bool = false;
            let mut eqCrossLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut subLoop: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut mapIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut paths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut allPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut simpleLoops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut tripleLoops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut paths0: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut paths1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut loopConnectors: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut connectedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut minAdj: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
            let mut map: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
            let mut mapping: (Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) = (metamodelica::nil(), Default::default());
            let mut optTripleMapping: Option<(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> = None;
            allPaths = getPathTillNextCrossEq(eqCrossLstIn.clone(), mIn.clone(), mTIn.clone(), eqCrossLstIn.clone(), metamodelica::nil(), metamodelica::nil())?;
            allPaths = List::sort(allPaths.clone(), (std::sync::Arc::new(List::listIsLonger) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
            paths1 = List::fold1(allPaths.clone(), (std::sync::Arc::new(getReverseDoubles) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> + 'static>), allPaths.clone(), metamodelica::nil())?;
            simpleLoops = getDoubles(paths1.clone(), metamodelica::nil());
            (_, paths, _) = List::intersection1OnTrue(paths1.clone(), simpleLoops.clone(), (std::sync::Arc::new(intLstIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>))?;
            if simpleLoops.clone().is_empty() {
                (eqCrossLst, paths1, mapping, minAdj) = findEqualPathStructure(eqCrossLstIn.clone(), paths1.clone())?;
                (mapIndices, map) = mapping.clone();
                (tripleLoops, paths0) = getTriples(eqCrossLst.clone(), minAdj.clone())?;
                optTripleMapping = Some((mapIndices.clone(), map.clone(), tripleLoops.clone()));
            } else {
                optTripleMapping = None;
                paths0 = List::sort(paths.clone(), (std::sync::Arc::new(List::listIsLonger) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
                (connectedPaths, loopConnectors) = connect2PathsToLoops(paths0.clone(), metamodelica::nil(), metamodelica::nil())?;
                loopConnectors = List::filter1OnTrue(loopConnectors.clone(), (std::sync::Arc::new(connectsLoops) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<bool> + 'static>), simpleLoops.clone())?;
                simpleLoops = listAppend(simpleLoops.clone(), loopConnectors.clone());
                subLoop = connectPathsToOneLoop(simpleLoops.clone(), metamodelica::nil())?;
                isNoSingleLoop = subLoop.clone().is_empty();
                simpleLoops = if (isNoSingleLoop.clone()) {simpleLoops.clone()} else {list![subLoop.clone()]};
                paths0 = listAppend(simpleLoops.clone(), connectedPaths.clone());
                paths0 = sortPathsAsChain(paths0.clone())?;
                if findExactlyOneLoop.clone() {
                    if !(paths0.clone().is_empty()) {
                        ::match_deref::match_deref! { match &(paths0.clone()) {
                            Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => (),
                            _ => bail!("pattern mismatch"),
                        } };
                    }
                }
            }
            (paths0.clone(), optTripleMapping.clone())
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
            let mut paths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut paths0: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut paths1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut closedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            paths = getPathTillNextCrossEq(varCrossLstIn.clone(), mTIn.clone(), mIn.clone(), varCrossLstIn.clone(), metamodelica::nil(), metamodelica::nil())?;
            paths = List::sort(paths.clone(), (std::sync::Arc::new(List::listIsLonger) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
            paths = paths.clone().reverse();
            (paths0, paths1) = List::extract1OnTrue(paths.clone(), (std::sync::Arc::new(fnptr!(listLengthIs, Arc<metamodelica::List<i32>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), (List::last(paths.clone())?.len() as i32))?;
            paths1 = if (paths1.clone().is_empty()) {paths0.clone()} else {paths1.clone()};
            closedPaths = List::map1(paths1.clone(), (std::sync::Arc::new(closePathDirectly) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), paths0.clone())?;
            closedPaths = List::fold1(closedPaths.clone(), (std::sync::Arc::new(getReverseDoubles) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> + 'static>), closedPaths.clone(), metamodelica::nil())?;
            closedPaths = List::map(closedPaths.clone(), std::sync::Arc::new(fnptr!(List::unique, _)))?;
            closedPaths = List::map1(closedPaths.clone(), (std::sync::Arc::new(getEqNodesForVarLoop) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), mTIn.clone())?;
            if findExactlyOneLoop.clone() {
                if !(closedPaths.clone().is_empty()) {
                    ::match_deref::match_deref! { match &(closedPaths.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => (),
                        _ => bail!("pattern mismatch"),
                    } };
                }
            }
            (closedPaths.clone(), None)
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            let mut subLoop: Arc<metamodelica::List<i32>> = metamodelica::nil();
            subLoop = eqsIn.clone();
            for mut e in &*eqsIn.clone() {
                let mut e = e.clone();
                if ({let __elt = mIn.borrow()[(e.clone()-1) as usize].clone(); __elt}).is_empty() {
                    subLoop = metamodelica::nil();
                    break;
                }
            }
            (list![subLoop.clone()], None)
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
            let mut paths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut eqCrossSet: Arc<AvlSetInt::Tree> = Arc::new(AvlSetInt::Tree::EMPTY);
            for mut i in 1..=metamodelica::arrayLength(mIn.clone()) {
                {let _arr = mIn.clone(); let _idx = i.clone(); let _val = List::heapSortIntList(({let __elt = mIn.borrow()[(i.clone()-1) as usize].clone(); __elt})); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
            }
            for mut i in 1..=metamodelica::arrayLength(mTIn.clone()) {
                {let _arr = mTIn.clone(); let _idx = i.clone(); let _val = List::heapSortIntList(({let __elt = mTIn.borrow()[(i.clone()-1) as usize].clone(); __elt})); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
            }
            eqCrossSet = AvlSetInt::addList(crate::AvlSetInt::Tree::interned_EMPTY(), eqCrossLstIn.clone())?;
            paths = getShortPathsBetweenEqCrossNodes(AvlSetInt::listKeysReverse(eqCrossSet.clone(), metamodelica::nil()), eqCrossSet.clone(), mIn.clone(), mTIn.clone(), metamodelica::nil(), findExactlyOneLoop.clone())?;
            (paths.clone(), None)
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
    let mut mapping: (Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) = (metamodelica::nil(), Default::default());
    let mut minAdj: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut mapIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut map: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    minAdj = getMinimalAdjacencyMatrix(crossNodes.clone(), uniquePaths.clone())?;
    (minAdj, uniquePaths, mapIndices, map, crossNodes) = removeEqualPaths(crossNodes.clone(), minAdj.clone(), uniquePaths.clone(), metamodelica::nil(), arrayCreate(({
        let mut __acc: Option<i32> = None;
        for mut cn in (crossNodes.clone()).into_iter().cloned() {
            let __x = cn.clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.unwrap_or((-i32::MAX))
    }), metamodelica::nil()), metamodelica::nil())?;
    mapping = (mapIndices.clone(), map.clone());
    Ok((crossNodes, uniquePaths, mapping, minAdj))
}

fn getMinimalAdjacencyMatrix(mut crossNodes: Arc<metamodelica::List<i32>>, mut uniquePaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut minAdj: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    minAdj = arrayCreate(({
        let mut __acc: Option<i32> = None;
        for mut cn in (crossNodes.clone()).into_iter().cloned() {
            let __x = cn.clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.unwrap_or((-i32::MAX))
    }), metamodelica::nil());
    for mut path in &*uniquePaths.clone() {
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
    for mut cn in &*crossNodes.clone() {
        let mut cn = cn.clone();
        {let _arr = minAdj.clone(); let _idx = cn.clone(); let _val = List::sort(metamodelica::arrayGet(minAdj.clone(), cn.clone())?, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?; _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
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
        (::match_deref::match_deref! { match &(crossNodes.clone()) {
        Deref @ metamodelica::List::Cons { head: cn1, tail: rest } => {
            if !(listMember(cn1.clone(), accCrossNodes.clone())) {
                accCrossNodes = metamodelica::cons(cn1.clone(), accCrossNodes.clone());
            }
            for mut cn2 in &*rest.clone() {
                let mut cn2 = cn2.clone();
                if HpcOmTaskGraph::equalLists(metamodelica::arrayGet(minAdj.clone(), cn1.clone())?, metamodelica::arrayGet(minAdj.clone(), cn2.clone())?) {
                    assigned = metamodelica::cons(cn2.clone(), assigned.clone());
                    {let _arr = minAdj.clone(); let _idx = cn2.clone(); let _val = metamodelica::nil(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
                    uniquePaths = removeNode(cn2.clone(), uniquePaths.clone(), metamodelica::nil());
                } else {
                    unassigned = metamodelica::cons(cn2.clone(), unassigned.clone());
                    if !(listMember(cn2.clone(), accCrossNodes.clone())) {
                        accCrossNodes = metamodelica::cons(cn2.clone(), accCrossNodes.clone());
                    }
                }
            }
            if !(assigned.clone().is_empty()) {
                mapIndices = metamodelica::cons(cn1.clone(), mapIndices.clone());
                map = Array::appendToElement(cn1.clone(), assigned.clone(), map.clone())?;
            }
            removeEqualPaths(unassigned.clone(), minAdj.clone(), uniquePaths.clone(), mapIndices.clone(), map.clone(), accCrossNodes.clone())?
        },
        _ => {
            (minAdj.clone(), uniquePaths.clone(), mapIndices.clone(), map.clone(), accCrossNodes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok((minAdj, uniquePaths, mapIndices, map, accCrossNodes))
}

fn removeNode(mut node: i32, mut inPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut accPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut accPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = accPaths;
    accPaths = (::match_deref::match_deref! { match &(inPaths.clone()) {
        Deref @ metamodelica::List::Cons { head: path, tail: rest } => {
            let mut acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            if !(pathContainsNode(node.clone(), path.clone())) {
                acc = metamodelica::cons(path.clone(), accPaths.clone());
            } else {
                acc = accPaths.clone();
            }
            removeNode(node.clone(), rest.clone(), acc.clone())
        },
        Deref @ metamodelica::List::Nil => {
            accPaths.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    accPaths
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn pathContainsNode(mut node: i32, mut inPath: Arc<metamodelica::List<i32>>) -> bool {
    let mut c: bool = false;
    c = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ metamodelica::List::Cons { head: n, tail: _ } if (intEq(n.clone(), node.clone())) => {
            true
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
            pathContainsNode(node.clone(), rest.clone())
        },
        Deref @ metamodelica::List::Nil => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    c
}

fn listContains(mut lst: Arc<metamodelica::List<i32>>, mut int: i32) -> bool {
    let mut res: bool = false;
    for mut i in &*lst.clone() {
        let mut i = i.clone();
        if intEq(i.clone(), int.clone()) {
            res = true;
            return res.clone();
        }
    }
    res
}

fn hasSameIntSortedExcept(mut inList1: Arc<metamodelica::List<i32>>, mut inList2: Arc<metamodelica::List<i32>>, mut excl: i32) -> Result<bool> {
    let mut rv: bool = false;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut l1: Arc<metamodelica::List<i32>> = inList1.clone();
    let mut l2: Arc<metamodelica::List<i32>> = inList2.clone();
    if inList1.clone().is_empty() || inList2.clone().is_empty() {
        return Ok(rv.clone());
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(l1.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    i1 = __pa0.clone();
    l1 = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(l2.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    i2 = __pa2.clone();
    l2 = __pa3.clone();
    loop {
        if i1.clone() > i2.clone() {
            if l2.clone().is_empty() {
                return Ok(rv.clone());
            }
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(l2.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                _ => bail!("pattern mismatch"),
            } };
            i2 = __pa4.clone();
            l2 = __pa5.clone();
        } else if i1.clone() < i2.clone() {
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
            if i1.clone() != excl.clone() {
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

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getShortPathsBetweenEqCrossNodes(mut eqCrossLstIn: Arc<metamodelica::List<i32>>, mut eqCrossSet: Arc<AvlSetInt::Tree>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut pathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut findExactlyOneLoop: bool) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut pathsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    pathsOut = ({
        let mut paths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(eqCrossLstIn.clone()) {
        Deref @ metamodelica::List::Cons { head: crossEq, tail: rest } => {
            let mut adjVar: i32 = 0;
            let mut adjEq: i32 = 0;
            let mut adjVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut newPath: Arc<metamodelica::List<i32>> = metamodelica::nil();
            adjVars = metamodelica::arrayGet(mIn.clone(), crossEq.clone())?;
            for mut adjVar in &*adjVars.clone() {
                let mut adjVar = adjVar.clone();
                for mut adjEq in &*metamodelica::arrayGet(mTIn.clone(), adjVar.clone())? {
                    let mut adjEq = adjEq.clone();
                    if if (adjEq.clone() > crossEq.clone()) {!(AvlSetInt::hasKey(eqCrossSet.clone(), adjEq.clone())?)} else {true} {
                        continue;
                    }
                    if hasSameIntSortedExcept(adjVars.clone(), metamodelica::arrayGet(mIn.clone(), adjEq.clone())?, adjVar.clone())? {
                        newPath = metamodelica::cons(adjEq.clone(), list![crossEq.clone()]);
                        paths = List::unionElt(newPath.clone(), paths.clone());
                        if if (findExactlyOneLoop.clone()) {!(pathsIn.clone().is_empty())} else {false} {
                            bail!("fail");
                        }
                    }
                }
            }
            paths = getShortPathsBetweenEqCrossNodes(rest.clone(), eqCrossSet.clone(), mIn.clone(), mTIn.clone(), listAppend(paths.clone(), pathsIn.clone()), findExactlyOneLoop.clone())?;
            paths.clone()
        },
        Deref @ metamodelica::List::Nil => {
            pathsIn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok(pathsOut)
}

fn connectsLoops(mut path: Arc<metamodelica::List<i32>>, mut allLoops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<bool> {
    let mut connected: bool = false;
    let mut b1: bool = false;
    let mut b2: bool = false;
    let mut startNode: i32 = 0;
    let mut endNode: i32 = 0;
    let mut loops1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut loops2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    startNode = listHead(path.clone())?;
    endNode = List::last(path.clone())?;
    loops1 = List::filter1OnTrue(allLoops.clone(), (std::sync::Arc::new(firstInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), startNode.clone())?;
    loops2 = List::filter1OnTrue(allLoops.clone(), (std::sync::Arc::new(lastInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), startNode.clone())?;
    b1 = !(loops1.clone().is_empty()) || !(loops2.clone().is_empty());
    loops1 = List::filter1OnTrue(allLoops.clone(), (std::sync::Arc::new(firstInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), endNode.clone())?;
    loops2 = List::filter1OnTrue(allLoops.clone(), (std::sync::Arc::new(lastInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), endNode.clone())?;
    b2 = !(loops1.clone().is_empty()) || !(loops2.clone().is_empty());
    connected = b1.clone() && b2.clone();
    Ok(connected)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn connectPathsToOneLoop(mut allPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut loopIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut loopOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    loopOut = 'mc: {
        let __mc_input = (allPathsIn.clone(), loopIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: startNode, tail: path }) => {
                    let mut endNode: i32 = 0;
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
                    let mut path: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut nextPath: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rest: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut nextPaths1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut nextPaths2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    nextPaths1 = List::filter1OnTrue(allPathsIn.clone(), (std::sync::Arc::new(firstInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), startNode.clone())?;
                    nextPaths2 = List::filter1OnTrue(allPathsIn.clone(), (std::sync::Arc::new(lastInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), startNode.clone())?;
                    nextPaths2 = listAppend(nextPaths1.clone(), nextPaths2.clone());
                    nextPath = listHead(nextPaths2.clone())?;
                    (rest, _) = List::deleteMemberOnTrue(nextPath.clone(), allPathsIn.clone(), (std::sync::Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
                    (nextPath, _) = List::deleteMemberOnTrue(startNode.clone(), nextPath.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    path = listAppend(nextPath.clone(), loopIn.clone());
                    path = connectPathsToOneLoop(rest.clone(), path.clone())?;
                    Ok(path.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: path, tail: rest }, Deref @ metamodelica::List::Nil) => {
                    let mut startNode: i32 = 0;
                    let mut nextPath: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut restPath: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut nextPaths1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut nextPaths2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
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
                    path = connectPathsToOneLoop(rest.clone(), path.clone())?;
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
        bail!("matchcontinue: no arm matched")
    };
    Ok(loopOut)
}

fn resolveLoops_resolveAndReplace(mut loopsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut eqCrossLstIn: Arc<metamodelica::List<i32>>, mut varCrossLstIn: Arc<metamodelica::List<i32>>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqMap: metamodelica::Array<i32>, mut varMap: metamodelica::Array<i32>, mut daeEqsIn: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut daeVarsIn: BackendDAE::Variables, mut replEqsIn: Arc<metamodelica::List<i32>>) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<i32>>)> {
    let mut daeEqsOut: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
    let mut replEqsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (daeEqsOut, replEqsOut) = (::match_deref::match_deref! { match &((loopsIn.clone(), eqCrossLstIn.clone(), varCrossLstIn.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => {
            (daeEqsIn.clone(), replEqsIn.clone())
        },
        (Deref @ metamodelica::List::Cons { head: loop1, tail: rest }, Deref @ metamodelica::List::Cons { head: _, tail: crossEqs }, Deref @ metamodelica::List::Nil) => {
            let mut pos: i32 = 0;
            let mut eqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut replEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut loopVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut adjVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut m_row: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqVars: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut resolvedEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut daeEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
            let mut loop1 = (*loop1).clone();
            let mut rest = (*rest).clone();
            let mut crossEqs = (*crossEqs).clone();
            loop1 = List::unique(loop1.clone());
            (resolvedEq, m_row) = resolveClosedLoop(loop1.clone(), mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqsIn.clone(), daeVarsIn.clone())?;
            (crossEqs, eqs, _) = List::intersection1OnTrue(loop1.clone(), eqCrossLstIn.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            replEqs = List::intersectionOnTrue(replEqsIn.clone(), loop1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            if !(eqs.clone().is_empty()) {
                pos = listHead(eqs.clone())?;
            } else if !(replEqs.clone().is_empty()) {
                pos = listHead(replEqs.clone())?;
            } else if !(crossEqs.clone().is_empty()) {
                pos = listHead(crossEqs.clone())?;
            } else {
                pos = -1;
            }
            (eqs, _) = List::deleteMemberOnTrue(pos.clone(), loop1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            eqVars = List::map1(loop1.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), mIn.clone())?;
            vars = List::flatten(eqVars.clone())?;
            loopVars = doubleEntriesInLst(vars.clone());
            (_, adjVars, _) = List::intersection1OnTrue(vars.clone(), loopVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            List::map2_0(loopVars.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), metamodelica::nil(), mTIn.clone())?;
            List::map2_0(adjVars.clone(), (std::sync::Arc::new(arrayGetDeleteInLst) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> + 'static>), loop1.clone(), mTIn.clone())?;
            List::map2_0(adjVars.clone(), (std::sync::Arc::new(arrayGetAppendLst) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> + 'static>), list![pos.clone()], mTIn.clone())?;
            List::map2_0(loop1.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), metamodelica::nil(), mIn.clone())?;
            {let _arr = mIn.clone(); let _idx = pos.clone(); let _val = adjVars.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
            rest = List::map2(rest.clone(), (std::sync::Arc::new(replaceContractedNodes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), pos.clone(), eqs.clone())?;
            rest = List::unique(rest.clone());
            replEqs = metamodelica::cons(pos.clone(), replEqsIn.clone());
            {let _arr = mIn.clone(); let _idx = pos.clone(); let _val = m_row.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
            pos = metamodelica::arrayGet(eqMap.clone(), pos.clone())?;
            daeEqs = BackendEquation::setAtIndex(daeEqsIn.clone(), pos.clone(), resolvedEq.clone())?;
            (daeEqs, replEqs) = resolveLoops_resolveAndReplace(rest.clone(), eqCrossLstIn.clone(), varCrossLstIn.clone(), mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqs.clone(), daeVarsIn.clone(), replEqs.clone())?;
            (daeEqs.clone(), replEqs.clone())
        },
        (Deref @ metamodelica::List::Cons { head: loop1, tail: rest }, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: _, tail: crossVars }) => {
            let mut pos: i32 = 0;
            let mut eqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut replEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut loopVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut adjVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut m_row: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqVars: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut resolvedEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut daeEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
            let mut loop1 = (*loop1).clone();
            let mut rest = (*rest).clone();
            let mut crossVars = (*crossVars).clone();
            loop1 = List::unique(loop1.clone());
            (resolvedEq, m_row) = resolveClosedLoop(loop1.clone(), mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqsIn.clone(), daeVarsIn.clone())?;
            (replEqs, _, eqs) = List::intersection1OnTrue(replEqsIn.clone(), loop1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            eqs = priorizeEqsWithVarCrosses(eqs.clone(), mIn.clone(), varCrossLstIn.clone())?;
            pos = if (!(replEqs.clone().is_empty())) {listHead(replEqs.clone())?} else {-1};
            pos = if (!(eqs.clone().is_empty())) {listHead(eqs.clone())?} else {pos.clone()};
            (eqs, _) = List::deleteMemberOnTrue(pos.clone(), loop1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            eqVars = List::map1(loop1.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), mIn.clone())?;
            vars = List::flatten(eqVars.clone())?;
            loopVars = doubleEntriesInLst(vars.clone());
            (crossVars, loopVars, _) = List::intersection1OnTrue(loopVars.clone(), varCrossLstIn.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            (_, adjVars, _) = List::intersection1OnTrue(vars.clone(), loopVars.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            adjVars = listAppend(crossVars.clone(), adjVars.clone());
            adjVars = List::unique(adjVars.clone());
            List::map2_0(loopVars.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), metamodelica::nil(), mTIn.clone())?;
            List::map2_0(adjVars.clone(), (std::sync::Arc::new(arrayGetDeleteInLst) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> + 'static>), loop1.clone(), mTIn.clone())?;
            List::map2_0(adjVars.clone(), (std::sync::Arc::new(arrayGetAppendLst) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> + 'static>), list![pos.clone()], mTIn.clone())?;
            List::map2_0(loop1.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), metamodelica::nil(), mIn.clone())?;
            {let _arr = mIn.clone(); let _idx = pos.clone(); let _val = adjVars.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
            rest = List::map2(rest.clone(), (std::sync::Arc::new(replaceContractedNodes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), pos.clone(), eqs.clone())?;
            rest = List::unique(rest.clone());
            replEqs = metamodelica::cons(pos.clone(), replEqsIn.clone());
            {let _arr = mIn.clone(); let _idx = pos.clone(); let _val = m_row.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
            pos = metamodelica::arrayGet(eqMap.clone(), pos.clone())?;
            daeEqs = BackendEquation::setAtIndex(daeEqsIn.clone(), pos.clone(), resolvedEq.clone())?;
            (daeEqs, replEqs) = resolveLoops_resolveAndReplace(rest.clone(), eqCrossLstIn.clone(), varCrossLstIn.clone(), mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqs.clone(), daeVarsIn.clone(), replEqs.clone())?;
            (daeEqs.clone(), replEqs.clone())
        },
        (Deref @ metamodelica::List::Cons { head: loop1, tail: rest }, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            let mut pos: i32 = 0;
            let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut crossEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut replEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut m_row: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqVars: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut resolvedEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut daeEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
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
            vars = List::flatten(eqVars.clone())?;
            List::map2_0(loop1.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), metamodelica::nil(), mIn.clone())?;
            List::map2_0(vars.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), metamodelica::nil(), mTIn.clone())?;
            replEqs = metamodelica::cons(pos.clone(), replEqsIn.clone());
            {let _arr = mIn.clone(); let _idx = pos.clone(); let _val = m_row.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
            pos = metamodelica::arrayGet(eqMap.clone(), pos.clone())?;
            daeEqs = BackendEquation::setAtIndex(daeEqsIn.clone(), pos.clone(), resolvedEq.clone())?;
            (daeEqs, replEqs) = resolveLoops_resolveAndReplace(rest.clone(), eqCrossLstIn.clone(), varCrossLstIn.clone(), mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqs.clone(), daeVarsIn.clone(), replEqs.clone())?;
            (daeEqs.clone(), replEqs.clone())
        },
        (Deref @ metamodelica::List::Cons { head: loop1, tail: rest }, Deref @ metamodelica::List::Cons { head: _, tail: _ }, Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
            let mut pos: i32 = 0;
            let mut eq1: i32 = 0;
            let mut eq2: i32 = 0;
            let mut replEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut m_row: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut resolvedEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut daeEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
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
                if (BackendEquation::equationVars(BackendEquation::get(daeEqsIn.clone(), metamodelica::arrayGet(eqMap.clone(), eq1.clone())?)?, daeVarsIn.clone())?.len() as i32) >= (BackendEquation::equationVars(BackendEquation::get(daeEqsIn.clone(), metamodelica::arrayGet(eqMap.clone(), eq2.clone())?)?, daeVarsIn.clone())?.len() as i32) {
                    pos = eq1.clone();
                } else {
                    pos = eq2.clone();
                }
                replEqs = metamodelica::cons(pos.clone(), replEqsIn.clone());
                {let _arr = mIn.clone(); let _idx = pos.clone(); let _val = m_row.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
                pos = metamodelica::arrayGet(eqMap.clone(), pos.clone())?;
                daeEqs = BackendEquation::setAtIndex(daeEqsIn.clone(), pos.clone(), resolvedEq.clone())?;
            } else {
                replEqs = replEqsIn.clone();
                daeEqs = daeEqsIn.clone();
            }
            (daeEqs, replEqs) = resolveLoops_resolveAndReplace(rest.clone(), eqCrossLstIn.clone(), varCrossLstIn.clone(), mIn.clone(), mTIn.clone(), eqMap.clone(), varMap.clone(), daeEqs.clone(), daeVarsIn.clone(), replEqs.clone())?;
            (daeEqs.clone(), replEqs.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((daeEqsOut, replEqsOut))
}

fn eqIsConst(mut eq: Arc<BackendDAE::Equation>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::RCONST { .. }, scalar: Deref @ DAE::Exp::CREF { .. }, .. } => true,
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { .. }, scalar: Deref @ DAE::Exp::RCONST { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn arrayIsZeroAt(mut pos: i32, mut arr: metamodelica::Array<i32>) -> bool {
    let mut isZero: bool = false;
    isZero = intEq(0, ({let __elt = arr.borrow()[(pos.clone()-1) as usize].clone(); __elt}));
    isZero
}

fn markDeadEndsInBipartiteGraph(mut varIdx: i32, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut deadEndEqs: metamodelica::Array<i32>, mut deadEndVars: metamodelica::Array<i32>) -> Result<()> {
    let mut eqIdx: i32 = 0;
    let mut nextVarIdx: i32 = 0;
    let mut adjEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut adjVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    adjEqs = metamodelica::arrayGet(mTIn.clone(), varIdx.clone())?;
    adjEqs = List::filter1OnTrue(adjEqs.clone(), (std::sync::Arc::new(fnptr!(arrayIsZeroAt, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), deadEndEqs.clone())?;
    if (adjEqs.clone().len() as i32) == 1 {
        eqIdx = listHead(adjEqs.clone())?;
        {let _arr = deadEndVars.clone(); let _idx = varIdx.clone(); let _val = 1; _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
        adjVars = metamodelica::arrayGet(mIn.clone(), eqIdx.clone())?;
        adjVars = List::filter1OnTrue(adjVars.clone(), (std::sync::Arc::new(fnptr!(arrayIsZeroAt, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), deadEndVars.clone())?;
        if (adjVars.clone().len() as i32) == 1 {
            nextVarIdx = listHead(adjVars.clone())?;
            {let _arr = deadEndEqs.clone(); let _idx = eqIdx.clone(); let _val = 1; _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
            markDeadEndsInBipartiteGraph(nextVarIdx.clone(), mIn.clone(), mTIn.clone(), deadEndEqs.clone(), deadEndVars.clone())?;
        }
    }
    Ok(())
}

fn arrayGetDeleteInLst(mut idx: i32, mut delEntries: Arc<metamodelica::List<i32>>, mut arrIn: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut entry: Arc<metamodelica::List<i32>> = metamodelica::nil();
    entry = metamodelica::arrayGet(arrIn.clone(), idx.clone())?;
    (_, entry, _) = List::intersection1OnTrue(entry.clone(), delEntries.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    {let _arr = arrIn.clone(); let _idx = idx.clone(); let _val = entry.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
    Ok(())
}

fn arrayGetAppendLst(mut idx: i32, mut appLst: Arc<metamodelica::List<i32>>, mut arrIn: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut entry: Arc<metamodelica::List<i32>> = metamodelica::nil();
    entry = metamodelica::arrayGet(arrIn.clone(), idx.clone())?;
    {let _arr = arrIn.clone(); let _idx = idx.clone(); let _val = listAppend(entry.clone(), appLst.clone()); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
    Ok(())
}

fn getReverseDoubles(mut elem: Arc<metamodelica::List<i32>>, mut elemLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut foldLstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut foldLstOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    foldLstOut = 'mc: {
        let __mc_input = foldLstIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut elemR: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut foldLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
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
        bail!("matchcontinue: no arm matched")
    };
    Ok(foldLstOut)
}

fn getDoubles(mut elemLstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut lstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut lstOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    lstOut = (::match_deref::match_deref! { match &(elemLstIn.clone()) {
        Deref @ metamodelica::List::Nil => {
            lstIn.clone()
        },
        Deref @ metamodelica::List::Cons { head: elem, tail: elemLst } => {
            let mut lst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            if listMember(elem.clone(), elemLst.clone()) {
                lst = getDoubles(elemLst.clone(), metamodelica::cons(elem.clone(), lstIn.clone()));
            } else {
                lst = getDoubles(elemLst.clone(), lstIn.clone());
            }
            lst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    lstOut
}

fn getTriples(mut crossNodes: Arc<metamodelica::List<i32>>, mut minAdj: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut tripleLoops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut allPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut path1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut path2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut path3: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut c0 in &*crossNodes.clone() {
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
    let mut eqIdcs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varEqLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut eqLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    varEqLst = List::map1(varIdcs.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), mTIn.clone())?;
    eqLst = List::flatten(varEqLst.clone())?;
    eqIdcs = doubleEntriesInLst(eqLst.clone());
    Ok(eqIdcs)
}

fn resolveClosedLoop(mut loopIn: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut eqMap: metamodelica::Array<i32>, mut varMap: metamodelica::Array<i32>, mut daeEqsIn: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut daeVarsIn: BackendDAE::Variables) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)> {
    let mut eqOut: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut m_row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut startEqIdx: i32 = 0;
    let mut startEqDaeIdx: i32 = 0;
    let mut loop1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut restLoop: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(loopIn.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    startEqIdx = __pa0.clone();
    restLoop = __pa1.clone();
    startEqDaeIdx = metamodelica::arrayGet(eqMap.clone(), startEqIdx.clone())?;
    loop1 = sortLoop(restLoop.clone(), m.clone(), mT.clone(), list![startEqIdx.clone()])?;
    if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? && (loop1.clone().len() as i32) > 1 {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("solve the loop: ")); __mm_s.push_str(&*List::toString(loop1.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    eq = BackendEquation::get(daeEqsIn.clone(), startEqDaeIdx.clone())?;
    (eqOut, m_row) = resolveClosedLoop2(eq.clone(), loop1.clone(), m.clone(), metamodelica::arrayGet(m.clone(), startEqIdx.clone())?, eqMap.clone(), varMap.clone(), daeEqsIn.clone(), daeVarsIn.clone())?;
    Ok((eqOut, m_row))
}

fn resolveClosedLoop2(mut eq: Arc<BackendDAE::Equation>, mut loopIn: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut m_row: Arc<metamodelica::List<i32>>, mut eqMap: metamodelica::Array<i32>, mut varMap: metamodelica::Array<i32>, mut daeEqsIn: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut daeVarsIn: BackendDAE::Variables) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<i32>>)> {
    let mut eq: Arc<BackendDAE::Equation> = eq;
    let mut m_row: Arc<metamodelica::List<i32>> = m_row;
    (eq, m_row) = (::match_deref::match_deref! { match &(loopIn.clone()) {
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => {
            (eq.clone(), m_row.clone())
        },
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: eqIdx2, tail: restLoop } } => {
            let mut algSign: bool = false;
            let mut adjVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut adjVars1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut adjVars2: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut posVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut negVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut nonUnitVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut adjCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut eq2: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut eq3: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut resolvedEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
            let mut replacements: BackendVarTransform::VariableReplacements = <BackendVarTransform::VariableReplacements as ::std::default::Default>::default();
            eq2 = BackendEquation::get(daeEqsIn.clone(), metamodelica::arrayGet(eqMap.clone(), eqIdx2.clone())?)?;
            adjVars1 = m_row.clone();
            adjVars2 = metamodelica::arrayGet(m.clone(), eqIdx2.clone())?;
            (adjVars, adjVars1, adjVars2) = List::intersection1OnTrue(adjVars1.clone(), adjVars2.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            (adjVars, nonUnitVars) = List::splitOnTrue(adjVars.clone(), (std::sync::Arc::new({ let __pe_b1 = varMap.clone(); let __pe_b2 = daeVarsIn.clone(); let __pe_b3 = eq.clone(); let __pe_b4 = eq2.clone(); move |__pe_a0| varIsUnitCoeff(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            (posVars, negVars) = List::splitOnTrue(adjVars.clone(), (std::sync::Arc::new({ let __pe_b1 = varMap.clone(); let __pe_b2 = daeVarsIn.clone(); let __pe_b3 = eq.clone(); let __pe_b4 = eq2.clone(); move |__pe_a0| varSign(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            algSign = (posVars.clone().len() as i32) > (negVars.clone().len() as i32);
            adjCrefs = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
        for mut idx in (if (algSign.clone()) {posVars.clone()} else {negVars.clone()}).into_iter().cloned() {
            let __x = crefFromIndex(idx.clone(), varMap.clone(), daeVarsIn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            m_row = List::flatten(list![adjVars1.clone(), adjVars2.clone(), nonUnitVars.clone(), if (algSign.clone()) {negVars.clone()} else {posVars.clone()}])?;
            replacements = BackendVarTransform::emptyReplacementsSized((adjCrefs.clone().len() as i32));
            replacements = BackendVarTransform::addReplacements(replacements.clone(), adjCrefs.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut c in (adjCrefs.clone()).into_iter().cloned() {
            let __x = Expression::createZeroExpression(ComponentReference::crefTypeFull(c.clone())?)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), None)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendVarTransform::replaceEquations(list![eq.clone(), eq2.clone()], replacements.clone(), None)?) {
                (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } }, _) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            resolvedEq = __pa0.clone();
            eq3 = __pa1.clone();
            resolvedEq = sumUp2Equations(algSign.clone(), resolvedEq.clone(), eq3.clone())?;
            if Flags::isSet(Flags::RESOLVE_LOOPS_DUMP.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("From eqs \n")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*BackendDump::equationString(eq2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("resolved the eq \n")); __mm_s.push_str(&*BackendDump::equationString(resolvedEq.clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            resolveClosedLoop2(resolvedEq.clone(), metamodelica::cons(eqIdx2.clone(), restLoop.clone()), m.clone(), m_row.clone(), eqMap.clone(), varMap.clone(), daeEqsIn.clone(), daeVarsIn.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((eq, m_row))
}

fn crefFromIndex(mut varIdx: i32, mut varMap: metamodelica::Array<i32>, mut daeVarsIn: BackendDAE::Variables) -> Result<Arc<DAE::ComponentRef>> {
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut daeVarIdx: i32 = 0;
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    daeVarIdx = metamodelica::arrayGet(varMap.clone(), varIdx.clone())?;
    var = BackendVariable::getVarAt(daeVarsIn.clone(), daeVarIdx.clone())?;
    cref = BackendVariable::varCref(var.clone())?;
    Ok(cref)
}

fn varSign(mut index: i32, mut varMap: metamodelica::Array<i32>, mut daeVarsIn: BackendDAE::Variables, mut eq1: Arc<BackendDAE::Equation>, mut eq2: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut algSign: bool = false;
    let mut cref: Arc<DAE::ComponentRef> = crefFromIndex(index.clone(), varMap.clone(), daeVarsIn.clone())?;
    algSign = CRefIsPosOnRHS(cref.clone(), eq1.clone())? != CRefIsPosOnRHS(cref.clone(), eq2.clone())?;
    Ok(algSign)
}

fn varIsUnitCoeff(mut index: i32, mut varMap: metamodelica::Array<i32>, mut daeVarsIn: BackendDAE::Variables, mut eq1: Arc<BackendDAE::Equation>, mut eq2: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut isUnit: bool = false;
    let mut cref: Arc<DAE::ComponentRef> = crefFromIndex(index.clone(), varMap.clone(), daeVarsIn.clone())?;
    isUnit = crefHasUnitCoeff(cref.clone(), eq1.clone())? && crefHasUnitCoeff(cref.clone(), eq2.clone())?;
    Ok(isUnit)
}

fn crefHasUnitCoeff(mut cref: Arc<DAE::ComponentRef>, mut eq: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut isUnit: bool = false;
    isUnit = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. } => {
            crefUnitCoeffInExp(e1.clone(), cref.clone())? && crefUnitCoeffInExp(e2.clone(), cref.clone())?
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isUnit)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn crefUnitCoeffInExp(mut exp: Arc<DAE::Exp>, mut cref: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut isUnit: bool = false;
    isUnit = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } => {
            crefUnitCoeffInExp(e1.clone(), cref.clone())? && crefUnitCoeffInExp(e2.clone(), cref.clone())?
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
            crefUnitCoeffInExp(e1.clone(), cref.clone())? && crefUnitCoeffInExp(e2.clone(), cref.clone())?
        },
        Deref @ DAE::Exp::UNARY { exp: e1, .. } => {
            crefUnitCoeffInExp(e1.clone(), cref.clone())?
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { componentRef: c, .. }, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
            !(ComponentReferenceBasics::crefEqualNoStringCompare(cref.clone(), c.clone())?) || Expression::isOne(e2.clone()) || Expression::isConstMinusOne(e2.clone())
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CREF { componentRef: c, .. } } => {
            !(ComponentReferenceBasics::crefEqualNoStringCompare(cref.clone(), c.clone())?) || Expression::isOne(e1.clone()) || Expression::isConstMinusOne(e1.clone())
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isUnit)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn sortLoop(mut loopIn: Arc<metamodelica::List<i32>>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut sortLoopIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut sortLoopOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    sortLoopOut = 'mc: {
        let __mc_input = (loopIn.clone(), sortLoopIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(sortLoopIn.clone().reverse())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: start, tail: _ }) => {
                    let mut next: i32 = 0;
                    let mut rest: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut eqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut varEqs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    vars = metamodelica::arrayGet(m.clone(), start.clone())?;
                    varEqs = List::map1(vars.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), mT.clone())?;
                    eqs = List::flatten(varEqs.clone())?;
                    eqs = List::unique(eqs.clone());
                    eqs = List::intersectionOnTrue(eqs.clone(), loopIn.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    if eqs.clone().is_empty() {
                        next = listHead(loopIn.clone())?;
                    } else {
                        next = listHead(eqs.clone())?;
                    }
                    (rest, _) = List::deleteMemberOnTrue(next.clone(), loopIn.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    Ok(sortLoop(rest.clone(), m.clone(), mT.clone(), metamodelica::cons(next.clone(), sortLoopIn.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(sortLoopOut)
}

fn closePathDirectly(mut pathIn: Arc<metamodelica::List<i32>>, mut pathLstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut pathOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    pathOut = 'mc: {
        let __mc_input = pathLstIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut startNode: i32 = 0;
                    let mut endNode: i32 = 0;
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
                    let mut closed: bool = false;
                    let mut startNode: i32 = 0;
                    let mut endNode: i32 = 0;
                    let mut path: Arc<metamodelica::List<i32>> = metamodelica::nil();
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

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn findPathByEnds(mut pathLstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut startNodeIn: i32, mut endNodeIn: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut pathOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    pathOut = 'mc: {
        let __mc_input = pathLstIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: path, tail: pathLst } => {
                    let mut b1: bool = false;
                    let mut b2: bool = false;
                    let mut startNode: i32 = 0;
                    let mut endNode: i32 = 0;
                    let mut path = (*path).clone();
                    startNode = listHead(path.clone())?;
                    b1 = intEq(startNode.clone(), endNodeIn.clone());
                    endNode = List::last(path.clone())?;
                    b2 = intEq(endNode.clone(), startNodeIn.clone());
                    path = if (!(b1.clone() && b2.clone())) {findPathByEnds(pathLst.clone(), startNodeIn.clone(), endNodeIn.clone())?} else {path.clone()};
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
    for mut elem in &*lstIn.clone() {
        let mut elem = elem.clone();
        if listMember(elem.clone(), checkLst.clone()) {
            num = num.clone() + 1;
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
    for mut lst in &*lstIn.clone() {
        let mut lst = lst.clone();
        for mut elem in &*lst.clone() {
            let mut elem = elem.clone();
            if listMember(elem.clone(), checkLst.clone()) {
                num = num.clone() + 1;
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
    for mut i in &*lstIn.clone() {
        let mut i = i.clone();
        if listMember(i.clone(), checkLst.clone()) {
            doubleLst = metamodelica::cons(i.clone(), doubleLst.clone());
        } else {
            checkLst = metamodelica::cons(i.clone(), checkLst.clone());
        }
    }
    doubleLst
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getPathTillNextCrossEq(mut checkEqCrossNodes: Arc<metamodelica::List<i32>>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut allEqCrossNodes: Arc<metamodelica::List<i32>>, mut unfinPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut eqPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut eqPathsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    eqPathsOut = 'mc: {
        let __mc_input = (checkEqCrossNodes.clone(), unfinPathsIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: crossEq, tail: restCrossNodes }, Deref @ metamodelica::List::Nil) => {
                    let mut adjVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut nextEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut endEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut unfinEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut paths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut adjEqs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut unfinPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
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
                    let mut lastEq: i32 = 0;
                    let mut prevEq: i32 = 0;
                    let mut adjVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut nextEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut endEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut unfinEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut paths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut adjEqs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut unfinPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
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
    let mut outLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outLst = metamodelica::cons(elem.clone(), lst.clone());
    outLst
}

fn replaceContractedNodes(mut pathIn: Arc<metamodelica::List<i32>>, mut nodeIn: i32, mut replNodes: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut pathOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    pathOut = List::map2(pathIn.clone(), (std::sync::Arc::new(replaceContractedNodes2) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, Arc<metamodelica::List<i32>>) -> Result<i32> + 'static>), nodeIn.clone(), replNodes.clone())?;
    Ok(pathOut)
}

fn replaceContractedNodes2(mut entryIn: i32, mut nodeIn: i32, mut replNodes: Arc<metamodelica::List<i32>>) -> Result<i32> {
    let mut entryOut: i32 = 0;
    let mut repl: bool = false;
    repl = List::isMemberOnTrue(entryIn.clone(), replNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    entryOut = if (repl.clone()) {nodeIn.clone()} else {entryIn.clone()};
    Ok(entryOut)
}

fn priorizeEqsWithVarCrosses(mut eqsIn: Arc<metamodelica::List<i32>>, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut varCrossLst: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut eqsOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut priorities: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    priorities = arrayCreate(3, metamodelica::nil());
    for mut eq in &*eqsIn.clone() {
        let mut eq = eq.clone();
        priorizeEqsWithVarCrosses2(eq.clone(), mIn.clone(), varCrossLst.clone(), priorities.clone())?;
    }
    eqsOut = List::flatten(Arc::new(priorities.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()))?;
    Ok(eqsOut)
}

fn priorizeEqsWithVarCrosses2(mut eq: i32, mut mIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut varCrossLst: Arc<metamodelica::List<i32>>, mut priorities: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut eqVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut crossVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    eqVars = metamodelica::arrayGet(mIn.clone(), eq.clone())?;
    crossVars = List::intersectionOnTrue(eqVars.clone(), varCrossLst.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    if crossVars.clone().is_empty() {
        arrayGetAppendLst(1, list![eq.clone()], priorities.clone())?;
    } else if List::hasOneElement(crossVars.clone()) {
        arrayGetAppendLst(2, list![eq.clone()], priorities.clone())?;
    } else {
        arrayGetAppendLst(3, list![eq.clone()], priorities.clone())?;
    }
    Ok(())
}

fn evaluateLoop(mut loopIn: Arc<metamodelica::List<i32>>, mut tplIn: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>)) -> Result<bool> {
    let mut resolve: bool = true;
    let mut r1: bool = false;
    let mut r2: bool = false;
    let mut numInLoop: i32 = 0;
    let mut numOutLoop: i32 = 0;
    let mut eqCrossLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut chk: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut dup: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqVars: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    if !(intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 3)) {
        (m, _, eqCrossLst) = tplIn.clone();
        eqVars = List::map1(loopIn.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), m.clone())?;
        (numInLoop, chk, dup) = countDoubleEntriesInLstLst(eqVars.clone(), chk.clone(), dup.clone());
        numOutLoop = (chk.clone().len() as i32) - (dup.clone().len() as i32);
        r1 = intGe(numInLoop.clone(), numOutLoop.clone() - 1) && intLe(numInLoop.clone(), 6);
        r2 = intGe(numInLoop.clone(), numOutLoop.clone() - 2);
        r1 = if (intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 1)) {r1.clone()} else {false};
        resolve = if (intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 2)) {r2.clone()} else {r1.clone()};
    }
    Ok(resolve)
}

fn evaluateTripleLoop(mut loopIn: Arc<metamodelica::List<i32>>, mut tplIn: (metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<bool> {
    let mut resolve: bool = true;
    let mut r1: bool = false;
    let mut r2: bool = false;
    let mut n: i32 = 0;
    let mut numInLoop: i32 = 0;
    let mut numOutLoop: i32 = 0;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut map: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut mapIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut chk: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut dup: Arc<metamodelica::List<i32>> = metamodelica::nil();
    if !(intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 3)) {
        (m, mapIndices, map) = tplIn.clone();
        for mut j in &*loopIn.clone() {
            let mut j = j.clone();
            (n, chk, dup) = countDoubleEntriesInLst(metamodelica::arrayGet(m.clone(), j.clone())?, chk.clone(), dup.clone());
            numInLoop = numInLoop.clone() + n.clone();
        }
        for mut i in &*mapIndices.clone() {
            let mut i = i.clone();
            for mut j in &*metamodelica::arrayGet(map.clone(), i.clone())? {
                let mut j = j.clone();
                (n, chk, dup) = countDoubleEntriesInLst(metamodelica::arrayGet(m.clone(), j.clone())?, chk.clone(), dup.clone());
                numInLoop = numInLoop.clone() + n.clone();
            }
        }
        numOutLoop = (chk.clone().len() as i32) - (dup.clone().len() as i32);
        r1 = intGe(numInLoop.clone(), numOutLoop.clone() - 1) && intLe(numInLoop.clone(), 10);
        r2 = intGe(numInLoop.clone(), numOutLoop.clone() - 2);
        r1 = if (intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 1)) {r1.clone()} else {false};
        resolve = if (intEq(Flags::getConfigInt(Flags::RESHUFFLE.clone())?, 2)) {r2.clone()} else {r1.clone()};
    }
    Ok(resolve)
}

fn updateTripleLoop(mut loopFull: Arc<metamodelica::List<i32>>, mut tplIn: (metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) -> Result<Arc<metamodelica::List<i32>>> {
    let mut loopFull: Arc<metamodelica::List<i32>> = loopFull;
    let mut map: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut mapIndices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (_, mapIndices, map) = tplIn.clone();
    for mut i in &*mapIndices.clone() {
        let mut i = i.clone();
        loopFull = listAppend(metamodelica::arrayGet(map.clone(), i.clone())?, loopFull.clone());
    }
    Ok(loopFull)
}

fn sumUp2Equations(mut sumUp: bool, mut eq1: Arc<BackendDAE::Equation>, mut eq2: Arc<BackendDAE::Equation>) -> Result<Arc<BackendDAE::Equation>> {
    let mut eqOut: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut exp1: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut exp2: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut exp3: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut exp4: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eq1.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp1 = __pa0.clone();
    exp2 = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(eq2.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { exp: __pa2, scalar: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp3 = __pa2.clone();
    exp4 = __pa3.clone();
    exp1 = sumUp2Expressions(sumUp.clone(), exp1.clone(), exp3.clone())?;
    exp2 = sumUp2Expressions(sumUp.clone(), exp2.clone(), exp4.clone())?;
    exp2 = sumUp2Expressions(false, exp2.clone(), exp1.clone())?;
    (exp2, _) = ExpressionSimplify::simplify(exp2.clone())?;
    exp1 = Expression::createZeroExpression(Expression::r#typeof(exp2.clone())?)?;
    eqOut = Arc::new(BackendDAE::Equation::EQUATION { exp: exp1.clone(), scalar: exp2.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() });
    eqOut = simplifyZeroAssignment(eqOut.clone());
    Ok(eqOut)
}

fn simplifyZeroAssignment(mut eIn: Arc<BackendDAE::Equation>) -> Arc<BackendDAE::Equation> {
    let mut eOut: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    eOut = (::match_deref::match_deref! { match &(eIn.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::RCONST { real: __rlit_0 }, scalar: Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: _ }, operator: DAE::Operator::MUL { .. }, exp2: e @ Deref @ DAE::Exp::CREF { .. } }, source, attr } if __rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), scalar: e.clone(), source: source.clone(), attr: attr.clone() })
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::RCONST { real: __rlit_1 }, exp: Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RCONST { real: _ }, operator: DAE::Operator::MUL { .. }, exp2: e @ Deref @ DAE::Exp::CREF { .. } }, source, attr } if __rlit_1.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), scalar: e.clone(), source: source.clone(), attr: attr.clone() })
        },
        _ => {
            eIn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    eOut
}

fn CRefIsPosOnRHS(mut crefIn: Arc<DAE::ComponentRef>, mut eqIn: Arc<BackendDAE::Equation>) -> Result<bool> {
    let mut isPos: bool = false;
    isPos = 'mc: {
        let __mc_input = eqIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, .. } => {
                    let mut exists1: bool = false;
                    let mut sign1: bool = false;
                    let mut sign2: bool = false;
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
    let mut isInExp: bool = false;
    let mut algSign: bool = false;
    (isInExp, algSign) = (::match_deref::match_deref! { match &(expIn.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cref, .. } => {
            let mut sameCref: bool = false;
            sameCref = ComponentReferenceBasics::crefEqualNoStringCompare(crefIn.clone(), cref.clone())?;
            (sameCref.clone(), true)
        },
        Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::SUB { .. }, exp2 } => {
            let mut sign: bool = false;
            let mut sign1: bool = false;
            let mut sign2: bool = false;
            let mut exists: bool = false;
            let mut exists1: bool = false;
            let mut exists2: bool = false;
            (exists1, sign1) = expIsCref(exp1.clone(), crefIn.clone())?;
            (exists2, sign2) = expIsCref(exp2.clone(), crefIn.clone())?;
            sign2 = boolNot(sign2.clone());
            exists = boolOr(exists1.clone(), exists2.clone());
            sign = exists1.clone() && sign1.clone();
            sign = if (exists2.clone()) {sign2.clone()} else {sign.clone()};
            (exists.clone(), sign.clone())
        },
        Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD { .. }, exp2 } => {
            let mut sign: bool = false;
            let mut sign1: bool = false;
            let mut sign2: bool = false;
            let mut exists: bool = false;
            let mut exists1: bool = false;
            let mut exists2: bool = false;
            (exists1, sign1) = expIsCref(exp1.clone(), crefIn.clone())?;
            (exists2, sign2) = expIsCref(exp2.clone(), crefIn.clone())?;
            exists = boolOr(exists1.clone(), exists2.clone());
            sign = exists1.clone() && sign1.clone();
            sign = if (exists2.clone()) {sign2.clone()} else {sign.clone()};
            (exists.clone(), sign.clone())
        },
        Deref @ DAE::Exp::BINARY { exp1: exp1 @ Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::RCONST { real: r } } => {
            let mut sign: bool = false;
            let mut exists: bool = false;
            (exists, _) = expIsCref(exp1.clone(), crefIn.clone())?;
            sign = r.clone() > metamodelica::OrderedFloat((0) as f64);
            (exists.clone(), sign.clone())
        },
        Deref @ DAE::Exp::BINARY { exp1: exp1 @ Deref @ DAE::Exp::RCONST { real: r }, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CREF { .. } } => {
            let mut sign: bool = false;
            let mut exists: bool = false;
            (exists, _) = expIsCref(exp1.clone(), crefIn.clone())?;
            sign = r.clone() > metamodelica::OrderedFloat((0) as f64);
            (exists.clone(), sign.clone())
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: exp1 } => {
            let mut sign: bool = false;
            let mut exists: bool = false;
            (exists, sign) = expIsCref(exp1.clone(), crefIn.clone())?;
            sign = boolNot(sign.clone());
            (exists.clone(), sign.clone())
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            (false, false)
        },
        Deref @ DAE::Exp::ICONST { .. } => {
            (false, false)
        },
        _ => {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("add a case to expIsCref:")); __mm_s.push_str(&*ExpressionBasics::printExpStr(expIn.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            (false, false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((isInExp, algSign))
}

fn listLengthIs(mut lst: Arc<metamodelica::List<i32>>, mut value: i32) -> bool {
    let mut bOut: bool = false;
    bOut = intEq((lst.clone().len() as i32), value.clone());
    bOut
}

pub fn partitionBipartiteGraph(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut partitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut numEqs: i32 = 0;
    let mut numVars: i32 = 0;
    let mut markEqs: metamodelica::Array<i32> = Default::default();
    let mut markVars: metamodelica::Array<i32> = Default::default();
    numEqs = metamodelica::arrayLength(m.clone());
    numVars = metamodelica::arrayLength(mT.clone());
    if numEqs.clone() == 0 || numVars.clone() == 0 {
        partitions = list![metamodelica::nil()];
    } else {
        markEqs = arrayCreate(numEqs.clone(), -1);
        markVars = arrayCreate(numVars.clone(), -1);
        (_, partitions) = colorNodePartitions(m.clone(), mT.clone(), list![1], markEqs.clone(), markVars.clone(), 1, metamodelica::nil(), 1)?;
    }
    Ok(partitions)
}

fn colorNodePartitions(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut checkNextIn: Arc<metamodelica::List<i32>>, mut markEqs: metamodelica::Array<i32>, mut markVars: metamodelica::Array<i32>, mut currNumberIn: i32, mut partitionsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut nextIndex: i32) -> Result<(i32, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut currNumberOut: i32 = 0;
    let mut partitionsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut eq: i32 = 0;
    let mut next_index: i32 = 0;
    let mut rest: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut part: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut restPart: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut partitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    (currNumberOut, partitionsOut) = (::match_deref::match_deref! { match &(checkNextIn.clone()) {
        Deref @ metamodelica::List::Cons { head: 0, tail: Deref @ metamodelica::List::Nil } => (currNumberIn.clone() - 1, partitionsIn.clone()),
        Deref @ metamodelica::List::Cons { head: __esc_eq, tail: __esc_rest } => {
            eq = (*__esc_eq).clone();
            rest = (*__esc_rest).clone();
            if arrayGetIsNotPositive(eq.clone(), markEqs.clone())? {
                {let _arr = markEqs.clone(); let _idx = eq.clone(); let _val = currNumberIn.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
                if partitionsIn.clone().is_empty() {
                    partitions = list![list![eq.clone()]];
                } else {
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(partitionsIn.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    part = __pa0.clone();
                    restPart = __pa1.clone();
                    part = metamodelica::cons(eq.clone(), part.clone());
                    partitions = metamodelica::cons(part.clone(), restPart.clone());
                }
                vars = metamodelica::arrayGet(m.clone(), eq.clone())?;
                let true = (!(vars.clone().is_empty())) else { bail!("pattern mismatch") };
                vars = List::filter1OnTrue(vars.clone(), (std::sync::Arc::new(arrayGetIsNotPositive) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), markVars.clone())?;
                List::map2_0(vars.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), currNumberIn.clone(), markVars.clone())?;
                eqs = List::fold1(vars.clone(), (std::sync::Arc::new(getArrayEntryAndAppend) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), mT.clone(), metamodelica::nil())?;
                eqs = List::filter1OnTrue(eqs.clone(), (std::sync::Arc::new(arrayGetIsNegative) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), markEqs.clone())?;
                List::map2_0(eqs.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), 0, markEqs.clone())?;
                rest = listAppend(rest.clone(), eqs.clone());
            } else {
                partitions = partitionsIn.clone();
            }
            colorNodePartitions(m.clone(), mT.clone(), rest.clone(), markEqs.clone(), markVars.clone(), currNumberIn.clone(), partitions.clone(), nextIndex.clone())?
        },
        Deref @ metamodelica::List::Nil => {
            eq = 0;
            next_index = nextIndex.clone();
            for mut i in nextIndex.clone()..=metamodelica::arrayLength(markEqs.clone()) {
                if ({let __elt = markEqs.borrow()[(i.clone()-1) as usize].clone(); __elt}) == -1 {
                    eq = i.clone();
                    next_index = i.clone() + 1;
                    break;
                }
            }
            colorNodePartitions(m.clone(), mT.clone(), list![eq.clone()], markEqs.clone(), markVars.clone(), currNumberIn.clone() + 1, metamodelica::cons(metamodelica::nil(), partitionsIn.clone()), next_index.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((currNumberOut, partitionsOut))
}

fn arrayGetIsNotPositive(mut idx: i32, mut arrayIn: metamodelica::Array<i32>) -> Result<bool> {
    let mut isNonZero: bool = false;
    isNonZero = metamodelica::arrayGet(arrayIn.clone(), idx.clone())? <= 0;
    Ok(isNonZero)
}

fn arrayGetIsNegative(mut idx: i32, mut arrayIn: metamodelica::Array<i32>) -> Result<bool> {
    let mut isNonZero: bool = false;
    isNonZero = metamodelica::arrayGet(arrayIn.clone(), idx.clone())? < 0;
    Ok(isNonZero)
}

fn getArrayEntryAndAppend(mut entry: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut lstIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut lstOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut lst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    lst = metamodelica::arrayGet(m.clone(), entry.clone())?;
    lstOut = listAppend(lst.clone(), lstIn.clone());
    Ok(lstOut)
}

fn gatherCrossNodes(mut idx: i32, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut lstIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut lstOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut isCross: bool = false;
    let mut num: i32 = 0;
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    row = metamodelica::arrayGet(m.clone(), idx.clone())?;
    num = (row.clone().len() as i32);
    isCross = intGt(num.clone(), 2);
    lstOut = if (isCross.clone()) {metamodelica::cons(idx.clone(), lstIn.clone())} else {lstIn.clone()};
    Ok(lstOut)
}

fn isAddOrSubExp(mut inExp: Arc<DAE::Exp>, mut inTuple: (bool, BackendDAE::Variables)) -> Result<(Arc<DAE::Exp>, (bool, BackendDAE::Variables))> {
    let mut outExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut outTuple: (bool, BackendDAE::Variables) = (false, <BackendDAE::Variables as ::std::default::Default>::default());
    (outExp, outTuple) = (::match_deref::match_deref! { match &((inExp.clone(), inTuple.clone())) {
        (Deref @ DAE::Exp::CREF { .. }, (true, vars)) => {
            (inExp.clone(), (true, vars.clone()))
        },
        (Deref @ DAE::Exp::UNARY { exp: exp1, .. }, (true, vars)) => {
            let mut b: bool = false;
            let (_, (__pa0, _)) = isAddOrSubExp(exp1.clone(), (true, vars.clone()))?;
            b = __pa0.clone();
            (inExp.clone(), (b.clone(), vars.clone()))
        },
        (Deref @ DAE::Exp::RCONST { .. }, (true, vars)) => {
            (inExp.clone(), (true, vars.clone()))
        },
        (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD { .. }, exp2 }, (true, vars)) => {
            let mut b: bool = false;
            let (_, (__pa0, _)) = isAddOrSubExp(exp1.clone(), (true, vars.clone()))?;
            b = __pa0.clone();
            let (_, (__pa1, _)) = isAddOrSubExp(exp2.clone(), (b.clone(), vars.clone()))?;
            b = __pa1.clone();
            (inExp.clone(), (b.clone(), vars.clone()))
        },
        (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::SUB { .. }, exp2 }, (true, vars)) => {
            let mut b: bool = false;
            let (_, (__pa0, _)) = isAddOrSubExp(exp1.clone(), (true, vars.clone()))?;
            b = __pa0.clone();
            let (_, (__pa1, _)) = isAddOrSubExp(exp2.clone(), (b.clone(), vars.clone()))?;
            b = __pa1.clone();
            (inExp.clone(), (b.clone(), vars.clone()))
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { componentRef: cref, .. }, operator: DAE::Operator::MUL { .. }, exp2 }, (true, vars)) => {
            let mut b: bool = false;
            b = BackendVariable::isState(cref.clone(), vars.clone())? && Expression::isConst(exp2.clone())?;
            (inExp.clone(), (b.clone(), vars.clone()))
        },
        (Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::MUL { .. }, exp2: Deref @ DAE::Exp::CREF { componentRef: cref, .. } }, (true, vars)) => {
            let mut b: bool = false;
            b = Expression::isConst(exp1.clone())? && BackendVariable::isState(cref.clone(), vars.clone())?;
            (inExp.clone(), (b.clone(), vars.clone()))
        },
        _ => {
            (inExp.clone(), (false, Util::tuple22(inTuple.clone())))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTuple))
}

fn sumUp2Expressions(mut sumUp: bool, mut exp1: Arc<DAE::Exp>, mut exp2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut expOut: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut op: DAE::Operator = <DAE::Operator as ::std::default::Default>::default();
    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ty = DAE::T_REAL_DEFAULT().clone();
    op = if (sumUp.clone()) {DAE::Operator::ADD { ty: ty.clone() }} else {DAE::Operator::SUB { ty: ty.clone() }};
    expOut = Arc::new(DAE::Exp::BINARY { exp1: exp1.clone(), operator: op.clone(), exp2: exp2.clone() });
    (expOut, _) = ExpressionSimplify::simplify(expOut.clone())?;
    Ok(expOut)
}

fn intLstIsEqual(mut lst1: Arc<metamodelica::List<i32>>, mut lst2: Arc<metamodelica::List<i32>>) -> Result<bool> {
    let mut bOut: bool = false;
    bOut = List::isEqualOnTrue(lst1.clone(), lst2.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    Ok(bOut)
}

fn sortPathsAsChain(mut pathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut pathsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
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
                    let mut pathLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
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
        bail!("matchcontinue: no arm matched")
    };
    Ok(pathsOut)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn sortPathsAsChain1(mut pathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut firstNode: i32, mut lastNode: i32, mut sortedPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut sortedPathsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    sortedPathsOut = 'mc: {
        let __mc_input = (pathsIn.clone(), firstNode.clone(), lastNode.clone(), sortedPathsIn.clone());
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
                    let mut startNode: i32 = 0;
                    let mut endNode: i32 = 0;
                    let mut sortedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
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
                    let mut endNode: i32 = 0;
                    let mut path: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rest: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut paths1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut paths2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut allPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut sortedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    paths1 = List::filter1OnTrue(pathsIn.clone(), (std::sync::Arc::new(firstInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), lastNode.clone())?;
                    paths2 = List::filter1OnTrue(pathsIn.clone(), (std::sync::Arc::new(lastInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), lastNode.clone())?;
                    allPaths = listAppend(paths1.clone(), paths2.clone());
                    let false = (allPaths.clone().is_empty()) else { bail!("pattern mismatch") };
                    path = listHead(allPaths.clone())?;
                    endNode = if (!(allPaths.clone().is_empty())) {List::last(path.clone())?} else {-1};
                    endNode = if (!(paths2.clone().is_empty())) {listHead(path.clone())?} else {-1};
                    (rest, _) = List::deleteMemberOnTrue(path.clone(), pathsIn.clone(), (std::sync::Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
                    sortedPaths = listAppend(sortedPathsIn.clone(), list![path.clone()]);
                    sortedPaths = sortPathsAsChain1(rest.clone(), firstNode.clone(), endNode.clone(), sortedPaths.clone())?;
                    Ok(sortedPaths.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut startNode: i32 = 0;
                    let mut path: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rest: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut paths1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut paths2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut allPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut sortedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    paths1 = List::filter1OnTrue(pathsIn.clone(), (std::sync::Arc::new(firstInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), firstNode.clone())?;
                    paths2 = List::filter1OnTrue(pathsIn.clone(), (std::sync::Arc::new(lastInListIsEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<bool> + 'static>), firstNode.clone())?;
                    allPaths = listAppend(paths1.clone(), paths2.clone());
                    let false = (allPaths.clone().is_empty()) else { bail!("pattern mismatch") };
                    path = listHead(allPaths.clone())?;
                    startNode = if (!(allPaths.clone().is_empty())) {List::last(path.clone())?} else {-1};
                    startNode = if (!(paths2.clone().is_empty())) {listHead(path.clone())?} else {-1};
                    (rest, _) = List::deleteMemberOnTrue(path.clone(), pathsIn.clone(), (std::sync::Arc::new({ let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| List::isEqualOnTrue(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
                    sortedPaths = metamodelica::cons(path.clone(), sortedPathsIn.clone());
                    sortedPaths = sortPathsAsChain1(rest.clone(), startNode.clone(), lastNode.clone(), sortedPaths.clone())?;
                    Ok(sortedPaths.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut startNode: i32 = 0;
                    let mut path: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut rest: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut sortedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(pathsIn.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    path = __pa0.clone();
                    rest = __pa1.clone();
                    sortedPaths = metamodelica::cons(path.clone(), sortedPathsIn.clone());
                    startNode = listHead(path.clone())?;
                    sortedPaths = sortPathsAsChain1(rest.clone(), startNode.clone(), lastNode.clone(), sortedPaths.clone())?;
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
    let mut isEq: bool = false;
    let mut first: i32 = 0;
    first = listHead(lstIn.clone())?;
    isEq = intEq(first.clone(), value.clone());
    Ok(isEq)
}

fn lastInListIsEqual(mut lstIn: Arc<metamodelica::List<i32>>, mut value: i32) -> Result<bool> {
    let mut isEq: bool = false;
    let mut last: i32 = 0;
    last = List::last(lstIn.clone())?;
    isEq = intEq(last.clone(), value.clone());
    Ok(isEq)
}

fn connect2PathsToLoops(mut pathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut loopsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut restPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut pathsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut restPathsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    (pathsOut, restPathsOut) = 'mc: {
        let __mc_input = pathsIn.clone();
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
                    let mut closedALoop: bool = false;
                    let mut startNode: i32 = 0;
                    let mut endNode: i32 = 0;
                    let mut loops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut restPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
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
                    let mut startNode: i32 = 0;
                    let mut endNode: i32 = 0;
                    let mut loops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut restPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
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
                    let mut closedALoop: bool = false;
                    let mut startNode: i32 = 0;
                    let mut endNode: i32 = 0;
                    let mut endPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut startPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut newLoops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut loops: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
                    let mut restPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
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
    let mut loopsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut path: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(pathIn.clone()) {
        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    path = __pa0.clone();
    path = List::stripLast(path.clone())?;
    loopsOut = List::map1(closingPaths.clone(), Arc::new(fnptr!(listAppend, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)), path.clone())?;
    Ok(loopsOut)
}

//____________________________________________________
//reshuffle systems of equations, not yet finished
//____________________________________________________
pub fn reshuffling_post(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    if Flags::isSet(Flags::RESHUFFLE_POST.clone())? {
        eqSystems = List::map1(inDAE.eqs.clone(), (std::sync::Arc::new(reshuffling_post0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), inDAE.shared.clone())?;
        outDAE = Arc::new(BackendDAE::BackendDAE { eqs: eqSystems.clone(), shared: inDAE.shared.clone() });
    } else {
        outDAE = inDAE.clone();
    }
    Ok(outDAE)
}

fn reshuffling_post0(mut isyst: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    osyst = List::fold1(comps.clone(), (std::sync::Arc::new(reshuffling_post1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, Arc<BackendDAE::Shared>, Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>), shared.clone(), isyst.clone())?;
    Ok(osyst)
}

fn reshuffling_post1(mut compIn: Arc<BackendDAE::StrongComponent>, mut shared: Arc<BackendDAE::Shared>, mut systIn: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut systOut: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    systOut = 'mc: {
        let __mc_input = compIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eqIdcs, vars: vIdcs, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: ojac }, jacType: jacType @ BackendDAE::JacobianType::JAC_LINEAR { .. }, .. } => {
                    let mut eqSys: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
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
        bail!("matchcontinue: no arm matched")
    };
    Ok(systOut)
}

fn reshuffling_post2(mut eqIdcs: Arc<metamodelica::List<i32>>, mut varIdcs: Arc<metamodelica::List<i32>>, mut dae: Arc<BackendDAE::EqSystem>, mut shared: Arc<BackendDAE::Shared>, mut ojac: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>>, mut jacType: BackendDAE::JacobianType) -> Result<(Arc<BackendDAE::EqSystem>, bool)> {
    let mut daeOut: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outRunMatching: bool = false;
    let mut size: i32 = 0;
    let mut resEqs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut ass1: metamodelica::Array<i32> = Default::default();
    let mut ass2: metamodelica::Array<i32> = Default::default();
    let mut ass1Sys: metamodelica::Array<i32> = Default::default();
    let mut ass2Sys: metamodelica::Array<i32> = Default::default();
    let mut varAtts: Arc<metamodelica::List<(bool, ArcStr)>> = metamodelica::nil();
    let mut eqAtts: Arc<metamodelica::List<(bool, ArcStr)>> = metamodelica::nil();
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
    let mut daeEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut daeVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut subSys: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>> = Default::default();
    let mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>> = Default::default();
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut eqsInLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
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
    varLst = List::map1r(varIdcs.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), daeVars.clone())?;
    vars = BackendVariable::listVar1(varLst.clone())?;
    subSys = BackendDAEUtil::createEqSystem(vars.clone(), eqs.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
    (me, meT, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(subSys.clone(), shared.clone(), false)?;
    (_, m, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(subSys.clone(), openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(BackendDAEUtil::getFunctions(shared.clone())?), BackendDAEUtil::isInitializationDAE(shared.clone()))?;
    ass1 = arrayCreate(size.clone(), -1);
    ass2 = arrayCreate(size.clone(), -1);
    varAtts = List::threadMap(List::fill(false, (varLst.clone().len() as i32)), List::map(eqIdcs.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
    eqAtts = List::threadMap(List::fill(false, (eqLst.clone().len() as i32)), List::map(varIdcs.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
    BackendDump::dumpBipartiteGraphStrongComponent2(vars.clone(), eqs.clone(), m.clone(), varAtts.clone(), eqAtts.clone(), (literal!("shuffle_pre")).clone())?;
    resEqs = reshuffling_post3_selectShuffleEqs(me.clone(), meT.clone())?;
    eqsInLst = reshuffling_post4_resolveAndReplace(resEqs.clone(), eqLst.clone(), varLst.clone(), me.clone(), meT.clone())?;
    daeEqs = List::threadFold(eqIdcs.clone(), eqsInLst.clone(), (std::sync::Arc::new(BackendEquation::setAtIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<BackendDAE::Equation>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> + 'static>), daeEqs.clone())?;
    daeOut = BackendDAEUtil::setEqSystEqs(dae.clone(), daeEqs.clone());
    daeOut = BackendDAEUtil::setEqSystMatching(daeOut.clone(), Arc::new(BackendDAE::Matching::MATCHING { ass1: ass1Sys.clone(), ass2: ass2Sys.clone(), comps: metamodelica::nil() }))?;
    (daeOut, _, _, _, _) = BackendDAEUtil::getAdjacencyMatrixScalar(daeOut.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(shared.clone()))?;
    outRunMatching = true;
    Ok((daeOut, outRunMatching))
}

fn reshuffling_post3_selectShuffleEqs(mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut resolveEqs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    resolveEqs = 'mc: {
        let __mc_input = meT.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut bArr: metamodelica::Array<bool> = Default::default();
            let mut suitableEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut eqPairs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
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
        bail!("matchcontinue: no arm matched")
    };
    Ok(resolveEqs)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn reshuffling_post4_resolveAndReplace(mut resolveEqLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut unassEqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut unassVarsIn: Arc<metamodelica::List<BackendDAE::Var>>, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut unassEqsOut: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    unassEqsOut = 'mc: {
        let __mc_input = resolveEqLst.clone();
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
                    let mut maxNum: i32 = 0;
                    let mut replEqIdx: i32 = 0;
                    let mut numOfAdjVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut unassEqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut resolvedEq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
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
    let mut lstOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    vars = List::map(metamodelica::arrayGet(me.clone(), eq.clone())?, std::sync::Arc::new(fnptr!(Util::tuple31, _)))?;
    eqs = List::map(List::flatten(List::map1(vars.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), meT.clone())?)?, std::sync::Arc::new(fnptr!(Util::tuple31, _)))?;
    eqs = getDoublicates(eqs.clone())?;
    lstOut = List::consOnTrue(!(listMember(eq.clone(), eqs.clone())), eq.clone(), eqs.clone());
    Ok(lstOut)
}

fn chooseEquation(mut row: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>) -> Result<bool> {
    let mut chooseThis: bool = false;
    let mut b1: bool = false;
    let mut b2: bool = false;
    let mut b3: bool = false;
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut numEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    vars = List::map(row.clone(), std::sync::Arc::new(fnptr!(Util::tuple31, _)))?;
    b1 = intEq((row.clone().len() as i32), 2);
    eqLst = List::mapList(List::map1(vars.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), meT.clone())?, std::sync::Arc::new(fnptr!(Util::tuple31, _)))?;
    numEqs = List::map(eqLst.clone(), std::sync::Arc::new(fnptr!(listLength, _)))?;
    b3 = List::applyAndFold1(numEqs.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 2, false)?;
    eqs = List::flatten(eqLst.clone())?;
    b2 = intEq((eqs.clone().len() as i32), (List::unique(eqs.clone()).len() as i32) + 2);
    b1 = b1.clone() && b2.clone() && b3.clone();
    chooseThis = b1.clone() && List::applyAndFold(row.clone(), (std::sync::Arc::new(fnptr!(boolAnd, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(isSolvable) as std::sync::Arc<dyn ::std::ops::Fn((i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> + 'static>), true)?;
    Ok(chooseThis)
}

fn getDoublicates(mut lstIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut lstOut: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut max: i32 = 0;
    let mut arr: metamodelica::Array<i32> = Default::default();
    max = List::fold(lstIn.clone(), (std::sync::Arc::new(fnptr!(intMax, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), listHead(lstIn.clone())?)?;
    arr = arrayCreate(max.clone(), -1);
    List::map1_0(lstIn.clone(), (std::sync::Arc::new(getDoublicates2) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<()> + 'static>), arr.clone())?;
    (_, lstOut) = List::filter1OnTrueSync(Arc::new(arr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), (std::sync::Arc::new(fnptr!(intGe, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 1, List::intRange(metamodelica::arrayLength(arr.clone())))?;
    Ok(lstOut)
}

fn getDoublicates2(mut idx: i32, mut arr: metamodelica::Array<i32>) -> Result<()> {
    let mut entry: i32 = 0;
    entry = metamodelica::arrayGet(arr.clone(), idx.clone())?;
    {let _arr = arr.clone(); let _idx = idx.clone(); let _val = entry.clone() + 1; _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
    Ok(())
}

fn isSolvable(mut entry: (i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)) -> Result<bool> {
    let mut solvable: bool = false;
    solvable = !(Tearing::unsolvable(list![entry.clone()])?);
    Ok(solvable)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn resolveEquations(mut eq: Option<Arc<BackendDAE::Equation>>, mut loopIn: Arc<metamodelica::List<i32>>, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut meT: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut eqsIn: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut varsIn: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<Arc<BackendDAE::Equation>> {
    let mut eqOut: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    eqOut = 'mc: {
        let __mc_input = (eq.clone(), loopIn.clone());
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
                    let mut nextEq: i32 = 0;
                    let mut sharedVar: i32 = 0;
                    let mut vars1: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vars2: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut numEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut eq1: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut eq2: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
                    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
                    let mut attr: BackendDAE::EquationAttributes = <BackendDAE::EquationAttributes as ::std::default::Default>::default();
                    let mut lhs1: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
                    let mut lhs2: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
                    let mut rhs1: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
                    let mut rhs2: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
                    let mut varExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
                    let mut eqExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
                    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
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
pub fn solveLinearSystem(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut maxSize: i32 = Flags::getConfigInt(Flags::MAX_SIZE_FOR_SOLVE_LINIEAR_SYSTEM.clone())?;
    let mut b: bool = 1 < maxSize.clone();
    if b.clone() {
        (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(inDAE.clone(), (std::sync::Arc::new(solveLinearSystem0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (bool, i32, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (bool, i32, i32))> + 'static>), (false, 1, maxSize.clone()))?;
    } else {
        outDAE = inDAE.clone();
    }
    Ok(outDAE)
}

fn solveLinearSystem0(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut inTpl: (bool, i32, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (bool, i32, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut outShared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut outTpl: (bool, i32, i32) = (false, 0, 0);
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    comps = __pa0.clone();
    (osyst, outShared, outTpl) = solveLinearSystem1(isyst.clone(), inShared.clone(), comps.clone(), inTpl.clone())?;
    Ok((osyst, outShared, outTpl))
}

fn solveLinearSystem1(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut inComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut inTpl: (bool, i32, i32)) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (bool, i32, i32))> {
    let mut osyst: Arc<BackendDAE::EqSystem> = isyst.clone();
    let mut oshared: Arc<BackendDAE::Shared> = ishared.clone();
    let mut outTpl: (bool, i32, i32) = (false, 0, 0);
    let mut b: bool = false;
    let mut runMatching: bool = false;
    let mut ii: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut offset: i32 = 0;
    let mut maxSize: i32 = 0;
    (runMatching, offset, maxSize) = inTpl.clone();
    for mut comp in &*inComps.clone() {
        let mut comp = comp.clone();
        (osyst, oshared, b, ii, offset) = solveLinearSystem2(osyst.clone(), oshared.clone(), comp.clone(), ii.clone(), offset.clone(), maxSize.clone())?;
        runMatching = runMatching.clone() || b.clone();
    }
    outTpl = (runMatching.clone(), offset.clone(), maxSize.clone());
    if runMatching.clone() {
        osyst = (::match_deref::match_deref! { match &(osyst.clone()) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. } => {
            let mut syst = (*syst).clone();
            let mut eqns = (*eqns).clone();
            eqns = List::fold(ii.clone(), (std::sync::Arc::new(BackendEquation::delete) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> + 'static>), eqns.clone())?;
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

fn solveLinearSystem2(mut isyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut comp: Arc<BackendDAE::StrongComponent>, mut ii: Arc<metamodelica::List<i32>>, mut offset: i32, mut maxSize: i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, bool, Arc<metamodelica::List<i32>>, i32)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut oshared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut outRunMatching: bool = false;
    let mut oi: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut offset_: i32 = 0;
    (osyst, oshared, outRunMatching, oi, offset_) = 'mc: {
        let __mc_input = (isyst.clone(), ishared.clone(), comp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. }, shared, Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eindex, vars: vindx, jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: Some(jac) }, jacType: BackendDAE::JacobianType::JAC_LINEAR { .. }, .. }) => {
                    let mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
                    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
                    let mut toffset: i32 = 0;
                    let mut syst = (*syst).clone();
                    let mut shared = (*shared).clone();
                    eqn_lst = BackendEquation::getList(eindex.clone(), eqns.clone())?;
                    var_lst = List::map1r(vindx.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), vars.clone())?;
                    let true = ((var_lst.clone().len() as i32) <= maxSize.clone()) else { bail!("pattern mismatch") };
                    ::match_deref::match_deref! { match &(List::splitOnTrue(var_lst.clone(), (std::sync::Arc::new(fnptr!(BackendVariable::isStateVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>))?) {
                        (Deref @ metamodelica::List::Nil, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    (syst, shared, toffset) = solveLinearSystem3(syst.clone(), shared.clone(), eqn_lst.clone(), eindex.clone(), var_lst.clone(), vindx.clone(), jac.clone(), offset.clone())?;
                    Ok((syst.clone(), shared.clone(), true, listAppend(eindex.clone(), ii.clone()), toffset.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), ishared.clone(), false, ii.clone(), offset.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((osyst, oshared, outRunMatching, oi, offset_))
}

fn solveLinearSystem3(mut inSyst: Arc<BackendDAE::EqSystem>, mut ishared: Arc<BackendDAE::Shared>, mut eqn_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut eqn_indxs: Arc<metamodelica::List<i32>>, mut var_lst: Arc<metamodelica::List<BackendDAE::Var>>, mut var_indxs: Arc<metamodelica::List<i32>>, mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut offset: i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut oshared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut offset_: i32 = 0;
    (osyst, oshared, offset_) = (::match_deref::match_deref! { match &((inSyst.clone(), ishared.clone())) {
        (syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. }, shared @ Deref @ BackendDAE::Shared { functionTree: funcs, .. }) => {
            let mut beqs: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut names: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut n: i32 = 0;
            let mut syst = (*syst).clone();
            let mut vars = (*vars).clone();
            let mut eqns = (*eqns).clone();
            let mut shared = (*shared).clone();
            (beqs, _) = BackendDAEUtil::getEqnSysRhs(BackendEquation::listEquation(eqn_lst.clone())?, BackendVariable::listVar1(var_lst.clone())?, Some(funcs.clone()))?;
            beqs = beqs.clone().reverse();
            n = (beqs.clone().len() as i32);
            names = List::map(var_lst.clone(), (std::sync::Arc::new(BackendVariable::varCref) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
            (eqns, vars, n, shared) = solveLinearSystem4(beqs.clone(), jac.clone(), names.clone(), var_lst.clone(), n.clone(), eqns.clone(), vars.clone(), offset.clone(), shared.clone())?;
            assign_field!(
                syst.orderedVars = vars.clone(),
                syst.orderedEqs = eqns.clone()
            );
            syst = BackendDAEUtil::setEqSystMatrices(syst.clone(), None, None, None)?;
            (syst.clone(), shared.clone(), n.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((osyst, oshared, offset_))
}

fn solveLinearSystem4(mut b_lst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut jac: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>, mut cr_x: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut var_lst: Arc<metamodelica::List<BackendDAE::Var>>, mut n: i32, mut ieqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ivars: BackendDAE::Variables, mut offset: i32, mut ishared: Arc<BackendDAE::Shared>) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, i32, Arc<BackendDAE::Shared>)> {
    let mut oeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = ieqns.clone();
    let mut ovars: BackendDAE::Variables = ivars.clone();
    let mut offset_: i32 = offset.clone() + 1;
    let mut oshared: Arc<BackendDAE::Shared> = ishared.clone();
    let mut R: metamodelica::Array<Arc<DAE::Exp>> = Default::default();
    let mut Qb: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut b: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut A: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n.clone() * n.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut ax: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut scaled_x: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut scaleA: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut a: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut m: i32 = 0;
    let mut ii: i32 = 0;
    let mut jj: i32 = 0;
    let mut mm: i32 = 0;
    let mut x_lst: Arc<metamodelica::List<Arc<DAE::Exp>>> = List::map(cr_x.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>))?;
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = var_lst.clone();
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut jac_: Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>> = jac.clone();
    mm = (jac.clone().len() as i32);
    for mut i in 1..=mm.clone() {
        let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(jac_.clone()) {
            Deref @ metamodelica::List::Cons { head: (__pa0, __pa1, Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: __pa2, .. }), tail: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        jj = __pa0.clone();
        ii = __pa1.clone();
        a = __pa2.clone();
        jac_ = __pa3.clone();
        m = ii.clone() + (jj.clone() - 1) * n.clone();
        (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(a.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$A$")); __mm_s.push_str(&*intString(m.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        {let _arr = A.clone(); let _idx = m.clone(); let _val = a.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
    }
    for mut i in 1..=n.clone() {
        m = (i.clone() - 1) * n.clone();
        a = Expression::makeSum1(({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut j in (1..=n.clone()).into_iter() {
            let __x = Expression::makeAbs(metamodelica::arrayGet(A.clone(), m.clone() + j.clone())?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), false)?;
        {let _arr = scaleA.clone(); let _idx = i.clone(); let _val = a.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
    }
    for mut i in 1..=n.clone() {
        m = (i.clone() - 1) * n.clone();
        for mut j in 1..=n.clone() {
            a = metamodelica::arrayGet(A.clone(), j.clone() + m.clone())?;
            if !(Expression::isZero(a.clone())?) {
                a = Expression::expDiv(a.clone(), metamodelica::arrayGet(scaleA.clone(), j.clone())?)?;
                (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(a.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$sA$")); __mm_s.push_str(&*intString(i.clone() + (j.clone() - 1) * n.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
                {let _arr = A.clone(); let _idx = j.clone() + m.clone(); let _val = a.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
            }
        }
    }
    m = 1;
    for mut b_ in &*b_lst.clone() {
        let mut b_ = b_.clone();
        (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(b_.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$b$")); __mm_s.push_str(&*intString(m.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        {let _arr = b.clone(); let _idx = m.clone(); let _val = a.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
        m = m.clone() + 1;
    }
    m = 1;
    for mut xx in &*x_lst.clone() {
        let mut xx = xx.clone();
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(vars.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        var = __pa4.clone();
        vars = __pa5.clone();
        if BackendVariable::isStateVar(var.clone()) {
            {let _arr = ax.clone(); let _idx = m.clone(); let _val = Expression::expDer(xx.clone()); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
        } else {
            {let _arr = ax.clone(); let _idx = m.clone(); let _val = xx.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
        }
        m = m.clone() + 1;
    }
    for mut i in 1..=n.clone() {
        a = Expression::expMul(metamodelica::arrayGet(ax.clone(), i.clone())?, metamodelica::arrayGet(scaleA.clone(), i.clone())?)?;
        (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(a.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$sx$")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        {let _arr = scaled_x.clone(); let _idx = i.clone(); let _val = a.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
    }
    (R, Qb, oeqns, ovars, oshared) = qrDecompositionHouseholder(A.clone(), n.clone(), b.clone(), oeqns.clone(), ovars.clone(), offset.clone(), oshared.clone())?;
    for mut i in ({let __s=n.clone(); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        m = (i.clone() - 1) * n.clone();
        a = Expression::makeSum1(({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut j in (i.clone()..=n.clone()).into_iter() {
            let __x = Expression::expMul(metamodelica::arrayGet(R.clone(), m.clone() + j.clone())?, metamodelica::arrayGet(scaled_x.clone(), j.clone())?)?;
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
    let mut cA: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut v: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut alpha: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut y1: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut h: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut h2: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut e1: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut e2: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut e: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut m: i32 = 0;
    let mut nn: i32 = n.clone() - 1;
    let mut idxVars: i32 = 1;
    let mut shift: i32 = 0;
    for mut iter in 1..=nn.clone() {
        m = n.clone() - iter.clone() + 1;
        qrGet_cA(A.clone(), iter.clone(), 1, n.clone(), v.clone())?;
        y1 = metamodelica::arrayGet(v.clone(), 1)?;
        alpha = qrCalc_alpha(v.clone(), y1.clone(), m.clone())?;
        (alpha, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(alpha.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$a$")); __mm_s.push_str(&*intString(iter.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        e = Expression::expAdd(y1.clone(), alpha.clone())?;
        (e, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(e.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$y1$")); __mm_s.push_str(&*intString(iter.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        {let _arr = v.clone(); let _idx = 1; let _val = e.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
        h = Expression::expAdd(y1.clone(), alpha.clone())?;
        h = Expression::expMul(alpha.clone(), h.clone())?;
        h = Expression::negate(h.clone())?;
        (h, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(h.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$h$")); __mm_s.push_str(&*intString(iter.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        shift = (iter.clone() - 1) * n.clone() + iter.clone();
        {let _arr = R.clone(); let _idx = shift.clone(); let _val = Expression::negate(alpha.clone())?; _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
        for mut j in 2..=m.clone() {
            {let _arr = R.clone(); let _idx = shift.clone() + (j.clone() - 1) * n.clone(); let _val = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
        }
        for mut col in 2..=m.clone() {
            qrGet_cA(A.clone(), iter.clone(), col.clone(), n.clone(), cA.clone())?;
            h2 = Expression::makeScalarProduct(v.clone(), cA.clone())?;
            h2 = Expression::expDiv(h2.clone(), h.clone())?;
            (h2, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(h2.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$h2$")); __mm_s.push_str(&*intString(idxVars.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
            idxVars = idxVars.clone() + 1;
            for mut j in 1..=m.clone() {
                e1 = metamodelica::arrayGet(cA.clone(), j.clone())?;
                e2 = metamodelica::arrayGet(v.clone(), j.clone())?;
                e = Expression::expAdd(e1.clone(), Expression::expMul(h2.clone(), e2.clone())?)?;
                (e, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(e.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$R$")); __mm_s.push_str(&*intString(idxVars.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
                idxVars = idxVars.clone() + 1;
                {let _arr = A.clone(); let _idx = shift.clone() + (j.clone() - 1) * n.clone() + col.clone() - 1; let _val = e.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
            }
        }
        for mut j in 1..=m.clone() {
            {let _arr = cA.clone(); let _idx = j.clone(); let _val = metamodelica::arrayGet(b.clone(), iter.clone() - 1 + j.clone())?; _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
        }
        h2 = Expression::makeScalarProduct(v.clone(), cA.clone())?;
        h2 = Expression::expDiv(h2.clone(), h.clone())?;
        (h2, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(h2.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$b_$")); __mm_s.push_str(&*intString(idxVars.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        idxVars = idxVars.clone() + 1;
        for mut j in 1..=m.clone() {
            e1 = metamodelica::arrayGet(cA.clone(), j.clone())?;
            e2 = metamodelica::arrayGet(v.clone(), j.clone())?;
            e = Expression::expAdd(e1.clone(), Expression::expMul(h2.clone(), e2.clone())?)?;
            e = Expression::expand(e.clone())?;
            e = ExpressionSimplify::simplify2(e.clone(), true, true)?;
            (e, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(e.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$b$")); __mm_s.push_str(&*intString(idxVars.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
            idxVars = idxVars.clone() + 1;
            {let _arr = b.clone(); let _idx = iter.clone() - 1 + j.clone(); let _val = e.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
        }
    }
    Ok((R, b, oeqns, ovars, oshared))
}

fn qrGet_cA(mut A: metamodelica::Array<Arc<DAE::Exp>>, mut iter: i32, mut j: i32, mut n: i32, mut cA: metamodelica::Array<Arc<DAE::Exp>>) -> Result<()> {
    let mut shift: i32 = (iter.clone() - 1) * n.clone() + iter.clone() + j.clone() - 1;
    let mut m: i32 = n.clone() - iter.clone() + 1;
    for mut i in 1..=m.clone() {
        {let _arr = cA.clone(); let _idx = i.clone(); let _val = metamodelica::arrayGet(A.clone(), shift.clone() + (i.clone() - 1) * n.clone())?; _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
    }
    for mut i in m.clone() + 1..=n.clone() {
        {let _arr = cA.clone(); let _idx = i.clone(); let _val = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
    }
    Ok(())
}

fn qrCalc_alpha(mut y: metamodelica::Array<Arc<DAE::Exp>>, mut y1: Arc<DAE::Exp>, mut m: i32) -> Result<Arc<DAE::Exp>> {
    let mut alpha: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut sgn_y1: Arc<DAE::Exp> = Expression::makeSign(y1.clone());
    let mut norm_y: Arc<DAE::Exp> = Expression::lenVec(y.clone())?;
    norm_y = Expression::makeSum1(({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut j in (1..=m.clone()).into_iter() {
            let __x = Expression::expPow(metamodelica::arrayGet(y.clone(), j.clone())?, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), false)?;
    norm_y = Expression::makePureBuiltinCall((literal!("sqrt")).clone(), list![norm_y.clone()], DAE::T_REAL_DEFAULT().clone());
    alpha = Expression::expMul(sgn_y1.clone(), norm_y.clone())?;
    Ok(alpha)
}

fn qrDecomposition(mut A: metamodelica::Array<Arc<DAE::Exp>>, mut n: i32, mut ib: metamodelica::Array<Arc<DAE::Exp>>, mut ieqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ivars: BackendDAE::Variables, mut offset: i32, mut ishared: Arc<BackendDAE::Shared>) -> Result<(metamodelica::Array<Arc<DAE::Exp>>, metamodelica::Array<Arc<DAE::Exp>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, Arc<BackendDAE::Shared>)> {
    let mut R: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n.clone() * n.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut b: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut oeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = ieqns.clone();
    let mut ovars: BackendDAE::Variables = ivars.clone();
    let mut oshared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut Q: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n.clone() * n.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut v: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut u: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(n.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut x: metamodelica::Array<Arc<DAE::Exp>> = Default::default();
    let mut y: metamodelica::Array<Arc<DAE::Exp>> = Default::default();
    let mut a: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut kk: i32 = 1;
    let mut m: i32 = n.clone() - 1;
    let mut nn: i32 = 0;
    v = qrDecomposition1(A.clone(), n.clone(), kk.clone())?;
    (u, oeqns, ovars, oshared) = BackendEquation::normalizationVec(v.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$NOM$")); __mm_s.push_str(&*intString(kk.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), ishared.clone())?;
    for mut j in 1..=n.clone() {
        (a, _) = ExpressionSimplify::simplify(metamodelica::arrayGet(u.clone(), j.clone())?)?;
        (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(a.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$Q$")); __mm_s.push_str(&*intString(kk.clone() + (j.clone() - 1) * n.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        {let _arr = Q.clone(); let _idx = kk.clone() + (j.clone() - 1) * n.clone(); let _val = a.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
    }
    for mut k in 1..=m.clone() {
        v = qrDecomposition1(A.clone(), n.clone(), k.clone() + 1)?;
        for mut j in 1..=k.clone() {
            u = qrDecomposition1(Q.clone(), n.clone(), j.clone())?;
            (v, oeqns, ovars, oshared) = gramSchmidtProcessHelper(v.clone(), u.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$W$")); __mm_s.push_str(&*intString(kk.clone())); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*intString(kk.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone())?;
            kk = kk.clone() + 1;
        }
        (u, oeqns, ovars, oshared) = BackendEquation::normalizationVec(v.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$NOM$")); __mm_s.push_str(&*intString(k.clone() + 1)); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone())?;
        for mut j in 1..=n.clone() {
            nn = k.clone() + 1 + (j.clone() - 1) * n.clone();
            (a, _) = ExpressionSimplify::simplify(metamodelica::arrayGet(u.clone(), j.clone())?)?;
            (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(a.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$Q$")); __mm_s.push_str(&*intString(nn.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
            {let _arr = Q.clone(); let _idx = nn.clone(); let _val = a.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
        }
    }
    for mut i in 1..=n.clone() {
        x = qrDecomposition1(Q.clone(), n.clone(), i.clone())?;
        m = (i.clone() - 1) * n.clone();
        for mut j in i.clone()..=n.clone() {
            y = qrDecomposition1(A.clone(), n.clone(), j.clone())?;
            a = Expression::makeScalarProduct(x.clone(), y.clone())?;
            (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(a.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$R$")); __mm_s.push_str(&*intString(m.clone() + j.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
            {let _arr = R.clone(); let _idx = m.clone() + j.clone(); let _val = a.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
        }
    }
    for mut i in 1..=n.clone() {
        x = qrDecomposition1(Q.clone(), n.clone(), i.clone())?;
        a = Expression::makeScalarProduct(x.clone(), ib.clone())?;
        (a, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(a.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("QR$Qb$")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        {let _arr = b.clone(); let _idx = i.clone(); let _val = a.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
    }
    Ok((R, b, oeqns, ovars, oshared))
}

fn qrDecomposition1(mut A: metamodelica::Array<Arc<DAE::Exp>>, mut sizeA: i32, mut i: i32) -> Result<metamodelica::Array<Arc<DAE::Exp>>> {
    let mut column: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(sizeA.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    for mut j in 1..=sizeA.clone() {
        {let _arr = column.clone(); let _idx = j.clone(); let _val = metamodelica::arrayGet(A.clone(), i.clone() + (j.clone() - 1) * sizeA.clone())?; _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
    }
    Ok(column)
}

fn qrDecomposition2(mut A: metamodelica::Array<Arc<DAE::Exp>>, mut sizeA: i32, mut i: i32) -> Result<metamodelica::Array<Arc<DAE::Exp>>> {
    let mut row: metamodelica::Array<Arc<DAE::Exp>> = arrayCreate(sizeA.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    let mut k: i32 = i.clone() - 1;
    for mut j in 1..=sizeA.clone() {
        {let _arr = row.clone(); let _idx = j.clone(); let _val = metamodelica::arrayGet(A.clone(), j.clone() + k.clone() * sizeA.clone())?; _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
    }
    Ok(row)
}

fn qrDecomposition3(mut A: metamodelica::Array<Arc<DAE::Exp>>, mut sizeA: i32, mut isMat: bool, mut s: ArcStr) -> Result<()> {
    let mut n: i32 = sizeA.clone();
    let mut m: i32 = if (isMat.clone()) {sizeA.clone()} else {1};
    metamodelica::print((literal!("\n")).clone());
    for mut i in 1..=n.clone() {
        metamodelica::print((literal!("\n")).clone());
        for mut j in 1..=m.clone() {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(j.clone())); __mm_s.push_str(&*literal!(") = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(metamodelica::arrayGet(A.clone(), (i.clone() - 1) * m.clone() + j.clone())?)?); __mm_s.push_str(&*literal!("\t")); ArcStr::from(__mm_s) }).clone());
        }
    }
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

fn gramSchmidtProcessHelper(mut w: metamodelica::Array<Arc<DAE::Exp>>, mut u: metamodelica::Array<Arc<DAE::Exp>>, mut name: ArcStr, mut offset: i32, mut ieqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut ivars: BackendDAE::Variables, mut ishared: Arc<BackendDAE::Shared>) -> Result<(metamodelica::Array<Arc<DAE::Exp>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, Arc<BackendDAE::Shared>)> {
    let mut v: metamodelica::Array<Arc<DAE::Exp>> = Default::default();
    let mut oeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
    let mut ovars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut oshared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut h: Arc<DAE::Exp> = Expression::makeScalarProduct(w.clone(), u.clone())?;
    let mut n: i32 = metamodelica::arrayLength(w.clone());
    (h, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(h.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("_h")); ArcStr::from(__mm_s) }).clone(), offset.clone(), ieqns.clone(), ivars.clone(), ishared.clone(), false)?;
    v = Array::map1(u.clone(), (std::sync::Arc::new(Expression::expMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), h.clone())?;
    v = Expression::subVec(w.clone(), v.clone())?;
    for mut i in 1..=n.clone() {
        (h, oeqns, ovars, oshared, _, _) = BackendEquation::makeTmpEqnForExp(metamodelica::arrayGet(v.clone(), i.clone())?, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), offset.clone(), oeqns.clone(), ovars.clone(), oshared.clone(), false)?;
        {let _arr = v.clone(); let _idx = i.clone(); let _val = h.clone(); _arr.borrow_mut()[(_idx-1) as usize] = _val; _arr};
    }
    Ok((v, oeqns, ovars, oshared))
}

