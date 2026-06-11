// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use openmodelica_ast::Absyn;
use openmodelica_backend::CodegenUtil;
use openmodelica_backend::HpcOmScheduler;
use openmodelica_backend::HpcOmTaskGraph;
use openmodelica_backend::SimCodeUtil;
use openmodelica_codegen_cpp::CodegenCpp;
use openmodelica_codegen_cpp_common::CodegenCppCommon;
use openmodelica_codegen_cpp_common::CodegenCppInit;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::HashTableCrIListArray;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_simcode_types::HpcOmSimCode;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_tpl::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

fn fun_51(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include <mpi.h>")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_52(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = mpiInit(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_53(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = mpiFinalize(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_54(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fileNamePrefix: ArcStr, mut in_a_stateDerVectorName: Tpl::Text, mut in_a_className: Tpl::Text, mut in_a_extraFuncsDeclInit: Tpl::Text, mut in_a_extraFuncsInit: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    let mut out_a_extraFuncsDeclInit: Tpl::Text;
    let mut out_a_extraFuncsInit: Tpl::Text;
    (out_txt, out_a_stateDerVectorName, out_a_extraFuncsDeclInit, out_a_extraFuncsInit) = (match (in_txt, in_mArg, in_a_fileNamePrefix, in_a_stateDerVectorName, in_a_className, in_a_extraFuncsDeclInit, in_a_extraFuncsInit, in_a_simCode) {
        (mut txt, true, mut a_fileNamePrefix, mut a_stateDerVectorName, mut a_className, mut a_extraFuncsDeclInit, mut a_extraFuncsInit, mut a_simCode) => {
            let mut txt_5: Tpl::Text;
            let mut txt_4: Tpl::Text;
            let mut txt_3: Tpl::Text;
            let mut txt_2: Tpl::Text;
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeText(Tpl::emptyTxt.clone(), a_className.clone())?;
            txt_0 = Tpl::writeTok(txt_0, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Initialize")).clone() }))?;
            (txt_1, a_extraFuncsInit, a_extraFuncsDeclInit, txt_0, a_stateDerVectorName) = CodegenCpp::simulationInitParameterCppFile(Tpl::emptyTxt.clone(), a_simCode.clone(), a_extraFuncsInit.clone(), a_extraFuncsDeclInit.clone(), txt_0, a_stateDerVectorName.clone(), false)?;
            txt_2 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_2 = Tpl::writeStr(txt_2, (a_fileNamePrefix.clone()).clone())?;
            txt_2 = Tpl::writeTok(txt_2, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("InitializeParameter.cpp")).clone() }))?;
            Tpl::textFile(txt_1, (Tpl::textString(txt_2)?).clone())?;
            txt_3 = Tpl::writeText(Tpl::emptyTxt.clone(), a_className.clone())?;
            txt_3 = Tpl::writeTok(txt_3, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Initialize")).clone() }))?;
            (txt_4, a_extraFuncsInit, a_extraFuncsDeclInit, txt_3, a_stateDerVectorName) = CodegenCpp::simulationInitAlgVarsCppFile(Tpl::emptyTxt.clone(), a_simCode.clone(), a_extraFuncsInit.clone(), a_extraFuncsDeclInit.clone(), txt_3, a_stateDerVectorName.clone(), false)?;
            txt_5 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_5 = Tpl::writeStr(txt_5, (a_fileNamePrefix.clone()).clone())?;
            txt_5 = Tpl::writeTok(txt_5, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("InitializeAlgVars.cpp")).clone() }))?;
            Tpl::textFile(txt_4, (Tpl::textString(txt_5)?).clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDeclInit.clone(), a_extraFuncsInit.clone())
        },
        (mut txt, _, _, mut a_stateDerVectorName, _, mut a_extraFuncsDeclInit, mut a_extraFuncsInit, _) => {
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDeclInit.clone(), a_extraFuncsInit.clone())
        },
    });
    Ok((out_txt, out_a_stateDerVectorName, out_a_extraFuncsDeclInit, out_a_extraFuncsInit))
}

fn lm_55(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::JacobianColumn>>>, mut a_stateDerVectorName: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    for mut lstElt_55 in &*items {
        let mut lstElt_55 = lstElt_55.clone();
        (txt, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs) = (::match_deref::match_deref! { match &(lstElt_55.clone()) {
        Deref @ SimCode::JacobianColumn { columnEqns: i_eqs, .. } => {
            (txt, a_extraFuncs, a_extraFuncsDecl, _, a_stateDerVectorName) = CodegenCpp::algloopfiles(txt.clone(), i_eqs.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), SimCodeFunction::contextAlgloopJacobian().clone(), 0, a_stateDerVectorName.clone(), false)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        _ => {
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs))
}

fn lm_56(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::JacobianMatrix>>>, mut a_stateDerVectorName: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    for mut lstElt_56 in &*items {
        let mut lstElt_56 = lstElt_56.clone();
        (txt, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs) = (::match_deref::match_deref! { match &(lstElt_56.clone()) {
        Deref @ SimCode::JacobianMatrix { columns: i_mat, .. } => {
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs) = lm_55(txt.clone(), i_mat.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        _ => {
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs))
}

fn fun_57(mut in_txt: Tpl::Text, mut in_a_subPartition: SimCode::SubPartition, mut in_a_stateDerVectorName: Tpl::Text, mut in_a_i: i32, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    (out_txt, out_a_stateDerVectorName, out_a_extraFuncsDecl, out_a_extraFuncs) = (match (in_txt, in_a_subPartition, in_a_stateDerVectorName, in_a_i, in_a_extraFuncsDecl, in_a_extraFuncs, in_a_simCode) {
        (mut txt, SimCode::SubPartition { equations: ref i_equations, removedEquations: ref i_removedEquations, .. }, mut a_stateDerVectorName, mut a_i, mut a_extraFuncsDecl, mut a_extraFuncs, mut a_simCode) => {
            let mut ret_0: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
            ret_0 = listAppend(i_equations.clone(), i_removedEquations.clone());
            (txt, a_extraFuncs, a_extraFuncsDecl, _, a_stateDerVectorName) = CodegenCpp::algloopfiles(txt.clone(), ret_0, a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), SimCodeFunction::contextAlgloop().clone(), a_i.clone(), a_stateDerVectorName.clone(), false)?;
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        (mut txt, _, mut a_stateDerVectorName, _, mut a_extraFuncsDecl, mut a_extraFuncs, _) => {
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
    });
    Ok((out_txt, out_a_stateDerVectorName, out_a_extraFuncsDecl, out_a_extraFuncs))
}

fn lm_58(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCode::SubPartition>>, mut a_stateDerVectorName: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_stateDerVectorName: Tpl::Text = a_stateDerVectorName;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    for mut lstElt_58 in &*items {
        let mut lstElt_58 = lstElt_58.clone();
        (txt, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs) = (match lstElt_58.clone() {
        mut i_subPartition => {
            let mut x_i: i32;
            x_i = Tpl::getIteri_i0(txt.clone())?;
            (txt, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs) = fun_57(txt.clone(), i_subPartition.clone(), a_stateDerVectorName.clone(), x_i, a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
    });
    }
    Ok((txt, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs))
}

pub fn translateModel(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_simCode) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelInfo: ref i_modelInfo @ SimCode::ModelInfo { name: ref i_modelInfo_name, functions: ref i_modelInfo_functions, .. }, makefileParams: SimCodeFunction::MakefileParams { ccompiler: _, .. }, hpcomData: HpcOmSimCode::HpcOmData { hpcOmMemory: ref i_hpcomData_hpcOmMemory, schedules: ref i_hpcomData_schedules }, fileNamePrefix: ref i_fileNamePrefix, allEquations: ref i_allEquations, varToArrayIndexMapping: ref i_varToArrayIndexMapping, literals: ref i_literals, externalFunctionIncludes: ref i_externalFunctionIncludes, jacobianMatrices: ref i_jacobianMatrices, initialEquations: ref i_initialEquations, clockedPartitions: ref i_clockedPartitions, .. }) => {
            let mut txt_100: Tpl::Text;
            let mut txt_99: Tpl::Text;
            let mut txt_98: Tpl::Text;
            let mut txt_97: Tpl::Text;
            let mut ret_96: Arc<metamodelica::List<SimCode::SubPartition>>;
            let mut l_clk: Tpl::Text;
            let mut ret_94: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
            let mut l_alg: Tpl::Text;
            let mut l_jac: Tpl::Text;
            let mut txt_91: Tpl::Text;
            let mut txt_90: Tpl::Text;
            let mut txt_89: Tpl::Text;
            let mut txt_88: Tpl::Text;
            let mut txt_87: Tpl::Text;
            let mut txt_86: Tpl::Text;
            let mut txt_85: Tpl::Text;
            let mut txt_84: Tpl::Text;
            let mut txt_83: Tpl::Text;
            let mut txt_82: Tpl::Text;
            let mut txt_81: Tpl::Text;
            let mut txt_80: Tpl::Text;
            let mut txt_79: Tpl::Text;
            let mut txt_78: Tpl::Text;
            let mut txt_77: Tpl::Text;
            let mut txt_76: Tpl::Text;
            let mut txt_75: Tpl::Text;
            let mut txt_74: Tpl::Text;
            let mut txt_73: Tpl::Text;
            let mut txt_72: Tpl::Text;
            let mut txt_71: Tpl::Text;
            let mut ret_70: bool;
            let mut l_jacobianVarsInit: Tpl::Text;
            let mut txt_68: Tpl::Text;
            let mut txt_67: Tpl::Text;
            let mut txt_66: Tpl::Text;
            let mut ret_65: bool;
            let mut ret_64: bool;
            let mut ret_63: bool;
            let mut l_0__: Tpl::Text;
            let mut txt_61: Tpl::Text;
            let mut txt_60: Tpl::Text;
            let mut txt_59: Tpl::Text;
            let mut txt_58: Tpl::Text;
            let mut txt_57: Tpl::Text;
            let mut l_complexStartExpressions: Tpl::Text;
            let mut l_extraFuncsDeclInit: Tpl::Text;
            let mut l_extraFuncsInit: Tpl::Text;
            let mut txt_53: Tpl::Text;
            let mut txt_52: Tpl::Text;
            let mut txt_51: Tpl::Text;
            let mut txt_50: Tpl::Text;
            let mut l_extraFuncsDeclFun: Tpl::Text;
            let mut l_extraFuncsFun: Tpl::Text;
            let mut txt_47: Tpl::Text;
            let mut txt_46: Tpl::Text;
            let mut txt_45: Tpl::Text;
            let mut txt_44: Tpl::Text;
            let mut txt_43: Tpl::Text;
            let mut txt_42: Tpl::Text;
            let mut txt_41: Tpl::Text;
            let mut ret_40: bool;
            let mut txt_39: Tpl::Text;
            let mut txt_38: Tpl::Text;
            let mut txt_37: Tpl::Text;
            let mut txt_36: Tpl::Text;
            let mut txt_35: Tpl::Text;
            let mut txt_34: Tpl::Text;
            let mut txt_33: Tpl::Text;
            let mut txt_32: Tpl::Text;
            let mut txt_31: Tpl::Text;
            let mut txt_30: Tpl::Text;
            let mut txt_29: Tpl::Text;
            let mut txt_28: Tpl::Text;
            let mut txt_27: Tpl::Text;
            let mut txt_26: Tpl::Text;
            let mut txt_25: Tpl::Text;
            let mut txt_24: Tpl::Text;
            let mut txt_23: Tpl::Text;
            let mut txt_22: Tpl::Text;
            let mut txt_21: Tpl::Text;
            let mut txt_19: Tpl::Text;
            let mut ret_19: bool;
            let mut txt_17: Tpl::Text;
            let mut ret_17: bool;
            let mut txt_15: Tpl::Text;
            let mut ret_15: bool;
            let mut l_numPreVars: Tpl::Text;
            let mut l_numStringVars: Tpl::Text;
            let mut l_numBoolVars: Tpl::Text;
            let mut l_numIntVars: Tpl::Text;
            let mut l_numRealVars: Tpl::Text;
            let mut l_className: Tpl::Text;
            let mut ret_8: bool;
            let mut l_useMemoryOptimization: Tpl::Text;
            let mut l_stateDerVectorName: Tpl::Text;
            let mut l_dummyTypeElemCreation: Tpl::Text;
            let mut l_extraResidualsFuncsDecl: Tpl::Text;
            let mut l_extraFuncsDecl: Tpl::Text;
            let mut l_extraFuncs: Tpl::Text;
            let mut ret_1: ArcStr;
            let mut l_target: Tpl::Text;
            ret_1 = (Config::simulationCodeTarget()?).clone();
            l_target = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1).clone())?;
            l_extraFuncs = Tpl::emptyTxt.clone();
            l_extraFuncsDecl = Tpl::emptyTxt.clone();
            l_extraResidualsFuncsDecl = Tpl::emptyTxt.clone();
            l_dummyTypeElemCreation = Tpl::emptyTxt.clone();
            l_stateDerVectorName = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__zDot")).clone() }))?;
            ret_8 = Flags::isSet(Flags::HPCOM_MEMORY_OPT.clone())?;
            l_useMemoryOptimization = Tpl::writeStr(Tpl::emptyTxt.clone(), (Tpl::booleanString(ret_8)).clone())?;
            l_className = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            l_numRealVars = numRealvarsHpcom(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_hpcomData_hpcOmMemory.clone())?;
            l_numIntVars = numIntvarsHpcom(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_hpcomData_hpcOmMemory.clone())?;
            l_numBoolVars = numBoolvarsHpcom(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_hpcomData_hpcOmMemory.clone())?;
            l_numStringVars = CodegenCpp::numStringvars(Tpl::emptyTxt.clone(), i_modelInfo.clone())?;
            l_numPreVars = numPreVarsHpcom(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_hpcomData_hpcOmMemory.clone())?;
            ret_15 = Flags::isSet(Flags::USEMPI.clone())?;
            txt_15 = fun_51(Tpl::emptyTxt.clone(), ret_15)?;
            ret_17 = Flags::isSet(Flags::USEMPI.clone())?;
            txt_17 = fun_52(Tpl::emptyTxt.clone(), ret_17)?;
            ret_19 = Flags::isSet(Flags::USEMPI.clone())?;
            txt_19 = fun_53(Tpl::emptyTxt.clone(), ret_19)?;
            (txt_21, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::simulationMainFile(Tpl::emptyTxt.clone(), (Tpl::textString(l_target.clone())?).clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), (Tpl::textString(txt_15)?).clone(), (Tpl::textString(txt_17)?).clone(), (Tpl::textString(txt_19)?).clone(), (Tpl::textString(l_numRealVars.clone())?).clone(), (Tpl::textString(l_numIntVars.clone())?).clone(), (Tpl::textString(l_numBoolVars.clone())?).clone(), (Tpl::textString(l_numStringVars.clone())?).clone(), (Tpl::textString(l_numPreVars)?).clone())?;
            txt_22 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_22 = Tpl::writeStr(txt_22, (i_fileNamePrefix.clone()).clone())?;
            txt_22 = Tpl::writeTok(txt_22, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Main.cpp")).clone() }))?;
            Tpl::textFile(txt_21, (Tpl::textString(txt_22)?).clone())?;
            (txt_23, l_extraFuncs, l_extraFuncsDecl, _, l_stateDerVectorName) = updateHpcom(Tpl::emptyTxt.clone(), i_allEquations.clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), SimCodeFunction::contextOther().clone(), l_stateDerVectorName, false)?;
            txt_24 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numRealVars.clone())?;
            txt_24 = Tpl::writeTok(txt_24, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_25 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numIntVars.clone())?;
            txt_25 = Tpl::writeTok(txt_25, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_26 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numBoolVars.clone())?;
            txt_26 = Tpl::writeTok(txt_26, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_27 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numStringVars.clone())?;
            txt_27 = Tpl::writeTok(txt_27, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_28 = additionalHpcomConstructorDefinitions(Tpl::emptyTxt.clone(), i_hpcomData_schedules.clone())?;
            txt_29 = CodegenUtil::dotPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            txt_30 = additionalHpcomConstructorBodyStatements(Tpl::emptyTxt.clone(), i_hpcomData_schedules.clone(), (Tpl::textString(l_className.clone())?).clone(), (Tpl::textString(txt_29)?).clone())?;
            txt_31 = additionalHpcomDestructorBodyStatements(Tpl::emptyTxt.clone(), i_hpcomData_schedules.clone())?;
            (txt_32, txt_23, txt_24, txt_25, txt_26, txt_27, l_extraFuncs, l_extraFuncsDecl, l_className, txt_28, txt_30, txt_31, l_stateDerVectorName) = CodegenCpp::simulationCppFile(Tpl::emptyTxt.clone(), i_simCode.clone(), SimCodeFunction::contextOther().clone(), txt_23, txt_24, txt_25, txt_26, txt_27, l_extraFuncs, l_extraFuncsDecl, l_className, txt_28, txt_30, txt_31, l_stateDerVectorName, false)?;
            txt_33 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_33 = Tpl::writeStr(txt_33, (i_fileNamePrefix.clone()).clone())?;
            txt_33 = Tpl::writeTok(txt_33, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".cpp")).clone() }))?;
            Tpl::textFile(txt_32, (Tpl::textString(txt_33)?).clone())?;
            (txt_34, l_extraFuncs, l_extraFuncsDecl, l_className) = additionalHpcomIncludes(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, l_className, false)?;
            (txt_35, l_extraFuncs, l_extraFuncsDecl, _) = additionalHpcomProtectedMemberDeclaration(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), false)?;
            txt_36 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numRealVars.clone())?;
            txt_36 = Tpl::writeTok(txt_36, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_37 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numIntVars.clone())?;
            txt_37 = Tpl::writeTok(txt_37, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_38 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numBoolVars.clone())?;
            txt_38 = Tpl::writeTok(txt_38, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_39 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numStringVars.clone())?;
            txt_39 = Tpl::writeTok(txt_39, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            ret_40 = Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())?;
            (txt_41, txt_36, txt_37, txt_38, txt_39) = CodegenCpp::memberVariableDefine(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_varToArrayIndexMapping.clone(), txt_36, txt_37, txt_38, txt_39, ret_40, false)?;
            (txt_42, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::simulationHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), SimCodeFunction::contextOther().clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), (Tpl::textString(txt_34)?).clone(), (literal!("")).clone(), (Tpl::textString(txt_35)?).clone(), (Tpl::textString(txt_41)?).clone(), false)?;
            txt_43 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_43 = Tpl::writeStr(txt_43, (i_fileNamePrefix.clone()).clone())?;
            txt_43 = Tpl::writeTok(txt_43, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".h")).clone() }))?;
            Tpl::textFile(txt_42, (Tpl::textString(txt_43)?).clone())?;
            (txt_44, l_extraFuncs, l_extraFuncsDecl, _, l_dummyTypeElemCreation, l_stateDerVectorName) = CodegenCpp::simulationTypesHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), l_dummyTypeElemCreation, i_modelInfo_functions.clone(), i_literals.clone(), l_stateDerVectorName, false)?;
            txt_45 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_45 = Tpl::writeStr(txt_45, (i_fileNamePrefix.clone()).clone())?;
            txt_45 = Tpl::writeTok(txt_45, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Types.h")).clone() }))?;
            Tpl::textFile(txt_44, (Tpl::textString(txt_45)?).clone())?;
            (txt_46, l_extraFuncs, l_extraFuncsDecl, _) = simulationMakefile(Tpl::emptyTxt.clone(), (Tpl::textString(l_target)?).clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            txt_47 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_47 = Tpl::writeTok(txt_47, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".makefile")).clone() }))?;
            Tpl::textFile(txt_46, (Tpl::textString(txt_47)?).clone())?;
            l_extraFuncsFun = Tpl::emptyTxt.clone();
            l_extraFuncsDeclFun = Tpl::emptyTxt.clone();
            (txt_50, l_extraFuncsFun, l_extraFuncsDeclFun, _, l_stateDerVectorName) = CodegenCpp::simulationFunctionsHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncsFun, l_extraFuncsDeclFun, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), i_modelInfo_functions.clone(), i_literals.clone(), l_stateDerVectorName, false)?;
            txt_51 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_51 = Tpl::writeStr(txt_51, (i_fileNamePrefix.clone()).clone())?;
            txt_51 = Tpl::writeTok(txt_51, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Functions.h")).clone() }))?;
            Tpl::textFile(txt_50, (Tpl::textString(txt_51)?).clone())?;
            (txt_52, l_extraFuncsFun, l_extraFuncsDeclFun, _, l_stateDerVectorName) = CodegenCpp::simulationFunctionsFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncsFun, l_extraFuncsDeclFun, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), i_modelInfo_functions.clone(), i_literals.clone(), i_externalFunctionIncludes.clone(), l_stateDerVectorName, false)?;
            txt_53 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_53 = Tpl::writeStr(txt_53, (i_fileNamePrefix.clone()).clone())?;
            txt_53 = Tpl::writeTok(txt_53, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Functions.cpp")).clone() }))?;
            Tpl::textFile(txt_52, (Tpl::textString(txt_53)?).clone())?;
            l_extraFuncsInit = Tpl::emptyTxt.clone();
            l_extraFuncsDeclInit = Tpl::emptyTxt.clone();
            l_complexStartExpressions = Tpl::emptyTxt.clone();
            (txt_57, l_complexStartExpressions, l_stateDerVectorName) = CodegenCppInit::modelInitXMLFile(Tpl::emptyTxt.clone(), i_simCode.clone(), (Tpl::textString(l_numRealVars)?).clone(), (Tpl::textString(l_numIntVars)?).clone(), (Tpl::textString(l_numBoolVars)?).clone(), (Tpl::textString(l_numStringVars)?).clone(), (literal!("")).clone(), (literal!("")).clone(), (literal!("")).clone(), false, (literal!("")).clone(), l_complexStartExpressions, l_stateDerVectorName)?;
            txt_58 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_58 = Tpl::writeTok(txt_58, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_init.xml")).clone() }))?;
            Tpl::textFile(txt_57, (Tpl::textString(txt_58)?).clone())?;
            txt_59 = Tpl::writeText(Tpl::emptyTxt.clone(), l_className.clone())?;
            txt_59 = Tpl::writeTok(txt_59, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Initialize")).clone() }))?;
            (txt_60, l_extraFuncsInit, l_extraFuncsDeclInit, txt_59, l_dummyTypeElemCreation, l_stateDerVectorName, l_complexStartExpressions) = CodegenCpp::simulationInitCppFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncsInit, l_extraFuncsDeclInit, txt_59, l_dummyTypeElemCreation, l_stateDerVectorName, false, l_complexStartExpressions)?;
            txt_61 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_61 = Tpl::writeStr(txt_61, (i_fileNamePrefix.clone()).clone())?;
            txt_61 = Tpl::writeTok(txt_61, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Initialize.cpp")).clone() }))?;
            Tpl::textFile(txt_60, (Tpl::textString(txt_61)?).clone())?;
            ret_63 = Flags::isSet(Flags::HARDCODED_START_VALUES.clone())?;
            ret_64 = Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())?;
            ret_65 = boolOr(ret_63, ret_64);
            (l_0__, l_stateDerVectorName, l_extraFuncsDeclInit, l_extraFuncsInit) = fun_54(Tpl::emptyTxt.clone(), ret_65, (i_fileNamePrefix.clone()).clone(), l_stateDerVectorName, l_className.clone(), l_extraFuncsDeclInit, l_extraFuncsInit, i_simCode.clone())?;
            txt_66 = Tpl::writeText(Tpl::emptyTxt.clone(), l_className.clone())?;
            txt_66 = Tpl::writeTok(txt_66, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Initialize")).clone() }))?;
            (txt_67, l_extraFuncsInit, l_extraFuncsDeclInit, txt_66) = CodegenCpp::simulationInitHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncsInit, l_extraFuncsDeclInit, txt_66)?;
            txt_68 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_68 = Tpl::writeStr(txt_68, (i_fileNamePrefix.clone()).clone())?;
            txt_68 = Tpl::writeTok(txt_68, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Initialize.h")).clone() }))?;
            Tpl::textFile(txt_67, (Tpl::textString(txt_68)?).clone())?;
            l_jacobianVarsInit = Tpl::emptyTxt.clone();
            ret_70 = Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())?;
            (txt_71, l_extraFuncs, l_extraFuncsDecl, _, l_jacobianVarsInit) = CodegenCpp::simulationJacobianHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), l_jacobianVarsInit, ret_70)?;
            txt_72 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_72 = Tpl::writeStr(txt_72, (i_fileNamePrefix.clone()).clone())?;
            txt_72 = Tpl::writeTok(txt_72, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Jacobian.h")).clone() }))?;
            Tpl::textFile(txt_71, (Tpl::textString(txt_72)?).clone())?;
            (txt_73, l_extraFuncs, l_extraFuncsDecl, _, l_jacobianVarsInit, l_stateDerVectorName) = CodegenCpp::simulationJacobianCppFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), l_jacobianVarsInit, l_stateDerVectorName, false)?;
            txt_74 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_74 = Tpl::writeStr(txt_74, (i_fileNamePrefix.clone()).clone())?;
            txt_74 = Tpl::writeTok(txt_74, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Jacobian.cpp")).clone() }))?;
            Tpl::textFile(txt_73, (Tpl::textString(txt_74)?).clone())?;
            (txt_75, l_extraFuncs, l_extraFuncsDecl, _, l_stateDerVectorName) = CodegenCpp::simulationStateSelectionCppFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), l_stateDerVectorName, false)?;
            txt_76 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_76 = Tpl::writeStr(txt_76, (i_fileNamePrefix.clone()).clone())?;
            txt_76 = Tpl::writeTok(txt_76, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("StateSelection.cpp")).clone() }))?;
            Tpl::textFile(txt_75, (Tpl::textString(txt_76)?).clone())?;
            (txt_77, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::simulationStateSelectionHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            txt_78 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_78 = Tpl::writeStr(txt_78, (i_fileNamePrefix.clone()).clone())?;
            txt_78 = Tpl::writeTok(txt_78, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("StateSelection.h")).clone() }))?;
            Tpl::textFile(txt_77, (Tpl::textString(txt_78)?).clone())?;
            (txt_79, l_extraResidualsFuncsDecl, l_className, l_stateDerVectorName) = CodegenCpp::updateResiduals(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraResidualsFuncsDecl, l_className, l_stateDerVectorName, false)?;
            (txt_80, txt_79, l_extraFuncs, l_extraFuncsDecl, _, l_stateDerVectorName) = CodegenCpp::simulationMixedSystemCppFile(Tpl::emptyTxt.clone(), i_simCode.clone(), txt_79, l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), l_stateDerVectorName, false)?;
            txt_81 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_81 = Tpl::writeStr(txt_81, (i_fileNamePrefix.clone()).clone())?;
            txt_81 = Tpl::writeTok(txt_81, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Mixed.cpp")).clone() }))?;
            Tpl::textFile(txt_80, (Tpl::textString(txt_81)?).clone())?;
            (txt_82, l_extraResidualsFuncsDecl) = CodegenCpp::simulationMixedSystemHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraResidualsFuncsDecl)?;
            txt_83 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_83 = Tpl::writeStr(txt_83, (i_fileNamePrefix.clone()).clone())?;
            txt_83 = Tpl::writeTok(txt_83, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Mixed.h")).clone() }))?;
            Tpl::textFile(txt_82, (Tpl::textString(txt_83)?).clone())?;
            (txt_84, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::simulationWriteOutputHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            txt_85 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_85 = Tpl::writeStr(txt_85, (i_fileNamePrefix.clone()).clone())?;
            txt_85 = Tpl::writeTok(txt_85, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WriteOutput.h")).clone() }))?;
            Tpl::textFile(txt_84, (Tpl::textString(txt_85)?).clone())?;
            (txt_86, l_extraFuncs, l_extraFuncsDecl, _, l_stateDerVectorName) = CodegenCpp::simulationWriteOutputCppFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), l_stateDerVectorName, false)?;
            txt_87 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_87 = Tpl::writeStr(txt_87, (i_fileNamePrefix.clone()).clone())?;
            txt_87 = Tpl::writeTok(txt_87, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WriteOutput.cpp")).clone() }))?;
            Tpl::textFile(txt_86, (Tpl::textString(txt_87)?).clone())?;
            (txt_88, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::simulationFactoryFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            txt_89 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_89 = Tpl::writeStr(txt_89, (i_fileNamePrefix.clone()).clone())?;
            txt_89 = Tpl::writeTok(txt_89, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FactoryExport.cpp")).clone() }))?;
            Tpl::textFile(txt_88, (Tpl::textString(txt_89)?).clone())?;
            (txt_90, l_extraFuncs, l_extraFuncsDecl, _) = simulationMainRunScript(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            txt_91 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            (txt_91, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::simulationMainRunScriptSuffix(txt_91, i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            Tpl::textFile(txt_90, (Tpl::textString(txt_91)?).clone())?;
            l_jac = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_jac, l_stateDerVectorName, l_extraFuncsDecl, l_extraFuncs) = lm_56(l_jac, i_jacobianMatrices.clone(), l_stateDerVectorName, l_extraFuncsDecl, l_extraFuncs, i_simCode.clone())?;
            l_jac = Tpl::popIter(l_jac)?;
            ret_94 = listAppend(i_allEquations.clone(), i_initialEquations.clone());
            (l_alg, l_extraFuncs, l_extraFuncsDecl, _, l_stateDerVectorName) = CodegenCpp::algloopfiles(Tpl::emptyTxt.clone(), ret_94, i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), SimCodeFunction::contextAlgloop().clone(), 0, l_stateDerVectorName, false)?;
            ret_96 = SimCodeUtil::getSubPartitions(i_clockedPartitions.clone())?;
            l_clk = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_clk, l_stateDerVectorName, l_extraFuncsDecl, l_extraFuncs) = lm_58(l_clk, ret_96, l_stateDerVectorName, l_extraFuncsDecl, l_extraFuncs, i_simCode.clone())?;
            l_clk = Tpl::popIter(l_clk)?;
            (txt_97, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::algloopMainfile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), SimCodeFunction::contextAlgloop().clone())?;
            txt_98 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_98 = Tpl::writeStr(txt_98, (i_fileNamePrefix.clone()).clone())?;
            txt_98 = Tpl::writeTok(txt_98, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("AlgLoopMain.cpp")).clone() }))?;
            Tpl::textFile(txt_97, (Tpl::textString(txt_98)?).clone())?;
            (txt_99, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::calcHelperMainfile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs, l_extraFuncsDecl, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            txt_100 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_100 = Tpl::writeStr(txt_100, (i_fileNamePrefix.clone()).clone())?;
            txt_100 = Tpl::writeTok(txt_100, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CalcHelperMain.cpp")).clone() }))?;
            Tpl::textFile(txt_99, (Tpl::textString(txt_100)?).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_60(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (match (in_txt, in_a_simCode, in_a_extraFuncs, in_a_extraFuncsDecl, in_a_extraFuncsNamespace) {
        (mut txt, mut i_simCode @ SimCode::SimCode { modelInfo: _, .. }, mut a_extraFuncs, mut a_extraFuncsDecl, mut a_extraFuncsNamespace) => {
            (txt, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = additionalHpcomIncludesForParallelCode(txt.clone(), i_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())?;
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (mut txt, _, mut a_extraFuncs, mut a_extraFuncsDecl, mut a_extraFuncsNamespace) => {
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
    });
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub(crate) fn additionalHpcomIncludes(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = fun_60(txt, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace)?;
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_62(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg)) {
        (txt, Deref @ "openmp") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include <omp.h>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads") => {
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin") => {
            txt.clone()
        },
        (txt, Deref @ "tbb") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#include <tbb/tbb.h>\n")).clone(), (literal!("#include <tbb/flow_graph.h>\n")).clone(), (literal!("#include <tbb/tbb_stddef.h>\n")).clone(), (literal!("#include <boost/function.hpp>\n")).clone(), (literal!("#include <boost/bind.hpp>\n")).clone(), (literal!("#if TBB_INTERFACE_VERSION >= 8000\n")).clone(), (literal!("#include <tbb/task_arena.h>\n")).clone(), (literal!("#endif")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, Deref @ "mpi") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include <mpi.h>")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#include <boost/thread/mutex.hpp>\n")).clone(), (literal!("#include <boost/thread.hpp>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn additionalHpcomIncludesForParallelCode(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut str_2: ArcStr;
    let mut ret_1: ArcStr;
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1).clone())?;
    str_2 = (Tpl::textString(l_type)?).clone();
    out_txt = fun_62(txt, (str_2).clone())?;
    out_a_extraFuncs = a_extraFuncs;
    out_a_extraFuncsDecl = a_extraFuncsDecl;
    out_a_extraFuncsNamespace = a_extraFuncsNamespace;
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_64(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg)) {
        (txt, Deref @ "openmp") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return (long unsigned int)omp_get_thread_num();")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "mpi") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return -1; //not supported")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "tbb") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return -1; //not supported")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#if defined(USE_THREAD)\n")).clone(), (literal!("  #if !defined(USE_CPP_03)\n")).clone(), (literal!("    return std::hash<std::thread::id>()(std::this_thread::get_id());\n")).clone(), (literal!("  #else\n")).clone(), (literal!("    boost::hash<std::string> string_hash;\n")).clone(), (literal!("    return (long unsigned int)string_hash(boost::lexical_cast<std::string>(boost::this_thread::get_id()));\n")).clone(), (literal!("  #endif\n")).clone(), (literal!("#else\n")).clone(), (literal!("  return 0;\n")).clone(), (literal!("#endif")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_65(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_65 in &*items {
        let mut lstElt_65 = lstElt_65.clone();
        txt = (match lstElt_65.clone() {
        mut i_threadIdx => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MeasureTimeValues* measuredSchedulerStartValues_")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_66(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_66 in &*items {
        let mut lstElt_66 = lstElt_66.clone();
        txt = (match lstElt_66.clone() {
        mut i_threadIdx => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MeasureTimeValues* measuredSchedulerEndValues_")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_67(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            let mut ret_3: Arc<metamodelica::List<i32>>;
            let mut ret_2: i32;
            let mut ret_1: Arc<metamodelica::List<i32>>;
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("std::vector<MeasureTimeData*> *measureTimeArrayHpcom;\n")).clone(), (literal!("std::vector<MeasureTimeData*> *measureTimeSchedulerArrayHpcom_evaluateODE;\n")).clone(), (literal!("std::vector<MeasureTimeData*> *measureTimeSchedulerArrayHpcom_evaluateDAE;\n")).clone(), (literal!("std::vector<MeasureTimeData*> *measureTimeSchedulerArrayHpcom_evaluateZeroFuncs;\n")).clone(), (literal!("//MeasureTimeValues *measuredStartValuesODE, *measuredEndValuesODE;\n")).clone(), (literal!("MeasureTimeValues *measuredSchedulerStartValues, *measuredSchedulerEndValues;\n")).clone(), (literal!("\n")).clone(), (literal!("#ifdef MEASURETIME_MODELFUNCTIONS\n")).clone(), (literal!("std::vector<MeasureTimeData*> *measureTimeThreadArrayOdeHpcom;\n")).clone(), (literal!("std::vector<MeasureTimeData*> *measureTimeThreadArrayDaeHpcom;\n")).clone(), (literal!("std::vector<MeasureTimeData*> *measureTimeThreadArrayZeroFuncHpcom;\n")).clone()], lastHasNewLine: true }))?;
            ret_0 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_1 = List::intRange(ret_0);
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_65(txt.clone(), ret_1)?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_2 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_3 = List::intRange(ret_2);
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_66(txt.clone(), ret_3)?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#endif //MEASURETIME_MODELFUNCTIONS")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_68(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncsDecl: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl) = (match (in_txt, in_a_simCode, in_a_extraFuncsDecl) {
        (mut txt, SimCode::SimCode { modelInfo: SimCode::ModelInfo { name: _, .. }, hpcomData: HpcOmSimCode::HpcOmData { schedules: mut i_hpcomData_schedules, .. }, .. }, mut a_extraFuncsDecl) => {
            let mut ret_5: bool;
            let mut ret_4: bool;
            let mut ret_3: ArcStr;
            let mut str_2: ArcStr;
            let mut ret_1: ArcStr;
            let mut l_type: Tpl::Text;
            a_extraFuncsDecl = generateAdditionalFunctionHeaders(a_extraFuncsDecl.clone(), i_hpcomData_schedules.clone())?;
            a_extraFuncsDecl = generateAdditionalHpcomVarHeaders(a_extraFuncsDecl.clone(), i_hpcomData_schedules.clone())?;
            ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
            l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("static long unsigned int getThreadNumber()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            str_2 = (Tpl::textString(l_type)?).clone();
            txt = fun_64(txt.clone(), (str_2).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            ret_3 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_4 = stringEq((ret_3).clone(), (literal!("none")).clone());
            ret_5 = boolNot(ret_4);
            txt = fun_67(txt.clone(), ret_5)?;
            (txt.clone(), a_extraFuncsDecl.clone())
        },
        (mut txt, _, mut a_extraFuncsDecl) => {
            (txt.clone(), a_extraFuncsDecl.clone())
        },
    });
    Ok((out_txt, out_a_extraFuncsDecl))
}

pub(crate) fn additionalHpcomProtectedMemberDeclaration(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl) = fun_68(txt, a_simCode, a_extraFuncsDecl)?;
    out_a_extraFuncs = a_extraFuncs;
    out_a_extraFuncsNamespace = a_extraFuncsNamespace;
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_70(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg)) {
        (txt, Deref @ "openmp") => {
            txt.clone()
        },
        (txt, Deref @ "tbb") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("//Required for Intel TBB\n")).clone(), (literal!("struct VoidFunctionBody {\n")).clone(), (literal!("  function<void(void)> void_function;\n")).clone(), (literal!("  VoidFunctionBody(function<void(void)> void_function) : void_function(void_function) { }\n")).clone(), (literal!("  FORCE_INLINE void operator()( tbb::flow::continue_msg ) const\n")).clone(), (literal!("  {\n")).clone(), (literal!("    void_function();\n")).clone(), (literal!("  }\n")).clone(), (literal!("};\n")).clone(), (literal!("#if TBB_INTERFACE_VERSION >= 8000\n")).clone(), (literal!("struct TbbArenaFunctor\n")).clone(), (literal!("{\n")).clone(), (literal!("  tbb::flow::graph * g;\n")).clone(), (literal!("  tbb::flow::broadcast_node<tbb::flow::continue_msg> * sn;\n")).clone(), (literal!("\n")).clone(), (literal!("  TbbArenaFunctor( )\n")).clone(), (literal!("  {\n")).clone(), (literal!("    g = NULL;\n")).clone(), (literal!("    sn = NULL;\n")).clone(), (literal!("  }\n")).clone(), (literal!("\n")).clone(), (literal!("  TbbArenaFunctor( tbb::flow::graph & in_g , tbb::flow::broadcast_node<tbb::flow::continue_msg> & in_sn )\n")).clone(), (literal!("  {\n")).clone(), (literal!("    g = &in_g;\n")).clone(), (literal!("    sn = &in_sn;\n")).clone(), (literal!("  }\n")).clone(), (literal!("\n")).clone(), (literal!("  void operator()()\n")).clone(), (literal!("  {\n")).clone(), (literal!("    sn->try_put( tbb::flow::continue_msg() );\n")).clone(), (literal!("    g->wait_for_all();\n")).clone(), (literal!("  }\n")).clone(), (literal!("\n")).clone(), (literal!("};\n")).clone(), (literal!("#endif")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_71(mut in_txt: Tpl::Text, mut in_a_odeSchedule: Arc<HpcOmSimCode::Schedule>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_odeSchedule, in_a_type)) {
        (txt, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: _ }, a_type) => {
            let mut str_0: ArcStr;
            let mut txt = (*txt).clone();
            str_0 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_70(txt.clone(), (str_0).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn generateAdditionalStructHeaders(mut txt: Tpl::Text, mut a_odeSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: ArcStr;
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1).clone())?;
    out_txt = fun_71(txt, a_odeSchedule, l_type)?;
    Ok(out_txt)
}

fn lm_73(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_73 in &*items {
        let mut lstElt_73 = lstElt_73.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_73.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThreadFunctionHeaderDecl(txt.clone(), x_i0)?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_74(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_odeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_odeSchedule_threadTasks.clone())) {
        (txt, Deref @ "openmp", _) => {
            txt.clone()
        },
        (txt, _, a_odeSchedule_threadTasks) => {
            let mut ret_2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_headers: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_2 = listRest(ret_1)?;
            l_headers = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_headers = lm_73(l_headers, ret_2)?;
            l_headers = Tpl::popIter(l_headers)?;
            txt = Tpl::writeText(txt.clone(), l_headers)?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_75(mut in_txt: Tpl::Text, mut in_a_task: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_task)) {
        (txt, (Deref @ HpcOmSimCode::Task::CALCTASK { index: i_task_index, .. }, _)) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void taskFuncOde_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("();")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_76(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_76 in &*items {
        let mut lstElt_76 = lstElt_76.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_76.clone()) {
        i_task => {
            txt = fun_75(txt.clone(), i_task.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_77(mut in_txt: Tpl::Text, mut in_a_task: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_task)) {
        (txt, (Deref @ HpcOmSimCode::Task::CALCTASK { index: i_task_index, .. }, _)) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void taskFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("();")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_78(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_78 in &*items {
        let mut lstElt_78 = lstElt_78.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_78.clone()) {
        i_task => {
            txt = fun_77(txt.clone(), i_task.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_79(mut in_txt: Tpl::Text, mut in_a_task: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_task)) {
        (txt, (Deref @ HpcOmSimCode::Task::CALCTASK { index: i_task_index, .. }, _)) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void taskFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("();")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_80(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_80 in &*items {
        let mut lstElt_80 = lstElt_80.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_80.clone()) {
        i_task => {
            txt = fun_79(txt.clone(), i_task.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_81(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_odeSchedule: Arc<HpcOmSimCode::Schedule>, mut in_a_zeroFuncSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_daeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_odeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_odeSchedule, in_a_zeroFuncSchedule_tasks, in_a_daeSchedule_tasks, in_a_odeSchedule_tasks)) {
        (txt, Deref @ "openmp", _, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ "tbb", a_odeSchedule, a_zeroFuncSchedule_tasks, a_daeSchedule_tasks, a_odeSchedule_tasks) => {
            let mut l_voidfuncsZeroFunc: Tpl::Text;
            let mut l_voidfuncsDae: Tpl::Text;
            let mut l_voidfuncsOde: Tpl::Text;
            let mut txt = (*txt).clone();
            l_voidfuncsOde = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_voidfuncsOde = lm_76(l_voidfuncsOde, a_odeSchedule_tasks.clone())?;
            l_voidfuncsOde = Tpl::popIter(l_voidfuncsOde)?;
            l_voidfuncsDae = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_voidfuncsDae = lm_78(l_voidfuncsDae, a_daeSchedule_tasks.clone())?;
            l_voidfuncsDae = Tpl::popIter(l_voidfuncsDae)?;
            l_voidfuncsZeroFunc = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_voidfuncsZeroFunc = lm_80(l_voidfuncsZeroFunc, a_zeroFuncSchedule_tasks.clone())?;
            l_voidfuncsZeroFunc = Tpl::popIter(l_voidfuncsZeroFunc)?;
            txt = generateAdditionalStructHeaders(txt.clone(), a_odeSchedule.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_voidfuncsOde)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_voidfuncsDae)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_voidfuncsZeroFunc)?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_82(mut in_txt: Tpl::Text, mut in_a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_schedulesOpt, in_a_type)) {
        (txt, Some((Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: i_odeSchedule_threadTasks, .. }, _, _)), a_type) => {
            let mut str_0: ArcStr;
            let mut txt = (*txt).clone();
            str_0 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_74(txt.clone(), (str_0).clone(), i_odeSchedule_threadTasks.clone())?;
            txt.clone()
        },
        (txt, Some((i_odeSchedule @ Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_odeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_daeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_zeroFuncSchedule_tasks })), a_type) => {
            let mut str_1: ArcStr;
            let mut txt = (*txt).clone();
            str_1 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_81(txt.clone(), (str_1).clone(), i_odeSchedule.clone(), i_zeroFuncSchedule_tasks.clone(), i_daeSchedule_tasks.clone(), i_odeSchedule_tasks.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn generateAdditionalFunctionHeaders(mut txt: Tpl::Text, mut a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: ArcStr;
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1).clone())?;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("FORCE_INLINE void evaluateParallel(const UPDATETYPE command, int evaluateMode);\n")).clone() }))?;
    out_txt = fun_82(out_txt, a_schedulesOpt, l_type)?;
    Ok(out_txt)
}

fn lm_84(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_84 in &*items {
        let mut lstElt_84 = lstElt_84.clone();
        txt = (match lstElt_84.clone() {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThreadHeaderDecl(txt.clone(), x_i0, (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_85(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_85 in &*items {
        let mut lstElt_85 = lstElt_85.clone();
        txt = (match lstElt_85.clone() {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThreadHeaderDecl(txt.clone(), x_i0, (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_86(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_type)) {
        (txt, Deref @ "pthreads", a_type) => {
            let mut ret_2: i32;
            let mut ret_1: Arc<metamodelica::List<i32>>;
            let mut ret_0: i32;
            let mut txt = (*txt).clone();
            ret_0 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_1 = List::intRange(ret_0);
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: None, alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_84(txt.clone(), ret_1, a_type.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_2 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = createBarrierByName(txt.clone(), (literal!("levelBarrier")).clone(), (literal!("")).clone(), ret_2, (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = createLockByLockName(txt.clone(), (literal!("measureTimeArrayLock")).clone(), (literal!("")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("bool _simulationFinished;")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_type) => {
            let mut ret_5: i32;
            let mut ret_4: Arc<metamodelica::List<i32>>;
            let mut ret_3: i32;
            let mut txt = (*txt).clone();
            ret_3 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_4 = List::intRange(ret_3);
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: None, alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_85(txt.clone(), ret_4, a_type.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_5 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = createBarrierByName(txt.clone(), (literal!("levelBarrier")).clone(), (literal!("")).clone(), ret_5, (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = createLockByLockName(txt.clone(), (literal!("measureTimeArrayLock")).clone(), (literal!("")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("bool _simulationFinished;")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_87(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_87 in &*items {
        let mut lstElt_87 = lstElt_87.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_87.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThreadHeaderDecl(txt.clone(), x_i0, (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_88(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_88 in &*items {
        let mut lstElt_88 = lstElt_88.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_88.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThreadHeaderDecl(txt.clone(), x_i0, (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_89(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_89 in &*items {
        let mut lstElt_89 = lstElt_89.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_89.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = createLockByLockName(txt.clone(), (intString(x_i0)).clone(), (literal!("th_lock")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_90(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_90 in &*items {
        let mut lstElt_90 = lstElt_90.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_90.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = createLockByLockName(txt.clone(), (intString(x_i0)).clone(), (literal!("th_lock1")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_91(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_zeroFuncLocks: Tpl::Text, mut in_a_daeLocks: Tpl::Text, mut in_a_odeLocks: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_odeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_zeroFuncLocks, in_a_daeLocks, in_a_odeLocks, in_a_type, in_a_odeSchedule_threadTasks.clone())) {
        (txt, Deref @ "openmp", a_zeroFuncLocks, a_daeLocks, a_odeLocks, a_type, a_odeSchedule_threadTasks) => {
            let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadDecl: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadDecl = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadDecl = lm_87(l_threadDecl, ret_1, a_type.clone())?;
            l_threadDecl = Tpl::popIter(l_threadDecl)?;
            txt = Tpl::writeText(txt.clone(), a_odeLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_daeLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_zeroFuncLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadDecl)?;
            txt.clone()
        },
        (txt, Deref @ "mpi", _, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//MF Todo BLABLUB")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_zeroFuncLocks, a_daeLocks, a_odeLocks, a_type, a_odeSchedule_threadTasks) => {
            let mut ret_9: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_8: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_thLocks1: Tpl::Text;
            let mut ret_6: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_5: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_thLocks: Tpl::Text;
            let mut ret_3: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadDecl: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_2 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_3 = listRest(ret_2)?;
            l_threadDecl = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadDecl = lm_88(l_threadDecl, ret_3, a_type.clone())?;
            l_threadDecl = Tpl::popIter(l_threadDecl)?;
            ret_5 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_6 = listRest(ret_5)?;
            l_thLocks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_thLocks = lm_89(l_thLocks, ret_6, a_type.clone())?;
            l_thLocks = Tpl::popIter(l_thLocks)?;
            ret_8 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_9 = listRest(ret_8)?;
            l_thLocks1 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_thLocks1 = lm_90(l_thLocks1, ret_9, a_type.clone())?;
            l_thLocks1 = Tpl::popIter(l_thLocks1)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("bool _terminateThreads;\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_odeLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_daeLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_zeroFuncLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_thLocks)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_thLocks1)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadDecl)?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_92(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg)) {
        (txt, Deref @ "openmp") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "tbb") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("tbb::flow::graph _tbbGraphOde;\n")).clone(), (literal!("tbb::flow::broadcast_node<tbb::flow::continue_msg> _tbbStartNodeOde;\n")).clone(), (literal!("tbb::flow::graph _tbbGraphAll;\n")).clone(), (literal!("tbb::flow::broadcast_node<tbb::flow::continue_msg> _tbbStartNodeAll;\n")).clone(), (literal!("tbb::flow::graph _tbbGraphZeroFunc;\n")).clone(), (literal!("tbb::flow::broadcast_node<tbb::flow::continue_msg> _tbbStartNodeZeroFunc;\n")).clone(), (literal!("std::vector<tbb::flow::continue_node<tbb::flow::continue_msg>* > _tbbNodeListOde;\n")).clone(), (literal!("std::vector<tbb::flow::continue_node<tbb::flow::continue_msg>* > _tbbNodeListAll;\n")).clone(), (literal!("std::vector<tbb::flow::continue_node<tbb::flow::continue_msg>* > _tbbNodeListZeroFunc;\n")).clone(), (literal!("#if TBB_INTERFACE_VERSION >= 8000\n")).clone(), (literal!("tbb::task_arena _tbbArena;\n")).clone(), (literal!("TbbArenaFunctor _tbbArenaFunctorOde;\n")).clone(), (literal!("TbbArenaFunctor _tbbArenaFunctorAll;\n")).clone(), (literal!("TbbArenaFunctor _tbbArenaFunctorZeroFunc;\n")).clone(), (literal!("#endif")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_93(mut in_txt: Tpl::Text, mut in_a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_schedulesOpt, in_a_type)) {
        (txt, Some((Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, .. }, _, _)), a_type) => {
            let mut str_0: ArcStr;
            let mut txt = (*txt).clone();
            str_0 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_86(txt.clone(), (str_0).clone(), a_type.clone())?;
            txt.clone()
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: i_odeSchedule_outgoingDepTasks, threadTasks: i_odeSchedule_threadTasks, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: i_daeSchedule_outgoingDepTasks, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: i_zeroFuncSchedule_outgoingDepTasks, .. })), a_type) => {
            let mut str_7: ArcStr;
            let mut ret_6: i32;
            let mut l_zeroFuncLocks: Tpl::Text;
            let mut ret_4: i32;
            let mut l_daeLocks: Tpl::Text;
            let mut ret_2: i32;
            let mut l_odeLocks: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_2 = (i_odeSchedule_outgoingDepTasks.clone().len() as i32);
            l_odeLocks = createLockArrayByName(Tpl::emptyTxt.clone(), ret_2, (literal!("_lockOde")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_4 = (i_daeSchedule_outgoingDepTasks.clone().len() as i32);
            l_daeLocks = createLockArrayByName(Tpl::emptyTxt.clone(), ret_4, (literal!("_lockDae")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_6 = (i_zeroFuncSchedule_outgoingDepTasks.clone().len() as i32);
            l_zeroFuncLocks = createLockArrayByName(Tpl::emptyTxt.clone(), ret_6, (literal!("_lockZeroFunc")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            str_7 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_91(txt.clone(), (str_7).clone(), l_zeroFuncLocks, l_daeLocks, l_odeLocks, a_type.clone(), i_odeSchedule_threadTasks.clone())?;
            txt.clone()
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: _ }, _, _)), a_type) => {
            let mut str_8: ArcStr;
            let mut txt = (*txt).clone();
            str_8 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_92(txt.clone(), (str_8).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn generateAdditionalHpcomVarHeaders(mut txt: Tpl::Text, mut a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: ArcStr;
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1).clone())?;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("UPDATETYPE _command;\n")).clone(), (literal!("int _evaluateMode;\n")).clone()], lastHasNewLine: true }))?;
    out_txt = fun_93(out_txt, a_schedulesOpt, l_type)?;
    Ok(out_txt)
}

fn fun_95(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_threadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_threadIdx)) {
        (txt, Deref @ "openmp", _) => {
            txt.clone()
        },
        (txt, _, a_threadIdx) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("thread* evaluateThread")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_threadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn generateThreadHeaderDecl(mut txt: Tpl::Text, mut a_threadIdx: i32, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_95(txt, (a_iType).clone(), a_threadIdx)?;
    Ok(out_txt)
}

pub(crate) fn generateThreadFunctionHeaderDecl(mut txt: Tpl::Text, mut a_threadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void evaluateThreadFunc")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt, (intString(a_threadIdx)).clone())?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("();")).clone() }))?;
    Ok(out_txt)
}

fn fun_98(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_type)) {
        (txt, Deref @ "pthreads", a_type) => {
            let mut ret_0: i32;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",_command(IContinuous::UNDEF_UPDATE)\n")).clone(), (literal!(",_simulationFinished(false)\n")).clone(), (literal!(",")).clone()], lastHasNewLine: false }))?;
            ret_0 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = initializeBarrierByName(txt.clone(), (literal!("levelBarrier")).clone(), (literal!("")).clone(), ret_0, (Tpl::textString(a_type.clone())?).clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_type) => {
            let mut ret_1: i32;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",_command(IContinuous::UNDEF_UPDATE)\n")).clone(), (literal!(",_simulationFinished(false)\n")).clone(), (literal!(",")).clone()], lastHasNewLine: false }))?;
            ret_1 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = initializeBarrierByName(txt.clone(), (literal!("levelBarrier")).clone(), (literal!("")).clone(), ret_1, (Tpl::textString(a_type.clone())?).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_99(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_zeroFuncSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_daeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_odeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_zeroFuncSchedule_tasks, in_a_daeSchedule_tasks, in_a_odeSchedule_tasks)) {
        (txt, Deref @ "tbb", a_zeroFuncSchedule_tasks, a_daeSchedule_tasks, a_odeSchedule_tasks) => {
            let mut ret_2: i32;
            let mut ret_1: i32;
            let mut ret_0: i32;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",_tbbGraphOde()\n")).clone(), (literal!(",_tbbGraphAll()\n")).clone(), (literal!(",_tbbGraphZeroFunc()\n")).clone(), (literal!(",_tbbStartNodeOde(_tbbGraphOde)\n")).clone(), (literal!(",_tbbStartNodeAll(_tbbGraphAll)\n")).clone(), (literal!(",_tbbStartNodeZeroFunc(_tbbGraphZeroFunc)\n")).clone(), (literal!(",_tbbNodeListOde(")).clone()], lastHasNewLine: false }))?;
            ret_0 = (a_odeSchedule_tasks.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",NULL)\n")).clone(), (literal!(",_tbbNodeListAll(")).clone()], lastHasNewLine: false }))?;
            ret_1 = (a_daeSchedule_tasks.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",NULL)\n")).clone(), (literal!(",_tbbNodeListZeroFunc(")).clone()], lastHasNewLine: false }))?;
            ret_2 = (a_zeroFuncSchedule_tasks.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_2)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",NULL)")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_100(mut in_txt: Tpl::Text, mut in_a_scheduleOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_scheduleOpt, in_a_type)) {
        (txt, Some((Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, .. }, _, _)), a_type) => {
            let mut str_0: ArcStr;
            let mut txt = (*txt).clone();
            str_0 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_98(txt.clone(), (str_0).clone(), a_type.clone())?;
            txt.clone()
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_odeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_daeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_zeroFuncSchedule_tasks })), a_type) => {
            let mut str_1: ArcStr;
            let mut txt = (*txt).clone();
            str_1 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_99(txt.clone(), (str_1).clone(), i_zeroFuncSchedule_tasks.clone(), i_daeSchedule_tasks.clone(), i_odeSchedule_tasks.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn additionalHpcomConstructorDefinitions(mut txt: Tpl::Text, mut a_scheduleOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: ArcStr;
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1).clone())?;
    out_txt = fun_100(txt, a_scheduleOpt, l_type)?;
    Ok(out_txt)
}

fn fun_102(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fullModelName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_fullModelName) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_fullModelName) => {
            let mut ret_0: i32;
            ret_0 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = generateThreadMeasureTimeDeclaration(txt.clone(), (a_fullModelName.clone()).clone(), ret_0)?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_103(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>, mut a_modelNamePrefixStr: ArcStr, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_103 in &*items {
        let mut lstElt_103 = lstElt_103.clone();
        txt = (match lstElt_103.clone() {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThread(txt.clone(), x_i0, (Tpl::textString(a_type.clone())?).clone(), (a_modelNamePrefixStr.clone()).clone(), (literal!("evaluateThreadFunc")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_104(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_104 in &*items {
        let mut lstElt_104 = lstElt_104.clone();
        txt = (match lstElt_104.clone() {
        mut i_levelIdx => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeSchedulerArrayHpcom_evaluateODE)[")).clone() }))?;
            ret_0 = intSub(i_levelIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateODE_level_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_levelIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_105(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_105 in &*items {
        let mut lstElt_105 = lstElt_105.clone();
        txt = (match lstElt_105.clone() {
        mut i_levelIdx => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeSchedulerArrayHpcom_evaluateDAE)[")).clone() }))?;
            ret_0 = intSub(i_levelIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateDAE_level_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_levelIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_106(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_106 in &*items {
        let mut lstElt_106 = lstElt_106.clone();
        txt = (match lstElt_106.clone() {
        mut i_levelIdx => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeSchedulerArrayHpcom_evaluateZeroFuncs)[")).clone() }))?;
            ret_0 = intSub(i_levelIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateZeroFunc_level_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_levelIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_107(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_zeroFuncSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_daeSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_fullModelName: ArcStr, mut in_a_odeSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_zeroFuncSchedule_tasksOfLevels, in_a_daeSchedule_tasksOfLevels, in_a_fullModelName, in_a_odeSchedule_tasksOfLevels)) {
        (txt, false, _, _, _, _) => {
            txt.clone()
        },
        (txt, _, a_zeroFuncSchedule_tasksOfLevels, a_daeSchedule_tasksOfLevels, a_fullModelName, a_odeSchedule_tasksOfLevels) => {
            let mut ret_8: Arc<metamodelica::List<i32>>;
            let mut ret_7: i32;
            let mut ret_6: i32;
            let mut ret_5: Arc<metamodelica::List<i32>>;
            let mut ret_4: i32;
            let mut ret_3: i32;
            let mut ret_2: Arc<metamodelica::List<i32>>;
            let mut ret_1: i32;
            let mut ret_0: i32;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#ifdef MEASURETIME_MODELFUNCTIONS\n")).clone(), (literal!("measureTimeSchedulerArrayHpcom_evaluateODE = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
            ret_0 = (a_odeSchedule_tasksOfLevels.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fullModelName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"functions_HPCOM_Sections_ODE\",measureTimeSchedulerArrayHpcom_evaluateODE);\n")).clone(), (literal!("measuredSchedulerStartValues = MeasureTime::getZeroValues();\n")).clone(), (literal!("measuredSchedulerEndValues = MeasureTime::getZeroValues();\n")).clone()], lastHasNewLine: true }))?;
            ret_1 = (a_odeSchedule_tasksOfLevels.clone().len() as i32);
            ret_2 = List::intRange(ret_1);
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_104(txt.clone(), ret_2)?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("measureTimeSchedulerArrayHpcom_evaluateDAE = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
            ret_3 = (a_daeSchedule_tasksOfLevels.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_3)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fullModelName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"functions_HPCOM_Sections_DAE\",measureTimeSchedulerArrayHpcom_evaluateDAE);\n")).clone(), (literal!("measuredSchedulerStartValues = MeasureTime::getZeroValues();\n")).clone(), (literal!("measuredSchedulerEndValues = MeasureTime::getZeroValues();\n")).clone()], lastHasNewLine: true }))?;
            ret_4 = (a_daeSchedule_tasksOfLevels.clone().len() as i32);
            ret_5 = List::intRange(ret_4);
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_105(txt.clone(), ret_5)?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("measureTimeSchedulerArrayHpcom_evaluateZeroFuncs = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
            ret_6 = (a_zeroFuncSchedule_tasksOfLevels.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_6)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fullModelName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"functions_HPCOM_Sections_ZeroFuncs\",measureTimeSchedulerArrayHpcom_evaluateZeroFuncs);\n")).clone(), (literal!("measuredSchedulerStartValues = MeasureTime::getZeroValues();\n")).clone(), (literal!("measuredSchedulerEndValues = MeasureTime::getZeroValues();\n")).clone()], lastHasNewLine: true }))?;
            ret_7 = (a_zeroFuncSchedule_tasksOfLevels.clone().len() as i32);
            ret_8 = List::intRange(ret_7);
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_106(txt.clone(), ret_8)?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#endif //MEASURETIME_MODELFUNCTIONS")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_108(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>, mut a_modelNamePrefixStr: ArcStr, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_108 in &*items {
        let mut lstElt_108 = lstElt_108.clone();
        txt = (match lstElt_108.clone() {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThread(txt.clone(), x_i0, (Tpl::textString(a_type.clone())?).clone(), (a_modelNamePrefixStr.clone()).clone(), (literal!("evaluateThreadFunc")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_109(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_109 in &*items {
        let mut lstElt_109 = lstElt_109.clone();
        txt = (match lstElt_109.clone() {
        mut i_levelIdx => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeSchedulerArrayHpcom_evaluateODE)[")).clone() }))?;
            ret_0 = intSub(i_levelIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateODE_level_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_levelIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_110(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_110 in &*items {
        let mut lstElt_110 = lstElt_110.clone();
        txt = (match lstElt_110.clone() {
        mut i_levelIdx => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeSchedulerArrayHpcom_evaluateDAE)[")).clone() }))?;
            ret_0 = intSub(i_levelIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateDAE_level_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_levelIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_111(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_111 in &*items {
        let mut lstElt_111 = lstElt_111.clone();
        txt = (match lstElt_111.clone() {
        mut i_levelIdx => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeSchedulerArrayHpcom_evaluateZeroFuncs)[")).clone() }))?;
            ret_0 = intSub(i_levelIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateZeroFunc_level_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_levelIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_112(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_zeroFuncSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_daeSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_fullModelName: ArcStr, mut in_a_odeSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_zeroFuncSchedule_tasksOfLevels, in_a_daeSchedule_tasksOfLevels, in_a_fullModelName, in_a_odeSchedule_tasksOfLevels)) {
        (txt, false, _, _, _, _) => {
            txt.clone()
        },
        (txt, _, a_zeroFuncSchedule_tasksOfLevels, a_daeSchedule_tasksOfLevels, a_fullModelName, a_odeSchedule_tasksOfLevels) => {
            let mut ret_8: Arc<metamodelica::List<i32>>;
            let mut ret_7: i32;
            let mut ret_6: i32;
            let mut ret_5: Arc<metamodelica::List<i32>>;
            let mut ret_4: i32;
            let mut ret_3: i32;
            let mut ret_2: Arc<metamodelica::List<i32>>;
            let mut ret_1: i32;
            let mut ret_0: i32;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#ifdef MEASURETIME_MODELFUNCTIONS\n")).clone(), (literal!("measureTimeSchedulerArrayHpcom_evaluateODE = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
            ret_0 = (a_odeSchedule_tasksOfLevels.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fullModelName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"functions_HPCOM_Sections_ODE\",measureTimeSchedulerArrayHpcom_evaluateODE);\n")).clone(), (literal!("measuredSchedulerStartValues = MeasureTime::getZeroValues();\n")).clone(), (literal!("measuredSchedulerEndValues = MeasureTime::getZeroValues();\n")).clone()], lastHasNewLine: true }))?;
            ret_1 = (a_odeSchedule_tasksOfLevels.clone().len() as i32);
            ret_2 = List::intRange(ret_1);
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_109(txt.clone(), ret_2)?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("measureTimeSchedulerArrayHpcom_evaluateDAE = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
            ret_3 = (a_daeSchedule_tasksOfLevels.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_3)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fullModelName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"functions_HPCOM_Sections_DAE\",measureTimeSchedulerArrayHpcom_evaluateDAE);\n")).clone(), (literal!("measuredSchedulerStartValues = MeasureTime::getZeroValues();\n")).clone(), (literal!("measuredSchedulerEndValues = MeasureTime::getZeroValues();\n")).clone()], lastHasNewLine: true }))?;
            ret_4 = (a_daeSchedule_tasksOfLevels.clone().len() as i32);
            ret_5 = List::intRange(ret_4);
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_110(txt.clone(), ret_5)?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("measureTimeSchedulerArrayHpcom_evaluateZeroFuncs = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
            ret_6 = (a_zeroFuncSchedule_tasksOfLevels.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_6)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fullModelName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"functions_HPCOM_Sections_ZeroFuncs\",measureTimeSchedulerArrayHpcom_evaluateZeroFuncs);\n")).clone(), (literal!("measuredSchedulerStartValues = MeasureTime::getZeroValues();\n")).clone(), (literal!("measuredSchedulerEndValues = MeasureTime::getZeroValues();\n")).clone()], lastHasNewLine: true }))?;
            ret_7 = (a_zeroFuncSchedule_tasksOfLevels.clone().len() as i32);
            ret_8 = List::intRange(ret_7);
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_111(txt.clone(), ret_8)?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#endif //MEASURETIME_MODELFUNCTIONS")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_113(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_zeroFuncSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_daeSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_fullModelName: ArcStr, mut in_a_odeSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_zeroFuncSchedule_tasksOfLevels, in_a_daeSchedule_tasksOfLevels, in_a_fullModelName, in_a_odeSchedule_tasksOfLevels, in_a_modelNamePrefixStr, in_a_type)) {
        (txt, Deref @ "pthreads", a_zeroFuncSchedule_tasksOfLevels, a_daeSchedule_tasksOfLevels, a_fullModelName, a_odeSchedule_tasksOfLevels, a_modelNamePrefixStr, a_type) => {
            let mut ret_6: bool;
            let mut ret_5: bool;
            let mut ret_4: ArcStr;
            let mut ret_3: Arc<metamodelica::List<i32>>;
            let mut ret_2: i32;
            let mut ret_1: i32;
            let mut l_threadFuncs: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_2 = intSub(ret_1, 1);
            ret_3 = List::intRange(ret_2);
            l_threadFuncs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadFuncs = lm_103(l_threadFuncs, ret_3, (a_modelNamePrefixStr.clone()).clone(), a_type.clone())?;
            l_threadFuncs = Tpl::popIter(l_threadFuncs)?;
            txt = Tpl::writeText(txt.clone(), l_threadFuncs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            ret_4 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_5 = stringEq((ret_4).clone(), (literal!("none")).clone());
            ret_6 = boolNot(ret_5);
            txt = fun_107(txt.clone(), ret_6, a_zeroFuncSchedule_tasksOfLevels.clone(), a_daeSchedule_tasksOfLevels.clone(), (a_fullModelName.clone()).clone(), a_odeSchedule_tasksOfLevels.clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_zeroFuncSchedule_tasksOfLevels, a_daeSchedule_tasksOfLevels, a_fullModelName, a_odeSchedule_tasksOfLevels, a_modelNamePrefixStr, a_type) => {
            let mut ret_12: bool;
            let mut ret_11: bool;
            let mut ret_10: ArcStr;
            let mut ret_9: Arc<metamodelica::List<i32>>;
            let mut ret_8: i32;
            let mut ret_7: i32;
            let mut l_threadFuncs: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_7 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_8 = intSub(ret_7, 1);
            ret_9 = List::intRange(ret_8);
            l_threadFuncs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadFuncs = lm_108(l_threadFuncs, ret_9, (a_modelNamePrefixStr.clone()).clone(), a_type.clone())?;
            l_threadFuncs = Tpl::popIter(l_threadFuncs)?;
            txt = Tpl::writeText(txt.clone(), l_threadFuncs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            ret_10 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_11 = stringEq((ret_10).clone(), (literal!("none")).clone());
            ret_12 = boolNot(ret_11);
            txt = fun_112(txt.clone(), ret_12, a_zeroFuncSchedule_tasksOfLevels.clone(), a_daeSchedule_tasksOfLevels.clone(), (a_fullModelName.clone()).clone(), a_odeSchedule_tasksOfLevels.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_114(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_modelNamePrefixStr: ArcStr, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_114 in &*items {
        let mut lstElt_114 = lstElt_114.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_114.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThread(txt.clone(), x_i0, (Tpl::textString(a_type.clone())?).clone(), (a_modelNamePrefixStr.clone()).clone(), (literal!("evaluateThreadFunc")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_115(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_modelNamePrefixStr: ArcStr, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_115 in &*items {
        let mut lstElt_115 = lstElt_115.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_115.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThread(txt.clone(), x_i0, (Tpl::textString(a_type.clone())?).clone(), (a_modelNamePrefixStr.clone()).clone(), (literal!("evaluateThreadFunc")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_116(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_116 in &*items {
        let mut lstElt_116 = lstElt_116.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_116.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = initializeLockByLockName(txt.clone(), (intString(x_i0)).clone(), (literal!("th_lock")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_117(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_117 in &*items {
        let mut lstElt_117 = lstElt_117.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_117.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = initializeLockByLockName(txt.clone(), (intString(x_i0)).clone(), (literal!("th_lock1")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_118(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_118 in &*items {
        let mut lstElt_118 = lstElt_118.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_118.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = assignLockByLockName(txt.clone(), (intString(x_i0)).clone(), (literal!("th_lock")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_119(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_119 in &*items {
        let mut lstElt_119 = lstElt_119.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_119.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = assignLockByLockName(txt.clone(), (intString(x_i0)).clone(), (literal!("th_lock1")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_120(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_assignLocksZeroFunc: Tpl::Text, mut in_a_assignLocksOde: Tpl::Text, mut in_a_assignLocksDae: Tpl::Text, mut in_a_initLocksZeroFunc: Tpl::Text, mut in_a_initLocksDae: Tpl::Text, mut in_a_initLocksOde: Tpl::Text, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_type: Tpl::Text, mut in_a_odeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_assignLocksZeroFunc, in_a_assignLocksOde, in_a_assignLocksDae, in_a_initLocksZeroFunc, in_a_initLocksDae, in_a_initLocksOde, in_a_modelNamePrefixStr, in_a_type, in_a_odeSchedule_threadTasks.clone())) {
        (txt, Deref @ "openmp", _, _, _, a_initLocksZeroFunc, a_initLocksDae, a_initLocksOde, a_modelNamePrefixStr, a_type, a_odeSchedule_threadTasks) => {
            let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadFuncs: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadFuncs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadFuncs = lm_114(l_threadFuncs, ret_1, (a_modelNamePrefixStr.clone()).clone(), a_type.clone())?;
            l_threadFuncs = Tpl::popIter(l_threadFuncs)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("omp_set_dynamic(0);\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_threadFuncs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_initLocksOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_initLocksDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_initLocksZeroFunc.clone())?;
            txt.clone()
        },
        (txt, Deref @ "mpi", _, _, _, _, _, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//MF: Initialize MPI related stuff - nothing todo?")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_assignLocksZeroFunc, a_assignLocksOde, a_assignLocksDae, a_initLocksZeroFunc, a_initLocksDae, a_initLocksOde, a_modelNamePrefixStr, a_type, a_odeSchedule_threadTasks) => {
            let mut ret_15: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_14: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadAssignLocks1: Tpl::Text;
            let mut ret_12: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_11: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadAssignLocks: Tpl::Text;
            let mut ret_9: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_8: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadLocksInit1: Tpl::Text;
            let mut ret_6: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_5: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadLocksInit: Tpl::Text;
            let mut ret_3: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadFuncs: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_2 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_3 = listRest(ret_2)?;
            l_threadFuncs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadFuncs = lm_115(l_threadFuncs, ret_3, (a_modelNamePrefixStr.clone()).clone(), a_type.clone())?;
            l_threadFuncs = Tpl::popIter(l_threadFuncs)?;
            ret_5 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_6 = listRest(ret_5)?;
            l_threadLocksInit = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadLocksInit = lm_116(l_threadLocksInit, ret_6, a_type.clone())?;
            l_threadLocksInit = Tpl::popIter(l_threadLocksInit)?;
            ret_8 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_9 = listRest(ret_8)?;
            l_threadLocksInit1 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadLocksInit1 = lm_117(l_threadLocksInit1, ret_9, a_type.clone())?;
            l_threadLocksInit1 = Tpl::popIter(l_threadLocksInit1)?;
            ret_11 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_12 = listRest(ret_11)?;
            l_threadAssignLocks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadAssignLocks = lm_118(l_threadAssignLocks, ret_12, a_type.clone())?;
            l_threadAssignLocks = Tpl::popIter(l_threadAssignLocks)?;
            ret_14 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_15 = listRest(ret_14)?;
            l_threadAssignLocks1 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadAssignLocks1 = lm_119(l_threadAssignLocks1, ret_15, a_type.clone())?;
            l_threadAssignLocks1 = Tpl::popIter(l_threadAssignLocks1)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_terminateThreads = false;\n")).clone(), (literal!("_command = IContinuous::UNDEF_UPDATE;\n")).clone(), (literal!("_evaluateMode = -1;\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), a_initLocksOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_initLocksDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_initLocksZeroFunc.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadLocksInit)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadLocksInit1)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_assignLocksDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_assignLocksOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_assignLocksZeroFunc.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadAssignLocks)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadAssignLocks1)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_threadFuncs)?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_121(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_zeroFuncSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_daeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_odeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_modelNamePrefixStr, in_a_zeroFuncSchedule_tasks, in_a_daeSchedule_tasks, in_a_odeSchedule_tasks)) {
        (txt, Deref @ "tbb", a_modelNamePrefixStr, a_zeroFuncSchedule_tasks, a_daeSchedule_tasks, a_odeSchedule_tasks) => {
            let mut l_tbbVars: Tpl::Text;
            let mut txt = (*txt).clone();
            l_tbbVars = generateTbbConstructorExtension(Tpl::emptyTxt.clone(), a_odeSchedule_tasks.clone(), a_daeSchedule_tasks.clone(), a_zeroFuncSchedule_tasks.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_tbbVars)?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_122(mut in_txt: Tpl::Text, mut in_a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut in_a_fullModelName: ArcStr, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_schedulesOpt, in_a_fullModelName, in_a_modelNamePrefixStr, in_a_type)) {
        (txt, Some((Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, tasksOfLevels: i_odeSchedule_tasksOfLevels }, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, tasksOfLevels: i_daeSchedule_tasksOfLevels }, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, tasksOfLevels: i_zeroFuncSchedule_tasksOfLevels })), a_fullModelName, a_modelNamePrefixStr, a_type) => {
            let mut str_0: ArcStr;
            let mut txt = (*txt).clone();
            str_0 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_113(txt.clone(), (str_0).clone(), i_zeroFuncSchedule_tasksOfLevels.clone(), i_daeSchedule_tasksOfLevels.clone(), (a_fullModelName.clone()).clone(), i_odeSchedule_tasksOfLevels.clone(), (a_modelNamePrefixStr.clone()).clone(), a_type.clone())?;
            txt.clone()
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: i_odeSchedule_outgoingDepTasks, threadTasks: i_odeSchedule_threadTasks, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: i_daeSchedule_outgoingDepTasks, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: i_zeroFuncSchedule_outgoingDepTasks, .. })), _, a_modelNamePrefixStr, a_type) => {
            let mut str_13: ArcStr;
            let mut ret_12: i32;
            let mut l_assignLocksZeroFunc: Tpl::Text;
            let mut ret_10: i32;
            let mut l_initLocksZeroFunc: Tpl::Text;
            let mut ret_8: i32;
            let mut l_assignLocksDae: Tpl::Text;
            let mut ret_6: i32;
            let mut l_initLocksDae: Tpl::Text;
            let mut ret_4: i32;
            let mut l_assignLocksOde: Tpl::Text;
            let mut ret_2: i32;
            let mut l_initLocksOde: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_2 = (i_odeSchedule_outgoingDepTasks.clone().len() as i32);
            l_initLocksOde = initializeArrayLocks(Tpl::emptyTxt.clone(), ret_2, (literal!("_lockOde")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_4 = (i_odeSchedule_outgoingDepTasks.clone().len() as i32);
            l_assignLocksOde = assignArrayLocks(Tpl::emptyTxt.clone(), ret_4, (literal!("_lockOde")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_6 = (i_daeSchedule_outgoingDepTasks.clone().len() as i32);
            l_initLocksDae = initializeArrayLocks(Tpl::emptyTxt.clone(), ret_6, (literal!("_lockDae")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_8 = (i_daeSchedule_outgoingDepTasks.clone().len() as i32);
            l_assignLocksDae = assignArrayLocks(Tpl::emptyTxt.clone(), ret_8, (literal!("_lockDae")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_10 = (i_zeroFuncSchedule_outgoingDepTasks.clone().len() as i32);
            l_initLocksZeroFunc = initializeArrayLocks(Tpl::emptyTxt.clone(), ret_10, (literal!("_lockZeroFunc")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_12 = (i_zeroFuncSchedule_outgoingDepTasks.clone().len() as i32);
            l_assignLocksZeroFunc = assignArrayLocks(Tpl::emptyTxt.clone(), ret_12, (literal!("_lockZeroFunc")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            str_13 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_120(txt.clone(), (str_13).clone(), l_assignLocksZeroFunc, l_assignLocksOde, l_assignLocksDae, l_initLocksZeroFunc, l_initLocksDae, l_initLocksOde, (a_modelNamePrefixStr.clone()).clone(), a_type.clone(), i_odeSchedule_threadTasks.clone())?;
            txt.clone()
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_odeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_daeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_zeroFuncSchedule_tasks })), _, a_modelNamePrefixStr, a_type) => {
            let mut str_14: ArcStr;
            let mut txt = (*txt).clone();
            str_14 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_121(txt.clone(), (str_14).clone(), (a_modelNamePrefixStr.clone()).clone(), i_zeroFuncSchedule_tasks.clone(), i_daeSchedule_tasks.clone(), i_odeSchedule_tasks.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn additionalHpcomConstructorBodyStatements(mut txt: Tpl::Text, mut a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut a_modelNamePrefixStr: ArcStr, mut a_fullModelName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_schedulerSpecificReturn: Tpl::Text;
    let mut ret_5: bool;
    let mut ret_4: bool;
    let mut ret_3: ArcStr;
    let mut l_threadMeasureTimeBlocks: Tpl::Text;
    let mut ret_1: ArcStr;
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1).clone())?;
    ret_3 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
    ret_4 = stringEq((ret_3).clone(), (literal!("none")).clone());
    ret_5 = boolNot(ret_4);
    l_threadMeasureTimeBlocks = fun_102(Tpl::emptyTxt.clone(), ret_5, (a_fullModelName.clone()).clone())?;
    l_schedulerSpecificReturn = fun_122(Tpl::emptyTxt.clone(), a_schedulesOpt, (a_fullModelName).clone(), (a_modelNamePrefixStr).clone(), l_type)?;
    out_txt = Tpl::writeText(txt, l_schedulerSpecificReturn)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeText(out_txt, l_threadMeasureTimeBlocks)?;
    Ok(out_txt)
}

fn lm_124(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_124 in &*items {
        let mut lstElt_124 = lstElt_124.clone();
        txt = (match lstElt_124.clone() {
        mut i_threadIdx => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("measuredSchedulerStartValues_")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = MeasureTime::getZeroValues();")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_125(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_125 in &*items {
        let mut lstElt_125 = lstElt_125.clone();
        txt = (match lstElt_125.clone() {
        mut i_threadIdx => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("measuredSchedulerEndValues_")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = MeasureTime::getZeroValues();")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_126(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_126 in &*items {
        let mut lstElt_126 = lstElt_126.clone();
        txt = (match lstElt_126.clone() {
        mut i_threadIdx => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayOdeHpcom)[")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateODE_thread_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_threadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_127(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_127 in &*items {
        let mut lstElt_127 = lstElt_127.clone();
        txt = (match lstElt_127.clone() {
        mut i_threadIdx => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayDaeHpcom)[")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateDAE_thread_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_threadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_128(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_128 in &*items {
        let mut lstElt_128 = lstElt_128.clone();
        txt = (match lstElt_128.clone() {
        mut i_threadIdx => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayZeroFuncHpcom)[")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateZeroFunc_thread_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_threadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

pub(crate) fn generateThreadMeasureTimeDeclaration(mut txt: Tpl::Text, mut a_fullModelName: ArcStr, mut a_numberOfThreads: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_4: Arc<metamodelica::List<i32>>;
    let mut ret_3: Arc<metamodelica::List<i32>>;
    let mut ret_2: Arc<metamodelica::List<i32>>;
    let mut ret_1: Arc<metamodelica::List<i32>>;
    let mut ret_0: Arc<metamodelica::List<i32>>;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#ifdef MEASURETIME_MODELFUNCTIONS\n")).clone(), (literal!("measureTimeThreadArrayOdeHpcom = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeStr(out_txt, (intString(a_numberOfThreads)).clone())?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("measureTimeThreadArrayDaeHpcom = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeStr(out_txt, (intString(a_numberOfThreads)).clone())?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("measureTimeThreadArrayZeroFuncHpcom = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeStr(out_txt, (intString(a_numberOfThreads)).clone())?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeStr(out_txt, (a_fullModelName.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"evaluateODE_threads\",measureTimeThreadArrayOdeHpcom);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeStr(out_txt, (a_fullModelName.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"evaluateDAE_threads\",measureTimeThreadArrayDaeHpcom);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeStr(out_txt, (a_fullModelName).clone())?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\",\"evaluateZeroFunc_threads\",measureTimeThreadArrayZeroFuncHpcom);\n")).clone() }))?;
    ret_0 = List::intRange(a_numberOfThreads);
    out_txt = Tpl::pushIter(out_txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_124(out_txt, ret_0)?;
    out_txt = Tpl::popIter(out_txt)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    ret_1 = List::intRange(a_numberOfThreads);
    out_txt = Tpl::pushIter(out_txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_125(out_txt, ret_1)?;
    out_txt = Tpl::popIter(out_txt)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    ret_2 = List::intRange(a_numberOfThreads);
    out_txt = Tpl::pushIter(out_txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_126(out_txt, ret_2)?;
    out_txt = Tpl::popIter(out_txt)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    ret_3 = List::intRange(a_numberOfThreads);
    out_txt = Tpl::pushIter(out_txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_127(out_txt, ret_3)?;
    out_txt = Tpl::popIter(out_txt)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    ret_4 = List::intRange(a_numberOfThreads);
    out_txt = Tpl::pushIter(out_txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_128(out_txt, ret_4)?;
    out_txt = Tpl::popIter(out_txt)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#endif //MEASURETIME_MODELFUNCTIONS")).clone() }))?;
    Ok(out_txt)
}

fn fun_130(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_numComms: i32, mut in_a_lockName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_numComms, in_a_lockName)) {
        (txt, Deref @ "openmp", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_init_lock(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i]);")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i] = new alignedLock();")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i] = new alignedSpinlock();")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//Unsupported parallel instrumentation")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn initializeArrayLocks(mut txt: Tpl::Text, mut a_numComms: i32, mut a_lockName: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_130(txt, (a_iType).clone(), a_numComms, (a_lockName).clone())?;
    Ok(out_txt)
}

fn fun_132(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_numComms: i32, mut in_a_lockName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_numComms, in_a_lockName)) {
        (txt, Deref @ "openmp", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_set_lock(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i]);")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i]->lock();")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i]->lock();")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//Unsupported parallel instrumentation")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn assignArrayLocks(mut txt: Tpl::Text, mut a_numComms: i32, mut a_lockName: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_132(txt, (a_iType).clone(), a_numComms, (a_lockName).clone())?;
    Ok(out_txt)
}

fn fun_134(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_numComms: i32, mut in_a_lockName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_numComms, in_a_lockName)) {
        (txt, Deref @ "openmp", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_lock_t ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("alignedLock* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("alignedSpinlock* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//Unsupported parallel instrumentation")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn createLockArrayByName(mut txt: Tpl::Text, mut a_numComms: i32, mut a_lockName: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_134(txt, (a_iType).clone(), a_numComms, (a_lockName).clone())?;
    Ok(out_txt)
}

fn fun_136(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_numComms: i32, mut in_a_lockName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_numComms, in_a_lockName)) {
        (txt, Deref @ "openmp", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_destroy_lock(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i]);")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("delete ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i];")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("delete ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i];")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//Unsupported parallel instrumentation")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn destroyArrayLocks(mut txt: Tpl::Text, mut a_numComms: i32, mut a_lockName: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_136(txt, (a_iType).clone(), a_numComms, (a_lockName).clone())?;
    Ok(out_txt)
}

fn fun_138(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg)) {
        (txt, Deref @ "pthreads") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_simulationFinished = true;\n")).clone(), (literal!("//_evaluateBarrier.wait();\n")).clone(), (literal!("_levelBarrier.wait();\n")).clone(), (literal!("//_evaluateBarrier.wait();\n")).clone(), (literal!("_levelBarrier.wait();")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_simulationFinished = true;\n")).clone(), (literal!("//_evaluateBarrier.wait();\n")).clone(), (literal!("_levelBarrier.wait();\n")).clone(), (literal!("//_evaluateBarrier.wait();\n")).clone(), (literal!("_levelBarrier.wait();")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_139(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_139 in &*items {
        let mut lstElt_139 = lstElt_139.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_139.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = function_HPCOM_destroyThread(txt.clone(), (intString(x_i0)).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_140(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_140 in &*items {
        let mut lstElt_140 = lstElt_140.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_140.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = destroyLockByLockName(txt.clone(), (intString(x_i0)).clone(), (literal!("th_lock")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_141(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_141 in &*items {
        let mut lstElt_141 = lstElt_141.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_141.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = destroyLockByLockName(txt.clone(), (intString(x_i0)).clone(), (literal!("th_lock1")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_142(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_142 in &*items {
        let mut lstElt_142 = lstElt_142.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_142.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = function_HPCOM_joinThread(txt.clone(), (intString(x_i0)).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_143(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_143 in &*items {
        let mut lstElt_143 = lstElt_143.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_143.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = releaseLockByLockName(txt.clone(), (intString(x_i0)).clone(), (literal!("th_lock")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_144(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_type: Tpl::Text, mut in_a_odeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_destroyLocksZeroFunc: Tpl::Text, mut in_a_destroyLocksDae: Tpl::Text, mut in_a_destroyLocksOde: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_type, in_a_odeSchedule_threadTasks.clone(), in_a_destroyLocksZeroFunc, in_a_destroyLocksDae, in_a_destroyLocksOde)) {
        (txt, Deref @ "openmp", _, _, a_destroyLocksZeroFunc, a_destroyLocksDae, a_destroyLocksOde) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_destroyLocksOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_destroyLocksDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_destroyLocksZeroFunc.clone())?;
            txt.clone()
        },
        (txt, Deref @ "mpi", _, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//MF: Destruct MPI related stuff - nothing at the moment.")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_type, a_odeSchedule_threadTasks, a_destroyLocksZeroFunc, a_destroyLocksDae, a_destroyLocksOde) => {
            let mut ret_14: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_13: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadReleaseLocks: Tpl::Text;
            let mut ret_11: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_10: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_joinThreads: Tpl::Text;
            let mut ret_8: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_7: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadLocksDel1: Tpl::Text;
            let mut ret_5: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_4: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadLocksDel: Tpl::Text;
            let mut ret_2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_destroyThreads: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_2 = listRest(ret_1)?;
            l_destroyThreads = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_destroyThreads = lm_139(l_destroyThreads, ret_2, a_type.clone())?;
            l_destroyThreads = Tpl::popIter(l_destroyThreads)?;
            ret_4 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_5 = listRest(ret_4)?;
            l_threadLocksDel = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadLocksDel = lm_140(l_threadLocksDel, ret_5, a_type.clone())?;
            l_threadLocksDel = Tpl::popIter(l_threadLocksDel)?;
            ret_7 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_8 = listRest(ret_7)?;
            l_threadLocksDel1 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadLocksDel1 = lm_141(l_threadLocksDel1, ret_8, a_type.clone())?;
            l_threadLocksDel1 = Tpl::popIter(l_threadLocksDel1)?;
            ret_10 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_11 = listRest(ret_10)?;
            l_joinThreads = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_joinThreads = lm_142(l_joinThreads, ret_11, a_type.clone())?;
            l_joinThreads = Tpl::popIter(l_joinThreads)?;
            ret_13 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_14 = listRest(ret_13)?;
            l_threadReleaseLocks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadReleaseLocks = lm_143(l_threadReleaseLocks, ret_14, a_type.clone())?;
            l_threadReleaseLocks = Tpl::popIter(l_threadReleaseLocks)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_terminateThreads = true;\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_threadReleaseLocks)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_joinThreads)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_destroyLocksOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_destroyLocksDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_destroyLocksZeroFunc.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadLocksDel)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadLocksDel1)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_destroyThreads)?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_145(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg)) {
        (txt, Deref @ "tbb") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("for(std::vector<tbb::flow::continue_node<tbb::flow::continue_msg>* >::iterator it = _tbbNodeListOde.begin(); it != _tbbNodeListOde.end(); it++)\n")).clone(), (literal!("  delete *it;\n")).clone(), (literal!("for(std::vector<tbb::flow::continue_node<tbb::flow::continue_msg>* >::iterator it = _tbbNodeListAll.begin(); it != _tbbNodeListAll.end(); it++)\n")).clone(), (literal!("  delete *it;\n")).clone(), (literal!("for(std::vector<tbb::flow::continue_node<tbb::flow::continue_msg>* >::iterator it = _tbbNodeListZeroFunc.begin(); it != _tbbNodeListZeroFunc.end(); it++)\n")).clone(), (literal!("  delete *it;")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_146(mut in_txt: Tpl::Text, mut in_a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_schedulesOpt, in_a_type)) {
        (txt, Some((Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, .. }, _, _)), a_type) => {
            let mut str_0: ArcStr;
            let mut txt = (*txt).clone();
            str_0 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_138(txt.clone(), (str_0).clone())?;
            txt.clone()
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: i_odeSchedule_outgoingDepTasks, threadTasks: i_odeSchedule_threadTasks, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: i_daeSchedule_outgoingDepTasks, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: i_zeroFuncSchedule_outgoingDepTasks, .. })), a_type) => {
            let mut str_7: ArcStr;
            let mut ret_6: i32;
            let mut l_destroyLocksZeroFunc: Tpl::Text;
            let mut ret_4: i32;
            let mut l_destroyLocksDae: Tpl::Text;
            let mut ret_2: i32;
            let mut l_destroyLocksOde: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_2 = (i_odeSchedule_outgoingDepTasks.clone().len() as i32);
            l_destroyLocksOde = destroyArrayLocks(Tpl::emptyTxt.clone(), ret_2, (literal!("_lockOde")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_4 = (i_daeSchedule_outgoingDepTasks.clone().len() as i32);
            l_destroyLocksDae = destroyArrayLocks(Tpl::emptyTxt.clone(), ret_4, (literal!("_lockDae")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_6 = (i_zeroFuncSchedule_outgoingDepTasks.clone().len() as i32);
            l_destroyLocksZeroFunc = destroyArrayLocks(Tpl::emptyTxt.clone(), ret_6, (literal!("_lockZeroFunc")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            str_7 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_144(txt.clone(), (str_7).clone(), a_type.clone(), i_odeSchedule_threadTasks.clone(), l_destroyLocksZeroFunc, l_destroyLocksDae, l_destroyLocksOde)?;
            txt.clone()
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: _ }, _, _)), a_type) => {
            let mut str_8: ArcStr;
            let mut txt = (*txt).clone();
            str_8 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_145(txt.clone(), (str_8).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_147(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_147 in &*items {
        let mut lstElt_147 = lstElt_147.clone();
        txt = (match lstElt_147.clone() {
        mut i_threadIdx => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("delete measuredSchedulerStartValues_")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_148(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_148 in &*items {
        let mut lstElt_148 = lstElt_148.clone();
        txt = (match lstElt_148.clone() {
        mut i_threadIdx => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("delete measuredSchedulerEndValues_")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

pub(crate) fn additionalHpcomDestructorBodyStatements(mut txt: Tpl::Text, mut a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_6: Arc<metamodelica::List<i32>>;
    let mut ret_5: i32;
    let mut ret_4: Arc<metamodelica::List<i32>>;
    let mut ret_3: i32;
    let mut l_schedulerSpecificCode: Tpl::Text;
    let mut ret_1: ArcStr;
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1).clone())?;
    l_schedulerSpecificCode = fun_146(Tpl::emptyTxt.clone(), a_schedulesOpt, l_type)?;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#ifdef MEASURETIME_MODELFUNCTIONS\n")).clone() }))?;
    ret_3 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
    ret_4 = List::intRange(ret_3);
    out_txt = Tpl::pushIter(out_txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_147(out_txt, ret_4)?;
    out_txt = Tpl::popIter(out_txt)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    ret_5 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
    ret_6 = List::intRange(ret_5);
    out_txt = Tpl::pushIter(out_txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_148(out_txt, ret_6)?;
    out_txt = Tpl::popIter(out_txt)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#endif //MEASURETIME_MODELFUNCTIONS\n")).clone() }))?;
    out_txt = Tpl::writeText(out_txt, l_schedulerSpecificCode)?;
    Ok(out_txt)
}

fn fun_150(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_useFlatArrayNotation: bool, mut in_a_stateDerVectorName: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_stateDerVectorName, out_a_extraFuncsNamespace, out_a_extraFuncsDecl) = (::match_deref::match_deref! { match &((in_txt, in_a_simCode, in_a_extraFuncs, in_a_useFlatArrayNotation, in_a_stateDerVectorName, in_a_context, in_a_extraFuncsNamespace, in_a_extraFuncsDecl, in_a_allEquationsPlusWhen)) {
        (txt, i_simCode @ SimCode::SimCode { modelInfo: SimCode::ModelInfo { name: i_modelInfo_name, .. }, hpcomData: HpcOmSimCode::HpcOmData { schedules: i_hpcomData_schedules, .. }, allEquations: i_allEquations, clockedPartitions: i_clockedPartitions, .. }, a_extraFuncs, a_useFlatArrayNotation, a_stateDerVectorName, a_context, a_extraFuncsNamespace, a_extraFuncsDecl, a_allEquationsPlusWhen) => {
            let mut ret_6: bool;
            let mut ret_5: bool;
            let mut ret_4: ArcStr;
            let mut ret_3: Arc<metamodelica::List<SimCode::SubPartition>>;
            let mut txt_2: Tpl::Text;
            let mut l_parCode: Tpl::Text;
            let mut l_extraFuncsPar: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            l_extraFuncsPar = Tpl::emptyTxt.clone();
            txt_2 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            (l_parCode, l_extraFuncsPar, a_extraFuncsDecl, a_extraFuncsNamespace, a_stateDerVectorName) = generateParallelEvaluate(Tpl::emptyTxt.clone(), a_allEquationsPlusWhen.clone(), i_modelInfo_name.clone(), i_simCode.clone(), l_extraFuncsPar, a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), i_hpcomData_schedules.clone(), a_context.clone(), a_stateDerVectorName.clone(), (Tpl::textString(txt_2)?).clone(), a_useFlatArrayNotation.clone())?;
            (txt, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_stateDerVectorName) = CodegenCpp::equationFunctions(txt.clone(), i_allEquations.clone(), i_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_stateDerVectorName.clone(), a_useFlatArrayNotation.clone(), false)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            (txt, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_stateDerVectorName) = CodegenCpp::createEvaluateConditions(txt.clone(), i_allEquations.clone(), i_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), SimCodeFunction::contextOther().clone(), a_stateDerVectorName.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            ret_3 = SimCodeUtil::getSubPartitions(i_clockedPartitions.clone())?;
            ret_4 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_5 = stringEq((ret_4).clone(), (literal!("none")).clone());
            ret_6 = boolNot(ret_5);
            (txt, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_stateDerVectorName) = CodegenCpp::clockedFunctions(txt.clone(), ret_3, i_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_stateDerVectorName.clone(), a_useFlatArrayNotation.clone(), ret_6)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_parCode)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), l_extraFuncsPar)?;
            (txt.clone(), a_extraFuncs.clone(), a_stateDerVectorName.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone())
        },
        (txt, _, a_extraFuncs, _, a_stateDerVectorName, _, a_extraFuncsNamespace, a_extraFuncsDecl, _) => {
            (txt.clone(), a_extraFuncs.clone(), a_stateDerVectorName.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncs, out_a_stateDerVectorName, out_a_extraFuncsNamespace, out_a_extraFuncsDecl))
}

pub(crate) fn updateHpcom(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_context: SimCodeFunction::Context, mut a_stateDerVectorName: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    l_varDecls = Tpl::emptyTxt.clone();
    (out_txt, out_a_extraFuncs, out_a_stateDerVectorName, out_a_extraFuncsNamespace, out_a_extraFuncsDecl) = fun_150(txt, a_simCode, a_extraFuncs, a_useFlatArrayNotation, a_stateDerVectorName, a_context, a_extraFuncsNamespace, a_extraFuncsDecl, a_allEquationsPlusWhen)?;
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace, out_a_stateDerVectorName))
}

fn fun_152(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = CodegenCpp::generateMeasureTimeStartCode(txt.clone(), (literal!("measuredFunctionStartValues")).clone(), (literal!("evaluateODE")).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_153(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("measuredFunctionStartValues")).clone(), (literal!("measuredFunctionEndValues")).clone(), (literal!("(*measureTimeFunctionsArray)[0]")).clone(), (literal!("evaluateODE")).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_154(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = CodegenCpp::generateMeasureTimeStartCode(txt.clone(), (literal!("measuredFunctionStartValues")).clone(), (literal!("evaluateAll")).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_155(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("measuredFunctionStartValues")).clone(), (literal!("measuredFunctionEndValues")).clone(), (literal!("(*measureTimeFunctionsArray)[1]")).clone(), (literal!("evaluateAll")).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_156(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = CodegenCpp::generateMeasureTimeStartCode(txt.clone(), (literal!("measuredFunctionStartValues")).clone(), (literal!("evaluateZeroFuncs")).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_157(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("measuredFunctionStartValues")).clone(), (literal!("measuredFunctionEndValues")).clone(), (literal!("(*measureTimeFunctionsArray)[4]")).clone(), (literal!("evaluateZeroFuncs")).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_158(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut a_useFlatArrayNotation: bool, mut a_name: Arc<Absyn::Path>, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_type: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_158 in &*items {
        let mut lstElt_158 = lstElt_158.clone();
        (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (match lstElt_158.clone() {
        mut i_tasks => {
            let mut txt_0: Tpl::Text;
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelCodeForLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
    });
    }
    Ok((txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_159(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut a_useFlatArrayNotation: bool, mut a_name: Arc<Absyn::Path>, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_type: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_159 in &*items {
        let mut lstElt_159 = lstElt_159.clone();
        (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (match lstElt_159.clone() {
        mut i_tasks => {
            let mut txt_0: Tpl::Text;
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelCodeForLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
    });
    }
    Ok((txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_160(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut a_useFlatArrayNotation: bool, mut a_name: Arc<Absyn::Path>, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_type: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_160 in &*items {
        let mut lstElt_160 = lstElt_160.clone();
        (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (match lstElt_160.clone() {
        mut i_tasks => {
            let mut txt_0: Tpl::Text;
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelCodeForLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
    });
    }
    Ok((txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn fun_161(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_type: Tpl::Text, mut in_a_functionHead: Tpl::Text, mut in_a_zeroFuncEqs: Tpl::Text, mut in_a_daeEqs: Tpl::Text, mut in_a_odeEqs: Tpl::Text, mut in_a_name: Arc<Absyn::Path>, mut in_a_extraFuncsDecl: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl) = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_type, in_a_functionHead, in_a_zeroFuncEqs, in_a_daeEqs, in_a_odeEqs, in_a_name, in_a_extraFuncsDecl)) {
        (txt, Deref @ "openmp", _, a_functionHead, a_zeroFuncEqs, a_daeEqs, a_odeEqs, a_name, a_extraFuncsDecl) => {
            let mut ret_2: i32;
            let mut ret_1: i32;
            let mut ret_0: i32;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("void evaluateODE_Parallel();\n")).clone(), (literal!("void evaluateAll_Parallel();\n")).clone(), (literal!("void evaluateZeroFuncs_Parallel();")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateODE_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_0 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_odeEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateAll_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_1 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_daeEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateZeroFuncs_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_2 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_2)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_zeroFuncEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_evaluateMode = _evaluateMode;\n")).clone(), (literal!("  this->_command = command;\n")).clone(), (literal!("  if(evaluateMode == 0)\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateODE_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else if(evaluateMode < 0)\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateAll_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateZeroFuncs_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_extraFuncsDecl.clone())
        },
        (txt, _, a_type, a_functionHead, _, _, _, _, a_extraFuncsDecl) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("{\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("throw std::runtime_error(\"Type ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_type.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" is unsupported for level scheduling.\");\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_extraFuncsDecl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncsDecl))
}

fn lm_162(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_name: Arc<Absyn::Path>, mut a_varDecls: Tpl::Text, mut a_type: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_162 in &*items {
        let mut lstElt_162 = lstElt_162.clone();
        (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (match lstElt_162.clone() {
        mut i_tasks => {
            let mut txt_0: Tpl::Text;
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelFixedCodeForLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_name.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
    });
    }
    Ok((txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_163(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_name: Arc<Absyn::Path>, mut a_varDecls: Tpl::Text, mut a_type: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_163 in &*items {
        let mut lstElt_163 = lstElt_163.clone();
        (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (match lstElt_163.clone() {
        mut i_tasks => {
            let mut txt_0: Tpl::Text;
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelFixedCodeForLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_name.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
    });
    }
    Ok((txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_164(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_name: Arc<Absyn::Path>, mut a_varDecls: Tpl::Text, mut a_type: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_164 in &*items {
        let mut lstElt_164 = lstElt_164.clone();
        (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (match lstElt_164.clone() {
        mut i_tasks => {
            let mut txt_0: Tpl::Text;
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelFixedCodeForLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_name.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
    });
    }
    Ok((txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_165(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_name: Arc<Absyn::Path>, mut a_varDecls: Tpl::Text, mut a_type: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_165 in &*items {
        let mut lstElt_165 = lstElt_165.clone();
        (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_165.clone()) {
        i_tasks => {
            let mut x_i0: i32;
            let mut txt_0: Tpl::Text;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelFixedCodeForThread(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), x_i0, (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_name.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_166(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_166 in &*items {
        let mut lstElt_166 = lstElt_166.clone();
        txt = (match lstElt_166.clone() {
        mut i_tt => {
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threadLock")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(i_tt.clone())).clone())?;
            txt = createLockByLockName(txt.clone(), (Tpl::textString(txt_0.clone())?).clone(), (literal!("")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_167(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_name: Arc<Absyn::Path>, mut a_varDecls: Tpl::Text, mut a_type: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_167 in &*items {
        let mut lstElt_167 = lstElt_167.clone();
        (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_167.clone()) {
        i_tasks => {
            let mut x_i0: i32;
            let mut txt_0: Tpl::Text;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelFixedCodeForThread(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), x_i0, (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_name.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_168(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_168 in &*items {
        let mut lstElt_168 = lstElt_168.clone();
        txt = (match lstElt_168.clone() {
        mut i_tt => {
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threadLock")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(i_tt.clone())).clone())?;
            txt = createLockByLockName(txt.clone(), (Tpl::textString(txt_0.clone())?).clone(), (literal!("")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_169(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_functionHead: Tpl::Text, mut in_a_zeroFuncSchedule: Arc<HpcOmSimCode::Schedule>, mut in_a_daeSchedule: Arc<HpcOmSimCode::Schedule>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_name: Arc<Absyn::Path>, mut in_a_varDecls: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_odeSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_extraFuncsNamespace, in_a_functionHead, in_a_zeroFuncSchedule, in_a_daeSchedule, in_a_useFlatArrayNotation, in_a_extraFuncsDecl, in_a_extraFuncs, in_a_simCode, in_a_name, in_a_varDecls, in_a_type, in_a_allEquationsPlusWhen, in_a_odeSchedule)) {
        (txt, Deref @ "openmp", a_extraFuncsNamespace, a_functionHead, a_zeroFuncSchedule, a_daeSchedule, a_useFlatArrayNotation, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_name, a_varDecls, a_type, a_allEquationsPlusWhen, a_odeSchedule) => {
            let mut ret_11: i32;
            let mut ret_10: i32;
            let mut ret_9: i32;
            let mut ret_8: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>>;
            let mut ret_7: i32;
            let mut l_zeroFuncEqs: Tpl::Text;
            let mut ret_5: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>>;
            let mut ret_4: i32;
            let mut l_daeEqs: Tpl::Text;
            let mut ret_2: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>>;
            let mut ret_1: i32;
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_1 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_2 = HpcOmScheduler::convertFixedLevelScheduleToLevelThreadLists(a_odeSchedule.clone(), ret_1)?;
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_odeEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_162(l_odeEqs, ret_2, a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs)?;
            ret_4 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_5 = HpcOmScheduler::convertFixedLevelScheduleToLevelThreadLists(a_daeSchedule.clone(), ret_4)?;
            l_daeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_daeEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_163(l_daeEqs, ret_5, a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_daeEqs = Tpl::popIter(l_daeEqs)?;
            ret_7 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_8 = HpcOmScheduler::convertFixedLevelScheduleToLevelThreadLists(a_zeroFuncSchedule.clone(), ret_7)?;
            l_zeroFuncEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_zeroFuncEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_164(l_zeroFuncEqs, ret_8, a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_zeroFuncEqs = Tpl::popIter(l_zeroFuncEqs)?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("void evaluateODE_Parallel();\n")).clone(), (literal!("void evaluateAll_Parallel();\n")).clone(), (literal!("void evaluateZeroFuncs_Parallel();")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateODE_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_9 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_9)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone(), (literal!("  int threadNum = getThreadNumber();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateAll_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_10 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_10)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone(), (literal!("  int threadNum = getThreadNumber();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_daeEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateZeroFuncs_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_11 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_11)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone(), (literal!("  int threadNum = getThreadNumber();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_zeroFuncEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_evaluateMode = _evaluateMode;\n")).clone(), (literal!("  this->_command = command;\n")).clone(), (literal!("  if(evaluateMode == 0)\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateODE_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else if(evaluateMode < 0)\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateAll_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateZeroFuncs_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ "pthreads", a_extraFuncsNamespace, a_functionHead, a_zeroFuncSchedule, a_daeSchedule, a_useFlatArrayNotation, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_name, a_varDecls, a_type, a_allEquationsPlusWhen, a_odeSchedule) => {
            let mut ret_18: Arc<metamodelica::List<i32>>;
            let mut ret_17: i32;
            let mut l_threadLocks: Tpl::Text;
            let mut ret_15: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>>;
            let mut ret_14: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>;
            let mut ret_13: i32;
            let mut l_eqsFuncs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_13 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_14 = HpcOmScheduler::convertFixedLevelScheduleToTaskLists(a_odeSchedule.clone(), a_daeSchedule.clone(), a_zeroFuncSchedule.clone(), ret_13)?;
            ret_15 = Arc::new(ret_14.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_eqsFuncs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_eqsFuncs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_165(l_eqsFuncs, ret_15, a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_eqsFuncs = Tpl::popIter(l_eqsFuncs)?;
            ret_17 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_18 = List::intRange(ret_17);
            l_threadLocks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadLocks = lm_166(l_threadLocks, ret_18, a_type.clone())?;
            l_threadLocks = Tpl::popIter(l_threadLocks)?;
            txt = Tpl::writeText(txt.clone(), l_eqsFuncs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_command = command;\n")).clone(), (literal!("  this->_evaluateMode = evaluateMode;\n")).clone(), (literal!("\n")).clone(), (literal!("  if(evaluateMode == 0) //evaluate ODE\n")).clone(), (literal!("  {\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    evaluateThreadFuncODE_0();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            (txt, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateStateVarPrefetchCode(txt.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  else if(evaluateMode < 0) //evaluate All\n")).clone(), (literal!("  {\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    evaluateThreadFuncAll_0();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else //evaluate ZeroFuncs\n")).clone(), (literal!("  {\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    evaluateThreadFuncZeroFunc_0();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ "pthreads_spin", a_extraFuncsNamespace, a_functionHead, a_zeroFuncSchedule, a_daeSchedule, a_useFlatArrayNotation, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_name, a_varDecls, a_type, a_allEquationsPlusWhen, a_odeSchedule) => {
            let mut ret_23: Arc<metamodelica::List<i32>>;
            let mut ret_22: i32;
            let mut ret_21: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>>;
            let mut ret_20: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>;
            let mut ret_19: i32;
            let mut l_threadLocks: Tpl::Text;
            let mut l_eqsFuncs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_19 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_20 = HpcOmScheduler::convertFixedLevelScheduleToTaskLists(a_odeSchedule.clone(), a_daeSchedule.clone(), a_zeroFuncSchedule.clone(), ret_19)?;
            ret_21 = Arc::new(ret_20.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_eqsFuncs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_eqsFuncs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_167(l_eqsFuncs, ret_21, a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_eqsFuncs = Tpl::popIter(l_eqsFuncs)?;
            ret_22 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_23 = List::intRange(ret_22);
            l_threadLocks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadLocks = lm_168(l_threadLocks, ret_23, a_type.clone())?;
            l_threadLocks = Tpl::popIter(l_threadLocks)?;
            txt = Tpl::writeText(txt.clone(), l_eqsFuncs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_command = command;\n")).clone(), (literal!("  this->_evaluateMode = evaluateMode;\n")).clone(), (literal!("\n")).clone(), (literal!("  if(evaluateMode == 0) //evaluate ODE\n")).clone(), (literal!("  {\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    evaluateThreadFuncODE_0();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            (txt, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateStateVarPrefetchCode(txt.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  else if(evaluateMode < 0) //evaluate All\n")).clone(), (literal!("  {\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    evaluateThreadFuncAll_0();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else //evaluate ZeroFuncs\n")).clone(), (literal!("  {\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    evaluateThreadFuncZeroFunc_0();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, _, a_extraFuncsNamespace, a_functionHead, _, _, _, a_extraFuncsDecl, a_extraFuncs, _, _, a_varDecls, a_type, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("{\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("throw std::runtime_error(\"Type ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_type.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" is unsupported for levelfix scheduling.\");\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_170(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text, mut a_threadTasksOde: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_170 in &*items {
        let mut lstElt_170 = lstElt_170.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_170.clone()) {
        _ => {
            let mut x_i0: i32;
            let mut ret_1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut ret_0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = intAdd(x_i0, 1);
            ret_1 = metamodelica::arrayGet(a_threadTasksOde.clone(), ret_0)?;
            txt = function_HPCOM_assignThreadLocks(txt.clone(), ret_1.clone(), (literal!("_lockOde")).clone(), x_i0, (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_171(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text, mut a_threadTasksOde: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_171 in &*items {
        let mut lstElt_171 = lstElt_171.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_171.clone()) {
        _ => {
            let mut x_i0: i32;
            let mut ret_1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut ret_0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = intAdd(x_i0, 1);
            ret_1 = metamodelica::arrayGet(a_threadTasksOde.clone(), ret_0)?;
            txt = function_HPCOM_releaseThreadLocks(txt.clone(), ret_1.clone(), (literal!("_lockOde")).clone(), x_i0, (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_172(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text, mut a_threadTasksDae: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_172 in &*items {
        let mut lstElt_172 = lstElt_172.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_172.clone()) {
        _ => {
            let mut x_i0: i32;
            let mut ret_1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut ret_0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = intAdd(x_i0, 1);
            ret_1 = metamodelica::arrayGet(a_threadTasksDae.clone(), ret_0)?;
            txt = function_HPCOM_assignThreadLocks(txt.clone(), ret_1.clone(), (literal!("_lockDae")).clone(), x_i0, (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_173(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text, mut a_threadTasksDae: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_173 in &*items {
        let mut lstElt_173 = lstElt_173.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_173.clone()) {
        _ => {
            let mut x_i0: i32;
            let mut ret_1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut ret_0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = intAdd(x_i0, 1);
            ret_1 = metamodelica::arrayGet(a_threadTasksDae.clone(), ret_0)?;
            txt = function_HPCOM_releaseThreadLocks(txt.clone(), ret_1.clone(), (literal!("_lockDae")).clone(), x_i0, (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_174(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text, mut a_threadTasksZeroFunc: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_174 in &*items {
        let mut lstElt_174 = lstElt_174.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_174.clone()) {
        _ => {
            let mut x_i0: i32;
            let mut ret_1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut ret_0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = intAdd(x_i0, 1);
            ret_1 = metamodelica::arrayGet(a_threadTasksZeroFunc.clone(), ret_0)?;
            txt = function_HPCOM_assignThreadLocks(txt.clone(), ret_1.clone(), (literal!("_lockZeroFunc")).clone(), x_i0, (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_175(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text, mut a_threadTasksZeroFunc: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_175 in &*items {
        let mut lstElt_175 = lstElt_175.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_175.clone()) {
        _ => {
            let mut x_i0: i32;
            let mut ret_1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut ret_0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = intAdd(x_i0, 1);
            ret_1 = metamodelica::arrayGet(a_threadTasksZeroFunc.clone(), ret_0)?;
            txt = function_HPCOM_releaseThreadLocks(txt.clone(), ret_1.clone(), (literal!("_lockZeroFunc")).clone(), x_i0, (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_176(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_useFlatArrayNotation: bool, mut a_name: Arc<Absyn::Path>, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_type: Tpl::Text, mut a_threadTasksOde: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_176 in &*items {
        let mut lstElt_176 = lstElt_176.clone();
        (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_176.clone()) {
        i_tt => {
            let mut x_i0: i32;
            let mut txt_2: Tpl::Text;
            let mut ret_1: i32;
            let mut ret_0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = metamodelica::arrayLength(a_threadTasksOde.clone());
            ret_1 = intSub(ret_0, 1);
            txt_2 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_2) = parallelThreadCodeWithSplit(txt.clone(), a_allEquationsPlusWhen.clone(), i_tt.clone(), x_i0, ret_1, (Tpl::textString(a_type.clone())?).clone(), (literal!("_lockOde")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_2.clone(), (literal!("evaluateODE")).clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_177(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_useFlatArrayNotation: bool, mut a_name: Arc<Absyn::Path>, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_type: Tpl::Text, mut a_threadTasksDae: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_177 in &*items {
        let mut lstElt_177 = lstElt_177.clone();
        (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_177.clone()) {
        i_tt => {
            let mut x_i0: i32;
            let mut txt_2: Tpl::Text;
            let mut ret_1: i32;
            let mut ret_0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = metamodelica::arrayLength(a_threadTasksDae.clone());
            ret_1 = intSub(ret_0, 1);
            txt_2 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_2) = parallelThreadCodeWithSplit(txt.clone(), a_allEquationsPlusWhen.clone(), i_tt.clone(), x_i0, ret_1, (Tpl::textString(a_type.clone())?).clone(), (literal!("_lockDae")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_2.clone(), (literal!("evaluateAll")).clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_178(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_useFlatArrayNotation: bool, mut a_name: Arc<Absyn::Path>, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_type: Tpl::Text, mut a_threadTasksZeroFunc: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_178 in &*items {
        let mut lstElt_178 = lstElt_178.clone();
        (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_178.clone()) {
        i_tt => {
            let mut x_i0: i32;
            let mut txt_2: Tpl::Text;
            let mut ret_1: i32;
            let mut ret_0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = metamodelica::arrayLength(a_threadTasksZeroFunc.clone());
            ret_1 = intSub(ret_0, 1);
            txt_2 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_2) = parallelThreadCodeWithSplit(txt.clone(), a_allEquationsPlusWhen.clone(), i_tt.clone(), x_i0, ret_1, (Tpl::textString(a_type.clone())?).clone(), (literal!("_lockZeroFunc")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_2.clone(), (literal!("evaluateZeroFunc")).clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_179(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>, mut a_useFlatArrayNotation: bool, mut a_mainThreadCode: Tpl::Text, mut a_name: Arc<Absyn::Path>, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_modelNamePrefixStr: ArcStr, mut a_type: Tpl::Text, mut a_zeroFuncSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut a_daeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut a_odeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_mainThreadCode: Tpl::Text = a_mainThreadCode;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_179 in &*items {
        let mut lstElt_179 = lstElt_179.clone();
        (txt, a_mainThreadCode, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (match lstElt_179.clone() {
        mut i_threadIdx => {
            let mut txt_4: Tpl::Text;
            let mut ret_3: i32;
            let mut ret_2: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut ret_1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            let mut ret_0: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>;
            ret_0 = metamodelica::arrayGet(a_odeSchedule_threadTasks.clone(), i_threadIdx.clone())?;
            ret_1 = metamodelica::arrayGet(a_daeSchedule_threadTasks.clone(), i_threadIdx.clone())?;
            ret_2 = metamodelica::arrayGet(a_zeroFuncSchedule_threadTasks.clone(), i_threadIdx.clone())?;
            ret_3 = intSub(i_threadIdx.clone(), 1);
            txt_4 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_4, a_mainThreadCode) = generateThreadFunc(txt.clone(), a_allEquationsPlusWhen.clone(), ret_0.clone(), ret_1.clone(), ret_2.clone(), (Tpl::textString(a_type.clone())?).clone(), ret_3, (a_modelNamePrefixStr.clone()).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_4.clone(), a_mainThreadCode.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_mainThreadCode.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
    });
    }
    Ok((txt, a_mainThreadCode, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_180(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_180 in &*items {
        let mut lstElt_180 = lstElt_180.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_180.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = assignLockByLockName(txt.clone(), (intString(x_i0)).clone(), (literal!("th_lock1")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_181(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_181 in &*items {
        let mut lstElt_181 = lstElt_181.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_181.clone()) {
        _ => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = releaseLockByLockName(txt.clone(), (intString(x_i0)).clone(), (literal!("th_lock")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_182(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_zeroFuncSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_daeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_odeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_functionHead: Tpl::Text, mut in_a_useFlatArrayNotation: bool, mut in_a_name: Arc<Absyn::Path>, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_threadTasksZeroFunc: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_threadTasksDae: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_type: Tpl::Text, mut in_a_threadTasksOde: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_modelNamePrefixStr, in_a_zeroFuncSchedule_threadTasks.clone(), in_a_daeSchedule_threadTasks.clone(), in_a_odeSchedule_threadTasks.clone(), in_a_functionHead, in_a_useFlatArrayNotation, in_a_name, in_a_extraFuncsDecl, in_a_extraFuncs, in_a_simCode, in_a_varDecls, in_a_allEquationsPlusWhen, in_a_threadTasksZeroFunc.clone(), in_a_threadTasksDae.clone(), in_a_type, in_a_threadTasksOde.clone())) {
        (txt, Deref @ "openmp", _, _, _, _, a_functionHead, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen, a_threadTasksZeroFunc, a_threadTasksDae, a_type, a_threadTasksOde) => {
            let mut ret_20: i32;
            let mut ret_19: i32;
            let mut ret_18: i32;
            let mut ret_17: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_zeroFuncEqs: Tpl::Text;
            let mut ret_15: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_daeEqs: Tpl::Text;
            let mut ret_13: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_odeEqs: Tpl::Text;
            let mut ret_11: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadReleaseLocksZeroFunc: Tpl::Text;
            let mut ret_9: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadAssignLocksZeroFunc: Tpl::Text;
            let mut ret_7: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadReleaseLocksDae: Tpl::Text;
            let mut ret_5: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadAssignLocksDae: Tpl::Text;
            let mut ret_3: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadReleaseLocksOde: Tpl::Text;
            let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadAssignLocksOde: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_1 = Arc::new(a_threadTasksOde.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadAssignLocksOde = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadAssignLocksOde = lm_170(l_threadAssignLocksOde, ret_1, a_type.clone(), a_threadTasksOde.clone())?;
            l_threadAssignLocksOde = Tpl::popIter(l_threadAssignLocksOde)?;
            ret_3 = Arc::new(a_threadTasksOde.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadReleaseLocksOde = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadReleaseLocksOde = lm_171(l_threadReleaseLocksOde, ret_3, a_type.clone(), a_threadTasksOde.clone())?;
            l_threadReleaseLocksOde = Tpl::popIter(l_threadReleaseLocksOde)?;
            ret_5 = Arc::new(a_threadTasksOde.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadAssignLocksDae = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadAssignLocksDae = lm_172(l_threadAssignLocksDae, ret_5, a_type.clone(), a_threadTasksDae.clone())?;
            l_threadAssignLocksDae = Tpl::popIter(l_threadAssignLocksDae)?;
            ret_7 = Arc::new(a_threadTasksOde.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadReleaseLocksDae = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadReleaseLocksDae = lm_173(l_threadReleaseLocksDae, ret_7, a_type.clone(), a_threadTasksDae.clone())?;
            l_threadReleaseLocksDae = Tpl::popIter(l_threadReleaseLocksDae)?;
            ret_9 = Arc::new(a_threadTasksZeroFunc.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadAssignLocksZeroFunc = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadAssignLocksZeroFunc = lm_174(l_threadAssignLocksZeroFunc, ret_9, a_type.clone(), a_threadTasksZeroFunc.clone())?;
            l_threadAssignLocksZeroFunc = Tpl::popIter(l_threadAssignLocksZeroFunc)?;
            ret_11 = Arc::new(a_threadTasksZeroFunc.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadReleaseLocksZeroFunc = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadReleaseLocksZeroFunc = lm_175(l_threadReleaseLocksZeroFunc, ret_11, a_type.clone(), a_threadTasksZeroFunc.clone())?;
            l_threadReleaseLocksZeroFunc = Tpl::popIter(l_threadReleaseLocksZeroFunc)?;
            ret_13 = Arc::new(a_threadTasksOde.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_odeEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_176(l_odeEqs, ret_13, a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_threadTasksOde.clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs)?;
            ret_15 = Arc::new(a_threadTasksDae.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_daeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_daeEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_177(l_daeEqs, ret_15, a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_threadTasksDae.clone(), a_allEquationsPlusWhen.clone())?;
            l_daeEqs = Tpl::popIter(l_daeEqs)?;
            ret_17 = Arc::new(a_threadTasksZeroFunc.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_zeroFuncEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_zeroFuncEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_178(l_zeroFuncEqs, ret_17, a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_threadTasksZeroFunc.clone(), a_allEquationsPlusWhen.clone())?;
            l_zeroFuncEqs = Tpl::popIter(l_zeroFuncEqs)?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("void evaluateODE_Parallel();\n")).clone(), (literal!("void evaluateAll_Parallel();\n")).clone(), (literal!("void evaluateZeroFuncs_Parallel();")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateODE_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_18 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_18)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone(), (literal!("  int threadNum = getThreadNumber();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_threadAssignLocksOde)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#pragma omp barrier\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#pragma omp barrier\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_threadReleaseLocksOde)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateAll_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_19 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_19)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone(), (literal!("  int threadNum = getThreadNumber();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_threadAssignLocksDae)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#pragma omp barrier\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_daeEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#pragma omp barrier\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_threadReleaseLocksDae)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateZeroFuncs_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_20 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_20)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone(), (literal!("  int threadNum = getThreadNumber();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_threadAssignLocksZeroFunc)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#pragma omp barrier\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_zeroFuncEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#pragma omp barrier\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_threadReleaseLocksZeroFunc)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_evaluateMode = _evaluateMode;\n")).clone(), (literal!("  this->_command = command;\n")).clone(), (literal!("  if(evaluateMode == 0)\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateODE_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else if(evaluateMode < 0)\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateAll_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateZeroFuncs_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ "mpi", _, _, _, _, a_functionHead, _, _, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  // MFlehmig: Todo\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, _, a_modelNamePrefixStr, a_zeroFuncSchedule_threadTasks, a_daeSchedule_threadTasks, a_odeSchedule_threadTasks, a_functionHead, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen, _, _, a_type, _) => {
            let mut ret_30: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_29: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadReleaseLocks: Tpl::Text;
            let mut ret_27: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut ret_26: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
            let mut l_threadAssignLocks1: Tpl::Text;
            let mut ret_24: Arc<metamodelica::List<i32>>;
            let mut ret_23: i32;
            let mut l_threadFuncs: Tpl::Text;
            let mut l_mainThreadCode: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_mainThreadCode = Tpl::emptyTxt.clone();
            ret_23 = metamodelica::arrayLength(a_odeSchedule_threadTasks.clone());
            ret_24 = List::intRange(ret_23);
            l_threadFuncs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_threadFuncs, l_mainThreadCode, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_179(l_threadFuncs, ret_24, a_useFlatArrayNotation.clone(), l_mainThreadCode, a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_modelNamePrefixStr.clone()).clone(), a_type.clone(), a_zeroFuncSchedule_threadTasks.clone(), a_daeSchedule_threadTasks.clone(), a_odeSchedule_threadTasks.clone(), a_allEquationsPlusWhen.clone())?;
            l_threadFuncs = Tpl::popIter(l_threadFuncs)?;
            ret_26 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_27 = listRest(ret_26)?;
            l_threadAssignLocks1 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadAssignLocks1 = lm_180(l_threadAssignLocks1, ret_27, a_type.clone())?;
            l_threadAssignLocks1 = Tpl::popIter(l_threadAssignLocks1)?;
            ret_29 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_30 = listRest(ret_29)?;
            l_threadReleaseLocks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_threadReleaseLocks = lm_181(l_threadReleaseLocks, ret_30, a_type.clone())?;
            l_threadReleaseLocks = Tpl::popIter(l_threadReleaseLocks)?;
            txt = Tpl::writeText(txt.clone(), l_threadFuncs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_evaluateMode = _evaluateMode;\n")).clone(), (literal!("  this->_command = command;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_threadReleaseLocks)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_mainThreadCode)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadAssignLocks1)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn fun_183(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_name: Arc<Absyn::Path>, mut in_a_functionHead: Tpl::Text, mut in_a_zeroFuncSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_daeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_odeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_name, in_a_functionHead, in_a_zeroFuncSchedule_tasks, in_a_daeSchedule_tasks, in_a_useFlatArrayNotation, in_a_extraFuncsNamespace, in_a_extraFuncsDecl, in_a_extraFuncs, in_a_simCode, in_a_varDecls, in_a_type, in_a_allEquationsPlusWhen, in_a_odeSchedule_tasks)) {
        (txt, Deref @ "openmp", _, a_functionHead, a_zeroFuncSchedule_tasks, a_daeSchedule_tasks, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_type, a_allEquationsPlusWhen, a_odeSchedule_tasks) => {
            let mut l_zeroFuncTaskEqs: Tpl::Text;
            let mut l_daeTaskEqs: Tpl::Text;
            let mut l_odeTaskEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_odeTaskEqs, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep(Tpl::emptyTxt.clone(), a_odeSchedule_tasks.clone(), a_allEquationsPlusWhen.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            (l_daeTaskEqs, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep(Tpl::emptyTxt.clone(), a_daeSchedule_tasks.clone(), a_allEquationsPlusWhen.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            (l_zeroFuncTaskEqs, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep(Tpl::emptyTxt.clone(), a_zeroFuncSchedule_tasks.clone(), a_allEquationsPlusWhen.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_evaluateMode = _evaluateMode;\n")).clone(), (literal!("  this->_command = command;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_varDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("if(_evaluateMode == 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_odeTaskEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else if(_evaluateMode < 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_daeTaskEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_zeroFuncTaskEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ "tbb", a_name, a_functionHead, a_zeroFuncSchedule_tasks, a_daeSchedule_tasks, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_type, a_allEquationsPlusWhen, a_odeSchedule_tasks) => {
            let mut l_taskFuncs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_taskFuncs, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep_voidfunc(Tpl::emptyTxt.clone(), a_odeSchedule_tasks.clone(), a_daeSchedule_tasks.clone(), a_zeroFuncSchedule_tasks.clone(), a_allEquationsPlusWhen.clone(), (Tpl::textString(a_type.clone())?).clone(), a_name.clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("//void functions for functionhandling in tbb_nodes\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_taskFuncs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_evaluateMode = _evaluateMode;\n")).clone(), (literal!("  this->_command = command;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_varDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("if(_evaluateMode == 0)\n")).clone(), (literal!("{\n")).clone(), (literal!("  #if TBB_INTERFACE_VERSION >= 8000\n")).clone(), (literal!("    _tbbArena.execute(_tbbArenaFunctorOde);\n")).clone(), (literal!("  #else\n")).clone(), (literal!("    _tbbStartNodeOde.try_put(tbb::flow::continue_msg());\n")).clone(), (literal!("    _tbbGraphOde.wait_for_all();\n")).clone(), (literal!("  #endif\n")).clone(), (literal!("}\n")).clone(), (literal!("else if(_evaluateMode < 0)\n")).clone(), (literal!("{\n")).clone(), (literal!("  #if TBB_INTERFACE_VERSION >= 8000\n")).clone(), (literal!("    _tbbArena.execute(_tbbArenaFunctorAll);\n")).clone(), (literal!("  #else\n")).clone(), (literal!("    _tbbStartNodeAll.try_put(tbb::flow::continue_msg());\n")).clone(), (literal!("    _tbbGraphAll.wait_for_all();\n")).clone(), (literal!("  #endif\n")).clone(), (literal!("}\n")).clone(), (literal!("else\n")).clone(), (literal!("{\n")).clone(), (literal!("  #if TBB_INTERFACE_VERSION >= 8000\n")).clone(), (literal!("    _tbbArena.execute(_tbbArenaFunctorZeroFunc);\n")).clone(), (literal!("  #else\n")).clone(), (literal!("    _tbbStartNodeZeroFunc.try_put(tbb::flow::continue_msg());\n")).clone(), (literal!("    _tbbGraphZeroFunc.wait_for_all();\n")).clone(), (literal!("  #endif\n")).clone(), (literal!("}\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, _, _, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn fun_184(mut in_txt: Tpl::Text, mut in_a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_useFlatArrayNotation: bool, mut in_a_name: Arc<Absyn::Path>, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_functionHead: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt, in_a_schedulesOpt, in_a_modelNamePrefixStr, in_a_extraFuncsNamespace, in_a_type, in_a_useFlatArrayNotation, in_a_name, in_a_extraFuncsDecl, in_a_extraFuncs, in_a_simCode, in_a_varDecls, in_a_allEquationsPlusWhen, in_a_functionHead)) {
        (txt, Some((Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: i_taskListOde, .. } }, Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: i_taskListDae, .. } }, Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: i_taskListZeroFunc, .. } })), _, a_extraFuncsNamespace, _, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen, a_functionHead) => {
            let mut txt_2: Tpl::Text;
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  if(evaluateMode == 0) //evaluate ODE\n")).clone(), (literal!("  {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = parallelThreadCodeWithSplit(txt.clone(), a_allEquationsPlusWhen.clone(), i_taskListOde.clone(), 1, 1, (literal!("")).clone(), (literal!("")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0, (literal!("evaluateODE_Th1")).clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  else if(evaluateMode < 0) //evaluate All\n")).clone(), (literal!("  {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt_1 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_1) = parallelThreadCodeWithSplit(txt.clone(), a_allEquationsPlusWhen.clone(), i_taskListDae.clone(), 1, 1, (literal!("")).clone(), (literal!("")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_1, (literal!("evaluateAll_Th1")).clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  else //evaluate ZeroFuncs\n")).clone(), (literal!("  {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt_2 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_2) = parallelThreadCodeWithSplit(txt.clone(), a_allEquationsPlusWhen.clone(), i_taskListZeroFunc.clone(), 1, 1, (literal!("")).clone(), (literal!("")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_2, (literal!("evaluateZeroFunc_Th1")).clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: false, tasksOfLevels: i_tasksOfLevelsOde }, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: false, tasksOfLevels: i_tasksOfLevelsDae }, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: false, tasksOfLevels: i_tasksOfLevelsZeroFunc })), _, a_extraFuncsNamespace, a_type, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen, a_functionHead) => {
            let mut str_6: ArcStr;
            let mut l_zeroFuncEqs: Tpl::Text;
            let mut l_daeEqs: Tpl::Text;
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_odeEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_158(l_odeEqs, i_tasksOfLevelsOde.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs)?;
            l_daeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_daeEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_159(l_daeEqs, i_tasksOfLevelsDae.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_daeEqs = Tpl::popIter(l_daeEqs)?;
            l_zeroFuncEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_zeroFuncEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_160(l_zeroFuncEqs, i_tasksOfLevelsZeroFunc.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_zeroFuncEqs = Tpl::popIter(l_zeroFuncEqs)?;
            str_6 = (Tpl::textString(a_type.clone())?).clone();
            (txt, a_extraFuncsDecl) = fun_161(txt.clone(), (str_6).clone(), a_type.clone(), a_functionHead.clone(), l_zeroFuncEqs, l_daeEqs, l_odeEqs, a_name.clone(), a_extraFuncsDecl.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Some((i_odeSchedule @ Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, .. }, i_daeSchedule @ Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, .. }, i_zeroFuncSchedule @ Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, .. })), _, a_extraFuncsNamespace, a_type, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen, a_functionHead) => {
            let mut str_7: ArcStr;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            str_7 = (Tpl::textString(a_type.clone())?).clone();
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = fun_169(txt.clone(), (str_7).clone(), a_extraFuncsNamespace.clone(), a_functionHead.clone(), i_zeroFuncSchedule.clone(), i_daeSchedule.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone(), i_odeSchedule.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: i_odeSchedule_threadTasks @ i_threadTasksOde, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: i_daeSchedule_threadTasks @ i_threadTasksDae, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: i_zeroFuncSchedule_threadTasks @ i_threadTasksZeroFunc, .. })), a_modelNamePrefixStr, a_extraFuncsNamespace, a_type, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen, a_functionHead) => {
            let mut str_8: ArcStr;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            str_8 = (Tpl::textString(a_type.clone())?).clone();
            (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = fun_182(txt.clone(), (str_8).clone(), (a_modelNamePrefixStr.clone()).clone(), i_zeroFuncSchedule_threadTasks.clone(), i_daeSchedule_threadTasks.clone(), i_odeSchedule_threadTasks.clone(), a_functionHead.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_allEquationsPlusWhen.clone(), i_threadTasksZeroFunc.clone(), i_threadTasksDae.clone(), a_type.clone(), i_threadTasksOde.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_odeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_daeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_zeroFuncSchedule_tasks })), _, a_extraFuncsNamespace, a_type, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen, a_functionHead) => {
            let mut str_9: ArcStr;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            str_9 = (Tpl::textString(a_type.clone())?).clone();
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = fun_183(txt.clone(), (str_9).clone(), a_name.clone(), a_functionHead.clone(), i_zeroFuncSchedule_tasks.clone(), i_daeSchedule_tasks.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone(), i_odeSchedule_tasks.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_extraFuncsNamespace, _, _, _, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn fun_185(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_name: Arc<Absyn::Path>, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (::match_deref::match_deref! { match &((in_txt, in_a_simCode, in_a_allEquationsPlusWhen, in_a_name, in_a_extraFuncs, in_a_extraFuncsDecl, in_a_extraFuncsNamespace, in_a_schedulesOpt, in_a_modelNamePrefixStr, in_a_useFlatArrayNotation)) {
        (txt, i_simCode @ SimCode::SimCode { modelInfo: SimCode::ModelInfo { name: _, .. }, clockedPartitions: i_clockedPartitions, .. }, a_allEquationsPlusWhen, a_name, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_schedulesOpt, a_modelNamePrefixStr, a_useFlatArrayNotation) => {
            let mut txt_28: Tpl::Text;
            let mut l_functionHead: Tpl::Text;
            let mut ret_26: ArcStr;
            let mut l_type: Tpl::Text;
            let mut ret_24: bool;
            let mut ret_23: bool;
            let mut ret_22: ArcStr;
            let mut l_measureTimeEvaluateZeroFuncEnd: Tpl::Text;
            let mut ret_20: bool;
            let mut ret_19: bool;
            let mut ret_18: ArcStr;
            let mut l_measureTimeEvaluateZeroFuncStart: Tpl::Text;
            let mut ret_16: bool;
            let mut ret_15: bool;
            let mut ret_14: ArcStr;
            let mut l_measureTimeEvaluateAllEnd: Tpl::Text;
            let mut ret_12: bool;
            let mut ret_11: bool;
            let mut ret_10: ArcStr;
            let mut l_measureTimeEvaluateAllStart: Tpl::Text;
            let mut ret_8: bool;
            let mut ret_7: bool;
            let mut ret_6: ArcStr;
            let mut l_measureTimeEvaluateOdeEnd: Tpl::Text;
            let mut ret_4: bool;
            let mut ret_3: bool;
            let mut ret_2: ArcStr;
            let mut l_measureTimeEvaluateOdeStart: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            l_varDecls = Tpl::emptyTxt.clone();
            ret_2 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_3 = stringEq((ret_2).clone(), (literal!("none")).clone());
            ret_4 = boolNot(ret_3);
            l_measureTimeEvaluateOdeStart = fun_152(Tpl::emptyTxt.clone(), ret_4)?;
            ret_6 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_7 = stringEq((ret_6).clone(), (literal!("none")).clone());
            ret_8 = boolNot(ret_7);
            l_measureTimeEvaluateOdeEnd = fun_153(Tpl::emptyTxt.clone(), ret_8)?;
            ret_10 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_11 = stringEq((ret_10).clone(), (literal!("none")).clone());
            ret_12 = boolNot(ret_11);
            l_measureTimeEvaluateAllStart = fun_154(Tpl::emptyTxt.clone(), ret_12)?;
            ret_14 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_15 = stringEq((ret_14).clone(), (literal!("none")).clone());
            ret_16 = boolNot(ret_15);
            l_measureTimeEvaluateAllEnd = fun_155(Tpl::emptyTxt.clone(), ret_16)?;
            ret_18 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_19 = stringEq((ret_18).clone(), (literal!("none")).clone());
            ret_20 = boolNot(ret_19);
            l_measureTimeEvaluateZeroFuncStart = fun_156(Tpl::emptyTxt.clone(), ret_20)?;
            ret_22 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_23 = stringEq((ret_22).clone(), (literal!("none")).clone());
            ret_24 = boolNot(ret_23);
            l_measureTimeEvaluateZeroFuncEnd = fun_157(Tpl::emptyTxt.clone(), ret_24)?;
            ret_26 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
            l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_26).clone())?;
            l_functionHead = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//using type: ")).clone() }))?;
            l_functionHead = Tpl::writeText(l_functionHead, l_type.clone())?;
            l_functionHead = Tpl::softNewLine(l_functionHead)?;
            l_functionHead = Tpl::writeTok(l_functionHead, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            l_functionHead = CodegenCpp::lastIdentOfPath(l_functionHead, a_name.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateZeroFuncs(const UPDATETYPE command)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            l_functionHead = Tpl::pushBlock(l_functionHead, Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            l_functionHead = Tpl::writeText(l_functionHead, l_measureTimeEvaluateZeroFuncStart)?;
            l_functionHead = Tpl::softNewLine(l_functionHead)?;
            l_functionHead = Tpl::writeTok(l_functionHead, Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("evaluateParallel(command, 1);\n")).clone() }))?;
            l_functionHead = Tpl::writeText(l_functionHead, l_measureTimeEvaluateZeroFuncEnd)?;
            l_functionHead = Tpl::softNewLine(l_functionHead)?;
            l_functionHead = Tpl::popBlock(l_functionHead)?;
            l_functionHead = Tpl::writeTok(l_functionHead, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("bool ")).clone()], lastHasNewLine: false }))?;
            l_functionHead = CodegenCpp::lastIdentOfPath(l_functionHead, a_name.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateAll(const UPDATETYPE command)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            l_functionHead = Tpl::pushBlock(l_functionHead, Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            l_functionHead = Tpl::writeText(l_functionHead, l_measureTimeEvaluateAllStart)?;
            l_functionHead = Tpl::softNewLine(l_functionHead)?;
            l_functionHead = Tpl::writeTok(l_functionHead, openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt_28 = CodegenCppCommon::timeEventLength(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            l_functionHead = CodegenCpp::createTimeConditionTreatments(l_functionHead, (Tpl::textString(txt_28)?).clone(), i_clockedPartitions.clone())?;
            l_functionHead = Tpl::softNewLine(l_functionHead)?;
            l_functionHead = Tpl::writeTok(l_functionHead, openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            l_functionHead = Tpl::writeText(l_functionHead, l_varDecls.clone())?;
            l_functionHead = Tpl::softNewLine(l_functionHead)?;
            l_functionHead = Tpl::writeTok(l_functionHead, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("evaluateParallel(command, -1);\n")).clone()], lastHasNewLine: true }))?;
            l_functionHead = Tpl::writeText(l_functionHead, l_measureTimeEvaluateAllEnd)?;
            l_functionHead = Tpl::softNewLine(l_functionHead)?;
            l_functionHead = Tpl::writeTok(l_functionHead, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("return _state_var_reinitialized;\n")).clone()], lastHasNewLine: true }))?;
            l_functionHead = Tpl::popBlock(l_functionHead)?;
            l_functionHead = Tpl::writeTok(l_functionHead, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            l_functionHead = CodegenCpp::lastIdentOfPath(l_functionHead, a_name.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateODE(const UPDATETYPE command)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            l_functionHead = Tpl::pushBlock(l_functionHead, Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            l_functionHead = Tpl::writeText(l_functionHead, l_measureTimeEvaluateOdeStart)?;
            l_functionHead = Tpl::softNewLine(l_functionHead)?;
            l_functionHead = Tpl::writeTok(l_functionHead, Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("evaluateParallel(command, 0);\n")).clone() }))?;
            l_functionHead = Tpl::writeText(l_functionHead, l_measureTimeEvaluateOdeEnd)?;
            l_functionHead = Tpl::softNewLine(l_functionHead)?;
            l_functionHead = Tpl::popBlock(l_functionHead)?;
            l_functionHead = Tpl::writeTok(l_functionHead, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("//evaluateMode = 0 : evaluateODE\n")).clone(), (literal!("//evaluateMode < 0 : evaluateAll\n")).clone(), (literal!("//evaluateMode > 0 : evaluateZeroFunc\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            l_functionHead = CodegenCpp::lastIdentOfPath(l_functionHead, a_name.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateParallel(const UPDATETYPE command, int evaluateMode)")).clone() }))?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, l_varDecls) = fun_184(txt.clone(), a_schedulesOpt.clone(), (a_modelNamePrefixStr.clone()).clone(), a_extraFuncsNamespace.clone(), l_type, a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), i_simCode.clone(), l_varDecls, a_allEquationsPlusWhen.clone(), l_functionHead)?;
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, _, _, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _, _, _) => {
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub(crate) fn generateParallelEvaluate(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_name: Arc<Absyn::Path>, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut a_context: SimCodeFunction::Context, mut a_stateDerVectorName: Tpl::Text, mut a_modelNamePrefixStr: ArcStr, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = fun_185(txt, a_simCode, a_allEquationsPlusWhen, a_name, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_schedulesOpt, (a_modelNamePrefixStr).clone(), a_useFlatArrayNotation)?;
    out_a_stateDerVectorName = a_stateDerVectorName;
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace, out_a_stateDerVectorName))
}

fn lm_187(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_187 in &*items {
        let mut lstElt_187 = lstElt_187.clone();
        txt = (match lstElt_187.clone() {
        mut i_index => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("PREFETCH(&__z[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("], 0, 3);")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_188(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_simCode) {
        (mut txt, SimCode::SimCode { modelInfo: SimCode::ModelInfo { vars: SimCodeVar::SimVars { stateVars: ref i_vars_stateVars, .. }, .. }, .. }) => {
            let mut ret_2: Arc<metamodelica::List<i32>>;
            let mut ret_1: i32;
            let mut ret_0: i32;
            ret_0 = (i_vars_stateVars.clone().len() as i32);
            ret_1 = intSub(ret_0, 1);
            ret_2 = List::intRange3(0, 8, ret_1)?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_187(txt.clone(), ret_2)?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn generateStateVarPrefetchCode(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    out_txt = fun_188(txt, a_simCode)?;
    out_a_extraFuncs = a_extraFuncs;
    out_a_extraFuncsDecl = a_extraFuncsDecl;
    out_a_extraFuncsNamespace = a_extraFuncsNamespace;
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_190(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_iType: ArcStr, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_190 in &*items {
        let mut lstElt_190 = lstElt_190.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_190.clone()) {
        i_task => {
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelCodeForTask(txt.clone(), a_allEquationsPlusWhen.clone(), i_task.clone(), (a_iType.clone()).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_191(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_iType: ArcStr, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_191 in &*items {
        let mut lstElt_191 = lstElt_191.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_191.clone()) {
        i_task => {
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelCodeForTask(txt.clone(), a_allEquationsPlusWhen.clone(), i_task.clone(), (a_iType.clone()).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn fun_192(mut in_txt: Tpl::Text, mut in_a_tasksOfLevel: HpcOmSimCode::TaskList, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_iType: ArcStr, mut in_a_varDecls: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (::match_deref::match_deref! { match &((in_txt, in_a_tasksOfLevel, in_a_allEquationsPlusWhen, in_a_iType, in_a_varDecls, in_a_simCode, in_a_extraFuncs, in_a_extraFuncsDecl, in_a_extraFuncsNamespace, in_a_useFlatArrayNotation)) {
        (txt, HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: i_tasks }, a_allEquationsPlusWhen, a_iType, a_varDecls, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_odeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_190(l_odeEqs, i_tasks.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#pragma omp sections\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: i_tasks, .. }, a_allEquationsPlusWhen, a_iType, a_varDecls, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_odeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_191(l_odeEqs, i_tasks.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#pragma omp master\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("#pragma omp barrier")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, _, _, _, a_varDecls, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _) => {
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub(crate) fn generateLevelCodeForLevel(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_tasksOfLevel: HpcOmSimCode::TaskList, mut a_iType: ArcStr, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = fun_192(txt, a_tasksOfLevel, a_allEquationsPlusWhen, (a_iType).clone(), a_varDecls, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation)?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub(crate) fn generateLevelCodeForTask(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_iTask: Arc<HpcOmSimCode::Task>, mut a_iType: ArcStr, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#pragma omp section\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt, Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = taskCode(out_txt, a_allEquationsPlusWhen, a_iTask, (a_iType).clone(), (literal!("")).clone(), a_varDecls, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::popBlock(out_txt)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_195(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_iType: ArcStr, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_195 in &*items {
        let mut lstElt_195 = lstElt_195.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_195.clone()) {
        i_t => {
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = taskCode(txt.clone(), a_allEquationsPlusWhen.clone(), i_t.clone(), (a_iType.clone()).clone(), (literal!("")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_196(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_iType: ArcStr, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_196 in &*items {
        let mut lstElt_196 = lstElt_196.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_196.clone()) {
        i_threadTasks => {
            let mut x_i0: i32;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if(threadNum == ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_i0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_195(txt.clone(), i_threadTasks.clone(), a_useFlatArrayNotation, a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

pub(crate) fn generateLevelFixedCodeForLevel(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_tasksOfLevel: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut a_iType: ArcStr, mut a_varDecls: Tpl::Text, mut a_name: Arc<Absyn::Path>, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
    let mut l_eqs: Tpl::Text;
    ret_1 = Arc::new(a_tasksOfLevel.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    l_eqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (l_eqs, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_196(l_eqs, ret_1, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, (a_iType).clone(), a_allEquationsPlusWhen)?;
    l_eqs = Tpl::popIter(l_eqs)?;
    out_txt = Tpl::writeText(txt, l_eqs)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp barrier")).clone() }))?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_198(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_iType: ArcStr, mut a_iThreadIdx: i32, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_198 in &*items {
        let mut lstElt_198 = lstElt_198.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_198.clone()) {
        i_tasks => {
            let mut x_levelIdx: i32;
            x_levelIdx = Tpl::getIteri_i0(txt.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelFixedCodeForThreadLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), a_iThreadIdx, (literal!("evaluateODE")).clone(), (a_iType.clone()).clone(), x_levelIdx, a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_199(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_iType: ArcStr, mut a_iThreadIdx: i32, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_199 in &*items {
        let mut lstElt_199 = lstElt_199.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_199.clone()) {
        i_tasks => {
            let mut x_levelIdx: i32;
            x_levelIdx = Tpl::getIteri_i0(txt.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelFixedCodeForThreadLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), a_iThreadIdx, (literal!("evaluateDAE")).clone(), (a_iType.clone()).clone(), x_levelIdx, a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_200(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_iType: ArcStr, mut a_iThreadIdx: i32, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_200 in &*items {
        let mut lstElt_200 = lstElt_200.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_200.clone()) {
        i_tasks => {
            let mut x_levelIdx: i32;
            x_levelIdx = Tpl::getIteri_i0(txt.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelFixedCodeForThreadLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), a_iThreadIdx, (literal!("evaluateZeroFuncs")).clone(), (a_iType.clone()).clone(), x_levelIdx, a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn fun_201(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("MeasureTimeValues *valuesStart = MeasureTime::getZeroValues();\n")).clone(), (literal!("MeasureTimeValues *valuesEnd = MeasureTime::getZeroValues();")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_202(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_iThreadIdx) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateODEThread")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0, (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeStartCode(txt.clone(), (literal!("valuesStart")).clone(), (Tpl::textString(txt_0)?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_203(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_iThreadIdx) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayOdeHpcom)[")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0, (intString(a_iThreadIdx.clone())).clone())?;
            txt_0 = Tpl::writeTok(txt_0, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateODEThread")).clone() }))?;
            txt_1 = Tpl::writeStr(txt_1, (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("valuesStart")).clone(), (literal!("valuesEnd")).clone(), (Tpl::textString(txt_0)?).clone(), (Tpl::textString(txt_1)?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_204(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_iThreadIdx) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayDaeHpcom)[")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0, (intString(a_iThreadIdx.clone())).clone())?;
            txt_0 = Tpl::writeTok(txt_0, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateDaeThread")).clone() }))?;
            txt_1 = Tpl::writeStr(txt_1, (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("valuesStart")).clone(), (literal!("valuesEnd")).clone(), (Tpl::textString(txt_0)?).clone(), (Tpl::textString(txt_1)?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_205(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_iThreadIdx) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayZeroFuncHpcom)[")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0, (intString(a_iThreadIdx.clone())).clone())?;
            txt_0 = Tpl::writeTok(txt_0, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateZeroFuncThread")).clone() }))?;
            txt_1 = Tpl::writeStr(txt_1, (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("valuesStart")).clone(), (literal!("valuesEnd")).clone(), (Tpl::textString(txt_0)?).clone(), (Tpl::textString(txt_1)?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_206(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("delete valuesStart;\n")).clone(), (literal!("delete valuesEnd;")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_207(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32, mut in_a_name: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_iThreadIdx, in_a_name)) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_iThreadIdx, a_name) => {
            let mut ret_17: bool;
            let mut ret_16: bool;
            let mut ret_15: ArcStr;
            let mut ret_14: bool;
            let mut ret_13: bool;
            let mut ret_12: ArcStr;
            let mut ret_11: bool;
            let mut ret_10: bool;
            let mut ret_9: ArcStr;
            let mut ret_8: bool;
            let mut ret_7: bool;
            let mut ret_6: ArcStr;
            let mut ret_5: bool;
            let mut ret_4: bool;
            let mut ret_3: ArcStr;
            let mut ret_2: bool;
            let mut ret_1: bool;
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFunc")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_0 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_1 = stringEq((ret_0).clone(), (literal!("none")).clone());
            ret_2 = boolNot(ret_1);
            txt = fun_201(txt.clone(), ret_2)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("while(!_simulationFinished)\n")).clone(), (literal!("{\n")).clone(), (literal!("    //_evaluateBarrier.wait();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    if(_simulationFinished)\n")).clone(), (literal!("    {\n")).clone(), (literal!("        //_evaluateBarrier.wait();\n")).clone(), (literal!("        _levelBarrier.wait();\n")).clone(), (literal!("        break;\n")).clone(), (literal!("    }\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            ret_3 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_4 = stringEq((ret_3).clone(), (literal!("none")).clone());
            ret_5 = boolNot(ret_4);
            txt = fun_202(txt.clone(), ret_5, a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("if(_evaluateMode == 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncODE_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            ret_6 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_7 = stringEq((ret_6).clone(), (literal!("none")).clone());
            ret_8 = boolNot(ret_7);
            txt = fun_203(txt.clone(), ret_8, a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else if(_evaluateMode < 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            ret_9 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_10 = stringEq((ret_9).clone(), (literal!("none")).clone());
            ret_11 = boolNot(ret_10);
            txt = fun_204(txt.clone(), ret_11, a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            ret_12 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_13 = stringEq((ret_12).clone(), (literal!("none")).clone());
            ret_14 = boolNot(ret_13);
            txt = fun_205(txt.clone(), ret_14, a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("//_evaluateBarrier.wait();\n")).clone(), (literal!("_levelBarrier.wait();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            ret_15 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_16 = stringEq((ret_15).clone(), (literal!("none")).clone());
            ret_17 = boolNot(ret_16);
            txt = fun_206(txt.clone(), ret_17)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_208(mut in_txt: Tpl::Text, mut in_a_tasksOfLevels: (Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>), mut in_a_name: Arc<Absyn::Path>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_iThreadIdx: i32, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt, in_a_tasksOfLevels, in_a_name, in_a_useFlatArrayNotation, in_a_extraFuncsNamespace, in_a_extraFuncsDecl, in_a_extraFuncs, in_a_simCode, in_a_varDecls, in_a_iType, in_a_iThreadIdx, in_a_allEquationsPlusWhen)) {
        (txt, (i_odeTasksOfLevel, i_daeTasksOfLevel, i_zeroFuncTasksOfLevel), a_name, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_iThreadIdx, a_allEquationsPlusWhen) => {
            let mut ret_3: bool;
            let mut l_zeroFuncEqs: Tpl::Text;
            let mut l_daeEqs: Tpl::Text;
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_odeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_198(l_odeEqs, i_odeTasksOfLevel.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs)?;
            l_daeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_daeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_199(l_daeEqs, i_daeTasksOfLevel.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            l_daeEqs = Tpl::popIter(l_daeEqs)?;
            l_zeroFuncEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_zeroFuncEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_200(l_zeroFuncEqs, i_zeroFuncTasksOfLevel.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            l_zeroFuncEqs = Tpl::popIter(l_zeroFuncEqs)?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void evaluateThreadFuncODE_")).clone() }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFuncAll_")).clone()], lastHasNewLine: false }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFuncZeroFunc_")).clone()], lastHasNewLine: false }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFunc")).clone()], lastHasNewLine: false }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncODE_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_daeEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_zeroFuncEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            ret_3 = intGt(a_iThreadIdx.clone(), 0);
            txt = fun_207(txt.clone(), ret_3, a_iThreadIdx.clone(), a_name.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_209(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_iType: ArcStr, mut a_iThreadIdx: i32, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_209 in &*items {
        let mut lstElt_209 = lstElt_209.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_209.clone()) {
        i_tasks => {
            let mut x_levelIdx: i32;
            x_levelIdx = Tpl::getIteri_i0(txt.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelFixedCodeForThreadLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), a_iThreadIdx, (literal!("evaluateODE")).clone(), (a_iType.clone()).clone(), x_levelIdx, a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_210(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_iType: ArcStr, mut a_iThreadIdx: i32, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_210 in &*items {
        let mut lstElt_210 = lstElt_210.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_210.clone()) {
        i_tasks => {
            let mut x_levelIdx: i32;
            x_levelIdx = Tpl::getIteri_i0(txt.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelFixedCodeForThreadLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), a_iThreadIdx, (literal!("evaluateDAE")).clone(), (a_iType.clone()).clone(), x_levelIdx, a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_211(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_iType: ArcStr, mut a_iThreadIdx: i32, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_211 in &*items {
        let mut lstElt_211 = lstElt_211.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_211.clone()) {
        i_tasks => {
            let mut x_levelIdx: i32;
            x_levelIdx = Tpl::getIteri_i0(txt.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelFixedCodeForThreadLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), a_iThreadIdx, (literal!("evaluateZeroFuncs")).clone(), (a_iType.clone()).clone(), x_levelIdx, a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn fun_212(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("MeasureTimeValues *valuesStart = MeasureTime::getZeroValues();\n")).clone(), (literal!("MeasureTimeValues *valuesEnd = MeasureTime::getZeroValues();")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_213(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_iThreadIdx) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateODEThread")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0, (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeStartCode(txt.clone(), (literal!("valuesStart")).clone(), (Tpl::textString(txt_0)?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_214(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_iThreadIdx) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayOdeHpcom)[")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0, (intString(a_iThreadIdx.clone())).clone())?;
            txt_0 = Tpl::writeTok(txt_0, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateODEThread")).clone() }))?;
            txt_1 = Tpl::writeStr(txt_1, (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("valuesStart")).clone(), (literal!("valuesEnd")).clone(), (Tpl::textString(txt_0)?).clone(), (Tpl::textString(txt_1)?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_215(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_iThreadIdx) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayDaeHpcom)[")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0, (intString(a_iThreadIdx.clone())).clone())?;
            txt_0 = Tpl::writeTok(txt_0, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateDaeThread")).clone() }))?;
            txt_1 = Tpl::writeStr(txt_1, (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("valuesStart")).clone(), (literal!("valuesEnd")).clone(), (Tpl::textString(txt_0)?).clone(), (Tpl::textString(txt_1)?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_216(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_iThreadIdx) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayZeroFuncHpcom)[")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0, (intString(a_iThreadIdx.clone())).clone())?;
            txt_0 = Tpl::writeTok(txt_0, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateZeroFuncThread")).clone() }))?;
            txt_1 = Tpl::writeStr(txt_1, (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("valuesStart")).clone(), (literal!("valuesEnd")).clone(), (Tpl::textString(txt_0)?).clone(), (Tpl::textString(txt_1)?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_217(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("delete valuesStart;\n")).clone(), (literal!("delete valuesEnd;")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_218(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32, mut in_a_name: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_iThreadIdx, in_a_name)) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_iThreadIdx, a_name) => {
            let mut ret_17: bool;
            let mut ret_16: bool;
            let mut ret_15: ArcStr;
            let mut ret_14: bool;
            let mut ret_13: bool;
            let mut ret_12: ArcStr;
            let mut ret_11: bool;
            let mut ret_10: bool;
            let mut ret_9: ArcStr;
            let mut ret_8: bool;
            let mut ret_7: bool;
            let mut ret_6: ArcStr;
            let mut ret_5: bool;
            let mut ret_4: bool;
            let mut ret_3: ArcStr;
            let mut ret_2: bool;
            let mut ret_1: bool;
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFunc")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_0 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_1 = stringEq((ret_0).clone(), (literal!("none")).clone());
            ret_2 = boolNot(ret_1);
            txt = fun_212(txt.clone(), ret_2)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("while(!_simulationFinished)\n")).clone(), (literal!("{\n")).clone(), (literal!("    //_evaluateBarrier.wait();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    if(_simulationFinished)\n")).clone(), (literal!("    {\n")).clone(), (literal!("        //_evaluateBarrier.wait();\n")).clone(), (literal!("        _levelBarrier.wait();\n")).clone(), (literal!("        break;\n")).clone(), (literal!("    }\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            ret_3 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_4 = stringEq((ret_3).clone(), (literal!("none")).clone());
            ret_5 = boolNot(ret_4);
            txt = fun_213(txt.clone(), ret_5, a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("if(_evaluateMode == 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncODE_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            ret_6 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_7 = stringEq((ret_6).clone(), (literal!("none")).clone());
            ret_8 = boolNot(ret_7);
            txt = fun_214(txt.clone(), ret_8, a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else if(_evaluateMode < 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            ret_9 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_10 = stringEq((ret_9).clone(), (literal!("none")).clone());
            ret_11 = boolNot(ret_10);
            txt = fun_215(txt.clone(), ret_11, a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            ret_12 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_13 = stringEq((ret_12).clone(), (literal!("none")).clone());
            ret_14 = boolNot(ret_13);
            txt = fun_216(txt.clone(), ret_14, a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("//_evaluateBarrier.wait();\n")).clone(), (literal!("_levelBarrier.wait();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            ret_15 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_16 = stringEq((ret_15).clone(), (literal!("none")).clone());
            ret_17 = boolNot(ret_16);
            txt = fun_217(txt.clone(), ret_17)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_219(mut in_txt: Tpl::Text, mut in_a_tasksOfLevels: (Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>), mut in_a_name: Arc<Absyn::Path>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_iThreadIdx: i32, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt, in_a_tasksOfLevels, in_a_name, in_a_useFlatArrayNotation, in_a_extraFuncsNamespace, in_a_extraFuncsDecl, in_a_extraFuncs, in_a_simCode, in_a_varDecls, in_a_iType, in_a_iThreadIdx, in_a_allEquationsPlusWhen)) {
        (txt, (i_odeTasksOfLevel, i_daeTasksOfLevel, i_zeroFuncTasksOfLevel), a_name, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_iThreadIdx, a_allEquationsPlusWhen) => {
            let mut ret_3: bool;
            let mut l_zeroFuncEqs: Tpl::Text;
            let mut l_daeEqs: Tpl::Text;
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_odeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_209(l_odeEqs, i_odeTasksOfLevel.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs)?;
            l_daeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_daeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_210(l_daeEqs, i_daeTasksOfLevel.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            l_daeEqs = Tpl::popIter(l_daeEqs)?;
            l_zeroFuncEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_zeroFuncEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_211(l_zeroFuncEqs, i_zeroFuncTasksOfLevel.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            l_zeroFuncEqs = Tpl::popIter(l_zeroFuncEqs)?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void evaluateThreadFuncODE_")).clone() }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFuncAll_")).clone()], lastHasNewLine: false }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFuncZeroFunc_")).clone()], lastHasNewLine: false }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFunc")).clone()], lastHasNewLine: false }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncODE_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_daeEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_zeroFuncEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            ret_3 = intGt(a_iThreadIdx.clone(), 0);
            txt = fun_218(txt.clone(), ret_3, a_iThreadIdx.clone(), a_name.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn fun_220(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_tasksOfLevels: (Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>), mut in_a_iThreadIdx: i32, mut in_a_varDecls: Tpl::Text, mut in_a_name: Arc<Absyn::Path>, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_allEquationsPlusWhen, in_a_tasksOfLevels, in_a_iThreadIdx, in_a_varDecls, in_a_name, in_a_simCode, in_a_extraFuncs, in_a_extraFuncsDecl, in_a_extraFuncsNamespace, in_a_useFlatArrayNotation)) {
        (txt, i_iType @ Deref @ "pthreads", a_allEquationsPlusWhen, a_tasksOfLevels, a_iThreadIdx, a_varDecls, a_name, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = fun_208(txt.clone(), a_tasksOfLevels.clone(), a_name.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (i_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, i_iType @ Deref @ "pthreads_spin", a_allEquationsPlusWhen, a_tasksOfLevels, a_iThreadIdx, a_varDecls, a_name, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = fun_219(txt.clone(), a_tasksOfLevels.clone(), a_name.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (i_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, _, _, _, _, a_varDecls, _, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _) => {
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub(crate) fn generateLevelFixedCodeForThread(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_tasksOfLevels: (Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>), mut a_iThreadIdx: i32, mut a_iType: ArcStr, mut a_varDecls: Tpl::Text, mut a_name: Arc<Absyn::Path>, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = fun_220(txt, (a_iType).clone(), a_allEquationsPlusWhen, a_tasksOfLevels, a_iThreadIdx, a_varDecls, a_name, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation)?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_222(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_iType: ArcStr, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_222 in &*items {
        let mut lstElt_222 = lstElt_222.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_222.clone()) {
        i_t => {
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = taskCode(txt.clone(), a_allEquationsPlusWhen.clone(), i_t.clone(), (a_iType.clone()).clone(), (literal!("")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn fun_223(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iLevelIdx: i32, mut in_a_functionName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_iLevelIdx, in_a_functionName) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iLevelIdx, mut a_functionName) => {
            let mut txt_0: Tpl::Text;
            let mut ret_0: i32;
            txt_0 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_functionName.clone()).clone())?;
            txt_0 = Tpl::writeTok(txt_0, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_level_")).clone() }))?;
            ret_0 = intAdd(a_iLevelIdx.clone(), 1);
            txt_0 = Tpl::writeStr(txt_0, (intString(ret_0)).clone())?;
            txt = CodegenCpp::generateMeasureTimeStartCode(txt.clone(), (literal!("measuredSchedulerStartValues")).clone(), (Tpl::textString(txt_0)?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_224(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Tpl::Text {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    out_txt
}

fn fun_225(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iLevelIdx: i32, mut in_a_functionName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_iLevelIdx, in_a_functionName) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iLevelIdx, mut a_functionName) => {
            let mut txt_1: Tpl::Text;
            let mut ret_1: i32;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeSchedulerArrayHpcom_")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0, (a_functionName.clone()).clone())?;
            txt_0 = Tpl::writeTok(txt_0, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")[")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0, (intString(a_iLevelIdx.clone())).clone())?;
            txt_0 = Tpl::writeTok(txt_0, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt_1 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_functionName.clone()).clone())?;
            txt_1 = Tpl::writeTok(txt_1, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_level_")).clone() }))?;
            ret_1 = intAdd(a_iLevelIdx.clone(), 1);
            txt_1 = Tpl::writeStr(txt_1, (intString(ret_1)).clone())?;
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("measuredSchedulerStartValues")).clone(), (literal!("measuredSchedulerEndValues")).clone(), (Tpl::textString(txt_0)?).clone(), (Tpl::textString(txt_1)?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn generateLevelFixedCodeForThreadLevel(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_tasksOfLevel: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_iThreadIdx: i32, mut a_functionName: ArcStr, mut a_iType: ArcStr, mut a_iLevelIdx: i32, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut ret_3: bool;
    let mut ret_2: bool;
    let mut ret_1: bool;
    let mut l_tasks: Tpl::Text;
    l_tasks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (l_tasks, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_222(l_tasks, a_tasksOfLevel, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, (a_iType).clone(), a_allEquationsPlusWhen)?;
    l_tasks = Tpl::popIter(l_tasks)?;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//Start of Level ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt, (intString(a_iLevelIdx)).clone())?;
    out_txt = Tpl::softNewLine(out_txt)?;
    ret_1 = intEq(a_iThreadIdx, 0);
    out_txt = fun_223(out_txt, ret_1, a_iLevelIdx, (a_functionName.clone()).clone())?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeTok(out_txt, openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
    ret_2 = stringEq((Tpl::textString(l_tasks.clone())?).clone(), (literal!("")).clone());
    out_txt = fun_224(out_txt, ret_2);
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeText(out_txt, l_tasks)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_levelBarrier.wait();\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
    ret_3 = intEq(a_iThreadIdx, 0);
    out_txt = fun_225(out_txt, ret_3, a_iLevelIdx, (a_functionName).clone())?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//End of Level ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt, (intString(a_iLevelIdx)).clone())?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_227(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_iType: ArcStr, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_227 in &*items {
        let mut lstElt_227 = lstElt_227.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_227.clone()) {
        i_t => {
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep0(txt.clone(), i_t.clone(), a_allEquationsPlusWhen.clone(), (a_iType.clone()).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

pub(crate) fn function_HPCOM_TaskDep(mut txt: Tpl::Text, mut a_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_iType: ArcStr, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut l_odeEqs: Tpl::Text;
    l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (l_odeEqs, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_227(l_odeEqs, a_tasks, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, (a_iType).clone(), a_allEquationsPlusWhen)?;
    l_odeEqs = Tpl::popIter(l_odeEqs)?;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("int t[0];\n")).clone(), (literal!("#pragma omp parallel\n")).clone(), (literal!("{\n")).clone(), (literal!("  #pragma omp master\n")).clone(), (literal!("  {\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt, Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
    out_txt = Tpl::writeText(out_txt, l_odeEqs)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::popBlock(out_txt)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_229(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_229 in &*items {
        let mut lstElt_229 = lstElt_229.clone();
        txt = (match lstElt_229.clone() {
        mut i_p => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("t[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_p.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_230(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_230 in &*items {
        let mut lstElt_230 = lstElt_230.clone();
        txt = (match lstElt_230.clone() {
        mut i_p => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_p.clone())).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_231(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_parentDependencies: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_parentDependencies) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_parentDependencies) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("depend(in:")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_parentDependencies.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn function_HPCOM_TaskDep0(mut in_txt: Tpl::Text, mut in_a_taskIn: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>), mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_iType: ArcStr, mut in_a_varDecls: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (::match_deref::match_deref! { match &((in_txt, in_a_taskIn, in_a_allEquationsPlusWhen, in_a_iType, in_a_varDecls, in_a_simCode, in_a_extraFuncs, in_a_extraFuncsDecl, in_a_extraFuncsNamespace, in_a_useFlatArrayNotation)) {
        (txt, (i_task @ Deref @ HpcOmSimCode::Task::CALCTASK { index: i_task_index, .. }, i_parents), a_allEquationsPlusWhen, a_iType, a_varDecls, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut ret_5: bool;
            let mut ret_4: i32;
            let mut l_depIn: Tpl::Text;
            let mut l_taskDependencies: Tpl::Text;
            let mut l_parentDependencies: Tpl::Text;
            let mut l_taskEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            (l_taskEqs, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = taskCode(Tpl::emptyTxt.clone(), a_allEquationsPlusWhen.clone(), i_task.clone(), (a_iType.clone()).clone(), (literal!("")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            l_parentDependencies = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_parentDependencies = lm_229(l_parentDependencies, i_parents.clone())?;
            l_parentDependencies = Tpl::popIter(l_parentDependencies)?;
            l_taskDependencies = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_taskDependencies = lm_230(l_taskDependencies, i_parents.clone())?;
            l_taskDependencies = Tpl::popIter(l_taskDependencies)?;
            ret_4 = (i_parents.clone().len() as i32);
            ret_5 = intGt(ret_4, 0);
            l_depIn = fun_231(Tpl::emptyTxt.clone(), ret_5, l_parentDependencies)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//TG_NODE: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" TG_PARENTS: ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_taskDependencies)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp task ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_depIn)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("depend(out:t[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("])\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_taskEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, _, _, _, a_varDecls, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _) => {
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_233(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_233 in &*items {
        let mut lstElt_233 = lstElt_233.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_233.clone()) {
        i_t => {
            let mut x_i: i32;
            x_i = Tpl::getIteri_i0(txt.clone())?;
            txt = generateTbbConstructorExtensionNodes(txt.clone(), i_t.clone(), x_i, (literal!("Ode")).clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_234(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_234 in &*items {
        let mut lstElt_234 = lstElt_234.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_234.clone()) {
        i_t => {
            let mut x_i: i32;
            x_i = Tpl::getIteri_i0(txt.clone())?;
            txt = generateTbbConstructorExtensionEdges(txt.clone(), i_t.clone(), x_i, (literal!("Ode")).clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_235(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_235 in &*items {
        let mut lstElt_235 = lstElt_235.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_235.clone()) {
        i_t => {
            let mut x_i: i32;
            x_i = Tpl::getIteri_i0(txt.clone())?;
            txt = generateTbbConstructorExtensionNodes(txt.clone(), i_t.clone(), x_i, (literal!("All")).clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_236(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_236 in &*items {
        let mut lstElt_236 = lstElt_236.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_236.clone()) {
        i_t => {
            let mut x_i: i32;
            x_i = Tpl::getIteri_i0(txt.clone())?;
            txt = generateTbbConstructorExtensionEdges(txt.clone(), i_t.clone(), x_i, (literal!("All")).clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_237(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_237 in &*items {
        let mut lstElt_237 = lstElt_237.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_237.clone()) {
        i_t => {
            let mut x_i: i32;
            x_i = Tpl::getIteri_i0(txt.clone())?;
            txt = generateTbbConstructorExtensionNodes(txt.clone(), i_t.clone(), x_i, (literal!("ZeroFunc")).clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_238(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_238 in &*items {
        let mut lstElt_238 = lstElt_238.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_238.clone()) {
        i_t => {
            let mut x_i: i32;
            x_i = Tpl::getIteri_i0(txt.clone())?;
            txt = generateTbbConstructorExtensionEdges(txt.clone(), i_t.clone(), x_i, (literal!("ZeroFunc")).clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn generateTbbConstructorExtension(mut txt: Tpl::Text, mut a_odeTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_daeTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_zeroFuncTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_6: i32;
    let mut l_zeroFuncEdges: Tpl::Text;
    let mut l_zeroFuncNodes: Tpl::Text;
    let mut l_daeEdges: Tpl::Text;
    let mut l_daeNodes: Tpl::Text;
    let mut l_odeEdges: Tpl::Text;
    let mut l_odeNodes: Tpl::Text;
    l_odeNodes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    l_odeNodes = lm_233(l_odeNodes, a_odeTasks.clone(), (a_modelNamePrefixStr.clone()).clone())?;
    l_odeNodes = Tpl::popIter(l_odeNodes)?;
    l_odeEdges = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    l_odeEdges = lm_234(l_odeEdges, a_odeTasks, (a_modelNamePrefixStr.clone()).clone())?;
    l_odeEdges = Tpl::popIter(l_odeEdges)?;
    l_daeNodes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    l_daeNodes = lm_235(l_daeNodes, a_daeTasks.clone(), (a_modelNamePrefixStr.clone()).clone())?;
    l_daeNodes = Tpl::popIter(l_daeNodes)?;
    l_daeEdges = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    l_daeEdges = lm_236(l_daeEdges, a_daeTasks, (a_modelNamePrefixStr.clone()).clone())?;
    l_daeEdges = Tpl::popIter(l_daeEdges)?;
    l_zeroFuncNodes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    l_zeroFuncNodes = lm_237(l_zeroFuncNodes, a_zeroFuncTasks.clone(), (a_modelNamePrefixStr.clone()).clone())?;
    l_zeroFuncNodes = Tpl::popIter(l_zeroFuncNodes)?;
    l_zeroFuncEdges = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    l_zeroFuncEdges = lm_238(l_zeroFuncEdges, a_zeroFuncTasks, (a_modelNamePrefixStr).clone())?;
    l_zeroFuncEdges = Tpl::popIter(l_zeroFuncEdges)?;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("tbb::flow::continue_node<tbb::flow::continue_msg> *tbb_task;\n")).clone() }))?;
    out_txt = Tpl::writeText(out_txt, l_odeNodes)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeText(out_txt, l_odeEdges)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeText(out_txt, l_daeNodes)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeText(out_txt, l_daeEdges)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeText(out_txt, l_zeroFuncNodes)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeText(out_txt, l_zeroFuncEdges)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#if TBB_INTERFACE_VERSION >= 8000\n")).clone(), (literal!("_tbbArena = tbb::task_arena(")).clone()], lastHasNewLine: false }))?;
    ret_6 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
    out_txt = Tpl::writeStr(out_txt, (intString(ret_6)).clone())?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(");\n")).clone(), (literal!("_tbbArenaFunctorOde = TbbArenaFunctor(_tbbGraphOde,_tbbStartNodeOde);\n")).clone(), (literal!("_tbbArenaFunctorAll = TbbArenaFunctor(_tbbGraphAll,_tbbStartNodeAll);\n")).clone(), (literal!("_tbbArenaFunctorZeroFunc = TbbArenaFunctor(_tbbGraphZeroFunc,_tbbStartNodeZeroFunc);\n")).clone(), (literal!("#endif")).clone()], lastHasNewLine: false }))?;
    Ok(out_txt)
}

pub(crate) fn generateTbbConstructorExtensionNodes(mut in_txt: Tpl::Text, mut in_a_taskIn: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>), mut in_a_taskIndex: i32, mut in_a_funcSuffix: ArcStr, mut in_a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_taskIn, in_a_taskIndex, in_a_funcSuffix, in_a_modelNamePrefixStr)) {
        (txt, (Deref @ HpcOmSimCode::Task::CALCTASK { index: i_task_index, .. }, _), a_taskIndex, a_funcSuffix, a_modelNamePrefixStr) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tbb_task = new tbb::flow::continue_node<tbb::flow::continue_msg>(_tbbGraph")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",VoidFunctionBody(bind<void>(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::taskFunc")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",this)));\n")).clone(), (literal!("_tbbNodeList")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".at(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_taskIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") = tbb_task;")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_241(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>, mut a_taskIndex: i32, mut a_funcSuffix: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_241 in &*items {
        let mut lstElt_241 = lstElt_241.clone();
        txt = (match lstElt_241.clone() {
        mut i_p => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tbb::flow::make_edge(*(_tbbNodeList")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".at(")).clone() }))?;
            ret_0 = intSub(i_p.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")),*(_tbbNodeList")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".at(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_taskIndex)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")));")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_242(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_taskIndex: i32, mut in_a_funcSuffix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_taskIndex, in_a_funcSuffix) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_taskIndex, mut a_funcSuffix) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tbb::flow::make_edge(_tbbStartNode")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",*(_tbbNodeList")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".at(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_taskIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")));")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_243(mut in_txt: Tpl::Text, mut in_a_taskIn: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>), mut in_a_taskIndex: i32, mut in_a_funcSuffix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_taskIn, in_a_taskIndex, in_a_funcSuffix)) {
        (txt, (Deref @ HpcOmSimCode::Task::CALCTASK { weighting: _, .. }, i_parents), a_taskIndex, a_funcSuffix) => {
            let mut ret_3: bool;
            let mut ret_2: i32;
            let mut l_startNodeEdge: Tpl::Text;
            let mut l_parentEdges: Tpl::Text;
            let mut txt = (*txt).clone();
            l_parentEdges = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_parentEdges = lm_241(l_parentEdges, i_parents.clone(), a_taskIndex.clone(), (a_funcSuffix.clone()).clone())?;
            l_parentEdges = Tpl::popIter(l_parentEdges)?;
            ret_2 = (i_parents.clone().len() as i32);
            ret_3 = intEq(0, ret_2);
            l_startNodeEdge = fun_242(Tpl::emptyTxt.clone(), ret_3, a_taskIndex.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_parentEdges)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_startNodeEdge)?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn generateTbbConstructorExtensionEdges(mut txt: Tpl::Text, mut a_taskIn: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>), mut a_taskIndex: i32, mut a_funcSuffix: ArcStr, mut a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_243(txt, a_taskIn, a_taskIndex, (a_funcSuffix).clone())?;
    Ok(out_txt)
}

fn lm_245(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_name: Arc<Absyn::Path>, mut a_iType: ArcStr, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_245 in &*items {
        let mut lstElt_245 = lstElt_245.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_245.clone()) {
        i_t => {
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep_voidfunc0(txt.clone(), i_t.clone(), a_allEquationsPlusWhen.clone(), (a_iType.clone()).clone(), (literal!("Ode")).clone(), a_name.clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_246(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_name: Arc<Absyn::Path>, mut a_iType: ArcStr, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_246 in &*items {
        let mut lstElt_246 = lstElt_246.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_246.clone()) {
        i_t => {
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep_voidfunc0(txt.clone(), i_t.clone(), a_allEquationsPlusWhen.clone(), (a_iType.clone()).clone(), (literal!("All")).clone(), a_name.clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_247(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_name: Arc<Absyn::Path>, mut a_iType: ArcStr, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_247 in &*items {
        let mut lstElt_247 = lstElt_247.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_247.clone()) {
        i_t => {
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep_voidfunc0(txt.clone(), i_t.clone(), a_allEquationsPlusWhen.clone(), (a_iType.clone()).clone(), (literal!("ZeroFunc")).clone(), a_name.clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

pub(crate) fn function_HPCOM_TaskDep_voidfunc(mut txt: Tpl::Text, mut a_odeTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_daeTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_zeroFuncTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_iType: ArcStr, mut a_name: Arc<Absyn::Path>, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut l_funcTasksZeroFunc: Tpl::Text;
    let mut l_funcTasksDae: Tpl::Text;
    let mut l_funcTasksOde: Tpl::Text;
    l_funcTasksOde = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (l_funcTasksOde, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_245(l_funcTasksOde, a_odeTasks, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode.clone(), a_varDecls, a_name.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
    l_funcTasksOde = Tpl::popIter(l_funcTasksOde)?;
    l_funcTasksDae = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (l_funcTasksDae, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_246(l_funcTasksDae, a_daeTasks, a_useFlatArrayNotation, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, a_simCode.clone(), out_a_varDecls, a_name.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
    l_funcTasksDae = Tpl::popIter(l_funcTasksDae)?;
    l_funcTasksZeroFunc = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (l_funcTasksZeroFunc, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_247(l_funcTasksZeroFunc, a_zeroFuncTasks, a_useFlatArrayNotation, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, a_simCode, out_a_varDecls, a_name, (a_iType).clone(), a_allEquationsPlusWhen)?;
    l_funcTasksZeroFunc = Tpl::popIter(l_funcTasksZeroFunc)?;
    out_txt = Tpl::writeText(txt, l_funcTasksOde)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeText(out_txt, l_funcTasksDae)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeText(out_txt, l_funcTasksZeroFunc)?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_249(mut in_txt: Tpl::Text, mut in_a_taskIn: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>), mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_iType: ArcStr, mut in_a_funcSuffix: ArcStr, mut in_a_name: Arc<Absyn::Path>, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (::match_deref::match_deref! { match &((in_txt, in_a_taskIn, in_a_allEquationsPlusWhen, in_a_iType, in_a_funcSuffix, in_a_name, in_a_simCode, in_a_extraFuncs, in_a_extraFuncsDecl, in_a_extraFuncsNamespace, in_a_useFlatArrayNotation)) {
        (txt, (i_task @ Deref @ HpcOmSimCode::Task::CALCTASK { index: i_task_index, .. }, _), a_allEquationsPlusWhen, a_iType, a_funcSuffix, a_name, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut l_taskEqs: Tpl::Text;
            let mut l_tempvarDecl: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            l_tempvarDecl = Tpl::emptyTxt.clone();
            (l_taskEqs, l_tempvarDecl, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = taskCode(Tpl::emptyTxt.clone(), a_allEquationsPlusWhen.clone(), i_task.clone(), (a_iType.clone()).clone(), (literal!("")).clone(), l_tempvarDecl, a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::taskFunc")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_tempvarDecl)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_taskEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, _, _, _, _, _, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _) => {
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub(crate) fn function_HPCOM_TaskDep_voidfunc0(mut txt: Tpl::Text, mut a_taskIn: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>), mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_iType: ArcStr, mut a_funcSuffix: ArcStr, mut a_name: Arc<Absyn::Path>, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = fun_249(txt, a_taskIn, a_allEquationsPlusWhen, (a_iType).clone(), (a_funcSuffix).clone(), a_name, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation)?;
    out_a_varDecls = a_varDecls;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_251(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_relLock: Tpl::Text, mut in_a_assLock: Tpl::Text, mut in_a_taskEqsZeroFunc: Tpl::Text, mut in_a_taskEqsDae: Tpl::Text, mut in_a_taskEqsOde: Tpl::Text, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_iThreadIdx: i32, mut in_a_varDeclsLoc: Tpl::Text, mut in_a_mainThreadCode: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_mainThreadCode: Tpl::Text;
    (out_txt, out_a_mainThreadCode) = (match (in_txt, in_mArg, in_a_relLock, in_a_assLock, in_a_taskEqsZeroFunc, in_a_taskEqsDae, in_a_taskEqsOde, in_a_modelNamePrefixStr, in_a_iThreadIdx, in_a_varDeclsLoc, in_a_mainThreadCode) {
        (mut txt, false, _, _, mut a_taskEqsZeroFunc, mut a_taskEqsDae, mut a_taskEqsOde, mut a_modelNamePrefixStr, mut a_iThreadIdx, mut a_varDeclsLoc, mut a_mainThreadCode) => {
            a_mainThreadCode = Tpl::writeText(a_mainThreadCode.clone(), a_varDeclsLoc.clone())?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#ifdef MEASURETIME_MODELFUNCTIONS\n")).clone(), (literal!("MeasureTimeValues *measuredSchedulerStartValues = measuredSchedulerStartValues_0;\n")).clone(), (literal!("MeasureTimeValues *measuredSchedulerEndValues = measuredSchedulerEndValues_0;\n")).clone(), (literal!("#endif //MEASURETIME_MODELFUNCTIONS\n")).clone(), (literal!("if(_evaluateMode == 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            a_mainThreadCode = Tpl::pushBlock(a_mainThreadCode.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncODE_")).clone() }))?;
            a_mainThreadCode = Tpl::writeStr(a_mainThreadCode.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            a_mainThreadCode = Tpl::popBlock(a_mainThreadCode.clone())?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else if(_evaluateMode < 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            a_mainThreadCode = Tpl::pushBlock(a_mainThreadCode.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncAll_")).clone() }))?;
            a_mainThreadCode = Tpl::writeStr(a_mainThreadCode.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            a_mainThreadCode = Tpl::popBlock(a_mainThreadCode.clone())?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            a_mainThreadCode = Tpl::pushBlock(a_mainThreadCode.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncZeroFunc_")).clone() }))?;
            a_mainThreadCode = Tpl::writeStr(a_mainThreadCode.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            a_mainThreadCode = Tpl::popBlock(a_mainThreadCode.clone())?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncODE_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_taskEqsOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_taskEqsDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_taskEqsZeroFunc.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_mainThreadCode.clone())
        },
        (mut txt, _, mut a_relLock, mut a_assLock, mut a_taskEqsZeroFunc, mut a_taskEqsDae, mut a_taskEqsOde, mut a_modelNamePrefixStr, mut a_iThreadIdx, mut a_varDeclsLoc, mut a_mainThreadCode) => {
            let mut ret_1: i32;
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncODE_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_taskEqsOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_taskEqsDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_taskEqsZeroFunc.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFunc")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone(), (literal!("  #ifdef MEASURETIME_MODELFUNCTIONS\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MeasureTimeValues *measuredSchedulerStartValues = measuredSchedulerStartValues_")).clone() }))?;
            ret_0 = intSub(a_iThreadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("MeasureTimeValues *measuredSchedulerEndValues = measuredSchedulerEndValues_")).clone()], lastHasNewLine: false }))?;
            ret_1 = intSub(a_iThreadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("#endif //MEASURETIME_MODELFUNCTIONS\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), a_varDeclsLoc.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("while(1)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_assLock.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("if(_terminateThreads)\n")).clone(), (literal!("   return;\n")).clone(), (literal!("\n")).clone(), (literal!("if(_evaluateMode == 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncODE_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else if(_evaluateMode < 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_relLock.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_mainThreadCode.clone())
        },
    });
    Ok((out_txt, out_a_mainThreadCode))
}

pub(crate) fn generateThreadFunc(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_threadTasksOde: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_threadTasksDae: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_threadTasksZeroFunc: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_iType: ArcStr, mut a_iThreadIdx: i32, mut a_modelNamePrefixStr: ArcStr, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_mainThreadCode: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_mainThreadCode: Tpl::Text;
    let mut ret_6: bool;
    let mut l_relLock: Tpl::Text;
    let mut l_assLock: Tpl::Text;
    let mut l_taskEqsZeroFunc: Tpl::Text;
    let mut l_taskEqsDae: Tpl::Text;
    let mut l_taskEqsOde: Tpl::Text;
    let mut l_varDeclsLoc: Tpl::Text;
    l_varDeclsLoc = Tpl::emptyTxt.clone();
    (l_taskEqsOde, l_varDeclsLoc, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = parallelThreadCode(Tpl::emptyTxt.clone(), a_allEquationsPlusWhen.clone(), a_threadTasksOde, a_iThreadIdx, (a_iType.clone()).clone(), (literal!("_lockOde")).clone(), l_varDeclsLoc, a_simCode.clone(), a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, (literal!("evaluateODE")).clone(), a_useFlatArrayNotation)?;
    (l_taskEqsDae, l_varDeclsLoc, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = parallelThreadCode(Tpl::emptyTxt.clone(), a_allEquationsPlusWhen.clone(), a_threadTasksDae, a_iThreadIdx, (a_iType.clone()).clone(), (literal!("_lockDae")).clone(), l_varDeclsLoc, a_simCode.clone(), out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace, (literal!("evaluateAll")).clone(), a_useFlatArrayNotation)?;
    (l_taskEqsZeroFunc, l_varDeclsLoc, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = parallelThreadCode(Tpl::emptyTxt.clone(), a_allEquationsPlusWhen, a_threadTasksZeroFunc, a_iThreadIdx, (a_iType.clone()).clone(), (literal!("_lockZeroFunc")).clone(), l_varDeclsLoc, a_simCode, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace, (literal!("evaluateZeroFunc")).clone(), a_useFlatArrayNotation)?;
    l_assLock = assignLockByLockName(Tpl::emptyTxt.clone(), (intString(a_iThreadIdx)).clone(), (literal!("th_lock")).clone(), (a_iType.clone()).clone())?;
    l_relLock = releaseLockByLockName(Tpl::emptyTxt.clone(), (intString(a_iThreadIdx)).clone(), (literal!("th_lock1")).clone(), (a_iType).clone())?;
    out_a_extraFuncsDecl = Tpl::writeTok(out_a_extraFuncsDecl, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void evaluateThreadFuncODE_")).clone() }))?;
    out_a_extraFuncsDecl = Tpl::writeStr(out_a_extraFuncsDecl, (intString(a_iThreadIdx)).clone())?;
    out_a_extraFuncsDecl = Tpl::writeTok(out_a_extraFuncsDecl, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFuncAll_")).clone()], lastHasNewLine: false }))?;
    out_a_extraFuncsDecl = Tpl::writeStr(out_a_extraFuncsDecl, (intString(a_iThreadIdx)).clone())?;
    out_a_extraFuncsDecl = Tpl::writeTok(out_a_extraFuncsDecl, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFuncZeroFunc_")).clone()], lastHasNewLine: false }))?;
    out_a_extraFuncsDecl = Tpl::writeStr(out_a_extraFuncsDecl, (intString(a_iThreadIdx)).clone())?;
    out_a_extraFuncsDecl = Tpl::writeTok(out_a_extraFuncsDecl, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("();")).clone() }))?;
    ret_6 = intGt(a_iThreadIdx, 0);
    (out_txt, out_a_mainThreadCode) = fun_251(txt, ret_6, l_relLock, l_assLock, l_taskEqsZeroFunc, l_taskEqsDae, l_taskEqsOde, (a_modelNamePrefixStr).clone(), a_iThreadIdx, l_varDeclsLoc, a_mainThreadCode)?;
    out_a_varDecls = a_varDecls;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace, out_a_mainThreadCode))
}

fn fun_253(mut in_txt: Tpl::Text, mut in_a_tt: Arc<HpcOmSimCode::Task>, mut in_a_iType: ArcStr, mut in_a_iLockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_tt, in_a_iType, in_a_iLockPrefix)) {
        (txt, i_task @ Deref @ HpcOmSimCode::Task::DEPTASK { outgoing: true, .. }, a_iType, a_iLockPrefix) => {
            let mut txt = (*txt).clone();
            txt = assignLockByDepTask(txt.clone(), i_task.clone(), (a_iLockPrefix.clone()).clone(), (a_iType.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_254(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_iType: ArcStr, mut a_iLockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_254 in &*items {
        let mut lstElt_254 = lstElt_254.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_254.clone()) {
        i_tt => {
            txt = fun_253(txt.clone(), i_tt.clone(), (a_iType.clone()).clone(), (a_iLockPrefix.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_255(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("else ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn function_HPCOM_assignThreadLocks(mut txt: Tpl::Text, mut a_iThreadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_iLockPrefix: ArcStr, mut a_iThreadNum: i32, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: bool;
    let mut l_lockAssign: Tpl::Text;
    l_lockAssign = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    l_lockAssign = lm_254(l_lockAssign, a_iThreadTasks, (a_iType).clone(), (a_iLockPrefix).clone())?;
    l_lockAssign = Tpl::popIter(l_lockAssign)?;
    ret_1 = intNe(a_iThreadNum, 0);
    out_txt = fun_255(txt, ret_1)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if(threadNum == ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt, (intString(a_iThreadNum)).clone())?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt, Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeText(out_txt, l_lockAssign)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::popBlock(out_txt)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
    Ok(out_txt)
}

fn fun_257(mut in_txt: Tpl::Text, mut in_a_tt: Arc<HpcOmSimCode::Task>, mut in_a_iType: ArcStr, mut in_a_iLockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_tt, in_a_iType, in_a_iLockPrefix)) {
        (txt, i_tt @ Deref @ HpcOmSimCode::Task::DEPTASK { outgoing: false, .. }, a_iType, a_iLockPrefix) => {
            let mut txt = (*txt).clone();
            txt = releaseLockByDepTask(txt.clone(), i_tt.clone(), (a_iLockPrefix.clone()).clone(), (a_iType.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_258(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_iType: ArcStr, mut a_iLockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_258 in &*items {
        let mut lstElt_258 = lstElt_258.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_258.clone()) {
        i_tt => {
            txt = fun_257(txt.clone(), i_tt.clone(), (a_iType.clone()).clone(), (a_iLockPrefix.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_259(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("else ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn function_HPCOM_releaseThreadLocks(mut txt: Tpl::Text, mut a_iThreadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_iLockPrefix: ArcStr, mut a_iThreadNum: i32, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: bool;
    let mut l_lockAssign: Tpl::Text;
    l_lockAssign = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    l_lockAssign = lm_258(l_lockAssign, a_iThreadTasks, (a_iType).clone(), (a_iLockPrefix).clone())?;
    l_lockAssign = Tpl::popIter(l_lockAssign)?;
    ret_1 = intNe(a_iThreadNum, 0);
    out_txt = fun_259(txt, ret_1)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if(threadNum == ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt, (intString(a_iThreadNum)).clone())?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt, Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeText(out_txt, l_lockAssign)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::popBlock(out_txt)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
    Ok(out_txt)
}

fn lm_261(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut a_useFlatArrayNotation: bool, mut a_iThreadNum: i32, mut a_extraFunctionName: ArcStr, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_261 in &*items {
        let mut lstElt_261 = lstElt_261.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_261.clone()) {
        i_tt => {
            let mut x_i0: i32;
            let mut txt_0: Tpl::Text;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt_0 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_extraFunctionName.clone()).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_Th")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(a_iThreadNum)).clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = parallelThreadCode(txt.clone(), a_allEquationsPlusWhen.clone(), i_tt.clone(), x_i0, (a_iType.clone()).clone(), (a_lockPrefix.clone()).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), (Tpl::textString(txt_0.clone())?).clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn fun_262(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("switch(threadNum) ")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_263(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_264(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_iMaxThreadNumber: i32, mut in_a_functionCalls: Tpl::Text, mut in_a_iThreadNum: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_iMaxThreadNumber, in_a_functionCalls, in_a_iThreadNum)) {
        (txt, Deref @ "openmp", a_iMaxThreadNumber, a_functionCalls, a_iThreadNum) => {
            let mut ret_1: bool;
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_iThreadNum.clone(), 0);
            txt = fun_262(txt.clone(), ret_0)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadNum.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(":\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_functionCalls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("break;\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            ret_1 = intEq(a_iThreadNum.clone(), a_iMaxThreadNumber.clone());
            txt = fun_263(txt.clone(), ret_1)?;
            txt.clone()
        },
        (txt, Deref @ "mpi", _, a_functionCalls, a_iThreadNum) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (world_rank == ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadNum.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_functionCalls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, a_functionCalls, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_functionCalls.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn parallelThreadCodeWithSplit(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_threadTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_iThreadNum: i32, mut a_iMaxThreadNumber: i32, mut a_iType: ArcStr, mut a_lockPrefix: ArcStr, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFunctionName: ArcStr, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>;
    let mut l_functionCalls: Tpl::Text;
    ret_1 = List::partition(a_threadTaskList, 100)?;
    l_functionCalls = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (l_functionCalls, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_261(l_functionCalls, ret_1, a_useFlatArrayNotation, a_iThreadNum, (a_extraFunctionName).clone(), a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, (a_lockPrefix).clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen)?;
    l_functionCalls = Tpl::popIter(l_functionCalls)?;
    out_txt = fun_264(txt, (a_iType).clone(), a_iMaxThreadNumber, l_functionCalls, a_iThreadNum)?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_266(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_266 in &*items {
        let mut lstElt_266 = lstElt_266.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (::match_deref::match_deref! { match &(lstElt_266.clone()) {
        i_tt => {
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = taskCode(txt.clone(), a_allEquationsPlusWhen.clone(), i_tt.clone(), (a_iType.clone()).clone(), (a_lockPrefix.clone()).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

pub(crate) fn parallelThreadCode(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_threadTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_iPartitionIndex: i32, mut a_iType: ArcStr, mut a_lockPrefix: ArcStr, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFunctionName: ArcStr, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut l_threadTasks: Tpl::Text;
    l_threadTasks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    (l_threadTasks, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_266(l_threadTasks, a_threadTaskList, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, (a_lockPrefix).clone(), (a_iType).clone(), a_allEquationsPlusWhen)?;
    l_threadTasks = Tpl::popIter(l_threadTasks)?;
    out_a_extraFuncs = Tpl::writeTok(out_a_extraFuncs, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
    out_a_extraFuncs = Tpl::writeText(out_a_extraFuncs, out_a_extraFuncsNamespace.clone())?;
    out_a_extraFuncs = Tpl::writeTok(out_a_extraFuncs, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::")).clone() }))?;
    out_a_extraFuncs = Tpl::writeStr(out_a_extraFuncs, (a_extraFunctionName.clone()).clone())?;
    out_a_extraFuncs = Tpl::writeTok(out_a_extraFuncs, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    out_a_extraFuncs = Tpl::writeStr(out_a_extraFuncs, (intString(a_iPartitionIndex)).clone())?;
    out_a_extraFuncs = Tpl::writeTok(out_a_extraFuncs, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
    out_a_extraFuncs = Tpl::pushBlock(out_a_extraFuncs, Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_a_extraFuncs = Tpl::writeText(out_a_extraFuncs, l_threadTasks)?;
    out_a_extraFuncs = Tpl::softNewLine(out_a_extraFuncs)?;
    out_a_extraFuncs = Tpl::popBlock(out_a_extraFuncs)?;
    out_a_extraFuncs = Tpl::writeTok(out_a_extraFuncs, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
    out_a_extraFuncs = Tpl::writeTok(out_a_extraFuncs, openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
    out_a_extraFuncs = Tpl::writeTok(out_a_extraFuncs, openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
    out_a_extraFuncsDecl = Tpl::writeTok(out_a_extraFuncsDecl, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
    out_a_extraFuncsDecl = Tpl::writeStr(out_a_extraFuncsDecl, (a_extraFunctionName.clone()).clone())?;
    out_a_extraFuncsDecl = Tpl::writeTok(out_a_extraFuncsDecl, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    out_a_extraFuncsDecl = Tpl::writeStr(out_a_extraFuncsDecl, (intString(a_iPartitionIndex)).clone())?;
    out_a_extraFuncsDecl = Tpl::writeTok(out_a_extraFuncsDecl, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("();")).clone() }))?;
    out_txt = Tpl::writeStr(txt, (a_extraFunctionName).clone())?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt, (intString(a_iPartitionIndex)).clone())?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("();")).clone() }))?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_268(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_268 in &*items {
        let mut lstElt_268 = lstElt_268.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (match lstElt_268.clone() {
        mut i_eq => {
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = equationNamesHPCOM_(txt.clone(), i_eq.clone(), a_allEquationsPlusWhen.clone(), SimCodeFunction::contextSimulationNonDiscrete().clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
    });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_269(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>, mut a_useFlatArrayNotation: bool, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncs: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_varDecls: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_extraFuncsNamespace: Tpl::Text = a_extraFuncsNamespace;
    let mut a_extraFuncsDecl: Tpl::Text = a_extraFuncsDecl;
    let mut a_extraFuncs: Tpl::Text = a_extraFuncs;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_269 in &*items {
        let mut lstElt_269 = lstElt_269.clone();
        (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = (match lstElt_269.clone() {
        mut i_eq => {
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = equationNamesHPCOM_(txt.clone(), i_eq.clone(), a_allEquationsPlusWhen.clone(), SimCodeFunction::contextSimulationNonDiscrete().clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
    });
    }
    Ok((txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls))
}

fn lm_270(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_270 in &*items {
        let mut lstElt_270 = lstElt_270.clone();
        txt = (match lstElt_270.clone() {
        mut i_task => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_task.clone())).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_271(mut in_txt: Tpl::Text, mut in_a_iTask: Arc<HpcOmSimCode::Task>, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_iType: ArcStr, mut in_a_lockPrefix: ArcStr, mut in_a_varDecls: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (::match_deref::match_deref! { match &((in_txt, in_a_iTask, in_a_allEquationsPlusWhen, in_a_iType, in_a_lockPrefix, in_a_varDecls, in_a_simCode, in_a_extraFuncs, in_a_extraFuncsDecl, in_a_extraFuncsNamespace, in_a_useFlatArrayNotation)) {
        (txt, Deref @ HpcOmSimCode::Task::CALCTASK { eqIdc: i_task_eqIdc, index: i_task_index, .. }, a_allEquationsPlusWhen, _, _, a_varDecls, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut l_varDeclsLocal: Tpl::Text;
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_odeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_268(l_odeEqs, i_task_eqIdc.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs)?;
            l_varDeclsLocal = Tpl::emptyTxt.clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("// Task ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("// End Task ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { eqIdc: i_task_eqIdc, nodeIdc: i_task_nodeIdc, .. }, a_allEquationsPlusWhen, _, _, a_varDecls, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut l_taskStr: Tpl::Text;
            let mut l_varDeclsLocal: Tpl::Text;
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_odeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_269(l_odeEqs, i_task_eqIdc.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs)?;
            l_taskStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_taskStr = lm_270(l_taskStr, i_task_nodeIdc.clone())?;
            l_taskStr = Tpl::popIter(l_taskStr)?;
            l_varDeclsLocal = Tpl::emptyTxt.clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("// Tasks ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_taskStr)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs)?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, i_task @ Deref @ HpcOmSimCode::Task::DEPTASK { outgoing: false, .. }, _, a_iType, a_lockPrefix, a_varDecls, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _) => {
            let mut l_assLck: Tpl::Text;
            let mut txt = (*txt).clone();
            l_assLck = assignLockByDepTask(Tpl::emptyTxt.clone(), i_task.clone(), (a_lockPrefix.clone()).clone(), (a_iType.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_assLck)?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, i_task @ Deref @ HpcOmSimCode::Task::DEPTASK { outgoing: true, .. }, _, a_iType, a_lockPrefix, a_varDecls, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _) => {
            let mut l_relLck: Tpl::Text;
            let mut txt = (*txt).clone();
            l_relLck = releaseLockByDepTask(Tpl::emptyTxt.clone(), i_task.clone(), (a_lockPrefix.clone()).clone(), (a_iType.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_relLck)?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, _, _, _, _, a_varDecls, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _) => {
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub(crate) fn taskCode(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_iTask: Arc<HpcOmSimCode::Task>, mut a_iType: ArcStr, mut a_lockPrefix: ArcStr, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = fun_271(txt, a_iTask, a_allEquationsPlusWhen, (a_iType).clone(), (a_lockPrefix).clone(), a_varDecls, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation)?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub(crate) fn equationNamesHPCOM_(mut txt: Tpl::Text, mut a_idx: i32, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut ret_1: Arc<SimCode::SimEqSystem>;
    let mut l_eq: Tpl::Text;
    ret_1 = HpcOmTaskGraph::getSimCodeEqByIndex(a_allEquationsPlusWhen, a_idx)?;
    (l_eq, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = equationHPCOM_(Tpl::emptyTxt.clone(), ret_1, a_idx, a_context, a_varDecls, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation)?;
    out_txt = Tpl::writeText(txt, l_eq)?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub(crate) fn equationHPCOM_(mut txt: Tpl::Text, mut a_eq: Arc<SimCode::SimEqSystem>, mut a_idx: i32, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, _) = CodegenCpp::equation_function_call(txt, a_eq, a_context, a_simCode, Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluate")).clone() })))?;
    out_a_varDecls = a_varDecls;
    out_a_extraFuncs = a_extraFuncs;
    out_a_extraFuncsDecl = a_extraFuncsDecl;
    out_a_extraFuncsNamespace = a_extraFuncsNamespace;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_275(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_threadIdx: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_threadIdx)) {
        (txt, Deref @ "openmp", _) => {
            txt.clone()
        },
        (txt, _, a_threadIdx) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThread")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_threadIdx.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("->join();")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn function_HPCOM_joinThread(mut txt: Tpl::Text, mut a_threadIdx: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_275(txt, (a_iType).clone(), (a_threadIdx).clone())?;
    Ok(out_txt)
}

fn fun_277(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_threadIdx: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_threadIdx)) {
        (txt, Deref @ "openmp", _) => {
            txt.clone()
        },
        (txt, _, a_threadIdx) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("delete evaluateThread")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_threadIdx.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn function_HPCOM_destroyThread(mut txt: Tpl::Text, mut a_threadIdx: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_277(txt, (a_iType).clone(), (a_threadIdx).clone())?;
    Ok(out_txt)
}

fn fun_279(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_threadIdx: i32, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_funcName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_threadIdx, in_a_modelNamePrefixStr, in_a_funcName)) {
        (txt, Deref @ "openmp", _, _, _) => {
            txt.clone()
        },
        (txt, _, a_threadIdx, a_modelNamePrefixStr, a_funcName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThread")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_threadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = new thread(bind(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcName.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_threadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", this));")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn generateThread(mut txt: Tpl::Text, mut a_threadIdx: i32, mut a_iType: ArcStr, mut a_modelNamePrefixStr: ArcStr, mut a_funcName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_279(txt, (a_iType).clone(), a_threadIdx, (a_modelNamePrefixStr).clone(), (a_funcName).clone())?;
    Ok(out_txt)
}

pub(crate) fn getLockNameByDepTask(mut in_txt: Tpl::Text, mut in_a_depTask: Arc<HpcOmSimCode::Task>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_depTask)) {
        (txt, Deref @ HpcOmSimCode::Task::DEPTASK { id: i_task_id, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_id.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("invalidLockTask")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn initializeLockByDepTask(mut txt: Tpl::Text, mut a_depTask: Arc<HpcOmSimCode::Task>, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_lockName: Tpl::Text;
    l_lockName = getLockNameByDepTask(Tpl::emptyTxt.clone(), a_depTask)?;
    out_txt = initializeLockByLockName(txt, (Tpl::textString(l_lockName)?).clone(), (a_lockPrefix).clone(), (a_iType).clone())?;
    Ok(out_txt)
}

fn fun_283(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_lockName: ArcStr, mut in_a_lockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_lockName, in_a_lockPrefix)) {
        (txt, Deref @ "openmp", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_init_lock(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = new alignedLock();")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = new alignedSpinlock();")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn initializeLockByLockName(mut txt: Tpl::Text, mut a_lockName: ArcStr, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_283(txt, (a_iType).clone(), (a_lockName).clone(), (a_lockPrefix).clone())?;
    Ok(out_txt)
}

fn fun_285(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_lockName: ArcStr, mut in_a_lockPrefix: ArcStr, mut in_a_numberOfThreads: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_lockName, in_a_lockPrefix, in_a_numberOfThreads)) {
        (txt, Deref @ "pthreads", a_lockName, a_lockPrefix, a_numberOfThreads) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numberOfThreads.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_lockName, a_lockPrefix, a_numberOfThreads) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numberOfThreads.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn initializeBarrierByName(mut txt: Tpl::Text, mut a_lockName: ArcStr, mut a_lockPrefix: ArcStr, mut a_numberOfThreads: i32, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_285(txt, (a_iType).clone(), (a_lockName).clone(), (a_lockPrefix).clone(), a_numberOfThreads)?;
    Ok(out_txt)
}

pub(crate) fn createLockByDepTask(mut txt: Tpl::Text, mut a_depTask: Arc<HpcOmSimCode::Task>, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_lockName: Tpl::Text;
    l_lockName = getLockNameByDepTask(Tpl::emptyTxt.clone(), a_depTask)?;
    out_txt = createLockByLockName(txt, (Tpl::textString(l_lockName)?).clone(), (a_lockPrefix).clone(), (a_iType).clone())?;
    Ok(out_txt)
}

fn fun_288(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_lockName: ArcStr, mut in_a_lockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_lockName, in_a_lockPrefix)) {
        (txt, Deref @ "openmp", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_lock_t ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("alignedLock* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("alignedSpinlock* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn createLockByLockName(mut txt: Tpl::Text, mut a_lockName: ArcStr, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_288(txt, (a_iType).clone(), (a_lockName).clone(), (a_lockPrefix).clone())?;
    Ok(out_txt)
}

fn fun_290(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_lockName: ArcStr, mut in_a_lockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_lockName, in_a_lockPrefix)) {
        (txt, Deref @ "pthreads", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("busywaiting_barrier ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("busywaiting_barrier ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn createBarrierByName(mut txt: Tpl::Text, mut a_lockName: ArcStr, mut a_lockPrefix: ArcStr, mut a_numOfThreads: i32, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_290(txt, (a_iType).clone(), (a_lockName).clone(), (a_lockPrefix).clone())?;
    Ok(out_txt)
}

pub(crate) fn destroyLockByDepTask(mut txt: Tpl::Text, mut a_depTask: Arc<HpcOmSimCode::Task>, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_lockName: Tpl::Text;
    l_lockName = getLockNameByDepTask(Tpl::emptyTxt.clone(), a_depTask)?;
    out_txt = destroyLockByLockName(txt, (Tpl::textString(l_lockName)?).clone(), (a_lockPrefix).clone(), (a_iType).clone())?;
    Ok(out_txt)
}

fn fun_293(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_lockName: ArcStr, mut in_a_lockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_lockName, in_a_lockPrefix)) {
        (txt, Deref @ "openmp", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_destroy_lock(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("delete ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("delete ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn destroyLockByLockName(mut txt: Tpl::Text, mut a_lockName: ArcStr, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_293(txt, (a_iType).clone(), (a_lockName).clone(), (a_lockPrefix).clone())?;
    Ok(out_txt)
}

pub(crate) fn assignLockByDepTask(mut in_txt: Tpl::Text, mut in_a_depTask: Arc<HpcOmSimCode::Task>, mut in_a_lockPrefix: ArcStr, mut in_a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_depTask, in_a_lockPrefix, in_a_iType)) {
        (txt, i_depTask @ Deref @ HpcOmSimCode::Task::DEPTASK { sourceTask: _, .. }, a_lockPrefix, a_iType) => {
            let mut l_lockName: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lockName = getLockNameByDepTask(Tpl::emptyTxt.clone(), i_depTask.clone())?;
            txt = assignLockByLockName(txt.clone(), (Tpl::textString(l_lockName)?).clone(), (a_lockPrefix.clone()).clone(), (a_iType.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn printCommunicationInfoVariables(mut txt: Tpl::Text, mut a_commInfo: HpcOmSimCode::CommunicationInfo) -> Tpl::Text {
    let mut out_txt: Tpl::Text;
    out_txt = txt;
    out_txt
}

fn fun_297(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_lockName: ArcStr, mut in_a_lockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_lockName, in_a_lockPrefix)) {
        (txt, Deref @ "openmp", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_set_lock(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("->lock();")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("->lock();")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn assignLockByLockName(mut txt: Tpl::Text, mut a_lockName: ArcStr, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_297(txt, (a_iType).clone(), (a_lockName).clone(), (a_lockPrefix).clone())?;
    Ok(out_txt)
}

pub(crate) fn releaseLockByDepTask(mut txt: Tpl::Text, mut a_depTask: Arc<HpcOmSimCode::Task>, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_lockName: Tpl::Text;
    l_lockName = getLockNameByDepTask(Tpl::emptyTxt.clone(), a_depTask)?;
    out_txt = releaseLockByLockName(txt, (Tpl::textString(l_lockName)?).clone(), (a_lockPrefix).clone(), (a_iType).clone())?;
    Ok(out_txt)
}

fn fun_300(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_lockName: ArcStr, mut in_a_lockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iType, in_a_lockName, in_a_lockPrefix)) {
        (txt, Deref @ "openmp", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_unset_lock(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("->unlock();")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("->unlock();")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn releaseLockByLockName(mut txt: Tpl::Text, mut a_lockName: ArcStr, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_300(txt, (a_iType).clone(), (a_lockName).clone(), (a_lockPrefix).clone())?;
    Ok(out_txt)
}

pub(crate) fn mpiFinalize(mut txt: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("} // End sequential\n")).clone(), (literal!("MPI_Finalize();")).clone()], lastHasNewLine: false }))?;
    Ok(out_txt)
}

pub(crate) fn mpiInit(mut txt: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("char** argvNotConst = const_cast<char**>(argv);\n")).clone(), (literal!("MPI_Init(&argc, &argvNotConst);\n")).clone(), (literal!("int world_rank, world_size;\n")).clone(), (literal!("MPI_Comm_size(MPI_COMM_WORLD, &world_size);\n")).clone(), (literal!("MPI_Comm_rank(MPI_COMM_WORLD, &world_rank);\n")).clone(), (literal!("std::cout << \"Hello world! This is MPI process \" << world_rank\n")).clone(), (literal!("          << \" of \" << world_size << \" processes.\"  << endl;\n")).clone(), (literal!("\n")).clone(), (literal!("// Run simulation in sequential\n")).clone(), (literal!("if (0 == world_rank) {\n")).clone(), (literal!("  std::cout << \"Remark: Simulation is not (yet) MPI parallel!\\n\";")).clone()], lastHasNewLine: false }))?;
    Ok(out_txt)
}

pub(crate) fn mpiRunCommandInRunScript(mut in_txt: Tpl::Text, mut in_a_type: ArcStr, mut in_a_getNumOfProcs: Tpl::Text, mut in_a_execCommandLinux: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_getNumOfProcs: Tpl::Text;
    let mut out_a_execCommandLinux: Tpl::Text;
    (out_txt, out_a_getNumOfProcs, out_a_execCommandLinux) = (::match_deref::match_deref! { match &((in_txt, in_a_type, in_a_getNumOfProcs, in_a_execCommandLinux)) {
        (txt, Deref @ "mpi", a_getNumOfProcs, a_execCommandLinux) => {
            let mut a_getNumOfProcs = (*a_getNumOfProcs).clone();
            let mut a_execCommandLinux = (*a_execCommandLinux).clone();
            a_execCommandLinux = Tpl::writeTok(a_execCommandLinux.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mpirun -np ${NPROCESSORS}")).clone() }))?;
            a_getNumOfProcs = Tpl::writeTok(a_getNumOfProcs.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("if [ $# -gt 0 ]; then\n")).clone(), (literal!("  NPROCESSORS=$1\n")).clone(), (literal!(" shift \n")).clone(), (literal!("else\n")).clone(), (literal!("  NPROCESSORS=1\n")).clone(), (literal!("fi\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            (txt.clone(), a_getNumOfProcs.clone(), a_execCommandLinux.clone())
        },
        (txt, _, a_getNumOfProcs, a_execCommandLinux) => {
            let mut a_execCommandLinux = (*a_execCommandLinux).clone();
            a_execCommandLinux = Tpl::writeTok(a_execCommandLinux.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("exec")).clone() }))?;
            (txt.clone(), a_getNumOfProcs.clone(), a_execCommandLinux.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_getNumOfProcs, out_a_execCommandLinux))
}

fn fun_305(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mpi")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn simulationMainRunScript(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut l_preRunCommandWindows: Tpl::Text;
    let mut l_0__: Tpl::Text;
    let mut l_execCommandLinux: Tpl::Text;
    let mut l_preRunCommandLinux: Tpl::Text;
    let mut ret_1: bool;
    let mut l_type: Tpl::Text;
    ret_1 = Flags::isSet(Flags::USEMPI.clone())?;
    l_type = fun_305(Tpl::emptyTxt.clone(), ret_1)?;
    l_preRunCommandLinux = Tpl::emptyTxt.clone();
    l_execCommandLinux = Tpl::emptyTxt.clone();
    (l_0__, l_preRunCommandLinux, l_execCommandLinux) = mpiRunCommandInRunScript(Tpl::emptyTxt.clone(), (Tpl::textString(l_type)?).clone(), l_preRunCommandLinux, l_execCommandLinux)?;
    l_preRunCommandWindows = Tpl::emptyTxt.clone();
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = CodegenCpp::simulationMainRunScript(txt, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, (Tpl::textString(l_preRunCommandLinux)?).clone(), (Tpl::textString(l_preRunCommandWindows)?).clone(), (Tpl::textString(l_execCommandLinux)?).clone())?;
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_307(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -fopenmp")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_308(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -I\"$(INTEL_TBB_INCLUDE)\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_309(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/openmp")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_310(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-L$(INTEL_TBB_LIBS) $(INTEL_TBB_LIBRARIES) ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_311(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -fopenmp")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn getAdditionalMakefileFlags(mut txt: Tpl::Text, mut a_additionalLinkerFlags__GCC: Tpl::Text, mut a_additionalLinkerFlags__MSVC: Tpl::Text, mut a_additionalCFlags__GCC: Tpl::Text, mut a_additionalCFlags__MSVC: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_additionalLinkerFlags__GCC: Tpl::Text;
    let mut out_a_additionalLinkerFlags__MSVC: Tpl::Text;
    let mut out_a_additionalCFlags__GCC: Tpl::Text;
    let mut out_a_additionalCFlags__MSVC: Tpl::Text;
    let mut ret_6: bool;
    let mut ret_5: bool;
    let mut ret_4: bool;
    let mut ret_3: bool;
    let mut ret_2: bool;
    let mut ret_1: ArcStr;
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1).clone())?;
    ret_2 = stringEq((Tpl::textString(l_type.clone())?).clone(), (literal!("openmp")).clone());
    out_a_additionalCFlags__GCC = fun_307(a_additionalCFlags__GCC, ret_2)?;
    ret_3 = stringEq((Tpl::textString(l_type.clone())?).clone(), (literal!("tbb")).clone());
    out_a_additionalCFlags__GCC = fun_308(out_a_additionalCFlags__GCC, ret_3)?;
    ret_4 = stringEq((Tpl::textString(l_type.clone())?).clone(), (literal!("openmp")).clone());
    out_a_additionalCFlags__MSVC = fun_309(a_additionalCFlags__MSVC, ret_4)?;
    ret_5 = stringEq((Tpl::textString(l_type.clone())?).clone(), (literal!("tbb")).clone());
    out_a_additionalLinkerFlags__GCC = fun_310(a_additionalLinkerFlags__GCC, ret_5)?;
    ret_6 = stringEq((Tpl::textString(l_type)?).clone(), (literal!("openmp")).clone());
    out_a_additionalLinkerFlags__GCC = fun_311(out_a_additionalLinkerFlags__GCC, ret_6)?;
    out_txt = txt;
    out_a_additionalLinkerFlags__MSVC = a_additionalLinkerFlags__MSVC;
    Ok((out_txt, out_a_additionalLinkerFlags__GCC, out_a_additionalLinkerFlags__MSVC, out_a_additionalCFlags__GCC, out_a_additionalCFlags__MSVC))
}

pub(crate) fn simulationMakefile(mut txt: Tpl::Text, mut a_target: ArcStr, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut ret_4: bool;
    let mut l_additionalLinkerFlags__MSVC: Tpl::Text;
    let mut l_additionalLinkerFlags__GCC: Tpl::Text;
    let mut l_additionalCFlags__MSVC: Tpl::Text;
    let mut l_additionalCFlags__GCC: Tpl::Text;
    l_additionalCFlags__GCC = Tpl::emptyTxt.clone();
    l_additionalCFlags__MSVC = Tpl::emptyTxt.clone();
    l_additionalLinkerFlags__GCC = Tpl::emptyTxt.clone();
    l_additionalLinkerFlags__MSVC = Tpl::emptyTxt.clone();
    (out_txt, l_additionalLinkerFlags__GCC, l_additionalLinkerFlags__MSVC, l_additionalCFlags__GCC, l_additionalCFlags__MSVC) = getAdditionalMakefileFlags(txt, l_additionalLinkerFlags__GCC, l_additionalLinkerFlags__MSVC, l_additionalCFlags__GCC, l_additionalCFlags__MSVC)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    ret_4 = Flags::isSet(Flags::USEMPI.clone())?;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = CodegenCpp::simulationMakefile(out_txt, (a_target).clone(), a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, (Tpl::textString(l_additionalLinkerFlags__GCC)?).clone(), (Tpl::textString(l_additionalLinkerFlags__MSVC)?).clone(), (Tpl::textString(l_additionalCFlags__GCC)?).clone(), (Tpl::textString(l_additionalCFlags__MSVC)?).clone(), ret_4)?;
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_314(mut in_txt: Tpl::Text, mut in_a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_hpcOmMemoryOpt, in_a_modelInfo) {
        (mut txt, Some(HpcOmSimCode::MemoryMap::MEMORYMAP_ARRAY { floatArraySize: mut i_floatArraySize, intArraySize: mut i_intArraySize, boolArraySize: mut i_boolArraySize, .. }), _) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_floatArraySize.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" + ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_intArraySize.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" + ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_boolArraySize.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_modelInfo) => {
            txt = CodegenCpp::getPreVarsCount(txt.clone(), a_modelInfo.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn numPreVarsHpcom(mut txt: Tpl::Text, mut a_modelInfo: SimCode::ModelInfo, mut a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_314(txt, a_hpcOmMemoryOpt, a_modelInfo)?;
    Ok(out_txt)
}

fn fun_316(mut in_txt: Tpl::Text, mut in_a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_hpcOmMemoryOpt, in_a_modelInfo) {
        (mut txt, Some(HpcOmSimCode::MemoryMap::MEMORYMAP_ARRAY { floatArraySize: mut i_floatArraySize, .. }), _) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_floatArraySize.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_modelInfo) => {
            txt = CodegenCpp::numRealvars(txt.clone(), a_modelInfo.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn numRealvarsHpcom(mut txt: Tpl::Text, mut a_modelInfo: SimCode::ModelInfo, mut a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_316(txt, a_hpcOmMemoryOpt, a_modelInfo)?;
    Ok(out_txt)
}

fn fun_318(mut in_txt: Tpl::Text, mut in_a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_hpcOmMemoryOpt, in_a_modelInfo) {
        (mut txt, Some(HpcOmSimCode::MemoryMap::MEMORYMAP_ARRAY { intArraySize: mut i_intArraySize, .. }), _) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_intArraySize.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_modelInfo) => {
            txt = CodegenCpp::numIntvars(txt.clone(), a_modelInfo.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn numIntvarsHpcom(mut txt: Tpl::Text, mut a_modelInfo: SimCode::ModelInfo, mut a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_318(txt, a_hpcOmMemoryOpt, a_modelInfo)?;
    Ok(out_txt)
}

fn fun_320(mut in_txt: Tpl::Text, mut in_a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_hpcOmMemoryOpt, in_a_modelInfo) {
        (mut txt, Some(HpcOmSimCode::MemoryMap::MEMORYMAP_ARRAY { boolArraySize: mut i_boolArraySize, .. }), _) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_boolArraySize.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_modelInfo) => {
            txt = CodegenCpp::numBoolvars(txt.clone(), a_modelInfo.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn numBoolvarsHpcom(mut txt: Tpl::Text, mut a_modelInfo: SimCode::ModelInfo, mut a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_320(txt, a_hpcOmMemoryOpt, a_modelInfo)?;
    Ok(out_txt)
}

fn fun_322(mut in_txt: Tpl::Text, mut in_a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_hpcOmMemoryOpt, in_a_modelInfo) {
        (mut txt, Some(HpcOmSimCode::MemoryMap::MEMORYMAP_ARRAY { stringArraySize: mut i_stringArraySize, .. }), _) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_stringArraySize.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_modelInfo) => {
            txt = CodegenCpp::numStringvars(txt.clone(), a_modelInfo.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn numStringvarsHpcom(mut txt: Tpl::Text, mut a_modelInfo: SimCode::ModelInfo, mut a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_322(txt, a_hpcOmMemoryOpt, a_modelInfo)?;
    Ok(out_txt)
}

