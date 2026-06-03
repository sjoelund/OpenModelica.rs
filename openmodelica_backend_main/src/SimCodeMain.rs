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

use crate::CevalScriptBackend;
use crate::Interactive;
use openmodelica_ast::Absyn;
use openmodelica_backend::BackendDAECreate;
use openmodelica_backend::BackendDAEUtil;
use openmodelica_backend::BackendDump;
use openmodelica_backend::BackendEquation;
use openmodelica_backend::BackendVariable;
use openmodelica_backend::DAEMode;
use openmodelica_backend::HpcOmSimCodeMain;
use openmodelica_backend::HpcOmTaskGraph;
use openmodelica_backend::RuntimeSources;
use openmodelica_backend::SerializeInitXML;
use openmodelica_backend::SerializeModelInfo;
use openmodelica_backend::SerializeSparsityPattern;
use openmodelica_backend::SerializeTaskSystemInfo;
use openmodelica_backend::SimCodeUtil;
use openmodelica_backend::SymbolTable;
use openmodelica_backend::SymbolicJacobian;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_types::ZeroCrossings;
use openmodelica_codegen::CodegenEmbeddedC;
use openmodelica_codegen::CodegenJS;
use openmodelica_codegen_c::CodegenC;
use openmodelica_codegen_cpp::CodegenCpp;
use openmodelica_codegen_cpp_ext::CodegenCppHpcom;
use openmodelica_codegen_cpp_ext::CodegenFMUCpp;
use openmodelica_codegen_cpp_ext::CodegenFMUCppHpcom;
use openmodelica_codegen_cpp_omsi_ext::CodegenOMSICpp;
use openmodelica_codegen_fmu_c::CodegenFMU;
use openmodelica_codegen_fmu_omsi::CodegenOMSIC;
use openmodelica_codegen_fmu_omsi::CodegenOMSI_common;
use openmodelica_codegen_xml::CodegenXML;
use openmodelica_frontend::Builtin;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend::StateMachineFlatten;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_dump::HashTableCrIListArray;
use openmodelica_frontend_dump::HashTableCrILst;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_nbackend::NBackendDAE;
use openmodelica_nbackend::NSimCode;
use openmodelica_nf_frontend::NFConvertDAE;
use openmodelica_nf_frontend::NFFlatModel as FlatModel;
use openmodelica_nf_frontend::NFFlatten::FunctionTree;
use openmodelica_nf_frontend::NFFlatten::FunctionTreeImpl;
use openmodelica_nf_frontend::NFFunction;
use openmodelica_program_util::ProgramUtil;
use openmodelica_simcode_types::HashTableCrefSimVar;
use openmodelica_simcode_types::HpcOmSimCode;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_simcode_util::SimCodeFunctionUtil;
use openmodelica_simcode_util::SimCodeUtilShared;
use openmodelica_susan::Tpl;
use openmodelica_util::Autoconf;
use openmodelica_util::AvlSetString;
use openmodelica_util::BaseHashTable;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ExecStat;
use openmodelica_util::FMI;
use openmodelica_util::File;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::SemanticVersion;
use openmodelica_util::Settings;
use openmodelica_util::StackOverflow;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

/* used for new backend */
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TranslateModelKind {
    NORMAL,
    XML,
    FMU {
        kind: ArcStr,
        targetName: ArcStr,
    },
}
pub use self::TranslateModelKind::{NORMAL,XML,FMU};

pub fn createSimulationSettings(mut startTime: metamodelica::Real, mut stopTime: metamodelica::Real, mut inumberOfIntervals: i32, mut tolerance: metamodelica::Real, mut method: ArcStr, mut options: ArcStr, mut outputFormat: ArcStr, mut variableFilter: ArcStr, mut cflags: ArcStr, mut simflags: ArcStr) -> SimCode::SimulationSettings {
    let mut simSettings: SimCode::SimulationSettings = <SimCode::SimulationSettings as ::std::default::Default>::default();
    let mut stepSize: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut numberOfIntervals: i32 = 0;
    numberOfIntervals = if (inumberOfIntervals.clone() <= 0) {1} else {inumberOfIntervals.clone()};
    stepSize = (stopTime.clone() - startTime.clone()) / intReal(numberOfIntervals.clone());
    simSettings = SimCode::SimulationSettings { startTime: startTime.clone(), stopTime: stopTime.clone(), numberOfIntervals: numberOfIntervals.clone(), stepSize: stepSize.clone(), tolerance: tolerance.clone(), method: (method.clone()).clone(), options: (options.clone()).clone(), outputFormat: (outputFormat.clone()).clone(), variableFilter: (variableFilter.clone()).clone(), cflags: (cflags.clone()).clone(), simflags: (simflags.clone()).clone() };
    simSettings
}

fn generateModelCodeFMU(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE_lambda0: Option<Arc<BackendDAE::BackendDAE>>, mut inFMIDer: Arc<metamodelica::List<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))>>, mut inRemovedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut p: Absyn::Program, mut className: Arc<Absyn::Path>, mut FMUVersion: ArcStr, mut FMUType: ArcStr, mut filenamePrefix: ArcStr, mut fmuTargetName: ArcStr, mut simSettings: Option<SimCode::SimulationSettings>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr, metamodelica::Real, metamodelica::Real)> {
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut fileDir: ArcStr = arcstr::literal!("");
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut includes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
    let mut simCode: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
    let mut recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
    let mut a_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut libPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut literals: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>);
    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    a_cref = AbsynUtil::pathToCref(className.clone())?;
    if Config::simCodeTarget()? == literal!("omsic") {
        fileDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*listHead(AbsynUtil::pathToStringList(className.clone())?)?); __mm_s.push_str(&*literal!(".tmp")); ArcStr::from(__mm_s) }).clone();
    } else {
        fileDir = (ProgramUtil::getFileDir(a_cref.clone(), p.clone())?).clone();
    }
    (libs, libPaths, includes, includeDirs, recordDecls, functions, literals) = SimCodeUtilShared::createFunctions(p.clone(), inBackendDAE.shared.functionTree.clone())?;
    simCode = createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), None, inRemovedInitialEquationLst.clone(), className.clone(), (filenamePrefix.clone()).clone(), (fileDir.clone()).clone(), functions.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), p.clone(), simSettings.clone(), recordDecls.clone(), literals.clone(), Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::nil() }), true, (FMUVersion.clone()).clone(), (fmuTargetName.clone()).clone(), inFMIDer.clone())?;
    timeSimCode = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    ExecStat::execStat((literal!("SimCode")).clone())?;
    System::realtimeTick(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    if Config::simCodeTarget()? == literal!("omsicpp") {
        callTargetTemplatesFMU(simCode.clone(), (literal!("C")).clone(), (FMUVersion.clone()).clone(), (FMUType.clone()).clone(), p.clone())?;
    } else {
        callTargetTemplatesFMU(simCode.clone(), (Config::simCodeTarget()?).clone(), (FMUVersion.clone()).clone(), (FMUType.clone()).clone(), p.clone())?;
    }
    timeTemplates = System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    Ok((libs, fileDir, timeSimCode, timeTemplates))
}

fn generateModelCodeXML(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE_lambda0: Option<Arc<BackendDAE::BackendDAE>>, mut inRemovedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut p: Absyn::Program, mut className: Arc<Absyn::Path>, mut filenamePrefix: ArcStr, mut simSettingsOpt: Option<SimCode::SimulationSettings>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr, metamodelica::Real, metamodelica::Real)> {
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut fileDir: ArcStr = arcstr::literal!("");
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut includes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
    let mut simCode: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
    let mut recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
    let mut libPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut a_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut literals: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>);
    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    a_cref = AbsynUtil::pathToCref(className.clone())?;
    fileDir = (ProgramUtil::getFileDir(a_cref.clone(), p.clone())?).clone();
    (libs, libPaths, includes, includeDirs, recordDecls, functions, literals) = SimCodeUtilShared::createFunctions(p.clone(), inBackendDAE.shared.functionTree.clone())?;
    (simCode, _) = SimCodeUtil::createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), None, inRemovedInitialEquationLst.clone(), className.clone(), (filenamePrefix.clone()).clone(), (fileDir.clone()).clone(), functions.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), p.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::nil() }), false, (literal!("")).clone(), (literal!("")).clone(), metamodelica::nil())?;
    timeSimCode = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    ExecStat::execStat((literal!("SimCode")).clone())?;
    System::realtimeTick(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    callTargetTemplatesXML(simCode.clone(), (Config::simCodeTarget()?).clone())?;
    timeTemplates = System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    Ok((libs, fileDir, timeSimCode, timeTemplates))
}

pub fn generateModelCode(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE_lambda0: Option<Arc<BackendDAE::BackendDAE>>, mut inInlineData: Option<BackendDAE::InlineData>, mut inRemovedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut p: Absyn::Program, mut className: Arc<Absyn::Path>, mut filenamePrefix: ArcStr, mut simSettingsOpt: Option<SimCode::SimulationSettings>, mut args: Arc<Absyn::FunctionArgs>, mut inFMIDer: Arc<metamodelica::List<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))>>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr, metamodelica::Real, metamodelica::Real)> {
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut fileDir: ArcStr = arcstr::literal!("");
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut includes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut libPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
    let mut simCode: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
    let mut recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
    let mut a_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut literals: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>);
    let mut numCheckpoints: i32 = 0;
    numCheckpoints = ErrorExt::getNumCheckpoints();
    StackOverflow::clearStacktraceMessages();
    if Flags::isSet(Flags::GRAPHML.clone())? {
        HpcOmTaskGraph::dumpTaskGraph(inBackendDAE.clone(), (filenamePrefix.clone()).clone())?;
        BackendDump::dumpBackendDAEBipartiteGraph(inBackendDAE.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BipartiteGraph_CompleteDAE_")); __mm_s.push_str(&*filenamePrefix.clone()); ArcStr::from(__mm_s) }).clone())?;
    }
    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    a_cref = AbsynUtil::pathToCref(className.clone())?;
    fileDir = (ProgramUtil::getFileDir(a_cref.clone(), p.clone())?).clone();
    (libs, libPaths, includes, includeDirs, recordDecls, functions, literals) = SimCodeUtilShared::createFunctions(p.clone(), inBackendDAE.shared.functionTree.clone())?;
    simCode = createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), inInlineData.clone(), inRemovedInitialEquationLst.clone(), className.clone(), (filenamePrefix.clone()).clone(), (fileDir.clone()).clone(), functions.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), p.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args.clone(), false, (literal!("")).clone(), (literal!("")).clone(), inFMIDer.clone())?;
    timeSimCode = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    ExecStat::execStat((literal!("SimCode")).clone())?;
    if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
        serializeNotify(simCode.clone(), (literal!("SimCode")).clone())?;
        ExecStat::execStat((literal!("Serialize simCode")).clone())?;
    }
    System::realtimeTick(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    callTargetTemplates(simCode.clone(), (Config::simCodeTarget()?).clone())?;
    timeTemplates = System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    ExecStat::execStat((literal!("Templates")).clone())?;
    return Ok((libs.clone(), fileDir.clone(), timeSimCode.clone(), timeTemplates.clone()));
    bail!("fail");
    Ok((libs, fileDir, timeSimCode, timeTemplates))
}

fn createSimCode(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE_lambda0: Option<Arc<BackendDAE::BackendDAE>>, mut inInlineData: Option<BackendDAE::InlineData>, mut inRemovedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inClassName: Arc<Absyn::Path>, mut filenamePrefix: ArcStr, mut inString11: ArcStr, mut functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>, mut externalFunctionIncludes: Arc<metamodelica::List<ArcStr>>, mut includeDirs: Arc<metamodelica::List<ArcStr>>, mut libs: Arc<metamodelica::List<ArcStr>>, mut libPaths: Arc<metamodelica::List<ArcStr>>, mut program: Absyn::Program, mut simSettingsOpt: Option<SimCode::SimulationSettings>, mut recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>, mut literals: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>), mut args: Arc<Absyn::FunctionArgs>, mut isFMU: bool, mut FMUVersion: ArcStr, mut fmuTargetName: ArcStr, mut inFMIDer: Arc<metamodelica::List<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))>>) -> Result<SimCode::SimCode> {
    let mut simCode: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
    simCode = 'mc: {
        let __mc_input = args.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::MULTIRATE_PARTITION.clone())?) else { bail!("pattern mismatch") };
                    Ok(HpcOmSimCodeMain::createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), inRemovedInitialEquationLst.clone(), inClassName.clone(), (filenamePrefix.clone()).clone(), (inString11.clone()).clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut numProc: i32 = 0;
                    let true = (Flags::isSet(Flags::HPCOM.clone())?) else { bail!("pattern mismatch") };
                    numProc = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
                    let true = (numProc.clone() == 0) else { bail!("pattern mismatch") };
                    metamodelica::print((literal!("hpcom computes the ideal number of processors. If you want to set the number manually, use the flag +n=_\n")).clone());
                    Ok(HpcOmSimCodeMain::createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), inRemovedInitialEquationLst.clone(), inClassName.clone(), (filenamePrefix.clone()).clone(), (inString11.clone()).clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut numProc: i32 = 0;
                    let true = (Flags::isSet(Flags::HPCOM.clone())?) else { bail!("pattern mismatch") };
                    numProc = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
                    let true = (numProc.clone() > 0) else { bail!("pattern mismatch") };
                    Ok(HpcOmSimCodeMain::createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), inRemovedInitialEquationLst.clone(), inClassName.clone(), (filenamePrefix.clone()).clone(), (inString11.clone()).clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tmpSimCode: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
                    (tmpSimCode, _) = SimCodeUtil::createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), inInlineData.clone(), inRemovedInitialEquationLst.clone(), inClassName.clone(), (filenamePrefix.clone()).clone(), (inString11.clone()).clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args.clone(), isFMU.clone(), (FMUVersion.clone()).clone(), (fmuTargetName.clone()).clone(), inFMIDer.clone())?;
                    Ok(tmpSimCode.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(simCode)
}

fn generateModelCodeNewBackend(mut bdae: Arc<NBackendDAE::NBackendDAE>, mut className: Arc<Absyn::Path>, mut fileNamePrefix: ArcStr, mut simSettingsOpt: Option<SimCode::SimulationSettings>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr, metamodelica::Real, metamodelica::Real, Arc<AvlTreePathFunction::Tree>)> {
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut fileDir: ArcStr = arcstr::literal!("");
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut oldFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut numCheckpoints: i32 = 0;
    let mut simCode: Arc<NSimCode::SimCode::SimCode> = Arc::new(<NSimCode::SimCode::SimCode as ::std::default::Default>::default());
    let mut oldSimCode: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
    numCheckpoints = ErrorExt::getNumCheckpoints();
    StackOverflow::clearStacktraceMessages();
    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    (simCode, oldFunctionTree) = NSimCode::SimCode::create(bdae.clone(), className.clone(), (fileNamePrefix.clone()).clone(), simSettingsOpt.clone(), SymbolTable::getAbsyn())?;
    if Flags::isSet(Flags::DUMP_SIMCODE.clone())? {
        metamodelica::print((NSimCode::SimCode::toString(simCode.clone(), (literal!("")).clone())?).clone());
    }
    (fileDir, libs) = NSimCode::SimCode::getDirectoryAndLibs(simCode.clone())?;
    oldSimCode = NSimCode::SimCode::convert(simCode.clone())?;
    if Flags::isSet(Flags::DUMP_SIMCODE.clone())? {
        SimCodeUtil::dumpSimCodeDebug(oldSimCode.clone())?;
    }
    timeSimCode = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    ExecStat::execStat((literal!("SimCode")).clone())?;
    if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
        serializeNotify(oldSimCode.clone(), (literal!("SimCode")).clone())?;
        ExecStat::execStat((literal!("Serialize simCode")).clone())?;
    }
    System::realtimeTick(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    callTargetTemplates(oldSimCode.clone(), (Config::simCodeTarget()?).clone())?;
    timeTemplates = System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    ExecStat::execStat((literal!("Templates")).clone())?;
    Ok((libs, fileDir, timeSimCode, timeTemplates, oldFunctionTree))
}

type PartialRunTpl = std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>;

type FuncText = std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>;

fn runTplWriteFile(mut func: Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>, mut file: ArcStr) -> (bool, Arc<metamodelica::List<ArcStr>>) {
    let mut res: (bool, Arc<metamodelica::List<ArcStr>>) = (false, metamodelica::nil());
    let mut nErr: i32 = 0;
    res = (false, metamodelica::nil());
    if '__try0: {
        unwrap_break_err!(SimCodeUtil::resetFunctionIndex(), '__try0);
        SimCodeFunctionUtil::codegenResetTryThrowIndex();
        if unwrap_break_err!(Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone()), '__try0) {
            unwrap_break_err!(Tpl::textFileConvertLines(unwrap_break_err!(Tpl::tplCallWithFailErrorNoArg(func.clone(), Tpl::emptyTxt.clone()), '__try0), (file.clone()).clone()), '__try0);
        } else {
            nErr = Error::getNumErrorMessages();
            unwrap_break_err!(Tpl::closeFile(unwrap_break_err!(Tpl::tplCallWithFailErrorNoArg(func.clone(), unwrap_break_err!(Tpl::redirectToFile(Tpl::emptyTxt.clone(), (file.clone()).clone()), '__try0)), '__try0)), '__try0);
            unwrap_break_err!(Tpl::failIfTrue(Error::getNumErrorMessages() > nErr.clone()), '__try0);
        }
        res = (true, SimCodeUtil::getFunctionIndex());
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    res
}

fn runTpl(mut func: Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>) -> (bool, Arc<metamodelica::List<ArcStr>>) {
    let mut res: (bool, Arc<metamodelica::List<ArcStr>>) = (false, metamodelica::nil());
    res = (false, metamodelica::nil());
    if '__try0: {
        unwrap_break_err!(SimCodeUtil::resetFunctionIndex(), '__try0);
        SimCodeFunctionUtil::codegenResetTryThrowIndex();
        unwrap_break_err!(Tpl::tplCallWithFailErrorNoArg(func.clone(), Tpl::emptyTxt.clone()), '__try0);
        res = (true, SimCodeUtil::getFunctionIndex());
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    res
}

// TODO: use another switch ... later make it first class option like -target or so
fn callTargetTemplates(mut simCode: SimCode::SimCode, mut target: ArcStr) -> Result<()> {
    type Func = std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>;

    type FuncText = std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>;

    type BoolFunc = std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>;

    fn runToStr(mut func: Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static>) -> (bool, Arc<metamodelica::List<ArcStr>>) {
        pub type Func = std::sync::Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static>;

        let mut res: (bool, Arc<metamodelica::List<ArcStr>>) = (false, metamodelica::nil());
        res = (false, metamodelica::nil());
        if '__try0: {
            unwrap_break_err!(SimCodeUtil::resetFunctionIndex(), '__try0);
            SimCodeFunctionUtil::codegenResetTryThrowIndex();
            unwrap_break_err!(func(), '__try0);
            res = (true, SimCodeUtil::getFunctionIndex());
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
        res
    }

    fn runCodegenFunc(mut func: Arc<dyn ::std::ops::Fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>) -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> {
        let mut res: (bool, Arc<metamodelica::List<ArcStr>>) = (false, metamodelica::nil());
        let mut b: bool = false;
        let __pa1 @ (__pa0, _) = &(func()?);
        b = __pa0.clone();
        res = __pa1.clone();
        if !(b.clone()) {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*(System::dladdr(func.clone())).0); __mm_s.push_str(&*literal!(" failed\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        }
        if ErrorExt::getNumMessages() > 0 {
            ErrorExt::moveMessagesToParentThread();
        }
        Ok(res)
    }

    fn runToBoolean(mut func: Arc<dyn ::std::ops::Fn() -> Result<bool> + 'static>) -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> {
        type Func = std::sync::Arc<dyn ::std::ops::Fn() -> Result<bool> + 'static>;

        let mut res: (bool, Arc<metamodelica::List<ArcStr>>) = (false, metamodelica::nil());
        res = (func()?, metamodelica::nil());
        Ok(res)
    }

    let mut func: Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>;
    let mut txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut generatedObjects: Arc<AvlSetString::Tree> = Arc::new(openmodelica_util::AvlSetString::Tree::EMPTY);
    { let __v = Some(simCode.clone()); openmodelica_backend::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = __v) };
    let () = ({
        let mut res: Arc<metamodelica::List<(bool, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(target.clone()) {
        Deref @ "Cpp" => {
            let mut r#str: ArcStr = arcstr::literal!("");
            callTargetTemplatesCPP(simCode.clone())?;
            for mut r#str in &*list![(literal!("CalcHelperMain.o\n")).clone(), (literal!(".so\n")).clone()] {
                let mut r#str = r#str.clone();
                generatedObjects = AvlSetString::add(generatedObjects.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("OMCpp")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?;
            }
            ()
        },
        Deref @ "C" => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut guid: ArcStr = arcstr::literal!("");
            let mut codegenFuncs: Arc<metamodelica::List<PartialRunTpl>> = metamodelica::nil();
            let mut numThreads: i32 = 0;
            let mut n: i32 = 0;
            let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut tmp: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut matches: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            guid = (System::getUUIDStr()).clone();
            System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
            codegenFuncs = metamodelica::nil();
            codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0: Arc<dyn ::std::ops::Fn() -> Result<bool> + 'static> = (std::sync::Arc::new({ let __pe_b0 = simCode.clone(); let __pe_b1 = (guid.clone()).clone(); move || Ok(SerializeInitXML::simulationInitFileReturnBool(__pe_b0.clone(), __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<bool> + 'static>); move || runToBoolean(__pe_b0.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), codegenFuncs.clone());
            codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0 = (std::sync::Arc::new({ let __pe_b1 = simCode.clone(); move |__pe_a0| CodegenC::translateModel(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>); move || Ok(runTpl(__pe_b0.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), codegenFuncs.clone());
            for mut f in &*list![((std::sync::Arc::new(CodegenC::simulationFile_exo) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_01exo.c")), ((std::sync::Arc::new(CodegenC::simulationFile_nls) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_02nls.c")), ((std::sync::Arc::new(CodegenC::simulationFile_lsy) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_03lsy.c")), ((std::sync::Arc::new(CodegenC::simulationFile_set) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_04set.c")), ((std::sync::Arc::new(CodegenC::simulationFile_evt) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_05evt.c")), ((std::sync::Arc::new(CodegenC::simulationFile_inz) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_06inz.c")), ((std::sync::Arc::new(CodegenC::simulationFile_dly) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_07dly.c")), ((std::sync::Arc::new(CodegenC::simulationFile_bnd) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_08bnd.c")), ((std::sync::Arc::new(CodegenC::simulationFile_alg) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_09alg.c")), ((std::sync::Arc::new(CodegenC::simulationFile_asr) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_10asr.c")), ((std::sync::Arc::new(CodegenC::simulationFile_jac) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_12jac.c")), ((std::sync::Arc::new(CodegenC::simulationFile_jac_header) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_12jac.h")), ((std::sync::Arc::new(CodegenC::simulationFile_opt) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_13opt.c")), ((std::sync::Arc::new(CodegenC::simulationFile_opt_header) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_13opt.h")), ((std::sync::Arc::new(CodegenC::simulationFile_lnz) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_14lnz.c")), ((std::sync::Arc::new(CodegenC::simulationFile_syn) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_15syn.c")), ((std::sync::Arc::new(CodegenC::simulationFile_dae) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_16dae.c")), ((std::sync::Arc::new(CodegenC::simulationFile_dae_header) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_16dae.h")), ((std::sync::Arc::new(CodegenC::simulationFile_inl) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_17inl.c")), ((std::sync::Arc::new(CodegenC::simulationFile_spd) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_18spd.c")), ((std::sync::Arc::new(CodegenC::simulationHeaderFile) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_model.h"))] {
                let mut f = f.clone();
                (func, r#str) = f.clone();
                codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0 = (std::sync::Arc::new({ let __pe_b1 = simCode.clone(); move |__pe_a0| func(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>); let __pe_b1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone(); move || Ok(runTplWriteFile(__pe_b0.clone(), __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), codegenFuncs.clone());
                (n, matches) = System::regex((r#str.clone()).clone(), (literal!("\\(.*\\)[.]c$")).clone(), 2, false, false);
                if n.clone() == 2 {
                    let __pa0 = ::match_deref::match_deref! { match &(matches.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r#str = __pa0.clone();
                    generatedObjects = AvlSetString::add(generatedObjects.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(".o\n")); ArcStr::from(__mm_s) }).clone())?;
                }
            }
            for mut r#str in &*list![(literal!("_11mix.o\n")).clone(), (literal!("_functions.o\n")).clone(), (literal!("_info.json\n")).clone(), (literal!("_init.xml\n")).clone()] {
                let mut r#str = r#str.clone();
                generatedObjects = AvlSetString::add(generatedObjects.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?;
            }
            codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0 = (std::sync::Arc::new({ let __pe_b1 = simCode.clone(); let __pe_b2 = (simCode.fileNamePrefix.clone()).clone(); move |__pe_a0| CodegenC::simulationFile_mixAndHeader(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>); move || Ok(runTpl(__pe_b0.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), codegenFuncs.clone());
            codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0 = (std::sync::Arc::new({ let __pe_b1 = simCode.clone(); let __pe_b2 = (guid.clone()).clone(); let __pe_b3 = (literal!("")).clone(); move |__pe_a0| CodegenC::simulationFile(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>); let __pe_b1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!(".c")); ArcStr::from(__mm_s) }).clone(); move || Ok(runTplWriteFile(__pe_b0.clone(), __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), codegenFuncs.clone());
            codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0 = (std::sync::Arc::new({ let __pe_b1 = (simCode.fileNamePrefix.clone()).clone(); let __pe_b2 = simCode.modelInfo.functions.clone(); let __pe_b3 = simCode.generic_loop_calls.clone(); move |__pe_a0| CodegenC::simulationFunctionsFile(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>); let __pe_b1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_functions.c")); ArcStr::from(__mm_s) }).clone(); move || Ok(runTplWriteFile(__pe_b0.clone(), __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), codegenFuncs.clone());
            codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0: Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static> = (std::sync::Arc::new({ let __pe_b0 = simCode.clone(); move || SerializeSparsityPattern::serialize(__pe_b0.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static>); move || Ok(runToStr(__pe_b0.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), codegenFuncs.clone());
            codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0: Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static> = (std::sync::Arc::new({ let __pe_b0 = simCode.clone(); let __pe_b1 = Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?; move || SerializeModelInfo::serialize(__pe_b0.clone(), __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static>); move || Ok(runToStr(__pe_b0.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), codegenFuncs.clone());
            if Flags::getConfigBool(Flags::PARMODAUTO.clone())? {
                codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0: Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static> = (std::sync::Arc::new({ let __pe_b0 = simCode.clone(); let __pe_b1 = Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?; move || SerializeTaskSystemInfo::serializeParMod(__pe_b0.clone(), __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static>); move || Ok(runToStr(__pe_b0.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), codegenFuncs.clone());
                generatedObjects = AvlSetString::add(generatedObjects.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_ode.json\n")); ArcStr::from(__mm_s) }).clone())?;
            }
            if arcstr::literal!(Autoconf::os) == literal!("Windows_NT") {
                codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0: Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static> = (std::sync::Arc::new({ let __pe_b0 = simCode.clone(); move || SimCodeUtil::generateRunnerBatScript(__pe_b0.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static>); move || Ok(runToStr(__pe_b0.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), codegenFuncs.clone());
            }
            numThreads = std::cmp::max(1, if (Testsuite::isRunning()?) {std::cmp::min(2, System::numProcessors())} else {Config::noProc()?});
            if !(Flags::isSet(Flags::PARALLEL_CODEGEN.clone())?) || numThreads.clone() == 1 {
                res = ({
        let mut __acc: Arc<metamodelica::List<(bool, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
        for mut codegen_func in (codegenFuncs.clone()).into_iter().cloned() {
            let __x = codegen_func()?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            } else {
                res = System::launchParallelTasks(numThreads.clone(), codegenFuncs.clone(), (std::sync::Arc::new(runCodegenFunc) as std::sync::Arc<dyn ::std::ops::Fn(Arc<dyn ::std::ops::Fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>) -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>))?;
            }
            strs = metamodelica::nil();
            for mut tpl in &*res.clone() {
                let mut tpl = tpl.clone();
                let __pa2 = ::match_deref::match_deref! { match &(tpl.clone()) {
                    (true, __pa2) => __pa2.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                tmp = __pa2.clone();
                strs = List::append_reverse(tmp.clone(), strs.clone());
            }
            strs = strs.clone().reverse();
            for mut r#str in &*strs.clone() {
                let mut r#str = r#str.clone();
                (n, matches) = System::regex((r#str.clone()).clone(), (literal!("\\(.*\\)[.]c$")).clone(), 2, false, false);
                if n.clone() == 2 {
                    let __pa3 = ::match_deref::match_deref! { match &(matches.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: _ } } => __pa3.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r#str = __pa3.clone();
                    generatedObjects = AvlSetString::add(generatedObjects.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(".o\n")); ArcStr::from(__mm_s) }).clone())?;
                }
            }
            Tpl::closeFile(Tpl::tplCallWithFailError3((std::sync::Arc::new(CodegenC::simulationMakefile) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, ArcStr, SimCode::SimCode, Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> + 'static>), (Config::simulationCodeTarget()?).clone(), simCode.clone(), strs.clone(), Tpl::redirectToFile(Tpl::emptyTxt.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!(".makefile")); ArcStr::from(__mm_s) }).clone())?)?)?;
            ()
        },
        Deref @ "ExperimentalEmbeddedC" => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut codegenFuncs: Arc<metamodelica::List<PartialRunTpl>> = metamodelica::nil();
            let mut numThreads: i32 = 0;
            let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut tmp: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
            codegenFuncs = metamodelica::nil();
            for mut f in &*list![((std::sync::Arc::new(CodegenEmbeddedC::mainFile) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), literal!("_main.c"))] {
                let mut f = f.clone();
                (func, r#str) = f.clone();
                codegenFuncs = metamodelica::cons((std::sync::Arc::new({ let __pe_b0 = (std::sync::Arc::new({ let __pe_b1 = simCode.clone(); move |__pe_a0| func(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>); let __pe_b1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone(); move || Ok(runTplWriteFile(__pe_b0.clone(), __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>), codegenFuncs.clone());
            }
            numThreads = std::cmp::max(1, if (Testsuite::isRunning()?) {std::cmp::min(2, System::numProcessors())} else {Config::noProc()?});
            if !(Flags::isSet(Flags::PARALLEL_CODEGEN.clone())?) || numThreads.clone() == 1 {
                res = ({
        let mut __acc: Arc<metamodelica::List<(bool, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
        for mut func in (codegenFuncs.clone()).into_iter().cloned() {
            let __x = func()?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            } else {
                res = System::launchParallelTasks(numThreads.clone(), codegenFuncs.clone(), (std::sync::Arc::new(runCodegenFunc) as std::sync::Arc<dyn ::std::ops::Fn(Arc<dyn ::std::ops::Fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>) -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> + 'static>))?;
            }
            strs = metamodelica::nil();
            for mut tpl in &*res.clone() {
                let mut tpl = tpl.clone();
                let __pa0 = ::match_deref::match_deref! { match &(tpl.clone()) {
                    (true, __pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                tmp = __pa0.clone();
                strs = List::append_reverse(tmp.clone(), strs.clone());
            }
            strs = strs.clone().reverse();
            ()
        },
        Deref @ "JavaScript" => {
            let mut guid: ArcStr = arcstr::literal!("");
            guid = (System::getUUIDStr()).clone();
            Tpl::tplNoret((std::sync::Arc::new(CodegenC::translateModel) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), simCode.clone())?;
            SerializeInitXML::simulationInitFile(simCode.clone(), (guid.clone()).clone())?;
            System::covertTextFileToCLiteral(({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_init.xml")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_init.c")); ArcStr::from(__mm_s) }).clone(), (Config::simulationCodeTarget()?).clone());
            SerializeSparsityPattern::serialize(simCode.clone())?;
            SerializeModelInfo::serialize(simCode.clone(), Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?)?;
            Tpl::tplNoret((std::sync::Arc::new(CodegenJS::markdownFile) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), simCode.clone())?;
            ()
        },
        Deref @ "XML" => {
            Tpl::tplNoret((std::sync::Arc::new(CodegenXML::translateModel) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), simCode.clone())?;
            ()
        },
        Deref @ "None" => {
            ()
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown template target: ")); __mm_s.push_str(&*target.clone()); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    if Testsuite::isRunning()? {
        System::appendFile((Testsuite::getTempFilesFile()?).clone(), stringAppendList(AvlSetString::listKeys(generatedObjects.clone(), metamodelica::nil())))?;
    }
    { let __v = None; openmodelica_backend::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = __v) };
    Ok(())
}

fn callTargetTemplatesCPP(mut iSimCode: SimCode::SimCode) -> Result<()> {
    if Flags::isSet(Flags::HPCOM.clone())? {
        Tpl::tplNoret((std::sync::Arc::new(CodegenCppHpcom::translateModel) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), iSimCode.clone())?;
    } else {
        Tpl::tplNoret((std::sync::Arc::new(CodegenCpp::translateModel) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), iSimCode.clone())?;
    }
    Ok(())
}

fn callTargetTemplatesOMSICpp(mut iSimCode: SimCode::SimCode, mut program: Absyn::Program) -> Result<()> {
    let mut fmuVersion: ArcStr = arcstr::literal!("");
    let mut fmuType: ArcStr = arcstr::literal!("");
    fmuVersion = (literal!("2.0")).clone();
    fmuType = (literal!("me")).clone();
    Tpl::tplNoret3((std::sync::Arc::new(CodegenOMSICpp::translateModel) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode, ArcStr, ArcStr) -> Result<Tpl::Text> + 'static>), iSimCode.clone(), (fmuVersion.clone()).clone(), (fmuType.clone()).clone())?;
    callTargetTemplatesFMU(iSimCode.clone(), (literal!("C")).clone(), (fmuVersion.clone()).clone(), (fmuType.clone()).clone(), program.clone())?;
    Ok(())
}

fn callTargetTemplatesFMU(mut simCode: SimCode::SimCode, mut target: ArcStr, mut FMUVersion: ArcStr, mut FMUType: ArcStr, mut program: Absyn::Program) -> Result<()> {
    { let __v = Some(simCode.clone()); openmodelica_backend::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = __v) };
    let () = ({
        let mut needSundials: bool = false;
        (::match_deref::match_deref! { match &((simCode.clone(), target.clone())) {
        (SimCode::SimCode { .. }, Deref @ "C") => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut newdir: ArcStr = arcstr::literal!("");
            let mut newpath: ArcStr = arcstr::literal!("");
            let mut resourcesDir: ArcStr = arcstr::literal!("");
            let mut dirname: ArcStr = arcstr::literal!("");
            let mut htmlFile: ArcStr = arcstr::literal!("");
            let mut fmutmp: ArcStr = arcstr::literal!("");
            let mut b: bool = false;
            let mut exportDocumentation: bool = false;
            let mut fileNamePrefixHash: ArcStr = arcstr::literal!("");
            let mut install_include_omc_dir: ArcStr = arcstr::literal!("");
            let mut install_include_omc_c_dir: ArcStr = arcstr::literal!("");
            let mut install_share_buildproject_dir: ArcStr = arcstr::literal!("");
            let mut install_fmu_sources_dir: ArcStr = arcstr::literal!("");
            let mut fmu_tmp_sources_dir: ArcStr = arcstr::literal!("");
            let mut cmakelistsStr: ArcStr = arcstr::literal!("");
            let mut needCvode: ArcStr = arcstr::literal!("");
            let mut cvodeDirectory: ArcStr = arcstr::literal!("");
            let mut modelDefinesHeaderStr: ArcStr = arcstr::literal!("");
            let mut model_desc_src_files: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut fmi2HeaderFiles: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut modelica_standard_table_sources: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut dgesv_sources: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut cminpack_sources: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut simrt_c_sundials_sources: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut simrt_linear_solver_sources: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut simrt_non_linear_solver_sources: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut simrt_mixed_solver_sources: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut fmi_export_files: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut model_gen_files: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut model_all_gen_files: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut shared_source_files: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut varInfo: SimCode::VarInfo = <SimCode::VarInfo as ::std::default::Default>::default();
            fileNamePrefixHash = (Util::hashFileNamePrefix((simCode.fileNamePrefix.clone()).clone())?).clone();
            fmutmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fileNamePrefixHash.clone()); __mm_s.push_str(&*literal!(".fmutmp")); ArcStr::from(__mm_s) }).clone();
            if System::directoryExists((fmutmp.clone()).clone()) {
                if !(System::removeDirectory((fmutmp.clone()).clone())) {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to remove directory: ")); __mm_s.push_str(&*fmutmp.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    bail!("fail");
                }
            }
            Util::createDirectoryTree(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources/include/")); ArcStr::from(__mm_s) }).clone())?;
            resourcesDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/resources/")); ArcStr::from(__mm_s) }).clone();
            Util::createDirectoryTree((resourcesDir.clone()).clone())?;
            for mut path in &*simCode.modelInfo.resourcePaths.clone() {
                let mut path = path.clone();
                dirname = (System::dirname((path.clone()).clone())).clone();
                if arcstr::literal!(Autoconf::os) == literal!("Windows_NT") {
                    dirname = (System::stringReplace((dirname.clone()).clone(), (literal!(":")).clone(), (literal!("")).clone())?).clone();
                }
                newdir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*resourcesDir.clone()); __mm_s.push_str(&*dirname.clone()); ArcStr::from(__mm_s) }).clone();
                newpath = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*resourcesDir.clone()); __mm_s.push_str(&*path.clone()); ArcStr::from(__mm_s) }).clone();
                if System::regularFileExists((newpath.clone()).clone()) || System::directoryExists((newpath.clone()).clone()) {
                    continue;
                }
                Util::createDirectoryTree((newdir.clone()).clone())?;
                if 0 != System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cp -rf \"")); __mm_s.push_str(&*path.clone()); __mm_s.push_str(&*literal!("\" \"")); __mm_s.push_str(&*newdir.clone()); __mm_s.push_str(&*literal!("/\"")); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone()) {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to copy path ")); __mm_s.push_str(&*path.clone()); __mm_s.push_str(&*literal!(" to ")); __mm_s.push_str(&*resourcesDir.clone()); __mm_s.push_str(&*dirname.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                }
            }
            let () = (match simCode.fmiSimulationFlags.clone() {
        Some(SimCode::FmiSimulationFlags::FMI_SIMULATION_FLAGS_FILE { path: mut pathToFlagsJson }) => {
            needSundials = true;
            if 0 != System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cp -rf \"")); __mm_s.push_str(&*pathToFlagsJson.clone()); __mm_s.push_str(&*literal!("\" \"")); __mm_s.push_str(&*resourcesDir.clone()); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_flags.json\"")); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone()) {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to copy ")); __mm_s.push_str(&*pathToFlagsJson.clone()); __mm_s.push_str(&*literal!(" to ")); __mm_s.push_str(&*resourcesDir.clone()); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_flags.json")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            }
            ()
        },
        _ => {
            ()
        },
    });
            SerializeSparsityPattern::serialize(simCode.clone())?;
            for mut jac in &*simCode.jacobianMatrices.clone() {
                let mut jac = jac.clone();
                if !(jac.sparsity.clone().is_empty()) {
                    if 0 != System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("mv '")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Jac")); __mm_s.push_str(&*jac.matrixName.clone()); __mm_s.push_str(&*literal!(".bin")); __mm_s.push_str(&*literal!("' '")); __mm_s.push_str(&*resourcesDir.clone()); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone()) {
                        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to move ")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Jac")); __mm_s.push_str(&*jac.matrixName.clone()); __mm_s.push_str(&*literal!(".bin file")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    }
                }
            }
            SerializeModelInfo::serialize(simCode.clone(), Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?)?;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources/")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); ArcStr::from(__mm_s) }).clone();
            if FMUVersion.clone() == literal!("1.0") {
                b = System::covertTextFileToCLiteral(({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_info.json")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("_info.c")); ArcStr::from(__mm_s) }).clone(), (Flags::getConfigString(Flags::TARGET.clone())?).clone());
                if !(b.clone()) {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("System.covertTextFileToCLiteral failed. Could not write ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("_info.c\n")); ArcStr::from(__mm_s) }).clone()])?;
                    bail!("fail");
                }
            } else {
                if Flags::getConfigEnum(Flags::FMI_FILTER.clone())? != Flags::FMI_BLACKBOX.clone() && Flags::getConfigEnum(Flags::FMI_FILTER.clone())? != Flags::FMI_PROTECTED.clone() {
                    if 0 != System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("mv '")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_info.json")); __mm_s.push_str(&*literal!("' '")); __mm_s.push_str(&*resourcesDir.clone()); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone()) {
                        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to move ")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_info.json file")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    }
                }
            }
            (htmlFile, exportDocumentation) = exportHTMLDocumentation(program.clone(), simCode.clone(), (FMUVersion.clone()).clone())?;
            if exportDocumentation.clone() {
                Util::createDirectoryTree(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/documentation/")); ArcStr::from(__mm_s) }).clone())?;
                if 0 != System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("mv '")); __mm_s.push_str(&*htmlFile.clone()); __mm_s.push_str(&*literal!("' '")); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/documentation/")); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone()) {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to move documentation file ")); __mm_s.push_str(&*htmlFile.clone()); __mm_s.push_str(&*literal!("")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                }
            }
            SimCodeUtil::resetFunctionIndex()?;
            varInfo = simCode.modelInfo.varInfo.clone();
            install_include_omc_dir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/include/omc/")); ArcStr::from(__mm_s) }).clone();
            install_include_omc_c_dir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*install_include_omc_dir.clone()); __mm_s.push_str(&*literal!("c/")); ArcStr::from(__mm_s) }).clone();
            install_share_buildproject_dir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/share/omc/runtime/c/fmi/buildproject/")); ArcStr::from(__mm_s) }).clone();
            install_fmu_sources_dir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*arcstr::literal!(RuntimeSources::fmu_sources_dir)); ArcStr::from(__mm_s) }).clone();
            fmu_tmp_sources_dir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources/")); ArcStr::from(__mm_s) }).clone();
            copyFiles(RuntimeSources::simrt_c_headers.clone(), (install_include_omc_c_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            copyFiles(RuntimeSources::simrt_c_sources.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            copyFiles(RuntimeSources::dgesv_headers.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            copyFiles(RuntimeSources::dgesv_sources.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            dgesv_sources = RuntimeSources::dgesv_sources.clone();
            copyFiles(RuntimeSources::cminpack_headers.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            copyFiles(RuntimeSources::cminpack_sources.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            cminpack_sources = RuntimeSources::cminpack_sources.clone();
            if SimCodeUtil::cvodeFmiFlagIsSet(simCode.fmiSimulationFlags.clone())? {
                copyFiles(RuntimeSources::sundials_headers.clone(), (install_include_omc_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
                copyFiles(RuntimeSources::simrt_c_sundials_sources.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
                simrt_c_sundials_sources = RuntimeSources::simrt_c_sundials_sources.clone();
            } else {
                simrt_c_sundials_sources = metamodelica::nil();
            }
            simrt_linear_solver_sources = if (varInfo.numLinearSystems.clone() > 0) {RuntimeSources::simrt_linear_solver_sources.clone()} else {metamodelica::nil()};
            copyFiles(simrt_linear_solver_sources.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            simrt_non_linear_solver_sources = if (varInfo.numNonLinearSystems.clone() > 0) {RuntimeSources::simrt_non_linear_solver_sources.clone()} else {metamodelica::nil()};
            copyFiles(simrt_non_linear_solver_sources.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            simrt_mixed_solver_sources = if (varInfo.numMixedSystems.clone() > 0) {RuntimeSources::simrt_mixed_solver_sources.clone()} else {metamodelica::nil()};
            copyFiles(simrt_mixed_solver_sources.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            if FMUVersion.clone() == literal!("1.0") {
                copyFiles(RuntimeSources::fmi1Files.clone(), (install_include_omc_c_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
                fmi_export_files = RuntimeSources::fmi1Files.clone();
            } else {
                copyFiles(RuntimeSources::fmi2_sources.clone(), (install_include_omc_c_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
                copyFiles(RuntimeSources::fmi2_headers.clone(), (install_include_omc_c_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
                fmi_export_files = RuntimeSources::fmi2_sources.clone();
            }
            fmi2HeaderFiles = list![(literal!("fmi/fmi2Functions.h")).clone(), (literal!("fmi/fmi2FunctionTypes.h")).clone(), (literal!("fmi/fmi2TypesPlatform.h")).clone(), (literal!("fmi/fmiModelFunctions.h")).clone(), (literal!("fmi/fmiModelTypes.h")).clone()];
            copyFiles(fmi2HeaderFiles.clone(), (install_include_omc_c_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            copyFiles(RuntimeSources::modelica_external_c_sources.clone(), (install_include_omc_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            copyFiles(RuntimeSources::modelica_external_c_headers.clone(), (install_include_omc_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            modelica_standard_table_sources = RuntimeSources::modelica_external_c_sources.clone();
            System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources/isfmi")); __mm_s.push_str(&*if (FMUVersion.clone() == literal!("1.0")) {literal!("1")} else {literal!("2")}); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone())?;
            model_gen_files = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut f in (RuntimeSources::defaultFileSuffixes.clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*f.clone()); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            shared_source_files = List::flatten(list![fmi_export_files.clone(), RuntimeSources::simrt_c_sources.clone(), simrt_linear_solver_sources.clone(), simrt_non_linear_solver_sources.clone(), simrt_mixed_solver_sources.clone()])?;
            if !(Flags::getConfigBool(Flags::FMI_SOURCES.clone())?) || Flags::getConfigEnum(Flags::FMI_FILTER.clone())? == Flags::FMI_BLACKBOX.clone() {
                model_desc_src_files = metamodelica::nil();
            } else {
                model_desc_src_files = List::flatten(list![List::sort(model_gen_files.clone(), (std::sync::Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?, List::sort(shared_source_files.clone(), (std::sync::Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?, List::sort(dgesv_sources.clone(), (std::sync::Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?, List::sort(cminpack_sources.clone(), (std::sync::Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?, List::sort(simrt_c_sundials_sources.clone(), (std::sync::Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?, List::sort(modelica_standard_table_sources.clone(), (std::sync::Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?])?;
            }
            Tpl::tplNoret((std::sync::Arc::new({ let __pe_b2 = (FMUVersion.clone()).clone(); let __pe_b3 = (FMUType.clone()).clone(); let __pe_b4 = model_desc_src_files.clone(); move |__pe_a0, __pe_a1| CodegenFMU::translateModel(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), simCode.clone())?;
            model_all_gen_files = listAppend(model_gen_files.clone(), SimCodeUtil::getFunctionIndex());
            System::copyFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*install_share_buildproject_dir.clone()); __mm_s.push_str(&*literal!("CMakeLists.txt.in")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmu_tmp_sources_dir.clone()); __mm_s.push_str(&*literal!("CMakeLists.txt")); ArcStr::from(__mm_s) }).clone());
            cmakelistsStr = (System::readFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmu_tmp_sources_dir.clone()); __mm_s.push_str(&*literal!("CMakeLists.txt")); ArcStr::from(__mm_s) }).clone())?).clone();
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@FMU_NAME_HASH_IN@")).clone(), (fileNamePrefixHash.clone()).clone())?).clone();
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@FMU_NAME_IN@")).clone(), (simCode.fileNamePrefix.clone()).clone())?).clone();
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@FMU_TARGET_NAME@")).clone(), (simCode.fmuTargetName.clone()).clone())?).clone();
            if Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())? {
                cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@CMAKE_BUILD_TYPE@")).clone(), (literal!("Debug")).clone())?).clone();
            } else {
                cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@CMAKE_BUILD_TYPE@")).clone(), (literal!("Release")).clone())?).clone();
            }
            let () = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::FMU_RUNTIME_DEPENDS.clone())?) {
        Deref @ "default" => {
            let mut cmakeVersion: SemanticVersion::Version = <SemanticVersion::Version as ::std::default::Default>::default();
            let mut minimumVersion: SemanticVersion::Version = <SemanticVersion::Version as ::std::default::Default>::default();
            cmakeVersion = SimCodeUtil::getCMakeVersion((arcstr::literal!(Autoconf::cmake)).clone())?;
            minimumVersion = SemanticVersion::Version::SEMVER { major: 3, minor: 21, patch: 0, prerelease: metamodelica::nil(), meta: metamodelica::nil() };
            if SemanticVersion::compare(minimumVersion.clone(), cmakeVersion.clone(), true, false)? <= 0 {
                cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@RUNTIME_DEPENDENCIES_LEVEL@")).clone(), (literal!("\"modelica\"")).clone())?).clone();
            } else {
                cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@RUNTIME_DEPENDENCIES_LEVEL@")).clone(), (literal!("\"none\"")).clone())?).clone();
            }
            ()
        },
        Deref @ "none" => {
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@RUNTIME_DEPENDENCIES_LEVEL@")).clone(), (literal!("\"none\"")).clone())?).clone();
            ()
        },
        Deref @ "modelica" => {
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@RUNTIME_DEPENDENCIES_LEVEL@")).clone(), (literal!("\"modelica\"")).clone())?).clone();
            ()
        },
        Deref @ "all" => {
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@RUNTIME_DEPENDENCIES_LEVEL@")).clone(), (literal!("\"all\"")).clone())?).clone();
            ()
        },
        _ => {
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unsupported value ")); __mm_s.push_str(&*Flags::getConfigString(Flags::FMU_RUNTIME_DEPENDS.clone())?); __mm_s.push_str(&*literal!("for compiler flag 'fmuRuntimeDepends'.")); ArcStr::from(__mm_s) }).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@FMI_INTERFACE_HEADER_FILES_DIRECTORY@")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/include/omc/c/fmi")); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone())?).clone();
            (needCvode, cvodeDirectory) = SimCodeUtil::getCmakeSundialsLinkCode(simCode.fmiSimulationFlags.clone())?;
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@NEED_CVODE@")).clone(), (needCvode.clone()).clone())?).clone();
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@CVODE_DIRECTORY@")).clone(), (cvodeDirectory.clone()).clone())?).clone();
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@FMU_ADDITIONAL_LIBS@")).clone(), (SimCodeUtil::getCmakeLinkLibrariesCode(simCode.makefileParams.libs.clone())?).clone())?).clone();
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@FMU_ADDITIONAL_INCLUDES@")).clone(), (SimCodeUtil::make2CMakeInclude(simCode.makefileParams.includes.clone())).clone())?).clone();
            System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmu_tmp_sources_dir.clone()); __mm_s.push_str(&*literal!("CMakeLists.txt")); ArcStr::from(__mm_s) }).clone(), (cmakelistsStr.clone()).clone())?;
            modelDefinesHeaderStr = (System::readFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmu_tmp_sources_dir.clone()); __mm_s.push_str(&*literal!("fmi-export/fmu2_model_interface.c")); ArcStr::from(__mm_s) }).clone())?).clone();
            modelDefinesHeaderStr = (System::stringReplace((modelDefinesHeaderStr.clone()).clone(), (literal!("fmu2_dummy_model_defines.h")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("../")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_FMU.h")); ArcStr::from(__mm_s) }).clone())?).clone();
            System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmu_tmp_sources_dir.clone()); __mm_s.push_str(&*literal!("fmi-export/fmu2_model_interface.c")); ArcStr::from(__mm_s) }).clone(), (modelDefinesHeaderStr.clone()).clone())?;
            Tpl::closeFile(Tpl::tplCallWithFailErrorNoArg((std::sync::Arc::new({ let __pe_b1 = (Config::simulationCodeTarget()?).clone(); let __pe_b2 = simCode.clone(); let __pe_b3 = (FMUVersion.clone()).clone(); let __pe_b4 = model_all_gen_files.clone(); let __pe_b5 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut f in (shared_source_files.clone()).into_iter().cloned() {
            let __x = System::stringReplace((f.clone()).clone(), (literal!(".c")).clone(), (literal!(".o")).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }); let __pe_b6 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut f in (dgesv_sources.clone()).into_iter().cloned() {
            let __x = System::stringReplace((f.clone()).clone(), (literal!(".c")).clone(), (literal!(".o")).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }); let __pe_b7 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut f in (cminpack_sources.clone()).into_iter().cloned() {
            let __x = System::stringReplace((f.clone()).clone(), (literal!(".c")).clone(), (literal!(".o")).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }); let __pe_b8 = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut f in (simrt_c_sundials_sources.clone()).into_iter().cloned() {
            let __x = System::stringReplace((f.clone()).clone(), (literal!(".c")).clone(), (literal!(".o")).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }); move |__pe_a0| CodegenFMU::fmuMakefile(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_b8.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>), Tpl::redirectToFile(Tpl::emptyTxt.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources/Makefile.in")); ArcStr::from(__mm_s) }).clone())?)?)?;
            Tpl::closeFile(Tpl::tplCallWithFailError((std::sync::Arc::new(CodegenFMU::settingsfile) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), simCode.clone(), Tpl::redirectToFile(Tpl::emptyTxt.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources/omc_simulation_settings.h")); ArcStr::from(__mm_s) }).clone())?)?)?;
            if Config::simCodeTarget()? == literal!("omsicpp") {
                runTpl((std::sync::Arc::new({ let __pe_b1 = simCode.clone(); let __pe_b2 = (FMUVersion.clone()).clone(); let __pe_b3 = (FMUType.clone()).clone(); move |__pe_a0| CodegenOMSICpp::translateModel(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>));
            }
            ()
        },
        (_, Deref @ "omsic") => {
            let mut guid: ArcStr = arcstr::literal!("");
            let mut fileprefix: ArcStr = arcstr::literal!("");
            guid = (System::getUUIDStr()).clone();
            fileprefix = (simCode.fileNamePrefix.clone()).clone();
            if System::directoryExists((simCode.fullPathPrefix.clone()).clone()) {
                if !(System::removeDirectory((simCode.fullPathPrefix.clone()).clone())) {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to remove directory: ")); __mm_s.push_str(&*simCode.fullPathPrefix.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    bail!("fail");
                }
            }
            if !(System::createDirectory((simCode.fullPathPrefix.clone()).clone())) {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to create tmp folder ")); __mm_s.push_str(&*simCode.fullPathPrefix.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                System::fflush();
                bail!("fail");
            }
            SerializeInitXML::simulationInitFileReturnBool(simCode.clone(), (guid.clone()).clone());
            SerializeSparsityPattern::serialize(simCode.clone())?;
            SerializeModelInfo::serialize(simCode.clone(), Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?)?;
            runTpl((std::sync::Arc::new({ let __pe_b1 = simCode.clone(); let __pe_b2 = (guid.clone()).clone(); let __pe_b3 = (FMUVersion.clone()).clone(); let __pe_b4 = (FMUType.clone()).clone(); let __pe_b5 = metamodelica::nil(); let __pe_b6 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fullPathPrefix.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*literal!("modelDescription.xml")); ArcStr::from(__mm_s) }).clone(); move |__pe_a0| CodegenOMSI_common::generateFMUModelDescriptionFile(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>));
            runTplWriteFile((std::sync::Arc::new({ let __pe_b1 = simCode.clone(); let __pe_b2 = (Config::simulationCodeTarget()?).clone(); let __pe_b3 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fileprefix.clone()); __mm_s.push_str(&*literal!("_FMU.makefile")); ArcStr::from(__mm_s) }).clone(); move |__pe_a0| CodegenOMSIC::createMakefile(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fullPathPrefix.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*fileprefix.clone()); __mm_s.push_str(&*literal!("_FMU.makefile")); ArcStr::from(__mm_s) }).clone());
            runTplWriteFile((std::sync::Arc::new({ let __pe_b1 = simCode.clone(); move |__pe_a0| CodegenOMSIC::generateOMSIC(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fullPathPrefix.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*fileprefix.clone()); __mm_s.push_str(&*literal!("_omsic.c")); ArcStr::from(__mm_s) }).clone());
            runTpl((std::sync::Arc::new({ let __pe_b1 = simCode.clone(); let __pe_b2 = (fileprefix.clone()).clone(); move |__pe_a0| CodegenOMSI_common::generateEquationsCode(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text) -> Result<Tpl::Text> + 'static>));
            ()
        },
        (_, Deref @ "Cpp") => {
            if Flags::isSet(Flags::HPCOM.clone())? {
                Tpl::tplNoret3((std::sync::Arc::new(CodegenFMUCppHpcom::translateModel) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode, ArcStr, ArcStr) -> Result<Tpl::Text> + 'static>), simCode.clone(), (FMUVersion.clone()).clone(), (FMUType.clone()).clone())?;
            } else {
                Tpl::tplNoret((std::sync::Arc::new({ let __pe_b2 = (FMUVersion.clone()).clone(); let __pe_b3 = (FMUType.clone()).clone(); let __pe_b4 = metamodelica::nil(); move |__pe_a0, __pe_a1| CodegenFMUCpp::translateModel(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), simCode.clone())?;
            }
            ()
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown FMU template target: ")); __mm_s.push_str(&*target.clone()); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    { let __v = None; openmodelica_backend::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = __v) };
    Ok(())
}

fn exportHTMLDocumentation(mut program: Absyn::Program, mut simCode: SimCode::SimCode, mut FMUVersion: ArcStr) -> Result<(ArcStr, bool)> {
    let mut fileName: ArcStr = arcstr::literal!("");
    let mut export: bool = true;
    let mut file: File::File;
    let mut info: ArcStr = arcstr::literal!("");
    let mut revisions: ArcStr = arcstr::literal!("");
    let mut infoHeader: ArcStr = arcstr::literal!("");
    (info, revisions, infoHeader) = ProgramUtil::getNamedAnnotationExp(simCode.modelInfo.name.clone(), program.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("Documentation")).clone() }), Some((literal!(""), literal!(""), literal!(""))), (std::sync::Arc::new(Interactive::getDocumentationAnnotationString) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<(ArcStr, ArcStr, ArcStr)> + 'static>))?;
    if stringEmpty((info.clone()).clone()) && stringEmpty((revisions.clone()).clone()) && stringEmpty((infoHeader.clone()).clone()) {
        export = false;
    }
    if FMUVersion.clone() == literal!("1.0") {
        fileName = (literal!("_main.html")).clone();
    } else {
        fileName = (literal!("index.html")).clone();
    }
    file = File::File(File::noReference())?;
    File::open(file.clone(), (fileName.clone()).clone(), File::Mode::Write.clone());
    File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*infoHeader.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<h1>")); __mm_s.push_str(&*AbsynUtil::pathString(simCode.modelInfo.name.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("</h1>\n")); ArcStr::from(__mm_s) }).clone());
    File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<p> <i>")); __mm_s.push_str(&*simCode.modelInfo.description.clone()); __mm_s.push_str(&*literal!("</i> </p>\n")); ArcStr::from(__mm_s) }).clone());
    File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<h4> <u> Information </u> </h4>")); __mm_s.push_str(&*info.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<h4> <u> Revisions </u> </h4>")); __mm_s.push_str(&*revisions.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok((fileName, export))
}

fn callTargetTemplatesXML(mut simCode: SimCode::SimCode, mut target: ArcStr) -> Result<()> {
    Tpl::tplNoret((std::sync::Arc::new(CodegenXML::translateModel) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>), simCode.clone())?;
    Ok(())
}

pub fn translateModel(mut kind: TranslateModelKind, mut cache: FCore::Cache, mut inEnv: FCore::Graph, mut className: Arc<Absyn::Path>, mut inFileNamePrefix: ArcStr, mut runBackend: bool, mut useDAEMode: bool, mut runSilent: bool, mut inSimSettingsOpt: Option<SimCode::SimulationSettings>, mut args: Arc<Absyn::FunctionArgs>) -> Result<(bool, FCore::Cache, Arc<metamodelica::List<ArcStr>>, ArcStr, Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>>)> {
    let mut success: bool = false;
    let mut cache: FCore::Cache = cache;
    let mut outLibs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outFileDir: ArcStr = arcstr::literal!("");
    let mut resultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>> = metamodelica::nil();
    let mut inCache: FCore::Cache = cache.clone();
    let mut timeFrontend: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut env: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    let mut odae: Option<DAE::DAElist> = None;
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut allRoots: Arc<metamodelica::List<Option<i32>>> = metamodelica::nil();
    let mut flatModel: Arc<FlatModel::NFFlatModel> = Arc::new(<FlatModel::NFFlatModel as ::std::default::Default>::default());
    let mut funcTree: Arc<FunctionTreeImpl::Tree> = Arc::new(FunctionTreeImpl::Tree::EMPTY);
    let mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<NFFunction::Function::Function>>> = <Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<NFFunction::Function::Function>>> as ::std::default::Default>::default();
    let mut dumpValidFlatModelicaNF: bool = false;
    let mut flatString: ArcStr = literal!("");
    let mut NFFlatString: ArcStr = literal!("");
    FlagsUtil::setConfigBool(Flags::BUILDING_MODEL.clone(), true)?;
    outLibs = metamodelica::nil();
    outFileDir = (literal!("")).clone();
    resultValues = metamodelica::nil();
    dumpValidFlatModelicaNF = !(runSilent.clone()) && Config::flatModelica()?;
    if Flags::getConfigBool(Flags::NEW_BACKEND.clone())? {
        System::realtimeTick(ClockIndexes::RT_CLOCK_FRONTEND.clone())?;
        ExecStat::execStatReset()?;
        (flatModel, funcTree, NFFlatString) = CevalScriptBackend::runFrontEndWorkNF(className.clone(), false, dumpValidFlatModelicaNF.clone())?;
        timeFrontend = System::realtimeTock(ClockIndexes::RT_CLOCK_FRONTEND.clone())?;
        ExecStat::execStat((literal!("FrontEnd")).clone())?;
        if runBackend.clone() {
            funcMap = UnorderedMap::fromLists(FunctionTreeImpl::listKeys(funcTree.clone(), metamodelica::nil()), FunctionTreeImpl::listValues(funcTree.clone(), metamodelica::nil()), (std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>))?;
            (outLibs, outFileDir, resultValues, funcs) = translateModelCallBackendNB(flatModel.clone(), funcMap.clone(), className.clone(), (inFileNamePrefix.clone()).clone(), inSimSettingsOpt.clone())?;
        } else {
            funcs = NFConvertDAE::convertFunctionTree(funcTree.clone())?;
        }
        if dumpValidFlatModelicaNF.clone() {
            flatString = (NFFlatString.clone()).clone();
        } else if !(runSilent.clone()) {
            dae = NFConvertDAE::convertModel(flatModel.clone())?;
            flatString = (DAEDump::dumpStr(dae.clone(), funcs.clone())?).clone();
        }
    } else {
        System::realtimeTick(ClockIndexes::RT_CLOCK_FRONTEND.clone())?;
        ExecStat::execStatReset()?;
        (cache, env, odae, NFFlatString) = CevalScriptBackend::runFrontEnd(cache.clone(), inEnv.clone(), className.clone(), false, dumpValidFlatModelicaNF.clone(), false)?;
        ExecStat::execStat((literal!("FrontEnd")).clone())?;
        let __pa0 = ::match_deref::match_deref! { match &(odae.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        dae = __pa0.clone();
        if dumpValidFlatModelicaNF.clone() {
            flatString = (NFFlatString.clone()).clone();
        } else if !(runSilent.clone()) {
            funcs = FCore::getFunctionTree(cache.clone());
            flatString = (DAEDump::dumpStr(dae.clone(), funcs.clone())?).clone();
        }
        if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
            allRoots = metamodelica::nil();
            for mut i in 1..=300 {
                if '__try1: {
                    allRoots = metamodelica::cons(metamodelica::getGlobalRoot(i.clone())?, allRoots.clone());
                    Ok::<(), anyhow::Error>(())
                }.is_err() {
                }
            }
            serializeNotify(allRoots.clone(), (literal!("All local+global roots (1:300)")).clone())?;
            serializeNotify(dae.clone(), (literal!("FrontEnd DAE")).clone())?;
            serializeNotify((env.clone(), inEnv.clone(), cache.clone(), inCache.clone()), (literal!("FCore.Graph + Cache + Old graph + Old cache")).clone())?;
            serializeNotify((SymbolTable::get(), dae.clone(), env.clone(), inEnv.clone(), cache.clone(), inCache.clone()), (literal!("Symbol Table, DAE, Graph, OldGraph, Cache, OldCache")).clone())?;
            ExecStat::execStat((literal!("Serialize FrontEnd")).clone())?;
        }
        timeFrontend = System::realtimeTock(ClockIndexes::RT_CLOCK_FRONTEND.clone())?;
        if runBackend.clone() {
            if useDAEMode.clone() {
                (cache, outLibs, outFileDir, resultValues) = translateModelCallBackendOBDAEMode(cache.clone(), env.clone(), dae.clone(), className.clone(), (inFileNamePrefix.clone()).clone(), inSimSettingsOpt.clone(), args.clone())?;
            } else {
                (cache, outLibs, outFileDir, resultValues) = translateModelCallBackendOB(kind.clone(), cache.clone(), env.clone(), dae.clone(), className.clone(), (inFileNamePrefix.clone()).clone(), inSimSettingsOpt.clone(), args.clone())?;
            }
        }
    }
    resultValues = List::appendElt((literal!("timeFrontend"), Arc::new(Values::Value::REAL { real: timeFrontend.clone() })), resultValues.clone());
    FlagsUtil::setConfigBool(Flags::BUILDING_MODEL.clone(), false)?;
    if !(stringEmpty((flatString.clone()).clone())) && runSilent.clone() {
        Error::addInternalError((literal!("Flat model string generated but is not being dumped. Please make sure it is not generated if it is not shown.")).clone(), metamodelica::sourceInfo!())?;
    } else if stringEmpty((flatString.clone()).clone()) && !(runSilent.clone()) {
        Error::addInternalError((literal!("Flat model string generated but is empty.")).clone(), metamodelica::sourceInfo!())?;
    } else {
        metamodelica::print((flatString.clone()).clone());
    }
    success = true;
    Ok((success, cache, outLibs, outFileDir, resultValues))
}

pub fn translateModelCallBackend(mut flatModel: Arc<FlatModel::NFFlatModel>, mut functions: Arc<FunctionTreeImpl::Tree>, mut className: Arc<Absyn::Path>, mut fileNamePrefix: ArcStr, mut useDAEMode: bool, mut simSettings: Option<SimCode::SimulationSettings>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr, Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>>)> {
    let mut outLibs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outFileDir: ArcStr = arcstr::literal!("");
    let mut resultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>> = metamodelica::nil();
    let mut func_map: Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<NFFunction::Function::Function>>> = <Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<NFFunction::Function::Function>>> as ::std::default::Default>::default();
    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut dae_funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut env: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut file_name_prefix: ArcStr = arcstr::literal!("");
    file_name_prefix = (if (fileNamePrefix.clone() == literal!("<default>")) {AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?} else {fileNamePrefix.clone()}).clone();
    if Flags::getConfigBool(Flags::NEW_BACKEND.clone())? {
        func_map = UnorderedMap::fromLists(FunctionTreeImpl::listKeys(functions.clone(), metamodelica::nil()), FunctionTreeImpl::listValues(functions.clone(), metamodelica::nil()), (std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>))?;
        (outLibs, outFileDir, resultValues, _) = translateModelCallBackendNB(flatModel.clone(), func_map.clone(), className.clone(), (file_name_prefix.clone()).clone(), simSettings.clone())?;
    } else {
        dae = NFConvertDAE::convertModel(flatModel.clone())?;
        dae_funcs = NFConvertDAE::convertFunctionTree(functions.clone())?;
        env = FGraph::new((literal!("graph")).clone(), FCore::dummyTopModel.clone())?;
        cache = FCore::emptyCache();
        FCore::setCachedFunctionTree(cache.clone(), dae_funcs.clone());
        if useDAEMode.clone() {
            (cache, outLibs, outFileDir, resultValues) = translateModelCallBackendOBDAEMode(cache.clone(), env.clone(), dae.clone(), className.clone(), (file_name_prefix.clone()).clone(), simSettings.clone(), Absyn::emptyFunctionArgs.clone())?;
        } else {
            (cache, outLibs, outFileDir, resultValues) = translateModelCallBackendOB(crate::SimCodeMain::TranslateModelKind::NORMAL, cache.clone(), env.clone(), dae.clone(), className.clone(), (file_name_prefix.clone()).clone(), simSettings.clone(), Absyn::emptyFunctionArgs.clone())?;
        }
    }
    Ok((outLibs, outFileDir, resultValues))
}

fn simSettingsSimflags(mut inSimSettingsOpt: Option<SimCode::SimulationSettings>) -> Option<ArcStr> {
    let mut simflags: Option<ArcStr> = None;
    simflags = (match inSimSettingsOpt.clone() {
        Some(SimCode::SimulationSettings { simflags: mut s, .. }) => {
            Some((s.clone()).clone())
        },
        _ => {
            None
        },
    });
    simflags
}

fn translateModelCallBackendOB(mut kind: TranslateModelKind, mut cache: FCore::Cache, mut inEnv: FCore::Graph, mut inDae: DAE::DAElist, mut className: Arc<Absyn::Path>, mut inFileNamePrefix: ArcStr, mut inSimSettingsOpt: Option<SimCode::SimulationSettings>, mut args: Arc<Absyn::FunctionArgs>) -> Result<(FCore::Cache, Arc<metamodelica::List<ArcStr>>, ArcStr, Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>>)> {
    let mut cache: FCore::Cache = cache;
    let mut outLibs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outFileDir: ArcStr = arcstr::literal!("");
    let mut resultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>> = metamodelica::nil();
    let mut generateFunctions: bool = false;
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeBackend: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    FlagsUtil::setConfigBool(Flags::BUILDING_MODEL.clone(), true)?;
    (outLibs, outFileDir) = (match inEnv.clone() {
        mut graph => {
            let mut file_dir: ArcStr = arcstr::literal!("");
            let mut description: ArcStr = arcstr::literal!("");
            let mut fmuType: ArcStr = arcstr::literal!("");
            let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
            let mut dlow: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
            let mut initDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
            let mut initDAE_lambda0: Option<Arc<BackendDAE::BackendDAE>> = None;
            let mut inlineData: Option<BackendDAE::InlineData> = None;
            let mut removedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut strPreOptModules: Option<Arc<metamodelica::List<ArcStr>>> = None;
            let mut isFMI2: bool = false;
            let mut fmiDer: Arc<metamodelica::List<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))>> = metamodelica::nil();
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            System::realtimeTick(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
            dae = DAEUtil::transformationsBeforeBackend(cache.clone(), graph.clone(), inDae.clone(), (std::sync::Arc::new(StateMachineFlatten::stateMachineToDataFlow) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, DAE::DAElist) -> Result<DAE::DAElist> + 'static>))?;
            ExecStat::execStat((literal!("Transformations before backend")).clone())?;
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dae.clone(), (literal!("FrontEnd DAE after transformations")).clone())?;
                serializeNotify((dae.clone(), inDae.clone()), (literal!("FrontEnd DAE before+after transformations")).clone())?;
                ExecStat::execStat((literal!("Serialize DAE (2)")).clone())?;
            }
            GCExt::free(inDae.clone());
            generateFunctions = FlagsUtil::set(Flags::GEN.clone(), false)?;
            if !(Flags::isSet(Flags::BACKEND_KEEP_ENV_GRAPH.clone())?) {
                (cache, graph) = Builtin::initialGraph(cache.clone())?;
            }
            description = (DAEUtil::daeDescription(dae.clone())).clone();
            dlow = BackendDAECreate::lower(dae.clone(), cache.clone(), graph.clone(), BackendDAE::ExtraInfo { description: (description.clone()).clone(), fileNamePrefix: (inFileNamePrefix.clone()).clone(), simflags: simSettingsSimflags(inSimSettingsOpt.clone()) })?;
            GCExt::free(dae.clone());
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dlow.clone(), (literal!("BackendDAECreate.lower")).clone())?;
                ExecStat::execStat((literal!("Serialize dlow")).clone())?;
            }
            isFMI2 = (match kind.clone() {
        TranslateModelKind::FMU { kind: mut fmuType, .. } => FMI::isFMIVersion20((FMI::getFMIVersionString()?).clone())?,
        _ => false,
    });
            strPreOptModules = if (isFMI2.clone()) {Some(metamodelica::cons((literal!("introduceOutputAliases")).clone(), BackendDAEUtil::getPreOptModulesString()?))} else {None};
            if isFMI2.clone() && fmuType.clone() == literal!("cs") {
                strPreOptModules = Some(metamodelica::cons((literal!("introduceOutputRealDerivatives")).clone(), Util::getOption(strPreOptModules.clone())?));
            }
            (dlow, initDAE, initDAE_lambda0, inlineData, removedInitialEquationLst) = BackendDAEUtil::getSolvedSystem(dlow.clone(), (inFileNamePrefix.clone()).clone(), strPreOptModules.clone(), None, None, None)?;
            if isFMI2.clone() && !(Flags::isSet(Flags::FMI20_DEPENDENCIES.clone())?) {
                (fmiDer, funcs) = SymbolicJacobian::createFMIModelDerivatives(dlow.clone())?;
                dlow = BackendDAEUtil::setFunctionTree(dlow.clone(), funcs.clone())?;
            } else {
                fmiDer = metamodelica::nil();
            }
            timeBackend = System::realtimeTock(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dlow.clone(), (literal!("BackendDAE (simulation)")).clone())?;
                serializeNotify(initDAE.clone(), (literal!("BackendDAE (initialization)")).clone())?;
                serializeNotify(initDAE_lambda0.clone(), (literal!("BackendDAE (lambda0)")).clone())?;
                serializeNotify((dlow.clone(), initDAE.clone(), initDAE_lambda0.clone(), inlineData.clone(), removedInitialEquationLst.clone()), (literal!("BackendDAE (simulation+initialization+lambda0+inlineData+removedInitialEquationLst)")).clone())?;
                ExecStat::execStat((literal!("Serialize solved system")).clone())?;
            }
            (libs, file_dir, timeSimCode, timeTemplates) = (match kind.clone() {
        TranslateModelKind::NORMAL => {
            (libs, file_dir, timeSimCode, timeTemplates) = generateModelCode(dlow.clone(), initDAE.clone(), initDAE_lambda0.clone(), inlineData.clone(), removedInitialEquationLst.clone(), SymbolTable::getAbsyn(), className.clone(), (inFileNamePrefix.clone()).clone(), inSimSettingsOpt.clone(), args.clone(), fmiDer.clone())?;
            (libs.clone(), file_dir.clone(), timeSimCode.clone(), timeTemplates.clone())
        },
        TranslateModelKind::FMU { .. } => {
            (libs, file_dir, timeSimCode, timeTemplates) = generateModelCodeFMU(dlow.clone(), initDAE.clone(), initDAE_lambda0.clone(), fmiDer.clone(), removedInitialEquationLst.clone(), SymbolTable::getAbsyn(), className.clone(), (FMI::getFMIVersionString()?).clone(), (var_field!(kind.kind, TranslateModelKind::FMU).clone()).clone(), (inFileNamePrefix.clone()).clone(), (var_field!(kind.targetName, TranslateModelKind::FMU).clone()).clone(), inSimSettingsOpt.clone())?;
            (libs.clone(), file_dir.clone(), timeSimCode.clone(), timeTemplates.clone())
        },
        TranslateModelKind::XML => {
            (libs, file_dir, timeSimCode, timeTemplates) = generateModelCodeXML(dlow.clone(), initDAE.clone(), initDAE_lambda0.clone(), removedInitialEquationLst.clone(), SymbolTable::getAbsyn(), className.clone(), (inFileNamePrefix.clone()).clone(), inSimSettingsOpt.clone())?;
            (libs.clone(), file_dir.clone(), timeSimCode.clone(), timeTemplates.clone())
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown translateModel kind: ")); __mm_s.push_str(&*anyString(kind.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
            (libs.clone(), file_dir.clone())
        },
    });
    if generateFunctions.clone() {
        FlagsUtil::set(Flags::GEN.clone(), true)?;
    }
    resultValues = list![(literal!("timeTemplates"), Arc::new(Values::Value::REAL { real: timeTemplates.clone() })), (literal!("timeSimCode"), Arc::new(Values::Value::REAL { real: timeSimCode.clone() })), (literal!("timeBackend"), Arc::new(Values::Value::REAL { real: timeBackend.clone() }))];
    Ok((cache, outLibs, outFileDir, resultValues))
}

pub fn translateModelCallBackendOBDAEMode(mut cache: FCore::Cache, mut inEnv: FCore::Graph, mut inDae: DAE::DAElist, mut className: Arc<Absyn::Path>, mut inFileNamePrefix: ArcStr, mut inSimSettingsOpt: Option<SimCode::SimulationSettings>, mut args: Arc<Absyn::FunctionArgs>) -> Result<(FCore::Cache, Arc<metamodelica::List<ArcStr>>, ArcStr, Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>>)> {
    let mut cache: FCore::Cache = cache;
    let mut outLibs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outFileDir: ArcStr = arcstr::literal!("");
    let mut resultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>> = metamodelica::nil();
    let mut generateFunctions: bool = false;
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeBackend: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (outLibs, outFileDir) = 'mc: {
        let __mc_input = inEnv.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let mut graph = __mc_input.clone() else { bail!("nomatch") };
            let mut file_dir: ArcStr = arcstr::literal!("");
            let mut description: ArcStr = arcstr::literal!("");
            let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
            let mut dlow: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
            let mut initDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
            let mut initDAE_lambda0_option: Option<Arc<BackendDAE::BackendDAE>> = None;
            let mut removedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut cache: FCore::Cache = cache.clone();
            let mut generateFunctions: bool = generateFunctions.clone();
            let mut timeBackend: metamodelica::Real = timeBackend.clone();
            let mut timeSimCode: metamodelica::Real = timeSimCode.clone();
            let mut timeTemplates: metamodelica::Real = timeTemplates.clone();
            System::realtimeTick(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
            dae = DAEUtil::transformationsBeforeBackend(cache.clone(), graph.clone(), inDae.clone(), (std::sync::Arc::new(StateMachineFlatten::stateMachineToDataFlow) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, DAE::DAElist) -> Result<DAE::DAElist> + 'static>))?;
            ExecStat::execStat((literal!("Transformations before backend")).clone())?;
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dae.clone(), (literal!("dae2")).clone())?;
                ExecStat::execStat((literal!("Serialize DAE (2)")).clone())?;
            }
            GCExt::free(inDae.clone());
            generateFunctions = FlagsUtil::set(Flags::GEN.clone(), false)?;
            if !(Flags::isSet(Flags::BACKEND_KEEP_ENV_GRAPH.clone())?) {
                (cache, graph) = Builtin::initialGraph(cache.clone())?;
            }
            description = (DAEUtil::daeDescription(dae.clone())).clone();
            dlow = BackendDAECreate::lower(dae.clone(), cache.clone(), graph.clone(), BackendDAE::ExtraInfo { description: (description.clone()).clone(), fileNamePrefix: (inFileNamePrefix.clone()).clone(), simflags: simSettingsSimflags(inSimSettingsOpt.clone()) })?;
            GCExt::free(dae.clone());
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dlow.clone(), (literal!("dlow")).clone())?;
                ExecStat::execStat((literal!("Serialize dlow")).clone())?;
            }
            (dlow, initDAE, initDAE_lambda0_option, removedInitialEquationLst) = DAEMode::getEqSystemDAEmode(dlow.clone(), (inFileNamePrefix.clone()).clone(), None, None, None, None)?;
            ExecStat::execStat((literal!("Backend")).clone())?;
            timeBackend = System::realtimeTock(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dlow.clone(), (literal!("simDAE")).clone())?;
                serializeNotify(initDAE.clone(), (literal!("initDAE")).clone())?;
                serializeNotify(removedInitialEquationLst.clone(), (literal!("removedInitialEquationLst")).clone())?;
                ExecStat::execStat((literal!("Serialize solved system")).clone())?;
            }
            (libs, file_dir, timeSimCode, timeTemplates) = generateModelCodeDAE(dlow.clone(), initDAE.clone(), initDAE_lambda0_option.clone(), removedInitialEquationLst.clone(), SymbolTable::getAbsyn(), className.clone(), (inFileNamePrefix.clone()).clone(), inSimSettingsOpt.clone(), args.clone())?;
            timeSimCode = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
            timeTemplates = System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
            Ok(((libs.clone(), file_dir.clone()), cache.clone()))
        })() { cache = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut resstr: ArcStr = arcstr::literal!("");
            resstr = AbsynUtil::pathStringNoQual(className.clone(), (literal!(".")).clone(), true, false)?;
            resstr = stringAppendList(list![(literal!("SimCode DAEmode: The model ")).clone(), (resstr.clone()).clone(), (literal!(" could not be translated")).clone()]);
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(resstr.clone()).clone()])?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    if generateFunctions.clone() {
        FlagsUtil::set(Flags::GEN.clone(), true)?;
    }
    resultValues = list![(literal!("timeTemplates"), Arc::new(Values::Value::REAL { real: timeTemplates.clone() })), (literal!("timeSimCode"), Arc::new(Values::Value::REAL { real: timeSimCode.clone() })), (literal!("timeBackend"), Arc::new(Values::Value::REAL { real: timeBackend.clone() }))];
    Ok((cache, outLibs, outFileDir, resultValues))
}

fn translateModelCallBackendNB(mut inFlatModel: Arc<FlatModel::NFFlatModel>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<NFFunction::Function::Function>>>, mut inClassName: Arc<Absyn::Path>, mut inFileNamePrefix: ArcStr, mut inSimSettingsOpt: Option<SimCode::SimulationSettings>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr, Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>>, Arc<AvlTreePathFunction::Tree>)> {
    let mut outLibs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outFileDir: ArcStr = arcstr::literal!("");
    let mut resultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>> = metamodelica::nil();
    let mut oldFunctionTree: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeBackend: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut bdae: Arc<NBackendDAE::NBackendDAE> = Arc::new(<NBackendDAE::NBackendDAE as ::std::default::Default>::default());
    let mut nf_api: bool = false;
    FlagsUtil::setConfigBool(Flags::BUILDING_MODEL.clone(), true)?;
    nf_api = FlagsUtil::set(Flags::NF_API.clone(), false)?;
    System::realtimeTick(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
    bdae = NBackendDAE::lower(inFlatModel.clone(), funcMap.clone())?;
    if Flags::isSet(Flags::OPT_DAE_DUMP.clone())? {
        metamodelica::print((NBackendDAE::toString(bdae.clone(), (literal!("(After Lowering)")).clone())?).clone());
    }
    bdae = NBackendDAE::main(bdae.clone())?;
    timeBackend = System::realtimeTock(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
    ExecStat::execStat((literal!("backend")).clone())?;
    FlagsUtil::set(Flags::NF_API.clone(), nf_api.clone())?;
    (outLibs, outFileDir, timeSimCode, timeTemplates, oldFunctionTree) = generateModelCodeNewBackend(bdae.clone(), inClassName.clone(), (inFileNamePrefix.clone()).clone(), inSimSettingsOpt.clone())?;
    resultValues = list![(literal!("timeTemplates"), Arc::new(Values::Value::REAL { real: timeTemplates.clone() })), (literal!("timeSimCode"), Arc::new(Values::Value::REAL { real: timeSimCode.clone() })), (literal!("timeBackend"), Arc::new(Values::Value::REAL { real: timeBackend.clone() }))];
    Ok((outLibs, outFileDir, resultValues, oldFunctionTree))
}

fn generateModelCodeDAE(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE: Arc<BackendDAE::BackendDAE>, mut initDAE_lambda0_option: Option<Arc<BackendDAE::BackendDAE>>, mut inRemovedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut p: Absyn::Program, mut className: Arc<Absyn::Path>, mut filenamePrefix: ArcStr, mut simSettingsOpt: Option<SimCode::SimulationSettings>, mut args: Arc<Absyn::FunctionArgs>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr, metamodelica::Real, metamodelica::Real)> {
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut fileDir: ArcStr = arcstr::literal!("");
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let debug: bool = false;
    let mut includes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut libPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
    let mut simCode: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
    let mut recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
    let mut a_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut literals: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>);
    let mut lits: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut numCheckpoints: i32 = 0;
    let mut tempVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut emptyBDAE: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut initDAE_lambda0: Arc<BackendDAE::BackendDAE> = Arc::new(<BackendDAE::BackendDAE as ::std::default::Default>::default());
    let mut modelInfo: SimCode::ModelInfo = <SimCode::ModelInfo as ::std::default::Default>::default();
    let mut extObjInfo: SimCode::ExtObjInfo = <SimCode::ExtObjInfo as ::std::default::Default>::default();
    let mut crefToSimVarHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr));
    let mut makefileParams: SimCodeFunction::MakefileParams = <SimCodeFunction::MakefileParams as ::std::default::Default>::default();
    let mut spatialInfo: SimCode::SpatialDistributionInfo = <SimCode::SpatialDistributionInfo as ::std::default::Default>::default();
    let mut delayedExps: Arc<metamodelica::List<(i32, (Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::Exp>))>> = metamodelica::nil();
    let mut maxDelayedExpIndex: i32 = 0;
    let mut uniqueEqIndex: i32 = 1;
    let mut tmpB: bool = false;
    let mut varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
    let mut varToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut crefToClockIndexHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    let mut discreteModelVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut timeEvents: Arc<metamodelica::List<BackendDAE::TimeEvent>> = metamodelica::nil();
    let mut zeroCrossingsSet: BackendDAE::ZeroCrossingSet = <BackendDAE::ZeroCrossingSet as ::std::default::Default>::default();
    let mut sampleZCSet: BackendDAE::ZeroCrossingSet = <BackendDAE::ZeroCrossingSet as ::std::default::Default>::default();
    let mut de_relations: DoubleEnded::MutableList<BackendDAE::ZeroCrossing> = <DoubleEnded::MutableList<BackendDAE::ZeroCrossing> as ::std::default::Default>::default();
    let mut zeroCrossings: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
    let mut sampleZC: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
    let mut relations: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
    let mut daeVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut resVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut algStateVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut auxVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut varsLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut daeModeSP: Option<Arc<SimCode::JacobianMatrix>> = None;
    let mut daeModeData: Option<SimCode::DaeModeData> = None;
    let mut daeModeConf: SimCode::DaeModeConfig = SimCode::DaeModeConfig::ALL_EQUATIONS;
    let mut matrixnames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut daeEquations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>> = metamodelica::nil();
    let mut residualVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut algebraicStateVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut auxiliaryVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut daeModeJacobian: (Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32)) = (None, (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), 0), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), 0));
    let mut daeModeJac: Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> = None;
    let mut jacH: Option<Arc<BackendDAE::Jacobian>> = None;
    let mut daeModeSparsity: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32) = (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), 0);
    let mut daeModeColoring: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
    let mut nonlinearPattern: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32) = (metamodelica::nil(), metamodelica::nil(), (metamodelica::nil(), metamodelica::nil()), 0);
    let mut symDAESparsPattern: Arc<SimCode::JacobianMatrix> = Arc::new(<SimCode::JacobianMatrix as ::std::default::Default>::default());
    let mut symJacs: Arc<metamodelica::List<Arc<SimCode::JacobianMatrix>>> = metamodelica::nil();
    let mut SymbolicJacs: Arc<metamodelica::List<Arc<SimCode::JacobianMatrix>>> = metamodelica::nil();
    let mut SymbolicJacsNLS: Arc<metamodelica::List<Arc<SimCode::JacobianMatrix>>> = metamodelica::nil();
    let mut SymbolicJacsTemp: Arc<metamodelica::List<Arc<SimCode::JacobianMatrix>>> = metamodelica::nil();
    let mut initialEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut initialEquations_lambda0: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut removedInitialEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut jacobianSimvars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut seedVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut startValueEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut maxValueEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut minValueEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut nominalValueEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut parameterEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut jacobianEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    numCheckpoints = ErrorExt::getNumCheckpoints();
    StackOverflow::clearStacktraceMessages();
    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    a_cref = AbsynUtil::pathToCref(className.clone())?;
    fileDir = (ProgramUtil::getFileDir(a_cref.clone(), p.clone())?).clone();
    (libs, libPaths, includes, includeDirs, recordDecls, functions, literals) = SimCodeUtilShared::createFunctions(p.clone(), inBackendDAE.shared.functionTree.clone())?;
    extObjInfo = SimCodeUtil::createExtObjInfo(inBackendDAE.shared.clone())?;
    makefileParams = SimCodeFunctionUtil::createMakefileParams(includeDirs.clone(), libs.clone(), libPaths.clone(), false, false)?;
    (delayedExps, maxDelayedExpIndex) = SimCodeUtil::extractDelayedExpressions(inBackendDAE.clone())?;
    spatialInfo = SimCodeUtil::extractSpatialDistributionInfo(inBackendDAE.clone())?;
    timeEvents = inBackendDAE.shared.eventInfo.timeEvents.clone();
    (zeroCrossings, relations, sampleZC) = (match inBackendDAE.shared.eventInfo.clone() {
        BackendDAE::EventInfo { samples: mut sampleZCSet, relations: mut de_relations, zeroCrossings: mut zeroCrossingsSet, .. } => (ZeroCrossings::toList(zeroCrossingsSet.clone()), DoubleEnded::toListNoCopyNoClear(de_relations.clone()), ZeroCrossings::toList(sampleZCSet.clone())),
    });
    (initialEquations, uniqueEqIndex, tempVars) = SimCodeUtil::createInitialEquations(inInitDAE.clone(), uniqueEqIndex.clone(), tempVars.clone())?;
    if isSome(initDAE_lambda0_option.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(initDAE_lambda0_option.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        initDAE_lambda0 = __pa0.clone();
        (initialEquations_lambda0, uniqueEqIndex, tempVars) = SimCodeUtil::createInitialEquations_lambda0(initDAE_lambda0.clone(), uniqueEqIndex.clone(), tempVars.clone())?;
    } else {
        initialEquations_lambda0 = metamodelica::nil();
    }
    let (__pa1, (__pa2, _), __pa3) = SimCodeUtil::createNonlinearResidualEquations(inRemovedInitialEquationLst.clone(), (uniqueEqIndex.clone(), 0), tempVars.clone(), inBackendDAE.shared.functionTree.clone())?;
    removedInitialEquations = __pa1.clone();
    uniqueEqIndex = __pa2.clone();
    tempVars = __pa3.clone();
    ExecStat::execStat((literal!("simCode: created initialization part")).clone())?;
    (uniqueEqIndex, startValueEquations, _) = BackendDAEUtil::foldEqSystem(inInitDAE.clone(), (std::sync::Arc::new(SimCodeUtil::createStartValueEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, BackendDAE::Variables)) -> Result<(i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, BackendDAE::Variables)> + 'static>), (uniqueEqIndex.clone(), metamodelica::nil(), inBackendDAE.shared.globalKnownVars.clone()))?;
    if debug.clone() {
        ExecStat::execStat((literal!("simCode: createStartValueEquations")).clone())?;
    }
    (uniqueEqIndex, nominalValueEquations) = SimCodeUtil::createValueEquationsShared(inBackendDAE.shared.clone(), (std::sync::Arc::new(SimCodeUtil::createInitialAssignmentsFromNominal) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, BackendDAE::Variables)) -> Result<(BackendDAE::Var, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, BackendDAE::Variables))> + 'static>), (uniqueEqIndex.clone(), nominalValueEquations.clone()))?;
    if debug.clone() {
        ExecStat::execStat((literal!("simCode: createNominalValueEquationsShared")).clone())?;
    }
    (uniqueEqIndex, nominalValueEquations) = BackendDAEUtil::foldEqSystem(inBackendDAE.clone(), (std::sync::Arc::new(SimCodeUtil::createNominalValueEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)) -> Result<(i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)> + 'static>), (uniqueEqIndex.clone(), nominalValueEquations.clone()))?;
    if debug.clone() {
        ExecStat::execStat((literal!("simCode: createNominalValueEquations")).clone())?;
    }
    (uniqueEqIndex, minValueEquations) = SimCodeUtil::createValueEquationsShared(inBackendDAE.shared.clone(), (std::sync::Arc::new(SimCodeUtil::createInitialAssignmentsFromMin) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, BackendDAE::Variables)) -> Result<(BackendDAE::Var, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, BackendDAE::Variables))> + 'static>), (uniqueEqIndex.clone(), minValueEquations.clone()))?;
    if debug.clone() {
        ExecStat::execStat((literal!("simCode: createMinValueEquationsShared")).clone())?;
    }
    (uniqueEqIndex, minValueEquations) = BackendDAEUtil::foldEqSystem(inBackendDAE.clone(), (std::sync::Arc::new(SimCodeUtil::createMinValueEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)) -> Result<(i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)> + 'static>), (uniqueEqIndex.clone(), minValueEquations.clone()))?;
    if debug.clone() {
        ExecStat::execStat((literal!("simCode: createMinValueEquations")).clone())?;
    }
    (uniqueEqIndex, maxValueEquations) = SimCodeUtil::createValueEquationsShared(inBackendDAE.shared.clone(), (std::sync::Arc::new(SimCodeUtil::createInitialAssignmentsFromMax) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, BackendDAE::Variables)) -> Result<(BackendDAE::Var, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, BackendDAE::Variables))> + 'static>), (uniqueEqIndex.clone(), maxValueEquations.clone()))?;
    if debug.clone() {
        ExecStat::execStat((literal!("simCode: createMaxValueEquationsShared")).clone())?;
    }
    (uniqueEqIndex, maxValueEquations) = BackendDAEUtil::foldEqSystem(inBackendDAE.clone(), (std::sync::Arc::new(SimCodeUtil::createMaxValueEquations) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, (i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)) -> Result<(i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)> + 'static>), (uniqueEqIndex.clone(), maxValueEquations.clone()))?;
    if debug.clone() {
        ExecStat::execStat((literal!("simCode: createMaxValueEquations")).clone())?;
    }
    (uniqueEqIndex, parameterEquations, _) = SimCodeUtil::createParameterEquations(uniqueEqIndex.clone(), parameterEquations.clone(), inBackendDAE.shared.globalKnownVars.clone())?;
    if debug.clone() {
        ExecStat::execStat((literal!("simCode: createParameterEquations")).clone())?;
    }
    discreteModelVars = BackendDAEUtil::foldEqSystem(inBackendDAE.clone(), (std::sync::Arc::new(SimCodeUtil::extractDiscreteModelVars) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>), metamodelica::nil())?;
    (daeEquations, uniqueEqIndex, tempVars) = SimCodeUtil::createEquationsfromBackendDAE(inBackendDAE.clone(), uniqueEqIndex.clone(), tempVars.clone(), true, true, false, false)?;
    emptyBDAE = Arc::new(BackendDAE::BackendDAE { eqs: metamodelica::cons(BackendDAEUtil::createEqSystem(Util::getOption(inBackendDAE.shared.daeModeData.modelVars.clone())?, BackendEquation::emptyEqns(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns()), metamodelica::nil()), shared: inBackendDAE.shared.clone() });
    if Flags::getConfigString(Flags::GENERATE_DYNAMIC_JACOBIAN.clone())? == literal!("symbolic") {
        (daeModeJac, daeModeSparsity, daeModeColoring, nonlinearPattern) = (inBackendDAE.shared.symjacs.clone()).get(BackendDAE::SymbolicJacobianAIndex.clone())?;
        if isSome(inBackendDAE.shared.dataReconciliationData.clone()) {
            let BackendDAE::DATA_RECON { symbolicJacobian: _, setcVars: _, datareconinputs: _, setBVars: _, symbolicJacobianH: __pa4, .. } = (Util::getOption(inBackendDAE.shared.dataReconciliationData.clone())?) else { bail!("pattern mismatch") };
            jacH = __pa4.clone();
            if isSome(jacH.clone()) {
                matrixnames = list![(literal!("B")).clone(), (literal!("C")).clone(), (literal!("D")).clone(), (literal!("ADJ")).clone()];
            } else {
                matrixnames = list![(literal!("B")).clone(), (literal!("C")).clone(), (literal!("D")).clone(), (literal!("H")).clone(), (literal!("ADJ")).clone()];
            }
        } else {
            matrixnames = list![(literal!("B")).clone(), (literal!("C")).clone(), (literal!("D")).clone(), (literal!("F")).clone(), (literal!("H")).clone(), (literal!("ADJ")).clone()];
        }
        (daeModeSP, uniqueEqIndex, tempVars) = SimCodeUtil::createSymbolicSimulationJacobian(Arc::new(BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: daeModeJac.clone(), sparsePattern: daeModeSparsity.clone(), coloring: daeModeColoring.clone(), nonlinearPattern: nonlinearPattern.clone() }), uniqueEqIndex.clone(), tempVars.clone(), false)?;
        tmpB = FlagsUtil::set(Flags::NO_START_CALC.clone(), true)?;
        modelInfo = SimCodeUtil::createModelInfo(className.clone(), p.clone(), emptyBDAE.clone(), inInitDAE.clone(), functions.clone(), metamodelica::nil(), 0, spatialInfo.maxIndex.clone(), (fileDir.clone()).clone(), 0, tempVars.clone())?;
        FlagsUtil::set(Flags::NO_START_CALC.clone(), tmpB.clone())?;
        crefToSimVarHT = SimCodeUtil::createCrefToSimVarHT(modelInfo.clone())?;
        (symJacs, uniqueEqIndex) = SimCodeUtil::createSymbolicJacobianssSimCode(metamodelica::nil(), crefToSimVarHT.clone(), uniqueEqIndex.clone(), matrixnames.clone(), metamodelica::nil())?;
        symJacs = metamodelica::cons(Util::getOption(daeModeSP.clone())?, symJacs.clone()).reverse();
    } else {
        tmpB = FlagsUtil::set(Flags::NO_START_CALC.clone(), true)?;
        modelInfo = SimCodeUtil::createModelInfo(className.clone(), p.clone(), emptyBDAE.clone(), inInitDAE.clone(), functions.clone(), metamodelica::nil(), 0, spatialInfo.maxIndex.clone(), (fileDir.clone()).clone(), 0, tempVars.clone())?;
        FlagsUtil::set(Flags::NO_START_CALC.clone(), tmpB.clone())?;
        crefToSimVarHT = SimCodeUtil::createCrefToSimVarHT(modelInfo.clone())?;
        if isSome(inBackendDAE.shared.dataReconciliationData.clone()) {
            let BackendDAE::DATA_RECON { symbolicJacobian: _, setcVars: _, datareconinputs: _, setBVars: _, symbolicJacobianH: __pa5, .. } = (Util::getOption(inBackendDAE.shared.dataReconciliationData.clone())?) else { bail!("pattern mismatch") };
            jacH = __pa5.clone();
            if isSome(jacH.clone()) {
                matrixnames = list![(literal!("A")).clone(), (literal!("B")).clone(), (literal!("C")).clone(), (literal!("D")).clone(), (literal!("ADJ")).clone()];
            } else {
                matrixnames = list![(literal!("A")).clone(), (literal!("B")).clone(), (literal!("C")).clone(), (literal!("D")).clone(), (literal!("H")).clone(), (literal!("ADJ")).clone()];
            }
        } else {
            matrixnames = list![(literal!("A")).clone(), (literal!("B")).clone(), (literal!("C")).clone(), (literal!("D")).clone(), (literal!("F")).clone(), (literal!("H")).clone(), (literal!("ADJ")).clone()];
        }
        (symJacs, uniqueEqIndex) = SimCodeUtil::createSymbolicJacobianssSimCode(metamodelica::nil(), crefToSimVarHT.clone(), uniqueEqIndex.clone(), matrixnames.clone(), metamodelica::nil())?;
    }
    SymbolicJacsNLS = metamodelica::nil();
    (initialEquations, modelInfo, SymbolicJacsTemp) = SimCodeUtil::addAlgebraicLoopsModelInfo(initialEquations.clone(), modelInfo.clone())?;
    SymbolicJacsNLS = listAppend(SymbolicJacsTemp.clone(), SymbolicJacsNLS.clone());
    (initialEquations_lambda0, modelInfo, SymbolicJacsTemp) = SimCodeUtil::addAlgebraicLoopsModelInfo(initialEquations_lambda0.clone(), modelInfo.clone())?;
    SymbolicJacsNLS = listAppend(SymbolicJacsTemp.clone(), SymbolicJacsNLS.clone());
    (parameterEquations, modelInfo, SymbolicJacsTemp) = SimCodeUtil::addAlgebraicLoopsModelInfo(parameterEquations.clone(), modelInfo.clone())?;
    SymbolicJacsNLS = listAppend(SymbolicJacsTemp.clone(), SymbolicJacsNLS.clone());
    (SymbolicJacs, modelInfo, SymbolicJacsTemp) = SimCodeUtil::addAlgebraicLoopsModelInfoSymJacs(symJacs.clone(), modelInfo.clone());
    jacobianEquations = SimCodeUtil::collectAllJacobianEquations(SymbolicJacs.clone())?;
    if debug.clone() {
        ExecStat::execStat((literal!("simCode: create Jacobian linear code")).clone())?;
    }
    SymbolicJacs = listAppend(SymbolicJacsNLS.clone().reverse(), listAppend(SymbolicJacs.clone(), SymbolicJacsTemp.clone()));
    jacobianSimvars = SimCodeUtil::collectAllJacobianVars(SymbolicJacs.clone())?;
    modelInfo = SimCodeUtil::setJacobianVars(jacobianSimvars.clone(), modelInfo.clone());
    crefToSimVarHT = List::fold(jacobianSimvars.clone(), (std::sync::Arc::new(HashTableCrefSimVar::addSimVarToHashTable) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar) -> Result<ArcStr> + 'static>))> + 'static>), crefToSimVarHT.clone())?;
    seedVars = SimCodeUtil::collectAllSeedVars(SymbolicJacs.clone())?;
    modelInfo = SimCodeUtil::setSeedVars(seedVars.clone(), modelInfo.clone());
    crefToSimVarHT = List::fold(seedVars.clone(), (std::sync::Arc::new(HashTableCrefSimVar::addSimVarToHashTable) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar) -> Result<ArcStr> + 'static>))> + 'static>), crefToSimVarHT.clone())?;
    varsLst = BackendVariable::equationSystemsVarsLst(inBackendDAE.eqs.clone())?;
    daeVars = BackendVariable::listVar(varsLst.clone())?;
    (_, resVars) = BackendVariable::traverseBackendDAEVars(daeVars.clone(), (std::sync::Arc::new(BackendVariable::collectVarKindVarinVariables) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>, BackendDAE::Variables)) -> Result<(BackendDAE::Var, (Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>, BackendDAE::Variables))> + 'static>), ((std::sync::Arc::new(fnptr!(BackendVariable::isDAEmodeResVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
    (residualVars, _) = BackendVariable::traverseBackendDAEVars(resVars.clone(), (std::sync::Arc::new(SimCodeUtil::traversingdlowvarToSimvar) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (Arc<metamodelica::List<SimCodeVar::SimVar>>, BackendDAE::Variables)) -> Result<(BackendDAE::Var, (Arc<metamodelica::List<SimCodeVar::SimVar>>, BackendDAE::Variables))> + 'static>), (metamodelica::nil(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
    (residualVars, _) = SimCodeUtil::rewriteIndex(residualVars.clone(), 0);
    (residualVars, _, _) = SimCodeUtil::setVariableIndexHelper(residualVars.clone(), 0, 0)?;
    crefToSimVarHT = List::fold(residualVars.clone(), (std::sync::Arc::new(HashTableCrefSimVar::addSimVarToHashTable) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar) -> Result<ArcStr> + 'static>))> + 'static>), crefToSimVarHT.clone())?;
    (_, auxVars) = BackendVariable::traverseBackendDAEVars(daeVars.clone(), (std::sync::Arc::new(BackendVariable::collectVarKindVarinVariables) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>, BackendDAE::Variables)) -> Result<(BackendDAE::Var, (Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>, BackendDAE::Variables))> + 'static>), ((std::sync::Arc::new(fnptr!(BackendVariable::isDAEmodeAuxVar, BackendDAE::Var)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var) -> Result<bool> + 'static>), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
    (auxiliaryVars, _) = BackendVariable::traverseBackendDAEVars(auxVars.clone(), (std::sync::Arc::new(SimCodeUtil::traversingdlowvarToSimvar) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (Arc<metamodelica::List<SimCodeVar::SimVar>>, BackendDAE::Variables)) -> Result<(BackendDAE::Var, (Arc<metamodelica::List<SimCodeVar::SimVar>>, BackendDAE::Variables))> + 'static>), (metamodelica::nil(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
    auxiliaryVars = List::sort(auxiliaryVars.clone(), (std::sync::Arc::new(SimCodeUtil::simVarCompareByCrefSubsAtEndlLexical) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar, SimCodeVar::SimVar) -> Result<bool> + 'static>))?;
    (auxiliaryVars, _) = SimCodeUtil::rewriteIndex(auxiliaryVars.clone(), 0);
    (auxiliaryVars, _, _) = SimCodeUtil::setVariableIndexHelper(auxiliaryVars.clone(), 0, 0)?;
    crefToSimVarHT = List::fold(auxiliaryVars.clone(), (std::sync::Arc::new(HashTableCrefSimVar::addSimVarToHashTable) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar) -> Result<ArcStr> + 'static>))> + 'static>), crefToSimVarHT.clone())?;
    algStateVars = BackendVariable::listVar(inBackendDAE.shared.daeModeData.algStateVars.clone())?;
    (algebraicStateVars, _) = BackendVariable::traverseBackendDAEVars(algStateVars.clone(), (std::sync::Arc::new(SimCodeUtil::traversingdlowvarToSimvar) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (Arc<metamodelica::List<SimCodeVar::SimVar>>, BackendDAE::Variables)) -> Result<(BackendDAE::Var, (Arc<metamodelica::List<SimCodeVar::SimVar>>, BackendDAE::Variables))> + 'static>), (metamodelica::nil(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())))?;
    algebraicStateVars = SimCodeUtil::sortSimVarsAndWriteIndex(algebraicStateVars.clone(), crefToSimVarHT.clone())?;
    daeModeJacobian = (inBackendDAE.shared.symjacs.clone()).get(BackendDAE::SymbolicJacobianAIndex.clone())?;
    let (__pa6, __pa7) = ::match_deref::match_deref! { match &(SimCodeUtil::createSymbolicJacobianssSimCode(list![daeModeJacobian.clone()], crefToSimVarHT.clone(), uniqueEqIndex.clone(), list![(literal!("daeMode")).clone()], metamodelica::nil())?) {
        (Deref @ metamodelica::List::Cons { head: __pa6, tail: Deref @ metamodelica::List::Nil }, __pa7) => (__pa6.clone(), __pa7.clone()),
        _ => bail!("pattern mismatch"),
    } };
    symDAESparsPattern = __pa6.clone();
    uniqueEqIndex = __pa7.clone();
    daeModeSP = Some(symDAESparsPattern.clone());
    if Flags::getConfigString(Flags::GENERATE_DYNAMIC_JACOBIAN.clone())? == literal!("symbolic") {
        SymbolicJacs = ({
        let mut __acc: Arc<metamodelica::List<Arc<SimCode::JacobianMatrix>>> = metamodelica::nil();
        for mut symjac in (SymbolicJacs.clone()).into_iter().cloned() {
            let __x = SimCodeUtil::syncDAEandSimJac(symjac.clone(), symDAESparsPattern.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    }
    daeModeConf = openmodelica_simcode_types::SimCode::DaeModeConfig::ALL_EQUATIONS;
    daeModeData = Some(SimCode::DaeModeData { daeEquations: daeEquations.clone(), sparsityPattern: daeModeSP.clone(), residualVars: residualVars.clone(), algebraicVars: algebraicStateVars.clone(), auxiliaryVars: auxiliaryVars.clone(), modeCreated: daeModeConf.clone() });
    modelInfo = SimCodeUtil::addNumEqns(modelInfo.clone(), uniqueEqIndex.clone() - (jacobianEquations.clone().len() as i32));
    if stringEqual((Config::simCodeTarget()?).clone(), (literal!("Cpp")).clone()) {
        (varToArrayIndexMapping, varToIndexMapping) = SimCodeUtilShared::createVarToArrayIndexMapping(modelInfo.clone())?;
        (crefToClockIndexHT, _) = List::fold(inBackendDAE.eqs.clone().reverse(), (std::sync::Arc::new(SimCodeUtil::collectClockedVars) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, ((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), i32)) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), i32)> + 'static>), (HashTable::emptyHashTable(), 1))?;
    } else {
        varToArrayIndexMapping = HashTableCrIListArray::emptyHashTable();
        varToIndexMapping = HashTableCrILst::emptyHashTable();
        crefToClockIndexHT = HashTable::emptyHashTable();
    }
    simCode = SimCode::SimCode { scalarized: true, omsiData: None, inlineEquations: metamodelica::nil(), daeModeData: daeModeData.clone(), partitionData: SimCode::emptyPartitionData.clone(), fmiSimulationFlags: None, modelStructure: None, backendMapping: None, crefToClockIndexHT: crefToClockIndexHT.clone(), crefToSimVarHT: crefToSimVarHT.clone(), varToIndexMapping: varToIndexMapping.clone(), varToArrayIndexMapping: varToArrayIndexMapping.clone(), valueReferences: Arc::new(openmodelica_simcode_types::AvlTreeCRToInt::Tree::EMPTY), hpcomData: HpcOmSimCode::emptyHpcomData().clone(), fmuTargetName: (literal!("")).clone(), fullPathPrefix: (literal!("")).clone(), fileNamePrefix: (filenamePrefix.clone()).clone(), simulationSettingsOpt: simSettingsOpt.clone(), jacobianMatrices: SymbolicJacs.clone(), spatialInfo: spatialInfo.clone(), delayedExps: SimCode::DelayedExpression { delayedExps: delayedExps.clone(), maxDelayedIndex: maxDelayedExpIndex.clone() }, makefileParams: makefileParams.clone(), extObjInfo: extObjInfo.clone(), discreteModelVars: discreteModelVars.clone(), timeEvents: timeEvents.clone(), relations: ZeroCrossings::updateIndices(relations.clone()), zeroCrossings: ZeroCrossings::updateIndices(zeroCrossings.clone()), classAttributes: metamodelica::nil(), constraints: metamodelica::nil(), stateSets: metamodelica::nil(), jacobianEquations: jacobianEquations.clone(), equationsForZeroCrossings: metamodelica::nil(), algorithmAndEquationAsserts: metamodelica::nil(), removedEquations: metamodelica::nil(), parameterEquations: parameterEquations.clone(), maxValueEquations: maxValueEquations.clone(), minValueEquations: minValueEquations.clone(), nominalValueEquations: nominalValueEquations.clone(), startValueEquations: startValueEquations.clone(), removedInitialEquations: removedInitialEquations.clone(), initialEquations_lambda0: initialEquations_lambda0.clone(), initialEquations: initialEquations.clone(), clockedPartitions: metamodelica::nil(), algebraicEquations: metamodelica::nil(), odeEquations: metamodelica::nil(), allEquations: metamodelica::nil(), localKnownVars: metamodelica::nil(), generic_loop_calls: metamodelica::nil(), externalFunctionIncludes: includes.clone(), recordDecls: recordDecls.clone(), literals: metamodelica::nil(), modelInfo: modelInfo.clone() };
    let (__pa9, (_, _, __pa10)) = SimCodeUtil::traverseExpsSimCode(simCode.clone(), (std::sync::Arc::new(SimCodeFunctionUtil::findLiteralsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<DAE::Exp>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>))> + 'static>), literals.clone())?;
    simCode = __pa9.clone();
    lits = __pa10.clone();
    simCode.literals = lits.clone().reverse();
    timeSimCode = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    ExecStat::execStat((literal!("SimCode")).clone())?;
    if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
        serializeNotify(simCode.clone(), (literal!("SimCode")).clone())?;
        ExecStat::execStat((literal!("Serialize simCode")).clone())?;
    }
    if Flags::isSet(Flags::DUMP_SIMCODE.clone())? {
        SimCodeUtil::dumpSimCodeDebug(simCode.clone())?;
    }
    System::realtimeTick(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    callTargetTemplates(simCode.clone(), (Config::simCodeTarget()?).clone())?;
    timeTemplates = System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    ExecStat::execStat((literal!("Templates")).clone())?;
    return Ok((libs.clone(), fileDir.clone(), timeSimCode.clone(), timeTemplates.clone()));
    bail!("fail");
    Ok((libs, fileDir, timeSimCode, timeTemplates))
}

fn serializeNotify<T: Clone + 'static>(mut data: T, mut name: ArcStr) -> Result<()> {
    let mut sz: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut raw_sz: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut nonSharedStringSize: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (sz, raw_sz, nonSharedStringSize) = System::getSizeOfData(data.clone());
    Error::addMessage(Error::SERIALIZED_SIZE.clone(), list![(name.clone()).clone(), (StringUtil::bytesToReadableUnit(sz.clone(), 4, metamodelica::OrderedFloat((500) as f64))).clone(), (StringUtil::bytesToReadableUnit(raw_sz.clone(), 4, metamodelica::OrderedFloat((500) as f64))).clone(), (StringUtil::bytesToReadableUnit(nonSharedStringSize.clone(), 4, metamodelica::OrderedFloat((500) as f64))).clone()])?;
    Ok(())
}

fn copyFiles(mut files: Arc<metamodelica::List<ArcStr>>, mut source: ArcStr, mut destination: ArcStr) -> Result<()> {
    let mut f2: ArcStr = arcstr::literal!("");
    let mut d2: ArcStr = arcstr::literal!("");
    for mut f in &*files.clone() {
        let mut f = f.clone();
        f2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*destination.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*f.clone()); ArcStr::from(__mm_s) }).clone();
        d2 = (System::dirname((f2.clone()).clone())).clone();
        if !(System::directoryExists((d2.clone()).clone())) {
            Error::assertion(Util::createDirectoryTree((d2.clone()).clone())?, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to create directory ")); __mm_s.push_str(&*d2.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        }
        Error::assertion(System::copyFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*source.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*f.clone()); ArcStr::from(__mm_s) }).clone(), (f2.clone()).clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to copy file ")); __mm_s.push_str(&*f.clone()); __mm_s.push_str(&*literal!(" from ")); __mm_s.push_str(&*source.clone()); __mm_s.push_str(&*literal!(" to ")); __mm_s.push_str(&*destination.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    }
    Ok(())
}

