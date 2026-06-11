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
use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::HpcOmBenchmark;
use crate::HpcOmScheduler;
use crate::SimCodeUtil;
use openmodelica_backend_types::BackendDAE;
use openmodelica_codegen_graphml::GraphML;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::DAEDumpTypes;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_simcode_types::HpcOmSimCode;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

//----------------------------
//  Graph Structure
//----------------------------
pub type TaskGraph = metamodelica::Array<Arc<metamodelica::List<i32>>>;

pub type Communications = Arc<metamodelica::List<Communication>>;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Communication {
    pub numberOfVars: i32,
    pub integerVars: Arc<metamodelica::List<i32>>,
    pub floatVars: Arc<metamodelica::List<i32>>,
    pub booleanVars: Arc<metamodelica::List<i32>>,
    pub stringVars: Arc<metamodelica::List<i32>>,
    pub childNode: i32,
    pub requiredTime: metamodelica::Real,
}

impl metamodelica::gc::MMTrace for Communication {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.numberOfVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.integerVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.floatVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.booleanVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stringVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.childNode, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.requiredTime, __mmv)?;
        Ok(())
    }
}
impl Default for Communication {
    fn default() -> Self {
        Self {
            numberOfVars: Default::default(),
            integerVars: Default::default(),
            floatVars: Default::default(),
            booleanVars: Default::default(),
            stringVars: Default::default(),
            childNode: Default::default(),
            requiredTime: Default::default(),
        }
    }
}

pub type COMMUNICATION = Communication;


#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ComponentInfo {
    pub isPartOfODESystem: bool,
    pub isPartOfZeroFuncSystem: bool,
    pub isRemovedComponent: bool,
}

impl metamodelica::gc::MMTrace for ComponentInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.isPartOfODESystem, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isPartOfZeroFuncSystem, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isRemovedComponent, __mmv)?;
        Ok(())
    }
}
impl Default for ComponentInfo {
    fn default() -> Self {
        Self {
            isPartOfODESystem: Default::default(),
            isPartOfZeroFuncSystem: Default::default(),
            isRemovedComponent: Default::default(),
        }
    }
}

pub type COMPONENTINFO = ComponentInfo;


// TODO: Store compParamMapping, compNames and compDescs in ComponentInfo
// TODO: Change nodeMark to compMarks
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct TaskGraphMeta {
    pub inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>,
    pub varCompMapping: metamodelica::Array<(i32, i32, i32)>,
    pub eqCompMapping: metamodelica::Array<(i32, i32, i32)>,
    pub compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>,
    pub compNames: metamodelica::Array<ArcStr>,
    pub compDescs: metamodelica::Array<ArcStr>,
    pub exeCosts: metamodelica::Array<(i32, metamodelica::Real)>,
    pub commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>,
    pub nodeMark: metamodelica::Array<i32>,
    pub compInformations: metamodelica::Array<ComponentInfo>,
}

impl metamodelica::gc::MMTrace for TaskGraphMeta {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.inComps, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.varCompMapping, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.eqCompMapping, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.compParamMapping, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.compNames, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.compDescs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.exeCosts, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.commCosts, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nodeMark, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.compInformations, __mmv)?;
        Ok(())
    }
}
impl Default for TaskGraphMeta {
    fn default() -> Self {
        Self {
            inComps: Default::default(),
            varCompMapping: Default::default(),
            eqCompMapping: Default::default(),
            compParamMapping: Default::default(),
            compNames: Default::default(),
            compDescs: Default::default(),
            exeCosts: Default::default(),
            commCosts: Default::default(),
            nodeMark: Default::default(),
            compInformations: Default::default(),
        }
    }
}

pub type TASKGRAPHMETA = TaskGraphMeta;


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum VariableType {
    INTEGER = 1,
    REAL = 2,
    BOOLEAN = 3,
    STRING = 4,
}
impl PartialOrd for VariableType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for VariableType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl metamodelica::gc::MMTrace for VariableType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> { Ok(()) }
}

pub type VariableList = (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);

//variables <int, float, bool, string>
//----------------------------------------------------------
//  Functions to build the task graph from the BLT structure
//----------------------------------------------------------
pub(crate) fn createTaskGraph(mut iDAE: Arc<BackendDAE::BackendDAE>, mut iAnalyzeParameters: bool) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut oGraph: TaskGraph;
    let mut oGraphData: TaskGraphMeta;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut graph: TaskGraph;
    let mut graphData: TaskGraphMeta;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    shared = __pa1.clone();
    (graph, graphData) = getEmptyTaskGraph(0, 0, 0);
    (oGraph, oGraphData, _) = List::fold(systs, (std::sync::Arc::new({ let __pe_b1 = shared; let __pe_b2 = iAnalyzeParameters; move |__pe_a0, __pe_a3| createTaskGraph0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (metamodelica::Array<Arc<metamodelica::List<i32>>>, TaskGraphMeta, i32)) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, TaskGraphMeta, i32)> + 'static>), (graph.clone(), graphData, 1))?;
    Ok((oGraph, oGraphData))
}

pub(crate) fn createTaskGraph0(mut iSyst: Arc<BackendDAE::EqSystem>, mut iShared: Arc<BackendDAE::Shared>, mut iAnalyzeParameters: bool, mut iGraphInfo: (metamodelica::Array<Arc<metamodelica::List<i32>>>, TaskGraphMeta, i32)) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, TaskGraphMeta, i32)> {
    let mut oGrapInfo: (metamodelica::Array<Arc<metamodelica::List<i32>>>, TaskGraphMeta, i32);
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut vars: BackendDAE::Variables;
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut sharedFuncs: Arc<AvlTreePathFunction::Tree>;
    let mut iGraphData: TaskGraphMeta;
    let mut tmpGraphData: TaskGraphMeta;
    let mut iGraph: TaskGraph;
    let mut tmpGraph: TaskGraph;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut numberOfVars: i32;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let mut eqSysIdx: i32;
    let mut matching: Arc<BackendDAE::Matching>;
    let mut adjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(iSyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: __pa0, orderedVars: __pa1, orderedEqs: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    matching = __pa0.clone();
    vars = __pa1.clone();
    orderedEqs = __pa2.clone();
    comps = BackendDAEUtil::getCompsOfMatching(matching);
    let __pa3 = ::match_deref::match_deref! { match &(iShared.clone()) {
        Deref @ BackendDAE::Shared { functionTree: __pa3, .. } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    sharedFuncs = __pa3.clone();
    (iGraph, iGraphData, eqSysIdx) = iGraphInfo;
    (_, adjacencyMatrix, _) = BackendDAEUtil::getAdjacencyMatrix(iSyst.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(sharedFuncs), BackendDAEUtil::isInitializationDAE(iShared.clone()))?;
    numberOfVars = BackendVariable::varsSize(vars);
    (tmpGraph, tmpGraphData) = getEmptyTaskGraph((comps.clone().len() as i32), numberOfVars, ExpandableArray::getNumberOfElements(orderedEqs));
    let TaskGraphMeta { inComps: __pa4, compNames: __pa5, exeCosts: __pa6, commCosts: __pa7, nodeMark: __pa8, varCompMapping: __pa9, eqCompMapping: __pa10, compParamMapping: __pa11, compInformations: __pa12, .. } = (tmpGraphData) else { bail!("pattern mismatch") };
    inComps = __pa4.clone();
    compNames = __pa5.clone();
    exeCosts = __pa6.clone();
    commCosts = __pa7.clone();
    nodeMark = __pa8.clone();
    varCompMapping = __pa9.clone();
    eqCompMapping = __pa10.clone();
    compParamMapping = __pa11.clone();
    compInformations = __pa12.clone();
    (varCompMapping, eqCompMapping) = getVarEqCompMapping(comps.clone(), eqSysIdx, 0, 0, varCompMapping.clone(), eqCompMapping.clone())?;
    compDescs = getEquationStrings(comps.clone(), iSyst.clone())?;
    (tmpGraph, inComps, compParamMapping, commCosts, compNames, nodeMark, _) = List::fold(comps.clone(), (std::sync::Arc::new({ let __pe_b1 = (adjacencyMatrix.clone(), iSyst, iShared, (comps.len() as i32)); let __pe_b2 = (varCompMapping.clone(), eqCompMapping.clone(), metamodelica::nil()); let __pe_b3 = iAnalyzeParameters; move |__pe_a0, __pe_a4| createTaskGraph1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<Communication>>>, metamodelica::Array<ArcStr>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<Communication>>>, metamodelica::Array<ArcStr>, metamodelica::Array<i32>, i32)> + 'static>), (tmpGraph.clone(), inComps.clone(), compParamMapping.clone(), commCosts.clone(), compNames.clone(), nodeMark.clone(), 1))?;
    tmpGraph = Array::mapNoCopy(tmpGraph.clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0| List::sort(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
    tmpGraphData = TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    if intGt(eqSysIdx, 1) {
        (tmpGraph, tmpGraphData) = taskGraphAppend(iGraph.clone(), iGraphData, tmpGraph.clone(), tmpGraphData)?;
    }
    oGrapInfo = (tmpGraph.clone(), tmpGraphData, eqSysIdx + 1);
    Ok(oGrapInfo)
}

pub(crate) fn getSystemComponents(mut iDae: Arc<BackendDAE::BackendDAE>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>)> {
    let mut oComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut oMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let mut tmpSystems: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>> = metamodelica::nil();
    let mut tmpComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    (oComps, oMapping) = (::match_deref::match_deref! { match &(iDae) {
        Deref @ BackendDAE::BackendDAE { eqs: __esc_systs, .. } => {
            systs = (*__esc_systs).clone();
            (tmpComps, tmpSystems, _) = List::fold(systs.clone(), (std::sync::Arc::new(getSystemComponents0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>, i32)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>, i32)> + 'static>), (metamodelica::nil(), metamodelica::nil(), 1))?;
            (tmpComps, metamodelica::arrayFromVec(tmpSystems.into_iter().cloned().collect()))
        },
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oComps, oMapping))
}

fn getSystemComponents0(mut iSyst: Arc<BackendDAE::EqSystem>, mut iSystMapping: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>, i32)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>, i32)> {
    let mut oSystMapping: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>, i32);
    let mut tmpComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut matching: Arc<BackendDAE::Matching> = Arc::new(BackendDAE::Matching::NO_MATCHING);
    let mut tmpSystMapping: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>> = metamodelica::nil();
    let mut currentIdx: i32 = 0;
    oSystMapping = (::match_deref::match_deref! { match &((iSyst.clone(), iSystMapping)) {
        (Deref @ BackendDAE::EqSystem { matching: __esc_matching, .. }, (__esc_tmpComps, __esc_tmpSystMapping, __esc_currentIdx)) => {
            matching = (*__esc_matching).clone();
            tmpComps = (*__esc_tmpComps).clone();
            tmpSystMapping = (*__esc_tmpSystMapping).clone();
            currentIdx = (*__esc_currentIdx).clone();
            comps = BackendDAEUtil::getCompsOfMatching(matching.clone());
            tmpSystMapping = List::fold2(comps.clone(), (std::sync::Arc::new(fnptr!(getSystemComponents1, Arc<BackendDAE::StrongComponent>, Arc<BackendDAE::EqSystem>, i32, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, Arc<BackendDAE::EqSystem>, i32, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>> + 'static>), iSyst, currentIdx.clone(), tmpSystMapping.clone())?;
            comps = listAppend(tmpComps.clone(), comps);
            (comps, tmpSystMapping.clone(), currentIdx.clone() + 1)
        },
        _ => {
            metamodelica::print((literal!("getSystemComponents0 failed\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oSystMapping)
}

fn getSystemComponents1(mut icomp: Arc<BackendDAE::StrongComponent>, mut isyst: Arc<BackendDAE::EqSystem>, mut isystIdx: i32, mut iMapping: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>) -> Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>> {
    let mut oMapping: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>;
    oMapping = listAppend(iMapping, list![(isyst, isystIdx)]);
    oMapping
}

fn getNumberOfSystemComponents(mut iDae: Arc<BackendDAE::BackendDAE>) -> Result<i32> {
    let mut oNumOfComps: i32;
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let __pa0 = ::match_deref::match_deref! { match &(iDae) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqs = __pa0.clone();
    oNumOfComps = List::fold(eqs, (std::sync::Arc::new(getNumberOfEqSystemComponents) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, i32) -> Result<i32> + 'static>), 0)?;
    Ok(oNumOfComps)
}

fn getNumberOfEqSystemComponents(mut iEqSystem: Arc<BackendDAE::EqSystem>, mut iNumOfComps: i32) -> Result<i32> {
    let mut oNumOfComps: i32;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut matching: Arc<BackendDAE::Matching>;
    let __pa0 = ::match_deref::match_deref! { match &(iEqSystem) {
        Deref @ BackendDAE::EqSystem { matching: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    matching = __pa0.clone();
    comps = BackendDAEUtil::getCompsOfMatching(matching);
    oNumOfComps = iNumOfComps + (comps.len() as i32);
    Ok(oNumOfComps)
}

pub(crate) fn getEmptyTaskGraph(mut numComps: i32, mut numVars: i32, mut numEqs: i32) -> (TaskGraph, TaskGraphMeta) {
    let mut graph: TaskGraph;
    let mut graphData: TaskGraphMeta;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    graph = arrayCreate(numComps, metamodelica::nil());
    inComps = arrayCreate(numComps, metamodelica::nil());
    compParamMapping = arrayCreate(numComps, metamodelica::nil());
    varCompMapping = arrayCreate(numVars, (0, 0, 0));
    eqCompMapping = arrayCreate(numEqs, (0, 0, 0));
    compNames = arrayCreate(numComps, (literal!("")).clone());
    compDescs = arrayCreate(numComps, (literal!("")).clone());
    exeCosts = arrayCreate(numComps, (-1, metamodelica::OrderedFloat(-1.0_f64)));
    commCosts = arrayCreate(numComps, metamodelica::nil());
    nodeMark = arrayCreate(numComps, 0);
    compInformations = arrayCreate(numComps, ComponentInfo { isPartOfODESystem: false, isPartOfZeroFuncSystem: false, isRemovedComponent: false });
    graphData = TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    (graph, graphData)
}

pub(crate) fn copyTaskGraphMeta(mut graphDataIn: TaskGraphMeta) -> Result<TaskGraphMeta> {
    let mut graphDataOut: TaskGraphMeta;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inComps1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut varCompMapping1: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping1: metamodelica::Array<(i32, i32, i32)>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compParamMapping1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compNames1: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut compDescs1: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut exeCosts1: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut commCosts1: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut nodeMark1: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let mut compInformations1: metamodelica::Array<ComponentInfo>;
    let TaskGraphMeta { inComps: __pa0, varCompMapping: __pa1, eqCompMapping: __pa2, compParamMapping: __pa3, compNames: __pa4, compDescs: __pa5, exeCosts: __pa6, commCosts: __pa7, nodeMark: __pa8, compInformations: __pa9 } = (graphDataIn) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    varCompMapping = __pa1.clone();
    eqCompMapping = __pa2.clone();
    compParamMapping = __pa3.clone();
    compNames = __pa4.clone();
    compDescs = __pa5.clone();
    exeCosts = __pa6.clone();
    commCosts = __pa7.clone();
    nodeMark = __pa8.clone();
    compInformations = __pa9.clone();
    inComps1 = metamodelica::arrayFromVec(inComps.clone().borrow().clone());
    varCompMapping1 = metamodelica::arrayFromVec(varCompMapping.clone().borrow().clone());
    eqCompMapping1 = metamodelica::arrayFromVec(eqCompMapping.clone().borrow().clone());
    compParamMapping1 = metamodelica::arrayFromVec(compParamMapping.clone().borrow().clone());
    compNames1 = metamodelica::arrayFromVec(compNames.clone().borrow().clone());
    compDescs1 = metamodelica::arrayFromVec(compDescs.clone().borrow().clone());
    exeCosts1 = metamodelica::arrayFromVec(exeCosts.clone().borrow().clone());
    commCosts1 = metamodelica::arrayFromVec(commCosts.clone().borrow().clone());
    nodeMark1 = metamodelica::arrayFromVec(nodeMark.clone().borrow().clone());
    compInformations1 = metamodelica::arrayFromVec(compInformations.clone().borrow().clone());
    graphDataOut = TaskGraphMeta { inComps: inComps1.clone(), varCompMapping: varCompMapping1.clone(), eqCompMapping: eqCompMapping1.clone(), compParamMapping: compParamMapping1.clone(), compNames: compNames1.clone(), compDescs: compDescs1.clone(), exeCosts: exeCosts1.clone(), commCosts: commCosts1.clone(), nodeMark: nodeMark1.clone(), compInformations: compInformations1.clone() };
    Ok(graphDataOut)
}

fn taskGraphAppend(mut graph1In: TaskGraph, mut graphData1In: TaskGraphMeta, mut graph2In: TaskGraph, mut graphData2In: TaskGraphMeta) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut graphOut: TaskGraph;
    let mut graphDataOut: TaskGraphMeta;
    let mut eqOffset: i32;
    let mut idxOffset: i32;
    let mut varOffset: i32;
    let mut commCosts1: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut commCosts2: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut inComps1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inComps2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut eqCompMapping1: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping2: metamodelica::Array<(i32, i32, i32)>;
    let mut exeCosts1: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut exeCosts2: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut nodeMark1: metamodelica::Array<i32>;
    let mut nodeMark2: metamodelica::Array<i32>;
    let mut compParamMapping1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compParamMapping2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping1: metamodelica::Array<(i32, i32, i32)>;
    let mut varCompMapping2: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames1: metamodelica::Array<ArcStr>;
    let mut compNames2: metamodelica::Array<ArcStr>;
    let mut compDescs1: metamodelica::Array<ArcStr>;
    let mut compDescs2: metamodelica::Array<ArcStr>;
    let mut compInformations1: metamodelica::Array<ComponentInfo>;
    let mut compInformations2: metamodelica::Array<ComponentInfo>;
    let mut graph2: TaskGraph;
    let TaskGraphMeta { inComps: __pa0, varCompMapping: __pa1, eqCompMapping: __pa2, compParamMapping: __pa3, compNames: __pa4, compDescs: __pa5, exeCosts: __pa6, commCosts: __pa7, nodeMark: __pa8, compInformations: __pa9 } = (graphData1In) else { bail!("pattern mismatch") };
    inComps1 = __pa0.clone();
    varCompMapping1 = __pa1.clone();
    eqCompMapping1 = __pa2.clone();
    compParamMapping1 = __pa3.clone();
    compNames1 = __pa4.clone();
    compDescs1 = __pa5.clone();
    exeCosts1 = __pa6.clone();
    commCosts1 = __pa7.clone();
    nodeMark1 = __pa8.clone();
    compInformations1 = __pa9.clone();
    let TaskGraphMeta { inComps: __pa10, varCompMapping: __pa11, eqCompMapping: __pa12, compParamMapping: __pa13, compNames: __pa14, compDescs: __pa15, exeCosts: __pa16, commCosts: __pa17, nodeMark: __pa18, compInformations: __pa19 } = (graphData2In) else { bail!("pattern mismatch") };
    inComps2 = __pa10.clone();
    varCompMapping2 = __pa11.clone();
    eqCompMapping2 = __pa12.clone();
    compParamMapping2 = __pa13.clone();
    compNames2 = __pa14.clone();
    compDescs2 = __pa15.clone();
    exeCosts2 = __pa16.clone();
    commCosts2 = __pa17.clone();
    nodeMark2 = __pa18.clone();
    compInformations2 = __pa19.clone();
    eqOffset = metamodelica::arrayLength(eqCompMapping1.clone());
    idxOffset = metamodelica::arrayLength(graph1In.clone());
    varOffset = metamodelica::arrayLength(varCompMapping1.clone());
    eqOffset = metamodelica::arrayLength(eqCompMapping1.clone());
    graph2 = Array::map1(graph2In.clone(), (std::sync::Arc::new(updateTaskGraphSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<Arc<metamodelica::List<i32>>> + 'static>), idxOffset)?;
    graphOut = metamodelica::arrayAppend(graph1In.clone(), graph2.clone());
    inComps2 = Array::map1(inComps2.clone(), (std::sync::Arc::new(updateTaskGraphSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<Arc<metamodelica::List<i32>>> + 'static>), idxOffset)?;
    inComps2 = metamodelica::arrayAppend(inComps1.clone(), inComps2.clone());
    varCompMapping2 = Array::map1(varCompMapping2.clone(), (std::sync::Arc::new(fnptr!(modifyMapping, (i32, i32, i32), i32)) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32, i32), i32) -> Result<(i32, i32, i32)> + 'static>), idxOffset)?;
    varCompMapping2 = metamodelica::arrayAppend(varCompMapping1.clone(), varCompMapping2.clone());
    eqCompMapping2 = Array::map1(eqCompMapping2.clone(), (std::sync::Arc::new(fnptr!(modifyMapping, (i32, i32, i32), i32)) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32, i32), i32) -> Result<(i32, i32, i32)> + 'static>), idxOffset)?;
    eqCompMapping2 = metamodelica::arrayAppend(eqCompMapping1.clone(), eqCompMapping2.clone());
    compParamMapping2 = metamodelica::arrayAppend(compParamMapping1.clone(), compParamMapping2.clone());
    compNames2 = Array::map1(compNames2.clone(), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!(" subsys")).clone())?;
    compNames2 = metamodelica::arrayAppend(compNames1.clone(), compNames2.clone());
    compDescs2 = metamodelica::arrayAppend(compDescs1.clone(), compDescs2.clone());
    exeCosts2 = metamodelica::arrayAppend(exeCosts1.clone(), exeCosts2.clone());
    commCosts2 = Array::map1(commCosts2.clone(), (std::sync::Arc::new(updateCommCosts) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Communication>>, i32) -> Result<Arc<metamodelica::List<Communication>>> + 'static>), idxOffset)?;
    commCosts2 = metamodelica::arrayAppend(commCosts1.clone(), commCosts2.clone());
    nodeMark2 = metamodelica::arrayAppend(nodeMark1.clone(), nodeMark2.clone());
    compInformations2 = metamodelica::arrayAppend(compInformations1.clone(), compInformations2.clone());
    graphDataOut = TaskGraphMeta { inComps: inComps2.clone(), varCompMapping: varCompMapping2.clone(), eqCompMapping: eqCompMapping2.clone(), compParamMapping: compParamMapping2.clone(), compNames: compNames2.clone(), compDescs: compDescs2.clone(), exeCosts: exeCosts2.clone(), commCosts: commCosts2.clone(), nodeMark: nodeMark2.clone(), compInformations: compInformations2.clone() };
    Ok((graphOut, graphDataOut))
}

fn modifyMapping(mut iMappingTuple: (i32, i32, i32), mut iOffset: i32) -> (i32, i32, i32) {
    let mut oMappingTuple: (i32, i32, i32);
    let mut i1: i32;
    let mut i2: i32;
    let mut i3: i32;
    (i1, i2, i3) = iMappingTuple;
    oMappingTuple = (i1 + iOffset, i2, iOffset);
    oMappingTuple
}

fn updateCommCosts(mut commCostsIn: Communications, mut idxOffset: i32) -> Result<Communications> {
    let mut commCostsOut: Communications;
    commCostsOut = List::map1(commCostsIn, (std::sync::Arc::new(updateCommCosts1) as std::sync::Arc<dyn ::std::ops::Fn(Communication, i32) -> Result<Communication> + 'static>), idxOffset)?;
    Ok(commCostsOut)
}

fn updateCommCosts1(mut commCostsIn: Communication, mut idxOffset: i32) -> Result<Communication> {
    let mut commCostsOut: Communication;
    let mut numberOfVars: i32;
    let mut childNode: i32;
    let mut integerVars: Arc<metamodelica::List<i32>>;
    let mut floatVars: Arc<metamodelica::List<i32>>;
    let mut booleanVars: Arc<metamodelica::List<i32>>;
    let mut stringVars: Arc<metamodelica::List<i32>>;
    let mut requiredTime: metamodelica::Real;
    let Communication { numberOfVars: __pa0, integerVars: __pa1, floatVars: __pa2, booleanVars: __pa3, stringVars: __pa4, childNode: __pa5, requiredTime: __pa6 } = (commCostsIn) else { bail!("pattern mismatch") };
    numberOfVars = __pa0.clone();
    integerVars = __pa1.clone();
    floatVars = __pa2.clone();
    booleanVars = __pa3.clone();
    stringVars = __pa4.clone();
    childNode = __pa5.clone();
    requiredTime = __pa6.clone();
    childNode = childNode + idxOffset;
    commCostsOut = Communication { numberOfVars: numberOfVars, integerVars: integerVars, floatVars: floatVars, booleanVars: booleanVars, stringVars: stringVars, childNode: childNode, requiredTime: requiredTime };
    Ok(commCostsOut)
}

fn updateTaskGraphSystem(mut graphRowIn: Arc<metamodelica::List<i32>>, mut idxOffset: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut graphRowOut: Arc<metamodelica::List<i32>>;
    graphRowOut = List::map1(graphRowIn, (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), idxOffset)?;
    Ok(graphRowOut)
}

fn createTaskGraph1(mut iComponent: Arc<BackendDAE::StrongComponent>, mut iSystInfo: (metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32), mut iVarInfo: (metamodelica::Array<(i32, i32, i32)>, metamodelica::Array<(i32, i32, i32)>, Arc<metamodelica::List<i32>>), mut iAnalyzeParameters: bool, mut graphInfoIn: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<Communication>>>, metamodelica::Array<ArcStr>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<Communication>>>, metamodelica::Array<ArcStr>, metamodelica::Array<i32>, i32)> {
    let mut graphInfoOut: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<Communication>>>, metamodelica::Array<ArcStr>, metamodelica::Array<i32>, i32);
    let mut adjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut isyst: Arc<BackendDAE::EqSystem>;
    let mut ishared: Arc<BackendDAE::Shared>;
    let mut orderedVars: BackendDAE::Variables;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut localKnownVars: BackendDAE::Variables;
    let mut knownVars: BackendDAE::Variables;
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut graphIn: TaskGraph;
    let mut graphTmp: TaskGraph;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut commCostsOfNode: Communications;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut unsolvedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    let mut eventVarLst: Arc<metamodelica::List<i32>>;
    let mut componentIndex: i32;
    let mut numberOfComps: i32;
    let mut requiredSccs_RefCount: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>;
    let mut compName: ArcStr;
    let mut paramVars: Arc<metamodelica::List<i32>>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut requiredSccs: Arc<UnorderedMap::UnorderedMap<i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>;
    (adjacencyMatrix, isyst, ishared, numberOfComps) = iSystInfo;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ishared) {
        Deref @ BackendDAE::Shared { globalKnownVars: __pa0, localKnownVars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    globalKnownVars = __pa0.clone();
    localKnownVars = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(isyst) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa2, orderedEqs: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    orderedVars = __pa2.clone();
    orderedEqs = __pa3.clone();
    (varCompMapping, eqCompMapping, eventVarLst) = iVarInfo;
    (graphIn, inComps, compParamMapping, commCosts, compNames, nodeMark, componentIndex) = graphInfoIn;
    inComps = metamodelica::arrayUpdate(inComps.clone(), componentIndex, list![componentIndex])?;
    compName = (BackendDump::strongComponentString(iComponent.clone())?).clone();
    compNames = metamodelica::arrayUpdate(compNames.clone(), componentIndex, (compName).clone())?;
    HpcOmBenchmark::benchSystem()?;
    if iAnalyzeParameters {
        knownVars = BackendVariable::addVariables(globalKnownVars, localKnownVars)?;
    } else {
        knownVars = globalKnownVars;
    }
    (unsolvedVars, paramVars) = getUnsolvedVarsBySCC(iComponent, adjacencyMatrix.clone(), orderedVars, knownVars, orderedEqs, eventVarLst, iAnalyzeParameters)?;
    compParamMapping = metamodelica::arrayUpdate(compParamMapping.clone(), componentIndex, paramVars)?;
    requiredSccs = UnorderedMap::new(std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 1);
    for mut intVar in &*Util::tuple41(unsolvedVars.clone()) {
        let mut intVar = intVar.clone();
        fillRequiredSccs((intVar.clone(), 1), VariableType::INTEGER.clone(), varCompMapping.clone(), requiredSccs.clone())?;
    }
    for mut floatVar in &*Util::tuple42(unsolvedVars.clone()) {
        let mut floatVar = floatVar.clone();
        fillRequiredSccs(floatVar.clone(), VariableType::REAL.clone(), varCompMapping.clone(), requiredSccs.clone())?;
    }
    for mut boolVar in &*Util::tuple43(unsolvedVars.clone()) {
        let mut boolVar = boolVar.clone();
        fillRequiredSccs((boolVar.clone(), 1), VariableType::BOOLEAN.clone(), varCompMapping.clone(), requiredSccs.clone())?;
    }
    for mut stringVar in &*Util::tuple44(unsolvedVars) {
        let mut stringVar = stringVar.clone();
        fillRequiredSccs((stringVar.clone(), 1), VariableType::STRING.clone(), varCompMapping.clone(), requiredSccs.clone())?;
    }
    requiredSccs_RefCount = createRequiredSccsRefCount(requiredSccs);
    (commCosts, commCostsOfNode) = updateCommCostBySccRef(requiredSccs_RefCount, componentIndex, commCosts.clone())?;
    graphTmp = fillAdjacencyList(graphIn.clone(), componentIndex, commCostsOfNode, 1);
    graphInfoOut = (graphTmp.clone(), inComps.clone(), compParamMapping.clone(), commCosts.clone(), compNames.clone(), nodeMark.clone(), componentIndex + 1);
    Ok(graphInfoOut)
}

fn createRequiredSccsRefCount(mut requiredSccs: Arc<UnorderedMap::UnorderedMap<i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>) -> Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> {
    let mut requiredSccsRefCount: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut scc_idx: i32;
    let mut int_vars: Arc<metamodelica::List<i32>>;
    let mut float_vars: Arc<metamodelica::List<i32>>;
    let mut bool_vars: Arc<metamodelica::List<i32>>;
    let mut string_vars: Arc<metamodelica::List<i32>>;
    for mut e in &*UnorderedMap::toList(requiredSccs) {
        let mut e = e.clone();
        let (__pa0, (__pa1, __pa2, __pa3, __pa4)) = e.clone();
        scc_idx = __pa0.clone();
        int_vars = __pa1.clone();
        float_vars = __pa2.clone();
        bool_vars = __pa3.clone();
        string_vars = __pa4.clone();
        requiredSccsRefCount = metamodelica::cons((scc_idx, int_vars.clone(), float_vars.clone(), bool_vars.clone(), string_vars.clone()), requiredSccsRefCount.clone());
    }
    requiredSccsRefCount
}

fn updateCommCostBySccRef(mut requiredSccs_RefCount: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>, mut nodeIdx: i32, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<(metamodelica::Array<Arc<metamodelica::List<Communication>>>, Communications)> {
    let mut oCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut oNodeComms: Communications;
    let mut tmpComms: Communications;
    tmpComms = List::map1(requiredSccs_RefCount, (std::sync::Arc::new(fnptr!(createCommunicationObject, (i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn((i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), metamodelica::Real) -> Result<Communication> + 'static>), metamodelica::OrderedFloat(-1.0_f64))?;
    oCommCosts = List::fold1(tmpComms.clone(), (std::sync::Arc::new(updateCommCostBySccRef1) as std::sync::Arc<dyn ::std::ops::Fn(Communication, i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> + 'static>), nodeIdx, iCommCosts.clone())?;
    oNodeComms = tmpComms;
    Ok((oCommCosts, oNodeComms))
}

fn createCommunicationObject(mut iTuple: (i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), mut requiredTime: metamodelica::Real) -> Communication {
    let mut oComm: Communication;
    let mut integerVars: Arc<metamodelica::List<i32>>;
    let mut floatVars: Arc<metamodelica::List<i32>>;
    let mut booleanVars: Arc<metamodelica::List<i32>>;
    let mut stringVars: Arc<metamodelica::List<i32>>;
    let mut sccIdx: i32;
    let mut refCountSum: i32;
    (sccIdx, integerVars, floatVars, booleanVars, stringVars) = iTuple;
    refCountSum = (integerVars.clone().len() as i32) + (floatVars.clone().len() as i32) + (booleanVars.clone().len() as i32) + (stringVars.clone().len() as i32);
    oComm = Communication { numberOfVars: refCountSum, integerVars: integerVars, floatVars: floatVars, booleanVars: booleanVars, stringVars: stringVars, childNode: sccIdx, requiredTime: requiredTime };
    oComm
}

fn updateCommCostBySccRef1(mut iEdgeSource: Communication, mut iEdgeTarget: i32, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> {
    let mut oCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut oldComms: Communications;
    let mut sourceSccIdx: i32;
    let mut integerVars: Arc<metamodelica::List<i32>>;
    let mut floatVars: Arc<metamodelica::List<i32>>;
    let mut booleanVars: Arc<metamodelica::List<i32>>;
    let mut stringVars: Arc<metamodelica::List<i32>>;
    let mut numberOfVars: i32;
    let mut requiredTime: metamodelica::Real;
    let mut tmpComm: Communication;
    let Communication { numberOfVars: __pa0, integerVars: __pa1, floatVars: __pa2, booleanVars: __pa3, stringVars: __pa4, childNode: __pa5, requiredTime: __pa6 } = (iEdgeSource) else { bail!("pattern mismatch") };
    numberOfVars = __pa0.clone();
    integerVars = __pa1.clone();
    floatVars = __pa2.clone();
    booleanVars = __pa3.clone();
    stringVars = __pa4.clone();
    sourceSccIdx = __pa5.clone();
    requiredTime = __pa6.clone();
    oldComms = metamodelica::arrayGet(iCommCosts.clone(), sourceSccIdx)?;
    tmpComm = Communication { numberOfVars: numberOfVars, integerVars: integerVars, floatVars: floatVars, booleanVars: booleanVars, stringVars: stringVars, childNode: iEdgeTarget, requiredTime: requiredTime };
    oCommCosts = metamodelica::arrayUpdate(iCommCosts.clone(), sourceSccIdx, metamodelica::cons(tmpComm, oldComms))?;
    Ok(oCommCosts)
}

fn fillAdjacencyList(mut adjLstIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut childNode: i32, mut parentLst: Communications, mut Idx: i32) -> metamodelica::Array<Arc<metamodelica::List<i32>>> {
    let mut adjLstOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    adjLstOut = 'mc: {
        let __mc_input = Idx;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut parentNode: Communication;
            let mut parentRow: Arc<metamodelica::List<i32>>;
            let mut adjLst: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut parentNodeIdx: i32;
            let true = ((parentLst.clone().len() as i32) >= Idx) else { bail!("pattern mismatch") };
            parentNode = (parentLst.clone()).get(Idx)?;
            let Communication { childNode: __pa0, .. } = (parentNode.clone()) else { bail!("pattern mismatch") };
            parentNodeIdx = __pa0.clone();
            parentRow = metamodelica::arrayGet(adjLstIn.clone(), parentNodeIdx.clone())?;
            parentRow = metamodelica::cons(childNode, parentRow.clone());
            parentRow = List::removeOnTrue(parentNodeIdx.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), parentRow.clone())?;
            adjLst = metamodelica::arrayUpdate(adjLstIn.clone(), parentNodeIdx.clone(), parentRow.clone())?;
            adjLst = fillAdjacencyList(adjLst.clone(), childNode, parentLst.clone(), Idx + 1);
            Ok(adjLst.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(adjLstIn.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    adjLstOut
}

fn getEquationStrings(mut iComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut iEqSystem: Arc<BackendDAE::EqSystem>) -> Result<metamodelica::Array<ArcStr>> {
    let mut eqDescsOut: metamodelica::Array<ArcStr>;
    let mut eqDescs: Arc<metamodelica::List<ArcStr>>;
    eqDescs = List::fold1(iComps, (std::sync::Arc::new(fnptr!(getEquationStrings2, Arc<BackendDAE::StrongComponent>, Arc<BackendDAE::EqSystem>, Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, Arc<BackendDAE::EqSystem>, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), iEqSystem, metamodelica::nil())?;
    eqDescs = eqDescs.reverse();
    eqDescsOut = metamodelica::arrayFromVec(eqDescs.into_iter().cloned().collect());
    Ok(eqDescsOut)
}

fn getEquationStrings2(mut comp: Arc<BackendDAE::StrongComponent>, mut iEqSystem: Arc<BackendDAE::EqSystem>, mut iEqDesc: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut oEqDesc: Arc<metamodelica::List<ArcStr>>;
    oEqDesc = 'mc: {
        let __mc_input = (comp, iEqSystem);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: i, var: v }, Deref @ BackendDAE::EqSystem { orderedEqs, orderedVars, .. }) => {
                    let mut descLst: Arc<metamodelica::List<ArcStr>>;
                    let mut eqString: ArcStr;
                    let mut varString: ArcStr;
                    let mut desc: ArcStr;
                    eqString = (BackendDump::equationString(BackendEquation::get(orderedEqs.clone(), i.clone())?)?).clone();
                    varString = (getVarString(BackendVariable::getVarAt(orderedVars.clone(), v.clone())?)?).clone();
                    desc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*eqString.clone()); __mm_s.push_str(&*literal!(" FOR ")); __mm_s.push_str(&*varString.clone()); ArcStr::from(__mm_s) }).clone();
                    descLst = metamodelica::cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { jac: Deref @ BackendDAE::Jacobian::FULL_JACOBIAN { jacobian: _ }, .. }, Deref @ BackendDAE::EqSystem { .. }) => {
                    let mut descLst: Arc<metamodelica::List<ArcStr>>;
                    let mut desc: ArcStr;
                    desc = (literal!("Equation System")).clone();
                    descLst = metamodelica::cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: i, vars: vs }, Deref @ BackendDAE::EqSystem { orderedEqs, orderedVars, matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }) => {
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut descLst: Arc<metamodelica::List<ArcStr>>;
                    let mut eqString: ArcStr;
                    let mut desc: ArcStr;
                    eqString = (BackendDump::equationString(BackendEquation::get(orderedEqs.clone(), i.clone())?)?).clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    desc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ARRAY:")); __mm_s.push_str(&*eqString.clone()); __mm_s.push_str(&*literal!(" FOR THE VARS: ")); __mm_s.push_str(&*stringDelimitList(List::map1(vs.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), List::map(varLst.clone(), (std::sync::Arc::new(getVarString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<ArcStr> + 'static>))?)?, (literal!(" AND ")).clone())); ArcStr::from(__mm_s) }).clone();
                    descLst = metamodelica::cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: i, vars: vs }, Deref @ BackendDAE::EqSystem { orderedEqs, orderedVars, matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }) => {
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut descLst: Arc<metamodelica::List<ArcStr>>;
                    let mut eqString: ArcStr;
                    let mut desc: ArcStr;
                    eqString = (BackendDump::equationString(BackendEquation::get(orderedEqs.clone(), i.clone())?)?).clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    desc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ALGO: ")); __mm_s.push_str(&*eqString.clone()); __mm_s.push_str(&*literal!(" FOR THE VARS: ")); __mm_s.push_str(&*stringDelimitList(List::map1(vs.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), List::map(varLst.clone(), (std::sync::Arc::new(getVarString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<ArcStr> + 'static>))?)?, (literal!(" AND ")).clone())); ArcStr::from(__mm_s) }).clone();
                    descLst = metamodelica::cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: i, vars: vs }, Deref @ BackendDAE::EqSystem { orderedEqs, orderedVars, matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }) => {
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut descLst: Arc<metamodelica::List<ArcStr>>;
                    let mut eqString: ArcStr;
                    let mut desc: ArcStr;
                    eqString = (BackendDump::equationString(BackendEquation::get(orderedEqs.clone(), i.clone())?)?).clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    desc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("COMPLEX: ")); __mm_s.push_str(&*eqString.clone()); __mm_s.push_str(&*literal!(" FOR THE VARS: ")); __mm_s.push_str(&*stringDelimitList(List::map1(vs.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), List::map(varLst.clone(), (std::sync::Arc::new(getVarString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<ArcStr> + 'static>))?)?, (literal!(" AND ")).clone())); ArcStr::from(__mm_s) }).clone();
                    descLst = metamodelica::cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: i, vars: vs }, Deref @ BackendDAE::EqSystem { orderedEqs, orderedVars, matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }) => {
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut descLst: Arc<metamodelica::List<ArcStr>>;
                    let mut eqString: ArcStr;
                    let mut desc: ArcStr;
                    eqString = (BackendDump::equationString(BackendEquation::get(orderedEqs.clone(), i.clone())?)?).clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    desc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("WHEN:")); __mm_s.push_str(&*eqString.clone()); __mm_s.push_str(&*literal!(" FOR THE VARS: ")); __mm_s.push_str(&*stringDelimitList(List::map1(vs.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), List::map(varLst.clone(), (std::sync::Arc::new(getVarString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<ArcStr> + 'static>))?)?, (literal!(" AND ")).clone())); ArcStr::from(__mm_s) }).clone();
                    descLst = metamodelica::cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: i, vars: vs }, Deref @ BackendDAE::EqSystem { orderedEqs, orderedVars, matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }) => {
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut descLst: Arc<metamodelica::List<ArcStr>>;
                    let mut eqString: ArcStr;
                    let mut desc: ArcStr;
                    eqString = (BackendDump::equationString(BackendEquation::get(orderedEqs.clone(), i.clone())?)?).clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    desc = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("IFEQ:")); __mm_s.push_str(&*eqString.clone()); __mm_s.push_str(&*literal!(" FOR THE VARS: ")); __mm_s.push_str(&*stringDelimitList(List::map1(vs.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), List::map(varLst.clone(), (std::sync::Arc::new(getVarString) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<ArcStr> + 'static>))?)?, (literal!(" AND ")).clone())); ArcStr::from(__mm_s) }).clone();
                    descLst = metamodelica::cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: true, .. }, Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }) => {
                    let mut descLst: Arc<metamodelica::List<ArcStr>>;
                    let mut desc: ArcStr;
                    desc = (literal!("Torn linear System")).clone();
                    descLst = metamodelica::cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { linear: false, .. }, Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { .. }, .. }) => {
                    let mut descLst: Arc<metamodelica::List<ArcStr>>;
                    let mut desc: ArcStr;
                    desc = (literal!("Torn nonlinear System")).clone();
                    descLst = metamodelica::cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut descLst: Arc<metamodelica::List<ArcStr>>;
                    let mut desc: ArcStr;
                    desc = (literal!("no singleEquation")).clone();
                    descLst = metamodelica::cons((desc.clone()).clone(), iEqDesc.clone());
                    Ok(descLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oEqDesc
}

pub(crate) fn getVarString(mut inVar: BackendDAE::Var) -> Result<ArcStr> {
    let mut varString: ArcStr = arcstr::literal!("");
    varString = ('mc: {
        let __mc_input = inVar.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut varDescLst: Arc<metamodelica::List<ArcStr>>;
            let mut varString: ArcStr = varString.clone();
            let true = (BackendVariable::isNonStateVar(inVar.clone())) else { bail!("pattern mismatch") };
            varString = (BackendDump::varString(inVar.clone())?).clone();
            varDescLst = stringListStringChar((varString.clone()).clone());
            varDescLst = shortenVarString(varDescLst.clone())?;
            varString = (stringCharListString(varDescLst.clone())).clone();
            Ok((varString.clone(), varString.clone()))
        })() { varString = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut varDescLst: Arc<metamodelica::List<ArcStr>>;
            let mut varString: ArcStr = varString.clone();
            let false = (BackendVariable::isNonStateVar(inVar.clone())) else { bail!("pattern mismatch") };
            varString = (BackendDump::varString(inVar.clone())?).clone();
            varDescLst = stringListStringChar((varString.clone()).clone());
            varDescLst = shortenVarString(varDescLst.clone())?;
            varString = (stringCharListString(varDescLst.clone())).clone();
            varString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" der(")); __mm_s.push_str(&*varString.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
            Ok((varString.clone(), varString.clone()))
        })() { varString = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(varString)
}

fn shortenVarString(mut iString: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut oString: Arc<metamodelica::List<ArcStr>>;
    let mut pos: i32;
    pos = List::position((literal!(":")).clone(), iString.clone())? - 1;
    (oString, _) = List::split(iString, pos)?;
    Ok(oString)
}

fn getEventNodes(mut systIn: Arc<BackendDAE::BackendDAE>, mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut eventNodes: Arc<metamodelica::List<i32>>;
    let mut eqLst: Arc<metamodelica::List<i32>>;
    let mut systemsIn: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let __pa0 = ::match_deref::match_deref! { match &(systIn) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    systemsIn = __pa0.clone();
    (eqLst, _) = List::fold(systemsIn, (std::sync::Arc::new(getEventNodeEqs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32)> + 'static>), (metamodelica::nil(), 0))?;
    eventNodes = getArrayTuple31(eqLst, eqCompMapping.clone())?;
    Ok(eventNodes)
}

fn getEventNodeEqs(mut systIn: Arc<BackendDAE::EqSystem>, mut eventInfoIn: (Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32)> {
    let mut eventInfoOut: (Arc<metamodelica::List<i32>>, i32);
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut matching: Arc<BackendDAE::Matching>;
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eventEqs: Arc<metamodelica::List<i32>>;
    let mut eventEqsIn: Arc<metamodelica::List<i32>>;
    let mut offset: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(systIn) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, matching: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    orderedEqs = __pa0.clone();
    matching = __pa1.clone();
    comps = BackendDAEUtil::getCompsOfMatching(matching);
    (eventEqsIn, offset) = eventInfoIn;
    eventEqs = getEventNodeEqs1(comps, offset, metamodelica::nil())?;
    offset = offset + ExpandableArray::getNumberOfElements(orderedEqs);
    eventInfoOut = (listAppend(eventEqs, eventEqsIn), offset);
    Ok(eventInfoOut)
}

fn getEventNodeEqs1(mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut offset: i32, mut eventEqsIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut eventEqsOut: Arc<metamodelica::List<i32>>;
    eventEqsOut = 'mc: {
        let __mc_input = comps;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
                    let mut eqn: i32;
                    let mut eventEqs: Arc<metamodelica::List<i32>>;
                    let true = (isWhenEquation(head.clone())) else { bail!("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(head.clone()) {
                        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    eqn = __pa0.clone();
                    eqn = eqn.clone() + offset;
                    eventEqs = getEventNodeEqs1(rest.clone(), offset, metamodelica::cons(eqn.clone(), eventEqsIn.clone()))?;
                    Ok(eventEqs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
                    let mut eventEqs: Arc<metamodelica::List<i32>>;
                    let false = (isWhenEquation(head.clone())) else { bail!("pattern mismatch") };
                    eventEqs = getEventNodeEqs1(rest.clone(), offset, eventEqsIn.clone())?;
                    Ok(eventEqs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(eventEqsIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(eventEqsOut)
}

fn getArrayTuple31(mut list1: Arc<metamodelica::List<i32>>, mut assign: metamodelica::Array<(i32, i32, i32)>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut list2Out: Arc<metamodelica::List<i32>>;
    let mut tplLst: Arc<metamodelica::List<(i32, i32, i32)>>;
    tplLst = List::map1(list1, (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), assign.clone())?;
    list2Out = List::map(tplLst, std::sync::Arc::new(fnptr!(Util::tuple31, _)))?;
    Ok(list2Out)
}

fn isWhenEquation(mut inComp: Arc<BackendDAE::StrongComponent>) -> bool {
    let mut isWhenEq: bool;
    isWhenEq = (::match_deref::match_deref! { match &(inComp) {
        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isWhenEq
}

fn fillRequiredSccs(mut var: (i32, i32), mut varType: VariableType, mut varMapping: metamodelica::Array<(i32, i32, i32)>, mut requiredSccs: Arc<UnorderedMap::UnorderedMap<i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)>>) -> Result<()> {
    let mut var_idx: i32;
    let mut scc_idx: i32;
    let mut not_derived: i32;
    let mut integerVars: Arc<metamodelica::List<i32>>;
    let mut floatVars: Arc<metamodelica::List<i32>>;
    let mut booleanVars: Arc<metamodelica::List<i32>>;
    let mut stringVars: Arc<metamodelica::List<i32>>;
    (var_idx, not_derived) = var;
    if not_derived == 1 {
        (scc_idx, _, _) = ({let __elt = varMapping.borrow()[(var_idx-1) as usize].clone(); __elt});
        (integerVars, floatVars, booleanVars, stringVars) = UnorderedMap::getOrDefault(scc_idx, requiredSccs.clone(), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()))?;
        let () = (match varType {
        VariableType::INTEGER { .. } => {
            integerVars = metamodelica::cons(var_idx, integerVars);
            ()
        },
        VariableType::REAL { .. } => {
            floatVars = metamodelica::cons(var_idx, floatVars);
            ()
        },
        VariableType::BOOLEAN => {
            booleanVars = metamodelica::cons(var_idx, booleanVars);
            ()
        },
        VariableType::STRING { .. } => {
            stringVars = metamodelica::cons(var_idx, stringVars);
            ()
        },
    });
        UnorderedMap::add(scc_idx, (integerVars, floatVars, booleanVars, stringVars), requiredSccs)?;
    }
    Ok(())
}

fn getUnsolvedVarsBySCC(mut iComponent: Arc<BackendDAE::StrongComponent>, mut iAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iOrderedVars: BackendDAE::Variables, mut iKnownVars: BackendDAE::Variables, mut iOrderedEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut iEventVarLst: Arc<metamodelica::List<i32>>, mut iAnalyzeParameters: bool) -> Result<((Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), Arc<metamodelica::List<i32>>)> {
    let mut oUnsolvedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    let mut oParamVars: Arc<metamodelica::List<i32>>;
    (oUnsolvedVars, oParamVars) = 'mc: {
        let __mc_input = iComponent.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: varIdx, .. } => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), list![varIdx.clone()], iEventVarLst.clone(), iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: varIdc, .. } => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst.clone(), iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: varIdc, .. } => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst.clone(), iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: varIdc, .. } => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst.clone(), iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: varIdc, .. } => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst.clone(), iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: varIdc, .. } => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst.clone(), iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: varIdc, .. } => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst.clone(), iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: varIdc, .. }, .. } => {
                    let mut tmpVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
                    let mut paramVars: Arc<metamodelica::List<i32>>;
                    (tmpVars, paramVars) = getUnsolvedVarsBySCC0(iComponent.clone(), iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars.clone(), iOrderedEquations.clone(), varIdc.clone(), iEventVarLst.clone(), iAnalyzeParameters)?;
                    Ok((tmpVars.clone(), paramVars.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("getUnsolvedVarsBySCC failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oUnsolvedVars, oParamVars))
}

fn getUnsolvedVarsBySCC0(mut iComponent: Arc<BackendDAE::StrongComponent>, mut iAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iOrderedVars: BackendDAE::Variables, mut iKnownVars: BackendDAE::Variables, mut iOrderedEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut iVarIdc: Arc<metamodelica::List<i32>>, mut iEventVarLst: Arc<metamodelica::List<i32>>, mut iAnalyzeParameters: bool) -> Result<((Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), Arc<metamodelica::List<i32>>)> {
    let mut oUnsolvedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    let mut oParamVars: Arc<metamodelica::List<i32>>;
    let mut tmpVars: Arc<metamodelica::List<(i32, i32)>>;
    (tmpVars, oParamVars) = getVarsBySCC(iComponent, iAdjacencyMatrix.clone(), iOrderedVars.clone(), iKnownVars, iOrderedEquations, iAnalyzeParameters)?;
    tmpVars = List::filter1OnTrue(tmpVars, (std::sync::Arc::new(fnptr!(isTupleMember, (i32, i32), Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>), iVarIdc)?;
    tmpVars = removeEventVars(iEventVarLst, tmpVars, 1);
    oUnsolvedVars = List::fold1(tmpVars, (std::sync::Arc::new(getUnsolvedVarsBySCC1) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), BackendDAE::Variables, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> + 'static>), iOrderedVars, (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()))?;
    Ok((oUnsolvedVars, oParamVars))
}

fn getUnsolvedVarsBySCC1(mut iVarIdx: (i32, i32), mut orderedVars: BackendDAE::Variables, mut iUnsolvedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut oUnsolvedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    let mut var: BackendDAE::Var;
    let mut varType: Arc<DAE::Type>;
    var = BackendVariable::getVarAt(orderedVars, Util::tuple21(iVarIdx))?;
    varType = BackendVariable::getVarType(var);
    oUnsolvedVars = getUnsolvedVarsBySCC2(varType, iVarIdx, iUnsolvedVars);
    Ok(oUnsolvedVars)
}

fn getUnsolvedVarsBySCC2(mut iVarType: Arc<DAE::Type>, mut iVarIdx: (i32, i32), mut iUnsolvedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    '__tco: loop {
        let mut intVarIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut boolVarIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut stringVarIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut realVarIdc: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
        let mut varIdx: i32 = 0;
        let mut derived: i32 = 0;
        let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
        ::match_deref::match_deref! { match &((iVarType, iVarIdx, iUnsolvedVars.clone())) {
        (Deref @ DAE::Type::T_INTEGER { .. }, (__esc_varIdx, __esc_derived), (__esc_intVarIdc, __esc_realVarIdc, __esc_boolVarIdc, __esc_stringVarIdc)) => {
            varIdx = (*__esc_varIdx).clone();
            derived = (*__esc_derived).clone();
            intVarIdc = (*__esc_intVarIdc).clone();
            realVarIdc = (*__esc_realVarIdc).clone();
            boolVarIdc = (*__esc_boolVarIdc).clone();
            stringVarIdc = (*__esc_stringVarIdc).clone();
            intVarIdc = metamodelica::cons(varIdx.clone(), intVarIdc.clone());
            return (intVarIdc.clone(), realVarIdc.clone(), boolVarIdc.clone(), stringVarIdc.clone())
        },
        (Deref @ DAE::Type::T_REAL { .. }, (__esc_varIdx, __esc_derived), (__esc_intVarIdc, __esc_realVarIdc, __esc_boolVarIdc, __esc_stringVarIdc)) => {
            varIdx = (*__esc_varIdx).clone();
            derived = (*__esc_derived).clone();
            intVarIdc = (*__esc_intVarIdc).clone();
            realVarIdc = (*__esc_realVarIdc).clone();
            boolVarIdc = (*__esc_boolVarIdc).clone();
            stringVarIdc = (*__esc_stringVarIdc).clone();
            realVarIdc = metamodelica::cons((varIdx.clone(), derived.clone()), realVarIdc.clone());
            return (intVarIdc.clone(), realVarIdc.clone(), boolVarIdc.clone(), stringVarIdc.clone())
        },
        (Deref @ DAE::Type::T_BOOL { .. }, (__esc_varIdx, __esc_derived), (__esc_intVarIdc, __esc_realVarIdc, __esc_boolVarIdc, __esc_stringVarIdc)) => {
            varIdx = (*__esc_varIdx).clone();
            derived = (*__esc_derived).clone();
            intVarIdc = (*__esc_intVarIdc).clone();
            realVarIdc = (*__esc_realVarIdc).clone();
            boolVarIdc = (*__esc_boolVarIdc).clone();
            stringVarIdc = (*__esc_stringVarIdc).clone();
            boolVarIdc = metamodelica::cons(varIdx.clone(), boolVarIdc.clone());
            return (intVarIdc.clone(), realVarIdc.clone(), boolVarIdc.clone(), stringVarIdc.clone())
        },
        (Deref @ DAE::Type::T_ARRAY { ty: __esc_ty, .. }, (__esc_varIdx, __esc_derived), (__esc_intVarIdc, __esc_realVarIdc, __esc_boolVarIdc, __esc_stringVarIdc)) => {
            ty = (*__esc_ty).clone();
            varIdx = (*__esc_varIdx).clone();
            derived = (*__esc_derived).clone();
            intVarIdc = (*__esc_intVarIdc).clone();
            realVarIdc = (*__esc_realVarIdc).clone();
            boolVarIdc = (*__esc_boolVarIdc).clone();
            stringVarIdc = (*__esc_stringVarIdc).clone();
            { (iVarType, iVarIdx, iUnsolvedVars) = (ty.clone(), iVarIdx, iUnsolvedVars); continue '__tco; }
        },
        (Deref @ DAE::Type::T_ENUMERATION { .. }, (__esc_varIdx, __esc_derived), (__esc_intVarIdc, __esc_realVarIdc, __esc_boolVarIdc, __esc_stringVarIdc)) => {
            varIdx = (*__esc_varIdx).clone();
            derived = (*__esc_derived).clone();
            intVarIdc = (*__esc_intVarIdc).clone();
            realVarIdc = (*__esc_realVarIdc).clone();
            boolVarIdc = (*__esc_boolVarIdc).clone();
            stringVarIdc = (*__esc_stringVarIdc).clone();
            stringVarIdc = metamodelica::cons(varIdx.clone(), stringVarIdc.clone());
            return (intVarIdc.clone(), realVarIdc.clone(), boolVarIdc.clone(), stringVarIdc.clone())
        },
        (Deref @ DAE::Type::T_STRING { .. }, (__esc_varIdx, __esc_derived), (__esc_intVarIdc, __esc_realVarIdc, __esc_boolVarIdc, __esc_stringVarIdc)) => {
            varIdx = (*__esc_varIdx).clone();
            derived = (*__esc_derived).clone();
            intVarIdc = (*__esc_intVarIdc).clone();
            realVarIdc = (*__esc_realVarIdc).clone();
            boolVarIdc = (*__esc_boolVarIdc).clone();
            stringVarIdc = (*__esc_stringVarIdc).clone();
            stringVarIdc = metamodelica::cons(varIdx.clone(), stringVarIdc.clone());
            return (intVarIdc.clone(), realVarIdc.clone(), boolVarIdc.clone(), stringVarIdc.clone())
        },
        _ => {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getUnsolvedVarsBySCC2: Warning, unknown varType for variable ")); __mm_s.push_str(&*intString(Util::tuple21(iVarIdx))); __mm_s.push_str(&*literal!(" !\n")); ArcStr::from(__mm_s) }).clone());
            return iUnsolvedVars
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn removeEventVars(mut eventVarLst: Arc<metamodelica::List<i32>>, mut varLstIn: Arc<metamodelica::List<(i32, i32)>>, mut varIdx: i32) -> Arc<metamodelica::List<(i32, i32)>> {
    let mut varLstOut: Arc<metamodelica::List<(i32, i32)>>;
    varLstOut = 'mc: {
        let __mc_input = varIdx;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut varTpl: (i32, i32);
            let mut varLst: Arc<metamodelica::List<(i32, i32)>>;
            let mut var: i32;
            let true = (intLe(varIdx, (varLstIn.clone().len() as i32))) else { bail!("pattern mismatch") };
            varTpl = (varLstIn.clone()).get(varIdx)?;
            (var, _) = varTpl.clone();
            let true = (List::isMemberOnTrue(var.clone(), eventVarLst.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
            varLst = listDelete(varLstIn.clone(), varIdx)?;
            varLst = removeEventVars(eventVarLst.clone(), varLst.clone(), varIdx);
            Ok(varLst.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut varTpl: (i32, i32);
            let mut varLst: Arc<metamodelica::List<(i32, i32)>>;
            let mut var: i32;
            let true = (intLe(varIdx, (varLstIn.clone().len() as i32))) else { bail!("pattern mismatch") };
            varTpl = (varLstIn.clone()).get(varIdx)?;
            (var, _) = varTpl.clone();
            let false = (List::isMemberOnTrue(var.clone(), eventVarLst.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
            varLst = removeEventVars(eventVarLst.clone(), varLstIn.clone(), varIdx + 1);
            Ok(varLst.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(varLstIn.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    varLstOut
}

fn isTupleMember(mut inTuple: (i32, i32), mut varIdc: Arc<metamodelica::List<i32>>) -> bool {
    let mut isNotMember: bool;
    let mut varIdx: i32 = 0;
    let mut varState: i32 = 0;
    let mut returnValue: bool = false;
    isNotMember = 'mc: {
        let __mc_input = inTuple;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (mut varIdx, mut varState) = __mc_input.clone() else { bail!("nomatch") };
            let mut returnValue: bool = returnValue.clone();
            let true = (intGt(varIdx, 0)) else { bail!("pattern mismatch") };
            let true = (intEq(varState, 1)) else { bail!("pattern mismatch") };
            returnValue = List::isMemberOnTrue(varIdx, varIdc.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            Ok((!(returnValue), returnValue.clone()))
        })() { returnValue = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(true)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    isNotMember
}

fn compareTupleByVarIdx(mut varIdx: i32, mut var2Idx: (i32, i32)) -> bool {
    let mut equal: bool;
    equal = intEq(Util::tuple21(var2Idx), varIdx);
    equal
}

pub(crate) fn compareTasksByExecTime(mut iTask1: i32, mut iTask2: i32, mut iTaskComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iExeCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut iDescending: bool) -> Result<bool> {
    let mut oResult: bool;
    let mut exeCosts1: metamodelica::Real;
    let mut exeCosts2: metamodelica::Real;
    let mut taskComps1: Arc<metamodelica::List<i32>>;
    let mut taskComps2: Arc<metamodelica::List<i32>>;
    taskComps1 = metamodelica::arrayGet(iTaskComps.clone(), iTask1)?;
    taskComps2 = metamodelica::arrayGet(iTaskComps.clone(), iTask2)?;
    exeCosts1 = addUpExeCostsForNode(taskComps1, iExeCosts.clone(), metamodelica::OrderedFloat(0.0_f64))?;
    exeCosts2 = addUpExeCostsForNode(taskComps2, iExeCosts.clone(), metamodelica::OrderedFloat(0.0_f64))?;
    if iDescending {
        oResult = realLt(exeCosts1, exeCosts2);
    } else {
        oResult = realGt(exeCosts1, exeCosts2);
    }
    Ok(oResult)
}

fn getVarsBySCC(mut iComponent: Arc<BackendDAE::StrongComponent>, mut iAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iOrderedVars: BackendDAE::Variables, mut iKnownVars: BackendDAE::Variables, mut iOrderedEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut iAnalyzeParameters: bool) -> Result<(Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>)> {
    let mut oVars: Arc<metamodelica::List<(i32, i32)>>;
    let mut oParamVars: Arc<metamodelica::List<i32>>;
    (oVars, oParamVars) = (::match_deref::match_deref! { match &(iComponent) {
        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eqnIdx, .. } => {
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>>;
            let mut paramVars: Arc<metamodelica::List<i32>>;
            (eqnVars, paramVars) = getVarsByEqns(list![eqnIdx.clone()], iAdjacencyMatrix.clone(), iOrderedVars, iKnownVars, iOrderedEquations, iAnalyzeParameters)?;
            (eqnVars, paramVars)
        },
        Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns, .. } => {
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>>;
            let mut paramVars: Arc<metamodelica::List<i32>>;
            (eqnVars, paramVars) = getVarsByEqns(eqns.clone(), iAdjacencyMatrix.clone(), iOrderedVars, iKnownVars, iOrderedEquations, iAnalyzeParameters)?;
            (eqnVars, paramVars)
        },
        Deref @ BackendDAE::StrongComponent::SINGLEARRAY { eqn: eqnIdx, .. } => {
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>>;
            let mut paramVars: Arc<metamodelica::List<i32>>;
            (eqnVars, paramVars) = getVarsByEqns(list![eqnIdx.clone()], iAdjacencyMatrix.clone(), iOrderedVars, iKnownVars, iOrderedEquations, iAnalyzeParameters)?;
            (eqnVars, paramVars)
        },
        Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { eqn: eqnIdx, .. } => {
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>>;
            let mut paramVars: Arc<metamodelica::List<i32>>;
            (eqnVars, paramVars) = getVarsByEqns(list![eqnIdx.clone()], iAdjacencyMatrix.clone(), iOrderedVars, iKnownVars, iOrderedEquations, iAnalyzeParameters)?;
            (eqnVars, paramVars)
        },
        Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { eqn: eqnIdx, .. } => {
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>>;
            let mut paramVars: Arc<metamodelica::List<i32>>;
            (eqnVars, paramVars) = getVarsByEqns(list![eqnIdx.clone()], iAdjacencyMatrix.clone(), iOrderedVars, iKnownVars, iOrderedEquations, iAnalyzeParameters)?;
            (eqnVars, paramVars)
        },
        Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { eqn: eqnIdx, .. } => {
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>>;
            let mut paramVars: Arc<metamodelica::List<i32>>;
            (eqnVars, paramVars) = getVarsByEqns(list![eqnIdx.clone()], iAdjacencyMatrix.clone(), iOrderedVars, iKnownVars, iOrderedEquations, iAnalyzeParameters)?;
            (eqnVars, paramVars)
        },
        Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { eqn: eqnIdx, .. } => {
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>>;
            let mut paramVars: Arc<metamodelica::List<i32>>;
            (eqnVars, paramVars) = getVarsByEqns(list![eqnIdx.clone()], iAdjacencyMatrix.clone(), iOrderedVars, iKnownVars, iOrderedEquations, iAnalyzeParameters)?;
            (eqnVars, paramVars)
        },
        Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: resEqns, innerEquations, .. }, .. } => {
            let mut eqns: Arc<metamodelica::List<i32>>;
            let mut eqnVars: Arc<metamodelica::List<(i32, i32)>>;
            let mut paramVars: Arc<metamodelica::List<i32>>;
            (eqns, _, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>))?;
            (eqnVars, paramVars) = getVarsByEqns(listAppend(resEqns.clone(), eqns.clone()), iAdjacencyMatrix.clone(), iOrderedVars, iKnownVars, iOrderedEquations, iAnalyzeParameters)?;
            (eqnVars, paramVars)
        },
        _ => {
            metamodelica::print((literal!("Error in getVarsBySCC! Unsupported component-type \n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oVars, oParamVars))
}

fn tupleToString(mut inTuple: (i32, i32)) -> ArcStr {
    let mut result: ArcStr;
    result = ((match inTuple {
        (mut int1, mut int2) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(int1.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(int2.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
    })).clone();
    result
}

fn tuple3ToString(mut inTuple: (i32, i32, i32)) -> ArcStr {
    let mut result: ArcStr;
    result = ((match inTuple {
        (mut int1, mut int2, mut int3) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(int1.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(int2.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(int3.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
    })).clone();
    result
}

fn getVarsByEqns(mut iEqnIdc: Arc<metamodelica::List<i32>>, mut iAdjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iOrderedVars: BackendDAE::Variables, mut iKnownVars: BackendDAE::Variables, mut iOrderedEquations: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut iAnalyzeParameters: bool) -> Result<(Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>)> {
    let mut oAdjacencyVars: Arc<metamodelica::List<(i32, i32)>>;
    let mut oParamVars: Arc<metamodelica::List<i32>>;
    let mut adjacencyVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut paramVars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    for mut eqIdx in &*iEqnIdc {
        let mut eqIdx = eqIdx.clone();
        adjacencyVars = listAppend(metamodelica::arrayGet(iAdjacencyMatrix.clone(), eqIdx.clone())?, adjacencyVars.clone());
        eqs = metamodelica::cons(BackendEquation::get(iOrderedEquations.clone(), eqIdx.clone())?, eqs.clone());
    }
    oAdjacencyVars = List::map(adjacencyVars, (std::sync::Arc::new(fnptr!(getVarTuple, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<(i32, i32)> + 'static>))?;
    if iAnalyzeParameters {
        (paramVars, oParamVars) = BackendEquation::equationsParams(eqs, iKnownVars)?;
    } else {
        oParamVars = metamodelica::nil();
    }
    Ok((oAdjacencyVars, oParamVars))
}

fn getVarTuple(mut varIdx: i32) -> (i32, i32) {
    let mut outIdx: (i32, i32);
    outIdx = if (intLe(0, varIdx)) {(varIdx, 1)} else {(-(varIdx), 0)};
    outIdx
}

fn compareIntTuple2(mut tuple1: (i32, i32), mut tuple2: (i32, i32)) -> bool {
    let mut equals: bool;
    equals = (match (tuple1, tuple2) {
        ((mut int1, mut int2), (mut int3, mut int4)) if (int1.clone() == int3.clone() && int2.clone() == int4.clone()) => {
            true
        },
        _ => {
            false
        },
    });
    equals
}

fn getVarEqCompMapping(mut components: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut iEqSysIdx: i32, mut iVarIdxOffset: i32, mut iEqIdxOffset: i32, mut ivarCompMapping: metamodelica::Array<(i32, i32, i32)>, mut ieqCompMapping: metamodelica::Array<(i32, i32, i32)>) -> Result<(metamodelica::Array<(i32, i32, i32)>, metamodelica::Array<(i32, i32, i32)>)> {
    let mut ovarCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut oeqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    List::fold4(components, (std::sync::Arc::new(getVarEqCompMapping0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::StrongComponent>, metamodelica::Array<(i32, i32, i32)>, metamodelica::Array<(i32, i32, i32)>, i32, (i32, i32), i32) -> Result<i32> + 'static>), ivarCompMapping.clone(), ieqCompMapping.clone(), iEqSysIdx, (iVarIdxOffset, iEqIdxOffset), 1)?;
    ovarCompMapping = ivarCompMapping.clone();
    oeqCompMapping = ieqCompMapping.clone();
    Ok((ovarCompMapping, oeqCompMapping))
}

fn getVarEqCompMapping0(mut component: Arc<BackendDAE::StrongComponent>, mut varCompMapping: metamodelica::Array<(i32, i32, i32)>, mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>, mut iEqSysIdx: i32, mut iVarEqOffset: (i32, i32), mut iSccIdx: i32) -> Result<i32> {
    let mut oSccIdx: i32;
    oSccIdx = 'mc: {
        let __mc_input = (component.clone(), iVarEqOffset);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: compVarIdx, eqn: eq }, (iVarOffset, iEqOffset)) => {
                    metamodelica::arrayUpdate(varCompMapping.clone(), compVarIdx.clone() + iVarOffset.clone(), (iSccIdx, iEqSysIdx, iVarOffset.clone()))?;
                    metamodelica::arrayUpdate(eqCompMapping.clone(), eq.clone() + iEqOffset.clone(), (iSccIdx, iEqSysIdx, iEqOffset.clone()))?;
                    Ok(iSccIdx + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: compVarIdc, eqns, .. }, (iVarOffset, iEqOffset)) => {
                    List::fold3(compVarIdc.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx, iEqSysIdx, iVarOffset.clone(), varCompMapping.clone())?;
                    List::fold3(eqns.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx, iEqSysIdx, iEqOffset.clone(), eqCompMapping.clone())?;
                    Ok(iSccIdx + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars: compVarIdc, eqn: eq }, (iVarOffset, iEqOffset)) => {
                    List::fold3(compVarIdc.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx, iEqSysIdx, iVarOffset.clone(), varCompMapping.clone())?;
                    metamodelica::arrayUpdate(eqCompMapping.clone(), eq.clone() + iEqOffset.clone(), (iSccIdx, iEqSysIdx, iEqOffset.clone()))?;
                    Ok(iSccIdx + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars: compVarIdc, eqn: eq }, (iVarOffset, iEqOffset)) => {
                    List::fold3(compVarIdc.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx, iEqSysIdx, iVarOffset.clone(), varCompMapping.clone())?;
                    metamodelica::arrayUpdate(eqCompMapping.clone(), eq.clone() + iEqOffset.clone(), (iSccIdx, iEqSysIdx, iEqOffset.clone()))?;
                    Ok(iSccIdx + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars: compVarIdc, eqn: eq }, (iVarOffset, iEqOffset)) => {
                    List::fold3(compVarIdc.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx, iEqSysIdx, iVarOffset.clone(), varCompMapping.clone())?;
                    metamodelica::arrayUpdate(eqCompMapping.clone(), eq.clone() + iEqOffset.clone(), (iSccIdx, iEqSysIdx, iEqOffset.clone()))?;
                    Ok(iSccIdx + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars: compVarIdc, eqn: eq }, (iVarOffset, iEqOffset)) => {
                    List::fold3(compVarIdc.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx, iEqSysIdx, iVarOffset.clone(), varCompMapping.clone())?;
                    metamodelica::arrayUpdate(eqCompMapping.clone(), eq.clone() + iEqOffset.clone(), (iSccIdx, iEqSysIdx, iEqOffset.clone()))?;
                    Ok(iSccIdx + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { tearingvars: compVarIdc, residualequations: residuals, innerEquations, .. }, .. }, (iVarOffset, iEqOffset)) => {
                    let mut eqns: Arc<metamodelica::List<i32>>;
                    let mut othereqs: Arc<metamodelica::List<i32>>;
                    let mut othervars: Arc<metamodelica::List<i32>>;
                    let mut othervarsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut compVarIdc = (*compVarIdc).clone();
                    (othereqs, othervarsLst, _) = List::map_3(innerEquations.clone(), (std::sync::Arc::new(BackendDAEUtil::getEqnAndVarsFromInnerEquation) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::InnerEquation) -> Result<(i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<DAE::Constraint>>>)> + 'static>))?;
                    othervars = List::flatten(othervarsLst.clone())?;
                    compVarIdc = listAppend(othervars.clone(), compVarIdc.clone());
                    eqns = listAppend(othereqs.clone(), residuals.clone());
                    List::fold3(compVarIdc.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx, iEqSysIdx, iVarOffset.clone(), varCompMapping.clone())?;
                    List::fold3(eqns.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx, iEqSysIdx, iEqOffset.clone(), eqCompMapping.clone())?;
                    Ok(iSccIdx + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars: compVarIdc, eqn: eq }, (iVarOffset, iEqOffset)) => {
                    List::fold3(compVarIdc.clone(), (std::sync::Arc::new(updateMappingTuple) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> + 'static>), iSccIdx, iEqSysIdx, iVarOffset.clone(), varCompMapping.clone())?;
                    metamodelica::arrayUpdate(eqCompMapping.clone(), eq.clone() + iEqOffset.clone(), (iSccIdx, iEqSysIdx, iEqOffset.clone()))?;
                    Ok(iSccIdx + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut helperStr: ArcStr;
                    helperStr = (BackendDump::strongComponentString(component.clone())?).clone();
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getVarEqCompMapping0 - Unsupported component-type:\n")); __mm_s.push_str(&*helperStr.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oSccIdx)
}

pub(crate) fn getSccNodeMapping(mut iNumberOfSccs: i32, mut iTaskGraphMeta: TaskGraphMeta) -> Result<metamodelica::Array<i32>> {
    let mut oMapping: metamodelica::Array<i32>;
    let mut tmpMappingArray: metamodelica::Array<i32>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    tmpMappingArray = arrayCreate(iNumberOfSccs, -1);
    let TaskGraphMeta { inComps: __pa0, nodeMark: __pa1, .. } = (iTaskGraphMeta) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    nodeMark = __pa1.clone();
    (oMapping, _) = Array::fold(inComps.clone(), (std::sync::Arc::new({ let __pe_b1 = nodeMark.clone(); move |__pe_a0, __pe_a2| getSccNodeMapping0(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, i32)> + 'static>), (tmpMappingArray.clone(), 1))?;
    Ok(oMapping)
}

fn getSccNodeMapping0(mut iCompsOfNode: Arc<metamodelica::List<i32>>, mut iNodeMarks: metamodelica::Array<i32>, mut iArrayNodeIdx: (metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, i32)> {
    let mut oArrayNodeIdx: (metamodelica::Array<i32>, i32);
    let mut tmpMappingArray: metamodelica::Array<i32>;
    let mut nodeIdx: i32;
    (tmpMappingArray, nodeIdx) = List::fold1(iCompsOfNode, (std::sync::Arc::new(fnptr!(getSccNodeMapping1, i32, metamodelica::Array<i32>, (metamodelica::Array<i32>, i32))) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, (metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, i32)> + 'static>), iNodeMarks.clone(), iArrayNodeIdx)?;
    oArrayNodeIdx = (tmpMappingArray.clone(), nodeIdx + 1);
    Ok(oArrayNodeIdx)
}

fn getSccNodeMapping1(mut iCompIdx: i32, mut iNodeMark: metamodelica::Array<i32>, mut iArrayNodeIdx: (metamodelica::Array<i32>, i32)) -> (metamodelica::Array<i32>, i32) {
    let mut oArrayNodeIdx: (metamodelica::Array<i32>, i32);
    let mut iNodeIdx: i32 = 0;
    let mut nodeMark: i32 = 0;
    let mut iMappingArray: metamodelica::Array<i32> = Default::default();
    oArrayNodeIdx = 'mc: {
        let __mc_input = iArrayNodeIdx;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (mut iMappingArray, mut iNodeIdx) = __mc_input.clone() else { bail!("nomatch") };
            let mut nodeMark: i32 = nodeMark.clone();
            nodeMark = metamodelica::arrayGet(iNodeMark.clone(), iCompIdx)?;
            let true = (intNe(-1, nodeMark)) else { bail!("pattern mismatch") };
            iMappingArray = metamodelica::arrayUpdate(iMappingArray.clone(), iCompIdx, iNodeIdx)?;
            Ok(((iMappingArray.clone(), iNodeIdx), nodeMark.clone()))
        })() { nodeMark = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut iMappingArray, mut iNodeIdx) = __mc_input.clone() else { bail!("nomatch") };
            Ok((iMappingArray.clone(), iNodeIdx))
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oArrayNodeIdx
}

fn othersInTearComp(mut otherEqnVarTpl: (i32, Arc<metamodelica::List<i32>>), mut othersIn: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut othersOut: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    othersOut = 'mc: {
        let __mc_input = othersIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut eq: i32;
                    let mut eqLst: Arc<metamodelica::List<i32>>;
                    let mut varTplLst: Arc<metamodelica::List<i32>>;
                    let mut varLst: Arc<metamodelica::List<i32>>;
                    (eq, varTplLst) = otherEqnVarTpl.clone();
                    (varTplLst.clone()).get(1)?;
                    (eqLst, varLst) = othersIn.clone();
                    varLst = listAppend(varTplLst.clone(), varLst.clone());
                    eqLst = metamodelica::cons(eq.clone(), eqLst.clone());
                    Ok((eqLst.clone(), varLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("check number of vars in relation to number of eqs in otherEqnVarTpl in the torn system\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(othersOut)
}

fn updateMapping(mut varIdx: i32, mut sccIdx: i32, mut iMapping: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut oMapping: metamodelica::Array<i32>;
    oMapping = metamodelica::arrayUpdate(iMapping.clone(), varIdx, sccIdx)?;
    Ok(oMapping)
}

fn updateMappingTuple(mut varIdx: i32, mut sccIdx: i32, mut iEqSysIdx: i32, mut iVarOffset: i32, mut iMapping: metamodelica::Array<(i32, i32, i32)>) -> Result<metamodelica::Array<(i32, i32, i32)>> {
    let mut oMapping: metamodelica::Array<(i32, i32, i32)>;
    oMapping = metamodelica::arrayUpdate(iMapping.clone(), varIdx + iVarOffset, (sccIdx, iEqSysIdx, iVarOffset))?;
    Ok(oMapping)
}

//--------------------------------------------------------
//  Functions to get the ODEsystem graph and adjacencyList
//--------------------------------------------------------
pub(crate) fn getOdeSystem(mut graphIn: TaskGraph, mut graphDataIn: TaskGraphMeta, mut systIn: Arc<BackendDAE::BackendDAE>) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut graphOdeOut: TaskGraph;
    let mut graphDataOdeOut: TaskGraphMeta;
    let mut stateNodes: Arc<metamodelica::List<i32>>;
    let mut whenNodes: Arc<metamodelica::List<i32>>;
    let mut cutNodes: Arc<metamodelica::List<i32>>;
    let mut cutNodeChildren: Arc<metamodelica::List<i32>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut graphTmp: TaskGraph;
    let TaskGraphMeta { varCompMapping: __pa0, eqCompMapping: __pa1, inComps: __pa2, .. } = (graphDataIn.clone()) else { bail!("pattern mismatch") };
    varCompMapping = __pa0.clone();
    eqCompMapping = __pa1.clone();
    inComps = __pa2.clone();
    let __pa3 = ::match_deref::match_deref! { match &(systIn.clone()) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa3, shared: _ } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa3.clone();
    (stateNodes, _) = List::fold2(systs, (std::sync::Arc::new(getAllStateNodes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, metamodelica::Array<(i32, i32, i32)>, metamodelica::Array<Arc<metamodelica::List<i32>>>, (Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32)> + 'static>), varCompMapping.clone(), inComps.clone(), (metamodelica::nil(), 0))?;
    whenNodes = getEventNodes(systIn, eqCompMapping.clone())?;
    graphTmp = metamodelica::arrayFromVec(graphIn.clone().borrow().clone());
    (graphOdeOut, cutNodes) = cutTaskGraph(graphTmp.clone(), stateNodes, whenNodes.clone())?;
    cutNodeChildren = List::flatten(List::map1(listAppend(cutNodes.clone(), whenNodes), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), graphIn.clone())?)?;
    (_, cutNodeChildren, _) = List::intersection1OnTrue(cutNodeChildren, cutNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    graphDataOdeOut = cutSystemData(graphDataIn, listAppend(cutNodes, metamodelica::nil()), cutNodeChildren)?;
    Ok((graphOdeOut, graphDataOdeOut))
}

fn getAllStateNodes(mut systIn: Arc<BackendDAE::EqSystem>, mut varCompMapping: metamodelica::Array<(i32, i32, i32)>, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut stateInfoIn: (Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32)> {
    let mut stateInfoOut: (Arc<metamodelica::List<i32>>, i32);
    stateInfoOut = 'mc: {
        let __mc_input = stateInfoIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (stateNodesIn, varOffset) => {
                    let mut stateNodes: Arc<metamodelica::List<i32>>;
                    let mut stateVars: Arc<metamodelica::List<i32>>;
                    let mut varOffsetNew: i32;
                    let mut orderedVars: BackendDAE::Variables;
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let __pa0 = ::match_deref::match_deref! { match &(systIn.clone()) {
                        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    orderedVars = __pa0.clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    stateVars = getStates(varLst.clone(), metamodelica::nil(), 1)?;
                    let false = (stateVars.clone().is_empty()) else { bail!("pattern mismatch") };
                    stateVars = List::map1(stateVars.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), varOffset.clone())?;
                    stateNodes = getArrayTuple31(stateVars.clone(), varCompMapping.clone())?;
                    stateNodes = List::map3(stateNodes.clone(), (std::sync::Arc::new(getCompInComps) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>) -> Result<i32> + 'static>), 1, inComps.clone(), arrayCreate(metamodelica::arrayLength(inComps.clone()), 0))?;
                    stateNodes = listAppend(stateNodesIn.clone(), stateNodes.clone());
                    varOffsetNew = (varLst.clone().len() as i32) + varOffset.clone();
                    Ok((stateNodes.clone(), varOffsetNew.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (stateNodesIn, varOffset) => {
                    let mut stateVars: Arc<metamodelica::List<i32>>;
                    let mut varOffsetNew: i32;
                    let mut orderedVars: BackendDAE::Variables;
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let __pa0 = ::match_deref::match_deref! { match &(systIn.clone()) {
                        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    orderedVars = __pa0.clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    stateVars = getStates(varLst.clone(), metamodelica::nil(), 1)?;
                    let true = (stateVars.clone().is_empty()) else { bail!("pattern mismatch") };
                    varOffsetNew = (varLst.clone().len() as i32) + varOffset.clone();
                    Ok((stateNodesIn.clone(), varOffsetNew.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut stateVars: Arc<metamodelica::List<i32>>;
                    let mut orderedVars: BackendDAE::Variables;
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    let __pa0 = ::match_deref::match_deref! { match &(systIn.clone()) {
                        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    orderedVars = __pa0.clone();
                    varLst = BackendVariable::varList(orderedVars.clone())?;
                    stateVars = getStates(varLst.clone(), metamodelica::nil(), 1)?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getAllStateNodes failed! StateVars-Count: ")); __mm_s.push_str(&*intString((stateVars.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(stateInfoOut)
}

fn getStates(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut stateVarsIn: Arc<metamodelica::List<i32>>, mut Idx: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut stateVarsOut: Arc<metamodelica::List<i32>>;
    stateVarsOut = 'mc: {
        let __mc_input = inVarLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
                    let mut stateVars: Arc<metamodelica::List<i32>>;
                    let false = (BackendVariable::isStateVar(head.clone())) else { bail!("pattern mismatch") };
                    stateVars = getStates(rest.clone(), stateVarsIn.clone(), Idx + 1)?;
                    Ok(stateVars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
                    let mut stateVars: Arc<metamodelica::List<i32>>;
                    let true = (BackendVariable::isStateVar(head.clone())) else { bail!("pattern mismatch") };
                    stateVars = getStates(rest.clone(), metamodelica::cons(Idx, stateVarsIn.clone()), Idx + 1)?;
                    Ok(stateVars.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(stateVarsIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(stateVarsOut)
}

fn cutTaskGraph(mut graphIn: TaskGraph, mut exceptNodes: Arc<metamodelica::List<i32>>, mut whenNodes: Arc<metamodelica::List<i32>>) -> Result<(TaskGraph, Arc<metamodelica::List<i32>>)> {
    let mut graphOut: TaskGraph;
    let mut cutNodesOut: Arc<metamodelica::List<i32>>;
    (graphOut, cutNodesOut) = 'mc: {
        let __mc_input = exceptNodes.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (-1), tail: Deref @ metamodelica::List::Nil } => {
                    Ok((graphIn.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut sizeDAE: i32;
                    let mut sizeODE: i32;
                    let mut graphT: TaskGraph;
                    let mut graphODE: TaskGraph;
                    let mut cutNodes: Arc<metamodelica::List<i32>>;
                    let mut odeNodes: Arc<metamodelica::List<i32>>;
                    let mut odeMap: metamodelica::Array<i32>;
                    sizeDAE = metamodelica::arrayLength(graphIn.clone());
                    graphT = AdjacencyMatrix::transposeAdjacencyMatrix(graphIn.clone(), sizeDAE.clone())?;
                    odeNodes = listAppend(exceptNodes.clone(), getAllSuccessors(exceptNodes.clone(), graphT.clone())?);
                    (_, odeNodes, _) = List::intersection1OnTrue(odeNodes.clone(), whenNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    (odeNodes, _, _) = List::intersection1OnTrue(List::intRange(sizeDAE.clone()), odeNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    odeNodes = List::sort(odeNodes.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    sizeODE = (odeNodes.clone().len() as i32);
                    odeMap = arrayCreate(sizeDAE.clone(), -1);
                    List::threadMap1_0(odeNodes.clone(), List::intRange(sizeODE.clone()), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), odeMap.clone())?;
                    graphODE = arrayCreate(sizeODE.clone(), metamodelica::nil());
                    (graphODE, cutNodes) = cutTaskGraph2(List::intRange(sizeDAE.clone()), graphODE.clone(), metamodelica::nil(), graphIn.clone(), odeMap.clone())?;
                    Ok((graphODE.clone(), cutNodes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("cutTaskGraph failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((graphOut, cutNodesOut))
}

fn cutTaskGraph2(mut daeNodes: Arc<metamodelica::List<i32>>, mut graphODE: TaskGraph, mut cutNodesIn: Arc<metamodelica::List<i32>>, mut graphDAE: TaskGraph, mut odeMap: metamodelica::Array<i32>) -> Result<(TaskGraph, Arc<metamodelica::List<i32>>)> {
    let mut graphOut: TaskGraph;
    let mut cutNodesOut: Arc<metamodelica::List<i32>>;
    (graphOut, cutNodesOut) = 'mc: {
        let __mc_input = daeNodes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: daeIdx, tail: rest } => {
                    let mut odeIdx: i32;
                    let mut row: Arc<metamodelica::List<i32>>;
                    let mut cutNodes: Arc<metamodelica::List<i32>>;
                    odeIdx = metamodelica::arrayGet(odeMap.clone(), daeIdx.clone())?;
                    let true = (intGt(odeIdx.clone(), 0)) else { bail!("pattern mismatch") };
                    row = metamodelica::arrayGet(graphDAE.clone(), daeIdx.clone())?;
                    row = List::map1(row.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), odeMap.clone())?;
                    row = List::filter1OnTrue(row.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
                    metamodelica::arrayUpdate(graphODE.clone(), odeIdx.clone(), row.clone())?;
                    (_, cutNodes) = cutTaskGraph2(rest.clone(), graphODE.clone(), cutNodesIn.clone(), graphDAE.clone(), odeMap.clone())?;
                    Ok((graphODE.clone(), cutNodes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: daeIdx, tail: rest } => {
                    let mut odeIdx: i32;
                    let mut cutNodes: Arc<metamodelica::List<i32>>;
                    odeIdx = metamodelica::arrayGet(odeMap.clone(), daeIdx.clone())?;
                    let true = (intEq(odeIdx.clone(), -1)) else { bail!("pattern mismatch") };
                    (_, cutNodes) = cutTaskGraph2(rest.clone(), graphODE.clone(), metamodelica::cons(daeIdx.clone(), cutNodesIn.clone()), graphDAE.clone(), odeMap.clone())?;
                    Ok((graphODE.clone(), cutNodes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((graphODE.clone(), cutNodesIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((graphOut, cutNodesOut))
}

fn cutSystemData(mut graphDataIn: TaskGraphMeta, mut cutNodes: Arc<metamodelica::List<i32>>, mut cutNodeChildren: Arc<metamodelica::List<i32>>) -> Result<TaskGraphMeta> {
    let mut graphDataOut: TaskGraphMeta;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut rangeLst: Arc<metamodelica::List<i32>>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let TaskGraphMeta { inComps: __pa0, varCompMapping: __pa1, eqCompMapping: __pa2, compParamMapping: __pa3, compNames: __pa4, compDescs: __pa5, exeCosts: __pa6, commCosts: __pa7, nodeMark: __pa8, compInformations: __pa9 } = (graphDataIn) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    varCompMapping = __pa1.clone();
    eqCompMapping = __pa2.clone();
    compParamMapping = __pa3.clone();
    compNames = __pa4.clone();
    compDescs = __pa5.clone();
    exeCosts = __pa6.clone();
    commCosts = __pa7.clone();
    nodeMark = __pa8.clone();
    compInformations = __pa9.clone();
    inComps = metamodelica::arrayFromVec(List::deletePositions(Arc::new(inComps.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), cutNodes.clone(), false)?.into_iter().cloned().collect());
    rangeLst = List::intRange(metamodelica::arrayLength(nodeMark.clone()));
    nodeMark = List::fold1(rangeLst, (std::sync::Arc::new(markRemovedNodes) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), cutNodes, nodeMark.clone())?;
    graphDataOut = TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    Ok(graphDataOut)
}

fn markRemovedNodes(mut nodeMarkIdx: i32, mut removedNodes: Arc<metamodelica::List<i32>>, mut nodeMarkIn: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut nodeMarkOut: metamodelica::Array<i32>;
    nodeMarkOut = 'mc: {
        let __mc_input = nodeMarkIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (intEq(-2, metamodelica::arrayGet(nodeMarkIn.clone(), nodeMarkIdx)?)) else { bail!("pattern mismatch") };
            Ok(nodeMarkIn.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let false = (List::isMemberOnTrue(nodeMarkIdx, removedNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
            Ok(nodeMarkIn.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nodeMarkTmp: metamodelica::Array<i32>;
            let true = (List::isMemberOnTrue(nodeMarkIdx, removedNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
            nodeMarkTmp = Array::replaceAtWithFill(nodeMarkIdx, -1, 999, nodeMarkIn.clone())?;
            Ok(nodeMarkTmp.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(nodeMarkOut)
}

pub(crate) fn getCompInComps(mut compIn: i32, mut compIdx: i32, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nodeMark: metamodelica::Array<i32>) -> Result<i32> {
    let mut compOut: i32;
    compOut = 'mc: {
        let __mc_input = nodeMark.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut mergedComp: Arc<metamodelica::List<i32>>;
            let mut compTmp: i32;
            let true = (metamodelica::arrayLength(inComps.clone()) >= compIdx) else { bail!("pattern mismatch") };
            mergedComp = metamodelica::arrayGet(inComps.clone(), compIdx)?;
            let false = (List::isMemberOnTrue(compIn, mergedComp.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
            compTmp = getCompInComps(compIn, compIdx + 1, inComps.clone(), nodeMark.clone())?;
            Ok(compTmp.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut mergedComp: Arc<metamodelica::List<i32>>;
            let true = (metamodelica::arrayLength(inComps.clone()) >= compIdx) else { bail!("pattern mismatch") };
            mergedComp = metamodelica::arrayGet(inComps.clone(), compIdx)?;
            let true = (List::isMemberOnTrue(compIn, mergedComp.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
            Ok(compIdx)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nodeMarkEntry: i32;
            nodeMarkEntry = metamodelica::arrayGet(nodeMark.clone(), compIn)?;
            let true = (intLt(nodeMarkEntry.clone(), 0)) else { bail!("pattern mismatch") };
            Ok(-1)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getCompInComps failed! CompIn idx: ")); __mm_s.push_str(&*intString(compIn)); __mm_s.push_str(&*literal!(" | Component array-size: ")); __mm_s.push_str(&*intString(metamodelica::arrayLength(inComps.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(compOut)
}

pub(crate) fn getAllSuccessors(mut nodes: Arc<metamodelica::List<i32>>, mut graph: TaskGraph) -> Result<Arc<metamodelica::List<i32>>> {
    let mut successors: Arc<metamodelica::List<i32>>;
    successors = 'mc: {
        let __mc_input = graph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut alreadyVisited: metamodelica::Array<bool>;
            let mut check: Arc<metamodelica::List<bool>>;
            let mut successors1: Arc<metamodelica::List<i32>>;
            alreadyVisited = arrayCreate(metamodelica::arrayLength(graph.clone()), false);
            List::map2_0(nodes.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), true, alreadyVisited.clone())?;
            successors1 = List::flatten(List::map1(nodes.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), graph.clone())?)?;
            check = List::map1(successors1.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), alreadyVisited.clone())?;
            (_, successors1) = List::filterOnTrueSync(check.clone(), (std::sync::Arc::new(fnptr!(boolNot, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool) -> Result<bool> + 'static>), successors1.clone())?;
            successors1 = List::unique(successors1.clone());
            Ok(getAllSuccessors2(successors1.clone(), graph.clone(), alreadyVisited.clone(), successors1.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("getAllSuccessors failed!\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(successors)
}

fn getAllSuccessors2(mut nodes: Arc<metamodelica::List<i32>>, mut graph: TaskGraph, mut alreadyVisited: metamodelica::Array<bool>, mut successorsIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(nodes.clone()) {
        Deref @ metamodelica::List::Nil => {
            return Ok(List::unique(successorsIn))
        },
        _ => {
            let mut check: Arc<metamodelica::List<bool>>;
            let mut successors1: Arc<metamodelica::List<i32>>;
            successors1 = List::flatten(List::map1(nodes, (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), graph.clone())?)?;
            check = List::map1(successors1.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), alreadyVisited.clone())?;
            (_, successors1) = List::filterOnTrueSync(check, (std::sync::Arc::new(fnptr!(boolNot, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool) -> Result<bool> + 'static>), successors1)?;
            successors1 = List::unique(successors1);
            List::map2_0(successors1.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), true, alreadyVisited.clone())?;
            { (nodes, graph, alreadyVisited, successorsIn) = (successors1.clone(), graph.clone(), alreadyVisited.clone(), listAppend(successors1, successorsIn)); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn getChildNodes(mut adjacencyLstIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut parents: Arc<metamodelica::List<i32>>, mut childLstTmp: Arc<metamodelica::List<i32>>, mut Idx: i32) -> Arc<metamodelica::List<i32>> {
    let mut childLsts: Arc<metamodelica::List<i32>>;
    childLsts = 'mc: {
        let __mc_input = Idx;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut parent: i32;
            let mut row: Arc<metamodelica::List<i32>>;
            let mut childLst: Arc<metamodelica::List<i32>>;
            let true = ((parents.clone().len() as i32) >= Idx) else { bail!("pattern mismatch") };
            parent = (parents.clone()).get(Idx)?;
            row = metamodelica::arrayGet(adjacencyLstIn.clone(), parent.clone())?;
            childLst = listAppend(childLstTmp.clone(), row.clone());
            childLst = getChildNodes(adjacencyLstIn.clone(), parents.clone(), childLst.clone(), Idx + 1);
            Ok(childLst.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(childLstTmp.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    childLsts
}

pub(crate) fn updateContinuousEntriesInList(mut lstIn: Arc<metamodelica::List<i32>>, mut deleteEntriesIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut lstOut: Arc<metamodelica::List<i32>>;
    lstOut = (::match_deref::match_deref! { match &((lstIn.clone(), deleteEntriesIn.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (_, Deref @ metamodelica::List::Nil) => {
            lstIn
        },
        (Deref @ metamodelica::List::Cons { head: start, tail: rest }, _) => {
            let mut lstTmp: Arc<metamodelica::List<i32>>;
            let mut deleteArr: metamodelica::Array<i32>;
            deleteArr = arrayCreate(List::fold(listAppend(rest.clone(), deleteEntriesIn.clone()), (std::sync::Arc::new(fnptr!(intMax, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), start.clone())?, 0);
            List::map2_0(deleteEntriesIn, (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), 1, deleteArr.clone())?;
            (deleteArr, _) = Array::mapFold(deleteArr.clone(), (std::sync::Arc::new(setDeleteArr) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<(i32, i32)> + 'static>), 0)?;
            lstTmp = List::map1(lstIn, (std::sync::Arc::new(fnptr!(removeContinuousEntries1, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<i32> + 'static>), deleteArr.clone())?;
            lstTmp
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(lstOut)
}

fn setDeleteArr(mut entryIn: i32, mut offsetIn: i32) -> Result<(i32, i32)> {
    let mut entryOut: i32;
    let mut offsetOut: i32;
    (entryOut, offsetOut) = (match entryIn {
        0 => (offsetIn, offsetIn),
        1 => (offsetIn + 1, offsetIn + 1),
        _ => bail!("match: no arm matched"),
    });
    Ok((entryOut, offsetOut))
}

fn removeContinuousEntries1(mut entryIn: i32, mut deleteEntriesIn: metamodelica::Array<i32>) -> i32 {
    let mut entryOut: i32;
    entryOut = 'mc: {
        let __mc_input = deleteEntriesIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut offset: i32;
            offset = metamodelica::arrayGet(deleteEntriesIn.clone(), entryIn)?;
            Ok(entryIn - offset.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("removeContinuousEntries1 failed!\n")).clone());
            Ok(entryIn)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    entryOut
}

fn deleteRowInAdjLst(mut adjacencyLstIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowsDel: Arc<metamodelica::List<i32>>) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>)> {
    let mut adjacencyLstOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut odeMapping: Arc<metamodelica::List<i32>>;
    let mut adjLst: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut copiedRows: Arc<metamodelica::List<i32>>;
    let mut size: i32;
    size = metamodelica::arrayLength(adjacencyLstIn.clone()) - (rowsDel.clone().len() as i32);
    adjLst = arrayCreate(size, metamodelica::nil());
    copiedRows = List::intRange(metamodelica::arrayLength(adjacencyLstIn.clone()));
    copiedRows = List::deletePositions(copiedRows, rowsDel, false)?;
    adjacencyLstOut = arrayCopyRows(adjacencyLstIn.clone(), adjLst.clone(), copiedRows.clone(), 1);
    odeMapping = copiedRows;
    Ok((adjacencyLstOut, odeMapping))
}

fn arrayCopyRows(mut inArray: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut newArray: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut copiedRows: Arc<metamodelica::List<i32>>, mut Idx: i32) -> metamodelica::Array<Arc<metamodelica::List<i32>>> {
    let mut outArray: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    outArray = 'mc: {
        let __mc_input = Idx;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut copyRow: i32;
            let mut row: Arc<metamodelica::List<i32>>;
            let mut arrayTmp: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let true = ((copiedRows.clone().len() as i32) >= Idx) else { bail!("pattern mismatch") };
            copyRow = (copiedRows.clone()).get(Idx)?;
            row = metamodelica::arrayGet(inArray.clone(), copyRow.clone())?;
            arrayTmp = Array::replaceAtWithFill(Idx, row.clone(), list![111, 222], newArray.clone())?;
            arrayTmp = arrayCopyRows(inArray.clone(), arrayTmp.clone(), copiedRows.clone(), Idx + 1);
            Ok(arrayTmp.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(newArray.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outArray
}

pub(crate) fn getRootNodes(mut iTaskGraph: TaskGraph) -> Result<Arc<metamodelica::List<i32>>> {
    let mut rootsOut: Arc<metamodelica::List<i32>>;
    let mut size: i32;
    let mut taskGraphT: TaskGraph;
    size = metamodelica::arrayLength(iTaskGraph.clone());
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size)?;
    rootsOut = getLeafNodes(taskGraphT.clone())?;
    Ok(rootsOut)
}

pub(crate) fn getLeafNodes(mut iTaskGraph: TaskGraph) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oLeafNodes: Arc<metamodelica::List<i32>>;
    let mut tmpLeafNodes: Arc<metamodelica::List<i32>>;
    let mut nodeSuccessors: Arc<metamodelica::List<i32>>;
    let mut nodeIdx: i32 = 0;
    tmpLeafNodes = metamodelica::nil();
    for mut nodeIdx in 1..=metamodelica::arrayLength(iTaskGraph.clone()) {
        nodeSuccessors = metamodelica::arrayGet(iTaskGraph.clone(), nodeIdx)?;
        if nodeSuccessors.clone().is_empty() {
            tmpLeafNodes = metamodelica::cons(nodeIdx, tmpLeafNodes.clone());
        }
    }
    oLeafNodes = tmpLeafNodes;
    Ok(oLeafNodes)
}

pub(crate) fn getLevelNodes(mut iTaskGraph: TaskGraph) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut oLevelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut refCounter: metamodelica::Array<i32>;
    let mut roots: Arc<metamodelica::List<i32>>;
    refCounter = createRefCounter(iTaskGraph.clone())?;
    roots = getNodesWithRefCountZero(refCounter.clone())?;
    oLevelNodes = getLevelNodes0(iTaskGraph.clone(), refCounter.clone(), roots, metamodelica::nil())?;
    Ok(oLevelNodes)
}

fn getLevelNodes0(mut iTaskGraph: TaskGraph, mut iRefCounter: metamodelica::Array<i32>, mut iNodesWithRefZero: Arc<metamodelica::List<i32>>, mut iLevelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    '__tco: loop {
        let mut tmpLevelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        let mut zeroRefNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
        ::match_deref::match_deref! { match &(iNodesWithRefZero) {
        Deref @ metamodelica::List::Nil => return Ok(iLevelNodes.reverse()),
        __esc_zeroRefNodes => {
            zeroRefNodes = (*__esc_zeroRefNodes).clone();
            tmpLevelNodes = metamodelica::cons(zeroRefNodes.clone(), iLevelNodes);
            zeroRefNodes = List::fold2(zeroRefNodes.clone(), (std::sync::Arc::new(getLevelNodes1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iTaskGraph.clone(), iRefCounter.clone(), metamodelica::nil())?;
            { (iTaskGraph, iRefCounter, iNodesWithRefZero, iLevelNodes) = (iTaskGraph.clone(), iRefCounter.clone(), zeroRefNodes.clone(), tmpLevelNodes); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn getLevelNodes1(mut iNodeIdx: i32, mut iTaskGraph: TaskGraph, mut iRefCounter: metamodelica::Array<i32>, mut iNodesWithRefZero: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oNodesWithRefZero: Arc<metamodelica::List<i32>>;
    let mut childNodes: Arc<metamodelica::List<i32>>;
    let mut tmpNodesWithRefZero: Arc<metamodelica::List<i32>>;
    childNodes = metamodelica::arrayGet(iTaskGraph.clone(), iNodeIdx)?;
    tmpNodesWithRefZero = List::fold1(childNodes, (std::sync::Arc::new(fnptr!(getLevelNodes2, i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iRefCounter.clone(), metamodelica::nil())?;
    oNodesWithRefZero = listAppend(tmpNodesWithRefZero, iNodesWithRefZero);
    Ok(oNodesWithRefZero)
}

fn getLevelNodes2(mut iNodeIdx: i32, mut iRefCounter: metamodelica::Array<i32>, mut iNodesWithRefZero: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut oNodesWithRefZero: Arc<metamodelica::List<i32>>;
    let mut tmpNodesWithRefZero: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut refCounter: i32 = 0;
    oNodesWithRefZero = 'mc: {
        let __mc_input = iNodesWithRefZero.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                tmpNodesWithRefZero => {
                    let mut tmpNodesWithRefZero = (*tmpNodesWithRefZero).clone();
                    let mut refCounter: i32 = refCounter.clone();
                    refCounter = metamodelica::arrayGet(iRefCounter.clone(), iNodeIdx)? - 1;
                    metamodelica::arrayUpdate(iRefCounter.clone(), iNodeIdx, refCounter)?;
                    let true = (intEq(refCounter, 0)) else { bail!("pattern mismatch") };
                    tmpNodesWithRefZero = metamodelica::cons(iNodeIdx, tmpNodesWithRefZero.clone());
                    Ok((tmpNodesWithRefZero.clone(), refCounter.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { refCounter = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iNodesWithRefZero.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oNodesWithRefZero
}

fn createRefCounter(mut iTaskGraph: TaskGraph) -> Result<metamodelica::Array<i32>> {
    let mut oRefCounter: metamodelica::Array<i32>;
    let mut tmpRefCounter: metamodelica::Array<i32>;
    tmpRefCounter = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), 0);
    tmpRefCounter = Array::fold(iTaskGraph.clone(), (std::sync::Arc::new(createRefCounter0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), tmpRefCounter.clone())?;
    oRefCounter = tmpRefCounter.clone();
    Ok(oRefCounter)
}

fn createRefCounter0(mut iChildNodes: Arc<metamodelica::List<i32>>, mut iRefCounter: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    '__tco: loop {
        let mut tmpRefCounter: metamodelica::Array<i32> = Default::default();
        let mut counter: i32 = 0;
        let mut head: i32 = 0;
        let mut tail: Arc<metamodelica::List<i32>> = metamodelica::nil();
        ::match_deref::match_deref! { match &(iChildNodes) {
        Deref @ metamodelica::List::Nil => return Ok(iRefCounter.clone()),
        Deref @ metamodelica::List::Cons { head: __esc_head, tail: __esc_tail } => {
            head = (*__esc_head).clone();
            tail = (*__esc_tail).clone();
            counter = metamodelica::arrayGet(iRefCounter.clone(), head.clone())? + 1;
            tmpRefCounter = metamodelica::arrayUpdate(iRefCounter.clone(), head.clone(), counter)?;
            { (iChildNodes, iRefCounter) = (tail.clone(), tmpRefCounter.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn getNodesWithRefCountZero(mut iRefCounter: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oZeroIdc: Arc<metamodelica::List<i32>>;
    (oZeroIdc, _) = Array::fold(iRefCounter.clone(), (std::sync::Arc::new(fnptr!(getNodesWithRefCountZero0, i32, (Arc<metamodelica::List<i32>>, i32))) as std::sync::Arc<dyn ::std::ops::Fn(i32, (Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32)> + 'static>), (metamodelica::nil(), 1))?;
    Ok(oZeroIdc)
}

fn getNodesWithRefCountZero0(mut iRefCount: i32, mut iZeroIdc: (Arc<metamodelica::List<i32>>, i32)) -> (Arc<metamodelica::List<i32>>, i32) {
    let mut oZeroIdc: (Arc<metamodelica::List<i32>>, i32);
    let mut resultList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut currentNodeIdx: i32 = 0;
    oZeroIdc = (::match_deref::match_deref! { match &((iRefCount, iZeroIdc)) {
        (0, (__esc_resultList, __esc_currentNodeIdx)) => {
            resultList = (*__esc_resultList).clone();
            currentNodeIdx = (*__esc_currentNodeIdx).clone();
            resultList = metamodelica::cons(currentNodeIdx.clone(), resultList.clone());
            (resultList.clone(), currentNodeIdx.clone() + 1)
        },
        (_, (__esc_resultList, __esc_currentNodeIdx)) => {
            resultList = (*__esc_resultList).clone();
            currentNodeIdx = (*__esc_currentNodeIdx).clone();
            (resultList.clone(), currentNodeIdx.clone() + 1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oZeroIdc
}

//----------------------------------
//  Functions to get the event-graph
//----------------------------------
pub(crate) fn getZeroFuncsSystem(mut iTaskGraph: TaskGraph, mut iTaskGraphMeta: TaskGraphMeta, mut iBackendDAE: Arc<BackendDAE::BackendDAE>, mut iNumberOfSccs: i32, mut iZeroCrossingEquationIdc: Arc<metamodelica::List<i32>>, mut iSimCodeEqCompMapping: metamodelica::Array<i32>) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut oTaskGraph: TaskGraph;
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut nodeList: Arc<metamodelica::List<i32>>;
    let mut newNodeList: Arc<metamodelica::List<i32>>;
    let mut predecessors: Arc<metamodelica::List<i32>>;
    let mut successors: Arc<metamodelica::List<i32>>;
    let mut successorsTmp: Arc<metamodelica::List<i32>>;
    let mut predecessorsTmp: Arc<metamodelica::List<i32>>;
    let mut zeroFuncNodeMarks: metamodelica::Array<i32>;
    let mut sccNodeMapping: metamodelica::Array<i32>;
    let mut handledNodes: metamodelica::Array<bool>;
    let mut whenNodeMarks: metamodelica::Array<bool>;
    let mut iTaskGraphTCopy: TaskGraph;
    let mut iTaskGraphCopy: TaskGraph;
    let mut zeroFuncTaskGraph: TaskGraph;
    let mut zeroFuncTaskGraphMeta: TaskGraphMeta;
    let mut whenNodes: Arc<metamodelica::List<i32>>;
    let mut zeroFuncInComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqIdx: i32 = 0;
    let mut compIdx: i32;
    let mut nodeIdx: i32 = 0;
    let mut successor: i32 = 0;
    let mut predecessor: i32 = 0;
    let mut zeroFuncNodeMark: i32;
    let mut successorMark: i32;
    let mut zeroFuncNodeCount: i32;
    let mut zeroFuncNodeIdx: i32;
    let mut nodeToZeroFuncNodeMapping: metamodelica::Array<i32>;
    let mut stop: bool;
    let TaskGraphMeta { inComps: __pa0, eqCompMapping: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    eqCompMapping = __pa1.clone();
    zeroFuncNodeMarks = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), 0);
    handledNodes = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), false);
    nodeToZeroFuncNodeMapping = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), -1);
    whenNodes = getEventNodes(iBackendDAE, eqCompMapping.clone())?;
    whenNodeMarks = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), false);
    sccNodeMapping = getSccNodeMapping(iNumberOfSccs, iTaskGraphMeta.clone())?;
    iTaskGraphCopy = metamodelica::arrayFromVec(iTaskGraph.clone().borrow().clone());
    iTaskGraphTCopy = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
    for mut eqIdx in &*iZeroCrossingEquationIdc {
        let mut eqIdx = eqIdx.clone();
        compIdx = metamodelica::arrayGet(iSimCodeEqCompMapping.clone(), eqIdx)?;
        nodeIdx = metamodelica::arrayGet(sccNodeMapping.clone(), compIdx)?;
        zeroFuncNodeMarks = metamodelica::arrayUpdate(zeroFuncNodeMarks.clone(), nodeIdx, 1)?;
    }
    for mut nodeIdx in &*whenNodes {
        let mut nodeIdx = nodeIdx.clone();
        whenNodeMarks = metamodelica::arrayUpdate(whenNodeMarks.clone(), nodeIdx, true)?;
    }
    nodeList = getRootNodes(iTaskGraphTCopy.clone())?;
    zeroFuncNodeCount = 0;
    zeroFuncNodeIdx = 1;
    while boolNot(nodeList.clone().is_empty()) {
        newNodeList = metamodelica::nil();
        for mut nodeIdx in &*nodeList.clone() {
            let mut nodeIdx = nodeIdx.clone();
            if boolNot(metamodelica::arrayGet(handledNodes.clone(), nodeIdx)?) {
                handledNodes = metamodelica::arrayUpdate(handledNodes.clone(), nodeIdx, true)?;
                predecessors = metamodelica::arrayGet(iTaskGraphTCopy.clone(), nodeIdx)?;
                successors = metamodelica::arrayGet(iTaskGraphCopy.clone(), nodeIdx)?;
                zeroFuncNodeMark = -1;
                if metamodelica::arrayGet(whenNodeMarks.clone(), nodeIdx)? {
                    for mut predecessor in &*predecessors.clone() {
                        let mut predecessor = predecessor.clone();
                        successorsTmp = metamodelica::arrayGet(iTaskGraphCopy.clone(), predecessor)?;
                        metamodelica::arrayUpdate(iTaskGraphCopy.clone(), predecessor, listAppend(successorsTmp.clone(), successors.clone()))?;
                    }
                    for mut successor in &*successors.clone() {
                        let mut successor = successor.clone();
                        predecessorsTmp = metamodelica::arrayGet(iTaskGraphTCopy.clone(), successor)?;
                        metamodelica::arrayUpdate(iTaskGraphTCopy.clone(), successor, listAppend(predecessorsTmp.clone(), predecessors.clone()))?;
                    }
                } else {
                    if intGt(metamodelica::arrayGet(zeroFuncNodeMarks.clone(), nodeIdx)?, 0) {
                        zeroFuncNodeMark = zeroFuncNodeIdx;
                    } else {
                        stop = false;
                        while boolAnd(boolNot(stop), boolNot(successors.clone().is_empty())) {
                            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(successors.clone()) {
                                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                                _ => bail!("pattern mismatch"),
                            } };
                            successor = __pa2.clone();
                            successors = __pa3.clone();
                            successorMark = metamodelica::arrayGet(zeroFuncNodeMarks.clone(), successor)?;
                            if intGt(successorMark, 0) {
                                zeroFuncNodeMark = zeroFuncNodeIdx;
                                stop = true;
                            }
                        }
                    }
                    if intGt(zeroFuncNodeMark, 0) {
                        zeroFuncNodeCount = zeroFuncNodeCount + 1;
                        nodeToZeroFuncNodeMapping = metamodelica::arrayUpdate(nodeToZeroFuncNodeMapping.clone(), nodeIdx, zeroFuncNodeCount)?;
                        zeroFuncNodeIdx = zeroFuncNodeIdx + 1;
                    }
                }
                zeroFuncNodeMarks = metamodelica::arrayUpdate(zeroFuncNodeMarks.clone(), nodeIdx, zeroFuncNodeMark)?;
                newNodeList = List::append_reverse(predecessors.clone(), newNodeList.clone());
            }
        }
        nodeList = newNodeList.clone().reverse();
    }
    zeroFuncTaskGraph = arrayCreate(zeroFuncNodeCount, metamodelica::nil());
    zeroFuncInComps = arrayCreate(zeroFuncNodeCount, metamodelica::nil());
    nodeIdx = metamodelica::arrayLength(zeroFuncNodeMarks.clone());
    while intGt(nodeIdx, 0) {
        zeroFuncNodeIdx = metamodelica::arrayGet(zeroFuncNodeMarks.clone(), nodeIdx)?;
        if intGt(zeroFuncNodeIdx, 0) {
            successors = metamodelica::arrayGet(iTaskGraphCopy.clone(), nodeIdx)?;
            zeroFuncInComps = metamodelica::arrayUpdate(zeroFuncInComps.clone(), zeroFuncNodeIdx, metamodelica::arrayGet(inComps.clone(), nodeIdx)?)?;
            newNodeList = metamodelica::nil();
            while boolNot(successors.clone().is_empty()) {
                let (__pa4, __pa5) = ::match_deref::match_deref! { match &(successors.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                successor = __pa4.clone();
                successors = __pa5.clone();
                successor = metamodelica::arrayGet(zeroFuncNodeMarks.clone(), successor)?;
                if intGt(successor, 0) {
                    newNodeList = metamodelica::cons(successor, newNodeList.clone());
                }
            }
            newNodeList = List::sort(newNodeList.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            newNodeList = List::sortedUnique(newNodeList.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            zeroFuncTaskGraph = metamodelica::arrayUpdate(zeroFuncTaskGraph.clone(), zeroFuncNodeIdx, newNodeList.clone())?;
        }
        nodeIdx = nodeIdx - 1;
    }
    zeroFuncTaskGraphMeta = copyTaskGraphMeta(iTaskGraphMeta)?;
    zeroFuncTaskGraphMeta = setInCompsInMeta(zeroFuncInComps.clone(), zeroFuncTaskGraphMeta)?;
    (oTaskGraph, oTaskGraphMeta) = reverseTaskGraphIndices(zeroFuncTaskGraph.clone(), zeroFuncTaskGraphMeta)?;
    Ok((oTaskGraph, oTaskGraphMeta))
}

fn reverseTaskGraphIndices(mut iTaskGraph: TaskGraph, mut iTaskGraphMeta: TaskGraphMeta) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut oTaskGraph: TaskGraph;
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut nTasks: i32;
    let mut idxMap: metamodelica::Array<i32>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    nTasks = metamodelica::arrayLength(iTaskGraph.clone());
    idxMap = arrayCreate(nTasks, -1);
    let TaskGraphMeta { inComps: __pa0, varCompMapping: __pa1, eqCompMapping: __pa2, compParamMapping: __pa3, compNames: __pa4, compDescs: __pa5, exeCosts: __pa6, commCosts: __pa7, nodeMark: __pa8, compInformations: __pa9 } = (iTaskGraphMeta) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    varCompMapping = __pa1.clone();
    eqCompMapping = __pa2.clone();
    compParamMapping = __pa3.clone();
    compNames = __pa4.clone();
    compDescs = __pa5.clone();
    exeCosts = __pa6.clone();
    commCosts = __pa7.clone();
    nodeMark = __pa8.clone();
    compInformations = __pa9.clone();
    for mut i in 1..=nTasks {
        idxMap = metamodelica::arrayUpdate(idxMap.clone(), i.clone(), nTasks - i.clone() + 1)?;
    }
    (oTaskGraph, _) = Array::mapNoCopy_1(iTaskGraph.clone(), (std::sync::Arc::new(mapIntegers) as std::sync::Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<(Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)> + 'static>), idxMap.clone())?;
    oTaskGraph = Array::reverse(oTaskGraph.clone())?;
    inComps = Array::reverse(inComps.clone())?;
    oTaskGraphMeta = TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    Ok((oTaskGraph, oTaskGraphMeta))
}

fn mapIntegers(mut iTpl: (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<(Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)> {
    let mut oTpl: (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>);
    let mut map: metamodelica::Array<i32>;
    let mut iLst: Arc<metamodelica::List<i32>>;
    let mut oLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (iLst, map) = iTpl;
    for mut i in &*iLst {
        let mut i = i.clone();
        oLst = metamodelica::cons(metamodelica::arrayGet(map.clone(), i.clone())?, oLst.clone());
    }
    oLst = oLst.reverse();
    oTpl = (oLst, map.clone());
    Ok(oTpl)
}

fn getEventSystem(mut iTaskGraph: TaskGraph, mut iTaskGraphMeta: TaskGraphMeta, mut iSyst: Arc<BackendDAE::BackendDAE>, mut iZeroCrossings: Arc<metamodelica::List<BackendDAE::ZeroCrossing>>, mut iSimCodeEqCompMapping: metamodelica::Array<i32>) -> Result<(TaskGraph, TaskGraphMeta)> {
    let mut oTaskGraph: TaskGraph;
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut discreteNodes: Arc<metamodelica::List<i32>>;
    let mut cutNodes: Arc<metamodelica::List<i32>>;
    let mut cutNodeChildren: Arc<metamodelica::List<i32>>;
    let mut zeroCrossingNodes: Arc<metamodelica::List<i32>>;
    let mut sccsContainingTime: Arc<metamodelica::List<i32>>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut graphTmp: TaskGraph;
    let TaskGraphMeta { varCompMapping: __pa0, eqCompMapping: __pa1, inComps: __pa2, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    varCompMapping = __pa0.clone();
    eqCompMapping = __pa1.clone();
    inComps = __pa2.clone();
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(iSyst.clone()) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa3, shared: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa3.clone();
    shared = __pa4.clone();
    discreteNodes = getDiscreteNodes(iSyst, eqCompMapping.clone())?;
    zeroCrossingNodes = List::flatten(List::map1(iZeroCrossings, (std::sync::Arc::new(fnptr!(getComponentsOfZeroCrossing, BackendDAE::ZeroCrossing, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::ZeroCrossing, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iSimCodeEqCompMapping.clone())?)?;
    sccsContainingTime = metamodelica::nil();
    discreteNodes = List::flatten(list![discreteNodes, sccsContainingTime, zeroCrossingNodes])?;
    graphTmp = iTaskGraph.clone();
    (graphTmp, cutNodes) = cutTaskGraph(graphTmp.clone(), discreteNodes, metamodelica::nil())?;
    cutNodeChildren = List::flatten(List::map1(cutNodes.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), iTaskGraph.clone())?)?;
    (_, cutNodeChildren, _) = List::intersection1OnTrue(cutNodeChildren, cutNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    oTaskGraphMeta = cutSystemData(iTaskGraphMeta, cutNodes, cutNodeChildren)?;
    oTaskGraph = graphTmp.clone();
    Ok((oTaskGraph, oTaskGraphMeta))
}

fn getComponentsOfZeroCrossing(mut iZeroCrossing: BackendDAE::ZeroCrossing, mut iSimCodeEqCompMapping: metamodelica::Array<i32>) -> Arc<metamodelica::List<i32>> {
    let mut oCompIdc: Arc<metamodelica::List<i32>>;
    let mut occurEquLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpCompIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oCompIdc = 'mc: {
        let __mc_input = iZeroCrossing;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let BackendDAE::ZeroCrossing { occurEquLst: mut occurEquLst, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut occurEquLst = occurEquLst.clone();
            let mut tmpCompIdc: Arc<metamodelica::List<i32>> = tmpCompIdc.clone();
            occurEquLst = List::filter1OnTrue(occurEquLst.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getComponentsOfZeroCrossing: simEqs: ")); __mm_s.push_str(&*stringDelimitList(List::map(occurEquLst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            tmpCompIdc = List::map1(occurEquLst.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), iSimCodeEqCompMapping.clone())?;
            tmpCompIdc = List::filter1OnTrue(tmpCompIdc.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0)?;
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getComponentsOfZeroCrossing: components: ")); __mm_s.push_str(&*stringDelimitList(List::map(tmpCompIdc.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok((tmpCompIdc.clone(), tmpCompIdc.clone()))
        })() { tmpCompIdc = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(metamodelica::nil())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oCompIdc
}

fn getComponentsIncludingTime(mut iSystem: Arc<BackendDAE::EqSystem>, mut iEqCompMapping: metamodelica::Array<(i32, i32, i32)>, mut iOffsetResList: (i32, Arc<metamodelica::List<i32>>)) -> Result<(i32, Arc<metamodelica::List<i32>>)> {
    let mut oOffsetResList: (i32, Arc<metamodelica::List<i32>>);
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut offset: i32;
    let mut resultList: Arc<metamodelica::List<i32>>;
    let __pa0 = ::match_deref::match_deref! { match &(iSystem) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    orderedEqs = __pa0.clone();
    (offset, resultList) = iOffsetResList;
    (offset, resultList, _, _) = BackendEquation::traverseEquationArray(orderedEqs, (std::sync::Arc::new(fnptr!(getComponentsIncludingTime0, Arc<BackendDAE::Equation>, (i32, Arc<metamodelica::List<i32>>, metamodelica::Array<(i32, i32, i32)>, i32))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (i32, Arc<metamodelica::List<i32>>, metamodelica::Array<(i32, i32, i32)>, i32)) -> Result<(Arc<BackendDAE::Equation>, (i32, Arc<metamodelica::List<i32>>, metamodelica::Array<(i32, i32, i32)>, i32))> + 'static>), (offset, resultList, iEqCompMapping.clone(), 1))?;
    oOffsetResList = (offset, resultList);
    Ok(oOffsetResList)
}

fn getComponentsIncludingTime0(mut inEq: Arc<BackendDAE::Equation>, mut iOffsetResList: (i32, Arc<metamodelica::List<i32>>, metamodelica::Array<(i32, i32, i32)>, i32)) -> (Arc<BackendDAE::Equation>, (i32, Arc<metamodelica::List<i32>>, metamodelica::Array<(i32, i32, i32)>, i32)) {
    let mut outEq: Arc<BackendDAE::Equation>;
    let mut oOffsetResList: (i32, Arc<metamodelica::List<i32>>, metamodelica::Array<(i32, i32, i32)>, i32);
    let mut eq: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut offset: i32 = 0;
    let mut eqIdx: i32 = 0;
    let mut sccIdx: i32 = 0;
    let mut resultList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)> = Default::default();
    (outEq, oOffsetResList) = 'mc: {
        let __mc_input = (inEq, iOffsetResList);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eq, (offset, resultList, eqCompMapping, eqIdx)) => {
                    let mut resultList = (*resultList).clone();
                    let mut sccIdx: i32 = sccIdx.clone();
                    (sccIdx, _, _) = metamodelica::arrayGet(eqCompMapping.clone(), eqIdx.clone() + offset.clone())?;
                    let true = (BackendDAEUtil::traverseBackendDAEExpsOptEqn(Some(eq.clone()), (std::sync::Arc::new(getComponentsIncludingTime1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?) else { bail!("pattern mismatch") };
                    resultList = metamodelica::cons(sccIdx, resultList.clone());
                    Ok(((eq.clone(), (offset.clone(), resultList.clone(), eqCompMapping.clone(), eqIdx.clone() + 1)), sccIdx.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { sccIdx = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (eq, (offset, resultList, eqCompMapping, eqIdx)) => {
                    Ok((eq.clone(), (offset.clone(), resultList.clone(), eqCompMapping.clone(), eqIdx.clone() + 1)))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outEq, oOffsetResList)
}

fn getComponentsIncludingTime1(mut inExp: Arc<DAE::Exp>, mut inB: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut e: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut res: bool = false;
    (e, res) = (::match_deref::match_deref! { match &((inExp.clone(), inB)) {
        (__esc_e, false) => {
            e = (*__esc_e).clone();
            res = Expression::traverseCrefsFromExp(e.clone(), (std::sync::Arc::new(fnptr!(getComponentsIncludingTime2, Arc<DAE::ComponentRef>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, bool) -> Result<bool> + 'static>), false)?;
            (e.clone(), res)
        },
        _ => (inExp, inB),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((e, res))
}

fn getComponentsIncludingTime2(mut iRef: Arc<DAE::ComponentRef>, mut iIncludingTime: bool) -> bool {
    let mut oIncludingTime: bool;
    oIncludingTime = (::match_deref::match_deref! { match &(iRef) {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. } => true,
        _ => false || iIncludingTime,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oIncludingTime
}

fn getDiscreteNodes(mut systIn: Arc<BackendDAE::BackendDAE>, mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut eventNodes: Arc<metamodelica::List<i32>>;
    let mut eqLst: Arc<metamodelica::List<i32>>;
    let mut systemsIn: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let __pa0 = ::match_deref::match_deref! { match &(systIn) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    systemsIn = __pa0.clone();
    (eqLst, _) = List::fold(systemsIn, (std::sync::Arc::new(getDiscreteNodesEqs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32)> + 'static>), (metamodelica::nil(), 0))?;
    eventNodes = getArrayTuple31(eqLst, eqCompMapping.clone())?;
    Ok(eventNodes)
}

fn getDiscreteNodesEqs(mut systIn: Arc<BackendDAE::EqSystem>, mut eventInfoIn: (Arc<metamodelica::List<i32>>, i32)) -> Result<(Arc<metamodelica::List<i32>>, i32)> {
    let mut eventInfoOut: (Arc<metamodelica::List<i32>>, i32);
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut orderedVars: BackendDAE::Variables;
    let mut matching: Arc<BackendDAE::Matching>;
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eventEqs: Arc<metamodelica::List<i32>>;
    let mut eventEqsIn: Arc<metamodelica::List<i32>>;
    let mut offset: i32;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(systIn) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, orderedVars: __pa1, matching: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    orderedEqs = __pa0.clone();
    orderedVars = __pa1.clone();
    matching = __pa2.clone();
    comps = BackendDAEUtil::getCompsOfMatching(matching);
    (eventEqsIn, offset) = eventInfoIn;
    eventEqs = getDiscreteNodesEqs1(comps, offset, orderedVars, metamodelica::nil());
    offset = offset + ExpandableArray::getNumberOfElements(orderedEqs);
    eventInfoOut = (listAppend(eventEqs, eventEqsIn), offset);
    Ok(eventInfoOut)
}

fn getDiscreteNodesEqs1(mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut offset: i32, mut iOrderedVars: BackendDAE::Variables, mut discreteEqsIn: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut discreteEqsOut: Arc<metamodelica::List<i32>>;
    discreteEqsOut = 'mc: {
        let __mc_input = comps;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
                    let mut eqn: i32;
                    let mut eventEqs: Arc<metamodelica::List<i32>>;
                    let (true, __pa0) = (solvesDiscreteValue(head.clone(), iOrderedVars.clone())) else { bail!("pattern mismatch") };
                    eqn = __pa0.clone();
                    eqn = eqn.clone() + offset;
                    eventEqs = getDiscreteNodesEqs1(rest.clone(), offset, iOrderedVars.clone(), metamodelica::cons(eqn.clone(), discreteEqsIn.clone()));
                    Ok(eventEqs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut eventEqs: Arc<metamodelica::List<i32>>;
                    eventEqs = getDiscreteNodesEqs1(rest.clone(), offset, iOrderedVars.clone(), discreteEqsIn.clone());
                    Ok(eventEqs.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(discreteEqsIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    discreteEqsOut
}

fn solvesDiscreteValue(mut inComp: Arc<BackendDAE::StrongComponent>, mut iOrderedVars: BackendDAE::Variables) -> (bool, i32) {
    let mut oSolvesDiscreteValue: bool;
    let mut oFirstEqIdx: i32;
    (oSolvesDiscreteValue, oFirstEqIdx) = 'mc: {
        let __mc_input = inComp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var, eqn } => {
                    let mut backendVar: BackendDAE::Var;
                    let mut solvesDiscreteValue: bool;
                    backendVar = BackendVariable::getVarAt(iOrderedVars.clone(), var.clone())?;
                    solvesDiscreteValue = BackendVariable::isVarDiscrete(backendVar.clone());
                    Ok((solvesDiscreteValue.clone(), eqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars, eqns, .. } => {
                    let mut eqn: i32;
                    let mut backendVars: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut solvesDiscreteValue: bool;
                    backendVars = List::map1r(vars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), iOrderedVars.clone())?;
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(backendVars.clone());
                    eqn = listHead(eqns.clone())?;
                    Ok((solvesDiscreteValue.clone(), eqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEARRAY { vars, eqn } => {
                    let mut backendVars: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut solvesDiscreteValue: bool;
                    backendVars = List::map1r(vars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), iOrderedVars.clone())?;
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(backendVars.clone());
                    Ok((solvesDiscreteValue.clone(), eqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEWHENEQUATION { vars, eqn } => {
                    let mut backendVars: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut solvesDiscreteValue: bool;
                    backendVars = List::map1r(vars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), iOrderedVars.clone())?;
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(backendVars.clone());
                    Ok((solvesDiscreteValue.clone(), eqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLECOMPLEXEQUATION { vars, eqn } => {
                    let mut backendVars: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut solvesDiscreteValue: bool;
                    backendVars = List::map1r(vars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), iOrderedVars.clone())?;
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(backendVars.clone());
                    Ok((solvesDiscreteValue.clone(), eqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEALGORITHM { vars, eqn } => {
                    let mut backendVars: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut solvesDiscreteValue: bool;
                    backendVars = List::map1r(vars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), iOrderedVars.clone())?;
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(backendVars.clone());
                    Ok((solvesDiscreteValue.clone(), eqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::StrongComponent::SINGLEIFEQUATION { vars, eqn } => {
                    let mut backendVars: Arc<metamodelica::List<BackendDAE::Var>>;
                    let mut solvesDiscreteValue: bool;
                    backendVars = List::map1r(vars.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), iOrderedVars.clone())?;
                    solvesDiscreteValue = BackendVariable::hasDiscreteVar(backendVars.clone());
                    Ok((solvesDiscreteValue.clone(), eqn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((false, -1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (oSolvesDiscreteValue, oFirstEqIdx)
}

//------------------------------------------
//Methods to write blt-structure as xml-file
//------------------------------------------
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct GraphDumpOptions {
    pub visualizeCriticalPath: bool,
    pub visualizeTaskStartAndFinishTime: bool,
    pub visualizeTaskCalcTime: bool,
    pub visualizeCommTime: bool,
}

impl metamodelica::gc::MMTrace for GraphDumpOptions {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.visualizeCriticalPath, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.visualizeTaskStartAndFinishTime, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.visualizeTaskCalcTime, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.visualizeCommTime, __mmv)?;
        Ok(())
    }
}
pub type GRAPHDUMPOPTIONS = GraphDumpOptions;


pub fn dumpTaskGraph(mut dae: Arc<BackendDAE::BackendDAE>, mut fileName: ArcStr) -> Result<()> {
    let mut name: ArcStr;
    let mut taskGraph: TaskGraph;
    let mut taskGraphData: TaskGraphMeta;
    let mut schedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (taskGraph, taskGraphData) = createTaskGraph(dae, false)?;
    name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TaskGraph_")); __mm_s.push_str(&*fileName); __mm_s.push_str(&*literal!(".graphml")); ArcStr::from(__mm_s) }).clone();
    schedulerInfo = arrayCreate(metamodelica::arrayLength(taskGraph.clone()), (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
    sccSimEqMapping = arrayCreate(metamodelica::arrayLength(taskGraph.clone()), list![-1]);
    dumpAsGraphMLSccLevel(taskGraph.clone(), taskGraphData, (name).clone(), (literal!("")).clone(), metamodelica::nil(), metamodelica::nil(), sccSimEqMapping.clone(), schedulerInfo.clone(), GraphDumpOptions { visualizeCriticalPath: false, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true })?;
    Ok(())
}

pub(crate) fn dumpAsGraphMLSccLevel(mut iGraph: TaskGraph, mut iGraphData: TaskGraphMeta, mut iFileName: ArcStr, mut iCriticalPathInfo: ArcStr, mut iCriticalPath: Arc<metamodelica::List<(i32, i32)>>, mut iCriticalPathWoC: Arc<metamodelica::List<(i32, i32)>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iGraphDumpOptions: GraphDumpOptions) -> Result<()> {
    let mut graphInfo: GraphML::GraphInfo;
    graphInfo = convertToGraphMLSccLevel(iGraph.clone(), iGraphData, (iCriticalPathInfo).clone(), iCriticalPath, iCriticalPathWoC, iSccSimEqMapping.clone(), iSchedulerInfo.clone(), iGraphDumpOptions)?;
    GraphML::dumpGraph(graphInfo, (iFileName).clone())?;
    Ok(())
}

pub(crate) fn convertToGraphMLSccLevel(mut iGraph: TaskGraph, mut iGraphData: TaskGraphMeta, mut iCriticalPathInfo: ArcStr, mut iCriticalPath: Arc<metamodelica::List<(i32, i32)>>, mut iCriticalPathWoC: Arc<metamodelica::List<(i32, i32)>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iGraphDumpOptions: GraphDumpOptions) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut graphIdx: i32;
    let mut annotationInfo: metamodelica::Array<ArcStr>;
    let mut graphInfo: GraphML::GraphInfo;
    graphInfo = GraphML::createGraphInfo();
    let (__pa0, (_, __pa1)) = GraphML::addGraph((literal!("TaskGraph")).clone(), true, graphInfo)?;
    graphInfo = __pa0.clone();
    graphIdx = __pa1.clone();
    annotationInfo = arrayCreate(metamodelica::arrayLength(iGraph.clone()), (literal!("uncomment in HpcOmTaskGraph and +showAnnotations")).clone());
    oGraphInfo = convertToGraphMLSccLevelSubgraph(iGraph.clone(), iGraphData, (iCriticalPathInfo).clone(), iCriticalPath, iCriticalPathWoC, iSccSimEqMapping.clone(), iSchedulerInfo.clone(), annotationInfo.clone(), graphIdx, iGraphDumpOptions, graphInfo)?;
    Ok(oGraphInfo)
}

pub(crate) fn convertToGraphMLSccLevelSubgraph(mut iGraph: TaskGraph, mut iGraphData: TaskGraphMeta, mut iCriticalPathInfo: ArcStr, mut iCriticalPath: Arc<metamodelica::List<(i32, i32)>>, mut iCriticalPathWoC: Arc<metamodelica::List<(i32, i32)>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iAnnotationInfo: metamodelica::Array<ArcStr>, mut iGraphIdx: i32, mut iGraphDumpOptions: GraphDumpOptions, mut iGraphInfo: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo;
    let mut graphInfo: GraphML::GraphInfo = <GraphML::GraphInfo as ::std::default::Default>::default();
    let mut nameAttIdx: i32 = 0;
    let mut calcTimeAttIdx: i32 = 0;
    let mut opCountAttIdx: i32 = 0;
    let mut yCoordAttIdx: i32 = 0;
    let mut taskIdAttIdx: i32 = 0;
    let mut commCostAttIdx: i32 = 0;
    let mut commVarsAttIdx: i32 = 0;
    let mut commVarsIntAttIdx: i32 = 0;
    let mut commVarsFloatAttIdx: i32 = 0;
    let mut commVarsBoolAttIdx: i32 = 0;
    let mut critPathAttIdx: i32 = 0;
    let mut simCodeEqAttIdx: i32 = 0;
    let mut threadIdAttIdx: i32 = 0;
    let mut taskNumberAttIdx: i32 = 0;
    let mut annotAttIdx: i32 = 0;
    let mut compsIdAttIdx: i32 = 0;
    let mut partOfEventAttIdx: i32 = 0;
    let mut partOfOdeAttIdx: i32 = 0;
    let mut removedCompAttIdx: i32 = 0;
    let mut nodeIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oGraphInfo = (match iGraphInfo.clone() {
        _ => {
            let (__pa0, (_, __pa1)) = GraphML::addAttribute((literal!("")).clone(), (literal!("Name")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, iGraphInfo)?;
            graphInfo = __pa0.clone();
            nameAttIdx = __pa1.clone();
            let (__pa2, (_, __pa3)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("Operations")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_INTEGER, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
            graphInfo = __pa2.clone();
            opCountAttIdx = __pa3.clone();
            let (__pa4, (_, __pa5)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("CalcTime")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_DOUBLE, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
            graphInfo = __pa4.clone();
            calcTimeAttIdx = __pa5.clone();
            let (__pa6, (_, __pa7)) = GraphML::addAttribute((literal!("")).clone(), (literal!("TaskID")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
            graphInfo = __pa6.clone();
            taskIdAttIdx = __pa7.clone();
            let (__pa8, (_, __pa9)) = GraphML::addAttribute((literal!("")).clone(), (literal!("Components")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
            graphInfo = __pa8.clone();
            compsIdAttIdx = __pa9.clone();
            let (__pa10, (_, __pa11)) = GraphML::addAttribute((literal!("17")).clone(), (literal!("yCoord")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_INTEGER, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
            graphInfo = __pa10.clone();
            yCoordAttIdx = __pa11.clone();
            let (__pa12, (_, __pa13)) = GraphML::addAttribute((literal!("")).clone(), (literal!("SimCodeEqs")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
            graphInfo = __pa12.clone();
            simCodeEqAttIdx = __pa13.clone();
            let (__pa14, (_, __pa15)) = GraphML::addAttribute((literal!("")).clone(), (literal!("ThreadId")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
            graphInfo = __pa14.clone();
            threadIdAttIdx = __pa15.clone();
            let (__pa16, (_, __pa17)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("TaskNumber")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_INTEGER, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
            graphInfo = __pa16.clone();
            taskNumberAttIdx = __pa17.clone();
            let (__pa18, (_, __pa19)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("CommCost")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_DOUBLE, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_EDGE, graphInfo)?;
            graphInfo = __pa18.clone();
            commCostAttIdx = __pa19.clone();
            let (__pa20, (_, __pa21)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("CommVars")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_INTEGER, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_EDGE, graphInfo)?;
            graphInfo = __pa20.clone();
            commVarsAttIdx = __pa21.clone();
            let (__pa22, (_, __pa23)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("CommVarsInt")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_INTEGER, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_EDGE, graphInfo)?;
            graphInfo = __pa22.clone();
            commVarsIntAttIdx = __pa23.clone();
            let (__pa24, (_, __pa25)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("CommVarsFloat")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_INTEGER, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_EDGE, graphInfo)?;
            graphInfo = __pa24.clone();
            commVarsFloatAttIdx = __pa25.clone();
            let (__pa26, (_, __pa27)) = GraphML::addAttribute((literal!("-1")).clone(), (literal!("CommVarsBool")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_INTEGER, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_EDGE, graphInfo)?;
            graphInfo = __pa26.clone();
            commVarsBoolAttIdx = __pa27.clone();
            let (__pa28, (_, __pa29)) = GraphML::addAttribute((literal!("annotation")).clone(), (literal!("Annotations")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
            graphInfo = __pa28.clone();
            annotAttIdx = __pa29.clone();
            let (__pa30, (_, __pa31)) = GraphML::addAttribute((literal!("")).clone(), (literal!("CriticalPath")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_STRING, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_GRAPH, graphInfo)?;
            graphInfo = __pa30.clone();
            critPathAttIdx = __pa31.clone();
            let (__pa32, (_, __pa33)) = GraphML::addAttribute((literal!("false")).clone(), (literal!("isPartOfZeroFuncSystem")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_BOOLEAN, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
            graphInfo = __pa32.clone();
            partOfEventAttIdx = __pa33.clone();
            let (__pa34, (_, __pa35)) = GraphML::addAttribute((literal!("false")).clone(), (literal!("IsPartOfOdeSystem")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_BOOLEAN, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
            graphInfo = __pa34.clone();
            partOfOdeAttIdx = __pa35.clone();
            let (__pa36, (_, __pa37)) = GraphML::addAttribute((literal!("false")).clone(), (literal!("IsRemovedComponent")).clone(), openmodelica_codegen_graphml::GraphML::AttributeType::TYPE_BOOLEAN, openmodelica_codegen_graphml::GraphML::AttributeTarget::TARGET_NODE, graphInfo)?;
            graphInfo = __pa36.clone();
            removedCompAttIdx = __pa37.clone();
            graphInfo = GraphML::addGraphAttributeValue((critPathAttIdx, iCriticalPathInfo), iGraphIdx, graphInfo)?;
            nodeIdc = List::intRange(metamodelica::arrayLength(iGraph.clone()));
            (graphInfo, _) = List::fold(nodeIdc, (std::sync::Arc::new({ let __pe_b1 = (iGraph.clone(), iGraphData); let __pe_b2 = (nameAttIdx, opCountAttIdx, calcTimeAttIdx, taskIdAttIdx, compsIdAttIdx, yCoordAttIdx, commCostAttIdx, commVarsAttIdx, commVarsIntAttIdx, commVarsFloatAttIdx, commVarsBoolAttIdx, simCodeEqAttIdx, threadIdAttIdx, taskNumberAttIdx, annotAttIdx, partOfEventAttIdx, partOfOdeAttIdx, removedCompAttIdx); let __pe_b3 = iSccSimEqMapping.clone(); let __pe_b4 = (iCriticalPath, iCriticalPathWoC, iSchedulerInfo.clone(), iAnnotationInfo.clone()); let __pe_b5 = iGraphDumpOptions; move |__pe_a0, __pe_a6| addNodeToGraphML(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_a6) }) as std::sync::Arc<dyn ::std::ops::Fn(i32, (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> + 'static>), (graphInfo, iGraphIdx))?;
            graphInfo
        },
    });
    Ok(oGraphInfo)
}

fn addNodeToGraphML(mut nodeIdx: i32, mut tGraphDataTuple: (metamodelica::Array<Arc<metamodelica::List<i32>>>, TaskGraphMeta), mut attIdc: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32), mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSchedulerInfoCritPath: (Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>, metamodelica::Array<(i32, i32, metamodelica::Real)>, metamodelica::Array<ArcStr>), mut iGraphDumpOptions: GraphDumpOptions, mut iGraph: (GraphML::GraphInfo, i32)) -> Result<(GraphML::GraphInfo, i32)> {
    let mut oGraph: (GraphML::GraphInfo, i32);
    let mut tGraphIn: TaskGraph;
    let mut tGraphDataIn: TaskGraphMeta;
    let mut tmpGraph: GraphML::GraphInfo;
    let mut graphIdx: i32;
    let mut opCount: i32;
    let mut nameAttIdx: i32;
    let mut calcTimeAttIdx: i32;
    let mut opCountAttIdx: i32;
    let mut taskIdAttIdx: i32;
    let mut compsIdAttIdx: i32;
    let mut yCoordAttIdx: i32;
    let mut commCostAttIdx: i32;
    let mut commVarsAttIdx: i32;
    let mut commVarsAttIntIdx: i32;
    let mut commVarsAttFloatIdx: i32;
    let mut commVarsAttBoolIdx: i32;
    let mut yCoord: i32;
    let mut simCodeEqAttIdx: i32;
    let mut threadIdAttIdx: i32;
    let mut taskNumberAttIdx: i32;
    let mut annotationAttIdx: i32;
    let mut partOfEventAttIdx: i32;
    let mut partOfOdeAttIdx: i32;
    let mut removedCompAttIdx: i32;
    let mut calcTime: metamodelica::Real;
    let mut taskFinishTime: metamodelica::Real;
    let mut taskStartTime: metamodelica::Real;
    let mut primalComp: i32;
    let mut childNodes: Arc<metamodelica::List<i32>>;
    let mut components: Arc<metamodelica::List<i32>>;
    let mut simCodeEqs: Arc<metamodelica::List<i32>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut annotationInfo: metamodelica::Array<ArcStr>;
    let mut calcTimeString: ArcStr;
    let mut opCountString: ArcStr;
    let mut yCoordString: ArcStr;
    let mut taskFinishTimeString: ArcStr;
    let mut taskStartTimeString: ArcStr;
    let mut compText: ArcStr;
    let mut compsText: ArcStr;
    let mut nodeDesc: ArcStr;
    let mut componentsString: ArcStr;
    let mut simCodeEqString: ArcStr;
    let mut threadIdxString: ArcStr;
    let mut taskNumberString: ArcStr;
    let mut annotationString: ArcStr;
    let mut schedulerThreadId: i32;
    let mut schedulerTaskNumber: i32;
    let mut nodeLabels: Arc<metamodelica::List<GraphML::NodeLabel>>;
    let mut schedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut criticalPath: Arc<metamodelica::List<(i32, i32)>>;
    let mut criticalPathWoC: Arc<metamodelica::List<(i32, i32)>>;
    let mut visualizeTaskStartAndFinishTime: bool;
    let mut visualizeTaskCalcTime: bool;
    let mut isPartOfODESystem: bool;
    let mut isPartOfZeroFuncSystem: bool;
    let mut isRemovedComponent: bool;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    (tmpGraph, graphIdx) = iGraph;
    if intGt(nodeIdx, 0) {
        (tGraphIn, tGraphDataIn) = tGraphDataTuple;
        let TaskGraphMeta { inComps: __pa0, compNames: __pa1, compDescs: __pa2, exeCosts: __pa3, nodeMark: __pa4, compInformations: __pa5, .. } = (tGraphDataIn.clone()) else { bail!("pattern mismatch") };
        inComps = __pa0.clone();
        compNames = __pa1.clone();
        compDescs = __pa2.clone();
        exeCosts = __pa3.clone();
        nodeMark = __pa4.clone();
        compInformations = __pa5.clone();
        (nameAttIdx, opCountAttIdx, calcTimeAttIdx, taskIdAttIdx, compsIdAttIdx, yCoordAttIdx, commCostAttIdx, commVarsAttIdx, commVarsAttIntIdx, commVarsAttFloatIdx, commVarsAttBoolIdx, simCodeEqAttIdx, threadIdAttIdx, taskNumberAttIdx, annotationAttIdx, partOfEventAttIdx, partOfOdeAttIdx, removedCompAttIdx) = attIdc;
        (criticalPath, criticalPathWoC, schedulerInfo, annotationInfo) = iSchedulerInfoCritPath;
        let GraphDumpOptions { visualizeTaskStartAndFinishTime: __pa6, visualizeTaskCalcTime: __pa7, .. } = (iGraphDumpOptions) else { bail!("pattern mismatch") };
        visualizeTaskStartAndFinishTime = __pa6.clone();
        visualizeTaskCalcTime = __pa7.clone();
        components = metamodelica::arrayGet(inComps.clone(), nodeIdx)?;
        (isPartOfODESystem, isPartOfZeroFuncSystem, isRemovedComponent) = getNodeMembershipByComponents(components.clone(), compInformations.clone())?;
        if intNe((components.clone().len() as i32), 1) {
            primalComp = List::last(components.clone())?;
            simCodeEqs = List::flatten(List::map1(components.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), sccSimEqMapping.clone())?)?;
            nodeDesc = stringDelimitList(List::map1(components.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), compDescs.clone())?, (literal!("\n")).clone());
            (opCount, calcTime) = List::fold1(components.clone(), (std::sync::Arc::new(addNodeToGraphML1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<(i32, metamodelica::Real)>, (i32, metamodelica::Real)) -> Result<(i32, metamodelica::Real)> + 'static>), exeCosts.clone(), (0, metamodelica::OrderedFloat(0.0_f64)))?;
        } else {
            primalComp = (components.clone()).get(1)?;
            simCodeEqs = metamodelica::arrayGet(sccSimEqMapping.clone(), primalComp)?;
            nodeDesc = (metamodelica::arrayGet(compDescs.clone(), primalComp)?).clone();
            (_, calcTime) = metamodelica::arrayGet(exeCosts.clone(), primalComp)?;
            (opCount, calcTime) = metamodelica::arrayGet(exeCosts.clone(), primalComp)?;
        }
        compText = (metamodelica::arrayGet(compNames.clone(), primalComp)?).clone();
        compsText = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(List::map(components, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
        annotationString = (metamodelica::arrayGet(annotationInfo.clone(), nodeIdx)?).clone();
        calcTimeString = (realString(calcTime)).clone();
        yCoord = metamodelica::arrayGet(nodeMark.clone(), nodeIdx)? * 100;
        opCountString = (intString(opCount)).clone();
        yCoordString = (intString(yCoord)).clone();
        childNodes = metamodelica::arrayGet(tGraphIn.clone(), nodeIdx)?;
        simCodeEqString = stringDelimitList(List::map(simCodeEqs, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone());
        componentsString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*intString(nodeIdx)); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
        (schedulerThreadId, schedulerTaskNumber, taskFinishTime) = metamodelica::arrayGet(schedulerInfo.clone(), nodeIdx)?;
        taskStartTime = (taskFinishTime) - (calcTime);
        threadIdxString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Th ")); __mm_s.push_str(&*intString(schedulerThreadId)); ArcStr::from(__mm_s) }).clone();
        taskNumberString = (intString(schedulerTaskNumber)).clone();
        calcTimeString = (System::snprintff((literal!("%.0f")).clone(), 25, calcTime)?).clone();
        taskFinishTimeString = (System::snprintff((literal!("%.0f")).clone(), 25, taskFinishTime)?).clone();
        taskStartTimeString = (System::snprintff((literal!("%.0f")).clone(), 25, taskStartTime)?).clone();
        nodeLabels = list![GraphML::NodeLabel::NODELABEL_INTERNAL { text: (componentsString.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTPLAIN }];
        nodeLabels = if (visualizeTaskCalcTime) {metamodelica::cons(GraphML::NodeLabel::NODELABEL_CORNER { text: (calcTimeString.clone()).clone(), backgroundColor: Some((arcstr::literal!(GraphML::COLOR_YELLOW)).clone()), fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTBOLD, position: (literal!("se")).clone() }, nodeLabels)} else {nodeLabels};
        nodeLabels = if (visualizeTaskStartAndFinishTime) {listAppend(nodeLabels, list![GraphML::NodeLabel::NODELABEL_CORNER { text: (taskStartTimeString).clone(), backgroundColor: Some((arcstr::literal!(GraphML::COLOR_CYAN)).clone()), fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTBOLD, position: (literal!("nw")).clone() }, GraphML::NodeLabel::NODELABEL_CORNER { text: (taskFinishTimeString).clone(), backgroundColor: Some((arcstr::literal!(GraphML::COLOR_PINK)).clone()), fontStyle: openmodelica_codegen_graphml::GraphML::FontStyle::FONTBOLD, position: (literal!("sw")).clone() }])} else {nodeLabels};
        (tmpGraph, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(nodeIdx)); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_ORANGE)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), nodeLabels, openmodelica_codegen_graphml::GraphML::ShapeType::RECTANGLE, Some((nodeDesc).clone()), list![(nameAttIdx, compText), (calcTimeAttIdx, calcTimeString), (opCountAttIdx, opCountString), (taskIdAttIdx, componentsString), (compsIdAttIdx, compsText), (yCoordAttIdx, yCoordString), (simCodeEqAttIdx, simCodeEqString), (threadIdAttIdx, threadIdxString), (taskNumberAttIdx, taskNumberString), (annotationAttIdx, annotationString), (partOfEventAttIdx, boolString(isPartOfODESystem)), (partOfOdeAttIdx, boolString(isPartOfZeroFuncSystem)), (removedCompAttIdx, boolString(isRemovedComponent))], graphIdx, tmpGraph)?;
        tmpGraph = List::fold(childNodes, (std::sync::Arc::new({ let __pe_b1 = nodeIdx; let __pe_b2 = tGraphDataIn; let __pe_b3 = (commCostAttIdx, commVarsAttIdx, commVarsAttIntIdx, commVarsAttFloatIdx, commVarsAttBoolIdx); let __pe_b4 = (criticalPath, criticalPathWoC); let __pe_b5 = iGraphDumpOptions; move |__pe_a0, __pe_a6| addDepToGraph(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_a6) }) as std::sync::Arc<dyn ::std::ops::Fn(i32, GraphML::GraphInfo) -> Result<GraphML::GraphInfo> + 'static>), tmpGraph)?;
    } else {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("function addNodeToGraphML failed.")).clone()])?;
    }
    oGraph = (tmpGraph, graphIdx);
    Ok(oGraph)
}

fn addNodeToGraphML1(mut compIdx: i32, mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut exeCostsIn: (i32, metamodelica::Real)) -> Result<(i32, metamodelica::Real)> {
    let mut exeCostsOut: (i32, metamodelica::Real);
    let mut opCount: i32;
    let mut opCountIn: i32;
    let mut exeTimeIn: metamodelica::Real;
    let mut exeTime: metamodelica::Real;
    (opCountIn, exeTimeIn) = exeCostsIn;
    (opCount, exeTime) = metamodelica::arrayGet(exeCosts.clone(), compIdx)?;
    exeCostsOut = (opCountIn + opCount, (exeTimeIn) + (exeTime));
    Ok(exeCostsOut)
}

fn addDepToGraph(mut childIdx: i32, mut parentIdx: i32, mut tGraphDataIn: TaskGraphMeta, mut iCommAttIdc: (i32, i32, i32, i32, i32), mut iCriticalPathEdges: (Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<(i32, i32)>>), mut iGraphDumpOptions: GraphDumpOptions, mut iGraph: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraph: GraphML::GraphInfo;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut integerVars: Arc<metamodelica::List<i32>>;
    let mut floatVars: Arc<metamodelica::List<i32>>;
    let mut booleanVars: Arc<metamodelica::List<i32>>;
    let mut commCostAttIdx: i32;
    let mut commVarsAttIdx: i32;
    let mut commVarsAttIntIdx: i32;
    let mut commVarsAttFloatIdx: i32;
    let mut commVarsAttBoolIdx: i32;
    let mut numOfCommVars: i32;
    let mut commCost: metamodelica::Real;
    let mut commCostString: ArcStr;
    let mut numOfCommVarsString: ArcStr;
    let mut numOfCommVarsIntString: ArcStr;
    let mut numOfCommVarsFloatString: ArcStr;
    let mut numOfCommVarsBoolString: ArcStr;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut tmpGraph: GraphML::GraphInfo;
    let mut criticalPathEdges: Arc<metamodelica::List<(i32, i32)>>;
    let mut criticalPathEdgesWoC: Arc<metamodelica::List<(i32, i32)>>;
    let mut edgeColor: ArcStr = arcstr::literal!(GraphML::COLOR_BLACK);
    let mut visualizeCriticalPath: bool;
    let mut visualizeCommTime: bool;
    let mut edgeLabels: Arc<metamodelica::List<GraphML::EdgeLabel>>;
    let mut lineWidth: metamodelica::Real;
    let TaskGraphMeta { commCosts: __pa0, nodeMark: __pa1, inComps: __pa2, .. } = (tGraphDataIn.clone()) else { bail!("pattern mismatch") };
    commCosts = __pa0.clone();
    nodeMark = __pa1.clone();
    inComps = __pa2.clone();
    (commCostAttIdx, commVarsAttIdx, commVarsAttIntIdx, commVarsAttFloatIdx, commVarsAttBoolIdx) = iCommAttIdc;
    (criticalPathEdges, criticalPathEdgesWoC) = iCriticalPathEdges;
    let GraphDumpOptions { visualizeCriticalPath: __pa3, visualizeCommTime: __pa4, .. } = (iGraphDumpOptions) else { bail!("pattern mismatch") };
    visualizeCriticalPath = __pa3.clone();
    visualizeCommTime = __pa4.clone();
    if List::exist1(criticalPathEdges, (std::sync::Arc::new(fnptr!(compareIntTuple2, (i32, i32), (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), (i32, i32)) -> Result<bool> + 'static>), (parentIdx, childIdx))? {
        lineWidth = GraphML::LINEWIDTH_BOLD.clone();
        edgeColor = (if (visualizeCriticalPath) {arcstr::literal!(GraphML::COLOR_GRAY)} else {edgeColor}).clone();
    } else {
        lineWidth = GraphML::LINEWIDTH_STANDARD.clone();
    }
    let Communication { numberOfVars: __pa5, integerVars: __pa6, floatVars: __pa7, booleanVars: __pa8, requiredTime: __pa9, .. } = (getCommCostBetweenNodes(parentIdx, childIdx, tGraphDataIn)?) else { bail!("pattern mismatch") };
    numOfCommVars = __pa5.clone();
    integerVars = __pa6.clone();
    floatVars = __pa7.clone();
    booleanVars = __pa8.clone();
    commCost = __pa9.clone();
    numOfCommVarsString = (intString(numOfCommVars)).clone();
    numOfCommVarsIntString = (intString((integerVars.len() as i32))).clone();
    numOfCommVarsFloatString = (intString((floatVars.len() as i32))).clone();
    numOfCommVarsBoolString = (intString((booleanVars.len() as i32))).clone();
    commCostString = (System::snprintff((literal!("%.0f")).clone(), 25, commCost)?).clone();
    edgeLabels = if (visualizeCommTime) {list![GraphML::EdgeLabel { text: (commCostString.clone()).clone(), backgroundColor: Some((edgeColor.clone()).clone()), fontSize: GraphML::FONTSIZE_STANDARD.clone() }]} else {metamodelica::nil()};
    (tmpGraph, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Edge")); __mm_s.push_str(&*intString(parentIdx)); __mm_s.push_str(&*intString(childIdx)); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(childIdx)); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(parentIdx)); ArcStr::from(__mm_s) }).clone(), (edgeColor).clone(), openmodelica_codegen_graphml::GraphML::LineType::LINE, lineWidth, false, edgeLabels, (openmodelica_codegen_graphml::GraphML::ArrowType::ARROWNONE, openmodelica_codegen_graphml::GraphML::ArrowType::ARROWSTANDART), list![(commCostAttIdx, commCostString), (commVarsAttIdx, numOfCommVarsString), (commVarsAttIntIdx, numOfCommVarsIntString), (commVarsAttFloatIdx, numOfCommVarsFloatString), (commVarsAttBoolIdx, numOfCommVarsBoolString)], iGraph)?;
    oGraph = tmpGraph;
    Ok(oGraph)
}

fn getNodeMembershipByComponents(mut iNodeComponents: Arc<metamodelica::List<i32>>, mut iCompInformations: metamodelica::Array<ComponentInfo>) -> Result<(bool, bool, bool)> {
    let mut oMembership: (bool, bool, bool);
    let mut isPartOfODESystem: bool;
    let mut isPartOfZeroFuncSystem: bool;
    let mut isRemovedComponent: bool;
    let mut compIdx: i32 = 0;
    let mut tmpComponentInformation: ComponentInfo;
    tmpComponentInformation = ComponentInfo { isPartOfODESystem: false, isPartOfZeroFuncSystem: false, isRemovedComponent: false };
    for mut compIdx in &*iNodeComponents {
        let mut compIdx = compIdx.clone();
        tmpComponentInformation = combineComponentInformations(metamodelica::arrayGet(iCompInformations.clone(), compIdx)?, tmpComponentInformation)?;
    }
    let ComponentInfo { isPartOfODESystem: __pa0, isPartOfZeroFuncSystem: __pa1, isRemovedComponent: __pa2 } = (tmpComponentInformation) else { bail!("pattern mismatch") };
    isPartOfODESystem = __pa0.clone();
    isPartOfZeroFuncSystem = __pa1.clone();
    isRemovedComponent = __pa2.clone();
    oMembership = (isPartOfODESystem, isPartOfZeroFuncSystem, isRemovedComponent);
    Ok(oMembership)
}

//-----------------
//  Print functions
//-----------------
pub(crate) fn printTaskGraph(mut graphIn: TaskGraph) -> () {
    let mut graphLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    metamodelica::print((literal!("\n")).clone());
    metamodelica::print((literal!("--------------------------------\n")).clone());
    metamodelica::print((literal!("TASKGRAPH\n")).clone());
    metamodelica::print((literal!("--------------------------------\n")).clone());
    graphLst = Arc::new(graphIn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    dumpAdjacencyLst(graphLst, 1);
    metamodelica::print((literal!("\n")).clone());
    ()
}

fn dumpAdjacencyLst(mut inIntegerLstLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut rowIndex: i32) -> () {
    let () = (::match_deref::match_deref! { match &(inIntegerLstLst) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: row, tail: rows } => {
            metamodelica::print((intString(rowIndex)).clone());
            metamodelica::print((literal!(":")).clone());
            dumpAdjacencyRow(row.clone());
            dumpAdjacencyLst(rows.clone(), rowIndex + 1);
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn dumpAdjacencyRow(mut inIntegerLst: Arc<metamodelica::List<i32>>) -> () {
    let () = (::match_deref::match_deref! { match &(inIntegerLst) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::print((literal!("\n")).clone());
            ()
        },
        Deref @ metamodelica::List::Cons { head: x, tail: xs } => {
            let mut s: ArcStr;
            s = (intString(x.clone())).clone();
            metamodelica::print((s).clone());
            metamodelica::print((literal!(" ")).clone());
            dumpAdjacencyRow(xs.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub(crate) fn printTaskGraphMeta(mut metaDataIn: TaskGraphMeta) -> Result<()> {
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let TaskGraphMeta { inComps: __pa0, varCompMapping: __pa1, eqCompMapping: __pa2, compParamMapping: __pa3, compNames: __pa4, compDescs: __pa5, exeCosts: __pa6, commCosts: __pa7, nodeMark: __pa8, compInformations: __pa9 } = (metaDataIn) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    varCompMapping = __pa1.clone();
    eqCompMapping = __pa2.clone();
    compParamMapping = __pa3.clone();
    compNames = __pa4.clone();
    compDescs = __pa5.clone();
    exeCosts = __pa6.clone();
    commCosts = __pa7.clone();
    nodeMark = __pa8.clone();
    compInformations = __pa9.clone();
    metamodelica::print((literal!("\n")).clone());
    metamodelica::print((literal!("--------------------------------\n")).clone());
    metamodelica::print((literal!("TASKGRAPH METADATA\n")).clone());
    metamodelica::print((literal!("--------------------------------\n")).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(metamodelica::arrayLength(inComps.clone()))); __mm_s.push_str(&*literal!(" nodes include components:\n")); ArcStr::from(__mm_s) }).clone());
    printInComps(inComps.clone())?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(metamodelica::arrayLength(varCompMapping.clone()))); __mm_s.push_str(&*literal!(" vars are solved in the nodes \n")); ArcStr::from(__mm_s) }).clone());
    printVarCompMapping(varCompMapping.clone())?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(metamodelica::arrayLength(eqCompMapping.clone()))); __mm_s.push_str(&*literal!(" equations are computed in the nodes \n")); ArcStr::from(__mm_s) }).clone());
    printEqCompMapping(eqCompMapping.clone())?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(metamodelica::arrayLength(compParamMapping.clone()))); __mm_s.push_str(&*literal!(" parameters are part of the components \n")); ArcStr::from(__mm_s) }).clone());
    printCompParamMapping(compParamMapping.clone())?;
    metamodelica::print((literal!("the names of the components \n")).clone());
    printComponentNames(compNames.clone())?;
    metamodelica::print((literal!("the description of the node\n")).clone());
    printCompDescs(compDescs.clone())?;
    metamodelica::print((literal!("the execution costs of the nodes\n")).clone());
    printExeCosts(exeCosts.clone())?;
    metamodelica::print((literal!("the communication costs of the nodes\n")).clone());
    printCommCosts(commCosts.clone())?;
    metamodelica::print((literal!("the nodeMark of the nodes\n")).clone());
    printNodeMarks(nodeMark.clone())?;
    metamodelica::print((literal!("the component informations are\n")).clone());
    printComponentInformations(compInformations.clone())?;
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

fn printInComps(mut iInComps: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut nodeIdx: i32 = 0;
    let mut compRow: Arc<metamodelica::List<i32>>;
    for mut nodeIdx in 1..=metamodelica::arrayLength(iInComps.clone()) {
        compRow = metamodelica::arrayGet(iInComps.clone(), nodeIdx)?;
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("node ")); __mm_s.push_str(&*intString(nodeIdx)); __mm_s.push_str(&*literal!(" solves components: ")); __mm_s.push_str(&*stringDelimitList(List::map(compRow.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    metamodelica::print((literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printVarCompMapping(mut iVarCompMapping: metamodelica::Array<(i32, i32, i32)>) -> Result<()> {
    let mut varIdx: i32 = 0;
    let mut comp: i32;
    let mut eqSysIdx: i32;
    let mut varOffset: i32;
    for mut varIdx in 1..=metamodelica::arrayLength(iVarCompMapping.clone()) {
        (comp, eqSysIdx, varOffset) = metamodelica::arrayGet(iVarCompMapping.clone(), varIdx)?;
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("variable ")); __mm_s.push_str(&*intString(varIdx - varOffset)); __mm_s.push_str(&*literal!(" (offset: ")); __mm_s.push_str(&*intString(varOffset)); __mm_s.push_str(&*literal!(") of equation system ")); __mm_s.push_str(&*intString(eqSysIdx)); __mm_s.push_str(&*literal!(" is solved in component: ")); __mm_s.push_str(&*intString(comp)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    metamodelica::print((literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printEqCompMapping(mut iEqCompMapping: metamodelica::Array<(i32, i32, i32)>) -> Result<()> {
    let mut eqIdx: i32 = 0;
    let mut comp: i32;
    let mut eqSysIdx: i32;
    let mut eqOffset: i32;
    for mut eqIdx in 1..=metamodelica::arrayLength(iEqCompMapping.clone()) {
        (comp, eqSysIdx, eqOffset) = metamodelica::arrayGet(iEqCompMapping.clone(), eqIdx)?;
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("equation ")); __mm_s.push_str(&*intString(eqIdx)); __mm_s.push_str(&*literal!(" (offset: ")); __mm_s.push_str(&*intString(eqOffset)); __mm_s.push_str(&*literal!(") of equation system ")); __mm_s.push_str(&*intString(eqSysIdx)); __mm_s.push_str(&*literal!(" is computed in component: ")); __mm_s.push_str(&*intString(comp)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    metamodelica::print((literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printCompParamMapping(mut iCompParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut params: Arc<metamodelica::List<i32>>;
    for mut compIdx in 1..=metamodelica::arrayLength(iCompParamMapping.clone()) {
        params = metamodelica::arrayGet(iCompParamMapping.clone(), compIdx)?;
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*intString(compIdx)); __mm_s.push_str(&*literal!(" needs the parameters: ")); __mm_s.push_str(&*stringDelimitList(List::map(params.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    metamodelica::print((literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printComponentNames(mut iCompNames: metamodelica::Array<ArcStr>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut compName: ArcStr;
    for mut compIdx in 1..=metamodelica::arrayLength(iCompNames.clone()) {
        compName = (metamodelica::arrayGet(iCompNames.clone(), compIdx)?).clone();
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*intString(compIdx)); __mm_s.push_str(&*literal!(" is named ")); __mm_s.push_str(&*compName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    metamodelica::print((literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printCompDescs(mut iCompDescs: metamodelica::Array<ArcStr>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut compDesc: ArcStr;
    for mut compIdx in 1..=metamodelica::arrayLength(iCompDescs.clone()) {
        compDesc = (metamodelica::arrayGet(iCompDescs.clone(), compIdx)?).clone();
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*intString(compIdx)); __mm_s.push_str(&*literal!(" is described with: ")); __mm_s.push_str(&*compDesc.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    metamodelica::print((literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printExeCosts(mut iExeCosts: metamodelica::Array<(i32, metamodelica::Real)>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut opCount: i32;
    let mut execTime: metamodelica::Real;
    for mut compIdx in 1..=metamodelica::arrayLength(iExeCosts.clone()) {
        (opCount, execTime) = metamodelica::arrayGet(iExeCosts.clone(), compIdx)?;
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*intString(compIdx)); __mm_s.push_str(&*literal!(" has execution cost of: (")); __mm_s.push_str(&*intString(opCount)); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*realString(execTime)); __mm_s.push_str(&*literal!(")\n")); ArcStr::from(__mm_s) }).clone());
    }
    metamodelica::print((literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printCommCosts(mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<()> {
    let mut nodeIdx: i32 = 0;
    let mut nodeComms: Communications;
    for mut nodeIdx in 1..=metamodelica::arrayLength(iCommCosts.clone()) {
        nodeComms = metamodelica::arrayGet(iCommCosts.clone(), nodeIdx)?;
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("edges from node ")); __mm_s.push_str(&*intString(nodeIdx)); __mm_s.push_str(&*literal!(": with the communication costs ")); __mm_s.push_str(&*stringDelimitList(List::map(nodeComms.clone(), (std::sync::Arc::new(printCommCost) as std::sync::Arc<dyn ::std::ops::Fn(Communication) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    metamodelica::print((literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printCommCost(mut iComm: Communication) -> Result<ArcStr> {
    let mut oCommString: ArcStr;
    let mut numberOfVars: i32;
    let mut numberOfIntegers: i32;
    let mut numberOfFloats: i32;
    let mut numberOfBooleans: i32;
    let mut childNode: i32;
    let mut integerVars: Arc<metamodelica::List<i32>>;
    let mut floatVars: Arc<metamodelica::List<i32>>;
    let mut booleanVars: Arc<metamodelica::List<i32>>;
    let mut requiredTime: metamodelica::Real;
    let Communication { numberOfVars: __pa0, integerVars: __pa1, floatVars: __pa2, booleanVars: __pa3, childNode: __pa4, requiredTime: __pa5, .. } = (iComm) else { bail!("pattern mismatch") };
    numberOfVars = __pa0.clone();
    integerVars = __pa1.clone();
    floatVars = __pa2.clone();
    booleanVars = __pa3.clone();
    childNode = __pa4.clone();
    requiredTime = __pa5.clone();
    numberOfIntegers = (integerVars.len() as i32);
    numberOfFloats = (floatVars.len() as i32);
    numberOfBooleans = (booleanVars.len() as i32);
    oCommString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(target node: ")); __mm_s.push_str(&*intString(childNode)); __mm_s.push_str(&*literal!(" ints: ")); __mm_s.push_str(&*intString(numberOfIntegers)); __mm_s.push_str(&*literal!(" floats: ")); __mm_s.push_str(&*intString(numberOfFloats)); __mm_s.push_str(&*literal!(" booleans: ")); __mm_s.push_str(&*intString(numberOfBooleans)); __mm_s.push_str(&*literal!(" [requiredTime: ")); __mm_s.push_str(&*realString(requiredTime)); __mm_s.push_str(&*literal!(" for ")); __mm_s.push_str(&*intString(numberOfVars)); __mm_s.push_str(&*literal!(" variables)")); ArcStr::from(__mm_s) }).clone();
    Ok(oCommString)
}

fn printNodeMarks(mut iNodeMarks: metamodelica::Array<i32>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut mark: i32;
    for mut compIdx in 1..=metamodelica::arrayLength(iNodeMarks.clone()) {
        mark = metamodelica::arrayGet(iNodeMarks.clone(), compIdx)?;
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*intString(compIdx)); __mm_s.push_str(&*literal!(" has the nodeMark : ")); __mm_s.push_str(&*intString(mark)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    metamodelica::print((literal!("--------------------------------\n")).clone());
    Ok(())
}

fn printComponentInformations(mut iComponentInformations: metamodelica::Array<ComponentInfo>) -> Result<()> {
    let mut compIdx: i32 = 0;
    let mut isPartOfODESystem: bool;
    let mut isPartOfZeroFuncSystem: bool;
    let mut isRemovedComponent: bool;
    for mut compIdx in 1..=metamodelica::arrayLength(iComponentInformations.clone()) {
        let ComponentInfo { isPartOfODESystem: __pa0, isPartOfZeroFuncSystem: __pa1, isRemovedComponent: __pa2 } = (metamodelica::arrayGet(iComponentInformations.clone(), compIdx)?) else { bail!("pattern mismatch") };
        isPartOfODESystem = __pa0.clone();
        isPartOfZeroFuncSystem = __pa1.clone();
        isRemovedComponent = __pa2.clone();
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("component ")); __mm_s.push_str(&*intString(compIdx)); __mm_s.push_str(&*literal!(" has component information:\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("   Is part of ODE-System:   ")); __mm_s.push_str(&*boolString(isPartOfODESystem)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("   Is part of Event-System: ")); __mm_s.push_str(&*boolString(isPartOfZeroFuncSystem)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("   Is removed component:    ")); __mm_s.push_str(&*boolString(isRemovedComponent)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    metamodelica::print((literal!("--------------------------------\n")).clone());
    Ok(())
}

pub(crate) fn intLstString(mut lstIn: Arc<metamodelica::List<i32>>) -> Result<ArcStr> {
    let mut strOut: ArcStr;
    let mut r#str: ArcStr;
    r#str = stringDelimitList(List::map(lstIn.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone());
    strOut = (if (lstIn.is_empty()) {literal!("---")} else {r#str}).clone();
    Ok(strOut)
}

pub(crate) fn dumpCriticalPathInfo(mut iCriticalPaths: (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Real), mut iCriticalPathsWoC: (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Real)) -> Result<ArcStr> {
    let mut oString: ArcStr;
    let mut tmpString: ArcStr = arcstr::literal!("");
    let mut critPath: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut critPathWoC: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut costPath: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut costPathWoC: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    oString = ((::match_deref::match_deref! { match &((iCriticalPaths, iCriticalPathsWoC)) {
        ((Deref @ metamodelica::List::Nil, _), _) => literal!(""),
        ((__esc_critPath, __esc_costPath), (__esc_critPathWoC, __esc_costPathWoC)) => {
            critPath = (*__esc_critPath).clone();
            costPath = (*__esc_costPath).clone();
            critPathWoC = (*__esc_critPathWoC).clone();
            costPathWoC = (*__esc_costPathWoC).clone();
            tmpString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("critical path with costs of ")); __mm_s.push_str(&*realString(costPath.clone())); __mm_s.push_str(&*literal!(" cycles -- ")); ArcStr::from(__mm_s) }).clone();
            tmpString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpString); __mm_s.push_str(&*dumpCriticalPathInfo1(critPath.clone(), 1)?); ArcStr::from(__mm_s) }).clone();
            tmpString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ;; ")); __mm_s.push_str(&*tmpString); __mm_s.push_str(&*literal!("critical path' with costs of ")); __mm_s.push_str(&*realString(costPathWoC.clone())); __mm_s.push_str(&*literal!(" cycles -- ")); ArcStr::from(__mm_s) }).clone();
            tmpString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpString); __mm_s.push_str(&*dumpCriticalPathInfo1(critPathWoC.clone(), 1)?); ArcStr::from(__mm_s) }).clone();
            tmpString
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(oString)
}

fn dumpCriticalPathInfo1(mut criticalPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut cpIdx: i32) -> Result<ArcStr> {
    let mut oString: ArcStr;
    oString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intLstString((criticalPathsIn).get(cpIdx)?)?); __mm_s.push_str(&*literal!("")); ArcStr::from(__mm_s) }).clone();
    Ok(oString)
}

fn printCriticalPathInfo(mut criticalPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut cpCosts: metamodelica::Real) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(criticalPathsIn.clone()) {
        Deref @ metamodelica::List::Nil => (),
        _ => {
            metamodelica::print((literal!("--------------------------------\n")).clone());
            metamodelica::print((literal!(" CRITICAL PATH INFO\n")).clone());
            metamodelica::print((literal!("--------------------------------\n")).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("found ")); __mm_s.push_str(&*intString((criticalPathsIn.clone().len() as i32))); __mm_s.push_str(&*literal!(" critical paths with costs of ")); __mm_s.push_str(&*realString(cpCosts)); __mm_s.push_str(&*literal!(" sec\n")); ArcStr::from(__mm_s) }).clone());
            printCriticalPathInfo1(criticalPathsIn, 1)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printCriticalPathInfo1(mut criticalPathsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut cpIdx: i32) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(cpIdx)); __mm_s.push_str(&*literal!(". path: ")); __mm_s.push_str(&*intLstString((criticalPathsIn).get(cpIdx)?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

//--------------------------
//  Functions to merge nodes
//--------------------------
fn mergeSingleNodes(mut iTaskGraph: TaskGraph, mut iTaskGraphMeta: TaskGraphMeta, mut doNotMergeIn: Arc<metamodelica::List<i32>>) -> (TaskGraph, TaskGraphMeta, bool) {
    let mut oTaskGraph: TaskGraph;
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut changed: bool = false;
    (oTaskGraph, oTaskGraphMeta, changed) = 'mc: {
        let __mc_input = doNotMergeIn.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut numProc: i32;
                    let mut singleNodes: Arc<metamodelica::List<i32>>;
                    let mut singleNodes1: Arc<metamodelica::List<i32>>;
                    let mut pos: Arc<metamodelica::List<i32>>;
                    let mut exeCosts: Arc<metamodelica::List<metamodelica::Real>>;
                    let mut taskGraphT: TaskGraph;
                    let mut changed: bool = changed.clone();
                    numProc = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
                    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
                    (_, singleNodes) = List::filterOnTrueSync(Arc::new(iTaskGraph.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), std::sync::Arc::new(fnptr!(listEmpty, _)), List::intRange(metamodelica::arrayLength(iTaskGraph.clone())))?;
                    (_, singleNodes1) = List::filterOnTrueSync(Arc::new(taskGraphT.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), std::sync::Arc::new(fnptr!(listEmpty, _)), List::intRange(metamodelica::arrayLength(taskGraphT.clone())))?;
                    (singleNodes, _, _) = List::intersection1OnTrue(singleNodes.clone(), singleNodes1.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    (_, singleNodes, _) = List::intersection1OnTrue(singleNodes.clone(), doNotMergeIn.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    exeCosts = List::map1(singleNodes.clone(), (std::sync::Arc::new(getExeCostReqCycles) as std::sync::Arc<dyn ::std::ops::Fn(i32, TaskGraphMeta) -> Result<metamodelica::Real> + 'static>), iTaskGraphMeta.clone())?;
                    (exeCosts, pos) = HpcOmScheduler::quicksortWithOrder(exeCosts.clone())?;
                    singleNodes = List::map1(pos.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), singleNodes.clone())?;
                    singleNodes = singleNodes.clone().reverse();
                    exeCosts = exeCosts.clone().reverse();
                    distributeToClusters(singleNodes.clone(), exeCosts.clone(), numProc.clone())?;
                    changed = intGt((singleNodes.clone().len() as i32), numProc.clone());
                    Ok(((iTaskGraph.clone(), iTaskGraphMeta.clone(), changed), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { changed = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((iTaskGraph.clone(), iTaskGraphMeta.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (oTaskGraph, oTaskGraphMeta, changed)
}

pub(crate) fn distributeToClusters(mut items: Arc<metamodelica::List<i32>>, mut values: Arc<metamodelica::List<metamodelica::Real>>, mut numClusters: i32) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<metamodelica::Real>)> {
    let mut clustersOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut clusterValuesOut: metamodelica::Array<metamodelica::Real>;
    let mut b: bool;
    let mut itemArr: metamodelica::Array<i32>;
    let mut itemsCopy: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut clusters: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut clusterValues: metamodelica::Array<metamodelica::Real>;
    b = intGt((items.clone().len() as i32), numClusters);
    clusters = metamodelica::arrayFromVec(List::map(List::intRange((items.clone().len() as i32)), std::sync::Arc::new(fnptr!(List::create, _)))?.into_iter().cloned().collect());
    clusterValues = metamodelica::arrayFromVec(values.clone().into_iter().cloned().collect());
    itemArr = metamodelica::arrayFromVec(items.clone().into_iter().cloned().collect());
    itemsCopy = Array::map(itemArr.clone(), std::sync::Arc::new(fnptr!(List::create, _)))?;
    clusters = if (true) {Array::copy(itemsCopy.clone(), clusters.clone())?} else {clusters.clone()};
    clusterValues = if (!(b)) {Array::copy(metamodelica::arrayFromVec(values.clone().into_iter().cloned().collect()), clusterValues.clone())?} else {clusterValues.clone()};
    if b {
        (clustersOut, clusterValuesOut) = distributeToClusters1((items, values), (clusters.clone(), clusterValues.clone()), numClusters)?;
    } else {
        (clustersOut, clusterValuesOut) = (clusters.clone(), clusterValues.clone());
    }
    Ok((clustersOut, clusterValuesOut))
}

fn distributeToClusters1(mut tplIn: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<metamodelica::Real>>), mut tplFold: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<metamodelica::Real>), mut numClusters: i32) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<metamodelica::Real>)> {
    let mut clustersOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut clusterValuesOut: metamodelica::Array<metamodelica::Real>;
    (clustersOut, clusterValuesOut) = 'mc: {
        let __mc_input = (tplIn, tplFold);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((itemsIn, _), (clusters, clusterValues)) => {
                    let mut idcsLst1: Arc<metamodelica::List<i32>>;
                    let mut clustersFinal: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut clusterValuesFinal: metamodelica::Array<metamodelica::Real>;
                    let true = ((itemsIn.clone().len() as i32) <= numClusters) else { bail!("pattern mismatch") };
                    idcsLst1 = List::intRange(numClusters);
                    clustersFinal = Array::select(clusters.clone(), idcsLst1.clone())?;
                    clusterValuesFinal = Array::select(clusterValues.clone(), idcsLst1.clone())?;
                    Ok((clustersFinal.clone(), clusterValuesFinal.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((itemsIn, valuesIn), (clusters, clusterValues)) => {
                    let mut diff: i32;
                    let mut lst1: Arc<metamodelica::List<i32>>;
                    let mut idcsLst2: Arc<metamodelica::List<i32>>;
                    let mut idcsLst1: Arc<metamodelica::List<i32>>;
                    let mut entries: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut entries2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut values: Arc<metamodelica::List<metamodelica::Real>>;
                    let mut addValues: Arc<metamodelica::List<metamodelica::Real>>;
                    let mut clusters = (*clusters).clone();
                    let mut clusterValues = (*clusterValues).clone();
                    let true = ((itemsIn.clone().len() as i32) > numClusters) else { bail!("pattern mismatch") };
                    let true = (metamodelica::OrderedFloat(((itemsIn.clone().len() as i32)) as f64) / metamodelica::OrderedFloat((2) as f64) < metamodelica::OrderedFloat((numClusters) as f64)) else { bail!("pattern mismatch") };
                    (lst1, _) = List::split(itemsIn.clone(), numClusters)?;
                    diff = (itemsIn.clone().len() as i32) - numClusters;
                    idcsLst1 = List::intRange2(numClusters - diff.clone() + 1, numClusters);
                    idcsLst2 = List::intRange2(numClusters + 1, (itemsIn.clone().len() as i32));
                    entries = List::map1(idcsLst2.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), clusters.clone())?;
                    entries = entries.clone().reverse();
                    entries2 = List::map1(idcsLst1.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), clusters.clone())?;
                    entries = List::threadMap(entries.clone(), entries2.clone(), Arc::new(fnptr!(listAppend, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)))?;
                    List::threadMap1_0(idcsLst1.clone(), entries.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), clusters.clone())?;
                    values = List::map1(idcsLst1.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), clusterValues.clone())?;
                    addValues = List::map1(idcsLst2.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), clusterValues.clone())?;
                    values = List::threadMap(values.clone(), addValues.clone(), (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>))?;
                    List::threadMap1_0(idcsLst1.clone(), values.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), clusterValues.clone())?;
                    (clusters, clusterValues) = distributeToClusters1((lst1.clone(), valuesIn.clone()), (clusters.clone(), clusterValues.clone()), numClusters)?;
                    Ok((clusters.clone(), clusterValues.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((itemsIn, valuesIn), (clusters, clusterValues)) => {
                    let mut numCl: i32;
                    let mut lst1: Arc<metamodelica::List<i32>>;
                    let mut idcsLst1_2: Arc<metamodelica::List<i32>>;
                    let mut idcsLst2: Arc<metamodelica::List<i32>>;
                    let mut entries: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut entries2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut values: Arc<metamodelica::List<metamodelica::Real>>;
                    let mut addValues: Arc<metamodelica::List<metamodelica::Real>>;
                    let mut clusters = (*clusters).clone();
                    let mut clusterValues = (*clusterValues).clone();
                    let true = ((itemsIn.clone().len() as i32) > numClusters) else { bail!("pattern mismatch") };
                    let true = (metamodelica::OrderedFloat(((itemsIn.clone().len() as i32)) as f64) / metamodelica::OrderedFloat((2) as f64) >= metamodelica::OrderedFloat((numClusters) as f64)) else { bail!("pattern mismatch") };
                    numCl = nextGreaterPowerOf2(intReal((itemsIn.clone().len() as i32)))?;
                    (lst1, _) = List::split(itemsIn.clone(), intDiv(numCl.clone(), 2))?;
                    idcsLst2 = List::intRange2(intDiv(numCl.clone(), 2) + 1, (itemsIn.clone().len() as i32));
                    idcsLst1_2 = List::intRange2(intDiv(numCl.clone(), 2) - (idcsLst2.clone().len() as i32) + 1, intDiv(numCl.clone(), 2));
                    entries = List::map1(idcsLst2.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), clusters.clone())?;
                    entries = entries.clone().reverse();
                    entries2 = List::map1(idcsLst1_2.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), clusters.clone())?;
                    entries = List::threadMap(entries.clone(), entries2.clone(), Arc::new(fnptr!(listAppend, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)))?;
                    List::threadMap1_0(idcsLst1_2.clone(), entries.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), clusters.clone())?;
                    values = List::map1(idcsLst1_2.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), clusterValues.clone())?;
                    addValues = List::map1(idcsLst2.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), clusterValues.clone())?;
                    values = List::threadMap(values.clone(), addValues.clone(), (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>))?;
                    List::threadMap1_0(idcsLst1_2.clone(), values.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), clusterValues.clone())?;
                    (clusters, clusterValues) = distributeToClusters1((lst1.clone(), valuesIn.clone()), (clusters.clone(), clusterValues.clone()), numClusters)?;
                    Ok((clusters.clone(), clusterValues.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("distributeToClusters failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((clustersOut, clusterValuesOut))
}

fn nextGreaterPowerOf2(mut n: metamodelica::Real) -> Result<i32> {
    let mut powOf2: i32;
    powOf2 = nextGreaterPowerOf2_impl(n, 1)?;
    Ok(powOf2)
}

fn nextGreaterPowerOf2_impl(mut n: metamodelica::Real, mut pow: i32) -> Result<i32> {
    let mut powOf2: i32;
    powOf2 = 'mc: {
        let __mc_input = pow;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (n <= realPow(metamodelica::OrderedFloat(2.0_f64), intReal(pow))) else { bail!("pattern mismatch") };
            Ok(((realPow(metamodelica::OrderedFloat(2.0_f64), intReal(pow))).0.floor() as i32))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut n2: i32;
            let true = (n > realPow(metamodelica::OrderedFloat(2.0_f64), intReal(pow))) else { bail!("pattern mismatch") };
            n2 = nextGreaterPowerOf2_impl(n, pow + 1)?;
            Ok(n2.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(powOf2)
}

pub(crate) fn mergeSimpleNodes(mut graphIn: TaskGraph, mut graphTIn: TaskGraph, mut graphDataIn: TaskGraphMeta, mut contractedTasksIn: metamodelica::Array<i32>) -> Result<(TaskGraph, TaskGraph, TaskGraphMeta, metamodelica::Array<i32>, bool)> {
    let mut graphOut: TaskGraph;
    let mut graphTOut: TaskGraph;
    let mut graphDataOut: TaskGraphMeta;
    let mut contractedTasksOut: metamodelica::Array<i32>;
    let mut changed: bool;
    let mut allNodes: Arc<metamodelica::List<i32>>;
    let mut oneChildren: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    allNodes = List::intRange(metamodelica::arrayLength(graphIn.clone()));
    oneChildren = findOneChildParents(allNodes, graphIn.clone(), metamodelica::nil(), list![metamodelica::nil()], 0, contractedTasksIn.clone())?;
    oneChildren = listDelete(oneChildren.clone(), (oneChildren.len() as i32))?;
    oneChildren = List::removeOnTrue(1, (std::sync::Arc::new(fnptr!(compareListLengthOnTrue, i32, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>), oneChildren)?;
    (graphOut, graphTOut, graphDataOut, contractedTasksOut) = contractNodesInGraph(oneChildren.clone(), graphIn.clone(), graphTIn.clone(), graphDataIn, contractedTasksIn.clone())?;
    changed = !(oneChildren.is_empty());
    Ok((graphOut, graphTOut, graphDataOut, contractedTasksOut, changed))
}

pub(crate) fn mergeParentNodes(mut graphIn: TaskGraph, mut graphTIn: TaskGraph, mut graphDataIn: TaskGraphMeta, mut contractedTasksIn: metamodelica::Array<i32>) -> Result<(TaskGraph, TaskGraph, TaskGraphMeta, metamodelica::Array<i32>, bool)> {
    let mut graphOut: TaskGraph;
    let mut graphTOut: TaskGraph;
    let mut graphDataOut: TaskGraphMeta;
    let mut contractedTasksOut: metamodelica::Array<i32>;
    let mut changed: bool;
    let mut alreadyMerged: metamodelica::Array<i32>;
    let mut mergedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    alreadyMerged = arrayCreate(metamodelica::arrayLength(graphIn.clone()), 0);
    mergedNodes = mergeParentNodes0(graphIn.clone(), graphTIn.clone(), graphDataIn.clone(), contractedTasksIn.clone(), alreadyMerged.clone(), 1, metamodelica::nil());
    (graphOut, graphTOut, graphDataOut, contractedTasksOut) = contractNodesInGraph(mergedNodes.clone(), graphIn.clone(), graphTIn.clone(), graphDataIn, contractedTasksIn.clone())?;
    changed = !(mergedNodes.is_empty());
    Ok((graphOut, graphTOut, graphDataOut, contractedTasksOut, changed))
}

fn mergeParentNodes0(mut iGraph: TaskGraph, mut iGraphT: TaskGraph, mut iGraphData: TaskGraphMeta, mut contractedTasksIn: metamodelica::Array<i32>, mut alreadyMerged: metamodelica::Array<i32>, mut iNodeIdx: i32, mut iMergedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut oMergedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut highestParentExeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut sumParentExeCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut parentNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut mergeNodeList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut highestCommCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut parentExeCosts: Arc<metamodelica::List<(i32, metamodelica::Real)>> = metamodelica::nil();
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut parentCommCosts: Communications = metamodelica::nil();
    let mut parentChilds: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut tmpMergedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    oMergedNodes = 'mc: {
        let __mc_input = iGraphData.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8)) = (|| -> Result<_> {
            let TaskGraphMeta { exeCosts: mut exeCosts, commCosts: mut commCosts, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut highestCommCost: metamodelica::Real = highestCommCost.clone();
            let mut highestParentExeCost: metamodelica::Real = highestParentExeCost.clone();
            let mut mergeNodeList: Arc<metamodelica::List<i32>> = mergeNodeList.clone();
            let mut parentChilds: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = parentChilds.clone();
            let mut parentCommCosts: Arc<metamodelica::List<Communication>> = parentCommCosts.clone();
            let mut parentExeCosts: Arc<metamodelica::List<(i32, metamodelica::Real)>> = parentExeCosts.clone();
            let mut parentNodes: Arc<metamodelica::List<i32>> = parentNodes.clone();
            let mut sumParentExeCosts: metamodelica::Real = sumParentExeCosts.clone();
            let mut tmpMergedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = tmpMergedNodes.clone();
            let true = (intLe(iNodeIdx, metamodelica::arrayLength(iGraphT.clone()))) else { bail!("pattern mismatch") };
            let true = (intNe(metamodelica::arrayGet(contractedTasksIn.clone(), iNodeIdx)?, -1)) else { bail!("pattern mismatch") };
            let true = (intNe(metamodelica::arrayGet(alreadyMerged.clone(), iNodeIdx)?, -1)) else { bail!("pattern mismatch") };
            parentNodes = metamodelica::arrayGet(iGraphT.clone(), iNodeIdx)?;
            parentNodes = filterContractedNodes(parentNodes.clone(), contractedTasksIn.clone())?;
            let false = (List::exist1(parentNodes.clone(), (std::sync::Arc::new(isNodeContracted) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), alreadyMerged.clone())?) else { bail!("pattern mismatch") };
            parentCommCosts = List::map2(parentNodes.clone(), (std::sync::Arc::new(getCommCostBetweenNodes) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, TaskGraphMeta) -> Result<Communication> + 'static>), iNodeIdx, iGraphData.clone())?;
            let Communication { requiredTime: __pa0, .. } = (getHighestCommCost(parentCommCosts.clone(), Communication { numberOfVars: 0, integerVars: metamodelica::nil(), floatVars: metamodelica::nil(), booleanVars: metamodelica::nil(), stringVars: metamodelica::nil(), childNode: -1, requiredTime: metamodelica::OrderedFloat(-1.0_f64) })) else { bail!("pattern mismatch") };
            highestCommCost = __pa0.clone();
            parentExeCosts = List::map1(parentNodes.clone(), (std::sync::Arc::new(getExeCost) as std::sync::Arc<dyn ::std::ops::Fn(i32, TaskGraphMeta) -> Result<(i32, metamodelica::Real)> + 'static>), iGraphData.clone())?;
            (_, sumParentExeCosts) = List::fold(parentExeCosts.clone(), (std::sync::Arc::new(fnptr!(addUpExeCosts, (i32, metamodelica::Real), (i32, metamodelica::Real))) as std::sync::Arc<dyn ::std::ops::Fn((i32, metamodelica::Real), (i32, metamodelica::Real)) -> Result<(i32, metamodelica::Real)> + 'static>), (0, metamodelica::OrderedFloat(0.0_f64)))?;
            (_, highestParentExeCost) = getHighestExecCost(parentExeCosts.clone(), (0, metamodelica::OrderedFloat(0.0_f64)));
            let true = (realGt((highestCommCost) + (highestParentExeCost), sumParentExeCosts)) else { bail!("pattern mismatch") };
            parentChilds = List::map1(parentNodes.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), iGraph.clone())?;
            let true = (List::removeOnTrue(1, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), List::map(parentChilds.clone(), std::sync::Arc::new(fnptr!(listLength, _)))?)?.is_empty()) else { bail!("pattern mismatch") };
            mergeNodeList = metamodelica::cons(iNodeIdx, parentNodes.clone());
            tmpMergedNodes = metamodelica::cons(mergeNodeList.clone(), iMergedNodes.clone());
            List::map_0(mergeNodeList.clone(), (std::sync::Arc::new({ let __pe_b1 = -1; let __pe_b2 = alreadyMerged.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>))?;
            tmpMergedNodes = mergeParentNodes0(iGraph.clone(), iGraphT.clone(), iGraphData.clone(), contractedTasksIn.clone(), alreadyMerged.clone(), iNodeIdx + 1, tmpMergedNodes.clone());
            Ok((tmpMergedNodes.clone(), highestCommCost.clone(), highestParentExeCost.clone(), mergeNodeList.clone(), parentChilds.clone(), parentCommCosts.clone(), parentExeCosts.clone(), parentNodes.clone(), sumParentExeCosts.clone(), tmpMergedNodes.clone()))
        })() { highestCommCost = __wb0; highestParentExeCost = __wb1; mergeNodeList = __wb2; parentChilds = __wb3; parentCommCosts = __wb4; parentExeCosts = __wb5; parentNodes = __wb6; sumParentExeCosts = __wb7; tmpMergedNodes = __wb8; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut tmpMergedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = tmpMergedNodes.clone();
            let true = (intLe(iNodeIdx, metamodelica::arrayLength(iGraphT.clone()))) else { bail!("pattern mismatch") };
            tmpMergedNodes = mergeParentNodes0(iGraph.clone(), iGraphT.clone(), iGraphData.clone(), contractedTasksIn.clone(), alreadyMerged.clone(), iNodeIdx + 1, iMergedNodes.clone());
            Ok((tmpMergedNodes.clone(), tmpMergedNodes.clone()))
        })() { tmpMergedNodes = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iMergedNodes.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oMergedNodes
}

fn mergeSinkNodes(mut graphIn: TaskGraph, mut graphTIn: TaskGraph, mut graphDataIn: TaskGraphMeta, mut contractedTasksIn: metamodelica::Array<i32>) -> Result<(TaskGraph, TaskGraph, TaskGraphMeta, metamodelica::Array<i32>, bool)> {
    let mut graphOut: TaskGraph;
    let mut graphTOut: TaskGraph;
    let mut graphDataOut: TaskGraphMeta;
    let mut contractedTasksOut: metamodelica::Array<i32>;
    let mut changed: bool;
    let mut alreadyMerged: metamodelica::Array<i32>;
    let mut mergedNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    alreadyMerged = arrayCreate(metamodelica::arrayLength(graphIn.clone()), 0);
    mergedNodes = mergeParentNodes0(graphIn.clone(), graphTIn.clone(), graphDataIn.clone(), contractedTasksIn.clone(), alreadyMerged.clone(), 1, metamodelica::nil());
    (graphOut, graphTOut, graphDataOut, contractedTasksOut) = contractNodesInGraph(mergedNodes.clone(), graphIn.clone(), graphTIn.clone(), graphDataIn, contractedTasksIn.clone())?;
    changed = !(mergedNodes.is_empty());
    Ok((graphOut, graphTOut, graphDataOut, contractedTasksOut, changed))
}

pub(crate) fn markSystemComponents(mut iTaskGraph: TaskGraph, mut iTaskGraphMeta: TaskGraphMeta, mut iComponentMarks: (bool, bool, bool), mut iTargetTaskGraphMeta: TaskGraphMeta) -> Result<TaskGraphMeta> {
    let mut oTargetTaskGraphMeta: TaskGraphMeta;
    let mut odeInComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeComps: Arc<metamodelica::List<i32>>;
    let mut nodeIdx: i32 = 0;
    let mut compIdx: i32 = 0;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let mut componentInformation: ComponentInfo;
    let mut iComponentInformation: ComponentInfo;
    iComponentInformation = ComponentInfo { isPartOfODESystem: Util::tuple31(iComponentMarks), isPartOfZeroFuncSystem: Util::tuple32(iComponentMarks), isRemovedComponent: Util::tuple33(iComponentMarks) };
    let TaskGraphMeta { inComps: __pa0, .. } = (iTaskGraphMeta) else { bail!("pattern mismatch") };
    odeInComps = __pa0.clone();
    let TaskGraphMeta { inComps: __pa1, varCompMapping: __pa2, eqCompMapping: __pa3, compParamMapping: __pa4, compNames: __pa5, compDescs: __pa6, exeCosts: __pa7, commCosts: __pa8, nodeMark: __pa9, compInformations: __pa10 } = (iTargetTaskGraphMeta) else { bail!("pattern mismatch") };
    inComps = __pa1.clone();
    varCompMapping = __pa2.clone();
    eqCompMapping = __pa3.clone();
    compParamMapping = __pa4.clone();
    compNames = __pa5.clone();
    compDescs = __pa6.clone();
    exeCosts = __pa7.clone();
    commCosts = __pa8.clone();
    nodeMark = __pa9.clone();
    compInformations = __pa10.clone();
    for mut nodeIdx in 1..=metamodelica::arrayLength(iTaskGraph.clone()) {
        nodeComps = metamodelica::arrayGet(odeInComps.clone(), nodeIdx)?;
        for mut compIdx in &*nodeComps.clone() {
            let mut compIdx = compIdx.clone();
            componentInformation = combineComponentInformations(metamodelica::arrayGet(compInformations.clone(), compIdx)?, iComponentInformation)?;
            compInformations = metamodelica::arrayUpdate(compInformations.clone(), compIdx, componentInformation)?;
        }
    }
    oTargetTaskGraphMeta = TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    Ok(oTargetTaskGraphMeta)
}

fn combineComponentInformations(mut iComponentInfo: ComponentInfo, mut iComponentInfo2: ComponentInfo) -> Result<ComponentInfo> {
    let mut oComponentInfo: ComponentInfo;
    let mut isPartOfODESystem: bool;
    let mut iIsPartOfODESystem: bool;
    let mut isPartOfZeroFuncSystem: bool;
    let mut iisPartOfZeroFuncSystem: bool;
    let mut isRemovedComponent: bool;
    let mut iIsRemovedComponent: bool;
    let ComponentInfo { isPartOfODESystem: __pa0, isPartOfZeroFuncSystem: __pa1, isRemovedComponent: __pa2 } = (iComponentInfo) else { bail!("pattern mismatch") };
    isPartOfODESystem = __pa0.clone();
    isPartOfZeroFuncSystem = __pa1.clone();
    isRemovedComponent = __pa2.clone();
    let ComponentInfo { isPartOfODESystem: __pa3, isPartOfZeroFuncSystem: __pa4, isRemovedComponent: __pa5 } = (iComponentInfo2) else { bail!("pattern mismatch") };
    iIsPartOfODESystem = __pa3.clone();
    iisPartOfZeroFuncSystem = __pa4.clone();
    iIsRemovedComponent = __pa5.clone();
    oComponentInfo = ComponentInfo { isPartOfODESystem: boolOr(isPartOfODESystem, iIsPartOfODESystem), isPartOfZeroFuncSystem: boolOr(isPartOfZeroFuncSystem, iisPartOfZeroFuncSystem), isRemovedComponent: boolOr(isRemovedComponent, iIsRemovedComponent) };
    Ok(oComponentInfo)
}

fn addUpExeCosts(mut iExeCost1: (i32, metamodelica::Real), mut iExeCost2: (i32, metamodelica::Real)) -> (i32, metamodelica::Real) {
    let mut oExeCost: (i32, metamodelica::Real);
    let mut ex1: metamodelica::Real;
    let mut ex2: metamodelica::Real;
    let mut op1: i32;
    let mut op2: i32;
    (op1, ex1) = iExeCost1;
    (op2, ex2) = iExeCost2;
    oExeCost = (op1 + op2, (ex1) + (ex2));
    oExeCost
}

pub(crate) fn getExeCostReqCycles(mut iNodeIdx: i32, mut iGraphData: TaskGraphMeta) -> Result<metamodelica::Real> {
    let mut oExeCost: metamodelica::Real;
    oExeCost = Util::tuple22(getExeCost(iNodeIdx, iGraphData)?);
    Ok(oExeCost)
}

pub(crate) fn getExeCost(mut iNodeIdx: i32, mut iGraphData: TaskGraphMeta) -> Result<(i32, metamodelica::Real)> {
    let mut oExeCost: (i32, metamodelica::Real);
    let mut comp: i32 = 0;
    let mut opCount: i32;
    let mut opCount1: i32;
    let mut exeCost: metamodelica::Real;
    let mut exeCost1: metamodelica::Real;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut comps: Arc<metamodelica::List<i32>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let TaskGraphMeta { inComps: __pa0, exeCosts: __pa1, .. } = (iGraphData) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    exeCosts = __pa1.clone();
    exeCost = metamodelica::OrderedFloat(0.0_f64);
    opCount = 0;
    comps = metamodelica::arrayGet(inComps.clone(), iNodeIdx)?;
    for mut comp in &*comps {
        let mut comp = comp.clone();
        (opCount1, exeCost1) = metamodelica::arrayGet(exeCosts.clone(), comp)?;
        opCount = intAdd(opCount, opCount1);
        exeCost = (exeCost) + (exeCost1);
    }
    oExeCost = (opCount, exeCost);
    Ok(oExeCost)
}

fn getHighestExecCost(mut iExecCosts: Arc<metamodelica::List<(i32, metamodelica::Real)>>, mut iHighestTuple: (i32, metamodelica::Real)) -> (i32, metamodelica::Real) {
    let mut oHighestTuple: (i32, metamodelica::Real);
    let mut highestCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut currentCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut head: (i32, metamodelica::Real) = (0, metamodelica::OrderedFloat(0.0_f64));
    let mut rest: Arc<metamodelica::List<(i32, metamodelica::Real)>> = metamodelica::nil();
    oHighestTuple = 'mc: {
        let __mc_input = (iExecCosts, iHighestTuple);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ (_, currentCost), tail: rest }, (_, highestCost)) => {
                    let true = (realGt(currentCost.clone(), highestCost.clone())) else { bail!("pattern mismatch") };
                    Ok(getHighestExecCost(rest.clone(), head.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ (_, currentCost), tail: rest }, (_, highestCost)) => {
                    let true = (realGt(currentCost.clone(), highestCost.clone())) else { bail!("pattern mismatch") };
                    Ok(getHighestExecCost(rest.clone(), iHighestTuple))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iHighestTuple)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oHighestTuple
}

pub(crate) fn contractNodesInGraph(mut iContractNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iTaskGraph: TaskGraph, mut iTaskGraphT: TaskGraph, mut iTaskGraphMeta: TaskGraphMeta, mut iContractedTasks: metamodelica::Array<i32>) -> Result<(TaskGraph, TaskGraph, TaskGraphMeta, metamodelica::Array<i32>)> {
    let mut oTaskGraph: TaskGraph;
    let mut oTaskGraphT: TaskGraph;
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut oContractedTasks: metamodelica::Array<i32>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpTaskGraph: TaskGraph = iTaskGraph.clone();
    let mut tmpTaskGraphT: TaskGraph = iTaskGraphT.clone();
    let mut tmpContractedTasks: metamodelica::Array<i32> = iContractedTasks.clone();
    let mut nodeListHeadIdx: i32;
    let mut negNodeListHeadIdx: i32;
    let mut nodeIdx: i32 = 0;
    let mut parentChild: i32 = 0;
    let mut parentChildContractionValue: i32;
    let mut nodeListRestIdc: Arc<metamodelica::List<i32>>;
    let mut nodeCompIdc: Arc<metamodelica::List<i32>>;
    let mut headCompIdc: Arc<metamodelica::List<i32>>;
    let mut parentNodeChildList: Arc<metamodelica::List<i32>>;
    let mut parentNodeChildListNew: Arc<metamodelica::List<i32>>;
    let mut outgoingEdges: Arc<metamodelica::List<i32>>;
    let mut incomingEdges: Arc<metamodelica::List<i32>>;
    let mut nodeMarks: metamodelica::Array<i32>;
    let mut nodeMarksT: metamodelica::Array<i32>;
    let mut iNodeList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeList: Arc<metamodelica::List<i32>>;
    let mut childNodes: Arc<metamodelica::List<i32>>;
    let mut parentNodes: Arc<metamodelica::List<i32>>;
    let TaskGraphMeta { inComps: __pa0, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    nodeMarks = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), 0);
    nodeMarksT = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), 0);
    for mut iNodeList in &*iContractNodes {
        let mut iNodeList = iNodeList.clone();
        nodeList = metamodelica::nil();
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(iNodeList.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        nodeListHeadIdx = __pa1.clone();
        nodeListRestIdc = __pa2.clone();
        for mut nodeIdx in &*iNodeList.clone() {
            let mut nodeIdx = nodeIdx.clone();
            nodeIdx = getRealTaskIdxOfTask(nodeIdx, tmpContractedTasks.clone())?;
            if intNe(metamodelica::arrayGet(nodeMarks.clone(), nodeIdx)?, nodeListHeadIdx) {
                nodeMarks = metamodelica::arrayUpdate(nodeMarks.clone(), nodeIdx, nodeListHeadIdx)?;
                nodeList = metamodelica::cons(nodeIdx, nodeList.clone());
            }
        }
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(nodeList.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        nodeListHeadIdx = __pa3.clone();
        nodeListRestIdc = __pa4.clone();
        nodeListHeadIdx = getRealTaskIdxOfTask(nodeListHeadIdx, tmpContractedTasks.clone())?;
        negNodeListHeadIdx = intMul(-1, nodeListHeadIdx);
        for mut nodeIdx in &*nodeListRestIdc.clone() {
            let mut nodeIdx = nodeIdx.clone();
            nodeMarks = metamodelica::arrayUpdate(nodeMarks.clone(), nodeIdx, nodeListHeadIdx)?;
            nodeMarksT = metamodelica::arrayUpdate(nodeMarksT.clone(), nodeIdx, nodeListHeadIdx)?;
            tmpContractedTasks = metamodelica::arrayUpdate(tmpContractedTasks.clone(), nodeIdx, negNodeListHeadIdx)?;
        }
        nodeMarks = metamodelica::arrayUpdate(nodeMarks.clone(), nodeListHeadIdx, nodeListHeadIdx)?;
        nodeMarksT = metamodelica::arrayUpdate(nodeMarksT.clone(), nodeListHeadIdx, nodeListHeadIdx)?;
        outgoingEdges = metamodelica::arrayGet(tmpTaskGraph.clone(), nodeListHeadIdx)?;
        (outgoingEdges, _) = List::deleteMemberOnTrue(negNodeListHeadIdx, outgoingEdges.clone(), (std::sync::Arc::new({ let __pe_b2 = tmpContractedTasks.clone(); move |__pe_a0, __pe_a1| checkIfNodeBelongsToCluster(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        incomingEdges = metamodelica::arrayGet(tmpTaskGraphT.clone(), nodeListHeadIdx)?;
        List::map_0(outgoingEdges.clone(), (std::sync::Arc::new({ let __pe_b1 = nodeListHeadIdx; let __pe_b2 = nodeMarks.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>))?;
        List::map_0(incomingEdges.clone(), (std::sync::Arc::new({ let __pe_b1 = nodeListHeadIdx; let __pe_b2 = nodeMarksT.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>))?;
        childNodes = List::flatten(List::map(nodeListRestIdc.clone(), (std::sync::Arc::new({ let __pe_b1 = nodeListHeadIdx; let __pe_b2 = tmpTaskGraph.clone(); let __pe_b3 = tmpContractedTasks.clone(); let __pe_b4 = nodeMarks.clone(); move |__pe_a0| getContractedNodeChildren(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<metamodelica::List<i32>>> + 'static>))?)?;
        parentNodes = List::flatten(List::map(nodeList.clone(), (std::sync::Arc::new({ let __pe_b1 = nodeListHeadIdx; let __pe_b2 = iTaskGraphT.clone(); let __pe_b3 = tmpContractedTasks.clone(); let __pe_b4 = nodeMarks.clone(); move |__pe_a0| getContractedNodeChildren(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<metamodelica::List<i32>>> + 'static>))?)?;
        headCompIdc = metamodelica::arrayGet(inComps.clone(), nodeListHeadIdx)?;
        for mut nodeIdx in &*nodeListRestIdc.clone() {
            let mut nodeIdx = nodeIdx.clone();
            tmpTaskGraph = metamodelica::arrayUpdate(tmpTaskGraph.clone(), nodeIdx, metamodelica::nil())?;
            tmpTaskGraphT = metamodelica::arrayUpdate(tmpTaskGraphT.clone(), nodeIdx, metamodelica::nil())?;
            nodeCompIdc = metamodelica::arrayGet(inComps.clone(), nodeIdx)?;
            inComps = metamodelica::arrayUpdate(inComps.clone(), nodeIdx, metamodelica::nil())?;
            headCompIdc = List::insertListSorted(headCompIdc.clone(), nodeCompIdc.clone(), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        }
        metamodelica::arrayUpdate(inComps.clone(), nodeListHeadIdx, headCompIdc.clone())?;
        for mut nodeIdx in &*parentNodes.clone() {
            let mut nodeIdx = nodeIdx.clone();
            if intNe(metamodelica::arrayGet(nodeMarksT.clone(), nodeIdx)?, nodeListHeadIdx) {
                incomingEdges = metamodelica::cons(nodeIdx, incomingEdges.clone());
            }
        }
        tmpTaskGraphT = metamodelica::arrayUpdate(tmpTaskGraphT.clone(), nodeListHeadIdx, incomingEdges.clone())?;
        for mut nodeIdx in &*childNodes.clone() {
            let mut nodeIdx = nodeIdx.clone();
            parentNodeChildList = metamodelica::arrayGet(tmpTaskGraphT.clone(), nodeIdx)?;
            parentNodeChildListNew = metamodelica::nil();
            for mut parentChild in &*parentNodeChildList.clone() {
                let mut parentChild = parentChild.clone();
                parentChildContractionValue = metamodelica::arrayGet(tmpContractedTasks.clone(), parentChild)?;
                parentChild = getRealTaskIdxOfTask(parentChild, tmpContractedTasks.clone())?;
                if intEq(parentChild, nodeListHeadIdx) || intEq(parentChildContractionValue, negNodeListHeadIdx) {
                    if intNe(metamodelica::arrayGet(nodeMarksT.clone(), parentChild)?, nodeIdx) {
                        parentNodeChildListNew = metamodelica::cons(nodeListHeadIdx, parentNodeChildListNew.clone());
                        metamodelica::arrayUpdate(nodeMarksT.clone(), parentChild, nodeIdx)?;
                    }
                } else {
                    parentNodeChildListNew = metamodelica::cons(parentChild, parentNodeChildListNew.clone());
                }
            }
            tmpTaskGraphT = metamodelica::arrayUpdate(tmpTaskGraphT.clone(), nodeIdx, parentNodeChildListNew.clone())?;
        }
        outgoingEdges = listAppend(outgoingEdges.clone(), childNodes.clone());
        nodeMarks = metamodelica::arrayUpdate(nodeMarks.clone(), nodeListHeadIdx, 0)?;
        for mut nodeIdx in &*parentNodes.clone() {
            let mut nodeIdx = nodeIdx.clone();
            parentNodeChildList = metamodelica::arrayGet(tmpTaskGraph.clone(), nodeIdx)?;
            parentNodeChildListNew = metamodelica::nil();
            for mut parentChild in &*parentNodeChildList.clone() {
                let mut parentChild = parentChild.clone();
                parentChildContractionValue = metamodelica::arrayGet(tmpContractedTasks.clone(), parentChild)?;
                parentChild = getRealTaskIdxOfTask(parentChild, tmpContractedTasks.clone())?;
                if intEq(parentChild, nodeListHeadIdx) || intEq(parentChildContractionValue, negNodeListHeadIdx) {
                    if intNe(metamodelica::arrayGet(nodeMarks.clone(), parentChild)?, nodeIdx) {
                        parentNodeChildListNew = metamodelica::cons(nodeListHeadIdx, parentNodeChildListNew.clone());
                        metamodelica::arrayUpdate(nodeMarks.clone(), parentChild, nodeIdx)?;
                    }
                } else {
                    parentNodeChildListNew = metamodelica::cons(parentChild, parentNodeChildListNew.clone());
                }
            }
            tmpTaskGraph = metamodelica::arrayUpdate(tmpTaskGraph.clone(), nodeIdx, parentNodeChildListNew.clone())?;
        }
        tmpTaskGraph = metamodelica::arrayUpdate(tmpTaskGraph.clone(), nodeListHeadIdx, outgoingEdges.clone())?;
    }
    oTaskGraph = tmpTaskGraph.clone();
    oTaskGraphT = tmpTaskGraphT.clone();
    oTaskGraphMeta = iTaskGraphMeta;
    oContractedTasks = iContractedTasks.clone();
    Ok((oTaskGraph, oTaskGraphT, oTaskGraphMeta, oContractedTasks))
}

fn checkIfNodeBelongsToCluster(mut iNegativeRefValue: i32, mut iNodeIdx: i32, mut iContractedTasks: metamodelica::Array<i32>) -> Result<bool> {
    let mut oIsNodePartOfCluster: bool;
    oIsNodePartOfCluster = intEq(iNegativeRefValue, metamodelica::arrayGet(iContractedTasks.clone(), iNodeIdx)?);
    Ok(oIsNodePartOfCluster)
}

fn getContractedNodeChildren(mut iParentTask: i32, mut iRefValue: i32, mut iTaskGraph: TaskGraph, mut iContractedTasks: metamodelica::Array<i32>, mut iNodeMarks: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oChildTasks: Arc<metamodelica::List<i32>>;
    let mut task: i32 = 0;
    let mut taskMark: i32;
    let mut childTasks: Arc<metamodelica::List<i32>>;
    let mut resultTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    childTasks = metamodelica::arrayGet(iTaskGraph.clone(), iParentTask)?;
    for mut task in &*childTasks {
        let mut task = task.clone();
        task = getRealTaskIdxOfTask(task, iContractedTasks.clone())?;
        taskMark = metamodelica::arrayGet(iNodeMarks.clone(), task)?;
        if boolAnd(intNe(taskMark, iRefValue), intNe(task, iRefValue)) {
            resultTasks = metamodelica::cons(task, resultTasks.clone());
            metamodelica::arrayUpdate(iNodeMarks.clone(), task, iRefValue)?;
        }
    }
    oChildTasks = resultTasks;
    Ok(oChildTasks)
}

fn getRealTaskIdxOfTask(mut iTaskIdx: i32, mut iContractedTasks: metamodelica::Array<i32>) -> Result<i32> {
    '__tco: loop {
        let mut contractionMark: i32;
        contractionMark = metamodelica::arrayGet(iContractedTasks.clone(), iTaskIdx)?;
        if intLt(contractionMark, 0) {
            { (iTaskIdx, iContractedTasks) = (intMul(contractionMark, -1), iContractedTasks.clone()); continue '__tco; }
        } else {
            return Ok(iTaskIdx)
        }
    }
}

pub(crate) fn setInCompsInMeta(mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut metaIn: TaskGraphMeta) -> Result<TaskGraphMeta> {
    let mut metaOut: TaskGraphMeta;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let TaskGraphMeta { varCompMapping: __pa0, eqCompMapping: __pa1, compParamMapping: __pa2, compNames: __pa3, compDescs: __pa4, exeCosts: __pa5, commCosts: __pa6, nodeMark: __pa7, compInformations: __pa8, .. } = (metaIn) else { bail!("pattern mismatch") };
    varCompMapping = __pa0.clone();
    eqCompMapping = __pa1.clone();
    compParamMapping = __pa2.clone();
    compNames = __pa3.clone();
    compDescs = __pa4.clone();
    exeCosts = __pa5.clone();
    commCosts = __pa6.clone();
    nodeMark = __pa7.clone();
    compInformations = __pa8.clone();
    metaOut = TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    Ok(metaOut)
}

fn updateInCompsInfo(mut contrNode: i32, mut removedNodes: Arc<metamodelica::List<i32>>, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut comps: Arc<metamodelica::List<i32>>;
    let mut contrComps: Arc<metamodelica::List<i32>>;
    comps = metamodelica::arrayGet(inComps.clone(), contrNode)?;
    contrComps = List::flatten(List::map(removedNodes, (std::sync::Arc::new({ let __pe_b1 = inComps.clone(); move |__pe_a0| Array::getIndexFirst(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<_> + 'static>))?)?;
    comps = List::unique(listAppend(contrComps, comps));
    metamodelica::arrayUpdate(inComps.clone(), contrNode, comps)?;
    Ok(())
}

pub(crate) fn filterContractedNodes(mut nodesIn: Arc<metamodelica::List<i32>>, mut contrNodes: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut nodesOut: Arc<metamodelica::List<i32>>;
    nodesOut = List::filterOnFalse(nodesIn, (std::sync::Arc::new({ let __pe_b1 = contrNodes.clone(); move |__pe_a0| isNodeContracted(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
    Ok(nodesOut)
}

pub(crate) fn filterNonContractedNodes(mut nodesIn: Arc<metamodelica::List<i32>>, mut contrNodes: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut nodesOut: Arc<metamodelica::List<i32>>;
    nodesOut = List::filterOnTrue(nodesIn, (std::sync::Arc::new({ let __pe_b1 = contrNodes.clone(); move |__pe_a0| isNodeContracted(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
    Ok(nodesOut)
}

pub(crate) fn isNodeContracted(mut iNode: i32, mut iContrNodes: metamodelica::Array<i32>) -> Result<bool> {
    let mut oIsContracted: bool;
    if intLe(iNode, metamodelica::arrayLength(iContrNodes.clone())) {
        oIsContracted = intLt(metamodelica::arrayGet(iContrNodes.clone(), iNode)?, 0);
    } else {
        oIsContracted = false;
    }
    Ok(oIsContracted)
}

fn contractNodesInGraph1(mut contractNodes: Arc<metamodelica::List<i32>>, mut graphIn: TaskGraph) -> Result<TaskGraph> {
    let mut graphOut: TaskGraph;
    let mut graphInT: TaskGraph;
    let mut endNode: i32;
    let mut startNode: i32;
    let mut deleteEntries: Arc<metamodelica::List<i32>>;
    let mut startNodeChildren: Arc<metamodelica::List<i32>>;
    let mut endChildren: Arc<metamodelica::List<i32>>;
    let mut deleteNodesParents: Arc<metamodelica::List<i32>>;
    let mut graphTmp: TaskGraph;
    graphInT = AdjacencyMatrix::transposeAdjacencyMatrix(graphIn.clone(), metamodelica::arrayLength(graphIn.clone()))?;
    startNode = List::last(contractNodes.clone())?;
    (deleteEntries, _) = List::deleteMemberOnTrue(startNode, contractNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    deleteNodesParents = List::flatten(List::map1(deleteEntries.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), graphInT.clone())?)?;
    deleteNodesParents = List::sortedUnique(List::sort(deleteNodesParents, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    deleteNodesParents = List::setDifferenceOnTrue(deleteNodesParents, contractNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    endNode = listHead(contractNodes)?;
    endChildren = metamodelica::arrayGet(graphIn.clone(), endNode)?;
    startNodeChildren = metamodelica::arrayGet(graphIn.clone(), startNode)?;
    startNodeChildren = List::setDifferenceOnTrue(startNodeChildren, deleteEntries.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    graphTmp = metamodelica::arrayUpdate(graphIn.clone(), startNode, startNodeChildren)?;
    graphTmp = List::fold2(deleteNodesParents, (std::sync::Arc::new(contractNodesInGraph2) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), deleteEntries, startNode, graphTmp.clone())?;
    graphTmp = metamodelica::arrayUpdate(graphIn.clone(), startNode, endChildren)?;
    graphOut = graphTmp.clone();
    Ok(graphOut)
}

fn contractNodesInGraph2(mut iParentNode: i32, mut iDeletedNodes: Arc<metamodelica::List<i32>>, mut iNewNodeIdx: i32, mut iGraph: TaskGraph) -> Result<TaskGraph> {
    let mut oGraph: TaskGraph;
    let mut adjLstEntry: Arc<metamodelica::List<i32>>;
    adjLstEntry = metamodelica::arrayGet(iGraph.clone(), iParentNode)?;
    adjLstEntry = List::setDifferenceOnTrue(adjLstEntry, iDeletedNodes, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    adjLstEntry = metamodelica::cons(iNewNodeIdx, adjLstEntry);
    adjLstEntry = List::sortedUnique(List::sort(adjLstEntry, (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    oGraph = metamodelica::arrayUpdate(iGraph.clone(), iParentNode, adjLstEntry)?;
    Ok(oGraph)
}

fn compareListLengthOnTrue(mut inValue: i32, mut inLst: Arc<metamodelica::List<i32>>) -> bool {
    let mut equalLength: bool;
    equalLength = 'mc: {
        let __mc_input = inLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (intEq(inValue, (inLst.clone().len() as i32))) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    equalLength
}

fn getMergedSystemData(mut graphDataIn: TaskGraphMeta, mut contractNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<TaskGraphMeta> {
    let mut graphDataOut: TaskGraphMeta;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let TaskGraphMeta { inComps: __pa0, varCompMapping: __pa1, eqCompMapping: __pa2, compParamMapping: __pa3, compNames: __pa4, compDescs: __pa5, exeCosts: __pa6, commCosts: __pa7, nodeMark: __pa8, compInformations: __pa9 } = (graphDataIn) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    varCompMapping = __pa1.clone();
    eqCompMapping = __pa2.clone();
    compParamMapping = __pa3.clone();
    compNames = __pa4.clone();
    compDescs = __pa5.clone();
    exeCosts = __pa6.clone();
    commCosts = __pa7.clone();
    nodeMark = __pa8.clone();
    compInformations = __pa9.clone();
    inComps = updateInCompsForMerging(inComps.clone(), contractNodes)?;
    compNames = List::fold2(List::intRange(metamodelica::arrayLength(compNames.clone())), (std::sync::Arc::new(updateCompNamesForMerging) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, metamodelica::Array<ArcStr>) -> Result<metamodelica::Array<ArcStr>> + 'static>), inComps.clone(), nodeMark.clone(), compNames.clone())?;
    graphDataOut = TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    Ok(graphDataOut)
}

fn updateCompNamesForMerging(mut compIdx: i32, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nodeMark: metamodelica::Array<i32>, mut compNamesIn: metamodelica::Array<ArcStr>) -> Result<metamodelica::Array<ArcStr>> {
    let mut compNamesOut: metamodelica::Array<ArcStr>;
    compNamesOut = 'mc: {
        let __mc_input = compNamesIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut unionNode: i32;
            let mut mergedComps: Arc<metamodelica::List<i32>>;
            let true = (compIdx <= metamodelica::arrayLength(compNamesIn.clone())) else { bail!("pattern mismatch") };
            unionNode = getCompInComps(compIdx, 1, inComps.clone(), nodeMark.clone())?;
            let true = (unionNode.clone() != -1) else { bail!("pattern mismatch") };
            mergedComps = metamodelica::arrayGet(inComps.clone(), unionNode.clone())?;
            let true = ((mergedComps.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
            Ok(compNamesIn.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut unionNode: i32;
            let mut mergedComps: Arc<metamodelica::List<i32>>;
            let mut compNamesTmp: metamodelica::Array<ArcStr>;
            let mut compName: ArcStr;
            let true = (compIdx <= metamodelica::arrayLength(compNamesIn.clone())) else { bail!("pattern mismatch") };
            unionNode = getCompInComps(compIdx, 1, inComps.clone(), nodeMark.clone())?;
            let true = (unionNode.clone() != -1) else { bail!("pattern mismatch") };
            mergedComps = metamodelica::arrayGet(inComps.clone(), unionNode.clone())?;
            let false = ((mergedComps.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
            compName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("contracted comps ")); __mm_s.push_str(&*stringDelimitList(List::map(mergedComps.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone())); ArcStr::from(__mm_s) }).clone();
            compNamesTmp = metamodelica::arrayUpdate(compNamesIn.clone(), compIdx, (compName.clone()).clone())?;
            Ok(compNamesTmp.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut unionNode: i32;
            let true = (compIdx <= metamodelica::arrayLength(compNamesIn.clone())) else { bail!("pattern mismatch") };
            unionNode = getCompInComps(compIdx, 1, inComps.clone(), nodeMark.clone())?;
            let true = (unionNode.clone() == -1) else { bail!("pattern mismatch") };
            Ok(compNamesIn.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("updateCompNamesForMerging failed!\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(compNamesOut)
}

fn updateInCompsForMerging(mut inCompsIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mergedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut inCompsOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inCompsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut deleteNodes: Arc<metamodelica::List<i32>>;
    let mut startNodes: Arc<metamodelica::List<i32>>;
    startNodes = List::map(mergedPaths.clone(), (std::sync::Arc::new(List::last) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
    (_, deleteNodes, _) = List::intersection1OnTrue(List::flatten(mergedPaths.clone())?, startNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    inCompsLst = Arc::new(inCompsIn.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    inCompsLst = List::fold2(List::intRange(metamodelica::arrayLength(inCompsIn.clone())), (std::sync::Arc::new(fnptr!(updateInComps1, i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>), metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>), metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> + 'static>), (startNodes, deleteNodes, mergedPaths), inCompsIn.clone(), inCompsLst)?;
    inCompsLst = List::removeOnTrue(metamodelica::nil(), (std::sync::Arc::new(fnptr!(equalLists, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>), inCompsLst)?;
    inCompsOut = metamodelica::arrayFromVec(inCompsLst.into_iter().cloned().collect());
    Ok(inCompsOut)
}

fn updateInComps1(mut nodeIdx: i32, mut mergeInfo: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>), mut primInComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inCompLstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> {
    let mut inCompLstOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    inCompLstOut = 'mc: {
        let __mc_input = inCompLstIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut mergeGroupIdx: i32;
                    let mut inComps: Arc<metamodelica::List<i32>>;
                    let mut mergedSet: Arc<metamodelica::List<i32>>;
                    let mut mergedNodes: Arc<metamodelica::List<i32>>;
                    let mut startNodes: Arc<metamodelica::List<i32>>;
                    let mut mergedPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut inCompLstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    (startNodes, _, mergedPaths) = mergeInfo.clone();
                    inComps = (inCompLstIn.clone()).get(nodeIdx)?;
                    (inComps.clone()).get(1)?;
                    let true = (List::isMemberOnTrue(nodeIdx, startNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    mergeGroupIdx = List::position(nodeIdx, startNodes.clone())?;
                    mergedNodes = (mergedPaths.clone()).get(mergeGroupIdx.clone())?;
                    mergedSet = List::flatten(List::map1(mergedNodes.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), primInComps.clone())?)?;
                    inCompLstTmp = List::fold(mergedNodes.clone(), (std::sync::Arc::new(updateInComps2) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> + 'static>), inCompLstIn.clone())?;
                    inCompLstTmp = List::replaceAt(mergedSet.clone(), nodeIdx, inCompLstTmp.clone())?;
                    Ok(inCompLstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inCompLstIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    inCompLstOut
}

fn updateInComps2(mut iNodeIdx: i32, mut inCompLstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut inCompLstOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    inCompLstOut = List::replaceAt(metamodelica::nil(), iNodeIdx, inCompLstIn)?;
    Ok(inCompLstOut)
}

pub(crate) fn equalLists(mut inList1: Arc<metamodelica::List<i32>>, mut inList2: Arc<metamodelica::List<i32>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inList1, inList2)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return true
        },
        (Deref @ metamodelica::List::Nil, _) => {
            return false
        },
        (_, Deref @ metamodelica::List::Nil) => {
            return false
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: e2, tail: rest2 }) if (intEq(e1.clone(), e2.clone())) => {
            { (inList1, inList2) = (rest1.clone(), rest2.clone()); continue '__tco; }
        },
        _ => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn findOneChildParents(mut allNodes: Arc<metamodelica::List<i32>>, mut graphIn: TaskGraph, mut doNotMerge: Arc<metamodelica::List<i32>>, mut lstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inPath: i32, mut contrNodes: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut lstOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    lstOut = 'mc: {
        let __mc_input = allNodes.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(lstIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
                    let mut nodeChildren: Arc<metamodelica::List<i32>>;
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let true = (intEq(inPath, 0)) else { bail!("pattern mismatch") };
                    nodeChildren = metamodelica::arrayGet(graphIn.clone(), head.clone())?;
                    nodeChildren = filterContractedNodes(nodeChildren.clone(), contrNodes.clone())?;
                    let false = ((nodeChildren.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
                    lstTmp = findOneChildParents(rest.clone(), graphIn.clone(), doNotMerge.clone(), lstIn.clone(), 0, contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let true = (intEq(inPath, 0)) else { bail!("pattern mismatch") };
                    let true = (listMember(head.clone(), doNotMerge.clone())) else { bail!("pattern mismatch") };
                    lstTmp = findOneChildParents(rest.clone(), graphIn.clone(), doNotMerge.clone(), lstIn.clone(), 0, contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
                    let mut child: i32;
                    let mut nodeChildren: Arc<metamodelica::List<i32>>;
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let true = (intEq(inPath, 0)) else { bail!("pattern mismatch") };
                    nodeChildren = metamodelica::arrayGet(graphIn.clone(), head.clone())?;
                    nodeChildren = filterContractedNodes(nodeChildren.clone(), contrNodes.clone())?;
                    let true = ((nodeChildren.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
                    child = (nodeChildren.clone()).get(1)?;
                    let true = (listMember(child.clone(), doNotMerge.clone())) else { bail!("pattern mismatch") };
                    lstTmp = findOneChildParents(rest.clone(), graphIn.clone(), doNotMerge.clone(), lstIn.clone(), child.clone(), contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
                    let mut child: i32;
                    let mut nodeChildren: Arc<metamodelica::List<i32>>;
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let true = (intEq(inPath, 0)) else { bail!("pattern mismatch") };
                    nodeChildren = metamodelica::arrayGet(graphIn.clone(), head.clone())?;
                    nodeChildren = filterContractedNodes(nodeChildren.clone(), contrNodes.clone())?;
                    let true = ((nodeChildren.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
                    child = (nodeChildren.clone()).get(1)?;
                    lstTmp = metamodelica::cons(list![head.clone()], lstIn.clone());
                    lstTmp = findOneChildParents(rest.clone(), graphIn.clone(), doNotMerge.clone(), lstTmp.clone(), child.clone(), contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: _ } => {
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let false = (intEq(inPath, 0)) else { bail!("pattern mismatch") };
                    let true = (listMember(inPath, doNotMerge.clone())) else { bail!("pattern mismatch") };
                    lstTmp = findOneChildParents(allNodes.clone(), graphIn.clone(), doNotMerge.clone(), lstIn.clone(), 0, contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut child: i32;
                    let mut nodeChildren: Arc<metamodelica::List<i32>>;
                    let mut parents: Arc<metamodelica::List<i32>>;
                    let mut pathLst: Arc<metamodelica::List<i32>>;
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut rest = (*rest).clone();
                    let false = (intEq(inPath, 0)) else { bail!("pattern mismatch") };
                    nodeChildren = metamodelica::arrayGet(graphIn.clone(), inPath)?;
                    nodeChildren = filterContractedNodes(nodeChildren.clone(), contrNodes.clone())?;
                    parents = getParentNodes(inPath, graphIn.clone())?;
                    parents = filterContractedNodes(parents.clone(), contrNodes.clone())?;
                    let true = ((nodeChildren.clone().len() as i32) == 1 && !(nodeChildren.clone().is_empty()) && (parents.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
                    child = (nodeChildren.clone()).get(1)?;
                    pathLst = listHead(lstIn.clone())?;
                    pathLst = metamodelica::cons(inPath, pathLst.clone());
                    lstTmp = List::replaceAt(pathLst.clone(), 1, lstIn.clone())?;
                    (rest, _) = List::deleteMemberOnTrue(inPath, allNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    lstTmp = findOneChildParents(rest.clone(), graphIn.clone(), doNotMerge.clone(), lstTmp.clone(), child.clone(), contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut nodeChildren: Arc<metamodelica::List<i32>>;
                    let mut parents: Arc<metamodelica::List<i32>>;
                    let mut pathLst: Arc<metamodelica::List<i32>>;
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut rest = (*rest).clone();
                    let false = (intEq(inPath, 0)) else { bail!("pattern mismatch") };
                    nodeChildren = metamodelica::arrayGet(graphIn.clone(), inPath)?;
                    nodeChildren = filterContractedNodes(nodeChildren.clone(), contrNodes.clone())?;
                    parents = getParentNodes(inPath, graphIn.clone())?;
                    parents = filterContractedNodes(parents.clone(), contrNodes.clone())?;
                    pathLst = listHead(lstIn.clone())?;
                    pathLst = metamodelica::cons(inPath, pathLst.clone());
                    lstTmp = List::replaceAt(pathLst.clone(), 1, lstIn.clone())?;
                    (rest, _) = List::deleteMemberOnTrue(inPath, allNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    lstTmp = findOneChildParents(rest.clone(), graphIn.clone(), doNotMerge.clone(), lstTmp.clone(), 0, contrNodes.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("findOneChildParents failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(lstOut)
}

fn getParentNodes(mut nodeIdx: i32, mut graphIn: TaskGraph) -> Result<Arc<metamodelica::List<i32>>> {
    let mut parentNodes: Arc<metamodelica::List<i32>>;
    let mut graphInT: TaskGraph;
    graphInT = AdjacencyMatrix::transposeAdjacencyMatrix(graphIn.clone(), metamodelica::arrayLength(graphIn.clone()))?;
    parentNodes = metamodelica::arrayGet(graphInT.clone(), nodeIdx)?;
    Ok(parentNodes)
}

fn checkParentNode(mut lstIdx: i32, mut graphIn: TaskGraph, mut lstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut lstOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    lstOut = 'mc: {
        let __mc_input = lstIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut childLst: Arc<metamodelica::List<i32>>;
                    let mut child: i32;
                    let mut parent: i32;
                    let mut parents: Arc<metamodelica::List<i32>>;
                    let mut lstTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    childLst = (lstIn.clone()).get(lstIdx)?;
                    child = List::last(childLst.clone())?;
                    parents = getParentNodes(child.clone(), graphIn.clone())?;
                    let true = (intEq((parents.clone().len() as i32), 1)) else { bail!("pattern mismatch") };
                    parent = (parents.clone()).get(1)?;
                    childLst = childLst.clone().reverse();
                    childLst = metamodelica::cons(parent.clone(), childLst.clone());
                    childLst = childLst.clone().reverse();
                    lstTmp = List::replaceAt(childLst.clone(), lstIdx, lstIn.clone())?;
                    Ok(lstTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut childLst: Arc<metamodelica::List<i32>>;
                    let mut child: i32;
                    let mut parents: Arc<metamodelica::List<i32>>;
                    childLst = (lstIn.clone()).get(lstIdx)?;
                    child = List::last(childLst.clone())?;
                    parents = getParentNodes(child.clone(), graphIn.clone())?;
                    let false = (intEq((parents.clone().len() as i32), 1)) else { bail!("pattern mismatch") };
                    Ok(lstIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(lstOut)
}

//-----------------------------
//  Functions to generate costs
//-----------------------------
pub(crate) fn createCosts(mut iDae: Arc<BackendDAE::BackendDAE>, mut iBenchFilePrefix: ArcStr, mut iSimEqCompMapping: metamodelica::Array<i32>, mut iTaskGraphMeta: TaskGraphMeta) -> Result<TaskGraphMeta> {
    let mut oTaskGraphMeta: TaskGraphMeta;
    let mut compMapping: metamodelica::Array<Arc<BackendDAE::EqSystem>> = Default::default();
    let mut compMapping_withIdx: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)> = Default::default();
    let mut shared: Arc<BackendDAE::Shared> = Arc::new(<BackendDAE::Shared as ::std::default::Default>::default());
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut reqTimeCom: (i32, i32) = (0, 0);
    let mut reqTimeOpLstSimCode: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>> = metamodelica::nil();
    let mut reqTimeOpSimCode: metamodelica::Array<(i32, metamodelica::Real)> = Default::default();
    let mut tmpTaskGraphMeta: TaskGraphMeta = <TaskGraphMeta as ::std::default::Default>::default();
    let mut reqTimeOp: metamodelica::Array<metamodelica::Real> = Default::default();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>> = Default::default();
    oTaskGraphMeta = 'mc: {
        let __mc_input = (iDae.clone(), iTaskGraphMeta.clone());
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::BackendDAE { shared, .. }, TaskGraphMeta { inComps, commCosts, .. }) => {
                    let mut commCosts = (*commCosts).clone();
                    let mut compMapping: metamodelica::Array<Arc<BackendDAE::EqSystem>> = compMapping.clone();
                    let mut compMapping_withIdx: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)> = compMapping_withIdx.clone();
                    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = comps.clone();
                    let mut reqTimeCom: (i32, i32) = reqTimeCom.clone();
                    let mut reqTimeOp: metamodelica::Array<metamodelica::Real> = reqTimeOp.clone();
                    let mut reqTimeOpLstSimCode: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>> = reqTimeOpLstSimCode.clone();
                    let mut reqTimeOpSimCode: metamodelica::Array<(i32, metamodelica::Real)> = reqTimeOpSimCode.clone();
                    let mut tmpTaskGraphMeta: TaskGraphMeta = tmpTaskGraphMeta.clone();
                    (comps, compMapping_withIdx) = getSystemComponents(iDae.clone())?;
                    compMapping = Array::map(compMapping_withIdx.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)))?;
                    (_, reqTimeCom) = HpcOmBenchmark::benchSystem()?;
                    reqTimeOpLstSimCode = HpcOmBenchmark::readCalcTimesFromFile((iBenchFilePrefix.clone()).clone())?;
                    reqTimeOpSimCode = arrayCreate((reqTimeOpLstSimCode.clone().len() as i32), (-1, metamodelica::OrderedFloat(-1.0_f64)));
                    reqTimeOpSimCode = List::fold(reqTimeOpLstSimCode.clone(), (std::sync::Arc::new(createCosts1) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32, metamodelica::Real), metamodelica::Array<(i32, metamodelica::Real)>) -> Result<metamodelica::Array<(i32, metamodelica::Real)>> + 'static>), reqTimeOpSimCode.clone())?;
                    reqTimeOp = arrayCreate((comps.clone().len() as i32), metamodelica::OrderedFloat(-1.0_f64));
                    reqTimeOp = convertSimEqToSccCosts(reqTimeOpSimCode.clone(), iSimEqCompMapping.clone(), reqTimeOp.clone())?;
                    commCosts = createCommCosts(commCosts.clone(), 1, reqTimeCom);
                    (_, tmpTaskGraphMeta) = Array::fold(inComps.clone(), (std::sync::Arc::new({ let __pe_b1 = (comps.clone(), shared.clone()); let __pe_b2 = compMapping.clone(); let __pe_b3 = reqTimeOp.clone(); let __pe_b4 = reqTimeCom; move |__pe_a0, __pe_a5| createCosts0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_a5) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (i32, TaskGraphMeta)) -> Result<(i32, TaskGraphMeta)> + 'static>), (1, iTaskGraphMeta.clone()))?;
                    Ok((tmpTaskGraphMeta.clone(), compMapping.clone(), compMapping_withIdx.clone(), comps.clone(), reqTimeCom.clone(), reqTimeOp.clone(), reqTimeOpLstSimCode.clone(), reqTimeOpSimCode.clone(), tmpTaskGraphMeta.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { compMapping = __wb0; compMapping_withIdx = __wb1; comps = __wb2; reqTimeCom = __wb3; reqTimeOp = __wb4; reqTimeOpLstSimCode = __wb5; reqTimeOpSimCode = __wb6; tmpTaskGraphMeta = __wb7; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tmpTaskGraphMeta: TaskGraphMeta = tmpTaskGraphMeta.clone();
                    tmpTaskGraphMeta = estimateCosts(iDae.clone(), iTaskGraphMeta.clone())?;
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Warning: The costs have been estimated. Maybe ")); __mm_s.push_str(&*iBenchFilePrefix.clone()); __mm_s.push_str(&*literal!("-file is missing.\n")); ArcStr::from(__mm_s) }).clone());
                    Ok((tmpTaskGraphMeta.clone(), tmpTaskGraphMeta.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { tmpTaskGraphMeta = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oTaskGraphMeta)
}

fn estimateCosts(mut daeIn: Arc<BackendDAE::BackendDAE>, mut taskGraphMetaIn: TaskGraphMeta) -> Result<TaskGraphMeta> {
    let mut taskGraphMetaOut: TaskGraphMeta;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut comNumLst: Arc<metamodelica::List<i32>>;
    let mut exeCostsLst: Arc<metamodelica::List<(i32, metamodelica::Real)>>;
    let mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut compsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>>>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    let mut compIdx: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(daeIn) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eqSystems = __pa0.clone();
    shared = __pa1.clone();
    compsLst = List::map(eqSystems.clone(), (std::sync::Arc::new(fnptr!(BackendDAEUtil::getStrongComponents, Arc<BackendDAE::EqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>> + 'static>))?;
    comNumLst = List::map(compsLst.clone(), std::sync::Arc::new(fnptr!(listLength, _)))?;
    let TaskGraphMeta { inComps: __pa2, varCompMapping: __pa3, eqCompMapping: __pa4, compParamMapping: __pa5, compNames: __pa6, compDescs: __pa7, exeCosts: __pa8, commCosts: __pa9, nodeMark: __pa10, compInformations: __pa11 } = (taskGraphMetaIn) else { bail!("pattern mismatch") };
    inComps = __pa2.clone();
    varCompMapping = __pa3.clone();
    eqCompMapping = __pa4.clone();
    compParamMapping = __pa5.clone();
    compNames = __pa6.clone();
    compDescs = __pa7.clone();
    exeCosts = __pa8.clone();
    commCosts = __pa9.clone();
    nodeMark = __pa10.clone();
    compInformations = __pa11.clone();
    commCosts = getCommCostsOnly(commCosts.clone())?;
    exeCostsLst = List::flatten(List::map3(List::intRange((compsLst.clone().len() as i32)), (std::sync::Arc::new(estimateCosts0) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>>>, Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, Arc<BackendDAE::Shared>) -> Result<Arc<metamodelica::List<(i32, metamodelica::Real)>>> + 'static>), compsLst, eqSystems, shared)?)?;
    compIdx = 1;
    for mut exeCost in &*exeCostsLst {
        let mut exeCost = exeCost.clone();
        metamodelica::arrayUpdate(exeCosts.clone(), compIdx, exeCost.clone())?;
        compIdx = compIdx + 1;
    }
    taskGraphMetaOut = TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    Ok(taskGraphMetaOut)
}

fn estimateCosts0(mut systIdx: i32, mut compsLstIn: Arc<metamodelica::List<Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>>>, mut eqSystemsIn: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut sharedIn: Arc<BackendDAE::Shared>) -> Result<Arc<metamodelica::List<(i32, metamodelica::Real)>>> {
    let mut exeCostsOut: Arc<metamodelica::List<(i32, metamodelica::Real)>>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut eqSys: Arc<BackendDAE::EqSystem>;
    let mut compsInfos: Arc<metamodelica::List<Arc<BackendDAE::CompInfo>>>;
    comps = (compsLstIn).get(systIdx)?;
    eqSys = (eqSystemsIn).get(systIdx)?;
    compsInfos = BackendDAEOptimize::countOperationstraverseComps(comps, eqSys, sharedIn, metamodelica::nil())?.reverse();
    exeCostsOut = List::map(compsInfos, (std::sync::Arc::new(fnptr!(calculateCosts, Arc<BackendDAE::CompInfo>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::CompInfo>) -> Result<(i32, metamodelica::Real)> + 'static>))?;
    Ok(exeCostsOut)
}

pub(crate) fn calculateCosts(mut compInfo: Arc<BackendDAE::CompInfo>) -> (i32, metamodelica::Real) {
    let mut exeCost: (i32, metamodelica::Real);
    exeCost = (::match_deref::match_deref! { match &(compInfo) {
        Deref @ BackendDAE::CompInfo::COUNTER { comp, numAdds, numMul, numDiv, numTrig, numRelations: numRel, numLog, numOth, funcCalls: numFuncs } => {
            let mut costs: i32;
            let mut ops: i32;
            let mut offset: i32;
            ops = numAdds.clone() + numMul.clone() + numOth.clone() + numTrig.clone() + numRel.clone() + numLog.clone();
            if BackendDAEUtil::isSingleEquationComp(comp.clone()) {
                offset = 35;
            } else if BackendDAEUtil::isWhenComp(comp.clone()) {
                offset = 113;
            } else if BackendDAEUtil::isArrayComp(comp.clone()) {
                offset = 100;
            } else {
                offset = 0;
            }
            costs = offset + 12 * numAdds.clone() + 32 * numMul.clone() + 37 * numDiv.clone() + 236 * numTrig.clone() + 2 * numRel.clone() + 4 * numLog.clone() + 110 * numOth.clone() + 375 * numFuncs.clone();
            (ops, intReal(costs))
        },
        Deref @ BackendDAE::CompInfo::SYSTEM { size, density: dens, .. } => {
            let mut allOpCosts: metamodelica::Real;
            allOpCosts = (metamodelica::OrderedFloat(0.049_f64)) * (realPow((intReal(size.clone())) * ((metamodelica::OrderedFloat(1.0_f64)) + ((dens.clone()) * (metamodelica::OrderedFloat(19.0_f64)))), metamodelica::OrderedFloat(3.0_f64)));
            (1, allOpCosts)
        },
        Deref @ BackendDAE::CompInfo::TORN_ANALYSE { tornEqs: torn, otherEqs: other, tornSize: size, .. } => {
            let mut ops: i32;
            let mut ops1: i32;
            let mut allOpCosts: metamodelica::Real;
            let mut tornCosts: metamodelica::Real;
            let mut otherCosts: metamodelica::Real;
            (ops, tornCosts) = calculateCosts(torn.clone());
            (ops1, otherCosts) = calculateCosts(other.clone());
            allOpCosts = ((metamodelica::OrderedFloat(3000.0_f64)) + ((metamodelica::OrderedFloat(7.62_f64)) * (realPow(intReal(size.clone()), metamodelica::OrderedFloat(3.0_f64))))) + (((metamodelica::OrderedFloat(2.0_f64)) * (tornCosts)) + ((metamodelica::OrderedFloat(1.4_f64)) * (otherCosts)));
            (ops + ops1, allOpCosts)
        },
        Deref @ BackendDAE::CompInfo::NO_COMP { numAdds, numMul, numDiv, numTrig, numRelations: numRel, numLog, numOth, funcCalls: numFuncs } => {
            let mut costs: i32;
            let mut ops: i32;
            let mut offset: i32;
            ops = numAdds.clone() + numMul.clone() + numOth.clone() + numTrig.clone() + numRel.clone() + numLog.clone();
            offset = 50;
            costs = offset + 12 * numAdds.clone() + 32 * numMul.clone() + 37 * numDiv.clone() + 236 * numTrig.clone() + 2 * numRel.clone() + 4 * numLog.clone() + 110 * numOth.clone() + 375 * numFuncs.clone();
            (ops, intReal(costs))
        },
        _ => {
            metamodelica::print((literal!("calculate costs failed!\n")).clone());
            (-1, metamodelica::OrderedFloat(-1.0_f64))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exeCost
}

pub(crate) fn copyCosts(mut iSourceTaskGraphData: TaskGraphMeta, mut iTargetTaskGraphData: TaskGraphMeta) -> Result<TaskGraphMeta> {
    let mut oTaskGraphData: TaskGraphMeta;
    let mut inCompsSource: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inCompsTarget: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut exeCostsSource: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut exeCostsTarget: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut compIdx: i32;
    let mut commCostsTarget: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut reqTimeCom: (i32, i32);
    let TaskGraphMeta { inComps: __pa0, exeCosts: __pa1, .. } = (iSourceTaskGraphData) else { bail!("pattern mismatch") };
    inCompsSource = __pa0.clone();
    exeCostsSource = __pa1.clone();
    let TaskGraphMeta { inComps: __pa2, exeCosts: __pa3, commCosts: __pa4, .. } = (iTargetTaskGraphData.clone()) else { bail!("pattern mismatch") };
    inCompsTarget = __pa2.clone();
    exeCostsTarget = __pa3.clone();
    commCostsTarget = __pa4.clone();
    compIdx = intMin(metamodelica::arrayLength(exeCostsSource.clone()), metamodelica::arrayLength(exeCostsTarget.clone()));
    while intGt(compIdx, 0) {
        exeCostsTarget = metamodelica::arrayUpdate(exeCostsTarget.clone(), compIdx, metamodelica::arrayGet(exeCostsSource.clone(), compIdx)?)?;
        compIdx = compIdx - 1;
    }
    (_, reqTimeCom) = HpcOmBenchmark::benchSystem()?;
    commCostsTarget = createCommCosts(commCostsTarget.clone(), 1, reqTimeCom);
    oTaskGraphData = iTargetTaskGraphData;
    Ok(oTaskGraphData)
}

fn getCommCostsOnly(mut commCostsIn: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> {
    let mut commCostsOut: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut reqTimeCom: (i32, i32);
    (_, reqTimeCom) = HpcOmBenchmark::benchSystem()?;
    commCostsOut = createCommCosts(commCostsIn.clone(), 1, reqTimeCom);
    Ok(commCostsOut)
}

fn checkForExecutionCosts(mut dataIn: TaskGraphMeta) -> Result<bool> {
    let mut isFine: bool;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let TaskGraphMeta { inComps: __pa0, exeCosts: __pa1, .. } = (dataIn) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    exeCosts = __pa1.clone();
    isFine = checkForExecutionCosts1(exeCosts.clone(), inComps.clone(), 1);
    if !(isFine) {
        metamodelica::print((literal!("There are execution costs with value 0.0!\n")).clone());
    }
    Ok(isFine)
}

fn checkForExecutionCosts1(mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nodeIdx: i32) -> bool {
    let mut bOut: bool;
    bOut = 'mc: {
        let __mc_input = nodeIdx;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut b: bool;
            let mut isZero: bool;
            let mut comps: Arc<metamodelica::List<i32>>;
            let true = (metamodelica::arrayLength(inComps.clone()) >= nodeIdx) else { bail!("pattern mismatch") };
            comps = metamodelica::arrayGet(inComps.clone(), nodeIdx)?;
            isZero = List::fold1(comps.clone(), (std::sync::Arc::new(checkTpl2ForZero) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<(i32, metamodelica::Real)>, bool) -> Result<bool> + 'static>), exeCosts.clone(), false)?;
            let false = (isZero.clone()) else { bail!("pattern mismatch") };
            b = checkForExecutionCosts1(exeCosts.clone(), inComps.clone(), nodeIdx + 1);
            Ok(b.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (metamodelica::arrayLength(inComps.clone()) < nodeIdx) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    bOut
}

fn checkTpl2ForZero(mut comp: i32, mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut bIn: bool) -> Result<bool> {
    let mut bOut: bool;
    let mut b: bool;
    let mut value: metamodelica::Real;
    let mut tpl: (i32, metamodelica::Real);
    tpl = metamodelica::arrayGet(exeCosts.clone(), comp)?;
    (_, value) = tpl;
    b = realEq(value, metamodelica::OrderedFloat(0.0_f64));
    bOut = b || bIn;
    Ok(bOut)
}

pub(crate) fn convertNodeListToEdgeTuples(mut iNodeList: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<(i32, i32)>> {
    let mut oEdgeList: Arc<metamodelica::List<(i32, i32)>>;
    oEdgeList = convertNodeListToEdgeTuples0(iNodeList.clone(), (iNodeList.len() as i32), metamodelica::nil());
    oEdgeList
}

fn convertNodeListToEdgeTuples0(mut iNodeList: Arc<metamodelica::List<i32>>, mut iNodeIdx: i32, mut iEdgeList: Arc<metamodelica::List<(i32, i32)>>) -> Arc<metamodelica::List<(i32, i32)>> {
    let mut oEdgeList: Arc<metamodelica::List<(i32, i32)>>;
    let mut tmpEdgeList: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut elem: i32 = 0;
    let mut preElem: i32 = 0;
    oEdgeList = 'mc: {
        let __mc_input = iEdgeList.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                tmpEdgeList => {
                    let mut tmpEdgeList = (*tmpEdgeList).clone();
                    let mut elem: i32 = elem.clone();
                    let mut preElem: i32 = preElem.clone();
                    let true = (intGt(iNodeIdx, 1)) else { bail!("pattern mismatch") };
                    elem = (iNodeList.clone()).get(iNodeIdx)?;
                    preElem = (iNodeList.clone()).get(iNodeIdx - 1)?;
                    tmpEdgeList = metamodelica::cons((preElem, elem), tmpEdgeList.clone());
                    tmpEdgeList = convertNodeListToEdgeTuples0(iNodeList.clone(), iNodeIdx - 1, tmpEdgeList.clone());
                    Ok((tmpEdgeList.clone(), elem.clone(), preElem.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { elem = __wb0; preElem = __wb1; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iEdgeList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oEdgeList
}

fn convertSimEqToSccCosts(mut iReqTimeOpSimCode: metamodelica::Array<(i32, metamodelica::Real)>, mut iSimeqCompMapping: metamodelica::Array<i32>, mut iReqTimeOp: metamodelica::Array<metamodelica::Real>) -> Result<metamodelica::Array<metamodelica::Real>> {
    let mut oReqTimeOp: metamodelica::Array<metamodelica::Real>;
    (_, oReqTimeOp) = Array::fold(iReqTimeOpSimCode.clone(), (std::sync::Arc::new({ let __pe_b1 = iSimeqCompMapping.clone(); move |__pe_a0, __pe_a2| Ok(convertSimEqToSccCosts1(__pe_a0, __pe_b1.clone(), __pe_a2)) }) as std::sync::Arc<dyn ::std::ops::Fn((i32, metamodelica::Real), (i32, metamodelica::Array<metamodelica::Real>)) -> Result<(i32, metamodelica::Array<metamodelica::Real>)> + 'static>), (1, iReqTimeOp.clone()))?;
    Ok(oReqTimeOp)
}

fn convertSimEqToSccCosts1(mut iReqTimeOpSimCode: (i32, metamodelica::Real), mut iSimeqCompMapping: metamodelica::Array<i32>, mut iReqTimeOp: (i32, metamodelica::Array<metamodelica::Real>)) -> (i32, metamodelica::Array<metamodelica::Real>) {
    let mut oReqTimeOp: (i32, metamodelica::Array<metamodelica::Real>);
    let mut simEqCalcCount: i32 = 0;
    let mut simEqIdx: i32 = 0;
    let mut simEqCalcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut realSimEqCalcCount: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut reqTime: metamodelica::Array<metamodelica::Real> = Default::default();
    oReqTimeOp = 'mc: {
        let __mc_input = (iReqTimeOpSimCode, iReqTimeOp);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let ((mut simEqCalcCount, mut simEqCalcTime), (mut simEqIdx, mut reqTime)) = __mc_input.clone() else { bail!("nomatch") };
            let mut realSimEqCalcCount: metamodelica::Real = realSimEqCalcCount.clone();
            realSimEqCalcCount = intReal(simEqCalcCount);
            let true = (realNe(realSimEqCalcCount, metamodelica::OrderedFloat(0.0_f64))) else { bail!("pattern mismatch") };
            reqTime = convertSimEqToSccCosts2(reqTime.clone(), realDiv(simEqCalcTime, realSimEqCalcCount), simEqIdx, iSimeqCompMapping.clone());
            Ok(((simEqIdx + 1, reqTime.clone()), realSimEqCalcCount.clone()))
        })() { realSimEqCalcCount = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let ((mut simEqCalcCount, mut simEqCalcTime), (mut simEqIdx, mut reqTime)) = __mc_input.clone() else { bail!("nomatch") };
            let mut realSimEqCalcCount: metamodelica::Real = realSimEqCalcCount.clone();
            realSimEqCalcCount = intReal(simEqCalcCount);
            reqTime = convertSimEqToSccCosts2(reqTime.clone(), metamodelica::OrderedFloat(0.0_f64), simEqIdx, iSimeqCompMapping.clone());
            Ok(((simEqIdx + 1, reqTime.clone()), realSimEqCalcCount.clone()))
        })() { realSimEqCalcCount = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("convertSimEqToSccCosts1 failed!\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oReqTimeOp
}

fn convertSimEqToSccCosts2(mut iReqTime: metamodelica::Array<metamodelica::Real>, mut iSimEqCalcTime: metamodelica::Real, mut iSimEqIdx: i32, mut iSimeqCompMapping: metamodelica::Array<i32>) -> metamodelica::Array<metamodelica::Real> {
    let mut oReqTime: metamodelica::Array<metamodelica::Real>;
    let mut reqTime: metamodelica::Array<metamodelica::Real> = Default::default();
    let mut sccIdx: i32 = 0;
    oReqTime = 'mc: {
        let __mc_input = iReqTime.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let mut reqTime = __mc_input.clone() else { bail!("nomatch") };
            let mut sccIdx: i32 = sccIdx.clone();
            let true = (intGe(metamodelica::arrayLength(iSimeqCompMapping.clone()), iSimEqIdx)) else { bail!("pattern mismatch") };
            sccIdx = metamodelica::arrayGet(iSimeqCompMapping.clone(), iSimEqIdx)?;
            let true = (intGt(sccIdx, 0)) else { bail!("pattern mismatch") };
            reqTime = metamodelica::arrayUpdate(reqTime.clone(), sccIdx, iSimEqCalcTime)?;
            Ok((reqTime.clone(), sccIdx.clone()))
        })() { sccIdx = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iReqTime.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oReqTime
}

fn createCosts0(mut iNode: Arc<metamodelica::List<i32>>, mut iComps_shared: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<BackendDAE::Shared>), mut iCompMapping: metamodelica::Array<Arc<BackendDAE::EqSystem>>, mut reqTimeOp: metamodelica::Array<metamodelica::Real>, mut reqTimeCom: (i32, i32), mut iTaskGraphMeta: (i32, TaskGraphMeta)) -> Result<(i32, TaskGraphMeta)> {
    let mut oTaskGraphMeta: (i32, TaskGraphMeta);
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeRefCount: metamodelica::Array<i32>;
    let mut execCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut nodeNumber: i32;
    let mut taskGraphMeta: TaskGraphMeta;
    let mut compInformations: metamodelica::Array<ComponentInfo>;
    (nodeNumber, taskGraphMeta) = iTaskGraphMeta;
    let TaskGraphMeta { inComps: __pa0, varCompMapping: __pa1, eqCompMapping: __pa2, compParamMapping: __pa3, compNames: __pa4, compDescs: __pa5, exeCosts: __pa6, commCosts: __pa7, nodeMark: __pa8, compInformations: __pa9 } = (taskGraphMeta) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    varCompMapping = __pa1.clone();
    eqCompMapping = __pa2.clone();
    compParamMapping = __pa3.clone();
    compNames = __pa4.clone();
    compDescs = __pa5.clone();
    execCosts = __pa6.clone();
    commCosts = __pa7.clone();
    nodeRefCount = __pa8.clone();
    compInformations = __pa9.clone();
    createExecCost(iNode, iComps_shared, reqTimeOp.clone(), execCosts.clone(), iCompMapping.clone(), nodeNumber);
    oTaskGraphMeta = (nodeNumber + 1, TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: execCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeRefCount.clone(), compInformations: compInformations.clone() });
    Ok(oTaskGraphMeta)
}

fn createCosts1(mut iTuple: (i32, i32, metamodelica::Real), mut iReqTime: metamodelica::Array<(i32, metamodelica::Real)>) -> Result<metamodelica::Array<(i32, metamodelica::Real)>> {
    let mut oReqTime: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut tmpArray: metamodelica::Array<(i32, metamodelica::Real)> = Default::default();
    let mut simEqIdx: i32 = 0;
    let mut calcTimeCount: i32 = 0;
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    oReqTime = (match (iTuple, iReqTime.clone()) {
        ((0, mut __esc_calcTimeCount, mut __esc_calcTime), _) => {
            calcTimeCount = __esc_calcTimeCount.clone();
            calcTime = __esc_calcTime.clone();
            iReqTime.clone()
        },
        ((mut __esc_simEqIdx, mut __esc_calcTimeCount, mut __esc_calcTime), mut __esc_tmpArray) => {
            simEqIdx = __esc_simEqIdx.clone();
            calcTimeCount = __esc_calcTimeCount.clone();
            calcTime = __esc_calcTime.clone();
            tmpArray = __esc_tmpArray.clone();
            tmpArray = metamodelica::arrayUpdate(iReqTime.clone(), simEqIdx, (calcTimeCount, calcTime))?;
            tmpArray.clone()
        },
    });
    Ok(oReqTime)
}

fn createExecCost(mut iNodeSccs: Arc<metamodelica::List<i32>>, mut icomps_shared: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<BackendDAE::Shared>), mut iRequiredTime: metamodelica::Array<metamodelica::Real>, mut iExecCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut compMapping: metamodelica::Array<Arc<BackendDAE::EqSystem>>, mut iNodeIdx: i32) -> () {
    let () = 'mc: {
        let __mc_input = iNodeIdx;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut execCost: (i32, metamodelica::Real);
            execCost = List::fold3(iNodeSccs.clone(), (std::sync::Arc::new(createExecCost0) as std::sync::Arc<dyn ::std::ops::Fn(i32, (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<BackendDAE::Shared>), metamodelica::Array<Arc<BackendDAE::EqSystem>>, metamodelica::Array<metamodelica::Real>, (i32, metamodelica::Real)) -> Result<(i32, metamodelica::Real)> + 'static>), icomps_shared.clone(), compMapping.clone(), iRequiredTime.clone(), (0, metamodelica::OrderedFloat(0.0_f64)))?;
            metamodelica::arrayUpdate(iExecCosts.clone(), iNodeIdx, execCost.clone())?;
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

fn createExecCost0(mut sccIndex: i32, mut icomps_shared: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<BackendDAE::Shared>), mut compMapping: metamodelica::Array<Arc<BackendDAE::EqSystem>>, mut iRequiredTime: metamodelica::Array<metamodelica::Real>, mut iCosts: (i32, metamodelica::Real)) -> Result<(i32, metamodelica::Real)> {
    let mut oCosts: (i32, metamodelica::Real);
    let mut iCosts_op: i32;
    let mut iCosts_cyc: metamodelica::Real;
    let mut comp: Arc<BackendDAE::StrongComponent>;
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut syst: Arc<BackendDAE::EqSystem>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut reqTime: metamodelica::Real;
    (comps, shared) = icomps_shared;
    (iCosts_op, iCosts_cyc) = iCosts;
    comp = (comps).get(sccIndex)?;
    syst = metamodelica::arrayGet(compMapping.clone(), sccIndex)?;
    reqTime = metamodelica::arrayGet(iRequiredTime.clone(), sccIndex)?;
    oCosts = (-100 + iCosts_op, (iCosts_cyc) + (reqTime));
    Ok(oCosts)
}

fn createCommCosts(mut iCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>, mut iCurrentIndex: i32, mut iReqTimeCom: (i32, i32)) -> metamodelica::Array<Arc<metamodelica::List<Communication>>> {
    let mut oCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut tmpCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>> = Default::default();
    let mut currentCom: Communications = metamodelica::nil();
    oCosts = 'mc: {
        let __mc_input = iCosts.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let mut tmpCosts = __mc_input.clone() else { bail!("nomatch") };
            let mut currentCom: Arc<metamodelica::List<Communication>> = currentCom.clone();
            let true = (intLe(iCurrentIndex, metamodelica::arrayLength(iCosts.clone()))) else { bail!("pattern mismatch") };
            currentCom = metamodelica::arrayGet(tmpCosts.clone(), iCurrentIndex)?;
            currentCom = List::map1(currentCom.clone(), (std::sync::Arc::new(createCommCosts0) as std::sync::Arc<dyn ::std::ops::Fn(Communication, (i32, i32)) -> Result<Communication> + 'static>), iReqTimeCom)?;
            tmpCosts = metamodelica::arrayUpdate(tmpCosts.clone(), iCurrentIndex, currentCom.clone())?;
            tmpCosts = createCommCosts(tmpCosts.clone(), iCurrentIndex + 1, iReqTimeCom);
            Ok((tmpCosts.clone(), currentCom.clone()))
        })() { currentCom = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iCosts.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oCosts
}

fn createCommCosts0(mut iComm: Communication, mut iReqTimeCom: (i32, i32)) -> Result<Communication> {
    let mut oComm: Communication;
    let mut childNode: i32;
    let mut reqTimeM: i32;
    let mut reqTimeN: i32;
    let mut numberOfVars: i32;
    let mut requiredTime: metamodelica::Real;
    let mut integerVars: Arc<metamodelica::List<i32>>;
    let mut floatVars: Arc<metamodelica::List<i32>>;
    let mut booleanVars: Arc<metamodelica::List<i32>>;
    let mut stringVars: Arc<metamodelica::List<i32>>;
    let Communication { numberOfVars: __pa0, integerVars: __pa1, floatVars: __pa2, booleanVars: __pa3, stringVars: __pa4, childNode: __pa5, requiredTime: __pa6 } = (iComm) else { bail!("pattern mismatch") };
    numberOfVars = __pa0.clone();
    integerVars = __pa1.clone();
    floatVars = __pa2.clone();
    booleanVars = __pa3.clone();
    stringVars = __pa4.clone();
    childNode = __pa5.clone();
    requiredTime = __pa6.clone();
    (reqTimeM, reqTimeN) = iReqTimeCom;
    requiredTime = intReal(reqTimeN + numberOfVars * reqTimeM);
    oComm = Communication { numberOfVars: numberOfVars, integerVars: integerVars, floatVars: floatVars, booleanVars: booleanVars, stringVars: stringVars, childNode: childNode, requiredTime: requiredTime };
    Ok(oComm)
}

//---------------------------------
//  Functions to validate the graph
//---------------------------------
pub(crate) fn validateTaskGraphMeta(mut iMeta: TaskGraphMeta, mut iDae: Arc<BackendDAE::BackendDAE>) -> bool {
    let mut valid: bool;
    valid = 'mc: {
        let __mc_input = iDae.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut systComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
                    let mut graphComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
                    let mut systCompsArray: metamodelica::Array<Arc<BackendDAE::StrongComponent>>;
                    let mut systCompEqSysMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>;
                    let mut graphCompEqSysMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>;
                    let mut systCompEqSysMappingIdx: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>;
                    let mut graphCompEqSysMappingIdx: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>;
                    (systComps, systCompEqSysMapping) = getSystemComponents(iDae.clone())?;
                    systCompsArray = metamodelica::arrayFromVec(systComps.clone().into_iter().cloned().collect());
                    (graphComps, graphCompEqSysMapping) = getGraphComponents(iMeta.clone(), systCompsArray.clone(), systCompEqSysMapping.clone())?;
                    (_, _, systCompEqSysMappingIdx) = validateTaskGraphMeta0(systCompEqSysMapping.clone(), (1, systComps.clone(), metamodelica::nil()))?;
                    (_, _, graphCompEqSysMappingIdx) = validateTaskGraphMeta0(graphCompEqSysMapping.clone(), (1, graphComps.clone(), metamodelica::nil()))?;
                    let true = (validateComponents(graphCompEqSysMappingIdx.clone(), systCompEqSysMappingIdx.clone())) else { bail!("pattern mismatch") };
                    let true = (checkForDuplicates(graphCompEqSysMappingIdx.clone())?) else { bail!("pattern mismatch") };
                    let true = (checkForExecutionCosts(iMeta.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    valid
}

fn validateTaskGraphMeta0(mut iEqSysMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>, mut iCompsTpl: (i32, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>)) -> Result<(i32, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>)> {
    '__tco: loop {
        let mut currentIdx: i32 = 0;
        let mut eqSysIdx: i32 = 0;
        let mut rest: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
        let mut head: Arc<BackendDAE::StrongComponent> = Arc::new(<BackendDAE::StrongComponent as ::std::default::Default>::default());
        let mut iCompEqSysMapping: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>> = metamodelica::nil();
        let mut oCompEqSysMapping: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>> = metamodelica::nil();
        let mut tmpCompsTpl: (i32, Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>) = (0, metamodelica::nil(), metamodelica::nil());
        ::match_deref::match_deref! { match &(iCompsTpl.clone()) {
        (__esc_currentIdx, Deref @ metamodelica::List::Cons { head: __esc_head, tail: __esc_rest }, __esc_iCompEqSysMapping) => {
            currentIdx = (*__esc_currentIdx).clone();
            head = (*__esc_head).clone();
            rest = (*__esc_rest).clone();
            iCompEqSysMapping = (*__esc_iCompEqSysMapping).clone();
            (_, eqSysIdx) = metamodelica::arrayGet(iEqSysMapping.clone(), currentIdx.clone())?;
            oCompEqSysMapping = metamodelica::cons((head.clone(), eqSysIdx), iCompEqSysMapping.clone());
            { (iEqSysMapping, iCompsTpl) = (iEqSysMapping.clone(), (currentIdx.clone() + 1, rest.clone(), oCompEqSysMapping)); continue '__tco; }
        },
        _ => return Ok(iCompsTpl),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn validateComponents(mut graphComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>, mut systComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>) -> bool {
    let mut res: bool;
    let mut isEqual: bool = false;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut comp1: Arc<BackendDAE::StrongComponent> = Arc::new(<BackendDAE::StrongComponent as ::std::default::Default>::default());
    let mut comp2: Arc<BackendDAE::StrongComponent> = Arc::new(<BackendDAE::StrongComponent as ::std::default::Default>::default());
    let mut tpl1: (Arc<BackendDAE::StrongComponent>, i32) = (Arc::new(<BackendDAE::StrongComponent as ::std::default::Default>::default()), 0);
    let mut tpl2: (Arc<BackendDAE::StrongComponent>, i32) = (Arc::new(<BackendDAE::StrongComponent as ::std::default::Default>::default()), 0);
    let mut sortedGraphComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>> = metamodelica::nil();
    let mut sortedSystComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>> = metamodelica::nil();
    res = 'mc: {
        let __mc_input = systComps.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut comp1: Arc<BackendDAE::StrongComponent> = comp1.clone();
                    let mut comp2: Arc<BackendDAE::StrongComponent> = comp2.clone();
                    let mut i1: i32 = i1.clone();
                    let mut i2: i32 = i2.clone();
                    let mut isEqual: bool = isEqual.clone();
                    let mut sortedGraphComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>> = sortedGraphComps.clone();
                    let mut sortedSystComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>> = sortedSystComps.clone();
                    let mut tpl1: (Arc<BackendDAE::StrongComponent>, i32) = tpl1.clone();
                    let mut tpl2: (Arc<BackendDAE::StrongComponent>, i32) = tpl2.clone();
                    sortedGraphComps = List::sort(graphComps.clone(), (std::sync::Arc::new(compareComponents) as std::sync::Arc<dyn ::std::ops::Fn((Arc<BackendDAE::StrongComponent>, i32), (Arc<BackendDAE::StrongComponent>, i32)) -> Result<bool> + 'static>))?;
                    sortedSystComps = List::sort(systComps.clone(), (std::sync::Arc::new(compareComponents) as std::sync::Arc<dyn ::std::ops::Fn((Arc<BackendDAE::StrongComponent>, i32), (Arc<BackendDAE::StrongComponent>, i32)) -> Result<bool> + 'static>))?;
                    if intNe((sortedSystComps.clone().len() as i32), (sortedGraphComps.clone().len() as i32)) {
                        metamodelica::print((literal!("the graph and the system have a difference number of components.\n")).clone());
                    }
                    isEqual = true;
                    while isEqual && !(sortedGraphComps.clone().is_empty()) {
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(sortedGraphComps.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        tpl1 = __pa0.clone();
                        sortedGraphComps = __pa1.clone();
                        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(sortedSystComps.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        tpl2 = __pa2.clone();
                        sortedSystComps = __pa3.clone();
                        (comp1, i1) = tpl1.clone();
                        (comp2, i2) = tpl2.clone();
                        if componentsEqual(tpl1.clone(), tpl2.clone())? {
                            isEqual = true;
                        } else {
                            isEqual = false;
                            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("comp ")); __mm_s.push_str(&*intString(i1)); __mm_s.push_str(&*BackendDump::printComponent(comp1.clone(), None)?); __mm_s.push_str(&*literal!(" is not equal to ")); __mm_s.push_str(&*literal!("comp")); __mm_s.push_str(&*intString(i2)); __mm_s.push_str(&*BackendDump::printComponent(comp2.clone(), None)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        }
                    }
                    Ok((true, comp1.clone(), comp2.clone(), i1.clone(), i2.clone(), isEqual.clone(), sortedGraphComps.clone(), sortedSystComps.clone(), tpl1.clone(), tpl2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { comp1 = __wb0; comp2 = __wb1; i1 = __wb2; i2 = __wb3; isEqual = __wb4; sortedGraphComps = __wb5; sortedSystComps = __wb6; tpl1 = __wb7; tpl2 = __wb8; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("Different components in graph and system\n")).clone());
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    res
}

fn checkForDuplicates(mut iComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>) -> Result<bool> {
    let mut res: bool;
    let mut sortedComps: Arc<metamodelica::List<(Arc<BackendDAE::StrongComponent>, i32)>>;
    sortedComps = List::sort(iComps, (std::sync::Arc::new(compareComponents) as std::sync::Arc<dyn ::std::ops::Fn((Arc<BackendDAE::StrongComponent>, i32), (Arc<BackendDAE::StrongComponent>, i32)) -> Result<bool> + 'static>))?;
    (res, _) = List::fold(sortedComps, (std::sync::Arc::new(fnptr!(checkForDuplicates0, (Arc<BackendDAE::StrongComponent>, i32), (bool, Option<(Arc<BackendDAE::StrongComponent>, i32)>))) as std::sync::Arc<dyn ::std::ops::Fn((Arc<BackendDAE::StrongComponent>, i32), (bool, Option<(Arc<BackendDAE::StrongComponent>, i32)>)) -> Result<(bool, Option<(Arc<BackendDAE::StrongComponent>, i32)>)> + 'static>), (true, None))?;
    Ok(res)
}

fn checkForDuplicates0(mut currentComp_idx: (Arc<BackendDAE::StrongComponent>, i32), mut iLastComp: (bool, Option<(Arc<BackendDAE::StrongComponent>, i32)>)) -> (bool, Option<(Arc<BackendDAE::StrongComponent>, i32)>) {
    let mut oLastComp: (bool, Option<(Arc<BackendDAE::StrongComponent>, i32)>);
    let mut lastComp: Arc<BackendDAE::StrongComponent> = Arc::new(<BackendDAE::StrongComponent as ::std::default::Default>::default());
    let mut currentComp: Arc<BackendDAE::StrongComponent> = Arc::new(<BackendDAE::StrongComponent as ::std::default::Default>::default());
    let mut lastComp_idx: (Arc<BackendDAE::StrongComponent>, i32) = (Arc::new(<BackendDAE::StrongComponent as ::std::default::Default>::default()), 0);
    let mut idxLast: i32 = 0;
    let mut idxCurrent: i32 = 0;
    oLastComp = 'mc: {
        let __mc_input = (currentComp_idx.clone(), iLastComp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (false, _)) => {
                    Ok((false, Some(currentComp_idx.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (_, None)) => {
                    Ok((true, Some(currentComp_idx.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((currentComp, idxCurrent), (_, Some(lastComp_idx @ (lastComp, idxLast)))) => {
                    let true = (componentsEqual(currentComp_idx.clone(), lastComp_idx.clone())?) else { bail!("pattern mismatch") };
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Component duplicate detected: current: ")); __mm_s.push_str(&*BackendDump::printComponent(currentComp.clone(), None)?); __mm_s.push_str(&*literal!(" (eqSystem ")); __mm_s.push_str(&*intString(idxCurrent.clone())); __mm_s.push_str(&*literal!(") last ")); __mm_s.push_str(&*BackendDump::printComponent(lastComp.clone(), None)?); __mm_s.push_str(&*literal!(" (eqSystem ")); __mm_s.push_str(&*intString(idxLast.clone())); __mm_s.push_str(&*literal!(").\n")); ArcStr::from(__mm_s) }).clone());
                    Ok((false, Some(currentComp_idx.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((true, Some(currentComp_idx.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oLastComp
}

fn getGraphComponents(mut iTaskGraphMeta: TaskGraphMeta, mut iSystComps: metamodelica::Array<Arc<BackendDAE::StrongComponent>>, mut iCompEqSysMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>)> {
    let mut oComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut oCompEqGraphMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>;
    let mut tmpComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut tmpMapping: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeMarks: metamodelica::Array<i32>;
    tmpComps = metamodelica::nil();
    tmpMapping = metamodelica::nil();
    let TaskGraphMeta { inComps: __pa0, nodeMark: __pa1, .. } = (iTaskGraphMeta) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    nodeMarks = __pa1.clone();
    (tmpComps, tmpMapping) = Array::fold(inComps.clone(), (std::sync::Arc::new({ let __pe_b1 = iSystComps.clone(); let __pe_b2 = iCompEqSysMapping.clone(); move |__pe_a0, __pe_a3| getGraphComponents0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>)> + 'static>), (tmpComps, tmpMapping))?;
    let (_, (__pa2, __pa3)) = Array::fold(nodeMarks.clone(), (std::sync::Arc::new({ let __pe_b1 = iSystComps.clone(); let __pe_b2 = iCompEqSysMapping.clone(); move |__pe_a0, __pe_a3| getGraphComponents2(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(i32, (i32, (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>))) -> Result<(i32, (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>))> + 'static>), (1, (tmpComps, tmpMapping)))?;
    tmpComps = __pa2.clone();
    tmpMapping = __pa3.clone();
    oComps = tmpComps;
    oCompEqGraphMapping = metamodelica::arrayFromVec(tmpMapping.into_iter().cloned().collect());
    Ok((oComps, oCompEqGraphMapping))
}

fn getGraphComponents0(mut inComp: Arc<metamodelica::List<i32>>, mut systComps: metamodelica::Array<Arc<BackendDAE::StrongComponent>>, mut iCompEqSysMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>, mut iNodeComps_Mapping: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>)> {
    let mut oNodeComps_Mapping: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>);
    let mut iNodeComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut tmpNodeComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut iCompsMapping: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>;
    let mut tmpCompsMapping: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>;
    (iNodeComps, iCompsMapping) = iNodeComps_Mapping;
    (tmpNodeComps, tmpCompsMapping) = List::fold2(inComp, (std::sync::Arc::new(getGraphComponents1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<BackendDAE::StrongComponent>>, metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>, (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>)> + 'static>), systComps.clone(), iCompEqSysMapping.clone(), (metamodelica::nil(), metamodelica::nil()))?;
    tmpNodeComps = listAppend(iNodeComps, tmpNodeComps);
    tmpCompsMapping = listAppend(iCompsMapping, tmpCompsMapping);
    oNodeComps_Mapping = (tmpNodeComps, tmpCompsMapping);
    Ok(oNodeComps_Mapping)
}

fn getGraphComponents1(mut compIdx: i32, mut systComps: metamodelica::Array<Arc<BackendDAE::StrongComponent>>, mut iCompEqSysMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>, mut iNodeComps_Mapping: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>)) -> Result<(Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>)> {
    let mut oNodeComps_Mapping: (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>);
    let mut comp: Arc<BackendDAE::StrongComponent>;
    let mut eqSyst: (Arc<BackendDAE::EqSystem>, i32);
    let mut tmpComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    let mut tmpSysts: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>;
    (tmpComps, tmpSysts) = iNodeComps_Mapping;
    comp = metamodelica::arrayGet(systComps.clone(), compIdx)?;
    eqSyst = metamodelica::arrayGet(iCompEqSysMapping.clone(), compIdx)?;
    tmpComps = metamodelica::cons(comp, tmpComps);
    tmpSysts = metamodelica::cons(eqSyst, tmpSysts);
    oNodeComps_Mapping = (tmpComps, tmpSysts);
    Ok(oNodeComps_Mapping)
}

fn getGraphComponents2(mut nodeMark: i32, mut systComps: metamodelica::Array<Arc<BackendDAE::StrongComponent>>, mut iCompEqSysMapping: metamodelica::Array<(Arc<BackendDAE::EqSystem>, i32)>, mut iNodeComps_Mapping: (i32, (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>))) -> Result<(i32, (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>))> {
    let mut oNodeComps_Mapping: (i32, (Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>>));
    let mut nodeIdx: i32 = 0;
    let mut comp: Arc<BackendDAE::StrongComponent> = Arc::new(<BackendDAE::StrongComponent as ::std::default::Default>::default());
    let mut eqSyst: (Arc<BackendDAE::EqSystem>, i32) = (Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default()), 0);
    let mut comps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>> = metamodelica::nil();
    let mut eqSysts: Arc<metamodelica::List<(Arc<BackendDAE::EqSystem>, i32)>> = metamodelica::nil();
    oNodeComps_Mapping = 'mc: {
        let __mc_input = iNodeComps_Mapping;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (nodeIdx, (comps, eqSysts)) => {
                    let true = (intGe(nodeMark, 0)) else { bail!("pattern mismatch") };
                    Ok((nodeIdx.clone() + 1, (comps.clone(), eqSysts.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (nodeIdx, (comps, eqSysts)) => {
                    let true = (intEq(nodeMark, -2)) else { bail!("pattern mismatch") };
                    Ok((nodeIdx.clone() + 1, (comps.clone(), eqSysts.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (nodeIdx, (comps, eqSysts)) => {
                    let mut comps = (*comps).clone();
                    let mut eqSysts = (*eqSysts).clone();
                    let mut comp: Arc<BackendDAE::StrongComponent> = comp.clone();
                    let mut eqSyst: (Arc<BackendDAE::EqSystem>, i32) = eqSyst.clone();
                    comp = metamodelica::arrayGet(systComps.clone(), nodeIdx.clone())?;
                    eqSyst = metamodelica::arrayGet(iCompEqSysMapping.clone(), nodeIdx.clone())?;
                    comps = metamodelica::cons(comp.clone(), comps.clone());
                    eqSysts = metamodelica::cons(eqSyst.clone(), eqSysts.clone());
                    Ok(((nodeIdx.clone() + 1, (comps.clone(), eqSysts.clone())), comp.clone(), eqSyst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { comp = __wb0; eqSyst = __wb1; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oNodeComps_Mapping)
}

fn componentsEqual(mut iComp1: (Arc<BackendDAE::StrongComponent>, i32), mut iComp2: (Arc<BackendDAE::StrongComponent>, i32)) -> Result<bool> {
    let mut res: bool;
    let mut comp1Str: ArcStr;
    let mut comp2Str: ArcStr;
    let mut comp1Idx: i32;
    let mut comp2Idx: i32;
    let mut comp1: Arc<BackendDAE::StrongComponent>;
    let mut comp2: Arc<BackendDAE::StrongComponent>;
    (comp1, comp1Idx) = iComp1;
    (comp2, comp2Idx) = iComp2;
    comp1Str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BackendDump::printComponent(comp1, None)?); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(comp1Idx)); ArcStr::from(__mm_s) }).clone();
    comp2Str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BackendDump::printComponent(comp2, None)?); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(comp2Idx)); ArcStr::from(__mm_s) }).clone();
    if intNe(((comp1Str.clone()).clone().len() as i32), ((comp2Str.clone()).clone().len() as i32)) {
        res = false;
    } else {
        res = intEq(System::strncmp((comp1Str.clone()).clone(), (comp2Str).clone(), ((comp1Str).clone().len() as i32)), 0);
    }
    Ok(res)
}

fn compareComponents(mut iComp1: (Arc<BackendDAE::StrongComponent>, i32), mut iComp2: (Arc<BackendDAE::StrongComponent>, i32)) -> Result<bool> {
    let mut res: bool;
    let mut comp1Str: ArcStr;
    let mut comp2Str: ArcStr;
    let mut minLength: i32;
    let mut compRes: i32;
    let mut comp1Idx: i32;
    let mut comp2Idx: i32;
    let mut comp1: Arc<BackendDAE::StrongComponent>;
    let mut comp2: Arc<BackendDAE::StrongComponent>;
    if componentsEqual(iComp1.clone(), iComp2.clone())? {
        res = false;
    } else {
        (comp1, comp1Idx) = iComp1;
        (comp2, comp2Idx) = iComp2;
        comp1Str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BackendDump::printComponent(comp1, None)?); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(comp1Idx)); ArcStr::from(__mm_s) }).clone();
        comp2Str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*BackendDump::printComponent(comp2, None)?); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(comp2Idx)); ArcStr::from(__mm_s) }).clone();
        minLength = intMin(((comp1Str.clone()).clone().len() as i32), ((comp2Str.clone()).clone().len() as i32));
        compRes = System::strncmp((comp1Str.clone()).clone(), (comp2Str.clone()).clone(), minLength);
        if intEq(compRes, 0) {
            res = intLt(((comp1Str).clone().len() as i32), ((comp2Str).clone().len() as i32));
        } else {
            res = intLt(compRes, 0);
        }
    }
    Ok(res)
}

//------------------------------------
//  Evaluation and analysing functions
//------------------------------------
pub(crate) fn getCriticalPaths(mut graphIn: TaskGraph, mut graphDataIn: TaskGraphMeta) -> ((Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Real), (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Real)) {
    let mut criticalPathOut: (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Real);
    let mut criticalPathOutWoC: (Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Real);
    (criticalPathOut, criticalPathOutWoC) = 'mc: {
        let __mc_input = graphDataIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let TaskGraphMeta { .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut rootNodes: Arc<metamodelica::List<i32>>;
            let mut cpWCpaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut CpWoCpaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut cpWCcosts: metamodelica::Real;
            let mut cpWoCcosts: metamodelica::Real;
            let true = (metamodelica::arrayLength(graphIn.clone()) != 0) else { bail!("pattern mismatch") };
            rootNodes = getRootNodes(graphIn.clone())?;
            (cpWCpaths, cpWCcosts) = getCriticalPath(graphIn.clone(), graphDataIn.clone(), rootNodes.clone(), true)?;
            (CpWoCpaths, cpWoCcosts) = getCriticalPath(graphIn.clone(), graphDataIn.clone(), rootNodes.clone(), false)?;
            cpWCcosts = roundReal(cpWCcosts.clone(), 2);
            cpWoCcosts = roundReal(cpWoCcosts.clone(), 2);
            Ok(((cpWCpaths.clone(), cpWCcosts.clone()), (CpWoCpaths.clone(), cpWoCcosts.clone())))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (metamodelica::arrayLength(graphIn.clone()) == 0) else { bail!("pattern mismatch") };
            Ok(((list![metamodelica::nil()], metamodelica::OrderedFloat(0.0_f64)), (list![metamodelica::nil()], metamodelica::OrderedFloat(0.0_f64))))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("getCriticalPaths failed!\n")).clone());
            Ok(((list![metamodelica::nil()], metamodelica::OrderedFloat(0.0_f64)), (list![metamodelica::nil()], metamodelica::OrderedFloat(0.0_f64))))
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (criticalPathOut, criticalPathOutWoC)
}

fn getCriticalPath(mut iGraph: TaskGraph, mut iGraphData: TaskGraphMeta, mut iRootNodes: Arc<metamodelica::List<i32>>, mut iHandleCommCosts: bool) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Real)> {
    let mut oCriticalPathsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut oCpCosts: metamodelica::Real;
    let mut nodeCriticalPaths: metamodelica::Array<(metamodelica::Real, Arc<metamodelica::List<i32>>)>;
    let mut criticalPaths: Arc<metamodelica::List<(metamodelica::Real, Arc<metamodelica::List<i32>>)>>;
    let mut criticalPathIdx: i32;
    let mut criticalPath: Arc<metamodelica::List<i32>>;
    nodeCriticalPaths = arrayCreate(metamodelica::arrayLength(iGraph.clone()), (metamodelica::OrderedFloat(-1.0_f64), metamodelica::nil()));
    criticalPaths = List::map4(iRootNodes, (std::sync::Arc::new(getCriticalPath1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, TaskGraphMeta, bool, metamodelica::Array<(metamodelica::Real, Arc<metamodelica::List<i32>>)>) -> Result<(metamodelica::Real, Arc<metamodelica::List<i32>>)> + 'static>), iGraph.clone(), iGraphData, iHandleCommCosts, nodeCriticalPaths.clone())?;
    criticalPathIdx = getCriticalPath2(criticalPaths.clone(), 1, metamodelica::OrderedFloat(-1.0_f64), -1);
    (oCpCosts, criticalPath) = (criticalPaths).get(criticalPathIdx)?;
    oCriticalPathsOut = list![criticalPath];
    Ok((oCriticalPathsOut, oCpCosts))
}

fn getCriticalPath1(mut iNode: i32, mut iGraph: TaskGraph, mut iGraphData: TaskGraphMeta, mut iHandleCommCosts: bool, mut iNodeCriticalPaths: metamodelica::Array<(metamodelica::Real, Arc<metamodelica::List<i32>>)>) -> Result<(metamodelica::Real, Arc<metamodelica::List<i32>>)> {
    let mut criticalPathOut: (metamodelica::Real, Arc<metamodelica::List<i32>>);
    let mut cpCalcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut commTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut criticalPathIdx: i32 = 0;
    let mut commCost: Communication = <Communication as ::std::default::Default>::default();
    let mut childNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut criticalPathChild: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut criticalPath: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeComps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut criticalPaths: Arc<metamodelica::List<(metamodelica::Real, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)> = Default::default();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    criticalPathOut = 'mc: {
        let __mc_input = iGraphData.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let TaskGraphMeta { inComps: mut inComps, exeCosts: mut exeCosts, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut cpCalcTime: metamodelica::Real = cpCalcTime.clone();
            let mut criticalPath: Arc<metamodelica::List<i32>> = criticalPath.clone();
            (cpCalcTime, criticalPath) = metamodelica::arrayGet(iNodeCriticalPaths.clone(), iNode)?;
            let true = (realGe(cpCalcTime, metamodelica::OrderedFloat(0.0_f64))) else { bail!("pattern mismatch") };
            Ok(((cpCalcTime, criticalPath.clone()), cpCalcTime.clone(), criticalPath.clone()))
        })() { cpCalcTime = __wb0; criticalPath = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9)) = (|| -> Result<_> {
            let TaskGraphMeta { inComps: mut inComps, exeCosts: mut exeCosts, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut calcTime: metamodelica::Real = calcTime.clone();
            let mut childNodes: Arc<metamodelica::List<i32>> = childNodes.clone();
            let mut commCost: Communication = commCost.clone();
            let mut commTime: metamodelica::Real = commTime.clone();
            let mut cpCalcTime: metamodelica::Real = cpCalcTime.clone();
            let mut criticalPath: Arc<metamodelica::List<i32>> = criticalPath.clone();
            let mut criticalPathChild: Arc<metamodelica::List<i32>> = criticalPathChild.clone();
            let mut criticalPathIdx: i32 = criticalPathIdx.clone();
            let mut criticalPaths: Arc<metamodelica::List<(metamodelica::Real, Arc<metamodelica::List<i32>>)>> = criticalPaths.clone();
            let mut nodeComps: Arc<metamodelica::List<i32>> = nodeComps.clone();
            childNodes = metamodelica::arrayGet(iGraph.clone(), iNode)?;
            let false = (childNodes.clone().is_empty()) else { bail!("pattern mismatch") };
            criticalPaths = List::map4(childNodes.clone(), (std::sync::Arc::new(getCriticalPath1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, TaskGraphMeta, bool, metamodelica::Array<(metamodelica::Real, Arc<metamodelica::List<i32>>)>) -> Result<(metamodelica::Real, Arc<metamodelica::List<i32>>)> + 'static>), iGraph.clone(), iGraphData.clone(), iHandleCommCosts, iNodeCriticalPaths.clone())?;
            criticalPathIdx = getCriticalPath2(criticalPaths.clone(), 1, metamodelica::OrderedFloat(-1.0_f64), -1);
            (cpCalcTime, criticalPathChild) = (criticalPaths.clone()).get(criticalPathIdx)?;
            criticalPath = metamodelica::cons(iNode, criticalPathChild.clone());
            commCost = if (iHandleCommCosts) {getCommCostBetweenNodes(iNode, listHead(criticalPathChild.clone())?, iGraphData.clone())?} else {Communication { numberOfVars: 0, integerVars: metamodelica::nil(), floatVars: metamodelica::nil(), booleanVars: metamodelica::nil(), stringVars: metamodelica::nil(), childNode: -1, requiredTime: metamodelica::OrderedFloat(0.0_f64) }};
            nodeComps = metamodelica::arrayGet(inComps.clone(), iNode)?;
            calcTime = addUpExeCostsForNode(nodeComps.clone(), exeCosts.clone(), metamodelica::OrderedFloat(0.0_f64))?;
            calcTime = (cpCalcTime) + (calcTime);
            let Communication { requiredTime: __pa0, .. } = (commCost.clone()) else { bail!("pattern mismatch") };
            commTime = __pa0.clone();
            calcTime = (calcTime) + (commTime);
            metamodelica::arrayUpdate(iNodeCriticalPaths.clone(), iNode, (calcTime, criticalPath.clone()))?;
            Ok(((calcTime, criticalPath.clone()), calcTime.clone(), childNodes.clone(), commCost.clone(), commTime.clone(), cpCalcTime.clone(), criticalPath.clone(), criticalPathChild.clone(), criticalPathIdx.clone(), criticalPaths.clone(), nodeComps.clone()))
        })() { calcTime = __wb0; childNodes = __wb1; commCost = __wb2; commTime = __wb3; cpCalcTime = __wb4; criticalPath = __wb5; criticalPathChild = __wb6; criticalPathIdx = __wb7; criticalPaths = __wb8; nodeComps = __wb9; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            let TaskGraphMeta { inComps: mut inComps, exeCosts: mut exeCosts, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut calcTime: metamodelica::Real = calcTime.clone();
            let mut childNodes: Arc<metamodelica::List<i32>> = childNodes.clone();
            let mut criticalPath: Arc<metamodelica::List<i32>> = criticalPath.clone();
            let mut nodeComps: Arc<metamodelica::List<i32>> = nodeComps.clone();
            childNodes = metamodelica::arrayGet(iGraph.clone(), iNode)?;
            let true = (childNodes.clone().is_empty()) else { bail!("pattern mismatch") };
            criticalPath = metamodelica::cons(iNode, metamodelica::nil());
            nodeComps = metamodelica::arrayGet(inComps.clone(), iNode)?;
            calcTime = addUpExeCostsForNode(nodeComps.clone(), exeCosts.clone(), metamodelica::OrderedFloat(0.0_f64))?;
            metamodelica::arrayUpdate(iNodeCriticalPaths.clone(), iNode, (calcTime, criticalPath.clone()))?;
            Ok(((calcTime, criticalPath.clone()), calcTime.clone(), childNodes.clone(), criticalPath.clone(), nodeComps.clone()))
        })() { calcTime = __wb0; childNodes = __wb1; criticalPath = __wb2; nodeComps = __wb3; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("HpcOmTaskGraph.getCriticalPath_1 failed\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(criticalPathOut)
}

fn getCriticalPath2(mut iCriticalPaths: Arc<metamodelica::List<(metamodelica::Real, Arc<metamodelica::List<i32>>)>>, mut iListIdx: i32, mut iLongestPath: metamodelica::Real, mut iLongestPathIndex: i32) -> i32 {
    let mut oLongestPathIndex: i32;
    let mut cpCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut criticalPath: Arc<metamodelica::List<i32>>;
    let mut rest: Arc<metamodelica::List<(metamodelica::Real, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    oLongestPathIndex = 'mc: {
        let __mc_input = iCriticalPaths;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (cpCost, criticalPath), tail: rest } => {
                    let true = (realGt(cpCost.clone(), iLongestPath)) else { bail!("pattern mismatch") };
                    Ok(getCriticalPath2(rest.clone(), iListIdx + 1, cpCost.clone(), iListIdx))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (cpCost, criticalPath), tail: rest } => {
                    Ok(getCriticalPath2(rest.clone(), iListIdx + 1, iLongestPath, iLongestPathIndex))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iLongestPathIndex)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oLongestPathIndex
}

fn addUpExeCostsForNode(mut iNodeComps: Arc<metamodelica::List<i32>>, mut iExeCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut iExeCost: metamodelica::Real) -> Result<metamodelica::Real> {
    '__tco: loop {
        let mut head: i32 = 0;
        let mut rest: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut cost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        ::match_deref::match_deref! { match &(iNodeComps) {
        Deref @ metamodelica::List::Cons { head: __esc_head, tail: __esc_rest } => {
            head = (*__esc_head).clone();
            rest = (*__esc_rest).clone();
            (_, cost) = metamodelica::arrayGet(iExeCosts.clone(), head.clone())?;
            cost = (cost) + (iExeCost);
            { (iNodeComps, iExeCosts, iExeCost) = (rest.clone(), iExeCosts.clone(), cost); continue '__tco; }
        },
        _ => return Ok(iExeCost),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn gatherParallelSets(mut nodeInfo: metamodelica::Array<(i32, metamodelica::Real, i32)>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut parallelSetsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut numLevels: i32;
    numLevels = Array::fold(nodeInfo.clone(), (std::sync::Arc::new(fnptr!(numberOfLevels, (i32, metamodelica::Real, i32), i32)) as std::sync::Arc<dyn ::std::ops::Fn((i32, metamodelica::Real, i32), i32) -> Result<i32> + 'static>), 0)?;
    parallelSetsOut = List::fold1(List::intRange(metamodelica::arrayLength(nodeInfo.clone())), (std::sync::Arc::new(gatherParallelSets1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<(i32, metamodelica::Real, i32)>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> + 'static>), nodeInfo.clone(), List::fill(metamodelica::nil(), numLevels))?;
    Ok(parallelSetsOut)
}

fn numberOfLevels(mut nodeInfoEntry: (i32, metamodelica::Real, i32), mut numLevelsIn: i32) -> i32 {
    let mut numLevelsOut: i32;
    let mut levelIn: i32;
    (levelIn, _, _) = nodeInfoEntry;
    numLevelsOut = intMax(levelIn, numLevelsIn);
    numLevelsOut
}

fn gatherParallelSets1(mut idx: i32, mut nodeInfo: metamodelica::Array<(i32, metamodelica::Real, i32)>, mut parallelSetIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut parallelSetOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut level: i32;
    let mut pSet: Arc<metamodelica::List<i32>>;
    (level, _, _) = metamodelica::arrayGet(nodeInfo.clone(), idx)?;
    pSet = (parallelSetIn.clone()).get(level)?;
    pSet = metamodelica::cons(idx, pSet);
    parallelSetOut = List::replaceAt(pSet, level, parallelSetIn)?;
    Ok(parallelSetOut)
}

fn getCostsForNode(mut parentNode: i32, mut childNode: i32, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Real> {
    let mut costsOut: metamodelica::Real;
    costsOut = 'mc: {
        let __mc_input = parentNode;
        if let Ok(__v) = (|| -> Result<_> {
            let 0 = __mc_input.clone() else { bail!("nomatch") };
            let mut costs: metamodelica::Real;
            let mut primalChild: i32;
            let mut primalChildLst: Arc<metamodelica::List<i32>>;
            primalChildLst = metamodelica::arrayGet(inComps.clone(), childNode)?;
            let true = ((primalChildLst.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
            primalChild = (primalChildLst.clone()).get(1)?;
            (_, costs) = metamodelica::arrayGet(exeCosts.clone(), primalChild.clone())?;
            Ok(costs.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let 0 = __mc_input.clone() else { bail!("nomatch") };
            let mut costs: metamodelica::Real;
            let mut primalChildLst: Arc<metamodelica::List<i32>>;
            primalChildLst = metamodelica::arrayGet(inComps.clone(), childNode)?;
            let true = ((primalChildLst.clone().len() as i32) > 1) else { bail!("pattern mismatch") };
            (primalChildLst.clone()).get(1)?;
            costs = getCostsForContractedNodes(primalChildLst.clone(), exeCosts.clone())?;
            Ok(costs.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut costs: metamodelica::Real;
            let mut commCost: metamodelica::Real;
            let mut primalChild: i32;
            let mut primalParent: i32;
            let mut primalChildLst: Arc<metamodelica::List<i32>>;
            let mut primalParentLst: Arc<metamodelica::List<i32>>;
            primalChildLst = metamodelica::arrayGet(inComps.clone(), childNode)?;
            primalParentLst = metamodelica::arrayGet(inComps.clone(), parentNode)?;
            let true = ((primalChildLst.clone().len() as i32) == 1) else { bail!("pattern mismatch") };
            primalChild = (primalChildLst.clone()).get(1)?;
            primalParent = (primalParentLst.clone()).get(1)?;
            (_, costs) = metamodelica::arrayGet(exeCosts.clone(), primalChild.clone())?;
            let Communication { requiredTime: __pa0, .. } = (getCommunicationCost(primalChild.clone(), primalParent.clone(), commCosts.clone())?) else { bail!("pattern mismatch") };
            commCost = __pa0.clone();
            costs = costs.clone() + commCost.clone();
            Ok(costs.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut costs: metamodelica::Real;
            let mut primalChildLst: Arc<metamodelica::List<i32>>;
            primalChildLst = metamodelica::arrayGet(inComps.clone(), childNode)?;
            metamodelica::arrayGet(inComps.clone(), parentNode)?;
            let true = ((primalChildLst.clone().len() as i32) > 1) else { bail!("pattern mismatch") };
            costs = getCostsForContractedNodes(primalChildLst.clone(), exeCosts.clone())?;
            Ok(costs.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("getCostsForNode failed! \n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(costsOut)
}

pub(crate) fn getCostsForContractedNodes(mut nodeList: Arc<metamodelica::List<i32>>, mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>) -> Result<metamodelica::Real> {
    let mut costsOut: metamodelica::Real;
    costsOut = List::fold1(nodeList, (std::sync::Arc::new(getCostsForContractedNodes1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<(i32, metamodelica::Real)>, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), exeCosts.clone(), metamodelica::OrderedFloat(0.0_f64))?;
    Ok(costsOut)
}

fn getCostsForContractedNodes1(mut node: i32, mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>, mut costsIn: metamodelica::Real) -> Result<metamodelica::Real> {
    let mut costsOut: metamodelica::Real;
    let mut exeCost: metamodelica::Real;
    (_, exeCost) = metamodelica::arrayGet(exeCosts.clone(), node)?;
    costsOut = (costsIn) + (exeCost);
    Ok(costsOut)
}

fn getNodeCoords(mut parallelSets: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut graphIn: TaskGraph) -> Result<metamodelica::Array<(i32, i32)>> {
    let mut nodeCoordsOut: metamodelica::Array<(i32, i32)>;
    let mut nodeCoords: metamodelica::Array<(i32, i32)>;
    let mut size: i32;
    size = metamodelica::arrayLength(graphIn.clone());
    nodeCoords = arrayCreate(size, (0, 0));
    nodeCoords = List::fold1(List::intRange(size), (std::sync::Arc::new(getYCoordForNode) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, metamodelica::Array<(i32, i32)>) -> Result<metamodelica::Array<(i32, i32)>> + 'static>), parallelSets, nodeCoords.clone())?;
    nodeCoordsOut = nodeCoords.clone();
    Ok(nodeCoordsOut)
}

fn getYCoordForNode(mut compIdx: i32, mut parallelSets: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut nodeCoordsIn: metamodelica::Array<(i32, i32)>) -> Result<metamodelica::Array<(i32, i32)>> {
    let mut nodeCoordsOut: metamodelica::Array<(i32, i32)>;
    let mut parallelSetIdx: i32;
    let mut xCoord: i32;
    let mut yCoord: i32;
    let mut coords: (i32, i32);
    parallelSetIdx = getParallelSetForComp(compIdx, 1, parallelSets)?;
    (xCoord, yCoord) = metamodelica::arrayGet(nodeCoordsIn.clone(), compIdx)?;
    coords = (xCoord, parallelSetIdx);
    nodeCoordsOut = metamodelica::arrayUpdate(nodeCoordsIn.clone(), compIdx, coords)?;
    Ok(nodeCoordsOut)
}

fn getParallelSetForComp(mut compIn: i32, mut setIdx: i32, mut parallelSets: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<i32> {
    let mut parallelSetOut: i32;
    parallelSetOut = 'mc: {
        let __mc_input = parallelSets.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut parallelSet: Arc<metamodelica::List<i32>>;
                    let true = (setIdx <= (parallelSets.clone().len() as i32)) else { bail!("pattern mismatch") };
                    parallelSet = (parallelSets.clone()).get(setIdx)?;
                    let true = (List::isMemberOnTrue(compIn, parallelSet.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    Ok(setIdx)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut parallelSet: Arc<metamodelica::List<i32>>;
                    let mut parallelSetTmp: i32;
                    let true = (setIdx <= (parallelSets.clone().len() as i32)) else { bail!("pattern mismatch") };
                    parallelSet = (parallelSets.clone()).get(setIdx)?;
                    let false = (List::isMemberOnTrue(compIn, parallelSet.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    parallelSetTmp = getParallelSetForComp(compIn, setIdx + 1, parallelSets.clone())?;
                    Ok(parallelSetTmp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("getParallelSetForComp failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(parallelSetOut)
}

fn setLevelInNodeMark(mut nodeIdx: i32, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut nodeCoords: metamodelica::Array<(i32, i32)>, mut nodeMarkIn: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut nodeMarkOut: metamodelica::Array<i32>;
    nodeMarkOut = 'mc: {
        let __mc_input = nodeMarkIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut components: Arc<metamodelica::List<i32>>;
            let mut primalComp: i32;
            let mut nodeMarkEntry: i32;
            nodeMarkEntry = metamodelica::arrayGet(nodeMarkIn.clone(), nodeIdx)?;
            components = metamodelica::arrayGet(inComps.clone(), nodeIdx)?;
            primalComp = List::last(components.clone())?;
            nodeMarkEntry = metamodelica::arrayGet(nodeMarkIn.clone(), primalComp.clone())?;
            let true = (intEq(-1, nodeMarkEntry.clone())) else { bail!("pattern mismatch") };
            Ok(nodeMarkIn.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nodeMarkTmp: metamodelica::Array<i32>;
            let mut components: Arc<metamodelica::List<i32>>;
            let mut primalComp: i32;
            let mut nodeMarkEntry: i32;
            let mut yCoord: i32;
            nodeMarkEntry = metamodelica::arrayGet(nodeMarkIn.clone(), nodeIdx)?;
            components = metamodelica::arrayGet(inComps.clone(), nodeIdx)?;
            primalComp = List::last(components.clone())?;
            nodeMarkEntry = metamodelica::arrayGet(nodeMarkIn.clone(), primalComp.clone())?;
            let false = (intEq(-1, nodeMarkEntry.clone())) else { bail!("pattern mismatch") };
            (_, yCoord) = metamodelica::arrayGet(nodeCoords.clone(), nodeIdx)?;
            nodeMarkTmp = metamodelica::arrayUpdate(nodeMarkIn.clone(), primalComp.clone(), yCoord.clone())?;
            Ok(nodeMarkTmp.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(nodeMarkOut)
}

fn tupleToStringIntRealInt(mut inTuple: (i32, metamodelica::Real, i32)) -> ArcStr {
    let mut result: ArcStr;
    result = ((match inTuple {
        (mut int1, mut real1, mut int2) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(int1.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*realString(real1.clone())); __mm_s.push_str(&*literal!(" , ")); __mm_s.push_str(&*intString(int2.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }
        },
    })).clone();
    result
}

pub(crate) fn transposeCommCosts(mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> {
    let mut oCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut tmpCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    tmpCommCosts = arrayCreate(metamodelica::arrayLength(iCommCosts.clone()), metamodelica::nil());
    (_, tmpCommCosts) = Array::fold(iCommCosts.clone(), (std::sync::Arc::new(transposeCommCosts0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Communication>>, (i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>)) -> Result<(i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>)> + 'static>), (1, tmpCommCosts.clone()))?;
    oCommCosts = tmpCommCosts.clone();
    Ok(oCommCosts)
}

fn transposeCommCosts0(mut iCosts: Communications, mut iCommCosts: (i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>)) -> Result<(i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>)> {
    let mut oCommCosts: (i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>);
    let mut iParentCompIdx: i32;
    let mut tmpCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    (iParentCompIdx, tmpCommCosts) = iCommCosts;
    tmpCommCosts = List::fold1(iCosts, (std::sync::Arc::new(fnptr!(transposeCommCosts1, Communication, i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Communication, i32, metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> + 'static>), iParentCompIdx, tmpCommCosts.clone())?;
    oCommCosts = (iParentCompIdx + 1, tmpCommCosts.clone());
    Ok(oCommCosts)
}

fn transposeCommCosts1(mut iCost: Communication, mut iParentCompIdx: i32, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> metamodelica::Array<Arc<metamodelica::List<Communication>>> {
    let mut oCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut tmpCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>> = Default::default();
    let mut costs: Communications = metamodelica::nil();
    let mut numberOfVars: i32 = 0;
    let mut nodeIdx: i32 = 0;
    let mut integerVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut floatVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut booleanVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut stringVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut requiredTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    oCommCosts = 'mc: {
        let __mc_input = iCost;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let Communication { numberOfVars: mut numberOfVars, integerVars: mut integerVars, floatVars: mut floatVars, booleanVars: mut booleanVars, stringVars: mut stringVars, childNode: mut nodeIdx, requiredTime: mut requiredTime } = __mc_input.clone() else { bail!("nomatch") };
            let mut costs: Arc<metamodelica::List<Communication>> = costs.clone();
            let mut tmpCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>> = tmpCommCosts.clone();
            let true = (intLe(nodeIdx, metamodelica::arrayLength(iCommCosts.clone()))) else { bail!("pattern mismatch") };
            costs = metamodelica::arrayGet(iCommCosts.clone(), nodeIdx)?;
            costs = metamodelica::cons(Communication { numberOfVars: numberOfVars, integerVars: integerVars.clone(), floatVars: floatVars.clone(), booleanVars: booleanVars.clone(), stringVars: stringVars.clone(), childNode: iParentCompIdx, requiredTime: requiredTime }, costs.clone());
            tmpCommCosts = metamodelica::arrayUpdate(iCommCosts.clone(), nodeIdx, costs.clone())?;
            Ok((tmpCommCosts.clone(), costs.clone(), tmpCommCosts.clone()))
        })() { costs = __wb0; tmpCommCosts = __wb1; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iCommCosts.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oCommCosts
}

//TODO: Can this be merged with getCommCostBetweenNodes?
fn getCommunicationCost(mut childIdx: i32, mut parentIdx: i32, mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<Communication> {
    let mut oComm: Communication;
    let mut commRow: Communications;
    let mut commEntry: Communication;
    commRow = metamodelica::arrayGet(commCosts.clone(), parentIdx)?;
    commEntry = getCommunicationByChildIdx(commRow, childIdx)?;
    oComm = commEntry;
    Ok(oComm)
}

fn getCommunicationByChildIdx(mut iComms: Communications, mut iChildIdx: i32) -> Result<Communication> {
    let mut oComm: Communication;
    oComm = 'mc: {
        let __mc_input = iComms;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Communication { childNode: currentCommChild, .. }, tail: rest } => {
                    let mut tmpComm: Communication;
                    let false = (intEq(currentCommChild.clone(), iChildIdx)) else { bail!("pattern mismatch") };
                    tmpComm = getCommunicationByChildIdx(rest.clone(), iChildIdx)?;
                    Ok(tmpComm.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head @ Communication { childNode: currentCommChild, .. }, tail: _ } => {
                    let true = (intEq(currentCommChild.clone(), iChildIdx)) else { bail!("pattern mismatch") };
                    Ok(head.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getCommunicationByChildIdx failed! - the child idx ")); __mm_s.push_str(&*intString(iChildIdx)); __mm_s.push_str(&*literal!(" can not be found in the list of edges\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oComm)
}

pub(crate) fn getCommCostTimeBetweenNodes(mut iParentNodeIdx: i32, mut iChildNodeIdx: i32, mut iTaskGraphMeta: TaskGraphMeta) -> Result<metamodelica::Real> {
    let mut oCommCost: metamodelica::Real;
    let mut requiredTime: metamodelica::Real;
    let Communication { requiredTime: __pa0, .. } = (getCommCostBetweenNodes(iParentNodeIdx, iChildNodeIdx, iTaskGraphMeta)?) else { bail!("pattern mismatch") };
    requiredTime = __pa0.clone();
    oCommCost = requiredTime;
    Ok(oCommCost)
}

fn getCommCostBetweenNodes(mut iParentNodeIdx: i32, mut iChildNodeIdx: i32, mut iTaskGraphMeta: TaskGraphMeta) -> Result<Communication> {
    let mut oCommCost: Communication;
    let mut childComps: Arc<metamodelica::List<i32>>;
    let mut parentComps: Arc<metamodelica::List<i32>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut concreteCommCostsOpt: Arc<metamodelica::List<Option<Communication>>>;
    let mut concreteCommCosts: Communications;
    let TaskGraphMeta { inComps: __pa0, commCosts: __pa1, .. } = (iTaskGraphMeta) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    commCosts = __pa1.clone();
    parentComps = metamodelica::arrayGet(inComps.clone(), iParentNodeIdx)?;
    childComps = metamodelica::arrayGet(inComps.clone(), iChildNodeIdx)?;
    concreteCommCostsOpt = List::map2(parentComps, (std::sync::Arc::new(fnptr!(getCommCostBetweenNodes0, i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<Communication>>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<Option<Communication>> + 'static>), childComps, commCosts.clone())?;
    concreteCommCosts = ({
        let mut __acc: Arc<metamodelica::List<Communication>> = metamodelica::nil();
        for mut c in (concreteCommCostsOpt).into_iter().cloned() {
            if !(isSome(c.clone())) { continue; }
            let __x = Util::getOption(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    oCommCost = getHighestCommCost(concreteCommCosts, Communication { numberOfVars: 0, integerVars: metamodelica::nil(), floatVars: metamodelica::nil(), booleanVars: metamodelica::nil(), stringVars: metamodelica::nil(), childNode: -1, requiredTime: metamodelica::OrderedFloat(-1.0_f64) });
    Ok(oCommCost)
}

fn getCommCostBetweenNodes0(mut iParentComp: i32, mut iChildComps: Arc<metamodelica::List<i32>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Option<Communication> {
    let mut oHighestComm: Option<Communication>;
    let mut commCosts: Communications = metamodelica::nil();
    let mut filteredCommCosts: Communications = metamodelica::nil();
    let mut highestCommCost: Communication = <Communication as ::std::default::Default>::default();
    oHighestComm = 'mc: {
        let __mc_input = iCommCosts.clone();
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut commCosts: Arc<metamodelica::List<Communication>> = commCosts.clone();
            let mut filteredCommCosts: Arc<metamodelica::List<Communication>> = filteredCommCosts.clone();
            let mut highestCommCost: Communication = highestCommCost.clone();
            commCosts = metamodelica::arrayGet(iCommCosts.clone(), iParentComp)?;
            filteredCommCosts = List::filter1OnTrue(commCosts.clone(), (std::sync::Arc::new(getCommCostBetweenNodes1) as std::sync::Arc<dyn ::std::ops::Fn(Communication, Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>), iChildComps.clone())?;
            let false = (filteredCommCosts.clone().is_empty()) else { bail!("pattern mismatch") };
            highestCommCost = getHighestCommCost(filteredCommCosts.clone(), Communication { numberOfVars: 0, integerVars: metamodelica::nil(), floatVars: metamodelica::nil(), booleanVars: metamodelica::nil(), stringVars: metamodelica::nil(), childNode: -1, requiredTime: metamodelica::OrderedFloat(-1.0_f64) });
            Ok((Some(highestCommCost.clone()), commCosts.clone(), filteredCommCosts.clone(), highestCommCost.clone()))
        })() { commCosts = __wb0; filteredCommCosts = __wb1; highestCommCost = __wb2; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(None)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oHighestComm
}

fn getCommCostBetweenNodes1(mut iCommCost: Communication, mut iChildComps: Arc<metamodelica::List<i32>>) -> Result<bool> {
    let mut oResult: bool;
    let mut compIdx: i32;
    let Communication { childNode: __pa0, .. } = (iCommCost) else { bail!("pattern mismatch") };
    compIdx = __pa0.clone();
    oResult = List::exist1(iChildComps, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), compIdx)?;
    Ok(oResult)
}

fn getHighestCommCost(mut iCommCosts: Communications, mut iHighestTuple: Communication) -> Communication {
    let mut oHighestTuple: Communication;
    let mut highestCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut currentCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut head: Communication = <Communication as ::std::default::Default>::default();
    let mut rest: Communications = metamodelica::nil();
    oHighestTuple = 'mc: {
        let __mc_input = (iCommCosts, iHighestTuple.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ Communication { requiredTime: currentCost, .. }, tail: rest }, Communication { requiredTime: highestCost, .. }) => {
                    let true = (realGt(currentCost.clone(), highestCost.clone())) else { bail!("pattern mismatch") };
                    Ok(getHighestCommCost(rest.clone(), head.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head, tail: rest }, _) => {
                    Ok(getHighestCommCost(rest.clone(), iHighestTuple.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iHighestTuple.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oHighestTuple
}

pub(crate) fn sumUpExeCosts(mut iGraph: TaskGraph, mut iMeta: TaskGraphMeta) -> Result<(i32, metamodelica::Real)> {
    let mut execCosts: (i32, metamodelica::Real);
    let mut cost1: i32 = 0;
    let mut cost2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut comps: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)> = Default::default();
    let mut exeCostLst: Arc<metamodelica::List<(i32, metamodelica::Real)>> = metamodelica::nil();
    execCosts = (match iMeta {
        TaskGraphMeta { inComps: mut __esc_inComps, exeCosts: mut __esc_exeCosts, .. } => {
            inComps = __esc_inComps.clone();
            exeCosts = __esc_exeCosts.clone();
            comps = List::flatten(List::map1(List::intRange(metamodelica::arrayLength(iGraph.clone())), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), inComps.clone())?)?;
            exeCostLst = List::map1(comps, (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), exeCosts.clone())?;
            cost1 = List::fold(List::map(exeCostLst.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)))?, (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 0)?;
            cost2 = List::fold(List::map(exeCostLst, std::sync::Arc::new(fnptr!(Util::tuple22, _)))?, (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
            (cost1, cost2)
        },
        _ => (0, metamodelica::OrderedFloat(0.0_f64)),
    });
    Ok(execCosts)
}

pub(crate) fn getAllSCCsOfGraph(mut iTaskGraphMeta: TaskGraphMeta) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oSccs: Arc<metamodelica::List<i32>>;
    let mut taskIdx: i32 = 0;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut comps: Arc<metamodelica::List<i32>>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut tmpSccs: Arc<metamodelica::List<i32>>;
    tmpSccs = metamodelica::nil();
    let TaskGraphMeta { inComps: __pa0, nodeMark: __pa1, .. } = (iTaskGraphMeta) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    nodeMark = __pa1.clone();
    for mut taskIdx in 1..=metamodelica::arrayLength(inComps.clone()) {
        comps = metamodelica::arrayGet(inComps.clone(), taskIdx)?;
        tmpSccs = List::append_reverse(comps.clone(), tmpSccs.clone());
    }
    oSccs = tmpSccs.reverse();
    Ok(oSccs)
}

//TODO: Remove
pub(crate) fn roundReal(mut inReal: metamodelica::Real, mut nIn: i32) -> metamodelica::Real {
    let mut outReal: metamodelica::Real;
    let mut real: metamodelica::Real;
    real = inReal * (metamodelica::OrderedFloat(10.0_f64)).powf(metamodelica::OrderedFloat((nIn) as f64));
    real = (real).floor();
    outReal = real / (metamodelica::OrderedFloat(10.0_f64)).powf(metamodelica::OrderedFloat((nIn) as f64));
    outReal
}

//--------------------------------------------------------
//  Get annotations from backendDAE and display in graphML
//--------------------------------------------------------
fn setAnnotationsForTasks(mut taskGraphInfo: TaskGraphMeta, mut backendDAE: Arc<BackendDAE::BackendDAE>, mut annotInfoIn: metamodelica::Array<ArcStr>) -> Result<metamodelica::Array<ArcStr>> {
    let mut annotInfoOut: metamodelica::Array<ArcStr>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let __pa0 = ::match_deref::match_deref! { match &(backendDAE) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    systs = __pa0.clone();
    (_, annotInfoOut) = List::fold1(systs, (std::sync::Arc::new(setAnnotationsForTasks1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, TaskGraphMeta, (i32, metamodelica::Array<ArcStr>)) -> Result<(i32, metamodelica::Array<ArcStr>)> + 'static>), taskGraphInfo, (0, annotInfoIn.clone()))?;
    Ok(annotInfoOut)
}

fn setAnnotationsForTasks1(mut syst: Arc<BackendDAE::EqSystem>, mut taskGraphInfo: TaskGraphMeta, mut infoIn: (i32, metamodelica::Array<ArcStr>)) -> Result<(i32, metamodelica::Array<ArcStr>)> {
    let mut infoOut: (i32, metamodelica::Array<ArcStr>);
    let mut idx: i32;
    let mut annots: metamodelica::Array<ArcStr>;
    let mut vars: BackendDAE::Variables;
    let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    (idx, annots) = infoIn;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(syst) {
        Deref @ BackendDAE::EqSystem { orderedVars: __pa0, orderedEqs: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    vars = __pa0.clone();
    eqs = __pa1.clone();
    annots = List::fold3(List::intRange(BackendVariable::varsSize(vars.clone())), (std::sync::Arc::new(fnptr!(setAnnotationsForVar, i32, BackendDAE::Variables, TaskGraphMeta, i32, metamodelica::Array<ArcStr>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, BackendDAE::Variables, TaskGraphMeta, i32, metamodelica::Array<ArcStr>) -> Result<metamodelica::Array<ArcStr>> + 'static>), vars.clone(), taskGraphInfo, idx, annots.clone())?;
    infoOut = (BackendVariable::varsSize(vars) + idx, annots.clone());
    Ok(infoOut)
}

fn setAnnotationsForVar(mut backendVarIdx: i32, mut vars: BackendDAE::Variables, mut taskGraphInfo: TaskGraphMeta, mut eqSysOffset: i32, mut annotInfoIn: metamodelica::Array<ArcStr>) -> metamodelica::Array<ArcStr> {
    let mut annotInfoOut: metamodelica::Array<ArcStr>;
    annotInfoOut = 'mc: {
        let __mc_input = taskGraphInfo;
        if let Ok(__v) = (|| -> Result<_> {
            let TaskGraphMeta { inComps: mut inComps, varCompMapping: mut varCompMapping, nodeMark: mut nodeMark, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut compIdx: i32;
            let mut taskIdx: i32;
            let mut annotString: ArcStr;
            let mut var: BackendDAE::Var;
            let mut cr: Arc<DAE::ComponentRef>;
            let mut annot: Option<Arc<SCode::Comment>>;
            var = BackendVariable::getVarAt(vars.clone(), backendVarIdx)?;
            BackendDump::printVar(var.clone())?;
            let true = (BackendVariable::hasAnnotation(var.clone())) else { bail!("pattern mismatch") };
            (compIdx, _, _) = metamodelica::arrayGet(varCompMapping.clone(), backendVarIdx + eqSysOffset)?;
            taskIdx = getCompInComps(compIdx.clone(), 1, inComps.clone(), nodeMark.clone())?;
            annot = BackendVariable::getAnnotationComment(var.clone())?;
            annotString = (metamodelica::arrayGet(annotInfoIn.clone(), taskIdx.clone())?).clone();
            cr = BackendVariable::varCref(var.clone())?;
            annotString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*annotString.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cr.clone())?); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*DAEDumpTypes::dumpCommentAnnotationStr(annot.clone())); __mm_s.push_str(&*literal!(") ")); ArcStr::from(__mm_s) }).clone();
            metamodelica::arrayUpdate(annotInfoIn.clone(), taskIdx.clone(), (annotString.clone()).clone())?;
            Ok(annotInfoIn.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(annotInfoIn.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    annotInfoOut
}

//--------------------------------------------------------
//  Append removed equations like asserts to the DAE graph
//--------------------------------------------------------
pub(crate) fn appendRemovedEquations(mut dae: Arc<BackendDAE::BackendDAE>, mut graphIn: TaskGraph, mut graphDataIn: TaskGraphMeta) -> (TaskGraph, TaskGraphMeta) {
    let mut graphOut: TaskGraph;
    let mut graphDataOut: TaskGraphMeta;
    (graphOut, graphDataOut) = 'mc: {
        let __mc_input = graphDataIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut numNewComps: i32;
            let mut newComps: Arc<metamodelica::List<i32>>;
            let mut nodeVarLst: Arc<metamodelica::List<Arc<metamodelica::List<(i32, i32)>>>>;
            let mut varCompMap: metamodelica::Array<(i32, i32, i32)>;
            let mut graph: TaskGraph;
            let mut graphData: TaskGraphMeta;
            let mut remEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut shared: Arc<BackendDAE::Shared>;
            let mut eqLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            let mut crefsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>;
            let mut inComps1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut inComps2: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut varCompMapping1: metamodelica::Array<(i32, i32, i32)>;
            let mut eqCompMapping1: metamodelica::Array<(i32, i32, i32)>;
            let mut compParamMapping1: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut compNames1: metamodelica::Array<ArcStr>;
            let mut compNames2: metamodelica::Array<ArcStr>;
            let mut compDescs1: metamodelica::Array<ArcStr>;
            let mut compDescs2: metamodelica::Array<ArcStr>;
            let mut exeCosts1: metamodelica::Array<(i32, metamodelica::Real)>;
            let mut exeCosts2: metamodelica::Array<(i32, metamodelica::Real)>;
            let mut commCosts1: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
            let mut nodeMark1: metamodelica::Array<i32>;
            let mut nodeMark2: metamodelica::Array<i32>;
            let mut compInformations1: metamodelica::Array<ComponentInfo>;
            let mut compInformations2: metamodelica::Array<ComponentInfo>;
            let __pa0 = ::match_deref::match_deref! { match &(dae.clone()) {
                Deref @ BackendDAE::BackendDAE { shared: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            shared = __pa0.clone();
            remEqs = BackendDAEUtil::collapseRemovedEqs(dae.clone())?;
            let TaskGraphMeta { varCompMapping: __pa1, .. } = (graphDataIn.clone()) else { bail!("pattern mismatch") };
            varCompMap = __pa1.clone();
            eqLst = BackendEquation::equationList(remEqs.clone())?;
            numNewComps = (eqLst.clone().len() as i32);
            let true = (intNe(numNewComps.clone(), 0)) else { bail!("pattern mismatch") };
            crefsLst = List::map(eqLst.clone(), (std::sync::Arc::new(BackendEquation::equationCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>))?;
            nodeVarLst = List::map2(crefsLst.clone(), (std::sync::Arc::new(getNodeForCrefLst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<BackendDAE::BackendDAE>, metamodelica::Array<(i32, i32, i32)>) -> Result<Arc<metamodelica::List<(i32, i32)>>> + 'static>), dae.clone(), varCompMap.clone())?;
            let TaskGraphMeta { inComps: __pa2, varCompMapping: __pa3, eqCompMapping: __pa4, compParamMapping: __pa5, compNames: __pa6, compDescs: __pa7, exeCosts: __pa8, commCosts: __pa9, nodeMark: __pa10, compInformations: __pa11 } = (graphDataIn.clone()) else { bail!("pattern mismatch") };
            inComps1 = __pa2.clone();
            varCompMapping1 = __pa3.clone();
            eqCompMapping1 = __pa4.clone();
            compParamMapping1 = __pa5.clone();
            compNames1 = __pa6.clone();
            compDescs1 = __pa7.clone();
            exeCosts1 = __pa8.clone();
            commCosts1 = __pa9.clone();
            nodeMark1 = __pa10.clone();
            compInformations1 = __pa11.clone();
            graph = metamodelica::arrayAppend(graphIn.clone(), arrayCreate(numNewComps.clone(), metamodelica::nil()));
            newComps = List::intRange2(metamodelica::arrayLength(graphIn.clone()) + 1, metamodelica::arrayLength(graphIn.clone()) + numNewComps.clone());
            graph = List::threadFold(nodeVarLst.clone(), newComps.clone(), (std::sync::Arc::new(addEdgesToGraph) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(i32, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), graph.clone())?;
            inComps2 = metamodelica::arrayFromVec(List::map(newComps.clone(), std::sync::Arc::new(fnptr!(List::create, _)))?.into_iter().cloned().collect());
            compNames2 = arrayCreate(numNewComps.clone(), (literal!("assert")).clone());
            compDescs2 = metamodelica::arrayFromVec(List::map(eqLst.clone(), (std::sync::Arc::new(BackendDump::equationString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<ArcStr> + 'static>))?.into_iter().cloned().collect());
            nodeMark2 = arrayCreate(numNewComps.clone(), -2);
            exeCosts2 = metamodelica::arrayFromVec(List::map1(eqLst.clone(), (std::sync::Arc::new(estimateEquationCosts) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<BackendDAE::Shared>) -> Result<(i32, metamodelica::Real)> + 'static>), shared.clone())?.into_iter().cloned().collect());
            compInformations2 = arrayCreate(numNewComps.clone(), ComponentInfo { isPartOfODESystem: false, isPartOfZeroFuncSystem: false, isRemovedComponent: true });
            inComps1 = metamodelica::arrayAppend(inComps1.clone(), inComps2.clone());
            compNames1 = metamodelica::arrayAppend(compNames1.clone(), compNames2.clone());
            compDescs1 = metamodelica::arrayAppend(compDescs1.clone(), compDescs2.clone());
            nodeMark1 = metamodelica::arrayAppend(nodeMark1.clone(), nodeMark2.clone());
            exeCosts1 = metamodelica::arrayAppend(exeCosts1.clone(), exeCosts2.clone());
            compInformations1 = metamodelica::arrayAppend(compInformations1.clone(), compInformations2.clone());
            commCosts1 = List::threadFold1(nodeVarLst.clone(), newComps.clone(), (std::sync::Arc::new(setCommCostsToParent) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(i32, i32)>>, i32, metamodelica::Real, metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> + 'static>), metamodelica::OrderedFloat(74.0_f64), commCosts1.clone())?;
            graphData = TaskGraphMeta { inComps: inComps1.clone(), varCompMapping: varCompMapping1.clone(), eqCompMapping: eqCompMapping1.clone(), compParamMapping: compParamMapping1.clone(), compNames: compNames1.clone(), compDescs: compDescs1.clone(), exeCosts: exeCosts1.clone(), commCosts: commCosts1.clone(), nodeMark: nodeMark1.clone(), compInformations: compInformations1.clone() };
            Ok((graph.clone(), graphData.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((graphIn.clone(), graphDataIn.clone()))
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (graphOut, graphDataOut)
}

fn estimateEquationCosts(mut eqIn: Arc<BackendDAE::Equation>, mut sharedIn: Arc<BackendDAE::Shared>) -> Result<(i32, metamodelica::Real)> {
    let mut tplOut: (i32, metamodelica::Real);
    let mut numAdd: i32;
    let mut numMul: i32;
    let mut numDiv: i32;
    let mut numTrig: i32;
    let mut numRel: i32;
    let mut numOth: i32;
    let mut numFuncs: i32;
    let mut numLog: i32;
    let mut compInfo: Arc<BackendDAE::CompInfo>;
    let (_, (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7)) = BackendEquation::traverseExpsOfEquation(eqIn, (std::sync::Arc::new({ let __pe_b1 = sharedIn; move |__pe_a0, __pe_a2| BackendDAEOptimize::countOperationsExp(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32)) -> Result<(Arc<DAE::Exp>, (i32, i32, i32, i32, i32, i32, i32, i32))> + 'static>), (0, 0, 0, 0, 0, 0, 0, 0))?;
    numAdd = __pa0.clone();
    numMul = __pa1.clone();
    numDiv = __pa2.clone();
    numTrig = __pa3.clone();
    numRel = __pa4.clone();
    numLog = __pa5.clone();
    numOth = __pa6.clone();
    numFuncs = __pa7.clone();
    compInfo = Arc::new(BackendDAE::CompInfo::NO_COMP { numAdds: numAdd, numMul: numMul, numDiv: numDiv, numTrig: numTrig, numRelations: numRel, numLog: numLog, numOth: numOth, funcCalls: numFuncs });
    tplOut = calculateCosts(compInfo);
    Ok(tplOut)
}

fn printNodeVars(mut nodes: Arc<metamodelica::List<(i32, i32)>>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*stringDelimitList(List::map(nodes, (std::sync::Arc::new(fnptr!(printNodeVars1, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<ArcStr> + 'static>))?, (literal!(" | ")).clone())); ArcStr::from(__mm_s) }).clone();
    Ok(s)
}

fn printNodeVars1(mut node: (i32, i32)) -> ArcStr {
    let mut s: ArcStr;
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(Util::tuple21(node))); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(Util::tuple22(node))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    s
}

fn setCommCostsToParent(mut parents: Arc<metamodelica::List<(i32, i32)>>, mut child: i32, mut reqCycles: metamodelica::Real, mut commCostsIn: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> {
    let mut commCostsOut: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    commCostsOut = List::fold2(parents, (std::sync::Arc::new(setCommCosts) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), i32, metamodelica::Real, metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> + 'static>), child, reqCycles, commCostsIn.clone())?;
    Ok(commCostsOut)
}

fn setCommCosts(mut parent: (i32, i32), mut child: i32, mut reqCycles: metamodelica::Real, mut commCostsIn: metamodelica::Array<Arc<metamodelica::List<Communication>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Communication>>>> {
    let mut commCostsOut: metamodelica::Array<Arc<metamodelica::List<Communication>>>;
    let mut row: Communications;
    let mut parentNodeIdx: i32;
    let mut varIdx: i32;
    (parentNodeIdx, varIdx) = parent;
    row = metamodelica::arrayGet(commCostsIn.clone(), parentNodeIdx)?;
    row = List::filter1OnTrue(row, (std::sync::Arc::new(isCommunicationChildEqualToIdx) as std::sync::Arc<dyn ::std::ops::Fn(Communication, i32) -> Result<bool> + 'static>), child)?;
    row = metamodelica::cons(Communication { numberOfVars: 1, integerVars: metamodelica::nil(), floatVars: list![varIdx], booleanVars: metamodelica::nil(), stringVars: metamodelica::nil(), childNode: child, requiredTime: reqCycles }, row);
    commCostsOut = metamodelica::arrayUpdate(commCostsIn.clone(), parentNodeIdx, row)?;
    Ok(commCostsOut)
}

fn isCommunicationChildEqualToIdx(mut iComm: Communication, mut iIdx: i32) -> Result<bool> {
    let mut isEq: bool;
    let mut childNode: i32;
    let Communication { childNode: __pa0, .. } = (iComm) else { bail!("pattern mismatch") };
    childNode = __pa0.clone();
    isEq = intNe(childNode, iIdx);
    Ok(isEq)
}

fn addEdgesToGraph(mut parents: Arc<metamodelica::List<(i32, i32)>>, mut child: i32, mut graphIn: TaskGraph) -> Result<TaskGraph> {
    let mut graphOut: TaskGraph;
    graphOut = List::fold1(List::map(parents, std::sync::Arc::new(fnptr!(Util::tuple21, _)))?, (std::sync::Arc::new(addEdgeToGraph) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), child, graphIn.clone())?;
    Ok(graphOut)
}

fn addEdgeToGraph(mut parent: i32, mut child: i32, mut graphIn: TaskGraph) -> Result<TaskGraph> {
    let mut graphOut: TaskGraph;
    let mut row: Arc<metamodelica::List<i32>>;
    row = metamodelica::arrayGet(graphIn.clone(), parent)?;
    row = List::unique(metamodelica::cons(child, row));
    graphOut = metamodelica::arrayUpdate(graphIn.clone(), parent, row)?;
    Ok(graphOut)
}

fn getNodeForCrefLst(mut iCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut iDae: Arc<BackendDAE::BackendDAE>, mut iVarCompMap: metamodelica::Array<(i32, i32, i32)>) -> Result<Arc<metamodelica::List<(i32, i32)>>> {
    let mut oNodeVarLst: Arc<metamodelica::List<(i32, i32)>>;
    let mut tmpNodeVarLst: Arc<metamodelica::List<(i32, i32)>>;
    tmpNodeVarLst = List::map2(iCrefs, (std::sync::Arc::new(getNodeForCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<BackendDAE::BackendDAE>, metamodelica::Array<(i32, i32, i32)>) -> Result<(i32, i32)> + 'static>), iDae, iVarCompMap.clone())?;
    oNodeVarLst = List::filterOnTrue(tmpNodeVarLst, (std::sync::Arc::new(fnptr!(nodeIsDependent, (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<bool> + 'static>))?;
    Ok(oNodeVarLst)
}

fn nodeIsDependent(mut node: (i32, i32)) -> bool {
    let mut dep: bool;
    let mut tpl1: i32;
    (tpl1, _) = node;
    dep = intNe(tpl1, -1);
    dep
}

fn getNodeForCref(mut iCref: Arc<DAE::ComponentRef>, mut iDae: Arc<BackendDAE::BackendDAE>, mut iVarCompMapping: metamodelica::Array<(i32, i32, i32)>) -> Result<(i32, i32)> {
    let mut oNodeVarIdx: (i32, i32);
    let mut eqSysIdx: i32;
    let mut varIdx: i32;
    let mut nodeIdx: i32;
    let mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let __pa0 = ::match_deref::match_deref! { match &(iDae) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqSystems = __pa0.clone();
    (eqSysIdx, varIdx, _) = getNodeForCref1(eqSystems, iCref, 1)?;
    nodeIdx = getNodeForVarIdx(varIdx, eqSysIdx, iVarCompMapping.clone(), varIdx)?;
    oNodeVarIdx = (nodeIdx, varIdx);
    Ok(oNodeVarIdx)
}

fn getNodeForCref1(mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut cref: Arc<DAE::ComponentRef>, mut eqSysIdxIn: i32) -> Result<(i32, i32, bool)> {
    let mut eqSysIdx: i32;
    let mut varIdx: i32;
    let mut found: bool;
    (eqSysIdx, varIdx, found) = 'mc: {
        let __mc_input = eqSystems;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::EqSystem { orderedVars: vars, .. }, tail: _ } => {
                    let mut b: bool;
                    let mut esIdx: i32;
                    let mut vIdx: i32;
                    let mut lst: Arc<metamodelica::List<i32>>;
                    let mut varLst: Arc<metamodelica::List<BackendDAE::Var>>;
                    (varLst, lst) = BackendVariable::getVar(cref.clone(), vars.clone())?;
                    if intNe((lst.clone().len() as i32), 1) {
                        metamodelica::print((literal!("Check if there is a assert or something that is dependent of arrayEquations")).clone());
                    }
                    if BackendVariable::isStateVar(listHead(varLst.clone())?) {
                        (esIdx, vIdx, b) = (-1, -1, false);
                    } else {
                        (esIdx, vIdx, b) = (eqSysIdxIn, listHead(lst.clone())?, true);
                    }
                    Ok((esIdx.clone(), vIdx.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::EqSystem { .. }, tail: rest } => {
                    let mut b: bool;
                    let mut esIdx: i32;
                    let mut vIdx: i32;
                    (esIdx, vIdx, b) = getNodeForCref1(rest.clone(), cref.clone(), eqSysIdxIn + 1)?;
                    Ok((esIdx.clone(), vIdx.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((-1, -1, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((eqSysIdx, varIdx, found))
}

fn getNodeForVarIdx(mut varIdx: i32, mut eqSysIdx: i32, mut varCompMapping: metamodelica::Array<(i32, i32, i32)>, mut inTryThisIndex: i32) -> Result<i32> {
    let mut node: i32 = 0;
    let mut offset: i32;
    let mut eqSys: i32;
    let mut tryThisIndex: i32 = inTryThisIndex;
    let mut n: i32 = 0;
    let mut arrayLengthVarCompMapping: i32;
    arrayLengthVarCompMapping = metamodelica::arrayLength(varCompMapping.clone());
    loop {
        if tryThisIndex >= 1 && tryThisIndex <= arrayLengthVarCompMapping {
            (node, eqSys, offset) = metamodelica::arrayGet(varCompMapping.clone(), tryThisIndex)?;
            if eqSys == eqSysIdx {
                node = node + varIdx - 1;
                return Ok(node.clone());
            } else {
                tryThisIndex = offset + 2;
            }
        } else if varIdx == -1 && eqSysIdx == -1 {
            node = -1;
            return Ok(node.clone());
        } else {
            metamodelica::print((literal!("HpcOmTaskGraph.getNodeForVarIdx failed\n")).clone());
        }
        n = n + 1;
        if n > arrayLengthVarCompMapping {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HpcOmTaskGraph.getNodeForVarIdx")); __mm_s.push_str(&*literal!(" failed (there is a loop somewhere)")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/HpcOmTaskGraph.mo"))?;
            bail!("fail");
        }
    }
    Ok(node)
}

//----------------------------
//  MULTIRATE PARTITIONING
//----------------------------
pub(crate) fn multirate_partitioning(mut odeGraph: TaskGraph, mut odeGraphData: TaskGraphMeta, mut backendDAE: Arc<BackendDAE::BackendDAE>, mut simCode: SimCode::SimCode, mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<SimCode::PartitionData> {
    let mut partitionDataOut: SimCode::PartitionData;
    let mut stateTaskAssign: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut stateTasks: Arc<metamodelica::List<i32>>;
    let mut tasksPerLevel: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut partitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut odeGraphT: TaskGraph;
    let mut numPartitions: i32;
    let mut activatorsForPartitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut stateToActivators: Arc<metamodelica::List<i32>>;
    tasksPerLevel = getLevelNodes(odeGraph.clone())?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tasksPerLevel ")); __mm_s.push_str(&*stringDelimitList(List::map(tasksPerLevel.clone(), (std::sync::Arc::new(intLstString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))?, (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    stateTasks = getLeafNodes(odeGraph.clone())?;
    stateTasks = multirate_orderStateTasksInSimVarStateOrder(stateTasks, odeGraphData, backendDAE, simCode)?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("stateTasks ")); __mm_s.push_str(&*intLstString(stateTasks.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    odeGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(odeGraph.clone(), metamodelica::arrayLength(odeGraph.clone()))?;
    stateTaskAssign = multirate_assignTasksToStates(tasksPerLevel, stateTasks.clone(), odeGraphT.clone())?;
    dumpStateAssign(stateTaskAssign.clone())?;
    partitions = multirate_getPartitions(stateTaskAssign.clone(), stateTasks.clone(), odeGraphT.clone())?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("PARTITIONS :\n")); __mm_s.push_str(&*stringDelimitList(List::map(partitions.clone(), (std::sync::Arc::new(intLstString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))?, (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    activatorsForPartitions = List::mapMap(partitions.clone(), (std::sync::Arc::new(listHead) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>), (std::sync::Arc::new({ let __pe_b1 = stateTaskAssign.clone(); move |__pe_a0| Array::getIndexFirst(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<_> + 'static>))?;
    partitions = List::map1(partitions, (std::sync::Arc::new(getSimEqsIdxLstForSCCIdxLst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), sccSimEqMapping.clone())?;
    numPartitions = (partitions.clone().len() as i32);
    stateToActivators = List::intRange((stateTasks.len() as i32));
    partitionDataOut = SimCode::PartitionData { numPartitions: numPartitions, partitions: partitions, activatorsForPartitions: activatorsForPartitions, stateToActivators: stateToActivators };
    dumpPartitionData(partitionDataOut.clone())?;
    Ok(partitionDataOut)
}

fn multirate_orderStateTasksInSimVarStateOrder(mut stateTasks: Arc<metamodelica::List<i32>>, mut taskGraphData: TaskGraphMeta, mut dae: Arc<BackendDAE::BackendDAE>, mut simCode: SimCode::SimCode) -> Result<Arc<metamodelica::List<i32>>> {
    let mut orderedTasks: Arc<metamodelica::List<i32>>;
    let mut state: i32 = 0;
    let mut compIdx: i32;
    let mut eqSysIdx: i32;
    let mut offset: i32;
    let mut varIdx: i32;
    let mut simVarIdx: i32;
    let mut simVarIdxs: Arc<metamodelica::List<i32>>;
    let mut order: Arc<metamodelica::List<i32>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut var: BackendDAE::Var;
    let mut eqSys: Arc<BackendDAE::EqSystem>;
    let mut cref: Arc<DAE::ComponentRef>;
    let mut simVar: SimCodeVar::SimVar;
    let mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let __pa0 = ::match_deref::match_deref! { match &(dae) {
        Deref @ BackendDAE::BackendDAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqSystems = __pa0.clone();
    simVarIdxs = metamodelica::nil();
    for mut state in &*stateTasks.clone() {
        let mut state = state.clone();
        compIdx = listHead(metamodelica::arrayGet(taskGraphData.inComps.clone(), state)?)?;
        let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Array::findFirstOnTrueWithIdx(taskGraphData.varCompMapping.clone(), (std::sync::Arc::new({ let __pe_b1 = compIdx; move |__pe_a0| Ok(varMappingTupleCompEqual(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32, i32)) -> Result<bool> + 'static>))?) {
            (Some((__pa1, __pa2, __pa3)), __pa4) => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => bail!("pattern mismatch"),
        } };
        compIdx = __pa1.clone();
        eqSysIdx = __pa2.clone();
        offset = __pa3.clone();
        varIdx = __pa4.clone();
        eqSys = (eqSystems.clone()).get(eqSysIdx)?;
        varIdx = varIdx - offset;
        var = BackendVariable::getVarAt(eqSys.orderedVars.clone(), varIdx)?;
        cref = var.varName.clone();
        let __pa5 = ::match_deref::match_deref! { match &(SimCodeUtil::getSimVars2Crefs(list![cref.clone()], simCode.crefToSimVarHT.clone())) {
            Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil } => __pa5.clone(),
            _ => bail!("pattern mismatch"),
        } };
        simVar = __pa5.clone();
        simVarIdx = simVar.index.clone();
        simVarIdxs = metamodelica::cons(simVarIdx, simVarIdxs.clone());
    }
    (_, order) = HpcOmScheduler::quicksortWithOrder(List::map(simVarIdxs.reverse(), (std::sync::Arc::new(fnptr!(intReal, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<metamodelica::Real> + 'static>))?)?;
    orderedTasks = List::map1(order, (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), stateTasks)?;
    Ok(orderedTasks)
}

fn varMappingTupleCompEqual(mut tpl: (i32, i32, i32), mut compIdx: i32) -> bool {
    let mut compEqual: bool;
    compEqual = intEq(compIdx, Util::tuple31(tpl));
    compEqual
}

fn getSimEqIdxForSCCIdx(mut sccIdx: i32, mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<i32> {
    let mut simEqIdx: i32;
    simEqIdx = listHead(metamodelica::arrayGet(sccSimEqMapping.clone(), sccIdx)?)?;
    Ok(simEqIdx)
}

fn getSimEqsIdxLstForSCCIdxLst(mut sccIdxs: Arc<metamodelica::List<i32>>, mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut simEqIdxs: Arc<metamodelica::List<i32>>;
    simEqIdxs = List::map1(sccIdxs, (std::sync::Arc::new(getSimEqIdxForSCCIdx) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<i32> + 'static>), sccSimEqMapping.clone())?;
    Ok(simEqIdxs)
}

fn multirate_getPartitions(mut stateTaskAssign: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut stateTasks: Arc<metamodelica::List<i32>>, mut odeGraphT: TaskGraph) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut partitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut numStates: i32;
    let mut numAssigns: i32 = 0;
    let mut leaveNodes: Arc<metamodelica::List<i32>>;
    let mut samePartTasks: Arc<metamodelica::List<i32>>;
    let mut partition: Arc<metamodelica::List<i32>>;
    let mut otherPartTasks: Arc<metamodelica::List<i32>>;
    let mut stateAss: Arc<metamodelica::List<i32>>;
    let mut visitedTasks: metamodelica::Array<i32>;
    let mut leaveNodesWithNassigns: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    visitedTasks = arrayCreate(metamodelica::arrayLength(odeGraphT.clone()), -1);
    numStates = (stateTasks.clone().len() as i32);
    leaveNodesWithNassigns = arrayCreate(numStates, metamodelica::nil());
    metamodelica::arrayUpdate(leaveNodesWithNassigns.clone(), 1, stateTasks)?;
    for mut numAssigns in &*List::intRange(numStates) {
        let mut numAssigns = numAssigns.clone();
        leaveNodes = metamodelica::arrayGet(leaveNodesWithNassigns.clone(), numAssigns)?;
        leaveNodes = List::unique(leaveNodes.clone());
        while !(leaveNodes.clone().is_empty()) {
            stateAss = metamodelica::arrayGet(stateTaskAssign.clone(), listHead(leaveNodes.clone())?)?;
            (samePartTasks, leaveNodes) = List::separateOnTrue(leaveNodes.clone(), (std::sync::Arc::new({ let __pe_b1 = stateTaskAssign.clone(); let __pe_b2 = stateAss.clone(); move |__pe_a0| hasSameStateAssign(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
            (partition, otherPartTasks) = multirate_getPartitionPredecessors(samePartTasks.clone(), odeGraphT.clone(), stateTaskAssign.clone(), stateAss.clone(), visitedTasks.clone())?;
            partition = List::sort(partition.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            multirate_dispatchLeaveNodes(otherPartTasks.clone(), stateTaskAssign.clone(), leaveNodesWithNassigns.clone())?;
            partitions = metamodelica::cons(partition.clone(), partitions.clone());
        }
    }
    Ok(partitions)
}

fn multirate_dispatchLeaveNodes(mut tasksIn: Arc<metamodelica::List<i32>>, mut stateTaskAssign: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut leaveNodesWithNassigns: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut numAss: i32;
    let mut stateAss: Arc<metamodelica::List<i32>>;
    let mut leaveNodes: Arc<metamodelica::List<i32>>;
    for mut task in &*tasksIn {
        let mut task = task.clone();
        stateAss = metamodelica::arrayGet(stateTaskAssign.clone(), task.clone())?;
        numAss = (stateAss.clone().len() as i32);
        leaveNodes = metamodelica::arrayGet(leaveNodesWithNassigns.clone(), numAss)?;
        leaveNodes = metamodelica::cons(task.clone(), leaveNodes.clone());
        metamodelica::arrayUpdate(leaveNodesWithNassigns.clone(), numAss, leaveNodes.clone())?;
    }
    Ok(())
}

fn multirate_getPartitionPredecessors(mut leavesIn: Arc<metamodelica::List<i32>>, mut odeGraphT: TaskGraph, mut stateTaskAssign: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut refStateAssign: Arc<metamodelica::List<i32>>, mut visitedTasks: metamodelica::Array<i32>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut partitionTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut otherLeaveNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cont: bool;
    let mut task: i32;
    let mut tasks: Arc<metamodelica::List<i32>>;
    let mut predecessors: Arc<metamodelica::List<i32>>;
    let mut samePartTasks: Arc<metamodelica::List<i32>>;
    let mut otherLeaves: Arc<metamodelica::List<i32>>;
    cont = true;
    tasks = leavesIn;
    while cont {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(tasks.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        task = __pa0.clone();
        tasks = __pa1.clone();
        predecessors = metamodelica::arrayGet(odeGraphT.clone(), task)?;
        predecessors = List::filter1OnTrue(predecessors.clone(), (std::sync::Arc::new(taskIsNotVisited) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), visitedTasks.clone())?;
        (samePartTasks, otherLeaves) = List::separateOnTrue(predecessors.clone(), (std::sync::Arc::new({ let __pe_b1 = stateTaskAssign.clone(); let __pe_b2 = refStateAssign.clone(); move |__pe_a0| hasSameStateAssign(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?;
        partitionTasks = metamodelica::cons(task, partitionTasks.clone());
        partitionTasks = listAppend(samePartTasks.clone(), partitionTasks.clone());
        tasks = listAppend(samePartTasks.clone(), tasks.clone());
        otherLeaveNodes = listAppend(otherLeaves.clone(), otherLeaveNodes.clone());
        metamodelica::arrayUpdate(visitedTasks.clone(), task, 0)?;
        List::map2_0(samePartTasks.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), 0, visitedTasks.clone())?;
        List::map2_0(otherLeaves.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), 0, visitedTasks.clone())?;
        if tasks.clone().is_empty() {
            cont = false;
        }
    }
    partitionTasks = List::unique(partitionTasks);
    otherLeaveNodes = List::unique(otherLeaveNodes);
    Ok((partitionTasks, otherLeaveNodes))
}

fn taskIsNotVisited(mut task: i32, mut visitedTasks: metamodelica::Array<i32>) -> Result<bool> {
    let mut isNotVisited: bool;
    isNotVisited = intEq(-1, metamodelica::arrayGet(visitedTasks.clone(), task)?);
    Ok(isNotVisited)
}

fn hasSameStateAssign(mut task: i32, mut stateTaskAssign: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut refStateAssign: Arc<metamodelica::List<i32>>) -> Result<bool> {
    let mut sameStateAssign: bool;
    sameStateAssign = List::isEqual(metamodelica::arrayGet(stateTaskAssign.clone(), task)?, refStateAssign, true);
    Ok(sameStateAssign)
}

fn multirate_assignTasksToStates(mut tasksPerLevel: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut stateTasks: Arc<metamodelica::List<i32>>, mut odeGraphT: TaskGraph) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut stateTaskAssignOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskIdx: i32;
    let mut assignments: Arc<metamodelica::List<i32>>;
    let mut predecessors: Arc<metamodelica::List<i32>>;
    stateTaskAssignOut = arrayCreate(metamodelica::arrayLength(odeGraphT.clone()), metamodelica::nil());
    taskIdx = 1;
    for mut task in &*stateTasks {
        let mut task = task.clone();
        stateTaskAssignOut = metamodelica::arrayUpdate(stateTaskAssignOut.clone(), task.clone(), list![taskIdx])?;
        taskIdx = taskIdx + 1;
    }
    for mut levelTasks in &*tasksPerLevel.reverse() {
        let mut levelTasks = levelTasks.clone();
        for mut task in &*levelTasks.clone() {
            let mut task = task.clone();
            assignments = metamodelica::arrayGet(stateTaskAssignOut.clone(), task.clone())?;
            predecessors = metamodelica::arrayGet(odeGraphT.clone(), task.clone())?;
            stateTaskAssignOut = List::fold1(predecessors.clone(), (std::sync::Arc::new(appendToElementUnique) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<_> + 'static>), assignments.clone(), stateTaskAssignOut.clone())?;
        }
    }
    stateTaskAssignOut = Array::map1(stateTaskAssignOut.clone(), (std::sync::Arc::new(List::sort) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    Ok(stateTaskAssignOut)
}

fn appendToElementUnique<T: Clone + 'static + metamodelica::gc::MMTrace + PartialEq>(mut inIndex: i32, mut inElements: Arc<metamodelica::List<T>>, mut inArray: metamodelica::Array<Arc<metamodelica::List<T>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<T>>>> {
    let mut outArray: metamodelica::Array<Arc<metamodelica::List<T>>>;
    outArray = metamodelica::arrayUpdate(inArray.clone(), inIndex, List::unique(listAppend(({let __elt = inArray.borrow()[(inIndex-1) as usize].clone(); __elt}), inElements)))?;
    Ok(outArray)
}

fn dumpStateAssign(mut stateAssign: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("stateAssign ")); __mm_s.push_str(&*stringDelimitList(List::mapArray(stateAssign.clone(), (std::sync::Arc::new(intLstString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))?, (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

fn dumpPartitionData(mut partData: SimCode::PartitionData) -> Result<()> {
    let mut numPartitions: i32;
    let mut act: i32;
    let mut part: i32 = 0;
    let mut state: i32 = 0;
    let mut activatorsForPartitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut partitions: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut stateToActivators: Arc<metamodelica::List<i32>>;
    let SimCode::PARTITIONDATA { numPartitions: __pa0, partitions: __pa1, activatorsForPartitions: __pa2, stateToActivators: __pa3 } = (partData) else { bail!("pattern mismatch") };
    numPartitions = __pa0.clone();
    partitions = __pa1.clone();
    activatorsForPartitions = __pa2.clone();
    stateToActivators = __pa3.clone();
    metamodelica::print((literal!("Multirate Partition Data\n")).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(numPartitions)); __mm_s.push_str(&*literal!(" partitions:\n")); ArcStr::from(__mm_s) }).clone());
    act = 1;
    for mut state in &*stateToActivators.clone() {
        let mut state = state.clone();
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("activator ")); __mm_s.push_str(&*intString(act)); __mm_s.push_str(&*literal!(" is state ")); __mm_s.push_str(&*intString(state)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        act = act + 1;
    }
    metamodelica::print((literal!("\n")).clone());
    for mut part in 1..=numPartitions {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("activators: ")); __mm_s.push_str(&*intLstString((activatorsForPartitions.clone()).get(part)?)?); __mm_s.push_str(&*literal!("\t\t\t\tderStateTasks: ")); __mm_s.push_str(&*intLstString(List::map1((activatorsForPartitions.clone()).get(part)?, (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), stateToActivators.clone())?)?); __mm_s.push_str(&*literal!("\t\t\t\tnodes: \t")); __mm_s.push_str(&*intLstString((partitions.clone()).get(part)?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

//----------------------------
//  MAPPING FUNCTIONS
//----------------------------
pub(crate) fn setUpHpcOmMapping(mut daeIn: Arc<BackendDAE::BackendDAE>, mut simCodeIn: SimCode::SimCode, mut lastEqMappingIdx: i32, mut equationSccMappingIn: Arc<metamodelica::List<(i32, i32)>>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut simeqCompMapping: metamodelica::Array<i32>;
    let mut sccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut daeSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut highestSccIdx: i32;
    let mut compCountPlusDummy: i32;
    let mut equationSccMapping: Arc<metamodelica::List<(i32, i32)>>;
    let mut equationSccMapping1: Arc<metamodelica::List<(i32, i32)>>;
    let mut allComps: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>;
    (allComps, _) = getSystemComponents(daeIn)?;
    highestSccIdx = findHighestSccIdxInMapping(equationSccMappingIn.clone(), -1);
    compCountPlusDummy = (allComps.clone().len() as i32) + 1;
    equationSccMapping1 = removeDummyStateFromMapping(equationSccMappingIn.clone())?;
    equationSccMapping = if (intEq(highestSccIdx, compCountPlusDummy)) {equationSccMapping1} else {equationSccMappingIn};
    sccSimEqMapping = convertToSccSimEqMapping(equationSccMapping.clone(), (allComps.len() as i32))?;
    simeqCompMapping = convertToSimeqCompMapping(equationSccMapping, lastEqMappingIdx)?;
    daeSccSimEqMapping = metamodelica::arrayFromVec(List::map(SimCodeUtil::getRemovedEquationSimEqSysIdxes(simCodeIn)?, std::sync::Arc::new(fnptr!(List::create, _)))?.into_iter().cloned().collect());
    daeSccSimEqMapping = metamodelica::arrayAppend(sccSimEqMapping.clone(), daeSccSimEqMapping.clone());
    Ok((simeqCompMapping, sccSimEqMapping, daeSccSimEqMapping))
}

fn findHighestSccIdxInMapping(mut iEquationSccMapping: Arc<metamodelica::List<(i32, i32)>>, mut iHighestIndex: i32) -> i32 {
    let mut oIndex: i32;
    let mut eqIdx: i32;
    let mut sccIdx: i32 = 0;
    let mut rest: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    oIndex = 'mc: {
        let __mc_input = iEquationSccMapping;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (eqIdx, sccIdx), tail: rest } => {
                    let true = (intGt(sccIdx.clone(), iHighestIndex)) else { bail!("pattern mismatch") };
                    Ok(findHighestSccIdxInMapping(rest.clone(), sccIdx.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (eqIdx, sccIdx), tail: rest } => {
                    Ok(findHighestSccIdxInMapping(rest.clone(), iHighestIndex))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iHighestIndex)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oIndex
}

fn removeDummyStateFromMapping(mut iEquationSccMapping: Arc<metamodelica::List<(i32, i32)>>) -> Result<Arc<metamodelica::List<(i32, i32)>>> {
    let mut oEquationSccMapping: Arc<metamodelica::List<(i32, i32)>>;
    oEquationSccMapping = List::fold(iEquationSccMapping, (std::sync::Arc::new(fnptr!(removeDummyStateFromMapping1, (i32, i32), Arc<metamodelica::List<(i32, i32)>>)) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), Arc<metamodelica::List<(i32, i32)>>) -> Result<Arc<metamodelica::List<(i32, i32)>>> + 'static>), metamodelica::nil())?;
    Ok(oEquationSccMapping)
}

fn removeDummyStateFromMapping1(mut iTuple: (i32, i32), mut iNewList: Arc<metamodelica::List<(i32, i32)>>) -> Arc<metamodelica::List<(i32, i32)>> {
    let mut oNewList: Arc<metamodelica::List<(i32, i32)>>;
    let mut eqIdx: i32 = 0;
    let mut sccIdx: i32 = 0;
    let mut newElem: (i32, i32) = (0, 0);
    oNewList = 'mc: {
        let __mc_input = iTuple;
        if let Ok(__v) = (|| -> Result<_> {
            let (mut eqIdx, mut sccIdx) = __mc_input.clone() else { bail!("nomatch") };
            let true = (intEq(sccIdx, 1)) else { bail!("pattern mismatch") };
            Ok(iNewList.clone())
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (mut eqIdx, mut sccIdx) = __mc_input.clone() else { bail!("nomatch") };
            let mut newElem: (i32, i32) = newElem.clone();
            newElem = (eqIdx, sccIdx - 1);
            Ok((metamodelica::cons(newElem, iNewList.clone()), newElem.clone()))
        })() { newElem = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("removeDummyStateFromMapping1 failed\n")).clone());
            Ok(iNewList.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oNewList
}

fn convertToSccSimEqMapping(mut iMapping: Arc<metamodelica::List<(i32, i32)>>, mut numOfSccs: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut tmpMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    tmpMapping = arrayCreate(numOfSccs, metamodelica::nil());
    List::fold(iMapping, (std::sync::Arc::new(convertToSccSimEqMapping1) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), tmpMapping.clone())?;
    oMapping = tmpMapping.clone();
    Ok(oMapping)
}

fn convertToSccSimEqMapping1(mut iMapping: (i32, i32), mut iSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut i1: i32;
    let mut i2: i32;
    let mut tmpList: Arc<metamodelica::List<i32>>;
    (i1, i2) = iMapping;
    tmpList = metamodelica::arrayGet(iSccMapping.clone(), i2)?;
    tmpList = metamodelica::cons(i1, tmpList);
    oSccMapping = metamodelica::arrayUpdate(iSccMapping.clone(), i2, tmpList)?;
    Ok(oSccMapping)
}

fn convertToSimeqCompMapping(mut iMapping: Arc<metamodelica::List<(i32, i32)>>, mut numOfSimEqs: i32) -> Result<metamodelica::Array<i32>> {
    let mut oMapping: metamodelica::Array<i32>;
    let mut tmpMapping: metamodelica::Array<i32>;
    tmpMapping = arrayCreate(numOfSimEqs, -1);
    oMapping = List::fold(iMapping, (std::sync::Arc::new(convertToSimeqCompMapping1) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), tmpMapping.clone())?;
    Ok(oMapping)
}

fn convertToSimeqCompMapping1(mut iSimEqTuple: (i32, i32), mut iMapping: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut oMapping: metamodelica::Array<i32>;
    let mut simEqIdx: i32;
    let mut sccIdx: i32;
    (simEqIdx, sccIdx) = iSimEqTuple;
    oMapping = metamodelica::arrayUpdate(iMapping.clone(), simEqIdx, sccIdx)?;
    Ok(oMapping)
}

fn getSimEqIdxSimEqMapping(mut iAllEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut iSimEqSystemHighestIdx: i32) -> Result<metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>> {
    let mut oMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>;
    let mut tmpMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>;
    tmpMapping = arrayCreate(iSimEqSystemHighestIdx, None);
    oMapping = List::fold(iAllEquations, (std::sync::Arc::new(getSimEqIdxSimEqMapping1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>) -> Result<metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>> + 'static>), tmpMapping.clone())?;
    Ok(oMapping)
}

fn getSimEqIdxSimEqMapping1(mut iEquation: Arc<SimCode::SimEqSystem>, mut iMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>) -> Result<metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>> {
    let mut oMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>;
    let mut simEqIdx: i32 = 0;
    let mut tmpMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>> = Default::default();
    oMapping = 'mc: {
        let __mc_input = iMapping.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut simEqIdx: i32 = simEqIdx.clone();
            let mut tmpMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>> = tmpMapping.clone();
            (simEqIdx, _) = getIndexBySimCodeEq(iEquation.clone())?;
            tmpMapping = metamodelica::arrayUpdate(iMapping.clone(), simEqIdx, Some(iEquation.clone()))?;
            Ok((tmpMapping.clone(), simEqIdx.clone(), tmpMapping.clone()))
        })() { simEqIdx = __wb0; tmpMapping = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut simEqIdx: i32 = simEqIdx.clone();
            (simEqIdx, _) = getIndexBySimCodeEq(iEquation.clone())?;
            Ok((iMapping.clone(), simEqIdx.clone()))
        })() { simEqIdx = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oMapping)
}

fn getSimCodeEqByIndexAndMapping(mut iSimEqIdxSimEqMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>, mut iIdx: i32) -> Result<Arc<SimCode::SimEqSystem>> {
    let mut oSimEqSystem: Arc<SimCode::SimEqSystem>;
    let mut tmpSimEqSystem: Option<Arc<SimCode::SimEqSystem>>;
    tmpSimEqSystem = metamodelica::arrayGet(iSimEqIdxSimEqMapping.clone(), iIdx)?;
    oSimEqSystem = getSimCodeEqByIndexAndMapping1(tmpSimEqSystem, iIdx)?;
    Ok(oSimEqSystem)
}

fn getSimCodeEqByIndexAndMapping1(mut iSimEqSystem: Option<Arc<SimCode::SimEqSystem>>, mut iIdx: i32) -> Result<Arc<SimCode::SimEqSystem>> {
    let mut oSimEqSystem: Arc<SimCode::SimEqSystem>;
    let mut tmpSys: Arc<SimCode::SimEqSystem> = Arc::new(<SimCode::SimEqSystem as ::std::default::Default>::default());
    oSimEqSystem = (::match_deref::match_deref! { match &(iSimEqSystem) {
        Some(__esc_tmpSys) => {
            tmpSys = (*__esc_tmpSys).clone();
            tmpSys.clone()
        },
        _ => {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getSimCodeEqByIndexAndMapping1 failed. Looking for Index ")); __mm_s.push_str(&*intString(iIdx)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oSimEqSystem)
}

pub fn getSimCodeEqByIndex(mut iEqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut iIdx: i32) -> Result<Arc<SimCode::SimEqSystem>> {
    let mut oEq: Arc<SimCode::SimEqSystem>;
    let mut rest: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut head: Arc<SimCode::SimEqSystem> = Arc::new(<SimCode::SimEqSystem as ::std::default::Default>::default());
    let mut headIdx: i32 = 0;
    let mut headIdx2: i32 = 0;
    oEq = 'mc: {
        let __mc_input = iEqs;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
                    let mut headIdx: i32 = headIdx.clone();
                    let mut headIdx2: i32 = headIdx2.clone();
                    (headIdx, headIdx2) = getIndexBySimCodeEq(head.clone())?;
                    let true = (intEq(headIdx, iIdx) || intEq(headIdx2, iIdx)) else { bail!("pattern mismatch") };
                    Ok((head.clone(), headIdx.clone(), headIdx2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { headIdx = __wb0; headIdx2 = __wb1; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
                    Ok(getSimCodeEqByIndex(rest.clone(), iIdx)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getSimCodeEqByIndex failed. Looking for Index ")); __mm_s.push_str(&*intString(iIdx)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oEq)
}

fn getIndexBySimCodeEq(mut iEq: Arc<SimCode::SimEqSystem>) -> Result<(i32, i32)> {
    let mut oIdx: i32;
    let mut oIdx2: i32;
    let mut index: i32 = 0;
    let mut index2: i32 = 0;
    (oIdx, oIdx2) = (::match_deref::match_deref! { match &(iEq) {
        Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_IFEQUATION { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_ALGORITHM { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { index: __esc_index, .. }, alternativeTearing: None, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { index: __esc_index, .. }, alternativeTearing: None, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { index: __esc_index, .. }, alternativeTearing: Some(Deref @ SimCode::LinearSystem { index: __esc_index2, .. }), .. } => {
            index = (*__esc_index).clone();
            index2 = (*__esc_index2).clone();
            (index.clone(), index2.clone())
        },
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { index: __esc_index, .. }, alternativeTearing: Some(Deref @ SimCode::NonlinearSystem { index: __esc_index2, .. }), .. } => {
            index = (*__esc_index).clone();
            index2 = (*__esc_index2).clone();
            (index.clone(), index2.clone())
        },
        Deref @ SimCode::SimEqSystem::SES_MIXED { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_WHEN { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        Deref @ SimCode::SimEqSystem::SES_ALIAS { aliasOf: __esc_index, .. } => {
            index = (*__esc_index).clone();
            (index.clone(), 0)
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HpcOmTaskGraph.getIndexBySimCodeEq")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/HpcOmTaskGraph.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oIdx, oIdx2))
}

fn getSimCodeEqsByTaskList(mut iTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iSimEqIdxSimEqMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>) -> Result<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>> {
    let mut oSimEqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
    let mut tmpSimEqs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>>;
    tmpSimEqs = List::map1(iTaskList, (std::sync::Arc::new(getSimCodeEqsByTaskList0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>) -> Result<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>> + 'static>), iSimEqIdxSimEqMapping.clone())?;
    oSimEqs = List::flatten(tmpSimEqs)?;
    Ok(oSimEqs)
}

fn getSimCodeEqsByTaskList0(mut iTask: Arc<HpcOmSimCode::Task>, mut iSimEqIdxSimEqMapping: metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>) -> Result<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>> {
    let mut oSimEqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
    let mut eqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSimEqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    oSimEqs = (::match_deref::match_deref! { match &(iTask) {
        Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: __esc_eqIdc, .. } => {
            eqIdc = (*__esc_eqIdc).clone();
            tmpSimEqs = List::map1r(eqIdc.clone(), (std::sync::Arc::new(getSimCodeEqByIndexAndMapping) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>, i32) -> Result<Arc<SimCode::SimEqSystem>> + 'static>), iSimEqIdxSimEqMapping.clone())?;
            tmpSimEqs
        },
        Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { eqIdc: __esc_eqIdc, .. } => {
            eqIdc = (*__esc_eqIdc).clone();
            tmpSimEqs = List::map1r(eqIdc.clone(), (std::sync::Arc::new(getSimCodeEqByIndexAndMapping) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Option<Arc<SimCode::SimEqSystem>>>, i32) -> Result<Arc<SimCode::SimEqSystem>> + 'static>), iSimEqIdxSimEqMapping.clone())?;
            tmpSimEqs
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oSimEqs)
}

fn dumpSimEqSCCMapping(mut iSccMapping: metamodelica::Array<i32>) -> Result<()> {
    let mut text: ArcStr;
    text = (literal!("SimEqToSCCMapping")).clone();
    (_, text) = Array::fold(iSccMapping.clone(), (std::sync::Arc::new(fnptr!(dumpSimEqSCCMapping1, i32, (i32, ArcStr))) as std::sync::Arc<dyn ::std::ops::Fn(i32, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, text))?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*text); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

fn dumpSimEqSCCMapping1(mut iMapping: i32, mut iIndexText: (i32, ArcStr)) -> (i32, ArcStr) {
    let mut oIndexText: (i32, ArcStr);
    let mut iIndex: i32;
    let mut text: ArcStr;
    let mut iText: ArcStr;
    (iIndex, iText) = iIndexText;
    text = (intString(iMapping)).clone();
    text = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iText); __mm_s.push_str(&*literal!("\nSimEq ")); __mm_s.push_str(&*intString(iIndex)); __mm_s.push_str(&*literal!(": {")); __mm_s.push_str(&*text); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    oIndexText = (iIndex + 1, text);
    oIndexText
}

fn dumpSccSimEqMapping(mut iSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut text: ArcStr;
    text = (literal!("SccToSimEqMapping")).clone();
    (_, text) = Array::fold(iSccMapping.clone(), (std::sync::Arc::new(dumpSccSimEqMapping1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (i32, ArcStr)) -> Result<(i32, ArcStr)> + 'static>), (1, text))?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*text); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

fn dumpSccSimEqMapping1(mut iMapping: Arc<metamodelica::List<i32>>, mut iIndexText: (i32, ArcStr)) -> Result<(i32, ArcStr)> {
    let mut oIndexText: (i32, ArcStr);
    let mut iIndex: i32;
    let mut text: ArcStr;
    let mut iText: ArcStr;
    (iIndex, iText) = iIndexText;
    text = (List::fold(iMapping, (std::sync::Arc::new(fnptr!(dumpSccSimEqMapping2, i32, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(i32, ArcStr) -> Result<ArcStr> + 'static>), (literal!(" ")).clone())?).clone();
    text = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iText); __mm_s.push_str(&*literal!("\nSCC ")); __mm_s.push_str(&*intString(iIndex)); __mm_s.push_str(&*literal!(": {")); __mm_s.push_str(&*text); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
    oIndexText = (iIndex + 1, text);
    Ok(oIndexText)
}

fn dumpSccSimEqMapping2(mut iIndex: i32, mut iText: ArcStr) -> ArcStr {
    let mut oText: ArcStr;
    oText = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iText); __mm_s.push_str(&*intString(iIndex)); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
    oText
}

