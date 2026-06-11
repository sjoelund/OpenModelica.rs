// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use openmodelica_backend::CodegenUtil;
use openmodelica_backend_types::BackendDAE;
use openmodelica_codegen::CodegenUtilSimulation;
use openmodelica_codegen_c::CodegenC;
use openmodelica_codegen_cfunctions::CodegenCFunctions;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_tpl::Tpl;
use openmodelica_util::Error;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub(crate) fn equationFunctionPrototypes(mut txt: Tpl::Text, mut a_eq: Arc<SimCode::SimEqSystem>, mut a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_ix: Tpl::Text;
    l_ix = CodegenUtilSimulation::equationIndex(Tpl::emptyTxt.clone(), a_eq)?;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
    out_txt = CodegenUtil::symbolName(out_txt, (a_modelNamePrefixStr).clone(), (literal!("eqFunction")).clone())?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    out_txt = Tpl::writeText(out_txt, l_ix)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(Data_Struct_something* data, Data_Struct_something* threadData);")).clone() }))?;
    out_txt = Tpl::writeTok(out_txt, openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
    Ok(out_txt)
}

fn fun_52(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_eq)) {
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("resFunction")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("algSystFunction")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("eqFunction")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_53(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_eq)) {
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("struct omsi_function_t* this_function, const omsi_values* model_vars_and_params, void* data")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("struct omsi_function_t* this_function, const omsi_values* model_vars_and_params")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("struct omsi_function_t* this_function, const omsi_values* model_vars_and_params")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_54(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_eq)) {
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omsi_real* res = (omsi_real*) data;")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_55(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_varDecls: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_varDecls) {
        (mut txt, false, mut a_varDecls) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("/* Variables */\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_varDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn generateEquationFunction(mut txt: Tpl::Text, mut a_eq: Arc<SimCode::SimEqSystem>, mut a_modelNamePrefixStr: ArcStr, mut a_modelFunctionnamePrefixStr: ArcStr, mut a_context: SimCodeFunction::Context, mut a_functionPrototypes: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut ret_9: bool;
    let mut l_dataCast: Tpl::Text;
    let mut l_funcArguments: Tpl::Text;
    let mut l_funcName: Tpl::Text;
    let mut l_equationCode: Tpl::Text;
    let mut l_auxFunction: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    let mut ret_2: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
    let mut l_equationInfos: Tpl::Text;
    let mut l_ix: Tpl::Text;
    l_ix = CodegenUtilSimulation::equationIndex(Tpl::emptyTxt.clone(), a_eq.clone())?;
    ret_2 = List::fill(a_eq.clone(), 1);
    l_equationInfos = CodegenUtilSimulation::dumpEqs(Tpl::emptyTxt.clone(), ret_2)?;
    l_varDecls = Tpl::emptyTxt.clone();
    l_auxFunction = Tpl::emptyTxt.clone();
    (l_equationCode, l_varDecls, l_auxFunction) = equationCStr(Tpl::emptyTxt.clone(), a_eq.clone(), l_varDecls, l_auxFunction, a_context)?;
    l_funcName = fun_52(Tpl::emptyTxt.clone(), a_eq.clone())?;
    l_funcArguments = fun_53(Tpl::emptyTxt.clone(), a_eq.clone())?;
    l_dataCast = fun_54(Tpl::emptyTxt.clone(), a_eq)?;
    out_a_functionPrototypes = Tpl::writeTok(a_functionPrototypes, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
    out_a_functionPrototypes = CodegenUtil::symbolName(out_a_functionPrototypes, (a_modelNamePrefixStr.clone()).clone(), (Tpl::textString(l_funcName.clone())?).clone())?;
    out_a_functionPrototypes = Tpl::writeTok(out_a_functionPrototypes, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    out_a_functionPrototypes = Tpl::writeText(out_a_functionPrototypes, l_ix.clone())?;
    out_a_functionPrototypes = Tpl::writeTok(out_a_functionPrototypes, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
    out_a_functionPrototypes = Tpl::writeText(out_a_functionPrototypes, l_funcArguments.clone())?;
    out_a_functionPrototypes = Tpl::writeTok(out_a_functionPrototypes, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
    out_a_functionPrototypes = Tpl::writeTok(out_a_functionPrototypes, openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("/*\n")).clone() }))?;
    out_txt = Tpl::writeText(out_txt, l_equationInfos)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("*/\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt, Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
    out_txt = CodegenUtil::symbolName(out_txt, (a_modelNamePrefixStr).clone(), (Tpl::textString(l_funcName)?).clone())?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    out_txt = Tpl::writeText(out_txt, l_ix)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
    out_txt = Tpl::writeText(out_txt, l_funcArguments)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("){\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt, Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
    out_txt = Tpl::writeText(out_txt, l_dataCast)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeTok(out_txt, openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
    ret_9 = stringEq((Tpl::textString(l_varDecls.clone())?).clone(), (literal!("")).clone());
    out_txt = fun_55(out_txt, ret_9, l_varDecls)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeText(out_txt, l_auxFunction)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeText(out_txt, l_equationCode)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::popBlock(out_txt)?;
    out_txt = Tpl::popBlock(out_txt)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
    out_txt = Tpl::writeTok(out_txt, openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
    Ok((out_txt, out_a_functionPrototypes))
}

fn fun_57(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_auxFunction: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_auxFunction: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_auxFunction, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt, in_a_eq, in_a_auxFunction, in_a_varDecls, in_a_preExp, in_a_context)) {
        (txt, Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { cref: i_cref, exp: i_exp, .. }, a_auxFunction, a_varDecls, a_preExp, a_context) => {
            let mut l_expPart: Tpl::Text;
            let mut l_crefStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            l_crefStr = CodegenCFunctions::crefOMSI(Tpl::emptyTxt.clone(), i_cref.clone(), a_context.clone())?;
            (l_expPart, a_preExp, a_varDecls, a_auxFunction) = CodegenCFunctions::daeExp(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone(), a_auxFunction.clone())?;
            txt = Tpl::writeText(txt.clone(), a_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_crefStr)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { exp: i_exp, .. }, a_auxFunction, a_varDecls, a_preExp, a_context) => {
            let mut l_expPart: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_expPart, a_preExp, a_varDecls, a_auxFunction) = CodegenCFunctions::daeExp(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone(), a_auxFunction.clone())?;
            txt = Tpl::writeText(txt.clone(), a_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*res = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, i_eq @ Deref @ SimCode::SimEqSystem::SES_WHEN { index: _, .. }, a_auxFunction, a_varDecls, a_preExp, a_context) => {
            let mut l_whenEq: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_whenEq, a_varDecls, a_auxFunction) = equationWhen(Tpl::emptyTxt.clone(), i_eq.clone(), a_context.clone(), a_varDecls.clone(), a_auxFunction.clone())?;
            txt = Tpl::writeText(txt.clone(), l_whenEq)?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_auxFunction, a_varDecls, a_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NOT IMPLEMENTED YET Error in function equationCStr in template CodegenOMSIC_Equations")).clone() }))?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_auxFunction, out_a_varDecls, out_a_preExp))
}

pub(crate) fn equationCStr(mut txt: Tpl::Text, mut a_eq: Arc<SimCode::SimEqSystem>, mut a_varDecls: Tpl::Text, mut a_auxFunction: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_auxFunction: Tpl::Text;
    let mut l_preExp: Tpl::Text;
    l_preExp = Tpl::emptyTxt.clone();
    (out_txt, out_a_auxFunction, out_a_varDecls, l_preExp) = fun_57(txt, a_eq, a_auxFunction, a_varDecls, l_preExp, a_context)?;
    Ok((out_txt, out_a_varDecls, out_a_auxFunction))
}

fn fun_59(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_input: ArcStr, mut in_a_omsiName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_eq, in_a_modelNamePrefixStr, in_a_input, in_a_omsiName)) {
        (txt, Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: i_index, .. }, a_modelNamePrefixStr, a_input, _) => {
            let mut l_i: Tpl::Text;
            let mut txt = (*txt).clone();
            l_i = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(i_index.clone())).clone())?;
            txt = CodegenUtil::symbolName(txt.clone(), (a_modelNamePrefixStr.clone()).clone(), (literal!("eqFunction")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_i)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_input.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { index: i_index, .. }, a_modelNamePrefixStr, a_input, _) => {
            let mut l_i: Tpl::Text;
            let mut txt = (*txt).clone();
            l_i = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(i_index.clone())).clone())?;
            txt = CodegenUtil::symbolName(txt.clone(), (a_modelNamePrefixStr.clone()).clone(), (literal!("eqFunction")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_i)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_input.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: i_index, .. }, a_modelNamePrefixStr, a_input, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::symbolName(txt.clone(), (a_modelNamePrefixStr.clone()).clone(), (literal!("resFunction")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_input.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { algSysIndex: i_algSysIndex, .. }, a_modelNamePrefixStr, a_input, a_omsiName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("new_status = ")).clone() }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (a_modelNamePrefixStr.clone()).clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_algSystFunction_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_input.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(");\n")).clone(), (literal!("status = (new_status==omsi_ok && status==omsi_ok) ? omsi_ok:new_status;")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* equationCall not implemented yet */")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn equationCall(mut txt: Tpl::Text, mut a_eq: Arc<SimCode::SimEqSystem>, mut a_modelNamePrefixStr: ArcStr, mut a_modelFunctionnamePrefixStr: ArcStr, mut a_input: ArcStr, mut a_omsiName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_59(txt, a_eq, (a_modelNamePrefixStr).clone(), (a_input).clone(), (a_omsiName).clone())?;
    Ok(out_txt)
}

fn lm_61(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::OMSIFunction>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_61 in &*items {
        let mut lstElt_61 = lstElt_61.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_61.clone()) {
        i_col => {
            let mut l_columnsString: Tpl::Text;
            l_columnsString = generateMatrixColumnInitialization(Tpl::emptyTxt.clone(), i_col.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_62(mut in_txt: Tpl::Text, mut in_a_matrix: Option<Arc<SimCode::DerivativeMatrix>>, mut in_a_columnsString: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_matrix, in_a_columnsString)) {
        (txt, Some(Deref @ SimCode::DerivativeMatrix { columns: i_m_columns, .. }), a_columnsString) => {
            let mut l_0__: Tpl::Text;
            let mut txt = (*txt).clone();
            l_0__ = lm_61(Tpl::emptyTxt.clone(), i_m_columns.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_columnsString.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn generateMatrixInitialization(mut txt: Tpl::Text, mut a_matrix: Option<Arc<SimCode::DerivativeMatrix>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_columnsString: Tpl::Text;
    l_columnsString = Tpl::emptyTxt.clone();
    out_txt = fun_62(txt, a_matrix, l_columnsString)?;
    Ok(out_txt)
}

fn lm_64(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_omsiFunction_context: SimCodeFunction::Context, mut a_auxFunction: Tpl::Text, mut a_varDecls: Tpl::Text, mut a_body: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_auxFunction: Tpl::Text = a_auxFunction;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    let mut a_body: Tpl::Text = a_body;
    for mut lstElt_64 in &*items {
        let mut lstElt_64 = lstElt_64.clone();
        (txt, a_auxFunction, a_varDecls, a_body) = (::match_deref::match_deref! { match &(lstElt_64.clone()) {
        i_eq => {
            (a_body, a_varDecls, a_auxFunction) = equationCStr(a_body.clone(), i_eq.clone(), a_varDecls.clone(), a_auxFunction.clone(), a_omsiFunction_context.clone())?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_body.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_auxFunction, a_varDecls, a_body))
}

fn fun_65(mut in_txt: Tpl::Text, mut in_a_column: Arc<SimCode::OMSIFunction>, mut in_a_auxFunction: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_body: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_auxFunction: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_body: Tpl::Text;
    (out_txt, out_a_auxFunction, out_a_varDecls, out_a_body) = (::match_deref::match_deref! { match &((in_txt, in_a_column, in_a_auxFunction, in_a_varDecls, in_a_body)) {
        (txt, Deref @ SimCode::OMSIFunction { equations: i_equations, context: i_omsiFunction_context, .. }, a_auxFunction, a_varDecls, a_body) => {
            let mut l_0__: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_body = (*a_body).clone();
            (l_0__, a_auxFunction, a_varDecls, a_body) = lm_64(Tpl::emptyTxt.clone(), i_equations.clone(), i_omsiFunction_context.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_body.clone())?;
            txt = Tpl::writeText(txt.clone(), a_body.clone())?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_body.clone())
        },
        (txt, _, a_auxFunction, a_varDecls, a_body) => {
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_body.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_auxFunction, out_a_varDecls, out_a_body))
}

pub(crate) fn generateMatrixColumnInitialization(mut txt: Tpl::Text, mut a_column: Arc<SimCode::OMSIFunction>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_body: Tpl::Text;
    let mut l_auxFunction: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    l_varDecls = Tpl::emptyTxt.clone();
    l_auxFunction = Tpl::emptyTxt.clone();
    l_body = Tpl::emptyTxt.clone();
    (out_txt, l_auxFunction, l_varDecls, l_body) = fun_65(txt, a_column, l_auxFunction, l_varDecls, l_body)?;
    Ok(out_txt)
}

fn lm_67(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::OMSIFunction>>>, mut a_omsiName: ArcStr, mut a_functionPrototypes: Tpl::Text, mut a_index: ArcStr, mut a_modelName: ArcStr) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_functionPrototypes: Tpl::Text = a_functionPrototypes;
    for mut lstElt_67 in &*items {
        let mut lstElt_67 = lstElt_67.clone();
        (txt, a_functionPrototypes) = (::match_deref::match_deref! { match &(lstElt_67.clone()) {
        i_col => {
            (txt, a_functionPrototypes) = generateDereivativeMatrixColumnFunction(txt.clone(), i_col.clone(), (a_modelName.clone()).clone(), (a_index.clone()).clone(), a_functionPrototypes.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            (txt, a_functionPrototypes) = generateDereivativeMatrixColumnCall(txt.clone(), i_col.clone(), (a_modelName.clone()).clone(), (a_index.clone()).clone(), a_functionPrototypes.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_functionPrototypes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_functionPrototypes))
}

fn fun_68(mut in_txt: Tpl::Text, mut in_a_matrix: Option<Arc<SimCode::DerivativeMatrix>>, mut in_a_omsiName: ArcStr, mut in_a_functionPrototypes: Tpl::Text, mut in_a_index: ArcStr, mut in_a_modelName: ArcStr) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    (out_txt, out_a_functionPrototypes) = (::match_deref::match_deref! { match &((in_txt, in_a_matrix, in_a_omsiName, in_a_functionPrototypes, in_a_index, in_a_modelName)) {
        (txt, Some(Deref @ SimCode::DerivativeMatrix { columns: i_m_columns, .. }), a_omsiName, a_functionPrototypes, a_index, a_modelName) => {
            let mut l_columnsString: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            l_columnsString = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_columnsString, a_functionPrototypes) = lm_67(l_columnsString, i_m_columns.clone(), (a_omsiName.clone()).clone(), a_functionPrototypes.clone(), (a_index.clone()).clone(), (a_modelName.clone()).clone())?;
            l_columnsString = Tpl::popIter(l_columnsString)?;
            txt = Tpl::writeText(txt.clone(), l_columnsString)?;
            (txt.clone(), a_functionPrototypes.clone())
        },
        (txt, _, _, a_functionPrototypes, _, _) => {
            (txt.clone(), a_functionPrototypes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_functionPrototypes))
}

pub(crate) fn generateDerivativeMatrix(mut txt: Tpl::Text, mut a_matrix: Option<Arc<SimCode::DerivativeMatrix>>, mut a_modelName: ArcStr, mut a_index: ArcStr, mut a_functionPrototypes: Tpl::Text, mut a_omsiName: ArcStr) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut l_columnsString: Tpl::Text;
    l_columnsString = Tpl::emptyTxt.clone();
    (out_txt, out_a_functionPrototypes) = fun_68(txt, a_matrix, (a_omsiName).clone(), a_functionPrototypes, (a_index).clone(), (a_modelName).clone())?;
    Ok((out_txt, out_a_functionPrototypes))
}

fn lm_70(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_functionPrototypes: Tpl::Text, mut a_omsiFunction_context: SimCodeFunction::Context, mut a_modelName: ArcStr) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_functionPrototypes: Tpl::Text = a_functionPrototypes;
    for mut lstElt_70 in &*items {
        let mut lstElt_70 = lstElt_70.clone();
        (txt, a_functionPrototypes) = (::match_deref::match_deref! { match &(lstElt_70.clone()) {
        i_eq => {
            (txt, a_functionPrototypes) = generateEquationFunction(txt.clone(), i_eq.clone(), (a_modelName.clone()).clone(), (literal!("")).clone(), a_omsiFunction_context.clone(), a_functionPrototypes.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_functionPrototypes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_functionPrototypes))
}

fn fun_71(mut in_txt: Tpl::Text, mut in_a_column: Arc<SimCode::OMSIFunction>, mut in_a_functionPrototypes: Tpl::Text, mut in_a_modelName: ArcStr) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    (out_txt, out_a_functionPrototypes) = (::match_deref::match_deref! { match &((in_txt, in_a_column, in_a_functionPrototypes, in_a_modelName)) {
        (txt, Deref @ SimCode::OMSIFunction { equations: i_equations, context: i_omsiFunction_context, .. }, a_functionPrototypes, a_modelName) => {
            let mut l_bodyBuffer: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            l_bodyBuffer = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_bodyBuffer, a_functionPrototypes) = lm_70(l_bodyBuffer, i_equations.clone(), a_functionPrototypes.clone(), i_omsiFunction_context.clone(), (a_modelName.clone()).clone())?;
            l_bodyBuffer = Tpl::popIter(l_bodyBuffer)?;
            txt = Tpl::writeText(txt.clone(), l_bodyBuffer)?;
            (txt.clone(), a_functionPrototypes.clone())
        },
        (txt, _, a_functionPrototypes, _) => {
            (txt.clone(), a_functionPrototypes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_functionPrototypes))
}

pub(crate) fn generateDereivativeMatrixColumnFunction(mut txt: Tpl::Text, mut a_column: Arc<SimCode::OMSIFunction>, mut a_modelName: ArcStr, mut a_index: ArcStr, mut a_functionPrototypes: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut l_auxFunction: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    let mut l_preExp: Tpl::Text;
    let mut l_bodyBuffer: Tpl::Text;
    l_bodyBuffer = Tpl::emptyTxt.clone();
    l_preExp = Tpl::emptyTxt.clone();
    l_varDecls = Tpl::emptyTxt.clone();
    l_auxFunction = Tpl::emptyTxt.clone();
    (out_txt, out_a_functionPrototypes) = fun_71(txt, a_column, a_functionPrototypes, (a_modelName).clone())?;
    Ok((out_txt, out_a_functionPrototypes))
}

fn lm_73(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_omsiName: ArcStr, mut a_modelName: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_73 in &*items {
        let mut lstElt_73 = lstElt_73.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_73.clone()) {
        i_eq => {
            txt = equationCall(txt.clone(), i_eq.clone(), (a_modelName.clone()).clone(), (literal!("")).clone(), (literal!("this_function, model_vars_and_params")).clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_74(mut in_txt: Tpl::Text, mut in_a_column: Arc<SimCode::OMSIFunction>, mut in_a_index: ArcStr, mut in_a_functionPrototypes: Tpl::Text, mut in_a_omsiName: ArcStr, mut in_a_modelName: ArcStr) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    (out_txt, out_a_functionPrototypes) = (::match_deref::match_deref! { match &((in_txt, in_a_column, in_a_index, in_a_functionPrototypes, in_a_omsiName, in_a_modelName)) {
        (txt, Deref @ SimCode::OMSIFunction { equations: i_equations, .. }, a_index, a_functionPrototypes, a_omsiName, a_modelName) => {
            let mut l_bodyBuffer: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            l_bodyBuffer = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_bodyBuffer = lm_73(l_bodyBuffer, i_equations.clone(), (a_omsiName.clone()).clone(), (a_modelName.clone()).clone())?;
            l_bodyBuffer = Tpl::popIter(l_bodyBuffer)?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omsi_status ")).clone() }))?;
            a_functionPrototypes = CodegenUtil::symbolName(a_functionPrototypes.clone(), (a_modelName.clone()).clone(), (a_omsiName.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_derivativeMatFunc_")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (a_index.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(struct omsi_function_t* this_function, const omsi_values* model_vars_and_params, void* data);")).clone() }))?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/*\n")).clone(), (literal!("Description something\n")).clone(), (literal!("*/\n")).clone(), (literal!("omsi_status ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (a_modelName.clone()).clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_derivativeMatFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_index.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(struct omsi_function_t* this_function, const omsi_values* model_vars_and_params, void* data){\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_bodyBuffer)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("return omsi_ok;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_functionPrototypes.clone())
        },
        (txt, _, _, a_functionPrototypes, _, _) => {
            (txt.clone(), a_functionPrototypes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_functionPrototypes))
}

pub(crate) fn generateDereivativeMatrixColumnCall(mut txt: Tpl::Text, mut a_column: Arc<SimCode::OMSIFunction>, mut a_modelName: ArcStr, mut a_index: ArcStr, mut a_functionPrototypes: Tpl::Text, mut a_omsiName: ArcStr) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut l_auxFunction: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    let mut l_preExp: Tpl::Text;
    let mut l_bodyBuffer: Tpl::Text;
    l_bodyBuffer = Tpl::emptyTxt.clone();
    l_preExp = Tpl::emptyTxt.clone();
    l_varDecls = Tpl::emptyTxt.clone();
    l_auxFunction = Tpl::emptyTxt.clone();
    (out_txt, out_a_functionPrototypes) = fun_74(txt, a_column, (a_index).clone(), a_functionPrototypes, (a_omsiName).clone(), (a_modelName).clone())?;
    Ok((out_txt, out_a_functionPrototypes))
}

fn lm_76(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut a_context: SimCodeFunction::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_76 in &*items {
        let mut lstElt_76 = lstElt_76.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_76.clone()) {
        i_cr => {
            let mut ret_0: Arc<DAE::ComponentRef>;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = CodegenCFunctions::crefOMSI(txt.clone(), i_cr.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" && !")).clone() }))?;
            ret_0 = ComponentReference::crefPrefixPre(i_cr.clone());
            txt = CodegenCFunctions::crefOMSI(txt.clone(), ret_0.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" /* edge */)")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_77(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_context: SimCodeFunction::Context, mut in_a_conditions: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_context, in_a_conditions)) {
        (txt, false, a_context, a_conditions) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" || ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_76(txt.clone(), a_conditions.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_78(mut in_txt: Tpl::Text, mut in_mArg: Arc<DAE::Type>, mut in_a_val: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_stateVar: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_val, in_a_context, in_a_stateVar)) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: _, .. }, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TODO: Implement for arrays!")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_val, a_context, a_stateVar) => {
            let mut txt = (*txt).clone();
            txt = CodegenCFunctions::crefOMSI(txt.clone(), a_stateVar.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_val.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_79(mut in_txt: Tpl::Text, mut in_a_stmt: BackendDAE::WhenOperator, mut in_a_auxFunction: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_auxFunction: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_auxFunction, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt, in_a_stmt, in_a_auxFunction, in_a_varDecls, in_a_context)) {
        (txt, BackendDAE::WhenOperator::ASSIGN { left: Deref @ DAE::Exp::CREF { componentRef: i_left, .. }, right: i_right, .. }, a_auxFunction, a_varDecls, a_context) => {
            let mut l_rhs: Tpl::Text;
            let mut l_lhs: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            l_lhs = CodegenCFunctions::crefOMSI(Tpl::emptyTxt.clone(), i_left.clone(), a_context.clone())?;
            (l_rhs, l_preExp, a_varDecls, a_auxFunction) = CodegenCFunctions::daeExp(Tpl::emptyTxt.clone(), i_right.clone(), a_context.clone(), l_preExp, a_varDecls.clone(), a_auxFunction.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_preExp)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone())
        },
        (txt, BackendDAE::WhenOperator::REINIT { stateVar: i_stateVar, value: i_value, source: _ }, a_auxFunction, a_varDecls, a_context) => {
            let mut ret_4: Arc<DAE::Type>;
            let mut l_val: Tpl::Text;
            let mut l_lhs: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_val, l_preExp, a_varDecls, a_auxFunction) = CodegenCFunctions::daeExp(Tpl::emptyTxt.clone(), i_value.clone(), a_context.clone(), l_preExp, a_varDecls.clone(), a_auxFunction.clone())?;
            ret_4 = ComponentReference::crefTypeConsiderSubs(i_stateVar.clone())?;
            l_lhs = fun_78(Tpl::emptyTxt.clone(), ret_4, l_val, a_context.clone(), i_stateVar.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* ToDo: Add some info that variable was reinitialized */")).clone() }))?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone())
        },
        (txt, _, a_auxFunction, a_varDecls, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TODO: when expression not supported yet")).clone() }))?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_auxFunction, out_a_varDecls))
}

fn lm_80(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut a_auxFunction: Tpl::Text, mut a_varDecls: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_auxFunction: Tpl::Text = a_auxFunction;
    let mut a_varDecls: Tpl::Text = a_varDecls;
    for mut lstElt_80 in &*items {
        let mut lstElt_80 = lstElt_80.clone();
        (txt, a_auxFunction, a_varDecls) = (match lstElt_80.clone() {
        mut i_stmt => {
            (txt, a_auxFunction, a_varDecls) = fun_79(txt.clone(), i_stmt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone())
        },
    });
    }
    Ok((txt, a_auxFunction, a_varDecls))
}

pub(crate) fn equationWhen(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text, mut in_a_auxFunction: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_auxFunction: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_auxFunction) = (::match_deref::match_deref! { match &((in_txt, in_a_eq, in_a_context, in_a_varDecls, in_a_auxFunction)) {
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { whenStmtLst: i_whenStmtLst, conditions: i_conditions, elseWhen: None, .. }, a_context, a_varDecls, a_auxFunction) => {
            let mut l_assign: Tpl::Text;
            let mut ret_1: bool;
            let mut l_helpIf: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            ret_1 = i_conditions.clone().is_empty();
            l_helpIf = fun_77(Tpl::emptyTxt.clone(), ret_1, a_context.clone(), i_conditions.clone())?;
            l_assign = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_assign, a_auxFunction, a_varDecls) = lm_80(l_assign, i_whenStmtLst.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_context.clone())?;
            l_assign = Tpl::popIter(l_assign)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_helpIf)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_assign)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_auxFunction.clone())
        },
        (txt, _, _, a_varDecls, a_auxFunction) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TODO: Implement elsewhen")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_auxFunction.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_auxFunction))
}

