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
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::ExpressionDumpTpl;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;

pub fn symbolName(mut txt: Tpl::Text, mut a_modelNamePrefix: ArcStr, mut a_symbolName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefix.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_symbolName.clone()).clone())?;
    Ok(out_txt)
}

pub(crate) fn replaceDotAndUnderscore(mut txt: Tpl::Text, mut a_str: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_4: ArcStr;
    let mut ret_3: ArcStr;
    let mut l_str__underscores: Tpl::Text;
    let mut ret_1: ArcStr;
    let mut l_str__dots: Tpl::Text;
    ret_1 = (System::stringReplace((a_str.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
    l_str__dots = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
    ret_3 = (System::stringReplace((Tpl::textString(l_str__dots.clone())?).clone(), (literal!("_")).clone(), (literal!("__")).clone())?).clone();
    l_str__underscores = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_3.clone()).clone())?;
    ret_4 = (System::unquoteIdentifier((Tpl::textString(l_str__underscores.clone())?).clone())).clone();
    out_txt = Tpl::writeStr(txt.clone(), (ret_4.clone()).clone())?;
    Ok(out_txt)
}

pub fn getGeneralTarget(mut in_txt: Tpl::Text, mut in_a_str: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_str.clone())) {
        (txt, Deref @ "msvc10") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("msvc")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "msvc12") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("msvc")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "msvc13") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("msvc")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "msvc15") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("msvc")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "msvc19") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("msvc")).clone() }))?;
            txt.clone()
        },
        (txt, i_str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_str.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn underscorePath(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
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

pub fn crefStr(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: i_ident, subscriptLst: i_subscriptLst, .. }) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            ret_0 = (System::unquoteIdentifier((i_ident.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            return Ok(subscriptsStr(txt.clone(), i_subscriptLst.clone())?)
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$DER", componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("der(")).clone() }))?;
            txt = crefStr(txt.clone(), i_componentRef.clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$CLKPRE", componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("previous(")).clone() }))?;
            txt = crefStr(txt.clone(), i_componentRef.clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: i_ident, subscriptLst: i_subscriptLst, componentRef: i_componentRef, .. }) => {
            let mut ret_1: ArcStr;
            let mut txt = (*txt).clone();
            ret_1 = (System::unquoteIdentifier((i_ident.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = subscriptsStr(txt.clone(), i_subscriptLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("._")).clone() }))?;
            { (in_txt, in_a_cr) = (txt.clone(), i_componentRef.clone()); continue '__tco; }
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_NOT_IDENT_OR_QUAL")).clone() }))?)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn crefStrNoUnderscore(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: i_ident, subscriptLst: i_subscriptLst, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            return Ok(subscriptsStr(txt.clone(), i_subscriptLst.clone())?)
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$DER", componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("der(")).clone() }))?;
            txt = crefStrNoUnderscore(txt.clone(), i_componentRef.clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$CLKPRE", componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("previous(")).clone() }))?;
            txt = crefStrNoUnderscore(txt.clone(), i_componentRef.clone())?;
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?)
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: i_ident, subscriptLst: i_subscriptLst, componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt = subscriptsStr(txt.clone(), i_subscriptLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            { (in_txt, in_a_cr) = (txt.clone(), i_componentRef.clone()); continue '__tco; }
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_NOT_IDENT_OR_QUAL")).clone() }))?)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lm_49(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_49 in &*items.clone() {
        let mut lstElt_49 = lstElt_49.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_49.clone()) {
        i_s => {
            txt = subscriptStr(txt.clone(), i_s.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn subscriptsStr(mut in_txt: Tpl::Text, mut in_a_subscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscripts.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_subscripts) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_49(txt.clone(), i_subscripts.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn crefStrMatlabSafe(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: i_ident, subscriptLst: i_subscriptLst, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            return Ok(subscriptsStrMatlabSafe(txt.clone(), i_subscriptLst.clone())?)
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$DER", componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("der_")).clone() }))?;
            { (in_txt, in_a_cr) = (txt.clone(), i_componentRef.clone()); continue '__tco; }
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$CLKPRE", componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("pre_")).clone() }))?;
            { (in_txt, in_a_cr) = (txt.clone(), i_componentRef.clone()); continue '__tco; }
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: i_ident, subscriptLst: i_subscriptLst, componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt = subscriptsStrMatlabSafe(txt.clone(), i_subscriptLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            { (in_txt, in_a_cr) = (txt.clone(), i_componentRef.clone()); continue '__tco; }
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_NOT_IDENT_OR_QUAL")).clone() }))?)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn lm_52(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_52 in &*items.clone() {
        let mut lstElt_52 = lstElt_52.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_52.clone()) {
        i_s => {
            txt = subscriptStr(txt.clone(), i_s.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn subscriptsStrMatlabSafe(mut in_txt: Tpl::Text, mut in_a_subscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscripts.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_subscripts) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_52(txt.clone(), i_subscripts.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn subscriptStr(mut in_txt: Tpl::Text, mut in_a_subscript: Arc<DAE::Subscript>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscript.clone())) {
        (txt, Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i_i } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_i.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::BCONST { bool: i_i_1 } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_i_1.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ENUM_LITERAL { name: i_n, .. } }) => {
            let mut txt = (*txt).clone();
            txt = dotPath(txt.clone(), i_n.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::INDEX { exp: i_exp @ Deref @ DAE::Exp::CREF { componentRef: _, .. } }) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            ret_0 = (ExpressionBasics::printExpStr(i_exp.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ICONST { integer: i_i } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_i.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::INDEX { exp: i_exp }) => {
            let mut ret_1: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UNKNOWN_SUBSCRIPT /* ")).clone() }))?;
            ret_1 = (ExpressionBasics::printExpStr(i_exp.clone())?).clone();
            txt = escapeCComments(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::SLICE { exp: i_exp }) => {
            let mut ret_2: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UNKNOWN_SUBSCRIPT /* ")).clone() }))?;
            ret_2 = (ExpressionBasics::printExpStr(i_exp.clone())?).clone();
            txt = escapeCComments(txt.clone(), (ret_2.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::WHOLEDIM { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WHOLEDIM")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::WHOLE_NONEXP { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WHOLE_NONEXP")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UNKNOWN_SUBSCRIPT")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn escapeCComments(mut txt: Tpl::Text, mut a_stringWithCComments: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: ArcStr;
    let mut ret_0: ArcStr;
    ret_0 = (System::stringReplace((a_stringWithCComments.clone()).clone(), (literal!("/*")).clone(), (literal!("(*")).clone())?).clone();
    ret_1 = (System::stringReplace((ret_0.clone()).clone(), (literal!("*/")).clone(), (literal!("*)")).clone())?).clone();
    out_txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
    Ok(out_txt)
}

fn fun_56(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_vName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_vName.clone()) {
        (mut txt, false, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OBFUSCATED")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_vName) => {
            txt = escapeCComments(txt.clone(), (a_vName.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_57(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_vName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_vName.clone()) {
        (mut txt, false, mut a_vName) => {
            txt = escapeCComments(txt.clone(), (a_vName.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OBFUSCATED")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn crefCComment(mut in_txt: Tpl::Text, mut in_a_v: SimCodeVar::SimVar, mut in_a_vName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_v.clone(), in_a_vName.clone()) {
        (mut txt, SimCodeVar::SimVar { isProtected: true, .. }, mut a_vName) => {
            let mut ret_1: bool;
            let mut ret_0: ArcStr;
            ret_0 = (Flags::getConfigString(Flags::OBFUSCATE.clone())?).clone();
            ret_1 = stringEq((ret_0.clone()).clone(), (literal!("none")).clone());
            txt = fun_56(txt.clone(), ret_1.clone(), (a_vName.clone()).clone())?;
            txt.clone()
        },
        (mut txt, SimCodeVar::SimVar { name: _, .. }, mut a_vName) => {
            let mut ret_3: bool;
            let mut ret_2: ArcStr;
            ret_2 = (Flags::getConfigString(Flags::OBFUSCATE.clone())?).clone();
            ret_3 = stringEq((ret_2.clone()).clone(), (literal!("full")).clone());
            txt = fun_57(txt.clone(), ret_3.clone(), (a_vName.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_59(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_varKind: BackendDAE::VarKind, mut in_a_name: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_varKind.clone(), in_a_name.clone())) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_varKind, a_name) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* ")).clone() }))?;
            txt_0 = crefStrNoUnderscore(Tpl::emptyTxt.clone(), a_name.clone())?;
            txt = escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = variabilityString(txt.clone(), a_varKind.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_60(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_varKind: BackendDAE::VarKind, mut in_a_name: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_varKind.clone(), in_a_name.clone())) {
        (txt, false, a_varKind, a_name) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* ")).clone() }))?;
            txt_0 = crefStrNoUnderscore(Tpl::emptyTxt.clone(), a_name.clone())?;
            txt = escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = variabilityString(txt.clone(), a_varKind.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */")).clone() }))?;
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

pub fn crefCCommentWithVariability(mut in_txt: Tpl::Text, mut in_a_v: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_v.clone()) {
        (mut txt, SimCodeVar::SimVar { isProtected: true, name: ref i_name, varKind: mut i_varKind, .. }) => {
            let mut ret_1: bool;
            let mut ret_0: ArcStr;
            ret_0 = (Flags::getConfigString(Flags::OBFUSCATE.clone())?).clone();
            ret_1 = stringEq((ret_0.clone()).clone(), (literal!("none")).clone());
            txt = fun_59(txt.clone(), ret_1.clone(), i_varKind.clone(), i_name.clone())?;
            txt.clone()
        },
        (mut txt, SimCodeVar::SimVar { name: ref i_name, varKind: mut i_varKind, .. }) => {
            let mut ret_3: bool;
            let mut ret_2: ArcStr;
            ret_2 = (Flags::getConfigString(Flags::OBFUSCATE.clone())?).clone();
            ret_3 = stringEq((ret_2.clone()).clone(), (literal!("full")).clone());
            txt = fun_60(txt.clone(), ret_3.clone(), i_varKind.clone(), i_name.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn escapeSingleQuoteIdent(mut txt: Tpl::Text, mut a_ident: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: ArcStr;
    let mut ret_0: ArcStr;
    ret_0 = (System::stringReplace((a_ident.clone()).clone(), (literal!("\\'")).clone(), (literal!("\\\\'")).clone())?).clone();
    ret_1 = (System::stringReplace((ret_0.clone()).clone(), (literal!("'")).clone(), (literal!("\\'")).clone())?).clone();
    out_txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
    Ok(out_txt)
}

pub fn initDefaultValXml(mut in_txt: Tpl::Text, mut in_a_type__: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type__.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0.0")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("false")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        (txt, i_type__) => {
            let mut txt_0: Tpl::Text;
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("initial value of unknown type: ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(i_type__.clone())?).clone();
            txt_0 = Tpl::writeStr(txt_0.clone(), (ret_0.clone()).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenUtil.tpl")).clone(), 249, 14), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_64(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_stringQuotes: ArcStr) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_64 in &*items.clone() {
        let mut lstElt_64 = lstElt_64.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_64.clone()) {
        i_elem => {
            txt = initValXml(txt.clone(), i_elem.clone(), (a_stringQuotes.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_65(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringQuotes: ArcStr, mut in_a_expr: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_stringQuotes.clone(), in_a_expr.clone())) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_stringQuotes, a_expr) => {
            let mut txt = (*txt).clone();
            txt = initValXml(txt.clone(), a_expr.clone(), (a_stringQuotes.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn initValXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_stringQuotes: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_stringQuotes.clone())) {
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
        (txt, Deref @ DAE::Exp::SCONST { string: i_string }, a_stringQuotes) => {
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_stringQuotes.clone()).clone())?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_string.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_stringQuotes.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: i_bool }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_bool.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ARRAY { array: i_array, .. }, a_stringQuotes) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            txt = lm_64(txt.clone(), i_array.clone(), (a_stringQuotes.clone()).clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::REDUCTION { expr: i_expr, .. }, a_stringQuotes) => {
            let mut ret_1: bool;
            let mut txt = (*txt).clone();
            ret_1 = Expression::isSimpleLiteralValue(i_expr.clone(), true)?;
            txt = fun_65(txt.clone(), ret_1.clone(), (a_stringQuotes.clone()).clone(), i_expr.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { index: i_index, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, i_exp, _) => {
            let mut txt_2: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_2 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("initial value of unknown type: ")).clone() }))?;
            txt_2 = ExpressionDumpTpl::dumpExp(txt_2.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenUtil.tpl")).clone(), 262, 14), (Tpl::textString(txt_2.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn getVariablity(mut in_txt: Tpl::Text, mut in_a_varKind: BackendDAE::VarKind) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_varKind.clone()) {
        (mut txt, BackendDAE::VarKind::DISCRETE { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("discrete")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::PARAM { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parameter")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::CONST { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constant")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("continuous")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn variabilityString(mut in_txt: Tpl::Text, mut in_a_varKind: BackendDAE::VarKind) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_varKind.clone())) {
        (txt, BackendDAE::VarKind::VARIABLE { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("variable")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::STATE { derName: None, index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("STATE(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::STATE { derName: Some(i_dcr), index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("STATE(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = crefStrNoUnderscore(txt.clone(), i_dcr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::STATE_DER { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("STATE_DER")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::DUMMY_DER { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("DUMMY_DER")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::DUMMY_STATE { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("DUMMY_STATE")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::CLOCKED_STATE { previousName: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CLOCKED_STATE")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::DISCRETE { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("DISCRETE")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::PARAM { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("PARAM")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::CONST { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CONST")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::EXTOBJ { fullClassName: i_fullClassName }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EXTOBJ: ")).clone() }))?;
            txt = dotPath(txt.clone(), i_fullClassName.clone())?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::JAC_VAR { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("JACOBIAN_VAR")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::JAC_TMP_VAR { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("JACOBIAN_TMP_VAR")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::SEED_VAR { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SEED_VAR")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::OPT_CONSTR { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OPT_CONSTR")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::OPT_FCONSTR { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OPT_FCONSTR")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::OPT_INPUT_WITH_DER { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OPT_INPUT_WITH_DER")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::OPT_INPUT_DER { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OPT_INPUT_DER")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::OPT_TGRID { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OPT_TGRID")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::OPT_LOOP_INPUT { replaceExp: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OPT_LOOP_INPUT")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::ALG_STATE { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALG_STATE")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::DAE_RESIDUAL_VAR { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("DAE_RESIDUAL_VAR")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::DAE_AUX_VAR { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("DAE_AUX_VAR")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::LOOP_ITERATION { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("LOOP_ITERATION")).clone() }))?;
            txt.clone()
        },
        (txt, BackendDAE::VarKind::LOOP_SOLVED { .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("LOOP_SOLVED")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#UNKNOWN_VARKIND")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn getAliasVar(mut in_txt: Tpl::Text, mut in_a_aliasvar: SimCodeVar::AliasVariable) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_aliasvar.clone()) {
        (mut txt, SimCodeVar::AliasVariable::NOALIAS { .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"noAlias\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, SimCodeVar::AliasVariable::ALIAS { varName: ref i_varName }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"alias\" aliasVariable=\"")).clone() }))?;
            txt = crefStrNoUnderscore(txt.clone(), i_varName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, SimCodeVar::AliasVariable::NEGATEDALIAS { varName: ref i_varName }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"negatedAlias\" aliasVariable=\"")).clone() }))?;
            txt = crefStrNoUnderscore(txt.clone(), i_varName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"noAlias\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dotPath(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::QUALIFIED { name: i_name, path: i_path }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            { (in_txt, in_a_path) = (txt.clone(), i_path.clone()); continue '__tco; }
        },
        (txt, Deref @ Absyn::Path::IDENT { name: i_name_1 }) => {
            let mut txt = (*txt).clone();
            return Ok(Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?)
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

pub fn error(mut txt: Tpl::Text, mut a_srcInfo: SourceInfo, mut a_errMessage: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: ArcStr;
    Tpl::addSourceTemplateError((a_errMessage.clone()).clone(), a_srcInfo.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("#error \"")).clone()], lastHasNewLine: false }))?;
    ret_0 = (Error::infoStr(a_srcInfo.clone())?).clone();
    out_txt = Tpl::writeStr(out_txt.clone(), (ret_0.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_errMessage.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
    Ok(out_txt)
}

pub(crate) fn errorMsg(mut txt: Tpl::Text, mut a_errMessage: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    Tpl::addTemplateError((a_errMessage.clone()).clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("#error \"")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_errMessage.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), openmodelica_susan::Tpl::StringToken::interned_ST_NEW_LINE())?;
    Ok(out_txt)
}

fn fun_73(mut in_txt: Tpl::Text, mut in_a_language: ArcStr, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_language.clone(), in_a_name.clone())) {
        (txt, Deref @ "BUILTIN", a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ "C", a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ "FORTRAN 77", a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt.clone()
        },
        (txt, i_language, _) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Unsupported external language: ")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (i_language.clone()).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenUtil.tpl")).clone(), 375, 14), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn extFunctionName(mut txt: Tpl::Text, mut a_name: ArcStr, mut a_language: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_73(txt.clone(), (a_language.clone()).clone(), (a_name.clone()).clone())?;
    Ok(out_txt)
}

