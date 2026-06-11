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
use openmodelica_backend::MidCode;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_tpl::Tpl;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

fn lm_43(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Function>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_43 in &*items {
        let mut lstElt_43 = lstElt_43.clone();
        txt = (match lstElt_43.clone() {
        mut i_fn => {
            txt = genFunction(txt.clone(), i_fn.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

pub fn genProgram(mut in_txt: Tpl::Text, mut in_a_p: MidCode::Program) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_p) {
        (mut txt, MidCode::Program { functions: ref i_functions, name: mut i_name }) => {
            let mut ret_0: i32;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("// number of functions: ")).clone() }))?;
            ret_0 = (i_functions.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".h\"\n")).clone(), (literal!("#include \"util/modelica.h\"\n")).clone(), (literal!("\n")).clone(), (literal!("#include \"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_includes.h\"\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_43(txt.clone(), i_functions.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_45(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_outputs)) {
        (txt, false, a_outputs) => {
            let mut ret_0: MidCode::Var;
            let mut txt = (*txt).clone();
            ret_0 = listHead(a_outputs.clone())?;
            txt = genVarType(txt.clone(), ret_0)?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_46(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_46 in &*items {
        let mut lstElt_46 = lstElt_46.clone();
        txt = (match lstElt_46.clone() {
        mut i_i => {
            txt = genVarType(txt.clone(), i_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = genVarName(txt.clone(), i_i.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_47(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_47 in &*items {
        let mut lstElt_47 = lstElt_47.clone();
        txt = (match lstElt_47.clone() {
        mut i_o => {
            txt = genVarType(txt.clone(), i_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" *outPtr_")).clone() }))?;
            txt = genVarName(txt.clone(), i_o.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn smf_48(mut in_txt: Tpl::Text, mut in_it: Arc<Tpl::StringToken>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_it)) {
        (txt, i_it) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn smf_49(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_50(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn genFunction(mut in_txt: Tpl::Text, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_fn) {
        (mut txt, ref i_fn @ MidCode::Function { outputs: ref i_outputs, inputs: ref i_inputs, name: ref i_name, locals: ref i_locals, localBufs: ref i_localBufs, localBufPtrs: ref i_localBufPtrs, body: ref i_body, .. }) => {
            let mut txt_4: Tpl::Text;
            let mut ret_4: Arc<metamodelica::List<MidCode::Var>>;
            let mut txt_3: Tpl::Text;
            let mut l_arguments: Tpl::Text;
            let mut ret_1: bool;
            let mut l_returnType: Tpl::Text;
            ret_1 = i_outputs.clone().is_empty();
            l_returnType = fun_45(Tpl::emptyTxt.clone(), ret_1, i_outputs.clone())?;
            txt_3 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt_3 = lm_46(txt_3, i_inputs.clone())?;
            txt_3 = Tpl::popIter(txt_3)?;
            ret_4 = List::restOrEmpty(i_outputs.clone())?;
            txt_4 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt_4 = lm_47(txt_4, ret_4)?;
            txt_4 = Tpl::popIter(txt_4)?;
            l_arguments = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",\n")).clone(), (literal!("    ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_arguments = smf_48(l_arguments, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threadData_t *threadData")).clone() }))?;
            l_arguments = smf_49(l_arguments, txt_3)?;
            l_arguments = smf_50(l_arguments, txt_4)?;
            l_arguments = Tpl::popIter(l_arguments)?;
            txt = Tpl::writeText(txt.clone(), l_returnType)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" omc_")).clone() }))?;
            txt = underscorePath(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arguments)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 3 }))?;
            txt = genLocalDecls(txt.clone(), i_fn.clone(), i_locals.clone(), i_localBufs.clone(), i_localBufPtrs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = genEntry(txt.clone(), i_fn.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = genBlocks(txt.clone(), i_fn.clone(), i_body.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = genExit(txt.clone(), i_fn.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = genInFunction(txt.clone(), i_fn.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = genBoxPtrFunction(txt.clone(), i_fn.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_52(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_52 in &*items {
        let mut lstElt_52 = lstElt_52.clone();
        txt = (match lstElt_52.clone() {
        mut i_i => {
            txt = genVarType(txt.clone(), i_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = genVarName(txt.clone(), i_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_53(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_53 in &*items {
        let mut lstElt_53 = lstElt_53.clone();
        txt = (match lstElt_53.clone() {
        mut i_o => {
            txt = genVarType(txt.clone(), i_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = genVarName(txt.clone(), i_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_54(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_54 in &*items {
        let mut lstElt_54 = lstElt_54.clone();
        txt = (match lstElt_54.clone() {
        mut i_i => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if(")).clone() }))?;
            txt = varModelicaRead(txt.clone(), i_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") return 1;")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_55(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_55 in &*items {
        let mut lstElt_55 = lstElt_55.clone();
        txt = (match lstElt_55.clone() {
        mut i_o => {
            txt = varModelicaWrite(txt.clone(), i_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_56(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_outputs)) {
        (txt, false, a_outputs) => {
            let mut ret_0: MidCode::Var;
            let mut txt = (*txt).clone();
            ret_0 = listHead(a_outputs.clone())?;
            txt = genVarName(txt.clone(), ret_0)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_57(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_57 in &*items {
        let mut lstElt_57 = lstElt_57.clone();
        txt = (match lstElt_57.clone() {
        mut i_i => {
            txt = genVarName(txt.clone(), i_i.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_58(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_58 in &*items {
        let mut lstElt_58 = lstElt_58.clone();
        txt = (match lstElt_58.clone() {
        mut i_o => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("&")).clone() }))?;
            txt = genVarName(txt.clone(), i_o.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn smf_59(mut in_txt: Tpl::Text, mut in_it: Arc<Tpl::StringToken>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_it)) {
        (txt, i_it) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn smf_60(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_61(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_62(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_noretcall(outVar);")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn genInFunction(mut in_txt: Tpl::Text, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_fn) {
        (mut txt, MidCode::Function { inputs: ref i_inputs, outputs: ref i_outputs, name: ref i_name, .. }) => {
            let mut ret_10: bool;
            let mut txt_8: Tpl::Text;
            let mut ret_8: Arc<metamodelica::List<MidCode::Var>>;
            let mut txt_7: Tpl::Text;
            let mut l_callargs: Tpl::Text;
            let mut ret_5: bool;
            let mut l_callretval: Tpl::Text;
            let mut l_outputLines: Tpl::Text;
            let mut l_inputLines: Tpl::Text;
            let mut l_outputDefs: Tpl::Text;
            let mut l_inputDefs: Tpl::Text;
            l_inputDefs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_inputDefs = lm_52(l_inputDefs, i_inputs.clone())?;
            l_inputDefs = Tpl::popIter(l_inputDefs)?;
            l_outputDefs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_outputDefs = lm_53(l_outputDefs, i_outputs.clone())?;
            l_outputDefs = Tpl::popIter(l_outputDefs)?;
            l_inputLines = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_inputLines = lm_54(l_inputLines, i_inputs.clone())?;
            l_inputLines = Tpl::popIter(l_inputLines)?;
            l_outputLines = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_outputLines = lm_55(l_outputLines, i_outputs.clone())?;
            l_outputLines = Tpl::popIter(l_outputLines)?;
            ret_5 = i_outputs.clone().is_empty();
            l_callretval = fun_56(Tpl::emptyTxt.clone(), ret_5, i_outputs.clone())?;
            txt_7 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt_7 = lm_57(txt_7, i_inputs.clone())?;
            txt_7 = Tpl::popIter(txt_7)?;
            ret_8 = List::restOrEmpty(i_outputs.clone())?;
            txt_8 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt_8 = lm_58(txt_8, ret_8)?;
            txt_8 = Tpl::popIter(txt_8)?;
            l_callargs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",\n")).clone(), (literal!("    ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_callargs = smf_59(l_callargs, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threadData")).clone() }))?;
            l_callargs = smf_60(l_callargs, txt_7)?;
            l_callargs = smf_61(l_callargs, txt_8)?;
            l_callargs = Tpl::popIter(l_callargs)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int in_")).clone() }))?;
            txt = underscorePath(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(threadData_t *threadData, type_description *inArgs, type_description *outVar)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_inputDefs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_outputDefs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_inputLines)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("MMC_TRY_TOP_INTERNAL()\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_callretval)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omc_")).clone() }))?;
            txt = underscorePath(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_callargs)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(");\n")).clone(), (literal!("MMC_CATCH_TOP(return 1)\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_outputLines)?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_10 = i_outputs.clone().is_empty();
            txt = fun_62(txt.clone(), ret_10)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fflush(NULL);\n")).clone(), (literal!("return 0;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_64(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_65(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_65 in &*items {
        let mut lstElt_65 = lstElt_65.clone();
        txt = (match lstElt_65.clone() {
        mut i_i => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype ")).clone() }))?;
            txt = genVarName(txt.clone(), i_i.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_66(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_66 in &*items {
        let mut lstElt_66 = lstElt_66.clone();
        txt = (match lstElt_66.clone() {
        mut i_o => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype *out_")).clone() }))?;
            txt = genVarName(txt.clone(), i_o.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn smf_67(mut in_txt: Tpl::Text, mut in_it: Arc<Tpl::StringToken>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_it)) {
        (txt, i_it) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn smf_68(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_69(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_70(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text, mut in_a_i: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_i)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, _, a_i) => {
            let mut txt = (*txt).clone();
            txt = genVarType(txt.clone(), a_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" unbox_")).clone() }))?;
            txt = genVarName(txt.clone(), a_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_71(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_71 in &*items {
        let mut lstElt_71 = lstElt_71.clone();
        txt = (match lstElt_71.clone() {
        mut i_i => {
            let mut txt_0: Tpl::Text;
            txt_0 = varBoxType(Tpl::emptyTxt.clone(), i_i.clone())?;
            txt = fun_70(txt.clone(), txt_0.clone(), i_i.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_72(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text, mut in_a_o: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_o)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, _, a_o) => {
            let mut txt = (*txt).clone();
            txt = genVarType(txt.clone(), a_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = genVarName(txt.clone(), a_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_73(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_73 in &*items {
        let mut lstElt_73 = lstElt_73.clone();
        txt = (match lstElt_73.clone() {
        mut i_o => {
            let mut txt_0: Tpl::Text;
            txt_0 = varBoxType(Tpl::emptyTxt.clone(), i_o.clone())?;
            txt = fun_72(txt.clone(), txt_0.clone(), i_o.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_74(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_outputs)) {
        (txt, false, a_outputs) => {
            let mut ret_0: MidCode::Var;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype out_")).clone() }))?;
            ret_0 = listHead(a_outputs.clone())?;
            txt = genVarName(txt.clone(), ret_0)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
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

fn fun_75(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text, mut in_a_i: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_i)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, _, a_i) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unbox_")).clone() }))?;
            txt = genVarName(txt.clone(), a_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = varUnbox2(txt.clone(), a_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_76(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_76 in &*items {
        let mut lstElt_76 = lstElt_76.clone();
        txt = (match lstElt_76.clone() {
        mut i_i => {
            let mut txt_0: Tpl::Text;
            txt_0 = varBoxType(Tpl::emptyTxt.clone(), i_i.clone())?;
            txt = fun_75(txt.clone(), txt_0.clone(), i_i.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_77(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("out_")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_78(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_outputs)) {
        (txt, false, a_outputs) => {
            let mut ret_2: MidCode::Var;
            let mut txt_1: Tpl::Text;
            let mut ret_0: MidCode::Var;
            let mut txt = (*txt).clone();
            ret_0 = listHead(a_outputs.clone())?;
            txt_1 = varBoxType(Tpl::emptyTxt.clone(), ret_0)?;
            txt = fun_77(txt.clone(), txt_1)?;
            ret_2 = listHead(a_outputs.clone())?;
            txt = genVarName(txt.clone(), ret_2)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_79(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unbox_")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_80(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_80 in &*items {
        let mut lstElt_80 = lstElt_80.clone();
        txt = (match lstElt_80.clone() {
        mut i_i => {
            let mut txt_0: Tpl::Text;
            txt_0 = varBoxType(Tpl::emptyTxt.clone(), i_i.clone())?;
            txt = fun_79(txt.clone(), txt_0.clone())?;
            txt = genVarName(txt.clone(), i_i.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_81(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("out_")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("&")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_82(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_82 in &*items {
        let mut lstElt_82 = lstElt_82.clone();
        txt = (match lstElt_82.clone() {
        mut i_o => {
            let mut txt_0: Tpl::Text;
            txt_0 = varBoxType(Tpl::emptyTxt.clone(), i_o.clone())?;
            txt = fun_81(txt.clone(), txt_0.clone())?;
            txt = genVarName(txt.clone(), i_o.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn smf_83(mut in_txt: Tpl::Text, mut in_it: Arc<Tpl::StringToken>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_it)) {
        (txt, i_it) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn smf_84(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_85(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_86(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_outputs)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, _, a_outputs) => {
            let mut ret_1: MidCode::Var;
            let mut ret_0: MidCode::Var;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("out_")).clone() }))?;
            ret_0 = listHead(a_outputs.clone())?;
            txt = genVarName(txt.clone(), ret_0)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            ret_1 = listHead(a_outputs.clone())?;
            txt = varBox(txt.clone(), ret_1)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_87(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_outputs)) {
        (txt, false, a_outputs) => {
            let mut txt_1: Tpl::Text;
            let mut ret_0: MidCode::Var;
            let mut txt = (*txt).clone();
            ret_0 = listHead(a_outputs.clone())?;
            txt_1 = varBoxType(Tpl::emptyTxt.clone(), ret_0)?;
            txt = fun_86(txt.clone(), txt_1, a_outputs.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_88(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text, mut in_a_o: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_o)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, _, a_o) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if(out_")).clone() }))?;
            txt = genVarName(txt.clone(), a_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") *out_")).clone() }))?;
            txt = genVarName(txt.clone(), a_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = varBox(txt.clone(), a_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_89(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_89 in &*items {
        let mut lstElt_89 = lstElt_89.clone();
        txt = (match lstElt_89.clone() {
        mut i_o => {
            let mut txt_0: Tpl::Text;
            txt_0 = varBoxType(Tpl::emptyTxt.clone(), i_o.clone())?;
            txt = fun_88(txt.clone(), txt_0.clone(), i_o.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn smf_90(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_91(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_92(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_outputs)) {
        (txt, false, a_outputs) => {
            let mut ret_0: MidCode::Var;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return out_")).clone() }))?;
            ret_0 = listHead(a_outputs.clone())?;
            txt = genVarName(txt.clone(), ret_0)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return;")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn genBoxPtrFunction(mut in_txt: Tpl::Text, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_fn) {
        (mut txt, MidCode::Function { outputs: ref i_outputs, inputs: ref i_inputs, name: ref i_name, .. }) => {
            let mut ret_22: bool;
            let mut txt_20: Tpl::Text;
            let mut ret_20: Arc<metamodelica::List<MidCode::Var>>;
            let mut txt_18: Tpl::Text;
            let mut ret_18: bool;
            let mut l_boxes: Tpl::Text;
            let mut txt_15: Tpl::Text;
            let mut ret_15: Arc<metamodelica::List<MidCode::Var>>;
            let mut txt_14: Tpl::Text;
            let mut l_callvars: Tpl::Text;
            let mut ret_12: bool;
            let mut l_callretval: Tpl::Text;
            let mut l_unboxes: Tpl::Text;
            let mut ret_9: bool;
            let mut l_boxDefs: Tpl::Text;
            let mut l_callOutDefs: Tpl::Text;
            let mut l_unboxDefs: Tpl::Text;
            let mut txt_4: Tpl::Text;
            let mut ret_4: Arc<metamodelica::List<MidCode::Var>>;
            let mut txt_3: Tpl::Text;
            let mut l_arguments: Tpl::Text;
            let mut ret_1: bool;
            let mut l_returnType: Tpl::Text;
            ret_1 = i_outputs.clone().is_empty();
            l_returnType = fun_64(Tpl::emptyTxt.clone(), ret_1)?;
            txt_3 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt_3 = lm_65(txt_3, i_inputs.clone())?;
            txt_3 = Tpl::popIter(txt_3)?;
            ret_4 = List::restOrEmpty(i_outputs.clone())?;
            txt_4 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt_4 = lm_66(txt_4, ret_4)?;
            txt_4 = Tpl::popIter(txt_4)?;
            l_arguments = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",\n")).clone(), (literal!("    ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_arguments = smf_67(l_arguments, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threadData_t *threadData")).clone() }))?;
            l_arguments = smf_68(l_arguments, txt_3)?;
            l_arguments = smf_69(l_arguments, txt_4)?;
            l_arguments = Tpl::popIter(l_arguments)?;
            l_unboxDefs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_unboxDefs = lm_71(l_unboxDefs, i_inputs.clone())?;
            l_unboxDefs = Tpl::popIter(l_unboxDefs)?;
            l_callOutDefs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_callOutDefs = lm_73(l_callOutDefs, i_outputs.clone())?;
            l_callOutDefs = Tpl::popIter(l_callOutDefs)?;
            ret_9 = i_outputs.clone().is_empty();
            l_boxDefs = fun_74(Tpl::emptyTxt.clone(), ret_9, i_outputs.clone())?;
            l_unboxes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_unboxes = lm_76(l_unboxes, i_inputs.clone())?;
            l_unboxes = Tpl::popIter(l_unboxes)?;
            ret_12 = i_outputs.clone().is_empty();
            l_callretval = fun_78(Tpl::emptyTxt.clone(), ret_12, i_outputs.clone())?;
            txt_14 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt_14 = lm_80(txt_14, i_inputs.clone())?;
            txt_14 = Tpl::popIter(txt_14)?;
            ret_15 = List::restOrEmpty(i_outputs.clone())?;
            txt_15 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt_15 = lm_82(txt_15, ret_15)?;
            txt_15 = Tpl::popIter(txt_15)?;
            l_callvars = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",\n")).clone(), (literal!("    ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_callvars = smf_83(l_callvars, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threadData")).clone() }))?;
            l_callvars = smf_84(l_callvars, txt_14)?;
            l_callvars = smf_85(l_callvars, txt_15)?;
            l_callvars = Tpl::popIter(l_callvars)?;
            ret_18 = i_outputs.clone().is_empty();
            txt_18 = fun_87(Tpl::emptyTxt.clone(), ret_18, i_outputs.clone())?;
            ret_20 = List::restOrEmpty(i_outputs.clone())?;
            txt_20 = lm_89(Tpl::emptyTxt.clone(), ret_20)?;
            l_boxes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_boxes = smf_90(l_boxes, txt_18)?;
            l_boxes = smf_91(l_boxes, txt_20)?;
            l_boxes = Tpl::popIter(l_boxes)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#undef boxptr_")).clone() }))?;
            txt = underscorePath(txt.clone(), i_name.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_returnType)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" boxptr_")).clone() }))?;
            txt = underscorePath(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arguments)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_unboxDefs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_callOutDefs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_boxDefs)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_unboxes)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_callretval)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omc_")).clone() }))?;
            txt = underscorePath(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_callvars)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_boxes)?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_22 = i_outputs.clone().is_empty();
            txt = fun_92(txt.clone(), ret_22, i_outputs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_94(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>, mut a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_94 in &*items {
        let mut lstElt_94 = lstElt_94.clone();
        txt = (match lstElt_94.clone() {
        mut i_local => {
            txt = genLocalDecl(txt.clone(), a_fn.clone(), i_local.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_95(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::VarBuf>>, mut a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_95 in &*items {
        let mut lstElt_95 = lstElt_95.clone();
        txt = (match lstElt_95.clone() {
        mut i_local => {
            txt = genLocalBufDecl(txt.clone(), a_fn.clone(), i_local.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn lm_96(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::VarBufPtr>>, mut a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_96 in &*items {
        let mut lstElt_96 = lstElt_96.clone();
        txt = (match lstElt_96.clone() {
        mut i_local => {
            txt = genLocalBufPtrDecl(txt.clone(), a_fn.clone(), i_local.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

pub(crate) fn genLocalDecls(mut txt: Tpl::Text, mut a_fn: MidCode::Function, mut a_locals: Arc<metamodelica::List<MidCode::Var>>, mut a_localBufs: Arc<metamodelica::List<MidCode::VarBuf>>, mut a_localBufPtrs: Arc<metamodelica::List<MidCode::VarBufPtr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_94(out_txt, a_locals, a_fn.clone())?;
    out_txt = Tpl::popIter(out_txt)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::pushIter(out_txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_95(out_txt, a_localBufs, a_fn.clone())?;
    out_txt = Tpl::popIter(out_txt)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::pushIter(out_txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_96(out_txt, a_localBufPtrs, a_fn)?;
    out_txt = Tpl::popIter(out_txt)?;
    Ok(out_txt)
}

fn fun_98(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_var) {
        (mut txt, MidCode::Var { volatile: true, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("volatile ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn genLocalDecl(mut txt: Tpl::Text, mut a_fn: MidCode::Function, mut a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_98(txt, a_var.clone())?;
    out_txt = genVarType(out_txt, a_var.clone())?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_txt = genVarName(out_txt, a_var)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    Ok(out_txt)
}

pub(crate) fn genLocalBufDecl(mut txt: Tpl::Text, mut a_fn: MidCode::Function, mut a_var: MidCode::VarBuf) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("jmp_buf ")).clone() }))?;
    out_txt = genVarBufName(out_txt, a_var)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    Ok(out_txt)
}

pub(crate) fn genLocalBufPtrDecl(mut txt: Tpl::Text, mut a_fn: MidCode::Function, mut a_var: MidCode::VarBufPtr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("jmp_buf *")).clone() }))?;
    out_txt = genVarBufPtrName(out_txt, a_var)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    Ok(out_txt)
}

pub(crate) fn genEntry(mut in_txt: Tpl::Text, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_fn) {
        (mut txt, MidCode::Function { entryId: mut i_entryId, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("goto ")).clone() }))?;
            txt = genLabel(txt.clone(), i_entryId.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_103(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_outputs)) {
        (txt, false, a_outputs) => {
            let mut ret_0: MidCode::Var;
            let mut txt = (*txt).clone();
            ret_0 = listHead(a_outputs.clone())?;
            txt = genVarName(txt.clone(), ret_0)?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_104(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_104 in &*items {
        let mut lstElt_104 = lstElt_104.clone();
        txt = (match lstElt_104.clone() {
        mut i_v => {
            let mut l_outPtrName: Tpl::Text;
            l_outPtrName = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("outPtr_")).clone() }))?;
            l_outPtrName = genVarName(l_outPtrName.clone(), i_v.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_outPtrName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" != NULL)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_outPtrName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = genVarName(txt.clone(), i_v.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

pub(crate) fn genExit(mut in_txt: Tpl::Text, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_fn) {
        (mut txt, MidCode::Function { outputs: ref i_outputs, exitId: mut i_exitId, .. }) => {
            let mut ret_2: Arc<metamodelica::List<MidCode::Var>>;
            let mut ret_1: bool;
            let mut l_returnString: Tpl::Text;
            ret_1 = i_outputs.clone().is_empty();
            l_returnString = fun_103(Tpl::emptyTxt.clone(), ret_1, i_outputs.clone())?;
            txt = genLabel(txt.clone(), i_exitId.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(": // exit block\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_2 = List::restOrEmpty(i_outputs.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_104(txt.clone(), ret_2)?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_returnString)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_106(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Block>>, mut a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_106 in &*items {
        let mut lstElt_106 = lstElt_106.clone();
        txt = (match lstElt_106.clone() {
        mut i_block => {
            txt = genBlock(txt.clone(), a_fn.clone(), i_block.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

pub(crate) fn genBlocks(mut txt: Tpl::Text, mut a_fn: MidCode::Function, mut a_body: Arc<metamodelica::List<MidCode::Block>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_106(out_txt, a_body, a_fn)?;
    out_txt = Tpl::popIter(out_txt)?;
    Ok(out_txt)
}

fn lm_108(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Stmt>>, mut a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_108 in &*items {
        let mut lstElt_108 = lstElt_108.clone();
        txt = (match lstElt_108.clone() {
        mut i_stmt => {
            txt = genStmt(txt.clone(), a_fn.clone(), i_stmt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_109(mut in_txt: Tpl::Text, mut in_a_block: MidCode::Block, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_block, in_a_fn) {
        (mut txt, MidCode::Block { id: mut i_id, stmts: ref i_stmts, terminator: mut i_terminator }, mut a_fn) => {
            txt = genLabel(txt.clone(), i_id.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(":\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_108(txt.clone(), i_stmts.clone(), a_fn.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = genTerminator(txt.clone(), a_fn.clone(), i_terminator.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn genBlock(mut txt: Tpl::Text, mut a_fn: MidCode::Function, mut a_block: MidCode::Block) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_109(txt, a_block, a_fn)?;
    Ok(out_txt)
}

pub(crate) fn genLabel(mut txt: Tpl::Text, mut a_i: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("label_")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt, (intString(a_i)).clone())?;
    Ok(out_txt)
}

pub(crate) fn genVarName(mut in_txt: Tpl::Text, mut in_a_v: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_v) {
        (mut txt, MidCode::Var { name: mut i_name, .. }) => {
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn genVarBufName(mut in_txt: Tpl::Text, mut in_a_v: MidCode::VarBuf) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_v) {
        (mut txt, MidCode::VarBuf { name: mut i_name }) => {
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn genVarBufPtrName(mut in_txt: Tpl::Text, mut in_a_v: MidCode::VarBufPtr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_v) {
        (mut txt, MidCode::VarBufPtr { name: mut i_name }) => {
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_115(mut in_txt: Tpl::Text, mut in_a_stmt: MidCode::Stmt) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_stmt) {
        (mut txt, MidCode::Stmt::NOP { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; // NOP")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::Stmt::ASSIGN { dest: MidCode::Var { name: mut i_dest__name, ty: _, .. }, src: mut i_rvalue }) => {
            txt = Tpl::writeStr(txt.clone(), (i_dest__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = genRValue(txt.clone(), i_rvalue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn genStmt(mut txt: Tpl::Text, mut a_fn: MidCode::Function, mut a_stmt: MidCode::Stmt) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_115(txt, a_stmt)?;
    Ok(out_txt)
}

fn fun_117(mut in_txt: Tpl::Text, mut in_a_value: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_value) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_118(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_118 in &*items {
        let mut lstElt_118 = lstElt_118.clone();
        txt = (match lstElt_118.clone() {
        mut i_element => {
            txt = genVarName(txt.clone(), i_element.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn smf_119(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_120(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_121(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_122(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_123(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_124(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_125(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_126(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_elementargs: Tpl::Text, mut in_a_metatypeCtor: Tpl::Text, mut in_a_metatypeSlots: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_ty, in_a_elementargs, in_a_metatypeCtor, in_a_metatypeSlots)) {
        (txt, i_ty @ Deref @ DAE::Type::T_METARECORD { path: _, .. }, a_elementargs, a_metatypeCtor, a_metatypeSlots) => {
            let mut txt_2: Tpl::Text;
            let mut txt_1: Tpl::Text;
            let mut l_arguments: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_1 = Tpl::writeText(Tpl::emptyTxt.clone(), a_metatypeSlots.clone())?;
            txt_1 = Tpl::writeTok(txt_1, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+1")).clone() }))?;
            txt_2 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("&")).clone() }))?;
            txt_2 = genTypeUnderscorePath(txt_2, i_ty.clone())?;
            txt_2 = Tpl::writeTok(txt_2, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__desc")).clone() }))?;
            l_arguments = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_arguments = smf_119(l_arguments, txt_1)?;
            l_arguments = smf_120(l_arguments, a_metatypeCtor.clone())?;
            l_arguments = smf_121(l_arguments, txt_2)?;
            l_arguments = smf_122(l_arguments, a_elementargs.clone())?;
            l_arguments = Tpl::popIter(l_arguments)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_box(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arguments)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_elementargs, a_metatypeCtor, a_metatypeSlots) => {
            let mut l_arguments: Tpl::Text;
            let mut txt = (*txt).clone();
            l_arguments = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_arguments = smf_123(l_arguments, a_metatypeSlots.clone())?;
            l_arguments = smf_124(l_arguments, a_metatypeCtor.clone())?;
            l_arguments = smf_125(l_arguments, a_elementargs.clone())?;
            l_arguments = Tpl::popIter(l_arguments)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_box(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arguments)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn genRValue(mut in_txt: Tpl::Text, mut in_a_rvalue: MidCode::RValue) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_rvalue)) {
        (txt, MidCode::RValue::VARIABLE { src: MidCode::Var { name: i_src__name, ty: _, .. } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_src__name.clone()).clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::BINARYOP { op: i_op, lsrc: i_lsrc @ MidCode::Var { name: _, ty: Deref @ DAE::Type::T_STRING { varLst: _ }, .. }, rsrc: i_rsrc @ MidCode::Var { name: _, ty: Deref @ DAE::Type::T_STRING { varLst: _ }, .. } }) => {
            let mut txt = (*txt).clone();
            txt = genStringBinaryop(txt.clone(), i_op.clone(), i_lsrc.clone(), i_rsrc.clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::BINARYOP { op: MidCode::BinaryOp::POW { .. }, lsrc: MidCode::Var { name: i_lsrc__name, ty: _, .. }, rsrc: MidCode::Var { name: i_rsrc__name, ty: _, .. } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("pow(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_lsrc__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_rsrc__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, MidCode::RValue::BINARYOP { op: i_op, lsrc: MidCode::Var { name: i_lsrc__name, ty: _, .. }, rsrc: MidCode::Var { name: i_rsrc__name, ty: _, .. } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_lsrc__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = binaryopToString(txt.clone(), i_op.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_rsrc__name.clone()).clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::UNARYOP { op: MidCode::UnaryOp::BOX { .. }, src: i_src }) => {
            let mut txt = (*txt).clone();
            txt = varBox(txt.clone(), i_src.clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::UNARYOP { op: MidCode::UnaryOp::UNBOX { .. }, src: i_src }) => {
            let mut txt = (*txt).clone();
            txt = varUnbox(txt.clone(), i_src.clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::UNARYOP { op: i_op_1, src: i_src }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = genVarType(txt.clone(), i_src.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") ")).clone() }))?;
            txt = unaryopToString(txt.clone(), i_op_1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = genVarName(txt.clone(), i_src.clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::LITERALINTEGER { value: i_value }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_value.clone())).clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::LITERALBOOLEAN { value: i_value_1 }) => {
            let mut txt = (*txt).clone();
            txt = fun_117(txt.clone(), i_value_1.clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::LITERALREAL { value: i_value_2 }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (realString(i_value_2.clone())).clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::LITERALSTRING { value: i_value_3 }) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_scon(\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToCString((i_value_3.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\")")).clone() }))?;
            txt.clone()
        },
        (txt, MidCode::RValue::LITERALMETATYPE { elements: i_elements, ty: i_ty }) => {
            let mut l_elementargs: Tpl::Text;
            let mut l_metatypeCtor: Tpl::Text;
            let mut ret_2: i32;
            let mut l_metatypeSlots: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_2 = (i_elements.clone().len() as i32);
            l_metatypeSlots = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_2)).clone())?;
            l_metatypeCtor = genTypeCtorIndex(Tpl::emptyTxt.clone(), i_elements.clone(), i_ty.clone())?;
            l_elementargs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_elementargs = lm_118(l_elementargs, i_elements.clone())?;
            l_elementargs = Tpl::popIter(l_elementargs)?;
            txt = fun_126(txt.clone(), i_ty.clone(), l_elementargs, l_metatypeCtor, l_metatypeSlots)?;
            txt.clone()
        },
        (txt, MidCode::RValue::METAFIELD { src: i_src, index: i_index, ty: _ }) => {
            let mut ret_5: i32;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MMC_FETCH(MMC_OFFSET(MMC_UNTAGPTR(")).clone() }))?;
            txt = genVarName(txt.clone(), i_src.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("),")).clone() }))?;
            ret_5 = intAdd(i_index.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_5)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt.clone()
        },
        (txt, MidCode::RValue::UNIONTYPEVARIANT { src: i_src }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(MMC_HDRCTOR(MMC_GETHDR(")).clone() }))?;
            txt = genVarName(txt.clone(), i_src.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")) - 3)")).clone() }))?;
            txt.clone()
        },
        (txt, MidCode::RValue::ISSOME { src: i_src }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(0==MMC_HDRSLOTS(MMC_GETHDR(")).clone() }))?;
            txt = genVarName(txt.clone(), i_src.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")) ? 0 : 1)")).clone() }))?;
            txt.clone()
        },
        (txt, MidCode::RValue::ISCONS { src: i_src }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(MMC_GETHDR(")).clone() }))?;
            txt = genVarName(txt.clone(), i_src.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") == MMC_CONSHDR)")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("notimplemented")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn binaryopToString(mut in_txt: Tpl::Text, mut in_a_op: MidCode::BinaryOp) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_op) {
        (mut txt, MidCode::BinaryOp::ADD { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::SUB { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::MUL { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::DIV { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::LESS { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::LESSEQ { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<=")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::GREATER { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::GREATEREQ { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">=")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::EQUAL { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("==")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::NEQUAL { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("!=")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("notimplemented")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn unaryopToString(mut in_txt: Tpl::Text, mut in_a_op: MidCode::UnaryOp) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_op) {
        (mut txt, MidCode::UnaryOp::MOVE { .. }) => {
            txt.clone()
        },
        (mut txt, MidCode::UnaryOp::UMINUS { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::UnaryOp::NOT { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("!")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("notimplemented")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn genStringBinaryop(mut in_txt: Tpl::Text, mut in_a_op: MidCode::BinaryOp, mut in_a_lsrc: MidCode::Var, mut in_a_rsrc: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_op, in_a_lsrc, in_a_rsrc) {
        (mut txt, MidCode::BinaryOp::ADD { .. }, mut a_lsrc, mut a_rsrc) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("stringAppend(")).clone() }))?;
            txt = genVarName(txt.clone(), a_lsrc.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = genVarName(txt.clone(), a_rsrc.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (mut txt, mut i_op, mut a_lsrc, mut a_rsrc) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0 ")).clone() }))?;
            txt = binaryopToString(txt.clone(), i_op.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" stringCompare(")).clone() }))?;
            txt = genVarName(txt.clone(), a_lsrc.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = genVarName(txt.clone(), a_rsrc.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_131(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(i32, i32)>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_131 in &*items {
        let mut lstElt_131 = lstElt_131.clone();
        txt = (match lstElt_131.clone() {
        (mut i_from, mut i_to) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_from.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(": goto ")).clone() }))?;
            txt = genLabel(txt.clone(), i_to.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_132(mut in_txt: Tpl::Text, mut in_a_outputs: Arc<metamodelica::List<MidCode::OutVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_outputs)) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: MidCode::OutVar::OUT_WILD { .. }, tail: _ }) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: MidCode::OutVar::OUT_VAR { var: i_var }, tail: _ }) => {
            let mut txt = (*txt).clone();
            txt = genVarName(txt.clone(), i_var.clone())?;
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

fn fun_133(mut in_txt: Tpl::Text, mut in_a_builtin: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_builtin) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threadData")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_134(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_134 in &*items {
        let mut lstElt_134 = lstElt_134.clone();
        txt = (match lstElt_134.clone() {
        mut i_i => {
            txt = genVarName(txt.clone(), i_i.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_135(mut in_txt: Tpl::Text, mut in_a_o: MidCode::OutVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_o) {
        (mut txt, MidCode::OutVar::OUT_VAR { var: mut i_var }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("&")).clone() }))?;
            txt = genVarName(txt.clone(), i_var.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NULL")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_136(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<MidCode::OutVar>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_136 in &*items {
        let mut lstElt_136 = lstElt_136.clone();
        txt = (match lstElt_136.clone() {
        mut i_o => {
            txt = fun_135(txt.clone(), i_o.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn smf_137(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_138(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_139(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_it) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_140(mut in_txt: Tpl::Text, mut in_a_builtin: bool, mut in_a_func: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_builtin, in_a_func)) {
        (txt, false, a_func) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omc_")).clone() }))?;
            txt = underscorePath(txt.clone(), a_func.clone())?;
            txt.clone()
        },
        (txt, _, a_func) => {
            let mut txt = (*txt).clone();
            txt = identBuiltinCall(txt.clone(), a_func.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_141(mut in_txt: Tpl::Text, mut in_a_terminator: MidCode::Terminator, mut in_a_exitId: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_terminator, in_a_exitId) {
        (mut txt, MidCode::Terminator::RETURN { .. }, mut a_exitId) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("goto ")).clone() }))?;
            txt = genLabel(txt.clone(), a_exitId.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; // exit label")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::Terminator::GOTO { next: mut i_label }, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("goto ")).clone() }))?;
            txt = genLabel(txt.clone(), i_label.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::Terminator::BRANCH { condition: mut i_condition, onTrue: mut i_labelTrue, onFalse: mut i_labelFalse }, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (")).clone() }))?;
            txt = genVarName(txt.clone(), i_condition.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(")\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("goto ")).clone() }))?;
            txt = genLabel(txt.clone(), i_labelTrue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("else\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("goto ")).clone() }))?;
            txt = genLabel(txt.clone(), i_labelFalse.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, MidCode::Terminator::SWITCH { condition: mut i_condition, cases: ref i_cases }, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("switch (")).clone() }))?;
            txt = genVarName(txt.clone(), i_condition.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("){\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_131(txt.clone(), i_cases.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::Terminator::CALL { func: ref i_func, builtin: mut i_builtin, inputs: ref i_inputs, outputs: ref i_outputs, next: mut i_next }, _) => {
            let mut txt_4: Tpl::Text;
            let mut ret_4: Arc<metamodelica::List<MidCode::OutVar>>;
            let mut txt_3: Tpl::Text;
            let mut txt_2: Tpl::Text;
            let mut l_arguments: Tpl::Text;
            let mut l_returnAssignment: Tpl::Text;
            l_returnAssignment = fun_132(Tpl::emptyTxt.clone(), i_outputs.clone())?;
            txt_2 = fun_133(Tpl::emptyTxt.clone(), i_builtin.clone())?;
            txt_3 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt_3 = lm_134(txt_3, i_inputs.clone())?;
            txt_3 = Tpl::popIter(txt_3)?;
            ret_4 = List::restOrEmpty(i_outputs.clone())?;
            txt_4 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt_4 = lm_136(txt_4, ret_4)?;
            txt_4 = Tpl::popIter(txt_4)?;
            l_arguments = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",\n")).clone(), (literal!("    ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_arguments = smf_137(l_arguments, txt_2)?;
            l_arguments = smf_138(l_arguments, txt_3)?;
            l_arguments = smf_139(l_arguments, txt_4)?;
            l_arguments = Tpl::popIter(l_arguments)?;
            txt = Tpl::writeText(txt.clone(), l_returnAssignment)?;
            txt = fun_140(txt.clone(), i_builtin.clone(), i_func.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arguments)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(");\n")).clone(), (literal!("goto ")).clone()], lastHasNewLine: false }))?;
            txt = genLabel(txt.clone(), i_next.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::Terminator::LONGJMP { .. }, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("longjmp(*threadData->mmc_jumper,1);")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::Terminator::PUSHJMP { old_buf: mut i_old__buf, new_buf: mut i_new__buf, next: mut i_next }, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("// PUSHJMP\n")).clone() }))?;
            txt = genVarBufPtrName(txt.clone(), i_old__buf.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" = threadData->mmc_jumper;\n")).clone(), (literal!("threadData->mmc_jumper = &")).clone()], lastHasNewLine: false }))?;
            txt = genVarBufName(txt.clone(), i_new__buf.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("setjmp(&")).clone()], lastHasNewLine: false }))?;
            txt = genVarBufName(txt.clone(), i_new__buf.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(");\n")).clone(), (literal!("goto ")).clone()], lastHasNewLine: false }))?;
            txt = genLabel(txt.clone(), i_next.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::Terminator::POPJMP { old_buf: mut i_old__buf, next: mut i_next }, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("// POPJMP\n")).clone(), (literal!("threadData->mmc_jumper = ")).clone()], lastHasNewLine: false }))?;
            txt = genVarBufPtrName(txt.clone(), i_old__buf.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("goto ")).clone()], lastHasNewLine: false }))?;
            txt = genLabel(txt.clone(), i_next.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("notimplemented")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn genTerminator(mut in_txt: Tpl::Text, mut in_a_fn: MidCode::Function, mut in_a_terminator: MidCode::Terminator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_fn, in_a_terminator) {
        (mut txt, MidCode::Function { locals: _, inputs: _, outputs: _, body: _, entryId: _, exitId: mut i_exitId, .. }, mut a_terminator) => {
            txt = fun_141(txt.clone(), a_terminator.clone(), i_exitId.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_143(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_ty)) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_integer")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_integer")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_boolean")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_real")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_string")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METARECORD { path: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATYPE { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAOPTION { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAARRAY { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATUPLE { types: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAUNIONTYPE { paths: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METALIST { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_UNKNOWN { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unknown")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAPOLYMORPHIC { name: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("error_ metapolymorphic")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAUNIONTYPE { paths: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("error_ metauniontype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ANYTYPE { anyClassType: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("error_anytype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_TUPLE { types: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("error_tuple")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("notimplemented")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn genVarType(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_var) {
        (mut txt, MidCode::Var { name: _, ty: ref i_ty, .. }) => {
            txt = fun_143(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_145(mut in_txt: Tpl::Text, mut in_mArg: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, 0) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_146(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_elements: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_ty, in_a_elements)) {
        (txt, Deref @ DAE::Type::T_METARECORD { index: i_index, .. }, _) => {
            let mut ret_0: i32;
            let mut txt = (*txt).clone();
            ret_0 = intAdd(i_index.clone(), 3);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0)).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAARRAY { ty: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("2")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATUPLE { types: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAOPTION { ty: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METALIST { ty: _ }, a_elements) => {
            let mut ret_1: i32;
            let mut txt = (*txt).clone();
            ret_1 = (a_elements.clone().len() as i32);
            txt = fun_145(txt.clone(), ret_1)?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn genTypeCtorIndex(mut txt: Tpl::Text, mut a_elements: Arc<metamodelica::List<MidCode::Var>>, mut a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_146(txt, a_ty, a_elements)?;
    Ok(out_txt)
}

pub(crate) fn genTypeUnderscorePath(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_ty)) {
        (txt, Deref @ DAE::Type::T_METARECORD { path: i_path, .. }) => {
            let mut txt = (*txt).clone();
            txt = underscorePath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: i_path }, varLst: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = underscorePath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("error: genTypeUnderscorePath")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_149(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_ty)) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn varBoxType(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_var) {
        (mut txt, MidCode::Var { name: _, ty: ref i_ty, .. }) => {
            txt = fun_149(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_151(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_ty, in_a_name)) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_icon(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_icon(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_icon(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_rcon(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_string(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_box(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn varBox(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_var) {
        (mut txt, MidCode::Var { name: mut i_name, ty: ref i_ty, .. }) => {
            txt = fun_151(txt.clone(), i_ty.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_153(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_ty, in_a_name)) {
        (txt, Deref @ DAE::Type::T_METABOXED { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ } }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_integer(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. } }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_integer(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_integer(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: Deref @ DAE::Type::T_REAL { varLst: _ } }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_real(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_string(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn varUnbox(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_var) {
        (mut txt, MidCode::Var { name: mut i_name, ty: ref i_ty, .. }) => {
            txt = fun_153(txt.clone(), i_ty.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_155(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_ty, in_a_name)) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_integer(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_integer(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_integer(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_real(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_string(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn varUnbox2(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_var) {
        (mut txt, MidCode::Var { name: mut i_name, ty: ref i_ty, .. }) => {
            txt = fun_155(txt.clone(), i_ty.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_157(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_ty, in_a_name)) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_integer(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_integer(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_real(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_string(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_integer(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_metatype(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAUNIONTYPE { paths: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_metatype(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METALIST { ty: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_metatype(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAARRAY { ty: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_metatype(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAOPTION { ty: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_metatype(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATUPLE { types: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_metatype(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn varModelicaRead(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_var) {
        (mut txt, MidCode::Var { name: mut i_name, ty: ref i_ty, .. }) => {
            txt = fun_157(txt.clone(), i_ty.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_159(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_ty, in_a_name)) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_integer(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_integer(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_real(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_string(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_integer(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_metatype(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAUNIONTYPE { paths: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_metatype(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METALIST { ty: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_metatype(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAARRAY { ty: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_metatype(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAOPTION { ty: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_metatype(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATUPLE { types: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_metatype(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn varModelicaWrite(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_var) {
        (mut txt, MidCode::Var { name: mut i_name, ty: ref i_ty, .. }) => {
            txt = fun_159(txt.clone(), i_ty.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn identName(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_path)) {
        (txt, Deref @ Absyn::Path::IDENT { name: i_name }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn identBuiltinCall(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_path)) {
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "clock" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_clock")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "anyString" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_anyString")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "fail" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MMC_THROW_INTERNAL")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "intMod" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_mod_integer")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: i_name }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn replaceDotAndUnderscore(mut in_txt: Tpl::Text, mut in_a_str: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_str) {
        (mut txt, mut i_name) => {
            let mut ret_4: ArcStr;
            let mut ret_3: ArcStr;
            let mut l_str__underscores: Tpl::Text;
            let mut ret_1: ArcStr;
            let mut l_str__dots: Tpl::Text;
            ret_1 = (System::stringReplace((i_name.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
            l_str__dots = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1).clone())?;
            ret_3 = (System::stringReplace((Tpl::textString(l_str__dots)?).clone(), (literal!("_")).clone(), (literal!("__")).clone())?).clone();
            l_str__underscores = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_3).clone())?;
            ret_4 = (System::unquoteIdentifier((Tpl::textString(l_str__underscores)?).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_4).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn underscorePath(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt, in_a_path)) {
        (txt, Deref @ Absyn::Path::QUALIFIED { name: i_name, path: i_path }) => {
            let mut txt = (*txt).clone();
            txt = replaceDotAndUnderscore(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            { (in_txt, in_a_path) = (txt.clone(), i_path.clone()); continue '__tco; }
        },
        (txt, Deref @ Absyn::Path::IDENT { name: i_name_1 }) => {
            let mut txt = (*txt).clone();
            return Ok(replaceDotAndUnderscore(txt.clone(), (i_name_1.clone()).clone())?)
        },
        (txt, Deref @ Absyn::Path::FULLYQUALIFIED { path: i_path }) => {
            let mut txt = (*txt).clone();
            { (in_txt, in_a_path) = (txt.clone(), i_path.clone()); continue '__tco; }
        },
        (txt, _) => {
            return Ok(txt.clone())
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

