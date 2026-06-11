// Auto-generated from MetaModelica source
/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF AGPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GNU AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium)
 * Public License (OSMC-PL) are obtained from OSMC, either from the above
 * address, from the URLs:
 * http://www.openmodelica.org or
 * https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica,
 * and in the OpenModelica distribution.
 *
 * GNU AGPL version 3 is obtained from:
 * https://www.gnu.org/licenses/licenses.html#GPL
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInst as Inst;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Variability;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTypeCheck::MatchKind;
use crate::NFTyping as Typing;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NFFunctionDerivative {
    pub derivativeFn: Arc<InstNode::InstNode>,
    pub derivedFn: Arc<InstNode::InstNode>,
    /// Is evaluated to a literal Integer during typing
    pub order: Arc<Expression::NFExpression>,
    pub conditions: Arc<metamodelica::List<(i32, ArcStr, Condition)>>,
    pub lowerOrderDerivatives: Arc<metamodelica::List<Arc<InstNode::InstNode>>>,
}

impl metamodelica::gc::MMTrace for NFFunctionDerivative {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.derivativeFn, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.derivedFn, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.order, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.conditions, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lowerOrderDerivatives, __mmv)?;
        Ok(())
    }
}
impl Default for NFFunctionDerivative {
    fn default() -> Self {
        Self {
            derivativeFn: Default::default(),
            derivedFn: Default::default(),
            order: Default::default(),
            conditions: Default::default(),
            lowerOrderDerivatives: Default::default(),
        }
    }
}

pub type FUNCTION_DER = NFFunctionDerivative;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum Condition {
    ZERO_DERIVATIVE = 1,
    NO_DERIVATIVE = 2,
}
impl PartialOrd for Condition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Condition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl metamodelica::gc::MMTrace for Condition {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> { Ok(()) }
}
impl Default for Condition {
    fn default() -> Self { Self::ZERO_DERIVATIVE }
}

pub(crate) fn instDerivatives(mut fnNode: Arc<InstNode::InstNode>, mut r#fn: Arc<Function::Function>) -> Result<Arc<metamodelica::List<Arc<NFFunctionDerivative>>>> {
    let mut ders: Arc<metamodelica::List<Arc<NFFunctionDerivative>>> = metamodelica::nil();
    let mut der_mods: Arc<metamodelica::List<Arc<SCode::Mod>>>;
    let mut scope: Arc<InstNode::InstNode>;
    der_mods = getDerivativeAnnotations(InstNode::definition(fnNode.clone())?)?;
    scope = InstNode::parent(fnNode.clone());
    for mut m in &*der_mods {
        let mut m = m.clone();
        ders = instDerivativeMod(m.clone(), fnNode.clone(), r#fn.clone(), scope.clone(), ders.clone())?;
    }
    Ok(ders)
}

pub(crate) fn typeDerivative(mut fnDer: Arc<NFFunctionDerivative>) -> Result<()> {
    let mut mk: MatchKind;
    let mut order: Arc<Expression::NFExpression>;
    let mut order_ty: Arc<Type::NFType>;
    let mut var: Variability;
    let mut info: SourceInfo;
    Function::typeNodeCache(fnDer.derivativeFn.clone(), InstContext::FUNCTION.clone())?;
    info = InstNode::info(fnDer.derivedFn.clone());
    (order, order_ty, var, _) = Typing::typeExp(fnDer.order.clone(), InstContext::FUNCTION.clone(), info.clone(), false)?;
    (order, _, mk) = TypeCheck::matchTypes(order_ty.clone(), crate::NFType::interned_INTEGER(), order, TypeCheck::DEFAULT_OPTIONS.clone())?;
    if TypeCheck::isIncompatibleMatch(mk) {
        Error::addSourceMessage(Error::VARIABLE_BINDING_TYPE_MISMATCH.clone(), list![(literal!("order")).clone(), (Expression::toString(order.clone())?).clone(), (literal!("Integer")).clone(), (Type::toString(order_ty)?).clone()], info.clone())?;
        bail!("fail");
    }
    if var > Variability::CONSTANT.clone() {
        Error::addSourceMessage(Error::HIGHER_VARIABILITY_BINDING.clone(), list![(literal!("order")).clone(), (Prefixes::variabilityString(Variability::CONSTANT.clone())?).clone(), (Expression::toString(order.clone())?).clone(), (Prefixes::variabilityString(var)?).clone()], info.clone())?;
        bail!("fail");
    }
    order = Ceval::evalExp(order, Ceval::EvalTarget::new(info, InstContext::NO_CONTEXT.clone(), None))?;
    Ok(())
}

pub(crate) fn toDAE(mut fnDer: Arc<NFFunctionDerivative>) -> Result<DAE::FunctionDefinition> {
    let mut derDef: DAE::FunctionDefinition;
    let mut order: i32;
    let __pa0 = ::match_deref::match_deref! { match &(fnDer.order.clone()) {
        Deref @ Expression::INTEGER { value: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    order = __pa0.clone();
    derDef = DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivedFunction: Function::name(listHead(Function::getCachedFuncs(fnDer.derivedFn.clone())?)?), derivativeFunction: Function::name(listHead(Function::getCachedFuncs(fnDer.derivativeFn.clone())?)?), derivativeOrder: order, conditionRefs: ({
        let mut __acc: Arc<metamodelica::List<(i32, DAE::derivativeCond)>> = metamodelica::nil();
        for mut c in (fnDer.conditions.clone()).into_iter().cloned() {
            let __x = conditionToDAE(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), defaultDerivative: None, lowerOrderDerivatives: ({
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
        for mut r#fn in (fnDer.lowerOrderDerivatives.clone()).into_iter().cloned() {
            let __x = Function::name(listHead(Function::getCachedFuncs(r#fn.clone())?)?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) };
    Ok(derDef)
}

pub(crate) fn conditionToDAE(mut cond: (i32, ArcStr, Condition)) -> Result<(i32, DAE::derivativeCond)> {
    let mut daeCond: (i32, DAE::derivativeCond);
    let mut idx: i32;
    let mut c: Condition;
    (idx, _, c) = cond;
    daeCond = (match c {
        Condition::ZERO_DERIVATIVE => (idx, openmodelica_frontend_types::DAE::derivativeCond::ZERO_DERIVATIVE),
        Condition::NO_DERIVATIVE { .. } => (idx, DAE::derivativeCond::NO_DERIVATIVE { binding: Arc::new(DAE::Exp::ICONST { integer: 99 }) }),
    });
    Ok(daeCond)
}

pub(crate) fn toSubMod(mut fnDer: Arc<NFFunctionDerivative>) -> Result<Arc<SCode::SubMod>> {
    let mut subMod: Arc<SCode::SubMod>;
    let mut tpl: (i32, Condition) = (0, Condition::ZERO_DERIVATIVE);
    let mut condition: Condition;
    let mut id: ArcStr;
    let mut r#mod: Arc<SCode::Mod>;
    let mut orderMod: Arc<SCode::SubMod>;
    let mut subMods: Arc<metamodelica::List<Arc<SCode::SubMod>>>;
    let mut order: i32;
    let mut info: SourceInfo;
    let mut func: Arc<Function::Function>;
    info = InstNode::info(fnDer.derivedFn.clone());
    let __pa0 = ::match_deref::match_deref! { match &(fnDer.order.clone()) {
        Deref @ Expression::INTEGER { value: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    order = __pa0.clone();
    orderMod = Arc::new(SCode::SubMod { ident: (literal!("order")).clone(), r#mod: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::INTEGER { value: order })), comment: None, info: info.clone() }) });
    subMods = metamodelica::nil();
    for mut tpl in &*fnDer.conditions.clone() {
        let mut tpl = tpl.clone();
        (_, id, condition) = tpl.clone();
        subMods = metamodelica::cons(Arc::new(SCode::SubMod { ident: (conditionToString(condition)).clone(), r#mod: Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::nil(), binding: Some(Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (id.clone()).clone(), subscripts: metamodelica::nil() }) })), comment: None, info: info.clone() }) }), subMods.clone());
    }
    func = listHead(Function::getCachedFuncs(fnDer.derivativeFn.clone())?)?;
    r#mod = Arc::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: metamodelica::cons(orderMod, subMods), binding: Some(Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (AbsynUtil::pathString(func.path.clone(), (literal!(".")).clone(), true, false)?).clone(), subscripts: metamodelica::nil() }) })), comment: None, info: info });
    subMod = Arc::new(SCode::SubMod { ident: (literal!("derivative")).clone(), r#mod: r#mod });
    Ok(subMod)
}

pub(crate) fn perfectFit(mut fnDer: Arc<NFFunctionDerivative>, mut interface_map: Arc<UnorderedMap::UnorderedMap<ArcStr, bool>>) -> Result<bool> {
    let mut b: bool = true;
    let mut name: ArcStr;
    let mut cond: Condition;
    for mut condition in &*fnDer.conditions.clone() {
        let mut condition = condition.clone();
        (_, name, cond) = condition.clone();
        if cond == Condition::ZERO_DERIVATIVE.clone() && !(UnorderedMap::contains((name.clone()).clone(), interface_map.clone())?) {
            b = false;
            return Ok(b.clone());
        }
    }
    for mut condition in &*fnDer.conditions.clone() {
        let mut condition = condition.clone();
        (_, name, _) = condition.clone();
        UnorderedMap::add((name.clone()).clone(), true, interface_map.clone())?;
    }
    Ok(b)
}

pub fn conditionsFromMap(mut interface_map: Arc<UnorderedMap::UnorderedMap<ArcStr, bool>>) -> Arc<metamodelica::List<(i32, ArcStr, Condition)>> {
    let mut conditions: Arc<metamodelica::List<(i32, ArcStr, Condition)>> = metamodelica::nil();
    let mut name: ArcStr;
    let mut isZeroDer: bool;
    for mut tpl in &*UnorderedMap::toList(interface_map) {
        let mut tpl = tpl.clone();
        (name, isZeroDer) = tpl.clone();
        if isZeroDer {
            conditions = metamodelica::cons((0, name.clone(), Condition::ZERO_DERIVATIVE.clone()), conditions.clone());
        }
    }
    conditions
}

fn conditionToString(mut condition: Condition) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = ((match condition {
        Condition::NO_DERIVATIVE { .. } => literal!("noDerivative"),
        Condition::ZERO_DERIVATIVE => literal!("zeroDerivative"),
        _ => ArcStr::from(::std::format!("{:?}", condition)),
    })).clone();
    r#str
}

fn getDerivativeAnnotations(mut definition: Arc<SCode::Element>) -> Result<Arc<metamodelica::List<Arc<SCode::Mod>>>> {
    let mut derMods: Arc<metamodelica::List<Arc<SCode::Mod>>>;
    derMods = (::match_deref::match_deref! { match &(definition) {
        Deref @ SCode::Element::CLASS { cmt: Deref @ SCode::Comment { annotation_: Some(ann), .. }, .. } => {
            SCodeUtil::lookupAnnotations(ann.clone(), (literal!("derivative")).clone())?
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(derMods)
}

fn instDerivativeMod(mut r#mod: Arc<SCode::Mod>, mut fnNode: Arc<InstNode::InstNode>, mut r#fn: Arc<Function::Function>, mut scope: Arc<InstNode::InstNode>, mut fnDers: Arc<metamodelica::List<Arc<NFFunctionDerivative>>>) -> Result<Arc<metamodelica::List<Arc<NFFunctionDerivative>>>> {
    let mut fnDers: Arc<metamodelica::List<Arc<NFFunctionDerivative>>> = fnDers;
    fnDers = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { subModLst: attrs, binding: Some(Deref @ Absyn::Exp::CREF { componentRef: acref }), .. } => {
            let mut der_node: Arc<InstNode::InstNode>;
            let mut order: Arc<Expression::NFExpression>;
            let mut conds: Arc<metamodelica::List<(i32, ArcStr, Condition)>>;
            (_, der_node, _) = Function::instFunction(acref.clone(), scope, InstContext::NO_CONTEXT.clone(), var_field!((*r#mod).info, SCode::Mod::MOD).clone())?;
            addLowerOrderDerivative(der_node.clone(), fnNode.clone())?;
            (order, conds) = getDerivativeAttributes(attrs.clone(), r#fn, fnNode.clone(), var_field!((*r#mod).info, SCode::Mod::MOD).clone())?;
            metamodelica::cons(Arc::new(NFFunctionDerivative { derivativeFn: der_node.clone(), derivedFn: fnNode, order: order.clone(), conditions: conds.clone(), lowerOrderDerivatives: metamodelica::nil() }), fnDers)
        },
        Deref @ SCode::Mod::MOD { .. } => {
            Error::addStrictMessage(Error::MISSING_FUNCTION_DERIVATIVE_NAME.clone(), list![(AbsynUtil::pathString(Function::name(r#fn), (literal!(".")).clone(), true, false)?).clone()], var_field!((*r#mod).info, SCode::Mod::MOD).clone())?;
            fnDers
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFunctionDerivative.instDerivativeMod")); __mm_s.push_str(&*literal!(" got invalid modifier")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFFunctionDerivative.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(fnDers)
}

fn getDerivativeAttributes(mut attrs: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut r#fn: Arc<Function::Function>, mut scope: Arc<InstNode::InstNode>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<metamodelica::List<(i32, ArcStr, Condition)>>)> {
    let mut order: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::EMPTY { ty: crate::NFType::interned_UNKNOWN() });
    let mut conditions: Arc<metamodelica::List<(i32, ArcStr, Condition)>> = metamodelica::nil();
    let mut id: ArcStr;
    let mut r#mod: Arc<SCode::Mod>;
    let mut aexp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut index: i32 = 0;
    for mut attr in &*attrs {
        let mut attr = attr.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(attr.clone()) {
            Deref @ SCode::SubMod { ident: __pa0, r#mod: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        id = __pa0.clone();
        r#mod = __pa1.clone();
        let () = (::match_deref::match_deref! { match &((id.clone(), r#mod.clone())) {
        (Deref @ "order", Deref @ SCode::Mod::MOD { binding: Some(__esc_aexp), .. }) => {
            aexp = (*__esc_aexp).clone();
            if !(Expression::isEmpty(order.clone())) {
                Error::addSourceMessage(Error::DUPLICATE_MODIFICATIONS.clone(), list![(id.clone()).clone(), (literal!("derivative")).clone()], info.clone())?;
            }
            order = Inst::instExp(aexp.clone(), scope.clone(), InstContext::NO_CONTEXT.clone(), info.clone())?;
            ()
        },
        (Deref @ "noDerivative", Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: __esc_id, .. } }), .. }) => {
            id = (*__esc_id).clone();
            index = getInputIndex((id.clone()).clone(), r#fn.clone(), info.clone())?;
            conditions = metamodelica::cons((index, id.clone(), Condition::NO_DERIVATIVE.clone()), conditions.clone());
            ()
        },
        (Deref @ "zeroDerivative", Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: __esc_id, .. } }), .. }) => {
            id = (*__esc_id).clone();
            index = getInputIndex((id.clone()).clone(), r#fn.clone(), info.clone())?;
            conditions = metamodelica::cons((index, id.clone(), Condition::ZERO_DERIVATIVE.clone()), conditions.clone());
            ()
        },
        _ => {
            Error::addStrictMessage(Error::INVALID_FUNCTION_ANNOTATION_ATTR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*if (SCodeUtil::isEmptyMod(r#mod.clone())) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*SCodeDump::printModStr(r#mod.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) }}); ArcStr::from(__mm_s) }).clone(), (literal!("derivative")).clone()], info.clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    if Expression::isEmpty(order.clone()) {
        order = Arc::new(Expression::NFExpression::INTEGER { value: 1 });
    }
    Ok((order, conditions))
}

fn getInputIndex(mut name: ArcStr, mut r#fn: Arc<Function::Function>, mut info: SourceInfo) -> Result<i32> {
    let mut index: i32 = 1;
    for mut i in &*r#fn.inputs.clone() {
        let mut i = i.clone();
        if InstNode::name(i.clone())? == name.clone() {
            return Ok(index.clone());
        }
        index = index + 1;
    }
    Error::addSourceMessage(Error::INVALID_FUNCTION_ANNOTATION_INPUT.clone(), list![(name).clone(), (AbsynUtil::pathString(Function::name(r#fn), (literal!(".")).clone(), true, false)?).clone()], info)?;
    bail!("fail");
    Ok(index)
}

fn addLowerOrderDerivative(mut fnNode: Arc<InstNode::InstNode>, mut lowerDerNode: Arc<InstNode::InstNode>) -> Result<()> {
    Function::mapCachedFuncs(fnNode, (std::sync::Arc::new({ let __pe_b1 = lowerDerNode; move |__pe_a0| addLowerOrderDerivative2(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Function::Function>) -> Result<Arc<Function::Function>> + 'static>))?;
    Ok(())
}

fn addLowerOrderDerivative2(mut r#fn: Arc<Function::Function>, mut lowerDerNode: Arc<InstNode::InstNode>) -> Result<Arc<Function::Function>> {
    let mut r#fn: Arc<Function::Function> = r#fn;
    assign_field!(r#fn.derivatives = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFFunctionDerivative>>> = metamodelica::nil();
        for mut fn_der in (r#fn.derivatives.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(fn_der.clone()) {
        Deref @ NFFunctionDerivative { .. } => {
            assign_field!(fn_der.lowerOrderDerivatives = metamodelica::cons(lowerDerNode.clone(), fn_der.lowerOrderDerivatives.clone()));
            fn_der.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
    Ok(r#fn)
}


