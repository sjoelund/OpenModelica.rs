// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::CodegenUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_dump::ExpressionDumpTpl;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_susan::Tpl;
use openmodelica_util::Util;

fn lm_46(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_46 in &*items.clone() {
        let mut lstElt_46 = lstElt_46.clone();
        txt = (match lstElt_46.clone() {
        SimCodeVar::SimVar { name: ref i_v_name, .. } => {
            let mut x_index0: i32;
            let mut ret_1: ArcStr;
            let mut txt_0: Tpl::Text;
            x_index0 = Tpl::getIteri_i0(txt.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_index0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(": ")).clone() }))?;
            txt_0 = CodegenUtil::crefStrNoUnderscore(Tpl::emptyTxt.clone(), i_v_name.clone())?;
            ret_1 = (Util::escapeModelicaStringToXmlString((Tpl::textString(txt_0.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
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

pub(crate) fn dumpVarsShort(mut txt: Tpl::Text, mut a_vars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_varsString: Tpl::Text;
    l_varsString = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    l_varsString = lm_46(l_varsString.clone(), a_vars.clone())?;
    l_varsString = Tpl::popIter(l_varsString.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_varsString.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
    Ok(out_txt)
}

pub(crate) fn dumpAlias(mut in_txt: Tpl::Text, mut in_a_alias: SimCodeVar::AliasVariable) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_alias.clone()) {
        (mut txt, SimCodeVar::AliasVariable::ALIAS { varName: ref i_varName }) => {
            let mut ret_1: ArcStr;
            let mut txt_0: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<alias>")).clone() }))?;
            txt_0 = CodegenUtil::crefStrNoUnderscore(Tpl::emptyTxt.clone(), i_varName.clone())?;
            ret_1 = (Util::escapeModelicaStringToXmlString((Tpl::textString(txt_0.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</alias>")).clone() }))?;
            txt.clone()
        },
        (mut txt, SimCodeVar::AliasVariable::NEGATEDALIAS { varName: ref i_varName }) => {
            let mut ret_3: ArcStr;
            let mut txt_2: Tpl::Text;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<alias negated=\"true\">")).clone() }))?;
            txt_2 = CodegenUtil::crefStrNoUnderscore(Tpl::emptyTxt.clone(), i_varName.clone())?;
            ret_3 = (Util::escapeModelicaStringToXmlString((Tpl::textString(txt_2.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_3.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</alias>")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn printExpStrEscaped(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: ArcStr;
    let mut txt_0: Tpl::Text;
    txt_0 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), a_exp.clone(), (literal!("\"")).clone())?;
    ret_1 = (Util::escapeModelicaStringToXmlString((Tpl::textString(txt_0.clone())?).clone())?).clone();
    out_txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
    Ok(out_txt)
}

