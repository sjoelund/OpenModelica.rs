// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::AbsynDumpTpl;
use crate::ClassInfUtil;
use crate::DAEDumpTpl;
use crate::Dump;
use crate::ExpressionBasics;
use crate::TypesDump;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_tpl::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Flags;
use openmodelica_util::System;

fn fun_13(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_index: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_index) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_index) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_14(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_ty)) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_ty) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/*")).clone() }))?;
            ret_0 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/ ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_15(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_attr_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_attr_ty)) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_attr_ty) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/*")).clone() }))?;
            ret_0 = (TypesDump::unparseType(a_attr_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/ ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_16(mut in_txt: Tpl::Text, mut in_a_scalar: bool, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_scalar, in_a_ty)) {
        (txt, false, a_ty) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* non-scalar ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */ ")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_ty) => {
            let mut ret_1: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* scalar ")).clone() }))?;
            ret_1 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_17(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty: Arc<DAE::Type>, mut in_a_scalar: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_ty, in_a_scalar)) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_ty, a_scalar) => {
            let mut txt = (*txt).clone();
            txt = fun_16(txt.clone(), a_scalar.clone(), a_ty.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_18(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty: Arc<DAE::Type>, mut in_a_scalar: bool, mut in_a_stringDelimiter: ArcStr, mut in_a_array: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_ty, in_a_scalar, in_a_stringDelimiter, in_a_array)) {
        (txt, false, a_ty, a_scalar, a_stringDelimiter, a_array) => {
            let mut ret_1: bool;
            let mut l_expl: Tpl::Text;
            let mut txt = (*txt).clone();
            l_expl = dumpExpList(Tpl::emptyTxt.clone(), a_array.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            ret_1 = Config::typeinfo()?;
            txt = fun_17(txt.clone(), ret_1, a_ty.clone(), a_scalar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_expl)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fill(0,0)")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_19(mut in_txt: Tpl::Text, mut in_a_scalar: bool, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_scalar, in_a_ty)) {
        (txt, false, a_ty) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* non-scalar ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */ ")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_ty) => {
            let mut ret_1: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* scalar ")).clone() }))?;
            ret_1 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_20(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty: Arc<DAE::Type>, mut in_a_scalar: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_ty, in_a_scalar)) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_ty, a_scalar) => {
            let mut txt = (*txt).clone();
            txt = fun_19(txt.clone(), a_scalar.clone(), a_ty.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_21(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_21 in &*items {
        let mut lstElt_21 = lstElt_21.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_21.clone()) {
        i_row => {
            txt = dumpExpList(txt.clone(), i_row.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_22(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_ty)) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_ty) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* matrix ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */ ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_23(mut in_txt: Tpl::Text, mut in_a_step: Option<Arc<DAE::Exp>>, mut in_a_e: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_step, in_a_e)) {
        (txt, Some(i_step), a_e) => {
            let mut txt = (*txt).clone();
            txt = dumpOperand(txt.clone(), i_step.clone(), a_e.clone(), false)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_24(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_needs__paren)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_25(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_needs__paren)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_26(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/*ASUB*/")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_27(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_needs__paren)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_28(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_needs__paren)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_29(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_needs__paren)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_30(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_needs__paren)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_31(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_ty)) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_ty) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/*RSUB: ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_32(mut in_txt: Tpl::Text, mut in_a_sz: Option<Arc<DAE::Exp>>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_sz, in_a_stringDelimiter)) {
        (txt, Some(i_dim), a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_dim.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_33(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, mut a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_33 in &*items {
        let mut lstElt_33 = lstElt_33.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_33.clone()) {
        i_it => {
            txt = dumpReductionIterator(txt.clone(), i_it.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_34(mut in_txt: Tpl::Text, mut in_a_ri_iterType: Absyn::ReductionIterType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_ri_iterType) {
        (mut txt, Absyn::ReductionIterType::THREAD { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threaded ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_35(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::MatchCase>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_35 in &*items {
        let mut lstElt_35 = lstElt_35.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_35.clone()) {
        i_c => {
            txt = dumpMatchCase(txt.clone(), i_c.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_36(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_index: i32, mut in_a_stringDelimiter: ArcStr, mut in_a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_index, in_a_stringDelimiter, in_a_exp)) {
        (txt, false, _, a_stringDelimiter, a_exp) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), a_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        (txt, _, a_index, a_stringDelimiter, a_exp) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* Shared literal ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */ ")).clone() }))?;
            txt = dumpExp(txt.clone(), a_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_37(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/*pattern*/")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpExp(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_exp, in_a_stringDelimiter)) {
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
        (txt, Deref @ DAE::Exp::SCONST { string: i_string }, a_stringDelimiter) => {
            let mut ret_1: ArcStr;
            let mut l_str: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = (System::escapedString((i_string.clone()).clone(), false)).clone();
            l_str = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1).clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_str)?;
            txt = Tpl::writeStr(txt.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: i_bool }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_bool.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CLKCONST { clk: i_clk }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = dumpClockKind(txt.clone(), i_clk.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { index: i_index, name: i_name }, _) => {
            let mut ret_2: bool;
            let mut txt = (*txt).clone();
            ret_2 = Config::typeinfo()?;
            txt = fun_13(txt.clone(), ret_2, i_index.clone())?;
            txt = AbsynDumpTpl::dumpPath(txt.clone(), i_name.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CREF { ty: i_ty, componentRef: i_componentRef }, _) => {
            let mut ret_3: bool;
            let mut txt = (*txt).clone();
            ret_3 = Config::typeinfo()?;
            txt = fun_14(txt.clone(), ret_3, i_ty.clone())?;
            txt = dumpCref(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::BINARY { exp1: i_exp1, exp2: i_exp2, operator: i_operator }, _) => {
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp1.clone(), i_e.clone(), true)?;
            l_rhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp2.clone(), i_e.clone(), false)?;
            l_op__str = dumpBinOp(Tpl::emptyTxt.clone(), i_operator.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_op__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str)?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::UNARY { exp: i_exp, operator: i_operator }, _) => {
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp.clone(), i_e.clone(), false)?;
            l_op__str = dumpUnaryOp(Tpl::emptyTxt.clone(), i_operator.clone())?;
            txt = Tpl::writeText(txt.clone(), l_op__str)?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::LBINARY { exp1: i_exp1, exp2: i_exp2, operator: i_operator }, _) => {
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp1.clone(), i_e.clone(), true)?;
            l_rhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp2.clone(), i_e.clone(), false)?;
            l_op__str = dumpLogicalBinOp(Tpl::emptyTxt.clone(), i_operator.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_op__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str)?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::LUNARY { exp: i_exp, operator: i_operator }, _) => {
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp.clone(), i_e.clone(), false)?;
            l_op__str = dumpLogicalUnaryOp(Tpl::emptyTxt.clone(), i_operator.clone())?;
            txt = Tpl::writeText(txt.clone(), l_op__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::RELATION { exp1: i_exp1, exp2: i_exp2, operator: i_operator, .. }, _) => {
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp1.clone(), i_e.clone(), true)?;
            l_rhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp2.clone(), i_e.clone(), false)?;
            l_op__str = dumpRelationOp(Tpl::emptyTxt.clone(), i_operator.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_op__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str)?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::IFEXP { expCond: i_expCond, expThen: i_expThen, expElse: i_expElse }, a_stringDelimiter) => {
            let mut l_else__str: Tpl::Text;
            let mut l_then__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cond__str = dumpExp(Tpl::emptyTxt.clone(), i_expCond.clone(), (a_stringDelimiter.clone()).clone())?;
            l_then__str = dumpExp(Tpl::emptyTxt.clone(), i_expThen.clone(), (a_stringDelimiter.clone()).clone())?;
            l_else__str = dumpExp(Tpl::emptyTxt.clone(), i_expElse.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" then ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_then__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" else ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_else__str)?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: true, ty: i_attr_ty, .. }, path: i_path, expLst: i_expLst }, a_stringDelimiter) => {
            let mut ret_13: bool;
            let mut l_argl: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_func__str = AbsynDumpTpl::dumpPathNoQual(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_argl = dumpExpList(Tpl::emptyTxt.clone(), i_expLst.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            ret_13 = Config::typeinfo()?;
            txt = fun_15(txt.clone(), ret_13, i_attr_ty.clone())?;
            txt = Tpl::writeText(txt.clone(), l_func__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_argl)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: i_path, expLst: i_expLst, .. }, a_stringDelimiter) => {
            let mut l_argl: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_func__str = AbsynDumpTpl::dumpPathNoQual(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_argl = dumpExpList(Tpl::emptyTxt.clone(), i_expLst.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeText(txt.clone(), l_func__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_argl)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RECORD { path: i_path, exps: i_exps, .. }, a_stringDelimiter) => {
            let mut l_argl: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_func__str = AbsynDumpTpl::dumpPathNoQual(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_argl = dumpExpList(Tpl::emptyTxt.clone(), i_exps.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeText(txt.clone(), l_func__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_argl)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::PARTEVALFUNCTION { path: i_path, expList: i_expList, .. }, a_stringDelimiter) => {
            let mut l_argl: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_func__str = AbsynDumpTpl::dumpPathNoQual(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_argl = dumpExpList(Tpl::emptyTxt.clone(), i_expList.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_func__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_argl)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ARRAY { array: i_array @ Deref @ metamodelica::List::Nil, scalar: i_scalar, ty: i_ty }, a_stringDelimiter) => {
            let mut ret_14: bool;
            let mut txt = (*txt).clone();
            ret_14 = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
            txt = fun_18(txt.clone(), ret_14, i_ty.clone(), i_scalar.clone(), (a_stringDelimiter.clone()).clone(), i_array.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ARRAY { array: i_array, scalar: i_scalar, ty: i_ty }, a_stringDelimiter) => {
            let mut ret_16: bool;
            let mut l_expl: Tpl::Text;
            let mut txt = (*txt).clone();
            l_expl = dumpExpList(Tpl::emptyTxt.clone(), i_array.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            ret_16 = Config::typeinfo()?;
            txt = fun_20(txt.clone(), ret_16, i_ty.clone(), i_scalar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_expl)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::MATRIX { matrix: i_matrix, ty: i_ty, .. }, a_stringDelimiter) => {
            let mut ret_18: bool;
            let mut l_mat__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_mat__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_mat__str = lm_21(l_mat__str, i_matrix.clone(), (a_stringDelimiter.clone()).clone())?;
            l_mat__str = Tpl::popIter(l_mat__str)?;
            ret_18 = Config::typeinfo()?;
            txt = fun_22(txt.clone(), ret_18, i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mat__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}}")).clone() }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::RANGE { start: i_start, step: i_step, stop: i_stop, .. }, _) => {
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_start__str = dumpOperand(Tpl::emptyTxt.clone(), i_start.clone(), i_e.clone(), false)?;
            l_step__str = fun_23(Tpl::emptyTxt.clone(), i_step.clone(), i_e.clone())?;
            l_stop__str = dumpOperand(Tpl::emptyTxt.clone(), i_stop.clone(), i_e.clone(), false)?;
            txt = Tpl::writeText(txt.clone(), l_start__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_step__str)?;
            txt = Tpl::writeText(txt.clone(), l_stop__str)?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::TUPLE { PR: i_PR }, a_stringDelimiter) => {
            let mut l_tuple__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_tuple__str = dumpExpList(Tpl::emptyTxt.clone(), i_PR.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_tuple__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CAST { exp: i_exp, ty: i_ty }, a_stringDelimiter) => {
            let mut l_ty__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            l_ty__str = dumpType(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/*")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ASUB { exp: i_exp, sub: i_sub }, a_stringDelimiter) => {
            let mut ret_28: bool;
            let mut l_sub__str: Tpl::Text;
            let mut l_rparen: Tpl::Text;
            let mut l_lparen: Tpl::Text;
            let mut l_needs__paren: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_needs__paren = parenthesizeSubExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_lparen = fun_24(Tpl::emptyTxt.clone(), l_needs__paren.clone())?;
            l_rparen = fun_25(Tpl::emptyTxt.clone(), l_needs__paren)?;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            l_sub__str = dumpSubscripts(Tpl::emptyTxt.clone(), i_sub.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lparen)?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt = Tpl::writeText(txt.clone(), l_rparen)?;
            ret_28 = Config::typeinfo()?;
            txt = fun_26(txt.clone(), ret_28)?;
            txt = Tpl::writeText(txt.clone(), l_sub__str)?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::TSUB { exp: i_exp, ix: i_ix, .. }, a_stringDelimiter) => {
            let mut l_rparen: Tpl::Text;
            let mut l_lparen: Tpl::Text;
            let mut l_needs__paren: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_needs__paren = parenthesizeSubExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_lparen = fun_27(Tpl::emptyTxt.clone(), l_needs__paren.clone())?;
            l_rparen = fun_28(Tpl::emptyTxt.clone(), l_needs__paren)?;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_lparen)?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt = Tpl::writeText(txt.clone(), l_rparen)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_ix.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RSUB { exp: i_exp, ty: i_ty, fieldName: i_fieldName, .. }, a_stringDelimiter) => {
            let mut ret_29: bool;
            let mut l_rparen: Tpl::Text;
            let mut l_lparen: Tpl::Text;
            let mut l_needs__paren: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_needs__paren = parenthesizeSubExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_lparen = fun_29(Tpl::emptyTxt.clone(), l_needs__paren.clone())?;
            l_rparen = fun_30(Tpl::emptyTxt.clone(), l_needs__paren)?;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            ret_29 = Config::typeinfo()?;
            txt = fun_31(txt.clone(), ret_29, i_ty.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lparen)?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt = Tpl::writeText(txt.clone(), l_rparen)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fieldName.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SIZE { exp: i_exp, sz: i_sz }, a_stringDelimiter) => {
            let mut l_dim__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            l_dim__str = fun_32(Tpl::emptyTxt.clone(), i_sz.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("size(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt = Tpl::writeText(txt.clone(), l_dim__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CODE { code: i_code, .. }, _) => {
            let mut ret_32: ArcStr;
            let mut l_code__str: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_32 = (Dump::printCodeStr(i_code.clone())?).clone();
            l_code__str = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_32).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$Code(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_code__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::EMPTY { name: i_name_1, scope: i_scope, tyStr: i_tyStr, .. }, _) => {
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = dumpCref(Tpl::emptyTxt.clone(), i_name_1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<EMPTY(scope: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_scope.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", name: ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ty: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_tyStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: i_ri_path, iterType: i_ri_iterType, .. }, expr: i_expr, iterators: i_iterators }, a_stringDelimiter) => {
            let mut l_iter__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPathNoQual(Tpl::emptyTxt.clone(), i_ri_path.clone())?;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_expr.clone(), (a_stringDelimiter.clone()).clone())?;
            l_iter__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_iter__str = lm_33(l_iter__str, i_iterators.clone(), (a_stringDelimiter.clone()).clone())?;
            l_iter__str = Tpl::popIter(l_iter__str)?;
            txt = Tpl::writeText(txt.clone(), l_name__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" for ")).clone() }))?;
            txt = fun_34(txt.clone(), i_ri_iterType.clone())?;
            txt = Tpl::writeText(txt.clone(), l_iter__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::LIST { valList: i_valList }, a_stringDelimiter) => {
            let mut l_expl__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_expl__str = dumpExpList(Tpl::emptyTxt.clone(), i_valList.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("List(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_expl__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CONS { car: i_car, cdr: i_cdr }, a_stringDelimiter) => {
            let mut l_cdr__str: Tpl::Text;
            let mut l_car__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_car__str = dumpExp(Tpl::emptyTxt.clone(), i_car.clone(), (a_stringDelimiter.clone()).clone())?;
            l_cdr__str = dumpExp(Tpl::emptyTxt.clone(), i_cdr.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("listCons(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_car__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cdr__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::META_TUPLE { listExp: i_listExp }, a_stringDelimiter) => {
            let mut l_tuple__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_tuple__str = dumpExpList(Tpl::emptyTxt.clone(), i_listExp.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Tuple(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_tuple__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::META_OPTION { exp: Some(i_exp) }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::META_OPTION { exp: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::METARECORDCALL { path: i_path, args: i_args, .. }, a_stringDelimiter) => {
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_args__str = dumpExpList(Tpl::emptyTxt.clone(), i_args.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeText(txt.clone(), l_name__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::MATCHEXPRESSION { matchType: i_matchType, inputs: i_inputs, cases: i_cases, .. }, a_stringDelimiter) => {
            let mut l_case__str: Tpl::Text;
            let mut l_inputs__str: Tpl::Text;
            let mut l_match__ty: Tpl::Text;
            let mut txt = (*txt).clone();
            l_match__ty = dumpMatchType(Tpl::emptyTxt.clone(), i_matchType.clone())?;
            l_inputs__str = dumpExpList(Tpl::emptyTxt.clone(), i_inputs.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            l_case__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_case__str = lm_35(l_case__str, i_cases.clone())?;
            l_case__str = Tpl::popIter(l_case__str)?;
            txt = Tpl::writeText(txt.clone(), l_match__ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_inputs__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(")\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_case__str)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_match__ty)?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BOX { exp: i_exp }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#(")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::UNBOX { exp: i_exp, .. }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unbox(")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SHARED_LITERAL { exp: i_exp, index: i_index }, a_stringDelimiter) => {
            let mut ret_42: bool;
            let mut txt = (*txt).clone();
            ret_42 = Config::typeinfo()?;
            txt = fun_36(txt.clone(), ret_42, i_index.clone(), (a_stringDelimiter.clone()).clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::PATTERN { pattern: i_pattern }, _) => {
            let mut ret_43: bool;
            let mut txt = (*txt).clone();
            ret_43 = Config::typeinfo()?;
            txt = fun_37(txt.clone(), ret_43)?;
            txt = dumpPattern(txt.clone(), i_pattern.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpExp: Unknown expression.")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn parenthesizeSubExp(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_exp)) {
        (txt, Deref @ DAE::Exp::ICONST { integer: _ }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RCONST { real: _ }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SCONST { string: _ }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: _ }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { name: _, .. }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: _, .. }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: _, .. }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ARRAY { ty: _, .. }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::MATRIX { ty: _, .. }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::TUPLE { PR: _ }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CAST { ty: _, .. }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SIZE { exp: _, .. }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::REDUCTION { reductionInfo: _, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("y")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_40(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_40 in &*items {
        let mut lstElt_40 = lstElt_40.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_40.clone()) {
        i_exp => {
            txt = dumpExp(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn dumpExpList(mut txt: Tpl::Text, mut a_expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_stringDelimiter: ArcStr, mut a_expDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (a_expDelimiter).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_40(out_txt, a_expl, (a_stringDelimiter).clone())?;
    out_txt = Tpl::popIter(out_txt)?;
    Ok(out_txt)
}

fn lm_42(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_42 in &*items {
        let mut lstElt_42 = lstElt_42.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_42.clone()) {
        i_exp => {
            txt = dumpExpCrefs(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn dumpExpListCrefs(mut txt: Tpl::Text, mut a_expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_stringDelimiter: ArcStr, mut a_expDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (a_expDelimiter).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_42(out_txt, a_expl, (a_stringDelimiter).clone())?;
    out_txt = Tpl::popIter(out_txt)?;
    Ok(out_txt)
}

pub fn dumpClockKind(mut in_txt: Tpl::Text, mut in_a_clk: Arc<DAE::ClockKind>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_clk, in_a_stringDelimiter)) {
        (txt, Deref @ DAE::ClockKind::INFERRED_CLOCK { .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Clock()")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: i_intervalCounter, resolution: i_resolution }, a_stringDelimiter) => {
            let mut l_re__str: Tpl::Text;
            let mut l_ic__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ic__str = dumpExp(Tpl::emptyTxt.clone(), i_intervalCounter.clone(), (a_stringDelimiter.clone()).clone())?;
            l_re__str = dumpExp(Tpl::emptyTxt.clone(), i_resolution.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Clock(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ic__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_re__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ClockKind::REAL_CLOCK { interval: i_interval }, a_stringDelimiter) => {
            let mut l_interval__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_interval__str = dumpExp(Tpl::emptyTxt.clone(), i_interval.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Clock(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_interval__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ClockKind::EVENT_CLOCK { condition: i_condition, startInterval: i_startInterval }, a_stringDelimiter) => {
            let mut l_si__str: Tpl::Text;
            let mut l_condition__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_condition__str = dumpExp(Tpl::emptyTxt.clone(), i_condition.clone(), (a_stringDelimiter.clone()).clone())?;
            l_si__str = dumpExp(Tpl::emptyTxt.clone(), i_startInterval.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Clock(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_condition__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_si__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ClockKind::SOLVER_CLOCK { c: i_c, solverMethod: i_solverMethod }, a_stringDelimiter) => {
            let mut l_sm__str: Tpl::Text;
            let mut l_clk__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_clk__str = dumpExp(Tpl::emptyTxt.clone(), i_c.clone(), (a_stringDelimiter.clone()).clone())?;
            l_sm__str = dumpExp(Tpl::emptyTxt.clone(), i_solverMethod.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Clock(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_clk__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_sm__str)?;
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

fn fun_45(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_cref__str: Tpl::Text, mut in_a_sub__str: Tpl::Text, mut in_a_ident: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_cref__str, in_a_sub__str, in_a_ident) {
        (mut txt, false, mut a_cref__str, mut a_sub__str, mut a_ident) => {
            txt = Tpl::writeStr(txt.clone(), (a_ident.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), a_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_cref__str.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_cref__str, mut a_sub__str, mut a_ident) => {
            txt = Tpl::writeStr(txt.clone(), (a_ident.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), a_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_cref__str.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpCref(mut in_txt: Tpl::Text, mut in_a_cref: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_cref)) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: i_subscriptLst, ident: i_ident, .. }) => {
            let mut l_sub__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_sub__str = dumpSubscripts(Tpl::emptyTxt.clone(), i_subscriptLst.clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_sub__str)?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { subscriptLst: i_subscriptLst, componentRef: i_componentRef, ident: i_ident, .. }) => {
            let mut ret_2: bool;
            let mut l_cref__str: Tpl::Text;
            let mut l_sub__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_sub__str = dumpSubscripts(Tpl::emptyTxt.clone(), i_subscriptLst.clone())?;
            l_cref__str = dumpCref(Tpl::emptyTxt.clone(), i_componentRef.clone())?;
            ret_2 = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
            txt = fun_45(txt.clone(), ret_2, l_cref__str, l_sub__str, (i_ident.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::WILD { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::OPTIMICA_ATTR_INST_CREF { componentRef: i_componentRef, instant: i_instant }) => {
            let mut txt = (*txt).clone();
            txt = dumpCref(txt.clone(), i_componentRef.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_instant.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpCref: unknown cref")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_47(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_47 in &*items {
        let mut lstElt_47 = lstElt_47.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_47.clone()) {
        i_sub => {
            txt = dumpSubscript(txt.clone(), i_sub.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_48(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_48 in &*items {
        let mut lstElt_48 = lstElt_48.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_48.clone()) {
        i_sub => {
            txt = dumpSubscript(txt.clone(), i_sub.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_49(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_subscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_subscripts)) {
        (txt, false, a_subscripts) => {
            let mut l_sub__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_sub__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_sub__str = lm_47(l_sub__str, a_subscripts.clone())?;
            l_sub__str = Tpl::popIter(l_sub__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_sub__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_subscripts) => {
            let mut l_sub__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_sub__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_sub__str = lm_48(l_sub__str, a_subscripts.clone())?;
            l_sub__str = Tpl::popIter(l_sub__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_sub__str)?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn dumpSubscripts(mut in_txt: Tpl::Text, mut in_a_subscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_subscripts)) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_subscripts) => {
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            ret_0 = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
            txt = fun_49(txt.clone(), ret_0, i_subscripts.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpSubscript(mut in_txt: Tpl::Text, mut in_a_subscript: Arc<DAE::Subscript>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_subscript)) {
        (txt, Deref @ DAE::Subscript::WHOLEDIM { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::SLICE { exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::INDEX { exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::WHOLE_NONEXP { exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn dumpReductionIterator(mut in_txt: Tpl::Text, mut in_a_iterator: Arc<DAE::ReductionIterator>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_iterator, in_a_stringDelimiter)) {
        (txt, Deref @ DAE::ReductionIterator { guardExp: None, exp: i_exp, id: i_id, .. }, a_stringDelimiter) => {
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" in ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt.clone()
        },
        (txt, Deref @ DAE::ReductionIterator { guardExp: Some(i_gexp), exp: i_exp, id: i_id, .. }, a_stringDelimiter) => {
            let mut l_guard__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            l_guard__str = dumpExp(Tpl::emptyTxt.clone(), i_gexp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" guard ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_guard__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" in ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_53(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_op__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_op__str) {
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

pub(crate) fn dumpOperand(mut txt: Tpl::Text, mut a_operand: Arc<DAE::Exp>, mut a_operation: Arc<DAE::Exp>, mut a_lhs: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: bool;
    let mut l_op__str: Tpl::Text;
    l_op__str = dumpExp(Tpl::emptyTxt.clone(), a_operand.clone(), (literal!("\"")).clone())?;
    ret_1 = ExpressionBasics::shouldParenthesize(a_operand, a_operation, a_lhs)?;
    out_txt = fun_53(txt, ret_1, l_op__str)?;
    Ok(out_txt)
}

fn fun_55(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_op) {
        (mut txt, DAE::Operator::ADD { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("^")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".*")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("./")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("^")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARR2 { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".^")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".+")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB_SCALAR_ARRAY { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".-")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_SCALAR_ARRAY { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".^")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".^")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_SCALAR_PRODUCT { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_MATRIX_PRODUCT { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_SCALAR_ARRAY { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("./")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpBinOp: Unknown operator.")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_56(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_op) {
        (mut txt, DAE::Operator::ADD { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("^")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+ /* ADD_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("- /* SUB_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".* /* MUL_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("./ /* DIV_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("^ /* POW_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARR2 { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".^ /* POW_ARR2 */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("* /* MUL_ARR_SCA */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".+ /* ADD_ARR_SCA */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB_SCALAR_ARRAY { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".- /* SUB_SCA_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_SCALAR_ARRAY { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".^ /* POW_SCA_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".^ /* POW_ARR_SCA */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_SCALAR_PRODUCT { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("* /* MUL_SCA_PRO */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_MATRIX_PRODUCT { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("* /* MUL_MAT_PRO */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_SCALAR_ARRAY { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/ /* DIV_SCA_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/ /* DIV_ARR_SCA */")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpBinOp: Unknown operator.")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_57(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_op) {
        (mut txt, false, mut a_op) => {
            txt = fun_55(txt.clone(), a_op.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_op) => {
            txt = fun_56(txt.clone(), a_op.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn dumpBinOp(mut txt: Tpl::Text, mut a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: bool;
    ret_0 = Config::typeinfo()?;
    out_txt = fun_57(txt, ret_0, a_op)?;
    Ok(out_txt)
}

pub(crate) fn dumpUnaryOp(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_op) {
        (mut txt, DAE::Operator::UMINUS { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::UMINUS_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpUnaryOp: Unknown operator.")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn dumpLogicalBinOp(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_op) {
        (mut txt, DAE::Operator::AND { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("and")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::OR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("or")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpLogicalBinOp: Unknown operator.")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn dumpLogicalUnaryOp(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_op) {
        (mut txt, DAE::Operator::NOT { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("not")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpLogicalUnaryOp: Unknown operator.")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn dumpRelationOp(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_op) {
        (mut txt, DAE::Operator::LESS { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::LESSEQ { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<=")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::GREATER { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::GREATEREQ { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">=")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::EQUAL { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("==")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::NEQUAL { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<>")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::USERDEFINED { fqName: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("USERDEFINED")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpRelationOp: Unknown operator.")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_63(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::FuncArg>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_63 in &*items {
        let mut lstElt_63 = lstElt_63.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_63.clone()) {
        i_arg => {
            txt = dumpFuncArg(txt.clone(), i_arg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_64(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_64 in &*items {
        let mut lstElt_64 = lstElt_64.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_64.clone()) {
        i_ty => {
            txt = dumpType(txt.clone(), i_ty.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_65(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_65 in &*items {
        let mut lstElt_65 = lstElt_65.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_65.clone()) {
        i_ty => {
            txt = dumpType(txt.clone(), i_ty.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpType(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt, in_a_ty)) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Integer")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Real")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Bool")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("String")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { path: i_path, .. }) => {
            let mut txt = (*txt).clone();
            return Ok(AbsynDumpTpl::dumpPath(txt.clone(), i_path.clone())?)
        },
        (txt, Deref @ DAE::Type::T_ARRAY { dims: i_dims, ty: i_ty }) => {
            let mut l_ty__str: Tpl::Text;
            let mut l_dim__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_dim__str = dumpDimensions(Tpl::emptyTxt.clone(), i_dims.clone())?;
            l_ty__str = dumpType(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ty__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_dim__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: i_complexClassType, .. }) => {
            let mut txt = (*txt).clone();
            return Ok(dumpClassState(txt.clone(), i_complexClassType.clone())?)
        },
        (txt, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: i_complexClassType, .. }) => {
            let mut txt = (*txt).clone();
            return Ok(dumpClassState(txt.clone(), i_complexClassType.clone())?)
        },
        (txt, Deref @ DAE::Type::T_FUNCTION { funcArg: i_funcArg, funcResultType: i_funcResultType, .. }) => {
            let mut l_ret__str: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_arg__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_arg__str = lm_63(l_arg__str, i_funcArg.clone())?;
            l_arg__str = Tpl::popIter(l_arg__str)?;
            l_ret__str = dumpType(Tpl::emptyTxt.clone(), i_funcResultType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<function>(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arg__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") => ")).clone() }))?;
            return Ok(Tpl::writeText(txt.clone(), l_ret__str)?)
        },
        (txt, Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: i_functionType }) => {
            let mut txt = (*txt).clone();
            { (in_txt, in_a_ty) = (txt.clone(), i_functionType.clone()); continue '__tco; }
        },
        (txt, Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { functionType: i_functionType, .. }) => {
            let mut txt = (*txt).clone();
            { (in_txt, in_a_ty) = (txt.clone(), i_functionType.clone()); continue '__tco; }
        },
        (txt, Deref @ DAE::Type::T_TUPLE { types: i_types, .. }) => {
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ty__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_ty__str = lm_64(l_ty__str, i_types.clone())?;
            l_ty__str = Tpl::popIter(l_ty__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            return Ok(Tpl::writeText(txt.clone(), l_ty__str)?)
        },
        (txt, Deref @ DAE::Type::T_CODE { ty: _ }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#T_CODE#")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_METALIST { ty: i_ty }) => {
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ty__str = dumpType(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("list<")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_METATUPLE { types: i_types }) => {
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ty__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_ty__str = lm_65(l_ty__str, i_types.clone())?;
            l_ty__str = Tpl::popIter(l_ty__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tuple<")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_METAOPTION { ty: i_ty }) => {
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ty__str = dumpType(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Option<")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_METAUNIONTYPE { path: i_path, .. }) => {
            let mut txt = (*txt).clone();
            return Ok(AbsynDumpTpl::dumpPath(txt.clone(), i_path.clone())?)
        },
        (txt, Deref @ DAE::Type::T_METARECORD { path: i_path, .. }) => {
            let mut txt = (*txt).clone();
            return Ok(AbsynDumpTpl::dumpPath(txt.clone(), i_path.clone())?)
        },
        (txt, Deref @ DAE::Type::T_METAARRAY { ty: i_ty }) => {
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ty__str = dumpType(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("array<")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: i_ty }) => {
            let mut txt = (*txt).clone();
            { (in_txt, in_a_ty) = (txt.clone(), i_ty.clone()); continue '__tco; }
        },
        (txt, Deref @ DAE::Type::T_METAPOLYMORPHIC { name: i_name }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("polymorphic<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_METATYPE { ty: i_ty }) => {
            let mut txt = (*txt).clone();
            { (in_txt, in_a_ty) = (txt.clone(), i_ty.clone()); continue '__tco; }
        },
        (txt, Deref @ DAE::Type::T_UNKNOWN { .. }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#T_UNKNOWN#")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_ANYTYPE { anyClassType: _ }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Any")).clone() }))?)
        },
        (txt, Deref @ DAE::Type::T_NORETCALL { .. }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#T_NORETCALL#")).clone() }))?)
        },
        (txt, _) => {
            return Ok(txt.clone())
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn dumpFuncArg(mut in_txt: Tpl::Text, mut in_a_arg: Arc<DAE::FuncArg>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_arg)) {
        (txt, Deref @ DAE::FuncArg { name: i_arg_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_arg_name.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_68(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_68 in &*items {
        let mut lstElt_68 = lstElt_68.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_68.clone()) {
        i_dim => {
            txt = dumpDimension(txt.clone(), i_dim.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpDimensions(mut txt: Tpl::Text, mut a_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_68(out_txt, a_dims)?;
    out_txt = Tpl::popIter(out_txt)?;
    Ok(out_txt)
}

pub fn dumpDimension(mut in_txt: Tpl::Text, mut in_a_dim: Arc<DAE::Dimension>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_dim)) {
        (txt, Deref @ DAE::Dimension::DIM_INTEGER { integer: i_integer }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_integer.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Dimension::DIM_ENUM { enumTypeName: i_enumTypeName, .. }) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::dumpPath(txt.clone(), i_enumTypeName.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Dimension::DIM_EXP { exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Dimension::DIM_UNKNOWN { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn dumpClassState(mut txt: Tpl::Text, mut a_state: ClassInf::State) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: Arc<Absyn::Path>;
    ret_0 = ClassInfUtil::getStateName(a_state);
    out_txt = AbsynDumpTpl::dumpPath(txt, ret_0)?;
    Ok(out_txt)
}

pub(crate) fn dumpMatchType(mut in_txt: Tpl::Text, mut in_a_ty: DAE::MatchType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_ty)) {
        (txt, DAE::MatchType::MATCHCONTINUE { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("matchcontinue")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::MatchType::MATCH { switch: None }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("match")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::MatchType::MATCH { switch: Some(_) }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("match /* switch */")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn dumpMatchCase(mut in_txt: Tpl::Text, mut in_a_mcase: Arc<DAE::MatchCase>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_mcase)) {
        (txt, Deref @ DAE::MatchCase { body: Deref @ metamodelica::List::Nil, result: Some(i_result), patterns: i_patterns, .. }) => {
            let mut l_res__str: Tpl::Text;
            let mut l_pat__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_pat__str = dumpPatterns(Tpl::emptyTxt.clone(), i_patterns.clone())?;
            l_res__str = dumpExp(Tpl::emptyTxt.clone(), i_result.clone(), (literal!("\"")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_pat__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") then ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_res__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::MatchCase { body: Deref @ metamodelica::List::Nil, result: None, patterns: i_patterns, .. }) => {
            let mut l_pat__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_pat__str = dumpPatterns(Tpl::emptyTxt.clone(), i_patterns.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_pat__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") then fail();")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::MatchCase { result: Some(i_result), patterns: i_patterns, body: i_body, .. }) => {
            let mut l_body__str: Tpl::Text;
            let mut l_res__str: Tpl::Text;
            let mut l_pat__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_pat__str = dumpPatterns(Tpl::emptyTxt.clone(), i_patterns.clone())?;
            l_res__str = dumpExp(Tpl::emptyTxt.clone(), i_result.clone(), (literal!("\"")).clone())?;
            l_body__str = DAEDumpTpl::dumpStatements(Tpl::emptyTxt.clone(), i_body.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_pat__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("  algorithm\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  then\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_res__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::MatchCase { patterns: i_patterns, body: i_body, .. }) => {
            let mut l_body__str: Tpl::Text;
            let mut l_pat__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_pat__str = dumpPatterns(Tpl::emptyTxt.clone(), i_patterns.clone())?;
            l_body__str = DAEDumpTpl::dumpStatements(Tpl::emptyTxt.clone(), i_body.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_pat__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("  algorithm\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  then\n")).clone(), (literal!("    fail();")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_74(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Pattern>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_74 in &*items {
        let mut lstElt_74 = lstElt_74.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_74.clone()) {
        i_pat => {
            txt = dumpPattern(txt.clone(), i_pat.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn dumpPatterns(mut txt: Tpl::Text, mut a_patterns: Arc<metamodelica::List<Arc<DAE::Pattern>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_74(out_txt, a_patterns)?;
    out_txt = Tpl::popIter(out_txt)?;
    Ok(out_txt)
}

fn lm_76(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<(Arc<DAE::Pattern>, ArcStr, Arc<DAE::Type>)>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_76 in &*items {
        let mut lstElt_76 = lstElt_76.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_76.clone()) {
        i_pat => {
            txt = dumpNamedPattern(txt.clone(), i_pat.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn dumpPattern(mut in_txt: Tpl::Text, mut in_a_pattern: Arc<DAE::Pattern>) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt, in_a_pattern)) {
        (txt, Deref @ DAE::Pattern::PAT_WILD { .. }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?)
        },
        (txt, Deref @ DAE::Pattern::PAT_AS { pat: Deref @ DAE::Pattern::PAT_WILD { .. }, id: i_id, .. }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?)
        },
        (txt, Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { pat: Deref @ DAE::Pattern::PAT_WILD { .. }, id: i_id }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?)
        },
        (txt, Deref @ DAE::Pattern::PAT_SOME { pat: i_pat }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            txt = dumpPattern(txt.clone(), i_pat.clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Pattern::PAT_META_TUPLE { patterns: i_patterns }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = dumpPatterns(txt.clone(), i_patterns.clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Pattern::PAT_CALL_TUPLE { patterns: i_patterns }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = dumpPatterns(txt.clone(), i_patterns.clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Pattern::PAT_CALL { name: i_name, patterns: i_patterns, .. }) => {
            let mut l_pat__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_name.clone())?;
            l_pat__str = dumpPatterns(Tpl::emptyTxt.clone(), i_patterns.clone())?;
            txt = Tpl::writeText(txt.clone(), l_name__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_pat__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Pattern::PAT_CALL_NAMED { name: i_name, patterns: i_patterns_1 }) => {
            let mut l_pat__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_name.clone())?;
            l_pat__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_pat__str = lm_76(l_pat__str, i_patterns_1.clone())?;
            l_pat__str = Tpl::popIter(l_pat__str)?;
            txt = Tpl::writeText(txt.clone(), l_name__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_pat__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Pattern::PAT_CONS { head: i_head, tail: i_tail }) => {
            let mut txt = (*txt).clone();
            txt = dumpPattern(txt.clone(), i_head.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::")).clone() }))?;
            { (in_txt, in_a_pattern) = (txt.clone(), i_tail.clone()); continue '__tco; }
        },
        (txt, Deref @ DAE::Pattern::PAT_CONSTANT { exp: i_exp, .. }) => {
            let mut txt = (*txt).clone();
            return Ok(dumpExp(txt.clone(), i_exp.clone(), (literal!("\"")).clone())?)
        },
        (txt, Deref @ DAE::Pattern::PAT_AS { id: i_id, pat: i_pat, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" as ")).clone() }))?;
            { (in_txt, in_a_pattern) = (txt.clone(), i_pat.clone()); continue '__tco; }
        },
        (txt, Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { id: i_id, pat: i_pat }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" as ")).clone() }))?;
            { (in_txt, in_a_pattern) = (txt.clone(), i_pat.clone()); continue '__tco; }
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*PATTERN*")).clone() }))?)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn dumpNamedPattern(mut in_txt: Tpl::Text, mut in_a_pattern: (Arc<DAE::Pattern>, ArcStr, Arc<DAE::Type>)) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_pattern)) {
        (txt, (i_pat, i_id, _)) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = dumpPattern(txt.clone(), i_pat.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_79(mut in_txt: Tpl::Text, mut in_a_scalar: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_scalar) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* non-scalar */ ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* scalar */ ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_80(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_scalar: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_mArg, in_a_scalar) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_scalar) => {
            txt = fun_79(txt.clone(), a_scalar.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_81(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_81 in &*items {
        let mut lstElt_81 = lstElt_81.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_81.clone()) {
        i_row => {
            txt = dumpExpList(txt.clone(), i_row.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_82(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_mArg, in_a_ty)) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_ty) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* matrix ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */ ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_83(mut in_txt: Tpl::Text, mut in_a_step: Option<Arc<DAE::Exp>>, mut in_a_e: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_step, in_a_e)) {
        (txt, Some(i_step), a_e) => {
            let mut txt = (*txt).clone();
            txt = dumpOperand(txt.clone(), i_step.clone(), a_e.clone(), false)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_84(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_needs__paren)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_85(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_needs__paren)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_86(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_needs__paren)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_87(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_needs__paren)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_88(mut in_txt: Tpl::Text, mut in_a_sz: Option<Arc<DAE::Exp>>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_sz, in_a_stringDelimiter)) {
        (txt, Some(i_dim), a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_dim.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_89(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, mut a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_89 in &*items {
        let mut lstElt_89 = lstElt_89.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_89.clone()) {
        i_it => {
            txt = dumpReductionIterator(txt.clone(), i_it.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_90(mut in_txt: Tpl::Text, mut in_a_ri_iterType: Absyn::ReductionIterType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_ri_iterType) {
        (mut txt, Absyn::ReductionIterType::THREAD { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threaded ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_91(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::MatchCase>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_91 in &*items {
        let mut lstElt_91 = lstElt_91.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_91.clone()) {
        i_c => {
            txt = dumpMatchCase(txt.clone(), i_c.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpExpCrefs(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt, in_a_exp, in_a_stringDelimiter)) {
        (txt, Deref @ DAE::Exp::ICONST { integer: _ }, _) => {
            return Ok(txt.clone())
        },
        (txt, Deref @ DAE::Exp::RCONST { real: _ }, _) => {
            return Ok(txt.clone())
        },
        (txt, Deref @ DAE::Exp::SCONST { string: _ }, _) => {
            return Ok(txt.clone())
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: _ }, _) => {
            return Ok(txt.clone())
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { name: i_name, .. }, _) => {
            let mut txt = (*txt).clone();
            return Ok(AbsynDumpTpl::dumpPath(txt.clone(), i_name.clone())?)
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_componentRef, .. }, _) => {
            let mut txt = (*txt).clone();
            return Ok(dumpCref(txt.clone(), i_componentRef.clone())?)
        },
        (txt, Deref @ DAE::Exp::BINARY { exp1: i_exp1, exp2: i_exp2, .. }, a_stringDelimiter) => {
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp1.clone(), (a_stringDelimiter.clone()).clone())?;
            l_rhs__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp2.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            return Ok(Tpl::writeText(txt.clone(), l_rhs__str)?)
        },
        (txt, i_e @ Deref @ DAE::Exp::UNARY { exp: i_exp, operator: i_operator }, _) => {
            let mut l_op__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp.clone(), i_e.clone(), false)?;
            l_op__str = dumpUnaryOp(Tpl::emptyTxt.clone(), i_operator.clone())?;
            txt = Tpl::writeText(txt.clone(), l_op__str)?;
            return Ok(Tpl::writeText(txt.clone(), l_exp__str)?)
        },
        (txt, Deref @ DAE::Exp::LBINARY { exp1: i_exp1, exp2: i_exp2, .. }, a_stringDelimiter) => {
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp1.clone(), (a_stringDelimiter.clone()).clone())?;
            l_rhs__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp2.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            return Ok(Tpl::writeText(txt.clone(), l_rhs__str)?)
        },
        (txt, Deref @ DAE::Exp::LUNARY { exp: i_exp, .. }, a_stringDelimiter) => {
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            return Ok(Tpl::writeText(txt.clone(), l_lhs__str)?)
        },
        (txt, Deref @ DAE::Exp::RELATION { exp1: i_exp1, exp2: i_exp2, .. }, a_stringDelimiter) => {
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp1.clone(), (a_stringDelimiter.clone()).clone())?;
            l_rhs__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp2.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            return Ok(Tpl::writeText(txt.clone(), l_rhs__str)?)
        },
        (txt, Deref @ DAE::Exp::IFEXP { expCond: i_expCond, expThen: i_expThen, expElse: i_expElse }, a_stringDelimiter) => {
            let mut l_else__str: Tpl::Text;
            let mut l_then__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cond__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_expCond.clone(), (a_stringDelimiter.clone()).clone())?;
            l_then__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_expThen.clone(), (a_stringDelimiter.clone()).clone())?;
            l_else__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_expElse.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_cond__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_then__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            return Ok(Tpl::writeText(txt.clone(), l_else__str)?)
        },
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: true, .. }, expLst: i_expLst, .. }, a_stringDelimiter) => {
            let mut l_argl: Tpl::Text;
            let mut txt = (*txt).clone();
            l_argl = dumpExpListCrefs(Tpl::emptyTxt.clone(), i_expLst.clone(), (a_stringDelimiter.clone()).clone(), (literal!(" ")).clone())?;
            return Ok(Tpl::writeText(txt.clone(), l_argl)?)
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: i_expLst, .. }, a_stringDelimiter) => {
            let mut l_argl: Tpl::Text;
            let mut txt = (*txt).clone();
            l_argl = dumpExpListCrefs(Tpl::emptyTxt.clone(), i_expLst.clone(), (a_stringDelimiter.clone()).clone(), (literal!(" ")).clone())?;
            return Ok(Tpl::writeText(txt.clone(), l_argl)?)
        },
        (txt, Deref @ DAE::Exp::PARTEVALFUNCTION { path: i_path, expList: i_expList, .. }, a_stringDelimiter) => {
            let mut l_func__str: Tpl::Text;
            let mut l_argl: Tpl::Text;
            let mut txt = (*txt).clone();
            l_func__str = AbsynDumpTpl::dumpPathNoQual(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_argl = dumpExpList(Tpl::emptyTxt.clone(), i_expList.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_func__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_argl)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::ARRAY { array: i_array, scalar: i_scalar, .. }, a_stringDelimiter) => {
            let mut ret_10: bool;
            let mut l_expl: Tpl::Text;
            let mut txt = (*txt).clone();
            l_expl = dumpExpList(Tpl::emptyTxt.clone(), i_array.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            ret_10 = Config::typeinfo()?;
            txt = fun_80(txt.clone(), ret_10, i_scalar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_expl)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::MATRIX { matrix: i_matrix, ty: i_ty, .. }, a_stringDelimiter) => {
            let mut ret_12: bool;
            let mut l_mat__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_mat__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_mat__str = lm_81(l_mat__str, i_matrix.clone(), (a_stringDelimiter.clone()).clone())?;
            l_mat__str = Tpl::popIter(l_mat__str)?;
            ret_12 = Config::typeinfo()?;
            txt = fun_82(txt.clone(), ret_12, i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mat__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}}")).clone() }))?)
        },
        (txt, i_e @ Deref @ DAE::Exp::RANGE { start: i_start, step: i_step, stop: i_stop, .. }, _) => {
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_start__str = dumpOperand(Tpl::emptyTxt.clone(), i_start.clone(), i_e.clone(), false)?;
            l_step__str = fun_83(Tpl::emptyTxt.clone(), i_step.clone(), i_e.clone())?;
            l_stop__str = dumpOperand(Tpl::emptyTxt.clone(), i_stop.clone(), i_e.clone(), false)?;
            txt = Tpl::writeText(txt.clone(), l_start__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_step__str)?;
            return Ok(Tpl::writeText(txt.clone(), l_stop__str)?)
        },
        (txt, Deref @ DAE::Exp::TUPLE { PR: Deref @ metamodelica::List::Nil }, _) => {
            return Ok(txt.clone())
        },
        (txt, Deref @ DAE::Exp::TUPLE { PR: i_PR }, a_stringDelimiter) => {
            let mut l_tuple__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_tuple__str = dumpExpList(Tpl::emptyTxt.clone(), i_PR.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_tuple__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::CAST { exp: i_exp, .. }, a_stringDelimiter) => {
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::ASUB { exp: i_exp, sub: i_sub }, a_stringDelimiter) => {
            let mut l_sub__str: Tpl::Text;
            let mut l_rparen: Tpl::Text;
            let mut l_lparen: Tpl::Text;
            let mut l_needs__paren: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_needs__paren = parenthesizeSubExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_lparen = fun_84(Tpl::emptyTxt.clone(), l_needs__paren.clone())?;
            l_rparen = fun_85(Tpl::emptyTxt.clone(), l_needs__paren)?;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            l_sub__str = dumpSubscripts(Tpl::emptyTxt.clone(), i_sub.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lparen)?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt = Tpl::writeText(txt.clone(), l_rparen)?;
            return Ok(Tpl::writeText(txt.clone(), l_sub__str)?)
        },
        (txt, Deref @ DAE::Exp::TSUB { exp: i_exp, ix: i_ix, .. }, a_stringDelimiter) => {
            let mut l_rparen: Tpl::Text;
            let mut l_lparen: Tpl::Text;
            let mut l_needs__paren: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_needs__paren = parenthesizeSubExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_lparen = fun_86(Tpl::emptyTxt.clone(), l_needs__paren.clone())?;
            l_rparen = fun_87(Tpl::emptyTxt.clone(), l_needs__paren)?;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_lparen)?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt = Tpl::writeText(txt.clone(), l_rparen)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_ix.clone())).clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::SIZE { exp: i_exp, sz: i_sz }, a_stringDelimiter) => {
            let mut l_dim__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            l_dim__str = fun_88(Tpl::emptyTxt.clone(), i_sz.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("size(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt = Tpl::writeText(txt.clone(), l_dim__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::CODE { code: i_code, .. }, _) => {
            let mut ret_23: ArcStr;
            let mut l_code__str: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_23 = (Dump::printCodeStr(i_code.clone())?).clone();
            l_code__str = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_23).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$Code(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_code__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::EMPTY { name: i_name_1, scope: i_scope, tyStr: i_tyStr, .. }, _) => {
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = dumpCref(Tpl::emptyTxt.clone(), i_name_1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<EMPTY(scope: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_scope.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", name: ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ty: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_tyStr.clone()).clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")>")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: i_name, iterType: i_ri_iterType, .. }, expr: i_expr, iterators: i_iterators }, a_stringDelimiter) => {
            let mut l_iter__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPathNoQual(Tpl::emptyTxt.clone(), i_name.clone())?;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_expr.clone(), (a_stringDelimiter.clone()).clone())?;
            l_iter__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_iter__str = lm_89(l_iter__str, i_iterators.clone(), (a_stringDelimiter.clone()).clone())?;
            l_iter__str = Tpl::popIter(l_iter__str)?;
            txt = Tpl::writeText(txt.clone(), l_name__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" for ")).clone() }))?;
            txt = fun_90(txt.clone(), i_ri_iterType.clone())?;
            txt = Tpl::writeText(txt.clone(), l_iter__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::LIST { valList: i_valList }, a_stringDelimiter) => {
            let mut l_expl__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_expl__str = dumpExpList(Tpl::emptyTxt.clone(), i_valList.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("List(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_expl__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::CONS { car: i_car, cdr: i_cdr }, a_stringDelimiter) => {
            let mut l_cdr__str: Tpl::Text;
            let mut l_car__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_car__str = dumpExp(Tpl::emptyTxt.clone(), i_car.clone(), (a_stringDelimiter.clone()).clone())?;
            l_cdr__str = dumpExp(Tpl::emptyTxt.clone(), i_cdr.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("listCons(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_car__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cdr__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::META_TUPLE { listExp: i_listExp }, a_stringDelimiter) => {
            let mut l_tuple__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_tuple__str = dumpExpList(Tpl::emptyTxt.clone(), i_listExp.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Tuple(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_tuple__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::META_OPTION { exp: Some(i_exp) }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::META_OPTION { exp: _ }, _) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::METARECORDCALL { path: i_path, args: i_args, .. }, a_stringDelimiter) => {
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_args__str = dumpExpList(Tpl::emptyTxt.clone(), i_args.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeText(txt.clone(), l_name__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str)?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::MATCHEXPRESSION { matchType: i_matchType, inputs: i_inputs, cases: i_cases, .. }, a_stringDelimiter) => {
            let mut l_case__str: Tpl::Text;
            let mut l_inputs__str: Tpl::Text;
            let mut l_match__ty: Tpl::Text;
            let mut txt = (*txt).clone();
            l_match__ty = dumpMatchType(Tpl::emptyTxt.clone(), i_matchType.clone())?;
            l_inputs__str = dumpExpList(Tpl::emptyTxt.clone(), i_inputs.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            l_case__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_case__str = lm_91(l_case__str, i_cases.clone())?;
            l_case__str = Tpl::popIter(l_case__str)?;
            txt = Tpl::writeText(txt.clone(), l_match__ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_inputs__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(")\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_case__str)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_match__ty)?;
            return Ok(Tpl::popBlock(txt.clone())?)
        },
        (txt, Deref @ DAE::Exp::BOX { exp: i_exp }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#(")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::UNBOX { exp: i_exp, .. }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unbox(")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::Exp::SHARED_LITERAL { exp: i_exp, .. }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            { (in_txt, in_a_exp, in_a_stringDelimiter) = (txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone()); continue '__tco; }
        },
        (txt, Deref @ DAE::Exp::PATTERN { pattern: i_pattern }, _) => {
            let mut txt = (*txt).clone();
            return Ok(dumpPattern(txt.clone(), i_pattern.clone())?)
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            return Ok(errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpExp: Unknown expression.")).clone())?)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn errorMsg(mut txt: Tpl::Text, mut a_errMessage: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    Tpl::addTemplateError((a_errMessage.clone()).clone())?;
    out_txt = Tpl::writeStr(txt, (a_errMessage).clone())?;
    Ok(out_txt)
}

fn fun_94(mut in_txt: Tpl::Text, mut in_a_con: Arc<DAE::Constraint>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_con)) {
        (txt, Deref @ DAE::Constraint::CONSTRAINT_DT { constraint: i_c, localCon: true }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_c.clone(), (literal!("\"")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" (local)")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Constraint::CONSTRAINT_DT { constraint: i_c, localCon: false }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_c.clone(), (literal!("\"")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" (global)")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_95(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Constraint>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_95 in &*items {
        let mut lstElt_95 = lstElt_95.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_95.clone()) {
        i_con => {
            txt = fun_94(txt.clone(), i_con.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub fn dumpConstraints(mut txt: Tpl::Text, mut a_cons: Arc<metamodelica::List<Arc<DAE::Constraint>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt, Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
    out_txt = lm_95(out_txt, a_cons)?;
    out_txt = Tpl::popIter(out_txt)?;
    Ok(out_txt)
}

