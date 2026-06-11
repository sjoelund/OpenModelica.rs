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
use crate::BackendVarTransform;
use crate::HpcOmSchedulerExt;
use crate::HpcOmSimCodeMain;
use crate::HpcOmTaskGraph;
use crate::SimCodeUtil;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::HashTableCrefSimVar;
use openmodelica_simcode_types::HpcOmSimCode;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub type TaskAssignment = metamodelica::Array<i32>;

//the information which node <idx> is assigned to which processor <value>
//--------------
// No Scheduling
//--------------
pub(crate) fn createEmptySchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut allTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut taskIdx: i32 = 0;
    let mut weighting: i32;
    let mut index: i32;
    let mut threadIdx: i32;
    let mut calcTime: metamodelica::Real;
    let mut timeFinished: metamodelica::Real;
    let mut eqIdc: Arc<metamodelica::List<i32>>;
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
    allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta, (std::sync::Arc::new(convertNodeToTask) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>));
    for mut taskIdx in &*List::intRange(metamodelica::arrayLength(allCalcTasks.clone())).reverse() {
        let mut taskIdx = taskIdx.clone();
        let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(metamodelica::arrayGet(allCalcTasks.clone(), taskIdx)?) {
            (Deref @ HpcOmSimCode::Task::CALCTASK { weighting: __pa0, index: __pa1, calcTime: __pa2, timeFinished: __pa3, threadIdx: __pa4, eqIdc: __pa5 }, _) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        weighting = __pa0.clone();
        index = __pa1.clone();
        calcTime = __pa2.clone();
        timeFinished = __pa3.clone();
        threadIdx = __pa4.clone();
        eqIdc = __pa5.clone();
        eqIdc = List::map(List::map1(eqIdc.clone(), (std::sync::Arc::new(getSimEqSysIdxForComp) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iSccSimEqMapping.clone())?, (std::sync::Arc::new(List::last) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
        allTasks = metamodelica::cons(Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting, index: index, calcTime: calcTime, timeFinished: timeFinished, threadIdx: threadIdx, eqIdc: eqIdc.clone() }), allTasks.clone());
    }
    allTasks = List::sort(allTasks, (std::sync::Arc::new(compareTasksByEqIdc) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
    oSchedule = Arc::new(HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: allTasks, masterOnly: true } });
    Ok(oSchedule)
}

//----------------
// List Scheduling
//----------------
pub(crate) fn createListSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeList_refCount: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>;
    let mut nodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut rootNodes: Arc<metamodelica::List<i32>>;
    let mut threadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let HpcOmTaskGraph::TASKGRAPHMETA { commCosts: __pa0, inComps: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    commCosts = __pa0.clone();
    inComps = __pa1.clone();
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
    rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
    allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta, (std::sync::Arc::new(convertNodeToTask) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>));
    nodeList_refCount = List::map1(rootNodes, (std::sync::Arc::new(getTaskByIndex) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>) -> Result<(Arc<HpcOmSimCode::Task>, i32)> + 'static>), allCalcTasks.clone())?;
    nodeList = List::map(nodeList_refCount, std::sync::Arc::new(fnptr!(Util::tuple21, _)))?;
    nodeList = List::sort(nodeList, (std::sync::Arc::new(compareTasksByWeighting) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
    threadReadyTimes = arrayCreate(iNumberOfThreads, metamodelica::OrderedFloat(0.0_f64));
    threadTasks = arrayCreate(iNumberOfThreads, metamodelica::nil());
    tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
    (tmpSchedule, _) = createListSchedule1(nodeList, threadReadyTimes.clone(), iTaskGraph.clone(), taskGraphT.clone(), commCosts.clone(), inComps.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), (std::sync::Arc::new(getLocksByPredecessorList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>), tmpSchedule)?;
    tmpSchedule = addSuccessorLocksToSchedule(iTaskGraph.clone(), (std::sync::Arc::new(addReleaseLocksToSchedule) as std::sync::Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), commCosts.clone(), inComps.clone(), iSimVarMapping.clone(), tmpSchedule)?;
    oSchedule = setScheduleLockIds(tmpSchedule)?;
    Ok(oSchedule)
}

fn createListSchedule1(mut iNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iThreadReadyTimes: metamodelica::Array<metamodelica::Real>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iLockWithPredecessorHandler: Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>, mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<(Arc<HpcOmSimCode::Schedule>, metamodelica::Array<metamodelica::Real>)> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>;

    '__tco: loop {
        let mut head: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
        let mut newTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
        let mut newTaskRefCount: i32 = 0;
        let mut rest: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
        let mut lastChildFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        let mut lastChild: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
        let mut predecessors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
        let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
        let mut successorIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
        let mut newOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
        let mut threadFinishTimes: metamodelica::Array<metamodelica::Real> = Default::default();
        let mut firstEq: i32;
        let mut allThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = Default::default();
        let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
        let mut lockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
        let mut threadId: i32 = 0;
        let mut threadFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        let mut tmpThreadReadyTimes: metamodelica::Array<metamodelica::Real> = Default::default();
        let mut tmpNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
        let mut weighting: i32 = 0;
        let mut index: i32 = 0;
        let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        let mut eqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut simEqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = Arc::new(<HpcOmSimCode::Schedule as ::std::default::Default>::default());
        let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = Default::default();
        ::match_deref::match_deref! { match &((iNodeList, iSchedule.clone())) {
        (Deref @ metamodelica::List::Cons { head: __esc_head @ Deref @ HpcOmSimCode::Task::CALCTASK { weighting: __esc_weighting, index: __esc_index, calcTime: __esc_calcTime, eqIdc: __esc_eqIdc @ Deref @ metamodelica::List::Cons { head: __esc_firstEq, tail: _ }, .. }, tail: __esc_rest }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __esc_allThreadTasks, outgoingDepTasks: __esc_outgoingDepTasks, allCalcTasks: __esc_allCalcTasks, .. }) => {
            head = (*__esc_head).clone();
            weighting = (*__esc_weighting).clone();
            index = (*__esc_index).clone();
            calcTime = (*__esc_calcTime).clone();
            eqIdc = (*__esc_eqIdc).clone();
            firstEq = (*__esc_firstEq).clone();
            rest = (*__esc_rest).clone();
            allThreadTasks = (*__esc_allThreadTasks).clone();
            outgoingDepTasks = (*__esc_outgoingDepTasks).clone();
            allCalcTasks = (*__esc_allCalcTasks).clone();
            (predecessors, _) = getSuccessorsByTask(head.clone(), iTaskGraphT.clone(), allCalcTasks.clone())?;
            (successors, successorIdc) = getSuccessorsByTask(head.clone(), iTaskGraph.clone(), allCalcTasks.clone())?;
            if boolNot(predecessors.clone().is_empty()) {
                lastChild = getTaskWithHighestFinishTime(predecessors.clone(), None)?;
                let __pa0 = ::match_deref::match_deref! { match &(lastChild) {
                    Deref @ HpcOmSimCode::Task::CALCTASK { timeFinished: __pa0, .. } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                lastChildFinishTime = __pa0.clone();
            } else {
                lastChildFinishTime = metamodelica::OrderedFloat(0.0_f64);
            }
            threadFinishTimes = calculateFinishTimes(lastChildFinishTime, head.clone(), predecessors.clone(), iCommCosts.clone(), iThreadReadyTimes.clone());
            (threadId, threadFinishTime) = getThreadFinishTimesMin(1, threadFinishTimes.clone(), -1, metamodelica::OrderedFloat(0.0_f64));
            tmpThreadReadyTimes = metamodelica::arrayUpdate(iThreadReadyTimes.clone(), threadId, threadFinishTime)?;
            threadTasks = metamodelica::arrayGet(allThreadTasks.clone(), threadId)?;
            if boolNot(predecessors.clone().is_empty()) {
                (lockTasks, newOutgoingDepTasks) = iLockWithPredecessorHandler(head.clone(), predecessors, threadId, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                outgoingDepTasks = listAppend(outgoingDepTasks.clone(), newOutgoingDepTasks);
                threadTasks = listAppend(lockTasks, threadTasks);
                simEqIdc = List::map(List::map1(eqIdc.clone(), (std::sync::Arc::new(getSimEqSysIdxForComp) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iSccSimEqMapping.clone())?, (std::sync::Arc::new(List::last) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
            } else {
                simEqIdc = List::flatten(List::map1(eqIdc.clone(), (std::sync::Arc::new(getSimEqSysIdxForComp) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iSccSimEqMapping.clone())?)?;
            }
            newTask = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: threadFinishTime, threadIdx: threadId, eqIdc: simEqIdc });
            threadTasks = metamodelica::cons(newTask.clone(), threadTasks);
            allThreadTasks = metamodelica::arrayUpdate(allThreadTasks.clone(), threadId, threadTasks)?;
            (allCalcTasks, tmpNodeList) = updateRefCounterBySuccessorIdc(allCalcTasks.clone(), successorIdc, metamodelica::nil());
            tmpNodeList = listAppend(tmpNodeList, rest.clone());
            tmpNodeList = List::sort(tmpNodeList, (std::sync::Arc::new(compareTasksByWeighting) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
            (_, newTaskRefCount) = metamodelica::arrayGet(allCalcTasks.clone(), index.clone())?;
            metamodelica::arrayUpdate(allCalcTasks.clone(), index.clone(), (newTask, newTaskRefCount))?;
            { (iNodeList, iThreadReadyTimes, iTaskGraph, iTaskGraphT, iCommCosts, iCompTaskMapping, iSccSimEqMapping, iSimVarMapping, iLockWithPredecessorHandler, iSchedule) = (tmpNodeList, tmpThreadReadyTimes.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler.clone(), Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() })); continue '__tco; }
        },
        (Deref @ metamodelica::List::Nil, _) => return Ok((iSchedule, iThreadReadyTimes.clone())),
        _ => {
            metamodelica::print((literal!("HpcOmScheduler.createListSchedule1 failed\n")).clone());
            return Ok((iSchedule, iThreadReadyTimes.clone()))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

//----------------
// Random Scheduling
//----------------
pub(crate) fn createRandomSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeList_refCount: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>;
    let mut nodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut rootNodes: Arc<metamodelica::List<i32>>;
    let mut threadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let HpcOmTaskGraph::TASKGRAPHMETA { commCosts: __pa0, inComps: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    commCosts = __pa0.clone();
    inComps = __pa1.clone();
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
    rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
    allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta, (std::sync::Arc::new(convertNodeToTask) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>));
    nodeList_refCount = List::map1(rootNodes, (std::sync::Arc::new(getTaskByIndex) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>) -> Result<(Arc<HpcOmSimCode::Task>, i32)> + 'static>), allCalcTasks.clone())?;
    nodeList = List::map(nodeList_refCount, std::sync::Arc::new(fnptr!(Util::tuple21, _)))?;
    nodeList = List::sort(nodeList, (std::sync::Arc::new(compareTasksByWeighting) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
    threadReadyTimes = arrayCreate(iNumberOfThreads, metamodelica::OrderedFloat(0.0_f64));
    threadTasks = arrayCreate(iNumberOfThreads, metamodelica::nil());
    tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
    (tmpSchedule, _) = createRandomSchedule1(nodeList, threadReadyTimes.clone(), iTaskGraph.clone(), taskGraphT.clone(), commCosts.clone(), inComps.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), (std::sync::Arc::new(getLocksByPredecessorList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>), iNumberOfThreads, tmpSchedule)?;
    tmpSchedule = addSuccessorLocksToSchedule(iTaskGraph.clone(), (std::sync::Arc::new(addReleaseLocksToSchedule) as std::sync::Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), commCosts.clone(), inComps.clone(), iSimVarMapping.clone(), tmpSchedule)?;
    oSchedule = setScheduleLockIds(tmpSchedule)?;
    Ok(oSchedule)
}

fn createRandomSchedule1(mut iNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iThreadReadyTimes: metamodelica::Array<metamodelica::Real>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iLockWithPredecessorHandler: Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>, mut iNumberOfThreads: i32, mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<(Arc<HpcOmSimCode::Schedule>, metamodelica::Array<metamodelica::Real>)> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>;

    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut oThreadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut head: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut newTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut newTaskRefCount: i32 = 0;
    let mut rest: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut predecessors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut successorIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut newOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut threadFinishTimes: metamodelica::Array<metamodelica::Real> = Default::default();
    let mut firstEq: i32;
    let mut allThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = Default::default();
    let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut lockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut threadId: i32 = 0;
    let mut threadFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut tmpThreadReadyTimes: metamodelica::Array<metamodelica::Real> = Default::default();
    let mut tmpNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut weighting: i32 = 0;
    let mut index: i32 = 0;
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut eqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut simEqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = Arc::new(<HpcOmSimCode::Schedule as ::std::default::Default>::default());
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = Default::default();
    (oSchedule, oThreadReadyTimes) = 'mc: {
        let __mc_input = (iNodeList, iSchedule.clone());
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10, __wb11, __wb12, __wb13, __wb14)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ Deref @ HpcOmSimCode::Task::CALCTASK { weighting, index, calcTime, eqIdc: eqIdc @ Deref @ metamodelica::List::Cons { head: firstEq, tail: _ }, .. }, tail: rest }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks, outgoingDepTasks, allCalcTasks, .. }) => {
                    let mut allThreadTasks = (*allThreadTasks).clone();
                    let mut outgoingDepTasks = (*outgoingDepTasks).clone();
                    let mut allCalcTasks = (*allCalcTasks).clone();
                    let mut lockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = lockTasks.clone();
                    let mut newOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = newOutgoingDepTasks.clone();
                    let mut newTask: Arc<HpcOmSimCode::Task> = newTask.clone();
                    let mut newTaskRefCount: i32 = newTaskRefCount.clone();
                    let mut predecessors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = predecessors.clone();
                    let mut simEqIdc: Arc<metamodelica::List<i32>> = simEqIdc.clone();
                    let mut successorIdc: Arc<metamodelica::List<i32>> = successorIdc.clone();
                    let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = successors.clone();
                    let mut threadFinishTime: metamodelica::Real = threadFinishTime.clone();
                    let mut threadFinishTimes: metamodelica::Array<metamodelica::Real> = threadFinishTimes.clone();
                    let mut threadId: i32 = threadId.clone();
                    let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = threadTasks.clone();
                    let mut tmpNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = tmpNodeList.clone();
                    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = tmpSchedule.clone();
                    let mut tmpThreadReadyTimes: metamodelica::Array<metamodelica::Real> = tmpThreadReadyTimes.clone();
                    (predecessors, _) = getSuccessorsByTask(head.clone(), iTaskGraphT.clone(), allCalcTasks.clone())?;
                    (successors, successorIdc) = getSuccessorsByTask(head.clone(), iTaskGraph.clone(), allCalcTasks.clone())?;
                    let false = (predecessors.clone().is_empty()) else { bail!("pattern mismatch") };
                    threadId = System::intRandom(iNumberOfThreads) + 1;
                    threadFinishTimes = calculateFinishTimes(metamodelica::OrderedFloat(0.0_f64), head.clone(), metamodelica::nil(), iCommCosts.clone(), iThreadReadyTimes.clone());
                    threadFinishTime = metamodelica::arrayGet(threadFinishTimes.clone(), threadId)?;
                    tmpThreadReadyTimes = metamodelica::arrayUpdate(iThreadReadyTimes.clone(), threadId, threadFinishTime)?;
                    threadTasks = metamodelica::arrayGet(allThreadTasks.clone(), threadId)?;
                    (lockTasks, newOutgoingDepTasks) = iLockWithPredecessorHandler(head.clone(), predecessors.clone(), threadId, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                    outgoingDepTasks = listAppend(outgoingDepTasks.clone(), newOutgoingDepTasks.clone());
                    threadTasks = listAppend(lockTasks.clone(), threadTasks.clone());
                    simEqIdc = List::map(List::map1(eqIdc.clone(), (std::sync::Arc::new(getSimEqSysIdxForComp) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iSccSimEqMapping.clone())?, (std::sync::Arc::new(List::last) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
                    newTask = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: threadFinishTime, threadIdx: threadId, eqIdc: simEqIdc.clone() });
                    threadTasks = metamodelica::cons(newTask.clone(), threadTasks.clone());
                    allThreadTasks = metamodelica::arrayUpdate(allThreadTasks.clone(), threadId, threadTasks.clone())?;
                    (allCalcTasks, tmpNodeList) = updateRefCounterBySuccessorIdc(allCalcTasks.clone(), successorIdc.clone(), metamodelica::nil());
                    tmpNodeList = listAppend(tmpNodeList.clone(), rest.clone());
                    tmpNodeList = List::sort(tmpNodeList.clone(), (std::sync::Arc::new(compareTasksByWeighting) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
                    (_, newTaskRefCount) = metamodelica::arrayGet(allCalcTasks.clone(), index.clone())?;
                    metamodelica::arrayUpdate(allCalcTasks.clone(), index.clone(), (newTask.clone(), newTaskRefCount))?;
                    (tmpSchedule, tmpThreadReadyTimes) = createRandomSchedule1(tmpNodeList.clone(), tmpThreadReadyTimes.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler.clone(), iNumberOfThreads, Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() }))?;
                    Ok(((tmpSchedule.clone(), tmpThreadReadyTimes.clone()), lockTasks.clone(), newOutgoingDepTasks.clone(), newTask.clone(), newTaskRefCount.clone(), predecessors.clone(), simEqIdc.clone(), successorIdc.clone(), successors.clone(), threadFinishTime.clone(), threadFinishTimes.clone(), threadId.clone(), threadTasks.clone(), tmpNodeList.clone(), tmpSchedule.clone(), tmpThreadReadyTimes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { lockTasks = __wb0; newOutgoingDepTasks = __wb1; newTask = __wb2; newTaskRefCount = __wb3; predecessors = __wb4; simEqIdc = __wb5; successorIdc = __wb6; successors = __wb7; threadFinishTime = __wb8; threadFinishTimes = __wb9; threadId = __wb10; threadTasks = __wb11; tmpNodeList = __wb12; tmpSchedule = __wb13; tmpThreadReadyTimes = __wb14; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10, __wb11)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ Deref @ HpcOmSimCode::Task::CALCTASK { weighting, index, calcTime, eqIdc: eqIdc @ Deref @ metamodelica::List::Cons { head: firstEq, tail: _ }, .. }, tail: rest }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks, outgoingDepTasks, allCalcTasks, .. }) => {
                    let mut allThreadTasks = (*allThreadTasks).clone();
                    let mut allCalcTasks = (*allCalcTasks).clone();
                    let mut newTask: Arc<HpcOmSimCode::Task> = newTask.clone();
                    let mut newTaskRefCount: i32 = newTaskRefCount.clone();
                    let mut simEqIdc: Arc<metamodelica::List<i32>> = simEqIdc.clone();
                    let mut successorIdc: Arc<metamodelica::List<i32>> = successorIdc.clone();
                    let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = successors.clone();
                    let mut threadFinishTime: metamodelica::Real = threadFinishTime.clone();
                    let mut threadFinishTimes: metamodelica::Array<metamodelica::Real> = threadFinishTimes.clone();
                    let mut threadId: i32 = threadId.clone();
                    let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = threadTasks.clone();
                    let mut tmpNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = tmpNodeList.clone();
                    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = tmpSchedule.clone();
                    let mut tmpThreadReadyTimes: metamodelica::Array<metamodelica::Real> = tmpThreadReadyTimes.clone();
                    (successors, successorIdc) = getSuccessorsByTask(head.clone(), iTaskGraph.clone(), allCalcTasks.clone())?;
                    threadId = System::intRandom(iNumberOfThreads) + 1;
                    threadFinishTimes = calculateFinishTimes(metamodelica::OrderedFloat(0.0_f64), head.clone(), metamodelica::nil(), iCommCosts.clone(), iThreadReadyTimes.clone());
                    threadFinishTime = metamodelica::arrayGet(threadFinishTimes.clone(), threadId)?;
                    tmpThreadReadyTimes = metamodelica::arrayUpdate(iThreadReadyTimes.clone(), threadId, threadFinishTime)?;
                    threadTasks = metamodelica::arrayGet(allThreadTasks.clone(), threadId)?;
                    simEqIdc = List::flatten(List::map1(eqIdc.clone(), (std::sync::Arc::new(getSimEqSysIdxForComp) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iSccSimEqMapping.clone())?)?;
                    newTask = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: threadFinishTime, threadIdx: threadId, eqIdc: simEqIdc.clone() });
                    allThreadTasks = metamodelica::arrayUpdate(allThreadTasks.clone(), threadId, metamodelica::cons(newTask.clone(), threadTasks.clone()))?;
                    (allCalcTasks, tmpNodeList) = updateRefCounterBySuccessorIdc(allCalcTasks.clone(), successorIdc.clone(), metamodelica::nil());
                    tmpNodeList = listAppend(tmpNodeList.clone(), rest.clone());
                    tmpNodeList = List::sort(tmpNodeList.clone(), (std::sync::Arc::new(compareTasksByWeighting) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
                    (_, newTaskRefCount) = metamodelica::arrayGet(allCalcTasks.clone(), index.clone())?;
                    metamodelica::arrayUpdate(allCalcTasks.clone(), index.clone(), (newTask.clone(), newTaskRefCount))?;
                    (tmpSchedule, tmpThreadReadyTimes) = createRandomSchedule1(tmpNodeList.clone(), tmpThreadReadyTimes.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler.clone(), iNumberOfThreads, Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() }))?;
                    Ok(((tmpSchedule.clone(), tmpThreadReadyTimes.clone()), newTask.clone(), newTaskRefCount.clone(), simEqIdc.clone(), successorIdc.clone(), successors.clone(), threadFinishTime.clone(), threadFinishTimes.clone(), threadId.clone(), threadTasks.clone(), tmpNodeList.clone(), tmpSchedule.clone(), tmpThreadReadyTimes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { newTask = __wb0; newTaskRefCount = __wb1; simEqIdc = __wb2; successorIdc = __wb3; successors = __wb4; threadFinishTime = __wb5; threadFinishTimes = __wb6; threadId = __wb7; threadTasks = __wb8; tmpNodeList = __wb9; tmpSchedule = __wb10; tmpThreadReadyTimes = __wb11; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok((iSchedule.clone(), iThreadReadyTimes.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("HpcOmScheduler.createRandomSchedule1 failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oSchedule, oThreadReadyTimes))
}

//------------------------
// List Scheduling reverse
//------------------------
pub(crate) fn createListScheduleReverse(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodeList_refCount: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>;
    let mut nodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut leaveNodes: Arc<metamodelica::List<i32>>;
    let mut threadReadyTimes: metamodelica::Array<metamodelica::Real>;
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut commCostsT: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let HpcOmTaskGraph::TASKGRAPHMETA { commCosts: __pa0, inComps: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    commCosts = __pa0.clone();
    inComps = __pa1.clone();
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
    commCostsT = HpcOmTaskGraph::transposeCommCosts(commCosts.clone())?;
    leaveNodes = HpcOmTaskGraph::getLeafNodes(iTaskGraph.clone())?;
    allCalcTasks = convertTaskGraphToTasks(iTaskGraph.clone(), iTaskGraphMeta, (std::sync::Arc::new(convertNodeToTaskReverse) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>));
    nodeList_refCount = List::map1(leaveNodes, (std::sync::Arc::new(getTaskByIndex) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>) -> Result<(Arc<HpcOmSimCode::Task>, i32)> + 'static>), allCalcTasks.clone())?;
    nodeList = List::map(nodeList_refCount, std::sync::Arc::new(fnptr!(Util::tuple21, _)))?;
    nodeList = List::sort(nodeList, (std::sync::Arc::new(compareTasksByWeighting) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
    threadReadyTimes = arrayCreate(iNumberOfThreads, metamodelica::OrderedFloat(0.0_f64));
    threadTasks = arrayCreate(iNumberOfThreads, metamodelica::nil());
    tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
    (tmpSchedule, _) = createListSchedule1(nodeList, threadReadyTimes.clone(), taskGraphT.clone(), iTaskGraph.clone(), commCostsT.clone(), inComps.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), (std::sync::Arc::new(getLockTasksByPredecessorListReverse) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>), tmpSchedule)?;
    tmpSchedule = addSuccessorLocksToSchedule(taskGraphT.clone(), (std::sync::Arc::new(addAssignLocksToSchedule) as std::sync::Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), commCosts.clone(), inComps.clone(), iSimVarMapping.clone(), tmpSchedule)?;
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(tmpSchedule) {
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __pa2, outgoingDepTasks: __pa3, .. } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    threadTasks = __pa2.clone();
    outgoingDepTasks = __pa3.clone();
    threadTasks = Array::map(threadTasks.clone(), Arc::new(fnptr!(metamodelica::listReverse, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)))?;
    tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks, scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
    oSchedule = setScheduleLockIds(tmpSchedule)?;
    Ok(oSchedule)
}

fn addSuccessorLocksToSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCreateLockFunction: Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>;

    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut allThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = Default::default();
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = Arc::new(<HpcOmSimCode::Schedule as ::std::default::Default>::default());
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = Default::default();
    oSchedule = (::match_deref::match_deref! { match &(iSchedule) {
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __esc_allThreadTasks, outgoingDepTasks: __esc_outgoingDepTasks, allCalcTasks: __esc_allCalcTasks, .. } => {
            allThreadTasks = (*__esc_allThreadTasks).clone();
            outgoingDepTasks = (*__esc_outgoingDepTasks).clone();
            allCalcTasks = (*__esc_allCalcTasks).clone();
            (allThreadTasks, _) = Array::fold(allThreadTasks.clone(), (std::sync::Arc::new({ let __pe_b1 = iTaskGraph.clone(); let __pe_b2 = allCalcTasks.clone(); let __pe_b3 = iSimVarMapping.clone(); let __pe_b4 = iCommCosts.clone(); let __pe_b5 = iCompTaskMapping.clone(); let __pe_b6: Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static> = iCreateLockFunction.clone(); move |__pe_a0, __pe_a7| addSuccessorLocksToSchedule0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_a7) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, (metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32)) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32)> + 'static>), (allThreadTasks.clone(), 1))?;
            tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            tmpSchedule
        },
        _ => {
            metamodelica::print((literal!("HpcOmScheduler.addReleaseLocksToSchedule failed\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oSchedule)
}

fn addSuccessorLocksToSchedule0(mut iThreadTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCreateLockFunction: Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>, mut iThreadTasks: (metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32)) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32)> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>;

    let mut oThreadTasks: (metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32);
    let mut threadId: i32;
    let mut allThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    (allThreadTasks, threadId) = iThreadTasks;
    threadTasks = List::fold(iThreadTaskList, (std::sync::Arc::new({ let __pe_b1 = iTaskGraph.clone(); let __pe_b2 = iAllCalcTasks.clone(); let __pe_b3 = iSimVarMapping.clone(); let __pe_b4 = iCommCosts.clone(); let __pe_b5 = iCompTaskMapping.clone(); let __pe_b6 = (threadId, iCreateLockFunction.clone()); move |__pe_a0, __pe_a7| addSuccessorLocksToSchedule1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_a7) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), metamodelica::nil())?;
    allThreadTasks = metamodelica::arrayUpdate(allThreadTasks.clone(), threadId, threadTasks)?;
    oThreadTasks = (allThreadTasks.clone(), threadId + 1);
    Ok(oThreadTasks)
}

fn addSuccessorLocksToSchedule1(mut iTask: Arc<HpcOmSimCode::Task>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iThreadIdLockFunction: (i32, Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), mut iThreadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>;

    let mut oThreadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut threadIdx: i32 = 0;
    let mut index: i32;
    let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut tmpThreadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut releaseTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut iCreateLockFunction: Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>;
    oThreadTasks = (::match_deref::match_deref! { match &((iTask.clone(), iThreadIdLockFunction, iThreadTasks)) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { threadIdx: __esc_threadIdx, index: __esc_index, .. }, (_, __esc_iCreateLockFunction), __esc_tmpThreadTasks) => {
            threadIdx = (*__esc_threadIdx).clone();
            index = (*__esc_index).clone();
            iCreateLockFunction = (*__esc_iCreateLockFunction).clone();
            tmpThreadTasks = (*__esc_tmpThreadTasks).clone();
            (successors, _) = getSuccessorsByTask(iTask.clone(), iTaskGraph.clone(), iAllCalcTasks.clone())?;
            successors = List::removeOnTrue(threadIdx.clone(), (std::sync::Arc::new(compareTaskWithThreadIdx) as std::sync::Arc<dyn ::std::ops::Fn(i32, (Arc<HpcOmSimCode::Task>, i32)) -> Result<bool> + 'static>), successors)?;
            releaseTasks = List::fold4(successors, iCreateLockFunction.clone(), iTask.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone(), metamodelica::nil())?;
            tmpThreadTasks = listAppend(releaseTasks, tmpThreadTasks.clone());
            tmpThreadTasks = metamodelica::cons(iTask, tmpThreadTasks.clone());
            tmpThreadTasks.clone()
        },
        (_, _, __esc_tmpThreadTasks) => {
            tmpThreadTasks = (*__esc_tmpThreadTasks).clone();
            tmpThreadTasks = metamodelica::cons(iTask, tmpThreadTasks.clone());
            tmpThreadTasks.clone()
        },
        _ => {
            metamodelica::print((literal!("HpcOmScheduler.addReleaseLocksToSchedule0 failed\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oThreadTasks)
}

fn addReleaseLocksToSchedule(mut iSuccessorTask: (Arc<HpcOmSimCode::Task>, i32), mut iTask: Arc<HpcOmSimCode::Task>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iReleaseTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut oReleaseTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut tmpTask: Arc<HpcOmSimCode::Task>;
    let mut successorTask: Arc<HpcOmSimCode::Task>;
    (successorTask, _) = iSuccessorTask;
    tmpTask = createDepTaskAndCommunicationInfo(iTask, successorTask, true, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
    oReleaseTasks = metamodelica::cons(tmpTask, iReleaseTasks);
    Ok(oReleaseTasks)
}

fn addAssignLocksToSchedule(mut iSuccessorTask: (Arc<HpcOmSimCode::Task>, i32), mut iTask: Arc<HpcOmSimCode::Task>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iReleaseTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut oReleaseTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut tmpTask: Arc<HpcOmSimCode::Task>;
    let mut successorTask: Arc<HpcOmSimCode::Task>;
    (successorTask, _) = iSuccessorTask;
    tmpTask = createDepTaskAndCommunicationInfo(successorTask, iTask, false, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
    oReleaseTasks = metamodelica::cons(tmpTask, iReleaseTasks);
    Ok(oReleaseTasks)
}

fn getSimEqSysIdxForComp(mut compIdx: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut simEqSysIdcs: Arc<metamodelica::List<i32>>;
    simEqSysIdcs = metamodelica::arrayGet(iSccSimEqMapping.clone(), compIdx)?;
    Ok(simEqSysIdcs)
}

fn getSimEqSysIdcsForCompLst(mut compIdcs: Arc<metamodelica::List<i32>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut simEqSysIdcs: Arc<metamodelica::List<i32>>;
    simEqSysIdcs = List::flatten(List::map1(compIdcs, (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), iSccSimEqMapping.clone())?)?;
    Ok(simEqSysIdcs)
}

pub(crate) fn getSimEqSysIdcsForNodeLst(mut nodeIdcs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut simEqSysIdcsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    simEqSysIdcsLst = List::map1(nodeIdcs, (std::sync::Arc::new(getSimEqSysIdcsForCompLst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iSccSimEqMapping.clone())?;
    Ok(simEqSysIdcsLst)
}

fn getLocksByPredecessorList(mut iTask: Arc<HpcOmSimCode::Task>, mut iPredecessorList: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, mut iThreadIdx: i32, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> {
    let mut oLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut oOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    oLockTasks = List::fold(iPredecessorList, (std::sync::Arc::new({ let __pe_b1 = iTask; let __pe_b2 = iThreadIdx; let __pe_b3 = iCommCosts.clone(); let __pe_b4 = iCompTaskMapping.clone(); let __pe_b5 = iSimVarMapping.clone(); move |__pe_a0, __pe_a6| Ok(getLockTasksByPredecessorList(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_a6)) }) as std::sync::Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), metamodelica::nil())?;
    oOutgoingDepTasks = oLockTasks.clone();
    Ok((oLockTasks, oOutgoingDepTasks))
}

fn getLockTasksByPredecessorList(mut iPredecessorTask: (Arc<HpcOmSimCode::Task>, i32), mut iTask: Arc<HpcOmSimCode::Task>, mut iThreadIdx: i32, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> {
    let mut oLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut threadIdx: i32 = 0;
    let mut tmpLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut tmpTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut predTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    oLockTasks = 'mc: {
        let __mc_input = (iPredecessorTask, iTask.clone(), iLockTasks.clone());
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((predTask @ Deref @ HpcOmSimCode::Task::CALCTASK { threadIdx, index: _, .. }, _), Deref @ HpcOmSimCode::Task::CALCTASK { index: _, .. }, tmpLockTasks) => {
                    let mut tmpLockTasks = (*tmpLockTasks).clone();
                    let mut tmpTask: Arc<HpcOmSimCode::Task> = tmpTask.clone();
                    let true = (intNe(iThreadIdx, threadIdx.clone())) else { bail!("pattern mismatch") };
                    tmpTask = createDepTaskAndCommunicationInfo(predTask.clone(), iTask.clone(), false, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                    tmpLockTasks = metamodelica::cons(tmpTask.clone(), tmpLockTasks.clone());
                    Ok((tmpLockTasks.clone(), tmpTask.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { tmpTask = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iLockTasks.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oLockTasks
}

fn getLockTasksByPredecessorListReverse(mut iTask: Arc<HpcOmSimCode::Task>, mut iPredecessorList: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, mut iThreadIdx: i32, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> {
    let mut oLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut oOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    oLockTasks = List::fold(iPredecessorList, (std::sync::Arc::new({ let __pe_b1 = iTask; let __pe_b2 = iThreadIdx; let __pe_b3 = iCommCosts.clone(); let __pe_b4 = iCompTaskMapping.clone(); let __pe_b5 = iSimVarMapping.clone(); move |__pe_a0, __pe_a6| Ok(getLockTasksByPredecessorListReverse0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_a6)) }) as std::sync::Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), metamodelica::nil())?;
    oOutgoingDepTasks = oLockTasks.clone();
    Ok((oLockTasks, oOutgoingDepTasks))
}

fn getLockTasksByPredecessorListReverse0(mut iPredecessorTask: (Arc<HpcOmSimCode::Task>, i32), mut iTask: Arc<HpcOmSimCode::Task>, mut iThreadIdx: i32, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> {
    let mut oLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut index: i32;
    let mut threadIdx: i32 = 0;
    let mut predTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut tmpTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut tmpLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oLockTasks = 'mc: {
        let __mc_input = iPredecessorTask;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (predTask @ Deref @ HpcOmSimCode::Task::CALCTASK { threadIdx, index, .. }, _) => {
                    let mut tmpLockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = tmpLockTasks.clone();
                    let mut tmpTask: Arc<HpcOmSimCode::Task> = tmpTask.clone();
                    let true = (intNe(iThreadIdx, threadIdx.clone())) else { bail!("pattern mismatch") };
                    tmpTask = createDepTaskAndCommunicationInfo(iTask.clone(), predTask.clone(), true, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                    tmpLockTasks = metamodelica::cons(tmpTask.clone(), iLockTasks.clone());
                    Ok((tmpLockTasks.clone(), tmpLockTasks.clone(), tmpTask.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { tmpLockTasks = __wb0; tmpTask = __wb1; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iLockTasks.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oLockTasks
}

fn getCommunicationObjBetweenMergedTasks(mut parentNode: i32, mut node: i32, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>) -> Result<HpcOmTaskGraph::Communication> {
    let mut oCommunication: HpcOmTaskGraph::Communication;
    let mut nodeTasks: Arc<metamodelica::List<i32>>;
    let mut parentTasks: Arc<metamodelica::List<i32>>;
    let mut commFold: HpcOmTaskGraph::Communication;
    let mut edgesFromParents: Arc<metamodelica::List<HpcOmTaskGraph::Communication>>;
    nodeTasks = metamodelica::arrayGet(inComps.clone(), node)?;
    parentTasks = metamodelica::arrayGet(inComps.clone(), parentNode)?;
    commFold = HpcOmTaskGraph::Communication { numberOfVars: 0, integerVars: metamodelica::nil(), floatVars: metamodelica::nil(), booleanVars: metamodelica::nil(), stringVars: metamodelica::nil(), childNode: node, requiredTime: metamodelica::OrderedFloat(-1.0_f64) };
    edgesFromParents = List::flatten(List::map1(parentTasks, (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), inCommCosts.clone())?)?;
    oCommunication = List::fold(edgesFromParents, (std::sync::Arc::new({ let __pe_b1 = nodeTasks; move |__pe_a0, __pe_a2| Ok(getCommunicationObjBetweenMergedTasks1(__pe_a0, __pe_b1.clone(), __pe_a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(HpcOmTaskGraph::Communication, HpcOmTaskGraph::Communication) -> Result<HpcOmTaskGraph::Communication> + 'static>), commFold)?;
    Ok(oCommunication)
}

fn getCommunicationObjBetweenMergedTasks1(mut parentCommCost: HpcOmTaskGraph::Communication, mut tasks: Arc<metamodelica::List<i32>>, mut iCommunication: HpcOmTaskGraph::Communication) -> HpcOmTaskGraph::Communication {
    let mut oCommunication: HpcOmTaskGraph::Communication;
    oCommunication = 'mc: {
        let __mc_input = (parentCommCost, iCommunication.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (HpcOmTaskGraph::Communication { numberOfVars: mut nV1, integerVars: ref ints1, floatVars: ref fl1, booleanVars: ref b1, stringVars: ref s1, childNode: mut childNode, requiredTime: mut reqT1 }, HpcOmTaskGraph::Communication { numberOfVars: mut nV2, integerVars: ref ints2, floatVars: ref fl2, booleanVars: ref b2, stringVars: ref s2, childNode: _, requiredTime: mut reqT2 }) = __mc_input.clone() else { bail!("nomatch") };
            let true = (listMember(childNode.clone(), tasks.clone())) else { bail!("pattern mismatch") };
            Ok(HpcOmTaskGraph::Communication { numberOfVars: nV1.clone() + nV2.clone(), integerVars: listAppend(ints1.clone(), ints2.clone()), floatVars: listAppend(fl1.clone(), fl2.clone()), booleanVars: listAppend(b1.clone(), b2.clone()), stringVars: listAppend(s1.clone(), s2.clone()), childNode: childNode.clone(), requiredTime: reqT1.clone() + reqT2.clone() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iCommunication.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oCommunication
}

fn convertCommunicationToCommInfo(mut iCommunication: HpcOmTaskGraph::Communication, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<HpcOmSimCode::CommunicationInfo> {
    let mut oCommInfo: HpcOmSimCode::CommunicationInfo;
    let mut integerVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut floatVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut booleanVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut intSimVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut floatSimVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolSimVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    oCommInfo = (match iCommunication {
        HpcOmTaskGraph::Communication { integerVars: mut __esc_integerVars, floatVars: mut __esc_floatVars, booleanVars: mut __esc_booleanVars, .. } => {
            integerVars = __esc_integerVars.clone();
            floatVars = __esc_floatVars.clone();
            booleanVars = __esc_booleanVars.clone();
            intSimVars = List::fold1(integerVars.clone(), (std::sync::Arc::new(convertVarIdxToSimVar) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Arc<metamodelica::List<SimCodeVar::SimVar>>> + 'static>), iSimVarMapping.clone(), metamodelica::nil())?;
            floatSimVars = List::fold1(floatVars.clone(), (std::sync::Arc::new(convertVarIdxToSimVar) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Arc<metamodelica::List<SimCodeVar::SimVar>>> + 'static>), iSimVarMapping.clone(), metamodelica::nil())?;
            boolSimVars = List::fold1(booleanVars.clone(), (std::sync::Arc::new(convertVarIdxToSimVar) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Arc<metamodelica::List<SimCodeVar::SimVar>>> + 'static>), iSimVarMapping.clone(), metamodelica::nil())?;
            HpcOmSimCode::CommunicationInfo { floatVars: floatSimVars, intVars: intSimVars, boolVars: boolSimVars }
        },
    });
    Ok(oCommInfo)
}

fn convertVarIdxToSimVar(mut iVarIdx: i32, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iSimVar: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Arc<metamodelica::List<SimCodeVar::SimVar>>> {
    let mut oSimVar: Arc<metamodelica::List<SimCodeVar::SimVar>>;
    let mut tmpSimVars: Arc<metamodelica::List<SimCodeVar::SimVar>>;
    tmpSimVars = metamodelica::arrayGet(iSimVarMapping.clone(), iVarIdx)?;
    oSimVar = listAppend(iSimVar, tmpSimVars);
    Ok(oSimVar)
}

fn createDepTask(mut iSourceTask: Arc<HpcOmSimCode::Task>, mut iTargetTask: Arc<HpcOmSimCode::Task>, mut iOutgoing: bool, mut commInfo: HpcOmSimCode::CommunicationInfo) -> Arc<HpcOmSimCode::Task> {
    let mut oAssignTask: Arc<HpcOmSimCode::Task>;
    oAssignTask = Arc::new(HpcOmSimCode::Task::DEPTASK { sourceTask: iSourceTask, targetTask: iTargetTask, outgoing: iOutgoing, id: 0, communicationInfo: commInfo });
    oAssignTask
}

fn createDepTaskAndCommunicationInfo(mut iSourceTask: Arc<HpcOmSimCode::Task>, mut iTargetTask: Arc<HpcOmSimCode::Task>, mut iOutgoing: bool, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oAssignTask: Arc<HpcOmSimCode::Task>;
    let mut predIndex: i32 = 0;
    let mut taskIndex: i32 = 0;
    let mut tmpTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut commBetweenTasks: HpcOmTaskGraph::Communication = <HpcOmTaskGraph::Communication as ::std::default::Default>::default();
    let mut commInfo: HpcOmSimCode::CommunicationInfo = <HpcOmSimCode::CommunicationInfo as ::std::default::Default>::default();
    oAssignTask = 'mc: {
        let __mc_input = (iSourceTask.clone(), iTargetTask.clone());
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ HpcOmSimCode::Task::CALCTASK { index: predIndex, .. }, Deref @ HpcOmSimCode::Task::CALCTASK { index: taskIndex, .. }) => {
                    let mut commBetweenTasks: HpcOmTaskGraph::Communication = commBetweenTasks.clone();
                    let mut commInfo: HpcOmSimCode::CommunicationInfo = commInfo.clone();
                    let mut tmpTask: Arc<HpcOmSimCode::Task> = tmpTask.clone();
                    commBetweenTasks = getCommunicationObjBetweenMergedTasks(predIndex.clone(), taskIndex.clone(), iCompTaskMapping.clone(), iCommCosts.clone())?;
                    commInfo = convertCommunicationToCommInfo(commBetweenTasks.clone(), iSimVarMapping.clone())?;
                    tmpTask = createDepTask(iSourceTask.clone(), iTargetTask.clone(), iOutgoing, commInfo.clone());
                    Ok((tmpTask.clone(), commBetweenTasks.clone(), commInfo.clone(), tmpTask.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { commBetweenTasks = __wb0; commInfo = __wb1; tmpTask = __wb2; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("CreateDepTaskAndCommunicationInfo failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oAssignTask)
}

fn createDepTaskByTaskIdc(mut iSourceTaskIdx: i32, mut iTargetTaskIdx: i32, mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, mut iOutgoing: bool, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oAssignTask: Arc<HpcOmSimCode::Task>;
    let mut sourceTask: Arc<HpcOmSimCode::Task>;
    let mut targetTask: Arc<HpcOmSimCode::Task>;
    sourceTask = Util::tuple21(metamodelica::arrayGet(iAllCalcTasks.clone(), iSourceTaskIdx)?);
    targetTask = Util::tuple21(metamodelica::arrayGet(iAllCalcTasks.clone(), iTargetTaskIdx)?);
    oAssignTask = createDepTaskAndCommunicationInfo(sourceTask, targetTask, iOutgoing, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
    Ok(oAssignTask)
}

fn createDepTaskByTaskIdcR(mut iSourceTaskIdx: i32, mut iTargetTaskIdx: i32, mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, mut iOutgoing: bool, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oAssignTask: Arc<HpcOmSimCode::Task>;
    oAssignTask = createDepTaskByTaskIdc(iTargetTaskIdx, iSourceTaskIdx, iAllCalcTasks.clone(), iOutgoing, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
    Ok(oAssignTask)
}

fn updateRefCounterBySuccessorIdc(mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, mut iSuccessorIdc: Arc<metamodelica::List<i32>>, mut iRefZeroTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> (metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) {
    let mut oAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut oRefZeroTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut head: i32 = 0;
    let mut currentRefCount: i32 = 0;
    let mut rest: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpRefZeroTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut currentTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut tmpAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = Default::default();
    (oAllCalcTasks, oRefZeroTasks) = 'mc: {
        let __mc_input = iSuccessorIdc;
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
                    let mut currentRefCount: i32 = currentRefCount.clone();
                    let mut currentTask: Arc<HpcOmSimCode::Task> = currentTask.clone();
                    let mut tmpAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = tmpAllCalcTasks.clone();
                    let mut tmpRefZeroTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = tmpRefZeroTasks.clone();
                    (currentTask, currentRefCount) = metamodelica::arrayGet(iAllCalcTasks.clone(), head.clone())?;
                    let true = (intEq(currentRefCount, 1)) else { bail!("pattern mismatch") };
                    tmpAllCalcTasks = metamodelica::arrayUpdate(iAllCalcTasks.clone(), head.clone(), (currentTask.clone(), 0))?;
                    tmpRefZeroTasks = metamodelica::cons(currentTask.clone(), iRefZeroTasks.clone());
                    (tmpAllCalcTasks, tmpRefZeroTasks) = updateRefCounterBySuccessorIdc(tmpAllCalcTasks.clone(), rest.clone(), tmpRefZeroTasks.clone());
                    Ok(((tmpAllCalcTasks.clone(), tmpRefZeroTasks.clone()), currentRefCount.clone(), currentTask.clone(), tmpAllCalcTasks.clone(), tmpRefZeroTasks.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { currentRefCount = __wb0; currentTask = __wb1; tmpAllCalcTasks = __wb2; tmpRefZeroTasks = __wb3; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: rest } => {
                    let mut currentRefCount: i32 = currentRefCount.clone();
                    let mut currentTask: Arc<HpcOmSimCode::Task> = currentTask.clone();
                    let mut tmpAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = tmpAllCalcTasks.clone();
                    let mut tmpRefZeroTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = tmpRefZeroTasks.clone();
                    (currentTask, currentRefCount) = metamodelica::arrayGet(iAllCalcTasks.clone(), head.clone())?;
                    tmpAllCalcTasks = metamodelica::arrayUpdate(iAllCalcTasks.clone(), head.clone(), (currentTask.clone(), currentRefCount - 1))?;
                    (tmpAllCalcTasks, tmpRefZeroTasks) = updateRefCounterBySuccessorIdc(tmpAllCalcTasks.clone(), rest.clone(), iRefZeroTasks.clone());
                    Ok(((tmpAllCalcTasks.clone(), tmpRefZeroTasks.clone()), currentRefCount.clone(), currentTask.clone(), tmpAllCalcTasks.clone(), tmpRefZeroTasks.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { currentRefCount = __wb0; currentTask = __wb1; tmpAllCalcTasks = __wb2; tmpRefZeroTasks = __wb3; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((iAllCalcTasks.clone(), iRefZeroTasks.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (oAllCalcTasks, oRefZeroTasks)
}

fn getThreadFinishTimesMin(mut iThreadIdx: i32, mut iThreadFinishTimes: metamodelica::Array<metamodelica::Real>, mut iCurrentMinThreadIdx: i32, mut iCurrentMinFinishTime: metamodelica::Real) -> (i32, metamodelica::Real) {
    let mut minThreadTime_Idx: (i32, metamodelica::Real);
    let mut threadFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    minThreadTime_Idx = 'mc: {
        let __mc_input = iCurrentMinFinishTime;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (intGt(iThreadIdx, metamodelica::arrayLength(iThreadFinishTimes.clone()))) else { bail!("pattern mismatch") };
            Ok((iCurrentMinThreadIdx, iCurrentMinFinishTime))
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut threadFinishTime: metamodelica::Real = threadFinishTime.clone();
            threadFinishTime = metamodelica::arrayGet(iThreadFinishTimes.clone(), iThreadIdx)?;
            let true = (realLt(threadFinishTime, iCurrentMinFinishTime) || intEq(iCurrentMinThreadIdx, -1)) else { bail!("pattern mismatch") };
            Ok((getThreadFinishTimesMin(iThreadIdx + 1, iThreadFinishTimes.clone(), iThreadIdx, threadFinishTime), threadFinishTime.clone()))
        })() { threadFinishTime = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(getThreadFinishTimesMin(iThreadIdx + 1, iThreadFinishTimes.clone(), iCurrentMinThreadIdx, iCurrentMinFinishTime))
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    minThreadTime_Idx
}

fn getTaskWithHighestFinishTime(mut iTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, mut iCurrentTask: Option<Arc<HpcOmSimCode::Task>>) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oTask: Arc<HpcOmSimCode::Task>;
    let mut head: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut tmpTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut tail: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut timeFinishedHead: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeFinishedCurrent: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    oTask = 'mc: {
        let __mc_input = (iTasks, iCurrentTask.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (head, _), tail: tail }, None) => {
                    Ok(getTaskWithHighestFinishTime(tail.clone(), Some(head.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (head @ Deref @ HpcOmSimCode::Task::CALCTASK { timeFinished: timeFinishedHead, .. }, _), tail: tail }, Some(Deref @ HpcOmSimCode::Task::CALCTASK { timeFinished: timeFinishedCurrent, .. })) => {
                    let true = (realGt(timeFinishedHead.clone(), timeFinishedCurrent.clone())) else { bail!("pattern mismatch") };
                    Ok(getTaskWithHighestFinishTime(tail.clone(), Some(head.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: (head, _), tail: tail }, Some(_)) => {
                    Ok(getTaskWithHighestFinishTime(tail.clone(), iCurrentTask.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Some(tmpTask)) => {
                    Ok(tmpTask.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("HpcOmScheduler.getTaskWithHighestFinishTime failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oTask)
}

fn convertTaskGraphToTasks(mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iConverterFunc: Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>) -> metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>;

    let mut oTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut tmpTaskArray: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    tmpTaskArray = arrayCreate(metamodelica::arrayLength(iTaskGraphT.clone()), (openmodelica_simcode_types::HpcOmSimCode::Task::interned_TASKEMPTY(), 0));
    oTasks = convertTaskGraphToTasks1(iTaskGraphMeta, iTaskGraphT.clone(), 1, iConverterFunc.clone(), tmpTaskArray.clone());
    oTasks
}

fn convertTaskGraphToTasks1(mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iIndex: i32, mut iConverterFunc: Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>, mut iTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>) -> metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>;

    let mut oTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut tmpTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = Default::default();
    let mut refCount: i32 = 0;
    let mut newTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    oTasks = 'mc: {
        let __mc_input = iTasks.clone();
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut newTask: Arc<HpcOmSimCode::Task> = newTask.clone();
            let mut refCount: i32 = refCount.clone();
            let mut tmpTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = tmpTasks.clone();
            let true = (intLe(iIndex, metamodelica::arrayLength(iTaskGraphT.clone()))) else { bail!("pattern mismatch") };
            refCount = (metamodelica::arrayGet(iTaskGraphT.clone(), iIndex)?.len() as i32);
            newTask = iConverterFunc(iIndex, iTaskGraphMeta.clone())?;
            tmpTasks = metamodelica::arrayUpdate(iTasks.clone(), iIndex, (newTask.clone(), refCount))?;
            tmpTasks = convertTaskGraphToTasks1(iTaskGraphMeta.clone(), iTaskGraphT.clone(), iIndex + 1, iConverterFunc.clone(), tmpTasks.clone());
            Ok((tmpTasks.clone(), newTask.clone(), refCount.clone(), tmpTasks.clone()))
        })() { newTask = __wb0; refCount = __wb1; tmpTasks = __wb2; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iTasks.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oTasks
}

fn convertNodeToTask(mut iNodeIdx: i32, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oTask: Arc<HpcOmSimCode::Task>;
    let mut nodeMark: i32 = 0;
    let mut primalComp: i32 = 0;
    let mut components: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut exeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut nodeMarks: metamodelica::Array<i32> = Default::default();
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    oTask = (match iTaskGraphMeta.clone() {
        HpcOmTaskGraph::TaskGraphMeta { inComps: mut __esc_inComps, nodeMark: mut __esc_nodeMarks, exeCosts: mut __esc_exeCosts, .. } => {
            inComps = __esc_inComps.clone();
            nodeMarks = __esc_nodeMarks.clone();
            exeCosts = __esc_exeCosts.clone();
            components = metamodelica::arrayGet(inComps.clone(), iNodeIdx)?;
            primalComp = (components.clone()).get(1)?;
            nodeMark = metamodelica::arrayGet(nodeMarks.clone(), primalComp)?;
            (_, exeCost) = HpcOmTaskGraph::getExeCost(iNodeIdx, iTaskGraphMeta)?;
            Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: nodeMark, index: iNodeIdx, calcTime: exeCost, timeFinished: metamodelica::OrderedFloat(-1.0_f64), threadIdx: -1, eqIdc: components })
        },
        _ => {
            metamodelica::print((literal!("HpcOmScheduler.convertNodeToTask failed!\n")).clone());
            bail!("fail")
        },
    });
    Ok(oTask)
}

fn convertNodeToTaskReverse(mut iNodeIdx: i32, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oTask: Arc<HpcOmSimCode::Task>;
    let mut nodeMark: i32 = 0;
    let mut primalComp: i32 = 0;
    let mut components: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut exeCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut nodeMarks: metamodelica::Array<i32> = Default::default();
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)> = Default::default();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    oTask = (match iTaskGraphMeta {
        HpcOmTaskGraph::TaskGraphMeta { inComps: mut __esc_inComps, nodeMark: mut __esc_nodeMarks, exeCosts: mut __esc_exeCosts, .. } => {
            inComps = __esc_inComps.clone();
            nodeMarks = __esc_nodeMarks.clone();
            exeCosts = __esc_exeCosts.clone();
            components = metamodelica::arrayGet(inComps.clone(), iNodeIdx)?;
            primalComp = (components.clone()).get(1)?;
            nodeMark = metamodelica::arrayGet(nodeMarks.clone(), primalComp)?;
            (_, exeCost) = metamodelica::arrayGet(exeCosts.clone(), iNodeIdx)?;
            nodeMark = nodeMark * -1;
            Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: nodeMark, index: iNodeIdx, calcTime: exeCost, timeFinished: metamodelica::OrderedFloat(-1.0_f64), threadIdx: -1, eqIdc: components })
        },
        _ => {
            metamodelica::print((literal!("HpcOmScheduler.convertNodeToTask failed!\n")).clone());
            bail!("fail")
        },
    });
    Ok(oTask)
}

fn calculateFinishTimes(mut iPredecessorTaskLastFinished: metamodelica::Real, mut iTask: Arc<HpcOmSimCode::Task>, mut iPredecessorTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iThreadReadyTimes: metamodelica::Array<metamodelica::Real>) -> metamodelica::Array<metamodelica::Real> {
    let mut oFinishTimes: metamodelica::Array<metamodelica::Real>;
    let mut tmpFinishTimes: metamodelica::Array<metamodelica::Real>;
    tmpFinishTimes = arrayCreate(metamodelica::arrayLength(iThreadReadyTimes.clone()), metamodelica::OrderedFloat(0.0_f64));
    tmpFinishTimes = calculateFinishTimes1(iPredecessorTaskLastFinished, iTask, iPredecessorTasks, iCommCosts.clone(), iThreadReadyTimes.clone(), 1, tmpFinishTimes.clone());
    oFinishTimes = tmpFinishTimes.clone();
    oFinishTimes
}

fn calculateFinishTimes1(mut iPredecessorTaskLastFinished: metamodelica::Real, mut iTask: Arc<HpcOmSimCode::Task>, mut iPredecessorTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iThreadReadyTimes: metamodelica::Array<metamodelica::Real>, mut iThreadIdx: i32, mut iFinishTimes: metamodelica::Array<metamodelica::Real>) -> metamodelica::Array<metamodelica::Real> {
    let mut oFinishTimes: metamodelica::Array<metamodelica::Real>;
    let mut thFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut thReadyTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut tmpFinishTimes: metamodelica::Array<metamodelica::Real> = Default::default();
    oFinishTimes = 'mc: {
        let __mc_input = iFinishTimes.clone();
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut thFinishTime: metamodelica::Real = thFinishTime.clone();
            let mut thReadyTime: metamodelica::Real = thReadyTime.clone();
            let mut tmpFinishTimes: metamodelica::Array<metamodelica::Real> = tmpFinishTimes.clone();
            let true = (intLe(iThreadIdx, metamodelica::arrayLength(iThreadReadyTimes.clone()))) else { bail!("pattern mismatch") };
            thReadyTime = metamodelica::arrayGet(iThreadReadyTimes.clone(), iThreadIdx)?;
            thFinishTime = calculateFinishTimeByThreadId(thReadyTime, iPredecessorTaskLastFinished, iThreadIdx, iTask.clone(), iPredecessorTasks.clone(), iCommCosts.clone())?;
            tmpFinishTimes = metamodelica::arrayUpdate(iFinishTimes.clone(), iThreadIdx, thFinishTime)?;
            Ok((calculateFinishTimes1(iPredecessorTaskLastFinished, iTask.clone(), iPredecessorTasks.clone(), iCommCosts.clone(), iThreadReadyTimes.clone(), iThreadIdx + 1, tmpFinishTimes.clone()), thFinishTime.clone(), thReadyTime.clone(), tmpFinishTimes.clone()))
        })() { thFinishTime = __wb0; thReadyTime = __wb1; tmpFinishTimes = __wb2; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iFinishTimes.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oFinishTimes
}

fn calculateFinishTimeByThreadId(mut iThreadReadyTime: metamodelica::Real, mut iPredecessorTaskLastFinished: metamodelica::Real, mut iThreadId: i32, mut iTask: Arc<HpcOmSimCode::Task>, mut iPredecessorTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>) -> Result<metamodelica::Real> {
    let mut oFinishTime: metamodelica::Real;
    let mut predecessorTasksOtherTh: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut commCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut startTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    oFinishTime = (::match_deref::match_deref! { match &(iTask.clone()) {
        Deref @ HpcOmSimCode::Task::CALCTASK { calcTime: __esc_calcTime, .. } => {
            calcTime = (*__esc_calcTime).clone();
            predecessorTasksOtherTh = List::removeOnTrue(iThreadId, (std::sync::Arc::new(compareTaskWithThreadIdx) as std::sync::Arc<dyn ::std::ops::Fn(i32, (Arc<HpcOmSimCode::Task>, i32)) -> Result<bool> + 'static>), iPredecessorTasks)?;
            startTime = realMax(iThreadReadyTime, iPredecessorTaskLastFinished);
            commCost = getMaxCommCostsByTaskList(iTask, predecessorTasksOtherTh, iCommCosts.clone())?;
            ((startTime) + (commCost)) + (calcTime.clone())
        },
        _ => {
            metamodelica::print((literal!("HpcOmScheduler.calculateFinishTimeByThreadId can only handle CALCTASKs\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oFinishTime)
}

fn getMaxCommCostsByTaskList(mut iParentTask: Arc<HpcOmSimCode::Task>, mut iTaskList: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>) -> Result<metamodelica::Real> {
    let mut oCommCost: metamodelica::Real;
    oCommCost = List::fold2(iTaskList, (std::sync::Arc::new(fnptr!(getMaxCommCostsByTaskList1, (Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), iParentTask, iCommCosts.clone(), metamodelica::OrderedFloat(0.0_f64))?;
    Ok(oCommCost)
}

fn getMaxCommCostsByTaskList1(mut iTask: (Arc<HpcOmSimCode::Task>, i32), mut iParentTask: Arc<HpcOmSimCode::Task>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCurrentMax: metamodelica::Real) -> metamodelica::Real {
    let mut oCommCost: metamodelica::Real;
    let mut reqCycles: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut eqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut parentEqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut childCommCosts: Arc<metamodelica::List<HpcOmTaskGraph::Communication>> = metamodelica::nil();
    oCommCost = 'mc: {
        let __mc_input = (iTask, iParentTask);
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((Deref @ HpcOmSimCode::Task::CALCTASK { index: _, eqIdc, .. }, _), Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: parentEqIdc, .. }) => {
                    let mut childCommCosts: Arc<metamodelica::List<HpcOmTaskGraph::Communication>> = childCommCosts.clone();
                    let mut reqCycles: metamodelica::Real = reqCycles.clone();
                    childCommCosts = metamodelica::arrayGet(iCommCosts.clone(), listHead(eqIdc.clone())?)?;
                    let HpcOmTaskGraph::COMMUNICATION { requiredTime: __pa0, .. } = (getMaxCommCostsByTaskList2(childCommCosts.clone(), listHead(parentEqIdc.clone())?)?) else { bail!("pattern mismatch") };
                    reqCycles = __pa0.clone();
                    let true = (realGt(reqCycles, iCurrentMax)) else { bail!("pattern mismatch") };
                    Ok((reqCycles, childCommCosts.clone(), reqCycles.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { childCommCosts = __wb0; reqCycles = __wb1; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iCurrentMax)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oCommCost
}

fn getMaxCommCostsByTaskList2(mut iCommCosts: Arc<metamodelica::List<HpcOmTaskGraph::Communication>>, mut iIdx: i32) -> Result<HpcOmTaskGraph::Communication> {
    let mut oComm: HpcOmTaskGraph::Communication;
    let mut childIdxHead: i32 = 0;
    let mut tail: Arc<metamodelica::List<HpcOmTaskGraph::Communication>> = metamodelica::nil();
    let mut head: HpcOmTaskGraph::Communication = <HpcOmTaskGraph::Communication as ::std::default::Default>::default();
    oComm = 'mc: {
        let __mc_input = iCommCosts;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head @ HpcOmTaskGraph::Communication { childNode: childIdxHead, .. }, tail: tail } => {
                    let true = (intEq(childIdxHead.clone(), iIdx)) else { bail!("pattern mismatch") };
                    Ok(head.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: tail } => {
                    Ok(getMaxCommCostsByTaskList2(tail.clone(), iIdx)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("HpcOmScheduler.getMaxCommCostsByTaskList2 failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oComm)
}

fn getTaskByIndex(mut iTaskIdx: i32, mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>) -> Result<(Arc<HpcOmSimCode::Task>, i32)> {
    let mut oTask: (Arc<HpcOmSimCode::Task>, i32);
    oTask = metamodelica::arrayGet(iAllCalcTasks.clone(), iTaskIdx)?;
    Ok(oTask)
}

pub(crate) fn getSuccessorsByTask(mut iTask: Arc<HpcOmSimCode::Task>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>) -> Result<(Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, Arc<metamodelica::List<i32>>)> {
    let mut oTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>;
    let mut oTaskIdc: Arc<metamodelica::List<i32>>;
    let mut taskIdx: i32 = 0;
    let mut successors: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    (oTasks, oTaskIdc) = 'mc: {
        let __mc_input = iTask;
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Task::CALCTASK { index: taskIdx, .. } => {
                    let mut successors: Arc<metamodelica::List<i32>> = successors.clone();
                    let mut tmpTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = tmpTasks.clone();
                    successors = metamodelica::arrayGet(iTaskGraph.clone(), taskIdx.clone())?;
                    tmpTasks = List::map1(successors.clone(), (std::sync::Arc::new(getTaskByIndex) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>) -> Result<(Arc<HpcOmSimCode::Task>, i32)> + 'static>), iAllCalcTasks.clone())?;
                    Ok(((tmpTasks.clone(), successors.clone()), successors.clone(), tmpTasks.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { successors = __wb0; tmpTasks = __wb1; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("HpcOmScheduler.getSuccessorsByTask can only handle CALCTASKs.")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oTasks, oTaskIdc))
}

fn compareTasksByWeighting(mut iTask1: Arc<HpcOmSimCode::Task>, mut iTask2: Arc<HpcOmSimCode::Task>) -> Result<bool> {
    let mut oResult: bool;
    let mut weightingTask1: i32 = 0;
    let mut weightingTask2: i32 = 0;
    oResult = (::match_deref::match_deref! { match &((iTask1.clone(), iTask2.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { weighting: __esc_weightingTask1, .. }, Deref @ HpcOmSimCode::Task::CALCTASK { weighting: __esc_weightingTask2, .. }) => {
            weightingTask1 = (*__esc_weightingTask1).clone();
            weightingTask2 = (*__esc_weightingTask2).clone();
            intGt(weightingTask1.clone(), weightingTask2.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HpcOmScheduler.compareTasksByWeighting can only compare CALCTASKs! Task 1 has type ")); __mm_s.push_str(&*getTaskTypeString(iTask1)); __mm_s.push_str(&*literal!(" and task 2 has type ")); __mm_s.push_str(&*getTaskTypeString(iTask2)); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oResult)
}

fn compareTasksByEqIdc(mut iTask1: Arc<HpcOmSimCode::Task>, mut iTask2: Arc<HpcOmSimCode::Task>) -> Result<bool> {
    let mut oResult: bool;
    let mut eqIdcTask1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqIdcTask2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oResult = (::match_deref::match_deref! { match &((iTask1.clone(), iTask2.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: __esc_eqIdcTask1, .. }, Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: __esc_eqIdcTask2, .. }) => {
            eqIdcTask1 = (*__esc_eqIdcTask1).clone();
            eqIdcTask2 = (*__esc_eqIdcTask2).clone();
            intGt(List::last(eqIdcTask1.clone())?, List::last(eqIdcTask2.clone())?)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HpcOmScheduler.compareTasksByEqIdc can only compare CALCTASKs with at least one equation index! Task 1 has type ")); __mm_s.push_str(&*getTaskTypeString(iTask1)); __mm_s.push_str(&*literal!(" and task 2 has type ")); __mm_s.push_str(&*getTaskTypeString(iTask2)); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oResult)
}

fn compareTaskWithThreadIdx(mut iThreadIdx: i32, mut iTask1: (Arc<HpcOmSimCode::Task>, i32)) -> Result<bool> {
    let mut oMatch: bool;
    let mut threadIdx: i32 = 0;
    oMatch = (::match_deref::match_deref! { match &(iTask1) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { threadIdx: __esc_threadIdx, .. }, _) => {
            threadIdx = (*__esc_threadIdx).clone();
            intEq(threadIdx.clone(), iThreadIdx)
        },
        _ => {
            metamodelica::print((literal!("HpcOmScheduler.compareTaskWithThreadIdx can only compare CALCTASKs!\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oMatch)
}

fn dumpThreadSchedule(mut iTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iThreadIdx: i32) -> Result<(ArcStr, i32)> {
    let mut r#str: ArcStr;
    let mut oThreadIdx: i32;
    r#str = (literal!("--------------\n")).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("Thread ")); __mm_s.push_str(&*intString(iThreadIdx)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("--------------\n")); ArcStr::from(__mm_s) }).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*dumpTaskList(iTaskList)?); ArcStr::from(__mm_s) }).clone();
    oThreadIdx = iThreadIdx + 1;
    Ok((r#str, oThreadIdx))
}

fn dumpTaskDepSchedule(mut iTaskInfo: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: ArcStr;
    let mut iTask: Arc<HpcOmSimCode::Task>;
    let mut iDependencies: Arc<metamodelica::List<i32>>;
    (iTask, iDependencies) = iTaskInfo;
    s = (literal!("Task: \n")).clone();
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*dumpTask(iTask)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("-> Parents: ")); __mm_s.push_str(&*stringDelimitList(List::map(iDependencies, (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("---------------------\n")); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

fn printTaskList(mut iTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<()> {
    metamodelica::print((dumpTaskList(iTaskList)?).clone());
    Ok(())
}

fn dumpTaskList(mut iTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(List::map(iTaskList, (std::sync::Arc::new(dumpTask) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>) -> Result<ArcStr> + 'static>))?, (literal!("")).clone());
    Ok(r#str)
}

fn dumpTask(mut iTask: Arc<HpcOmSimCode::Task>) -> Result<ArcStr> {
    let mut oString: ArcStr;
    let mut weighting: i32;
    let mut index: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut compIdx: i32 = 0;
    let mut numThreads: i32 = 0;
    let mut sourceIndex: i32 = 0;
    let mut targetIndex: i32 = 0;
    let mut eqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut timeFinished: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut s: ArcStr = arcstr::literal!("");
    let mut taskSchedule: Arc<HpcOmSimCode::Schedule> = Arc::new(<HpcOmSimCode::Schedule as ::std::default::Default>::default());
    let mut outgoing: bool = false;
    let mut threadIdx: i32 = 0;
    oString = ((::match_deref::match_deref! { match &(iTask) {
        Deref @ HpcOmSimCode::Task::SCHEDULED_TASK { compIdx: __esc_compIdx, numThreads: __esc_numThreads, taskSchedule: __esc_taskSchedule } => {
            compIdx = (*__esc_compIdx).clone();
            numThreads = (*__esc_numThreads).clone();
            taskSchedule = (*__esc_taskSchedule).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Scheduled Task (comp: ")); __mm_s.push_str(&*intString(compIdx.clone())); __mm_s.push_str(&*literal!(", numThreads: ")); __mm_s.push_str(&*intString(numThreads.clone())); __mm_s.push_str(&*literal!("):\n------------------------------------------------------\n")); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*System::stringReplace((dumpSchedule(taskSchedule.clone())?).clone(), (literal!("\n")).clone(), (literal!("\n\t")).clone())?); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("------------------------------------------------------\n")); ArcStr::from(__mm_s) }).clone();
            s
        },
        Deref @ HpcOmSimCode::Task::CALCTASK { weighting: __esc_weighting, timeFinished: __esc_timeFinished, index: __esc_index, eqIdc: __esc_eqIdc, .. } => {
            weighting = (*__esc_weighting).clone();
            timeFinished = (*__esc_timeFinished).clone();
            index = (*__esc_index).clone();
            eqIdc = (*__esc_eqIdc).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Calculation task with index ")); __mm_s.push_str(&*intString(index.clone())); __mm_s.push_str(&*literal!(" including the equations: ")); __mm_s.push_str(&*stringDelimitList(List::map(eqIdc.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!(" is finished at  ")); __mm_s.push_str(&*realString(timeFinished.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }
        },
        Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { eqIdc: __esc_eqIdc, nodeIdc: __esc_nodeIdc, threadIdx: None } => {
            eqIdc = (*__esc_eqIdc).clone();
            nodeIdc = (*__esc_nodeIdc).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Calculation task (")); __mm_s.push_str(&*stringDelimitList(List::map(nodeIdc.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!(") including the equations: ")); __mm_s.push_str(&*stringDelimitList(List::map(eqIdc.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }
        },
        Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { eqIdc: __esc_eqIdc, nodeIdc: __esc_nodeIdc, threadIdx: Some(__esc_threadIdx) } => {
            eqIdc = (*__esc_eqIdc).clone();
            nodeIdc = (*__esc_nodeIdc).clone();
            threadIdx = (*__esc_threadIdx).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Calculation task (")); __mm_s.push_str(&*stringDelimitList(List::map(nodeIdc.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!(") including the equations: ")); __mm_s.push_str(&*stringDelimitList(List::map(eqIdc.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); __mm_s.push_str(&*literal!(" by thread ")); __mm_s.push_str(&*intString(threadIdx.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }
        },
        Deref @ HpcOmSimCode::Task::DEPTASK { sourceTask: Deref @ HpcOmSimCode::Task::CALCTASK { index: __esc_sourceIndex, .. }, targetTask: Deref @ HpcOmSimCode::Task::CALCTASK { index: __esc_targetIndex, .. }, outgoing: __esc_outgoing, .. } => {
            sourceIndex = (*__esc_sourceIndex).clone();
            targetIndex = (*__esc_targetIndex).clone();
            outgoing = (*__esc_outgoing).clone();
            s = (literal!("Dependency task ")).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*if (outgoing.clone()) {literal!("(outgoing)")} else {literal!("(incoming)")}); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(" between ")); __mm_s.push_str(&*intString(sourceIndex.clone())); __mm_s.push_str(&*literal!(" and ")); __mm_s.push_str(&*intString(targetIndex.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            s
        },
        Deref @ HpcOmSimCode::Task::TASKEMPTY { .. } => literal!("empty task\n"),
        _ => {
            metamodelica::print((literal!("HpcOmScheduler.dumpTask failed\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(oString)
}

pub(crate) fn printTask(mut iTask: Arc<HpcOmSimCode::Task>) -> Result<()> {
    metamodelica::print((dumpTask(iTask)?).clone());
    Ok(())
}

pub(crate) fn convertScheduleStrucToInfo(mut iSchedule: Arc<HpcOmSimCode::Schedule>, mut iTaskCount: i32) -> Result<metamodelica::Array<(i32, i32, metamodelica::Real)>> {
    let mut oScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    let mut tmpScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)> = Default::default();
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = Default::default();
    let mut tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut allTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oScheduleInfo = (::match_deref::match_deref! { match &(iSchedule) {
        Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: __esc_allTasks, .. } } => {
            allTasks = (*__esc_allTasks).clone();
            tmpScheduleInfo = arrayCreate(iTaskCount, (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
            threadTasks = arrayCreate(1, allTasks.clone());
            tmpScheduleInfo = Array::fold(threadTasks.clone(), (std::sync::Arc::new(convertScheduleStrucToInfo0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, metamodelica::Array<(i32, i32, metamodelica::Real)>) -> Result<metamodelica::Array<(i32, i32, metamodelica::Real)>> + 'static>), tmpScheduleInfo.clone())?;
            tmpScheduleInfo.clone()
        },
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __esc_threadTasks, .. } => {
            threadTasks = (*__esc_threadTasks).clone();
            tmpScheduleInfo = arrayCreate(iTaskCount, (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
            tmpScheduleInfo = Array::fold(threadTasks.clone(), (std::sync::Arc::new(convertScheduleStrucToInfo0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, metamodelica::Array<(i32, i32, metamodelica::Real)>) -> Result<metamodelica::Array<(i32, i32, metamodelica::Real)>> + 'static>), tmpScheduleInfo.clone())?;
            tmpScheduleInfo.clone()
        },
        Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: __esc_tasksOfLevels, .. } => {
            tasksOfLevels = (*__esc_tasksOfLevels).clone();
            tmpScheduleInfo = arrayCreate(iTaskCount, (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
            tmpScheduleInfo = convertScheduleStrucToInfoLevel(tasksOfLevels.clone(), 1, tmpScheduleInfo.clone())?;
            tmpScheduleInfo.clone()
        },
        Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: _ } => {
            tmpScheduleInfo = arrayCreate(iTaskCount, (-1, -1, metamodelica::OrderedFloat(-1.0_f64)));
            tmpScheduleInfo.clone()
        },
        _ => {
            metamodelica::print((literal!("HpcOmScheduler.convertScheduleStrucToInfo unknown Schedule-Type.\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oScheduleInfo)
}

fn convertScheduleStrucToInfo0(mut iTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> Result<metamodelica::Array<(i32, i32, metamodelica::Real)>> {
    let mut oScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    (oScheduleInfo, _) = List::fold(iTaskList, (std::sync::Arc::new(convertScheduleStrucToInfo1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, (metamodelica::Array<(i32, i32, metamodelica::Real)>, i32)) -> Result<(metamodelica::Array<(i32, i32, metamodelica::Real)>, i32)> + 'static>), (iScheduleInfo.clone(), 1))?;
    Ok(oScheduleInfo)
}

fn convertScheduleStrucToInfo1(mut iTask: Arc<HpcOmSimCode::Task>, mut iScheduleInfo: (metamodelica::Array<(i32, i32, metamodelica::Real)>, i32)) -> Result<(metamodelica::Array<(i32, i32, metamodelica::Real)>, i32)> {
    let mut oScheduleInfo: (metamodelica::Array<(i32, i32, metamodelica::Real)>, i32);
    let mut taskIdx: i32 = 0;
    let mut taskNumber: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut tmpScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)> = Default::default();
    let mut timeFinished: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    oScheduleInfo = (::match_deref::match_deref! { match &((iTask, iScheduleInfo.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { index: __esc_taskIdx, threadIdx: __esc_threadIdx, timeFinished: __esc_timeFinished, .. }, (__esc_tmpScheduleInfo, __esc_taskNumber)) => {
            taskIdx = (*__esc_taskIdx).clone();
            threadIdx = (*__esc_threadIdx).clone();
            timeFinished = (*__esc_timeFinished).clone();
            tmpScheduleInfo = (*__esc_tmpScheduleInfo).clone();
            taskNumber = (*__esc_taskNumber).clone();
            tmpScheduleInfo = metamodelica::arrayUpdate(tmpScheduleInfo.clone(), taskIdx.clone(), (threadIdx.clone(), taskNumber.clone(), timeFinished.clone()))?;
            (tmpScheduleInfo.clone(), taskNumber.clone() + 1)
        },
        (Deref @ HpcOmSimCode::Task::DEPTASK { .. }, _) => iScheduleInfo,
        _ => {
            metamodelica::print((literal!("HpcOmScheduler.convertScheduleStrucToInfo1 failed. Unknown Task-Type.\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oScheduleInfo)
}

fn convertScheduleStrucToInfoLevel(mut taskLst: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut sectionsNumber: i32, mut iScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> Result<metamodelica::Array<(i32, i32, metamodelica::Real)>> {
    let mut oScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
    oScheduleInfo = 'mc: {
        let __mc_input = taskLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(iScheduleInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks }, tail: rest } => {
                    let mut scheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
                    scheduleInfo = convertScheduleStrucToInfoLevel1(tasks.clone(), sectionsNumber, 1, iScheduleInfo.clone())?;
                    Ok(convertScheduleStrucToInfoLevel(rest.clone(), sectionsNumber + 1, scheduleInfo.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks, .. }, tail: rest } => {
                    let mut scheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>;
                    scheduleInfo = convertScheduleStrucToInfoLevel1(tasks.clone(), sectionsNumber, 1, iScheduleInfo.clone())?;
                    Ok(convertScheduleStrucToInfoLevel(rest.clone(), sectionsNumber + 1, scheduleInfo.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("convertScheduleStrucToInfoLevel failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oScheduleInfo)
}

fn convertScheduleStrucToInfoLevel1(mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut sectionsNumber: i32, mut sectionIdx: i32, mut iScheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> Result<metamodelica::Array<(i32, i32, metamodelica::Real)>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(tasks) {
        Deref @ metamodelica::List::Nil => {
            return Ok(iScheduleInfo.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc, threadIdx: threadIdxOpt, .. }, tail: rest } => {
            let mut numNodes: i32;
            let mut threadIdx: i32;
            let mut tuplLst: Arc<metamodelica::List<(i32, i32, metamodelica::Real)>>;
            numNodes = (nodeIdc.clone().len() as i32);
            threadIdx = Util::getOptionOrDefault(threadIdxOpt.clone(), -1);
            tuplLst = List::threadMap1(List::fill(threadIdx.clone(), numNodes.clone()), List::fill(-1, numNodes.clone()), std::sync::Arc::new(fnptr!(Util::make3Tuple, _, _, _)), metamodelica::OrderedFloat(0.0_f64))?;
            List::threadMap1_0(nodeIdc.clone(), tuplLst.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), iScheduleInfo.clone())?;
            { (tasks, sectionsNumber, sectionIdx, iScheduleInfo) = (rest.clone(), sectionsNumber, sectionIdx + 1, iScheduleInfo.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

//-----------------
// Balanced Level Scheduling
//-----------------
pub(crate) fn createBalancedLevelScheduling(mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(Arc<HpcOmSimCode::Schedule>, HpcOmTaskGraph::TaskGraphMeta)> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut oMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut targetCost: metamodelica::Real;
    let mut levelAss: metamodelica::Array<i32>;
    let mut nodeMark: metamodelica::Array<i32>;
    let mut critPathNodes: Arc<metamodelica::List<i32>>;
    let mut critPathCosts: Arc<metamodelica::List<metamodelica::Real>>;
    let mut level: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut allSections: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut graphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut levelTasks: Arc<metamodelica::List<HpcOmSimCode::TaskList>>;
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
    let mut compNames: metamodelica::Array<ArcStr>;
    let mut compDescs: metamodelica::Array<ArcStr>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut compInformations: metamodelica::Array<HpcOmTaskGraph::ComponentInfo>;
    targetCost = metamodelica::OrderedFloat(1000.0_f64);
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = (iMeta.clone()) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    graphT = AdjacencyMatrix::transposeAdjacencyMatrix(iGraph.clone(), metamodelica::arrayLength(iGraph.clone()))?;
    level = HpcOmTaskGraph::getLevelNodes(iGraph.clone())?;
    levelAss = arrayCreate(metamodelica::arrayLength(inComps.clone()), -1);
    (_, levelAss) = List::fold(level.clone(), (std::sync::Arc::new(getLevelAssignment) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (i32, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>)> + 'static>), (1, levelAss.clone()))?;
    let __pa1 = ::match_deref::match_deref! { match &(HpcOmTaskGraph::getCriticalPaths(iGraph.clone(), iMeta.clone())) {
        (_, (Deref @ metamodelica::List::Cons { head: __pa1, tail: _ }, _)) => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    critPathNodes = __pa1.clone();
    critPathCosts = List::map1(critPathNodes.clone(), (std::sync::Arc::new(HpcOmTaskGraph::getExeCostReqCycles) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> + 'static>), iMeta.clone())?;
    allSections = BLS_fillParallelSections(level, levelAss.clone(), critPathNodes, 1, targetCost, iGraph.clone(), graphT.clone(), iMeta.clone(), metamodelica::nil(), metamodelica::nil())?;
    allSections = List::map2(allSections, (std::sync::Arc::new(BLS_mergeSmallSections) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, HpcOmTaskGraph::TaskGraphMeta, metamodelica::Real) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> + 'static>), iMeta.clone(), targetCost)?;
    levelTasks = List::map2(allSections.clone(), (std::sync::Arc::new(BLS_generateSchedule) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, HpcOmTaskGraph::TaskGraphMeta, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<HpcOmSimCode::TaskList> + 'static>), iMeta.clone(), iSccSimEqMapping.clone())?;
    oSchedule = Arc::new(HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: levelTasks, useFixedAssignments: false });
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa2, varCompMapping: __pa3, eqCompMapping: __pa4, compParamMapping: __pa5, compNames: __pa6, compDescs: __pa7, exeCosts: __pa8, commCosts: __pa9, compInformations: __pa10, .. } = (iMeta) else { bail!("pattern mismatch") };
    inComps = __pa2.clone();
    varCompMapping = __pa3.clone();
    eqCompMapping = __pa4.clone();
    compParamMapping = __pa5.clone();
    compNames = __pa6.clone();
    compDescs = __pa7.clone();
    exeCosts = __pa8.clone();
    commCosts = __pa9.clone();
    compInformations = __pa10.clone();
    nodeMark = arrayCreate(metamodelica::arrayLength(inComps.clone()), -1);
    level = List::map(allSections, (std::sync::Arc::new(List::flatten) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
    (_, nodeMark) = List::fold(level, (std::sync::Arc::new(getLevelAssignment) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (i32, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>)> + 'static>), (1, nodeMark.clone()))?;
    oMeta = HpcOmTaskGraph::TaskGraphMeta { inComps: inComps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
    Ok((oSchedule, oMeta))
}

fn BLS_mergeSmallSections(mut sectionsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut targetCosts: metamodelica::Real) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut sectionsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    sectionsOut = (match targetCosts {
        _ => {
            let mut costs: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
            let mut mergedSectionIdcs: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut sectionsNew: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut sectionsNewUnflattened: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>>;
            let mut sectionCosts: Arc<metamodelica::List<metamodelica::Real>>;
            costs = List::map1List(sectionsIn.clone(), (std::sync::Arc::new(HpcOmTaskGraph::getExeCostReqCycles) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> + 'static>), iMeta)?;
            sectionCosts = List::map(costs.clone(), (std::sync::Arc::new(realSum) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<metamodelica::Real>>) -> Result<metamodelica::Real> + 'static>))?;
            (mergedSectionIdcs, _) = BLS_mergeToTargetSize(List::intRange((sectionsIn.clone().len() as i32)), sectionCosts.clone(), targetCosts, metamodelica::nil())?;
            sectionsNewUnflattened = List::map1List(mergedSectionIdcs.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), sectionsIn)?;
            sectionsNew = List::map(sectionsNewUnflattened.clone(), (std::sync::Arc::new(List::flatten) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
            sectionsNew = List::map1(sectionsNew.clone(), (std::sync::Arc::new(List::sort) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            sectionsNew.clone()
        },
    });
    Ok(sectionsOut)
}

fn BLS_generateSchedule(mut level: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<HpcOmSimCode::TaskList> {
    let mut taskLstOut: HpcOmSimCode::TaskList;
    taskLstOut = 'mc: {
        let __mc_input = (level.clone(), iMeta);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: section, tail: Deref @ metamodelica::List::Nil }, HpcOmTaskGraph::TaskGraphMeta { inComps, .. }) => {
                    let mut task: Arc<HpcOmSimCode::Task>;
                    let mut taskLst: HpcOmSimCode::TaskList;
                    task = makeCalcTaskLevel(section.clone(), inComps.clone(), iSccSimEqMapping.clone())?;
                    taskLst = HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: list![task.clone()], masterOnly: true };
                    Ok(taskLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: _ }, HpcOmTaskGraph::TaskGraphMeta { inComps, .. }) => {
                    let mut taskLst: HpcOmSimCode::TaskList;
                    taskLst = makeCalcLevelParTaskLstForMergedNodes(level.clone(), iSccSimEqMapping.clone(), inComps.clone())?;
                    Ok(taskLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(taskLstOut)
}

fn BLS_fillParallelSections(mut levelIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut levelAssIn: metamodelica::Array<i32>, mut critPathNodes: Arc<metamodelica::List<i32>>, mut levelIdx: i32, mut targetCosts: metamodelica::Real, mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut unassNodesIn: Arc<metamodelica::List<i32>>, mut sectionsIn: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>>> {
    let mut sectionsOut: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>>;
    sectionsOut = 'mc: {
        let __mc_input = critPathNodes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(sectionsIn.clone().reverse())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: critPathNode, tail: Deref @ metamodelica::List::Nil } => {
                    let mut critNodeLevel: i32;
                    let mut levelNodes: Arc<metamodelica::List<i32>>;
                    let mut unassNodes: Arc<metamodelica::List<i32>>;
                    let mut levelNodeCluster: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut followingLevel: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut sectionLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>>;
                    critNodeLevel = metamodelica::arrayGet(levelAssIn.clone(), critPathNode.clone())?;
                    critNodeLevel = intMin(levelIdx, critNodeLevel.clone());
                    (_, followingLevel) = List::split(levelIn.clone(), critNodeLevel.clone() - 1)?;
                    levelNodes = List::flatten(followingLevel.clone())?;
                    unassNodes = listAppend(levelNodes.clone(), unassNodesIn.clone());
                    levelNodeCluster = BLS_mergeDependentLevelTask(unassNodes.clone(), iGraph.clone(), iGraphT.clone(), metamodelica::nil())?;
                    sectionLst = metamodelica::cons(levelNodeCluster.clone(), sectionsIn.clone());
                    sectionLst = BLS_fillParallelSections(levelIn.clone(), levelAssIn.clone(), metamodelica::nil(), critNodeLevel.clone() + 1, targetCosts, iGraph.clone(), iGraphT.clone(), iMeta.clone(), unassNodes.clone(), sectionLst.clone())?;
                    Ok(sectionLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: critPathNode, tail: restCritNodes } => {
                    let mut critPathCost: metamodelica::Real;
                    let mut critNodeLevel: i32;
                    let mut section: Arc<metamodelica::List<i32>>;
                    let mut levelNodes: Arc<metamodelica::List<i32>>;
                    let mut unassNodes: Arc<metamodelica::List<i32>>;
                    let mut necessaryPredecessors: Arc<metamodelica::List<i32>>;
                    let mut level: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut sectionLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>>;
                    critPathCost = HpcOmTaskGraph::getExeCostReqCycles(critPathNode.clone(), iMeta.clone())?;
                    critNodeLevel = metamodelica::arrayGet(levelAssIn.clone(), critPathNode.clone())?;
                    let true = (critPathCost.clone() < targetCosts) else { bail!("pattern mismatch") };
                    levelNodes = List::flatten(List::map1(List::intRange2(levelIdx, critNodeLevel.clone()), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), levelIn.clone())?)?;
                    (levelNodes, _) = List::deleteMemberOnTrue(critPathNode.clone(), levelNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    necessaryPredecessors = metamodelica::arrayGet(iGraphT.clone(), listHead(restCritNodes.clone())?)?;
                    unassNodes = listAppend(levelNodes.clone(), unassNodesIn.clone());
                    necessaryPredecessors = List::flatten(List::map4(List::map(necessaryPredecessors.clone(), std::sync::Arc::new(fnptr!(List::create, _)))?, (std::sync::Arc::new(BLS_getDependentGroups) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iGraph.clone(), iGraphT.clone(), unassNodes.clone(), metamodelica::nil())?)?;
                    necessaryPredecessors = List::unique(necessaryPredecessors.clone());
                    (necessaryPredecessors, _, unassNodes) = List::intersection1OnTrue(necessaryPredecessors.clone(), unassNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    section = metamodelica::cons(critPathNode.clone(), necessaryPredecessors.clone());
                    section = List::unique(section.clone());
                    sectionLst = metamodelica::cons(list![section.clone()], sectionsIn.clone());
                    List::map2_0(section.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), critNodeLevel.clone(), levelAssIn.clone())?;
                    level = List::map1(levelIn.clone(), (std::sync::Arc::new(deleteIntListMembers) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), section.clone())?;
                    level = List::set(level.clone(), critNodeLevel.clone(), section.clone())?;
                    sectionLst = BLS_fillParallelSections(level.clone(), levelAssIn.clone(), restCritNodes.clone(), critNodeLevel.clone() + 1, targetCosts, iGraph.clone(), iGraphT.clone(), iMeta.clone(), unassNodes.clone(), sectionLst.clone())?;
                    Ok(sectionLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: critPathNode, tail: restCritNodes } => {
                    let mut critPathCost: metamodelica::Real;
                    let mut critNodeLevel: i32;
                    let mut levelNodes: Arc<metamodelica::List<i32>>;
                    let mut unassNodes: Arc<metamodelica::List<i32>>;
                    let mut level: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut levelNodeCluster: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut sectionLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>>>;
                    critPathCost = HpcOmTaskGraph::getExeCostReqCycles(critPathNode.clone(), iMeta.clone())?;
                    critNodeLevel = metamodelica::arrayGet(levelAssIn.clone(), critPathNode.clone())?;
                    let true = (critPathCost.clone() >= targetCosts) else { bail!("pattern mismatch") };
                    levelNodes = List::flatten(List::map1(List::intRange2(levelIdx, critNodeLevel.clone()), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), levelIn.clone())?)?;
                    (levelNodes, _) = List::deleteMemberOnTrue(critPathNode.clone(), levelNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    metamodelica::arrayGet(iGraphT.clone(), listHead(restCritNodes.clone())?)?;
                    unassNodes = listAppend(unassNodesIn.clone(), levelNodes.clone());
                    unassNodes = metamodelica::cons(critPathNode.clone(), unassNodes.clone());
                    unassNodes = List::unique(unassNodes.clone());
                    levelNodeCluster = BLS_mergeDependentLevelTask(unassNodes.clone(), iGraph.clone(), iGraphT.clone(), metamodelica::nil())?;
                    (_, unassNodes, _) = List::intersection1OnTrue(unassNodes.clone(), List::flatten(levelNodeCluster.clone())?, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    sectionLst = metamodelica::cons(levelNodeCluster.clone(), sectionsIn.clone());
                    List::map2_0(List::flatten(levelNodeCluster.clone())?, (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), critNodeLevel.clone(), levelAssIn.clone())?;
                    level = List::map1(levelIn.clone(), (std::sync::Arc::new(deleteIntListMembers) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), List::flatten(levelNodeCluster.clone())?)?;
                    level = List::set(level.clone(), critNodeLevel.clone(), List::flatten(levelNodeCluster.clone())?)?;
                    sectionLst = BLS_fillParallelSections(level.clone(), levelAssIn.clone(), restCritNodes.clone(), critNodeLevel.clone() + 1, targetCosts, iGraph.clone(), iGraphT.clone(), iMeta.clone(), metamodelica::nil(), sectionLst.clone())?;
                    Ok(sectionLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(sectionsOut)
}

fn BLS_mergeDependentLevelTask(mut nodesIn: Arc<metamodelica::List<i32>>, mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut sectionsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(nodesIn.clone()) {
        Deref @ metamodelica::List::Nil => {
            return Ok(sectionsIn.reverse())
        },
        Deref @ metamodelica::List::Cons { head: node, tail: rest } => {
            let mut dependentNodes: Arc<metamodelica::List<i32>>;
            let mut section: Arc<metamodelica::List<i32>>;
            let mut sections: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut rest = (*rest).clone();
            dependentNodes = BLS_getDependentGroups(list![node.clone()], iGraph.clone(), iGraphT.clone(), nodesIn, metamodelica::nil())?;
            section = metamodelica::cons(node.clone(), dependentNodes.clone());
            section = List::unique(section.clone());
            (_, rest, _) = List::intersection1OnTrue(rest.clone(), dependentNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            section = section.clone().reverse();
            { (nodesIn, iGraph, iGraphT, sectionsIn) = (rest.clone(), iGraph.clone(), iGraphT.clone(), metamodelica::cons(section.clone(), sectionsIn)); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn BLS_getDependentGroups(mut nodes: Arc<metamodelica::List<i32>>, mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut referenceNodesIn: Arc<metamodelica::List<i32>>, mut dependentsIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut dependentsOut: Arc<metamodelica::List<i32>>;
    dependentsOut = 'mc: {
        let __mc_input = nodes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(List::unique(dependentsIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: node, tail: rest } => {
                    let mut successors: Arc<metamodelica::List<i32>>;
                    let mut predecessors: Arc<metamodelica::List<i32>>;
                    let mut dependentNodes: Arc<metamodelica::List<i32>>;
                    let mut referenceNodes: Arc<metamodelica::List<i32>>;
                    let mut allNodes: Arc<metamodelica::List<i32>>;
                    successors = metamodelica::arrayGet(iGraph.clone(), node.clone())?;
                    predecessors = metamodelica::arrayGet(iGraphT.clone(), node.clone())?;
                    (successors, _, referenceNodes) = List::intersection1OnTrue(successors.clone(), referenceNodesIn.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    (predecessors, _, referenceNodes) = List::intersection1OnTrue(predecessors.clone(), referenceNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    dependentNodes = listAppend(predecessors.clone(), successors.clone());
                    allNodes = metamodelica::cons(node.clone(), dependentNodes.clone());
                    dependentNodes = BLS_getDependentGroups(listAppend(rest.clone(), dependentNodes.clone()), iGraph.clone(), iGraphT.clone(), referenceNodes.clone(), listAppend(allNodes.clone(), dependentsIn.clone()))?;
                    Ok(dependentNodes.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("BLS_getDependentGroups failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(dependentsOut)
}

fn BLS_mergeToTargetSize(mut nodesIn: Arc<metamodelica::List<i32>>, mut costsIn: Arc<metamodelica::List<metamodelica::Real>>, mut targetSize: metamodelica::Real, mut mergedNodesIn: Arc<metamodelica::List<(Arc<metamodelica::List<i32>>, metamodelica::Real)>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<metamodelica::Real>>)> {
    let mut clustersOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut clusterCostsOut: Arc<metamodelica::List<metamodelica::Real>>;
    (clustersOut, clusterCostsOut) = 'mc: {
        let __mc_input = (nodesIn, costsIn, mergedNodesIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil, _) => {
                    let mut cluster: Arc<metamodelica::List<i32>>;
                    let mut clusterTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut clusterCostsTmp: Arc<metamodelica::List<metamodelica::Real>>;
                    clusterCostsTmp = List::map(mergedNodesIn.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)))?;
                    clusterTmp = List::map(mergedNodesIn.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)))?.reverse();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(clusterTmp.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    cluster = __pa0.clone();
                    clusterTmp = __pa1.clone();
                    cluster = if (clusterTmp.clone().is_empty()) {cluster.clone().reverse()} else {cluster.clone()};
                    clusterTmp = metamodelica::cons(cluster.clone(), clusterTmp.clone());
                    Ok((clusterTmp.clone(), clusterCostsTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: nodeRest }, Deref @ metamodelica::List::Cons { head: cost, tail: costRest }, Deref @ metamodelica::List::Nil) => {
                    let mut clusterTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut clusterCostsTmp: Arc<metamodelica::List<metamodelica::Real>>;
                    (clusterTmp, clusterCostsTmp) = BLS_mergeToTargetSize(nodeRest.clone(), costRest.clone(), targetSize, list![(list![node.clone()], cost.clone())])?;
                    Ok((clusterTmp.clone(), clusterCostsTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: nodeRest }, Deref @ metamodelica::List::Cons { head: cost, tail: costRest }, Deref @ metamodelica::List::Cons { head: group, tail: restGroups }) => {
                    let mut clusterCost: metamodelica::Real;
                    let mut cluster: Arc<metamodelica::List<i32>>;
                    let mut clusterTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut clusterCostsTmp: Arc<metamodelica::List<metamodelica::Real>>;
                    let mut group = (*group).clone();
                    (cluster, clusterCost) = group.clone();
                    let true = (clusterCost.clone() + cost.clone() < targetSize) else { bail!("pattern mismatch") };
                    group = (metamodelica::cons(node.clone(), cluster.clone()), cost.clone() + clusterCost.clone());
                    (clusterTmp, clusterCostsTmp) = BLS_mergeToTargetSize(nodeRest.clone(), costRest.clone(), targetSize, metamodelica::cons(group.clone(), restGroups.clone()))?;
                    Ok((clusterTmp.clone(), clusterCostsTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: node, tail: nodeRest }, Deref @ metamodelica::List::Cons { head: cost, tail: costRest }, Deref @ metamodelica::List::Cons { head: group, tail: restGroups }) => {
                    let mut clusterCost: metamodelica::Real;
                    let mut cluster: Arc<metamodelica::List<i32>>;
                    let mut clusterTmp: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut clusterCostsTmp: Arc<metamodelica::List<metamodelica::Real>>;
                    let mut group = (*group).clone();
                    let mut restGroups = (*restGroups).clone();
                    (cluster, clusterCost) = group.clone();
                    let true = (clusterCost.clone() + cost.clone() >= targetSize) else { bail!("pattern mismatch") };
                    cluster = cluster.clone().reverse();
                    restGroups = metamodelica::cons((cluster.clone(), clusterCost.clone()), restGroups.clone());
                    group = (list![node.clone()], cost.clone());
                    (clusterTmp, clusterCostsTmp) = BLS_mergeToTargetSize(nodeRest.clone(), costRest.clone(), targetSize, metamodelica::cons(group.clone(), restGroups.clone()))?;
                    Ok((clusterTmp.clone(), clusterCostsTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("BLS_mergeToTargetSize failed!")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((clustersOut, clusterCostsOut))
}

fn realSum(mut reals: Arc<metamodelica::List<metamodelica::Real>>) -> Result<metamodelica::Real> {
    let mut sum: metamodelica::Real;
    sum = List::fold(reals, (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
    Ok(sum)
}

fn deleteIntListMembers(mut lst1: Arc<metamodelica::List<i32>>, mut lst2: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut lstOut: Arc<metamodelica::List<i32>>;
    (_, lstOut, _) = List::intersection1OnTrue(lst1, lst2, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    Ok(lstOut)
}

//-----------------
// Level Scheduling
//-----------------
pub(crate) fn createLevelSchedule(mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(Arc<HpcOmSimCode::Schedule>, HpcOmTaskGraph::TaskGraphMeta)> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut oMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut levelTasks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut levelTaskLists: Arc<metamodelica::List<HpcOmSimCode::TaskList>>;
    levelTasks = HpcOmTaskGraph::getLevelNodes(iGraph.clone())?;
    levelTaskLists = List::fold(levelTasks, (std::sync::Arc::new({ let __pe_b1 = iGraph.clone(); let __pe_b2 = iMeta.clone(); let __pe_b3 = iSccSimEqMapping.clone(); move |__pe_a0, __pe_a4| createLevelScheduleForLevel(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<HpcOmSimCode::TaskList>>) -> Result<Arc<metamodelica::List<HpcOmSimCode::TaskList>>> + 'static>), metamodelica::nil())?;
    levelTaskLists = levelTaskLists.reverse();
    oSchedule = Arc::new(HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: levelTaskLists, useFixedAssignments: false });
    oMeta = iMeta;
    Ok((oSchedule, oMeta))
}

fn createLevelScheduleForLevel(mut iTasksOfLevel: Arc<metamodelica::List<i32>>, mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iLevelTaskLists: Arc<metamodelica::List<HpcOmSimCode::TaskList>>) -> Result<Arc<metamodelica::List<HpcOmSimCode::TaskList>>> {
    let mut oLevelTaskLists: Arc<metamodelica::List<HpcOmSimCode::TaskList>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut taskList: HpcOmSimCode::TaskList;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut sortedTasksOfLevel: Arc<metamodelica::List<i32>>;
    let HpcOmTaskGraph::TASKGRAPHMETA { exeCosts: __pa0, inComps: __pa1, .. } = (iMeta) else { bail!("pattern mismatch") };
    exeCosts = __pa0.clone();
    inComps = __pa1.clone();
    sortedTasksOfLevel = iTasksOfLevel;
    taskList = makeCalcLevelParTaskLst(sortedTasksOfLevel, iSccSimEqMapping.clone(), inComps.clone())?;
    oLevelTaskLists = metamodelica::cons(taskList, iLevelTaskLists);
    Ok(oLevelTaskLists)
}

fn getLevelAssignment(mut level: Arc<metamodelica::List<i32>>, mut tplIn: (i32, metamodelica::Array<i32>)) -> Result<(i32, metamodelica::Array<i32>)> {
    let mut tplOut: (i32, metamodelica::Array<i32>);
    let mut idx: i32;
    let mut ass: metamodelica::Array<i32>;
    (idx, ass) = tplIn;
    List::map2_0(level, (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), idx, ass.clone())?;
    tplOut = (idx + 1, ass.clone());
    Ok(tplOut)
}

fn makeCalcLevelParTaskLst(mut iNodeIdc: Arc<metamodelica::List<i32>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<HpcOmSimCode::TaskList> {
    let mut oTasks: HpcOmSimCode::TaskList;
    let mut tmpList: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut nodeIdx: i32 = 0;
    for mut nodeIdx in &*iNodeIdc.reverse() {
        let mut nodeIdx = nodeIdx.clone();
        tmpList = metamodelica::cons(list![nodeIdx], tmpList.clone());
    }
    oTasks = makeCalcLevelParTaskLstForMergedNodes(tmpList, iSccSimEqMapping.clone(), iNodeSccMapping.clone())?;
    Ok(oTasks)
}

fn makeCalcLevelParTaskLstForMergedNodes(mut iNodeIdc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<HpcOmSimCode::TaskList> {
    let mut oTasks: HpcOmSimCode::TaskList;
    let mut tmpList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    tmpList = List::map(iNodeIdc, (std::sync::Arc::new({ let __pe_b1 = iNodeSccMapping.clone(); let __pe_b2 = iSccSimEqMapping.clone(); move |__pe_a0| makeCalcTaskLevel(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<Arc<HpcOmSimCode::Task>> + 'static>))?;
    oTasks = HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: tmpList };
    Ok(oTasks)
}

fn makeCalcTaskLevel(mut iNodeIdc: Arc<metamodelica::List<i32>>, mut iNodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oTask: Arc<HpcOmSimCode::Task>;
    let mut simEqs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sccs: Arc<metamodelica::List<i32>>;
    let mut sccIdx: i32 = 0;
    for mut nodeIdx in &*iNodeIdc.clone() {
        let mut nodeIdx = nodeIdx.clone();
        sccs = metamodelica::arrayGet(iNodeSccMapping.clone(), nodeIdx.clone())?;
        for mut sccIdx in &*sccs.clone() {
            let mut sccIdx = sccIdx.clone();
            simEqs = List::append_reverse(metamodelica::arrayGet(iSccSimEqMapping.clone(), sccIdx)?, simEqs.clone());
        }
    }
    oTask = Arc::new(HpcOmSimCode::Task::CALCTASK_LEVEL { eqIdc: simEqs.reverse(), nodeIdc: iNodeIdc, threadIdx: None });
    Ok(oTask)
}

pub(crate) fn makeCalcTask(mut simEqs: Arc<metamodelica::List<i32>>, mut node: i32, mut threadIdx: i32) -> Arc<HpcOmSimCode::Task> {
    let mut taskOut: Arc<HpcOmSimCode::Task>;
    taskOut = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: 0, index: node, calcTime: metamodelica::OrderedFloat(1.0_f64), timeFinished: metamodelica::OrderedFloat(1.0_f64), threadIdx: threadIdx, eqIdc: simEqs });
    taskOut
}

fn arrayIntIsNegative(mut node: i32, mut ass: metamodelica::Array<i32>) -> Result<bool> {
    let mut isAss: bool;
    isAss = intLt(metamodelica::arrayGet(ass.clone(), node)?, 0);
    Ok(isAss)
}

fn dumpLevelSchedule(mut iLevelInfo: HpcOmSimCode::TaskList, mut iLevel: i32) -> Result<(ArcStr, i32)> {
    let mut levelStr: ArcStr;
    let mut oLevel: i32;
    let mut s: ArcStr = arcstr::literal!("");
    let mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    (levelStr, oLevel) = (match iLevelInfo {
        HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: mut __esc_tasks } => {
            tasks = __esc_tasks.clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Parallel Level ")); __mm_s.push_str(&*intString(iLevel)); __mm_s.push_str(&*literal!(":\n")); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*dumpTaskList(tasks.clone())?); ArcStr::from(__mm_s) }).clone();
            (s, iLevel + 1)
        },
        HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: mut __esc_tasks, .. } => {
            tasks = __esc_tasks.clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Serial Level ")); __mm_s.push_str(&*intString(iLevel)); __mm_s.push_str(&*literal!(":\n")); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*dumpTaskList(tasks.clone())?); ArcStr::from(__mm_s) }).clone();
            (s, iLevel + 1)
        },
        _ => {
            metamodelica::print((literal!("printLevelSchedule failed!\n")).clone());
            bail!("fail")
        },
    });
    Ok((levelStr, oLevel))
}

//-----------------------
// Fixed level Scheduling
//-----------------------
pub(crate) fn createFixedLevelSchedule(mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(Arc<HpcOmSimCode::Schedule>, HpcOmTaskGraph::TaskGraphMeta)> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut oMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut levelTasks: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut adviceLists: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut levelTaskLists: Arc<metamodelica::List<HpcOmSimCode::TaskList>>;
    levelTasks = HpcOmTaskGraph::getLevelNodes(iGraph.clone())?;
    adviceLists = arrayCreate(metamodelica::arrayLength(iGraph.clone()), metamodelica::nil());
    levelTaskLists = List::fold(levelTasks, (std::sync::Arc::new({ let __pe_b1 = adviceLists.clone(); let __pe_b2 = iGraph.clone(); let __pe_b3 = iMeta.clone(); let __pe_b4 = iNumberOfThreads; let __pe_b5 = iSccSimEqMapping.clone(); move |__pe_a0, __pe_a6| createFixedLevelScheduleForLevel(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_a6) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<HpcOmSimCode::TaskList>>) -> Result<Arc<metamodelica::List<HpcOmSimCode::TaskList>>> + 'static>), metamodelica::nil())?;
    levelTaskLists = levelTaskLists.reverse();
    oSchedule = Arc::new(HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: levelTaskLists, useFixedAssignments: true });
    oMeta = iMeta;
    Ok((oSchedule, oMeta))
}

fn createFixedLevelScheduleForLevel(mut iTasksOfLevel: Arc<metamodelica::List<i32>>, mut iAdviceList: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iLevelTaskLists: Arc<metamodelica::List<HpcOmSimCode::TaskList>>) -> Result<Arc<metamodelica::List<HpcOmSimCode::TaskList>>> {
    let mut oLevelTaskLists: Arc<metamodelica::List<HpcOmSimCode::TaskList>>;
    let mut levelExecCosts: metamodelica::Real;
    let mut threadReadyList: metamodelica::Array<metamodelica::Real>;
    let mut threadTaskList: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let mut taskList: HpcOmSimCode::TaskList;
    let mut tasksOfLevel: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut sortedTasksOfLevel: Arc<metamodelica::List<i32>>;
    let HpcOmTaskGraph::TASKGRAPHMETA { exeCosts: __pa0, inComps: __pa1, .. } = (iMeta.clone()) else { bail!("pattern mismatch") };
    exeCosts = __pa0.clone();
    inComps = __pa1.clone();
    levelExecCosts = HpcOmTaskGraph::getCostsForContractedNodes(iTasksOfLevel.clone(), exeCosts.clone())?;
    threadReadyList = arrayCreate(iNumberOfThreads, metamodelica::OrderedFloat(0.0_f64));
    threadTaskList = arrayCreate(iNumberOfThreads, metamodelica::nil());
    sortedTasksOfLevel = List::sort(iTasksOfLevel, (std::sync::Arc::new({ let __pe_b2 = inComps.clone(); let __pe_b3 = exeCosts.clone(); let __pe_b4 = true; move |__pe_a0, __pe_a1| HpcOmTaskGraph::compareTasksByExecTime(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    List::fold(sortedTasksOfLevel, (std::sync::Arc::new({ let __pe_b1 = levelExecCosts; let __pe_b2 = iAdviceList.clone(); let __pe_b3 = threadReadyList.clone(); let __pe_b4 = iGraph.clone(); let __pe_b5 = iMeta; move |__pe_a0, __pe_a6| createFixedLevelScheduleForTask(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_a6) }) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), threadTaskList.clone())?;
    threadTaskList = Array::map(threadTaskList.clone(), Arc::new(fnptr!(metamodelica::listReverse, Arc<metamodelica::List<i32>>)))?;
    (_, tasksOfLevel) = Array::fold(threadTaskList.clone(), (std::sync::Arc::new({ let __pe_b1 = inComps.clone(); let __pe_b2 = iSccSimEqMapping.clone(); move |__pe_a0, __pe_a3| createFixedLevelScheduleForLevel0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (i32, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)) -> Result<(i32, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>), (1, metamodelica::nil()))?;
    taskList = HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: tasksOfLevel };
    oLevelTaskLists = metamodelica::cons(taskList, iLevelTaskLists);
    Ok(oLevelTaskLists)
}

fn createFixedLevelScheduleForLevel0(mut iTaskList: Arc<metamodelica::List<i32>>, mut iComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iIdxTaskList: (i32, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)) -> Result<(i32, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> {
    let mut oIdxTaskList: (i32, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>);
    let mut threadIdx: i32;
    let mut taskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut newTask: Arc<HpcOmSimCode::Task>;
    let mut components: Arc<metamodelica::List<i32>>;
    let mut simEqs: Arc<metamodelica::List<i32>>;
    let mut taskIdx: i32 = 0;
    (threadIdx, taskList) = iIdxTaskList;
    for mut taskIdx in &*iTaskList {
        let mut taskIdx = taskIdx.clone();
        components = metamodelica::arrayGet(iComps.clone(), taskIdx)?;
        simEqs = List::flatten(List::map(List::map1(components.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), iSccSimEqMapping.clone())?, Arc::new(fnptr!(metamodelica::listReverse, _)))?)?;
        if !(simEqs.clone().is_empty()) {
            simEqs = simEqs.clone();
            newTask = Arc::new(HpcOmSimCode::Task::CALCTASK_LEVEL { eqIdc: simEqs.clone(), nodeIdc: list![taskIdx], threadIdx: Some(threadIdx) });
            taskList = metamodelica::cons(newTask.clone(), taskList.clone());
        }
    }
    oIdxTaskList = (threadIdx + 1, taskList);
    Ok(oIdxTaskList)
}

fn createFixedLevelScheduleForTask(mut iTaskIdx: i32, mut iLevelExecCosts: metamodelica::Real, mut iAdviceList: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iThreadReadyList: metamodelica::Array<metamodelica::Real>, mut iGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta, mut iThreadTasks: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oThreadTasks: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut adviceElem: Arc<metamodelica::List<i32>>;
    let mut threadTasks: Arc<metamodelica::List<i32>>;
    let mut successorList: Arc<metamodelica::List<i32>>;
    let mut threadIdx: i32;
    let mut threadReadyTime: metamodelica::Real;
    let mut exeCost: metamodelica::Real;
    adviceElem = metamodelica::arrayGet(iAdviceList.clone(), iTaskIdx)?;
    adviceElem = flattenAdviceList(adviceElem, metamodelica::arrayLength(iThreadReadyList.clone()))?;
    threadIdx = getBestFittingThread(adviceElem, iLevelExecCosts, iThreadReadyList.clone())?;
    threadTasks = metamodelica::arrayGet(iThreadTasks.clone(), threadIdx)?;
    successorList = metamodelica::arrayGet(iGraph.clone(), iTaskIdx)?;
    List::fold1(successorList, (std::sync::Arc::new(createFixedLevelScheduleForTask0) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), threadIdx, iAdviceList.clone())?;
    threadReadyTime = metamodelica::arrayGet(iThreadReadyList.clone(), threadIdx)?;
    (_, exeCost) = HpcOmTaskGraph::getExeCost(iTaskIdx, iMeta)?;
    threadReadyTime = (threadReadyTime) + (exeCost);
    metamodelica::arrayUpdate(iThreadReadyList.clone(), threadIdx, threadReadyTime)?;
    threadTasks = metamodelica::cons(iTaskIdx, threadTasks);
    oThreadTasks = metamodelica::arrayUpdate(iThreadTasks.clone(), threadIdx, threadTasks)?;
    Ok(oThreadTasks)
}

fn createFixedLevelScheduleForTask0(mut iSuccessor: i32, mut iThreadAdvice: i32, mut iAdviceList: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oAdviceList: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut adviceElem: Arc<metamodelica::List<i32>>;
    adviceElem = metamodelica::arrayGet(iAdviceList.clone(), iSuccessor)?;
    adviceElem = metamodelica::cons(iThreadAdvice, adviceElem);
    oAdviceList = metamodelica::arrayUpdate(iAdviceList.clone(), iSuccessor, adviceElem)?;
    Ok(oAdviceList)
}

fn flattenAdviceList(mut iAdviceList: Arc<metamodelica::List<i32>>, mut iNumOfThreads: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oAdviceList: Arc<metamodelica::List<i32>>;
    let mut counterArray: metamodelica::Array<i32>;
    let mut tupleList: Arc<metamodelica::List<(i32, i32)>>;
    counterArray = arrayCreate(iNumOfThreads, 0);
    counterArray = List::fold(iAdviceList, (std::sync::Arc::new(flattenAdviceListElem) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), counterArray.clone())?;
    tupleList = arrayToTupleListZeroRemoved(counterArray.clone(), 1, metamodelica::nil());
    oAdviceList = List::map(List::sort(tupleList, (std::sync::Arc::new(fnptr!(intTpl22Gt, (i32, i32), (i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), (i32, i32)) -> Result<bool> + 'static>))?, std::sync::Arc::new(fnptr!(Util::tuple21, _)))?;
    Ok(oAdviceList)
}

fn flattenAdviceListElem(mut iAdviceElem: i32, mut iCounterArray: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut oCounterArray: metamodelica::Array<i32>;
    let mut counter: i32;
    counter = metamodelica::arrayGet(iCounterArray.clone(), iAdviceElem)?;
    counter = counter + 1;
    oCounterArray = metamodelica::arrayUpdate(iCounterArray.clone(), iAdviceElem, counter)?;
    Ok(oCounterArray)
}

fn arrayToTupleListZeroRemoved(mut iArray: metamodelica::Array<i32>, mut iCurrentIdx: i32, mut iTupleList: Arc<metamodelica::List<(i32, i32)>>) -> Arc<metamodelica::List<(i32, i32)>> {
    let mut oTupleList: Arc<metamodelica::List<(i32, i32)>>;
    let mut tmpTupleList: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut currentValue: i32 = 0;
    oTupleList = 'mc: {
        let __mc_input = iTupleList.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut currentValue: i32 = currentValue.clone();
                    let mut tmpTupleList: Arc<metamodelica::List<(i32, i32)>> = tmpTupleList.clone();
                    let true = (intLe(iCurrentIdx, metamodelica::arrayLength(iArray.clone()))) else { bail!("pattern mismatch") };
                    currentValue = metamodelica::arrayGet(iArray.clone(), iCurrentIdx)?;
                    let true = (intNe(currentValue, 0)) else { bail!("pattern mismatch") };
                    tmpTupleList = metamodelica::cons((iCurrentIdx, currentValue), iTupleList.clone());
                    tmpTupleList = arrayToTupleListZeroRemoved(iArray.clone(), iCurrentIdx + 1, tmpTupleList.clone());
                    Ok((tmpTupleList.clone(), currentValue.clone(), tmpTupleList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { currentValue = __wb0; tmpTupleList = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tmpTupleList: Arc<metamodelica::List<(i32, i32)>> = tmpTupleList.clone();
                    let true = (intLe(iCurrentIdx, metamodelica::arrayLength(iArray.clone()))) else { bail!("pattern mismatch") };
                    tmpTupleList = arrayToTupleListZeroRemoved(iArray.clone(), iCurrentIdx + 1, iTupleList.clone());
                    Ok((tmpTupleList.clone(), tmpTupleList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { tmpTupleList = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iTupleList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oTupleList
}

fn intTpl22Gt(mut iTpl1: (i32, i32), mut iTpl2: (i32, i32)) -> bool {
    let mut oRes: bool;
    let mut val1: i32;
    let mut val2: i32;
    (_, val1) = iTpl1;
    (_, val2) = iTpl2;
    oRes = intGt(val1, val2);
    oRes
}

fn getBestFittingThread(mut iAdviceList: Arc<metamodelica::List<i32>>, mut iLevelExecCosts: metamodelica::Real, mut iThreadReadyList: metamodelica::Array<metamodelica::Real>) -> Result<i32> {
    let mut oThreadIdx: i32;
    let mut averageThreadTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut readyTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut numOfThreads: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut head: i32 = 0;
    let mut tail: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oThreadIdx = 'mc: {
        let __mc_input = iAdviceList;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let mut threadIdx: i32 = threadIdx.clone();
                    threadIdx = getFirstReadyThread(iThreadReadyList.clone())?;
                    Ok((threadIdx, threadIdx.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { threadIdx = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: tail } => {
                    let mut averageThreadTime: metamodelica::Real = averageThreadTime.clone();
                    let mut numOfThreads: i32 = numOfThreads.clone();
                    let mut readyTime: metamodelica::Real = readyTime.clone();
                    readyTime = metamodelica::arrayGet(iThreadReadyList.clone(), head.clone())?;
                    numOfThreads = metamodelica::arrayLength(iThreadReadyList.clone());
                    averageThreadTime = realDiv(iLevelExecCosts, intReal(numOfThreads));
                    let true = (realLt(readyTime, averageThreadTime)) else { bail!("pattern mismatch") };
                    Ok((head.clone(), averageThreadTime.clone(), numOfThreads.clone(), readyTime.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { averageThreadTime = __wb0; numOfThreads = __wb1; readyTime = __wb2; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: tail } => {
                    Ok(getBestFittingThread(tail.clone(), iLevelExecCosts, iThreadReadyList.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oThreadIdx)
}

fn getFirstReadyThread(mut iThreadReadyList: metamodelica::Array<metamodelica::Real>) -> Result<i32> {
    let mut oFirstReadyThreadIdx: i32;
    (oFirstReadyThreadIdx, _, _) = Array::fold(iThreadReadyList.clone(), (std::sync::Arc::new(fnptr!(getFirstReadyThread0, metamodelica::Real, (i32, metamodelica::Real, i32))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, (i32, metamodelica::Real, i32)) -> Result<(i32, metamodelica::Real, i32)> + 'static>), (-1, metamodelica::OrderedFloat(-1.0_f64), 1))?;
    Ok(oFirstReadyThreadIdx)
}

fn getFirstReadyThread0(mut iThreadReadyTime: metamodelica::Real, mut iFirstReadyThread: (i32, metamodelica::Real, i32)) -> (i32, metamodelica::Real, i32) {
    let mut oFirstReadyThread: (i32, metamodelica::Real, i32);
    let mut firstThreadIdx: i32 = 0;
    let mut currentThreadIdx: i32 = 0;
    let mut readyTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut isLower: bool = false;
    oFirstReadyThread = (match iFirstReadyThread.clone() {
        ((-1), _, mut __esc_currentThreadIdx) => {
            currentThreadIdx = __esc_currentThreadIdx.clone();
            (currentThreadIdx, iThreadReadyTime, currentThreadIdx + 1)
        },
        (mut __esc_firstThreadIdx, mut __esc_readyTime, mut __esc_currentThreadIdx) => {
            firstThreadIdx = __esc_firstThreadIdx.clone();
            readyTime = __esc_readyTime.clone();
            currentThreadIdx = __esc_currentThreadIdx.clone();
            isLower = realLt(iThreadReadyTime, readyTime);
            firstThreadIdx = if (isLower) {currentThreadIdx} else {firstThreadIdx};
            readyTime = if (isLower) {iThreadReadyTime} else {readyTime};
            (firstThreadIdx, readyTime, currentThreadIdx + 1)
        },
        _ => {
            metamodelica::print((literal!("getFirstReadyThread0 failed\n")).clone());
            iFirstReadyThread
        },
    });
    oFirstReadyThread
}

//---------------------------
// Task Dependency Scheduling
//---------------------------
pub(crate) fn createTaskDepSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = Arc::new(<HpcOmSimCode::Schedule as ::std::default::Default>::default());
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut nodeMark: metamodelica::Array<i32> = Default::default();
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut nodeLevelMap: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut filteredNodeLevelMap: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    oSchedule = 'mc: {
        let __mc_input = iTaskGraphMeta;
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            let HpcOmTaskGraph::TaskGraphMeta { inComps: mut inComps, nodeMark: mut nodeMark, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut filteredNodeLevelMap: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>> = filteredNodeLevelMap.clone();
            let mut nodeLevelMap: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)>> = nodeLevelMap.clone();
            let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>> = taskGraphT.clone();
            let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = tmpSchedule.clone();
            taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
            (_, nodeLevelMap) = Array::fold(taskGraphT.clone(), (std::sync::Arc::new({ let __pe_b1 = nodeMark.clone(); let __pe_b2 = inComps.clone(); let __pe_b3 = iSccSimEqMapping.clone(); move |__pe_a0, __pe_a4| createNodeLevelMapping(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (i32, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)>>)) -> Result<(i32, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)>>)> + 'static>), (1, metamodelica::nil()))?;
            nodeLevelMap = List::sort(nodeLevelMap.clone(), (std::sync::Arc::new(sortNodeLevelMapping) as std::sync::Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>), (Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)) -> Result<bool> + 'static>))?;
            filteredNodeLevelMap = List::map(nodeLevelMap.clone(), (std::sync::Arc::new(fnptr!(filterNodeLevelMapping, (Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>))) as std::sync::Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)) -> Result<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)> + 'static>))?;
            filteredNodeLevelMap = filteredNodeLevelMap.clone().reverse();
            tmpSchedule = Arc::new(HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: filteredNodeLevelMap.clone() });
            Ok((tmpSchedule.clone(), filteredNodeLevelMap.clone(), nodeLevelMap.clone(), taskGraphT.clone(), tmpSchedule.clone()))
        })() { filteredNodeLevelMap = __wb0; nodeLevelMap = __wb1; taskGraphT = __wb2; tmpSchedule = __wb3; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("HpcOmScheduler.createTaskDepSchedule failed.\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oSchedule)
}

fn createNodeLevelMapping(mut iNodeDependenciesT: Arc<metamodelica::List<i32>>, mut nodeMarks: metamodelica::Array<i32>, mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNodeInfo: (i32, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)>>)) -> Result<(i32, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)>>)> {
    let mut oNodeInfo: (i32, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)>>);
    let mut task: Arc<HpcOmSimCode::Task>;
    let mut nodeIdx: i32;
    let mut nodeMark: i32;
    let mut components: Arc<metamodelica::List<i32>>;
    let mut simEqIdc: Arc<metamodelica::List<i32>>;
    let mut nodeLevelMap: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)>>;
    (nodeIdx, nodeLevelMap) = iNodeInfo;
    components = metamodelica::arrayGet(inComps.clone(), nodeIdx)?;
    nodeMark = metamodelica::arrayGet(nodeMarks.clone(), List::last(components.clone())?)?;
    simEqIdc = List::map(List::map1(components, (std::sync::Arc::new(getSimEqSysIdxForComp) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iSccSimEqMapping.clone())?, (std::sync::Arc::new(List::last) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
    task = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: -1, index: nodeIdx, calcTime: metamodelica::OrderedFloat(-1.0_f64), timeFinished: metamodelica::OrderedFloat(-1.0_f64), threadIdx: -1, eqIdc: simEqIdc });
    nodeLevelMap = metamodelica::cons((task, nodeMark, iNodeDependenciesT), nodeLevelMap);
    oNodeInfo = (nodeIdx + 1, nodeLevelMap);
    Ok(oNodeInfo)
}

fn sortNodeLevelMapping(mut iElem1: (Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>), mut iElem2: (Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)) -> Result<bool> {
    let mut oResult: bool;
    let mut elemLvl1: i32;
    let mut elemLvl2: i32;
    let mut task1Idx: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iElem1) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { index: __pa0, .. }, __pa1, _) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    task1Idx = __pa0.clone();
    elemLvl1 = __pa1.clone();
    (_, elemLvl2, _) = iElem2;
    oResult = intGe(elemLvl1, elemLvl2);
    Ok(oResult)
}

fn filterNodeLevelMapping(mut iElem: (Arc<HpcOmSimCode::Task>, i32, Arc<metamodelica::List<i32>>)) -> (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>) {
    let mut oElem: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>);
    let mut task: Arc<HpcOmSimCode::Task>;
    let mut childTasks: Arc<metamodelica::List<i32>>;
    (task, _, childTasks) = iElem;
    oElem = (task, childTasks);
    oElem
}

//-----------------
// Metis Scheduling
//-----------------
pub(crate) fn createMetisSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut extInfo: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut xadj: metamodelica::Array<i32> = Default::default();
    let mut adjncy: metamodelica::Array<i32> = Default::default();
    let mut vwgt: metamodelica::Array<i32> = Default::default();
    let mut adjwgt: metamodelica::Array<i32> = Default::default();
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = Arc::new(<HpcOmSimCode::Schedule as ::std::default::Default>::default());
    let mut extInfoArr: metamodelica::Array<i32> = Default::default();
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = Default::default();
    let mut rootNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = Default::default();
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut priorityArr: metamodelica::Array<i32> = Default::default();
    let mut levelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
    let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut priorityTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut otherTasks: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut order: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut removeLocks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oSchedule = 'mc: {
        let __mc_input = iTaskGraphMeta.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10, __wb11, __wb12, __wb13, __wb14, __wb15, __wb16, __wb17)) = (|| -> Result<_> {
            let HpcOmTaskGraph::TaskGraphMeta { commCosts: mut commCosts, inComps: mut inComps, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut adjncy: metamodelica::Array<i32> = adjncy.clone();
            let mut adjwgt: metamodelica::Array<i32> = adjwgt.clone();
            let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = allCalcTasks.clone();
            let mut extInfo: Arc<metamodelica::List<i32>> = extInfo.clone();
            let mut extInfoArr: metamodelica::Array<i32> = extInfoArr.clone();
            let mut levelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = levelNodes.clone();
            let mut order: Arc<metamodelica::List<i32>> = order.clone();
            let mut otherTasks: Arc<metamodelica::List<i32>> = otherTasks.clone();
            let mut priorityArr: metamodelica::Array<i32> = priorityArr.clone();
            let mut priorityTasks: Arc<metamodelica::List<i32>> = priorityTasks.clone();
            let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>> = procAss.clone();
            let mut removeLocks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = removeLocks.clone();
            let mut rootNodes: Arc<metamodelica::List<i32>> = rootNodes.clone();
            let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>> = taskGraphT.clone();
            let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = threadTasks.clone();
            let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = tmpSchedule.clone();
            let mut vwgt: metamodelica::Array<i32> = vwgt.clone();
            let mut xadj: metamodelica::Array<i32> = xadj.clone();
            (xadj, adjncy, vwgt, adjwgt) = prepareMetis(iTaskGraph.clone(), iTaskGraphMeta.clone())?;
            if intGt(iNumberOfThreads, 1) {
                extInfo = HpcOmSchedulerExt::scheduleMetis(xadj.clone(), adjncy.clone(), vwgt.clone(), adjwgt.clone(), iNumberOfThreads)?;
                extInfoArr = metamodelica::arrayFromVec(extInfo.clone().into_iter().cloned().collect());
            } else {
                extInfoArr = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), 1);
                extInfo = Arc::new(extInfoArr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            }
            let true = (intEq(metamodelica::arrayLength(iTaskGraph.clone()), metamodelica::arrayLength(extInfoArr.clone()))) else { bail!("pattern mismatch") };
            taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
            rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
            priorityArr = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), 0);
            createMetisSchedule1(List::intRange(metamodelica::arrayLength(iTaskGraph.clone())), extInfoArr.clone(), iTaskGraph.clone(), taskGraphT.clone(), priorityArr.clone())?;
            levelNodes = HpcOmTaskGraph::getLevelNodes(iTaskGraph.clone())?;
            allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta.clone(), (std::sync::Arc::new(convertNodeToTask) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>));
            (priorityTasks, otherTasks) = createMetisSchedule2(levelNodes.clone(), priorityArr.clone(), metamodelica::nil(), metamodelica::nil())?;
            order = listAppend(priorityTasks.clone(), otherTasks.clone());
            procAss = arrayCreate(iNumberOfThreads, metamodelica::nil());
            List::map2_0(List::intRange(metamodelica::arrayLength(iTaskGraph.clone())), (std::sync::Arc::new(getProcAss) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> + 'static>), extInfoArr.clone(), procAss.clone())?;
            threadTasks = arrayCreate(iNumberOfThreads, metamodelica::nil());
            removeLocks = metamodelica::nil();
            tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            (tmpSchedule, removeLocks) = createScheduleFromAssignments(extInfoArr.clone(), procAss.clone(), Some(order.clone()), iTaskGraph.clone(), taskGraphT.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone(), removeLocks.clone(), order.clone(), iSimVarMapping.clone(), tmpSchedule.clone())?;
            if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("number of removed superfluous locks: ")); __mm_s.push_str(&*intString(intDiv((removeLocks.clone().len() as i32), 2))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            tmpSchedule = traverseAndUpdateThreadsInSchedule(tmpSchedule.clone(), (std::sync::Arc::new(removeLocksFromThread) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), removeLocks.clone())?;
            tmpSchedule = updateLockIdcsInThreadschedule(tmpSchedule.clone(), (std::sync::Arc::new(removeLocksFromLockList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), removeLocks.clone())?;
            Ok((setScheduleLockIds(tmpSchedule.clone())?, adjncy.clone(), adjwgt.clone(), allCalcTasks.clone(), extInfo.clone(), extInfoArr.clone(), levelNodes.clone(), order.clone(), otherTasks.clone(), priorityArr.clone(), priorityTasks.clone(), procAss.clone(), removeLocks.clone(), rootNodes.clone(), taskGraphT.clone(), threadTasks.clone(), tmpSchedule.clone(), vwgt.clone(), xadj.clone()))
        })() { adjncy = __wb0; adjwgt = __wb1; allCalcTasks = __wb2; extInfo = __wb3; extInfoArr = __wb4; levelNodes = __wb5; order = __wb6; otherTasks = __wb7; priorityArr = __wb8; priorityTasks = __wb9; procAss = __wb10; removeLocks = __wb11; rootNodes = __wb12; taskGraphT = __wb13; threadTasks = __wb14; tmpSchedule = __wb15; vwgt = __wb16; xadj = __wb17; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("HpcOmScheduler.createMetisSchedule not every node has a scheduler-info.\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oSchedule)
}

fn getProcAss(mut idx: i32, mut taskAss: metamodelica::Array<i32>, mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    let mut thread: i32;
    thread = metamodelica::arrayGet(taskAss.clone(), idx)?;
    Array::appendToElement(thread, list![idx], procAss.clone())?;
    Ok(())
}

fn createMetisSchedule2(mut levelNodes: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut priorityArr: metamodelica::Array<i32>, mut prioLstIn: Arc<metamodelica::List<i32>>, mut otherLstIn: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(levelNodes) {
        Deref @ metamodelica::List::Nil => {
            return Ok((prioLstIn, otherLstIn))
        },
        Deref @ metamodelica::List::Cons { head: level, tail: rest } => {
            let mut prioLst: Arc<metamodelica::List<i32>>;
            let mut otherLst: Arc<metamodelica::List<i32>>;
            (prioLst, otherLst) = List::split1OnTrue(level.clone(), (std::sync::Arc::new(isPrioNode) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), priorityArr.clone())?;
            prioLst = listAppend(prioLstIn, prioLst.clone());
            otherLst = listAppend(otherLstIn, otherLst.clone());
            { (levelNodes, priorityArr, prioLstIn, otherLstIn) = (rest.clone(), priorityArr.clone(), prioLst.clone(), otherLst.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn isPrioNode(mut idx: i32, mut prioArr: metamodelica::Array<i32>) -> Result<bool> {
    let mut isPrio: bool;
    isPrio = intEq(1, metamodelica::arrayGet(prioArr.clone(), idx)?);
    Ok(isPrio)
}

fn createMetisSchedule1(mut taskIdcs: Arc<metamodelica::List<i32>>, mut threadIds: metamodelica::Array<i32>, mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut priorityArr: metamodelica::Array<i32>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = taskIdcs;
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
                Deref @ metamodelica::List::Cons { head: taskIdx, tail: rest } => {
                    let mut preds: Arc<metamodelica::List<i32>>;
                    let mut rest = (*rest).clone();
                    let true = (intEq(1, metamodelica::arrayGet(priorityArr.clone(), taskIdx.clone())?)) else { bail!("pattern mismatch") };
                    preds = metamodelica::arrayGet(taskGraphT.clone(), taskIdx.clone())?;
                    preds = List::filter1OnTrue(preds.clone(), (std::sync::Arc::new(arrayIntIsNotOne) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), priorityArr.clone())?;
                    List::map2_0(preds.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), 1, priorityArr.clone())?;
                    rest = listAppend(preds.clone(), rest.clone());
                    createMetisSchedule1(rest.clone(), threadIds.clone(), taskGraph.clone(), taskGraphT.clone(), priorityArr.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: taskIdx, tail: rest } => {
                    let mut threadId: i32;
                    let mut preds: Arc<metamodelica::List<i32>>;
                    let mut predThreads: Arc<metamodelica::List<i32>>;
                    let mut rest = (*rest).clone();
                    threadId = metamodelica::arrayGet(threadIds.clone(), taskIdx.clone())?;
                    preds = metamodelica::arrayGet(taskGraphT.clone(), taskIdx.clone())?;
                    predThreads = List::map1(preds.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), threadIds.clone())?;
                    (predThreads, preds) = List::filter1OnTrueSync(predThreads.clone(), (std::sync::Arc::new(fnptr!(intNe, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), threadId.clone(), preds.clone())?;
                    if !(predThreads.clone().is_empty()) {
                        List::map2_0(preds.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), 1, priorityArr.clone())?;
                        rest = listAppend(preds.clone(), rest.clone());
                    } else {
                        metamodelica::arrayUpdate(priorityArr.clone(), taskIdx.clone(), 0)?;
                    }
                    createMetisSchedule1(rest.clone(), threadIds.clone(), taskGraph.clone(), taskGraphT.clone(), priorityArr.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn arrayIntIsNotOne(mut idx: i32, mut arr: metamodelica::Array<i32>) -> Result<bool> {
    let mut isOne: bool;
    isOne = intNe(1, metamodelica::arrayGet(arr.clone(), idx)?);
    Ok(isOne)
}

pub(crate) fn createHMetisSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut extInfo: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut xadj: metamodelica::Array<i32> = Default::default();
    let mut adjncy: metamodelica::Array<i32> = Default::default();
    let mut vwgt: metamodelica::Array<i32> = Default::default();
    let mut adjwgt: metamodelica::Array<i32> = Default::default();
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = Arc::new(<HpcOmSimCode::Schedule as ::std::default::Default>::default());
    let mut extInfoArr: metamodelica::Array<i32> = Default::default();
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = Default::default();
    let mut rootNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = Default::default();
    let mut nodeList_refCount: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut nodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>> = Default::default();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    oSchedule = 'mc: {
        let __mc_input = iTaskGraphMeta.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10, __wb11, __wb12)) = (|| -> Result<_> {
            let HpcOmTaskGraph::TaskGraphMeta { commCosts: mut commCosts, inComps: mut inComps, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut adjncy: metamodelica::Array<i32> = adjncy.clone();
            let mut adjwgt: metamodelica::Array<i32> = adjwgt.clone();
            let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = allCalcTasks.clone();
            let mut extInfo: Arc<metamodelica::List<i32>> = extInfo.clone();
            let mut extInfoArr: metamodelica::Array<i32> = extInfoArr.clone();
            let mut nodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = nodeList.clone();
            let mut nodeList_refCount: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = nodeList_refCount.clone();
            let mut rootNodes: Arc<metamodelica::List<i32>> = rootNodes.clone();
            let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>> = taskGraphT.clone();
            let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = threadTasks.clone();
            let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = tmpSchedule.clone();
            let mut vwgt: metamodelica::Array<i32> = vwgt.clone();
            let mut xadj: metamodelica::Array<i32> = xadj.clone();
            metamodelica::print((literal!("Funktionsaufruf!")).clone());
            (xadj, adjncy, vwgt, adjwgt) = preparehMetis(iTaskGraph.clone(), iTaskGraphMeta.clone())?;
            extInfo = HpcOmSchedulerExt::schedulehMetis(xadj.clone(), adjncy.clone(), vwgt.clone(), adjwgt.clone(), iNumberOfThreads)?;
            extInfoArr = metamodelica::arrayFromVec(extInfo.clone().into_iter().cloned().collect());
            metamodelica::print((literal!("Hier geht MetaModelica los!\n")).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("External scheduling info: ")); __mm_s.push_str(&*stringDelimitList(List::map(extInfo.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            let true = (intEq(metamodelica::arrayLength(iTaskGraph.clone()), metamodelica::arrayLength(extInfoArr.clone()))) else { bail!("pattern mismatch") };
            taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
            rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
            allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta.clone(), (std::sync::Arc::new(convertNodeToTask) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>));
            nodeList_refCount = List::map1(rootNodes.clone(), (std::sync::Arc::new(getTaskByIndex) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>) -> Result<(Arc<HpcOmSimCode::Task>, i32)> + 'static>), allCalcTasks.clone())?;
            nodeList = List::map(nodeList_refCount.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)))?;
            nodeList = List::sort(nodeList.clone(), (std::sync::Arc::new(compareTasksByWeighting) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
            threadTasks = arrayCreate(iNumberOfThreads, metamodelica::nil());
            tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            tmpSchedule = createExtSchedule1(nodeList.clone(), extInfoArr.clone(), iTaskGraph.clone(), taskGraphT.clone(), commCosts.clone(), inComps.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), (std::sync::Arc::new(getLocksByPredecessorList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>), tmpSchedule.clone())?;
            tmpSchedule = addSuccessorLocksToSchedule(iTaskGraph.clone(), (std::sync::Arc::new(addReleaseLocksToSchedule) as std::sync::Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), commCosts.clone(), inComps.clone(), iSimVarMapping.clone(), tmpSchedule.clone())?;
            Ok((setScheduleLockIds(tmpSchedule.clone())?, adjncy.clone(), adjwgt.clone(), allCalcTasks.clone(), extInfo.clone(), extInfoArr.clone(), nodeList.clone(), nodeList_refCount.clone(), rootNodes.clone(), taskGraphT.clone(), threadTasks.clone(), tmpSchedule.clone(), vwgt.clone(), xadj.clone()))
        })() { adjncy = __wb0; adjwgt = __wb1; allCalcTasks = __wb2; extInfo = __wb3; extInfoArr = __wb4; nodeList = __wb5; nodeList_refCount = __wb6; rootNodes = __wb7; taskGraphT = __wb8; threadTasks = __wb9; tmpSchedule = __wb10; vwgt = __wb11; xadj = __wb12; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("HpcOmScheduler.createHMetisSchedule not every node has a scheduler-info.\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oSchedule)
}

fn sumEdge(mut edges: Arc<metamodelica::List<i32>>, mut innumedge: i32) -> i32 {
    let mut outnumedge: i32;
    outnumedge = innumedge + (edges.len() as i32);
    outnumedge
}

fn getSingleRelations(mut edge: i32, mut n: i32, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut irelations: Arc<metamodelica::List<(i32, i32, i32)>>) -> Result<Arc<metamodelica::List<(i32, i32, i32)>>> {
    let mut orelations: Arc<metamodelica::List<(i32, i32, i32)>>;
    let mut costs: metamodelica::Real;
    let mut costsInt: i32;
    costs = HpcOmTaskGraph::getCommCostTimeBetweenNodes(n, edge, iTaskGraphMeta)?;
    costsInt = ((costs).0.floor() as i32);
    orelations = List::appendElt((edge, n, costsInt), irelations);
    orelations = List::appendElt((n, edge, costsInt), orelations);
    Ok(orelations)
}

fn getRelations(mut edges: Arc<metamodelica::List<i32>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut irelations: (Arc<metamodelica::List<(i32, i32, i32)>>, i32)) -> Result<(Arc<metamodelica::List<(i32, i32, i32)>>, i32)> {
    let mut orelations: (Arc<metamodelica::List<(i32, i32, i32)>>, i32);
    let mut n: i32;
    let mut relations: Arc<metamodelica::List<(i32, i32, i32)>>;
    let mut orel: Arc<metamodelica::List<(i32, i32, i32)>>;
    (relations, n) = irelations;
    orel = List::fold2(edges, (std::sync::Arc::new(getSingleRelations) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, HpcOmTaskGraph::TaskGraphMeta, Arc<metamodelica::List<(i32, i32, i32)>>) -> Result<Arc<metamodelica::List<(i32, i32, i32)>>> + 'static>), n, iTaskGraphMeta, relations)?;
    orelations = (orel, n + 1);
    Ok(orelations)
}

fn sortEdgeHelp(mut edge: (i32, i32, i32), mut actnode: i32, mut adjncy: metamodelica::Array<i32>, mut adjwgt: metamodelica::Array<i32>, mut imarker: i32) -> i32 {
    let mut omarker: i32;
    omarker = 'mc: {
        let __mc_input = edge;
        if let Ok(__v) = (|| -> Result<_> {
            let (mut fromnode, mut tonode, mut cost) = __mc_input.clone() else { bail!("nomatch") };
            let true = (intEq(fromnode.clone(), actnode)) else { bail!("pattern mismatch") };
            metamodelica::arrayUpdate(adjwgt.clone(), imarker, cost.clone())?;
            metamodelica::arrayUpdate(adjncy.clone(), imarker, tonode.clone() - 1)?;
            Ok(imarker + 1)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(imarker)
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    omarker
}

fn sortEdge(mut actnode: i32, mut xadj: metamodelica::Array<i32>, mut adjncy: metamodelica::Array<i32>, mut adjwgt: metamodelica::Array<i32>, mut help: Arc<metamodelica::List<(i32, i32, i32)>>, mut imarker: i32) -> Result<i32> {
    let mut omarker: i32;
    omarker = List::fold3(help, (std::sync::Arc::new(fnptr!(sortEdgeHelp, (i32, i32, i32), i32, metamodelica::Array<i32>, metamodelica::Array<i32>, i32)) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32, i32), i32, metamodelica::Array<i32>, metamodelica::Array<i32>, i32) -> Result<i32> + 'static>), actnode, adjncy.clone(), adjwgt.clone(), imarker)?;
    metamodelica::arrayUpdate(xadj.clone(), actnode + 1, omarker - 1)?;
    Ok(omarker)
}

fn setVwgt(mut node: i32, mut vwgt: metamodelica::Array<i32>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<()> {
    let mut value: (i32, metamodelica::Real);
    let mut rv: metamodelica::Real;
    value = HpcOmTaskGraph::getExeCost(node, iTaskGraphMeta)?;
    (_, rv) = value;
    metamodelica::arrayUpdate(vwgt.clone(), node, ((rv).0.floor() as i32))?;
    Ok(())
}

fn prepareMetis(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut xadj: metamodelica::Array<i32>;
    let mut adjncy: metamodelica::Array<i32>;
    let mut vwgt: metamodelica::Array<i32>;
    let mut adjwgt: metamodelica::Array<i32>;
    let mut n: i32;
    let mut m: i32;
    let mut adjundirected: (Arc<metamodelica::List<(i32, i32, i32)>>, i32);
    let mut help: Arc<metamodelica::List<(i32, i32, i32)>>;
    let mut allTheNodes: Arc<metamodelica::List<i32>>;
    help = metamodelica::nil();
    n = metamodelica::arrayLength(iTaskGraph.clone());
    xadj = arrayCreate(n + 1, 0);
    m = Array::fold(iTaskGraph.clone(), (std::sync::Arc::new(fnptr!(sumEdge, Arc<metamodelica::List<i32>>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<i32> + 'static>), 0)?;
    adjwgt = arrayCreate(2 * m, 0);
    adjundirected = Array::fold(iTaskGraph.clone(), (std::sync::Arc::new({ let __pe_b1 = iTaskGraphMeta.clone(); move |__pe_a0, __pe_a2| getRelations(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (Arc<metamodelica::List<(i32, i32, i32)>>, i32)) -> Result<(Arc<metamodelica::List<(i32, i32, i32)>>, i32)> + 'static>), (metamodelica::nil(), 1))?;
    (help, _) = adjundirected;
    allTheNodes = List::intRange(n);
    adjncy = arrayCreate(2 * m, 0);
    xadj = metamodelica::arrayUpdate(xadj.clone(), 1, 0)?;
    List::fold4(allTheNodes.clone(), (std::sync::Arc::new(sortEdge) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::Array<i32>, Arc<metamodelica::List<(i32, i32, i32)>>, i32) -> Result<i32> + 'static>), xadj.clone(), adjncy.clone(), adjwgt.clone(), help, 1)?;
    vwgt = arrayCreate(n, 0);
    List::map2_0(allTheNodes, (std::sync::Arc::new(setVwgt) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, HpcOmTaskGraph::TaskGraphMeta) -> Result<()> + 'static>), vwgt.clone(), iTaskGraphMeta)?;
    Ok((xadj, adjncy, vwgt, adjwgt))
}

fn listNodes(mut node: i32, mut l_eint: Arc<metamodelica::List<i32>>) -> Arc<metamodelica::List<i32>> {
    let mut l_eint_out: Arc<metamodelica::List<i32>>;
    let mut actnode: i32;
    actnode = node - 1;
    l_eint_out = listAppend(l_eint, list![actnode]);
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("l_eint length:")); __mm_s.push_str(&*intString((l_eint_out.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    l_eint_out
}

fn getHedge(mut childnodes: Arc<metamodelica::List<i32>>, mut actnode: (i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut actnode_out: (i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    actnode_out = (::match_deref::match_deref! { match &((childnodes.clone(), actnode)) {
        (Deref @ metamodelica::List::Nil, (node, position, l_eptr, l_eint, l_hewgts)) => {
            let mut help: (i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
            help = (node.clone() + 1, position.clone(), l_eptr.clone(), l_eint.clone(), l_hewgts.clone());
            help.clone()
        },
        (_, (node, position, l_eptr, l_eint, l_hewgts)) => {
            let mut n: i32;
            let mut help: (i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
            let mut l_eptr = (*l_eptr).clone();
            let mut l_eint = (*l_eint).clone();
            n = node.clone() - 1;
            l_eint = List::appendElt(n.clone(), l_eint.clone());
            l_eint = List::fold(childnodes.clone(), (std::sync::Arc::new(fnptr!(listNodes, i32, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), l_eint.clone())?;
            n = position.clone() + (childnodes.len() as i32) + 1;
            l_eptr = List::appendElt(n.clone(), l_eptr.clone());
            help = (node.clone() + 1, n.clone(), l_eptr.clone(), l_eint.clone(), l_hewgts.clone());
            help.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(actnode_out)
}

fn preparehMetis(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut vwgts: metamodelica::Array<i32>;
    let mut eptr: metamodelica::Array<i32>;
    let mut eint: metamodelica::Array<i32>;
    let mut hewgts: metamodelica::Array<i32>;
    let mut n: i32;
    let mut l_eptr: Arc<metamodelica::List<i32>>;
    let mut l_eint: Arc<metamodelica::List<i32>>;
    let mut l_hewgts: Arc<metamodelica::List<i32>>;
    let mut allTheNodes: Arc<metamodelica::List<i32>>;
    let mut result: (i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>);
    n = metamodelica::arrayLength(iTaskGraph.clone());
    result = Array::fold(iTaskGraph.clone(), (std::sync::Arc::new(getHedge) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)) -> Result<(i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> + 'static>), (1, 0, list![0], metamodelica::nil(), metamodelica::nil()))?;
    (_, _, l_eptr, l_eint, l_hewgts) = result;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Diagnostic length: ")); __mm_s.push_str(&*intString((l_eptr.clone().len() as i32))); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*intString((l_eint.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    allTheNodes = List::intRange(n);
    vwgts = arrayCreate(n, 0);
    List::map2_0(allTheNodes, (std::sync::Arc::new(setVwgt) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>, HpcOmTaskGraph::TaskGraphMeta) -> Result<()> + 'static>), vwgts.clone(), iTaskGraphMeta)?;
    eptr = metamodelica::arrayFromVec(l_eptr.into_iter().cloned().collect());
    eint = metamodelica::arrayFromVec(l_eint.into_iter().cloned().collect());
    hewgts = metamodelica::arrayFromVec(l_hewgts.into_iter().cloned().collect());
    Ok((vwgts, eptr, eint, hewgts))
}

//--------------------
// External Scheduling //TODO: Rename to Yed Scheduling
//--------------------
pub(crate) fn createExtSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iGraphMLFile: ArcStr) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut extInfo: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut extInfoArr: metamodelica::Array<i32> = Default::default();
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = Arc::new(<HpcOmSimCode::Schedule as ::std::default::Default>::default());
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = Default::default();
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>> = Default::default();
    let mut rootNodes: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = Default::default();
    let mut nodeList_refCount: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut nodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    oSchedule = 'mc: {
        let __mc_input = iTaskGraphMeta.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8)) = (|| -> Result<_> {
            let HpcOmTaskGraph::TaskGraphMeta { commCosts: mut commCosts, inComps: mut inComps, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = allCalcTasks.clone();
            let mut extInfo: Arc<metamodelica::List<i32>> = extInfo.clone();
            let mut extInfoArr: metamodelica::Array<i32> = extInfoArr.clone();
            let mut nodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = nodeList.clone();
            let mut nodeList_refCount: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = nodeList_refCount.clone();
            let mut rootNodes: Arc<metamodelica::List<i32>> = rootNodes.clone();
            let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>> = taskGraphT.clone();
            let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = threadTasks.clone();
            let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = tmpSchedule.clone();
            extInfo = HpcOmSchedulerExt::readScheduleFromGraphMl((iGraphMLFile.clone()).clone())?;
            extInfoArr = metamodelica::arrayFromVec(extInfo.clone().into_iter().cloned().collect());
            let true = (intEq(metamodelica::arrayLength(iTaskGraph.clone()), metamodelica::arrayLength(extInfoArr.clone()))) else { bail!("pattern mismatch") };
            taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
            rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
            allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta.clone(), (std::sync::Arc::new(convertNodeToTask) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>));
            nodeList_refCount = List::map1(rootNodes.clone(), (std::sync::Arc::new(getTaskByIndex) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>) -> Result<(Arc<HpcOmSimCode::Task>, i32)> + 'static>), allCalcTasks.clone())?;
            nodeList = List::map(nodeList_refCount.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)))?;
            nodeList = List::sort(nodeList.clone(), (std::sync::Arc::new(compareTasksByWeighting) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
            threadTasks = arrayCreate(iNumberOfThreads, metamodelica::nil());
            tmpSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            tmpSchedule = createExtSchedule1(nodeList.clone(), extInfoArr.clone(), iTaskGraph.clone(), taskGraphT.clone(), commCosts.clone(), inComps.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), (std::sync::Arc::new(getLocksByPredecessorList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>), tmpSchedule.clone())?;
            tmpSchedule = addSuccessorLocksToSchedule(iTaskGraph.clone(), (std::sync::Arc::new(addReleaseLocksToSchedule) as std::sync::Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, i32), Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), commCosts.clone(), inComps.clone(), iSimVarMapping.clone(), tmpSchedule.clone())?;
            Ok((tmpSchedule.clone(), allCalcTasks.clone(), extInfo.clone(), extInfoArr.clone(), nodeList.clone(), nodeList_refCount.clone(), rootNodes.clone(), taskGraphT.clone(), threadTasks.clone(), tmpSchedule.clone()))
        })() { allCalcTasks = __wb0; extInfo = __wb1; extInfoArr = __wb2; nodeList = __wb3; nodeList_refCount = __wb4; rootNodes = __wb5; taskGraphT = __wb6; threadTasks = __wb7; tmpSchedule = __wb8; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("HpcOmScheduler.createExtSchedule not every node has a scheduler-info.\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oSchedule)
}

fn createExtSchedule1(mut iNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iThreadAssignments: metamodelica::Array<i32>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iLockWithPredecessorHandler: Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>, mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    pub type FuncType = std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>>, i32, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>;

    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut head: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut newTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut newTaskRefCount: i32 = 0;
    let mut rest: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut predecessors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = metamodelica::nil();
    let mut successorIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut newOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut firstEq: i32;
    let mut allThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = Default::default();
    let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut lockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut threadId: i32 = 0;
    let mut threadFinishTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut tmpNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut weighting: i32 = 0;
    let mut index: i32 = 0;
    let mut calcTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut eqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut simEqIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = Default::default();
    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = Arc::new(<HpcOmSimCode::Schedule as ::std::default::Default>::default());
    oSchedule = 'mc: {
        let __mc_input = (iNodeList.clone(), iSchedule.clone());
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10, __wb11, __wb12)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ Deref @ HpcOmSimCode::Task::CALCTASK { weighting, index, calcTime, eqIdc: eqIdc @ Deref @ metamodelica::List::Cons { head: firstEq, tail: _ }, .. }, tail: rest }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks, outgoingDepTasks, allCalcTasks, .. }) => {
                    let mut allThreadTasks = (*allThreadTasks).clone();
                    let mut outgoingDepTasks = (*outgoingDepTasks).clone();
                    let mut allCalcTasks = (*allCalcTasks).clone();
                    let mut lockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = lockTasks.clone();
                    let mut newOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = newOutgoingDepTasks.clone();
                    let mut newTask: Arc<HpcOmSimCode::Task> = newTask.clone();
                    let mut newTaskRefCount: i32 = newTaskRefCount.clone();
                    let mut predecessors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = predecessors.clone();
                    let mut simEqIdc: Arc<metamodelica::List<i32>> = simEqIdc.clone();
                    let mut successorIdc: Arc<metamodelica::List<i32>> = successorIdc.clone();
                    let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = successors.clone();
                    let mut threadFinishTime: metamodelica::Real = threadFinishTime.clone();
                    let mut threadId: i32 = threadId.clone();
                    let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = threadTasks.clone();
                    let mut tmpNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = tmpNodeList.clone();
                    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = tmpSchedule.clone();
                    (predecessors, _) = getSuccessorsByTask(head.clone(), iTaskGraphT.clone(), allCalcTasks.clone())?;
                    (successors, successorIdc) = getSuccessorsByTask(head.clone(), iTaskGraph.clone(), allCalcTasks.clone())?;
                    let false = (predecessors.clone().is_empty()) else { bail!("pattern mismatch") };
                    threadId = metamodelica::arrayGet(iThreadAssignments.clone(), index.clone())?;
                    threadFinishTime = metamodelica::OrderedFloat(-1.0_f64);
                    threadTasks = metamodelica::arrayGet(allThreadTasks.clone(), threadId)?;
                    (lockTasks, newOutgoingDepTasks) = iLockWithPredecessorHandler(head.clone(), predecessors.clone(), threadId, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
                    outgoingDepTasks = listAppend(outgoingDepTasks.clone(), newOutgoingDepTasks.clone());
                    threadTasks = listAppend(lockTasks.clone(), threadTasks.clone());
                    simEqIdc = List::map(List::map1(eqIdc.clone(), (std::sync::Arc::new(getSimEqSysIdxForComp) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iSccSimEqMapping.clone())?, (std::sync::Arc::new(List::last) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
                    newTask = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: threadFinishTime, threadIdx: threadId, eqIdc: simEqIdc.clone() });
                    threadTasks = metamodelica::cons(newTask.clone(), threadTasks.clone());
                    allThreadTasks = metamodelica::arrayUpdate(allThreadTasks.clone(), threadId, threadTasks.clone())?;
                    (allCalcTasks, tmpNodeList) = updateRefCounterBySuccessorIdc(allCalcTasks.clone(), successorIdc.clone(), metamodelica::nil());
                    tmpNodeList = listAppend(tmpNodeList.clone(), rest.clone());
                    tmpNodeList = List::sort(tmpNodeList.clone(), (std::sync::Arc::new(compareTasksByWeighting) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
                    (_, newTaskRefCount) = metamodelica::arrayGet(allCalcTasks.clone(), index.clone())?;
                    metamodelica::arrayUpdate(allCalcTasks.clone(), index.clone(), (newTask.clone(), newTaskRefCount))?;
                    tmpSchedule = createExtSchedule1(tmpNodeList.clone(), iThreadAssignments.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler.clone(), Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() }))?;
                    Ok((tmpSchedule.clone(), lockTasks.clone(), newOutgoingDepTasks.clone(), newTask.clone(), newTaskRefCount.clone(), predecessors.clone(), simEqIdc.clone(), successorIdc.clone(), successors.clone(), threadFinishTime.clone(), threadId.clone(), threadTasks.clone(), tmpNodeList.clone(), tmpSchedule.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { lockTasks = __wb0; newOutgoingDepTasks = __wb1; newTask = __wb2; newTaskRefCount = __wb3; predecessors = __wb4; simEqIdc = __wb5; successorIdc = __wb6; successors = __wb7; threadFinishTime = __wb8; threadId = __wb9; threadTasks = __wb10; tmpNodeList = __wb11; tmpSchedule = __wb12; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: head @ Deref @ HpcOmSimCode::Task::CALCTASK { weighting, index, calcTime, eqIdc: eqIdc @ Deref @ metamodelica::List::Cons { head: firstEq, tail: _ }, .. }, tail: rest }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks, outgoingDepTasks, allCalcTasks, .. }) => {
                    let mut allThreadTasks = (*allThreadTasks).clone();
                    let mut allCalcTasks = (*allCalcTasks).clone();
                    let mut newTask: Arc<HpcOmSimCode::Task> = newTask.clone();
                    let mut newTaskRefCount: i32 = newTaskRefCount.clone();
                    let mut simEqIdc: Arc<metamodelica::List<i32>> = simEqIdc.clone();
                    let mut successorIdc: Arc<metamodelica::List<i32>> = successorIdc.clone();
                    let mut successors: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, i32)>> = successors.clone();
                    let mut threadFinishTime: metamodelica::Real = threadFinishTime.clone();
                    let mut threadId: i32 = threadId.clone();
                    let mut threadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = threadTasks.clone();
                    let mut tmpNodeList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = tmpNodeList.clone();
                    let mut tmpSchedule: Arc<HpcOmSimCode::Schedule> = tmpSchedule.clone();
                    (successors, successorIdc) = getSuccessorsByTask(head.clone(), iTaskGraph.clone(), allCalcTasks.clone())?;
                    threadId = metamodelica::arrayGet(iThreadAssignments.clone(), index.clone())?;
                    threadFinishTime = metamodelica::OrderedFloat(-1.0_f64);
                    threadTasks = metamodelica::arrayGet(allThreadTasks.clone(), threadId)?;
                    simEqIdc = List::flatten(List::map1(eqIdc.clone(), (std::sync::Arc::new(getSimEqSysIdxForComp) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iSccSimEqMapping.clone())?)?;
                    newTask = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: threadFinishTime, threadIdx: threadId, eqIdc: simEqIdc.clone() });
                    allThreadTasks = metamodelica::arrayUpdate(allThreadTasks.clone(), threadId, metamodelica::cons(newTask.clone(), threadTasks.clone()))?;
                    (allCalcTasks, tmpNodeList) = updateRefCounterBySuccessorIdc(allCalcTasks.clone(), successorIdc.clone(), metamodelica::nil());
                    tmpNodeList = listAppend(tmpNodeList.clone(), rest.clone());
                    tmpNodeList = List::sort(tmpNodeList.clone(), (std::sync::Arc::new(compareTasksByWeighting) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
                    (_, newTaskRefCount) = metamodelica::arrayGet(allCalcTasks.clone(), index.clone())?;
                    metamodelica::arrayUpdate(allCalcTasks.clone(), index.clone(), (newTask.clone(), newTaskRefCount))?;
                    tmpSchedule = createExtSchedule1(tmpNodeList.clone(), iThreadAssignments.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSccSimEqMapping.clone(), iSimVarMapping.clone(), iLockWithPredecessorHandler.clone(), Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: allThreadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() }))?;
                    Ok((tmpSchedule.clone(), newTask.clone(), newTaskRefCount.clone(), simEqIdc.clone(), successorIdc.clone(), successors.clone(), threadFinishTime.clone(), threadId.clone(), threadTasks.clone(), tmpNodeList.clone(), tmpSchedule.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { newTask = __wb0; newTaskRefCount = __wb1; simEqIdc = __wb2; successorIdc = __wb3; successors = __wb4; threadFinishTime = __wb5; threadId = __wb6; threadTasks = __wb7; tmpNodeList = __wb8; tmpSchedule = __wb9; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(iSchedule.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("HpcOmScheduler.createExtSchedule1 failed. Tasks in List:\n")).clone());
                    printTaskList(iNodeList.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oSchedule)
}

//---------------------------------
// Task Duplication-based Scheduler
//---------------------------------
pub(crate) fn TDS_schedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut numProc: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut iSimCode: SimCode::SimCode) -> Result<(Arc<HpcOmSimCode::Schedule>, SimCode::SimCode, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut oSimCode: SimCode::SimCode;
    let mut oTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut oSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut size: i32;
    let mut queue: Arc<metamodelica::List<i32>>;
    let mut levels: Arc<metamodelica::List<metamodelica::Real>>;
    let mut ectArray: metamodelica::Array<metamodelica::Real>;
    let mut tdsLevelArray: metamodelica::Array<metamodelica::Real>;
    let mut lastArray: metamodelica::Array<metamodelica::Real>;
    let mut lactArray: metamodelica::Array<metamodelica::Real>;
    let mut fpredArray: metamodelica::Array<i32>;
    let mut initClusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let HpcOmTaskGraph::TASKGRAPHMETA { commCosts: __pa0, inComps: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    commCosts = __pa0.clone();
    inComps = __pa1.clone();
    size = metamodelica::arrayLength(iTaskGraph.clone());
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size)?;
    (_, _, ectArray) = computeGraphValuesBottomUp(iTaskGraph.clone(), iTaskGraphMeta.clone())?;
    (_, lastArray, lactArray, tdsLevelArray) = computeGraphValuesTopDown(iTaskGraph.clone(), iTaskGraphMeta.clone())?;
    fpredArray = computeFavouritePred(iTaskGraph.clone(), iTaskGraphMeta.clone(), ectArray.clone())?;
    (levels, queue) = quicksortWithOrder(Arc::new(tdsLevelArray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()))?;
    initClusters = TDS_InitialCluster(iTaskGraph.clone(), taskGraphT.clone(), iTaskGraphMeta.clone(), lastArray.clone(), lactArray.clone(), fpredArray.clone(), queue)?;
    (oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping) = TDS_schedule1(initClusters, iTaskGraph.clone(), taskGraphT.clone(), iTaskGraphMeta, tdsLevelArray.clone(), numProc, iSccSimEqMapping.clone(), iSimCode, commCosts.clone(), inComps.clone(), iSimVarMapping.clone())?;
    Ok((oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping))
}

fn insertLocksInSchedule(mut iSchedule: Arc<HpcOmSimCode::Schedule>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskAss: metamodelica::Array<i32>, mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut threads: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iSchedule) {
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __pa0, allCalcTasks: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    threadTasks = __pa0.clone();
    allCalcTasks = __pa1.clone();
    threads = Arc::new(threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    (threads, outgoingDepTasks) = List::fold(threads, (std::sync::Arc::new({ let __pe_b1 = (iTaskGraph.clone(), iTaskGraphT.clone()); let __pe_b2 = (taskAss.clone(), procAss.clone()); let __pe_b3 = allCalcTasks.clone(); let __pe_b4 = iCommCosts.clone(); let __pe_b5 = iCompTaskMapping.clone(); let __pe_b6 = iSimVarMapping.clone(); move |__pe_a0, __pe_a7| insertLocksInSchedule1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_a7) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, (Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> + 'static>), (metamodelica::nil(), metamodelica::nil()))?;
    threads = List::filterOnFalse(threads, std::sync::Arc::new(fnptr!(listEmpty, _)))?;
    threads = List::map(threads, Arc::new(fnptr!(metamodelica::listReverse, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)))?;
    threads = threads.reverse();
    threadTasks = metamodelica::arrayFromVec(threads.into_iter().cloned().collect());
    outgoingDepTasks = List::unique(outgoingDepTasks);
    oSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks, scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
    Ok(oSchedule)
}

fn insertLocksInSchedule1(mut threadsIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut iTaskGraphTransposed: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>), mut taskProcAss: (metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>), mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut foldIn: (Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((threadsIn.clone(), iTaskGraphTransposed.clone(), taskProcAss.clone(), foldIn)) {
        (Deref @ metamodelica::List::Nil, _, _, (threads, outgoingDepTasks)) => {
            let mut threads = (*threads).clone();
            threads = metamodelica::cons(metamodelica::nil(), threads.clone());
            return Ok((threads.clone(), outgoingDepTasks.clone()))
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ HpcOmSimCode::Task::CALCTASK { index: idx, threadIdx: thr, .. }, tail: rest }, (iTaskGraph, iTaskGraphT), (taskAss, _), (threads, outgoingDepTasks)) => {
            let mut preds: Arc<metamodelica::List<i32>>;
            let mut succs: Arc<metamodelica::List<i32>>;
            let mut predThr: Arc<metamodelica::List<i32>>;
            let mut succThr: Arc<metamodelica::List<i32>>;
            let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut relLocks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut assLocks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut task: Arc<HpcOmSimCode::Task>;
            let mut threads = (*threads).clone();
            let mut outgoingDepTasks = (*outgoingDepTasks).clone();
            task = listHead(threadsIn)?;
            preds = metamodelica::arrayGet(iTaskGraphT.clone(), idx.clone())?;
            succs = metamodelica::arrayGet(iTaskGraph.clone(), idx.clone())?;
            predThr = List::map1(preds.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), taskAss.clone())?;
            succThr = List::map1(succs.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), taskAss.clone())?;
            (_, preds) = List::filter1OnTrueSync(predThr.clone(), (std::sync::Arc::new(fnptr!(intNe, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), thr.clone(), preds.clone())?;
            (_, succs) = List::filter1OnTrueSync(succThr.clone(), (std::sync::Arc::new(fnptr!(intNe, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), thr.clone(), succs.clone())?;
            assLocks = List::map6(preds.clone(), (std::sync::Arc::new(createDepTaskByTaskIdc) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, bool, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Task>> + 'static>), idx.clone(), iAllCalcTasks.clone(), false, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
            relLocks = List::map6(succs.clone(), (std::sync::Arc::new(createDepTaskByTaskIdc) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, bool, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Task>> + 'static>), idx.clone(), iAllCalcTasks.clone(), true, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
            tasks = listAppend(listAppend(relLocks.clone(), list![task.clone()]), assLocks.clone());
            thread = if (!(threads.clone().is_empty())) {listHead(threads.clone())?} else {metamodelica::nil()};
            thread = listAppend(tasks.clone(), thread.clone());
            threads = if (!(threads.clone().is_empty())) {List::replaceAt(thread.clone(), 1, threads.clone())?} else {list![thread.clone()]};
            outgoingDepTasks = listAppend(relLocks.clone(), outgoingDepTasks.clone());
            outgoingDepTasks = listAppend(assLocks.clone(), outgoingDepTasks.clone());
            { (threadsIn, iTaskGraphTransposed, taskProcAss, iAllCalcTasks, iCommCosts, iCompTaskMapping, iSimVarMapping, foldIn) = (rest.clone(), iTaskGraphTransposed, taskProcAss, iAllCalcTasks.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone(), (threads.clone(), outgoingDepTasks.clone())); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn TDS_schedule1(mut clustersIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut TDSLevel: metamodelica::Array<metamodelica::Real>, mut numProc: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimCode: SimCode::SimCode, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<(Arc<HpcOmSimCode::Schedule>, SimCode::SimCode, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut oSimCode: SimCode::SimCode;
    let mut oTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut oTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta;
    let mut oSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping) = 'mc: {
        let __mc_input = iSimVarMapping.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut sccSimEqMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut schedule: Arc<HpcOmSimCode::Schedule>;
            let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut meta: HpcOmTaskGraph::TaskGraphMeta;
            let mut simCode: SimCode::SimCode;
            let true = ((clustersIn.clone().len() as i32) < numProc) else { bail!("pattern mismatch") };
            metamodelica::print((literal!("There are less initial clusters than processors. we need duplication, but since this is a rare case, it is not done. Less processors are used.\n")).clone());
            clusters = List::map(clustersIn.clone(), Arc::new(fnptr!(metamodelica::listReverse, Arc<metamodelica::List<i32>>)))?;
            FlagsUtil::setConfigInt(Flags::NUM_PROC.clone(), (clustersIn.clone().len() as i32))?;
            (schedule, simCode, taskGraph, meta, sccSimEqMap) = TDS_schedule1(clusters.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), TDSLevel.clone(), (clustersIn.clone().len() as i32), iSccSimEqMapping.clone(), iSimCode.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
            Ok((schedule.clone(), simCode.clone(), taskGraph.clone(), meta.clone(), sccSimEqMap.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut sccSimEqMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut schedule: Arc<HpcOmSimCode::Schedule>;
            let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut meta: HpcOmTaskGraph::TaskGraphMeta;
            let mut simCode: SimCode::SimCode;
            let true = ((clustersIn.clone().len() as i32) > numProc) else { bail!("pattern mismatch") };
            clusters = TDS_CompactClusters(clustersIn.clone(), iTaskGraph.clone(), iTaskGraphMeta.clone(), TDSLevel.clone(), numProc)?;
            (schedule, simCode, taskGraph, meta, sccSimEqMap) = TDS_schedule1(clusters.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), TDSLevel.clone(), numProc, iSccSimEqMapping.clone(), iSimCode.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
            Ok((schedule.clone(), simCode.clone(), taskGraph.clone(), meta.clone(), sccSimEqMap.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut sizeTasks: i32;
            let mut numDupl: i32;
            let mut threadIdx: i32;
            let mut compIdx: i32;
            let mut simVarIdx: i32;
            let mut simEqSysIdx: i32;
            let mut taskIdx: i32;
            let mut lsIdx: i32;
            let mut nlsIdx: i32;
            let mut mIdx: i32;
            let mut taskAss: metamodelica::Array<i32>;
            let mut taskDuplAss: metamodelica::Array<i32>;
            let mut nodeMark: metamodelica::Array<i32>;
            let mut newIdxAss: metamodelica::Array<i32>;
            let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut sccSimEqMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut comps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
            let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
            let mut varCompMapping: metamodelica::Array<(i32, i32, i32)>;
            let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)>;
            let mut idcs: (i32, i32, i32, i32, i32, i32, i32, i32);
            let mut compNames: metamodelica::Array<ArcStr>;
            let mut compDescs: metamodelica::Array<ArcStr>;
            let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut duplSccSimEqMap: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut duplComps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut schedule: Arc<HpcOmSimCode::Schedule>;
            let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut meta: HpcOmTaskGraph::TaskGraphMeta;
            let mut simCode: SimCode::SimCode;
            let mut simVars: SimCodeVar::SimVars;
            let mut algVars: Arc<metamodelica::List<SimCodeVar::SimVar>>;
            let mut threadTask: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
            let mut odes: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>>;
            let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
            let mut compParamMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut compInformations: metamodelica::Array<HpcOmTaskGraph::ComponentInfo>;
            let true = ((clustersIn.clone().len() as i32) == numProc) else { bail!("pattern mismatch") };
            clusters = List::map1(clustersIn.clone(), (std::sync::Arc::new(TDS_SortCompactClusters) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, metamodelica::Array<metamodelica::Real>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), TDSLevel.clone())?;
            let SimCode::SIMCODE { modelInfo: SimCode::MODELINFO { vars: __pa0, .. }, odeEquations: __pa1, .. } = (iSimCode.clone()) else { bail!("pattern mismatch") };
            simVars = __pa0.clone();
            odes = __pa1.clone();
            let SimCodeVar::SIMVARS { algVars: __pa2, .. } = (simVars.clone()) else { bail!("pattern mismatch") };
            algVars = __pa2.clone();
            let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa3, varCompMapping: __pa4, eqCompMapping: __pa5, compParamMapping: __pa6, compNames: __pa7, compDescs: __pa8, exeCosts: __pa9, commCosts: __pa10, nodeMark: __pa11, compInformations: __pa12 } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
            inComps = __pa3.clone();
            varCompMapping = __pa4.clone();
            eqCompMapping = __pa5.clone();
            compParamMapping = __pa6.clone();
            compNames = __pa7.clone();
            compDescs = __pa8.clone();
            exeCosts = __pa9.clone();
            commCosts = __pa10.clone();
            nodeMark = __pa11.clone();
            compInformations = __pa12.clone();
            sizeTasks = List::fold(List::map(clusters.clone(), std::sync::Arc::new(fnptr!(listLength, _)))?, (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 0)?;
            taskAss = arrayCreate(sizeTasks.clone(), -1);
            procAss = arrayCreate((clusters.clone().len() as i32), metamodelica::nil());
            taskGraph = arrayCreate(sizeTasks.clone(), metamodelica::nil());
            taskDuplAss = arrayCreate(sizeTasks.clone(), -1);
            threadTask = arrayCreate(numProc, metamodelica::nil());
            allCalcTasks = arrayCreate(sizeTasks.clone(), (openmodelica_simcode_types::HpcOmSimCode::Task::interned_TASKEMPTY(), 0));
            schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTask.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            duplSccSimEqMap = metamodelica::nil();
            duplComps = metamodelica::nil();
            threadIdx = 1;
            compIdx = metamodelica::arrayLength(iSccSimEqMapping.clone()) + 1;
            taskIdx = metamodelica::arrayLength(iTaskGraph.clone()) + 1;
            simVarIdx = ({
        let mut __acc: Option<i32> = None;
        for mut v in (algVars.clone()).into_iter().cloned() {
            let __x = v.index.clone();
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.unwrap_or((-i32::MAX))
    }) + 1;
            simEqSysIdx = SimCodeUtil::getMaxSimEqSystemIndex(iSimCode.clone())? + 1;
            lsIdx = List::fold(List::map(List::flatten(odes.clone())?, (std::sync::Arc::new(fnptr!(SimCodeUtil::getLSindex, Arc<SimCode::SimEqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>) -> Result<i32> + 'static>))?, (std::sync::Arc::new(fnptr!(intMax, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 0)? + 1;
            nlsIdx = List::fold(List::map(List::flatten(odes.clone())?, (std::sync::Arc::new(fnptr!(SimCodeUtil::getNLSindex, Arc<SimCode::SimEqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>) -> Result<i32> + 'static>))?, (std::sync::Arc::new(fnptr!(intMax, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 0)? + 1;
            mIdx = List::fold(List::map(List::flatten(odes.clone())?, (std::sync::Arc::new(fnptr!(SimCodeUtil::getMixedindex, Arc<SimCode::SimEqSystem>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>) -> Result<i32> + 'static>))?, (std::sync::Arc::new(fnptr!(intMax, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 0)? + 1;
            (taskAss, procAss, taskGraph, taskDuplAss, idcs, simCode, schedule, duplSccSimEqMap, duplComps) = TDS_duplicateTasks(clusters.clone(), taskAss.clone(), procAss.clone(), (threadIdx.clone(), taskIdx.clone(), compIdx.clone(), simVarIdx.clone(), simEqSysIdx.clone(), lsIdx.clone(), nlsIdx.clone(), mIdx.clone()), iTaskGraph.clone(), iTaskGraphT.clone(), taskGraph.clone(), taskDuplAss.clone(), iTaskGraphMeta.clone(), iSimCode.clone(), schedule.clone(), iSccSimEqMapping.clone(), duplSccSimEqMap.clone(), duplComps.clone())?;
            simCode = TDS_updateModelInfo(simCode.clone(), idcs.clone());
            numDupl = List::fold(List::map(duplComps.clone(), std::sync::Arc::new(fnptr!(listLength, _)))?, (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 0)?;
            procAss = Array::map(procAss.clone(), Arc::new(fnptr!(metamodelica::listReverse, Arc<metamodelica::List<i32>>)))?;
            sccSimEqMap = metamodelica::arrayAppend(iSccSimEqMapping.clone(), metamodelica::arrayFromVec(duplSccSimEqMap.clone().reverse().into_iter().cloned().collect()));
            comps = metamodelica::arrayAppend(inComps.clone(), metamodelica::arrayFromVec(duplComps.clone().reverse().into_iter().cloned().collect()));
            varCompMapping = metamodelica::arrayAppend(varCompMapping.clone(), arrayCreate(numDupl.clone(), (0, 0, 0)));
            eqCompMapping = metamodelica::arrayAppend(eqCompMapping.clone(), arrayCreate(numDupl.clone(), (0, 0, 0)));
            compParamMapping = metamodelica::arrayAppend(compParamMapping.clone(), arrayCreate(numDupl.clone(), metamodelica::nil()));
            compNames = metamodelica::arrayAppend(compNames.clone(), arrayCreate(numDupl.clone(), (literal!("duplicated")).clone()));
            compDescs = metamodelica::arrayAppend(compDescs.clone(), arrayCreate(numDupl.clone(), (literal!("duplicated")).clone()));
            exeCosts = metamodelica::arrayAppend(exeCosts.clone(), arrayCreate(numDupl.clone(), (1, metamodelica::OrderedFloat(1.0_f64))));
            nodeMark = metamodelica::arrayAppend(nodeMark.clone(), arrayCreate(numDupl.clone(), -1));
            meta = HpcOmTaskGraph::TaskGraphMeta { inComps: comps.clone(), varCompMapping: varCompMapping.clone(), eqCompMapping: eqCompMapping.clone(), compParamMapping: compParamMapping.clone(), compNames: compNames.clone(), compDescs: compDescs.clone(), exeCosts: exeCosts.clone(), commCosts: commCosts.clone(), nodeMark: nodeMark.clone(), compInformations: compInformations.clone() };
            newIdxAss = arrayCreate(SimCodeUtil::getMaxSimEqSystemIndex(simCode.clone())?, -1);
            (simCode, newIdxAss) = TDS_assignNewSimEqSysIdxs(simCode.clone(), newIdxAss.clone())?;
            taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(taskGraph.clone(), metamodelica::arrayLength(taskGraph.clone()))?;
            schedule = insertLocksInSchedule(schedule.clone(), taskGraph.clone(), taskGraphT.clone(), taskAss.clone(), procAss.clone(), iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
            schedule = TDS_replaceSimEqSysIdxsInSchedule(schedule.clone(), newIdxAss.clone())?;
            Ok((schedule.clone(), simCode.clone(), taskGraph.clone(), meta.clone(), sccSimEqMap.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("TDS_schedule1 failed!\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oSchedule, oSimCode, oTaskGraph, oTaskGraphMeta, oSccSimEqMapping))
}

fn TDS_replaceSimEqSysIdxsInSchedule(mut scheduleIn: Arc<HpcOmSimCode::Schedule>, mut assIn: metamodelica::Array<i32>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut scheduleOut: Arc<HpcOmSimCode::Schedule>;
    scheduleOut = (::match_deref::match_deref! { match &(scheduleIn) {
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks, outgoingDepTasks, scheduledTasks, allCalcTasks } => {
            let mut threadTasks = (*threadTasks).clone();
            let mut scheduledTasks = (*scheduledTasks).clone();
            scheduledTasks = List::map1(scheduledTasks.clone(), (std::sync::Arc::new(fnptr!(TDS_replaceSimEqSysIdxsInTask, Arc<HpcOmSimCode::Task>, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, metamodelica::Array<i32>) -> Result<Arc<HpcOmSimCode::Task>> + 'static>), assIn.clone())?;
            threadTasks = Array::map1(threadTasks.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIdxsInTaskLst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), assIn.clone())?;
            Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: scheduledTasks.clone(), allCalcTasks: allCalcTasks.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(scheduleOut)
}

fn TDS_replaceSimEqSysIdxsInTask(mut taskIn: Arc<HpcOmSimCode::Task>, mut assIn: metamodelica::Array<i32>) -> Arc<HpcOmSimCode::Task> {
    let mut taskOut: Arc<HpcOmSimCode::Task>;
    taskOut = 'mc: {
        let __mc_input = taskIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Task::CALCTASK { weighting, index, calcTime, timeFinished, threadIdx, eqIdc } => {
                    let mut eqIdc = (*eqIdc).clone();
                    eqIdc = List::map1(eqIdc.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), assIn.clone())?;
                    Ok(Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: timeFinished.clone(), threadIdx: threadIdx.clone(), eqIdc: eqIdc.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(taskIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    taskOut
}

fn TDS_replaceSimEqSysIdxsInTaskLst(mut taskLstIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut assIn: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut taskLstOut: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    taskLstOut = List::map1(taskLstIn, (std::sync::Arc::new(fnptr!(TDS_replaceSimEqSysIdxsInTask, Arc<HpcOmSimCode::Task>, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, metamodelica::Array<i32>) -> Result<Arc<HpcOmSimCode::Task>> + 'static>), assIn.clone())?;
    Ok(taskLstOut)
}

fn TDS_assignNewSimEqSysIdxs(mut simCodeIn: SimCode::SimCode, mut idxAssIn: metamodelica::Array<i32>) -> Result<(SimCode::SimCode, metamodelica::Array<i32>)> {
    let mut simCodeOut: SimCode::SimCode = simCodeIn.clone();
    let mut idxAssOut: metamodelica::Array<i32>;
    let mut modelInfo: SimCode::ModelInfo;
    let mut varInfo: SimCode::VarInfo;
    let mut jacObts: Arc<metamodelica::List<Option<Arc<SimCode::JacobianMatrix>>>>;
    let mut eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
    let mut idx: i32;
    let mut ass: metamodelica::Array<i32>;
    modelInfo = simCodeOut.modelInfo.clone();
    varInfo = modelInfo.varInfo.clone();
    let (__pa0, (__pa1, __pa2)) = List::mapFold(simCodeOut.initialEquations.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndexWithUpdate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>))> + 'static>), (1, idxAssIn.clone()))?;
    eqs = __pa0.clone();
    idx = __pa1.clone();
    ass = __pa2.clone();
    simCodeOut.initialEquations = eqs;
    let (__pa3, (__pa4, __pa5)) = List::mapFold(simCodeOut.allEquations.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndexWithUpdate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>))> + 'static>), (idx, ass.clone()))?;
    eqs = __pa3.clone();
    idx = __pa4.clone();
    ass = __pa5.clone();
    simCodeOut.allEquations = eqs;
    let (__pa6, (__pa7, __pa8)) = List::mapFold(simCodeOut.startValueEquations.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndexWithUpdate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>))> + 'static>), (idx, ass.clone()))?;
    eqs = __pa6.clone();
    idx = __pa7.clone();
    ass = __pa8.clone();
    simCodeOut.startValueEquations = eqs;
    let (__pa9, (__pa10, __pa11)) = List::mapFold(simCodeOut.nominalValueEquations.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndexWithUpdate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>))> + 'static>), (idx, ass.clone()))?;
    eqs = __pa9.clone();
    idx = __pa10.clone();
    ass = __pa11.clone();
    simCodeOut.nominalValueEquations = eqs;
    let (__pa12, (__pa13, __pa14)) = List::mapFold(simCodeOut.minValueEquations.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndexWithUpdate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>))> + 'static>), (idx, ass.clone()))?;
    eqs = __pa12.clone();
    idx = __pa13.clone();
    ass = __pa14.clone();
    simCodeOut.minValueEquations = eqs;
    let (__pa15, (__pa16, __pa17)) = List::mapFold(simCodeOut.maxValueEquations.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndexWithUpdate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>))> + 'static>), (idx, ass.clone()))?;
    eqs = __pa15.clone();
    idx = __pa16.clone();
    ass = __pa17.clone();
    simCodeOut.maxValueEquations = eqs;
    let (__pa18, (__pa19, __pa20)) = List::mapFold(simCodeOut.parameterEquations.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndexWithUpdate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>))> + 'static>), (idx, ass.clone()))?;
    eqs = __pa18.clone();
    idx = __pa19.clone();
    ass = __pa20.clone();
    simCodeOut.parameterEquations = eqs;
    let (__pa21, (__pa22, __pa23)) = List::mapFold(simCodeOut.algorithmAndEquationAsserts.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndexWithUpdate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>))> + 'static>), (idx, ass.clone()))?;
    eqs = __pa21.clone();
    idx = __pa22.clone();
    ass = __pa23.clone();
    simCodeOut.algorithmAndEquationAsserts = eqs;
    simCodeOut.odeEquations = List::map1List(simCodeOut.odeEquations.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndex) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, metamodelica::Array<i32>) -> Result<Arc<SimCode::SimEqSystem>> + 'static>), ass.clone())?;
    simCodeOut.algebraicEquations = List::map1List(simCodeOut.algebraicEquations.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndex) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, metamodelica::Array<i32>) -> Result<Arc<SimCode::SimEqSystem>> + 'static>), ass.clone())?;
    simCodeOut.equationsForZeroCrossings = List::map1(simCodeOut.equationsForZeroCrossings.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndex) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, metamodelica::Array<i32>) -> Result<Arc<SimCode::SimEqSystem>> + 'static>), ass.clone())?;
    jacObts = List::map(simCodeOut.jacobianMatrices.clone(), std::sync::Arc::new(fnptr!(Util::makeOption, _)))?;
    jacObts = List::map1(jacObts, (std::sync::Arc::new(fnptr!(TDS_replaceSimEqSysIdxInJacobianMatrix, Option<Arc<SimCode::JacobianMatrix>>, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<SimCode::JacobianMatrix>>, metamodelica::Array<i32>) -> Result<Option<Arc<SimCode::JacobianMatrix>>> + 'static>), ass.clone())?;
    simCodeOut.jacobianMatrices = List::map(jacObts, (std::sync::Arc::new(Util::getOption) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
    varInfo.numEquations = idx;
    modelInfo.varInfo = varInfo;
    simCodeOut.modelInfo = modelInfo;
    idxAssOut = ass.clone();
    Ok((simCodeOut, idxAssOut))
}

fn TDS_replaceSimEqSysIndex(mut simEqIn: Arc<SimCode::SimEqSystem>, mut assIn: metamodelica::Array<i32>) -> Result<Arc<SimCode::SimEqSystem>> {
    let mut simEqOut: Arc<SimCode::SimEqSystem>;
    simEqOut = 'mc: {
        let __mc_input = simEqIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: nlSystem @ Deref @ SimCode::NonlinearSystem { eqs, jacobianMatrix, .. }, .. } => {
                    let mut newIdx: i32;
                    let mut oldIdx: i32;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut nlSystem = (*nlSystem).clone();
                    let mut eqs = (*eqs).clone();
                    let mut jacobianMatrix = (*jacobianMatrix).clone();
                    eqs = List::map1(eqs.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndex) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, metamodelica::Array<i32>) -> Result<Arc<SimCode::SimEqSystem>> + 'static>), assIn.clone())?;
                    oldIdx = SimCodeUtil::simEqSystemIndex(simEqIn.clone())?;
                    newIdx = metamodelica::arrayGet(assIn.clone(), oldIdx.clone())?;
                    jacobianMatrix = TDS_replaceSimEqSysIdxInJacobianMatrix(jacobianMatrix.clone(), assIn.clone());
                    assign_field!(
                        nlSystem.jacobianMatrix = jacobianMatrix.clone(),
                        nlSystem.index = newIdx.clone(),
                        nlSystem.eqs = eqs.clone()
                    );
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_NONLINEAR; nlSystem = nlSystem.clone());
                    Ok(simEqSys.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: lSystem @ Deref @ SimCode::LinearSystem { residual: eqs, jacobianMatrix, .. }, .. } => {
                    let mut newIdx: i32;
                    let mut oldIdx: i32;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut lSystem = (*lSystem).clone();
                    let mut eqs = (*eqs).clone();
                    let mut jacobianMatrix = (*jacobianMatrix).clone();
                    eqs = List::map1(eqs.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndex) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, metamodelica::Array<i32>) -> Result<Arc<SimCode::SimEqSystem>> + 'static>), assIn.clone())?;
                    oldIdx = SimCodeUtil::simEqSystemIndex(simEqIn.clone())?;
                    newIdx = metamodelica::arrayGet(assIn.clone(), oldIdx.clone())?;
                    jacobianMatrix = TDS_replaceSimEqSysIdxInJacobianMatrix(jacobianMatrix.clone(), assIn.clone());
                    assign_field!(
                        lSystem.jacobianMatrix = jacobianMatrix.clone(),
                        lSystem.index = newIdx.clone(),
                        lSystem.residual = eqs.clone()
                    );
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_LINEAR; lSystem = lSystem.clone());
                    Ok(simEqSys.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut newIdx: i32;
                    let mut oldIdx: i32;
                    let mut simEqSys: Arc<SimCode::SimEqSystem>;
                    oldIdx = SimCodeUtil::simEqSystemIndex(simEqIn.clone())?;
                    newIdx = metamodelica::arrayGet(assIn.clone(), oldIdx.clone())?;
                    simEqSys = SimCodeUtil::replaceSimEqSysIndex(simEqIn.clone(), newIdx.clone())?;
                    Ok(simEqSys.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(simEqOut)
}

fn TDS_replaceSimEqSysIndexWithUpdate(mut simEqIn: Arc<SimCode::SimEqSystem>, mut tplIn: (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>))> {
    let mut simEqOut: Arc<SimCode::SimEqSystem>;
    let mut tplOut: (i32, metamodelica::Array<i32>);
    (simEqOut, tplOut) = 'mc: {
        let __mc_input = (simEqIn.clone(), tplIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (simEqSys @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: nlSystem @ Deref @ SimCode::NonlinearSystem { index: oldIdx, eqs, jacobianMatrix, .. }, .. }, (newIdx, ass)) => {
                    let mut simEqSys = (*simEqSys).clone();
                    let mut nlSystem = (*nlSystem).clone();
                    let mut eqs = (*eqs).clone();
                    let mut jacobianMatrix = (*jacobianMatrix).clone();
                    let mut newIdx = (*newIdx).clone();
                    let mut ass = (*ass).clone();
                    let (__pa0, (__pa1, __pa2)) = List::mapFold(eqs.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndexWithUpdate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>))> + 'static>), (newIdx.clone(), ass.clone()))?;
                    eqs = __pa0.clone();
                    newIdx = __pa1.clone();
                    ass = __pa2.clone();
                    let (__pa3, (__pa4, __pa5)) = TDS_replaceSimEqSysIdxInJacobianMatrixWithUpdate(jacobianMatrix.clone(), (newIdx.clone(), ass.clone()));
                    jacobianMatrix = __pa3.clone();
                    newIdx = __pa4.clone();
                    ass = __pa5.clone();
                    ass = metamodelica::arrayUpdate(ass.clone(), oldIdx.clone(), newIdx.clone())?;
                    assign_field!(
                        nlSystem.jacobianMatrix = jacobianMatrix.clone(),
                        nlSystem.index = newIdx.clone(),
                        nlSystem.eqs = eqs.clone()
                    );
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_NONLINEAR; nlSystem = nlSystem.clone());
                    Ok((simEqSys.clone(), (newIdx.clone() + 1, ass.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (simEqSys @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: lSystem @ Deref @ SimCode::LinearSystem { index: oldIdx, residual: eqs, jacobianMatrix, .. }, .. }, (newIdx, ass)) => {
                    let mut simEqSys = (*simEqSys).clone();
                    let mut lSystem = (*lSystem).clone();
                    let mut eqs = (*eqs).clone();
                    let mut jacobianMatrix = (*jacobianMatrix).clone();
                    let mut newIdx = (*newIdx).clone();
                    let mut ass = (*ass).clone();
                    let (__pa0, (__pa1, __pa2)) = List::mapFold(eqs.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndexWithUpdate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>))> + 'static>), (newIdx.clone(), ass.clone()))?;
                    eqs = __pa0.clone();
                    newIdx = __pa1.clone();
                    ass = __pa2.clone();
                    let (__pa3, (__pa4, __pa5)) = TDS_replaceSimEqSysIdxInJacobianMatrixWithUpdate(jacobianMatrix.clone(), (newIdx.clone(), ass.clone()));
                    jacobianMatrix = __pa3.clone();
                    newIdx = __pa4.clone();
                    ass = __pa5.clone();
                    ass = metamodelica::arrayUpdate(ass.clone(), oldIdx.clone(), newIdx.clone())?;
                    assign_field!(
                        lSystem.jacobianMatrix = jacobianMatrix.clone(),
                        lSystem.index = newIdx.clone(),
                        lSystem.residual = eqs.clone()
                    );
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_LINEAR; lSystem = lSystem.clone());
                    Ok((simEqSys.clone(), (newIdx.clone() + 1, ass.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (simEqSys @ Deref @ SimCode::SimEqSystem::SES_MIXED { index: oldIdx, cont, discEqs: eqs, .. }, (newIdx, ass)) => {
                    let mut simEqSys = (*simEqSys).clone();
                    let mut cont = (*cont).clone();
                    let mut eqs = (*eqs).clone();
                    let mut newIdx = (*newIdx).clone();
                    let mut ass = (*ass).clone();
                    let (__pa0, (__pa1, __pa2)) = TDS_replaceSimEqSysIndexWithUpdate(cont.clone(), (newIdx.clone(), ass.clone()))?;
                    cont = __pa0.clone();
                    newIdx = __pa1.clone();
                    ass = __pa2.clone();
                    let (__pa3, (__pa4, __pa5)) = List::mapFold(eqs.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndexWithUpdate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>))> + 'static>), (newIdx.clone(), ass.clone()))?;
                    eqs = __pa3.clone();
                    newIdx = __pa4.clone();
                    ass = __pa5.clone();
                    ass = metamodelica::arrayUpdate(ass.clone(), oldIdx.clone(), newIdx.clone())?;
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_MIXED;
                        cont = cont.clone(),
                        discEqs = eqs.clone()
                    );
                    Ok((simEqSys.clone(), (newIdx.clone() + 1, ass.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (newIdx, ass)) => {
                    let mut oldIdx: i32;
                    let mut simEqSys: Arc<SimCode::SimEqSystem>;
                    let mut ass = (*ass).clone();
                    oldIdx = SimCodeUtil::simEqSystemIndex(simEqIn.clone())?;
                    ass = metamodelica::arrayUpdate(ass.clone(), oldIdx.clone(), newIdx.clone())?;
                    simEqSys = SimCodeUtil::replaceSimEqSysIndex(simEqIn.clone(), newIdx.clone())?;
                    Ok((simEqSys.clone(), (newIdx.clone() + 1, ass.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((simEqOut, tplOut))
}

fn TDS_replaceSimEqSysIdxInJacobianMatrixWithUpdate(mut jacIn: Option<Arc<SimCode::JacobianMatrix>>, mut tplIn: (i32, metamodelica::Array<i32>)) -> (Option<Arc<SimCode::JacobianMatrix>>, (i32, metamodelica::Array<i32>)) {
    let mut jacOut: Option<Arc<SimCode::JacobianMatrix>>;
    let mut tplOut: (i32, metamodelica::Array<i32>);
    (jacOut, tplOut) = 'mc: {
        let __mc_input = (jacIn.clone(), tplIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(Deref @ SimCode::JacobianMatrix { columns: jacCols, seedVars: vars, matrixName: name, sparsity, sparsityT, nonlinear: nonlinearPat, nonlinearT: nonlinearPatT, coloredCols: colCols, coloredRows: colRows, maxColorCols: maxCol, jacobianIndex: jacIdx, partitionIndex: partIdx, generic_loop_calls: Deref @ metamodelica::List::Nil, crefsHT: crefToSimVarHTJacobian, isAdjoint: isAdj }), (newIdx, ass)) => {
                    let mut jacCols = (*jacCols).clone();
                    let mut newIdx = (*newIdx).clone();
                    let mut ass = (*ass).clone();
                    let (__pa0, (__pa1, __pa2)) = List::mapFold(jacCols.clone(), (std::sync::Arc::new(fnptr!(TDS_replaceSimEqSysIdxInJacobianColumnWithUpdate, Arc<SimCode::JacobianColumn>, (i32, metamodelica::Array<i32>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::JacobianColumn>, (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::JacobianColumn>, (i32, metamodelica::Array<i32>))> + 'static>), (newIdx.clone(), ass.clone()))?;
                    jacCols = __pa0.clone();
                    newIdx = __pa1.clone();
                    ass = __pa2.clone();
                    Ok((Some(Arc::new(SimCode::JacobianMatrix { columns: jacCols.clone(), seedVars: vars.clone(), matrixName: (name.clone()).clone(), sparsity: sparsity.clone(), sparsityT: sparsityT.clone(), nonlinear: nonlinearPat.clone(), nonlinearT: nonlinearPatT.clone(), coloredCols: colCols.clone(), coloredRows: colRows.clone(), maxColorCols: maxCol.clone(), jacobianIndex: jacIdx.clone(), partitionIndex: partIdx.clone(), generic_loop_calls: metamodelica::nil(), crefsHT: crefToSimVarHTJacobian.clone(), isAdjoint: isAdj.clone() })), (newIdx.clone(), ass.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((jacIn.clone(), tplIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (jacOut, tplOut)
}

fn TDS_replaceSimEqSysIdxInJacobianColumnWithUpdate(mut jacIn: Arc<SimCode::JacobianColumn>, mut tplIn: (i32, metamodelica::Array<i32>)) -> (Arc<SimCode::JacobianColumn>, (i32, metamodelica::Array<i32>)) {
    let mut jacOut: Arc<SimCode::JacobianColumn>;
    let mut tplOut: (i32, metamodelica::Array<i32>);
    (jacOut, tplOut) = 'mc: {
        let __mc_input = (jacIn.clone(), tplIn.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SimCode::JacobianColumn { columnEqns: simEqs, columnVars: simVars, numberOfResultVars: rowLen, constantEqns: constEqns }, (newIdx, ass)) => {
                    let mut simEqs = (*simEqs).clone();
                    let mut newIdx = (*newIdx).clone();
                    let mut ass = (*ass).clone();
                    let (__pa0, (__pa1, __pa2)) = List::mapFold(simEqs.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndexWithUpdate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>)) -> Result<(Arc<SimCode::SimEqSystem>, (i32, metamodelica::Array<i32>))> + 'static>), (newIdx.clone(), ass.clone()))?;
                    simEqs = __pa0.clone();
                    newIdx = __pa1.clone();
                    ass = __pa2.clone();
                    Ok((Arc::new(SimCode::JacobianColumn { columnEqns: simEqs.clone(), columnVars: simVars.clone(), numberOfResultVars: rowLen.clone(), constantEqns: constEqns.clone() }), (newIdx.clone(), ass.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((jacIn.clone(), tplIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (jacOut, tplOut)
}

fn TDS_replaceSimEqSysIdxInJacobianMatrix(mut jacIn: Option<Arc<SimCode::JacobianMatrix>>, mut assIn: metamodelica::Array<i32>) -> Option<Arc<SimCode::JacobianMatrix>> {
    let mut jacOut: Option<Arc<SimCode::JacobianMatrix>> = jacIn.clone();
    jacOut = 'mc: {
        let __mc_input = jacIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(jacMatrix @ Deref @ SimCode::JacobianMatrix { .. }) => {
                    let mut jacMatrix = (*jacMatrix).clone();
                    assign_field!(jacMatrix.columns = List::map1(jacMatrix.columns.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIdxInJacobianColumn) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::JacobianColumn>, metamodelica::Array<i32>) -> Result<Arc<SimCode::JacobianColumn>> + 'static>), assIn.clone())?);
                    Ok(Some(jacMatrix.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(jacIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    jacOut
}

fn TDS_replaceSimEqSysIdxInJacobianColumn(mut jacIn: Arc<SimCode::JacobianColumn>, mut assIn: metamodelica::Array<i32>) -> Result<Arc<SimCode::JacobianColumn>> {
    let mut jacOut: Arc<SimCode::JacobianColumn> = jacIn.clone();
    assign_field!(jacOut.columnEqns = List::map1(jacOut.columnEqns.clone(), (std::sync::Arc::new(TDS_replaceSimEqSysIndex) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, metamodelica::Array<i32>) -> Result<Arc<SimCode::SimEqSystem>> + 'static>), assIn.clone())?);
    Ok(jacOut)
}

fn TDS_updateModelInfo(mut simCodeIn: SimCode::SimCode, mut idcs: (i32, i32, i32, i32, i32, i32, i32, i32)) -> SimCode::SimCode {
    let mut simCodeOut: SimCode::SimCode = simCodeIn.clone();
    let mut lsIdx: i32;
    let mut nlsIdx: i32;
    let mut mIdx: i32;
    let mut modelInfo: SimCode::ModelInfo;
    let mut varInfo: SimCode::VarInfo;
    (_, _, _, _, _, lsIdx, nlsIdx, mIdx) = idcs;
    modelInfo = simCodeIn.modelInfo.clone();
    varInfo = modelInfo.varInfo.clone();
    varInfo.numStateVars = (modelInfo.vars.stateVars.clone().len() as i32);
    varInfo.numAlgVars = (modelInfo.vars.algVars.clone().len() as i32);
    varInfo.numLinearSystems = if (intEq(varInfo.numLinearSystems.clone(), 0)) {0} else {lsIdx};
    varInfo.numNonLinearSystems = if (intEq(varInfo.numNonLinearSystems.clone(), 0)) {0} else {nlsIdx};
    modelInfo.varInfo = varInfo;
    simCodeOut.modelInfo = modelInfo;
    simCodeOut
}

fn TDS_duplicateTasks(mut clustersIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut taskAssIn: metamodelica::Array<i32>, mut procAssIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut idcsIn: (i32, i32, i32, i32, i32, i32, i32, i32), mut taskGraphOrig: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphTOrig: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskDuplAssIn: metamodelica::Array<i32>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut simCodeIn: SimCode::SimCode, mut scheduleIn: Arc<HpcOmSimCode::Schedule>, mut sccSimEqMappingIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut duplSccSimEqMapIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut duplCompsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, (i32, i32, i32, i32, i32, i32, i32, i32), SimCode::SimCode, Arc<HpcOmSimCode::Schedule>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut taskAssOut: metamodelica::Array<i32>;
    let mut procAssOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskGraphOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskDuplAssOut: metamodelica::Array<i32>;
    let mut idcsOut: (i32, i32, i32, i32, i32, i32, i32, i32);
    let mut simCodeOut: SimCode::SimCode;
    let mut scheduleOut: Arc<HpcOmSimCode::Schedule>;
    let mut duplSccSimEqMapOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut duplCompsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    (taskAssOut, procAssOut, taskGraphOut, taskDuplAssOut, idcsOut, simCodeOut, scheduleOut, duplSccSimEqMapOut, duplCompsOut) = (::match_deref::match_deref! { match &(clustersIn.clone()) {
        Deref @ metamodelica::List::Nil => {
            (taskAssIn.clone(), procAssIn.clone(), taskGraphIn.clone(), taskDuplAssIn.clone(), idcsIn, simCodeIn, scheduleIn, duplSccSimEqMapIn, duplCompsIn)
        },
        Deref @ metamodelica::List::Cons { head: cluster, tail: rest } => {
            let mut threadIdx: i32;
            let mut compIdx: i32;
            let mut simVarIdx: i32;
            let mut simEqSysIdx: i32;
            let mut taskIdx: i32;
            let mut lsIdx: i32;
            let mut nlsIdx: i32;
            let mut mIdx: i32;
            let mut duplSccSimEqMap: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut duplComps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut taskAss: metamodelica::Array<i32>;
            let mut taskDuplAss: metamodelica::Array<i32>;
            let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut idcs: (i32, i32, i32, i32, i32, i32, i32, i32);
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut simCode: SimCode::SimCode;
            let mut schedule: Arc<HpcOmSimCode::Schedule>;
            let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
            let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
            repl = BackendVarTransform::emptyReplacements();
            let (__pa0, __pa1, __pa2, __pa3, __pa4, (__pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12), __pa13, __pa14, __pa15) = TDS_duplicateTasks1(cluster.clone(), clustersIn, repl.clone(), taskAssIn.clone(), procAssIn.clone(), metamodelica::nil(), idcsIn, taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraphIn.clone(), taskDuplAssIn.clone(), iTaskGraphMeta.clone(), simCodeIn, sccSimEqMappingIn.clone(), duplSccSimEqMapIn, duplCompsIn)?;
            taskAss = __pa0.clone();
            procAss = __pa1.clone();
            taskGraph = __pa2.clone();
            taskDuplAss = __pa3.clone();
            thread = __pa4.clone();
            threadIdx = __pa5.clone();
            taskIdx = __pa6.clone();
            compIdx = __pa7.clone();
            simVarIdx = __pa8.clone();
            simEqSysIdx = __pa9.clone();
            lsIdx = __pa10.clone();
            nlsIdx = __pa11.clone();
            mIdx = __pa12.clone();
            simCode = __pa13.clone();
            duplSccSimEqMap = __pa14.clone();
            duplComps = __pa15.clone();
            let SimCode::SIMCODE { .. } = (simCode.clone()) else { bail!("pattern mismatch") };
            let (__pa16, __pa17, __pa18) = ::match_deref::match_deref! { match &(scheduleIn) {
                Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __pa16, outgoingDepTasks: __pa17, allCalcTasks: __pa18, .. } => (__pa16.clone(), __pa17.clone(), __pa18.clone()),
                _ => bail!("pattern mismatch"),
            } };
            threadTasks = __pa16.clone();
            outgoingDepTasks = __pa17.clone();
            allCalcTasks = __pa18.clone();
            threadTasks = metamodelica::arrayUpdate(threadTasks.clone(), threadIdx.clone(), thread.clone().reverse())?;
            schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            threadIdx = threadIdx.clone() + 1;
            (taskAss, procAss, taskGraph, taskDuplAss, idcs, simCode, schedule, duplSccSimEqMap, duplComps) = TDS_duplicateTasks(rest.clone(), taskAss.clone(), procAss.clone(), (threadIdx.clone(), taskIdx.clone(), compIdx.clone(), simVarIdx.clone(), simEqSysIdx.clone(), lsIdx.clone(), nlsIdx.clone(), mIdx.clone()), taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraph.clone(), taskDuplAss.clone(), iTaskGraphMeta, simCode.clone(), schedule.clone(), sccSimEqMappingIn.clone(), duplSccSimEqMap.clone(), duplComps.clone())?;
            (taskAssIn.clone(), procAssIn.clone(), taskGraph.clone(), taskDuplAss.clone(), idcs.clone(), simCode.clone(), schedule.clone(), duplSccSimEqMap.clone(), duplComps.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((taskAssOut, procAssOut, taskGraphOut, taskDuplAssOut, idcsOut, simCodeOut, scheduleOut, duplSccSimEqMapOut, duplCompsOut))
}

fn TDS_duplicateTasks1(mut clusterIn: Arc<metamodelica::List<i32>>, mut allCluster: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut replIn: BackendVarTransform::VariableReplacements, mut taskAssIn: metamodelica::Array<i32>, mut procAssIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut threadIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut idcsIn: (i32, i32, i32, i32, i32, i32, i32, i32), mut taskGraphOrig: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphTOrig: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskDuplAssIn: metamodelica::Array<i32>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut simCodeIn: SimCode::SimCode, mut sccSimEqMappingIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut duplSccSimEqMapIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut duplCompsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, (i32, i32, i32, i32, i32, i32, i32, i32), SimCode::SimCode, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut taskAssOut: metamodelica::Array<i32>;
    let mut procAssOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskGraphOut: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut taskDuplAssOut: metamodelica::Array<i32>;
    let mut threadOut: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut idcsOut: (i32, i32, i32, i32, i32, i32, i32, i32);
    let mut simCodeOut: SimCode::SimCode;
    let mut duplSccSimEqMapOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut duplCompsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    (taskAssOut, procAssOut, taskGraphOut, taskDuplAssOut, threadOut, idcsOut, simCodeOut, duplSccSimEqMapOut, duplCompsOut) = 'mc: {
        let __mc_input = clusterIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((taskAssIn.clone(), procAssIn.clone(), taskGraphIn.clone(), taskDuplAssIn.clone(), threadIn.clone(), idcsIn.clone(), simCodeIn.clone(), duplSccSimEqMapIn.clone(), duplCompsIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: node, tail: rest } => {
                    let mut ass: i32;
                    let mut duplSccSimEqMap: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut duplComps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut taskAss: metamodelica::Array<i32>;
                    let mut taskDuplAss: metamodelica::Array<i32>;
                    let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut idcs: (i32, i32, i32, i32, i32, i32, i32, i32);
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut simCode: SimCode::SimCode;
                    let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
                    ass = metamodelica::arrayGet(taskAssIn.clone(), node.clone())?;
                    let true = (intNe(ass.clone(), -1)) else { bail!("pattern mismatch") };
                    (repl, taskAss, procAss, taskGraph, taskDuplAss, thread, idcs, simCode, duplSccSimEqMap, duplComps) = TDS_duplicateTasks2(node.clone(), allCluster.clone(), replIn.clone(), taskAssIn.clone(), procAssIn.clone(), threadIn.clone(), idcsIn.clone(), taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraphIn.clone(), taskDuplAssIn.clone(), iTaskGraphMeta.clone(), simCodeIn.clone(), sccSimEqMappingIn.clone(), duplSccSimEqMapIn.clone(), duplCompsIn.clone())?;
                    (taskAss, procAss, taskGraph, taskDuplAss, thread, idcs, simCode, duplSccSimEqMap, duplComps) = TDS_duplicateTasks1(rest.clone(), allCluster.clone(), repl.clone(), taskAss.clone(), procAss.clone(), thread.clone(), idcs.clone(), taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraph.clone(), taskDuplAss.clone(), iTaskGraphMeta.clone(), simCode.clone(), sccSimEqMappingIn.clone(), duplSccSimEqMap.clone(), duplComps.clone())?;
                    Ok((taskAss.clone(), procAss.clone(), taskGraph.clone(), taskDuplAss.clone(), thread.clone(), idcs.clone(), simCode.clone(), duplSccSimEqMap.clone(), duplComps.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: node, tail: rest } => {
                    let mut ass: i32;
                    let mut threadIdx: i32;
                    let mut comps: Arc<metamodelica::List<i32>>;
                    let mut simEqs: Arc<metamodelica::List<i32>>;
                    let mut taskLst: Arc<metamodelica::List<i32>>;
                    let mut origPredTasks: Arc<metamodelica::List<i32>>;
                    let mut clPredTasks: Arc<metamodelica::List<i32>>;
                    let mut duplPredTasks: Arc<metamodelica::List<i32>>;
                    let mut clTasks: Arc<metamodelica::List<i32>>;
                    let mut pos: Arc<metamodelica::List<i32>>;
                    let mut duplSccSimEqMap: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut duplComps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut simEqsLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut taskAss: metamodelica::Array<i32>;
                    let mut taskDuplAss: metamodelica::Array<i32>;
                    let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut idcs: (i32, i32, i32, i32, i32, i32, i32, i32);
                    let mut task: Arc<HpcOmSimCode::Task>;
                    let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut simCode: SimCode::SimCode;
                    let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
                    let mut odes: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>>;
                    let mut simEqSysts: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
                    let mut allEqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
                    let mut taskGraphOut: metamodelica::Array<Arc<metamodelica::List<i32>>> = taskGraphOut.clone();
                    ass = metamodelica::arrayGet(taskAssIn.clone(), node.clone())?;
                    let true = (intEq(ass.clone(), -1)) else { bail!("pattern mismatch") };
                    (threadIdx, _, _, _, _, _, _, _) = idcsIn.clone();
                    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
                    inComps = __pa0.clone();
                    taskAss = metamodelica::arrayUpdate(taskAssIn.clone(), node.clone(), threadIdx.clone())?;
                    taskLst = metamodelica::arrayGet(procAssIn.clone(), threadIdx.clone())?;
                    procAss = metamodelica::arrayUpdate(procAssIn.clone(), threadIdx.clone(), metamodelica::cons(node.clone(), taskLst.clone()))?;
                    comps = metamodelica::arrayGet(inComps.clone(), node.clone())?;
                    simEqsLst = List::map1(comps.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), sccSimEqMappingIn.clone())?;
                    simEqs = List::flatten(simEqsLst.clone())?;
                    simEqs = simEqs.clone().reverse();
                    let SimCode::SIMCODE { odeEquations: __pa1, allEquations: __pa2, .. } = (simCodeIn.clone()) else { bail!("pattern mismatch") };
                    odes = __pa1.clone();
                    allEqs = __pa2.clone();
                    simEqSysts = List::map1(simEqs.clone(), (std::sync::Arc::new(SimCodeUtil::getSimEqSysForIndex) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Arc<SimCode::SimEqSystem>> + 'static>), List::flatten(odes.clone())?)?;
                    (simEqSysts, _) = replaceInSimEqSystemLst(simEqSysts.clone(), replIn.clone())?;
                    allEqs = replaceSimEqSystemLstWithSameIndex(simEqSysts.clone(), allEqs.clone())?;
                    odes = List::map1r(odes.clone(), (std::sync::Arc::new(replaceSimEqSystemLstWithSameIndex) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>> + 'static>), simEqSysts.clone())?;
                    simCode = SimCodeUtil::replaceODEandALLequations(allEqs.clone(), odes.clone(), simCodeIn.clone());
                    clTasks = listHead(allCluster.clone())?;
                    origPredTasks = metamodelica::arrayGet(taskGraphTOrig.clone(), node.clone())?;
                    (clPredTasks, origPredTasks, _) = List::intersection1OnTrue(origPredTasks.clone(), clTasks.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    pos = List::map1(clPredTasks.clone(), (std::sync::Arc::new(List::position) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<i32> + 'static>), clTasks.clone())?;
                    clTasks = metamodelica::arrayGet(procAssIn.clone(), threadIdx.clone())?;
                    clTasks = clTasks.clone().reverse();
                    clPredTasks = List::map1(pos.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), clTasks.clone())?;
                    (duplPredTasks, _, _) = List::intersection1OnTrue(clPredTasks.clone(), clTasks.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
                    taskGraph = List::fold1(duplPredTasks.clone(), (std::sync::Arc::new(Array::appendToElement) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<_> + 'static>), list![node.clone()], taskGraphIn.clone())?;
                    taskGraphOut = List::fold1(origPredTasks.clone(), (std::sync::Arc::new(Array::appendToElement) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<_> + 'static>), list![node.clone()], taskGraph.clone())?;
                    task = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: 1, index: node.clone(), calcTime: metamodelica::OrderedFloat(0.0_f64), timeFinished: metamodelica::OrderedFloat(-1.0_f64), threadIdx: threadIdx.clone(), eqIdc: simEqs.clone() });
                    thread = metamodelica::cons(task.clone(), threadIn.clone());
                    taskDuplAss = metamodelica::arrayUpdate(taskDuplAssIn.clone(), node.clone(), node.clone())?;
                    (taskAss, procAss, taskGraph, taskDuplAss, thread, idcs, simCode, duplSccSimEqMap, duplComps) = TDS_duplicateTasks1(rest.clone(), allCluster.clone(), replIn.clone(), taskAss.clone(), procAss.clone(), thread.clone(), idcsIn.clone(), taskGraphOrig.clone(), taskGraphTOrig.clone(), taskGraph.clone(), taskDuplAss.clone(), iTaskGraphMeta.clone(), simCode.clone(), sccSimEqMappingIn.clone(), duplSccSimEqMapIn.clone(), duplCompsIn.clone())?;
                    Ok(((taskAss.clone(), procAss.clone(), taskGraph.clone(), taskDuplAss.clone(), thread.clone(), idcs.clone(), simCode.clone(), duplSccSimEqMap.clone(), duplComps.clone()), taskGraphOut.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { taskGraphOut = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((taskAssOut, procAssOut, taskGraphOut, taskDuplAssOut, threadOut, idcsOut, simCodeOut, duplSccSimEqMapOut, duplCompsOut))
}

fn TDS_duplicateTasks2(mut node: i32, mut allCluster: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut replIn: BackendVarTransform::VariableReplacements, mut taskAssIn: metamodelica::Array<i32>, mut procAssIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut threadIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut idcsIn: (i32, i32, i32, i32, i32, i32, i32, i32), mut taskGraphOrig: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphTOrig: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskDuplAssIn: metamodelica::Array<i32>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut simCodeIn: SimCode::SimCode, mut sccSimEqMappingIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut duplSccSimEqMapIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut duplCompsIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<(BackendVarTransform::VariableReplacements, metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, (i32, i32, i32, i32, i32, i32, i32, i32), SimCode::SimCode, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
    let mut replOut: BackendVarTransform::VariableReplacements;
    let mut taskAssOut: metamodelica::Array<i32>;
    let mut procAssOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskGraphOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut taskDuplAssOut: metamodelica::Array<i32>;
    let mut threadOut: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut idcsOut: (i32, i32, i32, i32, i32, i32, i32, i32);
    let mut simCodeOut: SimCode::SimCode;
    let mut duplSccSimEqMapOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut duplCompsOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut crefAppend: ArcStr;
    let mut threadIdx: i32;
    let mut compIdx: i32;
    let mut simVarIdx: i32;
    let mut simVarIdx2: i32;
    let mut simEqSysIdx: i32;
    let mut simEqSysIdx2: i32;
    let mut simEqSysIdx3: i32;
    let mut numVars: i32;
    let mut numEqs: i32;
    let mut numInitEqs: i32;
    let mut taskIdx: i32;
    let mut lsIdx: i32;
    let mut nlsIdx: i32;
    let mut mIdx: i32;
    let mut comps: Arc<metamodelica::List<i32>>;
    let mut simVarSysIdcs2: Arc<metamodelica::List<i32>>;
    let mut simEqSysIdcs: Arc<metamodelica::List<i32>>;
    let mut simEqSysIdcs2: Arc<metamodelica::List<i32>>;
    let mut simEqSysIdcsInit: Arc<metamodelica::List<i32>>;
    let mut thread: Arc<metamodelica::List<i32>>;
    let mut clTasks: Arc<metamodelica::List<i32>>;
    let mut origPredTasks: Arc<metamodelica::List<i32>>;
    let mut clPredTasks: Arc<metamodelica::List<i32>>;
    let mut duplPredTasks: Arc<metamodelica::List<i32>>;
    let mut pos: Arc<metamodelica::List<i32>>;
    let mut simEqIdxLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut taskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut ht: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr));
    let mut simVars: SimCodeVar::SimVars;
    let mut simCode: SimCode::SimCode;
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut crefsDupl: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut crefLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>;
    let mut crefsDuplExp: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut simVarLst: Arc<metamodelica::List<SimCodeVar::SimVar>>;
    let mut simVarDupl: Arc<metamodelica::List<SimCodeVar::SimVar>>;
    let mut simEqSysts: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
    let mut simEqSystsDupl: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
    let mut initEqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
    let mut odes: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>>;
    let HpcOmTaskGraph::TASKGRAPHMETA { inComps: __pa0, .. } = (iTaskGraphMeta) else { bail!("pattern mismatch") };
    inComps = __pa0.clone();
    let SimCode::SIMCODE { modelInfo: SimCode::MODELINFO { vars: __pa1, .. }, odeEquations: __pa2, crefToSimVarHT: __pa3, .. } = (simCodeIn.clone()) else { bail!("pattern mismatch") };
    simVars = __pa1.clone();
    odes = __pa2.clone();
    ht = __pa3.clone();
    (threadIdx, taskIdx, compIdx, simVarIdx, simEqSysIdx, lsIdx, nlsIdx, mIdx) = idcsIn;
    comps = metamodelica::arrayGet(inComps.clone(), node)?;
    comps = comps.reverse();
    simEqIdxLst = List::map1(comps.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), sccSimEqMappingIn.clone())?;
    simEqSysIdcs = List::flatten(simEqIdxLst)?;
    crefLst = List::map1(simEqSysIdcs.clone(), (std::sync::Arc::new(SimCodeUtil::getAssignedCrefsOfSimEq) as std::sync::Arc<dyn ::std::ops::Fn(i32, SimCode::SimCode) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>), simCodeIn.clone())?;
    crefs = List::flatten(crefLst)?;
    simVarLst = List::map1(crefs.clone(), (std::sync::Arc::new(BaseHashTable::get) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), ht.clone())?;
    numVars = (simVarLst.clone().len() as i32);
    simVarSysIdcs2 = List::intRange2(simVarIdx, simVarIdx + numVars - 1);
    crefAppend = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("_thr")); __mm_s.push_str(&*intString(threadIdx)); ArcStr::from(__mm_s) }).clone();
    crefsDupl = List::map1r(crefs.clone(), (std::sync::Arc::new(ComponentReference::appendStringLastIdent) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), (crefAppend).clone())?;
    crefsDuplExp = List::map(crefsDupl.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>))?;
    simVarDupl = List::threadMap(crefsDupl.clone(), simVarLst, (std::sync::Arc::new(fnptr!(SimCodeUtil::replaceSimVarName, Arc<DAE::ComponentRef>, SimCodeVar::SimVar)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, SimCodeVar::SimVar) -> Result<SimCodeVar::SimVar> + 'static>))?;
    simVarDupl = List::threadMap(simVarSysIdcs2, simVarDupl, (std::sync::Arc::new(fnptr!(SimCodeUtil::replaceSimVarIndex, i32, SimCodeVar::SimVar)) as std::sync::Arc<dyn ::std::ops::Fn(i32, SimCodeVar::SimVar) -> Result<SimCodeVar::SimVar> + 'static>))?;
    simCode = List::fold(simVarDupl.clone(), (std::sync::Arc::new(fnptr!(SimCodeUtil::addSimVarToAlgVars, SimCodeVar::SimVar, SimCode::SimCode)) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar, SimCode::SimCode) -> Result<SimCode::SimCode> + 'static>), simCodeIn)?;
    simVarIdx2 = simVarIdx + numVars;
    ht = List::fold(simVarDupl, (std::sync::Arc::new(HashTableCrefSimVar::addSimVarToHashTable) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar) -> Result<ArcStr> + 'static>))> + 'static>), ht)?;
    repl = BackendVarTransform::addReplacements(replIn, crefs.clone(), crefsDuplExp, None)?;
    simEqSysts = List::map1(simEqSysIdcs, (std::sync::Arc::new(SimCodeUtil::getSimEqSysForIndex) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Arc<SimCode::SimEqSystem>> + 'static>), List::flatten(odes)?)?;
    numEqs = (simEqSysts.clone().len() as i32);
    simEqSysIdcs2 = List::intRange2(simEqSysIdx, simEqSysIdx + numEqs - 1);
    (simEqSystsDupl, _) = List::map1_2(simEqSysts, (std::sync::Arc::new(replaceExpsInSimEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, BackendVarTransform::VariableReplacements) -> Result<(Arc<SimCode::SimEqSystem>, bool)> + 'static>), repl.clone())?;
    let (__pa4, (__pa5, __pa6, __pa7)) = List::mapFold(simEqSystsDupl, (std::sync::Arc::new(fnptr!(replaceSystemIndex, Arc<SimCode::SimEqSystem>, (i32, i32, i32))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, (i32, i32, i32)) -> Result<(Arc<SimCode::SimEqSystem>, (i32, i32, i32))> + 'static>), (lsIdx, nlsIdx, mIdx))?;
    simEqSystsDupl = __pa4.clone();
    lsIdx = __pa5.clone();
    nlsIdx = __pa6.clone();
    mIdx = __pa7.clone();
    simEqSystsDupl = List::threadMap(simEqSystsDupl, simEqSysIdcs2.clone(), (std::sync::Arc::new(SimCodeUtil::replaceSimEqSysIndex) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, i32) -> Result<Arc<SimCode::SimEqSystem>> + 'static>))?;
    simEqSysIdx2 = simEqSysIdx + numEqs;
    (simEqSystsDupl, simEqSysIdx2) = TDS_duplicateSystemOfEquations(simEqSystsDupl, simEqSysIdx2, repl.clone(), metamodelica::nil())?;
    duplSccSimEqMapOut = listAppend(List::map(simEqSysIdcs2.clone(), std::sync::Arc::new(fnptr!(List::create, _)))?, duplSccSimEqMapIn);
    simCode = List::fold1(simEqSystsDupl, (std::sync::Arc::new(SimCodeUtil::addSimEqSysToODEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, i32, SimCode::SimCode) -> Result<SimCode::SimCode> + 'static>), 1, simCode)?;
    threadOut = metamodelica::cons(Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: 1, index: taskIdx, calcTime: metamodelica::OrderedFloat(0.0_f64), timeFinished: metamodelica::OrderedFloat(-1.0_f64), threadIdx: threadIdx, eqIdc: simEqSysIdcs2 }), threadIn);
    numInitEqs = (crefs.clone().len() as i32);
    simEqSysIdcsInit = List::intRange2(simEqSysIdx2, simEqSysIdx2 + numInitEqs - 1);
    initEqs = List::thread3Map(crefsDupl, crefs, simEqSysIdcsInit, (std::sync::Arc::new(makeSEScrefAssignment) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>, i32) -> Result<Arc<SimCode::SimEqSystem>> + 'static>))?;
    simCode = List::fold(initEqs, (std::sync::Arc::new(fnptr!(SimCodeUtil::addSimEqSysToInitialEquations, Arc<SimCode::SimEqSystem>, SimCode::SimCode)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, SimCode::SimCode) -> Result<SimCode::SimCode> + 'static>), simCode)?;
    simEqSysIdx3 = simEqSysIdx2 + numInitEqs;
    let SimCode::SIMCODE { odeEquations: __pa8, .. } = (simCode.clone()) else { bail!("pattern mismatch") };
    odes = __pa8.clone();
    taskAssOut = metamodelica::arrayUpdate(taskAssIn.clone(), taskIdx, threadIdx)?;
    thread = metamodelica::arrayGet(procAssIn.clone(), threadIdx)?;
    thread = metamodelica::cons(taskIdx, thread);
    procAssOut = metamodelica::arrayUpdate(procAssIn.clone(), threadIdx, thread)?;
    comps = List::intRange2(compIdx, compIdx + (comps.len() as i32) - 1);
    compIdx = compIdx + (comps.clone().len() as i32);
    duplCompsOut = metamodelica::cons(comps, duplCompsIn);
    taskDuplAssOut = metamodelica::arrayUpdate(taskDuplAssIn.clone(), taskIdx, node)?;
    clTasks = listHead(allCluster)?;
    origPredTasks = metamodelica::arrayGet(taskGraphTOrig.clone(), node)?;
    (clPredTasks, origPredTasks, _) = List::intersection1OnTrue(origPredTasks, clTasks.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    pos = List::map1(clPredTasks, (std::sync::Arc::new(List::position) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<i32> + 'static>), clTasks)?;
    clTasks = metamodelica::arrayGet(procAssOut.clone(), threadIdx)?;
    clTasks = clTasks.reverse();
    clPredTasks = List::map1(pos, (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), clTasks.clone())?;
    (duplPredTasks, _, _) = List::intersection1OnTrue(clPredTasks, clTasks, (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    taskGraph = List::fold1(duplPredTasks, (std::sync::Arc::new(Array::appendToElement) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<_> + 'static>), list![taskIdx], taskGraphIn.clone())?;
    taskGraphOut = List::fold1(origPredTasks, (std::sync::Arc::new(Array::appendToElement) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<_> + 'static>), list![taskIdx], taskGraph.clone())?;
    idcsOut = (threadIdx, taskIdx + 1, compIdx, simVarIdx2, simEqSysIdx3, lsIdx, nlsIdx, mIdx);
    simCodeOut = simCode;
    replOut = repl;
    Ok((replOut, taskAssOut, procAssOut, taskGraphOut, taskDuplAssOut, threadOut, idcsOut, simCodeOut, duplSccSimEqMapOut, duplCompsOut))
}

fn TDS_duplicateSystemOfEquations(mut simEqsIn: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut simEqSysIdxIn: i32, mut repl: BackendVarTransform::VariableReplacements, mut simEqsFold: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, i32)> {
    let mut simEqsOut: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
    let mut simEqSysIdxOut: i32;
    (simEqsOut, simEqSysIdxOut) = 'mc: {
        let __mc_input = simEqsIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((simEqsFold.clone().reverse(), simEqSysIdxIn))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: simEqSys @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: lSystem @ Deref @ SimCode::LinearSystem { residual, .. }, .. }, tail: rest } => {
                    let mut simEqSysIdx: i32;
                    let mut numEqs: i32;
                    let mut systSimEqSysIdcs2: Arc<metamodelica::List<i32>>;
                    let mut duplicated: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut lSystem = (*lSystem).clone();
                    numEqs = (residual.clone().len() as i32);
                    systSimEqSysIdcs2 = if (intEq(numEqs.clone(), 0)) {metamodelica::nil()} else {List::intRange2(simEqSysIdxIn, simEqSysIdxIn + numEqs.clone() - 1)};
                    (duplicated, _) = List::map1_2(residual.clone(), (std::sync::Arc::new(replaceExpsInSimEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, BackendVarTransform::VariableReplacements) -> Result<(Arc<SimCode::SimEqSystem>, bool)> + 'static>), repl.clone())?;
                    duplicated = List::threadMap(duplicated.clone(), systSimEqSysIdcs2.clone(), (std::sync::Arc::new(SimCodeUtil::replaceSimEqSysIndex) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, i32) -> Result<Arc<SimCode::SimEqSystem>> + 'static>))?;
                    assign_field!(lSystem.residual = duplicated.clone());
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_LINEAR; lSystem = lSystem.clone());
                    simEqSysIdx = simEqSysIdxIn + numEqs.clone();
                    (duplicated, simEqSysIdx) = TDS_duplicateSystemOfEquations(rest.clone(), simEqSysIdx.clone(), repl.clone(), metamodelica::cons(simEqSys.clone(), simEqsFold.clone()))?;
                    Ok((duplicated.clone(), simEqSysIdx.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut simEqSysIdx: i32;
                    let mut simEqSys: Arc<SimCode::SimEqSystem>;
                    let mut rest: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
                    let mut duplicated: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(simEqsIn.clone()) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    simEqSys = __pa0.clone();
                    rest = __pa1.clone();
                    (duplicated, simEqSysIdx) = TDS_duplicateSystemOfEquations(rest.clone(), simEqSysIdxIn, repl.clone(), metamodelica::cons(simEqSys.clone(), simEqsFold.clone()))?;
                    Ok((duplicated.clone(), simEqSysIdx.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((simEqsOut, simEqSysIdxOut))
}

fn makeSEScrefAssignment(mut lhs: Arc<DAE::ComponentRef>, mut rhs: Arc<DAE::ComponentRef>, mut idx: i32) -> Result<Arc<SimCode::SimEqSystem>> {
    let mut sesOut: Arc<SimCode::SimEqSystem>;
    let mut ty: Arc<DAE::Type>;
    ty = ComponentReference::crefType(rhs.clone())?;
    sesOut = Arc::new(SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: idx, cref: lhs, exp: Arc::new(DAE::Exp::CREF { componentRef: rhs, ty: ty }), source: DAE::emptyElementSource().clone(), eqAttr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() });
    Ok(sesOut)
}

fn replaceSimEqSystemLstWithSameIndex(mut eqSystsIn: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut eqSysLstIn: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>> {
    let mut eqSysLstOut: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
    eqSysLstOut = List::fold(eqSystsIn, (std::sync::Arc::new(fnptr!(replaceSimEqSystemWithSameIndex, Arc<SimCode::SimEqSystem>, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>> + 'static>), eqSysLstIn)?;
    Ok(eqSysLstOut)
}

fn replaceSimEqSystemWithSameIndex(mut eqSysIn: Arc<SimCode::SimEqSystem>, mut eqSysLstIn: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> {
    let mut eqSysLstOut: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
    eqSysLstOut = 'mc: {
        let __mc_input = eqSysLstIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut pos: i32;
                    let mut eqSysLst: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
                    pos = List::position1OnTrue(eqSysLstIn.clone(), (std::sync::Arc::new(SimCodeUtil::equationIndexEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, Arc<SimCode::SimEqSystem>) -> Result<bool> + 'static>), eqSysIn.clone())?;
                    eqSysLst = List::replaceAt(eqSysIn.clone(), pos.clone(), eqSysLstIn.clone())?;
                    Ok(eqSysLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(eqSysLstIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    eqSysLstOut
}

fn replaceSystemIndex(mut simEqSysIn: Arc<SimCode::SimEqSystem>, mut idcsIn: (i32, i32, i32)) -> (Arc<SimCode::SimEqSystem>, (i32, i32, i32)) {
    let mut simEqSysOut: Arc<SimCode::SimEqSystem>;
    let mut idcsOut: (i32, i32, i32);
    (simEqSysOut, idcsOut) = (::match_deref::match_deref! { match &(simEqSysIn.clone()) {
        simEqSys @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem, .. } => {
            let mut lsIdx: i32;
            let mut nlsIdx: i32;
            let mut mIdx: i32;
            let mut simEqSys = (*simEqSys).clone();
            let mut lSystem = (*lSystem).clone();
            (lsIdx, nlsIdx, mIdx) = idcsIn;
            assign_field!(lSystem.indexLinearSystem = lsIdx.clone());
            assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_LINEAR; lSystem = lSystem.clone());
            (simEqSys.clone(), (lsIdx.clone() + 1, nlsIdx.clone(), mIdx.clone()))
        },
        simEqSys @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem, .. } => {
            let mut lsIdx: i32;
            let mut nlsIdx: i32;
            let mut mIdx: i32;
            let mut simEqSys = (*simEqSys).clone();
            let mut nlSystem = (*nlSystem).clone();
            (lsIdx, nlsIdx, mIdx) = idcsIn;
            assign_field!(nlSystem.indexNonLinearSystem = nlsIdx.clone());
            assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_NONLINEAR; nlSystem = nlSystem.clone());
            (simEqSys.clone(), (lsIdx.clone(), nlsIdx.clone() + 1, mIdx.clone()))
        },
        simEqSys @ Deref @ SimCode::SimEqSystem::SES_MIXED { .. } => {
            let mut lsIdx: i32;
            let mut nlsIdx: i32;
            let mut mIdx: i32;
            let mut simEqSys = (*simEqSys).clone();
            (lsIdx, nlsIdx, mIdx) = idcsIn;
            assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_MIXED; indexMixedSystem = mIdx.clone());
            (simEqSys.clone(), (lsIdx.clone(), nlsIdx.clone(), mIdx.clone() + 1))
        },
        _ => {
            (simEqSysIn, idcsIn)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (simEqSysOut, idcsOut)
}

fn replaceInSimEqSystemLst(mut simEqSysLstIn: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut replIn: BackendVarTransform::VariableReplacements) -> Result<(Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, Arc<metamodelica::List<bool>>)> {
    let mut simEqSysLstOut: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
    let mut changedOut: Arc<metamodelica::List<bool>>;
    (simEqSysLstOut, changedOut) = List::map1_2(simEqSysLstIn, (std::sync::Arc::new(replaceExpsInSimEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, BackendVarTransform::VariableReplacements) -> Result<(Arc<SimCode::SimEqSystem>, bool)> + 'static>), replIn)?;
    Ok((simEqSysLstOut, changedOut))
}

fn replaceExpsInSimEqSystem(mut simEqSysIn: Arc<SimCode::SimEqSystem>, mut replIn: BackendVarTransform::VariableReplacements) -> Result<(Arc<SimCode::SimEqSystem>, bool)> {
    let mut simEqSysOut: Arc<SimCode::SimEqSystem>;
    let mut changedOut: bool;
    (simEqSysOut, changedOut) = 'mc: {
        let __mc_input = simEqSysIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_RESIDUAL { .. } => {
                    let mut changed: bool;
                    let mut exp: Arc<DAE::Exp>;
                    let mut simEqSys = (*simEqSys).clone();
                    (exp, changed) = BackendVarTransform::replaceExp(var_field!((*simEqSys).exp, SimCode::SimEqSystem::SES_RESIDUAL).clone(), replIn.clone(), None);
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_RESIDUAL; exp = exp.clone());
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { cref, exp, .. } => {
                    let mut changed: bool;
                    let mut hasRepl: bool;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut cref = (*cref).clone();
                    let mut exp = (*exp).clone();
                    hasRepl = BackendVarTransform::hasReplacement(replIn.clone(), cref.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(if (hasRepl.clone()) {BackendVarTransform::getReplacement(replIn.clone(), cref.clone())?} else {Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone() })}) {
                        Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cref = __pa0.clone();
                    (exp, changed) = BackendVarTransform::replaceExp(exp.clone(), replIn.clone(), None);
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_SIMPLE_ASSIGN;
                        cref = cref.clone(),
                        exp = exp.clone()
                    );
                    Ok((simEqSys.clone(), changed.clone() || hasRepl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { cref, exp, .. } => {
                    let mut changed: bool;
                    let mut hasRepl: bool;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut cref = (*cref).clone();
                    let mut exp = (*exp).clone();
                    hasRepl = BackendVarTransform::hasReplacement(replIn.clone(), cref.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(if (hasRepl.clone()) {BackendVarTransform::getReplacement(replIn.clone(), cref.clone())?} else {Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone() })}) {
                        Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cref = __pa0.clone();
                    (exp, changed) = BackendVarTransform::replaceExp(exp.clone(), replIn.clone(), None);
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS;
                        cref = cref.clone(),
                        exp = exp.clone()
                    );
                    Ok((simEqSys.clone(), changed.clone() || hasRepl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { lhs, exp, .. } => {
                    let mut changed: bool;
                    let mut hasRepl: bool;
                    let mut cref: Arc<DAE::ComponentRef>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut lhs = (*lhs).clone();
                    let mut exp = (*exp).clone();
                    cref = Expression::expCref(lhs.clone())?;
                    hasRepl = BackendVarTransform::hasReplacement(replIn.clone(), cref.clone())?;
                    lhs = if (hasRepl.clone()) {BackendVarTransform::getReplacement(replIn.clone(), cref.clone())?} else {Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone() })};
                    (exp, changed) = BackendVarTransform::replaceExp(exp.clone(), replIn.clone(), None);
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN;
                        lhs = lhs.clone(),
                        exp = exp.clone()
                    );
                    Ok((simEqSys.clone(), changed.clone() || hasRepl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_IFEQUATION { ifbranches: ifs, elsebranch, .. } => {
                    let mut changed: bool;
                    let mut bLst: Arc<metamodelica::List<bool>>;
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut simEqSysLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut ifs = (*ifs).clone();
                    let mut elsebranch = (*elsebranch).clone();
                    expLst = List::map(ifs.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)))?;
                    (expLst, changed) = BackendVarTransform::replaceExpList(expLst.clone(), replIn.clone(), None);
                    simEqSysLstLst = List::map(ifs.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)))?;
                    (simEqSysLstLst, _) = List::map1_2(simEqSysLstLst.clone(), (std::sync::Arc::new(replaceInSimEqSystemLst) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, BackendVarTransform::VariableReplacements) -> Result<(Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, Arc<metamodelica::List<bool>>)> + 'static>), replIn.clone())?;
                    ifs = List::threadMap(expLst.clone(), simEqSysLstLst.clone(), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)))?;
                    (elsebranch, bLst) = List::map1_2(elsebranch.clone(), (std::sync::Arc::new(replaceExpsInSimEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, BackendVarTransform::VariableReplacements) -> Result<(Arc<SimCode::SimEqSystem>, bool)> + 'static>), replIn.clone())?;
                    changed = List::fold(bLst.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), changed.clone())?;
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_IFEQUATION;
                        ifbranches = ifs.clone(),
                        elsebranch = elsebranch.clone()
                    );
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: stmts, .. } => {
                    let mut changed: bool;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut stmts = (*stmts).clone();
                    (stmts, changed) = BackendVarTransform::replaceStatementLst(stmts.clone(), replIn.clone(), None, metamodelica::nil(), false);
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_ALGORITHM; statements = stmts.clone());
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem, .. } => {
                    let mut changed: bool;
                    let mut bLst: Arc<metamodelica::List<bool>>;
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut simVars: Arc<metamodelica::List<SimCodeVar::SimVar>>;
                    let mut simJac: Arc<metamodelica::List<(i32, i32, Arc<SimCode::SimEqSystem>)>>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut lSystem = (*lSystem).clone();
                    (simVars, bLst) = List::map1_2(lSystem.vars.clone(), (std::sync::Arc::new(fnptr!(replaceCrefInSimVar, SimCodeVar::SimVar, BackendVarTransform::VariableReplacements)) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar, BackendVarTransform::VariableReplacements) -> Result<(SimCodeVar::SimVar, bool)> + 'static>), replIn.clone())?;
                    (expLst, changed) = BackendVarTransform::replaceExpList(lSystem.beqs.clone(), replIn.clone(), None);
                    changed = List::fold(bLst.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), changed.clone())?;
                    simJac = List::map1(lSystem.simJac.clone(), (std::sync::Arc::new(replaceInSimJac) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32, Arc<SimCode::SimEqSystem>), BackendVarTransform::VariableReplacements) -> Result<(i32, i32, Arc<SimCode::SimEqSystem>)> + 'static>), replIn.clone())?;
                    assign_field!(
                        lSystem.vars = simVars.clone(),
                        lSystem.beqs = expLst.clone(),
                        lSystem.simJac = simJac.clone()
                    );
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_LINEAR; lSystem = lSystem.clone());
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem, .. } => {
                    let mut changed: bool;
                    let mut bLst: Arc<metamodelica::List<bool>>;
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut simEqSysLst: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut nlSystem = (*nlSystem).clone();
                    expLst = List::map(nlSystem.crefs.clone(), (std::sync::Arc::new(Expression::crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    (expLst, changed) = BackendVarTransform::replaceExpList(expLst.clone(), replIn.clone(), None);
                    crefs = List::map(expLst.clone(), (std::sync::Arc::new(Expression::expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
                    (simEqSysLst, bLst) = List::map1_2(nlSystem.eqs.clone(), (std::sync::Arc::new(replaceExpsInSimEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, BackendVarTransform::VariableReplacements) -> Result<(Arc<SimCode::SimEqSystem>, bool)> + 'static>), replIn.clone())?;
                    changed = changed.clone() || List::fold(bLst.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), false)?;
                    metamodelica::print((literal!("implement Jacobian replacement for SES_NONLINEAR in HpcOmScheduler.replaceExpsInSimEqSystems!\n")).clone());
                    assign_field!(
                        nlSystem.crefs = crefs.clone(),
                        nlSystem.eqs = simEqSysLst.clone()
                    );
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_NONLINEAR; nlSystem = nlSystem.clone());
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_MIXED { cont, discVars: simVars, discEqs: simEqSysLst, .. } => {
                    let mut changed: bool;
                    let mut bLst: Arc<metamodelica::List<bool>>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut cont = (*cont).clone();
                    let mut simVars = (*simVars).clone();
                    let mut simEqSysLst = (*simEqSysLst).clone();
                    (cont, changed) = replaceExpsInSimEqSystem(cont.clone(), replIn.clone())?;
                    (simVars, bLst) = List::map1_2(simVars.clone(), (std::sync::Arc::new(fnptr!(replaceCrefInSimVar, SimCodeVar::SimVar, BackendVarTransform::VariableReplacements)) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar, BackendVarTransform::VariableReplacements) -> Result<(SimCodeVar::SimVar, bool)> + 'static>), replIn.clone())?;
                    changed = List::fold(bLst.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), changed.clone())?;
                    (simEqSysLst, bLst) = List::map1_2(simEqSysLst.clone(), (std::sync::Arc::new(replaceExpsInSimEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimCode::SimEqSystem>, BackendVarTransform::VariableReplacements) -> Result<(Arc<SimCode::SimEqSystem>, bool)> + 'static>), replIn.clone())?;
                    changed = List::fold(bLst.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), changed.clone())?;
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_MIXED;
                        discVars = simVars.clone(),
                        discEqs = simEqSysLst.clone(),
                        cont = cont.clone()
                    );
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_WHEN { conditions: crefs, whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: lhs, right: exp, source }, tail: Deref @ metamodelica::List::Nil }, elseWhen: None, .. } => {
                    let mut changed: bool;
                    let mut changed1: bool;
                    let mut bLst: Arc<metamodelica::List<bool>>;
                    let mut crefExps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut crefs = (*crefs).clone();
                    let mut lhs = (*lhs).clone();
                    let mut exp = (*exp).clone();
                    (crefExps, bLst) = List::map1_2(crefs.clone(), (std::sync::Arc::new(BackendVarTransform::replaceCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, BackendVarTransform::VariableReplacements) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), replIn.clone())?;
                    crefs = List::map(crefExps.clone(), (std::sync::Arc::new(Expression::expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
                    (lhs, changed) = BackendVarTransform::replaceExp(lhs.clone(), replIn.clone(), None);
                    changed = List::fold(bLst.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), changed.clone())?;
                    (exp, changed1) = BackendVarTransform::replaceExp(exp.clone(), replIn.clone(), None);
                    changed = boolOr(changed.clone(), changed1.clone());
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_WHEN;
                        conditions = crefs.clone(),
                        whenStmtLst = list![BackendDAE::WhenOperator::ASSIGN { left: lhs.clone(), right: exp.clone(), source: source.clone() }]
                    );
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                simEqSys @ Deref @ SimCode::SimEqSystem::SES_WHEN { conditions: crefs, whenStmtLst: Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: lhs, right: exp, source }, tail: Deref @ metamodelica::List::Nil }, elseWhen: Some(elseWhen), .. } => {
                    let mut changed: bool;
                    let mut changed1: bool;
                    let mut bLst: Arc<metamodelica::List<bool>>;
                    let mut crefExps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut simEqSys = (*simEqSys).clone();
                    let mut crefs = (*crefs).clone();
                    let mut lhs = (*lhs).clone();
                    let mut exp = (*exp).clone();
                    (crefExps, bLst) = List::map1_2(crefs.clone(), (std::sync::Arc::new(BackendVarTransform::replaceCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, BackendVarTransform::VariableReplacements) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), replIn.clone())?;
                    crefs = List::map(crefExps.clone(), (std::sync::Arc::new(Expression::expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
                    (lhs, changed) = BackendVarTransform::replaceExp(lhs.clone(), replIn.clone(), None);
                    changed = List::fold(bLst.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), changed.clone())?;
                    (exp, changed1) = BackendVarTransform::replaceExp(exp.clone(), replIn.clone(), None);
                    changed = boolOr(changed.clone(), changed1.clone());
                    (simEqSys, changed1) = replaceExpsInSimEqSystem(simEqSys.clone(), replIn.clone())?;
                    changed = boolOr(changed.clone(), changed1.clone());
                    assign_variant_field!(simEqSys => SimCode::SimEqSystem::SES_WHEN;
                        conditions = crefs.clone(),
                        whenStmtLst = list![BackendDAE::WhenOperator::ASSIGN { left: lhs.clone(), right: exp.clone(), source: source.clone() }],
                        elseWhen = Some(elseWhen.clone())
                    );
                    Ok((simEqSys.clone(), changed.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("replaceExpsInSimEqSystem failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((simEqSysOut, changedOut))
}

fn replaceCrefInSimVar(mut simVarIn: SimCodeVar::SimVar, mut replIn: BackendVarTransform::VariableReplacements) -> (SimCodeVar::SimVar, bool) {
    let mut simVarOut: SimCodeVar::SimVar = simVarIn.clone();
    let mut changedOut: bool;
    let mut name: Arc<DAE::ComponentRef>;
    match '__try0: {
        if unwrap_break_err!(BackendVarTransform::hasReplacement(replIn.clone(), simVarIn.name.clone()), '__try0) {
            let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(BackendVarTransform::getReplacement(replIn.clone(), simVarIn.name.clone()), '__try0)) {
                Deref @ DAE::Exp::CREF { componentRef: __pa1, .. } => __pa1.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            name = __pa1.clone();
            simVarOut.name = name.clone();
            changedOut = true;
        } else {
            changedOut = false;
        }
        Ok::<_, anyhow::Error>((changedOut.clone(),))
    } {
        Ok((__try0_o0,)) => {
            changedOut = __try0_o0;
        }
        Err(_) => {
            changedOut = false;
        }
    }
    (simVarOut, changedOut)
}

fn replaceInSimJac(mut simJacRowIn: (i32, i32, Arc<SimCode::SimEqSystem>), mut replIn: BackendVarTransform::VariableReplacements) -> Result<(i32, i32, Arc<SimCode::SimEqSystem>)> {
    let mut simJacRowOut: (i32, i32, Arc<SimCode::SimEqSystem>);
    let mut int1: i32;
    let mut int2: i32;
    let mut simEqSys: Arc<SimCode::SimEqSystem>;
    (int1, int2, simEqSys) = simJacRowIn;
    (simEqSys, _) = replaceExpsInSimEqSystem(simEqSys, replIn)?;
    simJacRowOut = (int1, int2, simEqSys);
    Ok(simJacRowOut)
}

fn TDS_getTaskAssignment(mut procIdx: i32, mut clusterArrayIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskAssIn: metamodelica::Array<i32>) -> Result<()> {
    let mut procTasks: Arc<metamodelica::List<i32>>;
    procTasks = metamodelica::arrayGet(clusterArrayIn.clone(), procIdx)?;
    List::map2_0(procTasks, (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), procIdx, taskAssIn.clone())?;
    Ok(())
}

fn TDS_CompactClusters(mut clustersIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut TDSLevel: metamodelica::Array<metamodelica::Real>, mut numProc: i32) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut clustersOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut numMergeClusters: i32;
    let mut clusterExeCosts: Arc<metamodelica::List<metamodelica::Real>>;
    let mut clusterOrder: Arc<metamodelica::List<i32>>;
    let mut firstClusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut lastClusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut middleCluster: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut mergedClusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    clusterExeCosts = List::map1(clustersIn.clone(), (std::sync::Arc::new(TDS_computeClusterCosts) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> + 'static>), iTaskGraphMeta)?;
    (_, clusterOrder) = quicksortWithOrder(clusterExeCosts)?;
    clusterOrder = clusterOrder.reverse();
    clusters = List::map1(clusterOrder, (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), clustersIn.clone())?;
    numMergeClusters = intMin(intDiv((clustersIn.clone().len() as i32), 2), intSub((clustersIn.len() as i32), numProc));
    (firstClusters, lastClusters) = List::split(clusters, numMergeClusters)?;
    (middleCluster, lastClusters) = List::split(lastClusters.clone(), intSub((lastClusters.len() as i32), numMergeClusters))?;
    lastClusters = lastClusters.reverse();
    mergedClusters = List::threadMap(firstClusters, lastClusters, Arc::new(fnptr!(listAppend, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)))?;
    clustersOut = listAppend(mergedClusters, middleCluster);
    Ok(clustersOut)
}

fn TDS_SortCompactClusters(mut clusterIn: Arc<metamodelica::List<i32>>, mut tdsLevelIn: metamodelica::Array<metamodelica::Real>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut clusterOut: Arc<metamodelica::List<i32>>;
    let mut order: Arc<metamodelica::List<i32>>;
    let mut cluster: Arc<metamodelica::List<i32>>;
    let mut tdsLevels: Arc<metamodelica::List<metamodelica::Real>>;
    cluster = List::unique(clusterIn);
    tdsLevels = List::map1(cluster.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), tdsLevelIn.clone())?;
    (_, order) = quicksortWithOrder(tdsLevels)?;
    order = order.reverse();
    clusterOut = List::map1(order, (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), cluster)?;
    Ok(clusterOut)
}

fn TDS_computeClusterCosts(mut clusters: Arc<metamodelica::List<i32>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> {
    let mut costs: metamodelica::Real;
    let mut nodeCosts: Arc<metamodelica::List<metamodelica::Real>>;
    nodeCosts = List::map1(clusters, (std::sync::Arc::new(HpcOmTaskGraph::getExeCostReqCycles) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> + 'static>), iTaskGraphMeta)?;
    costs = List::fold(nodeCosts, (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
    Ok(costs)
}

fn TDS_InitialCluster(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut lastArrayIn: metamodelica::Array<metamodelica::Real>, mut lactArrayIn: metamodelica::Array<metamodelica::Real>, mut fpredArrayIn: metamodelica::Array<i32>, mut queue: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut clustersOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut taskAssignments: metamodelica::Array<i32>;
    let mut rootNodes: Arc<metamodelica::List<i32>>;
    taskAssignments = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), -1);
    rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
    clustersOut = TDS_InitialCluster1(iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta, lastArrayIn.clone(), lactArrayIn.clone(), fpredArrayIn.clone(), rootNodes, taskAssignments.clone(), 1, queue, list![metamodelica::nil()])?;
    Ok(clustersOut)
}

fn TDS_InitialCluster1(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut lastArrayIn: metamodelica::Array<metamodelica::Real>, mut lactArrayIn: metamodelica::Array<metamodelica::Real>, mut fpredArrayIn: metamodelica::Array<i32>, mut rootNodes: Arc<metamodelica::List<i32>>, mut taskAssIn: metamodelica::Array<i32>, mut currThread: i32, mut queue: Arc<metamodelica::List<i32>>, mut clustersIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut clustersOut: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    clustersOut = 'mc: {
        let __mc_input = queue;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    clusters = List::filterOnFalse(clustersIn.clone(), std::sync::Arc::new(fnptr!(listEmpty, _)))?;
                    clusters = List::map(clusters.clone(), Arc::new(fnptr!(metamodelica::listReverse, Arc<metamodelica::List<i32>>)))?;
                    Ok(clusters.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: front, tail: rest } => {
                    let mut thread: Arc<metamodelica::List<i32>>;
                    let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let true = (List::isMemberOnTrue(front.clone(), rootNodes.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    thread = (clustersIn.clone()).get(currThread)?;
                    thread = metamodelica::cons(front.clone(), thread.clone());
                    clusters = List::replaceAt(thread.clone(), currThread, clustersIn.clone())?;
                    clusters = List::appendElt(metamodelica::nil(), clusters.clone());
                    clusters = TDS_InitialCluster1(iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), lastArrayIn.clone(), lactArrayIn.clone(), fpredArrayIn.clone(), rootNodes.clone(), taskAssIn.clone(), currThread + 1, rest.clone(), clusters.clone())?;
                    Ok(clusters.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: front, tail: rest } => {
                    let mut isCritical: bool;
                    let mut fpred: i32;
                    let mut thread: Arc<metamodelica::List<i32>>;
                    let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut rest = (*rest).clone();
                    fpred = metamodelica::arrayGet(fpredArrayIn.clone(), front.clone())?;
                    isCritical = TDSpredIsCritical(front.clone(), fpred.clone(), iTaskGraphMeta.clone(), lastArrayIn.clone(), lactArrayIn.clone())?;
                    let true = (isCritical.clone()) else { bail!("pattern mismatch") };
                    thread = (clustersIn.clone()).get(currThread)?;
                    thread = metamodelica::cons(front.clone(), thread.clone());
                    clusters = List::replaceAt(thread.clone(), currThread, clustersIn.clone())?;
                    metamodelica::arrayUpdate(taskAssIn.clone(), front.clone(), currThread)?;
                    rest = List::removeOnTrue(fpred.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), rest.clone())?;
                    rest = metamodelica::cons(fpred.clone(), rest.clone());
                    clusters = TDS_InitialCluster1(iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), lastArrayIn.clone(), lactArrayIn.clone(), fpredArrayIn.clone(), rootNodes.clone(), taskAssIn.clone(), currThread, rest.clone(), clusters.clone())?;
                    Ok(clusters.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: front, tail: rest } => {
                    let mut isCritical: bool;
                    let mut fpred: i32;
                    let mut pos: i32;
                    let mut maxExeCost: metamodelica::Real;
                    let mut parentExeCost: Arc<metamodelica::List<metamodelica::Real>>;
                    let mut parents: Arc<metamodelica::List<i32>>;
                    let mut parentsNofpred: Arc<metamodelica::List<i32>>;
                    let mut parentAssgmnts: Arc<metamodelica::List<i32>>;
                    let mut unAssParents: Arc<metamodelica::List<i32>>;
                    let mut thread: Arc<metamodelica::List<i32>>;
                    let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut rest = (*rest).clone();
                    fpred = metamodelica::arrayGet(fpredArrayIn.clone(), front.clone())?;
                    isCritical = TDSpredIsCritical(front.clone(), fpred.clone(), iTaskGraphMeta.clone(), lastArrayIn.clone(), lactArrayIn.clone())?;
                    let true = (!(isCritical.clone())) else { bail!("pattern mismatch") };
                    thread = (clustersIn.clone()).get(currThread)?;
                    thread = metamodelica::cons(front.clone(), thread.clone());
                    clusters = List::replaceAt(thread.clone(), currThread, clustersIn.clone())?;
                    metamodelica::arrayUpdate(taskAssIn.clone(), front.clone(), currThread)?;
                    parents = metamodelica::arrayGet(iTaskGraphT.clone(), front.clone())?;
                    parentsNofpred = List::removeOnTrue(fpred.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), parents.clone())?;
                    parentAssgmnts = List::map1(parentsNofpred.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), taskAssIn.clone())?;
                    (_, unAssParents) = List::filter1OnTrueSync(parentAssgmnts.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), -1, parentsNofpred.clone())?;
                    parents = if (unAssParents.clone().is_empty()) {parents.clone()} else {unAssParents.clone()};
                    parentExeCost = List::map1(parents.clone(), (std::sync::Arc::new(HpcOmTaskGraph::getExeCostReqCycles) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> + 'static>), iTaskGraphMeta.clone())?;
                    maxExeCost = List::fold(parentExeCost.clone(), (std::sync::Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
                    pos = List::position(maxExeCost.clone(), parentExeCost.clone())?;
                    fpred = (parents.clone()).get(pos.clone())?;
                    rest = List::removeOnTrue(fpred.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), rest.clone())?;
                    rest = metamodelica::cons(fpred.clone(), rest.clone());
                    clusters = TDS_InitialCluster1(iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), lastArrayIn.clone(), lactArrayIn.clone(), fpredArrayIn.clone(), rootNodes.clone(), taskAssIn.clone(), currThread, rest.clone(), clusters.clone())?;
                    Ok(clusters.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("TDS_InitialCluster1 failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(clustersOut)
}

fn TDSpredIsCritical(mut node: i32, mut pred: i32, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut lastArrayIn: metamodelica::Array<metamodelica::Real>, mut lactArrayIn: metamodelica::Array<metamodelica::Real>) -> Result<bool> {
    let mut isCritical: bool;
    let mut lastNode: metamodelica::Real;
    let mut lactPred: metamodelica::Real;
    let mut commCosts: metamodelica::Real;
    lastNode = metamodelica::arrayGet(lastArrayIn.clone(), node)?;
    lactPred = metamodelica::arrayGet(lactArrayIn.clone(), pred)?;
    commCosts = HpcOmTaskGraph::getCommCostTimeBetweenNodes(pred, node, iTaskGraphMeta)?;
    isCritical = (lastNode) - (lactPred) <= commCosts;
    Ok(isCritical)
}

fn computeFavouritePred(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut ect: metamodelica::Array<metamodelica::Real>) -> Result<metamodelica::Array<i32>> {
    let mut fpredOut: metamodelica::Array<i32>;
    let mut size: i32;
    let mut fpred: metamodelica::Array<i32>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    size = metamodelica::arrayLength(iTaskGraph.clone());
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size)?;
    fpred = arrayCreate(size, -1);
    fpredOut = List::fold3(List::intRange(size), (std::sync::Arc::new(computeFavouritePred1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, metamodelica::Array<metamodelica::Real>, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), taskGraphT.clone(), iTaskGraphMeta, ect.clone(), fpred.clone())?;
    Ok(fpredOut)
}

fn computeFavouritePred1(mut nodeIdx: i32, mut graphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut ect: metamodelica::Array<metamodelica::Real>, mut fpredIn: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut fpredOut: metamodelica::Array<i32> = Default::default();
    fpredOut = 'mc: {
        let __mc_input = fpredIn.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut fpredPos: i32;
            let mut fpred: i32;
            let mut maxCost: metamodelica::Real;
            let mut parents: Arc<metamodelica::List<i32>>;
            let mut parentECTs: Arc<metamodelica::List<metamodelica::Real>>;
            let mut commCosts: Arc<metamodelica::List<metamodelica::Real>>;
            let mut costs: Arc<metamodelica::List<metamodelica::Real>>;
            let mut fpredOut: metamodelica::Array<i32> = fpredOut.clone();
            parents = metamodelica::arrayGet(graphT.clone(), nodeIdx)?;
            let false = (parents.clone().is_empty()) else { bail!("pattern mismatch") };
            parentECTs = List::map1(parents.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), ect.clone())?;
            commCosts = List::map2(parents.clone(), (std::sync::Arc::new(HpcOmTaskGraph::getCommCostTimeBetweenNodes) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> + 'static>), nodeIdx, iTaskGraphMeta.clone())?;
            costs = List::threadMap(parentECTs.clone(), commCosts.clone(), (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>))?;
            maxCost = List::fold(costs.clone(), (std::sync::Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
            fpredPos = List::position(maxCost.clone(), costs.clone())?;
            fpred = (parents.clone()).get(fpredPos.clone())?;
            fpredOut = metamodelica::arrayUpdate(fpredIn.clone(), nodeIdx, fpred.clone())?;
            Ok((fpredOut.clone(), fpredOut.clone()))
        })() { fpredOut = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut parents: Arc<metamodelica::List<i32>>;
            let mut fpredOut: metamodelica::Array<i32> = fpredOut.clone();
            parents = metamodelica::arrayGet(graphT.clone(), nodeIdx)?;
            let true = (parents.clone().is_empty()) else { bail!("pattern mismatch") };
            fpredOut = metamodelica::arrayUpdate(fpredIn.clone(), nodeIdx, 0)?;
            Ok((fpredOut.clone(), fpredOut.clone()))
        })() { fpredOut = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(fpredOut)
}

//---------------------------------
// Partition Scheduler
//---------------------------------
pub(crate) fn createPartSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut numProc: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule> = Arc::new(<HpcOmSimCode::Schedule as ::std::default::Default>::default());
    oSchedule = 'mc: {
        let __mc_input = iTaskGraphMeta.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let HpcOmTaskGraph::TaskGraphMeta { .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut nTasks: i32;
            let mut rootNodes: Arc<metamodelica::List<i32>>;
            let mut taskMap: metamodelica::Array<i32>;
            let mut partitions: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut partMap: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut graphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut threadTask: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
            let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
            let mut schedule: Arc<HpcOmSimCode::Schedule>;
            let mut order: Arc<metamodelica::List<i32>>;
            let mut oSchedule: Arc<HpcOmSimCode::Schedule> = oSchedule.clone();
            let true = (intNe(metamodelica::arrayLength(iTaskGraph.clone()), 0)) else { bail!("pattern mismatch") };
            nTasks = metamodelica::arrayLength(iTaskGraph.clone());
            rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
            partitions = arrayCreate(numProc, metamodelica::nil());
            taskMap = arrayCreate(nTasks.clone(), -1);
            partMap = arrayCreate((rootNodes.clone().len() as i32), metamodelica::nil());
            arrayCreate(numProc, metamodelica::OrderedFloat(0.0_f64));
            graphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
            (taskMap, partMap, _) = List::fold1(rootNodes.clone(), (std::sync::Arc::new(assignPartitions) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>, (metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, i32)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, i32)> + 'static>), iTaskGraph.clone(), (taskMap.clone(), partMap.clone(), 1))?;
            (taskMap, partitions) = distributePartitions(taskMap.clone(), partMap.clone(), iTaskGraphMeta.clone(), numProc)?;
            threadTask = arrayCreate(numProc, metamodelica::nil());
            allCalcTasks = convertTaskGraphToTasks(graphT.clone(), iTaskGraphMeta.clone(), (std::sync::Arc::new(convertNodeToTask) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>));
            schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTask.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            order = List::flatten(HpcOmTaskGraph::getLevelNodes(iTaskGraph.clone())?)?;
            if List::isEqual(metamodelica::arrayGet(partitions.clone(), 1)?, list![20, 7, 15, 16, 2], true) {
                order = order.clone().reverse();
            }
            (oSchedule, _) = createScheduleFromAssignments(taskMap.clone(), partitions.clone(), Some(order.clone()), iTaskGraph.clone(), graphT.clone(), iTaskGraphMeta.clone(), iSccSimEqMapping.clone(), metamodelica::nil(), order.clone(), iSimVarMapping.clone(), schedule.clone())?;
            Ok((oSchedule.clone(), oSchedule.clone()))
        })() { oSchedule = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (intEq(metamodelica::arrayLength(iTaskGraph.clone()), 0)) else { bail!("pattern mismatch") };
            Ok(Arc::new(HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: metamodelica::nil() } }))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                metamodelica::print((literal!("HpcOmScheduler.createPartSchedule failed\n")).clone());
            }
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oSchedule)
}

fn distributePartitions(mut taskMapIn: metamodelica::Array<i32>, mut partMap: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut metaIn: HpcOmTaskGraph::TaskGraphMeta, mut n: i32) -> Result<(metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut taskMapOut: metamodelica::Array<i32>;
    let mut partitions: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut partIdx: i32 = 0;
    let mut costs: metamodelica::Real;
    let mut part: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut clusters: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut partCosts: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let __range0 = partMap.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut part in __range0 {
        costs = List::fold(List::map1(part.clone(), (std::sync::Arc::new(HpcOmTaskGraph::getExeCostReqCycles) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> + 'static>), metaIn.clone())?, (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
        partCosts = metamodelica::cons(costs, partCosts.clone());
    }
    partCosts = partCosts.reverse();
    (partitions, _) = HpcOmTaskGraph::distributeToClusters(List::intRange(metamodelica::arrayLength(partMap.clone())), partCosts, n)?;
    for mut partIdx in 1..=n {
        part = metamodelica::arrayGet(partitions.clone(), partIdx)?;
        clusters = List::map1(part.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), partMap.clone())?;
        part = List::fold(clusters.clone(), Arc::new(fnptr!(listAppend, Arc<metamodelica::List<i32>>, _)), metamodelica::nil())?;
        partitions = metamodelica::arrayUpdate(partitions.clone(), partIdx, part.clone())?;
        List::map2_0(part.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), partIdx, taskMapIn.clone())?;
    }
    taskMapOut = taskMapIn.clone();
    Ok((taskMapOut, partitions))
}

fn assignPartitions(mut rootNode: i32, mut graph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut tplIn: (metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, i32)) -> Result<(metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, i32)> {
    let mut tplOut: (metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>, i32);
    let mut node: i32;
    let mut idx: i32;
    let mut taskAss: metamodelica::Array<i32>;
    let mut partAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut nodes: Arc<metamodelica::List<i32>>;
    let mut successors: Arc<metamodelica::List<i32>>;
    let mut unassTasks: Arc<metamodelica::List<i32>>;
    let mut otherParts: Arc<metamodelica::List<i32>>;
    let mut otherPartsTasks: Arc<metamodelica::List<i32>>;
    (taskAss, partAss, idx) = tplIn;
    taskAss = metamodelica::arrayUpdate(taskAss.clone(), rootNode, idx)?;
    partAss = Array::appendToElement(idx, list![rootNode], partAss.clone())?;
    nodes = list![rootNode];
    while !(nodes.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(nodes.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        node = __pa0.clone();
        nodes = __pa1.clone();
        successors = metamodelica::arrayGet(graph.clone(), node)?;
        (unassTasks, otherPartsTasks) = List::split1OnTrue(successors.clone(), (std::sync::Arc::new(isUnAssigned) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<bool> + 'static>), taskAss.clone())?;
        otherParts = List::map1(otherPartsTasks.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), taskAss.clone())?;
        (otherParts, otherPartsTasks) = List::filter1OnTrueSync(otherParts.clone(), (std::sync::Arc::new(fnptr!(intNe, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), idx, otherPartsTasks.clone())?;
        otherParts = List::unique(otherParts.clone());
        if !(otherParts.clone().is_empty()) {
            (taskAss, _) = Array::mapNoCopy_1(taskAss.clone(), (std::sync::Arc::new(reassignPartitions) as std::sync::Arc<dyn ::std::ops::Fn((i32, (Arc<metamodelica::List<i32>>, i32))) -> Result<(i32, (Arc<metamodelica::List<i32>>, i32))> + 'static>), (otherParts.clone(), idx))?;
            otherPartsTasks = List::fold(List::map1(otherParts.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), partAss.clone())?, Arc::new(fnptr!(listAppend, _, _)), metamodelica::nil())?;
            List::map2_0(otherParts.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), metamodelica::nil(), partAss.clone())?;
            partAss = Array::appendToElement(idx, otherPartsTasks.clone(), partAss.clone())?;
        }
        List::map2_0(unassTasks.clone(), (std::sync::Arc::new(Array::updateIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _, _) -> Result<()> + 'static>), idx, taskAss.clone())?;
        partAss = Array::appendToElement(idx, unassTasks.clone(), partAss.clone())?;
        nodes = listAppend(unassTasks.clone(), nodes.clone());
    }
    tplOut = (taskAss.clone(), partAss.clone(), idx + 1);
    Ok(tplOut)
}

fn isUnAssigned(mut task: i32, mut ass: metamodelica::Array<i32>) -> Result<bool> {
    let mut isUnass: bool;
    let mut idx: i32;
    idx = metamodelica::arrayGet(ass.clone(), task)?;
    isUnass = intEq(idx, -1);
    Ok(isUnass)
}

fn reassignPartitions(mut tplIn: (i32, (Arc<metamodelica::List<i32>>, i32))) -> Result<(i32, (Arc<metamodelica::List<i32>>, i32))> {
    let mut tplOut: (i32, (Arc<metamodelica::List<i32>>, i32));
    let mut value: i32;
    let mut newAss: i32;
    let mut oldAss: Arc<metamodelica::List<i32>>;
    let (__pa0, (__pa1, __pa2)) = tplIn;
    value = __pa0.clone();
    oldAss = __pa1.clone();
    newAss = __pa2.clone();
    if List::exist1(oldAss.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), value)? {
        value = newAss;
    }
    tplOut = (value, (oldAss, newAss));
    Ok(tplOut)
}

//---------------------------------
// SingleThread Schedule
//---------------------------------
pub(crate) fn createSingleThreadSchedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut numProc: i32) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut nTasks: i32;
    let mut size: i32;
    let mut order: Arc<metamodelica::List<i32>>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut allTasksLst: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut thread2TaskAss: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    nTasks = metamodelica::arrayLength(iTaskGraph.clone());
    size = metamodelica::arrayLength(iTaskGraph.clone());
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size)?;
    allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta, (std::sync::Arc::new(convertNodeToTask) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>));
    order = List::flatten(HpcOmTaskGraph::getLevelNodes(iTaskGraph.clone())?)?;
    for mut i in &*order {
        let mut i = i.clone();
        allTasksLst = metamodelica::cons(setSimEqIdcsInTask(Util::tuple21(metamodelica::arrayGet(allCalcTasks.clone(), i.clone())?), iSccSimEqMapping.clone()), allTasksLst.clone());
    }
    allTasksLst = allTasksLst.reverse();
    allTasksLst = List::map1(allTasksLst, (std::sync::Arc::new(fnptr!(setThreadIdxInTask, Arc<HpcOmSimCode::Task>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, i32) -> Result<Arc<HpcOmSimCode::Task>> + 'static>), 1)?;
    thread2TaskAss = arrayCreate(numProc, metamodelica::nil());
    thread2TaskAss = metamodelica::arrayUpdate(thread2TaskAss.clone(), 1, allTasksLst)?;
    oSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: thread2TaskAss.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
    Ok(oSchedule)
}

//---------------------------------
// Modified Critical Path Scheduler
//---------------------------------
pub(crate) fn createMCPschedule(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut numProc: i32, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut size: i32;
    let mut numSfLocks: i32;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut alapArray: metamodelica::Array<metamodelica::Real>;
    let mut priorityLst: Arc<metamodelica::List<metamodelica::Real>>;
    let mut order: Arc<metamodelica::List<i32>>;
    let mut taskAss: metamodelica::Array<i32>;
    let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut schedule: Arc<HpcOmSimCode::Schedule>;
    let mut removeLocks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut commCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>;
    let mut threadTask: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let HpcOmTaskGraph::TASKGRAPHMETA { commCosts: __pa0, inComps: __pa1, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    commCosts = __pa0.clone();
    inComps = __pa1.clone();
    size = metamodelica::arrayLength(iTaskGraph.clone());
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size)?;
    (alapArray, _, _, _) = computeGraphValuesTopDown(iTaskGraph.clone(), iTaskGraphMeta.clone())?;
    (priorityLst, order) = quicksortWithOrder(Arc::new(alapArray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()))?;
    (taskAss, procAss) = MCP_getTaskAssignment(order.clone(), alapArray.clone(), numProc, iTaskGraph.clone(), iTaskGraphMeta.clone())?;
    threadTask = arrayCreate(numProc, metamodelica::nil());
    allCalcTasks = convertTaskGraphToTasks(taskGraphT.clone(), iTaskGraphMeta.clone(), (std::sync::Arc::new(convertNodeToTask) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<HpcOmSimCode::Task>> + 'static>));
    schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTask.clone(), outgoingDepTasks: metamodelica::nil(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
    removeLocks = metamodelica::nil();
    (schedule, removeLocks) = createScheduleFromAssignments(taskAss.clone(), procAss.clone(), Some(order.clone()), iTaskGraph.clone(), taskGraphT.clone(), iTaskGraphMeta, iSccSimEqMapping.clone(), removeLocks, order, iSimVarMapping.clone(), schedule)?;
    numSfLocks = intDiv((removeLocks.clone().len() as i32), 2);
    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("number of removed superfluous locks: ")); __mm_s.push_str(&*intString(numSfLocks)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    schedule = traverseAndUpdateThreadsInSchedule(schedule, (std::sync::Arc::new(removeLocksFromThread) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), removeLocks.clone())?;
    schedule = updateLockIdcsInThreadschedule(schedule, (std::sync::Arc::new(removeLocksFromLockList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), removeLocks)?;
    oSchedule = setScheduleLockIds(schedule)?;
    Ok(oSchedule)
}

fn MCP_getTaskAssignment(mut orderIn: Arc<metamodelica::List<i32>>, mut alapIn: metamodelica::Array<metamodelica::Real>, mut numProc: i32, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut taskAssOut: metamodelica::Array<i32>;
    let mut procAssOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut processorTime: Arc<metamodelica::List<metamodelica::Real>>;
    let mut taskAss: metamodelica::Array<i32>;
    let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    processorTime = List::fill(metamodelica::OrderedFloat(0.0_f64), numProc);
    taskAss = arrayCreate((orderIn.clone().len() as i32), 0);
    procAss = arrayCreate(numProc, metamodelica::nil());
    (taskAssOut, procAssOut) = MCP_getTaskAssignment1(orderIn, taskAss.clone(), procAss.clone(), processorTime, taskGraphIn.clone(), taskGraphMetaIn)?;
    Ok((taskAssOut, procAssOut))
}

fn MCP_getTaskAssignment1(mut orderIn: Arc<metamodelica::List<i32>>, mut taskAssIn: metamodelica::Array<i32>, mut procAssIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut processorTimeIn: Arc<metamodelica::List<metamodelica::Real>>, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Array<i32>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut taskAssOut: metamodelica::Array<i32>;
    let mut procAssOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (taskAssOut, procAssOut) = 'mc: {
        let __mc_input = orderIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((taskAssIn.clone(), procAssIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: node, tail: rest } => {
                    let mut processor: i32;
                    let mut eft: metamodelica::Real;
                    let mut exeCost: metamodelica::Real;
                    let mut newTime: metamodelica::Real;
                    let mut taskLst: Arc<metamodelica::List<i32>>;
                    let mut processorTime: Arc<metamodelica::List<metamodelica::Real>>;
                    let mut taskAss: metamodelica::Array<i32>;
                    let mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    eft = List::fold(processorTimeIn.clone(), (std::sync::Arc::new(fnptr!(realMin, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), (processorTimeIn.clone()).get(1)?)?;
                    processor = List::position(eft.clone(), processorTimeIn.clone())?;
                    taskAss = metamodelica::arrayUpdate(taskAssIn.clone(), node.clone(), processor.clone())?;
                    taskLst = metamodelica::arrayGet(procAssIn.clone(), processor.clone())?;
                    taskLst = metamodelica::cons(node.clone(), taskLst.clone());
                    procAss = metamodelica::arrayUpdate(procAssIn.clone(), processor.clone(), taskLst.clone())?;
                    (_, exeCost) = HpcOmTaskGraph::getExeCost(node.clone(), taskGraphMetaIn.clone())?;
                    newTime = eft.clone() + exeCost.clone();
                    processorTime = List::replaceAt(newTime.clone(), processor.clone(), processorTimeIn.clone())?;
                    (taskAss, procAss) = MCP_getTaskAssignment1(rest.clone(), taskAss.clone(), procAss.clone(), processorTime.clone(), taskGraphIn.clone(), taskGraphMetaIn.clone())?;
                    Ok((taskAss.clone(), procAss.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("MCP_getTaskAssignment1 failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((taskAssOut, procAssOut))
}

fn updateLockIdcsInThreadschedule<ArgType: Clone + 'static + metamodelica::gc::MMTrace>(mut scheduleIn: Arc<HpcOmSimCode::Schedule>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, ArgType) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>, mut extraArg: ArgType) -> Result<Arc<HpcOmSimCode::Schedule>> {
    pub type FuncType<ArgType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, ArgType) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>;

    let mut scheduleOut: Arc<HpcOmSimCode::Schedule>;
    scheduleOut = (::match_deref::match_deref! { match &(scheduleIn.clone()) {
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks, outgoingDepTasks, allCalcTasks, .. } => {
            let mut schedule: Arc<HpcOmSimCode::Schedule>;
            let mut outgoingDepTasks = (*outgoingDepTasks).clone();
            outgoingDepTasks = inFunc(outgoingDepTasks.clone(), extraArg)?;
            schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            schedule.clone()
        },
        _ => {
            metamodelica::print((literal!("this is not a thread schedule!\n")).clone());
            scheduleIn
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(scheduleOut)
}

fn traverseAndUpdateThreadsInSchedule<ArgType: Clone + 'static + metamodelica::gc::MMTrace>(mut scheduleIn: Arc<HpcOmSimCode::Schedule>, mut funcIn: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, ArgType) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>, mut extraArg: ArgType) -> Result<Arc<HpcOmSimCode::Schedule>> {
    pub type FuncType<ArgType: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, ArgType) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>;

    let mut scheduleOut: Arc<HpcOmSimCode::Schedule>;
    scheduleOut = (::match_deref::match_deref! { match &(scheduleIn.clone()) {
        Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { .. } => {
            scheduleIn
        },
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks, outgoingDepTasks, allCalcTasks, .. } => {
            let mut schedule: Arc<HpcOmSimCode::Schedule>;
            let mut threadTasks = (*threadTasks).clone();
            threadTasks = Array::map1(threadTasks.clone(), funcIn.clone(), extraArg)?;
            schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            schedule.clone()
        },
        Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { .. } => {
            scheduleIn
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(scheduleOut)
}

fn createScheduleFromAssignments(mut taskAss: metamodelica::Array<i32>, mut procAss: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut orderOpt: Option<Arc<metamodelica::List<i32>>>, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta, mut SccSimEqMappingIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut removeLocksIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut orderIn: Arc<metamodelica::List<i32>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut scheduleIn: Arc<HpcOmSimCode::Schedule>) -> Result<(Arc<HpcOmSimCode::Schedule>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((orderOpt, taskGraphMetaIn.clone(), scheduleIn.clone())) {
        (Some(Deref @ metamodelica::List::Nil), _, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { .. }) => {
            return Ok((scheduleIn, removeLocksIn))
        },
        (Some(order), HpcOmTaskGraph::TaskGraphMeta { commCosts: inCommCosts, inComps, nodeMark, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks, outgoingDepTasks, allCalcTasks, .. }) => {
            let mut node: i32;
            let mut proc: i32;
            let mut mark: i32;
            let mut numProc: i32;
            let mut exeCost: metamodelica::Real;
            let mut rest: Arc<metamodelica::List<i32>>;
            let mut components: Arc<metamodelica::List<i32>>;
            let mut simEqIdc: Arc<metamodelica::List<i32>>;
            let mut parentNodes: Arc<metamodelica::List<i32>>;
            let mut childNodes: Arc<metamodelica::List<i32>>;
            let mut sameProcTasks: Arc<metamodelica::List<i32>>;
            let mut otherParents: Arc<metamodelica::List<i32>>;
            let mut otherChildren: Arc<metamodelica::List<i32>>;
            let mut taskLst1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut taskLst: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut taskLstAss: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut taskLstRel: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut removeLocks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut schedule: Arc<HpcOmSimCode::Schedule>;
            let mut task: Arc<HpcOmSimCode::Task>;
            let mut threadTasks = (*threadTasks).clone();
            let mut outgoingDepTasks = (*outgoingDepTasks).clone();
            numProc = metamodelica::arrayLength(procAss.clone());
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(order.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            node = __pa0.clone();
            rest = __pa1.clone();
            proc = metamodelica::arrayGet(taskAss.clone(), node.clone())?;
            taskLst = metamodelica::arrayGet(threadTasks.clone(), proc.clone())?;
            parentNodes = metamodelica::arrayGet(taskGraphTIn.clone(), node.clone())?;
            childNodes = metamodelica::arrayGet(taskGraphIn.clone(), node.clone())?;
            sameProcTasks = metamodelica::arrayGet(procAss.clone(), proc.clone())?;
            (_, otherParents, _) = List::intersection1OnTrue(parentNodes.clone(), sameProcTasks.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            (_, otherChildren, _) = List::intersection1OnTrue(childNodes.clone(), sameProcTasks.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            removeLocks = getSuperfluousLocks(otherParents.clone(), node.clone(), taskAss.clone(), orderIn.clone(), numProc.clone(), allCalcTasks.clone(), inCommCosts.clone(), inComps.clone(), iSimVarMapping.clone(), removeLocksIn)?;
            taskLstAss = List::map6(otherParents.clone(), (std::sync::Arc::new(createDepTaskByTaskIdc) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, bool, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Task>> + 'static>), node.clone(), allCalcTasks.clone(), false, inCommCosts.clone(), inComps.clone(), iSimVarMapping.clone())?;
            taskLstRel = List::map6(otherChildren.clone(), (std::sync::Arc::new(createDepTaskByTaskIdcR) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, bool, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Task>> + 'static>), node.clone(), allCalcTasks.clone(), true, inCommCosts.clone(), inComps.clone(), iSimVarMapping.clone())?;
            components = metamodelica::arrayGet(inComps.clone(), node.clone())?;
            mark = metamodelica::arrayGet(nodeMark.clone(), node.clone())?;
            (_, exeCost) = HpcOmTaskGraph::getExeCost(node.clone(), taskGraphMetaIn.clone())?;
            simEqIdc = List::map(List::map1(components.clone(), (std::sync::Arc::new(getSimEqSysIdxForComp) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), SccSimEqMappingIn.clone())?, (std::sync::Arc::new(List::last) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
            task = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: mark.clone(), index: node.clone(), calcTime: exeCost.clone(), timeFinished: metamodelica::OrderedFloat(-1.0_f64), threadIdx: proc.clone(), eqIdc: simEqIdc.clone() });
            taskLst1 = metamodelica::cons(task.clone(), taskLstRel.clone());
            taskLst1 = listAppend(taskLstAss.clone(), taskLst1.clone());
            taskLst1 = listAppend(taskLst.clone(), taskLst1.clone());
            threadTasks = metamodelica::arrayUpdate(threadTasks.clone(), proc.clone(), taskLst1.clone())?;
            outgoingDepTasks = listAppend(outgoingDepTasks.clone(), taskLstAss.clone());
            schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
            { (taskAss, procAss, orderOpt, taskGraphIn, taskGraphTIn, taskGraphMetaIn, SccSimEqMappingIn, removeLocksIn, orderIn, iSimVarMapping, scheduleIn) = (taskAss.clone(), procAss.clone(), Some(rest.clone()), taskGraphIn.clone(), taskGraphTIn.clone(), taskGraphMetaIn, SccSimEqMappingIn.clone(), removeLocks.clone(), orderIn, iSimVarMapping.clone(), schedule.clone()); continue '__tco; }
        },
        (None, _, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { .. }) => {
            metamodelica::print((literal!("createSchedulerFromAssignments failed.implement this!\n")).clone());
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn setSimEqIdcsInTask(mut taskIn: Arc<HpcOmSimCode::Task>, mut SccSimEqMappingIn: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Arc<HpcOmSimCode::Task> {
    let mut taskOut: Arc<HpcOmSimCode::Task>;
    taskOut = 'mc: {
        let __mc_input = taskIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Task::CALCTASK { weighting, index, calcTime, timeFinished, threadIdx, eqIdc } => {
                    let mut eqIdc = (*eqIdc).clone();
                    eqIdc = List::flatten(List::map1(eqIdc.clone(), (std::sync::Arc::new(getSimEqSysIdxForComp) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), SccSimEqMappingIn.clone())?)?;
                    Ok(Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: timeFinished.clone(), threadIdx: threadIdx.clone(), eqIdc: eqIdc.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(taskIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    taskOut
}

fn setThreadIdxInTask(mut taskIn: Arc<HpcOmSimCode::Task>, mut threadIdx: i32) -> Arc<HpcOmSimCode::Task> {
    let mut taskOut: Arc<HpcOmSimCode::Task>;
    taskOut = 'mc: {
        let __mc_input = taskIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Task::CALCTASK { weighting, index, calcTime, timeFinished, eqIdc, .. } => {
                    Ok(Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting.clone(), index: index.clone(), calcTime: calcTime.clone(), timeFinished: timeFinished.clone(), threadIdx: threadIdx, eqIdc: eqIdc.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(taskIn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    taskOut
}

fn tasksEqual(mut task1: Arc<HpcOmSimCode::Task>, mut task2: Arc<HpcOmSimCode::Task>) -> bool {
    let mut isEqOut: bool;
    isEqOut = (::match_deref::match_deref! { match &((task1, task2)) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { index: id1, .. }, Deref @ HpcOmSimCode::Task::CALCTASK { index: id2, .. }) => {
            let mut isEq: bool;
            isEq = intEq(id1.clone(), id2.clone());
            isEq.clone()
        },
        (Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc: nodeIdc1, .. }, Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc: nodeIdc2, .. }) => {
            let mut isEq: bool;
            isEq = List::isEqual(nodeIdc1.clone(), nodeIdc2.clone(), true);
            isEq.clone()
        },
        (Deref @ HpcOmSimCode::Task::DEPTASK { sourceTask: sourceTask1, targetTask: targetTask1, .. }, Deref @ HpcOmSimCode::Task::DEPTASK { sourceTask: sourceTask2, targetTask: targetTask2, .. }) => {
            let mut isEq: bool;
            isEq = tasksEqual(sourceTask1.clone(), sourceTask2.clone());
            isEq = boolAnd(isEq.clone(), tasksEqual(targetTask1.clone(), targetTask2.clone()));
            isEq.clone()
        },
        (Deref @ HpcOmSimCode::Task::TASKEMPTY { .. }, Deref @ HpcOmSimCode::Task::TASKEMPTY { .. }) => {
            false
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEqOut
}

fn removeLocksFromLockList(mut lockIdsIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut lockTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut lockIdsOut: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    (_, lockIdsOut, _) = List::intersection1OnTrue(lockIdsIn, lockTasks, (std::sync::Arc::new(fnptr!(tasksEqual, Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
    Ok(lockIdsOut)
}

fn removeLocksFromThread(mut threadIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut lockLst: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut threadOut: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    (_, threadOut, _) = List::intersection1OnTrue(threadIn, lockLst, (std::sync::Arc::new(fnptr!(tasksEqual, Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<HpcOmSimCode::Task>) -> Result<bool> + 'static>))?;
    Ok(threadOut)
}

fn getSuperfluousLocks(mut otherParentsIn: Arc<metamodelica::List<i32>>, mut nodeIn: i32, mut taskAssIn: metamodelica::Array<i32>, mut orderIn: Arc<metamodelica::List<i32>>, mut numProc: i32, mut iAllCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, mut iCommCosts: metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, mut iCompTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSimVarMapping: metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>, mut removeLocksIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut removeLocksOut: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut parentsOnThreads: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut otherParentsProcs: Arc<metamodelica::List<i32>>;
    let mut lockCandidatesFlat: Arc<metamodelica::List<i32>>;
    let mut lockCandidates: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut removeLocks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut taskLstAss: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut taskLstRel: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    otherParentsProcs = List::map1(otherParentsIn.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), taskAssIn.clone())?;
    parentsOnThreads = arrayCreate(numProc, metamodelica::nil());
    parentsOnThreads = List::fold1(List::intRange((otherParentsProcs.clone().len() as i32)), (std::sync::Arc::new(listIndecesForValues) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), otherParentsProcs, parentsOnThreads.clone())?;
    parentsOnThreads = Array::map1(parentsOnThreads.clone(), (std::sync::Arc::new(mapListGet) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), otherParentsIn)?;
    lockCandidates = List::filterOnTrue(Arc::new(parentsOnThreads.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), (std::sync::Arc::new(fnptr!(lengthNotOne, Arc<metamodelica::List<i32>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<bool> + 'static>))?;
    lockCandidates = List::map1(lockCandidates, (std::sync::Arc::new(removeLatestTaskFromList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), orderIn)?;
    lockCandidatesFlat = List::flatten(lockCandidates)?;
    taskLstAss = List::map6(lockCandidatesFlat.clone(), (std::sync::Arc::new(createDepTaskByTaskIdc) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, bool, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Task>> + 'static>), nodeIn, iAllCalcTasks.clone(), false, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
    taskLstRel = List::map6(lockCandidatesFlat, (std::sync::Arc::new(createDepTaskByTaskIdc) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>, bool, metamodelica::Array<Arc<metamodelica::List<HpcOmTaskGraph::Communication>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<SimCodeVar::SimVar>>>) -> Result<Arc<HpcOmSimCode::Task>> + 'static>), nodeIn, iAllCalcTasks.clone(), true, iCommCosts.clone(), iCompTaskMapping.clone(), iSimVarMapping.clone())?;
    removeLocks = listAppend(removeLocksIn, taskLstAss);
    removeLocksOut = listAppend(removeLocks, taskLstRel);
    Ok(removeLocksOut)
}

fn removeLatestTaskFromList(mut taskLstIn: Arc<metamodelica::List<i32>>, mut taskOrderIn: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut taskLstOut: Arc<metamodelica::List<i32>>;
    taskLstOut = (::match_deref::match_deref! { match &(taskLstIn.clone()) {
        Deref @ metamodelica::List::Nil => {
            taskLstIn
        },
        _ => {
            let mut posInOrder: Arc<metamodelica::List<i32>>;
            let mut taskLst: Arc<metamodelica::List<i32>>;
            let mut latestTask: i32;
            posInOrder = List::map1(taskLstIn.clone(), (std::sync::Arc::new(List::position) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<i32> + 'static>), taskOrderIn.clone())?;
            posInOrder = List::map1(posInOrder.clone(), (std::sync::Arc::new(fnptr!(intSub, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1)?;
            latestTask = List::fold(posInOrder.clone(), (std::sync::Arc::new(fnptr!(intMax, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), -1)?;
            latestTask = (taskOrderIn).get(latestTask.clone() + 1)?;
            taskLst = List::removeOnTrue(latestTask.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), taskLstIn)?;
            taskLst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(taskLstOut)
}

fn lengthNotOne(mut lstIn: Arc<metamodelica::List<i32>>) -> bool {
    let mut b: bool;
    b = intNe((lstIn.len() as i32), 1);
    b
}

fn mapListGet(mut mapLstIn: Arc<metamodelica::List<i32>>, mut argLst: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut mapLstOut: Arc<metamodelica::List<i32>>;
    mapLstOut = List::map1(mapLstIn, (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), argLst)?;
    Ok(mapLstOut)
}

fn listIndecesForValues(mut idx: i32, mut lstIn: Arc<metamodelica::List<i32>>, mut arrayIn: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut arrayOut: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut value: i32;
    let mut valueLst: Arc<metamodelica::List<i32>>;
    value = (lstIn).get(idx)?;
    valueLst = metamodelica::arrayGet(arrayIn.clone(), value)?;
    valueLst = metamodelica::cons(idx, valueLst);
    arrayOut = metamodelica::arrayUpdate(arrayIn.clone(), value, valueLst)?;
    Ok(arrayOut)
}

//---------------------------
// quicksort with order
//---------------------------
pub(crate) fn quicksortWithOrder(mut lstIn: Arc<metamodelica::List<metamodelica::Real>>) -> Result<(Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<i32>>)> {
    let mut lstOut: Arc<metamodelica::List<metamodelica::Real>>;
    let mut orderOut: Arc<metamodelica::List<i32>>;
    (lstOut, orderOut) = 'mc: {
        let __mc_input = lstIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut length: i32;
                    let mut pivotIdx: i32;
                    let mut r1: metamodelica::Real;
                    let mut r2: metamodelica::Real;
                    let mut r3: metamodelica::Real;
                    let mut pivotValue: metamodelica::Real;
                    let mut orderTmp: Arc<metamodelica::List<i32>>;
                    let mut lstTmp: Arc<metamodelica::List<metamodelica::Real>>;
                    length = (lstIn.clone().len() as i32);
                    orderTmp = List::intRange(length.clone());
                    r1 = listHead(lstIn.clone())?;
                    r2 = List::last(lstIn.clone())?;
                    r3 = (lstIn.clone()).get(intDiv(length.clone(), 2))?;
                    (pivotValue, _) = getMedian3(r1.clone(), r2.clone(), r3.clone())?;
                    pivotIdx = List::position(pivotValue.clone(), lstIn.clone())?;
                    (lstTmp, orderTmp) = quicksortWithOrder1(lstIn.clone(), orderTmp.clone(), pivotIdx.clone(), lstIn.clone(), length.clone())?;
                    Ok((lstTmp.clone(), orderTmp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r1, tail: Deref @ metamodelica::List::Nil } => {
                    Ok((list![r1.clone()], list![1]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((lstOut, orderOut))
}

fn quicksortWithOrder1(mut lstIn: Arc<metamodelica::List<metamodelica::Real>>, mut orderIn: Arc<metamodelica::List<i32>>, mut pivotIdx: i32, mut markedIn: Arc<metamodelica::List<metamodelica::Real>>, mut size: i32) -> Result<(Arc<metamodelica::List<metamodelica::Real>>, Arc<metamodelica::List<i32>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((lstIn.clone(), markedIn.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok((metamodelica::nil(), metamodelica::nil()))
        },
        (Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, _) => {
            return Ok((list![e.clone()], list![1]))
        },
        (_, Deref @ metamodelica::List::Nil) => {
            return Ok((lstIn, orderIn))
        },
        _ => {
            let mut b1: bool;
            let mut b2: bool;
            let mut lIdx: i32;
            let mut rIdx: i32;
            let mut pivot: i32;
            let mut p: metamodelica::Real;
            let mut orderTmp: Arc<metamodelica::List<i32>>;
            let mut marked: Arc<metamodelica::List<metamodelica::Real>>;
            let mut lstTmp: Arc<metamodelica::List<metamodelica::Real>>;
            let mut leftLst: Arc<metamodelica::List<metamodelica::Real>>;
            let mut rightLst: Arc<metamodelica::List<metamodelica::Real>>;
            p = (lstIn.clone()).get(pivotIdx)?;
            (leftLst, rightLst) = List::split(lstIn.clone(), pivotIdx)?;
            rightLst = rightLst.clone().reverse();
            (_, lIdx, b1) = getMemberOnTrueWithIdx(p.clone(), leftLst.clone(), (std::sync::Arc::new(fnptr!(realLt, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>));
            (_, rIdx, b2) = getMemberOnTrueWithIdx(p.clone(), rightLst.clone(), (std::sync::Arc::new(fnptr!(realGt, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>));
            rIdx = size + 1 - rIdx.clone();
            lstTmp = if (b1.clone()) {swapEntriesInList(pivotIdx, lIdx.clone(), lstIn)?} else {lstIn};
            lstTmp = if (b2.clone()) {swapEntriesInList(pivotIdx, rIdx.clone(), lstTmp.clone())?} else {lstTmp.clone()};
            orderTmp = if (b1.clone()) {swapEntriesInList(pivotIdx, lIdx.clone(), orderIn)?} else {orderIn};
            orderTmp = if (b2.clone()) {swapEntriesInList(pivotIdx, rIdx.clone(), orderTmp.clone())?} else {orderTmp.clone()};
            if !(b1.clone()) && !(b2.clone()) {
                (marked, pivot) = getNextPivot(lstTmp.clone(), markedIn, pivotIdx)?;
            } else {
                marked = markedIn;
                pivot = pivotIdx;
            }
            { (lstIn, orderIn, pivotIdx, markedIn, size) = (lstTmp.clone(), orderTmp.clone(), pivot.clone(), marked.clone(), size); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn getNextPivot(mut lstIn: Arc<metamodelica::List<metamodelica::Real>>, mut markedLstIn: Arc<metamodelica::List<metamodelica::Real>>, mut pivotIdx: i32) -> Result<(Arc<metamodelica::List<metamodelica::Real>>, i32)> {
    let mut marked: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
    let mut newIdx: i32 = 0;
    (marked, newIdx) = (::match_deref::match_deref! { match &(markedLstIn.clone()) {
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => {
            (metamodelica::nil(), 0)
        },
        Deref @ metamodelica::List::Cons { head: _, tail: _ } => {
            let mut midIdx: i32;
            let mut pivotElement: metamodelica::Real;
            let mut r1: metamodelica::Real;
            let mut r2: metamodelica::Real;
            let mut r3: metamodelica::Real;
            pivotElement = (lstIn.clone()).get(pivotIdx)?;
            (marked, _) = List::deleteMemberOnTrue(pivotElement.clone(), markedLstIn, (std::sync::Arc::new(fnptr!(realEq, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>))?;
            r1 = listHead(marked.clone())?;
            r2 = List::last(marked.clone())?;
            midIdx = intDiv((marked.clone().len() as i32), 2);
            midIdx = if (intEq(midIdx.clone(), 0)) {1} else {midIdx.clone()};
            r3 = (marked.clone()).get(midIdx.clone())?;
            (pivotElement, _) = getMedian3(r1.clone(), r2.clone(), r3.clone())?;
            newIdx = List::position(pivotElement.clone(), lstIn)?;
            (marked, newIdx)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((marked, newIdx))
}

fn getMemberOnTrueWithIdx(mut inValue: metamodelica::Real, mut inList: Arc<metamodelica::List<metamodelica::Real>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>) -> (metamodelica::Real, i32, bool) {
    pub type CompFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>;

    let mut outElement: metamodelica::Real;
    let mut outIdx: i32;
    let mut found: bool;
    (outElement, outIdx, found) = getMemberOnTrueWithIdx1(1, inValue, inList, inCompFunc.clone());
    (outElement, outIdx, found)
}

fn getMemberOnTrueWithIdx1(mut inIdx: i32, mut inValue: metamodelica::Real, mut inList: Arc<metamodelica::List<metamodelica::Real>>, mut inCompFunc: Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>) -> (metamodelica::Real, i32, bool) {
    pub type CompFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>;

    let mut outElement: metamodelica::Real;
    let mut outIdx: i32;
    let mut found: bool;
    (outElement, outIdx, found) = 'mc: {
        let __mc_input = inList;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((metamodelica::OrderedFloat(0.0_f64), 0, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e, tail: _ } => {
                    let mut b: bool;
                    b = inCompFunc(inValue, e.clone())?;
                    let true = (b.clone()) else { bail!("pattern mismatch") };
                    Ok((e.clone(), inIdx, b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut value: metamodelica::Real;
                    let mut idx: i32;
                    let mut b: bool;
                    (value, idx, b) = getMemberOnTrueWithIdx1(inIdx + 1, inValue, rest.clone(), inCompFunc.clone());
                    Ok((value.clone(), idx.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outElement, outIdx, found)
}

fn swapEntriesInList<ElementType: Clone + 'static + metamodelica::gc::MMTrace>(mut idx1: i32, mut idx2: i32, mut lstIn: Arc<metamodelica::List<ElementType>>) -> Result<Arc<metamodelica::List<ElementType>>> {
    let mut lstOut: Arc<metamodelica::List<ElementType>>;
    let mut r1: ElementType;
    let mut r2: ElementType;
    let mut lstTmp: Arc<metamodelica::List<ElementType>>;
    r1 = (lstIn.clone()).get(idx1)?;
    r2 = (lstIn.clone()).get(idx2)?;
    lstTmp = List::replaceAt(r1, idx2, lstIn)?;
    lstOut = List::replaceAt(r2, idx1, lstTmp)?;
    Ok(lstOut)
}

fn getMedian3(mut r1: metamodelica::Real, mut r2: metamodelica::Real, mut r3: metamodelica::Real) -> Result<(metamodelica::Real, i32)> {
    let mut rOut: metamodelica::Real;
    let mut which: i32;
    let mut r: Arc<metamodelica::List<metamodelica::Real>>;
    r = List::sort(list![r1, r2, r3], (std::sync::Arc::new(fnptr!(realGt, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>))?;
    rOut = (r).get(2)?;
    which = List::position(rOut, list![r1, r2, r3])?;
    Ok((rOut, which))
}

//----------------------------
// traverse the task graph bottoms up (beginning at the root nodes)
//----------------------------
fn computeGraphValuesBottomUp(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>)> {
    let mut asapOut: metamodelica::Array<metamodelica::Real>;
    let mut estOut: metamodelica::Array<metamodelica::Real>;
    let mut ectOut: metamodelica::Array<metamodelica::Real>;
    let mut size: i32;
    let mut rootNodes: Arc<metamodelica::List<i32>>;
    let mut asap: metamodelica::Array<metamodelica::Real>;
    let mut ect: metamodelica::Array<metamodelica::Real>;
    let mut est: metamodelica::Array<metamodelica::Real>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    size = metamodelica::arrayLength(iTaskGraph.clone());
    rootNodes = HpcOmTaskGraph::getRootNodes(iTaskGraph.clone())?;
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size)?;
    asap = arrayCreate(size, metamodelica::OrderedFloat(-1.0_f64));
    est = arrayCreate(size, metamodelica::OrderedFloat(-1.0_f64));
    ect = arrayCreate(size, metamodelica::OrderedFloat(-1.0_f64));
    (asapOut, estOut, ectOut) = computeGraphValuesBottomUp1(rootNodes, iTaskGraph.clone(), taskGraphT.clone(), iTaskGraphMeta, asap.clone(), est.clone(), ect.clone())?;
    Ok((asapOut, estOut, ectOut))
}

fn computeGraphValuesBottomUp1(mut parentsIn: Arc<metamodelica::List<i32>>, mut graph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut graphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut asapIn: metamodelica::Array<metamodelica::Real>, mut estIn: metamodelica::Array<metamodelica::Real>, mut ectIn: metamodelica::Array<metamodelica::Real>) -> Result<(metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((parentsIn, asapIn.clone(), estIn.clone(), ectIn.clone())) {
        (Deref @ metamodelica::List::Cons { head: node, tail: rest }, asap, est, ect) => {
            let mut children: Arc<metamodelica::List<i32>>;
            let mut asap = (*asap).clone();
            let mut est = (*est).clone();
            let mut ect = (*ect).clone();
            (asap, est, ect, children) = computeGraphValuesBottomUp2(node.clone(), graph.clone(), graphT.clone(), iTaskGraphMeta.clone(), asap.clone(), est.clone(), ect.clone())?;
            { (parentsIn, graph, graphT, iTaskGraphMeta, asapIn, estIn, ectIn) = (listAppend(rest.clone(), children.clone()), graph.clone(), graphT.clone(), iTaskGraphMeta, asap.clone(), est.clone(), ect.clone()); continue '__tco; }
        },
        (Deref @ metamodelica::List::Nil, _, _, _) => {
            return Ok((asapIn.clone(), estIn.clone(), ectIn.clone()))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn computeGraphValuesBottomUp2(mut node: i32, mut graph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut graphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut asapIn: metamodelica::Array<metamodelica::Real>, mut estIn: metamodelica::Array<metamodelica::Real>, mut ectIn: metamodelica::Array<metamodelica::Real>) -> Result<(metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>, Arc<metamodelica::List<i32>>)> {
    let mut asapOut: metamodelica::Array<metamodelica::Real>;
    let mut estOut: metamodelica::Array<metamodelica::Real>;
    let mut ectOut: metamodelica::Array<metamodelica::Real>;
    let mut children: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (asapOut, estOut, ectOut, children) = 'mc: {
        let __mc_input = ectIn.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut maxASAP: metamodelica::Real;
            let mut maxEct: metamodelica::Real;
            let mut exeCost: metamodelica::Real;
            let mut asap: metamodelica::Array<metamodelica::Real>;
            let mut ect: metamodelica::Array<metamodelica::Real>;
            let mut est: metamodelica::Array<metamodelica::Real>;
            let mut parents: Arc<metamodelica::List<i32>>;
            let mut parentEcts: Arc<metamodelica::List<metamodelica::Real>>;
            let mut parentAsaps: Arc<metamodelica::List<metamodelica::Real>>;
            let mut parentAsaps2: Arc<metamodelica::List<metamodelica::Real>>;
            let mut parentsExeCosts: Arc<metamodelica::List<metamodelica::Real>>;
            let mut commCosts: Arc<metamodelica::List<metamodelica::Real>>;
            let mut children: Arc<metamodelica::List<i32>> = children.clone();
            parents = metamodelica::arrayGet(graphT.clone(), node)?;
            parentAsaps = List::map1(parents.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), asapIn.clone())?;
            let false = (List::isMemberOnTrue(metamodelica::OrderedFloat(-1.0_f64), parentAsaps.clone(), (std::sync::Arc::new(fnptr!(realEq, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
            exeCost = HpcOmTaskGraph::getExeCostReqCycles(node, iTaskGraphMeta.clone())?;
            parentsExeCosts = List::map1(parents.clone(), (std::sync::Arc::new(HpcOmTaskGraph::getExeCostReqCycles) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> + 'static>), iTaskGraphMeta.clone())?;
            commCosts = List::map2(parents.clone(), (std::sync::Arc::new(HpcOmTaskGraph::getCommCostTimeBetweenNodes) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> + 'static>), node, iTaskGraphMeta.clone())?;
            parentAsaps2 = List::threadMap(parentAsaps.clone(), parentsExeCosts.clone(), (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>))?;
            parentAsaps2 = List::threadMap(parentAsaps2.clone(), commCosts.clone(), (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>))?;
            maxASAP = List::fold(parentAsaps2.clone(), (std::sync::Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
            asap = metamodelica::arrayUpdate(asapIn.clone(), node, maxASAP.clone())?;
            parentEcts = List::map1(parents.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), ectIn.clone())?;
            maxEct = List::fold(parentEcts.clone(), (std::sync::Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
            est = metamodelica::arrayUpdate(estIn.clone(), node, maxEct.clone())?;
            ect = metamodelica::arrayUpdate(ectIn.clone(), node, (maxEct.clone()) + (exeCost.clone()))?;
            children = metamodelica::arrayGet(graph.clone(), node)?;
            Ok(((asap.clone(), est.clone(), ect.clone(), children.clone()), children.clone()))
        })() { children = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut parents: Arc<metamodelica::List<i32>>;
            let mut parentAsaps: Arc<metamodelica::List<metamodelica::Real>>;
            parents = metamodelica::arrayGet(graphT.clone(), node)?;
            parentAsaps = List::map1(parents.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), asapIn.clone())?;
            let true = (List::isMemberOnTrue(metamodelica::OrderedFloat(-1.0_f64), parentAsaps.clone(), (std::sync::Arc::new(fnptr!(realEq, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
            Ok((asapIn.clone(), estIn.clone(), ectIn.clone(), list![node]))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("computeGraphValuesBottomUp2 failed!\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((asapOut, estOut, ectOut, children))
}

//----------------------------
// traverse the task graph top down (beginning at the leaf nodes)
//----------------------------
fn computeGraphValuesTopDown(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>, metamodelica::Array<metamodelica::Real>)> {
    let mut alapOut: metamodelica::Array<metamodelica::Real>;
    let mut lastOut: metamodelica::Array<metamodelica::Real>;
    let mut lactOut: metamodelica::Array<metamodelica::Real>;
    let mut tdsLevelOut: metamodelica::Array<metamodelica::Real>;
    let mut size: i32;
    let mut lastNodeInCP: i32;
    let mut cp: metamodelica::Real;
    let mut cpWithComm: metamodelica::Real;
    let mut endNodes: Arc<metamodelica::List<i32>>;
    let mut alap: metamodelica::Array<metamodelica::Real>;
    let mut lact: metamodelica::Array<metamodelica::Real>;
    let mut last: metamodelica::Array<metamodelica::Real>;
    let mut tdsLevel: metamodelica::Array<metamodelica::Real>;
    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut visitedNodes: metamodelica::Array<bool>;
    size = metamodelica::arrayLength(iTaskGraph.clone());
    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(iTaskGraph.clone(), size)?;
    endNodes = HpcOmTaskGraph::getLeafNodes(iTaskGraph.clone())?;
    alap = arrayCreate(size, metamodelica::OrderedFloat(-1.0_f64));
    last = arrayCreate(size, metamodelica::OrderedFloat(-1.0_f64));
    lact = arrayCreate(size, metamodelica::OrderedFloat(-1.0_f64));
    tdsLevel = arrayCreate(size, metamodelica::OrderedFloat(-1.0_f64));
    visitedNodes = arrayCreate(size, false);
    computeGraphValuesTopDown1(endNodes, iTaskGraph.clone(), taskGraphT.clone(), iTaskGraphMeta, alap.clone(), last.clone(), lact.clone(), tdsLevel.clone(), visitedNodes.clone())?;
    cpWithComm = Array::fold(alap.clone(), (std::sync::Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
    lastNodeInCP = Array::position(alap.clone(), cpWithComm, size);
    cp = Array::fold(last.clone(), (std::sync::Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
    alapOut = Array::map1(alap.clone(), (std::sync::Arc::new(fnptr!(realSubr, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), cpWithComm)?;
    lactOut = Array::map1(lact.clone(), (std::sync::Arc::new(fnptr!(realSubr, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), cp)?;
    lastOut = Array::map1(last.clone(), (std::sync::Arc::new(fnptr!(realSubr, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), cp)?;
    tdsLevelOut = tdsLevel.clone();
    Ok((alapOut, lastOut, lactOut, tdsLevelOut))
}

fn computeGraphValuesTopDown1(mut nodesIn: Arc<metamodelica::List<i32>>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut alapIn: metamodelica::Array<metamodelica::Real>, mut lastIn: metamodelica::Array<metamodelica::Real>, mut lactIn: metamodelica::Array<metamodelica::Real>, mut tdsLevelIn: metamodelica::Array<metamodelica::Real>, mut visitedNodes: metamodelica::Array<bool>) -> Result<()> {
    let mut nodes: Arc<metamodelica::List<i32>> = nodesIn.clone();
    let mut alap: metamodelica::Array<metamodelica::Real> = alapIn.clone();
    let mut last: metamodelica::Array<metamodelica::Real> = lastIn.clone();
    let mut lact: metamodelica::Array<metamodelica::Real> = lactIn.clone();
    let mut tdsLevel: metamodelica::Array<metamodelica::Real> = tdsLevelIn.clone();
    while !(nodes.clone().is_empty()) {
        if metamodelica::arrayGet(visitedNodes.clone(), listHead(nodes.clone())?)? {
            nodes = listRest(nodes.clone())?;
        } else {
            nodes = computeGraphValuesTopDown2(nodes.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iTaskGraphMeta.clone(), alap.clone(), last.clone(), lact.clone(), tdsLevel.clone(), visitedNodes.clone())?;
        }
    }
    Ok(())
}

fn computeGraphValuesTopDown2(mut nodesIn: Arc<metamodelica::List<i32>>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut alapIn: metamodelica::Array<metamodelica::Real>, mut lastIn: metamodelica::Array<metamodelica::Real>, mut lactIn: metamodelica::Array<metamodelica::Real>, mut tdsLevelIn: metamodelica::Array<metamodelica::Real>, mut visitedNodes: metamodelica::Array<bool>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut nodesOut: Arc<metamodelica::List<i32>>;
    let mut nodeIdx: i32;
    let mut nodeExeCost: metamodelica::Real;
    let mut maxLevel: metamodelica::Real;
    let mut maxAlap: metamodelica::Real;
    let mut maxLast: metamodelica::Real;
    let mut rest: Arc<metamodelica::List<i32>>;
    let mut parentNodes: Arc<metamodelica::List<i32>>;
    let mut childNodes: Arc<metamodelica::List<i32>>;
    let mut childTDSLevels: Arc<metamodelica::List<metamodelica::Real>>;
    let mut childAlaps: Arc<metamodelica::List<metamodelica::Real>>;
    let mut childLasts: Arc<metamodelica::List<metamodelica::Real>>;
    let mut childLacts: Arc<metamodelica::List<metamodelica::Real>>;
    let mut commCostsToChilds: Arc<metamodelica::List<metamodelica::Real>>;
    let mut alap: metamodelica::Array<metamodelica::Real>;
    let mut last: metamodelica::Array<metamodelica::Real>;
    let mut lact: metamodelica::Array<metamodelica::Real>;
    let mut tdsLevel: metamodelica::Array<metamodelica::Real>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(nodesIn) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    nodeIdx = __pa0.clone();
    rest = __pa1.clone();
    childNodes = metamodelica::arrayGet(iTaskGraph.clone(), nodeIdx)?;
    nodeExeCost = HpcOmTaskGraph::getExeCostReqCycles(nodeIdx, iTaskGraphMeta.clone())?;
    metamodelica::arrayUpdate(visitedNodes.clone(), nodeIdx, true)?;
    if childNodes.clone().is_empty() {
        alap = metamodelica::arrayUpdate(alapIn.clone(), nodeIdx, nodeExeCost)?;
        last = metamodelica::arrayUpdate(lastIn.clone(), nodeIdx, nodeExeCost)?;
        lact = metamodelica::arrayUpdate(lactIn.clone(), nodeIdx, metamodelica::OrderedFloat(0.0_f64))?;
        tdsLevel = metamodelica::arrayUpdate(tdsLevelIn.clone(), nodeIdx, nodeExeCost)?;
        parentNodes = metamodelica::arrayGet(iTaskGraphT.clone(), nodeIdx)?;
        nodesOut = listAppend(rest, parentNodes);
    } else {
        childTDSLevels = List::map1(childNodes.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), tdsLevelIn.clone())?;
        if List::isMemberOnTrue(metamodelica::OrderedFloat(-1.0_f64), childTDSLevels.clone(), (std::sync::Arc::new(fnptr!(realEq, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<bool> + 'static>))? {
            nodesOut = listAppend(rest, list![nodeIdx]);
            metamodelica::arrayUpdate(visitedNodes.clone(), nodeIdx, false)?;
        } else {
            commCostsToChilds = ({
        let mut __acc: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
        for mut n in (childNodes.clone()).into_iter().cloned() {
            let __x = HpcOmTaskGraph::getCommCostTimeBetweenNodes(nodeIdx, n.clone(), iTaskGraphMeta.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            childAlaps = List::map1(childNodes.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), alapIn.clone())?;
            childAlaps = List::threadMap(childAlaps, commCostsToChilds, (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>))?;
            childLasts = List::map1(childNodes.clone(), (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), lastIn.clone())?;
            childLacts = List::map1(childNodes, (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), lactIn.clone())?;
            maxLevel = List::fold(childTDSLevels, (std::sync::Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
            maxAlap = List::fold(childAlaps, (std::sync::Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
            maxLast = List::fold(childLasts, (std::sync::Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
            tdsLevel = metamodelica::arrayUpdate(tdsLevelIn.clone(), nodeIdx, nodeExeCost + maxLevel)?;
            alap = metamodelica::arrayUpdate(alapIn.clone(), nodeIdx, nodeExeCost + maxAlap)?;
            last = metamodelica::arrayUpdate(lastIn.clone(), nodeIdx, nodeExeCost + maxLast)?;
            lact = metamodelica::arrayUpdate(lactIn.clone(), nodeIdx, maxLast)?;
            parentNodes = metamodelica::arrayGet(iTaskGraphT.clone(), nodeIdx)?;
            nodesOut = listAppend(rest, parentNodes);
        }
    }
    Ok(nodesOut)
}

fn realSubr(mut r1: metamodelica::Real, mut r2: metamodelica::Real) -> metamodelica::Real {
    let mut r3: metamodelica::Real;
    r3 = (r2) - (r1);
    r3
}

//-----
// Util
//-----
pub(crate) fn printSchedule(mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<()> {
    metamodelica::print((dumpSchedule(iSchedule)?).clone());
    Ok(())
}

fn dumpSchedule(mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: ArcStr = arcstr::literal!("");
    let mut sLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut allTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = Default::default();
    let mut tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut taskDepTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    r#str = ((::match_deref::match_deref! { match &(iSchedule) {
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __esc_threadTasks, outgoingDepTasks: __esc_outgoingDepTasks, .. } => {
            threadTasks = (*__esc_threadTasks).clone();
            outgoingDepTasks = (*__esc_outgoingDepTasks).clone();
            (sLst, _) = List::mapFold(Arc::new(threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), (std::sync::Arc::new(dumpThreadSchedule) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, i32) -> Result<(ArcStr, i32)> + 'static>), 1)?;
            s = stringDelimitList(sLst, (literal!("\n")).clone());
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("\nDependency tasks: {\n")); __mm_s.push_str(&*stringDelimitList(List::map(outgoingDepTasks.clone(), (std::sync::Arc::new(dumpTask) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>) -> Result<ArcStr> + 'static>))?, (literal!("")).clone())); __mm_s.push_str(&*literal!("}\n")); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("THREADSCHEDULE\n")); __mm_s.push_str(&*s); ArcStr::from(__mm_s) }).clone();
            s
        },
        Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: __esc_tasksOfLevels, .. } => {
            tasksOfLevels = (*__esc_tasksOfLevels).clone();
            (sLst, _) = List::mapFold(tasksOfLevels.clone(), (std::sync::Arc::new(dumpLevelSchedule) as std::sync::Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList, i32) -> Result<(ArcStr, i32)> + 'static>), 1)?;
            s = stringDelimitList(sLst, (literal!("\n")).clone());
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("LEVELSCHEDULE\n")); __mm_s.push_str(&*s); ArcStr::from(__mm_s) }).clone();
            s
        },
        Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: __esc_taskDepTasks } => {
            taskDepTasks = (*__esc_taskDepTasks).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringDelimitList(List::map(taskDepTasks.clone(), (std::sync::Arc::new(dumpTaskDepSchedule) as std::sync::Arc<dyn ::std::ops::Fn((Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)) -> Result<ArcStr> + 'static>))?, (literal!("\n")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TASKDEPSCHEDULE\n")); __mm_s.push_str(&*s); ArcStr::from(__mm_s) }).clone();
            s
        },
        Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: __esc_allTasks, .. } } => {
            allTasks = (*__esc_allTasks).clone();
            (s, _) = dumpThreadSchedule(allTasks.clone(), 1)?;
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("EMPTYSCHEDULE\n")); __mm_s.push_str(&*s); ArcStr::from(__mm_s) }).clone();
            s
        },
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

pub(crate) fn analyseScheduledTaskGraph(mut scheduleIn: Arc<HpcOmSimCode::Schedule>, mut numProcIn: i32, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta, mut inSystemName: ArcStr) -> ArcStr {
    let mut criticalPathInfoOut: ArcStr;
    criticalPathInfoOut = ('mc: {
        let __mc_input = scheduleIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: _ } => {
                    let mut criticalPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut criticalPathsWoC: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut cpCosts: metamodelica::Real;
                    let mut cpCostsWoC: metamodelica::Real;
                    let mut criticalPathInfo: ArcStr;
                    let ((__pa0, __pa1), (__pa2, __pa3)) = HpcOmTaskGraph::getCriticalPaths(taskGraphIn.clone(), taskGraphMetaIn.clone());
                    criticalPaths = __pa0.clone();
                    cpCosts = __pa1.clone();
                    criticalPathsWoC = __pa2.clone();
                    cpCostsWoC = __pa3.clone();
                    criticalPathInfo = (HpcOmTaskGraph::dumpCriticalPathInfo((criticalPaths.clone(), cpCosts.clone()), (criticalPathsWoC.clone(), cpCostsWoC.clone()))?).clone();
                    Ok(criticalPathInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels, useFixedAssignments: false } => {
                    let mut criticalPathInfo: ArcStr;
                    criticalPathInfo = (analyseScheduledTaskGraphLevel(tasksOfLevels.clone(), numProcIn, taskGraphIn.clone(), taskGraphMetaIn.clone(), (std::sync::Arc::new(getLevelParallelTime) as std::sync::Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, i32) -> Result<metamodelica::Real> + 'static>))?).clone();
                    Ok(criticalPathInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels, useFixedAssignments: true } => {
                    let mut criticalPathInfo: ArcStr;
                    criticalPathInfo = (analyseScheduledTaskGraphLevel(tasksOfLevels.clone(), numProcIn, taskGraphIn.clone(), taskGraphMetaIn.clone(), (std::sync::Arc::new(getLevelParallelTime) as std::sync::Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, i32) -> Result<metamodelica::Real> + 'static>))?).clone();
                    Ok(criticalPathInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks, .. } => {
                    let mut criticalPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut criticalPathsWoC: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut cpCosts: metamodelica::Real;
                    let mut cpCostsWoC: metamodelica::Real;
                    let mut serTime: metamodelica::Real;
                    let mut parTime: metamodelica::Real;
                    let mut speedUp: metamodelica::Real;
                    let mut speedUpMax: metamodelica::Real;
                    let mut criticalPathInfo: ArcStr;
                    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the number of locks: ")); __mm_s.push_str(&*intString((outgoingDepTasks.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    let ((__pa0, __pa1), (__pa2, __pa3)) = HpcOmTaskGraph::getCriticalPaths(taskGraphIn.clone(), taskGraphMetaIn.clone());
                    criticalPaths = __pa0.clone();
                    cpCosts = __pa1.clone();
                    criticalPathsWoC = __pa2.clone();
                    cpCostsWoC = __pa3.clone();
                    criticalPathInfo = (HpcOmTaskGraph::dumpCriticalPathInfo((criticalPaths.clone(), cpCosts.clone()), (criticalPathsWoC.clone(), cpCostsWoC.clone()))?).clone();
                    (serTime, parTime, speedUp, speedUpMax) = predictExecutionTime(scheduleIn.clone(), Some(cpCostsWoC.clone()), numProcIn, taskGraphIn.clone(), taskGraphMetaIn.clone())?;
                    serTime = HpcOmTaskGraph::roundReal(serTime.clone(), 2);
                    parTime = HpcOmTaskGraph::roundReal(parTime.clone(), 2);
                    cpCostsWoC = HpcOmTaskGraph::roundReal(cpCostsWoC.clone(), 2);
                    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the serialCosts: ")); __mm_s.push_str(&*realString(serTime.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the parallelCosts: ")); __mm_s.push_str(&*realString(parTime.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the cpCosts: ")); __mm_s.push_str(&*realString(cpCostsWoC.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    if realLe(speedUpMax.clone(), metamodelica::OrderedFloat(2.0_f64)) {
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("There is no parallel potential in the ")); __mm_s.push_str(&*inSystemName.clone()); __mm_s.push_str(&*literal!(" model!\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    if realLe(serTime.clone(), metamodelica::OrderedFloat(20000.0_f64)) {
                        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The ")); __mm_s.push_str(&*inSystemName.clone()); __mm_s.push_str(&*literal!(" model is not big enough to perform an effective parallel simulation!\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    printPredictedExeTimeInfo(serTime.clone(), parTime.clone(), speedUp.clone(), speedUpMax.clone(), numProcIn)?;
                    Ok(criticalPathInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { .. } => {
                    let mut criticalPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut criticalPathsWoC: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut cpCosts: metamodelica::Real;
                    let mut cpCostsWoC: metamodelica::Real;
                    let mut criticalPathInfo: ArcStr;
                    let ((__pa0, __pa1), (__pa2, __pa3)) = HpcOmTaskGraph::getCriticalPaths(taskGraphIn.clone(), taskGraphMetaIn.clone());
                    criticalPaths = __pa0.clone();
                    cpCosts = __pa1.clone();
                    criticalPathsWoC = __pa2.clone();
                    cpCostsWoC = __pa3.clone();
                    criticalPathInfo = (HpcOmTaskGraph::dumpCriticalPathInfo((criticalPaths.clone(), cpCosts.clone()), (criticalPathsWoC.clone(), cpCostsWoC.clone()))?).clone();
                    Ok(criticalPathInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("HpcOmScheduler.analyseScheduledTaskGraph failed\n")).clone());
                    Ok(literal!("HpcOmScheduler.analyseScheduledTaskGraph failed\n"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    }).clone();
    criticalPathInfoOut
}

fn analyseScheduledTaskGraphLevel(mut iLevelTasks: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut iNumProc: i32, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iParallelSectionCalculator: Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, i32) -> Result<metamodelica::Real> + 'static>) -> Result<ArcStr> {
    pub type LevelParallelSectionFunc = std::sync::Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList, metamodelica::Array<Arc<metamodelica::List<i32>>>, HpcOmTaskGraph::TaskGraphMeta, i32) -> Result<metamodelica::Real> + 'static>;

    let mut oCriticalPathInfo: ArcStr;
    let mut i: i32;
    let mut costShare: i32;
    let mut levelCosts: Arc<metamodelica::List<metamodelica::Real>>;
    let mut criticalPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut criticalPathsWoC: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut levelSectionCosts: Arc<metamodelica::List<Arc<metamodelica::List<metamodelica::Real>>>>;
    let mut cpCosts: metamodelica::Real;
    let mut cpCostsWoC: metamodelica::Real;
    let mut serTime: metamodelica::Real;
    let mut parTime: metamodelica::Real;
    let mut speedUp: metamodelica::Real;
    let mut speedUpMax: metamodelica::Real;
    let mut levelCost: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let ((__pa0, __pa1), (__pa2, __pa3)) = HpcOmTaskGraph::getCriticalPaths(iTaskGraph.clone(), iTaskGraphMeta.clone());
    criticalPaths = __pa0.clone();
    cpCosts = __pa1.clone();
    criticalPathsWoC = __pa2.clone();
    cpCostsWoC = __pa3.clone();
    levelSectionCosts = List::map1(iLevelTasks.clone(), (std::sync::Arc::new(getLevelListTaskCosts) as std::sync::Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList, HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<metamodelica::List<metamodelica::Real>>> + 'static>), iTaskGraphMeta.clone())?;
    serTime = realSum(List::map(levelSectionCosts, (std::sync::Arc::new(realSum) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<metamodelica::Real>>) -> Result<metamodelica::Real> + 'static>))?)?;
    serTime = HpcOmTaskGraph::roundReal(serTime, 2);
    levelCosts = List::map(iLevelTasks, (std::sync::Arc::new({ let __pe_b1 = iTaskGraph.clone(); let __pe_b2 = iTaskGraphMeta; let __pe_b3 = iNumProc; move |__pe_a0| iParallelSectionCalculator(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList) -> Result<metamodelica::Real> + 'static>))?;
    parTime = realSum(levelCosts.clone())?;
    parTime = HpcOmTaskGraph::roundReal(parTime, 2);
    oCriticalPathInfo = (HpcOmTaskGraph::dumpCriticalPathInfo((criticalPaths, cpCosts), (criticalPathsWoC, cpCostsWoC))?).clone();
    cpCostsWoC = HpcOmTaskGraph::roundReal(cpCostsWoC, 2);
    if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the serialCosts: ")); __mm_s.push_str(&*realString(serTime)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the parallelCosts: ")); __mm_s.push_str(&*realString(parTime)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("the cpCosts: ")); __mm_s.push_str(&*realString(cpCostsWoC)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        i = 1;
        for mut levelCost in &*levelCosts {
            let mut levelCost = levelCost.clone();
            costShare = intDiv(((levelCost).0.floor() as i32) * 100, ((parTime).0.floor() as i32));
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\tcosts for level ")); __mm_s.push_str(&*intString(i)); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*realString(levelCost)); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*System::snprintff((literal!("%.0f")).clone(), 5, metamodelica::OrderedFloat((costShare) as f64))?); __mm_s.push_str(&*literal!("%)\n")); ArcStr::from(__mm_s) }).clone());
            i = i + 1;
        }
    }
    speedUp = metamodelica::OrderedFloat(0.0_f64);
    speedUpMax = metamodelica::OrderedFloat(0.0_f64);
    if realNe(parTime, metamodelica::OrderedFloat(0.0_f64)) {
        speedUp = realDiv(serTime, parTime);
    }
    if realNe(cpCostsWoC, metamodelica::OrderedFloat(0.0_f64)) {
        speedUpMax = realDiv(serTime, cpCostsWoC);
    }
    printPredictedExeTimeInfo(serTime, parTime, speedUp, speedUpMax, iNumProc)?;
    Ok(oCriticalPathInfo)
}

fn getLevelParallelTime(mut iLevelTaskList: HpcOmSimCode::TaskList, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumProc: i32) -> Result<metamodelica::Real> {
    let mut oLevelCost: metamodelica::Real;
    let mut workload: metamodelica::Array<metamodelica::Real>;
    let mut levelTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    levelTasks = getTasksOfTaskList(iLevelTaskList);
    workload = arrayCreate(iNumProc, metamodelica::OrderedFloat(0.0_f64));
    workload = List::fold(levelTasks, (std::sync::Arc::new({ let __pe_b1 = iTaskGraphMeta; move |__pe_a0, __pe_a2| getLevelParallelTime1(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, metamodelica::Array<metamodelica::Real>) -> Result<metamodelica::Array<metamodelica::Real>> + 'static>), workload.clone())?;
    oLevelCost = Array::fold(workload.clone(), (std::sync::Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
    Ok(oLevelCost)
}

fn getLevelParallelTime1(mut iTask: Arc<HpcOmSimCode::Task>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iThreadWorkLoad: metamodelica::Array<metamodelica::Real>) -> Result<metamodelica::Array<metamodelica::Real>> {
    let mut oThreadWorkLoad: metamodelica::Array<metamodelica::Real>;
    let mut minWorkLoad: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut taskCosts: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut threadIdx: i32 = 0;
    let mut tmpThreadWorkLoad: metamodelica::Array<metamodelica::Real> = Default::default();
    oThreadWorkLoad = (::match_deref::match_deref! { match &(iTask.clone()) {
        Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { threadIdx: None, .. } => {
            taskCosts = getLevelTaskCosts(iTask, iTaskGraphMeta)?;
            minWorkLoad = Array::fold(iThreadWorkLoad.clone(), (std::sync::Arc::new(fnptr!(realMin, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::arrayGet(iThreadWorkLoad.clone(), 1)?)?;
            threadIdx = List::position(minWorkLoad, Arc::new(iThreadWorkLoad.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()))?;
            tmpThreadWorkLoad = metamodelica::arrayUpdate(iThreadWorkLoad.clone(), threadIdx, minWorkLoad + taskCosts)?;
            tmpThreadWorkLoad.clone()
        },
        Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { threadIdx: Some(__esc_threadIdx), .. } => {
            threadIdx = (*__esc_threadIdx).clone();
            taskCosts = getLevelTaskCosts(iTask, iTaskGraphMeta)?;
            tmpThreadWorkLoad = metamodelica::arrayUpdate(iThreadWorkLoad.clone(), threadIdx.clone(), metamodelica::arrayGet(iThreadWorkLoad.clone(), threadIdx.clone())? + taskCosts)?;
            tmpThreadWorkLoad.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oThreadWorkLoad)
}

fn getTasksOfTaskList(mut iTaskList: HpcOmSimCode::TaskList) -> Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> {
    let mut oTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oTasks = (match iTaskList {
        HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: mut __esc_tasks } => {
            tasks = __esc_tasks.clone();
            tasks.clone()
        },
        HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: mut __esc_tasks, .. } => {
            tasks = __esc_tasks.clone();
            tasks.clone()
        },
        _ => {
            metamodelica::print((literal!("getTasksOfTaskList failed! Unsupported task list.\n")).clone());
            metamodelica::nil()
        },
    });
    oTasks
}

fn getLevelListTaskCosts(mut iTaskList: HpcOmSimCode::TaskList, mut iMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<Arc<metamodelica::List<metamodelica::Real>>> {
    let mut costsOut: Arc<metamodelica::List<metamodelica::Real>>;
    let mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    tasks = getTasksOfTaskList(iTaskList);
    costsOut = List::map1(tasks, (std::sync::Arc::new(getLevelTaskCosts) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> + 'static>), iMeta)?;
    Ok(costsOut)
}

fn getLevelTaskCosts(mut levelTask: Arc<HpcOmSimCode::Task>, mut iMeta: HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> {
    let mut costsOut: metamodelica::Real;
    costsOut = (::match_deref::match_deref! { match &(levelTask) {
        Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc, .. } => {
            let mut nodeCosts: Arc<metamodelica::List<metamodelica::Real>>;
            let mut costs: metamodelica::Real;
            nodeCosts = List::map1(nodeIdc.clone(), (std::sync::Arc::new(HpcOmTaskGraph::getExeCostReqCycles) as std::sync::Arc<dyn ::std::ops::Fn(i32, HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> + 'static>), iMeta)?;
            costs = List::fold(nodeCosts.clone(), (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
            costs.clone()
        },
        _ => {
            metamodelica::print((literal!("getLevelTaskCosts failed!\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(costsOut)
}

pub(crate) fn predictExecutionTime(mut scheduleIn: Arc<HpcOmSimCode::Schedule>, mut cpCostsOption: Option<metamodelica::Real>, mut numProc: i32, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Real, metamodelica::Real, metamodelica::Real, metamodelica::Real)> {
    let mut serialTimeOut: metamodelica::Real;
    let mut parallelTimeOut: metamodelica::Real;
    let mut speedUpOut: metamodelica::Real;
    let mut speedUpMaxOut: metamodelica::Real;
    let mut parTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut serTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut speedUp: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut speedUpMax: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut helper: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    if intNe(metamodelica::arrayLength(taskGraphIn.clone()), 0) {
        serTime = getSerialExecutionTime(taskGraphMetaIn.clone())?;
        (_, parTime) = getFinishingTimesForSchedule(scheduleIn, numProc, taskGraphIn.clone(), taskGraphMetaIn)?;
        speedUp = serTime / parTime;
        helper = Util::getOptionOrDefault(cpCostsOption, (metamodelica::OrderedFloat(-1.0_f64)) * (serTime));
        speedUpMax = realDiv(serTime, helper);
    }
    serialTimeOut = serTime;
    parallelTimeOut = parTime;
    speedUpOut = speedUp;
    speedUpMaxOut = speedUpMax;
    Ok((serialTimeOut, parallelTimeOut, speedUpOut, speedUpMaxOut))
}

fn printPredictedExeTimeInfo(mut serTime: metamodelica::Real, mut parTime: metamodelica::Real, mut speedUp: metamodelica::Real, mut speedUpMax: metamodelica::Real, mut numProc: i32) -> Result<()> {
    let () = 'mc: {
        let __mc_input = speedUpMax;
        if let Ok(__v) = (|| -> Result<_> {
            let __rlit_0 = __mc_input.clone() else { bail!("nomatch") };
            if !(__rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64))) { bail!("guard") }
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (speedUpMax == metamodelica::OrderedFloat(-1.0_f64)) else { bail!("pattern mismatch") };
            if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The predicted SpeedUp with ")); __mm_s.push_str(&*intString(numProc)); __mm_s.push_str(&*literal!(" processors is ")); __mm_s.push_str(&*System::snprintff((literal!("%.2f")).clone(), 25, speedUp)?); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone());
            }
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if Flags::isSet(Flags::HPCOM_DUMP.clone())? {
                if speedUp > speedUpMax {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Something is weird. The predicted SpeedUp is ")); __mm_s.push_str(&*System::snprintff((literal!("%.2f")).clone(), 25, speedUp)?); __mm_s.push_str(&*literal!(" and the theoretical maximum speedUp is ")); __mm_s.push_str(&*System::snprintff((literal!("%.2f")).clone(), 25, speedUpMax)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                } else if speedUp <= speedUpMax {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The predicted SpeedUp with ")); __mm_s.push_str(&*intString(numProc)); __mm_s.push_str(&*literal!(" processors is: ")); __mm_s.push_str(&*System::snprintff((literal!("%.2f")).clone(), 25, speedUp)?); __mm_s.push_str(&*literal!(" With a theoretical maximmum speedUp of: ")); __mm_s.push_str(&*System::snprintff((literal!("%.2f")).clone(), 25, speedUpMax)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
            }
            Ok(())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

pub(crate) fn getSerialExecutionTime(mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta) -> Result<metamodelica::Real> {
    let mut serialTimeOut: metamodelica::Real;
    let mut odeComps: Arc<metamodelica::List<i32>>;
    let mut exeCostsReal: Arc<metamodelica::List<metamodelica::Real>>;
    let mut exeCosts1: metamodelica::Array<metamodelica::Real>;
    let mut inComps: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut exeCosts: metamodelica::Array<(i32, metamodelica::Real)>;
    let HpcOmTaskGraph::TASKGRAPHMETA { exeCosts: __pa0, inComps: __pa1, .. } = (taskGraphMetaIn) else { bail!("pattern mismatch") };
    exeCosts = __pa0.clone();
    inComps = __pa1.clone();
    odeComps = Array::fold(inComps.clone(), Arc::new(fnptr!(listAppend, Arc<metamodelica::List<i32>>, _)), metamodelica::nil())?;
    exeCosts1 = Array::map(exeCosts.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)))?;
    exeCostsReal = List::map1(odeComps, (std::sync::Arc::new(Array::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), exeCosts1.clone())?;
    serialTimeOut = List::fold(exeCostsReal, (std::sync::Arc::new(fnptr!(realAdd, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
    Ok(serialTimeOut)
}

fn getFinishingTimesForSchedule(mut scheduleIn: Arc<HpcOmSimCode::Schedule>, mut numProc: i32, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta) -> Result<(Arc<HpcOmSimCode::Schedule>, metamodelica::Real)> {
    let mut scheduleOut: Arc<HpcOmSimCode::Schedule>;
    let mut finishingTime: metamodelica::Real;
    (scheduleOut, finishingTime) = 'mc: {
        let __mc_input = scheduleIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks, outgoingDepTasks, allCalcTasks, .. } => {
                    let mut finTime: metamodelica::Real;
                    let mut taskIdcs: metamodelica::Array<i32>;
                    let mut finTimes: metamodelica::Array<metamodelica::Real>;
                    let mut taskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut checkedTasks: metamodelica::Array<Arc<HpcOmSimCode::Task>>;
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    taskIdcs = arrayCreate(metamodelica::arrayLength(threadTasks.clone()), 1);
                    taskGraphT = AdjacencyMatrix::transposeAdjacencyMatrix(taskGraphIn.clone(), metamodelica::arrayLength(taskGraphIn.clone()))?;
                    checkedTasks = arrayCreate(metamodelica::arrayLength(taskGraphIn.clone()), openmodelica_simcode_types::HpcOmSimCode::Task::interned_TASKEMPTY());
                    computeTimeFinished(threadTasks.clone(), taskIdcs.clone(), 1, checkedTasks.clone(), taskGraphIn.clone(), taskGraphT.clone(), taskGraphMetaIn.clone(), numProc, metamodelica::nil())?;
                    finTimes = Array::map(threadTasks.clone(), (std::sync::Arc::new(getTimeFinishedOfLastTask) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<metamodelica::Real> + 'static>))?;
                    finTime = Array::fold(finTimes.clone(), (std::sync::Arc::new(fnptr!(realMax, metamodelica::Real, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))?;
                    schedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: metamodelica::nil(), allCalcTasks: allCalcTasks.clone() });
                    Ok((schedule.clone(), finTime.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: _, useFixedAssignments: _ } => {
                    let mut finTime: metamodelica::Real;
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    schedule = scheduleIn.clone();
                    finTime = metamodelica::OrderedFloat(0.0_f64);
                    Ok((schedule.clone(), finTime.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { .. } => {
                    let mut finTime: metamodelica::Real;
                    let mut schedule: Arc<HpcOmSimCode::Schedule>;
                    schedule = scheduleIn.clone();
                    finTime = metamodelica::OrderedFloat(-1.0_f64);
                    Ok((schedule.clone(), finTime.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("getFinishingTimesForSchedule failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((scheduleOut, finishingTime))
}

fn getTimeFinishedOfLastTask(mut threadTasksIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<metamodelica::Real> {
    let mut finTimeOut: metamodelica::Real;
    finTimeOut = 'mc: {
        let __mc_input = threadTasksIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut lastTask: Arc<HpcOmSimCode::Task>;
                    let mut finTime: metamodelica::Real;
                    lastTask = List::last(threadTasksIn.clone())?;
                    finTime = getTimeFinished(lastTask.clone());
                    Ok(finTime.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::OrderedFloat(-1.0_f64))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(finTimeOut)
}

fn computeTimeFinished(mut threadTasksIn: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut taskIdcsIn: metamodelica::Array<i32>, mut threadIdxIn: i32, mut checkedTasksIn: metamodelica::Array<Arc<HpcOmSimCode::Task>>, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta, mut numProc: i32, mut closedThreadsIn: Arc<metamodelica::List<i32>>) -> Result<()> {
    let mut threadIdx: i32 = threadIdxIn;
    let mut closedThreads: Arc<metamodelica::List<i32>> = closedThreadsIn.clone();
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = threadTasksIn.clone();
    while !((closedThreads.clone().len() as i32) == numProc) {
        (threadIdx, closedThreads) = computeTimeFinished1(threadTasks.clone(), taskIdcsIn.clone(), threadIdx, checkedTasksIn.clone(), taskGraphIn.clone(), taskGraphTIn.clone(), taskGraphMetaIn.clone(), numProc, closedThreads.clone())?;
    }
    Ok(())
}

fn computeTimeFinished1(mut threadTasksIn: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut taskIdcsIn: metamodelica::Array<i32>, mut threadIdxIn: i32, mut checkedTasksIn: metamodelica::Array<Arc<HpcOmSimCode::Task>>, mut taskGraphIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta, mut numProc: i32, mut closedThreadsIn: Arc<metamodelica::List<i32>>) -> Result<(i32, Arc<metamodelica::List<i32>>)> {
    let mut threadIdxOut: i32;
    let mut closedThreadsOut: Arc<metamodelica::List<i32>>;
    (threadIdxOut, closedThreadsOut) = 'mc: {
        let __mc_input = closedThreadsIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut taskIdx: i32;
                    let mut nextThreadIdx: i32;
                    let mut nextTaskIdx: i32;
                    let mut task: Arc<HpcOmSimCode::Task>;
                    let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
                    let true = (threadIdxIn <= metamodelica::arrayLength(taskIdcsIn.clone())) else { bail!("pattern mismatch") };
                    taskIdx = metamodelica::arrayGet(taskIdcsIn.clone(), threadIdxIn)?;
                    thread = metamodelica::arrayGet(threadTasksIn.clone(), threadIdxIn)?;
                    let true = (taskIdx.clone() <= (thread.clone().len() as i32)) else { bail!("pattern mismatch") };
                    task = (thread.clone()).get(taskIdx.clone())?;
                    (_, _, nextTaskIdx) = updateFinishingTime(task.clone(), taskIdx.clone(), threadIdxIn, threadTasksIn.clone(), checkedTasksIn.clone(), taskGraphTIn.clone(), taskGraphMetaIn.clone())?;
                    metamodelica::arrayUpdate(taskIdcsIn.clone(), threadIdxIn, nextTaskIdx.clone())?;
                    nextThreadIdx = getNextThreadIdx(threadIdxIn, closedThreadsIn.clone(), numProc)?;
                    Ok((nextThreadIdx.clone(), closedThreadsIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut nextThreadIdx: i32;
                    let true = (threadIdxIn > metamodelica::arrayLength(taskIdcsIn.clone())) else { bail!("pattern mismatch") };
                    nextThreadIdx = if (intGe(threadIdxIn, numProc)) {1} else {threadIdxIn + 1};
                    Ok((nextThreadIdx.clone(), closedThreadsIn.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut taskIdx: i32;
                    let mut nextThreadIdx: i32;
                    let mut closedThreads1: Arc<metamodelica::List<i32>>;
                    let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
                    let true = (threadIdxIn <= metamodelica::arrayLength(taskIdcsIn.clone())) else { bail!("pattern mismatch") };
                    taskIdx = metamodelica::arrayGet(taskIdcsIn.clone(), threadIdxIn)?;
                    thread = metamodelica::arrayGet(threadTasksIn.clone(), threadIdxIn)?;
                    let true = (taskIdx.clone() > (thread.clone().len() as i32)) else { bail!("pattern mismatch") };
                    nextThreadIdx = if (intGe(threadIdxIn, numProc)) {1} else {threadIdxIn + 1};
                    closedThreads1 = metamodelica::cons(threadIdxIn, closedThreadsIn.clone());
                    closedThreads1 = List::unique(closedThreads1.clone());
                    Ok((nextThreadIdx.clone(), closedThreads1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("computeTimeFinished failed!\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((threadIdxOut, closedThreadsOut))
}

fn getNextThreadIdx(mut threadId: i32, mut closedThreads: Arc<metamodelica::List<i32>>, mut numThreads: i32) -> Result<i32> {
    '__tco: loop {
        let mut isLastThread: bool;
        let mut isClosed: bool;
        let mut nextThread: i32;
        isLastThread = intEq(threadId, numThreads);
        nextThread = if (isLastThread) {1} else {threadId + 1};
        isClosed = List::isMemberOnTrue(nextThread, closedThreads.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        if (isClosed) {{ (threadId, closedThreads, numThreads) = (nextThread, closedThreads, numThreads); continue '__tco; }} else {return Ok(nextThread)}
    }
}

fn updateFinishingTime(mut taskIn: Arc<HpcOmSimCode::Task>, mut taskIdxIn: i32, mut threadIdxIn: i32, mut threadTasksIn: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut checkedTasksIn: metamodelica::Array<Arc<HpcOmSimCode::Task>>, mut taskGraphTIn: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskGraphMetaIn: HpcOmTaskGraph::TaskGraphMeta) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, metamodelica::Array<Arc<HpcOmSimCode::Task>>, i32)> {
    let mut threadTasksOut: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut checkedTasksOut: metamodelica::Array<Arc<HpcOmSimCode::Task>>;
    let mut taskIdxOut: i32;
    (threadTasksOut, checkedTasksOut, taskIdxOut) = (::match_deref::match_deref! { match &(taskIn) {
        Deref @ HpcOmSimCode::Task::CALCTASK { index: taskID, .. } => {
            let mut isComputable: bool;
            let mut taskIdxNew: i32;
            let mut parentLst: Arc<metamodelica::List<i32>>;
            let mut latestTask: Arc<HpcOmSimCode::Task>;
            let mut checkedTasks: metamodelica::Array<Arc<HpcOmSimCode::Task>>;
            let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
            parentLst = metamodelica::arrayGet(taskGraphTIn.clone(), taskID.clone())?;
            (parentLst, latestTask) = List::fold1(parentLst.clone(), (std::sync::Arc::new(updateFinishingTime1) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<Arc<HpcOmSimCode::Task>>, (Arc<metamodelica::List<i32>>, Arc<HpcOmSimCode::Task>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<HpcOmSimCode::Task>)> + 'static>), checkedTasksIn.clone(), (metamodelica::nil(), openmodelica_simcode_types::HpcOmSimCode::Task::interned_TASKEMPTY()))?;
            isComputable = parentLst.clone().is_empty();
            taskIdxNew = if (isComputable.clone()) {taskIdxIn + 1} else {taskIdxIn};
            (threadTasks, checkedTasks) = if (isComputable.clone()) {computeFinishingTimeForOneTask((threadTasksIn.clone(), checkedTasksIn.clone(), taskIdxIn, threadIdxIn, latestTask.clone(), taskGraphMetaIn))?} else {(threadTasksIn.clone(), checkedTasksIn.clone())};
            (threadTasks.clone(), checkedTasks.clone(), taskIdxNew.clone())
        },
        Deref @ HpcOmSimCode::Task::DEPTASK { .. } => {
            let mut taskIdxNew: i32;
            taskIdxNew = taskIdxIn + 1;
            (threadTasksIn.clone(), checkedTasksIn.clone(), taskIdxNew.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((threadTasksOut, checkedTasksOut, taskIdxOut))
}

fn updateFinishingTime1(mut parentIdx: i32, mut checkedTaskIn: metamodelica::Array<Arc<HpcOmSimCode::Task>>, mut tplIn: (Arc<metamodelica::List<i32>>, Arc<HpcOmSimCode::Task>)) -> Result<(Arc<metamodelica::List<i32>>, Arc<HpcOmSimCode::Task>)> {
    let mut tplOut: (Arc<metamodelica::List<i32>>, Arc<HpcOmSimCode::Task>);
    let mut isCalc: bool;
    let mut finishingTime: metamodelica::Real;
    let mut finishingTimeIn: metamodelica::Real;
    let mut parentLst: Arc<metamodelica::List<i32>>;
    let mut parentLstIn: Arc<metamodelica::List<i32>>;
    let mut task: Arc<HpcOmSimCode::Task>;
    let mut taskIn: Arc<HpcOmSimCode::Task>;
    (parentLstIn, taskIn) = tplIn;
    finishingTimeIn = getTimeFinished(taskIn.clone());
    task = metamodelica::arrayGet(checkedTaskIn.clone(), parentIdx)?;
    isCalc = isCalcTask(task.clone());
    finishingTime = if (isCalc) {getTimeFinished(task.clone())} else {metamodelica::OrderedFloat(-1.0_f64)};
    task = if (realGt(finishingTime, finishingTimeIn)) {task} else {taskIn};
    parentLst = if (isCalc) {parentLstIn} else {metamodelica::cons(parentIdx, parentLstIn)};
    tplOut = (parentLst, task);
    Ok(tplOut)
}

fn computeFinishingTimeForOneTask(mut tplIn: (metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, metamodelica::Array<Arc<HpcOmSimCode::Task>>, i32, i32, Arc<HpcOmSimCode::Task>, HpcOmTaskGraph::TaskGraphMeta)) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, metamodelica::Array<Arc<HpcOmSimCode::Task>>)> {
    let mut tplOut: (metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, metamodelica::Array<Arc<HpcOmSimCode::Task>>);
    tplOut = 'mc: {
        let __mc_input = tplIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (threadTasksIn, checkedTasksIn, taskNum, threadIdx, latestTask, taskGraphMeta) => {
                    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
                    let mut checkedTasks: metamodelica::Array<Arc<HpcOmSimCode::Task>>;
                    let mut taskIdx: i32;
                    let mut finishingTime: metamodelica::Real;
                    let mut exeCost: metamodelica::Real;
                    let mut task: Arc<HpcOmSimCode::Task>;
                    let mut preTask: Arc<HpcOmSimCode::Task>;
                    let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
                    let mut threadIdx = (*threadIdx).clone();
                    let true = (isEmptyTask(latestTask.clone())) else { bail!("pattern mismatch") };
                    thread = metamodelica::arrayGet(threadTasksIn.clone(), threadIdx.clone())?;
                    task = (thread.clone()).get(taskNum.clone())?;
                    threadIdx = getThreadId(task.clone());
                    preTask = getPredecessorCalcTask(thread.clone(), taskNum.clone())?;
                    finishingTime = getTimeFinished(preTask.clone());
                    taskIdx = getTaskIdx(task.clone());
                    (_, exeCost) = HpcOmTaskGraph::getExeCost(taskIdx.clone(), taskGraphMeta.clone())?;
                    finishingTime = finishingTime.clone() + exeCost.clone();
                    task = updateTimeFinished(task.clone(), finishingTime.clone())?;
                    thread = List::replaceAt(task.clone(), taskNum.clone(), thread.clone())?;
                    threadTasks = metamodelica::arrayUpdate(threadTasksIn.clone(), threadIdx.clone(), thread.clone())?;
                    checkedTasks = metamodelica::arrayUpdate(checkedTasksIn.clone(), taskIdx.clone(), task.clone())?;
                    Ok((threadTasks.clone(), checkedTasks.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (threadTasksIn, checkedTasksIn, taskNum, threadIdx, latestTask, taskGraphMeta) => {
                    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
                    let mut checkedTasks: metamodelica::Array<Arc<HpcOmSimCode::Task>>;
                    let mut taskIdx: i32;
                    let mut taskIdxLatest: i32;
                    let mut threadIdxLatest: i32;
                    let mut commCost: metamodelica::Real;
                    let mut finishingTime: metamodelica::Real;
                    let mut finishingTime1: metamodelica::Real;
                    let mut finishingTimeComm: metamodelica::Real;
                    let mut exeCost: metamodelica::Real;
                    let mut task: Arc<HpcOmSimCode::Task>;
                    let mut preTask: Arc<HpcOmSimCode::Task>;
                    let mut thread: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
                    let false = (isEmptyTask(latestTask.clone())) else { bail!("pattern mismatch") };
                    finishingTime = getTimeFinished(latestTask.clone());
                    threadIdxLatest = getThreadId(latestTask.clone());
                    taskIdxLatest = getTaskIdx(latestTask.clone());
                    thread = metamodelica::arrayGet(threadTasksIn.clone(), threadIdx.clone())?;
                    task = (thread.clone()).get(taskNum.clone())?;
                    taskIdx = getTaskIdx(task.clone());
                    commCost = HpcOmTaskGraph::getCommCostTimeBetweenNodes(taskIdxLatest.clone(), taskIdx.clone(), taskGraphMeta.clone())?;
                    (_, exeCost) = HpcOmTaskGraph::getExeCost(taskIdx.clone(), taskGraphMeta.clone())?;
                    finishingTime = finishingTime.clone() + exeCost.clone();
                    finishingTimeComm = finishingTime.clone() + commCost.clone();
                    finishingTime = if (intEq(threadIdxLatest.clone(), threadIdx.clone())) {finishingTime.clone()} else {finishingTimeComm.clone()};
                    preTask = getPredecessorCalcTask(thread.clone(), taskNum.clone())?;
                    finishingTime1 = getTimeFinished(preTask.clone());
                    finishingTime1 = finishingTime1.clone() + exeCost.clone();
                    finishingTime = realMax(finishingTime.clone(), finishingTime1.clone());
                    task = updateTimeFinished(task.clone(), finishingTime.clone())?;
                    thread = List::replaceAt(task.clone(), taskNum.clone(), thread.clone())?;
                    threadTasks = metamodelica::arrayUpdate(threadTasksIn.clone(), threadIdx.clone(), thread.clone())?;
                    checkedTasks = metamodelica::arrayUpdate(checkedTasksIn.clone(), taskIdx.clone(), task.clone())?;
                    Ok((threadTasks.clone(), checkedTasks.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(tplOut)
}

fn getPredecessorCalcTask(mut threadIn: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut indexIn: i32) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut taskOut: Arc<HpcOmSimCode::Task>;
    taskOut = 'mc: {
        let __mc_input = indexIn;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (indexIn == 1) else { bail!("pattern mismatch") };
            Ok(openmodelica_simcode_types::HpcOmSimCode::Task::interned_TASKEMPTY())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut isCalc: bool;
            let mut index: i32;
            let mut preTask: Arc<HpcOmSimCode::Task>;
            let true = (indexIn >= 2) else { bail!("pattern mismatch") };
            index = indexIn - 1;
            preTask = (threadIn.clone()).get(index.clone())?;
            isCalc = isCalcTask(preTask.clone());
            preTask = if (boolNot(isCalc.clone())) {getPredecessorCalcTask(threadIn.clone(), index.clone())?} else {preTask.clone()};
            Ok(preTask.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(taskOut)
}

fn updateTimeFinished(mut taskIn: Arc<HpcOmSimCode::Task>, mut timeFinishedIn: metamodelica::Real) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut taskOut: Arc<HpcOmSimCode::Task>;
    let mut weighting: i32;
    let mut index: i32;
    let mut calcTime: metamodelica::Real;
    let mut timeFinished: metamodelica::Real;
    let mut threadIdx: i32;
    let mut eqIdc: Arc<metamodelica::List<i32>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(taskIn) {
        Deref @ HpcOmSimCode::Task::CALCTASK { weighting: __pa0, index: __pa1, calcTime: __pa2, timeFinished: __pa3, threadIdx: __pa4, eqIdc: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    weighting = __pa0.clone();
    index = __pa1.clone();
    calcTime = __pa2.clone();
    timeFinished = __pa3.clone();
    threadIdx = __pa4.clone();
    eqIdc = __pa5.clone();
    taskOut = Arc::new(HpcOmSimCode::Task::CALCTASK { weighting: weighting, index: index, calcTime: calcTime, timeFinished: timeFinishedIn, threadIdx: threadIdx, eqIdc: eqIdc });
    Ok(taskOut)
}

fn getTimeFinished(mut taskIn: Arc<HpcOmSimCode::Task>) -> metamodelica::Real {
    let mut finishingTime: metamodelica::Real;
    finishingTime = (::match_deref::match_deref! { match &(taskIn) {
        Deref @ HpcOmSimCode::Task::CALCTASK { timeFinished: fTime, .. } => {
            fTime.clone()
        },
        Deref @ HpcOmSimCode::Task::TASKEMPTY { .. } => {
            metamodelica::OrderedFloat(0.0_f64)
        },
        _ => {
            metamodelica::OrderedFloat(-1.0_f64)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    finishingTime
}

fn getThreadId(mut taskIn: Arc<HpcOmSimCode::Task>) -> i32 {
    let mut threadId: i32;
    threadId = (::match_deref::match_deref! { match &(taskIn) {
        Deref @ HpcOmSimCode::Task::CALCTASK { threadIdx, .. } => {
            threadIdx.clone()
        },
        _ => {
            -1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    threadId
}

fn getTaskIdx(mut taskIn: Arc<HpcOmSimCode::Task>) -> i32 {
    let mut idx: i32;
    idx = (::match_deref::match_deref! { match &(taskIn) {
        Deref @ HpcOmSimCode::Task::CALCTASK { index: taskIdx, .. } => {
            taskIdx.clone()
        },
        _ => {
            -1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    idx
}

fn getTaskTypeString(mut iTask: Arc<HpcOmSimCode::Task>) -> ArcStr {
    let mut oTypeString: ArcStr;
    oTypeString = ((::match_deref::match_deref! { match &(iTask) {
        Deref @ HpcOmSimCode::Task::SCHEDULED_TASK { .. } => literal!("Scheduled task"),
        Deref @ HpcOmSimCode::Task::CALCTASK { .. } => literal!("Calctask"),
        Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { .. } => literal!("Calctask level"),
        Deref @ HpcOmSimCode::Task::DEPTASK { .. } => literal!("Deptask"),
        Deref @ HpcOmSimCode::Task::PREFETCHTASK { .. } => literal!("Prefetch task"),
        Deref @ HpcOmSimCode::Task::TASKEMPTY { .. } => literal!("Empty task"),
        _ => literal!("Unknown"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    oTypeString
}

fn isCalcTask(mut taskIn: Arc<HpcOmSimCode::Task>) -> bool {
    let mut isCalc: bool;
    isCalc = (::match_deref::match_deref! { match &(taskIn) {
        Deref @ HpcOmSimCode::Task::CALCTASK { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isCalc
}

fn isEmptyTask(mut taskIn: Arc<HpcOmSimCode::Task>) -> bool {
    let mut isEmpty: bool;
    isEmpty = (::match_deref::match_deref! { match &(taskIn) {
        Deref @ HpcOmSimCode::Task::TASKEMPTY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub fn convertFixedLevelScheduleToLevelThreadLists(mut iSchedule: Arc<HpcOmSimCode::Schedule>, mut iNumOfThreads: i32) -> Result<Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>>> {
    let mut oLevelThreadLists: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>>;
    let mut tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut tmpLevelThreadLists: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>> = metamodelica::nil();
    oLevelThreadLists = (::match_deref::match_deref! { match &(iSchedule) {
        Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: __esc_tasksOfLevels, useFixedAssignments: true } => {
            tasksOfLevels = (*__esc_tasksOfLevels).clone();
            tmpLevelThreadLists = List::map(tasksOfLevels.clone(), (std::sync::Arc::new({ let __pe_b1 = iNumOfThreads; move |__pe_a0| convertFixedLevelScheduleToLevelThreadLists0(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> + 'static>))?;
            tmpLevelThreadLists
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oLevelThreadLists)
}

fn convertFixedLevelScheduleToLevelThreadLists0(mut iTasksOfLevel: HpcOmSimCode::TaskList, mut iNumOfThreads: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> {
    let mut oLevelThreadLists: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut task: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut threadIdx: i32;
    let mut tmpLevelThreadLists: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    tasks = getTasksOfTaskList(iTasksOfLevel);
    tmpLevelThreadLists = arrayCreate(iNumOfThreads, metamodelica::nil());
    for mut task in &*tasks.reverse() {
        let mut task = task.clone();
        let __pa0 = ::match_deref::match_deref! { match &(task.clone()) {
            Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { threadIdx: Some(__pa0), .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        threadIdx = __pa0.clone();
        tmpLevelThreadLists = metamodelica::arrayUpdate(tmpLevelThreadLists.clone(), threadIdx, metamodelica::cons(task.clone(), metamodelica::arrayGet(tmpLevelThreadLists.clone(), threadIdx)?))?;
    }
    oLevelThreadLists = tmpLevelThreadLists.clone();
    Ok(oLevelThreadLists)
}

pub fn convertFixedLevelScheduleToTaskLists(mut iOdeSchedule: Arc<HpcOmSimCode::Schedule>, mut iDaeSchedule: Arc<HpcOmSimCode::Schedule>, mut iZeroFuncSchedule: Arc<HpcOmSimCode::Schedule>, mut iNumOfThreads: i32) -> Result<metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>> {
    let mut oThreadLevelTasks: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>;
    let mut tasksOfLevelsOde: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut tasksOfLevelsDae: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut tasksOfLevelsZeroFunc: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut tmpThreadLevelTasksDae: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>> = metamodelica::nil();
    let mut tmpThreadLevelTasksOde: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>> = metamodelica::nil();
    let mut tmpThreadLevelTasksZeroFunc: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>> = metamodelica::nil();
    let mut tmpResultLists: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)> = Default::default();
    oThreadLevelTasks = (::match_deref::match_deref! { match &((iOdeSchedule, iDaeSchedule, iZeroFuncSchedule)) {
        (Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: __esc_tasksOfLevelsOde, useFixedAssignments: true }, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: __esc_tasksOfLevelsDae, useFixedAssignments: true }, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: __esc_tasksOfLevelsZeroFunc, useFixedAssignments: true }) => {
            tasksOfLevelsOde = (*__esc_tasksOfLevelsOde).clone();
            tasksOfLevelsDae = (*__esc_tasksOfLevelsDae).clone();
            tasksOfLevelsZeroFunc = (*__esc_tasksOfLevelsZeroFunc).clone();
            tmpResultLists = arrayCreate(iNumOfThreads, (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()));
            tmpThreadLevelTasksOde = List::map1(tasksOfLevelsOde.clone(), (std::sync::Arc::new(convertFixedLevelScheduleToTaskListsForLevel) as std::sync::Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList, i32) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> + 'static>), iNumOfThreads)?;
            tmpThreadLevelTasksDae = List::map1(tasksOfLevelsDae.clone(), (std::sync::Arc::new(convertFixedLevelScheduleToTaskListsForLevel) as std::sync::Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList, i32) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> + 'static>), iNumOfThreads)?;
            tmpThreadLevelTasksZeroFunc = List::map1(tasksOfLevelsZeroFunc.clone(), (std::sync::Arc::new(convertFixedLevelScheduleToTaskListsForLevel) as std::sync::Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList, i32) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> + 'static>), iNumOfThreads)?;
            tmpResultLists = List::fold(tmpThreadLevelTasksOde, (std::sync::Arc::new({ let __pe_b1 = 1; let __pe_b2 = 0; move |__pe_a0, __pe_a3| Ok(convertFixedLevelScheduleToTaskLists1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>) -> Result<metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>> + 'static>), tmpResultLists.clone())?;
            tmpResultLists = List::fold(tmpThreadLevelTasksDae, (std::sync::Arc::new({ let __pe_b1 = 1; let __pe_b2 = 1; move |__pe_a0, __pe_a3| Ok(convertFixedLevelScheduleToTaskLists1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>) -> Result<metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>> + 'static>), tmpResultLists.clone())?;
            tmpResultLists = List::fold(tmpThreadLevelTasksZeroFunc, (std::sync::Arc::new({ let __pe_b1 = 1; let __pe_b2 = 2; move |__pe_a0, __pe_a3| Ok(convertFixedLevelScheduleToTaskLists1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>) -> Result<metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>> + 'static>), tmpResultLists.clone())?;
            tmpResultLists = revertTaskLists(1, tmpResultLists.clone());
            tmpResultLists.clone()
        },
        _ => {
            tmpResultLists = arrayCreate(iNumOfThreads, (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()));
            tmpResultLists.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oThreadLevelTasks)
}

fn convertFixedLevelScheduleToTaskLists1(mut iLevelTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut iCurrentThreadIdx: i32, mut iModifiedSystemIdx: i32, mut iResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>) -> metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)> {
    let mut oResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>;
    let mut tmpResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)> = Default::default();
    let mut entryOde: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
    let mut entryDae: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
    let mut entryZeroFunc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
    oResultList = 'mc: {
        let __mc_input = iResultList.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut entryDae: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = entryDae.clone();
            let mut entryOde: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = entryOde.clone();
            let mut entryZeroFunc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = entryZeroFunc.clone();
            let mut tmpResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)> = tmpResultList.clone();
            let true = (intLe(iCurrentThreadIdx, metamodelica::arrayLength(iLevelTasks.clone()))) else { bail!("pattern mismatch") };
            (entryOde, entryDae, entryZeroFunc) = metamodelica::arrayGet(iResultList.clone(), iCurrentThreadIdx)?;
            if intEq(iModifiedSystemIdx, 0) {
                entryOde = metamodelica::cons(metamodelica::arrayGet(iLevelTasks.clone(), iCurrentThreadIdx)?, entryOde.clone());
            } else {
                if intEq(iModifiedSystemIdx, 1) {
                    entryDae = metamodelica::cons(metamodelica::arrayGet(iLevelTasks.clone(), iCurrentThreadIdx)?, entryDae.clone());
                } else {
                    entryZeroFunc = metamodelica::cons(metamodelica::arrayGet(iLevelTasks.clone(), iCurrentThreadIdx)?, entryZeroFunc.clone());
                }
            }
            tmpResultList = metamodelica::arrayUpdate(iResultList.clone(), iCurrentThreadIdx, (entryOde.clone(), entryDae.clone(), entryZeroFunc.clone()))?;
            tmpResultList = convertFixedLevelScheduleToTaskLists1(iLevelTasks.clone(), iCurrentThreadIdx + 1, iModifiedSystemIdx, tmpResultList.clone());
            Ok((tmpResultList.clone(), entryDae.clone(), entryOde.clone(), entryZeroFunc.clone(), tmpResultList.clone()))
        })() { entryDae = __wb0; entryOde = __wb1; entryZeroFunc = __wb2; tmpResultList = __wb3; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iResultList.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oResultList
}

fn revertTaskLists(mut iCurrentArrayIdx: i32, mut iResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>) -> metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)> {
    let mut oResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>;
    let mut entryOde: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
    let mut entryDae: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
    let mut entryZeroFunc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
    let mut tmpResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)> = Default::default();
    oResultList = 'mc: {
        let __mc_input = iResultList.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut entryDae: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = entryDae.clone();
            let mut entryOde: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = entryOde.clone();
            let mut entryZeroFunc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = entryZeroFunc.clone();
            let mut tmpResultList: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)> = tmpResultList.clone();
            let true = (intLe(iCurrentArrayIdx, metamodelica::arrayLength(iResultList.clone()))) else { bail!("pattern mismatch") };
            (entryOde, entryDae, entryZeroFunc) = metamodelica::arrayGet(iResultList.clone(), iCurrentArrayIdx)?;
            entryOde = entryOde.clone().reverse();
            entryDae = entryDae.clone().reverse();
            entryZeroFunc = entryZeroFunc.clone().reverse();
            tmpResultList = metamodelica::arrayUpdate(iResultList.clone(), iCurrentArrayIdx, (entryOde.clone(), entryDae.clone(), entryZeroFunc.clone()))?;
            tmpResultList = revertTaskLists(iCurrentArrayIdx + 1, tmpResultList.clone());
            Ok((tmpResultList.clone(), entryDae.clone(), entryOde.clone(), entryZeroFunc.clone(), tmpResultList.clone()))
        })() { entryDae = __wb0; entryOde = __wb1; entryZeroFunc = __wb2; tmpResultList = __wb3; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iResultList.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oResultList
}

fn revertTaskList(mut iCurrentArrayIdx: i32, mut iResultList: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut oResultList: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut entry: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut tmpResultList: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = Default::default();
    oResultList = 'mc: {
        let __mc_input = iResultList.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut entry: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = entry.clone();
            let mut tmpResultList: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = tmpResultList.clone();
            let true = (intLe(iCurrentArrayIdx, metamodelica::arrayLength(iResultList.clone()))) else { bail!("pattern mismatch") };
            entry = metamodelica::arrayGet(iResultList.clone(), iCurrentArrayIdx)?;
            entry = entry.clone().reverse();
            tmpResultList = metamodelica::arrayUpdate(iResultList.clone(), iCurrentArrayIdx, entry.clone())?;
            Ok((tmpResultList.clone(), entry.clone(), tmpResultList.clone()))
        })() { entry = __wb0; tmpResultList = __wb1; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(iResultList.clone())
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oResultList
}

//----------------
//  LockIdSetter
//----------------
fn setScheduleLockIds(mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut allThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut tmpFoldArray: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut newAllThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut scheduledTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut lockIds: metamodelica::Array<Arc<metamodelica::List<(i32, i32)>>>;
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut newOutgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)>;
    let mut newTuple: (i32, i32);
    let mut sourceTask: Arc<HpcOmSimCode::Task>;
    let mut targetTask: Arc<HpcOmSimCode::Task>;
    let mut iterTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut counter: i32;
    let mut id: i32;
    let mut sourceTaskId: i32;
    let mut targetTaskId: i32;
    let mut outgoing: bool;
    let mut communicationInfo: HpcOmSimCode::CommunicationInfo;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(iSchedule) {
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __pa0, outgoingDepTasks: __pa1, scheduledTasks: __pa2, allCalcTasks: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    allThreadTasks = __pa0.clone();
    outgoingDepTasks = __pa1.clone();
    scheduledTasks = __pa2.clone();
    allCalcTasks = __pa3.clone();
    lockIds = arrayCreate(metamodelica::arrayLength(allCalcTasks.clone()), metamodelica::nil());
    newAllThreadTasks = arrayCreate(metamodelica::arrayLength(allThreadTasks.clone()), metamodelica::nil());
    counter = 0;
    for mut iterTask in &*outgoingDepTasks {
        let mut iterTask = iterTask.clone();
        let (__pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(iterTask.clone()) {
            Deref @ HpcOmSimCode::Task::DEPTASK { sourceTask: __pa4, targetTask: __pa5, outgoing: __pa6, id: __pa7, communicationInfo: __pa8 } => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
            _ => bail!("pattern mismatch"),
        } };
        sourceTask = __pa4.clone();
        targetTask = __pa5.clone();
        outgoing = __pa6.clone();
        id = __pa7.clone();
        communicationInfo = __pa8.clone();
        let __pa9 = ::match_deref::match_deref! { match &(sourceTask.clone()) {
            Deref @ HpcOmSimCode::Task::CALCTASK { index: __pa9, .. } => __pa9.clone(),
            _ => bail!("pattern mismatch"),
        } };
        sourceTaskId = __pa9.clone();
        let __pa10 = ::match_deref::match_deref! { match &(targetTask.clone()) {
            Deref @ HpcOmSimCode::Task::CALCTASK { index: __pa10, .. } => __pa10.clone(),
            _ => bail!("pattern mismatch"),
        } };
        targetTaskId = __pa10.clone();
        newTuple = (targetTaskId, counter);
        metamodelica::arrayUpdate(lockIds.clone(), sourceTaskId, listAppend(metamodelica::arrayGet(lockIds.clone(), sourceTaskId)?, list![newTuple.clone()]))?;
        newOutgoingDepTasks = metamodelica::cons(Arc::new(HpcOmSimCode::Task::DEPTASK { sourceTask: sourceTask.clone(), targetTask: targetTask.clone(), outgoing: outgoing, id: counter, communicationInfo: communicationInfo.clone() }), newOutgoingDepTasks.clone());
        counter = counter + 1;
    }
    tmpFoldArray = arrayCreate(metamodelica::arrayLength(allThreadTasks.clone()), metamodelica::nil());
    (newAllThreadTasks, _) = Array::fold(allThreadTasks.clone(), (std::sync::Arc::new({ let __pe_b1 = lockIds.clone(); move |__pe_a0, __pe_a2| replaceDepTaskIdsByLockIds(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, (metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32)) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32)> + 'static>), (tmpFoldArray.clone(), 1))?;
    oSchedule = Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: newAllThreadTasks.clone(), outgoingDepTasks: newOutgoingDepTasks, scheduledTasks: scheduledTasks, allCalcTasks: allCalcTasks.clone() });
    Ok(oSchedule)
}

fn replaceDepTaskIdsByLockIds(mut inTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut lockIds: metamodelica::Array<Arc<metamodelica::List<(i32, i32)>>>, mut iAllThreadTasks: (metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32)) -> Result<(metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32)> {
    let mut oTasks: (metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, i32);
    let mut allThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut tmpList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut threadId: i32;
    (allThreadTasks, threadId) = iAllThreadTasks;
    tmpList = List::fold(inTasks, (std::sync::Arc::new({ let __pe_b1 = lockIds.clone(); move |__pe_a0, __pe_a2| replaceDepTasksInListByLockIds(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> + 'static>), metamodelica::nil())?.reverse();
    metamodelica::arrayUpdate(allThreadTasks.clone(), threadId, tmpList)?;
    oTasks = (allThreadTasks.clone(), threadId + 1);
    Ok(oTasks)
}

fn replaceDepTasksInListByLockIds(mut inTask: Arc<HpcOmSimCode::Task>, mut lockIds: metamodelica::Array<Arc<metamodelica::List<(i32, i32)>>>, mut tmpTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>) -> Result<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> {
    let mut oList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
    let mut tmpTask: Arc<HpcOmSimCode::Task>;
    tmpTask = findTaskWithLockId(lockIds.clone(), inTask)?;
    oList = metamodelica::cons(tmpTask, tmpTaskList);
    Ok(oList)
}

fn findTaskWithLockId(mut lockIds: metamodelica::Array<Arc<metamodelica::List<(i32, i32)>>>, mut iTask: Arc<HpcOmSimCode::Task>) -> Result<Arc<HpcOmSimCode::Task>> {
    let mut oTask: Arc<HpcOmSimCode::Task>;
    let mut tmpTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut sourceTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut targetTask: Arc<HpcOmSimCode::Task> = Arc::new(HpcOmSimCode::Task::TASKEMPTY);
    let mut outgoing: bool = false;
    let mut lockId: i32 = 0;
    let mut sourceTaskId: i32 = 0;
    let mut targetTaskId: i32 = 0;
    let mut communicationInfo: HpcOmSimCode::CommunicationInfo = <HpcOmSimCode::CommunicationInfo as ::std::default::Default>::default();
    oTask = (::match_deref::match_deref! { match &(iTask.clone()) {
        Deref @ HpcOmSimCode::Task::DEPTASK { sourceTask: __esc_sourceTask, targetTask: __esc_targetTask, outgoing: __esc_outgoing, communicationInfo: __esc_communicationInfo, .. } => {
            sourceTask = (*__esc_sourceTask).clone();
            targetTask = (*__esc_targetTask).clone();
            outgoing = (*__esc_outgoing).clone();
            communicationInfo = (*__esc_communicationInfo).clone();
            let __pa0 = ::match_deref::match_deref! { match &(sourceTask.clone()) {
                Deref @ HpcOmSimCode::Task::CALCTASK { index: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            sourceTaskId = __pa0.clone();
            let __pa1 = ::match_deref::match_deref! { match &(targetTask.clone()) {
                Deref @ HpcOmSimCode::Task::CALCTASK { index: __pa1, .. } => __pa1.clone(),
                _ => bail!("pattern mismatch"),
            } };
            targetTaskId = __pa1.clone();
            lockId = findInIntTuple1(metamodelica::arrayGet(lockIds.clone(), sourceTaskId)?, targetTaskId);
            tmpTask = Arc::new(HpcOmSimCode::Task::DEPTASK { sourceTask: sourceTask.clone(), targetTask: targetTask.clone(), outgoing: outgoing.clone(), id: lockId, communicationInfo: communicationInfo.clone() });
            tmpTask
        },
        _ => iTask,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oTask)
}

fn findInIntTuple1(mut liste: Arc<metamodelica::List<(i32, i32)>>, mut toFind: i32) -> i32 {
    let mut secondElement: i32 = 0;
    let mut first: i32;
    let mut second: i32;
    let mut iter: (i32, i32) = (0, 0);
    for mut iter in &*liste {
        let mut iter = iter.clone();
        (first, second) = iter.clone();
        if intEq(first, toFind) {
            secondElement = second;
            return secondElement.clone();
        }
    }
    secondElement
}

fn convertFixedLevelScheduleToTaskListsForLevel(mut iTasksOfLevel: HpcOmSimCode::TaskList, mut iThreadCount: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> {
    let mut oThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut tmpTaskLists: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = Default::default();
    let mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oThreadTasks = (match iTasksOfLevel {
        HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: mut __esc_tasks } => {
            tasks = __esc_tasks.clone();
            tmpTaskLists = arrayCreate(iThreadCount, metamodelica::nil());
            tmpTaskLists = List::fold(tasks.clone(), (std::sync::Arc::new(convertFixedLevelScheduleToTaskListsForTask) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> + 'static>), tmpTaskLists.clone())?;
            tmpTaskLists = revertTaskList(1, tmpTaskLists.clone());
            tmpTaskLists.clone()
        },
        HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: mut __esc_tasks, .. } => {
            tasks = __esc_tasks.clone();
            tmpTaskLists = arrayCreate(iThreadCount, metamodelica::nil());
            tmpTaskLists = metamodelica::arrayUpdate(tmpTaskLists.clone(), 1, tasks.clone())?;
            tmpTaskLists.clone()
        },
    });
    Ok(oThreadTasks)
}

fn convertFixedLevelScheduleToTaskListsForTask(mut iTask: Arc<HpcOmSimCode::Task>, mut iThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> {
    let mut oThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>;
    let mut tmpTaskLists: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = Default::default();
    let mut threadIdx: i32 = 0;
    let mut oldTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oThreadTasks = (::match_deref::match_deref! { match &(iTask.clone()) {
        Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { threadIdx: Some(__esc_threadIdx), .. } => {
            threadIdx = (*__esc_threadIdx).clone();
            oldTaskList = metamodelica::arrayGet(iThreadTasks.clone(), threadIdx.clone())?;
            tmpTaskLists = metamodelica::arrayUpdate(iThreadTasks.clone(), threadIdx.clone(), metamodelica::cons(iTask, oldTaskList))?;
            tmpTaskLists.clone()
        },
        _ => {
            metamodelica::print((literal!("ConvertFixedLevelScheduleToTaskListsForTask can just handle CALCTASK_LEVEL with defined thread idx!\n")).clone());
            iThreadTasks.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oThreadTasks)
}

fn printRealArray(mut inArray: metamodelica::Array<metamodelica::Real>, mut header: ArcStr) -> Result<()> {
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The ")); __mm_s.push_str(&*header.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    metamodelica::print((literal!("-----------------------------------------\n")).clone());
    Array::fold(inArray.clone(), (std::sync::Arc::new({ let __pe_b1 = (header).clone(); move |__pe_a0, __pe_a2| Ok(printRealArray1(__pe_a0, __pe_b1.clone(), __pe_a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real, i32) -> Result<i32> + 'static>), 1)?;
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

fn printRealArray1(mut inValue: metamodelica::Real, mut header: ArcStr, mut idxIn: i32) -> i32 {
    let mut idxOut: i32;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("node: ")); __mm_s.push_str(&*intString(idxIn)); __mm_s.push_str(&*literal!(" has the ")); __mm_s.push_str(&*header); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*realString(inValue)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    idxOut = idxIn + 1;
    idxOut
}

fn intListString(mut lstIn: Arc<metamodelica::List<i32>>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = stringDelimitList(List::map(lstIn.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(" , ")).clone());
    s = (if (lstIn.is_empty()) {literal!("{}")} else {s}).clone();
    Ok(s)
}

fn intListListString(mut lstIn: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = stringDelimitList(List::map(lstIn, (std::sync::Arc::new(intListString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))?, (literal!(" | ")).clone());
    Ok(s)
}

pub(crate) fn expandSchedule(mut iNumProc: i32, mut iNumUsedProc: i32, mut iSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<Arc<HpcOmSimCode::Schedule>> {
    let mut oSchedule: Arc<HpcOmSimCode::Schedule>;
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = Default::default();
    let mut outgoingDepTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut scheduledTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut allCalcTasks: metamodelica::Array<(Arc<HpcOmSimCode::Task>, i32)> = Default::default();
    oSchedule = (::match_deref::match_deref! { match &(iSchedule.clone()) {
        Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { .. } => iSchedule,
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __esc_threadTasks, outgoingDepTasks: __esc_outgoingDepTasks, scheduledTasks: __esc_scheduledTasks, allCalcTasks: __esc_allCalcTasks } => {
            threadTasks = (*__esc_threadTasks).clone();
            outgoingDepTasks = (*__esc_outgoingDepTasks).clone();
            scheduledTasks = (*__esc_scheduledTasks).clone();
            allCalcTasks = (*__esc_allCalcTasks).clone();
            threadTasks = Array::expandToSize(iNumProc, threadTasks.clone(), metamodelica::nil())?;
            Arc::new(HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: threadTasks.clone(), outgoingDepTasks: outgoingDepTasks.clone(), scheduledTasks: scheduledTasks.clone(), allCalcTasks: allCalcTasks.clone() })
        },
        Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { .. } => iSchedule,
        Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { .. } => iSchedule,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oSchedule)
}

