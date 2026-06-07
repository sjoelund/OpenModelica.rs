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
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::HpcOmScheduler;
use crate::HpcOmTaskGraph;
use crate::SimCodeUtil;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::HashTableCrIListArray;
use openmodelica_frontend_dump::HashTableCrILst;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::HpcOmSimCode;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_simcode_util::SimCodeUtilShared;
use openmodelica_susan::GraphML;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

// -------------------------------------------
// STRUCTURES
// -------------------------------------------
pub const VARDATATYPE_FLOAT: i32 = 1;

pub const VARDATATYPE_INTEGER: i32 = 2;

pub const VARDATATYPE_BOOLEAN: i32 = 3;

pub const VARDATATYPE_STRING: i32 = 4;

pub const VARTYPE_STATE: i32 = 1;

pub const VARTYPE_STATEDER: i32 = 2;

pub const VARTYPE_PARAM: i32 = 3;

pub const VARTYPE_ALIAS: i32 = 4;

pub const VARTYPE_OTHER: i32 = 5;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, metamodelica::ReferenceEq)]
pub enum CacheMap {
    CACHEMAP {
        cacheLineSize: i32,
        cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>>,
        cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>>,
        cacheLinesInt: Arc<metamodelica::List<CacheLineMap>>,
        cacheLinesBool: Arc<metamodelica::List<CacheLineMap>>,
    },
    UNIFORM_CACHEMAP {
        cacheLineSize: i32,
        cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>>,
        cacheLines: Arc<metamodelica::List<CacheLineMap>>,
    },
}
impl Default for CacheMap {
    fn default() -> Self {
        Self::UNIFORM_CACHEMAP {
            cacheLineSize: Default::default(),
            cacheVariables: Default::default(),
            cacheLines: Default::default(),
        }
    }
}
pub use self::CacheMap::{CACHEMAP,UNIFORM_CACHEMAP};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, metamodelica::ReferenceEq)]
pub struct CacheLineMap {
    pub idx: i32,
    pub numBytesFree: i32,
    pub entries: Arc<metamodelica::List<CacheLineEntry>>,
}

impl Default for CacheLineMap {
    fn default() -> Self {
        Self {
            idx: Default::default(),
            numBytesFree: Default::default(),
            entries: Default::default(),
        }
    }
}

pub type CACHELINEMAP = CacheLineMap;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, metamodelica::ReferenceEq)]
pub struct CacheLineEntry {
    pub start: i32,
    pub dataType: i32,
    pub size: i32,
    pub scVarIdx: i32,
    pub threadOwner: i32,
}

impl Default for CacheLineEntry {
    fn default() -> Self {
        Self {
            start: Default::default(),
            dataType: Default::default(),
            size: Default::default(),
            scVarIdx: Default::default(),
            threadOwner: Default::default(),
        }
    }
}

pub type CACHELINEENTRY = CacheLineEntry;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, metamodelica::ReferenceEq)]
pub struct CacheMapMeta {
    pub allSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>,
    pub simCodeVarTypes: metamodelica::Array<(i32, i32, i32)>,
    pub scVarCLMapping: metamodelica::Array<(i32, i32)>,
}

impl Default for CacheMapMeta {
    fn default() -> Self {
        Self {
            allSCVarsMapping: Default::default(),
            simCodeVarTypes: Default::default(),
            scVarCLMapping: Default::default(),
        }
    }
}

pub type CACHEMAPMETA = CacheMapMeta;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, metamodelica::ReferenceEq)]
pub enum PartlyFilledCacheLine {
    PARTLYFILLEDCACHELINE_LEVEL {
        cacheLineMap: CacheLineMap,
        prefetchLevel: Arc<metamodelica::List<i32>>,
        writeLevel: Arc<metamodelica::List<(i32, i32)>>,
    },
    PARTLYFILLEDCACHELINE_THREAD {
        cacheLineMap: CacheLineMap,
    },
}
impl Default for PartlyFilledCacheLine {
    fn default() -> Self {
        Self::PARTLYFILLEDCACHELINE_THREAD {
            cacheLineMap: Default::default(),
        }
    }
}
pub use self::PartlyFilledCacheLine::{PARTLYFILLEDCACHELINE_LEVEL,PARTLYFILLEDCACHELINE_THREAD};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, metamodelica::ReferenceEq)]
pub struct ScVarInfo {
    pub ownerThread: i32,
    pub isShared: bool,
}

impl Default for ScVarInfo {
    fn default() -> Self {
        Self {
            ownerThread: Default::default(),
            isShared: Default::default(),
        }
    }
}

pub type SCVARINFO = ScVarInfo;


pub type PartlyFilledCacheLines = (Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>);

pub type CacheLines = (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>);

// -------------------------------------------
// FUNCTIONS
// -------------------------------------------
pub fn createMemoryMap(mut iModelInfo: SimCode::ModelInfo, mut iVarToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut iVarToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut iFileNamePrefix: ArcStr, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iSchedule: Arc<HpcOmSimCode::Schedule>, mut iSccSimEqMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCriticalPaths: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iCriticalPathsWoC: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut iCriticalPathInfo: ArcStr, mut iNumberOfThreads: i32, mut iAllComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut isInitial: bool) -> Result<(Option<HpcOmSimCode::MemoryMap>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)))> {
    let mut oMemoryMap: Option<HpcOmSimCode::MemoryMap> = None;
    let mut oVarToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
    let mut oVarToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut simCodeVars: SimCodeVar::SimVars = <SimCodeVar::SimVars as ::std::default::Default>::default();
    let mut stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut derivativeVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut algVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut discreteAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut intAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut stringAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut inputVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut outputVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut aliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut paramVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut intParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut stringParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut intAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut stringAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsFloatOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = metamodelica::nil();
    let mut notOptimizedVarsIntOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = metamodelica::nil();
    let mut notOptimizedVarsBoolOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = metamodelica::nil();
    let mut notOptimizedVarsStringOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = metamodelica::nil();
    let mut notOptimizedVarsFloat: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsInt: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsBool: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsString: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) = (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil());
    let mut allVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>> = Default::default();
    let mut simVarIdxMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut numCL: i32 = 0;
    let mut threadAttIdx: i32 = 0;
    let mut clTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut scVarSolvedTaskMapping: metamodelica::Array<i32> = Default::default();
    let mut sccNodeMapping: metamodelica::Array<i32> = Default::default();
    let mut scVarUnsolvedTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut annotInfo: metamodelica::Array<ArcStr> = Default::default();
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut cacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut graphIdx: i32 = 0;
    let mut graphInfo: GraphML::GraphInfo = <GraphML::GraphInfo as ::std::default::Default>::default();
    let mut fileName: ArcStr = arcstr::literal!("");
    let mut eqSimCodeVarMapping: metamodelica::Array<metamodelica::Array<Arc<metamodelica::List<i32>>>> = Default::default();
    let mut eqCompMapping: metamodelica::Array<(i32, i32, i32)> = Default::default();
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)> = Default::default();
    let mut adjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut tmpMemoryMapOpt: Option<HpcOmSimCode::MemoryMap> = None;
    let mut varCount: i32 = 0;
    let mut stateVarsCnt: i32 = 0;
    let mut derivativeVarsCnt: i32 = 0;
    let mut algVarsCnt: i32 = 0;
    let mut discreteAlgVarsCnt: i32 = 0;
    let mut intAlgVarsCnt: i32 = 0;
    let mut boolAlgVarsCnt: i32 = 0;
    let mut stringAlgVarsCnt: i32 = 0;
    let mut inputVarsCnt: i32 = 0;
    let mut outputVarsCnt: i32 = 0;
    let mut aliasVarsCnt: i32 = 0;
    let mut intAliasVarsCnt: i32 = 0;
    let mut boolAliasVarsCnt: i32 = 0;
    let mut stringAliasVarsCnt: i32 = 0;
    let mut paramVarsCnt: i32 = 0;
    let mut intParamVarsCnt: i32 = 0;
    let mut boolParamVarsCnt: i32 = 0;
    let mut stringParamVarsCnt: i32 = 0;
    let mut VARSIZE_FLOAT: i32 = 0;
    let mut VARSIZE_INTEGER: i32 = 0;
    let mut VARSIZE_BOOLEAN: i32 = 0;
    let mut VARSIZE_STRING: i32 = 0;
    let mut CACHELINE_SIZE: i32 = 0;
    let mut simCodeVarTypes: metamodelica::Array<(i32, i32, i32)> = Default::default();
    let mut taskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut taskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut nodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut flatEqSimCodeVarMapping: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)> = Default::default();
    let mut sccEqMapping: metamodelica::Array<Arc<metamodelica::List<(i32, i32, i32)>>> = Default::default();
    let mut scVarInfos: metamodelica::Array<ScVarInfo> = Default::default();
    let mut varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
    let mut varToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    (oMemoryMap, oVarToArrayIndexMapping, oVarToIndexMapping) = 'mc: {
        let __mc_input = (iVarToArrayIndexMapping.clone(), iVarToIndexMapping.clone(), iTaskGraphMeta.clone());
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10, __wb11, __wb12, __wb13, __wb14, __wb15, __wb16, __wb17, __wb18, __wb19, __wb20, __wb21, __wb22, __wb23, __wb24, __wb25, __wb26, __wb27, __wb28, __wb29, __wb30, __wb31, __wb32, __wb33, __wb34, __wb35, __wb36, __wb37, __wb38, __wb39, __wb40, __wb41, __wb42, __wb43, __wb44, __wb45, __wb46, __wb47, __wb48, __wb49, __wb50, __wb51, __wb52, __wb53, __wb54, __wb55, __wb56, __wb57, __wb58, __wb59, __wb60, __wb61, __wb62, __wb63, __wb64, __wb65, __wb66, __wb67, __wb68, __wb69, __wb70, __wb71, __wb72)) = (|| -> Result<_> {
            let (mut varToArrayIndexMapping, mut varToIndexMapping, HpcOmTaskGraph::TaskGraphMeta { eqCompMapping: mut eqCompMapping, varCompMapping: mut varCompMapping, .. }) = __mc_input.clone() else { bail!("nomatch") };
            let mut CACHELINE_SIZE: i32 = CACHELINE_SIZE.clone();
            let mut VARSIZE_BOOLEAN: i32 = VARSIZE_BOOLEAN.clone();
            let mut VARSIZE_FLOAT: i32 = VARSIZE_FLOAT.clone();
            let mut VARSIZE_INTEGER: i32 = VARSIZE_INTEGER.clone();
            let mut VARSIZE_STRING: i32 = VARSIZE_STRING.clone();
            let mut adjacencyMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>> = adjacencyMatrix.clone();
            let mut algVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = algVars.clone();
            let mut algVarsCnt: i32 = algVarsCnt.clone();
            let mut aliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = aliasVars.clone();
            let mut aliasVarsCnt: i32 = aliasVarsCnt.clone();
            let mut allVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>> = allVarsMapping.clone();
            let mut annotInfo: metamodelica::Array<ArcStr> = annotInfo.clone();
            let mut boolAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = boolAlgVars.clone();
            let mut boolAlgVarsCnt: i32 = boolAlgVarsCnt.clone();
            let mut boolAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = boolAliasVars.clone();
            let mut boolAliasVarsCnt: i32 = boolAliasVarsCnt.clone();
            let mut boolParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = boolParamVars.clone();
            let mut boolParamVarsCnt: i32 = boolParamVarsCnt.clone();
            let mut cacheMap: CacheMap = cacheMap.clone();
            let mut clTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = clTaskMapping.clone();
            let mut derivativeVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = derivativeVars.clone();
            let mut derivativeVarsCnt: i32 = derivativeVarsCnt.clone();
            let mut discreteAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = discreteAlgVars.clone();
            let mut discreteAlgVarsCnt: i32 = discreteAlgVarsCnt.clone();
            let mut eqSimCodeVarMapping: metamodelica::Array<metamodelica::Array<Arc<metamodelica::List<i32>>>> = eqSimCodeVarMapping.clone();
            let mut fileName: ArcStr = fileName.clone();
            let mut flatEqSimCodeVarMapping: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)> = flatEqSimCodeVarMapping.clone();
            let mut graphIdx: i32 = graphIdx.clone();
            let mut graphInfo: GraphML::GraphInfo = graphInfo.clone();
            let mut inputVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = inputVars.clone();
            let mut inputVarsCnt: i32 = inputVarsCnt.clone();
            let mut intAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = intAlgVars.clone();
            let mut intAlgVarsCnt: i32 = intAlgVarsCnt.clone();
            let mut intAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = intAliasVars.clone();
            let mut intAliasVarsCnt: i32 = intAliasVarsCnt.clone();
            let mut intParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = intParamVars.clone();
            let mut intParamVarsCnt: i32 = intParamVarsCnt.clone();
            let mut nodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = nodeSccMapping.clone();
            let mut notOptimizedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) = notOptimizedVars.clone();
            let mut notOptimizedVarsBool: Arc<metamodelica::List<SimCodeVar::SimVar>> = notOptimizedVarsBool.clone();
            let mut notOptimizedVarsBoolOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = notOptimizedVarsBoolOpt.clone();
            let mut notOptimizedVarsFloat: Arc<metamodelica::List<SimCodeVar::SimVar>> = notOptimizedVarsFloat.clone();
            let mut notOptimizedVarsFloatOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = notOptimizedVarsFloatOpt.clone();
            let mut notOptimizedVarsInt: Arc<metamodelica::List<SimCodeVar::SimVar>> = notOptimizedVarsInt.clone();
            let mut notOptimizedVarsIntOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = notOptimizedVarsIntOpt.clone();
            let mut notOptimizedVarsString: Arc<metamodelica::List<SimCodeVar::SimVar>> = notOptimizedVarsString.clone();
            let mut notOptimizedVarsStringOpt: Arc<metamodelica::List<Option<SimCodeVar::SimVar>>> = notOptimizedVarsStringOpt.clone();
            let mut numCL: i32 = numCL.clone();
            let mut outputVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = outputVars.clone();
            let mut outputVarsCnt: i32 = outputVarsCnt.clone();
            let mut paramVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = paramVars.clone();
            let mut paramVarsCnt: i32 = paramVarsCnt.clone();
            let mut scVarCLMapping: metamodelica::Array<(i32, i32)> = scVarCLMapping.clone();
            let mut scVarInfos: metamodelica::Array<ScVarInfo> = scVarInfos.clone();
            let mut scVarSolvedTaskMapping: metamodelica::Array<i32> = scVarSolvedTaskMapping.clone();
            let mut scVarUnsolvedTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = scVarUnsolvedTaskMapping.clone();
            let mut sccEqMapping: metamodelica::Array<Arc<metamodelica::List<(i32, i32, i32)>>> = sccEqMapping.clone();
            let mut sccNodeMapping: metamodelica::Array<i32> = sccNodeMapping.clone();
            let mut simCodeVarTypes: metamodelica::Array<(i32, i32, i32)> = simCodeVarTypes.clone();
            let mut simCodeVars: SimCodeVar::SimVars = simCodeVars.clone();
            let mut simVarIdxMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
            let mut stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = stateVars.clone();
            let mut stateVarsCnt: i32 = stateVarsCnt.clone();
            let mut stringAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = stringAlgVars.clone();
            let mut stringAlgVarsCnt: i32 = stringAlgVarsCnt.clone();
            let mut stringAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = stringAliasVars.clone();
            let mut stringAliasVarsCnt: i32 = stringAliasVarsCnt.clone();
            let mut stringParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = stringParamVars.clone();
            let mut stringParamVarsCnt: i32 = stringParamVarsCnt.clone();
            let mut taskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = taskSolvedVarsMapping.clone();
            let mut taskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = taskUnsolvedVarsMapping.clone();
            let mut threadAttIdx: i32 = threadAttIdx.clone();
            let mut tmpMemoryMapOpt: Option<HpcOmSimCode::MemoryMap> = tmpMemoryMapOpt.clone();
            let mut varCount: i32 = varCount.clone();
            VARSIZE_FLOAT = 8;
            VARSIZE_INTEGER = 4;
            VARSIZE_BOOLEAN = 1;
            VARSIZE_STRING = 4;
            CACHELINE_SIZE = 64;
            let SimCode::MODELINFO { vars: __pa0, .. } = (iModelInfo.clone()) else { bail!("pattern mismatch") };
            simCodeVars = __pa0.clone();
            let SimCodeVar::SIMVARS { stateVars: __pa1, derivativeVars: __pa2, algVars: __pa3, discreteAlgVars: __pa4, intAlgVars: __pa5, boolAlgVars: __pa6, stringAlgVars: __pa7, inputVars: __pa8, outputVars: __pa9, aliasVars: __pa10, intAliasVars: __pa11, boolAliasVars: __pa12, stringAliasVars: __pa13, paramVars: __pa14, intParamVars: __pa15, boolParamVars: __pa16, stringParamVars: __pa17, .. } = (simCodeVars.clone()) else { bail!("pattern mismatch") };
            stateVars = __pa1.clone();
            derivativeVars = __pa2.clone();
            algVars = __pa3.clone();
            discreteAlgVars = __pa4.clone();
            intAlgVars = __pa5.clone();
            boolAlgVars = __pa6.clone();
            stringAlgVars = __pa7.clone();
            inputVars = __pa8.clone();
            outputVars = __pa9.clone();
            aliasVars = __pa10.clone();
            intAliasVars = __pa11.clone();
            boolAliasVars = __pa12.clone();
            stringAliasVars = __pa13.clone();
            paramVars = __pa14.clone();
            intParamVars = __pa15.clone();
            boolParamVars = __pa16.clone();
            stringParamVars = __pa17.clone();
            allVarsMapping = SimCodeUtil::createIdxSCVarMapping(simCodeVars.clone())?;
            simVarIdxMappingHashTable = HashTableCrILst::emptyHashTableSized(BaseHashTable::biggerBucketSize.clone());
            varCount = 0;
            stateVarsCnt = (stateVars.clone().len() as i32);
            varCount = varCount.clone() + stateVarsCnt.clone();
            derivativeVarsCnt = (derivativeVars.clone().len() as i32);
            varCount = varCount.clone() + derivativeVarsCnt.clone();
            simVarIdxMappingHashTable = fillSimVarHashTable(algVars.clone(), varCount.clone(), VARDATATYPE_FLOAT.clone(), simVarIdxMappingHashTable.clone())?;
            algVarsCnt = (algVars.clone().len() as i32);
            varCount = varCount.clone() + algVarsCnt.clone();
            simVarIdxMappingHashTable = fillSimVarHashTable(discreteAlgVars.clone(), varCount.clone(), VARDATATYPE_FLOAT.clone(), simVarIdxMappingHashTable.clone())?;
            discreteAlgVarsCnt = (discreteAlgVars.clone().len() as i32);
            varCount = varCount.clone() + discreteAlgVarsCnt.clone();
            simVarIdxMappingHashTable = fillSimVarHashTable(intAlgVars.clone(), varCount.clone(), VARDATATYPE_INTEGER.clone(), simVarIdxMappingHashTable.clone())?;
            intAlgVarsCnt = (intAlgVars.clone().len() as i32);
            varCount = varCount.clone() + intAlgVarsCnt.clone();
            simVarIdxMappingHashTable = fillSimVarHashTable(boolAlgVars.clone(), varCount.clone(), VARDATATYPE_BOOLEAN.clone(), simVarIdxMappingHashTable.clone())?;
            boolAlgVarsCnt = (boolAlgVars.clone().len() as i32);
            varCount = varCount.clone() + boolAlgVarsCnt.clone();
            simVarIdxMappingHashTable = fillSimVarHashTable(stringAlgVars.clone(), varCount.clone(), VARDATATYPE_STRING.clone(), simVarIdxMappingHashTable.clone())?;
            stringAlgVarsCnt = (stringAlgVars.clone().len() as i32);
            varCount = varCount.clone() + stringAlgVarsCnt.clone();
            simVarIdxMappingHashTable = fillSimVarHashTable(inputVars.clone(), varCount.clone(), VARDATATYPE_FLOAT.clone(), simVarIdxMappingHashTable.clone())?;
            inputVarsCnt = (inputVars.clone().len() as i32);
            varCount = varCount.clone() + inputVarsCnt.clone();
            simVarIdxMappingHashTable = fillSimVarHashTable(outputVars.clone(), varCount.clone(), VARDATATYPE_FLOAT.clone(), simVarIdxMappingHashTable.clone())?;
            outputVarsCnt = (outputVars.clone().len() as i32);
            varCount = varCount.clone() + outputVarsCnt.clone();
            aliasVarsCnt = (aliasVars.clone().len() as i32);
            varCount = varCount.clone() + aliasVarsCnt.clone();
            intAliasVarsCnt = (intAliasVars.clone().len() as i32);
            varCount = varCount.clone() + intAliasVarsCnt.clone();
            boolAliasVarsCnt = (boolAliasVars.clone().len() as i32);
            varCount = varCount.clone() + boolAliasVarsCnt.clone();
            simVarIdxMappingHashTable = fillSimVarHashTable(stringAliasVars.clone(), varCount.clone(), VARDATATYPE_STRING.clone(), simVarIdxMappingHashTable.clone())?;
            stringAliasVarsCnt = (stringAliasVars.clone().len() as i32);
            varCount = varCount.clone() + stringAliasVarsCnt.clone();
            simVarIdxMappingHashTable = fillSimVarHashTable(paramVars.clone(), varCount.clone(), VARDATATYPE_FLOAT.clone(), simVarIdxMappingHashTable.clone())?;
            paramVarsCnt = (paramVars.clone().len() as i32);
            varCount = varCount.clone() + paramVarsCnt.clone();
            simVarIdxMappingHashTable = fillSimVarHashTable(intParamVars.clone(), varCount.clone(), VARDATATYPE_INTEGER.clone(), simVarIdxMappingHashTable.clone())?;
            intParamVarsCnt = (intParamVars.clone().len() as i32);
            varCount = varCount.clone() + intParamVarsCnt.clone();
            simVarIdxMappingHashTable = fillSimVarHashTable(boolParamVars.clone(), varCount.clone(), VARDATATYPE_BOOLEAN.clone(), simVarIdxMappingHashTable.clone())?;
            boolParamVarsCnt = (boolParamVars.clone().len() as i32);
            varCount = varCount.clone() + boolParamVarsCnt.clone();
            simVarIdxMappingHashTable = fillSimVarHashTable(stringParamVars.clone(), varCount.clone(), VARDATATYPE_STRING.clone(), simVarIdxMappingHashTable.clone())?;
            stringParamVarsCnt = (stringParamVars.clone().len() as i32);
            varCount = varCount.clone() + stringParamVarsCnt.clone();
            simCodeVarTypes = arrayCreate(varCount.clone(), (-1, -1, -1));
            varCount = 0;
            varCount = varCount.clone() + stateVarsCnt.clone();
            varCount = varCount.clone() + derivativeVarsCnt.clone();
            if algVarsCnt.clone() > 0 {
                List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + algVarsCnt.clone()), (std::sync::Arc::new({ let __pe_b1 = (VARDATATYPE_FLOAT.clone(), VARSIZE_FLOAT.clone(), VARTYPE_OTHER.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>))?;
            }
            varCount = varCount.clone() + algVarsCnt.clone();
            if discreteAlgVarsCnt.clone() > 0 {
                List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + discreteAlgVarsCnt.clone()), (std::sync::Arc::new({ let __pe_b1 = (VARDATATYPE_FLOAT.clone(), VARSIZE_FLOAT.clone(), VARTYPE_OTHER.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>))?;
            }
            varCount = varCount.clone() + discreteAlgVarsCnt.clone();
            if intAlgVarsCnt.clone() > 0 {
                List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + intAlgVarsCnt.clone()), (std::sync::Arc::new({ let __pe_b1 = (VARDATATYPE_INTEGER.clone(), VARSIZE_INTEGER.clone(), VARTYPE_OTHER.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>))?;
            }
            varCount = varCount.clone() + intAlgVarsCnt.clone();
            if boolAlgVarsCnt.clone() > 0 {
                List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + boolAlgVarsCnt.clone()), (std::sync::Arc::new({ let __pe_b1 = (VARDATATYPE_BOOLEAN.clone(), VARSIZE_BOOLEAN.clone(), VARTYPE_OTHER.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>))?;
            }
            varCount = varCount.clone() + boolAlgVarsCnt.clone();
            if stringAlgVarsCnt.clone() > 0 {
                List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + stringAlgVarsCnt.clone()), (std::sync::Arc::new({ let __pe_b1 = (VARDATATYPE_STRING.clone(), VARSIZE_STRING.clone(), VARTYPE_OTHER.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>))?;
            }
            varCount = varCount.clone() + stringAlgVarsCnt.clone();
            if inputVarsCnt.clone() > 0 {
                List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + inputVarsCnt.clone()), (std::sync::Arc::new({ let __pe_b1 = (VARDATATYPE_FLOAT.clone(), VARSIZE_FLOAT.clone(), VARTYPE_OTHER.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>))?;
            }
            varCount = varCount.clone() + inputVarsCnt.clone();
            if outputVarsCnt.clone() > 0 {
                List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + outputVarsCnt.clone()), (std::sync::Arc::new({ let __pe_b1 = (VARDATATYPE_FLOAT.clone(), VARSIZE_FLOAT.clone(), VARTYPE_OTHER.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>))?;
            }
            varCount = varCount.clone() + outputVarsCnt.clone();
            varCount = varCount.clone() + aliasVarsCnt.clone();
            varCount = varCount.clone() + intAliasVarsCnt.clone();
            varCount = varCount.clone() + boolAliasVarsCnt.clone();
            varCount = varCount.clone() + stringAliasVarsCnt.clone();
            if paramVarsCnt.clone() > 0 {
                List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + paramVarsCnt.clone()), (std::sync::Arc::new({ let __pe_b1 = (VARDATATYPE_FLOAT.clone(), VARSIZE_FLOAT.clone(), VARTYPE_PARAM.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>))?;
            }
            varCount = varCount.clone() + paramVarsCnt.clone();
            if intParamVarsCnt.clone() > 0 {
                List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + intParamVarsCnt.clone()), (std::sync::Arc::new({ let __pe_b1 = (VARDATATYPE_INTEGER.clone(), VARSIZE_INTEGER.clone(), VARTYPE_PARAM.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>))?;
            }
            varCount = varCount.clone() + intParamVarsCnt.clone();
            if boolParamVarsCnt.clone() > 0 {
                List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + boolParamVarsCnt.clone()), (std::sync::Arc::new({ let __pe_b1 = (VARDATATYPE_BOOLEAN.clone(), VARSIZE_BOOLEAN.clone(), VARTYPE_PARAM.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>))?;
            }
            varCount = varCount.clone() + boolParamVarsCnt.clone();
            if stringParamVarsCnt.clone() > 0 {
                List::map_0(List::intRange2(varCount.clone() + 1, varCount.clone() + stringParamVarsCnt.clone()), (std::sync::Arc::new({ let __pe_b1 = (VARDATATYPE_STRING.clone(), VARSIZE_STRING.clone(), VARTYPE_PARAM.clone()); let __pe_b2 = simCodeVarTypes.clone(); move |__pe_a0| Array::updateIndexFirst(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>))?;
            }
            varCount = varCount.clone() + stringParamVarsCnt.clone();
            sccNodeMapping = HpcOmTaskGraph::getSccNodeMapping(metamodelica::arrayLength(iSccSimEqMapping.clone()), iTaskGraphMeta.clone())?;
            scVarSolvedTaskMapping = getSimCodeVarNodeMapping(iTaskGraphMeta.clone(), iEqSystems.clone(), varCount.clone(), sccNodeMapping.clone(), simVarIdxMappingHashTable.clone())?;
            eqSimCodeVarMapping = getEqSCVarMapping(iEqSystems.clone(), simVarIdxMappingHashTable.clone())?;
            sccEqMapping = invertEqCompMapping(eqCompMapping.clone(), metamodelica::arrayLength(sccNodeMapping.clone()))?;
            nodeSccMapping = invertSccNodeMapping(sccNodeMapping.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
            flatEqSimCodeVarMapping = flattenEqSimCodeVarMapping(eqSimCodeVarMapping.clone())?;
            (taskSolvedVarsMapping, taskUnsolvedVarsMapping) = getTaskSimVarMapping(sccEqMapping.clone(), nodeSccMapping.clone(), flatEqSimCodeVarMapping.clone(), scVarSolvedTaskMapping.clone(), simCodeVarTypes.clone())?;
            scVarUnsolvedTaskMapping = transposeTasksScVarsMapping(taskUnsolvedVarsMapping.clone(), varCount.clone())?;
            scVarInfos = createVarInfos(scVarSolvedTaskMapping.clone(), scVarUnsolvedTaskMapping.clone(), iSchedulerInfo.clone())?;
            if Flags::isSet(Flags::HPCOM_MEMORY_OPT.clone())? {
                (cacheMap, scVarCLMapping, numCL) = createCacheMapOptimized(iTaskGraph.clone(), iTaskGraphMeta.clone(), simCodeVars.clone(), allVarsMapping.clone(), simCodeVarTypes.clone(), scVarSolvedTaskMapping.clone(), scVarUnsolvedTaskMapping.clone(), CACHELINE_SIZE.clone(), iAllComponents.clone(), iSchedule.clone(), iSchedulerInfo.clone(), iNumberOfThreads.clone(), taskSolvedVarsMapping.clone(), taskUnsolvedVarsMapping.clone(), scVarInfos.clone())?;
            } else {
                (cacheMap, scVarCLMapping, numCL) = createCacheMapDefault(allVarsMapping.clone(), CACHELINE_SIZE.clone(), simCodeVars.clone(), scVarSolvedTaskMapping.clone(), iSchedulerInfo.clone(), simCodeVarTypes.clone())?;
            }
            (clTaskMapping, _) = getCacheLineTaskMapping(iTaskGraphMeta.clone(), iEqSystems.clone(), simVarIdxMappingHashTable.clone(), numCL.clone(), scVarCLMapping.clone())?;
            notOptimizedVars = getNotOptimizedVarsByCacheLineMapping(scVarCLMapping.clone(), allVarsMapping.clone(), simCodeVarTypes.clone())?;
            notOptimizedVarsFloatOpt = List::map(Util::tuple41(notOptimizedVars.clone()), (std::sync::Arc::new({ let __pe_b0 = allVarsMapping.clone(); move |__pe_a1| metamodelica::arrayGet(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<_> + 'static>))?;
            notOptimizedVarsIntOpt = List::map(Util::tuple42(notOptimizedVars.clone()), (std::sync::Arc::new({ let __pe_b0 = allVarsMapping.clone(); move |__pe_a1| metamodelica::arrayGet(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<_> + 'static>))?;
            notOptimizedVarsBoolOpt = List::map(Util::tuple43(notOptimizedVars.clone()), (std::sync::Arc::new({ let __pe_b0 = allVarsMapping.clone(); move |__pe_a1| metamodelica::arrayGet(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<_> + 'static>))?;
            notOptimizedVarsStringOpt = List::map(Util::tuple44(notOptimizedVars.clone()), (std::sync::Arc::new({ let __pe_b0 = allVarsMapping.clone(); move |__pe_a1| metamodelica::arrayGet(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<_> + 'static>))?;
            notOptimizedVarsFloat = List::map(notOptimizedVarsFloatOpt.clone(), (std::sync::Arc::new(Util::getOption) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
            notOptimizedVarsInt = List::map(notOptimizedVarsIntOpt.clone(), (std::sync::Arc::new(Util::getOption) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
            notOptimizedVarsBool = List::map(notOptimizedVarsBoolOpt.clone(), (std::sync::Arc::new(Util::getOption) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
            notOptimizedVarsString = List::map(notOptimizedVarsStringOpt.clone(), (std::sync::Arc::new(Util::getOption) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
            graphInfo = GraphML::createGraphInfo();
            let (__pa18, (_, __pa19)) = GraphML::addGraph((literal!("TasksGroupGraph")).clone(), true, graphInfo.clone())?;
            graphInfo = __pa18.clone();
            graphIdx = __pa19.clone();
            let (__pa20, _, (_, __pa21)) = GraphML::addGroupNode((literal!("TasksGroup")).clone(), graphIdx.clone(), false, (literal!("TG")).clone(), graphInfo.clone())?;
            graphInfo = __pa20.clone();
            graphIdx = __pa21.clone();
            annotInfo = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), (literal!("nothing")).clone());
            graphInfo = HpcOmTaskGraph::convertToGraphMLSccLevelSubgraph(iTaskGraph.clone(), iTaskGraphMeta.clone(), (iCriticalPathInfo.clone()).clone(), HpcOmTaskGraph::convertNodeListToEdgeTuples(listHead(iCriticalPaths.clone())?)?, HpcOmTaskGraph::convertNodeListToEdgeTuples(listHead(iCriticalPathsWoC.clone())?)?, iSccSimEqMapping.clone(), iSchedulerInfo.clone(), annotInfo.clone(), graphIdx.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: false, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true }, graphInfo.clone())?;
            let __pa22 = ::match_deref::match_deref! { match &(GraphML::getAttributeByNameAndTarget((literal!("ThreadId")).clone(), openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?) {
                Some((_, __pa22)) => __pa22.clone(),
                _ => bail!("pattern mismatch"),
            } };
            threadAttIdx = __pa22.clone();
            (_, adjacencyMatrix, _) = BackendDAEUtil::getAdjacencyMatrix(listHead(iEqSystems.clone())?, openmodelica_backend_types::BackendDAE::IndexType::ABSOLUTE, None, isInitial.clone())?;
            graphInfo = appendCacheLinesToGraph(cacheMap.clone(), metamodelica::arrayLength(iTaskGraph.clone()), eqSimCodeVarMapping.clone(), iEqSystems.clone(), simVarIdxMappingHashTable.clone(), eqCompMapping.clone(), scVarSolvedTaskMapping.clone(), iSchedulerInfo.clone(), threadAttIdx.clone(), sccNodeMapping.clone(), taskSolvedVarsMapping.clone(), taskUnsolvedVarsMapping.clone(), scVarCLMapping.clone(), scVarInfos.clone(), graphInfo.clone())?;
            fileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*iFileNamePrefix.clone()); __mm_s.push_str(&*literal!("ODE_schedule_CL.graphml")); ArcStr::from(__mm_s) }).clone();
            GraphML::dumpGraph(graphInfo.clone(), (fileName.clone()).clone())?;
            if Flags::isSet(Flags::HPCOM_MEMORY_OPT.clone())? {
                (varToArrayIndexMapping, varToIndexMapping, tmpMemoryMapOpt) = convertCacheToVarArrayMapping(cacheMap.clone(), CACHELINE_SIZE.clone(), stateVars.clone(), derivativeVars.clone(), aliasVars.clone(), intAliasVars.clone(), boolAliasVars.clone(), stringAliasVars.clone(), (VARSIZE_FLOAT.clone(), VARSIZE_INTEGER.clone(), VARSIZE_BOOLEAN.clone()), (notOptimizedVarsFloat.clone(), notOptimizedVarsInt.clone(), notOptimizedVarsBool.clone(), notOptimizedVarsString.clone()))?;
            } else {
                tmpMemoryMapOpt = None;
            }
            evaluateCacheBehaviour(varToIndexMapping.clone(), simVarIdxMappingHashTable.clone(), taskSolvedVarsMapping.clone(), taskUnsolvedVarsMapping.clone(), iTaskGraph.clone(), iTaskGraphT.clone(), iNumberOfThreads.clone(), CACHELINE_SIZE.clone(), simCodeVarTypes.clone(), iSchedulerInfo.clone());
            graphInfo = GraphML::createGraphInfo();
            let (__pa23, (_, __pa24)) = GraphML::addGraph((literal!("TasksGroupGraph")).clone(), true, graphInfo.clone())?;
            graphInfo = __pa23.clone();
            graphIdx = __pa24.clone();
            annotInfo = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), (literal!("nothing")).clone());
            graphInfo = HpcOmTaskGraph::convertToGraphMLSccLevelSubgraph(iTaskGraph.clone(), iTaskGraphMeta.clone(), (iCriticalPathInfo.clone()).clone(), HpcOmTaskGraph::convertNodeListToEdgeTuples(listHead(iCriticalPaths.clone())?)?, HpcOmTaskGraph::convertNodeListToEdgeTuples(listHead(iCriticalPathsWoC.clone())?)?, iSccSimEqMapping.clone(), iSchedulerInfo.clone(), annotInfo.clone(), graphIdx.clone(), HpcOmTaskGraph::GraphDumpOptions { visualizeCriticalPath: false, visualizeTaskStartAndFinishTime: false, visualizeTaskCalcTime: true, visualizeCommTime: true }, graphInfo.clone())?;
            let __pa25 = ::match_deref::match_deref! { match &(GraphML::getAttributeByNameAndTarget((literal!("ThreadId")).clone(), openmodelica_susan::GraphML::AttributeTarget::TARGET_NODE, graphInfo.clone())?) {
                Some((_, __pa25)) => __pa25.clone(),
                _ => bail!("pattern mismatch"),
            } };
            threadAttIdx = __pa25.clone();
            graphInfo = appendVariablesToGraph(taskSolvedVarsMapping.clone(), taskUnsolvedVarsMapping.clone(), metamodelica::arrayLength(scVarSolvedTaskMapping.clone()), graphIdx.clone(), threadAttIdx.clone(), simVarIdxMappingHashTable.clone(), allVarsMapping.clone(), scVarInfos.clone(), graphInfo.clone())?;
            fileName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("taskGraph")); __mm_s.push_str(&*iFileNamePrefix.clone()); __mm_s.push_str(&*literal!("ODE_schedule_vars.graphml")); ArcStr::from(__mm_s) }).clone();
            GraphML::dumpGraph(graphInfo.clone(), (fileName.clone()).clone())?;
            Ok(((tmpMemoryMapOpt.clone(), varToArrayIndexMapping.clone(), varToIndexMapping.clone()), CACHELINE_SIZE.clone(), VARSIZE_BOOLEAN.clone(), VARSIZE_FLOAT.clone(), VARSIZE_INTEGER.clone(), VARSIZE_STRING.clone(), adjacencyMatrix.clone(), algVars.clone(), algVarsCnt.clone(), aliasVars.clone(), aliasVarsCnt.clone(), allVarsMapping.clone(), annotInfo.clone(), boolAlgVars.clone(), boolAlgVarsCnt.clone(), boolAliasVars.clone(), boolAliasVarsCnt.clone(), boolParamVars.clone(), boolParamVarsCnt.clone(), cacheMap.clone(), clTaskMapping.clone(), derivativeVars.clone(), derivativeVarsCnt.clone(), discreteAlgVars.clone(), discreteAlgVarsCnt.clone(), eqSimCodeVarMapping.clone(), fileName.clone(), flatEqSimCodeVarMapping.clone(), graphIdx.clone(), graphInfo.clone(), inputVars.clone(), inputVarsCnt.clone(), intAlgVars.clone(), intAlgVarsCnt.clone(), intAliasVars.clone(), intAliasVarsCnt.clone(), intParamVars.clone(), intParamVarsCnt.clone(), nodeSccMapping.clone(), notOptimizedVars.clone(), notOptimizedVarsBool.clone(), notOptimizedVarsBoolOpt.clone(), notOptimizedVarsFloat.clone(), notOptimizedVarsFloatOpt.clone(), notOptimizedVarsInt.clone(), notOptimizedVarsIntOpt.clone(), notOptimizedVarsString.clone(), notOptimizedVarsStringOpt.clone(), numCL.clone(), outputVars.clone(), outputVarsCnt.clone(), paramVars.clone(), paramVarsCnt.clone(), scVarCLMapping.clone(), scVarInfos.clone(), scVarSolvedTaskMapping.clone(), scVarUnsolvedTaskMapping.clone(), sccEqMapping.clone(), sccNodeMapping.clone(), simCodeVarTypes.clone(), simCodeVars.clone(), stateVars.clone(), stateVarsCnt.clone(), stringAlgVars.clone(), stringAlgVarsCnt.clone(), stringAliasVars.clone(), stringAliasVarsCnt.clone(), stringParamVars.clone(), stringParamVarsCnt.clone(), taskSolvedVarsMapping.clone(), taskUnsolvedVarsMapping.clone(), threadAttIdx.clone(), tmpMemoryMapOpt.clone(), varCount.clone()))
        })() { CACHELINE_SIZE = __wb0; VARSIZE_BOOLEAN = __wb1; VARSIZE_FLOAT = __wb2; VARSIZE_INTEGER = __wb3; VARSIZE_STRING = __wb4; adjacencyMatrix = __wb5; algVars = __wb6; algVarsCnt = __wb7; aliasVars = __wb8; aliasVarsCnt = __wb9; allVarsMapping = __wb10; annotInfo = __wb11; boolAlgVars = __wb12; boolAlgVarsCnt = __wb13; boolAliasVars = __wb14; boolAliasVarsCnt = __wb15; boolParamVars = __wb16; boolParamVarsCnt = __wb17; cacheMap = __wb18; clTaskMapping = __wb19; derivativeVars = __wb20; derivativeVarsCnt = __wb21; discreteAlgVars = __wb22; discreteAlgVarsCnt = __wb23; eqSimCodeVarMapping = __wb24; fileName = __wb25; flatEqSimCodeVarMapping = __wb26; graphIdx = __wb27; graphInfo = __wb28; inputVars = __wb29; inputVarsCnt = __wb30; intAlgVars = __wb31; intAlgVarsCnt = __wb32; intAliasVars = __wb33; intAliasVarsCnt = __wb34; intParamVars = __wb35; intParamVarsCnt = __wb36; nodeSccMapping = __wb37; notOptimizedVars = __wb38; notOptimizedVarsBool = __wb39; notOptimizedVarsBoolOpt = __wb40; notOptimizedVarsFloat = __wb41; notOptimizedVarsFloatOpt = __wb42; notOptimizedVarsInt = __wb43; notOptimizedVarsIntOpt = __wb44; notOptimizedVarsString = __wb45; notOptimizedVarsStringOpt = __wb46; numCL = __wb47; outputVars = __wb48; outputVarsCnt = __wb49; paramVars = __wb50; paramVarsCnt = __wb51; scVarCLMapping = __wb52; scVarInfos = __wb53; scVarSolvedTaskMapping = __wb54; scVarUnsolvedTaskMapping = __wb55; sccEqMapping = __wb56; sccNodeMapping = __wb57; simCodeVarTypes = __wb58; simCodeVars = __wb59; stateVars = __wb60; stateVarsCnt = __wb61; stringAlgVars = __wb62; stringAlgVarsCnt = __wb63; stringAliasVars = __wb64; stringAliasVarsCnt = __wb65; stringParamVars = __wb66; stringParamVarsCnt = __wb67; taskSolvedVarsMapping = __wb68; taskUnsolvedVarsMapping = __wb69; threadAttIdx = __wb70; tmpMemoryMapOpt = __wb71; varCount = __wb72; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addInternalError((literal!("CreateMemoryMap failed!")).clone(), metamodelica::sourceInfo!("BackEnd/HpcOmMemory.mo"))?;
            Ok((None, iVarToArrayIndexMapping.clone(), iVarToIndexMapping.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((oMemoryMap, oVarToArrayIndexMapping, oVarToIndexMapping))
}

fn createCacheMapOptimized(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSimCodeVars: SimCodeVar::SimVars, mut iAllSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>, mut iScVarSolvedTaskMapping: metamodelica::Array<i32>, mut iScVarUnsolvedTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCacheLineSize: i32, mut iAllComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut iSchedule: Arc<HpcOmSimCode::Schedule>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iNumberOfThreads: i32, mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iScVarInfos: metamodelica::Array<ScVarInfo>) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut oNumCL: i32 = 0;
    let mut cacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut numCL: i32 = 0;
    let mut tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>> = metamodelica::nil();
    let mut scheduleInfo: metamodelica::Array<(i32, i32, metamodelica::Real)> = Default::default();
    let mut threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>> = Default::default();
    let mut allTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    (oCacheMap, oScVarCLMapping, oNumCL) = (::match_deref::match_deref! { match &(iSchedule.clone()) {
        Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: __esc_tasksOfLevels, useFixedAssignments: true } => {
            tasksOfLevels = (*__esc_tasksOfLevels).clone();
            metamodelica::print((literal!("Creating optimized cache map for fixed level scheduler\n")).clone());
            scheduleInfo = HpcOmScheduler::convertScheduleStrucToInfo(iSchedule.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
            (cacheMap, scVarCLMapping, numCL) = createCacheMapLevelFixedOptimized(iTaskGraph.clone(), iTaskGraphMeta.clone(), iAllSCVarsMapping.clone(), iSimCodeVarTypes.clone(), iScVarSolvedTaskMapping.clone(), iScVarUnsolvedTaskMapping.clone(), iCacheLineSize.clone(), iAllComponents.clone(), tasksOfLevels.clone(), iNumberOfThreads.clone(), scheduleInfo.clone(), iTaskSolvedVarsMapping.clone(), iTaskUnsolvedVarsMapping.clone(), iScVarInfos.clone())?;
            (cacheMap.clone(), scVarCLMapping.clone(), numCL.clone())
        },
        Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: __esc_threadTasks, .. } => {
            threadTasks = (*__esc_threadTasks).clone();
            metamodelica::print((literal!("Creating optimized cache map for thread scheduler\n")).clone());
            scheduleInfo = HpcOmScheduler::convertScheduleStrucToInfo(iSchedule.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
            (cacheMap, scVarCLMapping, numCL) = createCacheMapThreadOptimized(iTaskGraph.clone(), iTaskGraphMeta.clone(), iAllSCVarsMapping.clone(), iSimCodeVarTypes.clone(), iScVarSolvedTaskMapping.clone(), iScVarUnsolvedTaskMapping.clone(), iCacheLineSize.clone(), iAllComponents.clone(), threadTasks.clone(), iNumberOfThreads.clone(), scheduleInfo.clone(), iTaskSolvedVarsMapping.clone(), iTaskUnsolvedVarsMapping.clone(), iScVarInfos.clone())?;
            (cacheMap.clone(), scVarCLMapping.clone(), numCL.clone())
        },
        Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: __esc_allTasks, .. } } => {
            allTasks = (*__esc_allTasks).clone();
            metamodelica::print((literal!("Creating optimized cache map for empty scheduler\n")).clone());
            threadTasks = arrayCreate(1, allTasks.clone());
            scheduleInfo = HpcOmScheduler::convertScheduleStrucToInfo(iSchedule.clone(), metamodelica::arrayLength(iTaskGraph.clone()))?;
            (cacheMap, scVarCLMapping, numCL) = createCacheMapThreadOptimized(iTaskGraph.clone(), iTaskGraphMeta.clone(), iAllSCVarsMapping.clone(), iSimCodeVarTypes.clone(), iScVarSolvedTaskMapping.clone(), iScVarUnsolvedTaskMapping.clone(), iCacheLineSize.clone(), iAllComponents.clone(), threadTasks.clone(), 1, scheduleInfo.clone(), iTaskSolvedVarsMapping.clone(), iTaskUnsolvedVarsMapping.clone(), iScVarInfos.clone())?;
            (cacheMap.clone(), scVarCLMapping.clone(), numCL.clone())
        },
        _ => {
            metamodelica::print((literal!("No optimized cache map for the selected scheduler avaiable. Using default cacheMap!\n")).clone());
            (cacheMap, scVarCLMapping, numCL) = createCacheMapDefault(iAllSCVarsMapping.clone(), iCacheLineSize.clone(), iSimCodeVars.clone(), iScVarSolvedTaskMapping.clone(), iSchedulerInfo.clone(), iSimCodeVarTypes.clone())?;
            (cacheMap.clone(), scVarCLMapping.clone(), numCL.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapLevelOptimized(mut iAllSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>, mut iScVarTaskMapping: metamodelica::Array<i32>, mut iCacheLineSize: i32, mut iAllComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut iTasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut iNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut oNumCL: i32 = 0;
    let mut cacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut cacheMapMeta: CacheMapMeta = <CacheMapMeta as ::std::default::Default>::default();
    let mut numCL: i32 = 0;
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    cacheMap = CacheMap::CACHEMAP { cacheLineSize: iCacheLineSize.clone(), cacheVariables: metamodelica::nil(), cacheLinesFloat: metamodelica::nil(), cacheLinesInt: metamodelica::nil(), cacheLinesBool: metamodelica::nil() };
    scVarCLMapping = arrayCreate(metamodelica::arrayLength(iAllSCVarsMapping.clone()), (-1, -1));
    numCL = 0;
    cacheMapMeta = CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping.clone(), simCodeVarTypes: iSimCodeVarTypes.clone(), scVarCLMapping: scVarCLMapping.clone() };
    (_, cacheMap, cacheMapMeta, numCL) = List::fold1(iTasksOfLevels.clone(), (std::sync::Arc::new(createCacheMapLevelOptimized0) as std::sync::Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList, metamodelica::Array<Arc<metamodelica::List<i32>>>, (Arc<metamodelica::List<i32>>, CacheMap, CacheMapMeta, i32)) -> Result<(Arc<metamodelica::List<i32>>, CacheMap, CacheMapMeta, i32)> + 'static>), iNodeSimCodeVarMapping.clone(), (metamodelica::nil(), cacheMap.clone(), cacheMapMeta.clone(), numCL.clone()))?;
    oCacheMap = cacheMap.clone();
    let CacheMapMeta { scVarCLMapping: __pa0, .. } = (cacheMapMeta.clone()) else { bail!("pattern mismatch") };
    oScVarCLMapping = __pa0.clone();
    oNumCL = numCL.clone();
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapLevelOptimized0(mut iLevelTasks: HpcOmSimCode::TaskList, mut iNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iInfo: (Arc<metamodelica::List<i32>>, CacheMap, CacheMapMeta, i32)) -> Result<(Arc<metamodelica::List<i32>>, CacheMap, CacheMapMeta, i32)> {
    let mut oInfo: (Arc<metamodelica::List<i32>>, CacheMap, CacheMapMeta, i32) = (metamodelica::nil(), <CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0);
    let mut createdCL: i32 = 0;
    let mut numCL: i32 = 0;
    let mut cacheLineSize: i32 = 0;
    let mut allCL: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut availableCL: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut availableCLold: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut writtenCL: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cacheLinesPrevLevel: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut detailedCacheLineInfo: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut cacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut cacheMapMeta: CacheMapMeta = <CacheMapMeta as ::std::default::Default>::default();
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    (cacheLinesPrevLevel, cacheMap, cacheMapMeta, numCL) = iInfo.clone();
    allCL = List::intRange(numCL.clone());
    let CacheMap::CACHEMAP { cacheLinesFloat: __pa0, cacheLineSize: __pa1, .. } = (cacheMap.clone()) else { bail!("pattern mismatch") };
    cacheLinesFloat = __pa0.clone();
    cacheLineSize = __pa1.clone();
    availableCLold = List::setDifferenceIntN(allCL.clone(), cacheLinesPrevLevel.clone(), numCL.clone())?;
    detailedCacheLineInfo = createDetailedCacheMapInformation(availableCLold.clone(), cacheLinesFloat.clone(), cacheLineSize.clone())?;
    detailedCacheLineInfo = detailedCacheLineInfo.clone().reverse();
    (cacheMap, cacheMapMeta, createdCL, detailedCacheLineInfo) = List::fold1(getTaskListTasks(iLevelTasks.clone()), (std::sync::Arc::new(createCacheMapLevelOptimizedForTask) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, metamodelica::Array<Arc<metamodelica::List<i32>>>, (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>)) -> Result<(CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>)> + 'static>), iNodeSimCodeVarMapping.clone(), (cacheMap.clone(), cacheMapMeta.clone(), 0, detailedCacheLineInfo.clone()))?;
    availableCL = List::map(detailedCacheLineInfo.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)))?;
    writtenCL = List::setDifferenceIntN(availableCLold.clone(), availableCL.clone(), numCL.clone())?;
    writtenCL = listAppend(writtenCL.clone(), if (intLe(numCL.clone() + 1, numCL.clone() + createdCL.clone())) {List::intRange2(numCL.clone() + 1, numCL.clone() + createdCL.clone())} else {metamodelica::nil()});
    oInfo = (writtenCL.clone(), cacheMap.clone(), cacheMapMeta.clone(), numCL.clone() + createdCL.clone());
    Ok(oInfo)
}

fn createCacheMapLevelOptimizedForTask(mut iTask: Arc<HpcOmSimCode::Task>, mut iNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>)) -> Result<(CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>) = (<CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0, metamodelica::nil());
    let mut nodeIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>) = (<CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0, metamodelica::nil());
    oInfo = (::match_deref::match_deref! { match &(iTask.clone()) {
        Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc: __esc_nodeIdc, .. } => {
            nodeIdc = (*__esc_nodeIdc).clone();
            tmpInfo = List::fold(nodeIdc.clone(), (std::sync::Arc::new({ let __pe_b1 = -1; let __pe_b2 = iNodeSimCodeVarMapping.clone(); move |__pe_a0, __pe_a3| appendNodeVarsToCacheMap(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(i32, (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>)) -> Result<(CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>)> + 'static>), iInfo.clone())?;
            tmpInfo.clone()
        },
        _ => {
            metamodelica::print((literal!("createCacheMapLevelOptimized1: Unsupported task type\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oInfo)
}

fn createCacheMapLevelFixedOptimized(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iAllSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>, mut iScVarSolvedTaskMapping: metamodelica::Array<i32>, mut iScVarUnsolvedTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCacheLineSize: i32, mut iAllComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut iTasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut iNumberOfThreads: i32, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iScVarInfos: metamodelica::Array<ScVarInfo>) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut oNumCL: i32 = 0;
    let mut cacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut cacheMapMeta: CacheMapMeta = <CacheMapMeta as ::std::default::Default>::default();
    let mut handledVariables: metamodelica::Array<bool> = Default::default();
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut threadCacheLines: metamodelica::Array<(Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)> = Default::default();
    let mut sharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))> = Default::default();
    cacheMap = CacheMap::CACHEMAP { cacheLineSize: iCacheLineSize.clone(), cacheVariables: metamodelica::nil(), cacheLinesFloat: metamodelica::nil(), cacheLinesInt: metamodelica::nil(), cacheLinesBool: metamodelica::nil() };
    scVarCLMapping = arrayCreate(metamodelica::arrayLength(iAllSCVarsMapping.clone()), (-1, -1));
    handledVariables = arrayCreate(metamodelica::arrayLength(iSimCodeVarTypes.clone()), false);
    oNumCL = 0;
    threadCacheLines = arrayCreate(iNumberOfThreads.clone(), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()));
    sharedCacheLines = arrayCreate(iNumberOfThreads.clone(), ((metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil())));
    cacheMapMeta = CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping.clone(), simCodeVarTypes: iSimCodeVarTypes.clone(), scVarCLMapping: scVarCLMapping.clone() };
    (cacheMap, cacheMapMeta, oNumCL, _) = List::fold(iTasksOfLevels.clone(), (std::sync::Arc::new({ let __pe_b1 = iTaskGraph.clone(); let __pe_b2 = iTaskGraphMeta.clone(); let __pe_b3 = iNumberOfThreads.clone(); let __pe_b4 = iScVarInfos.clone(); let __pe_b5 = iTaskSolvedVarsMapping.clone(); let __pe_b6 = iTaskUnsolvedVarsMapping.clone(); let __pe_b7 = handledVariables.clone(); let __pe_b8 = iSchedulerInfo.clone(); let __pe_b9 = threadCacheLines.clone(); let __pe_b10 = sharedCacheLines.clone(); move |__pe_a0, __pe_a11| createCacheMapLevelFixedOptimizedForLevel(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_b8.clone(), __pe_b9.clone(), __pe_b10.clone(), __pe_a11) }) as std::sync::Arc<dyn ::std::ops::Fn(HpcOmSimCode::TaskList, (CacheMap, CacheMapMeta, i32, i32)) -> Result<(CacheMap, CacheMapMeta, i32, i32)> + 'static>), (cacheMap.clone(), cacheMapMeta.clone(), oNumCL.clone(), 1))?;
    for mut threadIdx in 1..=iNumberOfThreads.clone() {
        cacheMap = createCacheMapFromThreadAndSharedCLs(metamodelica::arrayGet(threadCacheLines.clone(), threadIdx.clone())?, metamodelica::arrayGet(sharedCacheLines.clone(), threadIdx.clone())?, cacheMap.clone())?;
    }
    oCacheMap = cacheMap.clone();
    let CacheMapMeta { scVarCLMapping: __pa0, .. } = (cacheMapMeta.clone()) else { bail!("pattern mismatch") };
    oScVarCLMapping = __pa0.clone();
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapLevelFixedOptimizedForLevel(mut iLevelTasks: HpcOmSimCode::TaskList, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iNumberOfThreads: i32, mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iHandledVariables: metamodelica::Array<bool>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iThreadCacheLines: metamodelica::Array<(Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)>, mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>, mut iInfo: (CacheMap, CacheMapMeta, i32, i32)) -> Result<(CacheMap, CacheMapMeta, i32, i32)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32, i32) = (<CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0, 0);
    let mut createdCL: i32 = 0;
    let mut numCL: i32 = 0;
    let mut cacheLineSize: i32 = 0;
    let mut level: i32 = 0;
    let mut allCL: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut cacheMapMeta: CacheMapMeta = <CacheMapMeta as ::std::default::Default>::default();
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    (cacheMap, cacheMapMeta, numCL, level) = iInfo.clone();
    let CacheMap::CACHEMAP { cacheVariables: __pa0, .. } = (cacheMap.clone()) else { bail!("pattern mismatch") };
    cacheVariables = __pa0.clone();
    allCL = List::intRange(numCL.clone());
    let CacheMap::CACHEMAP { cacheLinesFloat: __pa1, cacheLineSize: __pa2, .. } = (cacheMap.clone()) else { bail!("pattern mismatch") };
    cacheLinesFloat = __pa1.clone();
    cacheLineSize = __pa2.clone();
    (cacheMap, cacheMapMeta, createdCL) = List::fold(getTaskListTasks(iLevelTasks.clone()), (std::sync::Arc::new({ let __pe_b1 = iTaskGraph.clone(); let __pe_b2 = iTaskGraphMeta.clone(); let __pe_b3 = iSchedulerInfo.clone(); let __pe_b4 = iNumberOfThreads.clone(); let __pe_b5 = level.clone(); let __pe_b6 = iScVarInfos.clone(); let __pe_b7 = iTaskSolvedVarsMapping.clone(); let __pe_b8 = iTaskUnsolvedVarsMapping.clone(); let __pe_b9 = iHandledVariables.clone(); let __pe_b10 = iThreadCacheLines.clone(); let __pe_b11 = iSharedCacheLines.clone(); move |__pe_a0, __pe_a12| createCacheMapLevelFixedOptimizedForTask(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_b8.clone(), __pe_b9.clone(), __pe_b10.clone(), __pe_b11.clone(), __pe_a12) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> + 'static>), (cacheMap.clone(), cacheMapMeta.clone(), numCL.clone()))?;
    let CacheMap::CACHEMAP { cacheVariables: __pa3, .. } = (cacheMap.clone()) else { bail!("pattern mismatch") };
    cacheVariables = __pa3.clone();
    oInfo = (cacheMap.clone(), cacheMapMeta.clone(), createdCL.clone(), level.clone() + 1);
    Ok(oInfo)
}

fn createCacheMapLevelFixedOptimizedForTask(mut iTask: Arc<HpcOmSimCode::Task>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iNumberOfThreads: i32, mut iLevel: i32, mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iHandledVariables: metamodelica::Array<bool>, mut iThreadCacheLines: metamodelica::Array<(Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)>, mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>, mut iInfo: (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32) = (<CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0);
    let mut nodeIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut solvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unsolvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut cacheMapMeta: CacheMapMeta = <CacheMapMeta as ::std::default::Default>::default();
    let mut tmpInfo: (CacheMap, CacheMapMeta, i32) = (<CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0);
    let mut threadIdx: i32 = 0;
    let mut numNewCL: i32 = 0;
    let mut allSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>> = Default::default();
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    oInfo = (::match_deref::match_deref! { match &((iTask.clone(), iInfo.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc: __esc_nodeIdc, threadIdx: Some(__esc_threadIdx), .. }, (__esc_cacheMap, __esc_cacheMapMeta @ CacheMapMeta { allSCVarsMapping: __esc_allSCVarsMapping, .. }, __esc_numNewCL)) => {
            nodeIdc = (*__esc_nodeIdc).clone();
            threadIdx = (*__esc_threadIdx).clone();
            cacheMap = (*__esc_cacheMap).clone();
            cacheMapMeta = (*__esc_cacheMapMeta).clone();
            allSCVarsMapping = (*__esc_allSCVarsMapping).clone();
            numNewCL = (*__esc_numNewCL).clone();
            solvedVars = List::flatten(List::map(nodeIdc.clone(), (std::sync::Arc::new({ let __pe_b0 = iTaskSolvedVarsMapping.clone(); move |__pe_a1| metamodelica::arrayGet(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<_> + 'static>))?)?;
            unsolvedVars = getUnsolvedVarsByNodeList(nodeIdc.clone(), metamodelica::arrayLength(iScVarInfos.clone()), iTaskUnsolvedVarsMapping.clone())?;
            tmpInfo = List::fold(listAppend(solvedVars.clone(), unsolvedVars.clone()), (std::sync::Arc::new({ let __pe_b1 = threadIdx.clone(); let __pe_b2 = iScVarInfos.clone(); let __pe_b3 = iHandledVariables.clone(); let __pe_b4: Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, _, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static> = (std::sync::Arc::new(findMatchingSharedCLLevelfix) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, (i32, i32), metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>); let __pe_b5 = (iLevel.clone(), threadIdx.clone()); let __pe_b6: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, _) -> Result<PartlyFilledCacheLine> + 'static> = (std::sync::Arc::new(createSharedClLevelFix) as std::sync::Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, (i32, i32)) -> Result<PartlyFilledCacheLine> + 'static>); let __pe_b7 = iThreadCacheLines.clone(); let __pe_b8 = iSharedCacheLines.clone(); move |__pe_a0, __pe_a9| createCacheMapOptimizedForTask1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_b8.clone(), __pe_a9) }) as std::sync::Arc<dyn ::std::ops::Fn(i32, (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> + 'static>), (cacheMap.clone(), cacheMapMeta.clone(), numNewCL.clone()))?;
            let CacheMap::CACHEMAP { cacheVariables: __pa0, .. } = (Util::tuple31(tmpInfo.clone())) else { bail!("pattern mismatch") };
            cacheVariables = __pa0.clone();
            tmpInfo.clone()
        },
        (Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc: __esc_nodeIdc, threadIdx: None, .. }, _) => {
            nodeIdc = (*__esc_nodeIdc).clone();
            metamodelica::print((literal!("createCacheMapLevelOptimized1: Calctask without threadIdx given\n")).clone());
            bail!("fail")
        },
        _ => {
            metamodelica::print((literal!("createCacheMapLevelOptimized1: Unsupported task type\n")).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oInfo)
}

fn getUnsolvedVarsByNodeList(mut iNodeList: Arc<metamodelica::List<i32>>, mut iVarCount: i32, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oUnsolvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varMarks: metamodelica::Array<bool> = Default::default();
    let mut nodeIdx: i32 = 0;
    let mut varIdx: i32 = 0;
    let mut nodeUnsolvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpUnsolvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    varMarks = arrayCreate(iVarCount.clone(), false);
    for mut nodeIdx in &*iNodeList.clone() {
        let mut nodeIdx = nodeIdx.clone();
        nodeUnsolvedVars = metamodelica::arrayGet(iTaskUnsolvedVarsMapping.clone(), nodeIdx.clone())?;
        for mut varIdx in &*nodeUnsolvedVars.clone() {
            let mut varIdx = varIdx.clone();
            if boolNot(metamodelica::arrayGet(varMarks.clone(), varIdx.clone())?) {
                tmpUnsolvedVars = metamodelica::cons(varIdx.clone(), tmpUnsolvedVars.clone());
                varMarks = metamodelica::arrayUpdate(varMarks.clone(), varIdx.clone(), true)?;
            }
        }
    }
    oUnsolvedVars = tmpUnsolvedVars.clone();
    Ok(oUnsolvedVars)
}

fn createCacheMapThreadOptimized(mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iAllSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>, mut iScVarSolvedTaskMapping: metamodelica::Array<i32>, mut iScVarUnsolvedTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iCacheLineSize: i32, mut iAllComponents: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut iThreadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut iNumberOfThreads: i32, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iScVarInfos: metamodelica::Array<ScVarInfo>) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut oNumCL: i32 = 0;
    let mut threadCacheLines: metamodelica::Array<(Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)> = Default::default();
    let mut sharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))> = Default::default();
    let mut tmpCacheInfo: (CacheMap, CacheMapMeta, i32) = (<CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0);
    let mut cacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut cacheMapMeta: CacheMapMeta = <CacheMapMeta as ::std::default::Default>::default();
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut handledVariables: metamodelica::Array<bool> = Default::default();
    threadCacheLines = arrayCreate(iNumberOfThreads.clone(), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil()));
    sharedCacheLines = arrayCreate(iNumberOfThreads.clone(), ((metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), (metamodelica::nil(), metamodelica::nil(), metamodelica::nil())));
    handledVariables = arrayCreate(metamodelica::arrayLength(iSimCodeVarTypes.clone()), false);
    cacheMap = CacheMap::CACHEMAP { cacheLineSize: iCacheLineSize.clone(), cacheVariables: metamodelica::nil(), cacheLinesFloat: metamodelica::nil(), cacheLinesInt: metamodelica::nil(), cacheLinesBool: metamodelica::nil() };
    scVarCLMapping = arrayCreate(metamodelica::arrayLength(iAllSCVarsMapping.clone()), (-1, -1));
    oNumCL = 0;
    cacheMapMeta = CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping.clone(), simCodeVarTypes: iSimCodeVarTypes.clone(), scVarCLMapping: scVarCLMapping.clone() };
    tmpCacheInfo = (cacheMap.clone(), cacheMapMeta.clone(), oNumCL.clone());
    for mut threadIdx in 1..=iNumberOfThreads.clone() {
        (cacheMap, cacheMapMeta, oNumCL) = List::fold(metamodelica::arrayGet(iThreadTasks.clone(), threadIdx.clone())?, (std::sync::Arc::new({ let __pe_b1 = iTaskGraph.clone(); let __pe_b2 = iTaskGraphMeta.clone(); let __pe_b3 = iSchedulerInfo.clone(); let __pe_b4 = iTaskSolvedVarsMapping.clone(); let __pe_b5 = iTaskUnsolvedVarsMapping.clone(); let __pe_b6 = handledVariables.clone(); let __pe_b7 = iNumberOfThreads.clone(); let __pe_b8: Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, _, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static> = (std::sync::Arc::new(findMatchingSharedCLThread) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, i32, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>); let __pe_b9 = 0; let __pe_b10: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, _) -> Result<PartlyFilledCacheLine> + 'static> = (std::sync::Arc::new(fnptr!(createSharedClThread, Option<PartlyFilledCacheLine>, CacheLineMap, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, i32) -> Result<PartlyFilledCacheLine> + 'static>); let __pe_b11 = threadCacheLines.clone(); let __pe_b12 = sharedCacheLines.clone(); let __pe_b13 = iScVarInfos.clone(); move |__pe_a0, __pe_a14| createCacheMapOptimizedForTask(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_b8.clone(), __pe_b9.clone(), __pe_b10.clone(), __pe_b11.clone(), __pe_b12.clone(), __pe_b13.clone(), __pe_a14) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<HpcOmSimCode::Task>, (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> + 'static>), tmpCacheInfo.clone())?;
        cacheMap = createCacheMapFromThreadAndSharedCLs(metamodelica::arrayGet(threadCacheLines.clone(), threadIdx.clone())?, metamodelica::arrayGet(sharedCacheLines.clone(), threadIdx.clone())?, cacheMap.clone())?;
        tmpCacheInfo = (cacheMap.clone(), cacheMapMeta.clone(), oNumCL.clone());
    }
    oCacheMap = Util::tuple31(tmpCacheInfo.clone());
    let CacheMapMeta { scVarCLMapping: __pa0, .. } = (cacheMapMeta.clone()) else { bail!("pattern mismatch") };
    oScVarCLMapping = __pa0.clone();
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapOptimizedForTask<T: Clone + 'static>(mut iTask: Arc<HpcOmSimCode::Task>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iHandledVariables: metamodelica::Array<bool>, mut iNumberOfThreads: i32, mut iSharedClSelectFunction: Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, T, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>, mut iCompareFuncArgument: T, mut iFactoryMethod: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>, mut iThreadCacheLines: metamodelica::Array<(Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)>, mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>, mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iInfo: (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> {
    pub type HeuristicFunction<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, T, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>;

    pub type FactoryMethod<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>;

    let mut oInfo: (CacheMap, CacheMapMeta, i32) = (<CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0);
    let mut threadIdx: i32 = 0;
    let mut taskIdx: i32 = 0;
    let mut solvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unsolvedVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut cacheMapMeta: CacheMapMeta = <CacheMapMeta as ::std::default::Default>::default();
    let mut numOfCLs: i32 = 0;
    let mut tmpInfo: (CacheMap, CacheMapMeta, i32) = (<CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0);
    let mut allSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>> = Default::default();
    oInfo = (::match_deref::match_deref! { match &((iTask.clone(), iInfo.clone())) {
        (Deref @ HpcOmSimCode::Task::CALCTASK { index: __esc_taskIdx, threadIdx: __esc_threadIdx, .. }, (__esc_cacheMap, __esc_cacheMapMeta @ CacheMapMeta { allSCVarsMapping: __esc_allSCVarsMapping, .. }, __esc_numOfCLs)) => {
            taskIdx = (*__esc_taskIdx).clone();
            threadIdx = (*__esc_threadIdx).clone();
            cacheMap = (*__esc_cacheMap).clone();
            cacheMapMeta = (*__esc_cacheMapMeta).clone();
            allSCVarsMapping = (*__esc_allSCVarsMapping).clone();
            numOfCLs = (*__esc_numOfCLs).clone();
            solvedVars = metamodelica::arrayGet(iTaskSolvedVarsMapping.clone(), taskIdx.clone())?;
            unsolvedVars = metamodelica::arrayGet(iTaskUnsolvedVarsMapping.clone(), taskIdx.clone())?;
            vars = List::sort(listAppend(solvedVars.clone(), unsolvedVars.clone()), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            tmpInfo = List::fold(vars.clone(), (std::sync::Arc::new({ let __pe_b1 = threadIdx.clone(); let __pe_b2 = iScVarInfos.clone(); let __pe_b3 = iHandledVariables.clone(); let __pe_b4: Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, _, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static> = iSharedClSelectFunction.clone(); let __pe_b5 = iCompareFuncArgument.clone(); let __pe_b6: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, _) -> Result<PartlyFilledCacheLine> + 'static> = iFactoryMethod.clone(); let __pe_b7 = iThreadCacheLines.clone(); let __pe_b8 = iSharedCacheLines.clone(); move |__pe_a0, __pe_a9| createCacheMapOptimizedForTask1(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_b8.clone(), __pe_a9) }) as std::sync::Arc<dyn ::std::ops::Fn(i32, (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> + 'static>), (cacheMap.clone(), cacheMapMeta.clone(), numOfCLs.clone()))?;
            tmpInfo.clone()
        },
        (Deref @ HpcOmSimCode::Task::DEPTASK { sourceTask: _, .. }, (__esc_cacheMap, __esc_cacheMapMeta @ CacheMapMeta { allSCVarsMapping: __esc_allSCVarsMapping, .. }, __esc_numOfCLs)) => {
            cacheMap = (*__esc_cacheMap).clone();
            cacheMapMeta = (*__esc_cacheMapMeta).clone();
            allSCVarsMapping = (*__esc_allSCVarsMapping).clone();
            numOfCLs = (*__esc_numOfCLs).clone();
            iInfo.clone()
        },
        _ => {
            metamodelica::print((literal!("createCacheMapThreadOptimizedForTask failed!\n")).clone());
            iInfo.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oInfo)
}

fn createCacheMapOptimizedForTask1<T: Clone + 'static>(mut iScVar: i32, mut iThreadIdx: i32, mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iHandledVariables: metamodelica::Array<bool>, mut iSharedClSelectFunction: Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, T, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>, mut iCompareFuncArgument: T, mut iFactoryMethod: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>, mut iThreadCacheLines: metamodelica::Array<(Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)>, mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>, mut iInfo: (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> {
    pub type HeuristicFunction<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, T, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>;

    pub type FactoryMethod<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>;

    let mut oInfo: (CacheMap, CacheMapMeta, i32) = (<CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0);
    let mut isShared: bool = false;
    let mut cacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut cacheMapMeta: CacheMapMeta = <CacheMapMeta as ::std::default::Default>::default();
    let mut numOfCLs: i32 = 0;
    let mut ownerThread: i32 = 0;
    (cacheMap, cacheMapMeta, numOfCLs) = iInfo.clone();
    let ScVarInfo { ownerThread: __pa0, isShared: __pa1 } = (metamodelica::arrayGet(iScVarInfos.clone(), iScVar.clone())?) else { bail!("pattern mismatch") };
    ownerThread = __pa0.clone();
    isShared = __pa1.clone();
    if boolAnd(boolNot(boolAnd(intEq(ownerThread.clone(), -1), isShared.clone())), boolNot(metamodelica::arrayGet(iHandledVariables.clone(), iScVar.clone())?)) {
        if isShared.clone() {
            (cacheMap, cacheMapMeta, numOfCLs) = addVarsToSharedCL(list![iScVar.clone()], iSharedClSelectFunction.clone(), iFactoryMethod.clone(), iThreadIdx.clone(), iCompareFuncArgument.clone(), iSharedCacheLines.clone(), (cacheMap.clone(), cacheMapMeta.clone(), numOfCLs.clone()))?;
        } else {
            (cacheMap, cacheMapMeta, numOfCLs) = addVarsToThreadCL(list![iScVar.clone()], iThreadIdx.clone(), iThreadCacheLines.clone(), (cacheMap.clone(), cacheMapMeta.clone(), numOfCLs.clone()))?;
        }
    }
    metamodelica::arrayUpdate(iHandledVariables.clone(), iScVar.clone(), true)?;
    oInfo = (cacheMap.clone(), cacheMapMeta.clone(), numOfCLs.clone());
    Ok(oInfo)
}

fn createVarInfos(mut iScVarSolvedTaskMapping: metamodelica::Array<i32>, mut iScVarUnsolvedTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> Result<metamodelica::Array<ScVarInfo>> {
    let mut oVarInfos: metamodelica::Array<ScVarInfo> = Default::default();
    let mut tmpVarInfos: metamodelica::Array<ScVarInfo> = Default::default();
    let mut scVarIdx: i32 = 0;
    let mut numberOfScVars: i32 = 0;
    numberOfScVars = metamodelica::arrayLength(iScVarSolvedTaskMapping.clone());
    tmpVarInfos = arrayCreate(numberOfScVars.clone(), ScVarInfo { ownerThread: -1, isShared: false });
    for mut scVarIdx in 1..=numberOfScVars.clone() {
        tmpVarInfos = metamodelica::arrayUpdate(tmpVarInfos.clone(), scVarIdx.clone(), getVarInfoByScVarIdx(scVarIdx.clone(), iScVarSolvedTaskMapping.clone(), iScVarUnsolvedTaskMapping.clone(), iSchedulerInfo.clone())?)?;
    }
    oVarInfos = tmpVarInfos.clone();
    Ok(oVarInfos)
}

fn getVarInfoByScVarIdx(mut iScVarIdx: i32, mut iScVarSolvedTaskMapping: metamodelica::Array<i32>, mut iScVarUnsolvedTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> Result<ScVarInfo> {
    let mut oVarInfo: ScVarInfo = <ScVarInfo as ::std::default::Default>::default();
    let mut solvingThreadIdx: i32 = 0;
    let mut solvingTaskIdx: i32 = 0;
    let mut listLen: i32 = 0;
    let mut owner: i32 = -1;
    let mut isShared: bool = false;
    let mut threads: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unsolvingThreadIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut unsolvingTaskIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    solvingTaskIdx = metamodelica::arrayGet(iScVarSolvedTaskMapping.clone(), iScVarIdx.clone())?;
    unsolvingTaskIdc = metamodelica::arrayGet(iScVarUnsolvedTaskMapping.clone(), iScVarIdx.clone())?;
    if intGt(solvingTaskIdx.clone(), 0) {
        solvingThreadIdx = Util::tuple31(metamodelica::arrayGet(iSchedulerInfo.clone(), solvingTaskIdx.clone())?);
        owner = solvingThreadIdx.clone();
        threads = metamodelica::cons(owner.clone(), threads.clone());
    }
    listLen = (unsolvingTaskIdc.clone().len() as i32);
    unsolvingThreadIdc = List::map(List::map(unsolvingTaskIdc.clone(), (std::sync::Arc::new({ let __pe_b0 = iSchedulerInfo.clone(); move |__pe_a1| metamodelica::arrayGet(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<_> + 'static>))?, std::sync::Arc::new(fnptr!(Util::tuple31, _)))?;
    if intEq(listLen.clone(), 1) {
        if intLt(owner.clone(), 0) {
            owner = listHead(unsolvingThreadIdc.clone())?;
            threads = metamodelica::cons(owner.clone(), threads.clone());
        } else {
            isShared = true;
        }
    }
    if intGt(listLen.clone(), 1) {
        threads = List::unique(listAppend(unsolvingThreadIdc.clone(), threads.clone()));
        isShared = true;
    }
    oVarInfo = ScVarInfo { ownerThread: owner.clone(), isShared: isShared.clone() };
    Ok(oVarInfo)
}

fn addVarsToThreadCL(mut iNodeVars: Arc<metamodelica::List<i32>>, mut iThreadIdx: i32, mut iThreadCacheLines: metamodelica::Array<(Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)>, mut iInfo: (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32) = (<CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0);
    let mut lastCL: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    let mut cacheVariable: SimCodeVar::SimVar = <SimCodeVar::SimVar as ::std::default::Default>::default();
    let mut allSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>> = Default::default();
    let mut varIdx: i32 = 0;
    let mut varDataType: i32 = 0;
    let mut varNumBytesRequired: i32 = 0;
    let mut numCLs: i32 = 0;
    let mut cacheLineSize: i32 = 0;
    let mut simCodeVarTypes: metamodelica::Array<(i32, i32, i32)> = Default::default();
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut fullCLs: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut threadCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut lastCLidx: i32 = 0;
    let mut lastCLnumBytesFree: i32 = 0;
    let mut lastCLentries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut varEntry: CacheLineEntry = <CacheLineEntry as ::std::default::Default>::default();
    let mut cacheVarName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut threadCacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut threadCacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut threadCacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let (CacheMap::CACHEMAP { cacheLineSize: __pa0, cacheVariables: __pa1, cacheLinesFloat: __pa2, cacheLinesInt: __pa3, cacheLinesBool: __pa4 }, CacheMapMeta { allSCVarsMapping: __pa5, simCodeVarTypes: __pa6, scVarCLMapping: __pa7 }, __pa8) = (iInfo.clone()) else { bail!("pattern mismatch") };
    cacheLineSize = __pa0.clone();
    cacheVariables = __pa1.clone();
    cacheLinesFloat = __pa2.clone();
    cacheLinesInt = __pa3.clone();
    cacheLinesBool = __pa4.clone();
    allSCVarsMapping = __pa5.clone();
    simCodeVarTypes = __pa6.clone();
    scVarCLMapping = __pa7.clone();
    numCLs = __pa8.clone();
    for mut varIdx in &*iNodeVars.clone() {
        let mut varIdx = varIdx.clone();
        (varDataType, varNumBytesRequired, _) = metamodelica::arrayGet(simCodeVarTypes.clone(), varIdx.clone())?;
        (threadCacheLinesFloat, threadCacheLinesInt, threadCacheLinesBool, threadCacheLines) = getCacheLineForVarType(varDataType.clone(), metamodelica::arrayGet(iThreadCacheLines.clone(), iThreadIdx.clone())?);
        if !(threadCacheLines.clone().is_empty()) {
            let (__pa9, __pa10) = ::match_deref::match_deref! { match &(threadCacheLines.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa9, tail: __pa10 } => (__pa9.clone(), __pa10.clone()),
                _ => bail!("pattern mismatch"),
            } };
            lastCL = __pa9.clone();
            fullCLs = __pa10.clone();
        } else {
            lastCLidx = numCLs.clone() + 1;
            lastCLnumBytesFree = cacheLineSize.clone();
            lastCLentries = metamodelica::nil();
            lastCL = CacheLineMap { idx: lastCLidx.clone(), numBytesFree: lastCLnumBytesFree.clone(), entries: lastCLentries.clone() };
            numCLs = numCLs.clone() + 1;
            fullCLs = metamodelica::nil();
        }
        let CacheLineMap { idx: __pa11, numBytesFree: __pa12, entries: __pa13 } = (lastCL.clone()) else { bail!("pattern mismatch") };
        lastCLidx = __pa11.clone();
        lastCLnumBytesFree = __pa12.clone();
        lastCLentries = __pa13.clone();
        if intLt(lastCLnumBytesFree.clone(), varNumBytesRequired.clone()) {
            fullCLs = metamodelica::cons(lastCL.clone(), fullCLs.clone());
            lastCLidx = numCLs.clone() + 1;
            lastCLnumBytesFree = cacheLineSize.clone();
            lastCLentries = metamodelica::nil();
            lastCL = CacheLineMap { idx: lastCLidx.clone(), numBytesFree: lastCLnumBytesFree.clone(), entries: lastCLentries.clone() };
            numCLs = numCLs.clone() + 1;
        }
        let (__pa15, __pa14) = ::match_deref::match_deref! { match &(metamodelica::arrayGet(allSCVarsMapping.clone(), varIdx.clone())?) {
            Some(__pa15 @ SimCodeVar::SimVar { name: __pa14, .. }) => (__pa15.clone(), __pa14.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cacheVarName = __pa14.clone();
        cacheVariable = __pa15.clone();
        cacheVariables = metamodelica::cons(cacheVariable.clone(), cacheVariables.clone());
        scVarCLMapping = metamodelica::arrayUpdate(scVarCLMapping.clone(), varIdx.clone(), (lastCLidx.clone(), varDataType.clone()))?;
        varEntry = CacheLineEntry { start: cacheLineSize.clone() - lastCLnumBytesFree.clone(), dataType: varDataType.clone(), size: varNumBytesRequired.clone(), scVarIdx: (cacheVariables.clone().len() as i32), threadOwner: iThreadIdx.clone() };
        lastCL = CacheLineMap { idx: lastCLidx.clone(), numBytesFree: lastCLnumBytesFree.clone() - varNumBytesRequired.clone(), entries: metamodelica::cons(varEntry.clone(), lastCLentries.clone()) };
        metamodelica::arrayUpdate(iThreadCacheLines.clone(), iThreadIdx.clone(), contractCacheLineForVarType(varDataType.clone(), threadCacheLinesFloat.clone(), threadCacheLinesInt.clone(), threadCacheLinesBool.clone(), metamodelica::cons(lastCL.clone(), fullCLs.clone())))?;
    }
    oInfo = (CacheMap::CACHEMAP { cacheLineSize: cacheLineSize.clone(), cacheVariables: cacheVariables.clone(), cacheLinesFloat: cacheLinesFloat.clone(), cacheLinesInt: cacheLinesInt.clone(), cacheLinesBool: cacheLinesBool.clone() }, CacheMapMeta { allSCVarsMapping: allSCVarsMapping.clone(), simCodeVarTypes: simCodeVarTypes.clone(), scVarCLMapping: scVarCLMapping.clone() }, numCLs.clone());
    Ok(oInfo)
}

fn getCacheLineForVarType(mut iVarDataType: i32, mut iCacheLinesForTypes: CacheLines) -> (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>) {
    let mut oCacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut oCacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut oCacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut oVarCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    (oCacheLinesFloat, oCacheLinesInt, oCacheLinesBool) = iCacheLinesForTypes.clone();
    if intEq(iVarDataType.clone(), VARDATATYPE_FLOAT.clone()) {
        (oVarCacheLines, _, _) = iCacheLinesForTypes.clone();
    } else {
        if intEq(iVarDataType.clone(), VARDATATYPE_INTEGER.clone()) {
            (_, oVarCacheLines, _) = iCacheLinesForTypes.clone();
        } else {
            if intEq(iVarDataType.clone(), VARDATATYPE_BOOLEAN.clone()) {
                (_, _, oVarCacheLines) = iCacheLinesForTypes.clone();
            } else {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getCacheLineForVarType: Found Variable with unknown type ( ")); __mm_s.push_str(&*intString(iVarDataType.clone())); __mm_s.push_str(&*literal!(")!\n")); ArcStr::from(__mm_s) }).clone());
            }
        }
    }
    (oCacheLinesFloat, oCacheLinesInt, oCacheLinesBool, oVarCacheLines)
}

fn contractCacheLineForVarType(mut iVarDataType: i32, mut iCacheLinesFloat: Arc<metamodelica::List<CacheLineMap>>, mut iCacheLinesInt: Arc<metamodelica::List<CacheLineMap>>, mut iCacheLinesBool: Arc<metamodelica::List<CacheLineMap>>, mut iVarCacheLines: Arc<metamodelica::List<CacheLineMap>>) -> CacheLines {
    let mut oContractedCacheLines: CacheLines = (metamodelica::nil(), metamodelica::nil(), metamodelica::nil());
    if intEq(iVarDataType.clone(), VARDATATYPE_FLOAT.clone()) {
        oContractedCacheLines = (iVarCacheLines.clone(), iCacheLinesInt.clone(), iCacheLinesBool.clone());
    } else {
        if intEq(iVarDataType.clone(), VARDATATYPE_INTEGER.clone()) {
            oContractedCacheLines = (iCacheLinesFloat.clone(), iVarCacheLines.clone(), iCacheLinesBool.clone());
        } else {
            if intEq(iVarDataType.clone(), VARDATATYPE_BOOLEAN.clone()) {
                oContractedCacheLines = (iCacheLinesFloat.clone(), iCacheLinesInt.clone(), iVarCacheLines.clone());
            }
        }
    }
    oContractedCacheLines
}

fn addVarsToSharedCL<T: Clone + 'static>(mut iNodeVars: Arc<metamodelica::List<i32>>, mut iSharedClSelectFunction: Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, T, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>, mut iFactoryMethod: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>, mut iThreadIdx: i32, mut iCompareFuncArgument: T, mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>, mut iInfo: (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> {
    pub type HeuristicFunction<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(i32, i32, i32, i32, T, metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> + 'static>;

    pub type FactoryMethod<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>;

    let mut oInfo: (CacheMap, CacheMapMeta, i32) = (<CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0);
    let mut allSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>> = Default::default();
    let mut varIdx: i32 = 0;
    let mut varDataType: i32 = 0;
    let mut numOfCLs: i32 = 0;
    let mut cacheLineSize: i32 = 0;
    let mut varSize: i32 = 0;
    let mut simCodeVarTypes: metamodelica::Array<(i32, i32, i32)> = Default::default();
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut cacheMapMeta: CacheMapMeta = <CacheMapMeta as ::std::default::Default>::default();
    let mut matchedCacheLine: Option<(PartlyFilledCacheLine, i32)> = None;
    let (ref __pa3 @ CacheMap::CACHEMAP { cacheLineSize: ref __pa0, cacheVariables: ref __pa1, cacheLinesFloat: ref __pa2, .. }, ref __pa7 @ CacheMapMeta { allSCVarsMapping: ref __pa4, simCodeVarTypes: ref __pa5, scVarCLMapping: ref __pa6 }, __pa8) = (iInfo.clone()) else { bail!("pattern mismatch") };
    cacheLineSize = __pa0.clone();
    cacheVariables = __pa1.clone();
    cacheLinesFloat = __pa2.clone();
    cacheMap = __pa3.clone();
    allSCVarsMapping = __pa4.clone();
    simCodeVarTypes = __pa5.clone();
    scVarCLMapping = __pa6.clone();
    cacheMapMeta = __pa7.clone();
    numOfCLs = __pa8.clone();
    for mut varIdx in &*iNodeVars.clone() {
        let mut varIdx = varIdx.clone();
        (varDataType, varSize, _) = metamodelica::arrayGet(simCodeVarTypes.clone(), varIdx.clone())?;
        matchedCacheLine = iSharedClSelectFunction(varIdx.clone(), varSize.clone(), varDataType.clone(), iThreadIdx.clone(), iCompareFuncArgument.clone(), iSharedCacheLines.clone())?;
        (cacheMap, cacheMapMeta, numOfCLs) = addVarsToSharedCL0(matchedCacheLine.clone(), varIdx.clone(), iFactoryMethod.clone(), iCompareFuncArgument.clone(), iThreadIdx.clone(), iSharedCacheLines.clone(), (cacheMap.clone(), cacheMapMeta.clone(), numOfCLs.clone()))?;
    }
    oInfo = (cacheMap.clone(), cacheMapMeta.clone(), numOfCLs.clone());
    Ok(oInfo)
}

fn addVarsToSharedCL0<T: Clone + 'static>(mut iMatchedCacheLine: Option<(PartlyFilledCacheLine, i32)>, mut iVarIdx: i32, mut iFactoryMethod: Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>, mut iAdditionalArgument: T, mut iThreadIdx: i32, mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>, mut iInfo: (CacheMap, CacheMapMeta, i32)) -> Result<(CacheMap, CacheMapMeta, i32)> {
    pub type FactoryMethod<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Option<PartlyFilledCacheLine>, CacheLineMap, T) -> Result<PartlyFilledCacheLine> + 'static>;

    let mut oInfo: (CacheMap, CacheMapMeta, i32) = (<CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0);
    let mut threadPartlyFilledCacheLines: PartlyFilledCacheLines = (metamodelica::nil(), metamodelica::nil(), metamodelica::nil());
    let mut partlyFilledClFloat: Arc<metamodelica::List<PartlyFilledCacheLine>> = metamodelica::nil();
    let mut partlyFilledClInt: Arc<metamodelica::List<PartlyFilledCacheLine>> = metamodelica::nil();
    let mut partlyFilledClBool: Arc<metamodelica::List<PartlyFilledCacheLine>> = metamodelica::nil();
    let mut threadFullyFilledCacheLines: CacheLines = (metamodelica::nil(), metamodelica::nil(), metamodelica::nil());
    let mut fullyFilledClFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut fullyFilledClInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut fullyFilledClBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut allSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>> = Default::default();
    let mut simCodeVarTypes: metamodelica::Array<(i32, i32, i32)> = Default::default();
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut partlyFilledCacheLine: PartlyFilledCacheLine = <PartlyFilledCacheLine as ::std::default::Default>::default();
    let mut partlyFilledCacheLineOption: Option<PartlyFilledCacheLine> = None;
    let mut matchedClIndex: i32 = 0;
    let mut numOfCLs: i32 = 0;
    let mut clMapIdx: i32 = 0;
    let mut clMapNumBytesFree: i32 = 0;
    let mut varDataType: i32 = 0;
    let mut varSize: i32 = 0;
    let mut cacheLineSize: i32 = 0;
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut clMapEntries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut entry: CacheLineEntry = <CacheLineEntry as ::std::default::Default>::default();
    let mut cacheLineMap: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    let mut cacheVariable: SimCodeVar::SimVar = <SimCodeVar::SimVar as ::std::default::Default>::default();
    let (CacheMap::CACHEMAP { cacheLineSize: __pa0, cacheVariables: __pa1, cacheLinesFloat: __pa2, cacheLinesInt: __pa3, cacheLinesBool: __pa4 }, CacheMapMeta { allSCVarsMapping: __pa5, simCodeVarTypes: __pa6, scVarCLMapping: __pa7 }, __pa8) = (iInfo.clone()) else { bail!("pattern mismatch") };
    cacheLineSize = __pa0.clone();
    cacheVariables = __pa1.clone();
    cacheLinesFloat = __pa2.clone();
    cacheLinesInt = __pa3.clone();
    cacheLinesBool = __pa4.clone();
    allSCVarsMapping = __pa5.clone();
    simCodeVarTypes = __pa6.clone();
    scVarCLMapping = __pa7.clone();
    numOfCLs = __pa8.clone();
    (varDataType, varSize, _) = metamodelica::arrayGet(simCodeVarTypes.clone(), iVarIdx.clone())?;
    (threadPartlyFilledCacheLines, threadFullyFilledCacheLines) = metamodelica::arrayGet(iSharedCacheLines.clone(), iThreadIdx.clone())?;
    (partlyFilledClFloat, partlyFilledClInt, partlyFilledClBool) = threadPartlyFilledCacheLines.clone();
    (fullyFilledClFloat, fullyFilledClInt, fullyFilledClBool) = threadFullyFilledCacheLines.clone();
    if isSome(iMatchedCacheLine.clone()) {
        clMapIdx = numOfCLs.clone();
        let (__pa9, __pa10) = ::match_deref::match_deref! { match &(iMatchedCacheLine.clone()) {
            Some((__pa9, __pa10)) => (__pa9.clone(), __pa10.clone()),
            _ => bail!("pattern mismatch"),
        } };
        partlyFilledCacheLine = __pa9.clone();
        matchedClIndex = __pa10.clone();
        partlyFilledCacheLineOption = Some(partlyFilledCacheLine.clone());
        let CacheLineMap { idx: __pa11, numBytesFree: __pa12, entries: __pa13 } = (getCacheLineMapOfPartlyFilledCacheLine(partlyFilledCacheLine.clone())?) else { bail!("pattern mismatch") };
        clMapIdx = __pa11.clone();
        clMapNumBytesFree = __pa12.clone();
        clMapEntries = __pa13.clone();
    } else {
        numOfCLs = numOfCLs.clone() + 1;
        partlyFilledCacheLineOption = None;
        clMapIdx = numOfCLs.clone();
        clMapNumBytesFree = cacheLineSize.clone();
        clMapEntries = metamodelica::nil();
        matchedClIndex = -1;
    }
    clMapNumBytesFree = clMapNumBytesFree.clone() - varSize.clone();
    let __pa14 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(allSCVarsMapping.clone(), iVarIdx.clone())?) {
        Some(__pa14) => __pa14.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cacheVariable = __pa14.clone();
    cacheVariables = metamodelica::cons(cacheVariable.clone(), cacheVariables.clone());
    entry = CacheLineEntry { start: cacheLineSize.clone() - clMapNumBytesFree.clone() - varSize.clone(), dataType: varDataType.clone(), size: varSize.clone(), scVarIdx: (cacheVariables.clone().len() as i32), threadOwner: iThreadIdx.clone() };
    cacheLineMap = CacheLineMap { idx: clMapIdx.clone(), numBytesFree: clMapNumBytesFree.clone(), entries: metamodelica::cons(entry.clone(), clMapEntries.clone()) };
    partlyFilledCacheLine = iFactoryMethod(partlyFilledCacheLineOption.clone(), cacheLineMap.clone(), iAdditionalArgument.clone())?;
    scVarCLMapping = metamodelica::arrayUpdate(scVarCLMapping.clone(), iVarIdx.clone(), (clMapIdx.clone(), varDataType.clone()))?;
    if intEq(clMapNumBytesFree.clone(), 0) {
        if intEq(varDataType.clone(), VARDATATYPE_FLOAT.clone()) {
            partlyFilledClFloat = listDelete(partlyFilledClFloat.clone(), matchedClIndex.clone())?;
            fullyFilledClFloat = metamodelica::cons(cacheLineMap.clone(), fullyFilledClFloat.clone());
        } else {
            if intEq(varDataType.clone(), VARDATATYPE_INTEGER.clone()) {
                partlyFilledClInt = listDelete(partlyFilledClInt.clone(), matchedClIndex.clone())?;
                fullyFilledClInt = metamodelica::cons(cacheLineMap.clone(), fullyFilledClInt.clone());
            } else {
                partlyFilledClBool = listDelete(partlyFilledClBool.clone(), matchedClIndex.clone())?;
                fullyFilledClBool = metamodelica::cons(cacheLineMap.clone(), fullyFilledClBool.clone());
            }
        }
    } else {
        if intNe(matchedClIndex.clone(), -1) {
            if intEq(varDataType.clone(), VARDATATYPE_FLOAT.clone()) {
                partlyFilledClFloat = List::set(partlyFilledClFloat.clone(), matchedClIndex.clone(), partlyFilledCacheLine.clone())?;
            } else {
                if intEq(varDataType.clone(), VARDATATYPE_INTEGER.clone()) {
                    partlyFilledClInt = List::set(partlyFilledClInt.clone(), matchedClIndex.clone(), partlyFilledCacheLine.clone())?;
                } else {
                    partlyFilledClBool = List::set(partlyFilledClBool.clone(), matchedClIndex.clone(), partlyFilledCacheLine.clone())?;
                }
            }
        } else {
            if intEq(varDataType.clone(), VARDATATYPE_FLOAT.clone()) {
                partlyFilledClFloat = metamodelica::cons(partlyFilledCacheLine.clone(), partlyFilledClFloat.clone());
            } else {
                if intEq(varDataType.clone(), VARDATATYPE_INTEGER.clone()) {
                    partlyFilledClInt = metamodelica::cons(partlyFilledCacheLine.clone(), partlyFilledClInt.clone());
                } else {
                    partlyFilledClBool = metamodelica::cons(partlyFilledCacheLine.clone(), partlyFilledClBool.clone());
                }
            }
        }
    }
    metamodelica::arrayUpdate(iSharedCacheLines.clone(), iThreadIdx.clone(), ((partlyFilledClFloat.clone(), partlyFilledClInt.clone(), partlyFilledClBool.clone()), (fullyFilledClFloat.clone(), fullyFilledClInt.clone(), fullyFilledClBool.clone())))?;
    oInfo = (CacheMap::CACHEMAP { cacheLineSize: cacheLineSize.clone(), cacheVariables: cacheVariables.clone(), cacheLinesFloat: cacheLinesFloat.clone(), cacheLinesInt: cacheLinesInt.clone(), cacheLinesBool: cacheLinesBool.clone() }, CacheMapMeta { allSCVarsMapping: allSCVarsMapping.clone(), simCodeVarTypes: simCodeVarTypes.clone(), scVarCLMapping: scVarCLMapping.clone() }, numOfCLs.clone());
    Ok(oInfo)
}

fn getPartlyFilledCLByVarType(mut iVarType: i32, mut iSharedCacheLines: PartlyFilledCacheLines) -> Arc<metamodelica::List<PartlyFilledCacheLine>> {
    let mut oSharedCacheLinesForType: Arc<metamodelica::List<PartlyFilledCacheLine>> = metamodelica::nil();
    if intEq(iVarType.clone(), VARDATATYPE_FLOAT.clone()) {
        oSharedCacheLinesForType = Util::tuple31(iSharedCacheLines.clone());
    } else {
        if intEq(iVarType.clone(), VARDATATYPE_INTEGER.clone()) {
            oSharedCacheLinesForType = Util::tuple32(iSharedCacheLines.clone());
        } else {
            oSharedCacheLinesForType = Util::tuple33(iSharedCacheLines.clone());
        }
    }
    oSharedCacheLinesForType
}

fn findMatchingSharedCLLevelfix(mut iNodeVar: i32, mut iVarSize: i32, mut iVarType: i32, mut iThreadIdx: i32, mut iLevelThreadIdx: (i32, i32), mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> {
    let mut oMatchedCacheLine: Option<(PartlyFilledCacheLine, i32)> = None;
    let mut partlyFilledCacheLines: Arc<metamodelica::List<PartlyFilledCacheLine>> = metamodelica::nil();
    let mut sharedCacheLines: PartlyFilledCacheLines = (metamodelica::nil(), metamodelica::nil(), metamodelica::nil());
    let mut levelIdx: i32 = 0;
    (levelIdx, _) = iLevelThreadIdx.clone();
    sharedCacheLines = Util::tuple21(metamodelica::arrayGet(iSharedCacheLines.clone(), iThreadIdx.clone())?);
    oMatchedCacheLine = None;
    partlyFilledCacheLines = getPartlyFilledCLByVarType(iVarType.clone(), sharedCacheLines.clone());
    oMatchedCacheLine = findMatchingSharedCLLevelfix0(iNodeVar.clone(), iVarSize.clone(), levelIdx.clone(), iThreadIdx.clone(), 1, partlyFilledCacheLines.clone())?;
    Ok(oMatchedCacheLine)
}

fn findMatchingSharedCLLevelfix0(mut iNodeVar: i32, mut iVarSize: i32, mut iLevelIdx: i32, mut iThreadIdx: i32, mut iCurrentListIdx: i32, mut iSharedCacheLines: Arc<metamodelica::List<PartlyFilledCacheLine>>) -> Result<Option<(PartlyFilledCacheLine, i32)>> {
    let mut oMatchedCacheLine: Option<(PartlyFilledCacheLine, i32)> = None;
    let mut head: PartlyFilledCacheLine = <PartlyFilledCacheLine as ::std::default::Default>::default();
    let mut rest: Arc<metamodelica::List<PartlyFilledCacheLine>> = metamodelica::nil();
    let mut tmpMatchedCacheLine: Option<(PartlyFilledCacheLine, i32)> = None;
    let mut cacheLineMap: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    let mut numBytesFree: i32 = 0;
    let mut prefetchLevel: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut writeLevel: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    oMatchedCacheLine = (::match_deref::match_deref! { match &(iSharedCacheLines.clone()) {
        Deref @ metamodelica::List::Cons { head: __esc_head @ PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_LEVEL { cacheLineMap: __esc_cacheLineMap @ CacheLineMap { numBytesFree: __esc_numBytesFree, .. }, prefetchLevel: __esc_prefetchLevel, writeLevel: __esc_writeLevel }, tail: __esc_rest } => {
            head = (*__esc_head).clone();
            cacheLineMap = (*__esc_cacheLineMap).clone();
            numBytesFree = (*__esc_numBytesFree).clone();
            prefetchLevel = (*__esc_prefetchLevel).clone();
            writeLevel = (*__esc_writeLevel).clone();
            rest = (*__esc_rest).clone();
            if boolOr(intLt(numBytesFree.clone(), iVarSize.clone()), List::exist1(prefetchLevel.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), iLevelIdx.clone())?) {
                tmpMatchedCacheLine = findMatchingSharedCLLevelfix0(iNodeVar.clone(), iVarSize.clone(), iLevelIdx.clone(), iThreadIdx.clone(), iCurrentListIdx.clone() + 1, rest.clone())?;
            } else {
                if List::any(writeLevel.clone(), (std::sync::Arc::new({ let __pe_b1 = iLevelIdx.clone(); let __pe_b2 = iThreadIdx.clone(); move |__pe_a0| Ok(isCLWrittenByOtherThread(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<bool> + 'static>))? {
                    tmpMatchedCacheLine = findMatchingSharedCLLevelfix0(iNodeVar.clone(), iVarSize.clone(), iLevelIdx.clone(), iThreadIdx.clone(), iCurrentListIdx.clone() + 1, rest.clone())?;
                } else {
                    if List::any(writeLevel.clone(), (std::sync::Arc::new({ let __pe_b1 = iLevelIdx.clone() - 1; let __pe_b2 = iThreadIdx.clone(); move |__pe_a0| Ok(isCLWrittenByOtherThread(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32)) -> Result<bool> + 'static>))? {
                        tmpMatchedCacheLine = findMatchingSharedCLLevelfix0(iNodeVar.clone(), iVarSize.clone(), iLevelIdx.clone(), iThreadIdx.clone(), iCurrentListIdx.clone() + 1, rest.clone())?;
                    } else {
                        tmpMatchedCacheLine = Some((head.clone(), iCurrentListIdx.clone()));
                    }
                }
            }
            tmpMatchedCacheLine.clone()
        },
        Deref @ metamodelica::List::Nil => None,
        _ => {
            metamodelica::print((literal!("findMatchingSharedCLLevelfix0: Unknown partly filled cache line type given.\n")).clone());
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oMatchedCacheLine)
}

fn findMatchingSharedCLThread(mut iNodeVar: i32, mut iVarSize: i32, mut iVarType: i32, mut iThreadIdx: i32, mut iAdditionalArgument: i32, mut iSharedCacheLines: metamodelica::Array<((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>))>) -> Result<Option<(PartlyFilledCacheLine, i32)>> {
    let mut oMatchedCacheLine: Option<(PartlyFilledCacheLine, i32)> = None;
    let mut partlyFilledCacheLines: Arc<metamodelica::List<PartlyFilledCacheLine>> = metamodelica::nil();
    let mut partlyFilledCL: PartlyFilledCacheLine = <PartlyFilledCacheLine as ::std::default::Default>::default();
    let mut numBytesFree: i32 = 0;
    let mut listIdx: i32 = 0;
    oMatchedCacheLine = None;
    partlyFilledCacheLines = getPartlyFilledCLByVarType(iVarType.clone(), Util::tuple21(metamodelica::arrayGet(iSharedCacheLines.clone(), iThreadIdx.clone())?));
    listIdx = 1;
    for mut partlyFilledCL in &*partlyFilledCacheLines.clone() {
        let mut partlyFilledCL = partlyFilledCL.clone();
        let CacheLineMap { numBytesFree: __pa0, .. } = (getCacheLineMapOfPartlyFilledCacheLine(partlyFilledCL.clone())?) else { bail!("pattern mismatch") };
        numBytesFree = __pa0.clone();
        if intGe(numBytesFree.clone(), iVarSize.clone()) {
            oMatchedCacheLine = Some((partlyFilledCL.clone(), listIdx.clone()));
            break;
        }
        listIdx = listIdx.clone() + 1;
    }
    Ok(oMatchedCacheLine)
}

fn createSharedClThread(mut iOldPartlyFilledCacheLine: Option<PartlyFilledCacheLine>, mut iCacheLineMap: CacheLineMap, mut iAdditionalArgument: i32) -> PartlyFilledCacheLine {
    let mut oCreatedCacheLine: PartlyFilledCacheLine = <PartlyFilledCacheLine as ::std::default::Default>::default();
    oCreatedCacheLine = PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_THREAD { cacheLineMap: iCacheLineMap.clone() };
    oCreatedCacheLine
}

fn createSharedClLevelFix(mut iOldPartlyFilledCacheLine: Option<PartlyFilledCacheLine>, mut iCacheLineMap: CacheLineMap, mut iLevelThreadIdx: (i32, i32)) -> Result<PartlyFilledCacheLine> {
    let mut oCreatedCacheLine: PartlyFilledCacheLine = <PartlyFilledCacheLine as ::std::default::Default>::default();
    let mut prefetchLevel: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut writeLevel: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut levelIdx: i32 = 0;
    let mut threadIdx: i32 = 0;
    (levelIdx, threadIdx) = iLevelThreadIdx.clone();
    if isSome(iOldPartlyFilledCacheLine.clone()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iOldPartlyFilledCacheLine.clone()) {
            Some(PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_LEVEL { prefetchLevel: __pa0, writeLevel: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        prefetchLevel = __pa0.clone();
        writeLevel = __pa1.clone();
    } else {
        prefetchLevel = metamodelica::nil();
        writeLevel = metamodelica::nil();
    }
    if intGt(levelIdx.clone() - 1, 0) {
        prefetchLevel = metamodelica::cons(levelIdx.clone() - 1, prefetchLevel.clone());
    }
    writeLevel = metamodelica::cons((levelIdx.clone(), threadIdx.clone()), writeLevel.clone());
    oCreatedCacheLine = PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_LEVEL { cacheLineMap: iCacheLineMap.clone(), prefetchLevel: prefetchLevel.clone(), writeLevel: writeLevel.clone() };
    Ok(oCreatedCacheLine)
}

fn isCLWrittenByOtherThread(mut iLevelInfo: (i32, i32), mut iLevelIdx: i32, mut iThreadIdx: i32) -> bool {
    let mut oWrittenByOtherThread: bool = false;
    let mut levelIdx: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut ret: bool = false;
    (levelIdx, threadIdx) = iLevelInfo.clone();
    ret = boolAnd(intEq(levelIdx.clone(), iLevelIdx.clone()), intNe(threadIdx.clone(), iThreadIdx.clone()));
    oWrittenByOtherThread = ret.clone();
    oWrittenByOtherThread
}

fn createCacheMapFromThreadAndSharedCLs(mut iThreadCacheLines: CacheLines, mut iSharedCacheLines: ((Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>, Arc<metamodelica::List<PartlyFilledCacheLine>>), (Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>, Arc<metamodelica::List<CacheLineMap>>)), mut iCacheMap: CacheMap) -> Result<CacheMap> {
    let mut oCacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut cacheLineSize: i32 = 0;
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut fullyFilledSharedCacheLines: CacheLines = (metamodelica::nil(), metamodelica::nil(), metamodelica::nil());
    let mut partlyFilledCacheLines: PartlyFilledCacheLines = (metamodelica::nil(), metamodelica::nil(), metamodelica::nil());
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let CacheMap::CACHEMAP { cacheLineSize: __pa0, cacheVariables: __pa1, cacheLinesFloat: __pa2, cacheLinesInt: __pa3, cacheLinesBool: __pa4 } = (iCacheMap.clone()) else { bail!("pattern mismatch") };
    cacheLineSize = __pa0.clone();
    cacheVariables = __pa1.clone();
    cacheLinesFloat = __pa2.clone();
    cacheLinesInt = __pa3.clone();
    cacheLinesBool = __pa4.clone();
    (partlyFilledCacheLines, fullyFilledSharedCacheLines) = iSharedCacheLines.clone();
    cacheLinesFloat = listAppend(cacheLinesFloat.clone(), listAppend(Util::tuple31(iThreadCacheLines.clone()), Util::tuple31(fullyFilledSharedCacheLines.clone())));
    cacheLinesInt = listAppend(cacheLinesInt.clone(), listAppend(Util::tuple32(iThreadCacheLines.clone()), Util::tuple32(fullyFilledSharedCacheLines.clone())));
    cacheLinesBool = listAppend(cacheLinesBool.clone(), listAppend(Util::tuple33(iThreadCacheLines.clone()), Util::tuple33(fullyFilledSharedCacheLines.clone())));
    cacheLinesFloat = listAppend(cacheLinesFloat.clone(), List::map(Util::tuple31(partlyFilledCacheLines.clone()), (std::sync::Arc::new(getCacheLineMapOfPartlyFilledCacheLine) as std::sync::Arc<dyn ::std::ops::Fn(PartlyFilledCacheLine) -> Result<CacheLineMap> + 'static>))?);
    cacheLinesInt = listAppend(cacheLinesInt.clone(), List::map(Util::tuple32(partlyFilledCacheLines.clone()), (std::sync::Arc::new(getCacheLineMapOfPartlyFilledCacheLine) as std::sync::Arc<dyn ::std::ops::Fn(PartlyFilledCacheLine) -> Result<CacheLineMap> + 'static>))?);
    cacheLinesBool = listAppend(cacheLinesBool.clone(), List::map(Util::tuple33(partlyFilledCacheLines.clone()), (std::sync::Arc::new(getCacheLineMapOfPartlyFilledCacheLine) as std::sync::Arc<dyn ::std::ops::Fn(PartlyFilledCacheLine) -> Result<CacheLineMap> + 'static>))?);
    oCacheMap = CacheMap::CACHEMAP { cacheLineSize: cacheLineSize.clone(), cacheVariables: cacheVariables.clone(), cacheLinesFloat: cacheLinesFloat.clone(), cacheLinesInt: cacheLinesInt.clone(), cacheLinesBool: cacheLinesBool.clone() };
    Ok(oCacheMap)
}

fn createCacheMapDefault(mut iAllSCVars: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iCacheLineSize: i32, mut iSimCodeVars: SimCodeVar::SimVars, mut iScVarTaskMapping: metamodelica::Array<i32>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut oNumCL: i32 = 0;
    if stringEqual((Config::simCodeTarget()?).clone(), (literal!("Cpp")).clone()) {
        (oCacheMap, oScVarCLMapping, oNumCL) = createCacheMapDefaultCppRuntime(iAllSCVars.clone(), iCacheLineSize.clone(), iSimCodeVars.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), iSimCodeVarTypes.clone())?;
    } else {
        oCacheMap = CacheMap::UNIFORM_CACHEMAP { cacheLineSize: iCacheLineSize.clone(), cacheVariables: metamodelica::nil(), cacheLines: metamodelica::nil() };
        oNumCL = 0;
        oScVarCLMapping = arrayCreate(0, (-1, -1));
    }
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapDefaultCppRuntime(mut iAllSCVars: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iCacheLineSize: i32, mut iSimCodeVars: SimCodeVar::SimVars, mut iScVarTaskMapping: metamodelica::Array<i32>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>) -> Result<(CacheMap, metamodelica::Array<(i32, i32)>, i32)> {
    let mut oCacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut oScVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut oNumCL: i32 = 0;
    let mut stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut derivativeVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut algVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut discreteAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut paramVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut aliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut intAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut intParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut intAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut boolAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut inputVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut outputVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut lastCacheLine: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    let mut scVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut currentScVarIdx: i32 = 0;
    let mut paramVarsStart: i32 = 0;
    let mut aliasVarsStart: i32 = 0;
    let mut stateDerVarsStart: i32 = 0;
    let mut algVarsStart: i32 = 0;
    let mut discreteAlgVarsStart: i32 = 0;
    let mut intAlgVarsStart: i32 = 0;
    let mut intParamVarsStart: i32 = 0;
    let mut filledCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut allVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    (oCacheMap, oScVarCLMapping, oNumCL) = (match iSimCodeVars.clone() {
        SimCodeVar::SimVars { stateVars: mut __esc_stateVars, derivativeVars: mut __esc_derivativeVars, algVars: mut __esc_algVars, discreteAlgVars: mut __esc_discreteAlgVars, paramVars: mut __esc_paramVars, aliasVars: mut __esc_aliasVars, intAlgVars: mut __esc_intAlgVars, intParamVars: mut __esc_intParamVars, intAliasVars: mut __esc_intAliasVars, boolAlgVars: mut __esc_boolAlgVars, boolParamVars: mut __esc_boolParamVars, boolAliasVars: mut __esc_boolAliasVars, inputVars: mut __esc_inputVars, outputVars: mut __esc_outputVars, .. } => {
            stateVars = __esc_stateVars.clone();
            derivativeVars = __esc_derivativeVars.clone();
            algVars = __esc_algVars.clone();
            discreteAlgVars = __esc_discreteAlgVars.clone();
            paramVars = __esc_paramVars.clone();
            aliasVars = __esc_aliasVars.clone();
            intAlgVars = __esc_intAlgVars.clone();
            intParamVars = __esc_intParamVars.clone();
            intAliasVars = __esc_intAliasVars.clone();
            boolAlgVars = __esc_boolAlgVars.clone();
            boolParamVars = __esc_boolParamVars.clone();
            boolAliasVars = __esc_boolAliasVars.clone();
            inputVars = __esc_inputVars.clone();
            outputVars = __esc_outputVars.clone();
            currentScVarIdx = 1;
            stateDerVarsStart = (stateVars.clone().len() as i32) + 1;
            scVarCLMapping = arrayCreate(metamodelica::arrayLength(iAllSCVars.clone()), (-1, -1));
            filledCacheLines = metamodelica::nil();
            lastCacheLine = CacheLineMap { idx: 1, numBytesFree: iCacheLineSize.clone(), entries: metamodelica::nil() };
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(derivativeVars.clone(), currentScVarIdx.clone(), stateDerVarsStart.clone(), scVarCLMapping.clone(), filledCacheLines.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), lastCacheLine.clone(), iCacheLineSize.clone(), iSimCodeVarTypes.clone())?;
            filledCacheLines = metamodelica::cons(lastCacheLine.clone(), filledCacheLines.clone());
            lastCacheLine = CacheLineMap { idx: (filledCacheLines.clone().len() as i32) + 1, numBytesFree: iCacheLineSize.clone(), entries: metamodelica::nil() };
            allVars = derivativeVars.clone().reverse();
            algVarsStart = stateDerVarsStart.clone() + (derivativeVars.clone().len() as i32);
            discreteAlgVarsStart = algVarsStart.clone() + (algVars.clone().len() as i32);
            intAlgVarsStart = discreteAlgVarsStart.clone() + (discreteAlgVars.clone().len() as i32);
            aliasVarsStart = intAlgVarsStart.clone() + (boolAlgVars.clone().len() as i32) + (inputVars.clone().len() as i32) + (outputVars.clone().len() as i32);
            paramVarsStart = aliasVarsStart.clone() + (aliasVars.clone().len() as i32) + (intAliasVars.clone().len() as i32) + (boolAliasVars.clone().len() as i32);
            intParamVarsStart = paramVarsStart.clone() + (paramVars.clone().len() as i32);
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(algVars.clone(), currentScVarIdx.clone(), algVarsStart.clone(), scVarCLMapping.clone(), filledCacheLines.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), lastCacheLine.clone(), iCacheLineSize.clone(), iSimCodeVarTypes.clone())?;
            allVars = List::append_reverse(algVars.clone(), allVars.clone());
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(discreteAlgVars.clone(), currentScVarIdx.clone(), discreteAlgVarsStart.clone(), scVarCLMapping.clone(), filledCacheLines.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), lastCacheLine.clone(), iCacheLineSize.clone(), iSimCodeVarTypes.clone())?;
            allVars = List::append_reverse(discreteAlgVars.clone(), allVars.clone());
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(paramVars.clone(), currentScVarIdx.clone(), paramVarsStart.clone(), scVarCLMapping.clone(), filledCacheLines.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), lastCacheLine.clone(), iCacheLineSize.clone(), iSimCodeVarTypes.clone())?;
            allVars = List::append_reverse(paramVars.clone(), allVars.clone());
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(aliasVars.clone(), currentScVarIdx.clone(), aliasVarsStart.clone(), scVarCLMapping.clone(), filledCacheLines.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), lastCacheLine.clone(), iCacheLineSize.clone(), iSimCodeVarTypes.clone())?;
            allVars = List::append_reverse(aliasVars.clone(), allVars.clone());
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(intAlgVars.clone(), currentScVarIdx.clone(), intAlgVarsStart.clone(), scVarCLMapping.clone(), filledCacheLines.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), lastCacheLine.clone(), iCacheLineSize.clone(), iSimCodeVarTypes.clone())?;
            allVars = List::append_reverse(intAlgVars.clone(), allVars.clone());
            (filledCacheLines, lastCacheLine, currentScVarIdx) = createCacheMapDefaultCppRuntime0(intParamVars.clone(), currentScVarIdx.clone(), intAlgVarsStart.clone(), scVarCLMapping.clone(), filledCacheLines.clone(), iScVarTaskMapping.clone(), iSchedulerInfo.clone(), lastCacheLine.clone(), iCacheLineSize.clone(), iSimCodeVarTypes.clone())?;
            allVars = List::append_reverse(intParamVars.clone(), allVars.clone());
            cacheMap = CacheMap::UNIFORM_CACHEMAP { cacheLineSize: iCacheLineSize.clone(), cacheVariables: allVars.clone(), cacheLines: metamodelica::cons(lastCacheLine.clone(), filledCacheLines.clone()) };
            (cacheMap.clone(), scVarCLMapping.clone(), (filledCacheLines.clone().len() as i32) + 1)
        },
    });
    Ok((oCacheMap, oScVarCLMapping, oNumCL))
}

fn createCacheMapDefaultCppRuntime0(mut iVariables: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iScVarIdxStart: i32, mut iRealScVarIdxStart: i32, mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut iFilledCacheLines: Arc<metamodelica::List<CacheLineMap>>, mut iScVarTaskMapping: metamodelica::Array<i32>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iLastCacheLine: CacheLineMap, mut iCacheLineSize: i32, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>) -> Result<(Arc<metamodelica::List<CacheLineMap>>, CacheLineMap, i32)> {
    let mut oFilledCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut oLastCacheLine: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    let mut oScVarIdx: i32 = 0;
    let mut currentScVarIdx: i32 = 0;
    let mut varSize: i32 = 0;
    let mut varDataType: i32 = 0;
    let mut varTask: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut varCLIdx: i32 = 0;
    let mut var: SimCodeVar::SimVar = <SimCodeVar::SimVar as ::std::default::Default>::default();
    let mut entry: CacheLineEntry = <CacheLineEntry as ::std::default::Default>::default();
    let mut newCacheLineCreated: bool = false;
    let mut lastCacheLine: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    let mut lastCacheLineNew: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    let mut filledCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cachelineEntries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut nameStr: ArcStr = arcstr::literal!("");
    currentScVarIdx = 0;
    lastCacheLine = iLastCacheLine.clone();
    filledCacheLines = iFilledCacheLines.clone();
    for mut var in &*iVariables.clone() {
        let mut var = var.clone();
        let SimCodeVar::SIMVAR { name: __pa0, .. } = (var.clone()) else { bail!("pattern mismatch") };
        name = __pa0.clone();
        nameStr = (ComponentReferenceBasics::printComponentRefStr(name.clone())?).clone();
        if boolAnd(intLt(currentScVarIdx.clone(), metamodelica::arrayLength(iSimCodeVarTypes.clone())), intLt(currentScVarIdx.clone(), metamodelica::arrayLength(iScVarCLMapping.clone()))) {
            (varDataType, varSize, _) = metamodelica::arrayGet(iSimCodeVarTypes.clone(), currentScVarIdx.clone() + iRealScVarIdxStart.clone())?;
            if intLe(currentScVarIdx.clone() + iRealScVarIdxStart.clone(), metamodelica::arrayLength(iScVarTaskMapping.clone())) {
                varTask = metamodelica::arrayGet(iScVarTaskMapping.clone(), currentScVarIdx.clone() + iRealScVarIdxStart.clone())?;
            } else {
                varTask = -1;
            }
            if boolAnd(intGe(varTask.clone(), 1), intGe(metamodelica::arrayLength(iSchedulerInfo.clone()), varTask.clone())) {
                threadIdx = Util::tuple31(metamodelica::arrayGet(iSchedulerInfo.clone(), varTask.clone())?);
            } else {
                threadIdx = -1;
            }
            entry = CacheLineEntry { start: -1, dataType: varDataType.clone(), size: varSize.clone(), scVarIdx: currentScVarIdx.clone() + iScVarIdxStart.clone(), threadOwner: threadIdx.clone() };
            (entry, lastCacheLineNew, newCacheLineCreated) = createCacheMapDefaultCppRuntime1(entry.clone(), iCacheLineSize.clone(), lastCacheLine.clone())?;
            let CacheLineMap { idx: __pa1, entries: __pa2, .. } = (lastCacheLineNew.clone()) else { bail!("pattern mismatch") };
            varCLIdx = __pa1.clone();
            cachelineEntries = __pa2.clone();
            metamodelica::arrayUpdate(iScVarCLMapping.clone(), currentScVarIdx.clone() + iRealScVarIdxStart.clone(), (varCLIdx.clone(), varDataType.clone()))?;
            if newCacheLineCreated.clone() {
                filledCacheLines = metamodelica::cons(lastCacheLine.clone(), filledCacheLines.clone());
            }
            lastCacheLine = lastCacheLineNew.clone();
        }
        currentScVarIdx = currentScVarIdx.clone() + 1;
    }
    oFilledCacheLines = filledCacheLines.clone();
    oLastCacheLine = lastCacheLine.clone();
    oScVarIdx = currentScVarIdx.clone() + iScVarIdxStart.clone();
    Ok((oFilledCacheLines, oLastCacheLine, oScVarIdx))
}

fn createCacheMapDefaultCppRuntime1(mut iCacheLineEntry: CacheLineEntry, mut iCacheLineSize: i32, mut iLastCacheLine: CacheLineMap) -> Result<(CacheLineEntry, CacheLineMap, bool)> {
    let mut oCacheLineEntry: CacheLineEntry = <CacheLineEntry as ::std::default::Default>::default();
    let mut oLastCacheLine: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    let mut oNewOneCreated: bool = false;
    let mut numberOfFreeBytesLastCacheLine: i32 = 0;
    let mut lastCacheLineEntries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut cacheLine: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    let mut cacheLineEntry: CacheLineEntry = <CacheLineEntry as ::std::default::Default>::default();
    let mut entrySize: i32 = 0;
    let mut entryStart: i32 = 0;
    let mut entryType: i32 = 0;
    let mut entryVarIdx: i32 = 0;
    let mut entryThreadOwner: i32 = 0;
    let mut lastCacheLineIdx: i32 = 0;
    let CacheLineEntry { start: __pa0, dataType: __pa1, size: __pa2, scVarIdx: __pa3, threadOwner: __pa4 } = (iCacheLineEntry.clone()) else { bail!("pattern mismatch") };
    entryStart = __pa0.clone();
    entryType = __pa1.clone();
    entrySize = __pa2.clone();
    entryVarIdx = __pa3.clone();
    entryThreadOwner = __pa4.clone();
    let CacheLineMap { idx: __pa5, numBytesFree: __pa6, entries: __pa7 } = (iLastCacheLine.clone()) else { bail!("pattern mismatch") };
    lastCacheLineIdx = __pa5.clone();
    numberOfFreeBytesLastCacheLine = __pa6.clone();
    lastCacheLineEntries = __pa7.clone();
    if intGt(entrySize.clone(), numberOfFreeBytesLastCacheLine.clone()) {
        cacheLineEntry = CacheLineEntry { start: 0, dataType: entryType.clone(), size: entrySize.clone(), scVarIdx: entryVarIdx.clone(), threadOwner: entryThreadOwner.clone() };
        cacheLine = CacheLineMap { idx: lastCacheLineIdx.clone() + 1, numBytesFree: iCacheLineSize.clone() - entrySize.clone(), entries: list![cacheLineEntry.clone()] };
        oNewOneCreated = true;
    } else {
        cacheLineEntry = CacheLineEntry { start: iCacheLineSize.clone() - numberOfFreeBytesLastCacheLine.clone(), dataType: entryType.clone(), size: entrySize.clone(), scVarIdx: entryVarIdx.clone(), threadOwner: entryThreadOwner.clone() };
        cacheLine = CacheLineMap { idx: lastCacheLineIdx.clone(), numBytesFree: numberOfFreeBytesLastCacheLine.clone() - entrySize.clone(), entries: metamodelica::cons(cacheLineEntry.clone(), lastCacheLineEntries.clone()) };
        oNewOneCreated = false;
    }
    oCacheLineEntry = cacheLineEntry.clone();
    oLastCacheLine = cacheLine.clone();
    Ok((oCacheLineEntry, oLastCacheLine, oNewOneCreated))
}

fn appendNodeVarsToCacheMap(mut iNodeIdx: i32, mut iOwnerThread: i32, mut iNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>)) -> Result<(CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>) = (<CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0, metamodelica::nil());
    let mut simCodeVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut writtenCL: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut iCacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut iCacheMapMeta: CacheMapMeta = <CacheMapMeta as ::std::default::Default>::default();
    let mut iNumNewCL: i32 = 0;
    let mut varsString: ArcStr = arcstr::literal!("");
    let mut clCandidates: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    simCodeVars = metamodelica::arrayGet(iNodeSimCodeVarMapping.clone(), iNodeIdx.clone())?;
    (iCacheMap, iCacheMapMeta, iNumNewCL, clCandidates) = iInfo.clone();
    varsString = stringDelimitList(List::map(simCodeVars.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone());
    (iCacheMap, iCacheMapMeta, iNumNewCL, clCandidates, writtenCL, _) = List::fold(simCodeVars.clone(), (std::sync::Arc::new({ let __pe_b1 = iOwnerThread.clone(); move |__pe_a0, __pe_a2| appendSCVarToCacheMap(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(i32, (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, i32)) -> Result<(CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, i32)> + 'static>), (iCacheMap.clone(), iCacheMapMeta.clone(), iNumNewCL.clone(), clCandidates.clone(), metamodelica::nil(), 1))?;
    clCandidates = List::removeOnTrue(writtenCL.clone(), (std::sync::Arc::new(appendNodeVarsToCacheMap0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, (i32, i32)) -> Result<bool> + 'static>), clCandidates.clone())?;
    oInfo = (iCacheMap.clone(), iCacheMapMeta.clone(), iNumNewCL.clone(), clCandidates.clone());
    Ok(oInfo)
}

fn appendNodeVarsToCacheMap0(mut iWrittenCLs: Arc<metamodelica::List<i32>>, mut iDetailedCLInfo: (i32, i32)) -> Result<bool> {
    let mut oRemove: bool = false;
    let mut clIdx: i32 = 0;
    let mut freeBytes: i32 = 0;
    let mut res: bool = false;
    oRemove = 'mc: {
        let __mc_input = iDetailedCLInfo.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let (mut clIdx, mut freeBytes) = __mc_input.clone() else { bail!("nomatch") };
            let true = (intEq(freeBytes.clone(), 0)) else { bail!("pattern mismatch") };
            Ok(true)
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let (mut clIdx, mut freeBytes) = __mc_input.clone() else { bail!("nomatch") };
            let mut res: bool = res.clone();
            res = List::isMemberOnTrue(clIdx.clone(), iWrittenCLs.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
            Ok((res.clone(), res.clone()))
        })() { res = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("appendNodeVarsToCacheMap0 failed!\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oRemove)
}

fn appendSCVarToCacheMap(mut iSCVarIdx: i32, mut iOwnerThread: i32, mut iInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, i32)) -> Result<(CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, i32)> {
    let mut oInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, i32) = (<CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0, metamodelica::nil(), metamodelica::nil(), 0);
    let mut iAllSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>> = Default::default();
    let mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)> = Default::default();
    let mut iScVarCLMapping: metamodelica::Array<(i32, i32)> = Default::default();
    let mut currentCLCandidateIdx: i32 = 0;
    let mut currentCLCandidateCLIdx: i32 = 0;
    let mut clIdx: i32 = 0;
    let mut currentCLCandidateFreeBytes: i32 = 0;
    let mut cacheLineSize: i32 = 0;
    let mut numNewCL: i32 = 0;
    let mut varDataType: i32 = 0;
    let mut numBytesRequired: i32 = 0;
    let mut entryStart: i32 = 0;
    let mut currentCLCandidate: (i32, i32) = (0, 0);
    let mut cacheLineCandidates: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheLine: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    let mut CLentries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut scVar: SimCodeVar::SimVar = <SimCodeVar::SimVar as ::std::default::Default>::default();
    let mut numCacheVars: i32 = 0;
    let mut freeSpace: i32 = 0;
    let mut numBytesFree: i32 = 0;
    let mut cacheMap: CacheMap = <CacheMap as ::std::default::Default>::default();
    let mut cacheMapMeta: CacheMapMeta = <CacheMapMeta as ::std::default::Default>::default();
    let mut writtenCL: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, i32) = (<CacheMap as ::std::default::Default>::default(), <CacheMapMeta as ::std::default::Default>::default(), 0, metamodelica::nil(), metamodelica::nil(), 0);
    oInfo = 'mc: {
        let __mc_input = iInfo.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9, __wb10, __wb11)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cacheMap @ CacheMap::CACHEMAP { cacheLineSize, cacheVariables, cacheLinesFloat, cacheLinesInt, cacheLinesBool }, cacheMapMeta @ CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping, simCodeVarTypes: iSimCodeVarTypes, scVarCLMapping: iScVarCLMapping }, numNewCL, cacheLineCandidates, writtenCL, currentCLCandidateIdx) => {
                    let mut cacheMap = (*cacheMap).clone();
                    let mut cacheVariables = (*cacheVariables).clone();
                    let mut cacheLinesFloat = (*cacheLinesFloat).clone();
                    let mut cacheMapMeta = (*cacheMapMeta).clone();
                    let mut iScVarCLMapping = (*iScVarCLMapping).clone();
                    let mut cacheLineCandidates = (*cacheLineCandidates).clone();
                    let mut writtenCL = (*writtenCL).clone();
                    let mut CLentries: Arc<metamodelica::List<CacheLineEntry>> = CLentries.clone();
                    let mut cacheLine: CacheLineMap = cacheLine.clone();
                    let mut clIdx: i32 = clIdx.clone();
                    let mut currentCLCandidate: (i32, i32) = currentCLCandidate.clone();
                    let mut currentCLCandidateCLIdx: i32 = currentCLCandidateCLIdx.clone();
                    let mut currentCLCandidateFreeBytes: i32 = currentCLCandidateFreeBytes.clone();
                    let mut entryStart: i32 = entryStart.clone();
                    let mut numBytesFree: i32 = numBytesFree.clone();
                    let mut numBytesRequired: i32 = numBytesRequired.clone();
                    let mut numCacheVars: i32 = numCacheVars.clone();
                    let mut scVar: SimCodeVar::SimVar = scVar.clone();
                    let mut varDataType: i32 = varDataType.clone();
                    let true = (intGe((cacheLineCandidates.clone().len() as i32), currentCLCandidateIdx.clone())) else { bail!("pattern mismatch") };
                    currentCLCandidate = (cacheLineCandidates.clone()).get(currentCLCandidateIdx.clone())?;
                    (varDataType, numBytesRequired, _) = metamodelica::arrayGet(iSimCodeVarTypes.clone(), iSCVarIdx.clone())?;
                    let true = (doesSCVarFitIntoCL(currentCLCandidate.clone(), numBytesRequired.clone())) else { bail!("pattern mismatch") };
                    (currentCLCandidateCLIdx, currentCLCandidateFreeBytes) = currentCLCandidate.clone();
                    cacheLine = (cacheLinesFloat.clone()).get((cacheLinesFloat.clone().len() as i32) - currentCLCandidateCLIdx.clone() + 1)?;
                    let CacheLineMap { idx: __pa0, numBytesFree: __pa1, entries: __pa2 } = (cacheLine.clone()) else { bail!("pattern mismatch") };
                    clIdx = __pa0.clone();
                    numBytesFree = __pa1.clone();
                    CLentries = __pa2.clone();
                    entryStart = cacheLineSize.clone() - currentCLCandidateFreeBytes.clone();
                    numCacheVars = (cacheVariables.clone().len() as i32) + 1;
                    CLentries = metamodelica::cons(CacheLineEntry { start: entryStart.clone(), dataType: varDataType.clone(), size: numBytesRequired.clone(), scVarIdx: numCacheVars.clone(), threadOwner: iOwnerThread.clone() }, CLentries.clone());
                    cacheLine = CacheLineMap { idx: clIdx.clone(), numBytesFree: numBytesFree.clone() + numBytesRequired.clone(), entries: CLentries.clone() };
                    cacheLinesFloat = List::set(cacheLinesFloat.clone(), (cacheLinesFloat.clone().len() as i32) - currentCLCandidateCLIdx.clone() + 1, cacheLine.clone())?;
                    iScVarCLMapping = metamodelica::arrayUpdate(iScVarCLMapping.clone(), iSCVarIdx.clone(), (clIdx.clone(), varDataType.clone()))?;
                    let __pa3 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(iAllSCVarsMapping.clone(), iSCVarIdx.clone())?) {
                        Some(__pa3) => __pa3.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    scVar = __pa3.clone();
                    cacheVariables = metamodelica::cons(scVar.clone(), cacheVariables.clone());
                    writtenCL = metamodelica::cons(clIdx.clone(), writtenCL.clone());
                    currentCLCandidate = (currentCLCandidateCLIdx.clone(), currentCLCandidateFreeBytes.clone() - numBytesRequired.clone());
                    cacheLineCandidates = List::set(cacheLineCandidates.clone(), currentCLCandidateIdx.clone(), currentCLCandidate.clone())?;
                    cacheMap = CacheMap::CACHEMAP { cacheLineSize: cacheLineSize.clone(), cacheVariables: cacheVariables.clone(), cacheLinesFloat: cacheLinesFloat.clone(), cacheLinesInt: cacheLinesInt.clone(), cacheLinesBool: cacheLinesBool.clone() };
                    cacheMapMeta = CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping.clone(), simCodeVarTypes: iSimCodeVarTypes.clone(), scVarCLMapping: iScVarCLMapping.clone() };
                    Ok(((cacheMap.clone(), cacheMapMeta.clone(), numNewCL.clone(), cacheLineCandidates.clone(), writtenCL.clone(), currentCLCandidateIdx.clone()), CLentries.clone(), cacheLine.clone(), clIdx.clone(), currentCLCandidate.clone(), currentCLCandidateCLIdx.clone(), currentCLCandidateFreeBytes.clone(), entryStart.clone(), numBytesFree.clone(), numBytesRequired.clone(), numCacheVars.clone(), scVar.clone(), varDataType.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { CLentries = __wb0; cacheLine = __wb1; clIdx = __wb2; currentCLCandidate = __wb3; currentCLCandidateCLIdx = __wb4; currentCLCandidateFreeBytes = __wb5; entryStart = __wb6; numBytesFree = __wb7; numBytesRequired = __wb8; numCacheVars = __wb9; scVar = __wb10; varDataType = __wb11; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cacheMap @ CacheMap::CACHEMAP { cacheLineSize, cacheVariables, cacheLinesFloat, cacheLinesInt, cacheLinesBool }, cacheMapMeta @ CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping, simCodeVarTypes: iSimCodeVarTypes, scVarCLMapping: iScVarCLMapping }, numNewCL, cacheLineCandidates, writtenCL, currentCLCandidateIdx) => {
                    let mut numBytesRequired: i32 = numBytesRequired.clone();
                    let mut tmpInfo: (CacheMap, CacheMapMeta, i32, Arc<metamodelica::List<(i32, i32)>>, Arc<metamodelica::List<i32>>, i32) = tmpInfo.clone();
                    let mut varDataType: i32 = varDataType.clone();
                    let true = (intGe((cacheLineCandidates.clone().len() as i32), currentCLCandidateIdx.clone())) else { bail!("pattern mismatch") };
                    (varDataType, numBytesRequired, _) = metamodelica::arrayGet(iSimCodeVarTypes.clone(), iSCVarIdx.clone())?;
                    tmpInfo = appendSCVarToCacheMap(iSCVarIdx.clone(), iOwnerThread.clone(), (cacheMap.clone(), cacheMapMeta.clone(), numNewCL.clone(), cacheLineCandidates.clone(), writtenCL.clone(), currentCLCandidateIdx.clone() + 1))?;
                    Ok((tmpInfo.clone(), numBytesRequired.clone(), tmpInfo.clone(), varDataType.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { numBytesRequired = __wb0; tmpInfo = __wb1; varDataType = __wb2; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8, __wb9)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cacheMap @ CacheMap::CACHEMAP { cacheLineSize, cacheVariables, cacheLinesFloat, cacheLinesInt, cacheLinesBool }, CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping, simCodeVarTypes: iSimCodeVarTypes, scVarCLMapping: iScVarCLMapping }, numNewCL, cacheLineCandidates, writtenCL, currentCLCandidateIdx) => {
                    let mut cacheMap = (*cacheMap).clone();
                    let mut cacheVariables = (*cacheVariables).clone();
                    let mut cacheLinesFloat = (*cacheLinesFloat).clone();
                    let mut iScVarCLMapping = (*iScVarCLMapping).clone();
                    let mut cacheLineCandidates = (*cacheLineCandidates).clone();
                    let mut writtenCL = (*writtenCL).clone();
                    let mut CLentries: Arc<metamodelica::List<CacheLineEntry>> = CLentries.clone();
                    let mut cacheLine: CacheLineMap = cacheLine.clone();
                    let mut cacheMapMeta: CacheMapMeta = cacheMapMeta.clone();
                    let mut clIdx: i32 = clIdx.clone();
                    let mut entryStart: i32 = entryStart.clone();
                    let mut freeSpace: i32 = freeSpace.clone();
                    let mut numBytesRequired: i32 = numBytesRequired.clone();
                    let mut numCacheVars: i32 = numCacheVars.clone();
                    let mut scVar: SimCodeVar::SimVar = scVar.clone();
                    let mut varDataType: i32 = varDataType.clone();
                    (varDataType, numBytesRequired, _) = metamodelica::arrayGet(iSimCodeVarTypes.clone(), iSCVarIdx.clone())?;
                    entryStart = 0;
                    numCacheVars = (cacheVariables.clone().len() as i32) + 1;
                    CLentries = list![CacheLineEntry { start: entryStart.clone(), dataType: varDataType.clone(), size: numBytesRequired.clone(), scVarIdx: numCacheVars.clone(), threadOwner: iOwnerThread.clone() }];
                    clIdx = (cacheLinesFloat.clone().len() as i32) + 1;
                    cacheLine = CacheLineMap { idx: clIdx.clone(), numBytesFree: numBytesRequired.clone(), entries: CLentries.clone() };
                    cacheLinesFloat = metamodelica::cons(cacheLine.clone(), cacheLinesFloat.clone());
                    iScVarCLMapping = metamodelica::arrayUpdate(iScVarCLMapping.clone(), iSCVarIdx.clone(), (clIdx.clone(), varDataType.clone()))?;
                    let __pa0 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(iAllSCVarsMapping.clone(), iSCVarIdx.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    scVar = __pa0.clone();
                    cacheVariables = metamodelica::cons(scVar.clone(), cacheVariables.clone());
                    writtenCL = metamodelica::cons(clIdx.clone(), writtenCL.clone());
                    freeSpace = cacheLineSize.clone() - numBytesRequired.clone();
                    cacheLineCandidates = List::appendElt((clIdx.clone(), freeSpace.clone()), cacheLineCandidates.clone());
                    cacheMap = CacheMap::CACHEMAP { cacheLineSize: cacheLineSize.clone(), cacheVariables: cacheVariables.clone(), cacheLinesFloat: cacheLinesFloat.clone(), cacheLinesInt: cacheLinesInt.clone(), cacheLinesBool: cacheLinesBool.clone() };
                    cacheMapMeta = CacheMapMeta { allSCVarsMapping: iAllSCVarsMapping.clone(), simCodeVarTypes: iSimCodeVarTypes.clone(), scVarCLMapping: iScVarCLMapping.clone() };
                    Ok(((cacheMap.clone(), cacheMapMeta.clone(), numNewCL.clone() + 1, cacheLineCandidates.clone(), writtenCL.clone(), currentCLCandidateIdx.clone()), CLentries.clone(), cacheLine.clone(), cacheMapMeta.clone(), clIdx.clone(), entryStart.clone(), freeSpace.clone(), numBytesRequired.clone(), numCacheVars.clone(), scVar.clone(), varDataType.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { CLentries = __wb0; cacheLine = __wb1; cacheMapMeta = __wb2; clIdx = __wb3; entryStart = __wb4; freeSpace = __wb5; numBytesRequired = __wb6; numCacheVars = __wb7; scVar = __wb8; varDataType = __wb9; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("appendSCVarToCacheMap failed! Variable skipped.\n")).clone());
                    Ok(iInfo.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oInfo)
}

fn doesSCVarFitIntoCL(mut iCacheLineCandidate: (i32, i32), mut iNumBytes: i32) -> bool {
    let mut oResult: bool = false;
    let mut freeSpace: i32 = 0;
    (_, freeSpace) = iCacheLineCandidate.clone();
    oResult = intGe(freeSpace.clone(), iNumBytes.clone());
    oResult
}

fn createDetailedCacheMapInformation(mut iCacheLinesIdc: Arc<metamodelica::List<i32>>, mut iCacheLines: Arc<metamodelica::List<CacheLineMap>>, mut iCacheLineSize: i32) -> Result<Arc<metamodelica::List<(i32, i32)>>> {
    let mut oCacheLines: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut iCacheLinesArray: metamodelica::Array<CacheLineMap> = Default::default();
    iCacheLinesArray = metamodelica::arrayFromVec(iCacheLines.clone().into_iter().cloned().collect());
    oCacheLines = List::fold2(iCacheLinesIdc.clone(), (std::sync::Arc::new(createDetailedCacheMapInformation0) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<CacheLineMap>, i32, Arc<metamodelica::List<(i32, i32)>>) -> Result<Arc<metamodelica::List<(i32, i32)>>> + 'static>), iCacheLinesArray.clone(), iCacheLineSize.clone(), metamodelica::nil())?;
    Ok(oCacheLines)
}

fn createDetailedCacheMapInformation0(mut iCacheLineIdx: i32, mut iCacheLinesArray: metamodelica::Array<CacheLineMap>, mut iCacheLineSize: i32, mut iCacheLines: Arc<metamodelica::List<(i32, i32)>>) -> Result<Arc<metamodelica::List<(i32, i32)>>> {
    let mut oCacheLines: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    let mut cacheLineEntry: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    let mut numBytesFree: i32 = 0;
    let mut cacheLines: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    oCacheLines = 'mc: {
        let __mc_input = iCacheLines.clone();
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut cacheLineEntry: CacheLineMap = cacheLineEntry.clone();
                    let mut cacheLines: Arc<metamodelica::List<(i32, i32)>> = cacheLines.clone();
                    let mut numBytesFree: i32 = numBytesFree.clone();
                    cacheLineEntry = metamodelica::arrayGet(iCacheLinesArray.clone(), metamodelica::arrayLength(iCacheLinesArray.clone()) - iCacheLineIdx.clone() + 1)?;
                    numBytesFree = iCacheLineSize.clone() - getNumOfUsedBytesByCacheLine(cacheLineEntry.clone())?;
                    let true = (intGt(numBytesFree.clone(), 0)) else { bail!("pattern mismatch") };
                    cacheLines = metamodelica::cons((iCacheLineIdx.clone(), numBytesFree.clone()), iCacheLines.clone());
                    Ok((cacheLines.clone(), cacheLineEntry.clone(), cacheLines.clone(), numBytesFree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { cacheLineEntry = __wb0; cacheLines = __wb1; numBytesFree = __wb2; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iCacheLines.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oCacheLines)
}

fn getNumOfUsedBytesByCacheLine(mut iCacheLineMap: CacheLineMap) -> Result<i32> {
    let mut oNumBytes: i32 = 0;
    let mut entries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut firstEntryStart: i32 = 0;
    let mut firstEntrySize: i32 = 0;
    let CacheLineMap { entries: __pa0, .. } = (iCacheLineMap.clone()) else { bail!("pattern mismatch") };
    entries = __pa0.clone();
    entries = List::sort(entries.clone(), (std::sync::Arc::new(sortCacheLineEntriesByPos) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineEntry, CacheLineEntry) -> Result<bool> + 'static>))?;
    let CacheLineEntry { start: __pa1, size: __pa2, .. } = (List::last(entries.clone())?) else { bail!("pattern mismatch") };
    firstEntryStart = __pa1.clone();
    firstEntrySize = __pa2.clone();
    oNumBytes = firstEntryStart.clone() + firstEntrySize.clone();
    Ok(oNumBytes)
}

fn sortCacheLineEntriesByPos(mut iCacheLineEntry1: CacheLineEntry, mut iCacheLineEntry2: CacheLineEntry) -> Result<bool> {
    let mut oIsGreater: bool = false;
    let mut start1: i32 = 0;
    let mut start2: i32 = 0;
    let CacheLineEntry { start: __pa0, .. } = (iCacheLineEntry1.clone()) else { bail!("pattern mismatch") };
    start1 = __pa0.clone();
    let CacheLineEntry { start: __pa1, .. } = (iCacheLineEntry2.clone()) else { bail!("pattern mismatch") };
    start2 = __pa1.clone();
    oIsGreater = intGt(start1.clone(), start2.clone());
    Ok(oIsGreater)
}

fn reverseCacheLineMapEntries(mut iCacheLineMap: CacheLineMap) -> Result<CacheLineMap> {
    let mut oCacheLineMap: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    let mut idx: i32 = 0;
    let mut numBytesFree: i32 = 0;
    let mut entries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let CacheLineMap { idx: __pa0, numBytesFree: __pa1, entries: __pa2 } = (iCacheLineMap.clone()) else { bail!("pattern mismatch") };
    idx = __pa0.clone();
    numBytesFree = __pa1.clone();
    entries = __pa2.clone();
    entries = entries.clone().reverse();
    oCacheLineMap = CacheLineMap { idx: idx.clone(), numBytesFree: numBytesFree.clone(), entries: entries.clone() };
    Ok(oCacheLineMap)
}

fn compareCacheLineMapByIdx(mut iCacheLineMap: CacheLineMap, mut iCacheLineMap2: CacheLineMap) -> Result<bool> {
    let mut oIsGreater: bool = false;
    let mut idx1: i32 = 0;
    let mut idx2: i32 = 0;
    let CacheLineMap { idx: __pa0, .. } = (iCacheLineMap.clone()) else { bail!("pattern mismatch") };
    idx1 = __pa0.clone();
    let CacheLineMap { idx: __pa1, .. } = (iCacheLineMap2.clone()) else { bail!("pattern mismatch") };
    idx2 = __pa1.clone();
    oIsGreater = intGt(idx1.clone(), idx2.clone());
    Ok(oIsGreater)
}

fn convertCacheToVarArrayMapping(mut iCacheMap: CacheMap, mut iCacheLineSize: i32, mut iStateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iDerivativeVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iIntAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iBoolAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iStringAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iVarSizes: (i32, i32, i32), mut iNotOptimizedVars: (Arc<metamodelica::List<SimCodeVar::SimVar>>, Arc<metamodelica::List<SimCodeVar::SimVar>>, Arc<metamodelica::List<SimCodeVar::SimVar>>, Arc<metamodelica::List<SimCodeVar::SimVar>>)) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), Option<HpcOmSimCode::MemoryMap>)> {
    let mut oVarToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
    let mut oVarToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut oMemoryMap: Option<HpcOmSimCode::MemoryMap> = None;
    let mut cacheLineSize: i32 = 0;
    let mut maxNumElemsFloat: i32 = 0;
    let mut maxNumElemsInt: i32 = 0;
    let mut maxNumElemsBool: i32 = 0;
    let mut stateAndStateDerSize: i32 = 0;
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheVariablesArray: metamodelica::Array<SimCodeVar::SimVar> = Default::default();
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut allCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut varArrayIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
    let mut varIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut varSizeFloat: i32 = 0;
    let mut varSizeInt: i32 = 0;
    let mut varSizeBool: i32 = 0;
    let mut varSizeString: i32 = 0;
    let mut varIdxOffsets: metamodelica::Array<i32> = Default::default();
    let mut notOptimizedVarsFloat: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsInt: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsBool: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut notOptimizedVarsString: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut currentVarIndices: metamodelica::Array<i32> = Default::default();
    (oVarToArrayIndexMapping, oVarToIndexMapping, oMemoryMap) = (::match_deref::match_deref! { match &((iCacheMap.clone(), iVarSizes.clone(), iNotOptimizedVars.clone())) {
        (CacheMap::CACHEMAP { cacheLineSize: __esc_cacheLineSize, cacheVariables: __esc_cacheVariables, cacheLinesFloat: __esc_cacheLinesFloat, cacheLinesInt: __esc_cacheLinesInt, cacheLinesBool: __esc_cacheLinesBool }, (__esc_varSizeFloat, __esc_varSizeInt, __esc_varSizeBool), (__esc_notOptimizedVarsFloat, __esc_notOptimizedVarsInt, __esc_notOptimizedVarsBool, __esc_notOptimizedVarsString)) => {
            cacheLineSize = (*__esc_cacheLineSize).clone();
            cacheVariables = (*__esc_cacheVariables).clone();
            cacheLinesFloat = (*__esc_cacheLinesFloat).clone();
            cacheLinesInt = (*__esc_cacheLinesInt).clone();
            cacheLinesBool = (*__esc_cacheLinesBool).clone();
            varSizeFloat = (*__esc_varSizeFloat).clone();
            varSizeInt = (*__esc_varSizeInt).clone();
            varSizeBool = (*__esc_varSizeBool).clone();
            notOptimizedVarsFloat = (*__esc_notOptimizedVarsFloat).clone();
            notOptimizedVarsInt = (*__esc_notOptimizedVarsInt).clone();
            notOptimizedVarsBool = (*__esc_notOptimizedVarsBool).clone();
            notOptimizedVarsString = (*__esc_notOptimizedVarsString).clone();
            maxNumElemsFloat = intDiv(iCacheLineSize.clone(), varSizeFloat.clone());
            maxNumElemsInt = intDiv(iCacheLineSize.clone(), varSizeInt.clone());
            maxNumElemsBool = intDiv(iCacheLineSize.clone(), varSizeBool.clone());
            cacheVariablesArray = metamodelica::arrayFromVec(cacheVariables.clone().into_iter().cloned().collect());
            varArrayIndexMappingHashTable = HashTableCrIListArray::emptyHashTable();
            varIndexMappingHashTable = HashTableCrILst::emptyHashTable();
            currentVarIndices = arrayCreate(4, 1);
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(iStateVars.clone(), VARDATATYPE_FLOAT.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(iDerivativeVars.clone(), VARDATATYPE_FLOAT.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            stateAndStateDerSize = intAdd((iStateVars.clone().len() as i32), (iDerivativeVars.clone().len() as i32));
            if intEq(intMod(stateAndStateDerSize.clone(), maxNumElemsFloat.clone()), 0) {
                metamodelica::arrayUpdate(currentVarIndices.clone(), 1, stateAndStateDerSize.clone() + 1)?;
                metamodelica::arrayUpdate(currentVarIndices.clone(), 2, 1)?;
                metamodelica::arrayUpdate(currentVarIndices.clone(), 3, 1)?;
                metamodelica::arrayUpdate(currentVarIndices.clone(), 4, 1)?;
            } else {
                metamodelica::arrayUpdate(currentVarIndices.clone(), 1, stateAndStateDerSize.clone() + (maxNumElemsFloat.clone() - intMod(stateAndStateDerSize.clone(), maxNumElemsFloat.clone())) + 1)?;
                metamodelica::arrayUpdate(currentVarIndices.clone(), 2, 1)?;
                metamodelica::arrayUpdate(currentVarIndices.clone(), 3, 1)?;
                metamodelica::arrayUpdate(currentVarIndices.clone(), 4, 1)?;
            }
            varSizeFloat = metamodelica::arrayGet(currentVarIndices.clone(), 1)?;
            varIdxOffsets = arrayCreate(3, 1);
            varIdxOffsets = metamodelica::arrayUpdate(varIdxOffsets.clone(), 1, metamodelica::arrayGet(currentVarIndices.clone(), 1)? + 1)?;
            allCacheLines = List::sort(getAllCacheLinesOfCacheMap(iCacheMap.clone())?, (std::sync::Arc::new(compareCacheLineMapByIdx) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineMap, CacheLineMap) -> Result<bool> + 'static>))?;
            (varArrayIndexMappingHashTable, varIndexMappingHashTable) = List::fold(allCacheLines.clone(), (std::sync::Arc::new({ let __pe_b1 = cacheLineSize.clone(); let __pe_b2 = varIdxOffsets.clone(); let __pe_b3 = cacheVariablesArray.clone(); move |__pe_a0, __pe_a4| addCacheLineMapToVarArrayMapping(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineMap, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)))> + 'static>), (varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone()))?;
            metamodelica::arrayUpdate(currentVarIndices.clone(), 1, metamodelica::arrayGet(currentVarIndices.clone(), 1)? + intMul((cacheLinesFloat.clone().len() as i32), maxNumElemsFloat.clone()))?;
            metamodelica::arrayUpdate(currentVarIndices.clone(), 2, intMul((cacheLinesInt.clone().len() as i32), maxNumElemsInt.clone()) + 1)?;
            metamodelica::arrayUpdate(currentVarIndices.clone(), 3, intMul((cacheLinesBool.clone().len() as i32), maxNumElemsBool.clone()) + 1)?;
            metamodelica::arrayUpdate(currentVarIndices.clone(), 4, 1)?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(notOptimizedVarsFloat.clone().reverse(), VARDATATYPE_FLOAT.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(notOptimizedVarsInt.clone().reverse(), VARDATATYPE_INTEGER.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(notOptimizedVarsBool.clone().reverse(), VARDATATYPE_BOOLEAN.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(notOptimizedVarsString.clone().reverse(), VARDATATYPE_STRING.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(iAliasVars.clone(), VARDATATYPE_FLOAT.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(iIntAliasVars.clone(), VARDATATYPE_INTEGER.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(iBoolAliasVars.clone(), VARDATATYPE_BOOLEAN.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (currentVarIndices, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMappings(iStringAliasVars.clone(), VARDATATYPE_STRING.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            varSizeFloat = varSizeFloat.clone() + intMul((cacheLinesFloat.clone().len() as i32), maxNumElemsFloat.clone()) + (notOptimizedVarsFloat.clone().len() as i32);
            varSizeInt = intMul((cacheLinesInt.clone().len() as i32), maxNumElemsInt.clone()) + (notOptimizedVarsInt.clone().len() as i32);
            varSizeBool = intMul((cacheLinesBool.clone().len() as i32), maxNumElemsBool.clone()) + (notOptimizedVarsBool.clone().len() as i32);
            varSizeString = (notOptimizedVarsString.clone().len() as i32);
            (varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone(), Some(HpcOmSimCode::MemoryMap::MEMORYMAP_ARRAY { floatArraySize: varSizeFloat.clone(), intArraySize: varSizeInt.clone(), boolArraySize: varSizeBool.clone(), stringArraySize: varSizeString.clone() }))
        },
        (CacheMap::UNIFORM_CACHEMAP { .. }, _, _) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("ConvertCacheToVarArrayMapping: Uniform-CacheMap not supported!")).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("ConvertCacheToVarArrayMapping: CacheMap-Type not supported!")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oVarToArrayIndexMapping, oVarToIndexMapping, oMemoryMap))
}

fn addCacheLineMapToVarArrayMapping(mut iCacheLineMap: CacheLineMap, mut iCacheLineSize: i32, mut iVarIdxOffsets: metamodelica::Array<i32>, mut iCacheVariables: metamodelica::Array<SimCodeVar::SimVar>, mut iPositionMapping: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)))> {
    let mut oPositionMapping: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)));
    let mut varArrayIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
    let mut varIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut idx: i32 = 0;
    let mut entries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut dataType: i32 = 0;
    let mut size: i32 = 0;
    oPositionMapping = (match (iCacheLineMap.clone(), iPositionMapping.clone()) {
        (CacheLineMap { idx: mut __esc_idx, entries: mut __esc_entries, .. }, (mut __esc_varArrayIndexMappingHashTable, mut __esc_varIndexMappingHashTable)) => {
            idx = __esc_idx.clone();
            entries = __esc_entries.clone();
            varArrayIndexMappingHashTable = __esc_varArrayIndexMappingHashTable.clone();
            varIndexMappingHashTable = __esc_varIndexMappingHashTable.clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(entries.clone()) {
                Deref @ metamodelica::List::Cons { head: CacheLineEntry { dataType: __pa0, size: __pa1, .. }, tail: _ } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            dataType = __pa0.clone();
            size = __pa1.clone();
            (varArrayIndexMappingHashTable, varIndexMappingHashTable) = List::fold(entries.clone(), (std::sync::Arc::new({ let __pe_b1 = dataType.clone(); let __pe_b2 = (idx.clone(), iCacheLineSize.clone()); let __pe_b3 = iVarIdxOffsets.clone(); let __pe_b4 = iCacheVariables.clone(); move |__pe_a0, __pe_a5| addCacheLineEntryToVarArrayMapping(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_a5) }) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineEntry, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)))> + 'static>), iPositionMapping.clone())?;
            metamodelica::arrayUpdate(iVarIdxOffsets.clone(), dataType.clone(), intAdd(metamodelica::arrayGet(iVarIdxOffsets.clone(), dataType.clone())?, intDiv(iCacheLineSize.clone(), size.clone())))?;
            (varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("addCacheLineMapToVarArrayMapping failed! CacheLineMap-Type not supported!")).clone()])?;
            bail!("fail")
        },
    });
    Ok(oPositionMapping)
}

fn addCacheLineEntryToVarArrayMapping(mut iCacheLineEntry: CacheLineEntry, mut iArrayIdx: i32, mut iClIdxSize: (i32, i32), mut iVarIdxOffsets: metamodelica::Array<i32>, mut iCacheVariables: metamodelica::Array<SimCodeVar::SimVar>, mut iPositionMapping: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)))> {
    let mut oPositionMapping: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)));
    let mut varArrayIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
    let mut varIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut scVarIdx: i32 = 0;
    let mut start: i32 = 0;
    let mut size: i32 = 0;
    let mut arrayPosition: i32 = 0;
    let mut offset: i32 = 0;
    let mut currentVarIndices: metamodelica::Array<i32> = Default::default();
    oPositionMapping = (match (iCacheLineEntry.clone(), iPositionMapping.clone()) {
        (CacheLineEntry { scVarIdx: mut __esc_scVarIdx, start: mut __esc_start, size: mut __esc_size, .. }, (mut __esc_varArrayIndexMappingHashTable, mut __esc_varIndexMappingHashTable)) => {
            scVarIdx = __esc_scVarIdx.clone();
            start = __esc_start.clone();
            size = __esc_size.clone();
            varArrayIndexMappingHashTable = __esc_varArrayIndexMappingHashTable.clone();
            varIndexMappingHashTable = __esc_varIndexMappingHashTable.clone();
            offset = metamodelica::arrayGet(iVarIdxOffsets.clone(), iArrayIdx.clone())?;
            arrayPosition = intDiv(start.clone(), size.clone()) + offset.clone();
            currentVarIndices = arrayCreate(4, arrayPosition.clone());
            (_, varArrayIndexMappingHashTable, varIndexMappingHashTable) = SimCodeUtilShared::addVarToArrayIndexMapping(metamodelica::arrayGet(iCacheVariables.clone(), metamodelica::arrayLength(iCacheVariables.clone()) - scVarIdx.clone() + 1)?, iArrayIdx.clone(), currentVarIndices.clone(), varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())?;
            (varArrayIndexMappingHashTable.clone(), varIndexMappingHashTable.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("addCacheLineEntryToVarArrayMapping failed! Unsupported entry-type\n")).clone()])?;
            bail!("fail")
        },
    });
    Ok(oPositionMapping)
}

fn convertCacheToVarArrayMapping2Helper(mut iArray: metamodelica::Array<i32>, mut iOffset: i32, mut iIndex: i32) -> Result<metamodelica::Array<i32>> {
    let mut oArray: metamodelica::Array<i32> = Default::default();
    let mut tmpArray: metamodelica::Array<i32> = Default::default();
    let mut i: i32 = 0;
    tmpArray = iArray.clone();
    for mut i in 1..=metamodelica::arrayLength(tmpArray.clone()) {
        if intNe(i.clone(), iIndex.clone()) {
            tmpArray = metamodelica::arrayUpdate(tmpArray.clone(), i.clone(), metamodelica::arrayGet(tmpArray.clone(), i.clone())? + iOffset.clone())?;
        }
    }
    oArray = tmpArray.clone();
    Ok(oArray)
}

fn getNotOptimizedVarsByCacheLineMapping(mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut iAllVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut oNotOptimizedVars: (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) = (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil());
    (oNotOptimizedVars, _) = Array::fold(iScVarCLMapping.clone(), (std::sync::Arc::new({ let __pe_b1 = iAllVarsMapping.clone(); let __pe_b2 = iSimCodeVarTypes.clone(); move |__pe_a0, __pe_a3| getNotOptimizedVarsByCacheLineMapping0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32), ((Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), i32)) -> Result<((Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), i32)> + 'static>), ((metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), 1))?;
    Ok(oNotOptimizedVars)
}

fn getNotOptimizedVarsByCacheLineMapping0(mut iScVarCLMapping: (i32, i32), mut iAllVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>, mut iEntries: ((Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), i32)) -> Result<((Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), i32)> {
    let mut oEntries: ((Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>), i32) = ((metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil()), 0);
    let mut tmpSimVarsFloat: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSimVarsInt: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSimVarsBool: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpSimVarsString: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut scVarIdx: i32 = 0;
    let mut dataType: i32 = 0;
    oEntries = 'mc: {
        let __mc_input = (iScVarCLMapping.clone(), iEntries.clone());
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (((-1), _), ((tmpSimVarsFloat, tmpSimVarsInt, tmpSimVarsBool, tmpSimVarsString), scVarIdx)) => {
                    let mut tmpSimVarsFloat = (*tmpSimVarsFloat).clone();
                    let mut tmpSimVarsInt = (*tmpSimVarsInt).clone();
                    let mut tmpSimVarsBool = (*tmpSimVarsBool).clone();
                    let mut tmpSimVarsString = (*tmpSimVarsString).clone();
                    let mut dataType: i32 = dataType.clone();
                    dataType = Util::tuple31(metamodelica::arrayGet(iSimCodeVarTypes.clone(), scVarIdx.clone())?);
                    if intEq(dataType.clone(), VARDATATYPE_FLOAT.clone()) {
                        tmpSimVarsFloat = metamodelica::cons(scVarIdx.clone(), tmpSimVarsFloat.clone());
                    } else {
                        if intEq(dataType.clone(), VARDATATYPE_INTEGER.clone()) {
                            tmpSimVarsInt = metamodelica::cons(scVarIdx.clone(), tmpSimVarsInt.clone());
                        } else {
                            if intEq(dataType.clone(), VARDATATYPE_BOOLEAN.clone()) {
                                        tmpSimVarsBool = metamodelica::cons(scVarIdx.clone(), tmpSimVarsBool.clone());
                            } else {
                                        if intEq(dataType.clone(), VARDATATYPE_STRING.clone()) {
                                            tmpSimVarsString = metamodelica::cons(scVarIdx.clone(), tmpSimVarsString.clone());
                                        }
                            }
                        }
                    }
                    Ok((((tmpSimVarsFloat.clone(), tmpSimVarsInt.clone(), tmpSimVarsBool.clone(), tmpSimVarsString.clone()), scVarIdx.clone() + 1), dataType.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { dataType = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ((tmpSimVarsFloat, tmpSimVarsInt, tmpSimVarsBool, tmpSimVarsString), scVarIdx)) => {
                    Ok(((tmpSimVarsFloat.clone(), tmpSimVarsInt.clone(), tmpSimVarsBool.clone(), tmpSimVarsString.clone()), scVarIdx.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oEntries)
}

// -------------------------------------------
// ANALYSIS
// -------------------------------------------
fn evaluateCacheBehaviour(mut iVarToIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), mut iSimVarIdxMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), mut taskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut taskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNumberOfThreads: i32, mut iCacheLineSize: i32, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> () {
    ()
}

fn createVarCLMappingFromVarArrayIndexHashTable(mut iVarToIndexMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), mut iSimVarIdxMappingHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), mut iCacheLineSize: i32, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut oNumberOfVars: metamodelica::Array<i32> = Default::default();
    let mut oVarToCLMapping: metamodelica::Array<i32> = Default::default();
    let mut hashTableElements: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
    let mut hashTableElement: (Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>) = (Arc::new(DAE::ComponentRef::WILD), metamodelica::nil());
    let mut varToCLMapping: metamodelica::Array<i32> = Default::default();
    let mut numberOfVars: metamodelica::Array<i32> = Default::default();
    let mut pos: i32 = 0;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    varToCLMapping = arrayCreate(metamodelica::arrayLength(iSimCodeVarTypes.clone()), -1);
    numberOfVars = arrayCreate(3, 0);
    hashTableElements = BaseHashTable::hashTableList(iVarToIndexMappingHashTable.clone())?;
    for mut hashTableElement in &*hashTableElements.clone() {
        let mut hashTableElement = hashTableElement.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(hashTableElement.clone()) {
            (__pa0, Deref @ metamodelica::List::Cons { head: __pa1, tail: _ }) => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cref = __pa0.clone();
        pos = __pa1.clone();
    }
    oNumberOfVars = numberOfVars.clone();
    oVarToCLMapping = varToCLMapping.clone();
    Ok((oNumberOfVars, oVarToCLMapping))
}

fn createCacheLineThreadProperties(mut iCacheLine: CacheLineMap, mut iNumberOfThreads: i32, mut iCacheLineSize: i32, mut iCacheLineThreadProperties: metamodelica::Array<metamodelica::Array<metamodelica::Real>>) -> Result<()> {
    let mut bytesPerThread: metamodelica::Array<i32> = Default::default();
    let mut threadProperties: metamodelica::Array<metamodelica::Real> = Default::default();
    let mut cacheLineIdx: i32 = 0;
    let mut threadOwner: i32 = 0;
    let mut size: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut numBytesFree: i32 = 0;
    let mut numBytesUnassigned: i32 = 0;
    let mut entries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut entry: CacheLineEntry = <CacheLineEntry as ::std::default::Default>::default();
    let mut sizeReal: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let CacheLineMap { idx: __pa0, entries: __pa1, numBytesFree: __pa2 } = (iCacheLine.clone()) else { bail!("pattern mismatch") };
    cacheLineIdx = __pa0.clone();
    entries = __pa1.clone();
    numBytesFree = __pa2.clone();
    numBytesUnassigned = 0;
    threadProperties = arrayCreate(iNumberOfThreads.clone(), metamodelica::OrderedFloat(0.0_f64));
    bytesPerThread = arrayCreate(iNumberOfThreads.clone(), 0);
    for mut entry in &*entries.clone() {
        let mut entry = entry.clone();
        let CacheLineEntry { threadOwner: __pa3, size: __pa4, .. } = (entry.clone()) else { bail!("pattern mismatch") };
        threadOwner = __pa3.clone();
        size = __pa4.clone();
        if intLt(threadOwner.clone(), 0) {
            numBytesUnassigned = numBytesUnassigned.clone() + size.clone();
        } else {
            bytesPerThread = metamodelica::arrayUpdate(bytesPerThread.clone(), threadOwner.clone(), metamodelica::arrayGet(bytesPerThread.clone(), threadOwner.clone())? + size.clone())?;
        }
    }
    sizeReal = intReal(iCacheLineSize.clone() - numBytesFree.clone() - numBytesUnassigned.clone());
    if realGt(sizeReal.clone(), metamodelica::OrderedFloat((0) as f64)) {
        for mut threadIdx in 1..=iNumberOfThreads.clone() {
            metamodelica::arrayUpdate(threadProperties.clone(), threadIdx.clone(), realDiv(intReal(metamodelica::arrayGet(bytesPerThread.clone(), threadIdx.clone())?), sizeReal.clone()))?;
        }
    }
    metamodelica::arrayUpdate(iCacheLineThreadProperties.clone(), cacheLineIdx.clone(), threadProperties.clone())?;
    Ok(())
}

fn calculateLocCoRead(mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut cacheLineThreadProperties: metamodelica::Array<metamodelica::Array<metamodelica::Real>>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> Result<metamodelica::Real> {
    let mut oLocCoRead: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut nodeIdx: i32 = 0;
    let mut numberOfNodes: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut sum: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut locCoRead: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    numberOfNodes = metamodelica::arrayLength(iNodeSimCodeVarMapping.clone());
    sum = metamodelica::OrderedFloat(0.0_f64);
    for mut nodeIdx in 1..=numberOfNodes.clone() {
        threadIdx = Util::tuple31(metamodelica::arrayGet(iSchedulerInfo.clone(), nodeIdx.clone())?);
        locCoRead = calculateLocCoReadForTask(nodeIdx.clone(), threadIdx.clone(), iTaskGraphT.clone(), iNodeSimCodeVarMapping.clone(), iScVarCLMapping.clone(), cacheLineThreadProperties.clone())?;
        sum = sum.clone() + locCoRead.clone();
    }
    if intGt(numberOfNodes.clone(), 0) {
        oLocCoRead = realDiv(sum.clone(), metamodelica::OrderedFloat((numberOfNodes.clone()) as f64));
    } else {
        oLocCoRead = metamodelica::OrderedFloat(1.0_f64);
    }
    Ok(oLocCoRead)
}

fn calculateLocCoReadForTask(mut iNodeIdx: i32, mut iThreadIdx: i32, mut iTaskGraphT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut iCacheLineThreadProperties: metamodelica::Array<metamodelica::Array<metamodelica::Real>>) -> Result<metamodelica::Real> {
    let mut oLocCoRead: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut predecessor: i32 = 0;
    let mut numberOfPredecessors: i32 = 0;
    let mut predecessors: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut sum: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    sum = metamodelica::OrderedFloat(0.0_f64);
    predecessors = metamodelica::arrayGet(iTaskGraphT.clone(), iNodeIdx.clone())?;
    numberOfPredecessors = (predecessors.clone().len() as i32);
    for mut predecessor in &*predecessors.clone() {
        let mut predecessor = predecessor.clone();
        sum = sum.clone() + calculateLocCoForTask(predecessor.clone(), iThreadIdx.clone(), metamodelica::arrayGet(iNodeSimCodeVarMapping.clone(), predecessor.clone())?, iScVarCLMapping.clone(), iCacheLineThreadProperties.clone())?;
    }
    if intGt(numberOfPredecessors.clone(), 0) {
        oLocCoRead = realDiv(sum.clone(), metamodelica::OrderedFloat((numberOfPredecessors.clone()) as f64));
    } else {
        oLocCoRead = metamodelica::OrderedFloat(1.0_f64);
    }
    Ok(oLocCoRead)
}

fn calculateLocCoWrite(mut iNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut cacheLineThreadProperties: metamodelica::Array<metamodelica::Array<metamodelica::Real>>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>) -> Result<metamodelica::Real> {
    let mut oLocCoWrite: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut nodeIdx: i32 = 0;
    let mut numberOfNodes: i32 = 0;
    let mut threadIdx: i32 = 0;
    let mut sum: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut locCoWrite: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    numberOfNodes = metamodelica::arrayLength(iNodeSimCodeVarMapping.clone());
    sum = metamodelica::OrderedFloat(0.0_f64);
    for mut nodeIdx in 1..=numberOfNodes.clone() {
        threadIdx = Util::tuple31(metamodelica::arrayGet(iSchedulerInfo.clone(), nodeIdx.clone())?);
        locCoWrite = calculateLocCoForTask(nodeIdx.clone(), threadIdx.clone(), metamodelica::arrayGet(iNodeSimCodeVarMapping.clone(), nodeIdx.clone())?, iScVarCLMapping.clone(), cacheLineThreadProperties.clone())?;
        sum = sum.clone() + locCoWrite.clone();
    }
    if intGt(numberOfNodes.clone(), 0) {
        oLocCoWrite = realDiv(sum.clone(), metamodelica::OrderedFloat((numberOfNodes.clone()) as f64));
    } else {
        oLocCoWrite = metamodelica::OrderedFloat(1.0_f64);
    }
    Ok(oLocCoWrite)
}

fn calculateLocCoForTask(mut iTaskIdx: i32, mut iThreadIdx: i32, mut iNodeSimCodeVarMapping: Arc<metamodelica::List<i32>>, mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut iCacheLineThreadProperties: metamodelica::Array<metamodelica::Array<metamodelica::Real>>) -> Result<metamodelica::Real> {
    let mut oLocCo: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut simCodeVar: i32 = 0;
    let mut clIdx: i32 = 0;
    let mut sum: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    sum = metamodelica::OrderedFloat(0.0_f64);
    for mut simCodeVar in &*iNodeSimCodeVarMapping.clone() {
        let mut simCodeVar = simCodeVar.clone();
        clIdx = Util::tuple21(metamodelica::arrayGet(iScVarCLMapping.clone(), simCodeVar.clone())?);
        sum = sum.clone() + metamodelica::arrayGet(metamodelica::arrayGet(iCacheLineThreadProperties.clone(), clIdx.clone())?, iThreadIdx.clone())?;
    }
    oLocCo = realDiv(sum.clone(), intReal((iNodeSimCodeVarMapping.clone().len() as i32)));
    Ok(oLocCo)
}

// -------------------------------------------
// MAPPINGS
// -------------------------------------------
fn fillSimVarHashTable(mut iSimVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iOffset: i32, mut iType: i32, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))> {
    let mut oHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut tmpHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut simVar: SimCodeVar::SimVar = <SimCodeVar::SimVar as ::std::default::Default>::default();
    let mut index: i32 = 0;
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    tmpHashTable = iHt.clone();
    for mut simVar in &*iSimVars.clone() {
        let mut simVar = simVar.clone();
        let SimCodeVar::SIMVAR { name: __pa0, index: __pa1, .. } = (simVar.clone()) else { bail!("pattern mismatch") };
        name = __pa0.clone();
        index = __pa1.clone();
        index = index.clone() + 1;
        tmpHashTable = BaseHashTable::add((name.clone(), list![index.clone(), iOffset.clone(), iType.clone()]), tmpHashTable.clone())?;
    }
    oHt = tmpHashTable.clone();
    Ok(oHt)
}

fn transposeScVarTaskMapping(mut iScVarTaskMapping: metamodelica::Array<i32>, mut iTaskGraph: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut tmpNodeSimCodeVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut scVarIdx: i32 = 0;
    let mut taskIdx: i32 = 0;
    let mut oldList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    tmpNodeSimCodeVarMapping = arrayCreate(metamodelica::arrayLength(iTaskGraph.clone()), metamodelica::nil());
    for mut scVarIdx in 1..=metamodelica::arrayLength(iScVarTaskMapping.clone()) {
        taskIdx = metamodelica::arrayGet(iScVarTaskMapping.clone(), scVarIdx.clone())?;
        if intGt(taskIdx.clone(), 0) {
            oldList = metamodelica::arrayGet(tmpNodeSimCodeVarMapping.clone(), taskIdx.clone())?;
            oldList = metamodelica::cons(scVarIdx.clone(), oldList.clone());
            metamodelica::arrayUpdate(tmpNodeSimCodeVarMapping.clone(), taskIdx.clone(), oldList.clone())?;
        }
    }
    oNodeSimCodeVarMapping = tmpNodeSimCodeVarMapping.clone();
    Ok(oNodeSimCodeVarMapping)
}

fn transposeTasksScVarsMapping(mut iTasksScVarMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNumberOfScVars: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oScVarTasksMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut tmpScVarTasksMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut scVarIdx: i32 = 0;
    let mut taskIdx: i32 = 0;
    let mut oldList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut scVarIdc: Arc<metamodelica::List<i32>> = metamodelica::nil();
    tmpScVarTasksMapping = arrayCreate(iNumberOfScVars.clone(), metamodelica::nil());
    for mut taskIdx in 1..=metamodelica::arrayLength(iTasksScVarMapping.clone()) {
        scVarIdc = metamodelica::arrayGet(iTasksScVarMapping.clone(), taskIdx.clone())?;
        for mut scVarIdx in &*scVarIdc.clone() {
            let mut scVarIdx = scVarIdx.clone();
            if intGt(scVarIdx.clone(), 0) {
                oldList = metamodelica::arrayGet(tmpScVarTasksMapping.clone(), scVarIdx.clone())?;
                oldList = metamodelica::cons(taskIdx.clone(), oldList.clone());
                metamodelica::arrayUpdate(tmpScVarTasksMapping.clone(), scVarIdx.clone(), oldList.clone())?;
            }
        }
    }
    oScVarTasksMapping = tmpScVarTasksMapping.clone();
    Ok(oScVarTasksMapping)
}

fn getEqSCVarMapping(mut iEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))) -> Result<metamodelica::Array<metamodelica::Array<Arc<metamodelica::List<i32>>>>> {
    let mut oMapping: metamodelica::Array<metamodelica::Array<Arc<metamodelica::List<i32>>>> = Default::default();
    let mut tmpMapping: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<i32>>>>> = metamodelica::nil();
    tmpMapping = List::map1(iEqSystems.clone(), (std::sync::Arc::new(getEqSCVarMappingByEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), iHt.clone())?;
    oMapping = metamodelica::arrayFromVec(tmpMapping.clone().into_iter().cloned().collect());
    Ok(oMapping)
}

fn getEqSCVarMappingByEqSystem(mut iEqSystem: Arc<BackendDAE::EqSystem>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = <Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> as ::std::default::Default>::default();
    let mut equOptList: Arc<metamodelica::List<Option<Arc<BackendDAE::Equation>>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(iEqSystem.clone()) {
        Deref @ BackendDAE::EqSystem { orderedEqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    orderedEqs = __pa0.clone();
    equOptList = Arc::new(ExpandableArray::getData(orderedEqs.clone()).borrow().iter().cloned().collect::<metamodelica::List<_>>());
    oMapping = metamodelica::arrayFromVec(List::map1Option(equOptList.clone(), (std::sync::Arc::new(getEqSCVarMapping0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))) -> Result<Arc<metamodelica::List<i32>>> + 'static>), iHt.clone())?.into_iter().cloned().collect());
    Ok(oMapping)
}

fn getEqSCVarMapping0(mut iEquation: Arc<BackendDAE::Equation>, mut iHt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oMapping: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let (_, (_, (_, __pa0))) = BackendEquation::traverseExpsOfEquation(iEquation.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(createMemoryMapTraverse0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<i32>>))> + 'static>), (iHt.clone(), metamodelica::nil())))?;
    oMapping = __pa0.clone();
    Ok(oMapping)
}

fn createMemoryMapTraverse0(mut inExp: Arc<DAE::Exp>, mut inTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<i32>>)) -> Result<(Arc<DAE::Exp>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<i32>>))> {
    let mut outExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut oTpl: ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)), Arc<metamodelica::List<i32>>);
    let mut iVarList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut oVarList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varInfo: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varIdx: i32 = 0;
    let mut varHead: i32 = 0;
    let mut iHashTable: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut iExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut componentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (outExp, oTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (iExp @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, (iHashTable, iVarList)) => {
                    let mut iVarList = (*iVarList).clone();
                    let mut oVarList: Arc<metamodelica::List<i32>> = oVarList.clone();
                    let mut varHead: i32 = varHead.clone();
                    let mut varIdx: i32 = varIdx.clone();
                    let mut varInfo: Arc<metamodelica::List<i32>> = varInfo.clone();
                    varInfo = BaseHashTable::get(componentRef.clone(), iHashTable.clone())?;
                    varIdx = listHead(varInfo.clone())? + List::second(varInfo.clone())?;
                    if boolNot(iVarList.clone().is_empty()) {
                        varHead = listHead(iVarList.clone())?;
                        if intEq(varHead.clone(), varIdx.clone()) {
                            iVarList = listRest(iVarList.clone())?;
                        }
                    }
                    varInfo = BaseHashTable::get(ComponentReference::crefPrefixDer(componentRef.clone()), iHashTable.clone())?;
                    varIdx = listHead(varInfo.clone())? + List::second(varInfo.clone())?;
                    oVarList = metamodelica::cons(varIdx.clone(), iVarList.clone());
                    Ok(((iExp.clone(), (iHashTable.clone(), oVarList.clone())), oVarList.clone(), varHead.clone(), varIdx.clone(), varInfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { oVarList = __wb0; varHead = __wb1; varIdx = __wb2; varInfo = __wb3; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (iExp @ Deref @ DAE::Exp::CREF { componentRef, .. }, (iHashTable, iVarList)) => {
                    let mut oVarList: Arc<metamodelica::List<i32>> = oVarList.clone();
                    let mut varIdx: i32 = varIdx.clone();
                    let mut varInfo: Arc<metamodelica::List<i32>> = varInfo.clone();
                    varInfo = BaseHashTable::get(componentRef.clone(), iHashTable.clone())?;
                    varIdx = listHead(varInfo.clone())? + List::second(varInfo.clone())?;
                    oVarList = metamodelica::cons(varIdx.clone(), iVarList.clone());
                    Ok(((iExp.clone(), (iHashTable.clone(), oVarList.clone())), oVarList.clone(), varIdx.clone(), varInfo.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { oVarList = __wb0; varIdx = __wb1; varInfo = __wb2; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, oTpl))
}

fn getSimCodeVarNodeMapping(mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut iNumScVars: i32, mut iCompNodeMapping: metamodelica::Array<i32>, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))) -> Result<metamodelica::Array<i32>> {
    let mut oScVarTaskMapping: metamodelica::Array<i32> = Default::default();
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)> = Default::default();
    let mut scVarTaskMapping: metamodelica::Array<i32> = Default::default();
    scVarTaskMapping = arrayCreate(iNumScVars.clone(), -1);
    let HpcOmTaskGraph::TASKGRAPHMETA { varCompMapping: __pa0, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    varCompMapping = __pa0.clone();
    (oScVarTaskMapping, _) = Array::fold(varCompMapping.clone(), (std::sync::Arc::new({ let __pe_b1 = iEqSystems.clone(); let __pe_b2 = iVarNameSCVarIdxMapping.clone(); let __pe_b3 = iCompNodeMapping.clone(); move |__pe_a0, __pe_a4| getSimCodeVarNodeMapping0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32, i32), (metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, i32)> + 'static>), (scVarTaskMapping.clone(), 1))?;
    Ok(oScVarTaskMapping)
}

fn getSimCodeVarNodeMapping0(mut iCompIdx: (i32, i32, i32), mut iEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), mut iCompNodeMapping: metamodelica::Array<i32>, mut iScVarTaskMappingVarIdx: (metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<i32>, i32)> {
    let mut oScVarTaskMappingVarIdx: (metamodelica::Array<i32>, i32) = (Default::default(), 0);
    let mut iScVarTaskMapping: metamodelica::Array<i32> = Default::default();
    let mut varIdx: i32 = 0;
    let mut eqSysIdx: i32 = 0;
    let mut varOffset: i32 = 0;
    let mut scVarIdx: i32 = 0;
    let mut compIdx: i32 = 0;
    let mut nodeIdx: i32 = 0;
    let mut scVarOffset: i32 = 0;
    let mut eqSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut orderedVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut varName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut scVarValues: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut varNameString: ArcStr = arcstr::literal!("");
    oScVarTaskMappingVarIdx = 'mc: {
        let __mc_input = (iCompIdx.clone(), iScVarTaskMappingVarIdx.clone());
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8)) = (|| -> Result<_> {
            let ((mut compIdx, mut eqSysIdx, mut varOffset), (mut iScVarTaskMapping, mut varIdx)) = __mc_input.clone() else { bail!("nomatch") };
            let mut eqSystem: Arc<BackendDAE::EqSystem> = eqSystem.clone();
            let mut nodeIdx: i32 = nodeIdx.clone();
            let mut orderedVars: BackendDAE::Variables = orderedVars.clone();
            let mut scVarIdx: i32 = scVarIdx.clone();
            let mut scVarOffset: i32 = scVarOffset.clone();
            let mut scVarValues: Arc<metamodelica::List<i32>> = scVarValues.clone();
            let mut var: BackendDAE::Var = var.clone();
            let mut varName: Arc<DAE::ComponentRef> = varName.clone();
            let mut varNameString: ArcStr = varNameString.clone();
            let true = (intGt(compIdx.clone(), 0)) else { bail!("pattern mismatch") };
            eqSystem = (iEqSystems.clone()).get(eqSysIdx.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(eqSystem.clone()) {
                Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            orderedVars = __pa0.clone();
            var = BackendVariable::getVarAt(orderedVars.clone(), varIdx.clone() - varOffset.clone())?;
            let BackendDAE::VAR { varName: __pa1, .. } = (var.clone()) else { bail!("pattern mismatch") };
            varName = __pa1.clone();
            varName = getModifiedVarName(var.clone())?;
            scVarValues = BaseHashTable::get(varName.clone(), iVarNameSCVarIdxMapping.clone())?;
            varNameString = (ComponentReferenceBasics::printComponentRefStr(varName.clone())?).clone();
            scVarIdx = listHead(scVarValues.clone())?;
            scVarOffset = List::second(scVarValues.clone())?;
            scVarIdx = scVarIdx.clone() + scVarOffset.clone();
            nodeIdx = metamodelica::arrayGet(iCompNodeMapping.clone(), compIdx.clone())?;
            iScVarTaskMapping = metamodelica::arrayUpdate(iScVarTaskMapping.clone(), scVarIdx.clone(), nodeIdx.clone())?;
            Ok(((iScVarTaskMapping.clone(), varIdx.clone() + 1), eqSystem.clone(), nodeIdx.clone(), orderedVars.clone(), scVarIdx.clone(), scVarOffset.clone(), scVarValues.clone(), var.clone(), varName.clone(), varNameString.clone()))
        })() { eqSystem = __wb0; nodeIdx = __wb1; orderedVars = __wb2; scVarIdx = __wb3; scVarOffset = __wb4; scVarValues = __wb5; var = __wb6; varName = __wb7; varNameString = __wb8; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, (mut iScVarTaskMapping, mut varIdx)) = __mc_input.clone() else { bail!("nomatch") };
            Ok((iScVarTaskMapping.clone(), varIdx.clone() + 1))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oScVarTaskMappingVarIdx)
}

fn invertEqCompMapping(mut iEqCompMapping: metamodelica::Array<(i32, i32, i32)>, mut iNumOfComps: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<(i32, i32, i32)>>>> {
    let mut oCompEqMapping: metamodelica::Array<Arc<metamodelica::List<(i32, i32, i32)>>> = Default::default();
    let mut tmpCompEqMapping: metamodelica::Array<Arc<metamodelica::List<(i32, i32, i32)>>> = Default::default();
    let mut eqIdx: i32 = 0;
    let mut compIdx: i32 = 0;
    let mut eqSystemIdx: i32 = 0;
    let mut offset: i32 = 0;
    let mut compEqEntry: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    tmpCompEqMapping = arrayCreate(iNumOfComps.clone(), metamodelica::nil());
    for mut eqIdx in 1..=metamodelica::arrayLength(iEqCompMapping.clone()) {
        (compIdx, eqSystemIdx, offset) = metamodelica::arrayGet(iEqCompMapping.clone(), eqIdx.clone())?;
        compEqEntry = metamodelica::arrayGet(tmpCompEqMapping.clone(), compIdx.clone())?;
        tmpCompEqMapping = metamodelica::arrayUpdate(tmpCompEqMapping.clone(), compIdx.clone(), metamodelica::cons((eqIdx.clone(), eqSystemIdx.clone(), offset.clone()), compEqEntry.clone()))?;
    }
    oCompEqMapping = tmpCompEqMapping.clone();
    Ok(oCompEqMapping)
}

fn invertSccNodeMapping(mut iSccNodeMapping: metamodelica::Array<i32>, mut iNumberOfNodes: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut oNodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut tmpNodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut sccIdx: i32 = 0;
    let mut nodeIdx: i32 = 0;
    let mut nodeSccEntry: Arc<metamodelica::List<i32>> = metamodelica::nil();
    tmpNodeSccMapping = arrayCreate(iNumberOfNodes.clone(), metamodelica::nil());
    for mut sccIdx in 1..=metamodelica::arrayLength(iSccNodeMapping.clone()) {
        nodeIdx = metamodelica::arrayGet(iSccNodeMapping.clone(), sccIdx.clone())?;
        if intGt(nodeIdx.clone(), 0) {
            nodeSccEntry = metamodelica::arrayGet(tmpNodeSccMapping.clone(), nodeIdx.clone())?;
            tmpNodeSccMapping = metamodelica::arrayUpdate(tmpNodeSccMapping.clone(), nodeIdx.clone(), metamodelica::cons(sccIdx.clone(), nodeSccEntry.clone()))?;
        }
    }
    oNodeSccMapping = tmpNodeSccMapping.clone();
    Ok(oNodeSccMapping)
}

fn flattenEqSimCodeVarMapping(mut iEqSimCodeVarMapping: metamodelica::Array<metamodelica::Array<Arc<metamodelica::List<i32>>>>) -> Result<metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>> {
    let mut oFlatEqSimCodeVarMapping: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)> = Default::default();
    let mut simCodeVarList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut tmpFlatEqSimCodeVarMapping: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)> = Default::default();
    let mut eqCount: i32 = 0;
    let mut eqIdx: i32 = 0;
    let mut eqSysIdx: i32 = 0;
    let mut eqSimCodeVarIdx: i32 = 0;
    let mut eqSimCodeVarMappingEntry: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    eqCount = 0;
    for mut eqSysIdx in 1..=metamodelica::arrayLength(iEqSimCodeVarMapping.clone()) {
        eqSimCodeVarMappingEntry = metamodelica::arrayGet(iEqSimCodeVarMapping.clone(), eqSysIdx.clone())?;
        eqCount = eqCount.clone() + metamodelica::arrayLength(eqSimCodeVarMappingEntry.clone());
    }
    eqIdx = 1;
    tmpFlatEqSimCodeVarMapping = arrayCreate(eqCount.clone(), (-1, metamodelica::nil()));
    for mut eqSysIdx in 1..=metamodelica::arrayLength(iEqSimCodeVarMapping.clone()) {
        eqSimCodeVarMappingEntry = metamodelica::arrayGet(iEqSimCodeVarMapping.clone(), eqSysIdx.clone())?;
        for mut eqSimCodeVarIdx in 1..=metamodelica::arrayLength(eqSimCodeVarMappingEntry.clone()) {
            simCodeVarList = metamodelica::arrayGet(eqSimCodeVarMappingEntry.clone(), eqSimCodeVarIdx.clone())?;
            tmpFlatEqSimCodeVarMapping = metamodelica::arrayUpdate(tmpFlatEqSimCodeVarMapping.clone(), eqIdx.clone(), (eqSysIdx.clone(), simCodeVarList.clone()))?;
            eqIdx = eqIdx.clone() + 1;
        }
    }
    oFlatEqSimCodeVarMapping = tmpFlatEqSimCodeVarMapping.clone();
    Ok(oFlatEqSimCodeVarMapping)
}

fn getModifiedVarName(mut iVar: BackendDAE::Var) -> Result<Arc<DAE::ComponentRef>> {
    let mut oVarName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut iVarName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut tmpVarName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut varKind: BackendDAE::VarKind = BackendDAE::VarKind::ALG_STATE;
    oVarName = (match iVar.clone() {
        BackendDAE::Var { varName: ref __esc_iVarName, varKind: BackendDAE::VarKind::STATE { index: 1, .. }, .. } => {
            iVarName = __esc_iVarName.clone();
            tmpVarName = Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (arcstr::literal!(DAE::derivativeNamePrefix)).clone(), identType: Arc::new(DAE::Type::T_REAL { varLst: metamodelica::nil() }), subscriptLst: metamodelica::nil(), componentRef: iVarName.clone() });
            tmpVarName.clone()
        },
        BackendDAE::Var { varName: ref __esc_iVarName, varKind: mut __esc_varKind, .. } => {
            iVarName = __esc_iVarName.clone();
            varKind = __esc_varKind.clone();
            tmpVarName = iVarName.clone();
            tmpVarName.clone()
        },
    });
    Ok(oVarName)
}

fn getCacheLineTaskMapping(mut iTaskGraphMeta: HpcOmTaskGraph::TaskGraphMeta, mut iEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), mut iNumCacheLines: i32, mut iSCVarCLMapping: metamodelica::Array<(i32, i32)>) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>)> {
    let mut oCLTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut oScVarTaskMapping: metamodelica::Array<i32> = Default::default();
    let mut varCompMapping: metamodelica::Array<(i32, i32, i32)> = Default::default();
    let mut tmpCLTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut scVarTaskMapping: metamodelica::Array<i32> = Default::default();
    tmpCLTaskMapping = arrayCreate(iNumCacheLines.clone(), metamodelica::nil());
    scVarTaskMapping = arrayCreate(metamodelica::arrayLength(iSCVarCLMapping.clone()), -1);
    let HpcOmTaskGraph::TASKGRAPHMETA { varCompMapping: __pa0, .. } = (iTaskGraphMeta.clone()) else { bail!("pattern mismatch") };
    varCompMapping = __pa0.clone();
    (tmpCLTaskMapping, oScVarTaskMapping, _) = Array::fold(varCompMapping.clone(), (std::sync::Arc::new({ let __pe_b1 = iEqSystems.clone(); let __pe_b2 = iVarNameSCVarIdxMapping.clone(); let __pe_b3 = iSCVarCLMapping.clone(); move |__pe_a0, __pe_a4| getCacheLineTaskMapping0(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }) as std::sync::Arc<dyn ::std::ops::Fn((i32, i32, i32), (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)> + 'static>), (tmpCLTaskMapping.clone(), scVarTaskMapping.clone(), 1))?;
    tmpCLTaskMapping = Array::map1(tmpCLTaskMapping.clone(), (std::sync::Arc::new(List::sort) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    oCLTaskMapping = Array::map1(tmpCLTaskMapping.clone(), (std::sync::Arc::new(List::sortedUnique) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
    Ok((oCLTaskMapping, oScVarTaskMapping))
}

fn getCacheLineTaskMapping0(mut iNodeIdx: (i32, i32, i32), mut iEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), mut iSCVarCLMapping: metamodelica::Array<(i32, i32)>, mut iCLTaskMappingVarIdx: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32)> {
    let mut oCLTaskMappingVarIdx: (metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<i32>, i32) = (Default::default(), Default::default(), 0);
    let mut iClTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut iScVarTaskMapping: metamodelica::Array<i32> = Default::default();
    let mut varIdx: i32 = 0;
    let mut eqSysIdx: i32 = 0;
    let mut varOffset: i32 = 0;
    let mut scVarIdx: i32 = 0;
    let mut clIdx: i32 = 0;
    let mut nodeIdx: i32 = 0;
    let mut scVarOffset: i32 = 0;
    let mut eqSystem: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut orderedVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut var: BackendDAE::Var = <BackendDAE::Var as ::std::default::Default>::default();
    let mut varName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut oldVal: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut scVarValues: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oCLTaskMappingVarIdx = 'mc: {
        let __mc_input = (iNodeIdx.clone(), iCLTaskMappingVarIdx.clone());
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7, __wb8)) = (|| -> Result<_> {
            let ((mut nodeIdx, mut eqSysIdx, mut varOffset), (mut iClTaskMapping, mut iScVarTaskMapping, mut varIdx)) = __mc_input.clone() else { bail!("nomatch") };
            let mut clIdx: i32 = clIdx.clone();
            let mut eqSystem: Arc<BackendDAE::EqSystem> = eqSystem.clone();
            let mut oldVal: Arc<metamodelica::List<i32>> = oldVal.clone();
            let mut orderedVars: BackendDAE::Variables = orderedVars.clone();
            let mut scVarIdx: i32 = scVarIdx.clone();
            let mut scVarOffset: i32 = scVarOffset.clone();
            let mut scVarValues: Arc<metamodelica::List<i32>> = scVarValues.clone();
            let mut var: BackendDAE::Var = var.clone();
            let mut varName: Arc<DAE::ComponentRef> = varName.clone();
            let true = (intGt(nodeIdx.clone(), 0)) else { bail!("pattern mismatch") };
            eqSystem = (iEqSystems.clone()).get(eqSysIdx.clone())?;
            let __pa0 = ::match_deref::match_deref! { match &(eqSystem.clone()) {
                Deref @ BackendDAE::EqSystem { orderedVars: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            orderedVars = __pa0.clone();
            var = BackendVariable::getVarAt(orderedVars.clone(), varIdx.clone() - varOffset.clone())?;
            let BackendDAE::VAR { varName: __pa1, .. } = (var.clone()) else { bail!("pattern mismatch") };
            varName = __pa1.clone();
            varName = getModifiedVarName(var.clone())?;
            scVarValues = BaseHashTable::get(varName.clone(), iVarNameSCVarIdxMapping.clone())?;
            scVarIdx = listHead(scVarValues.clone())?;
            scVarOffset = List::second(scVarValues.clone())?;
            scVarIdx = scVarIdx.clone() + scVarOffset.clone();
            (clIdx, _) = metamodelica::arrayGet(iSCVarCLMapping.clone(), scVarIdx.clone())?;
            oldVal = metamodelica::arrayGet(iClTaskMapping.clone(), clIdx.clone())?;
            iClTaskMapping = metamodelica::arrayUpdate(iClTaskMapping.clone(), clIdx.clone(), metamodelica::cons(nodeIdx.clone(), oldVal.clone()))?;
            iScVarTaskMapping = metamodelica::arrayUpdate(iScVarTaskMapping.clone(), scVarIdx.clone(), nodeIdx.clone())?;
            Ok(((iClTaskMapping.clone(), iScVarTaskMapping.clone(), varIdx.clone() + 1), clIdx.clone(), eqSystem.clone(), oldVal.clone(), orderedVars.clone(), scVarIdx.clone(), scVarOffset.clone(), scVarValues.clone(), var.clone(), varName.clone()))
        })() { clIdx = __wb0; eqSystem = __wb1; oldVal = __wb2; orderedVars = __wb3; scVarIdx = __wb4; scVarOffset = __wb5; scVarValues = __wb6; var = __wb7; varName = __wb8; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, (mut iClTaskMapping, mut iScVarTaskMapping, mut varIdx)) = __mc_input.clone() else { bail!("nomatch") };
            Ok((iClTaskMapping.clone(), iScVarTaskMapping.clone(), varIdx.clone() + 1))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oCLTaskMappingVarIdx)
}

fn getTaskSimVarMapping(mut iSccEqMapping: metamodelica::Array<Arc<metamodelica::List<(i32, i32, i32)>>>, mut iNodeSccMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iEqSimCodeVarMapping: metamodelica::Array<(i32, Arc<metamodelica::List<i32>>)>, mut iScVarTaskMapping: metamodelica::Array<i32>, mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>) -> Result<(metamodelica::Array<Arc<metamodelica::List<i32>>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)> {
    let mut oSolvedVars: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut oNotSolvedVars: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut tmpSolvedVars: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut tmpNotSolvedVars: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut scVarMarks: metamodelica::Array<i32> = Default::default();
    let mut scSolvedVarMarks: metamodelica::Array<i32> = Default::default();
    let mut nodeSccs: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut eqVars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut nodeIdx: i32 = 0;
    let mut sccIdx: i32 = 0;
    let mut eqIdx: i32 = 0;
    let mut var: i32 = 0;
    let mut varTask: i32 = 0;
    let mut varMark: i32 = 0;
    let mut varType: i32 = 0;
    let mut nvar: i32 = 0;
    let mut var: i32 = 0;
    let mut sccEqs: Arc<metamodelica::List<(i32, i32, i32)>> = metamodelica::nil();
    let mut sccEq: (i32, i32, i32) = (0, 0, 0);
    match '__try0: {
        tmpSolvedVars = arrayCreate(metamodelica::arrayLength(iNodeSccMapping.clone()), metamodelica::nil());
        tmpNotSolvedVars = arrayCreate(metamodelica::arrayLength(iNodeSccMapping.clone()), metamodelica::nil());
        scVarMarks = arrayCreate(metamodelica::arrayLength(iScVarTaskMapping.clone()), -1);
        scSolvedVarMarks = arrayCreate(metamodelica::arrayLength(iScVarTaskMapping.clone()), -1);
        nvar = metamodelica::arrayLength(iScVarTaskMapping.clone());
        for mut nodeIdx in 1..=metamodelica::arrayLength(iNodeSccMapping.clone()) {
            nodeSccs = unwrap_break_err!(metamodelica::arrayGet(iNodeSccMapping.clone(), nodeIdx.clone()), '__try0);
            for mut sccIdx in &*nodeSccs.clone() {
                let mut sccIdx = sccIdx.clone();
                sccEqs = unwrap_break_err!(metamodelica::arrayGet(iSccEqMapping.clone(), sccIdx.clone()), '__try0);
                for mut sccEq in &*sccEqs.clone() {
                    let mut sccEq = sccEq.clone();
                    (eqIdx, _, _) = sccEq.clone();
                    (_, eqVars) = unwrap_break_err!(metamodelica::arrayGet(iEqSimCodeVarMapping.clone(), eqIdx.clone()), '__try0);
                    for mut v2 in &*eqVars.clone() {
                        let mut v2 = v2.clone();
                        var = if (v2.clone() > nvar.clone()) {v2.clone() - nvar.clone()} else {v2.clone()};
                        varTask = unwrap_break_err!(metamodelica::arrayGet(iScVarTaskMapping.clone(), var.clone()), '__try0);
                        varType = Util::tuple31(unwrap_break_err!(metamodelica::arrayGet(iSimCodeVarTypes.clone(), var.clone()), '__try0));
                        if intGt(varType.clone(), 0) {
                            if intEq(nodeIdx.clone(), varTask.clone()) {
                                varMark = unwrap_break_err!(metamodelica::arrayGet(scSolvedVarMarks.clone(), var.clone()), '__try0);
                                if intNe(varMark.clone(), nodeIdx.clone()) {
                                    tmpSolvedVars = unwrap_break_err!(metamodelica::arrayUpdate(tmpSolvedVars.clone(), nodeIdx.clone(), metamodelica::cons(var.clone(), unwrap_break_err!(metamodelica::arrayGet(tmpSolvedVars.clone(), nodeIdx.clone()), '__try0))), '__try0);
                                    scSolvedVarMarks = unwrap_break_err!(metamodelica::arrayUpdate(scSolvedVarMarks.clone(), var.clone(), nodeIdx.clone()), '__try0);
                                }
                            } else {
                                varMark = unwrap_break_err!(metamodelica::arrayGet(scVarMarks.clone(), var.clone()), '__try0);
                                if intNe(varMark.clone(), nodeIdx.clone()) {
                                    tmpNotSolvedVars = unwrap_break_err!(metamodelica::arrayUpdate(tmpNotSolvedVars.clone(), nodeIdx.clone(), metamodelica::cons(var.clone(), unwrap_break_err!(metamodelica::arrayGet(tmpNotSolvedVars.clone(), nodeIdx.clone()), '__try0))), '__try0);
                                    scVarMarks = unwrap_break_err!(metamodelica::arrayUpdate(scVarMarks.clone(), var.clone(), nodeIdx.clone()), '__try0);
                                }
                            }
                        }
                    }
                }
            }
        }
        oSolvedVars = tmpSolvedVars.clone();
        oNotSolvedVars = tmpNotSolvedVars.clone();
        Ok::<_, anyhow::Error>((nvar.clone(), oNotSolvedVars.clone(), oSolvedVars.clone(), scSolvedVarMarks.clone(), scVarMarks.clone(), tmpNotSolvedVars.clone(), tmpSolvedVars.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6)) => {
            nvar = __try0_o0;
            oNotSolvedVars = __try0_o1;
            oSolvedVars = __try0_o2;
            scSolvedVarMarks = __try0_o3;
            scVarMarks = __try0_o4;
            tmpNotSolvedVars = __try0_o5;
            tmpSolvedVars = __try0_o6;
        }
        Err(__try0_err) => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HpcOmMemory.getTaskSimVarMapping")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/HpcOmMemory.mo"))?;
            return Err(__try0_err);
        }
    }
    Ok((oSolvedVars, oNotSolvedVars))
}

// -------------------------------------------
// GRAPH
// -------------------------------------------
fn appendCacheLinesToGraph(mut iCacheMap: CacheMap, mut iNumberOfNodes: i32, mut iEqSimCodeVarMapping: metamodelica::Array<metamodelica::Array<Arc<metamodelica::List<i32>>>>, mut iEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), mut ieqCompMapping: metamodelica::Array<(i32, i32, i32)>, mut iScVarTaskMapping: metamodelica::Array<i32>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iThreadIdAttributeIdx: i32, mut iCompNodeMapping: metamodelica::Array<i32>, mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iGraphInfo: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo = <GraphML::GraphInfo as ::std::default::Default>::default();
    let mut clGroupNodeIdx: i32 = 0;
    let mut graphCount: i32 = 0;
    let mut tmpGraphInfo: GraphML::GraphInfo = <GraphML::GraphInfo as ::std::default::Default>::default();
    let mut knownEdges: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut addedVariables: metamodelica::Array<bool> = Default::default();
    let mut cacheVariables: metamodelica::Array<SimCodeVar::SimVar> = Default::default();
    let mut cacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    oGraphInfo = 'mc: {
        let __mc_input = iGraphInfo.clone();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
            let GraphML::GraphInfo::GRAPHINFO { graphCount: mut graphCount, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut addedVariables: metamodelica::Array<bool> = addedVariables.clone();
            let mut cacheLines: Arc<metamodelica::List<CacheLineMap>> = cacheLines.clone();
            let mut cacheVariables: metamodelica::Array<SimCodeVar::SimVar> = cacheVariables.clone();
            let mut clGroupNodeIdx: i32 = clGroupNodeIdx.clone();
            let mut knownEdges: metamodelica::Array<Arc<metamodelica::List<i32>>> = knownEdges.clone();
            let mut tmpGraphInfo: GraphML::GraphInfo = tmpGraphInfo.clone();
            let true = (intLe(1, graphCount.clone())) else { bail!("pattern mismatch") };
            knownEdges = arrayCreate(iNumberOfNodes.clone(), metamodelica::nil());
            addedVariables = arrayCreate(metamodelica::arrayLength(iScVarTaskMapping.clone()), false);
            let (__pa0, _, (_, __pa1)) = GraphML::addGroupNode((literal!("CL_GoupNode")).clone(), 1, false, (literal!("CL")).clone(), iGraphInfo.clone())?;
            tmpGraphInfo = __pa0.clone();
            clGroupNodeIdx = __pa1.clone();
            cacheLines = getAllCacheLinesOfCacheMap(iCacheMap.clone())?;
            cacheVariables = metamodelica::arrayFromVec(getCacheVariablesOfCacheMap(iCacheMap.clone())?.into_iter().cloned().collect());
            tmpGraphInfo = List::fold(cacheLines.clone(), (std::sync::Arc::new({ let __pe_b1 = cacheVariables.clone(); let __pe_b2 = addedVariables.clone(); let __pe_b3 = iSchedulerInfo.clone(); let __pe_b4 = (clGroupNodeIdx.clone(), iThreadIdAttributeIdx.clone()); let __pe_b5 = iScVarTaskMapping.clone(); let __pe_b6 = iVarNameSCVarIdxMapping.clone(); let __pe_b7 = iScVarInfos.clone(); move |__pe_a0, __pe_a8| appendCacheLineMapToGraph(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_a8) }) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineMap, GraphML::GraphInfo) -> Result<GraphML::GraphInfo> + 'static>), tmpGraphInfo.clone())?;
            tmpGraphInfo = appendTaskVarEdgesToGraph(iTaskSolvedVarsMapping.clone(), iTaskUnsolvedVarsMapping.clone(), tmpGraphInfo.clone())?;
            Ok((tmpGraphInfo.clone(), addedVariables.clone(), cacheLines.clone(), cacheVariables.clone(), clGroupNodeIdx.clone(), knownEdges.clone(), tmpGraphInfo.clone()))
        })() { addedVariables = __wb0; cacheLines = __wb1; cacheVariables = __wb2; clGroupNodeIdx = __wb3; knownEdges = __wb4; tmpGraphInfo = __wb5; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let GraphML::GraphInfo::GRAPHINFO { graphCount: mut graphCount, .. } = __mc_input.clone() else { bail!("nomatch") };
            let true = (intEq(graphCount.clone(), 0)) else { bail!("pattern mismatch") };
            Ok(iGraphInfo.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print((literal!("HpcOmSimCode.appendCacheLinesToGraph failed!\n")).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oGraphInfo)
}

fn appendVariablesToGraph(mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iNumberOfScVars: i32, mut iGraphIdx: i32, mut iThreadIdAttributeIdx: i32, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), mut iAllVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>, mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iGraphInfo: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo = <GraphML::GraphInfo as ::std::default::Default>::default();
    let mut tmpGraphInfo: GraphML::GraphInfo = iGraphInfo.clone();
    let mut description: ArcStr = arcstr::literal!("");
    let mut threadText: ArcStr = arcstr::literal!("");
    let mut simVarOpt: Option<SimCodeVar::SimVar> = None;
    let mut simVar: SimCodeVar::SimVar = <SimCodeVar::SimVar as ::std::default::Default>::default();
    let mut varCompRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut nodeLabel: GraphML::NodeLabel = <GraphML::NodeLabel as ::std::default::Default>::default();
    let mut isValidVar: bool = false;
    let mut realScVarIdxOffset: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut realScVarIdx: i32 = 0;
    let mut realScVarOffset: i32 = 0;
    let mut threadOwner: i32 = 0;
    for mut varIdx in 1..=iNumberOfScVars.clone() {
        isValidVar = true;
        simVarOpt = metamodelica::arrayGet(iAllVarsMapping.clone(), varIdx.clone())?;
        description = (literal!("unknown")).clone();
        threadText = (literal!("Th -1")).clone();
        if isSome(simVarOpt.clone()) {
            simVar = Util::getOption(simVarOpt.clone())?;
            varCompRef = simVar.name.clone();
            description = (ComponentReferenceBasics::printComponentRefStr(varCompRef.clone())?).clone();
            isValidVar = BaseHashTable::hasKey(varCompRef.clone(), iVarNameSCVarIdxMapping.clone())?;
            if BaseHashTable::hasKey(varCompRef.clone(), iVarNameSCVarIdxMapping.clone())? {
                realScVarIdxOffset = BaseHashTable::get(varCompRef.clone(), iVarNameSCVarIdxMapping.clone())?;
                realScVarIdx = (realScVarIdxOffset.clone()).get(1)?;
                realScVarOffset = (realScVarIdxOffset.clone()).get(2)?;
                realScVarIdx = realScVarIdx.clone() + realScVarOffset.clone();
                let ScVarInfo { ownerThread: __pa0, .. } = (metamodelica::arrayGet(iScVarInfos.clone(), realScVarIdx.clone())?) else { bail!("pattern mismatch") };
                threadOwner = __pa0.clone();
                threadText = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Th ")); __mm_s.push_str(&*intString(threadOwner.clone())); ArcStr::from(__mm_s) }).clone();
            }
        }
        if isValidVar.clone() {
            nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (intString(varIdx.clone())).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (tmpGraphInfo, _) = GraphML::addNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("var")); __mm_s.push_str(&*intString(varIdx.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_GREEN2)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![nodeLabel.clone()], openmodelica_susan::GraphML::ShapeType::ELLIPSE, Some((description.clone()).clone()), list![(iThreadIdAttributeIdx.clone(), threadText.clone())], iGraphIdx.clone(), tmpGraphInfo.clone())?;
        }
    }
    tmpGraphInfo = appendTaskVarEdgesToGraph(iTaskSolvedVarsMapping.clone(), iTaskUnsolvedVarsMapping.clone(), tmpGraphInfo.clone())?;
    oGraphInfo = tmpGraphInfo.clone();
    Ok(oGraphInfo)
}

fn appendTaskVarEdgesToGraph(mut iTaskSolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iTaskUnsolvedVarsMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut iGraphInfo: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo = <GraphML::GraphInfo as ::std::default::Default>::default();
    let mut tmpGraphInfo: GraphML::GraphInfo = iGraphInfo.clone();
    let mut taskIdx: i32 = 0;
    let mut varIdx: i32 = 0;
    let mut taskVarList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut taskIdx in 1..=metamodelica::arrayLength(iTaskSolvedVarsMapping.clone()) {
        taskVarList = metamodelica::arrayGet(iTaskSolvedVarsMapping.clone(), taskIdx.clone())?;
        for mut varIdx in &*taskVarList.clone() {
            let mut varIdx = varIdx.clone();
            (tmpGraphInfo, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("varEdge_")); __mm_s.push_str(&*intString(taskIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(varIdx.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("var")); __mm_s.push_str(&*intString(varIdx.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(taskIdx.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_BLACK)).clone(), openmodelica_susan::GraphML::LineType::LINE, GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_susan::GraphML::ArrowType::ARROWNONE, openmodelica_susan::GraphML::ArrowType::ARROWSTANDART), metamodelica::nil(), tmpGraphInfo.clone())?;
        }
    }
    for mut taskIdx in 1..=metamodelica::arrayLength(iTaskUnsolvedVarsMapping.clone()) {
        taskVarList = metamodelica::arrayGet(iTaskUnsolvedVarsMapping.clone(), taskIdx.clone())?;
        for mut varIdx in &*taskVarList.clone() {
            let mut varIdx = varIdx.clone();
            (tmpGraphInfo, _) = GraphML::addEdge(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("varEdge_")); __mm_s.push_str(&*intString(taskIdx.clone())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(varIdx.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node")); __mm_s.push_str(&*intString(taskIdx.clone())); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("var")); __mm_s.push_str(&*intString(varIdx.clone())); ArcStr::from(__mm_s) }).clone(), (arcstr::literal!(GraphML::COLOR_BLACK)).clone(), openmodelica_susan::GraphML::LineType::LINE, GraphML::LINEWIDTH_STANDARD.clone(), false, metamodelica::nil(), (openmodelica_susan::GraphML::ArrowType::ARROWNONE, openmodelica_susan::GraphML::ArrowType::ARROWSTANDART), metamodelica::nil(), tmpGraphInfo.clone())?;
        }
    }
    oGraphInfo = tmpGraphInfo.clone();
    Ok(oGraphInfo)
}

fn appendUnmappedVariablesToGraph(mut iScVarCLMapping: metamodelica::Array<(i32, i32)>, mut iGraphInfo: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo = <GraphML::GraphInfo as ::std::default::Default>::default();
    let mut tmpGraphInfo: GraphML::GraphInfo = iGraphInfo.clone();
    let mut scVarIdx: i32 = 0;
    let mut clIdx: i32 = 0;
    for mut scVarIdx in 1..=metamodelica::arrayLength(iScVarCLMapping.clone()) {
        (clIdx, _) = metamodelica::arrayGet(iScVarCLMapping.clone(), scVarIdx.clone())?;
        if intLt(clIdx.clone(), 1) {
        }
    }
    oGraphInfo = tmpGraphInfo.clone();
    Ok(oGraphInfo)
}

fn appendCacheLineMapToGraph(mut iCacheLineMap: CacheLineMap, mut iCacheVariables: metamodelica::Array<SimCodeVar::SimVar>, mut iAddedVariables: metamodelica::Array<bool>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iTopGraphAttThreadIdIdx: (i32, i32), mut iScVarTaskMapping: metamodelica::Array<i32>, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iGraphInfo: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo = <GraphML::GraphInfo as ::std::default::Default>::default();
    let mut idx: i32 = 0;
    let mut graphIdx: i32 = 0;
    let mut iTopGraphIdx: i32 = 0;
    let mut iAttThreadIdIdx: i32 = 0;
    let mut entries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut entry: CacheLineEntry = <CacheLineEntry as ::std::default::Default>::default();
    let mut tmpGraphInfo: GraphML::GraphInfo = <GraphML::GraphInfo as ::std::default::Default>::default();
    let mut entryThreadOwner: i32 = 0;
    let mut notOnlyParamters: bool = false;
    let CacheLineMap { idx: __pa0, entries: __pa1, .. } = (iCacheLineMap.clone()) else { bail!("pattern mismatch") };
    idx = __pa0.clone();
    entries = __pa1.clone();
    notOnlyParamters = false;
    for mut entry in &*entries.clone() {
        let mut entry = entry.clone();
        let CacheLineEntry { threadOwner: __pa2, .. } = (entry.clone()) else { bail!("pattern mismatch") };
        entryThreadOwner = __pa2.clone();
        notOnlyParamters = boolOr(notOnlyParamters.clone(), intNe(entryThreadOwner.clone(), -1));
    }
    if notOnlyParamters.clone() {
        (iTopGraphIdx, iAttThreadIdIdx) = iTopGraphAttThreadIdIdx.clone();
        let (__pa3, _, (_, __pa4)) = GraphML::addGroupNode(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CL_Meta_")); __mm_s.push_str(&*intString(idx.clone())); ArcStr::from(__mm_s) }).clone(), iTopGraphIdx.clone(), true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CL")); __mm_s.push_str(&*intString(idx.clone())); ArcStr::from(__mm_s) }).clone(), iGraphInfo.clone())?;
        tmpGraphInfo = __pa3.clone();
        graphIdx = __pa4.clone();
        oGraphInfo = List::fold(entries.clone(), (std::sync::Arc::new({ let __pe_b1 = iCacheVariables.clone(); let __pe_b2 = iAddedVariables.clone(); let __pe_b3 = iSchedulerInfo.clone(); let __pe_b4 = (graphIdx.clone(), iAttThreadIdIdx.clone()); let __pe_b5 = iScVarTaskMapping.clone(); let __pe_b6 = iVarNameSCVarIdxMapping.clone(); let __pe_b7 = iScVarInfos.clone(); move |__pe_a0, __pe_a8| appendCacheLineEntryToGraph(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_a8) }) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineEntry, GraphML::GraphInfo) -> Result<GraphML::GraphInfo> + 'static>), tmpGraphInfo.clone())?;
    } else {
        oGraphInfo = iGraphInfo.clone();
    }
    Ok(oGraphInfo)
}

fn appendCacheLineEntryToGraph(mut iCacheLineEntry: CacheLineEntry, mut iCacheVariables: metamodelica::Array<SimCodeVar::SimVar>, mut iAddedVariables: metamodelica::Array<bool>, mut iSchedulerInfo: metamodelica::Array<(i32, i32, metamodelica::Real)>, mut iTopGraphAttThreadIdIdx: (i32, i32), mut iScVarTaskMapping: metamodelica::Array<i32>, mut iVarNameSCVarIdxMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), mut iScVarInfos: metamodelica::Array<ScVarInfo>, mut iGraphInfo: GraphML::GraphInfo) -> Result<GraphML::GraphInfo> {
    let mut oGraphInfo: GraphML::GraphInfo = <GraphML::GraphInfo as ::std::default::Default>::default();
    let mut realScVarIdxOffset: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut scVarIdx: i32 = 0;
    let mut realScVarIdx: i32 = 0;
    let mut realScVarOffset: i32 = 0;
    let mut taskIdx: i32 = 0;
    let mut iTopGraphIdx: i32 = 0;
    let mut iAttThreadIdIdx: i32 = 0;
    let mut threadOwner: i32 = 0;
    let mut varString: ArcStr = arcstr::literal!("");
    let mut threadText: ArcStr = arcstr::literal!("");
    let mut nodeLabelText: ArcStr = arcstr::literal!("");
    let mut nodeId: ArcStr = arcstr::literal!("");
    let mut nodeLabel: GraphML::NodeLabel = <GraphML::NodeLabel as ::std::default::Default>::default();
    let mut iVar: SimCodeVar::SimVar = <SimCodeVar::SimVar as ::std::default::Default>::default();
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let CacheLineEntry { scVarIdx: __pa0, threadOwner: __pa1, .. } = (iCacheLineEntry.clone()) else { bail!("pattern mismatch") };
    scVarIdx = __pa0.clone();
    threadOwner = __pa1.clone();
    (iTopGraphIdx, iAttThreadIdIdx) = iTopGraphAttThreadIdIdx.clone();
    if intGe(metamodelica::arrayLength(iCacheVariables.clone()) - scVarIdx.clone() + 1, 1) {
        iVar = metamodelica::arrayGet(iCacheVariables.clone(), metamodelica::arrayLength(iCacheVariables.clone()) - scVarIdx.clone() + 1)?;
        let SimCodeVar::SIMVAR { name: __pa2, .. } = (iVar.clone()) else { bail!("pattern mismatch") };
        name = __pa2.clone();
        if BaseHashTable::hasKey(name.clone(), iVarNameSCVarIdxMapping.clone())? {
            realScVarIdxOffset = BaseHashTable::get(name.clone(), iVarNameSCVarIdxMapping.clone())?;
            realScVarIdx = (realScVarIdxOffset.clone()).get(1)?;
            realScVarOffset = (realScVarIdxOffset.clone()).get(2)?;
            realScVarIdx = realScVarIdx.clone() + realScVarOffset.clone();
            varString = (ComponentReferenceBasics::printComponentRefStr(name.clone())?).clone();
            taskIdx = metamodelica::arrayGet(iScVarTaskMapping.clone(), realScVarIdx.clone())?;
            let ScVarInfo { ownerThread: __pa3, .. } = (metamodelica::arrayGet(iScVarInfos.clone(), realScVarIdx.clone())?) else { bail!("pattern mismatch") };
            threadOwner = __pa3.clone();
            nodeId = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("var")); __mm_s.push_str(&*intString(realScVarIdx.clone())); ArcStr::from(__mm_s) }).clone();
            metamodelica::arrayUpdate(iAddedVariables.clone(), realScVarIdx.clone(), true)?;
            threadText = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Th ")); __mm_s.push_str(&*intString(threadOwner.clone())); ArcStr::from(__mm_s) }).clone();
            nodeLabelText = (intString(realScVarIdx.clone())).clone();
            nodeLabel = GraphML::NodeLabel::NODELABEL_INTERNAL { text: (nodeLabelText.clone()).clone(), backgroundColor: None, fontStyle: openmodelica_susan::GraphML::FontStyle::FONTPLAIN };
            (oGraphInfo, _) = GraphML::addNode((nodeId.clone()).clone(), (arcstr::literal!(GraphML::COLOR_GREEN2)).clone(), GraphML::BORDERWIDTH_STANDARD.clone(), list![nodeLabel.clone()], openmodelica_susan::GraphML::ShapeType::ELLIPSE, Some((varString.clone()).clone()), list![(iAttThreadIdIdx.clone(), threadText.clone())], iTopGraphIdx.clone(), iGraphInfo.clone())?;
        } else {
            oGraphInfo = iGraphInfo.clone();
        }
    } else {
        oGraphInfo = iGraphInfo.clone();
    }
    Ok(oGraphInfo)
}

// -------------------------------------------
// PRINT
// -------------------------------------------
fn printCacheMap(mut iCacheMap: CacheMap) -> Result<()> {
    let mut cacheLineSize: i32 = 0;
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let () = (match iCacheMap.clone() {
        CacheMap::CACHEMAP { cacheLineSize: mut __esc_cacheLineSize, cacheVariables: mut __esc_cacheVariables, cacheLinesFloat: mut __esc_cacheLinesFloat, cacheLinesInt: mut __esc_cacheLinesInt, cacheLinesBool: mut __esc_cacheLinesBool } => {
            cacheLineSize = __esc_cacheLineSize.clone();
            cacheVariables = __esc_cacheVariables.clone();
            cacheLinesFloat = __esc_cacheLinesFloat.clone();
            cacheLinesInt = __esc_cacheLinesInt.clone();
            cacheLinesBool = __esc_cacheLinesBool.clone();
            metamodelica::print((literal!("\n\nCacheMap\n---------------\n")).clone());
            metamodelica::print((literal!("  Variables\n")).clone());
            List::fold(cacheVariables.clone(), (std::sync::Arc::new(printCacheVariable) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar, i32) -> Result<i32> + 'static>), (cacheVariables.clone().len() as i32))?;
            metamodelica::print((literal!("  Float Cache Lines\n")).clone());
            List::map1_0(cacheLinesFloat.clone(), (std::sync::Arc::new(printCacheLineMap) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineMap, Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<()> + 'static>), cacheVariables.clone())?;
            metamodelica::print((literal!("  Int Cache Lines\n")).clone());
            List::map1_0(cacheLinesInt.clone(), (std::sync::Arc::new(printCacheLineMap) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineMap, Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<()> + 'static>), cacheVariables.clone())?;
            metamodelica::print((literal!("  Bool Cache Lines\n")).clone());
            List::map1_0(cacheLinesBool.clone(), (std::sync::Arc::new(printCacheLineMap) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineMap, Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<()> + 'static>), cacheVariables.clone())?;
            ()
        },
        CacheMap::UNIFORM_CACHEMAP { cacheLineSize: mut __esc_cacheLineSize, cacheVariables: mut __esc_cacheVariables, cacheLines: mut __esc_cacheLines } => {
            cacheLineSize = __esc_cacheLineSize.clone();
            cacheVariables = __esc_cacheVariables.clone();
            cacheLines = __esc_cacheLines.clone();
            metamodelica::print((literal!("\n\nUniform CacheMap\n---------------\n")).clone());
            metamodelica::print((literal!("  Variables.\n")).clone());
            List::map1_0(cacheLines.clone(), (std::sync::Arc::new(printCacheLineMap) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineMap, Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<()> + 'static>), cacheVariables.clone())?;
            ()
        },
        _ => {
            metamodelica::print((literal!("printCacheMap: Unsupported cache map type!\n")).clone());
            ()
        },
    });
    Ok(())
}

fn printCacheVariable(mut iCacheVariable: SimCodeVar::SimVar, mut iIdx: i32) -> Result<i32> {
    let mut oIdx: i32 = 0;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("    ")); __mm_s.push_str(&*intString(iIdx.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*dumpSimCodeVar(iCacheVariable.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    oIdx = iIdx.clone() - 1;
    Ok(oIdx)
}

fn printCacheLineMap(mut iCacheLineMap: CacheLineMap, mut iCacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<()> {
    let mut idx: i32 = 0;
    let mut entries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut iVarsString: ArcStr = arcstr::literal!("");
    let mut iBytesString: ArcStr = arcstr::literal!("");
    let CacheLineMap { idx: __pa0, entries: __pa1, .. } = (iCacheLineMap.clone()) else { bail!("pattern mismatch") };
    idx = __pa0.clone();
    entries = __pa1.clone();
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  CacheLineMap ")); __mm_s.push_str(&*intString(idx.clone())); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((entries.clone().len() as i32))); __mm_s.push_str(&*literal!(" entries)\n")); ArcStr::from(__mm_s) }).clone());
    (iVarsString, iBytesString) = List::fold1(entries.clone(), (std::sync::Arc::new(cacheLineEntryToString) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineEntry, Arc<metamodelica::List<SimCodeVar::SimVar>>, (ArcStr, ArcStr)) -> Result<(ArcStr, ArcStr)> + 'static>), iCacheVariables.clone(), (literal!(""), literal!("")))?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("    ")); __mm_s.push_str(&*iVarsString.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("    ")); __mm_s.push_str(&*iBytesString.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

fn printCacheLineMapClean(mut iCacheLineMap: CacheLineMap) -> Result<()> {
    let mut idx: i32 = 0;
    let mut entries: Arc<metamodelica::List<CacheLineEntry>> = metamodelica::nil();
    let mut iVarsString: ArcStr = arcstr::literal!("");
    let mut iBytesString: ArcStr = arcstr::literal!("");
    let CacheLineMap { idx: __pa0, entries: __pa1, .. } = (iCacheLineMap.clone()) else { bail!("pattern mismatch") };
    idx = __pa0.clone();
    entries = __pa1.clone();
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  CacheLineMap ")); __mm_s.push_str(&*intString(idx.clone())); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((entries.clone().len() as i32))); __mm_s.push_str(&*literal!(" entries)\n")); ArcStr::from(__mm_s) }).clone());
    (iVarsString, iBytesString) = List::fold(entries.clone(), (std::sync::Arc::new(cacheLineEntryToStringClean) as std::sync::Arc<dyn ::std::ops::Fn(CacheLineEntry, (ArcStr, ArcStr)) -> Result<(ArcStr, ArcStr)> + 'static>), (literal!(""), literal!("")))?;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("    ")); __mm_s.push_str(&*iVarsString.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("    ")); __mm_s.push_str(&*iBytesString.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

fn cacheLineEntryToString(mut iCacheLineEntry: CacheLineEntry, mut iCacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iString: (ArcStr, ArcStr)) -> Result<(ArcStr, ArcStr)> {
    let mut oString: (ArcStr, ArcStr) = (arcstr::literal!(""), arcstr::literal!(""));
    let mut start: i32 = 0;
    let mut dataType: i32 = 0;
    let mut size: i32 = 0;
    let mut scVarIdx: i32 = 0;
    let mut scVarStr: ArcStr = arcstr::literal!("");
    let mut iVar: SimCodeVar::SimVar = <SimCodeVar::SimVar as ::std::default::Default>::default();
    let mut iVarsString: ArcStr = arcstr::literal!("");
    let mut iBytesString: ArcStr = arcstr::literal!("");
    let mut iBytesStringNew: ArcStr = arcstr::literal!("");
    (iVarsString, iBytesString) = iString.clone();
    let CacheLineEntry { start: __pa0, dataType: __pa1, size: __pa2, scVarIdx: __pa3, .. } = (iCacheLineEntry.clone()) else { bail!("pattern mismatch") };
    start = __pa0.clone();
    dataType = __pa1.clone();
    size = __pa2.clone();
    scVarIdx = __pa3.clone();
    iVar = (iCacheVariables.clone()).get((iCacheVariables.clone().len() as i32) - scVarIdx.clone() + 1)?;
    scVarStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*dumpSimCodeVar(iVar.clone())?); __mm_s.push_str(&*literal!(" [")); __mm_s.push_str(&*intString(scVarIdx.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
    iVarsString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iVarsString.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*scVarStr.clone()); ArcStr::from(__mm_s) }).clone();
    if intGt(start.clone(), 0) {
        iVarsString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iVarsString.clone()); __mm_s.push_str(&*literal!(" | ")); ArcStr::from(__mm_s) }).clone();
        iBytesStringNew = (intString(start.clone())).clone();
    } else {
        iBytesStringNew = (literal!("")).clone();
    }
    iBytesStringNew = (Util::stringPadLeft((iBytesStringNew.clone()).clone(), 2 + ((scVarStr.clone()).clone().len() as i32) + ((iBytesStringNew.clone()).clone().len() as i32), (literal!(" ")).clone())).clone();
    iBytesString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iBytesString.clone()); __mm_s.push_str(&*iBytesStringNew.clone()); ArcStr::from(__mm_s) }).clone();
    oString = (iVarsString.clone(), iBytesString.clone());
    Ok(oString)
}

fn cacheLineEntryToStringClean(mut iCacheLineEntry: CacheLineEntry, mut iString: (ArcStr, ArcStr)) -> Result<(ArcStr, ArcStr)> {
    let mut oString: (ArcStr, ArcStr) = (arcstr::literal!(""), arcstr::literal!(""));
    let mut start: i32 = 0;
    let mut dataType: i32 = 0;
    let mut size: i32 = 0;
    let mut scVarIdx: i32 = 0;
    let mut scVarStr: ArcStr = arcstr::literal!("");
    let mut iVarsString: ArcStr = arcstr::literal!("");
    let mut iBytesString: ArcStr = arcstr::literal!("");
    let mut iBytesStringNew: ArcStr = arcstr::literal!("");
    (iVarsString, iBytesString) = iString.clone();
    let CacheLineEntry { start: __pa0, dataType: __pa1, size: __pa2, scVarIdx: __pa3, .. } = (iCacheLineEntry.clone()) else { bail!("pattern mismatch") };
    start = __pa0.clone();
    dataType = __pa1.clone();
    size = __pa2.clone();
    scVarIdx = __pa3.clone();
    scVarStr = (intString(scVarIdx.clone())).clone();
    iVarsString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iVarsString.clone()); __mm_s.push_str(&*literal!("| ")); __mm_s.push_str(&*scVarStr.clone()); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
    iBytesStringNew = (intString(start.clone())).clone();
    iBytesStringNew = (Util::stringPadRight((iBytesStringNew.clone()).clone(), 3 + ((scVarStr.clone()).clone().len() as i32), (literal!(" ")).clone())).clone();
    iBytesString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iBytesString.clone()); __mm_s.push_str(&*iBytesStringNew.clone()); ArcStr::from(__mm_s) }).clone();
    oString = (iVarsString.clone(), iBytesString.clone());
    Ok(oString)
}

fn dumpSimCodeVar(mut iVar: SimCodeVar::SimVar) -> Result<ArcStr> {
    let mut oString: ArcStr = arcstr::literal!("");
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let SimCodeVar::SIMVAR { name: __pa0, .. } = (iVar.clone()) else { bail!("pattern mismatch") };
    name = __pa0.clone();
    oString = (ComponentReferenceBasics::printComponentRefStr(name.clone())?).clone();
    Ok(oString)
}

fn printNodeSimCodeVarMapping(mut iMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    metamodelica::print((literal!("Node - SimCodeVar - Mapping\n------------------\n")).clone());
    Array::fold(iMapping.clone(), (std::sync::Arc::new(printNodeSimCodeVarMapping0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<i32> + 'static>), 1)?;
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

fn printNodeSimCodeVarMapping0(mut iMappingEntry: Arc<metamodelica::List<i32>>, mut iNodeIdx: i32) -> Result<i32> {
    let mut oNodeIdx: i32 = 0;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Node ")); __mm_s.push_str(&*intString(iNodeIdx.clone())); __mm_s.push_str(&*literal!(" uses sc-vars: ")); __mm_s.push_str(&*stringDelimitList(List::map(iMappingEntry.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    oNodeIdx = iNodeIdx.clone() + 1;
    Ok(oNodeIdx)
}

fn printScVarTaskMapping(mut iMapping: metamodelica::Array<i32>) -> Result<()> {
    metamodelica::print((literal!("----------------------\nSCVar - Task - Mapping\n----------------------\n")).clone());
    Array::fold(iMapping.clone(), (std::sync::Arc::new(fnptr!(printScVarTaskMapping0, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1)?;
    metamodelica::print((literal!("\n")).clone());
    Ok(())
}

fn printScVarTaskMapping0(mut iMappingEntry: i32, mut iScVarIdx: i32) -> i32 {
    let mut oScVarIdx: i32 = 0;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SCVar ")); __mm_s.push_str(&*intString(iScVarIdx.clone())); __mm_s.push_str(&*literal!(" is solved in task: ")); __mm_s.push_str(&*intString(iMappingEntry.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    oScVarIdx = iScVarIdx.clone() + 1;
    oScVarIdx
}

fn printCacheLineTaskMapping(mut iCacheLineTaskMapping: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<()> {
    Array::fold(iCacheLineTaskMapping.clone(), (std::sync::Arc::new(printCacheLineTaskMapping0) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, i32) -> Result<i32> + 'static>), 1)?;
    Ok(())
}

fn printCacheLineTaskMapping0(mut iTasks: Arc<metamodelica::List<i32>>, mut iCacheLineIdx: i32) -> Result<i32> {
    let mut oCacheLineIdx: i32 = 0;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Tasks that are writing to cacheline ")); __mm_s.push_str(&*intString(iCacheLineIdx.clone())); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*stringDelimitList(List::map(iTasks.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    oCacheLineIdx = iCacheLineIdx.clone() + 1;
    Ok(oCacheLineIdx)
}

fn printEqSimCodeVarMapping(mut iMapping: metamodelica::Array<metamodelica::Array<Arc<metamodelica::List<i32>>>>) -> Result<()> {
    let mut sysInformation: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut sysIdx: i32 = 0;
    let mut vars: Arc<metamodelica::List<i32>> = metamodelica::nil();
    for mut sysIdx in 1..=metamodelica::arrayLength(iMapping.clone()) {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("System ")); __mm_s.push_str(&*intString(sysIdx.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        sysInformation = metamodelica::arrayGet(iMapping.clone(), sysIdx.clone())?;
        for mut eqIdx in 1..=metamodelica::arrayLength(sysInformation.clone()) {
            vars = metamodelica::arrayGet(sysInformation.clone(), eqIdx.clone())?;
        }
    }
    Ok(())
}

fn printSccNodeMapping(mut iMapping: metamodelica::Array<i32>) -> Result<()> {
    metamodelica::print((literal!("--------------------\nScc - Node - Mapping\n--------------------\n")).clone());
    Array::fold(iMapping.clone(), (std::sync::Arc::new(fnptr!(printSccNodeMapping0, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1)?;
    Ok(())
}

fn printSccNodeMapping0(mut iMappingEntry: i32, mut iIdx: i32) -> i32 {
    let mut oIdx: i32 = 0;
    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Scc ")); __mm_s.push_str(&*intString(iIdx.clone())); __mm_s.push_str(&*literal!(" is solved by node ")); __mm_s.push_str(&*intString(iMappingEntry.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    oIdx = iIdx.clone() + 1;
    oIdx
}

fn printScVarInfos(mut iScVarInfos: metamodelica::Array<ScVarInfo>) -> Result<()> {
    let mut scVarIdx: i32 = 0;
    let mut ownerThread: i32 = 0;
    let mut isShared: bool = false;
    metamodelica::print((literal!("--------------------\nScVar - Infos\n--------------------\n")).clone());
    for mut scVarIdx in 1..=metamodelica::arrayLength(iScVarInfos.clone()) {
        let ScVarInfo { ownerThread: __pa0, isShared: __pa1 } = (metamodelica::arrayGet(iScVarInfos.clone(), scVarIdx.clone())?) else { bail!("pattern mismatch") };
        ownerThread = __pa0.clone();
        isShared = __pa1.clone();
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ScVar ")); __mm_s.push_str(&*intString(scVarIdx.clone())); __mm_s.push_str(&*literal!(" has thread owner ")); __mm_s.push_str(&*intString(ownerThread.clone())); __mm_s.push_str(&*literal!(" and shared state ")); __mm_s.push_str(&*boolString(isShared.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

fn dumpScVarsByIdx(mut iSimCodeVarIdx: i32, mut iAllSCVarsMapping: metamodelica::Array<Option<SimCodeVar::SimVar>>) -> Result<ArcStr> {
    let mut oString: ArcStr = arcstr::literal!("");
    let mut tmpString: ArcStr = arcstr::literal!("");
    let mut simVar: SimCodeVar::SimVar = <SimCodeVar::SimVar as ::std::default::Default>::default();
    oString = ('mc: {
        let __mc_input = iAllSCVarsMapping.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut simVar: SimCodeVar::SimVar = simVar.clone();
            let mut tmpString: ArcStr = tmpString.clone();
            let __pa0 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(iAllSCVarsMapping.clone(), iSimCodeVarIdx.clone())?) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            simVar = __pa0.clone();
            tmpString = (dumpSimCodeVar(simVar.clone())?).clone();
            Ok((tmpString.clone(), simVar.clone(), tmpString.clone()))
        })() { simVar = __wb0; tmpString = __wb1; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("dumpScVarsByIdx: Failed to find simcode-variable with index ")); __mm_s.push_str(&*intString(iSimCodeVarIdx.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok(literal!("NONE"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(oString)
}

fn printSimCodeVarTypes(mut iSimCodeVarTypes: metamodelica::Array<(i32, i32, i32)>) -> Result<()> {
    let mut varIdx: i32 = 0;
    let mut varDataType: i32 = 0;
    let mut varSize: i32 = 0;
    let mut varType: i32 = 0;
    for mut varIdx in 1..=metamodelica::arrayLength(iSimCodeVarTypes.clone()) {
        (varDataType, varSize, varType) = metamodelica::arrayGet(iSimCodeVarTypes.clone(), varIdx.clone())?;
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*intString(varIdx.clone())); __mm_s.push_str(&*literal!(" has data type ")); __mm_s.push_str(&*intString(varDataType.clone())); __mm_s.push_str(&*literal!(" and size ")); __mm_s.push_str(&*intString(varSize.clone())); __mm_s.push_str(&*literal!(" and type ")); __mm_s.push_str(&*intString(varType.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

// -------------------------------------------
// SUSAN
// -------------------------------------------
pub fn getSubscriptListOfArrayCref(mut iCref: Arc<DAE::ComponentRef>, mut iNumArrayElems: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>> {
    let mut oSubscriptList: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>> = metamodelica::nil();
    let mut tmpCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    tmpCrefs = expandCref(iCref.clone(), iNumArrayElems.clone())?;
    oSubscriptList = List::map(tmpCrefs.clone(), (std::sync::Arc::new(ComponentReference::crefLastSubs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> + 'static>))?;
    Ok(oSubscriptList)
}

pub fn expandCref(mut iCref: Arc<DAE::ComponentRef>, mut iNumArrayElems: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut oCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut elems: i32 = 0;
    let mut dims: i32 = 0;
    let mut dimElemCount: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    cref = removeSubscripts(iCref.clone());
    dims = getCrefDims(iCref.clone());
    dimElemCount = getDimElemCount(iNumArrayElems.clone().reverse(), dims.clone())?;
    elems = List::reduce(dimElemCount.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>))?;
    dims = (iNumArrayElems.clone().len() as i32);
    oCrefs = expandCref1(cref.clone(), elems.clone(), dimElemCount.clone())?;
    Ok(oCrefs)
}

pub fn expandCrefWithDims(mut iCref: Arc<DAE::ComponentRef>, mut iDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut oCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut dim: Arc<DAE::Dimension> = Arc::new(DAE::Dimension::DIM_BOOLEAN);
    let mut numArrayElems: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    numArrayElems = metamodelica::nil();
    for mut dim in &*iDims.clone() {
        let mut dim = dim.clone();
        numArrayElems = metamodelica::cons((getDimStringOfDimElement(dim.clone())).clone(), numArrayElems.clone());
    }
    oCrefs = expandCref(iCref.clone(), numArrayElems.clone())?;
    Ok(oCrefs)
}

fn getDimStringOfDimElement(mut iDim: Arc<DAE::Dimension>) -> ArcStr {
    let mut oDimString: ArcStr = arcstr::literal!("");
    let mut integer: i32 = 0;
    oDimString = ((::match_deref::match_deref! { match &(iDim.clone()) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: __esc_integer } => {
            integer = (*__esc_integer).clone();
            intString(integer.clone())
        },
        _ => {
            metamodelica::print((literal!("getDimStringOfDimElement: unsupported Dimension-type given!\n")).clone());
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    oDimString
}

fn removeSubscripts(mut iCref: Arc<DAE::ComponentRef>) -> Arc<DAE::ComponentRef> {
    let mut oCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut ident: ArcStr = arcstr::literal!("");
    let mut identType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let mut componentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    oCref = (::match_deref::match_deref! { match &(iCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: __esc_ident, identType: __esc_identType, subscriptLst: __esc_subscriptLst, componentRef: __esc_componentRef } => {
            ident = (*__esc_ident).clone();
            identType = (*__esc_identType).clone();
            subscriptLst = (*__esc_subscriptLst).clone();
            componentRef = (*__esc_componentRef).clone();
            componentRef = removeSubscripts(componentRef.clone());
            Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone(), componentRef: componentRef.clone() })
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_ident, identType: __esc_identType, subscriptLst: __esc_subscriptLst } => {
            ident = (*__esc_ident).clone();
            identType = (*__esc_identType).clone();
            subscriptLst = (*__esc_subscriptLst).clone();
            Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: metamodelica::nil() })
        },
        _ => iCref.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oCref
}

fn getDimElemCount(mut iNumArrayElems: Arc<metamodelica::List<ArcStr>>, mut iDims: i32) -> Result<Arc<metamodelica::List<i32>>> {
    let mut oNumArrayElems: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut dimList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut intNumArrayElems: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut dims: i32 = 0;
    dims = if (intLe(iDims.clone(), 0)) {(iNumArrayElems.clone().len() as i32)} else {iDims.clone()};
    dimList = List::intRange(dims.clone());
    intNumArrayElems = List::map(iNumArrayElems.clone(), (std::sync::Arc::new(stringInt) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>))?;
    oNumArrayElems = List::map1(dimList.clone(), (std::sync::Arc::new(List::getIndexFirst) as std::sync::Arc<dyn ::std::ops::Fn(i32, _) -> Result<_> + 'static>), intNumArrayElems.clone())?;
    Ok(oNumArrayElems)
}

fn getCrefDims(mut iCref: Arc<DAE::ComponentRef>) -> i32 {
    '__tco: loop {
        let mut componentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
        let mut subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
        let mut tmpDims: i32 = 0;
        ::match_deref::match_deref! { match &(iCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: __esc_componentRef, .. } => {
            componentRef = (*__esc_componentRef).clone();
            { iCref = componentRef.clone(); continue '__tco; }
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: __esc_subscriptLst, .. } => {
            subscriptLst = (*__esc_subscriptLst).clone();
            tmpDims = (subscriptLst.clone().len() as i32);
            return tmpDims.clone()
        },
        _ => {
            metamodelica::print((literal!("HpcOmMemory.getCrefDims failed!\n")).clone());
            return 0
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn expandCref1(mut iCref: Arc<DAE::ComponentRef>, mut iElems: i32, mut iDimElemCount: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut oCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut tmpCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut idxList: Arc<metamodelica::List<i32>> = metamodelica::nil();
    oCrefs = 'mc: {
        let __mc_input = iDimElemCount.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tmpCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = tmpCrefs.clone();
                    tmpCrefs = ComponentReference::expandCref(iCref.clone(), false)?;
                    let true = (intEq((tmpCrefs.clone().len() as i32), iElems.clone())) else { bail!("pattern mismatch") };
                    Ok((tmpCrefs.clone(), tmpCrefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { tmpCrefs = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut idxList: Arc<metamodelica::List<i32>> = idxList.clone();
                    let mut tmpCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = tmpCrefs.clone();
                    idxList = List::intRange(List::reduce(iDimElemCount.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>))?);
                    tmpCrefs = List::map2(idxList.clone(), (std::sync::Arc::new(createArrayIndexCref) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> + 'static>), iDimElemCount.clone(), iCref.clone())?;
                    Ok((tmpCrefs.clone(), idxList.clone(), tmpCrefs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { idxList = __wb0; tmpCrefs = __wb1; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oCrefs)
}

fn createArrayIndexCref(mut iIdx: i32, mut iDimElemCount: Arc<metamodelica::List<i32>>, mut iCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut oCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    (oCref, _) = createArrayIndexCref_impl(iIdx.clone(), iDimElemCount.clone(), (iCref.clone(), 1))?;
    Ok(oCref)
}

fn createArrayIndexCref_impl(mut iIdx: i32, mut iDimElemCount: Arc<metamodelica::List<i32>>, mut iRefCurrentDim: (Arc<DAE::ComponentRef>, i32)) -> Result<(Arc<DAE::ComponentRef>, i32)> {
    let mut oRefCurrentDim: (Arc<DAE::ComponentRef>, i32) = (Arc::new(DAE::ComponentRef::WILD), 0);
    let mut ident: ArcStr = arcstr::literal!("");
    let mut identType: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let mut componentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut currentDim: i32 = 0;
    let mut idxValue: i32 = 0;
    let mut dimElemsPre: i32 = 0;
    let mut dimElems: i32 = 0;
    oRefCurrentDim = 'mc: {
        let __mc_input = iRefCurrentDim.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType, subscriptLst, componentRef }, 1) => {
                    let mut componentRef = (*componentRef).clone();
                    let true = (intLe(1, (iDimElemCount.clone().len() as i32))) else { bail!("pattern mismatch") };
                    (componentRef, _) = createArrayIndexCref_impl(iIdx.clone(), iDimElemCount.clone(), (componentRef.clone(), 1))?;
                    Ok((Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone(), componentRef: componentRef.clone() }), 2))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident, identType, subscriptLst, componentRef }, currentDim) => {
                    let mut componentRef = (*componentRef).clone();
                    let true = (intLe(currentDim.clone(), (iDimElemCount.clone().len() as i32))) else { bail!("pattern mismatch") };
                    (componentRef, _) = createArrayIndexCref_impl(iIdx.clone(), iDimElemCount.clone(), (componentRef.clone(), currentDim.clone()))?;
                    Ok((Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone(), componentRef: componentRef.clone() }), currentDim.clone() + 1))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType, subscriptLst }, 1) => {
                    let mut subscriptLst = (*subscriptLst).clone();
                    let mut idxValue: i32 = idxValue.clone();
                    let true = (intLe(1, (iDimElemCount.clone().len() as i32))) else { bail!("pattern mismatch") };
                    idxValue = intMod(iIdx.clone() - 1, listHead(iDimElemCount.clone())?) + 1;
                    subscriptLst = metamodelica::cons(Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: idxValue.clone() }) }), subscriptLst.clone());
                    Ok((createArrayIndexCref_impl(iIdx.clone(), iDimElemCount.clone(), (Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone() }), 2))?, idxValue.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { idxValue = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType, subscriptLst }, currentDim) => {
                    let mut subscriptLst = (*subscriptLst).clone();
                    let mut dimElems: i32 = dimElems.clone();
                    let mut dimElemsPre: i32 = dimElemsPre.clone();
                    let mut idxValue: i32 = idxValue.clone();
                    let true = (intLe(currentDim.clone(), (iDimElemCount.clone().len() as i32))) else { bail!("pattern mismatch") };
                    dimElemsPre = List::reduce(List::sublist(iDimElemCount.clone(), 1, (iDimElemCount.clone().len() as i32) - currentDim.clone() + 1)?, (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>))?;
                    dimElems = (iDimElemCount.clone()).get(currentDim.clone())?;
                    idxValue = intMod(intDiv(iIdx.clone() - 1, dimElemsPre.clone()), dimElems.clone()) + 1;
                    subscriptLst = metamodelica::cons(Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: idxValue.clone() }) }), subscriptLst.clone());
                    Ok((createArrayIndexCref_impl(iIdx.clone(), iDimElemCount.clone(), (Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident.clone()).clone(), identType: identType.clone(), subscriptLst: subscriptLst.clone() }), currentDim.clone() + 1))?, dimElems.clone(), dimElemsPre.clone(), idxValue.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { dimElems = __wb0; dimElemsPre = __wb1; idxValue = __wb2; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident, identType, subscriptLst }, currentDim) => {
                    let false = (intLe(currentDim.clone(), (iDimElemCount.clone().len() as i32))) else { bail!("pattern mismatch") };
                    Ok(iRefCurrentDim.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print((literal!("createArrayIndexCref_impl failed!\n")).clone());
                    Ok(iRefCurrentDim.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oRefCurrentDim)
}

// -------------------------------------------
// UTIL
// -------------------------------------------
fn getTaskListTasks(mut iTaskList: HpcOmSimCode::TaskList) -> Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> {
    let mut oTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    let mut tasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
    oTasks = (match iTaskList.clone() {
        HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: mut __esc_tasks } => {
            tasks = __esc_tasks.clone();
            tasks.clone()
        },
        HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: mut __esc_tasks } => {
            tasks = __esc_tasks.clone();
            tasks.clone()
        },
        _ => {
            metamodelica::print((literal!("getTaskListTasks failed!\n")).clone());
            metamodelica::nil()
        },
    });
    oTasks
}

fn getCacheLineMapOfPartlyFilledCacheLine(mut iPartlyFilledCacheLine: PartlyFilledCacheLine) -> Result<CacheLineMap> {
    let mut oCacheLineMap: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    let mut cacheLineMap: CacheLineMap = <CacheLineMap as ::std::default::Default>::default();
    oCacheLineMap = (match iPartlyFilledCacheLine.clone() {
        PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_LEVEL { cacheLineMap: mut __esc_cacheLineMap, .. } => {
            cacheLineMap = __esc_cacheLineMap.clone();
            cacheLineMap.clone()
        },
        PartlyFilledCacheLine::PARTLYFILLEDCACHELINE_THREAD { cacheLineMap: mut __esc_cacheLineMap } => {
            cacheLineMap = __esc_cacheLineMap.clone();
            cacheLineMap.clone()
        },
    });
    Ok(oCacheLineMap)
}

fn getAllCacheLinesOfCacheMap(mut iCacheMap: CacheMap) -> Result<Arc<metamodelica::List<CacheLineMap>>> {
    let mut oCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesFloat: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesInt: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut cacheLinesBool: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    let mut allCacheLines: Arc<metamodelica::List<CacheLineMap>> = metamodelica::nil();
    oCacheLines = (match iCacheMap.clone() {
        CacheMap::CACHEMAP { cacheLinesFloat: mut __esc_cacheLinesFloat, cacheLinesInt: mut __esc_cacheLinesInt, cacheLinesBool: mut __esc_cacheLinesBool, .. } => {
            cacheLinesFloat = __esc_cacheLinesFloat.clone();
            cacheLinesInt = __esc_cacheLinesInt.clone();
            cacheLinesBool = __esc_cacheLinesBool.clone();
            allCacheLines = listAppend(cacheLinesFloat.clone(), listAppend(cacheLinesInt.clone(), cacheLinesBool.clone()));
            allCacheLines.clone()
        },
        CacheMap::UNIFORM_CACHEMAP { cacheLines: ref __esc_allCacheLines, .. } => {
            allCacheLines = __esc_allCacheLines.clone();
            allCacheLines.clone()
        },
    });
    Ok(oCacheLines)
}

fn getCacheVariablesOfCacheMap(mut iCacheMap: CacheMap) -> Result<Arc<metamodelica::List<SimCodeVar::SimVar>>> {
    let mut oCacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut cacheVariables: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    oCacheVariables = (match iCacheMap.clone() {
        CacheMap::CACHEMAP { cacheVariables: mut __esc_cacheVariables, .. } => {
            cacheVariables = __esc_cacheVariables.clone();
            cacheVariables.clone()
        },
        CacheMap::UNIFORM_CACHEMAP { cacheVariables: mut __esc_cacheVariables, .. } => {
            cacheVariables = __esc_cacheVariables.clone();
            cacheVariables.clone()
        },
    });
    Ok(oCacheVariables)
}

fn getCacheLineSizeOfCacheMap(mut iCacheMap: CacheMap) -> Result<i32> {
    let mut oCacheLineSize: i32 = 0;
    let mut cacheLineSize: i32 = 0;
    oCacheLineSize = (match iCacheMap.clone() {
        CacheMap::CACHEMAP { cacheLineSize: mut __esc_cacheLineSize, .. } => {
            cacheLineSize = __esc_cacheLineSize.clone();
            cacheLineSize.clone()
        },
        CacheMap::UNIFORM_CACHEMAP { cacheLineSize: mut __esc_cacheLineSize, .. } => {
            cacheLineSize = __esc_cacheLineSize.clone();
            cacheLineSize.clone()
        },
    });
    Ok(oCacheLineSize)
}

