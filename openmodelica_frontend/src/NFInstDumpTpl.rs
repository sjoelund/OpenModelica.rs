// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::NFInstDump;
use crate::NFInstTypes;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_dump::AbsynDumpTpl;
use openmodelica_frontend_dump::ExpressionDumpTpl;
use openmodelica_frontend_inst::NFInstPrefix;
use openmodelica_frontend_types::DAE;
use openmodelica_tpl::Tpl;

pub(crate) fn dumpModel(mut txt: Tpl::Text, mut a_name: ArcStr, mut a_cls: Arc<NFInstTypes::Class>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("class ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt, (a_name.clone()).clone())?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = dumpClass(out_txt, a_cls)?;
    out_txt = Tpl::softNewLine(out_txt)?;
    out_txt = Tpl::writeTok(out_txt, Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt, (a_name).clone())?;
    Ok(out_txt)
}

pub(crate) fn dumpComponent(mut in_txt: Tpl::Text, mut in_a_component: Arc<NFInstTypes::Component>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_component)) {
        (txt, i_component @ Deref @ NFInstTypes::Component::UNTYPED_COMPONENT { name: i_name, binding: i_binding, baseType: i_baseType, .. }) => {
            let mut ret_4: ArcStr;
            let mut l_dims__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut l_bind__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_name.clone())?;
            l_bind__str = dumpBinding(Tpl::emptyTxt.clone(), i_binding.clone())?;
            l_ty__str = ExpressionDumpTpl::dumpType(Tpl::emptyTxt.clone(), i_baseType.clone())?;
            ret_4 = (NFInstDump::dumpUntypedComponentDims(i_component.clone())?).clone();
            l_dims__str = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_4).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str)?;
            txt = Tpl::writeText(txt.clone(), l_dims__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__str)?;
            txt = Tpl::writeText(txt.clone(), l_bind__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ NFInstTypes::Component::TYPED_COMPONENT { name: i_name, binding: i_binding, ty: i_ty, .. }) => {
            let mut l_ty__str: Tpl::Text;
            let mut l_bind__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_name.clone())?;
            l_bind__str = dumpBinding(Tpl::emptyTxt.clone(), i_binding.clone())?;
            l_ty__str = ExpressionDumpTpl::dumpType(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ty__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__str)?;
            txt = Tpl::writeText(txt.clone(), l_bind__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ NFInstTypes::Component::CONDITIONAL_COMPONENT { name: i_name, .. }) => {
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("conditional ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ NFInstTypes::Component::DELETED_COMPONENT { name: i_name }) => {
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("deleted ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ NFInstTypes::Component::OUTER_COMPONENT { innerName: Some(i_in), name: i_name }) => {
            let mut l_inner__str: Tpl::Text;
            let mut l_outer__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_outer__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_name.clone())?;
            l_inner__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_in.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("outer ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_outer__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -> ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_inner__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ NFInstTypes::Component::OUTER_COMPONENT { name: i_name, .. }) => {
            let mut l_outer__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_outer__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("outer ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_outer__str)?;
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

fn fun_11(mut in_txt: Tpl::Text, mut in_a_cls__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_cls__str)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn dumpElement(mut in_txt: Tpl::Text, mut in_a_element: Arc<NFInstTypes::Element>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_element)) {
        (txt, Deref @ NFInstTypes::Element::ELEMENT { component: i_component, cls: i_cls }) => {
            let mut l_sep__str: Tpl::Text;
            let mut l_cls__str: Tpl::Text;
            let mut l_comp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_comp__str = dumpComponent(Tpl::emptyTxt.clone(), i_component.clone())?;
            l_cls__str = dumpClass(Tpl::emptyTxt.clone(), i_cls.clone())?;
            l_sep__str = fun_11(Tpl::emptyTxt.clone(), l_cls__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_comp__str)?;
            txt = Tpl::writeText(txt.clone(), l_sep__str)?;
            txt = Tpl::writeText(txt.clone(), l_cls__str)?;
            txt.clone()
        },
        (txt, Deref @ NFInstTypes::Element::CONDITIONAL_ELEMENT { component: i_component }) => {
            let mut l_comp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_comp__str = dumpComponent(Tpl::emptyTxt.clone(), i_component.clone())?;
            txt = Tpl::writeText(txt.clone(), l_comp__str)?;
            txt.clone()
        },
        (txt, Deref @ NFInstTypes::Element::EXTENDED_ELEMENTS { cls: i_cls, .. }) => {
            let mut l_cls__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cls__str = dumpClass(Tpl::emptyTxt.clone(), i_cls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cls__str)?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_13(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<NFInstTypes::Element>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_13 in &*items {
        let mut lstElt_13 = lstElt_13.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_13.clone()) {
        i_comp => {
            txt = dumpElement(txt.clone(), i_comp.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_14(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<NFInstTypes::Equation>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_14 in &*items {
        let mut lstElt_14 = lstElt_14.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_14.clone()) {
        i_ieq => {
            txt = dumpEquation(txt.clone(), i_ieq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn lm_15(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<NFInstTypes::Equation>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_15 in &*items {
        let mut lstElt_15 = lstElt_15.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_15.clone()) {
        i_eq => {
            txt = dumpEquation(txt.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

fn fun_16(mut in_txt: Tpl::Text, mut in_a_comp__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_comp__str)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_comp__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), i_comp__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_17(mut in_txt: Tpl::Text, mut in_a_eq__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_eq__str)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end equation;")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_18(mut in_txt: Tpl::Text, mut in_a_ieq__str: Tpl::Text, mut in_a_eq__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_ieq__str, in_a_eq__str)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, i_ieq__str, a_eq__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("initial equation\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), i_ieq__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = fun_17(txt.clone(), a_eq__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_19(mut in_txt: Tpl::Text, mut in_a_eq__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_eq__str)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_eq__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("equation\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), i_eq__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end equation;")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn dumpClass(mut in_txt: Tpl::Text, mut in_a_cls: Arc<NFInstTypes::Class>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_cls)) {
        (txt, Deref @ NFInstTypes::Class::COMPLEX_CLASS { components: i_components, initialEquations: i_initialEquations, equations: i_equations, .. }) => {
            let mut l_eq__seq__str: Tpl::Text;
            let mut l_ieq__seq__str: Tpl::Text;
            let mut l_comp__seq__str: Tpl::Text;
            let mut l_eq__str: Tpl::Text;
            let mut l_ieq__str: Tpl::Text;
            let mut l_comp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_comp__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_comp__str = lm_13(l_comp__str, i_components.clone())?;
            l_comp__str = Tpl::popIter(l_comp__str)?;
            l_ieq__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_ieq__str = lm_14(l_ieq__str, i_initialEquations.clone())?;
            l_ieq__str = Tpl::popIter(l_ieq__str)?;
            l_eq__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_eq__str = lm_15(l_eq__str, i_equations.clone())?;
            l_eq__str = Tpl::popIter(l_eq__str)?;
            l_comp__seq__str = fun_16(Tpl::emptyTxt.clone(), l_comp__str)?;
            l_ieq__seq__str = fun_18(Tpl::emptyTxt.clone(), l_ieq__str, l_eq__str.clone())?;
            l_eq__seq__str = fun_19(Tpl::emptyTxt.clone(), l_eq__str)?;
            txt = Tpl::writeText(txt.clone(), l_comp__seq__str)?;
            txt = Tpl::writeText(txt.clone(), l_ieq__seq__str)?;
            txt = Tpl::writeText(txt.clone(), l_eq__seq__str)?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn dumpExp(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = ExpressionDumpTpl::dumpExp(txt, a_exp, (literal!("\"")).clone())?;
    Ok(out_txt)
}

fn fun_22(mut in_txt: Tpl::Text, mut in_a_range: Option<Arc<DAE::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_range)) {
        (txt, Some(i_range__exp)) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("in ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_range__exp.clone())?;
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

fn lm_23(mut txt: Tpl::Text, mut items: Arc<metamodelica::List<Arc<NFInstTypes::Equation>>>) -> Result<Tpl::Text> {
    let mut txt: Tpl::Text = txt;
    for mut lstElt_23 in &*items {
        let mut lstElt_23 = lstElt_23.clone();
        txt = (::match_deref::match_deref! { match &(lstElt_23.clone()) {
        i_eq => {
            txt = dumpEquation(txt.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(txt)
}

pub(crate) fn dumpEquation(mut in_txt: Tpl::Text, mut in_a_equation: Arc<NFInstTypes::Equation>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_equation)) {
        (txt, Deref @ NFInstTypes::Equation::EQUALITY_EQUATION { lhs: i_lhs, rhs: i_rhs, .. }) => {
            let mut ret_5: Arc<DAE::Type>;
            let mut l_rhs__ty__str: Tpl::Text;
            let mut ret_3: Arc<DAE::Type>;
            let mut l_lhs__ty__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpExp(Tpl::emptyTxt.clone(), i_lhs.clone())?;
            l_rhs__str = dumpExp(Tpl::emptyTxt.clone(), i_rhs.clone())?;
            ret_3 = Expression::r#typeof(i_lhs.clone())?;
            l_lhs__ty__str = ExpressionDumpTpl::dumpType(Tpl::emptyTxt.clone(), ret_3)?;
            ret_5 = Expression::r#typeof(i_rhs.clone())?;
            l_rhs__ty__str = ExpressionDumpTpl::dumpType(Tpl::emptyTxt.clone(), ret_5)?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_lhs__ty__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__ty__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ NFInstTypes::Equation::FOR_EQUATION { indexType: i_indexType, range: i_range, body: i_body, name: i_name, index: i_index, .. }) => {
            let mut l_eql__str: Tpl::Text;
            let mut l_range__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ty__str = ExpressionDumpTpl::dumpType(Tpl::emptyTxt.clone(), i_indexType.clone())?;
            l_range__str = fun_22(Tpl::emptyTxt.clone(), i_range.clone())?;
            l_eql__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()), alignNum: 0, alignOfset: 0, alignSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE(), wrapWidth: 0, wrapSeparator: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }))?;
            l_eql__str = lm_23(l_eql__str, i_body.clone())?;
            l_eql__str = Tpl::popIter(l_eql__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" /* index ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_range__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" loop\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_eql__str)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end for;")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ NFInstTypes::Equation::IF_EQUATION { branches: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if equation;")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ NFInstTypes::Equation::ASSERT_EQUATION { condition: i_condition, message: i_message, .. }) => {
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cond__str = dumpExp(Tpl::emptyTxt.clone(), i_condition.clone())?;
            l_msg__str = dumpExp(Tpl::emptyTxt.clone(), i_message.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("assert(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_msg__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ NFInstTypes::Equation::TERMINATE_EQUATION { message: i_message, .. }) => {
            let mut l_msg__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_msg__str = dumpExp(Tpl::emptyTxt.clone(), i_message.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("terminate(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_msg__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ NFInstTypes::Equation::REINIT_EQUATION { cref: i_cref, reinitExp: i_reinitExp, .. }) => {
            let mut l_exp__str: Tpl::Text;
            let mut l_cref__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cref__str = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_cref.clone())?;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_reinitExp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("reinit(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cref__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ NFInstTypes::Equation::NORETCALL_EQUATION { exp: i_exp, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("dumpEquation: IMPLEMENT ME")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn dumpBinding(mut in_txt: Tpl::Text, mut in_a_binding: NFInstTypes::Binding) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_binding) {
        (mut txt, NFInstTypes::Binding::RAW_BINDING { bindingExp: ref i_aexp, .. }) => {
            let mut l_exp__str: Tpl::Text;
            l_exp__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_aexp.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("= <RAW> ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, NFInstTypes::Binding::UNTYPED_BINDING { bindingExp: ref i_bindingExp, .. }) => {
            let mut l_exp__str: Tpl::Text;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_bindingExp.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("= ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, NFInstTypes::Binding::TYPED_BINDING { bindingExp: ref i_bindingExp, bindingType: ref i_bindingType, .. }) => {
            let mut l_ty__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_bindingExp.clone())?;
            l_ty__str = ExpressionDumpTpl::dumpType(Tpl::emptyTxt.clone(), i_bindingType.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("= (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str)?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_26(mut in_txt: Tpl::Text, mut in_a_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_dims)) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_dims) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = ExpressionDumpTpl::dumpDimensions(txt.clone(), i_dims.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_27(mut in_txt: Tpl::Text, mut in_a_rest__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_rest__str)) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_rest__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_rest__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn dumpPrefix(mut in_txt: Tpl::Text, mut in_a_prefix: Arc<NFInstPrefix::Prefix>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt, in_a_prefix)) {
        (txt, Deref @ NFInstPrefix::Prefix::PREFIX { dims: i_dims, restPrefix: i_restPrefix, name: i_name }) => {
            let mut l_pre__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_dims__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_dims__str = fun_26(Tpl::emptyTxt.clone(), i_dims.clone())?;
            l_rest__str = dumpPrefix(Tpl::emptyTxt.clone(), i_restPrefix.clone())?;
            l_pre__str = fun_27(Tpl::emptyTxt.clone(), l_rest__str)?;
            txt = Tpl::writeText(txt.clone(), l_pre__str)?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_dims__str)?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub(crate) fn dumpDimension(mut in_txt: Tpl::Text, mut in_a_dim: NFInstTypes::Dimension) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt, in_a_dim) {
        (mut txt, NFInstTypes::Dimension::UNTYPED_DIMENSION { dimension: ref i_dimension, .. }) => {
            txt = ExpressionDumpTpl::dumpDimension(txt.clone(), i_dimension.clone())?;
            txt.clone()
        },
        (mut txt, NFInstTypes::Dimension::TYPED_DIMENSION { dimension: ref i_dimension }) => {
            txt = ExpressionDumpTpl::dumpDimension(txt.clone(), i_dimension.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub(crate) fn errorMsg(mut txt: Tpl::Text, mut a_errMessage: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    Tpl::addTemplateError((a_errMessage.clone()).clone())?;
    out_txt = Tpl::writeStr(txt, (a_errMessage).clone())?;
    Ok(out_txt)
}

