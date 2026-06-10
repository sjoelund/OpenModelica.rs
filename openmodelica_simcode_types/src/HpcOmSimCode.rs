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

use crate::SimCodeVar;

pub const fn emptyHpcomData() -> HpcOmData { HpcOmData { schedules: None, hpcOmMemory: None } }

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct HpcOmData {
    pub schedules: Option<(Arc<Schedule>, Arc<Schedule>, Arc<Schedule>)>,
    pub hpcOmMemory: Option<MemoryMap>,
}

impl metamodelica::gc::MMTrace for HpcOmData {
    fn mm_accept<__MMV: metamodelica::gc::dumpster::Visitor>(&self, __mmv: &mut __MMV) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.schedules, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.hpcOmMemory, __mmv)?;
        Ok(())
    }
}
impl Default for HpcOmData {
    fn default() -> Self {
        Self {
            schedules: Default::default(),
            hpcOmMemory: Default::default(),
        }
    }
}

pub type HPCOMDATA = HpcOmData;


#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum MemoryMap {
    MEMORYMAP_ARRAY {
        floatArraySize: i32,
        intArraySize: i32,
        boolArraySize: i32,
        stringArraySize: i32,
    },
    MEMORYMAP_UNIFORM,
}
impl metamodelica::gc::MMTrace for MemoryMap {
    fn mm_accept<__MMV: metamodelica::gc::dumpster::Visitor>(&self, __mmv: &mut __MMV) -> Result<(), ()> {
        match self {
            MemoryMap::MEMORYMAP_ARRAY { floatArraySize, intArraySize, boolArraySize, stringArraySize } => {
                metamodelica::gc::MMTrace::mm_accept(floatArraySize, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(intArraySize, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(boolArraySize, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(stringArraySize, __mmv)?;
                Ok(())
            }
            MemoryMap::MEMORYMAP_UNIFORM => Ok(()),
        }
    }
}
impl Default for MemoryMap {
    fn default() -> Self { Self::MEMORYMAP_UNIFORM }
}
pub use self::MemoryMap::{MEMORYMAP_ARRAY,MEMORYMAP_UNIFORM};

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct CommunicationInfo {
    pub floatVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub intVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
    pub boolVars: Arc<metamodelica::List<SimCodeVar::SimVar>>,
}

impl metamodelica::gc::MMTrace for CommunicationInfo {
    fn mm_accept<__MMV: metamodelica::gc::dumpster::Visitor>(&self, __mmv: &mut __MMV) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.floatVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.intVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.boolVars, __mmv)?;
        Ok(())
    }
}
impl Default for CommunicationInfo {
    fn default() -> Self {
        Self {
            floatVars: Default::default(),
            intVars: Default::default(),
            boolVars: Default::default(),
        }
    }
}

pub type COMMUNICATION_INFO = CommunicationInfo;


#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Task {
    SCHEDULED_TASK {
        compIdx: i32,
        numThreads: i32,
        taskSchedule: Arc<Schedule>,
    },
    CALCTASK {
        weighting: i32,
        index: i32,
        calcTime: metamodelica::Real,
        timeFinished: metamodelica::Real,
        threadIdx: i32,
        eqIdc: Arc<metamodelica::List<i32>>,
    },
    CALCTASK_LEVEL {
        eqIdc: Arc<metamodelica::List<i32>>,
        nodeIdc: Arc<metamodelica::List<i32>>,
        threadIdx: Option<i32>,
    },
    DEPTASK {
        sourceTask: Arc<Task>,
        targetTask: Arc<Task>,
        outgoing: bool,
        id: i32,
        communicationInfo: CommunicationInfo,
    },
    PREFETCHTASK {
        varIdc: Arc<metamodelica::List<i32>>,
        varArrayidx: i32,
    },
    TASKEMPTY,
}
impl metamodelica::gc::MMTrace for Task {
    fn mm_accept<__MMV: metamodelica::gc::dumpster::Visitor>(&self, __mmv: &mut __MMV) -> Result<(), ()> {
        match self {
            Task::SCHEDULED_TASK { compIdx, numThreads, taskSchedule } => {
                metamodelica::gc::MMTrace::mm_accept(compIdx, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(numThreads, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(taskSchedule, __mmv)?;
                Ok(())
            }
            Task::CALCTASK { weighting, index, calcTime, timeFinished, threadIdx, eqIdc } => {
                metamodelica::gc::MMTrace::mm_accept(weighting, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(calcTime, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(timeFinished, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(threadIdx, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqIdc, __mmv)?;
                Ok(())
            }
            Task::CALCTASK_LEVEL { eqIdc, nodeIdc, threadIdx } => {
                metamodelica::gc::MMTrace::mm_accept(eqIdc, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(nodeIdc, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(threadIdx, __mmv)?;
                Ok(())
            }
            Task::DEPTASK { sourceTask, targetTask, outgoing, id, communicationInfo } => {
                metamodelica::gc::MMTrace::mm_accept(sourceTask, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(targetTask, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(outgoing, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(id, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(communicationInfo, __mmv)?;
                Ok(())
            }
            Task::PREFETCHTASK { varIdc, varArrayidx } => {
                metamodelica::gc::MMTrace::mm_accept(varIdc, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(varArrayidx, __mmv)?;
                Ok(())
            }
            Task::TASKEMPTY => Ok(()),
        }
    }
}
impl Task {
    pub fn interned_TASKEMPTY() -> Arc<Task> {
        thread_local! {
            static INTERNED: Arc<Task> = Arc::new(Task::TASKEMPTY);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_TASKEMPTY() -> Arc<Task> { Task::interned_TASKEMPTY() }
impl Default for Task {
    fn default() -> Self { Self::TASKEMPTY }
}
pub use self::Task::{SCHEDULED_TASK,CALCTASK,CALCTASK_LEVEL,DEPTASK,PREFETCHTASK,TASKEMPTY};

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum TaskList {
    PARALLELTASKLIST {
        tasks: Arc<metamodelica::List<Arc<Task>>>,
    },
    SERIALTASKLIST {
        tasks: Arc<metamodelica::List<Arc<Task>>>,
        masterOnly: bool,
    },
}
impl metamodelica::gc::MMTrace for TaskList {
    fn mm_accept<__MMV: metamodelica::gc::dumpster::Visitor>(&self, __mmv: &mut __MMV) -> Result<(), ()> {
        match self {
            TaskList::PARALLELTASKLIST { tasks } => {
                metamodelica::gc::MMTrace::mm_accept(tasks, __mmv)?;
                Ok(())
            }
            TaskList::SERIALTASKLIST { tasks, masterOnly } => {
                metamodelica::gc::MMTrace::mm_accept(tasks, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(masterOnly, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for TaskList {
    fn default() -> Self {
        Self::PARALLELTASKLIST {
            tasks: Default::default(),
        }
    }
}
pub use self::TaskList::{PARALLELTASKLIST,SERIALTASKLIST};

//TODO: Use the TaskList for the other schedulers, too
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Schedule {
    LEVELSCHEDULE {
        tasksOfLevels: Arc<metamodelica::List<TaskList>>,
        useFixedAssignments: bool,
    },
    THREADSCHEDULE {
        threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<Task>>>>,
        outgoingDepTasks: Arc<metamodelica::List<Arc<Task>>>,
        scheduledTasks: Arc<metamodelica::List<Arc<Task>>>,
        allCalcTasks: metamodelica::Array<(Arc<Task>, i32)>,
    },
    TASKDEPSCHEDULE {
        tasks: Arc<metamodelica::List<(Arc<Task>, Arc<metamodelica::List<i32>>)>>,
    },
    EMPTYSCHEDULE {
        tasks: TaskList,
    },
}
impl metamodelica::gc::MMTrace for Schedule {
    fn mm_accept<__MMV: metamodelica::gc::dumpster::Visitor>(&self, __mmv: &mut __MMV) -> Result<(), ()> {
        match self {
            Schedule::LEVELSCHEDULE { tasksOfLevels, useFixedAssignments } => {
                metamodelica::gc::MMTrace::mm_accept(tasksOfLevels, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(useFixedAssignments, __mmv)?;
                Ok(())
            }
            Schedule::THREADSCHEDULE { threadTasks, outgoingDepTasks, scheduledTasks, allCalcTasks } => {
                metamodelica::gc::MMTrace::mm_accept(threadTasks, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(outgoingDepTasks, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scheduledTasks, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(allCalcTasks, __mmv)?;
                Ok(())
            }
            Schedule::TASKDEPSCHEDULE { tasks } => {
                metamodelica::gc::MMTrace::mm_accept(tasks, __mmv)?;
                Ok(())
            }
            Schedule::EMPTYSCHEDULE { tasks } => {
                metamodelica::gc::MMTrace::mm_accept(tasks, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Schedule {
    fn default() -> Self {
        Self::TASKDEPSCHEDULE {
            tasks: Default::default(),
        }
    }
}
pub use self::Schedule::{LEVELSCHEDULE,THREADSCHEDULE,TASKDEPSCHEDULE,EMPTYSCHEDULE};

