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

use crate::SimCodeFunctionUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::HashTableCrIListArray;
use openmodelica_frontend_dump::HashTableCrILst;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

fn simulationFindLiterals(mut fns: Arc<metamodelica::List<DAE::Function>>) -> Result<(Arc<metamodelica::List<DAE::Function>>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>))> {
    let mut ofns: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
    let mut literals: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>);
    (ofns, literals) = DAEUtil::traverseDAEFunctions(fns.clone(), (std::sync::Arc::new(SimCodeFunctionUtil::findLiteralsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>)) -> Result<(Arc<DAE::Exp>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>))> + 'static>), (0, HashTableExpToIndex::emptyHashTableSized(BaseHashTable::bigBucketSize.clone()), metamodelica::nil()))?;
    Ok((ofns, literals))
}

pub fn createFunctions(mut inProgram: Absyn::Program, mut functionTree: Arc<AvlTreePathFunction::Tree>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>, Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>, (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>)), Arc<metamodelica::List<Arc<DAE::Exp>>>))> {
    let mut outLibs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outLibPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outIncludes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outIncludeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outRecordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
    let mut outFunctions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
    let mut outLiterals: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>);
    let mut funcelems: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
    let mut lits: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    match '__try0: {
        funcelems = unwrap_break_err!(DAEUtil::getFunctionList(functionTree.clone(), false), '__try0);
        funcelems = unwrap_break_err!(Inline::inlineCallsInFunctions(funcelems.clone(), (None, list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE])), '__try0);
        let (__pa1, ref __pa3 @ (_, _, ref __pa2)) = unwrap_break_err!(simulationFindLiterals(funcelems.clone()), '__try0);
        funcelems = __pa1.clone();
        lits = __pa2.clone();
        outLiterals = __pa3.clone();
        (outFunctions, outRecordDecls, outIncludes, outIncludeDirs, outLibs, outLibPaths) = unwrap_break_err!(SimCodeFunctionUtil::elaborateFunctions(inProgram.clone(), funcelems.clone(), metamodelica::nil(), lits.clone(), metamodelica::nil()), '__try0);
        Ok::<_, anyhow::Error>((funcelems.clone(), lits.clone(), outFunctions.clone(), outIncludeDirs.clone(), outIncludes.clone(), outLibPaths.clone(), outLibs.clone(), outLiterals.clone(), outRecordDecls.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8)) => {
            funcelems = __try0_o0;
            lits = __try0_o1;
            outFunctions = __try0_o2;
            outIncludeDirs = __try0_o3;
            outIncludes = __try0_o4;
            outLibPaths = __try0_o5;
            outLibs = __try0_o6;
            outLiterals = __try0_o7;
            outRecordDecls = __try0_o8;
        }
        Err(__try0_err) => {
            Error::addInternalError((literal!("Creation of Modelica functions failed.")).clone(), metamodelica::sourceInfo!("SimCode/SimCodeUtilShared.mo"))?;
            return Err(__try0_err);
        }
    }
    Ok((outLibs, outLibPaths, outIncludes, outIncludeDirs, outRecordDecls, outFunctions, outLiterals))
}

pub fn createVarToArrayIndexMapping(mut iModelInfo: SimCode::ModelInfo) -> Result<((metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)))> {
    let mut oVarToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
    let mut oVarToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut sim_vars: SimCodeVar::SimVars = <SimCodeVar::SimVars as ::std::default::Default>::default();
    let mut vars: Arc<metamodelica::List<(Arc<metamodelica::List<SimCodeVar::SimVar>>, i32)>> = metamodelica::nil();
    let mut table_size: i32 = 0;
    let mut var_lst: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut var_type: i32 = 0;
    let mut currentVarIndices: metamodelica::Array<i32> = Default::default();
    sim_vars = iModelInfo.vars.clone();
    vars = list![(sim_vars.stateVars.clone(), 1), (sim_vars.derivativeVars.clone(), 1), (sim_vars.algVars.clone(), 1), (sim_vars.discreteAlgVars.clone(), 1), (sim_vars.intAlgVars.clone(), 2), (sim_vars.boolAlgVars.clone(), 3), (sim_vars.stringAlgVars.clone(), 4), (sim_vars.paramVars.clone(), 1), (sim_vars.intParamVars.clone(), 2), (sim_vars.boolParamVars.clone(), 3), (sim_vars.stringParamVars.clone(), 4), (sim_vars.constVars.clone(), 1), (sim_vars.intConstVars.clone(), 2), (sim_vars.boolConstVars.clone(), 3), (sim_vars.stringConstVars.clone(), 4), (sim_vars.realOptimizeConstraintsVars.clone(), 1), (sim_vars.realOptimizeFinalConstraintsVars.clone(), 1), (sim_vars.aliasVars.clone(), 1), (sim_vars.intAliasVars.clone(), 2), (sim_vars.boolAliasVars.clone(), 3), (sim_vars.stringAliasVars.clone(), 4)];
    for mut vl in &*vars.clone() {
        let mut vl = vl.clone();
        (var_lst, _) = vl.clone();
        table_size = table_size.clone() + (var_lst.clone().len() as i32);
    }
    table_size = Util::nextPrime(((metamodelica::OrderedFloat((table_size.clone()) as f64) * metamodelica::OrderedFloat(1.4_f64)).0 as i32));
    oVarToArrayIndexMapping = HashTableCrIListArray::emptyHashTableSized(table_size.clone());
    oVarToIndexMapping = HashTableCrILst::emptyHashTableSized(table_size.clone());
    currentVarIndices = arrayCreate(4, 1);
    for mut vl in &*vars.clone() {
        let mut vl = vl.clone();
        (var_lst, var_type) = vl.clone();
        (currentVarIndices, oVarToArrayIndexMapping, oVarToIndexMapping) = addVarToArrayIndexMappings(var_lst.clone(), var_type.clone(), currentVarIndices.clone(), oVarToArrayIndexMapping.clone(), oVarToIndexMapping.clone())?;
    }
    Ok((oVarToArrayIndexMapping, oVarToIndexMapping))
}

pub fn addVarToArrayIndexMappings(mut vars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut iVarType: i32, mut currentVarIndices: metamodelica::Array<i32>, mut varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut varToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)))> {
    let mut currentVarIndices: metamodelica::Array<i32> = currentVarIndices;
    let mut varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr)) = varToArrayIndexMapping;
    let mut varToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)) = varToIndexMapping;
    for mut v in &*vars.clone() {
        let mut v = v.clone();
        (currentVarIndices, varToArrayIndexMapping, varToIndexMapping) = addVarToArrayIndexMapping(v.clone(), iVarType.clone(), currentVarIndices.clone(), varToArrayIndexMapping.clone(), varToIndexMapping.clone())?;
    }
    Ok((currentVarIndices, varToArrayIndexMapping, varToIndexMapping))
}

pub fn addVarToArrayIndexMapping(mut iVar: SimCodeVar::SimVar, mut iVarType: i32, mut currentVarIndices: metamodelica::Array<i32>, mut varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), mut varToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>))) -> Result<(metamodelica::Array<i32>, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn((Arc<metamodelica::List<i32>>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>)), (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)))> {
    let mut currentVarIndices: metamodelica::Array<i32> = currentVarIndices;
    let mut varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr)) = varToArrayIndexMapping;
    let mut varToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr)) = varToIndexMapping;
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut arrayName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut varIdx: i32 = 0;
    let mut arrayIndex: i32 = 0;
    let mut varIndices: metamodelica::Array<i32> = Default::default();
    let mut arrayDimensions: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut numArrayElement: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut arraySubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
    let () = (match iVar.clone() {
        SimCodeVar::SimVar { numArrayElement: mut numArrayElement, name: mut name, .. } => {
            (currentVarIndices, varIdx) = getArrayIdxByVar(iVar.clone(), iVarType.clone(), varToIndexMapping.clone(), currentVarIndices.clone())?;
            varToIndexMapping = BaseHashTable::add((name.clone(), list![varIdx.clone()]), varToIndexMapping.clone())?;
            arraySubscripts = ComponentReference::crefLastSubs(name.clone())?;
            if numArrayElement.clone().is_empty() || checkIfSubscriptsContainsUnhandlableIndices(arraySubscripts.clone()) {
                arrayName = name.clone();
            } else {
                arrayName = ComponentReferenceBasics::crefStripLastSubs(name.clone())?;
            }
            if isArrayVar(iVar.clone()) {
                arrayDimensions = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (List::lastN(numArrayElement.clone(), (numArrayElement.clone().len() as i32))?).into_iter().cloned() {
            let __x = stringInt((e.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                varIndices = arrayCreate(1, varIdx.clone());
                varToArrayIndexMapping = BaseHashTable::add((arrayName.clone(), (arrayDimensions.clone(), varIndices.clone())), varToArrayIndexMapping.clone())?;
            } else if ComponentReferenceBasics::crefEqual(arrayName.clone(), name.clone())? {
                varIndices = arrayCreate(1, varIdx.clone());
                varToArrayIndexMapping = BaseHashTable::add((arrayName.clone(), (list![1], varIndices.clone())), varToArrayIndexMapping.clone())?;
            } else {
                if BaseHashTable::hasKey(arrayName.clone(), varToArrayIndexMapping.clone())? {
                    (arrayDimensions, varIndices) = BaseHashTable::get(arrayName.clone(), varToArrayIndexMapping.clone())?;
                } else {
                    arrayDimensions = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (List::lastN(numArrayElement.clone(), (arraySubscripts.clone().len() as i32))?).into_iter().cloned() {
            let __x = stringInt((e.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    varIndices = arrayCreate(List::fold(arrayDimensions.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), 1)?, 0);
                }
                arrayIndex = getScalarElementIndex(arraySubscripts.clone(), arrayDimensions.clone());
                varIndices = {let _arr = varIndices.clone(); _arr.borrow_mut()[(arrayIndex.clone()-1) as usize] = varIdx.clone(); _arr};
                varToArrayIndexMapping = BaseHashTable::add((arrayName.clone(), (arrayDimensions.clone(), varIndices.clone())), varToArrayIndexMapping.clone())?;
            }
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Unknown case for addVarToArrayIndexMapping.\n")).clone()])?;
            ()
        },
    });
    Ok((currentVarIndices, varToArrayIndexMapping, varToIndexMapping))
}

fn checkIfSubscriptsContainsUnhandlableIndices(mut iSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> bool {
    let mut oContainsUnhandledSubscripts: bool = false;
    let mut subscript: Arc<DAE::Subscript> = Arc::new(DAE::Subscript::WHOLEDIM);
    for mut subscript in &*iSubscripts.clone() {
        let mut subscript = subscript.clone();
        if DAEUtil::getSubscriptIndex(subscript.clone()) < 0 {
            oContainsUnhandledSubscripts = true;
            break;
        }
    }
    oContainsUnhandledSubscripts
}

fn getArrayIdxByVar(mut iVar: SimCodeVar::SimVar, mut iVarType: i32, mut iVarToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<i32>>) -> Result<ArcStr> + 'static>)), mut iCurrentVarIndices: metamodelica::Array<i32>) -> Result<(metamodelica::Array<i32>, i32)> {
    let mut iCurrentVarIndices: metamodelica::Array<i32> = iCurrentVarIndices;
    let mut oVarIndex: i32 = 0;
    let mut varName: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut name: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut varIdx: i32 = 0;
    let mut tmpCurrentVarIndices: metamodelica::Array<i32> = Default::default();
    oVarIndex = (match (iVar.clone(), iCurrentVarIndices.clone()) {
        (SimCodeVar::SimVar { aliasvar: SimCodeVar::AliasVariable::NOALIAS { .. }, name: mut __esc_name, .. }, mut tmpCurrentVarIndices) => {
            name = __esc_name.clone();
            (varIdx, tmpCurrentVarIndices) = getVarToArrayIndexByType(iVar.clone(), iVarType.clone(), tmpCurrentVarIndices.clone())?;
            varIdx.clone()
        },
        (SimCodeVar::SimVar { aliasvar: SimCodeVar::AliasVariable::NEGATEDALIAS { varName: mut varName }, name: mut __esc_name, .. }, _) => {
            name = __esc_name.clone();
            if BaseHashTable::hasKey(varName.clone(), iVarToIndexMapping.clone())? {
                let __pa0 = ::match_deref::match_deref! { match &(BaseHashTable::get(varName.clone(), iVarToIndexMapping.clone())?) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                varIdx = __pa0.clone();
                varIdx = intMul(varIdx.clone(), -1);
            } else if ComponentReference::isTime(varName.clone()) {
                varIdx = 0;
            } else {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Negated alias to unknown variable given.")).clone()])?;
                bail!("fail");
            }
            varIdx.clone()
        },
        (SimCodeVar::SimVar { aliasvar: SimCodeVar::AliasVariable::ALIAS { varName: mut varName }, name: mut __esc_name, .. }, _) => {
            name = __esc_name.clone();
            if BaseHashTable::hasKey(varName.clone(), iVarToIndexMapping.clone())? {
                let __pa0 = ::match_deref::match_deref! { match &(BaseHashTable::get(varName.clone(), iVarToIndexMapping.clone())?) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                varIdx = __pa0.clone();
            } else if ComponentReference::isTime(varName.clone()) {
                varIdx = 0;
            } else {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Alias to unknown variable given.")).clone()])?;
                bail!("fail");
            }
            varIdx.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((iCurrentVarIndices, oVarIndex))
}

fn getVarToArrayIndexByType(mut iVar: SimCodeVar::SimVar, mut iVarType: i32, mut iCurrentVarIndices: metamodelica::Array<i32>) -> Result<(i32, metamodelica::Array<i32>)> {
    let mut oVarIdx: i32 = 0;
    let mut iCurrentVarIndices: metamodelica::Array<i32> = iCurrentVarIndices;
    match '__try0: {
        oVarIdx = unwrap_break_err!(metamodelica::arrayGet(iCurrentVarIndices.clone(), iVarType.clone()), '__try0);
        {let _arr = iCurrentVarIndices.clone(); _arr.borrow_mut()[(iVarType.clone()-1) as usize] = oVarIdx.clone() + unwrap_break_err!(getNumElems(iVar.clone()), '__try0); _arr};
        Ok::<_, anyhow::Error>((oVarIdx.clone(),))
    } {
        Ok((__try0_o0,)) => {
            oVarIdx = __try0_o0;
        }
        Err(_) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("GetVarToArrayIndexByType with unknown type called.")).clone()])?;
            oVarIdx = -1;
        }
    }
    Ok((oVarIdx, iCurrentVarIndices))
}

pub fn getScalarElementIndex(mut arraySubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut arrayDimensions: Arc<metamodelica::List<i32>>) -> i32 {
    let mut arrayIndex: i32 = 0;
    let mut idx: i32 = 0;
    let mut fac: i32 = 0;
    arrayIndex = 1;
    fac = 1;
    for mut i in (1..=(arraySubscripts.clone().len() as i32)).rev() {
        idx = DAEUtil::getSubscriptIndex((arraySubscripts.clone()).get(i.clone()).unwrap());
        arrayIndex = arrayIndex.clone() + (idx.clone() - 1) * fac.clone();
        fac = fac.clone() * (arrayDimensions.clone()).get(i.clone()).unwrap();
    }
    arrayIndex
}

pub fn getNumElems(mut var: SimCodeVar::SimVar) -> Result<i32> {
    let mut numElems: i32 = 0;
    numElems = (::match_deref::match_deref! { match &(var.clone()) {
        SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            numElems = 1;
            for mut d in &*var.numArrayElement.clone() {
                let mut d = d.clone();
                numElems = numElems.clone() * stringInt((d.clone()).clone())?;
            }
            numElems.clone()
        },
        _ => 1,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(numElems)
}

pub fn isArrayVar(mut var: SimCodeVar::SimVar) -> bool {
    let mut isArray: bool = false;
    isArray = (::match_deref::match_deref! { match &(var.clone()) {
        SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_ARRAY { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isArray
}

