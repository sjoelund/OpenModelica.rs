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

use crate::BackendDAECreate;
use crate::BackendDAEFunc;
use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::IndexReduction;
use crate::Matching;
use crate::Sorting;
use crate::SymbolicJacobian;
use crate::SynchronousFeatures;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_util::BackendDAEEXT;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend::HashSet;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AvlSetCR;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// section for all public functions
//
// These are functions that can be used to access the initialization.
// =============================================================================
pub(crate) fn solveInitialSystem(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<(Arc<BackendDAE::BackendDAE>, Option<Arc<BackendDAE::BackendDAE>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, BackendDAE::Variables, Arc<BackendDAE::BackendDAE>)> {
    let mut outInitDAE: Arc<BackendDAE::BackendDAE>;
    let mut outInitDAE_lambda0: Option<Arc<BackendDAE::BackendDAE>>;
    let mut outRemovedInitialEquations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut outSimDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut dae: Arc<BackendDAE::BackendDAE>;
    let mut initdae: Arc<BackendDAE::BackendDAE>;
    let mut initdae0: Arc<BackendDAE::BackendDAE>;
    let mut initsyst: Arc<BackendDAE::EqSystem>;
    let mut initsyst0: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut reeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut reeqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut shared: Arc<BackendDAE::Shared>;
    let mut initVars: BackendDAE::Variables;
    let mut vars: BackendDAE::Variables;
    let mut fixvars: BackendDAE::Variables;
    let mut useHomotopy: bool;
    let mut datarecon: bool = false;
    let mut enabledModules: Arc<metamodelica::List<ArcStr>>;
    let mut disabledModules: Arc<metamodelica::List<ArcStr>>;
    let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut removedEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut dumpVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut outAllPrimaryParameters: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut allPrimaryParameters: Arc<AvlSetCR::Tree>;
    match '__try0: {
        dae = unwrap_break_err!(inlineWhenForInitialization(inDAE.clone()), '__try0);
        unwrap_break_err!(execStat((literal!("inlineWhenForInitialization (initialization)")).clone()), '__try0);
        (dae, initVars, outAllPrimaryParameters, outGlobalKnownVars) = unwrap_break_err!(selectInitializationVariablesDAE(dae.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_INITIAL_SYSTEM.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpVarList(outAllPrimaryParameters.clone(), (literal!("selected all primary parameters")).clone()), '__try0);
        }
        unwrap_break_err!(execStat((literal!("selectInitializationVariablesDAE (initialization)")).clone()), '__try0);
        hs = unwrap_break_err!(collectPreVariables(dae.clone()), '__try0);
        unwrap_break_err!(execStat((literal!("collectPreVariables (initialization)")).clone()), '__try0);
        vars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
        fixvars = unwrap_break_err!(BackendVariable::listVar(outAllPrimaryParameters.clone()), '__try0);
        eqns = BackendEquation::emptyEqnsSized(BackendVariable::varsSize(dae.shared.aliasVars.clone()) + BackendVariable::varsSize(dae.shared.globalKnownVars.clone()) + BackendVariable::varsSize(dae.shared.localKnownVars.clone()) + BackendEquation::getNumberOfEquations(dae.shared.initialEqs.clone()) + 2 * unwrap_break_err!(BackendDAEUtil::daeSize(dae.clone()), '__try0));
        reeqns = BackendEquation::emptyEqnsSized(BackendEquation::getNumberOfEquations(dae.shared.removedEqs.clone()));
        allPrimaryParameters = openmodelica_frontend_dump::AvlSetCR::Tree::interned_EMPTY();
        for mut v in &*outAllPrimaryParameters.clone() {
            let mut v = v.clone();
            allPrimaryParameters = unwrap_break_err!(AvlSetCR::add(allPrimaryParameters.clone(), unwrap_break_err!(BackendVariable::varCref(v.clone()), '__try0)), '__try0);
        }
        if isSome(inDAE.shared.dataReconciliationData.clone()) {
            datarecon = true;
        }
        (vars, fixvars, eqns, _) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(dae.shared.aliasVars.clone(), (std::sync::Arc::new(fnptr!(introducePreVarsForAliasVariables, BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))> + 'static>), (vars.clone(), fixvars.clone(), eqns.clone(), hs.clone())), '__try0);
        (vars, fixvars, eqns, _, _, _, _) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(dae.shared.globalKnownVars.clone(), (std::sync::Arc::new(collectInitialVars) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), Arc<AvlSetCR::Tree>, bool)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), Arc<AvlSetCR::Tree>, bool))> + 'static>), (vars.clone(), fixvars.clone(), eqns.clone(), arrayCreate(0, 0), hs.clone(), allPrimaryParameters.clone(), datarecon)), '__try0);
        (vars, fixvars, eqns, _, _, _, _) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(dae.shared.localKnownVars.clone(), (std::sync::Arc::new(collectInitialVars) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), Arc<AvlSetCR::Tree>, bool)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), Arc<AvlSetCR::Tree>, bool))> + 'static>), (vars.clone(), fixvars.clone(), eqns.clone(), arrayCreate(0, 0), hs.clone(), allPrimaryParameters.clone(), datarecon)), '__try0);
        (eqns, reeqns) = unwrap_break_err!(BackendEquation::traverseEquationArray(dae.shared.initialEqs.clone(), (std::sync::Arc::new(collectInitialEqns) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> + 'static>), (eqns.clone(), reeqns.clone())), '__try0);
        (eqns, reeqns) = unwrap_break_err!(BackendEquation::traverseEquationArray(dae.shared.removedEqs.clone(), (std::sync::Arc::new(collectInitialEqns) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> + 'static>), (eqns.clone(), reeqns.clone())), '__try0);
        unwrap_break_err!(execStat((literal!("collectInitialEqns (initialization)")).clone()), '__try0);
        (vars, fixvars, eqns, reeqns) = unwrap_break_err!(collectInitialVarsEqnsSystem(dae.eqs.clone(), vars.clone(), fixvars.clone(), eqns.clone(), reeqns.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon), '__try0);
        (eqns, reeqns) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(vars.clone(), (std::sync::Arc::new(collectInitialBindings) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(BackendDAE::Var, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> + 'static>), (eqns.clone(), reeqns.clone())), '__try0);
        unwrap_break_err!(execStat((literal!("collectInitialBindings (initialization)")).clone()), '__try0);
        eqnsLst = unwrap_break_err!(BackendEquation::equationList(eqns.clone()), '__try0);
        reeqnsLst = unwrap_break_err!(BackendEquation::equationList(reeqns.clone()), '__try0);
        (_, eqnsLst, reeqnsLst, _) = unwrap_break_err!(BackendDAECreate::patchRecordBindings(metamodelica::nil(), metamodelica::nil(), unwrap_break_err!(BackendVariable::varList(dae.shared.globalKnownVars.clone()), '__try0), eqnsLst.clone(), reeqnsLst.clone(), metamodelica::nil()), '__try0);
        eqns = unwrap_break_err!(BackendEquation::listEquation(eqnsLst.clone()), '__try0);
        reeqns = unwrap_break_err!(BackendEquation::listEquation(reeqnsLst.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::NF_SCALARIZE.clone()), '__try0) {
            vars = unwrap_break_err!(BackendVariable::scalarizeVariables(vars.clone()), '__try0);
            initVars = unwrap_break_err!(BackendVariable::scalarizeVariables(initVars.clone()), '__try0);
        }
        useHomotopy = unwrap_break_err!(BackendDAEUtil::traverseBackendDAEExpsEqns(eqns.clone(), (std::sync::Arc::new(simplifyInitialFunctions) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false), '__try0);
        unwrap_break_err!(execStat((literal!("simplifyInitialFunctions (initialization)")).clone()), '__try0);
        vars = unwrap_break_err!(BackendVariable::rehashVariables(vars.clone()), '__try0);
        fixvars = unwrap_break_err!(BackendVariable::rehashVariables(fixvars.clone()), '__try0);
        shared = unwrap_break_err!(BackendDAEUtil::createEmptyShared(openmodelica_backend_types::BackendDAE::BackendDAEType::INITIALSYSTEM, dae.shared.info.clone(), dae.shared.cache.clone(), dae.shared.graph.clone()), '__try0);
        shared = unwrap_break_err!(BackendDAEUtil::setSharedRemovedEqns(shared.clone(), BackendEquation::emptyEqns()), '__try0);
        shared = BackendDAEUtil::setSharedGlobalKnownVars(shared.clone(), fixvars.clone());
        shared = unwrap_break_err!(BackendDAEUtil::setSharedOptimica(shared.clone(), dae.shared.constraints.clone(), dae.shared.classAttrs.clone()), '__try0);
        shared = unwrap_break_err!(BackendDAEUtil::setSharedFunctionTree(shared.clone(), dae.shared.functionTree.clone()), '__try0);
        unwrap_break_err!(execStat((literal!("setup shared object (initialization)")).clone()), '__try0);
        initsyst = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
        initsyst = BackendDAEUtil::setEqSystRemovedEqns(initsyst.clone(), reeqns.clone());
        if useHomotopy {
            initsyst0 = unwrap_break_err!(BackendDAEUtil::copyEqSystem(initsyst.clone()), '__try0);
            enabledModules = if (unwrap_break_err!(Config::adaptiveHomotopy(), '__try0)) {list![(literal!("inlineHomotopy")).clone(), (literal!("generateHomotopyComponents")).clone()]} else {metamodelica::nil()};
            disabledModules = metamodelica::nil();
        } else {
            enabledModules = metamodelica::nil();
            disabledModules = list![(literal!("inlineHomotopy")).clone(), (literal!("generateHomotopyComponents")).clone()];
        }
        (initdae, dumpVars, outRemovedInitialEquations) = unwrap_break_err!(createInitialDAEFromSystem(initsyst.clone(), shared.clone(), initVars.clone(), enabledModules.clone(), disabledModules.clone(), outGlobalKnownVars.clone(), false), '__try0);
        (outSimDAE, _) = unwrap_break_err!(BackendVariable::traverseBackendDAE(outSimDAE.clone(), (std::sync::Arc::new(updateFixedAttribute) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> + 'static>), unwrap_break_err!(BackendVariable::listVar(dumpVars.clone()), '__try0)), '__try0);
        if useHomotopy && unwrap_break_err!(Config::globalHomotopy(), '__try0) {
            initsyst0 = unwrap_break_err!(replaceHomotopyWithSimplifiedEqs(initsyst0.clone()), '__try0);
            initdae0 = Arc::new(BackendDAE::BackendDAE { eqs: list![initsyst0.clone()], shared: shared.clone() });
            initdae0 = unwrap_break_err!(BackendDAEUtil::setFunctionTree(initdae0.clone(), unwrap_break_err!(BackendDAEUtil::getFunctions(initdae.shared.clone()), '__try0)), '__try0);
            (initdae0, _, removedEqns) = unwrap_break_err!(createInitialDAEFromSystem(initsyst0.clone(), shared.clone(), initVars.clone(), metamodelica::nil(), list![(literal!("inlineHomotopy")).clone(), (literal!("generateHomotopyComponents")).clone()], outGlobalKnownVars.clone(), true), '__try0);
            outRemovedInitialEquations = listAppend(removedEqns.clone(), outRemovedInitialEquations.clone());
            assign_field!(initdae0.shared = BackendDAEUtil::setSharedGlobalKnownVars(initdae0.shared.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())));
            outInitDAE_lambda0 = Some(initdae0.clone());
            initdae = unwrap_break_err!(BackendDAEUtil::setFunctionTree(initdae.clone(), unwrap_break_err!(BackendDAEUtil::getFunctions(initdae0.shared.clone()), '__try0)), '__try0);
        } else {
            outInitDAE_lambda0 = None;
        }
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_EQNINORDER.clone()), '__try0) && unwrap_break_err!(Flags::isSet(Flags::DUMP_INITIAL_SYSTEM.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpEqnsSolved(initdae.clone(), (literal!("initial system: eqns in order")).clone()), '__try0);
        }
        if unwrap_break_err!(Flags::isSet(Flags::ITERATION_VARS.clone()), '__try0) {
            unwrap_break_err!(BackendDAEOptimize::listAllIterationVariables(initdae.clone()), '__try0);
        }
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_BACKENDDAE_INFO.clone()), '__try0) || unwrap_break_err!(Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone()), '__try0) || unwrap_break_err!(Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpCompShort(initdae.clone()), '__try0);
        }
        outInitDAE = initdae.clone();
        Ok::<_, anyhow::Error>((allPrimaryParameters.clone(), dae.clone(), disabledModules.clone(), dumpVars.clone(), enabledModules.clone(), eqns.clone(), eqnsLst.clone(), fixvars.clone(), hs.clone(), initVars.clone(), initdae.clone(), initsyst.clone(), outAllPrimaryParameters.clone(), outGlobalKnownVars.clone(), outInitDAE.clone(), outInitDAE_lambda0.clone(), outRemovedInitialEquations.clone(), outSimDAE.clone(), reeqns.clone(), reeqnsLst.clone(), shared.clone(), useHomotopy.clone(), vars.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8, __try0_o9, __try0_o10, __try0_o11, __try0_o12, __try0_o13, __try0_o14, __try0_o15, __try0_o16, __try0_o17, __try0_o18, __try0_o19, __try0_o20, __try0_o21, __try0_o22)) => {
            allPrimaryParameters = __try0_o0;
            dae = __try0_o1;
            disabledModules = __try0_o2;
            dumpVars = __try0_o3;
            enabledModules = __try0_o4;
            eqns = __try0_o5;
            eqnsLst = __try0_o6;
            fixvars = __try0_o7;
            hs = __try0_o8;
            initVars = __try0_o9;
            initdae = __try0_o10;
            initsyst = __try0_o11;
            outAllPrimaryParameters = __try0_o12;
            outGlobalKnownVars = __try0_o13;
            outInitDAE = __try0_o14;
            outInitDAE_lambda0 = __try0_o15;
            outRemovedInitialEquations = __try0_o16;
            outSimDAE = __try0_o17;
            reeqns = __try0_o18;
            reeqnsLst = __try0_o19;
            shared = __try0_o20;
            useHomotopy = __try0_o21;
            vars = __try0_o22;
        }
        Err(__try0_err) => {
            Error::addCompilerError((literal!("No system for the symbolic initialization was generated")).clone())?;
            return Err(__try0_err);
        }
    }
    Ok((outInitDAE, outInitDAE_lambda0, outRemovedInitialEquations, outGlobalKnownVars, outSimDAE))
}

pub(crate) fn createInitialDAEFromSystem(mut inInitsyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut initVars: BackendDAE::Variables, mut enabledModules: Arc<metamodelica::List<ArcStr>>, mut disabledModules: Arc<metamodelica::List<ArcStr>>, mut globalKnownVars: BackendDAE::Variables, mut isLambda0: bool) -> Result<(Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut initdae: Arc<BackendDAE::BackendDAE>;
    let mut dumpVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut removedEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut systemStr: ArcStr = if (isLambda0) {literal!("initialization_lambda0")} else {literal!("initialization")};
    let mut shared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut initsyst: Arc<BackendDAE::EqSystem>;
    let mut systs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut dumpVars2: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut initOptModules: Arc<metamodelica::List<(BackendDAEFunc::optimizationModule, ArcStr)>>;
    let mut daeHandler: (BackendDAEFunc::StructurallySingularSystemHandlerFunc, ArcStr, BackendDAEFunc::stateDeselectionFunc, ArcStr);
    let mut matchingAlgorithm: (BackendDAEFunc::matchingAlgorithmFunc, ArcStr);
    let mut b1: bool;
    let mut b2: bool;
    let mut msg: ArcStr;
    (initsyst, dumpVars) = preBalanceInitialSystem(inInitsyst, initVars.clone(), isLambda0)?;
    execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("preBalanceInitialSystem (")); __mm_s.push_str(&*systemStr.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
    initdae = Arc::new(BackendDAE::BackendDAE { eqs: list![initsyst.clone()], shared: shared.clone() });
    if Flags::isSet(Flags::OPT_DAE_DUMP.clone())? {
        BackendDump::dumpBackendDAE(initdae.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("created ")); __mm_s.push_str(&*systemStr.clone()); __mm_s.push_str(&*literal!(" system")); ArcStr::from(__mm_s) }).clone())?;
    }
    if Flags::isSet(Flags::PARTITION_INITIALIZATION.clone())? {
        (systs, shared) = BackendDAEOptimize::partitionIndependentBlocksHelper(initsyst, shared, Error::getNumErrorMessages(), true)?;
        initdae = Arc::new(BackendDAE::BackendDAE { eqs: systs, shared: shared });
        execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("partitionIndependentBlocks (")); __mm_s.push_str(&*systemStr.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
    }
    if Flags::isSet(Flags::OPT_DAE_DUMP.clone())? {
        BackendDump::dumpBackendDAE(initdae.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("partitioned ")); __mm_s.push_str(&*systemStr.clone()); __mm_s.push_str(&*literal!(" system")); ArcStr::from(__mm_s) }).clone())?;
    }
    (initdae, dumpVars2, removedEqns) = analyzeInitialSystem(initdae, initVars, (std::sync::Arc::new(balanceInitialSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32, BackendDAE::Variables, DoubleEnded::MutableList<BackendDAE::Var>, DoubleEnded::MutableList<Arc<BackendDAE::Equation>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> + 'static>))?;
    dumpVars = listAppend(dumpVars, dumpVars2);
    execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("analyzeInitialSystem (")); __mm_s.push_str(&*systemStr.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
    if Flags::isSet(Flags::DUMP_INITIAL_SYSTEM.clone())? {
        BackendDump::dumpBackendDAE(initdae.clone(), (systemStr.clone()).clone())?;
    }
    initdae = BackendDAEUtil::mapEqSystem(initdae, (std::sync::Arc::new(solveInitialSystemEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> + 'static>))?;
    execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("solveInitialSystemEqSystem (")); __mm_s.push_str(&*systemStr.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
    initdae = BackendDAEUtil::transformBackendDAE(initdae, Some((openmodelica_backend_types::BackendDAE::IndexReduction::NO_INDEX_REDUCTION, openmodelica_backend_types::BackendDAE::EquationConstraints::EXACT)), None, None)?;
    execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("matching and sorting (n=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", BackendDAEUtil::daeSize(initdae.clone())?))); __mm_s.push_str(&*literal!(") (")); __mm_s.push_str(&*systemStr.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
    initdae = BackendDAEOptimize::addInitialStmtsToAlgorithms(initdae, true)?;
    initdae = BackendDAEUtil::setDAEGlobalKnownVars(initdae, globalKnownVars)?;
    initOptModules = BackendDAEUtil::getInitOptModules(None, enabledModules, disabledModules)?;
    matchingAlgorithm = BackendDAEUtil::getMatchingAlgorithm(None)?;
    daeHandler = BackendDAEUtil::getIndexReductionMethod(Some((literal!("none")).clone()))?;
    initdae = BackendDAEUtil::postOptimizeDAE(initdae, initOptModules, matchingAlgorithm, daeHandler)?;
    if Flags::isSet(Flags::DUMP_INITIAL_SYSTEM.clone())? {
        BackendDump::dumpBackendDAE(initdae.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("solved ")); __mm_s.push_str(&*systemStr.clone()); ArcStr::from(__mm_s) }).clone())?;
        if Flags::isSet(Flags::ADDITIONAL_GRAPHVIZ_DUMP.clone())? {
            BackendDump::graphvizBackendDAE(initdae.clone(), (literal!("dumpinitialsystem")).clone())?;
        }
    }
    assign_field!(initdae.shared = BackendDAEUtil::setSharedGlobalKnownVars(initdae.shared.clone(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone())));
    b1 = !(dumpVars.clone().is_empty());
    b2 = !(removedEqns.clone().is_empty());
    msg = (literal!("For more information set -d=initialization. In OMEdit Tools->Options->Simulation->Show additional information from the initialization process, in OMNotebook call setCommandLineOptions(\"-d=initialization\")")).clone();
    if Flags::isSet(Flags::INITIALIZATION.clone())? {
        if b1 {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Assuming fixed start value for the following ")); __mm_s.push_str(&*intString((dumpVars.clone().len() as i32))); __mm_s.push_str(&*literal!(" variables:\n")); __mm_s.push_str(&*warnAboutVars2(dumpVars.clone())?); ArcStr::from(__mm_s) }).clone())?;
        }
        if b2 {
            Error::addMessage(Error::INITIALIZATION_OVER_SPECIFIED.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following ")); __mm_s.push_str(&*intString((removedEqns.clone().len() as i32))); __mm_s.push_str(&*literal!(" initial equations are redundant, so they are removed from the ")); __mm_s.push_str(&*systemStr); __mm_s.push_str(&*literal!(" system:\n")); __mm_s.push_str(&*warnAboutEqns2(removedEqns.clone())?); ArcStr::from(__mm_s) }).clone()])?;
        }
    } else {
        if b1 {
            Error::addMessage(Error::INITIALIZATION_NOT_FULLY_SPECIFIED.clone(), list![(msg.clone()).clone()])?;
        }
        if b2 {
            Error::addMessage(Error::INITIALIZATION_OVER_SPECIFIED.clone(), list![(msg).clone()])?;
        }
    }
    Ok((initdae, dumpVars, removedEqns))
}

// =============================================================================
// section for helper functions of solveInitialSystem
//
// =============================================================================
fn solveInitialSystemEqSystem(mut isyst: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>)> {
    let mut osyst: Arc<BackendDAE::EqSystem> = isyst.clone();
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut nVars: i32;
    let mut nEqns: i32;
    nEqns = BackendDAEUtil::systemSize(isyst.clone())?;
    nVars = BackendVariable::varsSize(BackendVariable::daeVars(isyst.clone()));
    if intGt(nEqns, nVars) {
        if Flags::isSet(Flags::INITIALIZATION.clone())? {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It was not possible to solve the over-determined initial system (")); __mm_s.push_str(&*intString(nEqns)); __mm_s.push_str(&*literal!(" equations and ")); __mm_s.push_str(&*intString(nVars)); __mm_s.push_str(&*literal!(" variables)")); ArcStr::from(__mm_s) }).clone())?;
            BackendDump::dumpEqSystem(isyst.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It was not possible to solve the over-determined initial system (")); __mm_s.push_str(&*intString(nEqns)); __mm_s.push_str(&*literal!(" equations and ")); __mm_s.push_str(&*intString(nVars)); __mm_s.push_str(&*literal!(" variables)")); ArcStr::from(__mm_s) }).clone())?;
        }
        bail!("fail");
    }
    if intLt(nEqns, nVars) {
        if Flags::isSet(Flags::INITIALIZATION.clone())? {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It was not possible to solve the under-determined initial system (")); __mm_s.push_str(&*intString(nEqns)); __mm_s.push_str(&*literal!(" equations and ")); __mm_s.push_str(&*intString(nVars)); __mm_s.push_str(&*literal!(" variables)")); ArcStr::from(__mm_s) }).clone())?;
            BackendDump::dumpEqSystem(isyst, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It was not possible to solve the under-determined initial system (")); __mm_s.push_str(&*intString(nEqns)); __mm_s.push_str(&*literal!(" equations and ")); __mm_s.push_str(&*intString(nVars)); __mm_s.push_str(&*literal!(" variables)")); ArcStr::from(__mm_s) }).clone())?;
        }
        bail!("fail");
    }
    Ok((osyst, outShared))
}

// =============================================================================
// section for inlining when-clauses
//
// This section contains all the helper functions to replace all when-clauses
// from a given BackendDAE to get the initial equation system.
// =============================================================================
fn inlineWhenForInitialization(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut clockEqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut leftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = HashSet::emptyHashSet();
    assign_field!(outDAE.eqs = List::map(inDAE.eqs.clone(), (std::sync::Arc::new(inlineWhenForInitializationSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> + 'static>))?);
    (eqnlst, _) = BackendEquation::traverseEquationArray(inDAE.shared.removedEqs.clone(), (std::sync::Arc::new(inlineWhenForInitializationEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))> + 'static>), (metamodelica::nil(), leftCrs))?;
    clockEqnsLst = BackendEquation::traverseEquationArray(inDAE.shared.removedEqs.clone(), (std::sync::Arc::new(fnptr!(SynchronousFeatures::getBoolClockWhenClauses, Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<(Arc<BackendDAE::Equation>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> + 'static>), metamodelica::nil())?;
    eqnlst = listAppend(clockEqnsLst, eqnlst);
    assign_field!(outDAE.shared = BackendDAEUtil::setSharedRemovedEqns(outDAE.shared.clone(), BackendEquation::listEquation(eqnlst)?)?);
    Ok(outDAE)
}

fn inlineWhenForInitializationSystem(mut inEqSystem: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem>;
    let mut eqnlst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut leftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = HashSet::emptyHashSet();
    let mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    (eqnlst, leftCrs) = BackendEquation::traverseEquationArray(inEqSystem.orderedEqs.clone(), (std::sync::Arc::new(inlineWhenForInitializationEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))> + 'static>), (metamodelica::nil(), leftCrs))?;
    crefLst = BaseHashSet::hashSetList(leftCrs)?;
    eqnlst = generateInactiveWhenEquationForInitialization(crefLst, DAE::emptyElementSource().clone(), eqnlst)?;
    outEqSystem = BackendDAEUtil::setEqSystEqs(inEqSystem, BackendEquation::listEquation(eqnlst)?);
    outEqSystem = BackendDAEUtil::clearEqSyst(outEqSystem)?;
    Ok(outEqSystem)
}

fn inlineWhenForInitializationEquation(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) -> Result<(Arc<BackendDAE::Equation>, (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))))> {
    let mut outEq: Arc<BackendDAE::Equation> = inEq.clone();
    let mut outTpl: (Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
    let mut eqAttr: BackendDAE::EquationAttributes = <BackendDAE::EquationAttributes as ::std::default::Default>::default();
    let mut weqn: Arc<BackendDAE::WhenEquation> = Arc::new(<BackendDAE::WhenEquation as ::std::default::Default>::default());
    let mut alg: Arc<DAE::Algorithm> = Arc::new(<DAE::Algorithm as ::std::default::Default>::default());
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    let mut crefExpand: DAE::Expand = DAE::Expand::EXPAND;
    let mut leftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut size: i32 = 0;
    let mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut accEq: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>> = metamodelica::nil();
    (accEq, leftCrs) = inTpl;
    outTpl = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: __esc_weqn, source: __esc_source, attr: __esc_eqAttr, .. } => {
            weqn = (*__esc_weqn).clone();
            source = (*__esc_source).clone();
            eqAttr = (*__esc_eqAttr).clone();
            (leftCrs, eqns) = inlineWhenForInitializationWhenEquation(weqn.clone(), source.clone(), eqAttr.clone(), accEq, leftCrs)?;
            (eqns, leftCrs)
        },
        Deref @ BackendDAE::Equation::ALGORITHM { alg: __esc_alg, source: __esc_source, expand: __esc_crefExpand, .. } => {
            alg = (*__esc_alg).clone();
            source = (*__esc_source).clone();
            crefExpand = (*__esc_crefExpand).clone();
            let __pa0 = ::match_deref::match_deref! { match &(alg.clone()) {
                Deref @ DAE::Algorithm { statementLst: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            stmts = __pa0.clone();
            (stmts, leftCrs) = inlineWhenForInitializationWhenAlgorithm(stmts, metamodelica::nil(), leftCrs)?;
            alg = Arc::new(DAE::Algorithm { statementLst: stmts.clone() });
            size = (CheckModel::checkAndGetAlgorithmOutputs(alg.clone(), source.clone(), crefExpand.clone())?.len() as i32);
            eqns = List::consOnTrue(!(stmts.is_empty()), Arc::new(BackendDAE::Equation::ALGORITHM { size: size, alg: alg.clone(), source: source.clone(), expand: crefExpand.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), accEq);
            (eqns, leftCrs)
        },
        _ => (metamodelica::cons(inEq, accEq), leftCrs),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEq, outTpl))
}

fn inlineWhenForInitializationWhenEquation(mut inWEqn: Arc<BackendDAE::WhenEquation>, mut inSource: Arc<DAE::ElementSource>, mut inEqAttr: BackendDAE::EquationAttributes, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inLeftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outLeftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)) = inLeftCrs.clone();
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut lhs: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut condition: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut e: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut eLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut eqn: Arc<BackendDAE::Equation> = Arc::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut whenStmtLst: Arc<metamodelica::List<BackendDAE::WhenOperator>> = metamodelica::nil();
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut active: bool = false;
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    outEqns = (::match_deref::match_deref! { match &(inWEqn) {
        Deref @ BackendDAE::WhenEquation { condition: __esc_condition, whenStmtLst: __esc_whenStmtLst, .. } => {
            condition = (*__esc_condition).clone();
            whenStmtLst = (*__esc_whenStmtLst).clone();
            active = Expression::containsInitialCall(condition.clone())?;
            for mut stmt in &*whenStmtLst.clone() {
                let mut stmt = stmt.clone();
                let () = (::match_deref::match_deref! { match &(stmt.clone()) {
        BackendDAE::WhenOperator::ASSIGN { left: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, right: __esc_e, .. } => {
            cr = (*__esc_cr).clone();
            e = (*__esc_e).clone();
            if active {
                lhs = Expression::crefExp(cr.clone())?;
                eqn = BackendEquation::generateEquation(lhs.clone(), e.clone(), inSource.clone(), inEqAttr.clone())?;
                outEqns = metamodelica::cons(eqn.clone(), outEqns.clone());
            } else {
                outLeftCrs = List::fold(ComponentReference::expandCref(cr.clone(), true)?, (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), outLeftCrs.clone())?;
            }
            ()
        },
        BackendDAE::WhenOperator::ASSIGN { left: __esc_lhs @ Deref @ DAE::Exp::TUPLE { PR: __esc_eLst }, right: __esc_e, .. } => {
            lhs = (*__esc_lhs).clone();
            eLst = (*__esc_eLst).clone();
            e = (*__esc_e).clone();
            if active {
                eqn = BackendEquation::generateEquation(lhs.clone(), e.clone(), inSource.clone(), inEqAttr.clone())?;
                outEqns = metamodelica::cons(eqn.clone(), outEqns.clone());
            } else {
                crefLst = List::flatten(List::map(eLst.clone(), (std::sync::Arc::new(Expression::getAllCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>))?)?;
                for mut cr in &*crefLst.clone() {
                    let mut cr = cr.clone();
                    outLeftCrs = List::fold(ComponentReference::expandCref(cr.clone(), true)?, (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), outLeftCrs.clone())?;
                }
            }
            ()
        },
        BackendDAE::WhenOperator::NORETCALL { exp: __esc_e, source: __esc_source } => {
            e = (*__esc_e).clone();
            source = (*__esc_source).clone();
            if active {
                eqn = Arc::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: Arc::new(DAE::Algorithm { statementLst: list![Arc::new(DAE::Statement::STMT_NORETCALL { exp: e.clone(), source: source.clone() })] }), source: inSource.clone(), expand: openmodelica_frontend_types::DAE::Expand::EXPAND, attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                outEqns = metamodelica::cons(eqn.clone(), outEqns.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            outEqns
        },
        _ => outEqns,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outLeftCrs, outEqns))
}

fn inlineWhenForInitializationWhenAlgorithm(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inAcc: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut inLeftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inStmts) {
        Deref @ metamodelica::List::Nil => {
            return Ok((inAcc.reverse(), inLeftCrs))
        },
        Deref @ metamodelica::List::Cons { head: stmt @ Deref @ DAE::Statement::STMT_WHEN { .. }, tail: rest } => {
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>>;
            let mut leftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (stmts, leftCrs) = inlineWhenForInitializationWhenStmt(stmt.clone(), inLeftCrs, inAcc)?;
            { (inStmts, inAcc, inLeftCrs) = (rest.clone(), stmts, leftCrs); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: stmt, tail: rest } => {
            let mut stmts: Arc<metamodelica::List<Arc<DAE::Statement>>>;
            let mut leftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            { (inStmts, inAcc, inLeftCrs) = (rest.clone(), metamodelica::cons(stmt.clone(), inAcc), inLeftCrs); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn inlineWhenForInitializationWhenStmt(mut inWhenStatement: Arc<DAE::Statement>, mut inLeftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut inAcc: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Statement>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inWhenStatement) {
        Deref @ DAE::Statement::STMT_WHEN { exp: condition, statementLst: stmts, .. } if (Expression::containsInitialCall(condition.clone())?) => {
            let mut stmts = (*stmts).clone();
            stmts = List::foldr(stmts.clone(), std::sync::Arc::new(fnptr!(List::consr, _, _)), inAcc)?;
            return Ok((stmts.clone(), inLeftCrs))
        },
        Deref @ DAE::Statement::STMT_WHEN { statementLst: stmts, elseWhen: None, .. } => {
            let mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut leftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            crefLst = CheckModel::algorithmStatementListOutputs(stmts.clone(), openmodelica_frontend_types::DAE::Expand::EXPAND)?;
            leftCrs = List::fold(crefLst, (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), inLeftCrs)?;
            return Ok((inAcc, leftCrs))
        },
        Deref @ DAE::Statement::STMT_WHEN { statementLst: stmts, elseWhen: Some(stmt), .. } => {
            let mut crefLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            let mut leftCrs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut stmts = (*stmts).clone();
            crefLst = CheckModel::algorithmStatementListOutputs(stmts.clone(), openmodelica_frontend_types::DAE::Expand::EXPAND)?;
            leftCrs = List::fold(crefLst, (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), inLeftCrs)?;
            { (inWhenStatement, inLeftCrs, inAcc) = (stmt.clone(), leftCrs, inAcc); continue '__tco; }
        },
        _ => {
            Error::addInternalError((literal!("function inlineWhenForInitializationWhenStmt failed")).clone(), metamodelica::sourceInfo!("BackEnd/Initialization.mo"))?;
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn generateInactiveWhenEquationForInitialization(mut inCrLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inSource: Arc<DAE::ElementSource>, mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut identType: Arc<DAE::Type>;
    let mut crefExp: Arc<DAE::Exp>;
    let mut crefPreExp: Arc<DAE::Exp>;
    let mut eqn: Arc<BackendDAE::Equation>;
    for mut cr in &*inCrLst {
        let mut cr = cr.clone();
        identType = ComponentReference::crefTypeConsiderSubs(cr.clone())?;
        crefExp = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: identType.clone() });
        crefPreExp = Expression::makePureBuiltinCall((literal!("pre")).clone(), list![crefExp.clone()], DAE::T_BOOL_DEFAULT().clone());
        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: crefPreExp.clone(), source: inSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
        outEqns = metamodelica::cons(eqn.clone(), outEqns.clone());
    }
    Ok(outEqns)
}

// =============================================================================
// section for collecting all variables, of which the left limit is also used.
//
// collect all pre variables in time equations
// =============================================================================
fn collectPreVariables(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> {
    let mut outHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    outHS = List::fold(inDAE.eqs.clone(), (std::sync::Arc::new(collectPreVariablesEqSystem) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> + 'static>), HashSet::emptyHashSet())?;
    (_, outHS) = BackendDAEUtil::traverseBackendDAEExpsEqns(inDAE.shared.initialEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(collectPreVariablesTraverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> + 'static>), outHS))?;
    (_, outHS) = BackendDAEUtil::traverseBackendDAEExpsEqns(inDAE.shared.removedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(collectPreVariablesTraverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> + 'static>), outHS))?;
    Ok(outHS)
}

pub(crate) fn collectPreVariablesEqSystem(mut inSyst: Arc<BackendDAE::EqSystem>, mut inHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))> {
    let mut outHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    (_, outHS) = BackendDAEUtil::traverseBackendDAEExpsEqns(inSyst.orderedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(collectPreVariablesTraverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> + 'static>), inHS))?;
    (_, outHS) = BackendDAEUtil::traverseBackendDAEExpsEqns(inSyst.removedEqs.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(collectPreVariablesTraverseExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> + 'static>), outHS))?;
    Ok(outHS)
}

pub(crate) fn collectPreVariablesTraverseExp(mut inExp: Arc<DAE::Exp>, mut inHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    outHS = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => {
            (_, outHS) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new(collectPreVariablesTraverseExp2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> + 'static>), inHS)?;
            outHS
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. } => {
            (_, outHS) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new(collectPreVariablesTraverseExp2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> + 'static>), inHS)?;
            outHS
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. } => {
            (_, outHS) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new(collectPreVariablesTraverseExp2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> + 'static>), inHS)?;
            outHS
        },
        _ => inHS,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outHS))
}

fn collectPreVariablesTraverseExp2(mut inExp: Arc<DAE::Exp>, mut inHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))) -> Result<(Arc<DAE::Exp>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outHS: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    outHS = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            crefs = ComponentReference::expandCref(cr.clone(), true)?;
            outHS = List::fold(crefs, (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), inHS)?;
            outHS
        },
        _ => {
            inHS
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outHS))
}

fn warnAboutVars2(mut vars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut strs: Arc<metamodelica::List<ArcStr>>;
    let mut len: i32;
    let mut size: i32;
    if vars.clone().is_empty() {
        outString = (literal!("")).clone();
        return Ok(outString.clone());
    }
    strs = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (vars).into_iter().cloned() {
            let __x = BackendDump::varString(v.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    len = (strs.clone().len() as i32);
    size = ({
        let mut __acc: i32 = 0;
        for mut s in (strs.clone()).into_iter().cloned() {
            let __x = ((s.clone()).clone().len() as i32);
            __acc += __x;
        }
        __acc
    }) + len * 10;
    outString = (warnAboutVars2Work(strs, (literal!("         ")).clone(), (literal!("\n")).clone(), size)?).clone();
    Ok(outString)
}

fn warnAboutVars2Work(mut strs: Arc<metamodelica::List<ArcStr>>, mut prefix: ArcStr, mut suffix: ArcStr, mut size: i32) -> Result<ArcStr> {
    let mut s: ArcStr = literal!("");
    let mut sb: System::StringAllocator = System::StringAllocator(size)?;
    let mut i: i32 = 0;
    for mut r#str in &*strs {
        let mut r#str = r#str.clone();
        System::stringAllocatorStringCopy(sb.clone(), (prefix.clone()).clone(), i);
        i = i + ((prefix.clone()).clone().len() as i32);
        System::stringAllocatorStringCopy(sb.clone(), (r#str.clone()).clone(), i);
        i = i + ((r#str.clone()).clone().len() as i32);
        System::stringAllocatorStringCopy(sb.clone(), (suffix.clone()).clone(), i);
        i = i + ((suffix.clone()).clone().len() as i32);
    }
    s = (System::stringAllocatorResult(sb, (s).clone())).clone();
    Ok(s)
}

fn warnAboutEqns2(mut inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inEqns) {
        Deref @ metamodelica::List::Nil => {
            literal!("")
        },
        Deref @ metamodelica::List::Cons { head: eq, tail: Deref @ metamodelica::List::Nil } => {
            let mut crStr: ArcStr;
            crStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("         ")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); ArcStr::from(__mm_s) }).clone();
            crStr
        },
        Deref @ metamodelica::List::Cons { head: eq, tail: eqns } => {
            let mut crStr: ArcStr;
            let mut r#str: ArcStr;
            crStr = (BackendDump::equationString(eq.clone())?).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("         ")); __mm_s.push_str(&*crStr); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*warnAboutEqns2(eqns.clone())?); ArcStr::from(__mm_s) }).clone();
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

// =============================================================================
// section for selecting initialization variables
//
//   - unfixed state
//   - secondary parameter
//   - unfixed discrete -> pre(vd)
// =============================================================================
fn selectInitializationVariablesDAE(mut dae: Arc<BackendDAE::BackendDAE>) -> Result<(Arc<BackendDAE::BackendDAE>, BackendDAE::Variables, Arc<metamodelica::List<BackendDAE::Var>>, BackendDAE::Variables)> {
    let mut dae: Arc<BackendDAE::BackendDAE> = dae;
    let mut outInitVars: BackendDAE::Variables;
    let mut outAllPrimaryParameters: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut outGlobalKnownVars: BackendDAE::Variables = dae.shared.globalKnownVars.clone();
    let mut otherVariables: BackendDAE::Variables;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut globalKnownVarsEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut globalKnownVarsSystem: Arc<BackendDAE::EqSystem>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut ass1: metamodelica::Array<i32>;
    let mut ass2: metamodelica::Array<i32>;
    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut flatComps: Arc<metamodelica::List<i32>>;
    let mut nGlobalKnownVars: i32;
    let mut secondary: metamodelica::Array<i32>;
    let mut v: BackendDAE::Var;
    let mut bindExp: Arc<DAE::Exp>;
    let mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut globalKnownVarList: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    for mut var in &*BackendVariable::varList(dae.shared.globalKnownVars.clone())? {
        let mut var = var.clone();
        if BackendVariable::isInput(var.clone()) && !(Expression::isConstValue(BackendVariable::varStartValue(var.clone())?)?) && !(Types::isArray(BackendVariable::varType(var.clone())?)) {
            bindExp = BackendVariable::varStartValue(var.clone())?;
            (v, _) = BackendVariable::getVarSingle(Expression::expCref(bindExp.clone())?, dae.shared.globalKnownVars.clone())?;
            var = BackendVariable::setVarStartValueOption(var.clone(), v.bindExp.clone())?;
        }
        globalKnownVarList = metamodelica::cons(var.clone(), globalKnownVarList.clone());
    }
    dae = BackendDAEUtil::setDAEGlobalKnownVars(dae, BackendVariable::listVar(globalKnownVarList)?)?;
    globalKnownVars = BackendVariable::listVar(BackendVariable::varList(dae.shared.globalKnownVars.clone())?)?;
    outInitVars = selectInitializationVariables(dae.eqs.clone())?;
    outInitVars = BackendVariable::traverseBackendDAEVars(dae.shared.globalKnownVars.clone(), (std::sync::Arc::new(fnptr!(selectInitializationVariables2, BackendDAE::Var, BackendDAE::Variables)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> + 'static>), outInitVars)?;
    outInitVars = BackendVariable::traverseBackendDAEVars(dae.shared.aliasVars.clone(), (std::sync::Arc::new(fnptr!(selectInitializationVariables2, BackendDAE::Var, BackendDAE::Variables)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> + 'static>), outInitVars)?;
    globalKnownVars = BackendVariable::traverseBackendDAEVars(dae.shared.externalObjects.clone(), (std::sync::Arc::new(addExtObjToGlobalKnownVars) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> + 'static>), globalKnownVars)?;
    nGlobalKnownVars = BackendVariable::varsSize(globalKnownVars.clone());
    otherVariables = BackendVariable::emptyVarsSized(nGlobalKnownVars);
    globalKnownVarsEqns = BackendEquation::emptyEqnsSized(nGlobalKnownVars);
    globalKnownVarsEqns = BackendVariable::traverseBackendDAEVars(globalKnownVars.clone(), (std::sync::Arc::new(createGlobalKnownVarsEquations) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Var, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)> + 'static>), globalKnownVarsEqns)?;
    if nGlobalKnownVars > 0 {
        globalKnownVarsSystem = BackendDAEUtil::createEqSystem(globalKnownVars.clone(), globalKnownVarsEqns, metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
        (m, mT) = BackendDAEUtil::adjacencyMatrix(globalKnownVarsSystem, openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(dae.shared.clone()))?;
        (ass1, ass2) = Matching::PerfectMatching(m.clone())?;
        comps = Sorting::Tarjan(m.clone(), ass1.clone(), metamodelica::arrayLength(ass1.clone()))?;
        comps = mapListIndices(comps, ass2.clone())?;
        flatComps = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut comp in (comps).into_iter().cloned() {
            let __x = flattenParamComp(comp.clone(), globalKnownVars.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        secondary = arrayCreate(nGlobalKnownVars, 0);
        secondary = selectSecondaryParameters(flatComps.clone(), globalKnownVars.clone(), mT.clone(), secondary.clone())?;
        hs = HashSet::emptyHashSetSized(2 * nGlobalKnownVars + 1);
        for mut i in &*flatComps {
            let mut i = i.clone();
            v = BackendVariable::getVarAt(globalKnownVars.clone(), i.clone())?;
            bindExp = BackendVariable::varBindExpStartValueNoFail(v.clone())?;
            crefs = Expression::getAllCrefsExpanded(bindExp.clone())?;
            let () = (::match_deref::match_deref! { match &(v.clone()) {
        BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, .. } if (0 == ({let __elt = secondary.borrow()[(i.clone()-1) as usize].clone(); __elt}) && BaseHashSet::hasAll(crefs.clone(), hs.clone())?) => {
            outAllPrimaryParameters = metamodelica::cons(v.clone(), outAllPrimaryParameters.clone());
            hs = BaseHashSet::add(BackendVariable::varCref(v.clone())?, hs.clone())?;
            ()
        },
        BackendDAE::Var { varKind: BackendDAE::VarKind::EXTOBJ { .. }, bindExp: Some(__esc_bindExp), .. } if (0 == ({let __elt = secondary.borrow()[(i.clone()-1) as usize].clone(); __elt}) && BaseHashSet::hasAll(crefs.clone(), hs.clone())?) => {
            bindExp = (*__esc_bindExp).clone();
            outAllPrimaryParameters = metamodelica::cons(v.clone(), outAllPrimaryParameters.clone());
            v = BackendVariable::setVarFixed(v.clone(), true)?;
            outGlobalKnownVars = BackendVariable::addVar(v.clone(), outGlobalKnownVars.clone())?;
            hs = BaseHashSet::add(BackendVariable::varCref(v.clone())?, hs.clone())?;
            ()
        },
        BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, .. } => {
            otherVariables = BackendVariable::addVar(v.clone(), otherVariables.clone())?;
            v = BackendVariable::setVarFixed(v.clone(), false)?;
            outInitVars = BackendVariable::addVar(v.clone(), outInitVars.clone())?;
            outGlobalKnownVars = BackendVariable::addVar(v.clone(), outGlobalKnownVars.clone())?;
            ()
        },
        _ if (BackendVariable::isVarAlg(v.clone()) && 0 == ({let __elt = secondary.borrow()[(i.clone()-1) as usize].clone(); __elt}) && BaseHashSet::hasAll(crefs.clone(), hs.clone())?) => {
            otherVariables = BackendVariable::addVar(v.clone(), otherVariables.clone())?;
            v = BackendVariable::setVarFixed(v.clone(), true)?;
            v = BackendVariable::setVarFinal(v.clone(), true)?;
            outGlobalKnownVars = BackendVariable::addVar(v.clone(), outGlobalKnownVars.clone())?;
            hs = BaseHashSet::add(BackendVariable::varCref(v.clone())?, hs.clone())?;
            ()
        },
        _ if (BackendVariable::isVarAlg(v.clone())) => {
            otherVariables = BackendVariable::addVar(v.clone(), otherVariables.clone())?;
            v = BackendVariable::setVarFixed(v.clone(), false)?;
            outGlobalKnownVars = BackendVariable::addVar(v.clone(), outGlobalKnownVars.clone())?;
            ()
        },
        _ => {
            otherVariables = BackendVariable::addVar(v.clone(), otherVariables.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        GCExt::free(secondary.clone());
        outAllPrimaryParameters = outAllPrimaryParameters.reverse();
        dae = BackendDAEUtil::setDAEGlobalKnownVars(dae, otherVariables)?;
    }
    Ok((dae, outInitVars, outAllPrimaryParameters, outGlobalKnownVars))
}

fn addExtObjToGlobalKnownVars(mut extObj: BackendDAE::Var, mut globalKnownVars: BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> {
    let mut extObj: BackendDAE::Var = extObj;
    let mut globalKnownVars: BackendDAE::Variables = globalKnownVars;
    globalKnownVars = (::match_deref::match_deref! { match &(extObj.clone()) {
        BackendDAE::Var { varKind: BackendDAE::VarKind::EXTOBJ { .. }, bindExp: Some(_), .. } => {
            let mut var: BackendDAE::Var;
            var = BackendVariable::setVarFixed(extObj.clone(), true)?;
            globalKnownVars = BackendVariable::addVar(var, globalKnownVars)?;
            globalKnownVars
        },
        _ => {
            globalKnownVars
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((extObj, globalKnownVars))
}

fn createGlobalKnownVarsEquations(mut var: BackendDAE::Var, mut parameterEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Var, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)> {
    let mut var: BackendDAE::Var = var;
    let mut parameterEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = parameterEqns;
    let mut lhs: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut startValue: Arc<DAE::Exp>;
    let mut eqn: Arc<BackendDAE::Equation>;
    let mut v: BackendDAE::Var;
    let mut s: ArcStr;
    let mut r#str: ArcStr;
    let mut info: SourceInfo;
    lhs = BackendVariable::varExp(var.clone())?;
    if BackendVariable::isParam(var.clone()) && !(BackendVariable::varHasBindExp(var.clone())) && BackendVariable::varFixed(var.clone()) {
        s = (ExpressionBasics::printExpStr(lhs.clone())?).clone();
        startValue = BackendVariable::varStartValue(var.clone())?;
        r#str = (ExpressionBasics::printExpStr(startValue.clone())?).clone();
        v = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
        v = BackendVariable::setBindExp(v, Some(startValue));
        v = BackendVariable::setVarFixed(v, true)?;
        info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(v));
        Error::addSourceMessage(Error::UNBOUND_PARAMETER_WITH_START_VALUE_WARNING.clone(), list![(s).clone(), (r#str).clone()], info)?;
    }
    rhs = BackendVariable::varBindExpStartValueNoFail(var.clone())?;
    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs, scalar: rhs, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone() });
    parameterEqns = BackendEquation::add(eqn, parameterEqns)?;
    Ok((var, parameterEqns))
}

fn markIndex(mut inIndex: i32, mut inArray: metamodelica::Array<i32>) -> metamodelica::Array<i32> {
    let mut outArray: metamodelica::Array<i32> = inArray.clone();
    {
        let __cell0 = 1;
        let __idx0 = inIndex;
        outArray.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
    }
    outArray
}

fn selectSecondaryParameters(mut inOrdering: Arc<metamodelica::List<i32>>, mut inParameters: BackendDAE::Variables, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inSecondaryParams: metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> {
    let mut outSecondaryParams: metamodelica::Array<i32> = inSecondaryParams.clone();
    let mut param: BackendDAE::Var;
    for mut i in &*inOrdering {
        let mut i = i.clone();
        param = BackendVariable::getVarAt(inParameters.clone(), i.clone())?;
        outSecondaryParams = if (if (BackendVariable::isVarAlg(param.clone())) {false} else {!(BackendVariable::varFixed(param.clone()))} || 1 == ({let __elt = outSecondaryParams.borrow()[(i.clone()-1) as usize].clone(); __elt})) {List::fold(({let __elt = inM.borrow()[(i.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new(fnptr!(markIndex, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<metamodelica::Array<i32>> + 'static>), outSecondaryParams.clone())?} else {outSecondaryParams.clone()};
    }
    Ok(outSecondaryParams)
}

pub(crate) fn flattenParamComp(mut paramIndices: Arc<metamodelica::List<i32>>, mut inAllParameters: BackendDAE::Variables) -> Result<i32> {
    let mut outFlatComp: i32;
    outFlatComp = (::match_deref::match_deref! { match &(paramIndices.clone()) {
        Deref @ metamodelica::List::Cons { head: i, tail: Deref @ metamodelica::List::Nil } => {
            i.clone()
        },
        _ => {
            let mut i: i32 = 0;
            let mut paramLst: Arc<metamodelica::List<BackendDAE::Var>>;
            let mut param: BackendDAE::Var;
            paramLst = metamodelica::nil();
            for mut i in &*paramIndices {
                let mut i = i.clone();
                param = BackendVariable::getVarAt(inAllParameters.clone(), i)?;
                paramLst = metamodelica::cons(param.clone(), paramLst.clone());
            }
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cyclically dependent parameters found:\n")); __mm_s.push_str(&*warnAboutVars2(paramLst)?); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outFlatComp)
}

fn selectInitializationVariables(mut inEqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> Result<BackendDAE::Variables> {
    let mut outVars: BackendDAE::Variables;
    outVars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
    outVars = List::fold(inEqSystems, (std::sync::Arc::new(selectInitializationVariables1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, BackendDAE::Variables) -> Result<BackendDAE::Variables> + 'static>), outVars)?;
    Ok(outVars)
}

fn selectInitializationVariables1(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inVars: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut outVars: BackendDAE::Variables;
    outVars = BackendVariable::traverseBackendDAEVars(inEqSystem.orderedVars.clone(), (std::sync::Arc::new(fnptr!(selectInitializationVariables2, BackendDAE::Var, BackendDAE::Variables)) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> + 'static>), inVars)?;
    Ok(outVars)
}

fn selectInitializationVariables2(mut inVar: BackendDAE::Var, mut inVars: BackendDAE::Variables) -> (BackendDAE::Var, BackendDAE::Variables) {
    let mut outVar: BackendDAE::Var;
    let mut outVars: BackendDAE::Variables;
    (outVar, outVars) = 'mc: {
        let __mc_input = (inVar.clone(), inVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, mut vars) = __mc_input.clone() else { bail!("nomatch") };
            let false = (BackendVariable::varFixed(inVar.clone())) else { bail!("pattern mismatch") };
            vars = BackendVariable::addVar(inVar.clone(), vars.clone())?;
            Ok((inVar.clone(), vars.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (BackendDAE::Var { varName: ref cr, varKind: BackendDAE::VarKind::DISCRETE { .. }, varType: ref ty, arryDim: mut arryDim, .. }, mut vars) = __mc_input.clone() else { bail!("nomatch") };
            let mut preVar: BackendDAE::Var;
            let mut preCR: Arc<DAE::ComponentRef>;
            let false = (BackendVariable::varFixed(inVar.clone())) else { bail!("pattern mismatch") };
            preCR = ComponentReference::crefPrefixPre(cr.clone());
            preVar = BackendDAE::Var { varName: preCR.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty.clone(), bindExp: None, tplExp: None, arryDim: arryDim.clone(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
            vars = BackendVariable::addVar(preVar.clone(), vars.clone())?;
            Ok((inVar.clone(), vars.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((inVar.clone(), inVars.clone()))
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, outVars)
}

// =============================================================================
// section for simplifying initial functions
//
// =============================================================================
fn simplifyInitialFunctions(mut inExp: Arc<DAE::Exp>, mut inUseHomotopy: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outUseHomotopy: bool;
    (outExp, outUseHomotopy) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new(fnptr!(simplifyInitialFunctionsExp, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), inUseHomotopy)?;
    Ok((outExp, outUseHomotopy))
}

fn simplifyInitialFunctionsExp(mut inExp: Arc<DAE::Exp>, mut inUseHomotopy: bool) -> (Arc<DAE::Exp>, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outUseHomotopy: bool;
    (outExp, outUseHomotopy) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => {
            (Arc::new(DAE::Exp::BCONST { bool: true }), inUseHomotopy)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, .. } => {
            (Arc::new(DAE::Exp::BCONST { bool: false }), inUseHomotopy)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: expr, tail: _ } }, .. } => {
            (expr.clone(), inUseHomotopy)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, .. } => {
            (inExp, true)
        },
        _ => {
            (inExp, inUseHomotopy)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outUseHomotopy)
}

// =============================================================================
// section for pre-balancing the initial system
//
// This section removes unused pre variables and auto-fixes non-pre variables,
// which occur in no equation.
// =============================================================================
fn preBalanceInitialSystem(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut initVars: BackendDAE::Variables, mut isLambda0: bool) -> Result<(Arc<BackendDAE::EqSystem>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem> = inEqSystem.clone();
    let mut outDumpVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut orderedVars: BackendDAE::Variables;
    let mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut b: bool;
    let mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    (_, mt) = BackendDAEUtil::adjacencyMatrix(inEqSystem.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, None, true)?;
    (orderedVars, orderedEqs, b, outDumpVars) = preBalanceInitialSystem1(metamodelica::arrayLength(mt.clone()), mt.clone(), inEqSystem.orderedVars.clone(), inEqSystem.orderedEqs.clone(), initVars, isLambda0, false, metamodelica::nil())?;
    if b {
        assign_field!(
            outEqSystem.orderedEqs = orderedEqs,
            outEqSystem.orderedVars = orderedVars
        );
        outEqSystem = BackendDAEUtil::clearEqSyst(outEqSystem)?;
    }
    Ok((outEqSystem, outDumpVars))
}

fn preBalanceInitialSystem1(mut n: i32, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inVars: BackendDAE::Variables, mut inEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut initVars: BackendDAE::Variables, mut isLambda0: bool, mut inB: bool, mut inDumpVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, bool, Arc<metamodelica::List<BackendDAE::Var>>)> {
    '__tco: loop {
        match (n, inB) {
        (0, false) => {
            return Ok((inVars, inEqs, false, inDumpVars))
        },
        (0, true) => {
            let mut vars: BackendDAE::Variables;
            vars = BackendVariable::listVar1(BackendVariable::varList(inVars)?)?;
            return Ok((vars, inEqs, true, inDumpVars))
        },
        _ => {
            let mut b: bool;
            let mut vars: BackendDAE::Variables;
            let mut eqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut dumpVars: Arc<metamodelica::List<BackendDAE::Var>>;
            let true = (n > 0) else { bail!("pattern mismatch") };
            (vars, eqs, b, dumpVars) = preBalanceInitialSystem2(n, mt.clone(), inVars, inEqs, initVars.clone(), isLambda0, inB, inDumpVars)?;
            { (n, mt, inVars, inEqs, initVars, isLambda0, inB, inDumpVars) = (n - 1, mt.clone(), vars, eqs, initVars, isLambda0, b, dumpVars); continue '__tco; }
        },
    }
    }
}

fn preBalanceInitialSystem2(mut n: i32, mut mt: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inVars: BackendDAE::Variables, mut inEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut initVars: BackendDAE::Variables, mut isLambda0: bool, mut inB: bool, mut inDumpVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, bool, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut outVars: BackendDAE::Variables = inVars.clone();
    let mut outEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inEqs.clone();
    let mut outB: bool = inB;
    let mut outDumpVars: Arc<metamodelica::List<BackendDAE::Var>> = inDumpVars.clone();
    let mut row: Arc<metamodelica::List<i32>>;
    let mut var: BackendDAE::Var;
    let mut cref: Arc<DAE::ComponentRef>;
    let mut r#str: ArcStr;
    let mut err_str: ArcStr = literal!(" with unknown reason.");
    match '__try0: {
        row = ({let __elt = mt.borrow()[(n-1) as usize].clone(); __elt});
        if row.clone().is_empty() {
            outB = true;
            var = unwrap_break_err!(BackendVariable::getVarAt(inVars.clone(), n), '__try0);
            cref = unwrap_break_err!(BackendVariable::varCref(var.clone()), '__try0);
            if ComponentReference::isPreCref(cref.clone()) {
                (outVars, _) = BackendVariable::removeVars(list![n], inVars.clone(), metamodelica::nil());
            } else if BackendVariable::containsVar(var.clone(), initVars.clone()) {
                (outEqs, outDumpVars) = unwrap_break_err!(addStartValueEquations(list![var.clone()], inEqs.clone(), inDumpVars.clone()), '__try0);
            } else {
                r#str = (if (isLambda0) {literal!("lambda 0 ")} else {literal!("")}).clone();
                err_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" because variable ")); __mm_s.push_str(&*unwrap_break_err!(BackendDump::varString(var.clone()), '__try0)); __mm_s.push_str(&*literal!(" does not appear in any equation in the ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("initial system and is not fixable.")); ArcStr::from(__mm_s) }).clone();
                break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
            }
        }
        Ok::<_, anyhow::Error>((row.clone(),))
    } {
        Ok((__try0_o0,)) => {
            row = __try0_o0;
        }
        Err(__try0_err) => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Initialization.preBalanceInitialSystem2")); __mm_s.push_str(&*literal!(" failed")); __mm_s.push_str(&*err_str.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Initialization.mo"))?;
            return Err(__try0_err);
        }
    }
    Ok((outVars, outEqs, outB, outDumpVars))
}

fn analyzeInitialSystem(mut inInitDAE: Arc<BackendDAE::BackendDAE>, mut inInitVars: BackendDAE::Variables, mut func: Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32, BackendDAE::Variables, DoubleEnded::MutableList<BackendDAE::Var>, DoubleEnded::MutableList<Arc<BackendDAE::Equation>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> + 'static>) -> Result<(Arc<BackendDAE::BackendDAE>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<BackendDAE::Equation>>>)> {
    let mut outDAE: Arc<BackendDAE::BackendDAE>;
    let mut outDumpVars: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut outRemovedEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut dae: Arc<BackendDAE::BackendDAE>;
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>;
    let mut dumpVars: DoubleEnded::MutableList<BackendDAE::Var>;
    let mut removedEqns: DoubleEnded::MutableList<Arc<BackendDAE::Equation>>;
    let mut filtered_initial_eqs: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    eqs = metamodelica::nil();
    dumpVars = DoubleEnded::fromList(metamodelica::nil())?;
    removedEqns = DoubleEnded::fromList(metamodelica::nil())?;
    for mut syst in &*inInitDAE.eqs.clone() {
        let mut syst = syst.clone();
        if BackendDAEUtil::nonEmptySystem(syst.clone()) {
            eqs = metamodelica::cons(syst.clone(), eqs.clone());
        } else {
            filtered_initial_eqs = ({
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
        for mut eqn in (BackendEquation::equationList(syst.orderedEqs.clone())?).into_iter().cloned() {
            if !(BackendEquation::hasAnyUnknown(eqn.clone(), inInitVars.clone())?) { continue; }
            let __x = eqn.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            DoubleEnded::push_list_back(removedEqns.clone(), filtered_initial_eqs.clone())?;
            DoubleEnded::push_list_back(removedEqns.clone(), BackendEquation::equationList(syst.removedEqs.clone())?)?;
        }
    }
    dae = Arc::new(BackendDAE::BackendDAE { eqs: eqs, shared: inInitDAE.shared.clone() });
    (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(dae, (std::sync::Arc::new({ let __pe_b3 = inInitVars; let __pe_b4 = dumpVars.clone(); let __pe_b5 = removedEqns.clone(); move |__pe_a0, __pe_a1, __pe_a2| func(__pe_a0, __pe_a1, __pe_a2, __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> + 'static>), 0)?;
    outRemovedEqns = DoubleEnded::toListAndClear(removedEqns, metamodelica::nil())?;
    outDumpVars = DoubleEnded::toListAndClear(dumpVars, metamodelica::nil())?;
    Ok((outDAE, outDumpVars, outRemovedEqns))
}

fn getInitEqIndices(mut equations: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut indices: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut i: i32 = 1;
    for mut eq in &*equations {
        let mut eq = eq.clone();
        if BackendEquation::isInitialEquation(eq.clone())? {
            indices = metamodelica::cons(i, indices.clone());
        }
        i = i + 1;
    }
    indices = metamodelica::Dangerous::listReverseInPlace(indices);
    Ok(indices)
}

type constraintHandlerFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32, BackendDAE::Variables, DoubleEnded::MutableList<BackendDAE::Var>, DoubleEnded::MutableList<Arc<BackendDAE::Equation>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> + 'static>;

fn balanceInitialSystem(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut dummy: i32, mut initVars: BackendDAE::Variables, mut dumpVars: DoubleEnded::MutableList<BackendDAE::Var>, mut removedEqns: DoubleEnded::MutableList<Arc<BackendDAE::Equation>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut dummy: i32 = dummy;
    let mut debug: bool = false;
    let mut init_eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut sim_eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut funcs: Arc<AvlTreePathFunction::Tree>;
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut nVars: i32;
    let mut nEqns: i32;
    let mut scal_to_arr: metamodelica::Array<i32>;
    let mut var_to_eqn: metamodelica::Array<i32>;
    let mut eqn_to_var: metamodelica::Array<i32>;
    let mut changed: bool = false;
    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    let mut redundantEqns: Arc<metamodelica::List<i32>>;
    let mut unfixedVars: Arc<metamodelica::List<i32>>;
    let mut initASSC: bool = Flags::getConfigBool(Flags::INIT_ASSC.clone())?;
    if BackendVariable::varsSize(inEqSystem.orderedVars.clone()) > 0 {
        (init_eqns, sim_eqns) = List::splitOnTrue(BackendEquation::equationList(inEqSystem.orderedEqs.clone())?, (std::sync::Arc::new(BackendEquation::isInitialEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>) -> Result<bool> + 'static>))?;
        outEqSystem = BackendDAEUtil::createEqSystem(BackendVariable::sortInitialVars(inEqSystem.orderedVars.clone(), initVars.clone())?, BackendEquation::listEquation(sim_eqns)?, metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
        assign_field!(outEqSystem.removedEqs = inEqSystem.removedEqs.clone());
        funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
        (outEqSystem, m, mT, _, scal_to_arr) = BackendDAEUtil::getAdjacencyMatrixScalar(outEqSystem, openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), true)?;
        nVars = BackendVariable::varsSize(outEqSystem.orderedVars.clone());
        nEqns = BackendEquation::equationArraySize(inEqSystem.orderedEqs.clone())?;
        (eqn_to_var, var_to_eqn, _, _, _) = Matching::RegularMatching(mT.clone(), nEqns, nVars)?;
        assign_field!(outEqSystem.orderedEqs = BackendEquation::addList(init_eqns, outEqSystem.orderedEqs.clone())?);
        (outEqSystem, m, mT, _, scal_to_arr) = BackendDAEUtil::getAdjacencyMatrixScalar(outEqSystem, openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), true)?;
        (eqn_to_var, var_to_eqn, _, _, _) = Matching::ContinueMatching(mT.clone(), nEqns, nVars, eqn_to_var.clone(), var_to_eqn.clone(), false)?;
        unfixedVars = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=metamodelica::arrayLength(var_to_eqn.clone())).into_iter() {
            if !(({let __elt = var_to_eqn.borrow()[(i.clone()-1) as usize].clone(); __elt}) < 0) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        redundantEqns = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=metamodelica::arrayLength(eqn_to_var.clone())).into_iter() {
            if !(({let __elt = eqn_to_var.borrow()[(i.clone()-1) as usize].clone(); __elt}) < 0) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        if !(redundantEqns.clone().is_empty() && unfixedVars.clone().is_empty()) {
            (me, _, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(outEqSystem.clone(), inShared.clone(), false)?;
            consistencyCheck(redundantEqns.clone(), outEqSystem.orderedEqs.clone(), outEqSystem.orderedVars.clone(), inShared.clone(), 0, m.clone(), me.clone(), var_to_eqn.clone(), eqn_to_var.clone(), scal_to_arr.clone())?;
            redundantEqns = List::unique(({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (redundantEqns).into_iter().cloned() {
            let __x = ({let __elt = scal_to_arr.borrow()[(i.clone()-1) as usize].clone(); __elt});
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            outEqSystem = resolveOverAndUnderconstraints(outEqSystem, initVars.clone(), unfixedVars, redundantEqns, dumpVars.clone(), removedEqns.clone())?;
            (outEqSystem, m, mT, _, scal_to_arr) = BackendDAEUtil::getAdjacencyMatrixScalar(outEqSystem, openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), true)?;
            nVars = BackendVariable::varsSize(outEqSystem.orderedVars.clone());
            nEqns = BackendEquation::equationArraySize(outEqSystem.orderedEqs.clone())?;
            (eqn_to_var, var_to_eqn, _, _, _) = Matching::RegularMatching(mT.clone(), nEqns, nVars)?;
        } else if !(initASSC) {
            outEqSystem = inEqSystem;
        }
        if debug {
            BackendDump::dumpEqSystem(outEqSystem.clone(), (literal!("fixInitialSystem")).clone())?;
            BackendDump::dumpAdjacencyMatrixT(mT.clone())?;
            BackendDump::dumpMatchingVars(var_to_eqn.clone())?;
            BackendDump::dumpMatchingEqns(eqn_to_var.clone())?;
        }
        if initASSC {
            comps = Sorting::Tarjan(m.clone(), var_to_eqn.clone(), nEqns)?;
            for mut comp in &*comps {
                let mut comp = comp.clone();
                (eqn_to_var, var_to_eqn, outEqSystem, changed) = BackendDAEUtil::analyticalToStructuralSingularity(comp.clone(), eqn_to_var.clone(), var_to_eqn.clone(), outEqSystem.clone(), changed, true)?;
            }
            if changed {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(outEqSystem.clone()) {
                    Deref @ BackendDAE::EqSystem { m: Some(__pa0), mT: Some(__pa1), .. } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                m = __pa0.clone();
                mT = __pa1.clone();
                (outEqSystem, m, mT, _, scal_to_arr) = BackendDAEUtil::getAdjacencyMatrixScalar(outEqSystem, openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), true)?;
                (eqn_to_var, var_to_eqn, _, _, _) = Matching::ContinueMatching(mT.clone(), nEqns, nVars, eqn_to_var.clone(), var_to_eqn.clone(), false)?;
                unfixedVars = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=metamodelica::arrayLength(var_to_eqn.clone())).into_iter() {
            if !(({let __elt = var_to_eqn.borrow()[(i.clone()-1) as usize].clone(); __elt}) < 0) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                redundantEqns = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=metamodelica::arrayLength(eqn_to_var.clone())).into_iter() {
            if !(({let __elt = eqn_to_var.borrow()[(i.clone()-1) as usize].clone(); __elt}) < 0) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                redundantEqns = List::unique(({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (redundantEqns).into_iter().cloned() {
            let __x = ({let __elt = scal_to_arr.borrow()[(i.clone()-1) as usize].clone(); __elt});
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
                if !(redundantEqns.clone().is_empty() && unfixedVars.clone().is_empty()) {
                    (me, _, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(outEqSystem.clone(), inShared.clone(), false)?;
                    consistencyCheck(redundantEqns.clone(), outEqSystem.orderedEqs.clone(), outEqSystem.orderedVars.clone(), inShared, 0, m.clone(), me.clone(), var_to_eqn.clone(), eqn_to_var.clone(), scal_to_arr.clone())?;
                    outEqSystem = resolveOverAndUnderconstraints(outEqSystem, initVars, unfixedVars, redundantEqns, dumpVars, removedEqns)?;
                    (outEqSystem, m, mT, _, scal_to_arr) = BackendDAEUtil::getAdjacencyMatrixScalar(outEqSystem, openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(funcs), true)?;
                }
            }
        }
    } else {
        outEqSystem = inEqSystem;
    }
    Ok((outEqSystem, outShared, dummy))
}

fn resolveOverAndUnderconstraints(mut syst: Arc<BackendDAE::EqSystem>, mut initVars: BackendDAE::Variables, mut unfixedVars: Arc<metamodelica::List<i32>>, mut redundantEqns: Arc<metamodelica::List<i32>>, mut dumpVars: DoubleEnded::MutableList<BackendDAE::Var>, mut removedEqns: DoubleEnded::MutableList<Arc<BackendDAE::Equation>>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut syst: Arc<BackendDAE::EqSystem> = syst;
    let mut debug: bool = false;
    let mut redundant_lst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut failed_var_lst: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut var_lst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut new_eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    redundant_lst = BackendEquation::getList(redundantEqns.clone(), syst.orderedEqs.clone())?;
    DoubleEnded::push_list_back(removedEqns, redundant_lst.clone())?;
    new_eqns = BackendEquation::deleteList(syst.orderedEqs.clone(), redundantEqns)?;
    if debug {
        BackendDump::dumpEquationList(redundant_lst, (literal!("removed eqns")).clone())?;
    }
    var_lst = ({
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut i in (unfixedVars).into_iter().cloned() {
            let __x = BackendVariable::getVarAt(syst.orderedVars.clone(), i.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    (new_eqns, var_lst) = addStartValueEquations(var_lst, new_eqns, metamodelica::nil())?;
    DoubleEnded::push_list_back(dumpVars, var_lst.clone())?;
    if debug {
        failed_var_lst = ({
        let mut __acc: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
        for mut var in (var_lst.clone()).into_iter().cloned() {
            if !(!(BackendVariable::containsVar(var.clone(), initVars.clone()))) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        BackendDump::dumpVarList(var_lst, (literal!("fixed vars")).clone())?;
        BackendDump::dumpVarList(failed_var_lst, (literal!("failed vars")).clone())?;
    }
    syst = BackendDAEUtil::setEqSystEqs(syst, BackendEquation::sortInitialEqns(new_eqns)?);
    Ok(syst)
}

fn fixInitialSystem(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inShared: Arc<BackendDAE::Shared>, mut dummy: i32, mut initVars: BackendDAE::Variables, mut dumpVars: DoubleEnded::MutableList<BackendDAE::Var>, mut removedEqns: DoubleEnded::MutableList<Arc<BackendDAE::Equation>>) -> Result<(Arc<BackendDAE::EqSystem>, Arc<BackendDAE::Shared>, i32)> {
    let mut outEqSystem: Arc<BackendDAE::EqSystem>;
    let mut outShared: Arc<BackendDAE::Shared> = inShared.clone();
    let mut dummy: i32 = dummy;
    let mut eqns2: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut dumpVars2: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut removedEqns2: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut nVars: i32;
    let mut nEqns: i32;
    let mut nAddEqs: i32;
    let mut nAddVars: i32;
    let mut stateIndices: Arc<metamodelica::List<i32>>;
    let mut range: Arc<metamodelica::List<i32>>;
    let mut redundantEqns: Arc<metamodelica::List<i32>>;
    let mut initVarList: Arc<metamodelica::List<BackendDAE::Var>>;
    let mut ass1: metamodelica::Array<i32> = Default::default();
    let mut ass2: metamodelica::Array<i32> = Default::default();
    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut m_: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut syst: Arc<BackendDAE::EqSystem> = Arc::new(<BackendDAE::EqSystem as ::std::default::Default>::default());
    let mut funcs: Arc<AvlTreePathFunction::Tree>;
    let mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>;
    let mut mapIncRowEqn: metamodelica::Array<i32> = Default::default();
    let mut perfectMatching: bool;
    let mut maxMixedDeterminedIndex: i32 = intMax(0, Flags::getConfigInt(Flags::MAX_MIXED_DETERMINED_INDEX.clone())?);
    let mut eMarks: metamodelica::Array<bool> = Default::default();
    let mut vMarks: metamodelica::Array<bool> = Default::default();
    let mut singular_eqns_idx: Arc<metamodelica::List<i32>>;
    let mut singular_vars_idx: Arc<metamodelica::List<i32>>;
    let mut overDetIndex: i32;
    let mut underDetIndex: i32;
    let mut scalarEqnSize: i32;
    let mut eq: Arc<BackendDAE::Equation>;
    let debug: bool = false;
    for mut index in 0..=maxMixedDeterminedIndex {
        nVars = BackendVariable::varsSize(inEqSystem.orderedVars.clone());
        nEqns = BackendEquation::equationArraySize(inEqSystem.orderedEqs.clone())?;
        syst = BackendDAEUtil::createEqSystem(inEqSystem.orderedVars.clone(), inEqSystem.orderedEqs.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
        funcs = BackendDAEUtil::getFunctions(inShared.clone())?;
        (m_, _, _, mapIncRowEqn) = BackendDAEUtil::adjacencyMatrixScalar(syst.clone(), openmodelica_backend_types::BackendDAE::IndexType::SOLVABLE, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(inShared.clone()))?;
        if debug {
            BackendDump::dumpEqSystem(syst.clone(), (literal!("fixInitialSystem")).clone())?;
            BackendDump::dumpVariables(initVars.clone(), (literal!("selected initialization variables")).clone())?;
            BackendDump::dumpVariables(inEqSystem.orderedVars.clone(), (literal!("vars in the system")).clone())?;
            BackendDump::dumpAdjacencyMatrix(m_.clone())?;
        }
        stateIndices = BackendVariable::getVarIndexFromVariablesIndexInFirstSet(inEqSystem.orderedVars.clone(), initVars.clone())?;
        nAddEqs = intMax(nVars - nEqns + index.clone(), index.clone());
        if debug {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("nAddEqs: ")); __mm_s.push_str(&*intString(nAddEqs)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        m = fixUnderDeterminedSystem(m_.clone(), stateIndices.clone(), nEqns, nAddEqs)?;
        nAddVars = intMax(nEqns - nVars + index.clone(), index.clone());
        if debug {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("nAddVars: ")); __mm_s.push_str(&*intString(nAddVars)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        m = fixOverDeterminedSystem(m.clone(), inEqSystem.orderedEqs.clone(), nVars, nAddVars)?;
        (ass1, ass2, perfectMatching, eMarks, vMarks) = Matching::RegularMatching(m.clone(), nVars + nAddVars, nEqns + nAddEqs)?;
        if debug {
            BackendDump::dumpMatchingVars(ass1.clone())?;
            BackendDump::dumpMatchingEqns(ass2.clone())?;
        }
        if perfectMatching {
            if index.clone() > 0 {
                Error::addCompilerNotification(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The given system is mixed-determined.   [index = ")); __mm_s.push_str(&*intString(index.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone())?;
            }
            if nAddVars > 0 {
                range = List::intRange2(nVars + 1, nVars + nAddVars);
                redundantEqns = mapIndices(range.clone(), ass1.clone())?;
                (me, _, _, _) = BackendDAEUtil::getAdjacencyMatrixEnhancedScalar(syst.clone(), inShared.clone(), false)?;
                consistencyCheck(redundantEqns.clone(), inEqSystem.orderedEqs.clone(), inEqSystem.orderedVars.clone(), inShared.clone(), nAddVars, m_.clone(), me.clone(), ass1.clone(), ass2.clone(), mapIncRowEqn.clone())?;
                removedEqns2 = BackendEquation::getList(redundantEqns.clone(), inEqSystem.orderedEqs.clone())?;
                eqns2 = BackendEquation::deleteList(inEqSystem.orderedEqs.clone(), redundantEqns.clone())?;
                DoubleEnded::push_list_back(removedEqns.clone(), removedEqns2.clone())?;
            } else {
                eqns2 = inEqSystem.orderedEqs.clone();
            }
            if nAddEqs > 0 {
                range = List::intRange2(nEqns + 1, nEqns + nAddEqs);
                range = mapIndices(range.clone(), ass2.clone())?;
                initVarList = List::map1r(range.clone(), (std::sync::Arc::new(BackendVariable::getVarAt) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Variables, i32) -> Result<BackendDAE::Var> + 'static>), inEqSystem.orderedVars.clone())?;
                (eqns2, dumpVars2) = addStartValueEquations(initVarList.clone(), eqns2.clone(), metamodelica::nil())?;
                DoubleEnded::push_list_back(dumpVars.clone(), dumpVars2.clone())?;
            }
            outEqSystem = BackendDAEUtil::setEqSystEqs(inEqSystem.clone(), eqns2.clone());
            return Ok((outEqSystem.clone(), outShared.clone(), dummy.clone()));
        }
        if debug {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("index-")); __mm_s.push_str(&*intString(index.clone())); __mm_s.push_str(&*literal!(" ende\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    if Flags::isSet(Flags::INITIALIZATION.clone())? {
        overDetIndex = (({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=metamodelica::arrayLength(ass1.clone())).into_iter() {
            if !(({let __elt = ass1.borrow()[(i.clone()-1) as usize].clone(); __elt}) < 0) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).len() as i32);
        underDetIndex = (({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=metamodelica::arrayLength(ass2.clone())).into_iter() {
            if !(({let __elt = ass2.borrow()[(i.clone()-1) as usize].clone(); __elt}) < 0) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).len() as i32);
        singular_eqns_idx = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=metamodelica::arrayLength(mapIncRowEqn.clone())).into_iter() {
            if !(({let __elt = eMarks.borrow()[(i.clone()-1) as usize].clone(); __elt})) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        singular_vars_idx = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=BackendVariable::varsSize(syst.orderedVars.clone())).into_iter() {
            if !(({let __elt = vMarks.borrow()[(i.clone()-1) as usize].clone(); __elt})) { continue; }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        scalarEqnSize = (singular_eqns_idx.clone().len() as i32);
        singular_eqns_idx = List::uniqueOnTrue(({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (singular_eqns_idx).into_iter().cloned() {
            let __x = ({let __elt = mapIncRowEqn.borrow()[(i.clone()-1) as usize].clone(); __elt});
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        metamodelica::print((literal!("\n------------ UNBALANCED INITIAL SYSTEM ------------\n")).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The initial system is over- as well as underdetermined and it could not be resolved after ")); __mm_s.push_str(&*intString(maxMixedDeterminedIndex)); __mm_s.push_str(&*literal!(" iterations.\n\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("==== OVERDETERMINATION BY ")); __mm_s.push_str(&*intString(overDetIndex)); __mm_s.push_str(&*literal!(" EQUATION(S)\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("==== UNDERDETERMINATION OF ")); __mm_s.push_str(&*intString(underDetIndex)); __mm_s.push_str(&*literal!(" VARIABLE(S)\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n---- involved set eqns (")); __mm_s.push_str(&*intString(scalarEqnSize)); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString((singular_eqns_idx.clone().len() as i32))); __mm_s.push_str(&*literal!("):\n")); ArcStr::from(__mm_s) }).clone());
        for mut eqn in &*singular_eqns_idx {
            let mut eqn = eqn.clone();
            eq = BackendEquation::get(syst.orderedEqs.clone(), ({let __elt = mapIncRowEqn.borrow()[(eqn.clone()-1) as usize].clone(); __elt}))?;
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*intString(eqn.clone())); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString(BackendEquation::equationSize(eq.clone())?)); __mm_s.push_str(&*literal!("):\t")); __mm_s.push_str(&*BackendDump::equationString(eq.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n---- involved set vars (")); __mm_s.push_str(&*intString((singular_vars_idx.clone().len() as i32))); __mm_s.push_str(&*literal!("):\n")); ArcStr::from(__mm_s) }).clone());
        for mut var in &*singular_vars_idx {
            let mut var = var.clone();
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*intString(var.clone())); __mm_s.push_str(&*literal!(":\t")); __mm_s.push_str(&*BackendDump::varString(BackendVariable::getVarAt(syst.orderedVars.clone(), var.clone())?)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        metamodelica::print((literal!("--------------------------------------------------\n")).clone());
    }
    Error::addMessage(Error::MIXED_DETERMINED.clone(), list![(intString(maxMixedDeterminedIndex)).clone()])?;
    bail!("fail");
    Ok((outEqSystem, outShared, dummy))
}

fn updateFixedAttribute(mut var: BackendDAE::Var, mut vars: BackendDAE::Variables) -> Result<(BackendDAE::Var, BackendDAE::Variables)> {
    let mut var: BackendDAE::Var = var;
    let mut vars: BackendDAE::Variables = vars;
    let mut cr: Arc<DAE::ComponentRef>;
    cr = BackendVariable::varCref(var.clone())?;
    if BackendVariable::containsCref(cr, vars.clone()) {
        var = BackendVariable::setVarFixed(var, true)?;
    }
    Ok((var, vars))
}

fn fixUnderDeterminedSystem(mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inInitVarIndices: Arc<metamodelica::List<i32>>, mut inNEqns: i32, mut inNAddEqns: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut newEqIndices: Arc<metamodelica::List<i32>>;
    if inNAddEqns < 0 {
        Error::addInternalError((literal!("function fixUnderDeterminedSystem failed due to invalid input")).clone(), metamodelica::sourceInfo!("BackEnd/Initialization.mo"))?;
        bail!("fail");
    }
    if inNAddEqns > 0 {
        outM = arrayCreate(inNEqns + inNAddEqns, metamodelica::nil());
        outM = Array::copy(inM.clone(), outM.clone())?;
        newEqIndices = List::intRange2(inNEqns + 1, inNEqns + inNAddEqns);
        outM = List::fold1(newEqIndices, (std::sync::Arc::new(fnptr!(squareAdjacencyMatrix1, i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), inInitVarIndices, outM.clone())?;
    } else {
        outM = metamodelica::arrayFromVec(inM.clone().borrow().clone());
    }
    Ok(outM)
}

fn squareAdjacencyMatrix1(mut inPos: i32, mut inDependency: Arc<metamodelica::List<i32>>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> metamodelica::Array<Arc<metamodelica::List<i32>>> {
    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>> = inM.clone();
    {
        let __cell0 = inDependency;
        let __idx0 = inPos;
        outM.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
    }
    outM
}

fn fixOverDeterminedSystem(mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut orderedEqs: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inNVars: i32, mut inNAddVars: i32) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>>;
    let mut newVarIndices: Arc<metamodelica::List<i32>>;
    let mut initEqsIndices: Arc<metamodelica::List<i32>>;
    if inNAddVars < 0 {
        Error::addInternalError((literal!("function fixOverDeterminedSystem failed due to invalid input")).clone(), metamodelica::sourceInfo!("BackEnd/Initialization.mo"))?;
        bail!("fail");
    }
    if inNAddVars > 0 {
        initEqsIndices = getInitEqIndices(BackendEquation::equationList(orderedEqs)?)?;
        newVarIndices = List::intRange2(inNVars + 1, inNVars + inNAddVars);
        outM = List::fold1(initEqsIndices, (std::sync::Arc::new(fnptr!(squareAdjacencyMatrix2, i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, Arc<metamodelica::List<i32>>, metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> + 'static>), newVarIndices, inM.clone())?;
    } else {
        outM = inM.clone();
    }
    Ok(outM)
}

fn squareAdjacencyMatrix2(mut inPos: i32, mut inRange: Arc<metamodelica::List<i32>>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> metamodelica::Array<Arc<metamodelica::List<i32>>> {
    let mut outM: metamodelica::Array<Arc<metamodelica::List<i32>>> = inM.clone();
    {
        let __cell0 = listAppend(({let __elt = inM.borrow()[(inPos-1) as usize].clone(); __elt}), inRange);
        let __idx0 = inPos;
        outM.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
    }
    outM
}

fn addStartValueEquations(mut inVarLst: Arc<metamodelica::List<BackendDAE::Var>>, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inDumpVars: Arc<metamodelica::List<BackendDAE::Var>>) -> Result<(Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<metamodelica::List<BackendDAE::Var>>)> {
    let mut outEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = inEqns.clone();
    let mut outDumpVars: Arc<metamodelica::List<BackendDAE::Var>> = inDumpVars.clone();
    let mut dumpVar: BackendDAE::Var;
    let mut eqn: Arc<BackendDAE::Equation>;
    let mut e: Arc<DAE::Exp>;
    let mut crefExp: Arc<DAE::Exp>;
    let mut startExp: Arc<DAE::Exp>;
    let mut cref: Arc<DAE::ComponentRef>;
    let mut tp: Arc<DAE::Type>;
    let mut isPreCref: bool;
    for mut var in &*inVarLst {
        let mut var = var.clone();
        cref = BackendVariable::varCref(var.clone())?;
        tp = BackendVariable::varType(var.clone())?;
        crefExp = Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: tp.clone() });
        isPreCref = ComponentReference::isPreCref(cref.clone());
        if isPreCref {
            cref = ComponentReference::popPreCref(cref.clone());
        }
        e = Expression::crefExp(cref.clone())?;
        tp = Expression::r#typeof(e.clone())?;
        startExp = Expression::crefExp(ComponentReference::crefPrefixStart(cref.clone()))?;
        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
        outEqns = BackendEquation::add(eqn.clone(), outEqns.clone())?;
        if isPreCref {
            dumpVar = BackendVariable::copyVarNewName(cref.clone(), var.clone());
            dumpVar = BackendVariable::setVarFixed(dumpVar.clone(), true)?;
            outDumpVars = metamodelica::cons(dumpVar.clone(), outDumpVars.clone());
        } else {
            dumpVar = BackendVariable::setVarFixed(var.clone(), true)?;
            outDumpVars = metamodelica::cons(dumpVar.clone(), outDumpVars.clone());
        }
    }
    Ok((outEqns, outDumpVars))
}

// =============================================================================
// section for symbolic consistency check
//
// =============================================================================
fn consistencyCheck(mut inRedundantEqns: Arc<metamodelica::List<i32>>, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables, mut inShared: Arc<BackendDAE::Shared>, mut nAddVars: i32, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut me: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut vecVarToEqs: metamodelica::Array<i32>, mut vecEqsToVar: metamodelica::Array<i32>, mut mapIncRowEqn: metamodelica::Array<i32>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut outConsistentEquations: Arc<metamodelica::List<i32>>;
    let mut outInconsistentEquations: Arc<metamodelica::List<i32>>;
    let mut outUncheckedEquations: Arc<metamodelica::List<i32>>;
    (outConsistentEquations, outInconsistentEquations, outUncheckedEquations) = 'mc: {
        let __mc_input = inRedundantEqns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currRedundantEqn, tail: restRedundantEqns } => {
                    let mut outRange: Arc<metamodelica::List<i32>>;
                    let mut flatComps: Arc<metamodelica::List<i32>>;
                    let mut markedComps: Arc<metamodelica::List<i32>>;
                    let mut outLoopListComps: Arc<metamodelica::List<i32>>;
                    let mut consistentEquations: Arc<metamodelica::List<i32>>;
                    let mut consistentEquations2: Arc<metamodelica::List<i32>>;
                    let mut inconsistentEquations: Arc<metamodelica::List<i32>>;
                    let mut uncheckedEquations: Arc<metamodelica::List<i32>>;
                    let mut uncheckedEquations2: Arc<metamodelica::List<i32>>;
                    let mut nEqns: i32;
                    let mut redundantEqn: i32;
                    let mut comps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut substEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    nEqns = BackendEquation::equationArraySize(inEqns.clone())?;
                    comps = Sorting::Tarjan(inM.clone(), vecVarToEqs.clone(), nEqns.clone())?;
                    flatComps = List::flatten(comps.clone())?;
                    (_, outLoopListComps) = splitStrongComponents(comps.clone());
                    redundantEqn = mapIndex(currRedundantEqn.clone(), mapIncRowEqn.clone());
                    flatComps = mapIndices(flatComps.clone(), mapIncRowEqn.clone())?;
                    outLoopListComps = mapIndices(outLoopListComps.clone(), mapIncRowEqn.clone())?;
                    markedComps = compsMarker(currRedundantEqn.clone(), vecVarToEqs.clone(), inM.clone(), flatComps.clone(), outLoopListComps.clone())?;
                    repl = BackendVarTransform::emptyReplacements();
                    repl = setupVarReplacements(markedComps.clone(), inEqns.clone(), inVars.clone(), vecEqsToVar.clone(), repl.clone(), mapIncRowEqn.clone(), me.clone(), inShared.clone());
                    substEqns = applyVarReplacements(redundantEqn.clone(), inEqns.clone(), repl.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getConsistentEquation(redundantEqn.clone(), substEqns.clone(), inEqns.clone(), inM.clone(), vecVarToEqs.clone(), inVars.clone(), inShared.clone(), 1)?) {
                        (__pa0, true, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    outRange = __pa0.clone();
                    uncheckedEquations = __pa1.clone();
                    (consistentEquations, inconsistentEquations, uncheckedEquations2) = consistencyCheck(restRedundantEqns.clone(), inEqns.clone(), inVars.clone(), inShared.clone(), nAddVars, inM.clone(), me.clone(), vecVarToEqs.clone(), vecEqsToVar.clone(), mapIncRowEqn.clone())?;
                    consistentEquations2 = listAppend(consistentEquations.clone(), outRange.clone());
                    uncheckedEquations2 = listAppend(uncheckedEquations.clone(), uncheckedEquations2.clone());
                    Ok((consistentEquations2.clone(), inconsistentEquations.clone(), uncheckedEquations2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: currRedundantEqn, tail: restRedundantEqns } => {
                    let mut consistentEquations: Arc<metamodelica::List<i32>>;
                    let mut inconsistentEquations: Arc<metamodelica::List<i32>>;
                    let mut uncheckedEquations: Arc<metamodelica::List<i32>>;
                    (consistentEquations, inconsistentEquations, uncheckedEquations) = consistencyCheck(restRedundantEqns.clone(), inEqns.clone(), inVars.clone(), inShared.clone(), nAddVars, inM.clone(), me.clone(), vecVarToEqs.clone(), vecEqsToVar.clone(), mapIncRowEqn.clone())?;
                    Ok((consistentEquations.clone(), metamodelica::cons(currRedundantEqn.clone(), inconsistentEquations.clone()), uncheckedEquations.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outConsistentEquations, outInconsistentEquations, outUncheckedEquations))
}

fn isVarExplicitSolvable(mut inElem: Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>, mut inVarID: i32) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inElem) {
        Deref @ metamodelica::List::Nil => {
            return true
        },
        Deref @ metamodelica::List::Cons { head: (id, BackendDAE::Solvability::SOLVABILITY_UNSOLVABLE { .. }, _), tail: _ } if (intEq(id.clone(), inVarID)) => {
            return false
        },
        Deref @ metamodelica::List::Cons { head: (id, BackendDAE::Solvability::SOLVABILITY_NONLINEAR { .. }, _), tail: _ } if (intEq(id.clone(), inVarID)) => {
            return false
        },
        Deref @ metamodelica::List::Cons { head: (_, _, _), tail: elem } => {
            let mut b: bool;
            { (inElem, inVarID) = (elem.clone(), inVarID); continue '__tco; }
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn splitStrongComponents(mut inComps: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> (Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>) {
    let mut outListComps: Arc<metamodelica::List<i32>>;
    let mut outLoopListComps: Arc<metamodelica::List<i32>>;
    (outListComps, outLoopListComps) = (::match_deref::match_deref! { match &(inComps) {
        Deref @ metamodelica::List::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Cons { head: currIndex, tail: Deref @ metamodelica::List::Nil }, tail: restComps } => {
            let mut listComps: Arc<metamodelica::List<i32>>;
            let mut loopListComps: Arc<metamodelica::List<i32>>;
            (listComps, loopListComps) = splitStrongComponents(restComps.clone());
            (metamodelica::cons(currIndex.clone(), listComps), loopListComps)
        },
        Deref @ metamodelica::List::Cons { head: currComp, tail: restComps } => {
            let mut listComps: Arc<metamodelica::List<i32>>;
            let mut loopListComps: Arc<metamodelica::List<i32>>;
            (listComps, loopListComps) = splitStrongComponents(restComps.clone());
            loopListComps = listAppend(currComp.clone(), loopListComps);
            (listComps, loopListComps)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outListComps, outLoopListComps)
}

fn mapIndex(mut inIndex: i32, mut inMapping: metamodelica::Array<i32>) -> i32 {
    let mut outIndex: i32;
    outIndex = ({let __elt = inMapping.borrow()[(inIndex-1) as usize].clone(); __elt});
    outIndex
}

fn mapIndices(mut inIndices: Arc<metamodelica::List<i32>>, mut inMapping: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outIndices: Arc<metamodelica::List<i32>>;
    outIndices = List::map1(inIndices, (std::sync::Arc::new(fnptr!(mapIndex, i32, metamodelica::Array<i32>)) as std::sync::Arc<dyn ::std::ops::Fn(i32, metamodelica::Array<i32>) -> Result<i32> + 'static>), inMapping.clone())?;
    Ok(outIndices)
}

fn mapListIndices(mut inListIndices: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut inMapping: metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>> {
    let mut outListIndices: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
    outListIndices = List::map1(inListIndices, (std::sync::Arc::new(mapIndices) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>, metamodelica::Array<i32>) -> Result<Arc<metamodelica::List<i32>>> + 'static>), inMapping.clone())?;
    Ok(outListIndices)
}

fn compsMarker(mut inUnassignedEqn: i32, mut inVecVarToEq: metamodelica::Array<i32>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inFlatComps: Arc<metamodelica::List<i32>>, mut inLoopListComps: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outMarkedEqns: Arc<metamodelica::List<i32>>;
    let mut varList: Arc<metamodelica::List<i32>>;
    let mut markedEqns: Arc<metamodelica::List<i32>>;
    match '__try0: {
        let false = (listMember(inUnassignedEqn, inLoopListComps.clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        varList = ({let __elt = inM.borrow()[(inUnassignedEqn-1) as usize].clone(); __elt});
        markedEqns = unwrap_break_err!(compsMarker2(varList.clone(), inVecVarToEq.clone(), inM.clone(), inFlatComps.clone(), metamodelica::nil(), inLoopListComps.clone()), '__try0);
        outMarkedEqns = unwrap_break_err!(downCompsMarker(inFlatComps.clone().reverse(), inVecVarToEq.clone(), inM.clone(), inFlatComps.clone(), markedEqns.clone(), inLoopListComps.clone()), '__try0);
        Ok::<_, anyhow::Error>((markedEqns.clone(), outMarkedEqns.clone(), varList.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            markedEqns = __try0_o0;
            outMarkedEqns = __try0_o1;
            varList = __try0_o2;
        }
        Err(__try0_err) => {
            Error::addCompilerNotification((literal!("It was not possible to check the given initialization system for consistency symbolically, because the relevant equations are part of an algebraic loop. This is not supported yet.")).clone())?;
            return Err(__try0_err);
        }
    }
    Ok(outMarkedEqns)
}

fn compsMarker2(mut inVarList: Arc<metamodelica::List<i32>>, mut inVecVarToEq: metamodelica::Array<i32>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut inFlatComps: Arc<metamodelica::List<i32>>, mut inMarkedEqns: Arc<metamodelica::List<i32>>, mut inLoopListComps: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outMarkedEqns: Arc<metamodelica::List<i32>>;
    outMarkedEqns = 'mc: {
        let __mc_input = inVarList;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inMarkedEqns.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: indexVar, tail: var_list2 } => {
                    let mut indexEq: i32;
                    let mut markedEqns: Arc<metamodelica::List<i32>>;
                    indexEq = ({let __elt = inVecVarToEq.borrow()[(indexVar.clone()-1) as usize].clone(); __elt});
                    let false = (listMember(indexEq.clone(), inLoopListComps.clone())) else { bail!("pattern mismatch") };
                    let false = (listMember(indexEq.clone(), inMarkedEqns.clone())) else { bail!("pattern mismatch") };
                    markedEqns = compsMarker2(var_list2.clone(), inVecVarToEq.clone(), inM.clone(), inFlatComps.clone(), inMarkedEqns.clone(), inLoopListComps.clone())?;
                    Ok(metamodelica::cons(indexEq.clone(), markedEqns.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: indexVar, tail: var_list2 } => {
                    let mut indexEq: i32;
                    let mut markedEqns: Arc<metamodelica::List<i32>>;
                    indexEq = ({let __elt = inVecVarToEq.borrow()[(indexVar.clone()-1) as usize].clone(); __elt});
                    let false = (listMember(indexEq.clone(), inLoopListComps.clone())) else { bail!("pattern mismatch") };
                    let true = (listMember(indexEq.clone(), inMarkedEqns.clone())) else { bail!("pattern mismatch") };
                    markedEqns = compsMarker2(var_list2.clone(), inVecVarToEq.clone(), inM.clone(), inFlatComps.clone(), inMarkedEqns.clone(), inLoopListComps.clone())?;
                    Ok(markedEqns.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addCompilerNotification((literal!("It was not possible to check the given initialization system for consistency symbolically, because the relevant equations are part of an algebraic loop. This is not supported yet.")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outMarkedEqns)
}

fn downCompsMarker(mut unassignedEqns: Arc<metamodelica::List<i32>>, mut vecVarToEq: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut flatComps: Arc<metamodelica::List<i32>>, mut inMarkedEqns: Arc<metamodelica::List<i32>>, mut inLoopListComps: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut inMarkedEqns: Arc<metamodelica::List<i32>> = inMarkedEqns;
    for mut indexUnassigned in &*unassignedEqns {
        let mut indexUnassigned = indexUnassigned.clone();
        if listMember(indexUnassigned.clone(), inMarkedEqns.clone()) {
            inMarkedEqns = compsMarker2(({let __elt = m.borrow()[(indexUnassigned.clone()-1) as usize].clone(); __elt}), vecVarToEq.clone(), m.clone(), flatComps.clone(), inMarkedEqns.clone(), inLoopListComps.clone())?;
        }
    }
    Ok(inMarkedEqns)
}

fn setupVarReplacements(mut inMarkedEqns: Arc<metamodelica::List<i32>>, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inVars: BackendDAE::Variables, mut inVecEqToVar: metamodelica::Array<i32>, mut inRepls: BackendVarTransform::VariableReplacements, mut inMapIncRowEqn: metamodelica::Array<i32>, mut inME: metamodelica::Array<Arc<metamodelica::List<(i32, BackendDAE::Solvability, Arc<metamodelica::List<Arc<DAE::Constraint>>>)>>>, mut inShared: Arc<BackendDAE::Shared>) -> BackendVarTransform::VariableReplacements {
    let mut outRepls: BackendVarTransform::VariableReplacements;
    outRepls = 'mc: {
        let __mc_input = inMarkedEqns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(inRepls.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: markedEqn, tail: markedEqns } => {
                    let mut indexVar: i32;
                    let mut indexEq: i32;
                    let mut repls: BackendVarTransform::VariableReplacements;
                    let mut var: BackendDAE::Var;
                    let mut varName: Arc<DAE::ComponentRef>;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut cref: Arc<DAE::ComponentRef>;
                    let mut type_: Arc<DAE::Type>;
                    let mut exp: Arc<DAE::Exp>;
                    let mut exp1: Arc<DAE::Exp>;
                    let mut x: Arc<DAE::Exp>;
                    indexVar = ({let __elt = inVecEqToVar.borrow()[(markedEqn.clone()-1) as usize].clone(); __elt});
                    let true = (isVarExplicitSolvable(({let __elt = inME.borrow()[(markedEqn.clone()-1) as usize].clone(); __elt}), indexVar.clone())) else { bail!("pattern mismatch") };
                    var = BackendVariable::getVarAt(inVars.clone(), indexVar.clone())?;
                    indexEq = ({let __elt = inMapIncRowEqn.borrow()[(markedEqn.clone()-1) as usize].clone(); __elt});
                    eqn = BackendEquation::get(inEqns.clone(), indexEq.clone())?;
                    cref = BackendVariable::varCref(var.clone())?;
                    type_ = BackendVariable::varType(var.clone())?;
                    x = Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: type_.clone() });
                    let (__pa1, __pa0) = ::match_deref::match_deref! { match &(BackendEquation::solveEquation(eqn.clone(), x.clone(), Some(inShared.functionTree.clone()))?) {
                        __pa1 @ Deref @ BackendDAE::Equation::EQUATION { scalar: __pa0, .. } => (__pa1.clone(), __pa0.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    exp = __pa0.clone();
                    eqn = __pa1.clone();
                    varName = BackendVariable::varCref(var.clone())?;
                    (exp1, _) = Expression::traverseExpBottomUp(exp.clone(), (std::sync::Arc::new(fnptr!(BackendDAEUtil::replaceCrefsWithValues, Arc<DAE::Exp>, (BackendDAE::Variables, Arc<DAE::ComponentRef>))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<DAE::ComponentRef>)) -> Result<(Arc<DAE::Exp>, (BackendDAE::Variables, Arc<DAE::ComponentRef>))> + 'static>), (inVars.clone(), varName.clone()))?;
                    repls = BackendVarTransform::addReplacement(inRepls.clone(), varName.clone(), exp1.clone(), None)?;
                    repls = setupVarReplacements(markedEqns.clone(), inEqns.clone(), inVars.clone(), inVecEqToVar.clone(), repls.clone(), inMapIncRowEqn.clone(), inME.clone(), inShared.clone());
                    Ok(repls.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: markedEqns } => {
                    let mut repls: BackendVarTransform::VariableReplacements;
                    repls = setupVarReplacements(markedEqns.clone(), inEqns.clone(), inVars.clone(), inVecEqToVar.clone(), inRepls.clone(), inMapIncRowEqn.clone(), inME.clone(), inShared.clone());
                    Ok(repls.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outRepls
}

fn applyVarReplacements(mut inEqnIndex: i32, mut inEqnList: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inVarRepls: BackendVarTransform::VariableReplacements) -> Result<Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>> {
    let mut outEqnList: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut eqn: Arc<BackendDAE::Equation>;
    outEqnList = BackendEquation::copyEquationArray(inEqnList);
    eqn = BackendEquation::get(outEqnList.clone(), inEqnIndex)?;
    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceEquations(list![eqn], inVarRepls, None)?) {
        (Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }, _) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqn = __pa0.clone();
    outEqnList = BackendEquation::setAtIndex(outEqnList, inEqnIndex, eqn)?;
    Ok(outEqnList)
}

fn getConsistentEquation(mut inUnassignedEqn: i32, mut inEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inEqnsOrig: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut inM: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut vecVarToEqs: metamodelica::Array<i32>, mut vars: BackendDAE::Variables, mut shared: Arc<BackendDAE::Shared>, mut counter: i32) -> Result<(Arc<metamodelica::List<i32>>, bool, Arc<metamodelica::List<i32>>)> {
    let mut outUnassignedEqns: Arc<metamodelica::List<i32>>;
    let mut outConsistent: bool;
    let mut outRemovedEqns: Arc<metamodelica::List<i32>>;
    (outUnassignedEqns, outConsistent, outRemovedEqns) = 'mc: {
        let __mc_input = inUnassignedEqn;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nVars: i32;
            let mut nEqns: i32;
            let mut eqn: Arc<BackendDAE::Equation>;
            let mut lhs: Arc<DAE::Exp>;
            let mut rhs: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            nVars = BackendVariable::varsSize(vars.clone());
            nEqns = BackendEquation::equationArraySize(inEqnsOrig.clone())?;
            let true = (intLe(counter, nEqns.clone() - nVars.clone())) else { bail!("pattern mismatch") };
            eqn = BackendEquation::get(inEqns.clone(), inUnassignedEqn)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            lhs = __pa0.clone();
            rhs = __pa1.clone();
            exp = Arc::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: rhs.clone() });
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            let true = (Expression::isZero(exp.clone())?) else { bail!("pattern mismatch") };
            BackendEquation::get(inEqnsOrig.clone(), inUnassignedEqn)?;
            Ok((list![inUnassignedEqn], true, metamodelica::nil()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nVars: i32;
            let mut nEqns: i32;
            nVars = BackendVariable::varsSize(vars.clone());
            nEqns = BackendEquation::equationArraySize(inEqnsOrig.clone())?;
            let true = (intGt(counter, nEqns.clone() - nVars.clone())) else { bail!("pattern mismatch") };
            Error::addCompilerError((literal!("Initialization problem is structural singular. Please, check the initial conditions.")).clone())?;
            Ok((metamodelica::nil(), true, metamodelica::nil()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nVars: i32;
            let mut nEqns: i32;
            let mut eqn: Arc<BackendDAE::Equation>;
            let mut eqn2: Arc<BackendDAE::Equation>;
            let mut lhs: Arc<DAE::Exp>;
            let mut rhs: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            let mut listParameter: Arc<metamodelica::List<ArcStr>>;
            nVars = BackendVariable::varsSize(vars.clone());
            nEqns = BackendEquation::equationArraySize(inEqnsOrig.clone())?;
            let true = (intLe(counter, nEqns.clone() - nVars.clone())) else { bail!("pattern mismatch") };
            eqn = BackendEquation::get(inEqns.clone(), inUnassignedEqn)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            lhs = __pa0.clone();
            rhs = __pa1.clone();
            exp = Arc::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: rhs.clone() });
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            let false = (Expression::isZero(exp.clone())?) else { bail!("pattern mismatch") };
            let __pa2 = ::match_deref::match_deref! { match &(parameterCheck(exp.clone())?) {
                (__pa2, false) => __pa2.clone(),
                _ => bail!("pattern mismatch"),
            } };
            listParameter = __pa2.clone();
            let true = (listParameter.clone().is_empty()) else { bail!("pattern mismatch") };
            eqn2 = BackendEquation::get(inEqnsOrig.clone(), inUnassignedEqn)?;
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The initialization problem is inconsistent due to the following equation: ")); __mm_s.push_str(&*BackendDump::equationString(eqn2.clone())?); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*BackendDump::equationString(eqn.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
            Ok((metamodelica::nil(), false, metamodelica::nil()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nVars: i32;
            let mut nEqns: i32;
            let mut listVar: Arc<metamodelica::List<i32>>;
            let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
            let mut eqn: Arc<BackendDAE::Equation>;
            let mut lhs: Arc<DAE::Exp>;
            let mut rhs: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            let mut listParameter: Arc<metamodelica::List<ArcStr>>;
            let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
            let mut system: Arc<BackendDAE::EqSystem>;
            let mut funcs: Arc<AvlTreePathFunction::Tree>;
            let mut list_inEqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
            nVars = BackendVariable::varsSize(vars.clone());
            nEqns = BackendEquation::equationArraySize(inEqnsOrig.clone())?;
            let true = (intLe(counter, nEqns.clone() - nVars.clone())) else { bail!("pattern mismatch") };
            eqn = BackendEquation::get(inEqns.clone(), inUnassignedEqn)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            lhs = __pa0.clone();
            rhs = __pa1.clone();
            exp = Arc::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: rhs.clone() });
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            let false = (Expression::isZero(exp.clone())?) else { bail!("pattern mismatch") };
            (listParameter, _) = parameterCheck(exp.clone())?;
            let false = (listParameter.clone().is_empty()) else { bail!("pattern mismatch") };
            list_inEqns = BackendEquation::equationList(inEqns.clone())?;
            list_inEqns = List::set(list_inEqns.clone(), inUnassignedEqn, eqn.clone())?;
            eqns = BackendEquation::listEquation(list_inEqns.clone())?;
            funcs = BackendDAEUtil::getFunctions(shared.clone())?;
            system = BackendDAEUtil::createEqSystem(vars.clone(), eqns.clone(), metamodelica::nil(), openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns());
            (m, _) = BackendDAEUtil::adjacencyMatrix(system.clone(), openmodelica_backend_types::BackendDAE::IndexType::NORMAL, Some(funcs.clone()), BackendDAEUtil::isInitializationDAE(shared.clone()))?;
            listVar = ({let __elt = m.borrow()[(inUnassignedEqn-1) as usize].clone(); __elt});
            let false = (listVar.clone().is_empty()) else { bail!("pattern mismatch") };
            BackendEquation::get(inEqnsOrig.clone(), inUnassignedEqn)?;
            Error::addCompilerNotification((literal!("It was not possible to check the given initialization system for consistency symbolically, because the relevant equations are part of an algebraic loop. This is not supported yet.")).clone())?;
            Ok((metamodelica::nil(), false, metamodelica::nil()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut nVars: i32;
            let mut nEqns: i32;
            let mut eqn: Arc<BackendDAE::Equation>;
            let mut eqn2: Arc<BackendDAE::Equation>;
            let mut lhs: Arc<DAE::Exp>;
            let mut rhs: Arc<DAE::Exp>;
            let mut exp: Arc<DAE::Exp>;
            let mut listParameter: Arc<metamodelica::List<ArcStr>>;
            let mut anyStartValue: bool;
            nVars = BackendVariable::varsSize(vars.clone());
            nEqns = BackendEquation::equationArraySize(inEqnsOrig.clone())?;
            let true = (intLe(counter, nEqns.clone() - nVars.clone())) else { bail!("pattern mismatch") };
            eqn = BackendEquation::get(inEqns.clone(), inUnassignedEqn)?;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eqn.clone()) {
                Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            lhs = __pa0.clone();
            rhs = __pa1.clone();
            exp = Arc::new(DAE::Exp::BINARY { exp1: lhs.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: rhs.clone() });
            (exp, _) = ExpressionSimplify::simplify(exp.clone())?;
            let false = (Expression::isZero(exp.clone())?) else { bail!("pattern mismatch") };
            (listParameter, anyStartValue) = parameterCheck(exp.clone())?;
            let true = (!(listParameter.clone().is_empty()) || anyStartValue.clone()) else { bail!("pattern mismatch") };
            eqn2 = BackendEquation::get(inEqnsOrig.clone(), inUnassignedEqn)?;
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("It was not possible to determine if the initialization problem is consistent, because of not evaluable parameters/start values during compile time: ")); __mm_s.push_str(&*BackendDump::equationString(eqn2.clone())?); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*BackendDump::equationString(eqn.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
            Ok((metamodelica::nil(), true, list![inUnassignedEqn]))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outUnassignedEqns, outConsistent, outRemovedEqns))
}

fn parameterCheck(mut inExp: Arc<DAE::Exp>) -> Result<(Arc<metamodelica::List<ArcStr>>, bool)> {
    let mut outParameters: Arc<metamodelica::List<ArcStr>>;
    let mut outAnyStartValue: bool;
    let (_, (__pa0, __pa1)) = Expression::traverseExpTopDown(inExp, (std::sync::Arc::new(parameterCheck2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<ArcStr>>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, bool))> + 'static>), (metamodelica::nil(), false))?;
    outParameters = __pa0.clone();
    outAnyStartValue = __pa1.clone();
    Ok((outParameters, outAnyStartValue))
}

fn parameterCheck2(mut inExp: Arc<DAE::Exp>, mut inParams: (Arc<metamodelica::List<ArcStr>>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<ArcStr>>, bool))> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outContinue: bool;
    let mut outParams: (Arc<metamodelica::List<ArcStr>>, bool);
    let mut componentRef: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut parameters: Arc<metamodelica::List<ArcStr>>;
    let mut anyStartValue: bool;
    (parameters, anyStartValue) = inParams.clone();
    (outParams, outContinue) = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CREF { componentRef: __esc_componentRef, .. } => {
            componentRef = (*__esc_componentRef).clone();
            if ComponentReference::isStartCref(componentRef.clone()) {
                anyStartValue = true;
            } else {
                parameters = metamodelica::cons((ComponentReference::crefStr(componentRef.clone())?).clone(), parameters);
            }
            ((parameters, anyStartValue), !(anyStartValue))
        },
        _ => (inParams, true),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outContinue, outParams))
}

// =============================================================================
// section for introducing pre-variables for alias variables
//
// =============================================================================
fn introducePreVarsForAliasVariables(mut inVar: BackendDAE::Var, mut inTpl: (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) -> (BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))) {
    let mut outVar: BackendDAE::Var;
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::DISCRETE { .. }, varType: ty, arryDim, .. }, (vars, fixvars, eqns, hs)) => {
                    let mut preUsed: bool;
                    let mut isFixed: bool;
                    let mut startValue: Arc<DAE::Exp>;
                    let mut preCR: Arc<DAE::ComponentRef>;
                    let mut preVar: BackendDAE::Var;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    preUsed = BaseHashSet::has(cr.clone(), hs.clone())?;
                    isFixed = BackendVariable::varFixed(var.clone());
                    startValue = BackendVariable::varStartValue(var.clone())?;
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendDAE::Var { varName: preCR.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::DISCRETE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty.clone(), bindExp: None, tplExp: None, arryDim: arryDim.clone(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
                    preVar = BackendVariable::setVarFixed(preVar.clone(), false)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(startValue.clone()))?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), scalar: startValue.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    vars = if (preUsed.clone()) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                    eqns = if (preUsed.clone() && isFixed.clone()) {BackendEquation::add(eqn.clone(), eqns.clone())?} else {eqns.clone()};
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), hs.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varName: cr, varType: ty, arryDim, .. }, (vars, fixvars, eqns, hs)) => {
                    let mut preUsed: bool;
                    let mut preCR: Arc<DAE::ComponentRef>;
                    let mut preVar: BackendDAE::Var;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    preUsed = BaseHashSet::has(cr.clone(), hs.clone())?;
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendDAE::Var { varName: preCR.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR, varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, varType: ty.clone(), bindExp: None, tplExp: None, arryDim: arryDim.clone(), source: DAE::emptyElementSource().clone(), values: None, tearingSelectOption: None, hideResult: None, comment: None, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER, unreplaceable: false, initNonlinear: false, encrypted: false };
                    preVar = BackendVariable::setVarFixed(preVar.clone(), false)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() })))?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), scalar: Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    vars = if (preUsed.clone()) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                    eqns = if (preUsed.clone()) {BackendEquation::add(eqn.clone(), eqns.clone())?} else {eqns.clone()};
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), hs.clone())))
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

// =============================================================================
// section for collecting initial vars/eqns
//
// =============================================================================
fn collectInitialVarsEqnsSystem(mut eqSystems: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>, mut vars: BackendDAE::Variables, mut fixVars: BackendDAE::Variables, mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut reEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, mut hs: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), mut allPrimaryParams: Arc<AvlSetCR::Tree>, mut datareconFlag: bool) -> Result<(BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)> {
    let mut vars: BackendDAE::Variables = vars;
    let mut fixVars: BackendDAE::Variables = fixVars;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = eqns;
    let mut reEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>> = reEqns;
    let mut stateSetFixCounts: metamodelica::Array<i32> = Default::default();
    for mut eq in &*eqSystems {
        let mut eq = eq.clone();
        let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::EqSystem { partitionKind: BackendDAE::BaseClockPartitionKind::CLOCKED_PARTITION { .. }, .. } => {
            (vars, eqns) = BackendVariable::traverseBackendDAEVars(eq.orderedVars.clone(), (std::sync::Arc::new(collectInitialClockedVarsEqns) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> + 'static>), (vars.clone(), eqns.clone()))?;
            ()
        },
        _ => {
            stateSetFixCounts = arrayCreate((eq.stateSets.clone().len() as i32), 0);
            (vars, fixVars, eqns, stateSetFixCounts, _, _, _) = BackendVariable::traverseBackendDAEVars(eq.orderedVars.clone(), (std::sync::Arc::new(collectInitialVars) as std::sync::Arc<dyn ::std::ops::Fn(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), Arc<AvlSetCR::Tree>, bool)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), Arc<AvlSetCR::Tree>, bool))> + 'static>), (vars.clone(), fixVars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParams.clone(), datareconFlag))?;
            (eqns, reEqns) = BackendEquation::traverseEquationArray(eq.orderedEqs.clone(), (std::sync::Arc::new(collectInitialEqns) as std::sync::Arc<dyn ::std::ops::Fn(Arc<BackendDAE::Equation>, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> + 'static>), (eqns.clone(), reEqns.clone()))?;
            if Flags::getConfigBool(Flags::INITIAL_STATE_SELECTION.clone())? {
                (vars, eqns) = collectInitialStateSets(eq.stateSets.clone(), stateSetFixCounts.clone(), vars.clone(), eqns.clone())?;
            }
            GCExt::free(stateSetFixCounts.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((vars, fixVars, eqns, reEqns))
}

fn collectInitialStateSets(mut stateSets: Arc<metamodelica::List<BackendDAE::StateSet>>, mut stateSetFixCounts: metamodelica::Array<i32>, mut iVars: BackendDAE::Variables, mut iEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>) -> Result<(BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)> {
    let mut oVars: BackendDAE::Variables;
    let mut oEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut stateSet: BackendDAE::StateSet = <BackendDAE::StateSet as ::std::default::Default>::default();
    let mut eqn: Arc<BackendDAE::Equation>;
    let mut initEqn: Arc<BackendDAE::Equation>;
    let mut lhs: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut exp: Arc<DAE::Exp>;
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut crLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut statesToFix: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut unfixedStates: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut tp: Arc<DAE::Type>;
    let mut toFix: i32;
    let mut recordSize: Option<i32>;
    let mut source: Arc<DAE::ElementSource>;
    (oVars, oEqns) = (iVars, iEqns);
    for mut stateSet in &*stateSets {
        let mut stateSet = stateSet.clone();
        oVars = BackendVariable::addVars(stateSet.varA.clone(), oVars.clone())?;
        lhs = Expression::crefToExp(stateSet.crA.clone())?;
        tp = ComponentReference::crefTypeFull(stateSet.crA.clone())?;
        tp = DAEUtil::expTypeElementType(tp.clone());
        if DAEUtil::expTypeComplex(tp.clone()) {
            recordSize = Some(Expression::sizeOf(tp.clone()));
        } else {
            recordSize = None;
        }
        expLst = metamodelica::nil();
        crLst = SymbolicJacobian::getJacobianDependencies(stateSet.jacobian.clone())?;
        expLst = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut cr in (crLst.clone()).into_iter().cloned() {
            let __x = Expression::crefToExp(cr.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        expLst = metamodelica::cons(Arc::new(DAE::Exp::ICONST { integer: stateSet.index.clone() - 1 }), expLst.clone());
        rhs = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("$stateSelectionSet")).clone() }), expLst: expLst.clone(), attr: DAE::callAttrBuiltinOther().clone() });
        eqn = Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: list![(stateSet.varA.clone().len() as i32)], left: lhs.clone(), right: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone(), recordSize: recordSize.clone() });
        (oEqns, _) = ExpandableArray::add(eqn.clone(), oEqns.clone())?;
        if Flags::isSet(Flags::BLT_DUMP.clone())? || Flags::isSet(Flags::INITIALIZATION.clone())? {
            BackendDump::dumpEquationList(list![eqn.clone()], (literal!("initial state selection equation generated:")).clone())?;
        }
        if metamodelica::arrayLength(stateSetFixCounts.clone()) >= stateSet.index.clone() && metamodelica::arrayGet(stateSetFixCounts.clone(), stateSet.index.clone())? > 0 {
            unfixedStates = metamodelica::nil();
            for mut state in &*stateSet.statescandidates.clone() {
                let mut state = state.clone();
                if !(BackendVariable::varFixed(state.clone())) {
                    unfixedStates = metamodelica::cons(state.clone(), unfixedStates.clone());
                }
            }
            toFix = metamodelica::arrayGet(stateSetFixCounts.clone(), stateSet.index.clone())?;
            statesToFix = metamodelica::nil();
            statesToFix = SymbolicJacobian::getFixedStatesForSelfdependentSets(stateSet.clone(), unfixedStates.clone(), toFix)?;
            for mut state in &*statesToFix.clone() {
                let mut state = state.clone();
                lhs = Expression::crefToExp(state.varName.clone())?;
                rhs = IndexReduction::makeStartExp(state.varName.clone())?;
                initEqn = Arc::new(BackendDAE::Equation::EQUATION { exp: lhs.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                (oEqns, _) = ExpandableArray::add(initEqn.clone(), oEqns.clone())?;
            }
            if Flags::isSet(Flags::BLT_DUMP.clone())? || Flags::isSet(Flags::INITIALIZATION.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("StateSet ")); __mm_s.push_str(&*intString(stateSet.index.clone())); __mm_s.push_str(&*literal!(" is underconstraint for the initial system.\n")); ArcStr::from(__mm_s) }).clone());
                metamodelica::print((literal!("======================================\n")).clone());
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("# States left to fix: ")); __mm_s.push_str(&*intString(toFix)); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone());
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("# Unfixed candidates: ")); __mm_s.push_str(&*intString((stateSet.statescandidates.clone().len() as i32) - toFix)); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) }).clone());
                BackendDump::dumpVarList(statesToFix.clone(), (literal!("Chosen states to fix:")).clone())?;
            }
        }
    }
    Ok((oVars, oEqns))
}

fn collectInitialVars(mut inVar: BackendDAE::Var, mut inTpl: (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), Arc<AvlSetCR::Tree>, bool)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), Arc<AvlSetCR::Tree>, bool))> {
    let mut outVar: BackendDAE::Var;
    let mut outTpl: (BackendDAE::Variables, BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<Arc<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)), Arc<AvlSetCR::Tree>, bool);
    (outVar, outTpl) = 'mc: {
        let __mc_input = (inVar.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::STATE { .. }, varType: ty, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut preVar: BackendDAE::Var;
                    let mut derVar: BackendDAE::Var;
                    let mut startVar: BackendDAE::Var;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut preCR: Arc<DAE::ComponentRef>;
                    let mut derCR: Arc<DAE::ComponentRef>;
                    let mut startCR: Arc<DAE::ComponentRef>;
                    let mut isFixed: bool;
                    let mut preUsed: bool;
                    let mut startExp: Arc<DAE::Exp>;
                    let mut crefExp: Arc<DAE::Exp>;
                    let mut stateSetIdxString: ArcStr;
                    let mut stateSetSplit: Arc<metamodelica::List<ArcStr>>;
                    let mut stateSetIdx: i32;
                    let mut parameters: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    isFixed = BackendVariable::varFixed(var.clone());
                    preUsed = BaseHashSet::has(cr.clone(), hs.clone())?;
                    crefExp = Expression::crefExp(cr.clone())?;
                    startCR = ComponentReference::crefPrefixStart(cr.clone());
                    startVar = BackendVariable::copyVarNewName(startCR.clone(), var.clone());
                    startVar = BackendVariable::setBindExp(startVar.clone(), None);
                    startVar = BackendVariable::setVarDirection(startVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    startVar = BackendVariable::setVarFixed(startVar.clone(), false)?;
                    startVar = BackendVariable::setVarKind(startVar.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    startVar = BackendVariable::setVarStartValueOption(startVar.clone(), None)?;
                    startExp = BackendVariable::varStartValue(var.clone())?;
                    parameters = Expression::getAllCrefs(startExp.clone())?;
                    if !(({
        let mut __acc: Option<bool> = None;
        for mut p in (parameters.clone()).into_iter().cloned() {
                    let __x = AvlSetCR::hasKey(allPrimaryParameters.clone(), p.clone())?;
                    __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    })) {
                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(startCR.clone())?, scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                        eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                        vars = BackendVariable::addVar(startVar.clone(), vars.clone())?;
                    }
                    if isFixed.clone() {
                        if StringUtil::startsWith((ComponentReferenceBasics::crefFirstIdent(cr.clone())?).clone(), (literal!("$STATESET")).clone()) && Flags::getConfigBool(Flags::INITIAL_STATE_SELECTION.clone())? {
                            stateSetSplit = Util::stringSplitAtChar((ComponentReferenceBasics::crefFirstIdent(cr.clone())?).clone(), (literal!(".")).clone())?;
                            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(stateSetSplit.clone()) {
                                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                                        _ => bail!("pattern mismatch"),
                            } };
                            stateSetIdxString = __pa0.clone();
                            stateSetSplit = __pa1.clone();
                            stateSetIdxString = substring((stateSetIdxString.clone()).clone(), 10, ((stateSetIdxString.clone()).clone().len() as i32))?;
                            stateSetIdx = stringInt((stateSetIdxString.clone()).clone())?;
                            metamodelica::arrayUpdate(stateSetFixCounts.clone(), stateSetIdx.clone(), metamodelica::arrayGet(stateSetFixCounts.clone(), stateSetIdx.clone())? + 1)?;
                        } else {
                            if Expression::isConstValue(startExp.clone())? {
                                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: Expression::crefExp(startCR.clone())?, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                            } else {
                                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp.clone(), scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                            }
                            eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                        }
                    }
                    var = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    derCR = ComponentReference::crefPrefixDer(cr.clone());
                    derVar = BackendVariable::copyVarNewName(derCR.clone(), var.clone());
                    derVar = BackendVariable::setVarDirection(derVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    derVar = BackendVariable::setBindExp(derVar.clone(), None);
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendVariable::copyVarNewName(preCR.clone(), var.clone());
                    preVar = BackendVariable::setVarDirection(preVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    preVar = BackendVariable::setBindExp(preVar.clone(), None);
                    preVar = BackendVariable::setVarFixed(preVar.clone(), true)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() })))?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), scalar: Arc::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    vars = BackendVariable::addVar(derVar.clone(), vars.clone())?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    vars = if (preUsed.clone()) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                    eqns = if (preUsed.clone()) {BackendEquation::add(eqn.clone(), eqns.clone())?} else {eqns.clone()};
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::DISCRETE { .. }, varType: ty, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut preVar: BackendDAE::Var;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut preCR: Arc<DAE::ComponentRef>;
                    let mut startValue_: Arc<DAE::Exp>;
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    let true = (BaseHashSet::has(cr.clone(), hs.clone())?) else { bail!("pattern mismatch") };
                    let true = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    startValue_ = BackendVariable::varStartValue(var.clone())?;
                    var = BackendVariable::setVarFixed(var.clone(), false)?;
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendVariable::copyVarNewName(preCR.clone(), var.clone());
                    preVar = BackendVariable::setVarDirection(preVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    preVar = BackendVariable::setBindExp(preVar.clone(), None);
                    preVar = BackendVariable::setVarFixed(preVar.clone(), false)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(startValue_.clone()))?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), scalar: startValue_.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    vars = BackendVariable::addVar(preVar.clone(), vars.clone())?;
                    eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::DISCRETE { .. }, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut preVar: BackendDAE::Var;
                    let mut preCR: Arc<DAE::ComponentRef>;
                    let mut preUsed: bool;
                    let mut startValue: Option<Arc<DAE::Exp>>;
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    preUsed = BaseHashSet::has(cr.clone(), hs.clone())?;
                    startValue = BackendVariable::varStartValueOption(var.clone());
                    var = BackendVariable::setVarFixed(var.clone(), false)?;
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendVariable::copyVarNewName(preCR.clone(), var.clone());
                    preVar = BackendVariable::setVarDirection(preVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    preVar = BackendVariable::setBindExp(preVar.clone(), None);
                    preVar = BackendVariable::setVarFixed(preVar.clone(), false)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), startValue.clone())?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    vars = if (preUsed.clone()) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: None, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut startExp: Arc<DAE::Exp>;
                    let mut s: ArcStr;
                    let mut r#str: ArcStr;
                    let mut info: SourceInfo;
                    let mut var = (*var).clone();
                    let true = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    startExp = BackendVariable::varStartValueType(var.clone())?;
                    s = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    r#str = (ExpressionBasics::printExpStr(startExp.clone())?).clone();
                    var = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    var = BackendVariable::setBindExp(var.clone(), Some(startExp.clone()));
                    var = BackendVariable::setVarFixed(var.clone(), true)?;
                    info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(var.clone()));
                    Error::addSourceMessage(Error::UNBOUND_PARAMETER_WITH_START_VALUE_WARNING.clone(), list![(s.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(bindExp), varType: ty, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut s: ArcStr;
                    let mut r#str: ArcStr;
                    let mut info: SourceInfo;
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    let true = (intGt(Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, 31)) else { bail!("pattern mismatch") };
                    let false = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    var = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    var = BackendVariable::setBindExp(var.clone(), None);
                    s = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    r#str = (ExpressionBasics::printExpStr(bindExp.clone())?).clone();
                    info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(var.clone()));
                    Error::addSourceMessage(Error::UNFIXED_PARAMETER_WITH_BINDING.clone(), list![(s.clone()).clone(), (s.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
                    eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), scalar: bindExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(bindExp), .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut s: ArcStr;
                    let mut r#str: ArcStr;
                    let mut info: SourceInfo;
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let true = (intLe(Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, 31)) else { bail!("pattern mismatch") };
                    let false = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    var = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    var = BackendVariable::setBindExp(var.clone(), None);
                    ::match_deref::match_deref! { match &(BackendVariable::varStartValueOption(var.clone())) {
                        None => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    var = BackendVariable::setVarStartValue(var.clone(), bindExp.clone())?;
                    s = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    r#str = (ExpressionBasics::printExpStr(bindExp.clone())?).clone();
                    info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(var.clone()));
                    Error::addSourceMessage(Error::UNFIXED_PARAMETER_WITH_BINDING_31.clone(), list![(s.clone()).clone(), (s.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varName: cr, varKind: BackendDAE::VarKind::PARAM { .. }, bindExp: Some(bindExp), .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut startExp: Arc<DAE::Exp>;
                    let mut s: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sv: ArcStr;
                    let mut info: SourceInfo;
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let true = (intLe(Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, 31)) else { bail!("pattern mismatch") };
                    let false = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    var = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    var = BackendVariable::setBindExp(var.clone(), None);
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::varStartValueOption(var.clone())) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    startExp = __pa0.clone();
                    s = (ComponentReferenceBasics::printComponentRefStr(cr.clone())?).clone();
                    r#str = (ExpressionBasics::printExpStr(bindExp.clone())?).clone();
                    sv = (ExpressionBasics::printExpStr(startExp.clone())?).clone();
                    info = ElementSource::getElementSourceFileInfo(BackendVariable::getVarSource(var.clone()));
                    Error::addSourceMessage(Error::UNFIXED_PARAMETER_WITH_BINDING_AND_START_VALUE_31.clone(), list![(s.clone()).clone(), (sv.clone()).clone(), (s.clone()).clone(), (r#str.clone()).clone()], info.clone())?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    var = BackendVariable::setVarKind(var.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varKind: BackendDAE::VarKind::EXTOBJ { .. }, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut vars = (*vars).clone();
                    vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varKind: BackendDAE::VarKind::CONST { .. }, .. }, _) => {
                    Ok((var.clone(), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varName: cr, varType: ty, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut preVar: BackendDAE::Var;
                    let mut startVar: BackendDAE::Var;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut preCR: Arc<DAE::ComponentRef>;
                    let mut startCR: Arc<DAE::ComponentRef>;
                    let mut isInput: bool;
                    let mut preUsed: bool;
                    let mut startExp: Arc<DAE::Exp>;
                    let mut parameters: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut var = (*var).clone();
                    let mut vars = (*vars).clone();
                    let mut fixvars = (*fixvars).clone();
                    let mut eqns = (*eqns).clone();
                    let true = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    if datarecon.clone() {
                        isInput = checkComponentNames(var.varDirection.clone(), cr.clone());
                    } else {
                        isInput = BackendVariable::isVarOnTopLevelAndInput(var.clone());
                    }
                    preUsed = BaseHashSet::has(cr.clone(), hs.clone())?;
                    startCR = ComponentReference::crefPrefixStart(cr.clone());
                    startVar = BackendVariable::copyVarNewName(startCR.clone(), var.clone());
                    startVar = BackendVariable::setBindExp(startVar.clone(), None);
                    startVar = BackendVariable::setVarDirection(startVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    startVar = BackendVariable::setVarFixed(startVar.clone(), false)?;
                    startVar = BackendVariable::setVarKind(startVar.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    startVar = BackendVariable::setVarStartValueOption(startVar.clone(), None)?;
                    startExp = BackendVariable::varStartValue(var.clone())?;
                    parameters = Expression::getAllCrefs(startExp.clone())?;
                    if !(({
        let mut __acc: Option<bool> = None;
        for mut p in (parameters.clone()).into_iter().cloned() {
                    let __x = AvlSetCR::hasKey(allPrimaryParameters.clone(), p.clone())?;
                    __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    })) {
                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(startCR.clone())?, scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                        eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                        vars = BackendVariable::addVar(startVar.clone(), vars.clone())?;
                    }
                    var = BackendVariable::setVarFixed(var.clone(), false)?;
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendVariable::copyVarNewName(preCR.clone(), var.clone());
                    preVar = BackendVariable::setVarDirection(preVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    preVar = BackendVariable::setBindExp(preVar.clone(), None);
                    preVar = BackendVariable::setVarFixed(preVar.clone(), true)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() })))?;
                    if Expression::isConstValue(startExp.clone())? {
                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), scalar: Expression::crefExp(startCR.clone())?, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    } else {
                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    }
                    vars = if (!(isInput.clone())) {BackendVariable::addVar(var.clone(), vars.clone())?} else {vars.clone()};
                    fixvars = if (isInput.clone()) {BackendVariable::addVar(var.clone(), fixvars.clone())?} else {fixvars.clone()};
                    vars = if (preUsed.clone()) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                    eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ BackendDAE::Var { varName: cr, varType: ty, .. }, (vars, fixvars, eqns, stateSetFixCounts, hs, allPrimaryParameters, datarecon)) => {
                    let mut preVar: BackendDAE::Var;
                    let mut startVar: BackendDAE::Var;
                    let mut eqn: Arc<BackendDAE::Equation>;
                    let mut preCR: Arc<DAE::ComponentRef>;
                    let mut startCR: Arc<DAE::ComponentRef>;
                    let mut isInput: bool;
                    let mut preUsed: bool;
                    let mut startExp: Arc<DAE::Exp>;
                    let mut parameters: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
                    let mut vars = (*vars).clone();
                    let mut fixvars = (*fixvars).clone();
                    let mut eqns = (*eqns).clone();
                    let false = (BackendVariable::varFixed(var.clone())) else { bail!("pattern mismatch") };
                    if datarecon.clone() {
                        isInput = checkComponentNames(var.varDirection.clone(), cr.clone());
                    } else {
                        isInput = BackendVariable::isVarOnTopLevelAndInput(var.clone());
                    }
                    preUsed = BaseHashSet::has(cr.clone(), hs.clone())?;
                    startCR = ComponentReference::crefPrefixStart(cr.clone());
                    startVar = BackendVariable::copyVarNewName(startCR.clone(), var.clone());
                    startVar = BackendVariable::setBindExp(startVar.clone(), None);
                    startVar = BackendVariable::setVarDirection(startVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    startVar = BackendVariable::setVarFixed(startVar.clone(), false)?;
                    startVar = BackendVariable::setVarKind(startVar.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
                    startVar = BackendVariable::setVarStartValueOption(startVar.clone(), None)?;
                    startExp = BackendVariable::varStartValue(var.clone())?;
                    parameters = Expression::getAllCrefs(startExp.clone())?;
                    if !(({
        let mut __acc: Option<bool> = None;
        for mut p in (parameters.clone()).into_iter().cloned() {
                    let __x = AvlSetCR::hasKey(allPrimaryParameters.clone(), p.clone())?;
                    __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(true)
    })) {
                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Expression::crefExp(startCR.clone())?, scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                        eqns = BackendEquation::add(eqn.clone(), eqns.clone())?;
                        vars = BackendVariable::addVar(startVar.clone(), vars.clone())?;
                    }
                    preCR = ComponentReference::crefPrefixPre(cr.clone());
                    preVar = BackendVariable::copyVarNewName(preCR.clone(), var.clone());
                    preVar = BackendVariable::setVarDirection(preVar.clone(), openmodelica_frontend_types::DAE::VarDirection::BIDIR);
                    preVar = BackendVariable::setBindExp(preVar.clone(), None);
                    preVar = BackendVariable::setVarFixed(preVar.clone(), true)?;
                    preVar = BackendVariable::setVarStartValueOption(preVar.clone(), Some(Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() })))?;
                    if Expression::isConstValue(startExp.clone())? {
                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), scalar: Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() }), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    } else {
                        eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: preCR.clone(), ty: ty.clone() }), scalar: startExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
                    }
                    vars = if (!(isInput.clone())) {BackendVariable::addVar(var.clone(), vars.clone())?} else {vars.clone()};
                    fixvars = if (isInput.clone()) {BackendVariable::addVar(var.clone(), fixvars.clone())?} else {fixvars.clone()};
                    vars = if (preUsed.clone()) {BackendVariable::addVar(preVar.clone(), vars.clone())?} else {vars.clone()};
                    eqns = if (preUsed.clone()) {BackendEquation::add(eqn.clone(), eqns.clone())?} else {eqns.clone()};
                    Ok((var.clone(), (vars.clone(), fixvars.clone(), eqns.clone(), stateSetFixCounts.clone(), hs.clone(), allPrimaryParameters.clone(), datarecon.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function collectInitialVars failed for: ")); __mm_s.push_str(&*BackendDump::varString(inVar.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Initialization.mo"))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outVar, outTpl))
}

fn checkComponentNames(mut inVarDirection: DAE::VarDirection, mut inComponentRef: Arc<DAE::ComponentRef>) -> bool {
    let mut isTopLevel: bool;
    isTopLevel = (::match_deref::match_deref! { match &((inVarDirection, inComponentRef)) {
        (DAE::VarDirection::INPUT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => true,
        (DAE::VarDirection::INPUT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => true,
        (_, _) => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTopLevel
}

fn collectInitialClockedVarsEqns(mut inVar: BackendDAE::Var, mut inTpl: (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(BackendDAE::Var, (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> {
    let mut outVar: BackendDAE::Var;
    let mut outTpl: (BackendDAE::Variables, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>);
    let mut vars: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    (vars, eqns) = inTpl;
    (outVar, outTpl) = (match inVar {
        ref var @ BackendDAE::Var { varName: ref cr, varType: ref ty, varKind: ref kind, .. } => {
            let mut crExp: Arc<DAE::Exp>;
            let mut startExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
            crExp = Expression::crefExp(cr.clone())?;
            (vars, eqns) = (match kind.clone() {
        BackendDAE::VarKind::CLOCKED_STATE { previousName: ref previousCR, .. } => {
            let mut previousVar: BackendDAE::Var;
            let mut previousExp: Arc<DAE::Exp>;
            previousVar = BackendVariable::copyVarNewName(previousCR.clone(), var.clone());
            previousVar = BackendVariable::setVarKind(previousVar, openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)?;
            previousVar = BackendVariable::setVarDirection(previousVar, openmodelica_frontend_types::DAE::VarDirection::BIDIR);
            previousVar = BackendVariable::setBindExp(previousVar, None);
            previousVar = BackendVariable::setVarFixed(previousVar, true)?;
            previousVar = BackendVariable::setVarStartValueOption(previousVar, Some(Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() })))?;
            previousExp = Expression::crefExp(previousCR.clone())?;
            vars = BackendVariable::addVar(previousVar, vars)?;
            eqns = BackendEquation::add(Arc::new(BackendDAE::Equation::EQUATION { exp: previousExp, scalar: crExp.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() }), eqns)?;
            startExp = BackendVariable::varStartValue(var.clone())?;
            vars = BackendVariable::addVar(var.clone(), vars)?;
            eqns = BackendEquation::add(Arc::new(BackendDAE::Equation::EQUATION { exp: crExp, scalar: startExp, source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() }), eqns)?;
            (vars, eqns)
        },
        _ => {
            (vars, eqns)
        },
    });
            (var.clone(), (vars, eqns))
        },
    });
    Ok((outVar, outTpl))
}

fn collectInitialEqns(mut inEq: Arc<BackendDAE::Equation>, mut inTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(Arc<BackendDAE::Equation>, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> {
    let mut outEq: Arc<BackendDAE::Equation> = inEq.clone();
    let mut outTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>);
    let mut eqn1: Arc<BackendDAE::Equation>;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut reeqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut size: i32;
    let mut b: bool;
    (eqns, reeqns) = inTpl;
    (eqn1, _) = BackendEquation::traverseExpsOfEquation(inEq, (std::sync::Arc::new(Expression::traverseSubexpressionsDummyHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>) -> Result<(Arc<DAE::Exp>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>)> + 'static>), (std::sync::Arc::new(fnptr!(replaceDerPreCref, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
    size = BackendEquation::equationSize(eqn1.clone())?;
    b = intGt(size, 0);
    eqns = if (b) {BackendEquation::add(eqn1.clone(), eqns)?} else {eqns};
    reeqns = if (!(b)) {BackendEquation::add(eqn1, reeqns)?} else {reeqns};
    outTpl = (eqns, reeqns);
    Ok((outEq, outTpl))
}

fn replaceDerPreCref(mut inExp: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, attr: Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut dummyder: Arc<DAE::ComponentRef>;
            dummyder = ComponentReference::crefPrefixDer(cr.clone());
            Arc::new(DAE::Exp::CREF { componentRef: dummyder, ty: ty.clone() })
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, attr: Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut dummyder: Arc<DAE::ComponentRef>;
            dummyder = ComponentReference::crefPrefixPre(cr.clone());
            Arc::new(DAE::Exp::CREF { componentRef: dummyder, ty: ty.clone() })
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, attr: Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut dummyder: Arc<DAE::ComponentRef>;
            dummyder = ComponentReference::crefPrefixPrevious(cr.clone());
            Arc::new(DAE::Exp::CREF { componentRef: dummyder, ty: ty.clone() })
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

// =============================================================================
// section for bindings
//
// =============================================================================
fn collectInitialBindings(mut inVar: BackendDAE::Var, mut inTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>)) -> Result<(BackendDAE::Var, (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>))> {
    let mut outVar: BackendDAE::Var;
    let mut outTpl: (Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>, Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>);
    (outVar, outTpl) = (::match_deref::match_deref! { match &((inVar.clone(), inTpl.clone())) {
        (var @ BackendDAE::Var { bindExp: None, .. }, _) => {
            (var.clone(), inTpl)
        },
        (var @ BackendDAE::Var { varName: cr, bindExp: Some(bindExp), varKind: BackendDAE::VarKind::EXTOBJ { .. }, source, .. }, (eqns, reeqns)) => {
            let mut eqn: Arc<BackendDAE::Equation>;
            let mut eqns = (*eqns).clone();
            eqn = Arc::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cr.clone(), exp: bindExp.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
            eqns = BackendEquation::add(eqn, eqns.clone())?;
            (var.clone(), (eqns.clone(), reeqns.clone()))
        },
        (var @ BackendDAE::Var { varName: cr, bindExp: Some(bindExp), varType: ty, source, .. }, (eqns, reeqns)) => {
            let mut basic_ty: Arc<DAE::Type>;
            let mut crefExp: Arc<DAE::Exp>;
            let mut eqn: Arc<BackendDAE::Equation>;
            let mut record_size: Option<i32>;
            let mut eqns = (*eqns).clone();
            crefExp = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ty.clone() });
            if Types::isArray(ty.clone()) {
                basic_ty = Types::getBasicType(ty.clone());
                record_size = if (Types::isRecord(basic_ty.clone())) {Some(Types::getDimensionProduct(basic_ty)?)} else {None};
                eqn = Arc::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: Types::getDimensionSizes(ty.clone())?, left: crefExp, right: bindExp.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone(), recordSize: record_size });
            } else {
                eqn = Arc::new(BackendDAE::Equation::EQUATION { exp: crefExp, scalar: bindExp.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() });
            }
            eqns = BackendEquation::add(eqn, eqns.clone())?;
            (var.clone(), (eqns.clone(), reeqns.clone()))
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function collectInitialBindings failed for: ")); __mm_s.push_str(&*BackendDump::varString(inVar)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("BackEnd/Initialization.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVar, outTpl))
}

// =============================================================================
// section for post-optimization module "removeInitializationStuff"
//
// =============================================================================
pub(crate) fn removeInitializationStuff(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    let mut removedEqsList: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut shared: Arc<BackendDAE::Shared> = inDAE.shared.clone();
    for mut eqs in &*outDAE.eqs.clone() {
        let mut eqs = eqs.clone();
        BackendDAEUtil::traverseBackendDAEExpsEqns(eqs.orderedEqs.clone(), (std::sync::Arc::new(removeInitializationStuff1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
        BackendDAEUtil::traverseBackendDAEExpsEqns(eqs.removedEqs.clone(), (std::sync::Arc::new(removeInitializationStuff1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
    }
    BackendDAEUtil::traverseBackendDAEExpsEqns(shared.removedEqs.clone(), (std::sync::Arc::new(removeInitializationStuff1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
    for mut eq in &*BackendEquation::equationList(shared.removedEqs.clone())? {
        let mut eq = eq.clone();
        removedEqsList = (match BackendEquation::equationKind(eq.clone())? {
        BackendDAE::EquationKind::INITIAL_EQUATION { .. } => removedEqsList.clone(),
        _ => filterWhenEquation(eq.clone(), removedEqsList.clone())?,
    });
    }
    assign_field!(
        shared.removedEqs = BackendEquation::listEquation(removedEqsList.reverse())?,
        shared.initialEqs = BackendEquation::emptyEqns()
    );
    assign_field!(outDAE.shared = shared);
    Ok(outDAE)
}

fn filterWhenEquation(mut inEqn: Arc<BackendDAE::Equation>, mut inEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>) -> Result<Arc<metamodelica::List<Arc<BackendDAE::Equation>>>> {
    let mut outEqnLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>;
    let mut condition: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    outEqnLst = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { condition, elsewhenPart: None, .. }, .. } if ((BackendDAEUtil::getConditionList(condition.clone())?).0.is_empty()) => inEqnLst,
        _ => metamodelica::cons(inEqn, inEqnLst),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqnLst)
}

fn removeInitializationStuff1(mut inExp: Arc<DAE::Exp>, mut inUseHomotopy: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outUseHomotopy: bool;
    (outExp, outUseHomotopy) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new(fnptr!(removeInitializationStuff2, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), inUseHomotopy)?;
    Ok((outExp, outUseHomotopy))
}

fn removeInitializationStuff2(mut inExp: Arc<DAE::Exp>, mut inUseHomotopy: bool) -> (Arc<DAE::Exp>, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outUseHomotopy: bool;
    (outExp, outUseHomotopy) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => {
            (Arc::new(DAE::Exp::BCONST { bool: false }), inUseHomotopy)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, expLst: Deref @ metamodelica::List::Cons { head: actual, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, .. } => {
            (actual.clone(), true)
        },
        _ => {
            (inExp, inUseHomotopy)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outUseHomotopy)
}

// =============================================================================
// section for post-optimization module "replaceHomotopyWithSimplified"
//
// =============================================================================
pub(crate) fn replaceHomotopyWithSimplified(mut inDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outDAE: Arc<BackendDAE::BackendDAE> = inDAE.clone();
    assign_field!(outDAE.eqs = ({
        let mut __acc: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
        for mut eqs in (outDAE.eqs.clone()).into_iter().cloned() {
            let __x = replaceHomotopyWithSimplifiedEqs(eqs.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
    Ok(outDAE)
}

pub(crate) fn replaceHomotopyWithSimplifiedEqs(mut eqs: Arc<BackendDAE::EqSystem>) -> Result<Arc<BackendDAE::EqSystem>> {
    let mut eqs: Arc<BackendDAE::EqSystem> = eqs;
    BackendDAEUtil::traverseBackendDAEExpsEqns(eqs.orderedEqs.clone(), (std::sync::Arc::new(replaceHomotopyWithSimplified1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
    BackendDAEUtil::traverseBackendDAEExpsEqns(eqs.removedEqs.clone(), (std::sync::Arc::new(replaceHomotopyWithSimplified1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
    eqs = BackendDAEUtil::clearEqSyst(eqs)?;
    Ok(eqs)
}

fn replaceHomotopyWithSimplified1(mut inExp: Arc<DAE::Exp>, mut inUseHomotopy: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outUseHomotopy: bool;
    (outExp, outUseHomotopy) = Expression::traverseExpBottomUp(inExp, (std::sync::Arc::new(fnptr!(replaceHomotopyWithSimplified2, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), inUseHomotopy)?;
    Ok((outExp, outUseHomotopy))
}

fn replaceHomotopyWithSimplified2(mut inExp: Arc<DAE::Exp>, mut inUseHomotopy: bool) -> (Arc<DAE::Exp>, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outUseHomotopy: bool;
    (outExp, outUseHomotopy) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }, expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: simplified, tail: _ } }, .. } => {
            (simplified.clone(), true)
        },
        _ => {
            (inExp, inUseHomotopy)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outUseHomotopy)
}

