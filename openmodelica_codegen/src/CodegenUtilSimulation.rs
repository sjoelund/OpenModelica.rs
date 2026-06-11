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
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_dump::ExpressionDumpTpl;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_tpl::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn modelNamePrefix(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { fileNamePrefix: mut i_fileNamePrefix, .. }) => {
            let mut ret_0: ArcStr;
            ret_0 = (System::makeC89Identifier((i_fileNamePrefix.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fileNamePrefix(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { fileNamePrefix: mut i_fileNamePrefix, .. }) => {
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fullPathPrefix(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { fullPathPrefix: mut i_fullPathPrefix, .. }) => {
            txt = Tpl::writeStr(txt.clone(), (i_fullPathPrefix.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn equationIndex(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_FOR_RESIDUAL { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_GENERIC_RESIDUAL { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIZABLE_ASSIGN { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_GENERIC_ASSIGN { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ENTWINED_ASSIGN { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_IFEQUATION { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGORITHM { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_INVERSE_ALGORITHM { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { index: i_ls_index, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_ls_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { index: i_nls_index, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_nls_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_MIXED { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_FOR_LOOP { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_FOR_EQUATION { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALIAS { aliasOf: i_aliasOf, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_aliasOf.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenUtilSimulation.tpl")).clone(), 138, 14), (literal!("equationIndex failed")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn equationIndexAlternativeTearing(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_LINEAR { alternativeTearing: Some(Deref @ SimCode::LinearSystem { index: i_at_index, .. }), .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_at_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_NONLINEAR { alternativeTearing: Some(Deref @ SimCode::NonlinearSystem { index: i_at_index, .. }), .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_at_index.clone())).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn equationIndexGeneral(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, i_eq @ Deref @ SimCode::SimEqSystem::SES_LINEAR { alternativeTearing: Some(_), .. }) => {
            let mut txt = (*txt).clone();
            txt = equationIndexAlternativeTearing(txt.clone(), i_eq.clone())?;
            txt.clone()
        },
        (txt, i_eq @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { alternativeTearing: Some(_), .. }) => {
            let mut txt = (*txt).clone();
            txt = equationIndexAlternativeTearing(txt.clone(), i_eq.clone())?;
            txt.clone()
        },
        (txt, i_eq) => {
            let mut txt = (*txt).clone();
            txt = equationIndex(txt.clone(), i_eq.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_50(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_eqs.clone())) {
        (txt, Deref @ "none", a_eqs) => {
            let mut txt = (*txt).clone();
            txt = dumpEqsWork(txt.clone(), a_eqs.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpEqs(mut txt: Tpl::Text, mut a_eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: ArcStr;
    ret_0 = (Flags::getConfigString(Flags::OBFUSCATE.clone())?).clone();
    out_txt = fun_50(txt.clone(), (ret_0.clone()).clone(), a_eqs.clone())?;
    Ok(out_txt)
}

fn lm_52(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_52 in &*items.clone() {
        let mut lstElt_52 = lstElt_52.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_52.clone()) {
        i_stmt => {
            let mut ret_0: ArcStr;
            ret_0 = (DAEDump::ppStmtStr(i_stmt.clone(), 2)).clone();
            txt = CodegenUtil::escapeCComments(txt.clone(), (ret_0.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_53(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_53 in &*items.clone() {
        let mut lstElt_53 = lstElt_53.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_53.clone()) {
        i_stmt => {
            let mut ret_0: ArcStr;
            ret_0 = (DAEDump::ppStmtStr(i_stmt.clone(), 2)).clone();
            txt = CodegenUtil::escapeCComments(txt.clone(), (ret_0.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_54(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_54 in &*items.clone() {
        let mut lstElt_54 = lstElt_54.clone();
        txt = (match lstElt_54.clone() {
        SimCodeVar::SimVar { name: ref i_cr, .. } => {
            let mut txt_0: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<var>")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_cr.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</var>")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => {
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_55(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_55 in &*items.clone() {
        let mut lstElt_55 = lstElt_55.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_55.clone()) {
        i_exp => {
            let mut txt_0: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<cell>")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</cell>")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_56(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<residual>")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</residual>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_FOR_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_1: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<residual>")).clone() }))?;
            txt_1 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_1.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</residual>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_GENERIC_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_2: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<residual>")).clone() }))?;
            txt_2 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_2.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</residual>")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_57(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(i32, i32, Arc<SimCode::SimEqSystem>)>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_57 in &*items.clone() {
        let mut lstElt_57 = lstElt_57.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_57.clone()) {
        (i_i1, i_i2, i_eq) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<cell row=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_i1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" col=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_i2.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = fun_56(txt.clone(), i_eq.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</cell>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_58(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_58 in &*items.clone() {
        let mut lstElt_58 = lstElt_58.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_58.clone()) {
        i_cr => {
            let mut txt_0: Tpl::Text;
            txt_0 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_cr.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_59(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_59 in &*items.clone() {
        let mut lstElt_59 = lstElt_59.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_59.clone()) {
        i_eq => {
            txt = equationIndex(txt.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_60(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_60 in &*items.clone() {
        let mut lstElt_60 = lstElt_60.clone();
        txt = (match lstElt_60.clone() {
        SimCodeVar::SimVar { name: ref i_cr, .. } => {
            let mut txt_0: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<var>")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_cr.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</var>")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => {
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_61(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_61 in &*items.clone() {
        let mut lstElt_61 = lstElt_61.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_61.clone()) {
        i_eq => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<discrete index=\"")).clone() }))?;
            txt = equationIndex(txt.clone(), i_eq.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" />")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_62(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_62 in &*items.clone() {
        let mut lstElt_62 = lstElt_62.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_62.clone()) {
        i_eq => {
            txt = equationIndex(txt.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_63(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_63 in &*items.clone() {
        let mut lstElt_63 = lstElt_63.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_63.clone()) {
        i_cond => {
            let mut txt_0: Tpl::Text;
            txt_0 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_cond.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_64(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_64 in &*items.clone() {
        let mut lstElt_64 = lstElt_64.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_64.clone()) {
        (_, i_eqs) => {
            txt = dumpEqs(txt.clone(), i_eqs.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_65(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: RESIDUAL\n")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_FOR_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_1: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: FOR_RESIDUAL\n")).clone() }))?;
            txt_1 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_1.clone())?).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_GENERIC_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_2: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: GENERIC_RESIDUAL\n")).clone() }))?;
            txt_2 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_2.clone())?).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { cref: i_e_cref, exp: i_e_exp, .. }) => {
            let mut txt_4: Tpl::Text;
            let mut txt_3: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: SIMPLE_ASSIGN\n")).clone() }))?;
            txt_3 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_e_cref.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_3.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt_4 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_4.clone())?).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { cref: i_e_cref, exp: i_e_exp, cons: i_e_cons, .. }) => {
            let mut txt_7: Tpl::Text;
            let mut txt_6: Tpl::Text;
            let mut txt_5: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: SIMPLE_ASSIGN_CONSTRAINTS\n")).clone() }))?;
            txt_5 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_e_cref.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_5.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt_6 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_6.clone())?).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constraints: ")).clone() }))?;
            txt_7 = ExpressionDumpTpl::dumpConstraints(Tpl::emptyTxt.clone(), i_e_cons.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_7.clone())?).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { lhs: Deref @ DAE::Exp::CREF { componentRef: i_lhs_componentRef, .. }, exp: i_e_exp, .. }) => {
            let mut txt_9: Tpl::Text;
            let mut txt_8: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: ARRAY_CALL_ASSIGN\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt_8 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_lhs_componentRef.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_8.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt_9 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_9.clone())?).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_RESIZABLE_ASSIGN { call_index: i_e_call__index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("type: SES_RESIZABLE_ASSIGN call index: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_e_call__index.clone())).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_GENERIC_ASSIGN { call_index: i_e_call__index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("type: SES_GENERIC_ASSIGN call index: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_e_call__index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: Deref @ metamodelica::List::Nil, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("empty algorithm")).clone() }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: i_e_statements @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: ALGORITHM\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = lm_52(txt.clone(), i_e_statements.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_INVERSE_ALGORITHM { statements: i_e_statements @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: INVERSE ALGORITHM\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = lm_53(txt.clone(), i_e_statements.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { vars: i_ls_vars, beqs: i_ls_beqs, simJac: i_ls_simJac, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: LINEAR\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_54(txt.clone(), i_ls_vars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<row>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_55(txt.clone(), i_ls_beqs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</row>\n")).clone(), (literal!("<matrix>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = lm_57(txt.clone(), i_ls_simJac.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</matrix>")).clone() }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { indexNonLinearSystem: i_nls_indexNonLinearSystem, crefs: i_nls_crefs, eqs: i_nls_eqs, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("indexNonlinear: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_nls_indexNonLinearSystem.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: NONLINEAR\n")).clone(), (literal!("\n")).clone(), (literal!("vars: {")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_58(txt.clone(), i_nls_crefs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("eqns: {")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_59(txt.clone(), i_nls_eqs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_MIXED { cont: i_e_cont, discEqs: i_e_discEqs, discVars: i_e_discVars, .. }) => {
            let mut ret_10: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: MIXED\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            ret_10 = List::fill(i_e_cont.clone(), 1);
            txt = dumpEqs(txt.clone(), ret_10.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = dumpEqs(txt.clone(), i_e_discEqs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone(), (literal!("<mixed>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<continuous index=\"")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e_cont.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\" />\n")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_60(txt.clone(), i_e_discVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = lm_61(txt.clone(), i_e_discEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</mixed>")).clone() }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { residual: Deref @ SimCode::OMSIFunction { equations: i_residual_equations, .. }, matrix: i_matrix, linearSystem: i_e_linearSystem, .. }) => {
            let mut ret_12: i32;
            let mut l_detailedDescription: Tpl::Text;
            let mut txt = (*txt).clone();
            l_detailedDescription = dumpAlgSystemOps(Tpl::emptyTxt.clone(), i_matrix.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: ALGEBRAIC_SYSTEM\n")).clone(), (literal!("is linear: ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_e_linearSystem.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("depending functions indices: ")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_62(txt.clone(), i_residual_equations.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("dimension: ")).clone() }))?;
            ret_12 = (i_residual_equations.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_12.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_detailedDescription.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_WHEN { whenStmtLst: i_whenStmtLst, conditions: i_conditions, .. }) => {
            let mut l_body: Tpl::Text;
            let mut txt = (*txt).clone();
            l_body = dumpWhenOps(Tpl::emptyTxt.clone(), i_whenStmtLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: WHEN\n")).clone(), (literal!("\n")).clone(), (literal!("when {")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_63(txt.clone(), i_conditions.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("} then\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end when;")).clone() }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_IFEQUATION { ifbranches: i_ifbranches, elsebranch: i_elsebranch, .. }) => {
            let mut l_elsebr: Tpl::Text;
            let mut l_branches: Tpl::Text;
            let mut txt = (*txt).clone();
            l_branches = lm_64(Tpl::emptyTxt.clone(), i_ifbranches.clone())?;
            l_elsebr = dumpEqs(Tpl::emptyTxt.clone(), i_elsebranch.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: IFEQUATION\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_branches.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_elsebr.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_FOR_LOOP { iter: i_e_iter, startIt: i_e_startIt, endIt: i_e_endIt, cref: i_e_cref, exp: i_e_exp, .. }) => {
            let mut txt_21: Tpl::Text;
            let mut txt_20: Tpl::Text;
            let mut txt_19: Tpl::Text;
            let mut txt_18: Tpl::Text;
            let mut txt_17: Tpl::Text;
            let mut l_forstatement: Tpl::Text;
            let mut txt = (*txt).clone();
            l_forstatement = Tpl::emptyTxt.clone();
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for ")).clone() }))?;
            txt_17 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_iter.clone(), (literal!("\"")).clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_17.clone())?).clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" in ")).clone() }))?;
            txt_18 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_startIt.clone(), (literal!("\"")).clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_18.clone())?).clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : ")).clone() }))?;
            txt_19 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_endIt.clone(), (literal!("\"")).clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_19.clone())?).clone())?;
            l_forstatement = Tpl::pushBlock(l_forstatement.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("loop")).clone() }))?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            l_forstatement = Tpl::popBlock(l_forstatement.clone())?;
            l_forstatement = Tpl::pushBlock(l_forstatement.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt_20 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_e_cref.clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_20.clone())?).clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt_21 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_21.clone())?).clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; ")).clone() }))?;
            l_forstatement = Tpl::popBlock(l_forstatement.clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end for")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: FOR_LOOP\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_forstatement.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_FOR_EQUATION { iter: i_e_iter, startIt: i_e_startIt, endIt: i_e_endIt, body: i_e_body, .. }) => {
            let mut txt_24: Tpl::Text;
            let mut txt_23: Tpl::Text;
            let mut txt_22: Tpl::Text;
            let mut l_forstatement: Tpl::Text;
            let mut txt = (*txt).clone();
            l_forstatement = Tpl::emptyTxt.clone();
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for ")).clone() }))?;
            txt_22 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_iter.clone(), (literal!("\"")).clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_22.clone())?).clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" in ")).clone() }))?;
            txt_23 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_startIt.clone(), (literal!("\"")).clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_23.clone())?).clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : ")).clone() }))?;
            txt_24 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_endIt.clone(), (literal!("\"")).clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_24.clone())?).clone())?;
            l_forstatement = Tpl::pushBlock(l_forstatement.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("loop")).clone() }))?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            l_forstatement = Tpl::popBlock(l_forstatement.clone())?;
            l_forstatement = Tpl::pushBlock(l_forstatement.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            l_forstatement = dumpEqs(l_forstatement.clone(), i_e_body.clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            l_forstatement = Tpl::popBlock(l_forstatement.clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end for")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: FOR_EQUATION\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_forstatement.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_ALIAS { aliasOf: i_e_aliasOf, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: ALIAS\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("alias of ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_e_aliasOf.clone())).clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_ENTWINED_ASSIGN { single_calls: i_e_single__calls, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: ENTWINED_ASSIGN\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = dumpEqs(txt.clone(), i_e_single__calls.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unknown equation")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_66(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_66 in &*items.clone() {
        let mut lstElt_66 = lstElt_66.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_66.clone()) {
        i_eq => {
            txt = fun_65(txt.clone(), i_eq.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn dumpEqsWork(mut txt: Tpl::Text, mut a_eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = lm_66(txt.clone(), a_eqs.clone())?;
    Ok(out_txt)
}

fn lm_68(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::OMSIFunction>>>, mut a_varsBuffer: Tpl::Text, mut a_columnBuffer: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_varsBuffer: Tpl::Text = a_varsBuffer;
    let mut a_columnBuffer: Tpl::Text = a_columnBuffer;
    for mut lstElt_68 in &*items.clone() {
        let mut lstElt_68 = lstElt_68.clone();
        (txt, a_varsBuffer, a_columnBuffer) = (::match_deref::match_deref! { match &(lstElt_68.clone()) {
        i_column => {
            (txt, a_columnBuffer, a_varsBuffer) = dumpAlgSystemColumn(txt.clone(), i_column.clone(), a_columnBuffer.clone(), a_varsBuffer.clone())?;
            (txt.clone(), a_varsBuffer.clone(), a_columnBuffer.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_varsBuffer, a_columnBuffer))
}

fn fun_69(mut in_txt: Tpl::Text, mut in_a_derivativeMatrix: Option<Arc<SimCode::DerivativeMatrix>>, mut in_a_varsBuffer: Tpl::Text, mut in_a_columnBuffer: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varsBuffer: Tpl::Text;
    let mut out_a_columnBuffer: Tpl::Text;
    (out_txt, out_a_varsBuffer, out_a_columnBuffer) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_derivativeMatrix.clone(), in_a_varsBuffer.clone(), in_a_columnBuffer.clone())) {
        (txt, Some(Deref @ SimCode::DerivativeMatrix { columns: i_matrix_columns, .. }), a_varsBuffer, a_columnBuffer) => {
            let mut l_0__: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varsBuffer = (*a_varsBuffer).clone();
            let mut a_columnBuffer = (*a_columnBuffer).clone();
            (l_0__, a_varsBuffer, a_columnBuffer) = lm_68(Tpl::emptyTxt.clone(), i_matrix_columns.clone(), a_varsBuffer.clone(), a_columnBuffer.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("iteration vars: ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_varsBuffer.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_columnBuffer.clone())?;
            (txt.clone(), a_varsBuffer.clone(), a_columnBuffer.clone())
        },
        (txt, _, a_varsBuffer, a_columnBuffer) => {
            (txt.clone(), a_varsBuffer.clone(), a_columnBuffer.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varsBuffer, out_a_columnBuffer))
}

pub(crate) fn dumpAlgSystemOps(mut txt: Tpl::Text, mut a_derivativeMatrix: Option<Arc<SimCode::DerivativeMatrix>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_columnBuffer: Tpl::Text;
    let mut l_varsBuffer: Tpl::Text;
    l_varsBuffer = Tpl::emptyTxt.clone();
    l_columnBuffer = Tpl::emptyTxt.clone();
    (out_txt, l_varsBuffer, l_columnBuffer) = fun_69(txt.clone(), a_derivativeMatrix.clone(), l_varsBuffer.clone(), l_columnBuffer.clone())?;
    Ok(out_txt)
}

fn lm_71(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_71 in &*items.clone() {
        let mut lstElt_71 = lstElt_71.clone();
        txt = (match lstElt_71.clone() {
        SimCodeVar::SimVar { name: ref i_name, .. } => {
            txt = ExpressionDumpTpl::dumpCref(txt.clone(), i_name.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => {
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_72(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_columnBuffer: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_columnBuffer: Tpl::Text = a_columnBuffer;
    for mut lstElt_72 in &*items.clone() {
        let mut lstElt_72 = lstElt_72.clone();
        (txt, a_columnBuffer) = (::match_deref::match_deref! { match &(lstElt_72.clone()) {
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { cref: i_equation_cref, exp: i_equation_exp, .. } => {
            let mut txt_0: Tpl::Text;
            a_columnBuffer = ExpressionDumpTpl::dumpCref(a_columnBuffer.clone(), i_equation_cref.clone())?;
            a_columnBuffer = Tpl::writeTok(a_columnBuffer.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_equation_exp.clone(), (literal!("\"")).clone())?;
            a_columnBuffer = CodegenUtil::escapeCComments(a_columnBuffer.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            a_columnBuffer = Tpl::writeTok(a_columnBuffer.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            (txt.clone(), a_columnBuffer.clone())
        },
        _ => {
            (txt.clone(), a_columnBuffer.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_columnBuffer))
}

pub(crate) fn dumpAlgSystemColumn(mut in_txt: Tpl::Text, mut in_a_column: Arc<SimCode::OMSIFunction>, mut in_a_columnBuffer: Tpl::Text, mut in_a_varsBuffer: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_columnBuffer: Tpl::Text;
    let mut out_a_varsBuffer: Tpl::Text;
    (out_txt, out_a_columnBuffer, out_a_varsBuffer) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_column.clone(), in_a_columnBuffer.clone(), in_a_varsBuffer.clone())) {
        (txt, Deref @ SimCode::OMSIFunction { inputVars: i_inputVars, equations: i_equations, .. }, a_columnBuffer, a_varsBuffer) => {
            let mut l_0__: Tpl::Text;
            let mut a_columnBuffer = (*a_columnBuffer).clone();
            let mut a_varsBuffer = (*a_varsBuffer).clone();
            a_varsBuffer = Tpl::pushIter(a_varsBuffer.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            a_varsBuffer = lm_71(a_varsBuffer.clone(), i_inputVars.clone())?;
            a_varsBuffer = Tpl::popIter(a_varsBuffer.clone())?;
            (l_0__, a_columnBuffer) = lm_72(Tpl::emptyTxt.clone(), i_equations.clone(), a_columnBuffer.clone())?;
            (txt.clone(), a_columnBuffer.clone(), a_varsBuffer.clone())
        },
        (txt, _, a_columnBuffer, a_varsBuffer) => {
            (txt.clone(), a_columnBuffer.clone(), a_varsBuffer.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_columnBuffer, out_a_varsBuffer))
}

pub(crate) fn dumpWhenOps(mut in_txt: Tpl::Text, mut in_a_whenOps: Arc<metamodelica::List<BackendDAE::WhenOperator>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_whenOps.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: Deref @ DAE::Exp::CREF { componentRef: i_left_componentRef, .. }, right: i_e_right, .. }, tail: i_rest }) => {
            let mut txt_1: Tpl::Text;
            let mut l_restbody: Tpl::Text;
            let mut txt = (*txt).clone();
            l_restbody = dumpWhenOps(Tpl::emptyTxt.clone(), i_rest.clone())?;
            txt = ExpressionDumpTpl::dumpCref(txt.clone(), i_left_componentRef.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt_1 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_right.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_1.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_restbody.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: i_e_left, right: i_e_right, .. }, tail: i_rest }) => {
            let mut txt_2: Tpl::Text;
            let mut l_restbody: Tpl::Text;
            let mut txt = (*txt).clone();
            l_restbody = dumpWhenOps(Tpl::emptyTxt.clone(), i_rest.clone())?;
            txt = ExpressionDumpTpl::dumpExp(txt.clone(), i_e_left.clone(), (literal!("\"")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt_2 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_right.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_2.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_restbody.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::REINIT { stateVar: i_e_stateVar, value: i_e_value, .. }, tail: i_rest }) => {
            let mut txt_3: Tpl::Text;
            let mut l_restbody: Tpl::Text;
            let mut txt = (*txt).clone();
            l_restbody = dumpWhenOps(Tpl::emptyTxt.clone(), i_rest.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("reinit(")).clone() }))?;
            txt = ExpressionDumpTpl::dumpCref(txt.clone(), i_e_stateVar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",  ")).clone() }))?;
            txt_3 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_value.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_3.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_restbody.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSERT { condition: i_e_condition, message: i_e_message, level: i_e_level, .. }, tail: i_rest }) => {
            let mut txt_6: Tpl::Text;
            let mut txt_5: Tpl::Text;
            let mut txt_4: Tpl::Text;
            let mut l_restbody: Tpl::Text;
            let mut txt = (*txt).clone();
            l_restbody = dumpWhenOps(Tpl::emptyTxt.clone(), i_rest.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("assert(")).clone() }))?;
            txt_4 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_condition.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_4.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt_5 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_message.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_5.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt_6 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_level.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_6.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_restbody.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::TERMINATE { message: i_e_message, .. }, tail: i_rest }) => {
            let mut txt_7: Tpl::Text;
            let mut l_restbody: Tpl::Text;
            let mut txt = (*txt).clone();
            l_restbody = dumpWhenOps(Tpl::emptyTxt.clone(), i_rest.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("terminate(")).clone() }))?;
            txt_7 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_message.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_7.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(")%>);\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_restbody.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::NORETCALL { exp: i_e_exp, .. }, tail: i_rest }) => {
            let mut txt_8: Tpl::Text;
            let mut l_restbody: Tpl::Text;
            let mut txt = (*txt).clone();
            l_restbody = dumpWhenOps(Tpl::emptyTxt.clone(), i_rest.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("noReturnCall(")).clone() }))?;
            txt_8 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_8.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(")%>);\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_restbody.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenUtilSimulation.tpl")).clone(), 443, 14), (literal!("dumpEqs: Unknown equation")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_75(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_eqs.clone())) {
        (txt, Deref @ "none", a_eqs) => {
            let mut txt = (*txt).clone();
            txt = dumpEqsAlternativeTearingWork(txt.clone(), a_eqs.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpEqsAlternativeTearing(mut txt: Tpl::Text, mut a_eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: ArcStr;
    ret_0 = (Flags::getConfigString(Flags::OBFUSCATE.clone())?).clone();
    out_txt = fun_75(txt.clone(), (ret_0.clone()).clone(), a_eqs.clone())?;
    Ok(out_txt)
}

fn lm_77(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_77 in &*items.clone() {
        let mut lstElt_77 = lstElt_77.clone();
        txt = (match lstElt_77.clone() {
        SimCodeVar::SimVar { name: ref i_cr, .. } => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<var>")).clone() }))?;
            txt = ExpressionDumpTpl::dumpCref(txt.clone(), i_cr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</var>")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => {
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_78(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_78 in &*items.clone() {
        let mut lstElt_78 = lstElt_78.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_78.clone()) {
        i_exp => {
            let mut txt_0: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<cell>")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</cell>")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_79(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<residual>")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</residual>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_FOR_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_1: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<residual>")).clone() }))?;
            txt_1 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_1.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</residual>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_GENERIC_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_2: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<residual>")).clone() }))?;
            txt_2 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_2.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</residual>")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_80(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(i32, i32, Arc<SimCode::SimEqSystem>)>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_80 in &*items.clone() {
        let mut lstElt_80 = lstElt_80.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_80.clone()) {
        (i_i1, i_i2, i_eq) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<cell row=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_i1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" col=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_i2.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = fun_79(txt.clone(), i_eq.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</cell>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_81(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_81 in &*items.clone() {
        let mut lstElt_81 = lstElt_81.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_81.clone()) {
        i_cr => {
            txt = ExpressionDumpTpl::dumpCref(txt.clone(), i_cr.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_82(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_82 in &*items.clone() {
        let mut lstElt_82 = lstElt_82.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_82.clone()) {
        i_eq => {
            txt = equationIndex(txt.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_83(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_LINEAR { alternativeTearing: Some(Deref @ SimCode::LinearSystem { vars: i_at_vars, beqs: i_at_beqs, simJac: i_at_simJac, .. }), .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndexAlternativeTearing(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: LINEAR\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_77(txt.clone(), i_at_vars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<row>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_78(txt.clone(), i_at_beqs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</row>\n")).clone(), (literal!("<matrix>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = lm_80(txt.clone(), i_at_simJac.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</matrix>\n")).clone(), (literal!("\n")).clone(), (literal!("This is the alternative tearing set with casual solvability rules.\n")).clone(), (literal!("If it fails, this function will call the strict tearing set.")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { alternativeTearing: Some(Deref @ SimCode::NonlinearSystem { indexNonLinearSystem: i_at_indexNonLinearSystem, crefs: i_at_crefs, eqs: i_at_eqs, .. }), .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndexAlternativeTearing(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("indexNonlinear: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_at_indexNonLinearSystem.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: NONLINEAR\n")).clone(), (literal!("\n")).clone(), (literal!("vars: {")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_81(txt.clone(), i_at_crefs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("eqns: {")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_82(txt.clone(), i_at_eqs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("This is the alternative tearing set with casual solvability rules.\n")).clone(), (literal!("If it fails, this function will call the strict tearing set.")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unknown equation")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_84(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_84 in &*items.clone() {
        let mut lstElt_84 = lstElt_84.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_84.clone()) {
        i_eq => {
            txt = fun_83(txt.clone(), i_eq.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn dumpEqsAlternativeTearingWork(mut txt: Tpl::Text, mut a_eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = lm_84(txt.clone(), a_eqs.clone())?;
    Ok(out_txt)
}

