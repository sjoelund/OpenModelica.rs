// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::MMToJuliaUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Static;
use openmodelica_frontend_dump::AbsynDumpTpl;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_types::SCode;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;

fn lm_17(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::Class>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_17 in &*items.clone() {
        let mut lstElt_17 = lstElt_17.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_17.clone()) {
        i_cls => {
            txt = dumpClass(txt.clone(), i_cls.clone(), Dump::defaultDumpOptions.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpProgram(mut in_txt: Tpl::Text, mut in_a_program: Absyn::Program) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_program.clone())) {
        (txt, Absyn::Program { classes: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, Absyn::Program { classes: i_classes, .. }) => {
            let mut l_cls__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_cls__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_cls__str = lm_17(l_cls__str.clone(), i_classes.clone())?;
            l_cls__str = Tpl::popIter(l_cls__str.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_cls__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpSCodeElements(mut txt: Tpl::Text, mut a_elements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut ret_0: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    ret_0 = SCodeDump::filterElements(a_elements.clone(), SCodeDump::defaultOptions.clone())?;
    out_txt = dumpSCodeElements2(txt.clone(), ret_0.clone())?;
    Ok(out_txt)
}

fn fun_20(mut in_txt: Tpl::Text, mut in_a_el: Arc<SCode::Element>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_el.clone())) {
        (txt, Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_UNIONTYPE { .. }, name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("@UniontypeDecl ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (txt, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: i_parts_elementLst, .. }, partialPrefix: SCode::Partial::NOT_PARTIAL { .. }, restriction: SCode::Restriction::R_FUNCTION { .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpSCodeElements2(txt.clone(), i_parts_elementLst.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::Element::CLASS { partialPrefix: SCode::Partial::PARTIAL { .. }, restriction: SCode::Restriction::R_FUNCTION { .. }, name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = Function")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_21(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_21 in &*items.clone() {
        let mut lstElt_21 = lstElt_21.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_21.clone()) {
        i_el => {
            txt = fun_20(txt.clone(), i_el.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_22(mut in_txt: Tpl::Text, mut in_a_str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), i_str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpSCodeElements2(mut txt: Tpl::Text, mut a_elements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut l_str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    l_str = lm_21(Tpl::emptyTxt.clone(), a_elements.clone())?;
    out_txt = fun_22(txt.clone(), l_str.clone())?;
    Ok(out_txt)
}

pub fn dumpClass(mut txt: Tpl::Text, mut a_cls: Arc<Absyn::Class>, mut a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = dumpClassElement(txt.clone(), a_cls.clone(), a_options.clone(), MMToJuliaUtil::noContext.clone())?;
    Ok(out_txt)
}

fn lm_25(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_25 in &*items.clone() {
        let mut lstElt_25 = lstElt_25.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_25.clone()) {
        i_cp => {
            let mut ret_0: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            ret_0 = AbsynUtil::getElementItemsInClassPart(i_cp.clone());
            txt = dumpReturnTypeJL(txt.clone(), ret_0.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_26(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_26 in &*items.clone() {
        let mut lstElt_26 = lstElt_26.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_26.clone()) {
        i_cp => {
            let mut ret_0: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            ret_0 = AbsynUtil::getElementItemsInClassPart(i_cp.clone());
            txt = dumpReturnStrJL(txt.clone(), ret_0.clone(), MMToJuliaUtil::functionContext.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_27(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_27 in &*items.clone() {
        let mut lstElt_27 = lstElt_27.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_27.clone()) {
        i_cp => {
            let mut ret_0: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            ret_0 = AbsynUtil::getElementItemsInClassPart(i_cp.clone());
            txt = dumpInputsJL(txt.clone(), ret_0.clone(), MMToJuliaUtil::inputContext.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_28(mut in_txt: Tpl::Text, mut in_a_header: Tpl::Text, mut in_a_typevar__inputs: Tpl::Text, mut in_a_inputs__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_header.clone(), in_a_typevar__inputs.clone(), in_a_inputs__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _, a_inputs__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_inputs__str.clone())?;
            txt.clone()
        },
        (txt, _, a_typevar__inputs, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_typevar__inputs.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_29(mut in_txt: Tpl::Text, mut in_a_header: Tpl::Text, mut in_a_returnType: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_header.clone(), in_a_returnType.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_returnType) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_returnType.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_30(mut in_txt: Tpl::Text, mut in_a_encapsulatedPrefix: bool) -> Tpl::Text {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_encapsulatedPrefix.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    out_txt
}

fn fun_31(mut in_txt: Tpl::Text, mut in_a_partialPrefix: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_partialPrefix.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#=TODO: Originally partial =# ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_32(mut in_txt: Tpl::Text, mut in_a_restriction: Absyn::Restriction, mut in_a_context: MMToJuliaUtil::Context, mut in_a_options: Dump::DumpOptions, mut in_a_parts: Arc<Absyn::ClassDef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_restriction.clone(), in_a_context.clone(), in_a_options.clone(), in_a_parts.clone())) {
        (txt, Absyn::Restriction::R_PACKAGE { .. }, _, a_options, a_parts) => {
            let mut txt = (*txt).clone();
            txt = dumpClassDef(txt.clone(), a_parts.clone(), MMToJuliaUtil::packageContext.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, Absyn::Restriction::R_RECORD { .. }, _, a_options, a_parts) => {
            let mut ret_0: MMToJuliaUtil::Context = MMToJuliaUtil::Context::NO_CONTEXT;
            let mut txt = (*txt).clone();
            ret_0 = MMToJuliaUtil::makeUniontypeContext((literal!("")).clone());
            txt = dumpClassDef(txt.clone(), a_parts.clone(), ret_0.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, _, a_context, a_options, a_parts) => {
            let mut txt = (*txt).clone();
            txt = dumpClassDef(txt.clone(), a_parts.clone(), a_context.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_33(mut in_txt: Tpl::Text, mut in_a_forwardDeclarations: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_forwardDeclarations.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#= Necessary to write declarations for your uniontypes until Julia adds support for mutually recursive types =#")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_34(mut in_txt: Tpl::Text, mut in_a_restriction: Absyn::Restriction, mut in_a_cdef__str1: Tpl::Text, mut in_a_forwardDeclarations: Tpl::Text, mut in_a_inform: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_restriction.clone(), in_a_cdef__str1.clone(), in_a_forwardDeclarations.clone(), in_a_inform.clone()) {
        (mut txt, Absyn::Restriction::R_PACKAGE { .. }, mut a_cdef__str1, mut a_forwardDeclarations, mut a_inform) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("using MetaModelica\n")).clone(), (literal!("#= ExportAll is not good practice but it makes it so that we do not have to write export after each function :( =#\n")).clone(), (literal!("using ExportAll\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), a_inform.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_forwardDeclarations.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::writeText(txt.clone(), a_cdef__str1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("#= So that we can use wildcard imports and named imports when they do occur. Not good Julia practice =#\n")).clone(), (literal!("@exportAll()")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _, mut a_cdef__str1, _, _) => {
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_cdef__str1.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_35(mut in_txt: Tpl::Text, mut in_a_restriction: Absyn::Restriction) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_restriction.clone()) {
        (mut txt, Absyn::Restriction::R_RECORD { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("begin")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_36(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_36 in &*items.clone() {
        let mut lstElt_36 = lstElt_36.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_36.clone()) {
        i_earg => {
            txt = dumpElementArg(txt.clone(), i_earg.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_37(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_37 in &*items.clone() {
        let mut lstElt_37 = lstElt_37.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_37.clone()) {
        i_earg => {
            txt = dumpElementArg(txt.clone(), i_earg.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpClassElement(mut in_txt: Tpl::Text, mut in_a_class: Arc<Absyn::Class>, mut in_a_options: Dump::DumpOptions, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_class.clone(), in_a_options.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::Class { body: i_parts @ Deref @ Absyn::ClassDef::PARTS { comment: i_parts_comment, .. }, restriction: Absyn::Restriction::R_UNIONTYPE { .. }, name: i_name, .. }, a_options, _) => {
            let mut ret_2: MMToJuliaUtil::Context = MMToJuliaUtil::Context::NO_CONTEXT;
            let mut l_class__def__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_commentStr: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_commentStr = dumpCommentStrOpt(Tpl::emptyTxt.clone(), i_parts_comment.clone())?;
            ret_2 = MMToJuliaUtil::makeUniontypeContext((i_name.clone()).clone());
            l_class__def__str = dumpClassDef(Tpl::emptyTxt.clone(), i_parts.clone(), ret_2.clone(), a_options.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeText(txt.clone(), l_commentStr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("@Uniontype ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" begin\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeText(txt.clone(), l_class__def__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Class { partialPrefix: true, restriction: Absyn::Restriction::R_FUNCTION { functionRestriction: _ }, .. }, _, _) => {
            txt.clone()
        },
        (txt, Deref @ Absyn::Class { partialPrefix: false, body: i_parts @ Deref @ Absyn::ClassDef::PARTS { comment: i_parts_comment, classParts: i_parts_classParts, .. }, restriction: i_restriction @ Absyn::Restriction::R_FUNCTION { functionRestriction: _ }, name: i_name, .. }, a_options, _) => {
            let mut ret_10: MMToJuliaUtil::Context = MMToJuliaUtil::Context::NO_CONTEXT;
            let mut l_functionBodyStr: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_header: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut ret_7: ArcStr = arcstr::literal!("");
            let mut l_typevar__inputs: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_inputs__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_return__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_returnType: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_commentStr: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_commentStr = dumpCommentStrOpt(Tpl::emptyTxt.clone(), i_parts_comment.clone())?;
            l_returnType = lm_25(Tpl::emptyTxt.clone(), i_parts_classParts.clone())?;
            l_return__str = lm_26(Tpl::emptyTxt.clone(), i_parts_classParts.clone())?;
            l_inputs__str = lm_27(Tpl::emptyTxt.clone(), i_parts_classParts.clone())?;
            ret_7 = (System::stringReplace((Tpl::textString(l_inputs__str.clone())?).clone(), (literal!("<:")).clone(), (literal!("")).clone())?).clone();
            l_typevar__inputs = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_7.clone()).clone())?;
            l_header = dumpClassHeader(Tpl::emptyTxt.clone(), i_parts.clone(), i_restriction.clone())?;
            ret_10 = MMToJuliaUtil::makeFunctionContext((Tpl::textString(l_return__str.clone())?).clone());
            l_functionBodyStr = dumpClassDef(Tpl::emptyTxt.clone(), i_parts.clone(), ret_10.clone(), a_options.clone())?;
            txt = Tpl::writeText(txt.clone(), l_commentStr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = fun_28(txt.clone(), l_header.clone(), l_typevar__inputs.clone(), l_inputs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") ")).clone() }))?;
            txt = fun_29(txt.clone(), l_header.clone(), l_returnType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_header.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_functionBodyStr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_return__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Class { body: i_parts @ Deref @ Absyn::ClassDef::PARTS { classParts: i_parts_classParts, comment: i_parts_comment, .. }, encapsulatedPrefix: i_encapsulatedPrefix, partialPrefix: i_partialPrefix, restriction: i_restriction, name: i_name, .. }, a_options, a_context) => {
            let mut l_partial__str__and__class__type: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_footer__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_header__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_cmt__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_cdef__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_begin__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_cdef__str2: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_inform: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut ret_16: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut l_forwardDeclarations: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_cdef__str1: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_class__type__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_partial__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_enc__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_enc__str = fun_30(Tpl::emptyTxt.clone(), i_encapsulatedPrefix.clone());
            l_partial__str = fun_31(Tpl::emptyTxt.clone(), i_partialPrefix.clone())?;
            l_class__type__str = dumpClassType(Tpl::emptyTxt.clone(), i_restriction.clone())?;
            l_cdef__str1 = fun_32(Tpl::emptyTxt.clone(), i_restriction.clone(), a_context.clone(), a_options.clone(), i_parts.clone())?;
            ret_16 = AbsynToSCode::translateClassdefElements(i_parts_classParts.clone())?;
            l_forwardDeclarations = dumpSCodeElements(Tpl::emptyTxt.clone(), ret_16.clone())?;
            l_inform = fun_33(Tpl::emptyTxt.clone(), l_forwardDeclarations.clone())?;
            l_cdef__str2 = fun_34(Tpl::emptyTxt.clone(), i_restriction.clone(), l_cdef__str1.clone(), l_forwardDeclarations.clone(), l_inform.clone())?;
            l_begin__str = fun_35(Tpl::emptyTxt.clone(), i_restriction.clone())?;
            l_cdef__str = Tpl::writeText(Tpl::emptyTxt.clone(), l_cdef__str2.clone())?;
            l_cmt__str = dumpCommentStrOpt(Tpl::emptyTxt.clone(), i_parts_comment.clone())?;
            l_header__str = dumpClassHeader(Tpl::emptyTxt.clone(), i_parts.clone(), i_restriction.clone())?;
            l_footer__str = dumpClassFooter(Tpl::emptyTxt.clone(), i_parts.clone(), (Tpl::textString(l_cdef__str.clone())?).clone(), (i_name.clone()).clone(), (Tpl::textString(l_cmt__str.clone())?).clone(), (literal!("")).clone())?;
            l_partial__str__and__class__type = Tpl::writeText(Tpl::emptyTxt.clone(), l_partial__str.clone())?;
            l_partial__str__and__class__type = Tpl::writeText(l_partial__str__and__class__type.clone(), l_class__type__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_partial__str__and__class__type.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_begin__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_header__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_footer__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { comment: i_parts_comment_1, typeSpec: i_parts_typeSpec, arguments: i_parts_arguments, attributes: i_parts_attributes }, restriction: Absyn::Restriction::R_TYPE { .. }, name: i_name, .. }, _, a_context) => {
            let mut l_attr: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_args: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_spec: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_comment: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_comment = dumpCommentOpt(Tpl::emptyTxt.clone(), i_parts_comment_1.clone(), a_context.clone())?;
            l_spec = dumpTypeSpec(Tpl::emptyTxt.clone(), i_parts_typeSpec.clone(), a_context.clone())?;
            l_args = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_args = lm_36(l_args.clone(), i_parts_arguments.clone(), a_context.clone())?;
            l_args = Tpl::popIter(l_args.clone())?;
            l_attr = dumpElementAttr(Tpl::emptyTxt.clone(), i_parts_attributes.clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_spec.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_attr.clone())?;
            txt = Tpl::writeText(txt.clone(), l_comment.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { comment: i_parts_comment_1, typeSpec: i_parts_typeSpec, arguments: i_parts_arguments, attributes: i_parts_attributes }, restriction: Absyn::Restriction::R_FUNCTION { functionRestriction: _ }, name: i_name, .. }, _, a_context) => {
            let mut l_name__of__new__function: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_attr: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_args: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_spec: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_comment: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_comment = dumpCommentOpt(Tpl::emptyTxt.clone(), i_parts_comment_1.clone(), a_context.clone())?;
            l_spec = dumpTypeSpec(Tpl::emptyTxt.clone(), i_parts_typeSpec.clone(), a_context.clone())?;
            l_args = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_args = lm_37(l_args.clone(), i_parts_arguments.clone(), a_context.clone())?;
            l_args = Tpl::popIter(l_args.clone())?;
            l_attr = dumpElementAttr(Tpl::emptyTxt.clone(), i_parts_attributes.clone())?;
            l_name__of__new__function = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_comment.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("@ExtendedFunction ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__of__new__function.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_spec.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassHeader(mut in_txt: Tpl::Text, mut in_a_classDef: Arc<Absyn::ClassDef>, mut in_a_restriction: Absyn::Restriction) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_classDef.clone(), in_a_restriction.clone())) {
        (txt, Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: _, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("Extend  not supported")).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ClassDef::PARTS { typeVars: i_typeVars, .. }, a_restriction) => {
            let mut txt = (*txt).clone();
            txt = dumpClassTypeTypeVars(txt.clone(), a_restriction.clone(), i_typeVars.clone())?;
            txt = dumpClassTypeSuperType(txt.clone(), a_restriction.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpClassHeader: <%dumpClassTypeSuperType(classDef)%>")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassTypeSuperType(mut in_txt: Tpl::Text, mut in_a_r: Absyn::Restriction) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_r.clone()) {
        (mut txt, Absyn::Restriction::R_METARECORD { name: ref i_name, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<: ")).clone() }))?;
            txt = dumpPathJL(txt.clone(), i_name.clone())?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_FUNCTION { functionRestriction: _ }) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_41(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_41 in &*items.clone() {
        let mut lstElt_41 = lstElt_41.clone();
        txt = (match lstElt_41.clone() {
        mut i_tv => {
            txt = Tpl::writeStr(txt.clone(), (i_tv.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_42(mut in_txt: Tpl::Text, mut in_a_typeVars: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_typeVars.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_typeVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_41(txt.clone(), i_typeVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_43(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_43 in &*items.clone() {
        let mut lstElt_43 = lstElt_43.clone();
        txt = (match lstElt_43.clone() {
        mut i_tv => {
            txt = Tpl::writeStr(txt.clone(), (i_tv.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

fn fun_44(mut in_txt: Tpl::Text, mut in_a_typeVars: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_typeVars.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_typeVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("where {")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_43(txt.clone(), i_typeVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassTypeTypeVars(mut in_txt: Tpl::Text, mut in_a_restriction: Absyn::Restriction, mut in_a_typeVars: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_restriction.clone(), in_a_typeVars.clone())) {
        (txt, Absyn::Restriction::R_UNIONTYPE { .. }, a_typeVars) => {
            let mut txt = (*txt).clone();
            txt = fun_42(txt.clone(), a_typeVars.clone())?;
            txt.clone()
        },
        (txt, Absyn::Restriction::R_FUNCTION { functionRestriction: _ }, a_typeVars) => {
            let mut txt = (*txt).clone();
            txt = fun_44(txt.clone(), a_typeVars.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_46(mut in_txt: Tpl::Text, mut in_a_ann: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ann.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_ann) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_ann.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_47(mut in_txt: Tpl::Text, mut in_a_annotation__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_annotation__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_48(mut in_txt: Tpl::Text, mut in_a_cdefStr: ArcStr, mut in_a_annotation__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cdefStr.clone(), in_a_annotation__str.clone())) {
        (txt, Deref @ "", a_annotation__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_annotation__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end")).clone() }))?;
            txt.clone()
        },
        (txt, i_cdefStr, a_annotation__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeStr(txt.clone(), (i_cdefStr.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = fun_47(txt.clone(), a_annotation__str.clone())?;
            txt = Tpl::writeText(txt.clone(), a_annotation__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_49(mut in_txt: Tpl::Text, mut in_a_classDef: Arc<Absyn::ClassDef>, mut in_a_cdefStr: ArcStr, mut in_a_ann: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_classDef.clone(), in_a_cdefStr.clone(), in_a_ann.clone())) {
        (txt, Deref @ Absyn::ClassDef::DERIVED { typeSpec: _, .. }, _, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpClassFooter: Derived not yet supported.")).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ClassDef::ENUMERATION { enumLiterals: _, .. }, _, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpClassFooterf: ENUMERATION not yet supported.")).clone())?;
            txt.clone()
        },
        (txt, _, a_cdefStr, a_ann) => {
            let mut l_annotation__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_annotation__str = fun_46(Tpl::emptyTxt.clone(), (a_ann.clone()).clone())?;
            txt = fun_48(txt.clone(), (a_cdefStr.clone()).clone(), l_annotation__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassFooter(mut txt: Tpl::Text, mut a_classDef: Arc<Absyn::ClassDef>, mut a_cdefStr: ArcStr, mut a_name: ArcStr, mut a_cmt: ArcStr, mut a_ann: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = fun_49(txt.clone(), a_classDef.clone(), (a_cdefStr.clone()).clone(), (a_ann.clone()).clone())?;
    Ok(out_txt)
}

fn lm_51(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_51 in &*items.clone() {
        let mut lstElt_51 = lstElt_51.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_51.clone()) {
        i_ei => {
            let mut ret_3: MMToJuliaUtil::Context = MMToJuliaUtil::Context::NO_CONTEXT;
            let mut txt_2: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut ret_1: Option<Arc<Absyn::TypeSpec>> = None;
            let mut ret_0: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
            ret_0 = AbsynUtil::getComponentItemsFromElementItem(i_ei.clone());
            ret_1 = AbsynUtil::getTypeSpecFromElementItemOpt(i_ei.clone());
            txt_2 = dumpTypeSpecOpt(Tpl::emptyTxt.clone(), ret_1.clone(), MMToJuliaUtil::inputContext.clone())?;
            ret_3 = MMToJuliaUtil::makeInputContext((Tpl::textString(txt_2.clone())?).clone());
            txt = dumpComponentItems(txt.clone(), ret_0.clone(), ret_3.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpInputsJL(mut txt: Tpl::Text, mut a_inputs: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut ret_3: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut ret_2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut ret_1: Absyn::Direction = Absyn::Direction::BIDIR;
    let mut l_inputStr: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    ret_1 = MMToJuliaUtil::makeInputDirection();
    ret_2 = MMToJuliaUtil::filterOnDirection(a_inputs.clone(), ret_1.clone());
    ret_3 = ret_2.clone().reverse();
    l_inputStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    l_inputStr = lm_51(l_inputStr.clone(), ret_3.clone())?;
    l_inputStr = Tpl::popIter(l_inputStr.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_inputStr.clone())?;
    Ok(out_txt)
}

fn fun_53(mut in_txt: Tpl::Text, mut in_mArg: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_L @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::")).clone() }))?;
            txt = dumpOutputsJL(txt.clone(), i_L.clone())?;
            txt.clone()
        },
        (txt, i_L @ Deref @ metamodelica::List::Cons { head: _, tail: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::Tuple{")).clone() }))?;
            txt = dumpOutputsJL(txt.clone(), i_L.clone())?;
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

pub fn dumpReturnTypeJL(mut txt: Tpl::Text, mut a_outputs: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut ret_1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut ret_0: Absyn::Direction = Absyn::Direction::BIDIR;
    ret_0 = MMToJuliaUtil::makeOutputDirection();
    ret_1 = MMToJuliaUtil::filterOnDirection(a_outputs.clone(), ret_0.clone());
    out_txt = fun_53(txt.clone(), ret_1.clone())?;
    Ok(out_txt)
}

fn lm_55(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_55 in &*items.clone() {
        let mut lstElt_55 = lstElt_55.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_55.clone()) {
        i_e => {
            txt = dumpElementItemRaw(txt.clone(), i_e.clone(), Dump::defaultDumpOptions.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_56(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_56 in &*items.clone() {
        let mut lstElt_56 = lstElt_56.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_56.clone()) {
        i_e => {
            txt = dumpElementItemRaw(txt.clone(), i_e.clone(), Dump::defaultDumpOptions.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_57(mut in_txt: Tpl::Text, mut in_mArg: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, i_L @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, a_context) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_55(txt.clone(), i_L.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (txt, i_L @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, a_context) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_56(txt.clone(), i_L.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
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

pub fn dumpReturnStrJL(mut txt: Tpl::Text, mut a_outputs: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut ret_2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut ret_1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut ret_0: Absyn::Direction = Absyn::Direction::BIDIR;
    ret_0 = MMToJuliaUtil::makeOutputDirection();
    ret_1 = MMToJuliaUtil::filterOnDirection(a_outputs.clone(), ret_0.clone());
    ret_2 = ret_1.clone().reverse();
    out_txt = fun_57(txt.clone(), ret_2.clone(), a_context.clone())?;
    Ok(out_txt)
}

fn lm_59(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut a_options: Dump::DumpOptions, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_59 in &*items.clone() {
        let mut lstElt_59 = lstElt_59.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_59.clone()) {
        i_class__part => {
            let mut x_idx: i32 = 0;
            x_idx = Tpl::getIteri_i0(txt.clone())?;
            txt = dumpClassPart(txt.clone(), i_class__part.clone(), x_idx.clone(), a_context.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpClassDef(mut in_txt: Tpl::Text, mut in_a_cdef: Arc<Absyn::ClassDef>, mut in_a_context: MMToJuliaUtil::Context, mut in_a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cdef.clone(), in_a_context.clone(), in_a_options.clone())) {
        (txt, Deref @ Absyn::ClassDef::PARTS { classParts: i_classParts, .. }, a_context, a_options) => {
            let mut l_body__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_body__str = lm_59(l_body__str.clone(), i_classParts.clone(), a_options.clone(), a_context.clone())?;
            l_body__str = Tpl::popIter(l_body__str.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ClassDef::DERIVED { typeSpec: _, .. }, _, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpClassDef: Derived not yet supported.")).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: _, .. }, _, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpClassDef: CLASS_EXETENDS not yet supported.")).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ClassDef::ENUMERATION { enumLiterals: _, .. }, _, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpClassDef: CLASS_ENUMERATION not yet supported.")).clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TODO Unkown class definition")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassType(mut in_txt: Tpl::Text, mut in_a_restriction: Absyn::Restriction) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_restriction.clone()) {
        (mut txt, Absyn::Restriction::R_PACKAGE { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("module")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_METARECORD { name: _, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("struct")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_RECORD { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("@Record")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_UNIONTYPE { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("uniontype")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_TYPE { .. }) => {
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_FUNCTION { functionRestriction: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function")).clone() }))?;
            txt.clone()
        },
        (mut txt, mut i_restriction) => {
            let mut txt_0: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("AbsynToJulia.dumpClassType: Unknown restriction for class.")).clone() }))?;
            txt_0 = AbsynDumpTpl::dumpRestriction(txt_0.clone(), i_restriction.clone())?;
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_62(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_options: Dump::DumpOptions, mut in_a_context: MMToJuliaUtil::Context, mut in_a_contents: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_options.clone(), in_a_context.clone(), in_a_contents.clone())) {
        (txt, false, a_options, a_context, a_contents) => {
            let mut txt = (*txt).clone();
            txt = dumpElementItems(txt.clone(), a_contents.clone(), a_context.clone(), (literal!("")).clone(), true, a_options.clone())?;
            txt.clone()
        },
        (txt, _, a_options, a_context, a_contents) => {
            let mut ret_1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut ret_0: Absyn::Direction = Absyn::Direction::BIDIR;
            let mut txt = (*txt).clone();
            ret_0 = MMToJuliaUtil::makeOutputDirection();
            ret_1 = MMToJuliaUtil::filterOnDirection(a_contents.clone(), ret_0.clone());
            txt = dumpElementItems(txt.clone(), ret_1.clone(), a_context.clone(), (literal!("")).clone(), true, a_options.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_63(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_63 in &*items.clone() {
        let mut lstElt_63 = lstElt_63.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_63.clone()) {
        i_eq => {
            txt = dumpAlgorithmItem(txt.clone(), i_eq.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_64(mut in_txt: Tpl::Text, mut in_a_annotation__: Option<Arc<Absyn::Annotation>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_annotation__.clone(), in_a_context.clone())) {
        (txt, Some(i_ann), a_context) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = dumpAnnotation(txt.clone(), i_ann.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_65(mut in_txt: Tpl::Text, mut in_a_externalDecl: Arc<Absyn::ExternalDecl>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_externalDecl.clone())) {
        (txt, Deref @ Absyn::ExternalDecl { funcName: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#= TODO: Defined in the runtime =#")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_66(mut in_txt: Tpl::Text, mut in_a_class__part: Arc<Absyn::ClassPart>, mut in_a_context: MMToJuliaUtil::Context, mut in_a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_class__part.clone(), in_a_context.clone(), in_a_options.clone())) {
        (txt, Deref @ Absyn::ClassPart::PUBLIC { contents: i_contents }, a_context, a_options) => {
            let mut ret_1: bool = false;
            let mut l_el__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            ret_1 = MMToJuliaUtil::isFunctionContext(a_context.clone());
            l_el__str = fun_62(Tpl::emptyTxt.clone(), ret_1.clone(), a_options.clone(), a_context.clone(), i_contents.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_el__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ClassPart::PROTECTED { contents: i_contents }, a_context, a_options) => {
            let mut l_el__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_el__str = dumpElementItems(Tpl::emptyTxt.clone(), i_contents.clone(), a_context.clone(), (literal!("")).clone(), true, a_options.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_el__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ClassPart::CONSTRAINTS { contents: _ }, _, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpClassPart: CONSTRAINTS(__) not supported.")).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ClassPart::EQUATIONS { contents: _ }, _, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpClassPart: EQUATIONS(__) not supported.")).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: _ }, _, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpClassPart: INITIALEQUATIONS() not supported.")).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ClassPart::ALGORITHMS { contents: i_contents_1 }, a_context, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_63(txt.clone(), i_contents_1.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: _ }, _, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpClassPart: INITIALALGORITHMS() not supported.")).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ClassPart::EXTERNAL { annotation_: i_annotation__, externalDecl: i_externalDecl }, a_context, _) => {
            let mut l_ann__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_ann__str = fun_64(Tpl::emptyTxt.clone(), i_annotation__.clone(), a_context.clone())?;
            txt = fun_65(txt.clone(), i_externalDecl.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassPart(mut txt: Tpl::Text, mut a_class__part: Arc<Absyn::ClassPart>, mut a_idx: i32, mut a_context: MMToJuliaUtil::Context, mut a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = fun_66(txt.clone(), a_class__part.clone(), a_context.clone(), a_options.clone())?;
    Ok(out_txt)
}

fn fun_68(mut in_txt: Tpl::Text, mut in_a_first: bool, mut in_a_prevSpacing: ArcStr, mut in_a_spacing: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_first.clone(), in_a_prevSpacing.clone(), in_a_spacing.clone()) {
        (mut txt, false, mut a_prevSpacing, mut a_spacing) => {
            txt = dumpElementItemPreSpacing(txt.clone(), (Tpl::textString(a_spacing.clone())?).clone(), (a_prevSpacing.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_69(mut in_txt: Tpl::Text, mut in_a_rest__str: Tpl::Text, mut in_a_spacing: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_rest__str.clone(), in_a_spacing.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, _, a_spacing) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_spacing.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_70(mut in_txt: Tpl::Text, mut in_a_rest__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_rest__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_rest__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_rest__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElementItems(mut in_txt: Tpl::Text, mut in_a_items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut in_a_context: MMToJuliaUtil::Context, mut in_a_prevSpacing: ArcStr, mut in_a_first: bool, mut in_a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_items.clone(), in_a_context.clone(), in_a_prevSpacing.clone(), in_a_first.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: i_item, tail: i_rest__items }, a_context, a_prevSpacing, a_first, a_options) => {
            let mut l_post__spacing: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_rest__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_item__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_pre__spacing: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_spacing: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_spacing = dumpElementItemSpacing(Tpl::emptyTxt.clone(), i_item.clone())?;
            l_pre__spacing = fun_68(Tpl::emptyTxt.clone(), a_first.clone(), (a_prevSpacing.clone()).clone(), l_spacing.clone())?;
            l_item__str = dumpElementItem(Tpl::emptyTxt.clone(), i_item.clone(), a_options.clone(), a_context.clone())?;
            l_rest__str = dumpElementItems(Tpl::emptyTxt.clone(), i_rest__items.clone(), a_context.clone(), (Tpl::textString(l_spacing.clone())?).clone(), false, a_options.clone())?;
            l_post__spacing = fun_69(Tpl::emptyTxt.clone(), l_rest__str.clone(), l_spacing.clone())?;
            txt = Tpl::writeText(txt.clone(), l_pre__spacing.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_item__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_post__spacing.clone())?;
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt = fun_70(txt.clone(), l_rest__str.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_72(mut in_txt: Tpl::Text, mut in_a_prevSpacing: ArcStr, mut in_a_curSpacing: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_prevSpacing.clone(), in_a_curSpacing.clone())) {
        (txt, Deref @ "", a_curSpacing) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_curSpacing.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElementItemPreSpacing(mut txt: Tpl::Text, mut a_curSpacing: ArcStr, mut a_prevSpacing: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = fun_72(txt.clone(), (a_prevSpacing.clone()).clone(), (a_curSpacing.clone()).clone())?;
    Ok(out_txt)
}

pub fn dumpElementItemSpacing(mut in_txt: Tpl::Text, mut in_a_item: Arc<Absyn::ElementItem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_item.clone())) {
        (txt, Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { body: i_cdef, .. }, .. }, .. } }) => {
            let mut txt = (*txt).clone();
            txt = dumpClassDefSpacing(txt.clone(), i_cdef.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassDefSpacing(mut in_txt: Tpl::Text, mut in_a_cdef: Arc<Absyn::ClassDef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cdef.clone())) {
        (txt, Deref @ Absyn::ClassDef::PARTS { typeVars: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElementItem(mut in_txt: Tpl::Text, mut in_a_eitem: Arc<Absyn::ElementItem>, mut in_a_options: Dump::DumpOptions, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eitem.clone(), in_a_options.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::ElementItem::ELEMENTITEM { element: i_element }, a_options, a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpElement(txt.clone(), i_element.clone(), a_options.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ElementItem::LEXER_COMMENT { comment: i_comment }, _, _) => {
            let mut txt = (*txt).clone();
            txt = dumpCommentStr(txt.clone(), (i_comment.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_77(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_77 in &*items.clone() {
        let mut lstElt_77 = lstElt_77.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_77.clone()) {
        i_comp => {
            txt = dumpComponentItem(txt.clone(), i_comp.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_78(mut in_txt: Tpl::Text, mut in_a_specification: Arc<Absyn::ElementSpec>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_specification.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::ElementSpec::COMPONENTS { components: i_components, .. }, a_context) => {
            let mut l_comps__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_comps__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_comps__str = lm_77(l_comps__str.clone(), i_components.clone(), a_context.clone())?;
            l_comps__str = Tpl::popIter(l_comps__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_comps__str.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpElementItem: on none component type")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_79(mut in_txt: Tpl::Text, mut in_a_element: Arc<Absyn::Element>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_element.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::Element::ELEMENT { specification: i_specification, .. }, a_context) => {
            let mut txt = (*txt).clone();
            txt = fun_78(txt.clone(), i_specification.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpElementItem: on none component type")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_80(mut in_txt: Tpl::Text, mut in_a_eitem: Arc<Absyn::ElementItem>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eitem.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::ElementItem::ELEMENTITEM { element: i_element }, a_context) => {
            let mut txt = (*txt).clone();
            txt = fun_79(txt.clone(), i_element.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ElementItem::LEXER_COMMENT { comment: i_comment }, _) => {
            let mut txt = (*txt).clone();
            txt = dumpCommentStr(txt.clone(), (i_comment.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElementItemRaw(mut txt: Tpl::Text, mut a_eitem: Arc<Absyn::ElementItem>, mut a_options: Dump::DumpOptions, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = fun_80(txt.clone(), a_eitem.clone(), a_context.clone())?;
    Ok(out_txt)
}

fn fun_82(mut in_txt: Tpl::Text, mut in_a_redeclareKeywords: Option<Absyn::RedeclareKeywords>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_redeclareKeywords.clone()) {
        (mut txt, Some(mut i_re)) => {
            txt = dumpRedeclare(txt.clone(), i_re.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_83(mut in_txt: Tpl::Text, mut in_a_redeclareKeywords: Option<Absyn::RedeclareKeywords>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_redeclareKeywords.clone()) {
        (mut txt, Some(mut i_re)) => {
            txt = dumpReplaceable(txt.clone(), i_re.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_84(mut in_txt: Tpl::Text, mut in_a_constrainClass: Option<Arc<Absyn::ConstrainClass>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_constrainClass.clone(), in_a_context.clone())) {
        (txt, Some(i_cc), a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpConstrainClass(txt.clone(), i_cc.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_85(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_constrainClass: Option<Arc<Absyn::ConstrainClass>>, mut in_a_context: MMToJuliaUtil::Context, mut in_a_options: Dump::DumpOptions, mut in_a_specification: Arc<Absyn::ElementSpec>, mut in_a_redeclareKeywords: Option<Absyn::RedeclareKeywords>, mut in_a_finalPrefix: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_constrainClass.clone(), in_a_context.clone(), in_a_options.clone(), in_a_specification.clone(), in_a_redeclareKeywords.clone(), in_a_finalPrefix.clone())) {
        (txt, false, _, _, _, _, _, _) => {
            txt.clone()
        },
        (txt, _, a_constrainClass, a_context, a_options, a_specification, a_redeclareKeywords, a_finalPrefix) => {
            let mut l_constrainClass__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_elementSpec__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_repl__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_redecl__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_final__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_final__str = dumpFinal(Tpl::emptyTxt.clone(), a_finalPrefix.clone())?;
            l_redecl__str = fun_82(Tpl::emptyTxt.clone(), a_redeclareKeywords.clone())?;
            l_repl__str = fun_83(Tpl::emptyTxt.clone(), a_redeclareKeywords.clone())?;
            l_elementSpec__str = dumpElementSpec(Tpl::emptyTxt.clone(), a_specification.clone(), a_options.clone(), a_context.clone())?;
            l_constrainClass__str = fun_84(Tpl::emptyTxt.clone(), a_constrainClass.clone(), a_context.clone())?;
            txt = Tpl::writeText(txt.clone(), l_elementSpec__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_constrainClass__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_86(mut in_txt: Tpl::Text, mut in_a_optName: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_optName.clone()) {
        (mut txt, Some(mut i_name)) => {
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_87(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_string: ArcStr, mut in_a_info: SourceInfo, mut in_a_optName: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_string.clone(), in_a_info.clone(), in_a_optName.clone()) {
        (mut txt, false, _, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_string, mut a_info, mut a_optName) => {
            let mut l_info__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_name__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            l_name__str = fun_86(Tpl::emptyTxt.clone(), a_optName.clone())?;
            l_info__str = dumpInfo(Tpl::emptyTxt.clone(), a_info.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* Absyn.TEXT(SOME(\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"), \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_string.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", \"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_info__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"); */")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpElement(mut in_txt: Tpl::Text, mut in_a_elem: Arc<Absyn::Element>, mut in_a_options: Dump::DumpOptions, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_elem.clone(), in_a_options.clone(), in_a_context.clone())) {
        (txt, i_elem @ Deref @ Absyn::Element::ELEMENT { info: i_info, finalPrefix: i_finalPrefix, redeclareKeywords: i_redeclareKeywords, specification: i_specification, constrainClass: i_constrainClass, .. }, a_options, a_context) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = Dump::boolUnparseFileFromInfo(i_info.clone(), a_options.clone())?;
            ret_1 = AbsynUtil::isClassdef(i_elem.clone());
            ret_2 = boolNot(ret_1.clone());
            ret_3 = boolOr(ret_0.clone(), ret_2.clone());
            txt = fun_85(txt.clone(), ret_3.clone(), i_constrainClass.clone(), a_context.clone(), a_options.clone(), i_specification.clone(), i_redeclareKeywords.clone(), i_finalPrefix.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Element::DEFINEUNIT { name: _, .. }, _, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpElement: DEFINEUNIT(__) not supported")).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Element::TEXT { info: i_info, optName: i_optName, string: i_string }, a_options, _) => {
            let mut ret_4: bool = false;
            let mut txt = (*txt).clone();
            ret_4 = Dump::boolUnparseFileFromInfo(i_info.clone(), a_options.clone())?;
            txt = fun_87(txt.clone(), ret_4.clone(), (i_string.clone()).clone(), i_info.clone(), i_optName.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_89(mut in_txt: Tpl::Text, mut in_a_isReadOnly: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_isReadOnly.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("writable")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("readonly")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpInfo(mut in_txt: Tpl::Text, mut in_a_info: SourceInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_info.clone()) {
        (mut txt, SourceInfo { isReadOnly: mut i_isReadOnly, fileName: mut i_fileName, lineNumberStart: mut i_lineNumberStart, columnNumberStart: mut i_columnNumberStart, lineNumberEnd: mut i_lineNumberEnd, columnNumberEnd: mut i_columnNumberEnd, .. }) => {
            let mut l_rm__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            l_rm__str = fun_89(Tpl::emptyTxt.clone(), i_isReadOnly.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOURCEINFO(\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rm__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_lineNumberStart.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_columnNumberStart.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_lineNumberEnd.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_columnNumberEnd.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")\\n")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_91(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_91 in &*items.clone() {
        let mut lstElt_91 = lstElt_91.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_91.clone()) {
        i_earg => {
            txt = dumpElementArg(txt.clone(), i_earg.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpAnnotation(mut in_txt: Tpl::Text, mut in_a_ann: Arc<Absyn::Annotation>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ann.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::Annotation { elementArgs: Deref @ metamodelica::List::Nil }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#= annotation() =#")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Annotation { elementArgs: i_elementArgs }, a_context) => {
            let mut ret_1: Arc<Tpl::StringToken> = Arc::new(Tpl::StringToken::ST_NEW_LINE);
            let mut txt_0: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#= annotation(\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt_0 = Tpl::writeTok(txt_0.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
            ret_1 = Tpl::textStrTok(txt_0.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(ret_1.clone()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_91(txt.clone(), i_elementArgs.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") =#")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAnnotationOpt(mut in_txt: Tpl::Text, mut in_a_oann: Option<Arc<Absyn::Annotation>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_oann.clone(), in_a_context.clone())) {
        (txt, Some(i_ann), a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpAnnotation(txt.clone(), i_ann.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAnnotationOptSpace(mut in_txt: Tpl::Text, mut in_a_oann: Option<Arc<Absyn::Annotation>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_oann.clone(), in_a_context.clone())) {
        (txt, Some(i_ann), a_context) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = dumpAnnotation(txt.clone(), i_ann.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpComment(mut in_txt: Tpl::Text, mut in_a_cmt: Arc<Absyn::Comment>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cmt.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::Comment { comment: i_comment, annotation_: i_annotation__ }, a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpCommentStrOpt(txt.clone(), i_comment.clone())?;
            txt = dumpAnnotationOptSpace(txt.clone(), i_annotation__.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpCommentOpt(mut in_txt: Tpl::Text, mut in_a_ocmt: Option<Arc<Absyn::Comment>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ocmt.clone(), in_a_context.clone())) {
        (txt, Some(i_cmt), a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpComment(txt.clone(), i_cmt.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpCommentStrOpt(mut in_txt: Tpl::Text, mut in_a_comment: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_comment.clone()) {
        (mut txt, Some(mut i_cmt)) => {
            txt = dumpCommentStr(txt.clone(), (i_cmt.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpCommentStr(mut txt: Tpl::Text, mut a_comment: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut ret_2: ArcStr = arcstr::literal!("");
    let mut ret_1: ArcStr = arcstr::literal!("");
    let mut l_replaceAllRegular: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    l_replaceAllRegular = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    l_replaceAllRegular = Tpl::writeTok(l_replaceAllRegular.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#= ")).clone() }))?;
    ret_1 = (System::escapedString((a_comment.clone()).clone(), false)).clone();
    ret_2 = (System::stringReplace((ret_1.clone()).clone(), (literal!("//")).clone(), (literal!("")).clone())?).clone();
    l_replaceAllRegular = Tpl::writeStr(l_replaceAllRegular.clone(), (ret_2.clone()).clone())?;
    l_replaceAllRegular = Tpl::writeTok(l_replaceAllRegular.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" =#")).clone() }))?;
    out_txt = Tpl::writeText(txt.clone(), l_replaceAllRegular.clone())?;
    Ok(out_txt)
}

fn fun_99(mut in_txt: Tpl::Text, mut in_a_modification: Option<Arc<Absyn::Modification>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modification.clone(), in_a_context.clone())) {
        (txt, Some(i_mod), a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpModification(txt.clone(), i_mod.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_100(mut in_txt: Tpl::Text, mut in_a_constrainClass: Option<Arc<Absyn::ConstrainClass>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_constrainClass.clone(), in_a_context.clone())) {
        (txt, Some(i_cc), a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpConstrainClass(txt.clone(), i_cc.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElementArg(mut in_txt: Tpl::Text, mut in_a_earg: Arc<Absyn::ElementArg>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_earg.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::ElementArg::MODIFICATION { eachPrefix: i_eachPrefix, finalPrefix: i_finalPrefix, path: i_path, modification: i_modification, comment: i_comment, .. }, a_context) => {
            let mut l_cmt__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_mod__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_path__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_final__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_each__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_each__str = dumpEach(Tpl::emptyTxt.clone(), i_eachPrefix.clone())?;
            l_final__str = dumpFinal(Tpl::emptyTxt.clone(), i_finalPrefix.clone())?;
            l_path__str = dumpPathJL(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_mod__str = fun_99(Tpl::emptyTxt.clone(), i_modification.clone(), a_context.clone())?;
            l_cmt__str = dumpCommentStrOpt(Tpl::emptyTxt.clone(), i_comment.clone())?;
            txt = Tpl::writeText(txt.clone(), l_each__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_final__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_path__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_mod__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ElementArg::REDECLARATION { eachPrefix: i_eachPrefix, finalPrefix: i_finalPrefix, redeclareKeywords: i_redeclareKeywords, elementSpec: i_elementSpec, constrainClass: i_constrainClass, .. }, a_context) => {
            let mut l_cc__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_elem__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_eredecl__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_repl__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_redecl__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_final__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_each__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_each__str = dumpEach(Tpl::emptyTxt.clone(), i_eachPrefix.clone())?;
            l_final__str = dumpFinal(Tpl::emptyTxt.clone(), i_finalPrefix.clone())?;
            l_redecl__str = dumpRedeclare(Tpl::emptyTxt.clone(), i_redeclareKeywords.clone())?;
            l_repl__str = dumpReplaceable(Tpl::emptyTxt.clone(), i_redeclareKeywords.clone())?;
            l_eredecl__str = Tpl::writeText(Tpl::emptyTxt.clone(), l_redecl__str.clone())?;
            l_eredecl__str = Tpl::writeText(l_eredecl__str.clone(), l_each__str.clone())?;
            l_elem__str = dumpElementSpec(Tpl::emptyTxt.clone(), i_elementSpec.clone(), Dump::defaultDumpOptions.clone(), a_context.clone())?;
            l_cc__str = fun_100(Tpl::emptyTxt.clone(), i_constrainClass.clone(), a_context.clone())?;
            txt = Tpl::writeText(txt.clone(), l_elem__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cc__str.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpEach(mut in_txt: Tpl::Text, mut in_a_each: Absyn::Each) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_each.clone()) {
        (mut txt, Absyn::Each::EACH { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("each ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpFinal(mut in_txt: Tpl::Text, mut in_a_final: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_final.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("final ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpRedeclare(mut in_txt: Tpl::Text, mut in_a_redecl: Absyn::RedeclareKeywords) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_redecl.clone()) {
        (mut txt, Absyn::RedeclareKeywords::REDECLARE { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("redeclare ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("redeclare ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpReplaceable(mut in_txt: Tpl::Text, mut in_a_repl: Absyn::RedeclareKeywords) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_repl.clone()) {
        (mut txt, Absyn::RedeclareKeywords::REPLACEABLE { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("replaceable ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("replaceable ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_106(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_106 in &*items.clone() {
        let mut lstElt_106 = lstElt_106.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_106.clone()) {
        i_earg => {
            txt = dumpElementArg(txt.clone(), i_earg.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_107(mut in_txt: Tpl::Text, mut in_a_elementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_elementArgLst.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, i_elementArgLst, a_context) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_106(txt.clone(), i_elementArgLst.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpModification(mut in_txt: Tpl::Text, mut in_a_mod: Arc<Absyn::Modification>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_mod.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::Modification { elementArgLst: i_elementArgLst, eqMod: i_eqMod }, a_context) => {
            let mut l_eq__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_arg__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_arg__str = fun_107(Tpl::emptyTxt.clone(), i_elementArgLst.clone(), a_context.clone())?;
            l_eq__str = dumpEqMod(Tpl::emptyTxt.clone(), i_eqMod.clone(), a_context.clone())?;
            txt = Tpl::writeText(txt.clone(), l_arg__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_eq__str.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpEqMod(mut in_txt: Tpl::Text, mut in_a_eqmod: Arc<Absyn::EqMod>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eqmod.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::EqMod::EQMOD { exp: i_exp, .. }, a_context) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("= ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_110(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_110 in &*items.clone() {
        let mut lstElt_110 = lstElt_110.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_110.clone()) {
        i_earg => {
            txt = dumpElementArg(txt.clone(), i_earg.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_111(mut in_txt: Tpl::Text, mut in_a_args__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_args__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_112(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut a_ty__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_112 in &*items.clone() {
        let mut lstElt_112 = lstElt_112.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_112.clone()) {
        i_comp => {
            let mut ret_1: MMToJuliaUtil::Context = MMToJuliaUtil::Context::NO_CONTEXT;
            let mut l_comp__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            ret_1 = MMToJuliaUtil::makeFunctionReturnContext((literal!("")).clone(), (Tpl::textString(a_ty__str.clone())?).clone());
            l_comp__str = dumpComponentItem(Tpl::emptyTxt.clone(), i_comp.clone(), ret_1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("local ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_comp__str.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_113(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty__str: Tpl::Text, mut in_a_components: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ty__str.clone(), in_a_components.clone())) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_ty__str, a_components) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_112(txt.clone(), a_components.clone(), a_ty__str.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_114(mut in_txt: Tpl::Text, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_context.clone()) {
        (mut txt, MMToJuliaUtil::Context::PACKAGE { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("const ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_115(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut a_ty__str: Tpl::Text, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_115 in &*items.clone() {
        let mut lstElt_115 = lstElt_115.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_115.clone()) {
        i_comp => {
            let mut l_comp__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            l_comp__str = dumpComponentItem(Tpl::emptyTxt.clone(), i_comp.clone(), MMToJuliaUtil::noContext.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = fun_114(txt.clone(), a_context.clone())?;
            txt = Tpl::writeText(txt.clone(), l_comp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_ty__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_116(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty__str: Tpl::Text, mut in_a_context: MMToJuliaUtil::Context, mut in_a_components: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ty__str.clone(), in_a_context.clone(), in_a_components.clone())) {
        (txt, false, _, _, _) => {
            txt.clone()
        },
        (txt, _, a_ty__str, a_context, a_components) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_115(txt.clone(), a_components.clone(), a_ty__str.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_117(mut in_txt: Tpl::Text, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_context.clone()) {
        (mut txt, MMToJuliaUtil::Context::FUNCTION { retValsStr: mut i_retValsStr }) => {
            txt = Tpl::writeStr(txt.clone(), (i_retValsStr.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_118(mut in_txt: Tpl::Text, mut in_a_comps__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comps__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_comps__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_comps__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_119(mut in_txt: Tpl::Text, mut in_a_context: MMToJuliaUtil::Context, mut in_a_comps__str__no__local: Tpl::Text, mut in_a_comps__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_context.clone(), in_a_comps__str__no__local.clone(), in_a_comps__str.clone()) {
        (mut txt, MMToJuliaUtil::Context::FUNCTION { retValsStr: _ }, _, mut a_comps__str) => {
            txt = fun_118(txt.clone(), a_comps__str.clone())?;
            txt.clone()
        },
        (mut txt, MMToJuliaUtil::Context::UNIONTYPE { name: _ }, mut a_comps__str__no__local, _) => {
            txt = Tpl::writeText(txt.clone(), a_comps__str__no__local.clone())?;
            txt.clone()
        },
        (mut txt, MMToJuliaUtil::Context::PACKAGE { .. }, mut a_comps__str__no__local, _) => {
            txt = Tpl::writeText(txt.clone(), a_comps__str__no__local.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ERROR")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpElementSpec(mut in_txt: Tpl::Text, mut in_a_specification: Arc<Absyn::ElementSpec>, mut in_a_options: Dump::DumpOptions, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_specification.clone(), in_a_options.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::ElementSpec::CLASSDEF { class_: i_class__, .. }, a_options, a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpClassElement(txt.clone(), i_class__.clone(), a_options.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ElementSpec::EXTENDS { path: i_path, elementArg: i_elementArg, annotationOpt: i_annotationOpt }, _, a_context) => {
            let mut l_ann__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_mod__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_args__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_bc__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_bc__str = dumpPathJL(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_args__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_args__str = lm_110(l_args__str.clone(), i_elementArg.clone(), a_context.clone())?;
            l_args__str = Tpl::popIter(l_args__str.clone())?;
            l_mod__str = fun_111(Tpl::emptyTxt.clone(), l_args__str.clone())?;
            l_ann__str = dumpAnnotationOptSpace(Tpl::emptyTxt.clone(), i_annotationOpt.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("extends ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_bc__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_mod__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ann__str.clone())?;
            txt.clone()
        },
        (txt, i_specification @ Deref @ Absyn::ElementSpec::COMPONENTS { attributes: i_attributes, typeSpec: i_typeSpec, components: i_components }, _, a_context) => {
            let mut l_rStr: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut ret_9: bool = false;
            let mut l_comps__str__no__local: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut ret_7: bool = false;
            let mut l_comps__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_ty__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_attr__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_attr__str = dumpElementAttr(Tpl::emptyTxt.clone(), i_attributes.clone())?;
            l_ty__str = dumpTypeSpec(Tpl::emptyTxt.clone(), i_typeSpec.clone(), a_context.clone())?;
            ret_7 = MMToJuliaUtil::elementSpecIsOUTPUT_OR_BIDIR(i_specification.clone());
            l_comps__str = fun_113(Tpl::emptyTxt.clone(), ret_7.clone(), l_ty__str.clone(), i_components.clone())?;
            ret_9 = MMToJuliaUtil::elementSpecIsOUTPUT_OR_BIDIR(i_specification.clone());
            l_comps__str__no__local = fun_116(Tpl::emptyTxt.clone(), ret_9.clone(), l_ty__str.clone(), a_context.clone(), i_components.clone())?;
            l_rStr = fun_117(Tpl::emptyTxt.clone(), a_context.clone())?;
            txt = fun_119(txt.clone(), a_context.clone(), l_comps__str__no__local.clone(), l_comps__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ElementSpec::IMPORT { import_: i_import__, .. }, _, _) => {
            let mut l_imp__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_imp__str = dumpImport(Tpl::emptyTxt.clone(), i_import__.clone())?;
            txt = Tpl::writeText(txt.clone(), l_imp__str.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_121(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_121 in &*items.clone() {
        let mut lstElt_121 = lstElt_121.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_121.clone()) {
        i_comp => {
            txt = dumpComponentItem(txt.clone(), i_comp.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_122(mut in_txt: Tpl::Text, mut in_a_specification: Arc<Absyn::ElementSpec>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_specification.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::ElementSpec::COMPONENTS { components: i_components, .. }, a_context) => {
            let mut l_comps__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_comps__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_comps__str = lm_121(l_comps__str.clone(), i_components.clone(), a_context.clone())?;
            l_comps__str = Tpl::popIter(l_comps__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_comps__str.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElementSpecForComponents(mut txt: Tpl::Text, mut a_specification: Arc<Absyn::ElementSpec>, mut a_options: Dump::DumpOptions, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = fun_122(txt.clone(), a_specification.clone(), a_context.clone())?;
    Ok(out_txt)
}

pub fn dumpElementAttr(mut in_txt: Tpl::Text, mut in_a_attr: Absyn::ElementAttributes) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_attr.clone()) {
        (mut txt, Absyn::ElementAttributes { variability: mut i_variability, .. }) => {
            let mut l_var__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            l_var__str = dumpVariability(Tpl::emptyTxt.clone(), i_variability.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var__str.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpVariability(mut in_txt: Tpl::Text, mut in_a_var: Absyn::Variability) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, Absyn::Variability::VAR { .. }) => {
            txt.clone()
        },
        (mut txt, Absyn::Variability::CONST { .. }) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpVariability: Only const and var are supported")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_126(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_126 in &*items.clone() {
        let mut lstElt_126 = lstElt_126.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_126.clone()) {
        i_e => {
            txt = dumpElementArg(txt.clone(), i_e.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_127(mut in_txt: Tpl::Text, mut in_a_el: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_el.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, i_el, a_context) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_126(txt.clone(), i_el.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpConstrainClass(mut in_txt: Tpl::Text, mut in_a_cc: Arc<Absyn::ConstrainClass>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cc.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::ConstrainClass { elementSpec: Deref @ Absyn::ElementSpec::EXTENDS { path: i_p, elementArg: i_el, .. }, comment: i_comment }, a_context) => {
            let mut l_cmt__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_el__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_path__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_path__str = dumpPathJL(Tpl::emptyTxt.clone(), i_p.clone())?;
            l_el__str = fun_127(Tpl::emptyTxt.clone(), i_el.clone(), a_context.clone())?;
            l_cmt__str = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment.clone(), a_context.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constrainedby ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_path__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_el__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_129(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_129 in &*items.clone() {
        let mut lstElt_129 = lstElt_129.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_129.clone()) {
        i_ci => {
            txt = dumpComponentItemWithoutCondString(txt.clone(), i_ci.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpComponentItems(mut txt: Tpl::Text, mut a_componentItems: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_129(out_txt.clone(), a_componentItems.clone(), a_context.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpComponentItem(mut in_txt: Tpl::Text, mut in_a_comp: Arc<Absyn::ComponentItem>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comp.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::ComponentItem { component: i_component, condition: i_condition, comment: i_comment }, a_context) => {
            let mut l_cmt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_cond__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_comp__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_comp__str = dumpComponent(Tpl::emptyTxt.clone(), i_component.clone(), a_context.clone())?;
            l_cond__str = dumpComponentCondition(Tpl::emptyTxt.clone(), i_condition.clone(), a_context.clone())?;
            l_cmt = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment.clone(), a_context.clone())?;
            txt = Tpl::writeText(txt.clone(), l_comp__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpComponentItemWithoutCondString(mut in_txt: Tpl::Text, mut in_a_comp: Arc<Absyn::ComponentItem>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comp.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::ComponentItem { component: i_component, comment: i_comment, .. }, a_context) => {
            let mut l_cmt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_comp__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_comp__str = dumpComponent(Tpl::emptyTxt.clone(), i_component.clone(), a_context.clone())?;
            l_cmt = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment.clone(), a_context.clone())?;
            txt = Tpl::writeText(txt.clone(), l_comp__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_133(mut in_txt: Tpl::Text, mut in_a_modification: Option<Arc<Absyn::Modification>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modification.clone(), in_a_context.clone())) {
        (txt, Some(i_mod), a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpModification(txt.clone(), i_mod.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_134(mut in_txt: Tpl::Text, mut in_a_context: MMToJuliaUtil::Context, mut in_a_mod__str: Tpl::Text, mut in_a_dim__str: Tpl::Text, mut in_a_component__name: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_context.clone(), in_a_mod__str.clone(), in_a_dim__str.clone(), in_a_component__name.clone()) {
        (mut txt, MMToJuliaUtil::Context::FUNCTION_RETURN_CONTEXT { ty_str: mut i_ty__str, .. }, mut a_mod__str, mut a_dim__str, mut a_component__name) => {
            txt = Tpl::writeText(txt.clone(), a_component__name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_ty__str.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), a_dim__str.clone())?;
            txt = Tpl::writeText(txt.clone(), a_mod__str.clone())?;
            txt.clone()
        },
        (mut txt, MMToJuliaUtil::Context::FUNCTION { retValsStr: _ }, _, _, mut a_component__name) => {
            txt = Tpl::writeText(txt.clone(), a_component__name.clone())?;
            txt.clone()
        },
        (mut txt, MMToJuliaUtil::Context::INPUT_CONTEXT { ty_str: mut i_ty__str }, mut a_mod__str, mut a_dim__str, mut a_component__name) => {
            txt = Tpl::writeText(txt.clone(), a_component__name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_ty__str.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), a_dim__str.clone())?;
            txt = Tpl::writeText(txt.clone(), a_mod__str.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_mod__str, mut a_dim__str, mut a_component__name) => {
            txt = Tpl::writeText(txt.clone(), a_component__name.clone())?;
            txt = Tpl::writeText(txt.clone(), a_dim__str.clone())?;
            txt = Tpl::writeText(txt.clone(), a_mod__str.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpComponent(mut in_txt: Tpl::Text, mut in_a_comp: Absyn::Component, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_comp.clone(), in_a_context.clone()) {
        (mut txt, Absyn::Component { arrayDim: ref i_arrayDim, modification: mut i_modification, name: mut i_name }, mut a_context) => {
            let mut l_component__name: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_mod__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_dim__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            l_dim__str = dumpSubscripts(Tpl::emptyTxt.clone(), i_arrayDim.clone(), a_context.clone())?;
            l_mod__str = fun_133(Tpl::emptyTxt.clone(), i_modification.clone(), a_context.clone())?;
            l_component__name = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_name.clone()).clone())?;
            txt = fun_134(txt.clone(), a_context.clone(), l_mod__str.clone(), l_dim__str.clone(), l_component__name.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpComponentCondition(mut in_txt: Tpl::Text, mut in_a_cond: Option<Arc<Absyn::Exp>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cond.clone(), in_a_context.clone())) {
        (txt, Some(i_cexp), a_context) => {
            let mut l_exp__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_cexp.clone(), a_context.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_137(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_path__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_path__str.clone())) {
        (txt, Deref @ "Array", _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("import ArrayUtil")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "List", _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("import ListUtil")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_path__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("import ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_path__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_138(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Absyn::GroupImport>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_138 in &*items.clone() {
        let mut lstElt_138 = lstElt_138.clone();
        txt = (match lstElt_138.clone() {
        mut i_group => {
            txt = dumpGroupImport(txt.clone(), i_group.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    }
    Ok(txt)
}

pub fn dumpImport(mut in_txt: Tpl::Text, mut in_a_imp: Absyn::Import) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_imp.clone()) {
        (mut txt, Absyn::Import::NAMED_IMPORT { path: ref i_path, name: mut i_name }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("import ")).clone() }))?;
            txt = dumpPathJL(txt.clone(), i_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("=")).clone() }))?;
            txt = dumpPathJL(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (mut txt, Absyn::Import::QUAL_IMPORT { path: ref i_path }) => {
            let mut str_1: ArcStr = arcstr::literal!("");
            let mut l_path__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            l_path__str = dumpPathJL(Tpl::emptyTxt.clone(), i_path.clone())?;
            str_1 = (Tpl::textString(l_path__str.clone())?).clone();
            txt = fun_137(txt.clone(), (str_1.clone()).clone(), l_path__str.clone())?;
            txt.clone()
        },
        (mut txt, Absyn::Import::UNQUAL_IMPORT { path: ref i_path }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("using ")).clone() }))?;
            txt = dumpPathJL(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (mut txt, Absyn::Import::GROUP_IMPORT { prefix: ref i_prefix, groups: ref i_groups }) => {
            let mut l_groups__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_prefix__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            l_prefix__str = dumpPathJL(Tpl::emptyTxt.clone(), i_prefix.clone())?;
            l_groups__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_groups__str = lm_138(l_groups__str.clone(), i_groups.clone())?;
            l_groups__str = Tpl::popIter(l_groups__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("using ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_prefix__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(": ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_groups__str.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpGroupImport(mut in_txt: Tpl::Text, mut in_a_gimp: Absyn::GroupImport) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_gimp.clone()) {
        (mut txt, Absyn::GroupImport::GROUP_IMPORT_NAME { name: mut i_name }) => {
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, Absyn::GroupImport::GROUP_IMPORT_RENAME { rename: mut i_rename, name: mut i_name }) => {
            txt = Tpl::writeStr(txt.clone(), (i_rename.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpEquation(mut txt: Tpl::Text, mut a_eq: Arc<Absyn::Equation>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("No equations allowed. Translate them to algorithms")).clone() }))?;
    Ok(out_txt)
}

fn lm_142(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_142 in &*items.clone() {
        let mut lstElt_142 = lstElt_142.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_142.clone()) {
        i_alg => {
            txt = dumpAlgorithmItem(txt.clone(), i_alg.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpAlgorithmItems(mut txt: Tpl::Text, mut a_algs: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_142(out_txt.clone(), a_algs.clone(), a_context.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpAlgorithmItem(mut in_txt: Tpl::Text, mut in_a_alg: Arc<Absyn::AlgorithmItem>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_alg.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: i_algorithm__, comment: i_comment, .. }, a_context) => {
            let mut l_cmt__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_alg__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_alg__str = dumpAlgorithm(Tpl::emptyTxt.clone(), i_algorithm__.clone(), a_context.clone())?;
            l_cmt__str = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment.clone(), a_context.clone())?;
            txt = Tpl::writeText(txt.clone(), l_alg__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::AlgorithmItem::ALGORITHMITEMCOMMENT { comment: i_comment_1 }, _) => {
            let mut txt = (*txt).clone();
            txt = dumpCommentStr(txt.clone(), (i_comment_1.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_145(mut in_txt: Tpl::Text, mut in_a_assignComponent: Arc<Absyn::Exp>, mut in_a_rhs__str: Tpl::Text, mut in_a_lhs__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_assignComponent.clone(), in_a_rhs__str.clone(), in_a_lhs__str.clone())) {
        (txt, Deref @ Absyn::Exp::CONS { head: _, .. }, a_rhs__str, a_lhs__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("@match ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_rhs__str.clone())?;
            txt.clone()
        },
        (txt, _, a_rhs__str, a_lhs__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_rhs__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_146(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_assignComponent: Arc<Absyn::Exp>, mut in_a_rhs__str: Tpl::Text, mut in_a_lhs__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_assignComponent.clone(), in_a_rhs__str.clone(), in_a_lhs__str.clone())) {
        (txt, false, _, a_rhs__str, a_lhs__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("@match ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_rhs__str.clone())?;
            txt.clone()
        },
        (txt, _, a_assignComponent, a_rhs__str, a_lhs__str) => {
            let mut txt = (*txt).clone();
            txt = fun_145(txt.clone(), a_assignComponent.clone(), a_rhs__str.clone(), a_lhs__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_147(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_147 in &*items.clone() {
        let mut lstElt_147 = lstElt_147.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_147.clone()) {
        (i_c, i_b) => {
            txt = dumpAlgorithmBranch(txt.clone(), i_c.clone(), i_b.clone(), (literal!("elseif")).clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_148(mut in_txt: Tpl::Text, mut in_a_else__branch__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_else__branch__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_else__branch__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("else\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), i_else__branch__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_149(mut in_txt: Tpl::Text, mut in_a_equ: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_equ.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("...")).clone() }))?;
            txt.clone()
        },
        (txt, i_equ, a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgorithmItems(txt.clone(), i_equ.clone(), a_context.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAlgorithm(mut in_txt: Tpl::Text, mut in_a_alg: Arc<Absyn::Algorithm>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_alg.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::Algorithm::ALG_ASSIGN { assignComponent: i_assignComponent, value: i_value }, a_context) => {
            let mut ret_3: bool = false;
            let mut l_rhs__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut ret_1: MMToJuliaUtil::Context = MMToJuliaUtil::Context::NO_CONTEXT;
            let mut l_lhs__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            ret_1 = MMToJuliaUtil::makeFunctionContext((literal!("listMatchAssign")).clone());
            l_lhs__str = dumpLhsExp(Tpl::emptyTxt.clone(), i_assignComponent.clone(), ret_1.clone())?;
            l_rhs__str = dumpExp(Tpl::emptyTxt.clone(), i_value.clone(), a_context.clone())?;
            ret_3 = AbsynUtil::complexIsCref(i_assignComponent.clone())?;
            txt = fun_146(txt.clone(), ret_3.clone(), i_assignComponent.clone(), l_rhs__str.clone(), l_lhs__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Algorithm::ALG_IF { ifExp: i_ifExp, trueBranch: i_trueBranch, elseIfAlgorithmBranch: i_elseIfAlgorithmBranch, elseBranch: i_elseBranch }, a_context) => {
            let mut l_else__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_else__branch__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_elseif__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_if__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_if__str = dumpAlgorithmBranch(Tpl::emptyTxt.clone(), i_ifExp.clone(), i_trueBranch.clone(), (literal!("if")).clone(), a_context.clone())?;
            l_elseif__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_elseif__str = lm_147(l_elseif__str.clone(), i_elseIfAlgorithmBranch.clone(), a_context.clone())?;
            l_elseif__str = Tpl::popIter(l_elseif__str.clone())?;
            l_else__branch__str = dumpAlgorithmItems(Tpl::emptyTxt.clone(), i_elseBranch.clone(), a_context.clone())?;
            l_else__str = fun_148(Tpl::emptyTxt.clone(), l_else__branch__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_if__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_elseif__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_else__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Algorithm::ALG_FOR { iterators: i_iterators, forBody: i_forBody }, a_context) => {
            let mut l_body__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_iter__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_iter__str = dumpForIterators(Tpl::emptyTxt.clone(), i_iterators.clone(), a_context.clone())?;
            l_body__str = dumpAlgorithmItems(Tpl::emptyTxt.clone(), i_forBody.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_iter__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Algorithm::ALG_WHILE { boolExpr: i_boolExpr, whileBody: i_whileBody }, a_context) => {
            let mut l_while__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_while__str = dumpAlgorithmBranch(Tpl::emptyTxt.clone(), i_boolExpr.clone(), i_whileBody.clone(), (literal!("while")).clone(), a_context.clone())?;
            txt = Tpl::writeText(txt.clone(), l_while__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Algorithm::ALG_WHEN_A { boolExpr: _, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("When statements are not allowed!.")).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall: i_functionCall, functionArgs: i_functionArgs }, a_context) => {
            let mut l_args__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_name__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_name__str = dumpCref(Tpl::emptyTxt.clone(), i_functionCall.clone(), a_context.clone())?;
            l_args__str = dumpFunctionArgs(Tpl::emptyTxt.clone(), i_functionArgs.clone(), a_context.clone())?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Algorithm::ALG_RETURN { .. }, a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgReturnString(txt.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Algorithm::ALG_BREAK { .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("break")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Algorithm::ALG_FAILURE { equ: i_equ }, a_context) => {
            let mut l_arg__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_arg__str = fun_149(Tpl::emptyTxt.clone(), i_equ.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("@shouldFail ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arg__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Algorithm::ALG_TRY { body: i_body, elseBody: i_elseBody }, a_context) => {
            let mut l_arg2: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_arg1: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_arg1 = dumpAlgorithmItems(Tpl::emptyTxt.clone(), i_body.clone(), a_context.clone())?;
            l_arg2 = dumpAlgorithmItems(Tpl::emptyTxt.clone(), i_elseBody.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("try\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_arg1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("catch\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_arg2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Algorithm::ALG_CONTINUE { .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("continue")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAlgReturnString(mut in_txt: Tpl::Text, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_context.clone()) {
        (mut txt, MMToJuliaUtil::Context::FUNCTION { retValsStr: mut i_retValsStr }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_retValsStr.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_152(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_152 in &*items.clone() {
        let mut lstElt_152 = lstElt_152.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_152.clone()) {
        i_eq => {
            txt = dumpAlgorithmItem(txt.clone(), i_eq.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpAlgorithmBranch(mut txt: Tpl::Text, mut a_cond: Arc<Absyn::Exp>, mut a_body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut a_header: ArcStr, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut l_body__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut l_cond__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    l_cond__str = dumpExp(Tpl::emptyTxt.clone(), a_cond.clone(), a_context.clone())?;
    l_body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    l_body__str = lm_152(l_body__str.clone(), a_body.clone(), a_context.clone())?;
    l_body__str = Tpl::popIter(l_body__str.clone())?;
    out_txt = Tpl::writeStr(txt.clone(), (a_header.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_cond__str.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_body__str.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    Ok(out_txt)
}

fn fun_154(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_path: Arc<Absyn::Path>, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_path.clone(), in_a_name.clone())) {
        (txt, false, a_path, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = AbsynDumpTpl::dumpPath(txt.clone(), a_path.clone())?;
            txt.clone()
        },
        (txt, _, a_path, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__")).clone() }))?;
            txt = AbsynDumpTpl::dumpPath(txt.clone(), a_path.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_155(mut in_txt: Tpl::Text, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_name.clone())) {
        (txt, Deref @ "Real") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Float")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "Integer") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Integer")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "Boolean") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Bool")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "list") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("List")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "array") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Array")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "tuple") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Tuple")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "polymorphic") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Any")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "Mutable") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MutableType")).clone() }))?;
            txt.clone()
        },
        (txt, i_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpPathJL(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::FULLYQUALIFIED { path: i_path }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = AbsynDumpTpl::dumpPath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::QUALIFIED { name: i_name, path: i_path }) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
            txt = fun_154(txt.clone(), ret_0.clone(), i_path.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: i_name }) => {
            let mut txt = (*txt).clone();
            txt = fun_155(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpPathJL: Unknown path.")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpPathNoQual(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::FULLYQUALIFIED { path: i_path }) => {
            let mut txt = (*txt).clone();
            txt = dumpPathJL(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, i_path) => {
            let mut txt = (*txt).clone();
            txt = dumpPathJL(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpTypeSpecOpt(mut in_txt: Tpl::Text, mut in_a_typespecOpt: Option<Arc<Absyn::TypeSpec>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_typespecOpt.clone(), in_a_context.clone())) {
        (txt, Some(i_ts), a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpTypeSpec(txt.clone(), i_ts.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_159(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::TypeSpec>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_159 in &*items.clone() {
        let mut lstElt_159 = lstElt_159.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_159.clone()) {
        i_ty => {
            txt = dumpTypeSpec(txt.clone(), i_ty.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_160(mut in_txt: Tpl::Text, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_context.clone()) {
        (mut txt, MMToJuliaUtil::Context::INPUT_CONTEXT { ty_str: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("iofunc")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_161(mut in_txt: Tpl::Text, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_context.clone()) {
        (mut txt, MMToJuliaUtil::Context::PACKAGE { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("package")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_162(mut in_txt: Tpl::Text, mut in_a_isPackage: Tpl::Text, mut in_a_arraydim__str: Tpl::Text, mut in_a_ty__str: Tpl::Text, mut in_a_path__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_isPackage.clone(), in_a_arraydim__str.clone(), in_a_ty__str.clone(), in_a_path__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_arraydim__str, a_ty__str, a_path__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_path__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_arraydim__str.clone())?;
            txt.clone()
        },
        (txt, _, _, _, a_path__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_path__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_163(mut in_txt: Tpl::Text, mut in_a_isFunc: Tpl::Text, mut in_a_res: Tpl::Text, mut in_a_arraydim__str: Tpl::Text, mut in_a_ty__str: Tpl::Text, mut in_a_path__str: Tpl::Text, mut in_a_isPackage: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_isFunc.clone(), in_a_res.clone(), in_a_arraydim__str.clone(), in_a_ty__str.clone(), in_a_path__str.clone(), in_a_isPackage.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _, a_arraydim__str, a_ty__str, a_path__str, a_isPackage) => {
            let mut txt = (*txt).clone();
            txt = fun_162(txt.clone(), a_isPackage.clone(), a_arraydim__str.clone(), a_ty__str.clone(), a_path__str.clone())?;
            txt.clone()
        },
        (txt, _, a_res, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpTypeSpec(mut in_txt: Tpl::Text, mut in_a_typeSpec: Arc<Absyn::TypeSpec>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_typeSpec.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::TypeSpec::TPATH { path: i_path, arrayDim: i_arrayDim }, a_context) => {
            let mut l_arraydim__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_path__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_path__str = dumpPathJL(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_arraydim__str = dumpArrayDimOpt(Tpl::emptyTxt.clone(), i_arrayDim.clone(), a_context.clone())?;
            txt = Tpl::writeText(txt.clone(), l_path__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_arraydim__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::TypeSpec::TCOMPLEX { path: i_path, typeSpecs: i_typeSpecs, arrayDim: i_arrayDim }, a_context) => {
            let mut l_res: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_ty__str2: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_isPackage: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_isFunc: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_ty__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_arraydim__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_path__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_path__str = dumpPathJL(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_ty__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_ty__str = lm_159(l_ty__str.clone(), i_typeSpecs.clone(), a_context.clone())?;
            l_ty__str = Tpl::popIter(l_ty__str.clone())?;
            l_arraydim__str = dumpArrayDimOpt(Tpl::emptyTxt.clone(), i_arrayDim.clone(), a_context.clone())?;
            l_isFunc = fun_160(Tpl::emptyTxt.clone(), a_context.clone())?;
            l_isPackage = fun_161(Tpl::emptyTxt.clone(), a_context.clone())?;
            l_ty__str2 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{<:")).clone() }))?;
            l_ty__str2 = Tpl::writeText(l_ty__str2.clone(), l_ty__str.clone())?;
            l_ty__str2 = Tpl::writeTok(l_ty__str2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            l_res = Tpl::writeText(Tpl::emptyTxt.clone(), l_path__str.clone())?;
            l_res = Tpl::writeText(l_res.clone(), l_ty__str2.clone())?;
            l_res = Tpl::writeText(l_res.clone(), l_arraydim__str.clone())?;
            txt = fun_163(txt.clone(), l_isFunc.clone(), l_res.clone(), l_arraydim__str.clone(), l_ty__str.clone(), l_path__str.clone(), l_isPackage.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpArrayDimOptTypeSpec(mut in_txt: Tpl::Text, mut in_a_arraydim: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_arraydim.clone(), in_a_context.clone())) {
        (txt, Some(i_ad), a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpSubscriptsTypeSpec(txt.clone(), i_ad.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_166(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_166 in &*items.clone() {
        let mut lstElt_166 = lstElt_166.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_166.clone()) {
        _ => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Array")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_167(mut in_txt: Tpl::Text, mut in_a_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscripts.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_subscripts) => {
            let mut l_sub__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_sub__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_sub__str = lm_166(l_sub__str.clone(), i_subscripts.clone())?;
            l_sub__str = Tpl::popIter(l_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Array{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpSubscriptsTypeSpec(mut txt: Tpl::Text, mut a_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = fun_167(txt.clone(), a_subscripts.clone())?;
    Ok(out_txt)
}

pub fn dumpArrayDimOpt(mut in_txt: Tpl::Text, mut in_a_arraydim: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_arraydim.clone(), in_a_context.clone())) {
        (txt, Some(i_ad), a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpSubscripts(txt.clone(), i_ad.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_170(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_170 in &*items.clone() {
        let mut lstElt_170 = lstElt_170.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_170.clone()) {
        i_s => {
            txt = dumpSubscript(txt.clone(), i_s.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpSubscripts(mut in_txt: Tpl::Text, mut in_a_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscripts.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, i_subscripts, a_context) => {
            let mut l_sub__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_sub__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_sub__str = lm_170(l_sub__str.clone(), i_subscripts.clone(), a_context.clone())?;
            l_sub__str = Tpl::popIter(l_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpSubscript(mut in_txt: Tpl::Text, mut in_a_subscript: Arc<Absyn::Subscript>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscript.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::Subscript::NOSUB { .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: i_subscript }, a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_subscript.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_173(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_173 in &*items.clone() {
        let mut lstElt_173 = lstElt_173.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_173.clone()) {
        i_na => {
            txt = dumpNamedArgPattern3(txt.clone(), i_na.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_174(mut in_txt: Tpl::Text, mut in_a_functionArgs: Arc<Absyn::FunctionArgs>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_functionArgs.clone())) {
        (txt, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: i_argNames, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_173(txt.clone(), i_argNames.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_175(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_175 in &*items.clone() {
        let mut lstElt_175 = lstElt_175.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_175.clone()) {
        i_e => {
            txt = dumpExp(txt.clone(), i_e.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_176(mut in_txt: Tpl::Text, mut in_a_array__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_array__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("nil")).clone() }))?;
            txt.clone()
        },
        (txt, i_array__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_array__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_177(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_177 in &*items.clone() {
        let mut lstElt_177 = lstElt_177.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_177.clone()) {
        i_e => {
            txt = dumpExp(txt.clone(), i_e.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_178(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_178 in &*items.clone() {
        let mut lstElt_178 = lstElt_178.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_178.clone()) {
        i_row => {
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_177(txt.clone(), i_row.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_179(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_179 in &*items.clone() {
        let mut lstElt_179 = lstElt_179.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_179.clone()) {
        i_e => {
            txt = dumpExp(txt.clone(), i_e.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_180(mut in_txt: Tpl::Text, mut in_a_tuple__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_tuple__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("()")).clone() }))?;
            txt.clone()
        },
        (txt, i_tuple__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_tuple__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_181(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_181 in &*items.clone() {
        let mut lstElt_181 = lstElt_181.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_181.clone()) {
        i_e => {
            txt = dumpExp(txt.clone(), i_e.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

// NOTE: tail-call loop lowering disabled — the body needs `match_deref!{…}`
// (string-literal / tuple-of-Arc patterns) which the loop's `.as_ref()` path can't decode.
pub fn dumpExp(mut in_txt: Tpl::Text, mut in_a_exp: Arc<Absyn::Exp>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::Exp::INTEGER { value: i_value }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_value.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::REAL { value: i_value_1 }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_value_1.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::CREF { componentRef: i_componentRef }, a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpCref(txt.clone(), i_componentRef.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::STRING { value: i_value_1 }, _) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToJLString((i_value_1.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::BOOL { value: i_value_2 }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_value_2.clone())).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ Absyn::Exp::BINARY { exp1: i_exp1, exp2: i_exp2, op: i_op }, a_context) => {
            let mut l_op__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_rhs__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_lhs__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_lhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp1.clone(), i_e.clone(), true, a_context.clone())?;
            l_rhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp2.clone(), i_e.clone(), false, a_context.clone())?;
            l_op__str = dumpOperator(Tpl::emptyTxt.clone(), i_op.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ Absyn::Exp::UNARY { exp: i_exp, op: i_op }, a_context) => {
            let mut l_exp__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_op__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_exp__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp.clone(), i_e.clone(), false, a_context.clone())?;
            l_op__str = dumpOperator(Tpl::emptyTxt.clone(), i_op.clone())?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ Absyn::Exp::LBINARY { exp1: i_exp1, exp2: i_exp2, op: i_op }, a_context) => {
            let mut l_op__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_rhs__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_lhs__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_lhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp1.clone(), i_e.clone(), true, a_context.clone())?;
            l_rhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp2.clone(), i_e.clone(), false, a_context.clone())?;
            l_op__str = dumpOperator(Tpl::emptyTxt.clone(), i_op.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ Absyn::Exp::LUNARY { exp: i_exp, op: i_op }, a_context) => {
            let mut l_exp__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_op__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_exp__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp.clone(), i_e.clone(), false, a_context.clone())?;
            l_op__str = dumpOperator(Tpl::emptyTxt.clone(), i_op.clone())?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ Absyn::Exp::RELATION { exp1: i_exp1, exp2: i_exp2, op: i_op }, a_context) => {
            let mut l_op__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_rhs__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_lhs__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_lhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp1.clone(), i_e.clone(), true, a_context.clone())?;
            l_rhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp2.clone(), i_e.clone(), false, a_context.clone())?;
            l_op__str = dumpOperator(Tpl::emptyTxt.clone(), i_op.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ Absyn::Exp::IFEXP { ifExp: _, .. }, a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpIfExp(txt.clone(), i_exp.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "$array", .. }, functionArgs: i_functionArgs, .. }, a_context) => {
            let mut l_args__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_args__str = dumpFunctionArgs(Tpl::emptyTxt.clone(), i_functionArgs.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::CALL { function_: i_function__, functionArgs: i_functionArgs, .. }, a_context) => {
            let mut l_func__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_args__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_func__str = dumpCref(Tpl::emptyTxt.clone(), i_function__.clone(), a_context.clone())?;
            l_args__str = dumpFunctionArgs(Tpl::emptyTxt.clone(), i_functionArgs.clone(), a_context.clone())?;
            txt = Tpl::writeText(txt.clone(), l_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: i_function__, functionArgs: i_functionArgs }, a_context) => {
            let mut l_args2__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_func__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_args__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_func__str = dumpCref(Tpl::emptyTxt.clone(), i_function__.clone(), a_context.clone())?;
            l_args__str = dumpFunctionArgs(Tpl::emptyTxt.clone(), i_functionArgs.clone(), a_context.clone())?;
            l_args2__str = fun_174(Tpl::emptyTxt.clone(), i_functionArgs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args2__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") -> ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::ARRAY { arrayExp: i_arrayExp }, a_context) => {
            let mut l_array__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_array__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_array__str = lm_175(l_array__str.clone(), i_arrayExp.clone(), a_context.clone())?;
            l_array__str = Tpl::popIter(l_array__str.clone())?;
            txt = fun_176(txt.clone(), l_array__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::MATRIX { matrix: i_matrix }, a_context) => {
            let mut l_matrix__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_matrix__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_matrix__str = lm_178(l_matrix__str.clone(), i_matrix.clone(), a_context.clone())?;
            l_matrix__str = Tpl::popIter(l_matrix__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_matrix__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ Absyn::Exp::RANGE { step: Some(i_step), start: i_start, stop: i_stop }, a_context) => {
            let mut l_stop__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_step__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_start__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_start__str = dumpOperand(Tpl::emptyTxt.clone(), i_start.clone(), i_e.clone(), false, a_context.clone())?;
            l_step__str = dumpOperand(Tpl::emptyTxt.clone(), i_step.clone(), i_e.clone(), false, a_context.clone())?;
            l_stop__str = dumpOperand(Tpl::emptyTxt.clone(), i_stop.clone(), i_e.clone(), false, a_context.clone())?;
            txt = Tpl::writeText(txt.clone(), l_start__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_step__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_stop__str.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ Absyn::Exp::RANGE { step: None, start: i_start, stop: i_stop }, a_context) => {
            let mut l_stop__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_start__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_start__str = dumpOperand(Tpl::emptyTxt.clone(), i_start.clone(), i_e.clone(), false, a_context.clone())?;
            l_stop__str = dumpOperand(Tpl::emptyTxt.clone(), i_stop.clone(), i_e.clone(), false, a_context.clone())?;
            txt = Tpl::writeText(txt.clone(), l_start__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_stop__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::TUPLE { expressions: i_expressions }, a_context) => {
            let mut l_tuple__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_tuple__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_tuple__str = lm_179(l_tuple__str.clone(), i_expressions.clone(), a_context.clone())?;
            l_tuple__str = Tpl::popIter(l_tuple__str.clone())?;
            txt = fun_180(txt.clone(), l_tuple__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::END { .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::CODE { code: i_code }, a_context) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$Code(")).clone() }))?;
            txt = dumpCodeNode(txt.clone(), i_code.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::AS { exp: i_exp, id: i_id }, a_context) => {
            let mut l_exp__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(@match ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::CONS { head: i_head, rest: i_rest }, a_context) => {
            let mut l_rest__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_head__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_head__str = dumpExp(Tpl::emptyTxt.clone(), i_head.clone(), a_context.clone())?;
            l_rest__str = dumpExp(Tpl::emptyTxt.clone(), i_rest.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_cons(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_head__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rest__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ Absyn::Exp::MATCHEXP { matchTy: _, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = dumpMatchExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::LIST { exps: i_exps }, a_context) => {
            let mut l_list__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_list__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_list__str = lm_181(l_list__str.clone(), i_exps.clone(), a_context.clone())?;
            l_list__str = Tpl::popIter(l_list__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_list__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Exp::DOT { exp: i_exp, index: i_index }, a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = dumpExp(txt.clone(), i_index.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* AbsynDumpTpl.dumpExp: UNHANDLED Abyn.Exp */")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_183(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut a_as__str: Tpl::Text, mut a_context: MMToJuliaUtil::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_as__str: Tpl::Text = a_as__str;
    for mut lstElt_183 in &*items.clone() {
        let mut lstElt_183 = lstElt_183.clone();
        (txt, a_as__str) = (::match_deref::match_deref! { match &(lstElt_183.clone()) {
        i_e => {
            (txt, a_as__str) = dumpPattern(txt.clone(), i_e.clone(), a_context.clone(), a_as__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" <| ")).clone() }))?;
            (txt.clone(), a_as__str.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_as__str))
}

fn lm_184(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut a_as__str: Tpl::Text, mut a_context: MMToJuliaUtil::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_as__str: Tpl::Text = a_as__str;
    for mut lstElt_184 in &*items.clone() {
        let mut lstElt_184 = lstElt_184.clone();
        (txt, a_as__str) = (::match_deref::match_deref! { match &(lstElt_184.clone()) {
        i_e => {
            (txt, a_as__str) = dumpPattern(txt.clone(), i_e.clone(), a_context.clone(), a_as__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" <| ")).clone() }))?;
            (txt.clone(), a_as__str.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_as__str))
}

fn lm_185(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut a_as__str: Tpl::Text, mut a_context: MMToJuliaUtil::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_as__str: Tpl::Text = a_as__str;
    for mut lstElt_185 in &*items.clone() {
        let mut lstElt_185 = lstElt_185.clone();
        (txt, a_as__str) = (::match_deref::match_deref! { match &(lstElt_185.clone()) {
        i_e => {
            (txt, a_as__str) = dumpPattern(txt.clone(), i_e.clone(), a_context.clone(), a_as__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" <| ")).clone() }))?;
            (txt.clone(), a_as__str.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_as__str))
}

fn lm_186(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut a_as__str: Tpl::Text, mut a_context: MMToJuliaUtil::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_as__str: Tpl::Text = a_as__str;
    for mut lstElt_186 in &*items.clone() {
        let mut lstElt_186 = lstElt_186.clone();
        (txt, a_as__str) = (::match_deref::match_deref! { match &(lstElt_186.clone()) {
        i_e => {
            (txt, a_as__str) = dumpPattern(txt.clone(), i_e.clone(), a_context.clone(), a_as__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" <| ")).clone() }))?;
            (txt.clone(), a_as__str.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_as__str))
}

fn fun_187(mut in_txt: Tpl::Text, mut in_a_id: ArcStr, mut in_a_function__: Arc<Absyn::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_id.clone(), in_a_function__.clone())) {
        (txt, Deref @ "list", _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("List")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_function__) => {
            let mut txt = (*txt).clone();
            txt = dumpCref(txt.clone(), a_function__.clone(), MMToJuliaUtil::functionContext.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_188(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Deref @ "NONE") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_189(mut in_txt: Tpl::Text, mut in_a_isNone: Tpl::Text, mut in_a_func__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_isNone.clone(), in_a_func__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_func__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(__)")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_func__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("()")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_190(mut in_txt: Tpl::Text, mut in_a_args__str: Tpl::Text, mut in_a_func__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args__str.clone(), in_a_func__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_func__str) => {
            let mut str_1: ArcStr = arcstr::literal!("");
            let mut l_isNone: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            str_1 = (Tpl::textString(a_func__str.clone())?).clone();
            l_isNone = fun_188(Tpl::emptyTxt.clone(), (str_1.clone()).clone())?;
            txt = fun_189(txt.clone(), l_isNone.clone(), a_func__str.clone())?;
            txt.clone()
        },
        (txt, i_args__str, a_func__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_191(mut in_txt: Tpl::Text, mut in_a_args__str: Tpl::Text, mut in_a_func__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args__str.clone(), in_a_func__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_func__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(__)")).clone() }))?;
            txt.clone()
        },
        (txt, i_args__str, a_func__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_192(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut a_as__str: Tpl::Text, mut a_context: MMToJuliaUtil::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut txt: Tpl::Text = txt;
    let mut a_as__str: Tpl::Text = a_as__str;
    for mut lstElt_192 in &*items.clone() {
        let mut lstElt_192 = lstElt_192.clone();
        (txt, a_as__str) = (::match_deref::match_deref! { match &(lstElt_192.clone()) {
        i_e => {
            (txt, a_as__str) = dumpPattern(txt.clone(), i_e.clone(), a_context.clone(), a_as__str.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt.clone(), a_as__str.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((txt, a_as__str))
}

pub fn dumpPattern(mut in_txt: Tpl::Text, mut in_a_exp: Arc<Absyn::Exp>, mut in_a_context: MMToJuliaUtil::Context, mut in_a_as__str: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut out_a_as__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    (out_txt, out_a_as__str) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_as__str.clone())) {
        (txt, Deref @ Absyn::Exp::INTEGER { value: i_value }, _, a_as__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_value.clone())).clone())?;
            (txt.clone(), a_as__str.clone())
        },
        (txt, Deref @ Absyn::Exp::REAL { value: i_value_1 }, _, a_as__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_value_1.clone()).clone())?;
            (txt.clone(), a_as__str.clone())
        },
        (txt, Deref @ Absyn::Exp::CREF { componentRef: i_componentRef }, _, a_as__str) => {
            let mut txt = (*txt).clone();
            txt = dumpCref(txt.clone(), i_componentRef.clone(), MMToJuliaUtil::functionContext.clone())?;
            (txt.clone(), a_as__str.clone())
        },
        (txt, Deref @ Absyn::Exp::STRING { value: i_value_1 }, _, a_as__str) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_ABS_INDENT { width: 0 }))?;
            ret_0 = (System::stringReplace((i_value_1.clone()).clone(), (literal!("\\$")).clone(), (literal!("\\$")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            (txt.clone(), a_as__str.clone())
        },
        (txt, Deref @ Absyn::Exp::BOOL { value: i_value_2 }, _, a_as__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_value_2.clone())).clone())?;
            (txt.clone(), a_as__str.clone())
        },
        (txt, Deref @ Absyn::Exp::ARRAY { arrayExp: i_exps }, a_context, a_as__str) => {
            let mut txt = (*txt).clone();
            let mut a_as__str = (*a_as__str).clone();
            (txt, a_as__str) = lm_183(txt.clone(), i_exps.clone(), a_as__str.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" nil()")).clone() }))?;
            (txt.clone(), a_as__str.clone())
        },
        (txt, Deref @ Absyn::Exp::LIST { exps: i_exps }, a_context, a_as__str) => {
            let mut txt = (*txt).clone();
            let mut a_as__str = (*a_as__str).clone();
            (txt, a_as__str) = lm_184(txt.clone(), i_exps.clone(), a_as__str.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" nil()")).clone() }))?;
            (txt.clone(), a_as__str.clone())
        },
        (txt, Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "list", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: i_exps, .. }, .. }, a_context, a_as__str) => {
            let mut txt = (*txt).clone();
            let mut a_as__str = (*a_as__str).clone();
            (txt, a_as__str) = lm_185(txt.clone(), i_exps.clone(), a_as__str.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" nil()")).clone() }))?;
            (txt.clone(), a_as__str.clone())
        },
        (txt, Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "$array", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: i_exps, .. }, .. }, a_context, a_as__str) => {
            let mut txt = (*txt).clone();
            let mut a_as__str = (*a_as__str).clone();
            (txt, a_as__str) = lm_186(txt.clone(), i_exps.clone(), a_as__str.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" nil()")).clone() }))?;
            (txt.clone(), a_as__str.clone())
        },
        (txt, Deref @ Absyn::Exp::CALL { function_: i_function__ @ Deref @ Absyn::ComponentRef::CREF_IDENT { name: i_id, .. }, functionArgs: i_functionArgs, .. }, _, a_as__str) => {
            let mut l_func__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_args__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_args__str = dumpFunctionArgsPattern(Tpl::emptyTxt.clone(), i_functionArgs.clone())?;
            l_func__str = fun_187(Tpl::emptyTxt.clone(), (i_id.clone()).clone(), i_function__.clone())?;
            txt = fun_190(txt.clone(), l_args__str.clone(), l_func__str.clone())?;
            (txt.clone(), a_as__str.clone())
        },
        (txt, Deref @ Absyn::Exp::CALL { function_: i_function__, functionArgs: i_functionArgs, .. }, _, a_as__str) => {
            let mut l_func__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_args__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_func__str = dumpCref(Tpl::emptyTxt.clone(), i_function__.clone(), MMToJuliaUtil::functionContext.clone())?;
            l_args__str = dumpFunctionArgsPattern(Tpl::emptyTxt.clone(), i_functionArgs.clone())?;
            txt = fun_191(txt.clone(), l_args__str.clone(), l_func__str.clone())?;
            (txt.clone(), a_as__str.clone())
        },
        (txt, Deref @ Absyn::Exp::TUPLE { expressions: i_expressions }, a_context, a_as__str) => {
            let mut l_tuple__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            let mut a_as__str = (*a_as__str).clone();
            l_tuple__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            (l_tuple__str, a_as__str) = lm_192(l_tuple__str.clone(), i_expressions.clone(), a_as__str.clone(), a_context.clone())?;
            l_tuple__str = Tpl::popIter(l_tuple__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_tuple__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_as__str.clone())
        },
        (txt, Deref @ Absyn::Exp::AS { exp: i_exp, id: i_id }, a_context, a_as__str) => {
            let mut l_id__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_exp__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            let mut a_as__str = (*a_as__str).clone();
            (l_exp__str, a_as__str) = dumpPattern(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_as__str.clone())?;
            l_id__str = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_id__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" && ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            (txt.clone(), a_as__str.clone())
        },
        (txt, Deref @ Absyn::Exp::CONS { head: i_head, rest: i_rest }, a_context, a_as__str) => {
            let mut txt_8: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt_7: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_consOp: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            let mut a_as__str = (*a_as__str).clone();
            (txt_7, a_as__str) = dumpPattern(Tpl::emptyTxt.clone(), i_head.clone(), a_context.clone(), a_as__str.clone())?;
            (txt_8, a_as__str) = dumpPattern(Tpl::emptyTxt.clone(), i_rest.clone(), a_context.clone(), a_as__str.clone())?;
            l_consOp = dumpCons(Tpl::emptyTxt.clone(), (Tpl::textString(txt_7.clone())?).clone(), (Tpl::textString(txt_8.clone())?).clone())?;
            txt = Tpl::writeText(txt.clone(), l_consOp.clone())?;
            (txt.clone(), a_as__str.clone())
        },
        (txt, _, _, a_as__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#= AbsynDumpTpl.dumpPattern: UNHANDLED Abyn.Exp  =#")).clone() }))?;
            (txt.clone(), a_as__str.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_as__str))
}

pub fn dumpCons(mut txt: Tpl::Text, mut a_headString: ArcStr, mut a_tailString: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = Tpl::writeStr(txt.clone(), (a_headString.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" <| ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_tailString.clone()).clone())?;
    Ok(out_txt)
}

fn lm_195(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_195 in &*items.clone() {
        let mut lstElt_195 = lstElt_195.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_195.clone()) {
        i_arg => {
            (txt, _) = dumpPattern(txt.clone(), i_arg.clone(), MMToJuliaUtil::functionContext.clone(), Tpl::emptyTxt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_196(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_196 in &*items.clone() {
        let mut lstElt_196 = lstElt_196.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_196.clone()) {
        i_narg => {
            txt = dumpNamedArgPattern(txt.clone(), i_narg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_197(mut in_txt: Tpl::Text, mut in_a_argNames: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_argNames.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_198(mut in_txt: Tpl::Text, mut in_a_args__str: Tpl::Text, mut in_a_argNames: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args__str.clone(), in_a_argNames.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, _, a_argNames) => {
            let mut txt = (*txt).clone();
            txt = fun_197(txt.clone(), a_argNames.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpFunctionArgsPattern(mut in_txt: Tpl::Text, mut in_a_args: Arc<Absyn::FunctionArgs>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args.clone())) {
        (txt, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: i_args, argNames: i_argNames }) => {
            let mut l_separator: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_namedargs__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_args__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_args__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_args__str = lm_195(l_args__str.clone(), i_args.clone())?;
            l_args__str = Tpl::popIter(l_args__str.clone())?;
            l_namedargs__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_namedargs__str = lm_196(l_namedargs__str.clone(), i_argNames.clone())?;
            l_namedargs__str = Tpl::popIter(l_namedargs__str.clone())?;
            l_separator = fun_198(Tpl::emptyTxt.clone(), l_args__str.clone(), i_argNames.clone())?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_separator.clone())?;
            txt = Tpl::writeText(txt.clone(), l_namedargs__str.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ERROR FOR_ITER_FARG in pattern")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpNamedArgPattern(mut in_txt: Tpl::Text, mut in_a_narg: Arc<Absyn::NamedArg>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_narg.clone())) {
        (txt, Deref @ Absyn::NamedArg { argName: i_argName, argValue: i_argValue }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_argName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            (txt, _) = dumpPattern(txt.clone(), i_argValue.clone(), MMToJuliaUtil::functionContext.clone(), Tpl::emptyTxt.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpNamedArgPattern2(mut in_txt: Tpl::Text, mut in_a_narg: Arc<Absyn::NamedArg>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_narg.clone())) {
        (txt, Deref @ Absyn::NamedArg { argName: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<%argName%>")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpNamedArgPattern3(mut in_txt: Tpl::Text, mut in_a_narg: Arc<Absyn::NamedArg>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_narg.clone())) {
        (txt, Deref @ Absyn::NamedArg { argValue: i_argValue, .. }) => {
            let mut txt = (*txt).clone();
            (txt, _) = dumpPattern(txt.clone(), i_argValue.clone(), MMToJuliaUtil::functionContext.clone(), Tpl::emptyTxt.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpLhsExp(mut in_txt: Tpl::Text, mut in_a_lhs: Arc<Absyn::Exp>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_lhs.clone(), in_a_context.clone())) {
        (txt, i_lhs @ Deref @ Absyn::Exp::IFEXP { ifExp: _, .. }, a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_lhs.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, i_lhs, a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_lhs.clone(), a_context.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_204(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_op__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_op__str.clone()) {
        (mut txt, false, mut a_op__str) => {
            txt = Tpl::writeText(txt.clone(), a_op__str.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_op__str) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpOperand(mut txt: Tpl::Text, mut a_operand: Arc<Absyn::Exp>, mut a_operation: Arc<Absyn::Exp>, mut a_lhs: bool, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut ret_1: bool = false;
    let mut l_op__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    l_op__str = dumpExp(Tpl::emptyTxt.clone(), a_operand.clone(), a_context.clone())?;
    ret_1 = Dump::shouldParenthesize(a_operand.clone(), a_operation.clone(), a_lhs.clone())?;
    out_txt = fun_204(txt.clone(), ret_1.clone(), l_op__str.clone())?;
    Ok(out_txt)
}

pub fn dumpIfExp(mut in_txt: Tpl::Text, mut in_a_if__exp: Arc<Absyn::Exp>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_if__exp.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::Exp::IFEXP { ifExp: i_ifExp, trueBranch: i_trueBranch, elseBranch: i_elseBranch, elseIfBranch: i_elseIfBranch }, a_context) => {
            let mut l_else__if__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_else__branch__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_true__branch__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_cond__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_cond__str = dumpExp(Tpl::emptyTxt.clone(), i_ifExp.clone(), a_context.clone())?;
            l_true__branch__str = dumpExp(Tpl::emptyTxt.clone(), i_trueBranch.clone(), a_context.clone())?;
            l_else__branch__str = dumpExp(Tpl::emptyTxt.clone(), i_elseBranch.clone(), a_context.clone())?;
            l_else__if__str = dumpElseIfExp(Tpl::emptyTxt.clone(), i_elseIfBranch.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeText(txt.clone(), l_true__branch__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_else__if__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("else\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_else__branch__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_207(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_207 in &*items.clone() {
        let mut lstElt_207 = lstElt_207.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_207.clone()) {
        (i_cond, i_branch) => {
            let mut l_branch__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_cond__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            l_cond__str = dumpExp(Tpl::emptyTxt.clone(), i_cond.clone(), a_context.clone())?;
            l_branch__str = dumpExp(Tpl::emptyTxt.clone(), i_branch.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("elseif (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(")\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeText(txt.clone(), l_branch__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpElseIfExp(mut txt: Tpl::Text, mut a_else__if: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_207(out_txt.clone(), a_else__if.clone(), a_context.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpCodeNode(mut in_txt: Tpl::Text, mut in_a_code: Arc<Absyn::CodeNode>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_code.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::CodeNode::C_TYPENAME { path: i_path }, _) => {
            let mut txt = (*txt).clone();
            txt = dumpPathJL(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: i_componentRef }, a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpCref(txt.clone(), i_componentRef.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CodeNode::C_CONSTRAINTSECTION { boolean: _, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpCodeNode: C_CONSTRAINTSECTION not supported")).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CodeNode::C_EQUATIONSECTION { boolean: _, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpCodeNode: C_CONSTRAINTSECTION not supported")).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CodeNode::C_ALGORITHMSECTION { boolean: _, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::errorMsg(txt.clone(), (literal!("AbsynToJulia.dumpCodeNode: C_ALGORITHMSECTION not supported")).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CodeNode::C_ELEMENT { element: i_element }, a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpElement(txt.clone(), i_element.clone(), Dump::defaultDumpOptions.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CodeNode::C_EXPRESSION { exp: i_exp }, a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CodeNode::C_MODIFICATION { modification: i_modification }, a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpModification(txt.clone(), i_modification.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_210(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::Case>>>, mut a_inputExp: Arc<Absyn::Exp>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_210 in &*items.clone() {
        let mut lstElt_210 = lstElt_210.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_210.clone()) {
        i_c => {
            let mut ret_0: MMToJuliaUtil::Context = MMToJuliaUtil::Context::NO_CONTEXT;
            ret_0 = MMToJuliaUtil::makeMatchContext(a_inputExp.clone());
            txt = dumpMatchCase(txt.clone(), i_c.clone(), ret_0.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpMatchExp(mut in_txt: Tpl::Text, mut in_a_match__exp: Arc<Absyn::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_match__exp.clone())) {
        (txt, Deref @ Absyn::Exp::MATCHEXP { matchTy: i_matchTy, inputExp: i_inputExp, localDecls: i_localDecls, cases: i_cases, comment: i_comment }) => {
            let mut l_cmt__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_cases__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_locals__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_input__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_match__ty__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_match__ty__str = dumpMatchType(Tpl::emptyTxt.clone(), i_matchTy.clone())?;
            l_input__str = dumpExp(Tpl::emptyTxt.clone(), i_inputExp.clone(), MMToJuliaUtil::functionContext.clone())?;
            l_locals__str = dumpMatchLocals(Tpl::emptyTxt.clone(), i_localDecls.clone())?;
            l_cases__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_cases__str = lm_210(l_cases__str.clone(), i_cases.clone(), i_inputExp.clone())?;
            l_cases__str = Tpl::popIter(l_cases__str.clone())?;
            l_cmt__str = dumpCommentStrOpt(Tpl::emptyTxt.clone(), i_comment.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("begin\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_locals__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_match__ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_input__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" begin\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_cases__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("end\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpMatchType(mut in_txt: Tpl::Text, mut in_a_match__type: Absyn::MatchType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_match__type.clone()) {
        (mut txt, Absyn::MatchType::MATCH { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("@match")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::MatchType::MATCHCONTINUE { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("@matchcontinue")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_213(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_213 in &*items.clone() {
        let mut lstElt_213 = lstElt_213.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_213.clone()) {
        i_alg => {
            txt = dumpAlgorithmItem(txt.clone(), i_alg.clone(), MMToJuliaUtil::functionContext.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_214(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_214 in &*items.clone() {
        let mut lstElt_214 = lstElt_214.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_214.clone()) {
        i_alg => {
            txt = dumpAlgorithmItem(txt.clone(), i_alg.clone(), MMToJuliaUtil::functionContext.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpMatchContents(mut in_txt: Tpl::Text, mut in_a_cp: Arc<Absyn::ClassPart>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cp.clone())) {
        (txt, Deref @ Absyn::ClassPart::EQUATIONS { contents: Deref @ metamodelica::List::Nil }) => {
            txt.clone()
        },
        (txt, i_cp @ Deref @ Absyn::ClassPart::EQUATIONS { contents: _ }) => {
            let mut ret_0: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_0 = Static::fromEquationsToAlgAssignments(i_cp.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_213(txt.clone(), ret_0.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ClassPart::ALGORITHMS { contents: Deref @ metamodelica::List::Nil }) => {
            txt.clone()
        },
        (txt, Deref @ Absyn::ClassPart::ALGORITHMS { contents: i_algs }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_214(txt.clone(), i_algs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_216(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_216 in &*items.clone() {
        let mut lstElt_216 = lstElt_216.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_216.clone()) {
        i_decl => {
            txt = dumpElementItem(txt.clone(), i_decl.clone(), Dump::defaultDumpOptions.clone(), MMToJuliaUtil::functionContext.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpMatchLocals(mut in_txt: Tpl::Text, mut in_a_locals: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_locals.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_locals) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_216(txt.clone(), i_locals.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_218(mut in_txt: Tpl::Text, mut in_a_patternGuard: Option<Arc<Absyn::Exp>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_patternGuard.clone(), in_a_context.clone())) {
        (txt, Some(i_g), a_context) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("where (")).clone() }))?;
            txt = dumpExp(txt.clone(), i_g.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") ")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_219(mut in_txt: Tpl::Text, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_context.clone()) {
        (mut txt, ref i_context @ MMToJuliaUtil::Context::MATCH_CONTEXT { inputExp: ref i_inputExp }) => {
            txt = dumpExp(txt.clone(), i_inputExp.clone(), i_context.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_220(mut in_txt: Tpl::Text, mut in_a_as__str: Tpl::Text, mut in_a_result__str: Tpl::Text, mut in_a_eql__str: Tpl::Text, mut in_a_cmt__str: Tpl::Text, mut in_a_guard__str: Tpl::Text, mut in_a_pattern__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_as__str.clone(), in_a_result__str.clone(), in_a_eql__str.clone(), in_a_cmt__str.clone(), in_a_guard__str.clone(), in_a_pattern__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_result__str, a_eql__str, a_cmt__str, a_guard__str, a_pattern__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_pattern__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_guard__str.clone())?;
            txt = Tpl::writeText(txt.clone(), a_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" => begin\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_eql__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_result__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end")).clone() }))?;
            txt.clone()
        },
        (txt, i_as__str, a_result__str, a_eql__str, a_cmt__str, a_guard__str, a_pattern__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_pattern__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_guard__str.clone())?;
            txt = Tpl::writeText(txt.clone(), a_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" => begin\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), i_as__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_eql__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_result__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpMatchCase(mut in_txt: Tpl::Text, mut in_a_c: Arc<Absyn::Case>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_c.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::Case::CASE { pattern: i_pattern, patternGuard: i_patternGuard, classPart: i_classPart, result: i_result, comment: i_comment, .. }, a_context) => {
            let mut l_input__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_cmt__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_result__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_eql__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_guard__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_pattern__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_as__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_as__str = Tpl::emptyTxt.clone();
            (l_pattern__str, l_as__str) = dumpPattern(Tpl::emptyTxt.clone(), i_pattern.clone(), a_context.clone(), l_as__str.clone())?;
            l_guard__str = fun_218(Tpl::emptyTxt.clone(), i_patternGuard.clone(), a_context.clone())?;
            l_eql__str = dumpMatchContents(Tpl::emptyTxt.clone(), i_classPart.clone())?;
            l_result__str = dumpExp(Tpl::emptyTxt.clone(), i_result.clone(), a_context.clone())?;
            l_cmt__str = dumpCommentStrOpt(Tpl::emptyTxt.clone(), i_comment.clone())?;
            l_input__str = fun_219(Tpl::emptyTxt.clone(), a_context.clone())?;
            txt = fun_220(txt.clone(), l_as__str.clone(), l_result__str.clone(), l_eql__str.clone(), l_cmt__str.clone(), l_guard__str.clone(), l_pattern__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Case::ELSE { classPart: i_classPart, result: i_result, comment: i_comment, .. }, a_context) => {
            let mut l_cmt__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_result__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_eql__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_eql__str = dumpMatchContents(Tpl::emptyTxt.clone(), i_classPart.clone())?;
            l_result__str = dumpExp(Tpl::emptyTxt.clone(), i_result.clone(), a_context.clone())?;
            l_cmt__str = dumpCommentStrOpt(Tpl::emptyTxt.clone(), i_comment.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_ ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" => begin\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_eql__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_result__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpOperator(mut in_txt: Tpl::Text, mut in_a_op: Absyn::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_op.clone()) {
        (mut txt, Absyn::Operator::AND { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("&&")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::OR { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("||")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::NOT { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("!")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::NEQUAL { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("!=")).clone() }))?;
            txt.clone()
        },
        (mut txt, mut i_op) => {
            txt = AbsynDumpTpl::dumpOperator(txt.clone(), i_op.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_223(mut in_txt: Tpl::Text, mut in_a_name: ArcStr, mut in_a_c__str: Tpl::Text, mut in_a_ss__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_name.clone(), in_a_c__str.clone(), in_a_ss__str.clone())) {
        (txt, Deref @ "List", a_c__str, a_ss__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ListUtil")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_ss__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_c__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ "Array", a_c__str, a_ss__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ArrayUtil")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_ss__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_c__str.clone())?;
            txt.clone()
        },
        (txt, i_name, a_c__str, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_c__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_224(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: tail-call loop lowering disabled — the body needs `match_deref!{…}`
// (string-literal / tuple-of-Arc patterns) which the loop's `.as_ref()` path can't decode.
pub fn dumpCref(mut in_txt: Tpl::Text, mut in_a_cref: Arc<Absyn::ComponentRef>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cref.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::ComponentRef::CREF_QUAL { subscripts: i_subscripts, componentRef: i_componentRef, name: i_name }, a_context) => {
            let mut l_c__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_ss__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_ss__str = dumpSubscripts(Tpl::emptyTxt.clone(), i_subscripts.clone(), a_context.clone())?;
            l_c__str = dumpCref(Tpl::emptyTxt.clone(), i_componentRef.clone(), a_context.clone())?;
            txt = fun_223(txt.clone(), (i_name.clone()).clone(), l_c__str.clone(), l_ss__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ComponentRef::CREF_IDENT { name: i_name, subscripts: i_subscripts }, a_context) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = dumpSubscripts(txt.clone(), i_subscripts.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: i_componentRef }, a_context) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = dumpCref(txt.clone(), i_componentRef.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ComponentRef::WILD { .. }, _) => {
            let mut ret_2: bool = false;
            let mut txt = (*txt).clone();
            ret_2 = Config::acceptMetaModelicaGrammar()?;
            txt = fun_224(txt.clone(), ret_2.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ComponentRef::ALLWILD { .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_226(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_226 in &*items.clone() {
        let mut lstElt_226 = lstElt_226.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_226.clone()) {
        i_arg => {
            txt = dumpExp(txt.clone(), i_arg.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_227(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_227 in &*items.clone() {
        let mut lstElt_227 = lstElt_227.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_227.clone()) {
        i_narg => {
            txt = dumpNamedArg(txt.clone(), i_narg.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_228(mut in_txt: Tpl::Text, mut in_a_argNames: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_argNames.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_229(mut in_txt: Tpl::Text, mut in_a_args__str: Tpl::Text, mut in_a_argNames: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args__str.clone(), in_a_argNames.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, _, a_argNames) => {
            let mut txt = (*txt).clone();
            txt = fun_228(txt.clone(), a_argNames.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_230(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_230 in &*items.clone() {
        let mut lstElt_230 = lstElt_230.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_230.clone()) {
        i_i => {
            txt = dumpForIterator(txt.clone(), i_i.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_231(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_231 in &*items.clone() {
        let mut lstElt_231 = lstElt_231.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_231.clone()) {
        i_i => {
            txt = dumpForIteratorName(txt.clone(), i_i.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_232(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_232 in &*items.clone() {
        let mut lstElt_232 = lstElt_232.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_232.clone()) {
        i_i => {
            txt = dumpForIteratorRanges(txt.clone(), i_i.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_233(mut in_txt: Tpl::Text, mut in_a_iterType: Absyn::ReductionIterType, mut in_a_iter__str: Tpl::Text, mut in_a_iter__ranges: Tpl::Text, mut in_a_iter__names: Tpl::Text, mut in_a_exp__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (match (in_txt.clone(), in_a_iterType.clone(), in_a_iter__str.clone(), in_a_iter__ranges.clone(), in_a_iter__names.clone(), in_a_exp__str.clone()) {
        (mut txt, Absyn::ReductionIterType::THREAD { .. }, _, mut a_iter__ranges, mut a_iter__names, mut a_exp__str) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("@do_threaded_for ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_iter__names.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_iter__ranges.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_iter__str, _, _, mut a_exp__str) => {
            txt = Tpl::writeText(txt.clone(), a_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" for ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_iter__str.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpFunctionArgs(mut in_txt: Tpl::Text, mut in_a_args: Arc<Absyn::FunctionArgs>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: i_args, argNames: i_argNames }, a_context) => {
            let mut l_separator: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_namedargs__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_args__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_args__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_args__str = lm_226(l_args__str.clone(), i_args.clone(), a_context.clone())?;
            l_args__str = Tpl::popIter(l_args__str.clone())?;
            l_namedargs__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_namedargs__str = lm_227(l_namedargs__str.clone(), i_argNames.clone(), a_context.clone())?;
            l_namedargs__str = Tpl::popIter(l_namedargs__str.clone())?;
            l_separator = fun_229(Tpl::emptyTxt.clone(), l_args__str.clone(), i_argNames.clone())?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_separator.clone())?;
            txt = Tpl::writeText(txt.clone(), l_namedargs__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { exp: i_exp, iterators: i_iterators, iterType: i_iterType }, a_context) => {
            let mut l_iter__ranges: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_iter__names: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_iter__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_exp__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone())?;
            l_iter__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_iter__str = lm_230(l_iter__str.clone(), i_iterators.clone(), a_context.clone())?;
            l_iter__str = Tpl::popIter(l_iter__str.clone())?;
            l_iter__names = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_iter__names = lm_231(l_iter__names.clone(), i_iterators.clone(), a_context.clone())?;
            l_iter__names = Tpl::popIter(l_iter__names.clone())?;
            l_iter__ranges = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_iter__ranges = lm_232(l_iter__ranges.clone(), i_iterators.clone(), a_context.clone())?;
            l_iter__ranges = Tpl::popIter(l_iter__ranges.clone())?;
            txt = fun_233(txt.clone(), i_iterType.clone(), l_iter__str.clone(), l_iter__ranges.clone(), l_iter__names.clone(), l_exp__str.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpNamedArg(mut in_txt: Tpl::Text, mut in_a_narg: Arc<Absyn::NamedArg>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_narg.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::NamedArg { argName: i_argName, argValue: i_argValue }, a_context) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_argName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_argValue.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_236(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_236 in &*items.clone() {
        let mut lstElt_236 = lstElt_236.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_236.clone()) {
        i_i => {
            txt = dumpForIterator(txt.clone(), i_i.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpForIterators(mut txt: Tpl::Text, mut a_iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_236(out_txt.clone(), a_iters.clone(), a_context.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

fn fun_238(mut in_txt: Tpl::Text, mut in_a_range: Option<Arc<Absyn::Exp>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_range.clone(), in_a_context.clone())) {
        (txt, Some(i_r), a_context) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("in ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_r.clone(), a_context.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_239(mut in_txt: Tpl::Text, mut in_a_guardExp: Option<Arc<Absyn::Exp>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_guardExp.clone(), in_a_context.clone())) {
        (txt, Some(i_g), a_context) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_g.clone(), a_context.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpForIterator(mut in_txt: Tpl::Text, mut in_a_iterator: Arc<Absyn::ForIterator>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iterator.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::ForIterator { range: i_range, guardExp: i_guardExp, name: i_name }, a_context) => {
            let mut l_guard__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_range__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_range__str = fun_238(Tpl::emptyTxt.clone(), i_range.clone(), a_context.clone())?;
            l_guard__str = fun_239(Tpl::emptyTxt.clone(), i_guardExp.clone(), a_context.clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_range__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_guard__str.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_241(mut in_txt: Tpl::Text, mut in_a_range: Option<Arc<Absyn::Exp>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_range.clone(), in_a_context.clone())) {
        (txt, Some(i_r), a_context) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_r.clone(), a_context.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_242(mut in_txt: Tpl::Text, mut in_a_guardExp: Option<Arc<Absyn::Exp>>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_guardExp.clone(), in_a_context.clone())) {
        (txt, Some(i_g), a_context) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_g.clone(), a_context.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpForIteratorRanges(mut in_txt: Tpl::Text, mut in_a_iterator: Arc<Absyn::ForIterator>, mut in_a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iterator.clone(), in_a_context.clone())) {
        (txt, Deref @ Absyn::ForIterator { range: i_range, guardExp: i_guardExp, .. }, a_context) => {
            let mut l_guard__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut l_range__str: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            l_range__str = fun_241(Tpl::emptyTxt.clone(), i_range.clone(), a_context.clone())?;
            l_guard__str = fun_242(Tpl::emptyTxt.clone(), i_guardExp.clone(), a_context.clone())?;
            txt = Tpl::writeText(txt.clone(), l_range__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_guard__str.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_244(mut in_txt: Tpl::Text, mut in_a_iterator: Arc<Absyn::ForIterator>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iterator.clone())) {
        (txt, Deref @ Absyn::ForIterator { name: i_name, .. }) => {
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

pub fn dumpForIteratorName(mut txt: Tpl::Text, mut a_iterator: Arc<Absyn::ForIterator>, mut a_context: MMToJuliaUtil::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    out_txt = fun_244(txt.clone(), a_iterator.clone())?;
    Ok(out_txt)
}

fn lm_246(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_246 in &*items.clone() {
        let mut lstElt_246 = lstElt_246.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_246.clone()) {
        i_e => {
            let mut ret_0: Option<Arc<Absyn::TypeSpec>> = None;
            ret_0 = AbsynUtil::getTypeSpecFromElementItemOpt(i_e.clone());
            txt = dumpTypeSpecOpt(txt.clone(), ret_0.clone(), MMToJuliaUtil::functionContext.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpOutputsJL(mut txt: Tpl::Text, mut a_elements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    let mut ret_1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut l_outputStr: Tpl::Text = <Tpl::Text as ::std::default::Default>::default();
    ret_1 = a_elements.clone().reverse();
    l_outputStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    l_outputStr = lm_246(l_outputStr.clone(), ret_1.clone())?;
    l_outputStr = Tpl::popIter(l_outputStr.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_outputStr.clone())?;
    Ok(out_txt)
}

