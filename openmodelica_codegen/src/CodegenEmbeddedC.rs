// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::CodegenUtilSimulation;
use openmodelica_ast::Absyn;
use openmodelica_backend::CodegenUtil;
use openmodelica_backend::SimCodeUtil;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::DAEDumpTpl;
use openmodelica_frontend_dump::ExpressionDumpTpl;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_susan::Tpl;
use openmodelica_util::Error;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Util;

fn fun_51(mut in_txt: Tpl::Text, mut in_mArg: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, 0) => {
            txt.clone()
        },
        (mut txt, mut i_n) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2Real fmi2RealVars[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_n.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_52(mut in_txt: Tpl::Text, mut in_a_varInfo_numIntAlgVars: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_varInfo_numIntAlgVars.clone()) {
        (mut txt, 0) => {
            txt.clone()
        },
        (mut txt, mut i_n) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2Integer fmi2IntegerVars[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_n.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_53(mut in_txt: Tpl::Text, mut in_a_varInfo_numBoolAlgVars: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_varInfo_numBoolAlgVars.clone()) {
        (mut txt, 0) => {
            txt.clone()
        },
        (mut txt, mut i_n) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2Boolean fmi2BooleanVars[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_n.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_54(mut in_txt: Tpl::Text, mut in_a_varInfo_numStringAlgVars: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_varInfo_numStringAlgVars.clone()) {
        (mut txt, 0) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 145, 16), (literal!("String variables not supported yet")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_55(mut in_txt: Tpl::Text, mut in_a_varInfo_numParams: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_varInfo_numParams.clone()) {
        (mut txt, 0) => {
            txt.clone()
        },
        (mut txt, mut i_n) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2Real fmi2RealParameter[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_n.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_56(mut in_txt: Tpl::Text, mut in_a_varInfo_numIntParams: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_varInfo_numIntParams.clone()) {
        (mut txt, 0) => {
            txt.clone()
        },
        (mut txt, mut i_n) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2Integer fmi2IntegerParameter[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_n.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_57(mut in_txt: Tpl::Text, mut in_a_varInfo_numBoolParams: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_varInfo_numBoolParams.clone()) {
        (mut txt, 0) => {
            txt.clone()
        },
        (mut txt, mut i_n) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2Boolean fmi2BooleanParameter[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_n.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_58(mut in_txt: Tpl::Text, mut in_a_varInfo_numIntParams: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_varInfo_numIntParams.clone()) {
        (mut txt, 0) => {
            txt.clone()
        },
        (mut txt, mut i_n) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2String fmi2StringParameter[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_n.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_59(mut in_txt: Tpl::Text, mut in_mArg: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, 0) => {
            txt.clone()
        },
        (mut txt, mut i_n) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void* extObjs[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_n.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_60(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_60 in &*items.clone() {
        let mut lstElt_60 = lstElt_60.clone();
        txt = (match lstElt_60.clone() {
        mut i_var => {
            txt = startValue(txt.clone(), i_var.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_61(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_61 in &*items.clone() {
        let mut lstElt_61 = lstElt_61.clone();
        txt = (match lstElt_61.clone() {
        mut i_var => {
            txt = startValue(txt.clone(), i_var.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_62(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_62 in &*items.clone() {
        let mut lstElt_62 = lstElt_62.clone();
        txt = (match lstElt_62.clone() {
        mut i_var => {
            txt = startValue(txt.clone(), i_var.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_63(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_63 in &*items.clone() {
        let mut lstElt_63 = lstElt_63.clone();
        txt = (match lstElt_63.clone() {
        mut i_var => {
            txt = startValue(txt.clone(), i_var.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_64(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_64 in &*items.clone() {
        let mut lstElt_64 = lstElt_64.clone();
        txt = (match lstElt_64.clone() {
        mut i_var => {
            txt = startValue(txt.clone(), i_var.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_65(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_65 in &*items.clone() {
        let mut lstElt_65 = lstElt_65.clone();
        txt = (match lstElt_65.clone() {
        mut i_var => {
            txt = startValue(txt.clone(), i_var.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_66(mut in_txt: Tpl::Text, mut in_mArg: i32, mut in_a_vars_realOptimizeFinalConstraintsVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_vars_realOptimizeConstraintsVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_vars_discreteAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_vars_algVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_vars_derivativeVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_vars_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_vars_realOptimizeFinalConstraintsVars.clone(), in_a_vars_realOptimizeConstraintsVars.clone(), in_a_vars_discreteAlgVars.clone(), in_a_vars_algVars.clone(), in_a_vars_derivativeVars.clone(), in_a_vars_stateVars.clone())) {
        (txt, 0, _, _, _, _, _, _) => {
            txt.clone()
        },
        (txt, _, a_vars_realOptimizeFinalConstraintsVars, a_vars_realOptimizeConstraintsVars, a_vars_discreteAlgVars, a_vars_algVars, a_vars_derivativeVars, a_vars_stateVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmi2RealVars = {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = lm_60(txt.clone(), a_vars_stateVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = lm_61(txt.clone(), a_vars_derivativeVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = lm_62(txt.clone(), a_vars_algVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = lm_63(txt.clone(), a_vars_discreteAlgVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = lm_64(txt.clone(), a_vars_realOptimizeConstraintsVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = lm_65(txt.clone(), a_vars_realOptimizeFinalConstraintsVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("},")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_67(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_67 in &*items.clone() {
        let mut lstElt_67 = lstElt_67.clone();
        txt = (match lstElt_67.clone() {
        mut i_var => {
            txt = startValue(txt.clone(), i_var.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_68(mut in_txt: Tpl::Text, mut in_a_varInfo_numIntAlgVars: i32, mut in_a_vars_intAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_varInfo_numIntAlgVars.clone(), in_a_vars_intAlgVars.clone())) {
        (txt, 0, _) => {
            txt.clone()
        },
        (txt, _, a_vars_intAlgVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmi2IntegerVars = {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = lm_67(txt.clone(), a_vars_intAlgVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("},")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_69(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_69 in &*items.clone() {
        let mut lstElt_69 = lstElt_69.clone();
        txt = (match lstElt_69.clone() {
        mut i_var => {
            txt = startValue(txt.clone(), i_var.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_70(mut in_txt: Tpl::Text, mut in_a_varInfo_numBoolAlgVars: i32, mut in_a_vars_boolAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_varInfo_numBoolAlgVars.clone(), in_a_vars_boolAlgVars.clone())) {
        (txt, 0, _) => {
            txt.clone()
        },
        (txt, _, a_vars_boolAlgVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmi2BooleanVars = {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = lm_69(txt.clone(), a_vars_boolAlgVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("},")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_71(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_71 in &*items.clone() {
        let mut lstElt_71 = lstElt_71.clone();
        txt = (match lstElt_71.clone() {
        mut i_var => {
            txt = startValue(txt.clone(), i_var.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_72(mut in_txt: Tpl::Text, mut in_a_varInfo_numStringAlgVars: i32, mut in_a_vars_stringAlgVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_varInfo_numStringAlgVars.clone(), in_a_vars_stringAlgVars.clone())) {
        (txt, 0, _) => {
            txt.clone()
        },
        (txt, _, a_vars_stringAlgVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmi2StringVars = {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = lm_71(txt.clone(), a_vars_stringAlgVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("},")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_73(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_73 in &*items.clone() {
        let mut lstElt_73 = lstElt_73.clone();
        txt = (match lstElt_73.clone() {
        mut i_var => {
            txt = startValue(txt.clone(), i_var.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_74(mut in_txt: Tpl::Text, mut in_a_varInfo_numParams: i32, mut in_a_vars_paramVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_varInfo_numParams.clone(), in_a_vars_paramVars.clone())) {
        (txt, 0, _) => {
            txt.clone()
        },
        (txt, _, a_vars_paramVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmi2RealParameter = {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = lm_73(txt.clone(), a_vars_paramVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("},")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_75(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_75 in &*items.clone() {
        let mut lstElt_75 = lstElt_75.clone();
        txt = (match lstElt_75.clone() {
        mut i_var => {
            txt = startValue(txt.clone(), i_var.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_76(mut in_txt: Tpl::Text, mut in_a_varInfo_numIntParams: i32, mut in_a_vars_intParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_varInfo_numIntParams.clone(), in_a_vars_intParamVars.clone())) {
        (txt, 0, _) => {
            txt.clone()
        },
        (txt, _, a_vars_intParamVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmi2IntegerParameter = {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = lm_75(txt.clone(), a_vars_intParamVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("},")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_77(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_77 in &*items.clone() {
        let mut lstElt_77 = lstElt_77.clone();
        txt = (match lstElt_77.clone() {
        mut i_var => {
            txt = startValue(txt.clone(), i_var.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_78(mut in_txt: Tpl::Text, mut in_a_varInfo_numBoolParams: i32, mut in_a_vars_boolParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_varInfo_numBoolParams.clone(), in_a_vars_boolParamVars.clone())) {
        (txt, 0, _) => {
            txt.clone()
        },
        (txt, _, a_vars_boolParamVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmi2BooleanParameter = {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = lm_77(txt.clone(), a_vars_boolParamVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("},")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_79(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_79 in &*items.clone() {
        let mut lstElt_79 = lstElt_79.clone();
        txt = (match lstElt_79.clone() {
        mut i_var => {
            txt = startValue(txt.clone(), i_var.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_80(mut in_txt: Tpl::Text, mut in_a_varInfo_numStringParamVars: i32, mut in_a_vars_stringParamVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_varInfo_numStringParamVars.clone(), in_a_vars_stringParamVars.clone())) {
        (txt, 0, _) => {
            txt.clone()
        },
        (txt, _, a_vars_stringParamVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmi2StringParameter = {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = lm_79(txt.clone(), a_vars_stringParamVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("},")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_81(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_81 in &*items.clone() {
        let mut lstElt_81 = lstElt_81.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_81.clone()) {
        i_eq => {
            txt = equation_(txt.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_82(mut in_txt: Tpl::Text, mut in_a_odeEquations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_odeEquations.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eqs, tail: Deref @ metamodelica::List::Nil }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_81(txt.clone(), i_eqs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 276, 14), (literal!("TODO")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn smf_83(mut in_txt: Tpl::Text, mut in_it: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_it.clone())) {
        (txt, i_eq) => {
            let mut txt = (*txt).clone();
            txt = equation_(txt.clone(), i_eq.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn smf_84(mut in_txt: Tpl::Text, mut in_it: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_it.clone())) {
        (txt, i_eq) => {
            let mut txt = (*txt).clone();
            txt = equation_(txt.clone(), i_eq.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_85(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_85 in &*items.clone() {
        let mut lstElt_85 = lstElt_85.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_85.clone()) {
        i_eqs => {
            txt = smf_84(txt.clone(), i_eqs.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_86(mut in_txt: Tpl::Text, mut in_a_allEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_allEquations.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eqs, tail: Deref @ metamodelica::List::Nil }) => {
            let mut txt = (*txt).clone();
            txt = smf_83(txt.clone(), i_eqs.clone())?;
            txt.clone()
        },
        (txt, i_allEquations) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_85(txt.clone(), i_allEquations.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_87(mut in_txt: Tpl::Text, mut in_a_varInfo_numStateVars: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_varInfo_numStateVars.clone()) {
        (mut txt, 0) => {
            txt.clone()
        },
        (mut txt, mut i_varInfo_numStateVars) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("int i=0;\n")).clone(), (literal!("for (i=0; i<")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_varInfo_numStateVars.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("; i++) {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->fmi2RealVars[i] += comp->fmi2RealVars[i+")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_varInfo_numStateVars.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("]*communicationStepSize;\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_88(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_modelNamePrefixStr: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_modelNamePrefixStr.clone()) {
        (mut txt, SimCode::SimCode { simulationSettingsOpt: None, .. }, _) => {
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 96, 9), (literal!("Missing simulation settings")).clone())?;
            txt.clone()
        },
        (mut txt, SimCode::SimCode { modelInfo: SimCode::ModelInfo { functions: ref i_functions, varInfo: ref i_varInfo @ SimCode::VarInfo { numIntAlgVars: ref i_varInfo_numIntAlgVars, numBoolAlgVars: ref i_varInfo_numBoolAlgVars, numStringAlgVars: ref i_varInfo_numStringAlgVars, numParams: ref i_varInfo_numParams, numIntParams: ref i_varInfo_numIntParams, numBoolParams: ref i_varInfo_numBoolParams, numStringParamVars: ref i_varInfo_numStringParamVars, numStateVars: ref i_varInfo_numStateVars, .. }, vars: SimCodeVar::SimVars { stateVars: ref i_vars_stateVars, derivativeVars: ref i_vars_derivativeVars, algVars: ref i_vars_algVars, discreteAlgVars: ref i_vars_discreteAlgVars, realOptimizeConstraintsVars: ref i_vars_realOptimizeConstraintsVars, realOptimizeFinalConstraintsVars: ref i_vars_realOptimizeFinalConstraintsVars, intAlgVars: ref i_vars_intAlgVars, boolAlgVars: ref i_vars_boolAlgVars, stringAlgVars: ref i_vars_stringAlgVars, paramVars: ref i_vars_paramVars, intParamVars: ref i_vars_intParamVars, boolParamVars: ref i_vars_boolParamVars, stringParamVars: ref i_vars_stringParamVars, .. }, .. }, extObjInfo: ref i_extObjInfo @ SimCode::ExtObjInfo { vars: ref i_extObjVars, .. }, simulationSettingsOpt: Some(SimCode::SimulationSettings { startTime: mut i_settings_startTime, stopTime: mut i_settings_stopTime, stepSize: mut i_settings_stepSize, .. }), literals: ref i_literals, externalFunctionIncludes: ref i_externalFunctionIncludes, odeEquations: ref i_odeEquations, allEquations: ref i_allEquations, .. }, mut a_modelNamePrefixStr) => {
            let mut ret_2: i32 = 0;
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#define fmi2TypesPlatform_h\n")).clone(), (literal!("\n")).clone(), (literal!("#define fmi2TypesPlatform \"default\" /* Compatible */\n")).clone(), (literal!("\n")).clone(), (literal!("typedef struct ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("fmi2Component_s")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("* fmi2Component;\n")).clone(), (literal!("typedef void* fmi2ComponentEnvironment;    /* Pointer to FMU environment    */\n")).clone(), (literal!("typedef void* fmi2FMUstate;                /* Pointer to internal FMU state */\n")).clone(), (literal!("typedef unsigned int fmi2ValueReference;\n")).clone(), (literal!("typedef double fmi2Real;\n")).clone(), (literal!("typedef int fmi2Integer;\n")).clone(), (literal!("typedef int fmi2Boolean;\n")).clone(), (literal!("typedef char fmi2Char;\n")).clone(), (literal!("typedef const fmi2Char* fmi2String;\n")).clone(), (literal!("typedef char fmi2Byte;\n")).clone(), (literal!("\n")).clone(), (literal!("#define fmi2True 1\n")).clone(), (literal!("#define fmi2False 0\n")).clone(), (literal!("\n")).clone(), (literal!("#include \"fmi2/fmi2Functions.h\"\n")).clone(), (literal!("\n")).clone(), (literal!("#include <stdint.h>\n")).clone(), (literal!("#include <stdio.h>\n")).clone(), (literal!("\n")).clone(), (literal!("void ModelicaFormatMessage(const char *fmt, ...)\n")).clone(), (literal!("{\n")).clone(), (literal!("  va_list args;\n")).clone(), (literal!("  va_start(args, fmt);\n")).clone(), (literal!("  vprintf(fmt, args);\n")).clone(), (literal!("  va_end(args);\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("typedef struct ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("fmi2Component_s")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" {\n")).clone(), (literal!("  fmi2Real currentTime;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_0 = SimCodeUtil::nVariablesReal(i_varInfo.clone());
            txt = fun_51(txt.clone(), ret_0.clone())?;
            txt = fun_52(txt.clone(), i_varInfo_numIntAlgVars.clone())?;
            txt = fun_53(txt.clone(), i_varInfo_numBoolAlgVars.clone())?;
            txt = fun_54(txt.clone(), i_varInfo_numStringAlgVars.clone())?;
            txt = fun_55(txt.clone(), i_varInfo_numParams.clone())?;
            txt = fun_56(txt.clone(), i_varInfo_numIntParams.clone())?;
            txt = fun_57(txt.clone(), i_varInfo_numBoolParams.clone())?;
            txt = fun_58(txt.clone(), i_varInfo_numIntParams.clone())?;
            ret_1 = (i_extObjVars.clone().len() as i32);
            txt = fun_59(txt.clone(), ret_1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} ")).clone() }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("fmi2Component")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("fmi2Component")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("component")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" = {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_2 = SimCodeUtil::nVariablesReal(i_varInfo.clone());
            txt = fun_66(txt.clone(), ret_2.clone(), i_vars_realOptimizeFinalConstraintsVars.clone(), i_vars_realOptimizeConstraintsVars.clone(), i_vars_discreteAlgVars.clone(), i_vars_algVars.clone(), i_vars_derivativeVars.clone(), i_vars_stateVars.clone())?;
            txt = fun_68(txt.clone(), i_varInfo_numIntAlgVars.clone(), i_vars_intAlgVars.clone())?;
            txt = fun_70(txt.clone(), i_varInfo_numBoolAlgVars.clone(), i_vars_boolAlgVars.clone())?;
            txt = fun_72(txt.clone(), i_varInfo_numStringAlgVars.clone(), i_vars_stringAlgVars.clone())?;
            txt = fun_74(txt.clone(), i_varInfo_numParams.clone(), i_vars_paramVars.clone())?;
            txt = fun_76(txt.clone(), i_varInfo_numIntParams.clone(), i_vars_intParamVars.clone())?;
            txt = fun_78(txt.clone(), i_varInfo_numBoolParams.clone(), i_vars_boolParamVars.clone())?;
            txt = fun_80(txt.clone(), i_varInfo_numStringParamVars.clone(), i_vars_stringParamVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("};\n")).clone(), (literal!("\n")).clone(), (literal!("#include <math.h>\n")).clone(), (literal!("/* TODO: Generate used builtin functions before SimCode */\n")).clone(), (literal!("static inline double om_mod(double x, double y)\n")).clone(), (literal!("{\n")).clone(), (literal!("  return x-floor(x/y)*y;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = functionsFile(txt.clone(), i_functions.clone(), i_literals.clone(), i_externalFunctionIncludes.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("fmi2Component ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("fmi2Instantiate")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(fmi2String name, fmi2Type ty, fmi2String GUID, fmi2String resources, const fmi2CallbackFunctions* functions, fmi2Boolean visible, fmi2Boolean loggingOn)\n")).clone(), (literal!("{\n")).clone(), (literal!("  static int initDone=0;\n")).clone(), (literal!("  if (initDone) {\n")).clone(), (literal!("    return NULL;\n")).clone(), (literal!("  }\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return &")).clone() }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("component")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("fmi2Status ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("fmi2SetupExperiment")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(fmi2Component comp, fmi2Boolean toleranceDefined, fmi2Real tolerance, fmi2Real startTime, fmi2Boolean stopTimeDefined, fmi2Real stopTime)\n")).clone(), (literal!("{\n")).clone(), (literal!("  return fmi2OK;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("fmi2Status ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("fmi2EnterInitializationMode")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(fmi2Component comp)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = callExternalObjectConstructors(txt.clone(), i_extObjInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("return fmi2OK;\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("fmi2Status ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("fmi2ExitInitializationMode")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(fmi2Component comp)\n")).clone(), (literal!("{\n")).clone(), (literal!("  return fmi2OK;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("static fmi2Status ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("functionODE")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(fmi2Component comp)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = fun_82(txt.clone(), i_odeEquations.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("static fmi2Status ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("functionOutputs")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(fmi2Component comp)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = fun_86(txt.clone(), i_allEquations.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("fmi2Status ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("fmi2DoStep")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(fmi2Component comp, fmi2Real currentCommunicationPoint, fmi2Real communicationStepSize, fmi2Boolean noSetFMUStatePriorToCurrentPoint)\n")).clone(), (literal!("{\n")).clone(), (literal!("  comp->currentTime = currentCommunicationPoint;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = fun_87(txt.clone(), i_varInfo_numStateVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("/* TODO: Calculate time/state-dependent variables here... */\n")).clone() }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("functionOutputs")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(comp);\n")).clone(), (literal!("return fmi2OK;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("int main(int argc, char **argv)\n")).clone(), (literal!("{\n")).clone(), (literal!("  int terminateSimulation = 0;\n")).clone(), (literal!("  fmi2Status status = fmi2OK;\n")).clone(), (literal!("  fmi2CallbackFunctions cbf = {\n")).clone(), (literal!("  .logger = NULL,\n")).clone(), (literal!("  .allocateMemory = NULL /*calloc*/,\n")).clone(), (literal!("  .freeMemory = NULL /*free*/,\n")).clone(), (literal!("  .stepFinished = NULL, //synchronous execution\n")).clone(), (literal!("  .componentEnvironment = NULL\n")).clone(), (literal!("  };\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2Component comp = ")).clone() }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("fmi2Instantiate")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(\"\", fmi2CoSimulation, \"\", \"\", &cbf, fmi2False, fmi2False);\n")).clone(), (literal!("if (comp==NULL) {\n")).clone(), (literal!("  return 1;\n")).clone(), (literal!("}\n")).clone()], lastHasNewLine: true }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("fmi2SetupExperiment")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(comp, fmi2False, 0.0, ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_settings_startTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", fmi2False, ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_settings_stopTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("fmi2EnterInitializationMode")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(comp);\n")).clone(), (literal!("// Set start-values? Nah...\n")).clone()], lastHasNewLine: true }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("fmi2ExitInitializationMode")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(comp);\n")).clone(), (literal!("\n")).clone(), (literal!("double currentTime = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_settings_startTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("double h = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_settings_stepSize.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("uint32_t i = 0;\n")).clone(), (literal!("\n")).clone(), (literal!("while (status == fmi2OK) {\n")).clone(), (literal!("  //retrieve outputs\n")).clone(), (literal!("    // fmi2GetReal(m, ..., 1, &y1);\n")).clone(), (literal!("  //set inputs\n")).clone(), (literal!("    // fmi2SetReal(m, ..., 1, &y2);\n")).clone(), (literal!("\n")).clone(), (literal!("  //call slave and check status\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("status = ")).clone() }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(a_modelNamePrefixStr.clone())?).clone(), (literal!("fmi2DoStep")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(comp, currentTime, h, fmi2True);\n")).clone(), (literal!("switch (status) {\n")).clone(), (literal!("  case fmi2Discard:\n")).clone(), (literal!("  case fmi2Error:\n")).clone(), (literal!("  case fmi2Fatal:\n")).clone(), (literal!("  case fmi2Pending /* Cannot happen */:\n")).clone(), (literal!("    terminateSimulation = 1;\n")).clone(), (literal!("    break;\n")).clone(), (literal!("  case fmi2OK:\n")).clone(), (literal!("  case fmi2Warning:\n")).clone(), (literal!("    break;\n")).clone(), (literal!("}\n")).clone(), (literal!("if (terminateSimulation) {\n")).clone(), (literal!("  break;\n")).clone(), (literal!("}\n")).clone(), (literal!("i++;\n")).clone(), (literal!("/* increment master time */\n")).clone(), (literal!("currentTime = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_settings_startTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" + h*i;\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#if 0\n")).clone(), (literal!("  if ((status != fmi2Error) && (status != fmi2Fatal)) {\n")).clone(), (literal!("    fmi2Terminate(m);\n")).clone(), (literal!("  }\n")).clone(), (literal!("  if (status != fmi2Fatal) {\n")).clone(), (literal!("    fmi2FreeInstance(m);\n")).clone(), (literal!("  }\n")).clone(), (literal!("#endif\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn mainFile(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut l_modelNamePrefixStr: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    l_modelNamePrefixStr = CodegenUtilSimulation::modelNamePrefix(Tpl::emptyTxt.clone(), a_simCode.clone())?;
    out_txt = fun_88(txt.clone(), a_simCode.clone(), l_modelNamePrefixStr.clone())?;
    Ok(out_txt)
}

fn lm_90(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_90 in &*items.clone() {
        let mut lstElt_90 = lstElt_90.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_90.clone()) {
        i_stmt => {
            txt = statement(txt.clone(), i_stmt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn equation_(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { cref: i_cref, exp: i_exp, index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = cref(txt.clone(), i_cref.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; /* equation ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: i_statements, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_90(txt.clone(), i_statements.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 381, 14), (literal!("Unsupported equation: ...")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_92(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_92 in &*items.clone() {
        let mut lstElt_92 = lstElt_92.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_92.clone()) {
        i_stmt => {
            txt = statement(txt.clone(), i_stmt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn statement(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone())) {
        (txt, Deref @ DAE::Statement::STMT_ASSIGN { type_: Deref @ DAE::Type::T_ARRAY { ty: _, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 388, 16), (literal!("Array assignments are not supported")).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::CREF { componentRef: i_cr, .. }, exp: i_exp, .. }) => {
            let mut txt = (*txt).clone();
            txt = cref(txt.clone(), i_cr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Statement::STMT_NORETCALL { exp: i_exp, .. }) => {
            let mut txt = (*txt).clone();
            txt = daeExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Statement::STMT_IF { exp: i_exp, statementLst: i_statementLst, else_: i_else__, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_92(txt.clone(), i_statementLst.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt = elseStatement(txt.clone(), i_else__.clone())?;
            txt.clone()
        },
        (txt, i_stmt) => {
            let mut txt_0: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Unsupported statement: ")).clone() }))?;
            txt_0 = DAEDumpTpl::dumpStatement(txt_0.clone(), i_stmt.clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 399, 14), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_94(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_94 in &*items.clone() {
        let mut lstElt_94 = lstElt_94.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_94.clone()) {
        i_stmt => {
            txt = statement(txt.clone(), i_stmt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_95(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_95 in &*items.clone() {
        let mut lstElt_95 = lstElt_95.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_95.clone()) {
        i_stmt => {
            txt = statement(txt.clone(), i_stmt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn elseStatement(mut in_txt: Tpl::Text, mut in_a_else__: Arc<DAE::Else>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_else__.clone())) {
        (txt, Deref @ DAE::Else::NOELSE { .. }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Else::ELSEIF { exp: i_exp, statementLst: i_statementLst, else_: i_else__ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("else if (")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_94(txt.clone(), i_statementLst.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt = elseStatement(txt.clone(), i_else__.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Else::ELSE { statementLst: i_statementLst }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("else {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_95(txt.clone(), i_statementLst.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn cref(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->currentTime")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::WILD { .. }) => {
            txt.clone()
        },
        (txt, i_cr) => {
            let mut txt = (*txt).clone();
            txt = crefToCStr(txt.clone(), i_cr.clone(), 0, false)?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn crefLocal(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: i_ident, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("om_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 436, 14), (literal!("Only CREF_IDENT as local identifiers (for now)")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_99(mut in_txt: Tpl::Text, mut in_a_isPre: bool, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_ix: i32, mut in_a_componentRef: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_isPre.clone(), in_a_cr.clone(), in_a_ix.clone(), in_a_componentRef.clone())) {
        (txt, false, _, a_ix, a_componentRef) => {
            let mut txt = (*txt).clone();
            txt = crefToCStr(txt.clone(), a_componentRef.clone(), a_ix.clone(), true)?;
            txt.clone()
        },
        (txt, _, a_cr, _, _) => {
            let mut txt_0: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Got $PRE for something that is already pre: ")).clone() }))?;
            txt_0 = CodegenUtil::crefStr(txt_0.clone(), a_cr.clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 444, 26), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_100(mut in_txt: Tpl::Text, mut in_mArg: SimCodeVar::SimVar, mut in_a_isPre: bool, mut in_a_ix: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_isPre.clone(), in_a_ix.clone()) {
        (mut txt, SimCodeVar::SimVar { index: (-1), varKind: mut i_varKind, name: ref i_name, .. }, _, _) => {
            let mut txt_0: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("crefToCStr got index=-1 for ")).clone() }))?;
            txt_0 = CodegenUtil::variabilityString(txt_0.clone(), i_varKind.clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt_0 = CodegenUtil::crefStr(txt_0.clone(), i_name.clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 447, 43), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        (mut txt, ref i_var @ SimCodeVar::SimVar { index: ref i_index, name: ref i_name, varKind: ref i_varKind, .. }, mut a_isPre, mut a_ix) => {
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut txt_1: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            txt = varArrayNameValues(txt.clone(), i_var.clone(), a_ix.clone(), a_isPre.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] /* ")).clone() }))?;
            txt_1 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), i_name.clone())?;
            ret_2 = (Util::escapeModelicaStringToCString((Tpl::textString(txt_1.clone())?).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_2.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = CodegenUtil::variabilityString(txt.clone(), i_varKind.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_NOT_IDENT_OR_QUAL")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn crefToCStr(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_ix: i32, mut in_a_isPre: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone(), in_a_ix.clone(), in_a_isPre.clone())) {
        (txt, i_cr @ Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$PRE", subscriptLst: Deref @ metamodelica::List::Nil, componentRef: i_componentRef, .. }, a_ix, a_isPre) => {
            let mut txt = (*txt).clone();
            txt = fun_99(txt.clone(), a_isPre.clone(), i_cr.clone(), a_ix.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, i_cr, a_ix, a_isPre) => {
            let mut ret_1: SimCodeVar::SimVar = <SimCodeVar::SimVar as ::std::default::Default>::default();
            let mut ret_0: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            ret_0 = SimCodeUtil::getSimCode()?;
            ret_1 = SimCodeUtil::cref2simvar(i_cr.clone(), ret_0.clone())?;
            txt = fun_100(txt.clone(), ret_1.clone(), a_isPre.clone(), a_ix.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefShortType(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { identType: i_identType, .. }) => {
            let mut txt = (*txt).clone();
            txt = expTypeShort(txt.clone(), i_identType.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = crefShortType(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("crefType:ERROR")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn expTypeShort(mut in_txt: Tpl::Text, mut in_a_type: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2Integer")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2Real")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2String")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2Boolean")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2Integer")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: i_complexType, .. }) => {
            let mut txt = (*txt).clone();
            txt = expTypeShort(txt.clone(), i_complexType.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_ty, .. }) => {
            let mut txt = (*txt).clone();
            txt = expTypeShort(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void*")).clone() }))?;
            txt.clone()
        },
        (txt, i_type) => {
            let mut txt_0: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("expTypeShort: ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(i_type.clone())?).clone();
            txt_0 = Tpl::writeStr(txt_0.clone(), (ret_0.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 474, 14), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_104(mut in_txt: Tpl::Text, mut in_a_bool: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_bool.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2False")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2True")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn daeExp(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone())) {
        (txt, Deref @ DAE::Exp::ICONST { integer: i_integer }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_integer.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RCONST { real: i_real }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (realString(i_real.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SCONST { string: i_string }) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToCString((i_string.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: i_bool }) => {
            let mut txt = (*txt).clone();
            txt = fun_104(txt.clone(), i_bool.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("!(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ DAE::Exp::BINARY { exp1: i_exp1, operator: i_operator, exp2: i_exp2 }) => {
            let mut txt = (*txt).clone();
            txt = daeExpBinary(txt.clone(), i_exp1.clone(), i_operator.clone(), i_exp2.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ DAE::Exp::RELATION { exp1: i_exp1, operator: i_operator, exp2: i_exp2, .. }) => {
            let mut txt = (*txt).clone();
            txt = daeExpBinary(txt.clone(), i_exp1.clone(), i_operator.clone(), i_exp2.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::IFEXP { expCond: i_expCond, expThen: i_expThen, expElse: i_expElse }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), i_expCond.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") ? (")).clone() }))?;
            txt = daeExp(txt.clone(), i_expThen.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") : (")).clone() }))?;
            txt = daeExp(txt.clone(), i_expElse.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = daeExpCallBuiltin(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ DAE::Exp::CALL { path: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = daeExpCall(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: _, .. }, .. }) => {
            let mut txt_1: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF array... ")).clone() }))?;
            txt_1 = ExpressionDumpTpl::dumpExp(txt_1.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 492, 40), (Tpl::textString(txt_1.clone())?).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = cref(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_REAL { varLst: _ }, exp: i_e }) => {
            let mut txt = (*txt).clone();
            txt = daeExp(txt.clone(), i_e.clone())?;
            txt.clone()
        },
        (txt, i_exp) => {
            let mut txt_2: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt_2 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExp: Not supporting ")).clone() }))?;
            txt_2 = ExpressionDumpTpl::dumpExp(txt_2.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 495, 14), (Tpl::textString(txt_2.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_106(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator, mut in_a_exp1: Arc<DAE::Exp>, mut in_a_exp2: Arc<DAE::Exp>, mut in_a_origExp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_op.clone(), in_a_exp1.clone(), in_a_exp2.clone(), in_a_origExp.clone())) {
        (txt, DAE::Operator::ADD { ty: _ }, a_exp1, a_exp2, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")+(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::SUB { ty: _ }, a_exp1, a_exp2, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")-(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::MUL { ty: _ }, a_exp1, a_exp2, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")*(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::DIV { ty: _ }, a_exp1, a_exp2, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")/(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::POW { ty: _ }, a_exp1, a_exp2, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("pow((")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("),(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::GREATER { ty: _ }, a_exp1, a_exp2, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")>(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::GREATEREQ { ty: _ }, a_exp1, a_exp2, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")>=(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::LESS { ty: _ }, a_exp1, a_exp2, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")<(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::EQUAL { ty: _ }, a_exp1, a_exp2, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")==(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::NEQUAL { ty: _ }, a_exp1, a_exp2, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")!=(")).clone() }))?;
            txt = daeExp(txt.clone(), a_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, a_origExp) => {
            let mut txt_0: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary: Not supporting operator? ")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(txt_0.clone(), a_origExp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 514, 14), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn daeExpBinary(mut txt: Tpl::Text, mut a_exp1: Arc<DAE::Exp>, mut a_op: DAE::Operator, mut a_exp2: Arc<DAE::Exp>, mut a_origExp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = fun_106(txt.clone(), a_op.clone(), a_exp1.clone(), a_exp2.clone(), a_origExp.clone())?;
    Ok(out_txt)
}

pub fn daeExpCallBuiltin(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone())) {
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Cons { head: i_exp2, tail: _ } }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")/(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: i_exp2, tail: Deref @ metamodelica::List::Nil } }, .. }) => {
            let mut txt = (*txt).clone();
            txt = daeExp(txt.clone(), i_exp2.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("((int)")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fabs(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Cons { head: i_exp2, tail: Deref @ metamodelica::List::Nil } }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_REAL { varLst: _ }, .. } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmin(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Cons { head: i_exp2, tail: Deref @ metamodelica::List::Nil } }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_REAL { varLst: _ }, .. } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmax(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: i_name @ Deref @ "sin" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: i_name @ Deref @ "cos" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: i_name @ Deref @ "tan" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: i_name @ Deref @ "asin" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: i_name @ Deref @ "acos" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: i_name @ Deref @ "atan" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: i_name @ Deref @ "sinh" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: i_name @ Deref @ "cosh" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: i_name @ Deref @ "tanh" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: i_name @ Deref @ "exp" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: i_name @ Deref @ "log" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: i_name @ Deref @ "log10" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: i_name @ Deref @ "atan2" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Cons { head: i_exp2, tail: Deref @ metamodelica::List::Nil } }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Cons { head: i_exp2, tail: _ } }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("om_mod(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("floor(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ceil(")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, expLst: Deref @ metamodelica::List::Cons { head: i_exp1, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut txt = (*txt).clone();
            txt = daeExp(txt.clone(), i_exp1.clone())?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ DAE::Exp::CALL { path: _, .. }) => {
            let mut txt_0: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpCallBuiltin: Not supported: ")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(txt_0.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 549, 28), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_109(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_109 in &*items.clone() {
        let mut lstElt_109 = lstElt_109.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_109.clone()) {
        i_e => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = daeExp(txt.clone(), i_e.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn daeExpCall(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone())) {
        (txt, Deref @ DAE::Exp::CALL { path: i_path, attr: Deref @ DAE::CallAttributes { ty: _, .. }, expLst: i_expLst }) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::underscorePath(txt.clone(), i_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(comp")).clone() }))?;
            txt = lm_109(txt.clone(), i_expLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ DAE::Exp::CALL { path: _, .. }) => {
            let mut txt_0: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpCall: Not supported: ")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(txt_0.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 556, 28), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_111(mut in_txt: Tpl::Text, mut in_a_isPre: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_isPre.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("XXXPreVars???")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_112(mut in_txt: Tpl::Text, mut in_a_isPre: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_isPre.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Pre")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_113(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar, mut in_a_isPre: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_var.clone(), in_a_isPre.clone()) {
        (mut txt, SimCodeVar::SimVar { varKind: BackendDAE::VarKind::PARAM { .. }, name: ref i_name, .. }, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->")).clone() }))?;
            txt = crefShortType(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Parameter")).clone() }))?;
            txt.clone()
        },
        (mut txt, SimCodeVar::SimVar { varKind: BackendDAE::VarKind::OPT_TGRID { .. }, name: ref i_name, .. }, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->")).clone() }))?;
            txt = crefShortType(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Parameter")).clone() }))?;
            txt.clone()
        },
        (mut txt, SimCodeVar::SimVar { varKind: BackendDAE::VarKind::EXTOBJ { fullClassName: _ }, .. }, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->extObjs")).clone() }))?;
            txt.clone()
        },
        (mut txt, SimCodeVar::SimVar { name: ref i_name, .. }, mut a_isPre) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp")).clone() }))?;
            txt = fun_111(txt.clone(), a_isPre.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("->")).clone() }))?;
            txt = crefShortType(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Vars")).clone() }))?;
            txt = fun_112(txt.clone(), a_isPre.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_114(mut in_txt: Tpl::Text, mut in_a_ix: i32, mut in_a_var: SimCodeVar::SimVar, mut in_a_isPre: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_ix.clone(), in_a_var.clone(), in_a_isPre.clone()) {
        (mut txt, 0, mut a_var, mut a_isPre) => {
            txt = fun_113(txt.clone(), a_var.clone(), a_isPre.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 568, 14), (literal!("varArrayNameValues ix>0")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn varArrayNameValues(mut txt: Tpl::Text, mut a_var: SimCodeVar::SimVar, mut a_ix: i32, mut a_isPre: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = fun_114(txt.clone(), a_ix.clone(), a_var.clone(), a_isPre.clone())?;
    Ok(out_txt)
}

fn fun_116(mut in_txt: Tpl::Text, mut in_a_bool: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_bool.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2False")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2True")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_117(mut in_txt: Tpl::Text, mut in_a_ty__: Arc<DAE::Type>, mut in_a_value: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty__.clone(), in_a_value.clone())) {
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0.0")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_value) => {
            let mut txt_0: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("No start value for variable... ")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(txt_0.clone(), a_value.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 582, 16), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn constVal(mut in_txt: Tpl::Text, mut in_a_value: Arc<DAE::Exp>, mut in_a_ty__: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_value.clone(), in_a_ty__.clone())) {
        (txt, Deref @ DAE::Exp::ICONST { integer: i_integer }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_integer.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RCONST { real: i_real }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (realString(i_real.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SCONST { string: i_string }, _) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToCString((i_string.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: i_bool }, _) => {
            let mut txt = (*txt).clone();
            txt = fun_116(txt.clone(), i_bool.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { index: i_index, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, i_value, a_ty__) => {
            let mut txt = (*txt).clone();
            txt = fun_117(txt.clone(), a_ty__.clone(), i_value.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn startValue(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, SimCodeVar::SimVar { initialValue: Some(i_e), type_: i_ty, name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = constVal(txt.clone(), i_e.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" /*")).clone() }))?;
            txt = CodegenUtil::crefStr(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/,")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (txt, SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_REAL { varLst: _ }, name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0.0 /*")).clone() }))?;
            txt = CodegenUtil::crefStr(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/,")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (txt, SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_INTEGER { varLst: _ }, name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0 /*")).clone() }))?;
            txt = CodegenUtil::crefStr(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/,")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (txt, SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_BOOL { varLst: _ }, name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi2False /*")).clone() }))?;
            txt = CodegenUtil::crefStr(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/,")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (txt, SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_STRING { varLst: _ }, name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"\" /*")).clone() }))?;
            txt = CodegenUtil::crefStr(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/,")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (txt, SimCodeVar::SimVar { name: i_name, .. }) => {
            let mut txt_0: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("No start value for variable ")).clone() }))?;
            txt_0 = CodegenUtil::crefStr(txt_0.clone(), i_name.clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 594, 30), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_120(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_120 in &*items.clone() {
        let mut lstElt_120 = lstElt_120.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_120.clone()) {
        i_literal => {
            let mut x_i0: i32 = 0;
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = literalExpConst(txt.clone(), i_literal.clone(), x_i0.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_121(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_121 in &*items.clone() {
        let mut lstElt_121 = lstElt_121.clone();
        txt = (match lstElt_121.clone() {
        mut i_inc => {
            txt = Tpl::writeStr(txt.clone(), (i_inc.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_122(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_122 in &*items.clone() {
        let mut lstElt_122 = lstElt_122.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_122.clone()) {
        i_func => {
            txt = functionDeclaration(txt.clone(), i_func.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_123(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_123 in &*items.clone() {
        let mut lstElt_123 = lstElt_123.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_123.clone()) {
        i_func => {
            txt = functionBody(txt.clone(), i_func.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn functionsFile(mut txt: Tpl::Text, mut a_functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>, mut a_literals: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_externalFunctionIncludes: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_120(out_txt.clone(), a_literals.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_121(out_txt.clone(), a_externalFunctionIncludes.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_122(out_txt.clone(), a_functions.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_123(out_txt.clone(), a_functions.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn functionBody(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone())) {
        (txt, i_fn @ Deref @ SimCodeFunction::Function::FUNCTION { name: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = functionBodyRegularFunction(txt.clone(), i_fn.clone())?;
            txt.clone()
        },
        (txt, i_fn @ Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { name: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = functionBodyExternalFunction(txt.clone(), i_fn.clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Function::RECORD_CONSTRUCTOR { name: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 620, 57), (literal!("No records in embedded C yet")).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Function::KERNEL_FUNCTION { name: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 621, 57), (literal!("No kernel functions in embedded C")).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn functionDeclaration(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone())) {
        (txt, Deref @ SimCodeFunction::Function::FUNCTION { name: i_name, functionArguments: i_functionArguments, outVars: i_outVars, .. }) => {
            let mut txt_0: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt_0 = CodegenUtil::underscorePath(Tpl::emptyTxt.clone(), i_name.clone())?;
            (txt, txt_0) = functionPrototype(txt.clone(), txt_0.clone(), i_functionArguments.clone(), i_outVars.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { name: i_name, funArgs: i_funArgs, outVars: i_outVars, .. }) => {
            let mut txt_1: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("static inline ")).clone() }))?;
            txt_1 = CodegenUtil::underscorePath(Tpl::emptyTxt.clone(), i_name.clone())?;
            (txt, txt_1) = functionPrototype(txt.clone(), txt_1.clone(), i_funArgs.clone(), i_outVars.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Function::RECORD_CONSTRUCTOR { name: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 630, 57), (literal!("No records in embedded C yet")).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Function::KERNEL_FUNCTION { name: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 631, 57), (literal!("No kernel functions in embedded C")).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_127(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_127 in &*items.clone() {
        let mut lstElt_127 = lstElt_127.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_127.clone()) {
        i_stmt => {
            txt = statement(txt.clone(), i_stmt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_128(mut in_txt: Tpl::Text, mut in_a_outVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_outVars.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: i_v, tail: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return ")).clone() }))?;
            txt = varName(txt.clone(), i_v.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn functionBodyRegularFunction(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone())) {
        (txt, Deref @ SimCodeFunction::Function::FUNCTION { name: i_name, functionArguments: i_functionArguments, outVars: i_outVars, body: i_body, .. }) => {
            let mut l_bodyPart: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_prototype: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_fname: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_fname = CodegenUtil::underscorePath(Tpl::emptyTxt.clone(), i_name.clone())?;
            (l_prototype, l_fname) = functionPrototype(Tpl::emptyTxt.clone(), l_fname.clone(), i_functionArguments.clone(), i_outVars.clone())?;
            l_bodyPart = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_bodyPart = lm_127(l_bodyPart.clone(), i_body.clone())?;
            l_bodyPart = Tpl::popIter(l_bodyPart.clone())?;
            txt = Tpl::writeText(txt.clone(), l_prototype.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("{\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_bodyPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = fun_128(txt.clone(), i_outVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_130(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCodeFunction::SimExtArg::SimExtArg>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_130 in &*items.clone() {
        let mut lstElt_130 = lstElt_130.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_130.clone()) {
        i_arg => {
            txt = extArg(txt.clone(), i_arg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_131(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_131 in &*items.clone() {
        let mut lstElt_131 = lstElt_131.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_131.clone()) {
        i_arg => {
            txt = varType(txt.clone(), i_arg.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = varName(txt.clone(), i_arg.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_132(mut in_txt: Tpl::Text, mut in_a_extReturn: Arc<SimCodeFunction::SimExtArg::SimExtArg>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_extReturn.clone())) {
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { cref: i_c, .. }) => {
            let mut txt = (*txt).clone();
            txt = crefLocal(txt.clone(), i_c.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_133(mut in_txt: Tpl::Text, mut in_a_outVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_outVars.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ SimCodeFunction::Variable::VARIABLE { name: i_cref, .. }, tail: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return ")).clone() }))?;
            txt = crefLocal(txt.clone(), i_cref.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 673, 16), (literal!("Not variable return")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_134(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_134 in &*items.clone() {
        let mut lstElt_134 = lstElt_134.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_134.clone()) {
        i_var => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*out")).clone() }))?;
            txt = varName(txt.clone(), i_var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = varName(txt.clone(), i_var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_135(mut in_txt: Tpl::Text, mut in_a_outVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_outVars.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_outVars) => {
            let mut ret_0: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
            let mut txt = (*txt).clone();
            ret_0 = listRest(i_outVars.clone())?;
            txt = lm_134(txt.clone(), ret_0.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn functionBodyExternalFunction(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone())) {
        (txt, Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { language: Deref @ "C", name: i_name, funArgs: i_funArgs, outVars: i_outVars, extArgs: i_extArgs, extReturn: i_extReturn, extName: i_extName, .. }) => {
            let mut l_varAssign: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_returnStatement: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_returnAssign: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_varDecl: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_args: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_prototype: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_fname: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_fname = CodegenUtil::underscorePath(Tpl::emptyTxt.clone(), i_name.clone())?;
            (l_prototype, l_fname) = functionPrototype(Tpl::emptyTxt.clone(), l_fname.clone(), i_funArgs.clone(), i_outVars.clone())?;
            l_args = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_args = lm_130(l_args.clone(), i_extArgs.clone())?;
            l_args = Tpl::popIter(l_args.clone())?;
            l_varDecl = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_varDecl = lm_131(l_varDecl.clone(), i_outVars.clone())?;
            l_varDecl = Tpl::popIter(l_varDecl.clone())?;
            l_returnAssign = fun_132(Tpl::emptyTxt.clone(), i_extReturn.clone())?;
            l_returnStatement = fun_133(Tpl::emptyTxt.clone(), i_outVars.clone())?;
            l_varAssign = fun_135(Tpl::emptyTxt.clone(), i_outVars.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("static inline ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_prototype.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("{\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_varDecl.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_returnAssign.clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_extName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_varAssign.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_returnStatement.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 685, 14), (literal!("Unknown external language")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn extArg(mut in_txt: Tpl::Text, mut in_a_extArg: Arc<SimCodeFunction::SimExtArg::SimExtArg>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_extArg.clone())) {
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { isInput: true, isArray: false, cref: i_cref, .. }) => {
            let mut txt = (*txt).clone();
            txt = crefLocal(txt.clone(), i_cref.clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARGEXP { type_: Deref @ DAE::Type::T_REAL { varLst: _ }, exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = daeExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARGEXP { type_: Deref @ DAE::Type::T_INTEGER { varLst: _ }, exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = daeExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARGEXP { type_: Deref @ DAE::Type::T_STRING { varLst: _ }, exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = daeExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARGEXP { type_: Deref @ DAE::Type::T_BOOL { varLst: _ }, exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = daeExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 696, 14), (literal!("Unknown extArg")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_138(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_138 in &*items.clone() {
        let mut lstElt_138 = lstElt_138.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_138.clone()) {
        i_var => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = varType(txt.clone(), i_var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = varName(txt.clone(), i_var.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_139(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, i_var @ Deref @ SimCodeFunction::Variable::VARIABLE { name: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = varType(txt.clone(), i_var.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 705, 18), (literal!("modelica_fnptr")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_140(mut in_txt: Tpl::Text, mut in_a_outVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_outVars.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: _ }) => {
            let mut txt = (*txt).clone();
            txt = fun_139(txt.clone(), i_var.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_141(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_141 in &*items.clone() {
        let mut lstElt_141 = lstElt_141.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_141.clone()) {
        i_var => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = varType(txt.clone(), i_var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" *out")).clone() }))?;
            txt = varName(txt.clone(), i_var.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_142(mut in_txt: Tpl::Text, mut in_a_outVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_outVars.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_outVars) => {
            let mut ret_0: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>> = metamodelica::nil();
            let mut txt = (*txt).clone();
            ret_0 = listRest(i_outVars.clone())?;
            txt = lm_141(txt.clone(), ret_0.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn functionPrototype(mut txt: Tpl::Text, mut a_fname: Tpl::Text, mut a_fargs: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>, mut a_outVars: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut out_a_fname: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut l_outargs: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut l_outarg: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut l_fargsStr: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    l_fargsStr = lm_138(Tpl::emptyTxt.clone(), a_fargs.clone())?;
    l_outarg = fun_140(Tpl::emptyTxt.clone(), a_outVars.clone())?;
    l_outargs = fun_142(Tpl::emptyTxt.clone(), a_outVars.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_outarg.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), a_fname.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fmi2Component comp")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_fargsStr.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_outargs.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
    out_a_fname = a_fname.clone();
    Ok((out_txt, out_a_fname))
}

pub fn varName(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, Deref @ SimCodeFunction::Variable::VARIABLE { name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = crefLocal(txt.clone(), i_name.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 715, 14), (literal!("Not VARIABLE(__)")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn varType(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, Deref @ SimCodeFunction::Variable::VARIABLE { ty: i_ty, .. }) => {
            let mut txt = (*txt).clone();
            txt = expTypeShort(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 722, 14), (literal!("Not VARIABLE(__)")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn literalExpConst(mut in_txt: Tpl::Text, mut in_a_e: Arc<DAE::Exp>, mut in_a_i0: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_e.clone(), in_a_i0.clone())) {
        (txt, Deref @ DAE::Exp::SCONST { string: i_string }, a_i0) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("static const char * const OMCLIT")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_i0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = \"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToCString((i_string.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\";")).clone() }))?;
            txt.clone()
        },
        (txt, i_e, _) => {
            let mut txt_1: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Literal expression: ")).clone() }))?;
            txt_1 = ExpressionDumpTpl::dumpExp(txt_1.clone(), i_e.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenEmbeddedC.tpl")).clone(), 729, 14), (Tpl::textString(txt_1.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_147(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_147 in &*items.clone() {
        let mut lstElt_147 = lstElt_147.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_147.clone()) {
        SimCodeVar::SimVar { initialValue: Some(i_exp), name: i_var_name, .. } => {
            txt = cref(txt.clone(), i_var_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = daeExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn callExternalObjectConstructors(mut in_txt: Tpl::Text, mut in_a_extObjInfo: SimCode::ExtObjInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_extObjInfo.clone()) {
        (mut txt, SimCode::ExtObjInfo { vars: ref i_vars, .. }) => {
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_147(txt.clone(), i_vars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_149(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_149 in &*items.clone() {
        let mut lstElt_149 = lstElt_149.clone();
        txt = (match lstElt_149.clone() {
        SimCodeVar::SimVar { varKind: BackendDAE::VarKind::EXTOBJ { fullClassName: ref i_ext_fullClassName }, name: ref i_var_name, .. } => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omc_")).clone() }))?;
            txt = CodegenUtil::underscorePath(txt.clone(), i_ext_fullClassName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_destructor(threadData,")).clone() }))?;
            txt = cref(txt.clone(), i_var_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
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

pub fn callExternalObjectDestructors(mut in_txt: Tpl::Text, mut in_a_extObjInfo: SimCode::ExtObjInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_extObjInfo.clone()) {
        (mut txt, SimCode::ExtObjInfo { vars: ref i_vars, .. }) => {
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_149(txt.clone(), i_vars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

