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

use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFClockKind;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEvalFunction as EvalFunction;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFOperator::Op;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFTyping as Typing;
use crate::NFTyping::TypingError;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_util::Error;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub mod EvalTarget {
    use super::*;
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct EvalTarget {
        pub info: SourceInfo,
        pub context: i32,
        pub extra: Option<Arc<EvalTargetData>>,
    }

    impl metamodelica::gc::MMTrace for EvalTarget {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.info, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.context, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.extra, __mmv)?;
            Ok(())
        }
    }
    impl Default for EvalTarget {
        fn default() -> Self {
            Self {
                info: Default::default(),
                context: Default::default(),
                extra: Default::default(),
            }
        }
    }

    pub type EVAL_TARGET = EvalTarget;

    pub fn new(mut info: SourceInfo, mut context: i32, mut extra: Option<Arc<EvalTargetData>>) -> Arc<EvalTarget> {
        let mut target: Arc<EvalTarget> = Arc::new(EvalTarget { info: info.clone(), context: context, extra: extra.clone() });
        target
    }

    pub(crate) fn hasInfo(mut target: Arc<EvalTarget>) -> bool {
        let mut res: bool = !(stringEmpty(target.info.fileName.clone()));
        res
    }

    pub(crate) fn getInfo(mut target: Arc<EvalTarget>) -> SourceInfo {
        let mut info: SourceInfo = target.info.clone();
        info
    }

}

thread_local! { static __noTarget_TLS: Arc<EvalTarget::EvalTarget> = Arc::new(EvalTarget::EvalTarget { info: Absyn::dummyInfo.clone(), context: InstContext::NO_CONTEXT.clone(), extra: None }); }
pub fn noTarget() -> Arc<EvalTarget::EvalTarget> { __noTarget_TLS.with(|__t| __t.clone()) }

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct EvalTargetData {
    pub component: Arc<InstNode::InstNode>,
    pub index: i32,
    pub exp: Arc<Expression::NFExpression>,
}

impl metamodelica::gc::MMTrace for EvalTargetData {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.component, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.exp, __mmv)?;
        Ok(())
    }
}
impl Default for EvalTargetData {
    fn default() -> Self {
        Self {
            component: Default::default(),
            index: Default::default(),
            exp: Default::default(),
        }
    }
}

pub type DIMENSION_DATA = EvalTargetData;


pub(crate) fn tryEvalExpResizable(mut exp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    ErrorExt::setCheckpoint(literal!("NFCeval.tryEvalExpResizable"));
    match '__try0: {
        exp = unwrap_break_err!(evalExp(exp.clone(), target.clone()), '__try0);
        ErrorExt::delCheckpoint(literal!("NFCeval.tryEvalExpResizable"));
        Ok::<_, anyhow::Error>((exp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            exp = __try0_o0;
        }
        Err(_) => {
            exp = tryEvalExpPartial(exp.clone(), target.clone());
            if Expression::contains(exp.clone(), (std::sync::Arc::new(Expression::isResizableCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))? {
                ErrorExt::rollBack(literal!("NFCeval.tryEvalExpResizable"));
            } else {
                ErrorExt::delCheckpoint(literal!("NFCeval.tryEvalExpResizable"));
                bail!("fail");
            }
        }
    }
    Ok(exp)
}

pub fn tryEvalExp(mut exp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    ErrorExt::setCheckpoint(literal!("NFCeval.tryEvalExp"));
    if '__try0: {
        exp = unwrap_break_err!(evalExp(exp.clone(), target.clone()), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    ErrorExt::rollBack(literal!("NFCeval.tryEvalExp"));
    exp
}

pub fn evalExp(mut exp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            evalCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), exp, target, true, true)?
        },
        Deref @ Expression::TYPENAME { .. } => {
            evalTypename(var_field!((*exp).ty, Expression::NFExpression::TYPENAME).clone(), exp, target)?
        },
        Deref @ Expression::ARRAY { .. } => {
            if (var_field!((*exp).literal, Expression::NFExpression::ARRAY).clone()) {exp} else {Expression::makeArrayCheckLiteral(var_field!((*exp).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1 = target; move |__pe_a0| evalExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?)?}
        },
        Deref @ Expression::RANGE { .. } => {
            evalRange(exp, target)?
        },
        Deref @ Expression::TUPLE { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::TUPLE; elements = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone()).into_iter().cloned() {
            let __x = evalExp(e.clone(), target.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            exp
        },
        Deref @ Expression::RECORD { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::RECORD; elements = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).elements, Expression::NFExpression::RECORD).clone()).into_iter().cloned() {
            let __x = evalExp(e.clone(), target.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            exp
        },
        Deref @ Expression::CALL { .. } => {
            evalCall(var_field!((*exp).call, Expression::NFExpression::CALL).clone(), target)?
        },
        Deref @ Expression::SIZE { .. } => {
            evalSize(var_field!((*exp).exp, Expression::NFExpression::SIZE).clone(), var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone(), target)?
        },
        Deref @ Expression::BINARY { .. } => {
            let mut exp1: Arc<Expression::NFExpression>;
            let mut exp2: Arc<Expression::NFExpression>;
            exp1 = evalExp(var_field!((*exp).exp1, Expression::NFExpression::BINARY).clone(), target.clone())?;
            exp2 = evalExp(var_field!((*exp).exp2, Expression::NFExpression::BINARY).clone(), target.clone())?;
            evalBinaryOp(exp1.clone(), var_field!((*exp).operator, Expression::NFExpression::BINARY).clone(), exp2.clone(), target)?
        },
        Deref @ Expression::UNARY { .. } => {
            let mut exp1: Arc<Expression::NFExpression>;
            exp1 = evalExp(var_field!((*exp).exp, Expression::NFExpression::UNARY).clone(), target)?;
            evalUnaryOp(exp1.clone(), var_field!((*exp).operator, Expression::NFExpression::UNARY).clone())?
        },
        Deref @ Expression::LBINARY { .. } => {
            let mut exp1: Arc<Expression::NFExpression>;
            let mut exp2: Arc<Expression::NFExpression>;
            exp1 = evalExp(var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone(), target.clone())?;
            if Expression::isSplitSubscriptedExp(exp1.clone()) {
                exp2 = evalExp(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), target.clone())?;
            } else {
                exp2 = var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone();
            }
            evalLogicBinaryOp(exp1.clone(), var_field!((*exp).operator, Expression::NFExpression::LBINARY).clone(), exp2.clone(), target)?
        },
        Deref @ Expression::LUNARY { .. } => {
            let mut exp1: Arc<Expression::NFExpression>;
            exp1 = evalExp(var_field!((*exp).exp, Expression::NFExpression::LUNARY).clone(), target)?;
            evalLogicUnaryOp(exp1.clone(), var_field!((*exp).operator, Expression::NFExpression::LUNARY).clone())?
        },
        Deref @ Expression::RELATION { .. } => {
            let mut exp1: Arc<Expression::NFExpression>;
            let mut exp2: Arc<Expression::NFExpression>;
            exp1 = evalExp(var_field!((*exp).exp1, Expression::NFExpression::RELATION).clone(), target.clone())?;
            exp2 = evalExp(var_field!((*exp).exp2, Expression::NFExpression::RELATION).clone(), target)?;
            evalRelationOp(exp1.clone(), var_field!((*exp).operator, Expression::NFExpression::RELATION).clone(), exp2.clone())?
        },
        Deref @ Expression::IF { .. } => {
            evalIfExp(exp, target)?
        },
        Deref @ Expression::CAST { .. } => {
            let mut exp1: Arc<Expression::NFExpression>;
            exp1 = evalExp(var_field!((*exp).exp, Expression::NFExpression::CAST).clone(), target)?;
            evalCast(exp1.clone(), var_field!((*exp).ty, Expression::NFExpression::CAST).clone())?
        },
        Deref @ Expression::BOX { .. } => {
            evalExp(var_field!((*exp).exp, Expression::NFExpression::BOX).clone(), target)?
        },
        Deref @ Expression::UNBOX { .. } => {
            evalExp(var_field!((*exp).exp, Expression::NFExpression::UNBOX).clone(), target)?
        },
        Deref @ Expression::SUBSCRIPTED_EXP { .. } => {
            evalSubscriptedExp(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), var_field!((*exp).subscripts, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), target)?
        },
        Deref @ Expression::TUPLE_ELEMENT { .. } => {
            let mut exp1: Arc<Expression::NFExpression>;
            exp1 = evalExp(var_field!((*exp).tupleExp, Expression::NFExpression::TUPLE_ELEMENT).clone(), target)?;
            Expression::tupleElement(exp1.clone(), var_field!((*exp).ty, Expression::NFExpression::TUPLE_ELEMENT).clone(), var_field!((*exp).index, Expression::NFExpression::TUPLE_ELEMENT).clone())?
        },
        Deref @ Expression::RECORD_ELEMENT { .. } => {
            evalRecordElement(exp, target)?
        },
        Deref @ Expression::MUTABLE { .. } => {
            let mut exp1: Arc<Expression::NFExpression>;
            exp1 = evalExp(Mutable::access(var_field!((*exp).exp, Expression::NFExpression::MUTABLE).clone()), target)?;
            exp1.clone()
        },
        Deref @ Expression::INSTANCE_NAME { .. } => {
            evalGetInstanceName(var_field!((*exp).scope, Expression::NFExpression::INSTANCE_NAME).clone())?
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn tryEvalExpPartial(mut exp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    ErrorExt::setCheckpoint(literal!("NFCeval.tryEvalExpPartial"));
    if '__try0: {
        (exp, _) = unwrap_break_err!(evalExpPartial(exp.clone(), target.clone(), true), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    ErrorExt::rollBack(literal!("NFCeval.tryEvalExpPartial"));
    exp
}

pub(crate) fn evalExpPartialDefault(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    (exp, _) = evalExpPartial(exp, noTarget().clone(), true)?;
    Ok(exp)
}

pub(crate) fn evalExpPartial(mut exp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>, mut evaluated: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outEvaluated: bool;
    let mut e: Arc<Expression::NFExpression>;
    (e, outEvaluated) = Expression::mapFoldShallow(exp, (std::sync::Arc::new({ let __pe_b1 = target.clone(); move |__pe_a0, __pe_a2| evalExpPartial(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, bool) -> Result<(Arc<Expression::NFExpression>, bool)> + 'static>), true)?;
    outExp = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Expression::CREF { .. } => {
            if ComponentRef::isIterator(var_field!((*e).cref, Expression::NFExpression::CREF).clone()) {
                outExp = e;
                outEvaluated = false;
            } else {
                outExp = evalCref(var_field!((*e).cref, Expression::NFExpression::CREF).clone(), e, target, false, true)?;
                outEvaluated = Expression::isLiteral(outExp.clone())?;
            }
            outExp
        },
        Deref @ Expression::MUTABLE { .. } => {
            outEvaluated = false;
            e
        },
        _ => if (outEvaluated) {evalExp(e, target)?} else {e},
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEvaluated = evaluated && outEvaluated;
    Ok((outExp, outEvaluated))
}

pub(crate) fn evalCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut defaultExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>, mut evalSubscripts: bool, mut liftExp: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    let mut c: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    exp = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { node: __esc_c @ Deref @ InstNode::COMPONENT_NODE { .. }, .. } if (!(ComponentRef::isIterator(cref.clone())) && ComponentRef::nodeVariability(cref.clone())? < Variability::NON_STRUCTURAL_PARAMETER.clone()) => {
            c = (*__esc_c).clone();
            evalComponentBinding(c.clone(), cref.clone(), defaultExp, target, evalSubscripts, liftExp)?
        },
        _ => defaultExp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalComponentBinding(mut node: Arc<InstNode::InstNode>, mut cref: Arc<ComponentRef::NFComponentRef>, mut defaultExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>, mut evalSubscripts: bool, mut liftExp: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp_context: i32;
    let mut comp: Arc<Component::NFComponent>;
    let mut binding: Arc<Binding::NFBinding>;
    let mut evaluated: bool;
    let mut start_exp: Option<Arc<Expression::NFExpression>>;
    let mut cref_ty: Arc<Type::NFType>;
    let mut exp_ty: Arc<Type::NFType>;
    let mut dim_diff: i32;
    let mut errors: Arc<metamodelica::List<i32>> = metamodelica::nil();
    exp_context = InstContext::nodeContext(node.clone(), target.context.clone());
    Typing::typeComponentBinding(node.clone(), exp_context, false)?;
    comp = InstNode::component(node.clone())?;
    binding = Component::getBinding(comp.clone());
    if Binding::isUnbound(binding.clone()) {
        binding = makeComponentBinding(comp.clone(), node.clone(), Expression::toCref(defaultExp.clone())?, target.clone());
        if Binding::isUnbound(binding.clone()) {
            start_exp = evalComponentStartBinding(node.clone(), comp.clone(), cref.clone(), target.clone(), evalSubscripts)?;
            if isSome(start_exp.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(start_exp) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa0.clone();
                return Ok(exp.clone());
            }
        }
    }
    (exp, evaluated) = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ Binding::TYPED_BINDING { .. } => {
            exp = (match Mutable::access(var_field!((*binding).evalState, Binding::NFBinding::TYPED_BINDING).clone()) {
        Binding::EvalState::NOT_EVALUATED => {
            Mutable::update(var_field!((*binding).evalState, Binding::NFBinding::TYPED_BINDING).clone(), Binding::EvalState::EVALUATING.clone());
            ErrorExt::setCheckpoint(literal!("NFCeval.evalComponentBinding"));
            match '__try0: {
                exp = unwrap_break_err!(evalExp(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone(), target.clone()), '__try0);
                ErrorExt::delCheckpoint(literal!("NFCeval.evalComponentBinding"));
                Ok::<_, anyhow::Error>((exp.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    exp = __try0_o0;
                }
                Err(__try0_err) => {
                    Mutable::update(var_field!((*binding).evalState, Binding::NFBinding::TYPED_BINDING).clone(), Binding::EvalState::NOT_EVALUATED.clone());
                    errors = ErrorExt::popCheckPoint(literal!("NFCeval.evalComponentBinding"));
                    Error::addSourceMessage(Error::ERROR_FROM_HERE.clone(), metamodelica::nil(), var_field!((*binding).info, Binding::NFBinding::TYPED_BINDING).clone())?;
                    ErrorExt::pushMessages(errors.clone());
                    return Err(__try0_err);
                }
            }
            assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING; bindingExp = exp.clone());
            comp = Component::setBinding(binding.clone(), comp)?;
            InstNode::updateComponent(comp, node)?;
            Mutable::update(var_field!((*binding).evalState, Binding::NFBinding::TYPED_BINDING).clone(), Binding::EvalState::EVALUATED.clone());
            exp
        },
        Binding::EvalState::EVALUATED => var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone(),
        _ => {
            Error::addSourceMessage(Error::CIRCULAR_PARAM.clone(), list![(InstNode::name(node.clone())?).clone(), (Prefixes::variabilityString(Component::variability(comp)?)?).clone()], InstNode::info(node))?;
            bail!("fail")
        },
    });
            (exp, true)
        },
        Deref @ Binding::CEVAL_BINDING { .. } => (var_field!((*binding).bindingExp, Binding::NFBinding::CEVAL_BINDING).clone(), true),
        Deref @ Binding::UNBOUND => {
            printUnboundError(comp, target, defaultExp.clone())?;
            (defaultExp.clone(), false)
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalComponentBinding")); __mm_s.push_str(&*literal!(" failed on untyped binding")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if evaluated {
        exp = subscriptBinding(exp, cref, evalSubscripts)?;
    }
    if liftExp && !(Expression::contains(exp.clone(), (std::sync::Arc::new(fnptr!(Expression::isSplitSubscriptedExp, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?) {
        exp_ty = Expression::typeOf(exp.clone());
        cref_ty = Expression::typeOf(defaultExp);
        dim_diff = Type::dimensionDiff(cref_ty.clone(), exp_ty);
        if dim_diff > 0 {
            (exp, _) = Expression::liftArrayList(List::firstN(Type::arrayDims(cref_ty), dim_diff)?, exp)?;
        }
    }
    Ok(exp)
}

pub(crate) fn subscriptBinding(mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut evalSubscripts: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    subs = ComponentRef::getSubscripts(cref.clone());
    if evalSubscripts {
        subs = ({
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (subs).into_iter().cloned() {
            let __x = Subscript::eval(s.clone(), noTarget().clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    }
    subs = List::trimToLength(subs, Expression::dimensionCount(exp.clone())?)?;
    exp = Expression::applySubscripts(subs, exp, false)?;
    (exp, _) = subscriptBinding2(exp, cref, evalSubscripts, None)?;
    Ok(exp)
}

pub(crate) fn subscriptBinding2(mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut evalSubscripts: bool, mut subMap: Option<Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>>) -> Result<(Arc<Expression::NFExpression>, Option<Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>>)> {
    pub(crate) type SubscriptList = Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;

    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut subMap: Option<Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>> = subMap;
    let mut sub_map: Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> = <Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>> as ::std::default::Default>::default();
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut cref_parts: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    (exp, subMap) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::SUBSCRIPTED_EXP { subscripts: __esc_subs, .. } => {
            subs = (*__esc_subs).clone();
            if isSome(subMap.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(subMap.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                sub_map = __pa0.clone();
            } else {
                cref_parts = ComponentRef::toListReverse(cref.clone(), isFlatCref(cref.clone()), metamodelica::nil());
                sub_map = UnorderedMap::new((std::sync::Arc::new(InstNode::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(InstNode::refEqual, Arc<InstNode::InstNode>, Arc<InstNode::InstNode>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>, Arc<InstNode::InstNode>) -> Result<bool> + 'static>), Util::nextPrime((cref_parts.clone().len() as i32)));
                for mut cr in &*cref_parts {
                    let mut cr = cr.clone();
                    UnorderedMap::addUnique(ComponentRef::node(cr.clone())?, ComponentRef::getSubscripts(cr.clone()), sub_map.clone())?;
                }
                subMap = Some(sub_map.clone());
            }
            subs = ({
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = subscriptBinding3(s.clone(), sub_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            if evalSubscripts {
                subs = ({
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = Subscript::eval(s.clone(), noTarget().clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            }
            (e, subMap) = subscriptBinding2(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), cref, evalSubscripts, subMap)?;
            e = Expression::applySubscripts(subs.clone(), e, false)?;
            (e, subMap)
        },
        Deref @ Expression::ARRAY { literal: true, .. } => (exp, subMap),
        _ => Expression::mapFoldShallow(exp, (std::sync::Arc::new({ let __pe_b1 = cref; let __pe_b2 = evalSubscripts; move |__pe_a0, __pe_a3| subscriptBinding2(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Option<Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>>) -> Result<(Arc<Expression::NFExpression>, Option<Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>>)> + 'static>), subMap)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, subMap))
}

pub(crate) fn isFlatCref(mut cref: Arc<ComponentRef::NFComponentRef>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { origin: ComponentRef::Origin::SCOPE, .. } if (Type::isArray(var_field!((*cref).ty, ComponentRef::NFComponentRef::CREF).clone())) => return !(var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone().is_empty()),
        Deref @ ComponentRef::CREF { .. } => { cref = var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(); continue '__tco; },
        _ => return false,
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub(crate) fn subscriptBinding3(mut subscript: Arc<Subscript::NFSubscript>, mut subMap: Arc<UnorderedMap::UnorderedMap<Arc<InstNode::InstNode>, Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>>>) -> Result<Arc<Subscript::NFSubscript>> {
    let mut outSubscript: Arc<Subscript::NFSubscript> = Arc::new(Subscript::WHOLE);
    let mut osubs: Option<Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>> = None;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    outSubscript = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ Subscript::SPLIT_INDEX { .. } => {
            osubs = UnorderedMap::get(var_field!((*subscript).node, Subscript::NFSubscript::SPLIT_INDEX).clone(), subMap)?;
            if isSome(osubs.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(osubs) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                subs = __pa0.clone();
                if var_field!((*subscript).dimIndex, Subscript::NFSubscript::SPLIT_INDEX).clone() > (subs.clone().len() as i32) {
                    outSubscript = crate::NFSubscript::interned_WHOLE();
                } else {
                    outSubscript = (subs).get(var_field!((*subscript).dimIndex, Subscript::NFSubscript::SPLIT_INDEX).clone())?;
                }
            } else {
                outSubscript = subscript;
            }
            outSubscript
        },
        _ => subscript,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSubscript)
}

pub(crate) fn evalComponentStartBinding(mut node: Arc<InstNode::InstNode>, mut comp: Arc<Component::NFComponent>, mut cref: Arc<ComponentRef::NFComponentRef>, mut target: Arc<EvalTarget::EvalTarget>, mut evalSubscripts: bool) -> Result<Option<Arc<Expression::NFExpression>>> {
    let mut outExp: Option<Arc<Expression::NFExpression>> = None;
    let mut var: Variability;
    let mut start_node: Arc<InstNode::InstNode>;
    let mut start_comp: Arc<Component::NFComponent>;
    let mut binding: Arc<Binding::NFBinding>;
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    var = Component::variability(comp.clone())?;
    if var != Variability::PARAMETER.clone() && var != Variability::STRUCTURAL_PARAMETER.clone() || !(Component::isFixed(comp)?) {
        return Ok(outExp.clone());
    }
    if let Ok(__iflet0) = Class::lookupElement((literal!("start")).clone(), InstNode::getClass(node.clone())?) {
        start_node = __iflet0.0;
    } else {
        return Ok(outExp.clone());
    }
    start_comp = InstNode::component(start_node.clone())?;
    if !(Component::isTypeAttribute(start_comp.clone())) {
        return Ok(outExp.clone());
    }
    binding = Component::getBinding(start_comp.clone());
    outExp = (::match_deref::match_deref! { match &(binding.clone()) {
        Deref @ Binding::TYPED_BINDING { .. } => {
            exp = evalExp(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone(), target)?;
            if !(referenceEq(&*(exp.clone()),&*(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone()))) {
                assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING; bindingExp = exp.clone());
                start_comp = Component::setBinding(binding, start_comp)?;
                InstNode::updateComponent(start_comp, start_node)?;
            }
            Some(exp)
        },
        _ => outExp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn makeComponentBinding(mut component: Arc<Component::NFComponent>, mut node: Arc<InstNode::InstNode>, mut cref: Arc<ComponentRef::NFComponentRef>, mut target: Arc<EvalTarget::EvalTarget>) -> Arc<Binding::NFBinding> {
    let mut binding: Arc<Binding::NFBinding> = Arc::new(Binding::UNBOUND);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut rec_node: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    binding = 'mc: {
        let __mc_input = component.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut exp: Arc<Expression::NFExpression> = exp.clone();
                    exp = makeRecordFieldBindingFromParent(cref.clone(), target.clone())?;
                    Ok((if (Expression::isEmpty(exp.clone())) {Binding::EMPTY_BINDING().clone()} else {Arc::new(Binding::NFBinding::CEVAL_BINDING { bindingExp: exp.clone() })}, exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { exp = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Component::COMPONENT { ty: Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: rec_node, .. }, .. }, .. } => {
                    let mut binding: Arc<Binding::NFBinding> = binding.clone();
                    let mut exp: Arc<Expression::NFExpression> = exp.clone();
                    exp = makeRecordBindingExp(var_field!((*component).classInst, Component::NFComponent::COMPONENT).clone(), rec_node.clone(), var_field!((*component).ty, Component::NFComponent::COMPONENT).clone(), cref.clone(), target.clone())?;
                    binding = Arc::new(Binding::NFBinding::CEVAL_BINDING { bindingExp: exp.clone() });
                    if !(ComponentRef::hasSubscripts(cref.clone())?) {
                        InstNode::updateComponent(Component::setBinding(binding.clone(), component.clone())?, node.clone())?;
                    }
                    Ok((binding.clone(), binding.clone(), exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { binding = __wb0; exp = __wb1; break 'mc __v; }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Component::COMPONENT { ty: Deref @ Type::ARRAY { elementType: ty @ Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: rec_node, .. }, .. }, .. }, .. } => {
                    let mut binding: Arc<Binding::NFBinding> = binding.clone();
                    let mut exp: Arc<Expression::NFExpression> = exp.clone();
                    exp = Expression::mapCrefScalars(Expression::fromCref(cref.clone(), false)?, (std::sync::Arc::new({ let __pe_b0 = var_field!((*component).classInst, Component::NFComponent::COMPONENT).clone(); let __pe_b1 = rec_node.clone(); let __pe_b2 = ty.clone(); let __pe_b4 = target.clone(); move |__pe_a3| makeRecordBindingExp(__pe_b0.clone(), __pe_b1.clone(), __pe_b2.clone(), __pe_a3, __pe_b4.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
                    binding = Arc::new(Binding::NFBinding::CEVAL_BINDING { bindingExp: exp.clone() });
                    if !(ComponentRef::hasSubscripts(cref.clone())?) {
                        InstNode::updateComponent(Component::setBinding(binding.clone(), component.clone())?, node.clone())?;
                    }
                    Ok((binding.clone(), binding.clone(), exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { binding = __wb0; exp = __wb1; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Binding::EMPTY_BINDING().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    binding
}

pub(crate) fn makeRecordFieldBindingFromParent(mut cref: Arc<ComponentRef::NFComponentRef>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    let mut parent_cr: Arc<ComponentRef::NFComponentRef>;
    let mut parent: Arc<InstNode::InstNode>;
    let mut exp_context: i32;
    let mut binding: Arc<Binding::NFBinding>;
    let mut comp: Arc<Component::NFComponent>;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    parent_cr = ComponentRef::rest(cref.clone())?;
    parent = ComponentRef::node(parent_cr.clone())?;
    exp_context = InstContext::nodeContext(parent.clone(), target.context.clone());
    comp = InstNode::component(parent.clone())?;
    binding = Component::getBinding(comp.clone());
    subs = ComponentRef::getSubscripts(parent_cr.clone());
    if Binding::hasExp(binding.clone()) {
        if !(Binding::isTyped(binding.clone())) {
            binding = Typing::typeBinding(binding, InstContext::set(exp_context, InstContext::BINDING.clone()))?;
            comp = Component::setBinding(binding.clone(), comp)?;
            InstNode::updateComponent(comp, parent)?;
        }
        exp = Binding::getExp(binding)?;
        exp = Expression::applySubscripts(subs, exp, false)?;
        exp = Expression::recordElement((ComponentRef::firstName(cref.clone(), false)?).clone(), exp)?;
        exp = evalExp(exp, target)?;
        exp = Expression::map(exp, (std::sync::Arc::new({ let __pe_b1 = ComponentRef::nodesIncludingSplitSubs(cref, metamodelica::nil())?; move |__pe_a0| Expression::expandNonListedSplitIndices(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    } else {
        exp = makeRecordFieldBindingFromParent(parent_cr, target)?;
        exp = Expression::applySubscripts(subs, exp, false)?;
        exp = Expression::recordElement((ComponentRef::firstName(cref, false)?).clone(), exp)?;
    }
    Ok(exp)
}

pub(crate) fn makeRecordBindingExp(mut typeNode: Arc<InstNode::InstNode>, mut recordNode: Arc<InstNode::InstNode>, mut recordType: Arc<Type::NFType>, mut cref: Arc<ComponentRef::NFComponentRef>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    let mut tree: Arc<ClassTree::ClassTree>;
    let mut comps: metamodelica::Array<Arc<InstNode::InstNode>>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut ty: Arc<Type::NFType>;
    let mut c: Arc<InstNode::InstNode>;
    let mut cr: Arc<ComponentRef::NFComponentRef>;
    let mut arg: Arc<Expression::NFExpression>;
    tree = Class::classTree(InstNode::getClass(typeNode)?)?;
    comps = ClassTree::getComponents(tree)?;
    args = metamodelica::nil();
    ErrorExt::setCheckpoint(literal!("NFCeval.makeRecordBindingExp"));
    for mut i in ({let __s=metamodelica::arrayLength(comps.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
        c = ({let __elt = comps.borrow()[(i.clone()-1) as usize].clone(); __elt});
        ty = InstNode::getType(c.clone())?;
        cr = Arc::new(ComponentRef::NFComponentRef::CREF { node: c.clone(), subscripts: metamodelica::nil(), ty: ty.clone(), origin: ComponentRef::Origin::CREF.clone(), restCref: cref.clone() });
        arg = Arc::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: cr.clone() });
        if Component::variability(InstNode::component(c.clone())?)? <= Variability::PARAMETER.clone() {
            if '__try0: {
                arg = unwrap_break_err!(evalExp(arg.clone(), target.clone()), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
        }
        args = metamodelica::cons(arg.clone(), args.clone());
    }
    ErrorExt::rollBack(literal!("NFCeval.makeRecordBindingExp"));
    exp = Expression::makeRecord(InstNode::fullPath(recordNode, false)?, recordType, args);
    Ok(exp)
}

pub(crate) fn evalTypename(mut ty: Arc<Type::NFType>, mut originExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    exp = if (InstContext::inIterationRange(target.context.clone())) {ExpandExp::expandTypename(ty)?} else {originExp};
    Ok(exp)
}

pub(crate) fn evalRange(mut rangeExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut ty: Arc<Type::NFType>;
    let mut start_exp: Arc<Expression::NFExpression>;
    let mut stop_exp: Arc<Expression::NFExpression>;
    let mut step_exp: Option<Arc<Expression::NFExpression>>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(rangeExp) {
        Deref @ Expression::RANGE { ty: __pa0, start: __pa1, step: __pa2, stop: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    start_exp = __pa1.clone();
    step_exp = __pa2.clone();
    stop_exp = __pa3.clone();
    start_exp = evalExp(start_exp, target.clone())?;
    step_exp = Util::applyOption(step_exp, (std::sync::Arc::new({ let __pe_b1 = target.clone(); move |__pe_a0| evalExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    stop_exp = evalExp(stop_exp, target.clone())?;
    if InstContext::inIterationRange(target.context.clone()) {
        ty = TypeCheck::getRangeType(start_exp.clone(), step_exp.clone(), stop_exp.clone(), Type::arrayElementType(ty), EvalTarget::getInfo(target))?;
        result = Arc::new(Expression::NFExpression::RANGE { ty: ty, start: start_exp, step: step_exp, stop: stop_exp });
    } else {
        result = Arc::new(Expression::NFExpression::RANGE { ty: ty, start: start_exp, step: step_exp, stop: stop_exp });
        result = Expression::mapSplitExpressions(result, (std::sync::Arc::new(evalRangeExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    }
    Ok(result)
}

pub(crate) fn evalRangeExp(mut rangeExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    let mut start: Arc<Expression::NFExpression>;
    let mut step: Arc<Expression::NFExpression>;
    let mut stop: Arc<Expression::NFExpression>;
    let mut opt_step: Option<Arc<Expression::NFExpression>>;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut literals: Arc<metamodelica::List<ArcStr>>;
    let mut istep: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(SimplifyExp::simplify(Expression::map(rangeExp, (std::sync::Arc::new(Expression::replaceResizableParameter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, false)?) {
        Deref @ Expression::RANGE { start: __pa0, step: __pa1, stop: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    start = __pa0.clone();
    opt_step = __pa1.clone();
    stop = __pa2.clone();
    if isSome(opt_step.clone()) {
        let __pa3 = ::match_deref::match_deref! { match &(opt_step) {
            Some(__pa3) => __pa3.clone(),
            _ => bail!("pattern mismatch"),
        } };
        step = __pa3.clone();
        (ty, expl) = (::match_deref::match_deref! { match &((start.clone(), step.clone(), stop.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { value: __esc_istep }, Deref @ Expression::INTEGER { .. }) => {
            istep = (*__esc_istep).clone();
            expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (({let __s=var_field!((*start).value, Expression::NFExpression::INTEGER).clone(); let __e=var_field!((*stop).value, Expression::NFExpression::INTEGER).clone(); let __step=istep.clone(); (0i32..).map(move |__k| __s + __k * __step).take_while(move |&__v| __step != 0 && (if __step > 0 { __v <= __e } else { __v >= __e }))})).into_iter() {
            let __x = Arc::new(Expression::NFExpression::INTEGER { value: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (crate::NFType::interned_INTEGER(), expl)
        },
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => {
            expl = evalRangeReal(var_field!((*start).value, Expression::NFExpression::REAL).clone(), var_field!((*step).value, Expression::NFExpression::REAL).clone(), var_field!((*stop).value, Expression::NFExpression::REAL).clone());
            (crate::NFType::interned_REAL(), expl)
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalRangeExp"), list![start, step, stop], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    } else {
        (ty, expl) = (::match_deref::match_deref! { match &((start.clone(), stop.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => {
            expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (var_field!((*start).value, Expression::NFExpression::INTEGER).clone()..=var_field!((*stop).value, Expression::NFExpression::INTEGER).clone()).into_iter() {
            let __x = Arc::new(Expression::NFExpression::INTEGER { value: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (crate::NFType::interned_INTEGER(), expl)
        },
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => {
            expl = evalRangeReal(var_field!((*start).value, Expression::NFExpression::REAL).clone(), metamodelica::OrderedFloat(1.0_f64), var_field!((*stop).value, Expression::NFExpression::REAL).clone());
            (crate::NFType::interned_REAL(), expl)
        },
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => {
            expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut b in (({let __bs = var_field!((*start).value, Expression::NFExpression::BOOLEAN).clone(); let __be = var_field!((*stop).value, Expression::NFExpression::BOOLEAN).clone(); if !__bs && !__be { vec![false] } else if !__bs && __be { vec![false, true] } else if __bs && __be { vec![true] } else { Vec::<bool>::new() }})).into_iter() {
            let __x = Arc::new(Expression::NFExpression::BOOLEAN { value: b.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (crate::NFType::interned_BOOLEAN(), expl)
        },
        (Deref @ Expression::ENUM_LITERAL { ty: __esc_ty @ Deref @ Type::ENUMERATION { .. }, .. }, Deref @ Expression::ENUM_LITERAL { .. }) => {
            ty = (*__esc_ty).clone();
            expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (var_field!((*start).index, Expression::NFExpression::ENUM_LITERAL).clone()..=var_field!((*stop).index, Expression::NFExpression::ENUM_LITERAL).clone()).into_iter() {
            let __x = Arc::new(Expression::NFExpression::ENUM_LITERAL { ty: ty.clone(), name: ((var_field!((*ty).literals, Type::NFType::ENUMERATION).clone()).get(i.clone())?).clone(), index: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (ty.clone(), expl)
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalRangeExp"), list![start, stop], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    exp = Expression::makeArray(Arc::new(Type::NFType::ARRAY { elementType: ty, dimensions: list![Dimension::fromInteger((expl.clone().len() as i32), Prefixes::Variability::CONSTANT.clone())] }), metamodelica::arrayFromVec(expl.into_iter().cloned().collect()), true);
    Ok(exp)
}

pub(crate) fn evalRangeReal(mut start: metamodelica::Real, mut step: metamodelica::Real, mut stop: metamodelica::Real) -> Arc<metamodelica::List<Arc<Expression::NFExpression>>> {
    let mut result: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut steps: i32;
    steps = Util::realRangeSize(start, step, stop);
    if steps == 0 {
        result = metamodelica::nil();
    } else if steps == 1 {
        result = list![Arc::new(Expression::NFExpression::REAL { value: start })];
    } else {
        result = list![Arc::new(Expression::NFExpression::REAL { value: stop })];
        for mut i in ({let __s=steps - 2; let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
            result = metamodelica::cons(Arc::new(Expression::NFExpression::REAL { value: start + metamodelica::OrderedFloat((i.clone()) as f64) * step }), result.clone());
        }
        result = metamodelica::cons(Arc::new(Expression::NFExpression::REAL { value: start }), result);
    }
    result
}

pub(crate) fn printFailedEvalError(mut name: ArcStr, mut exp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<()> {
    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(" failed to evaluate ‘")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!("‘")); ArcStr::from(__mm_s) }).clone(), info)?;
    Ok(())
}

pub(crate) fn evalBinaryOp(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    exp = Expression::mapSplitExpressions(Arc::new(Expression::NFExpression::BINARY { exp1: exp1, operator: op, exp2: exp2 }), (std::sync::Arc::new({ let __pe_b1 = target; move |__pe_a0| evalBinaryExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(exp)
}

pub(crate) fn evalBinaryExp(mut binaryExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut op: Arc<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(binaryExp) {
        Deref @ Expression::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    op = __pa1.clone();
    e2 = __pa2.clone();
    result = evalBinaryOp_dispatch(e1, op, e2, target)?;
    Ok(result)
}

pub(crate) fn evalBinaryOp_dispatch(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    exp = (match op.op.clone() {
        Operator::Op::ADD => evalBinaryAdd(exp1, exp2)?,
        Operator::Op::SUB => evalBinarySub(exp1, exp2)?,
        Operator::Op::MUL => evalBinaryMul(exp1, exp2)?,
        Operator::Op::DIV => evalBinaryDiv(exp1, exp2, target)?,
        Operator::Op::POW => evalBinaryPow(exp1, exp2, target)?,
        Operator::Op::ADD_EW => evalBinaryAdd(exp1, exp2)?,
        Operator::Op::SUB_EW => evalBinarySub(exp1, exp2)?,
        Operator::Op::MUL_EW => evalBinaryMul(exp1, exp2)?,
        Operator::Op::ADD_SCALAR_ARRAY => evalBinaryScalarArray(exp1, exp2, (std::sync::Arc::new(evalBinaryAdd) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        Operator::Op::ADD_ARRAY_SCALAR { .. } => evalBinaryArrayScalar(exp1, exp2, (std::sync::Arc::new(evalBinaryAdd) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        Operator::Op::SUB_SCALAR_ARRAY { .. } => evalBinaryScalarArray(exp1, exp2, (std::sync::Arc::new(evalBinarySub) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        Operator::Op::SUB_ARRAY_SCALAR => evalBinaryArrayScalar(exp1, exp2, (std::sync::Arc::new(evalBinarySub) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        Operator::Op::MUL_SCALAR_ARRAY => evalBinaryScalarArray(exp1, exp2, (std::sync::Arc::new(evalBinaryMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        Operator::Op::MUL_ARRAY_SCALAR { .. } => evalBinaryArrayScalar(exp1, exp2, (std::sync::Arc::new(evalBinaryMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        Operator::Op::MUL_VECTOR_MATRIX => evalBinaryMulVectorMatrix(exp1, exp2)?,
        Operator::Op::MUL_MATRIX_VECTOR => evalBinaryMulMatrixVector(exp1, exp2)?,
        Operator::Op::SCALAR_PRODUCT => evalBinaryScalarProduct(exp1, exp2)?,
        Operator::Op::MATRIX_PRODUCT => evalBinaryMatrixProduct(exp1, exp2)?,
        Operator::Op::DIV_SCALAR_ARRAY { .. } => evalBinaryScalarArray(exp1, exp2, (std::sync::Arc::new({ let __pe_b2 = target; move |__pe_a0, __pe_a1| evalBinaryDiv(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        Operator::Op::DIV_ARRAY_SCALAR { .. } => evalBinaryArrayScalar(exp1, exp2, (std::sync::Arc::new({ let __pe_b2 = target; move |__pe_a0, __pe_a1| evalBinaryDiv(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        Operator::Op::POW_SCALAR_ARRAY { .. } => evalBinaryScalarArray(exp1, exp2, (std::sync::Arc::new({ let __pe_b2 = target; move |__pe_a0, __pe_a1| evalBinaryPow(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        Operator::Op::POW_ARRAY_SCALAR { .. } => evalBinaryArrayScalar(exp1, exp2, (std::sync::Arc::new({ let __pe_b2 = target; move |__pe_a0, __pe_a1| evalBinaryPow(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        Operator::Op::POW_MATRIX => evalBinaryPowMatrix(exp1, exp2)?,
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalBinaryOp_dispatch")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*Expression::toString(Arc::new(Expression::NFExpression::BINARY { exp1: exp1, operator: op, exp2: exp2 }))?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
    });
    Ok(exp)
}

pub(crate) fn evalBinaryAdd(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::REAL { .. }, Deref @ Expression::INTEGER { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() + metamodelica::OrderedFloat((var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) as f64) }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) as f64) + var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() + var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone() }),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() + var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => Arc::new(Expression::NFExpression::STRING { value: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*exp1).value, Expression::NFExpression::STRING).clone()); __mm_s.push_str(&*var_field!((*exp2).value, Expression::NFExpression::STRING).clone()); ArcStr::from(__mm_s) }).clone() }),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => Arc::new(Expression::NFExpression::STRING { value: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*exp1).value, Expression::NFExpression::STRING).clone()); __mm_s.push_str(&*var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()); ArcStr::from(__mm_s) }).clone() }),
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => Arc::new(Expression::NFExpression::STRING { value: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()); __mm_s.push_str(&*var_field!((*exp2).value, Expression::NFExpression::STRING).clone()); ArcStr::from(__mm_s) }).clone() }),
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => Arc::new(Expression::NFExpression::STRING { value: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()); __mm_s.push_str(&*var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()); ArcStr::from(__mm_s) }).clone() }),
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if (metamodelica::arrayLength(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone()) == metamodelica::arrayLength(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone())) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new(evalBinaryAdd) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, true),
        (Deref @ Expression::ARRAY { .. }, _) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1 = exp2.clone(); move |__pe_a0| evalBinaryAdd(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, var_field!((*exp1).literal, Expression::NFExpression::ARRAY).clone()),
        (_, Deref @ Expression::ARRAY { .. }) => Expression::makeArray(var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b0 = exp1.clone(); move |__pe_a1| evalBinaryAdd(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, var_field!((*exp2).literal, Expression::NFExpression::ARRAY).clone()),
        (Deref @ Expression::EMPTY { .. }, _) => exp2.clone(),
        (_, Deref @ Expression::EMPTY { .. }) => exp1.clone(),
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeAdd(crate::NFType::interned_UNKNOWN()), exp2: exp2.clone() });
            printFailedEvalError(literal!("NFCeval.evalBinaryAdd"), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinarySub(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::REAL { .. }, Deref @ Expression::INTEGER { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() - metamodelica::OrderedFloat((var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) as f64) }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) as f64) - var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() - var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone() }),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() - var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if (metamodelica::arrayLength(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone()) == metamodelica::arrayLength(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone())) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new(evalBinarySub) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, true),
        (Deref @ Expression::ARRAY { .. }, _) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1 = exp2.clone(); move |__pe_a0| evalBinarySub(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, var_field!((*exp1).literal, Expression::NFExpression::ARRAY).clone()),
        (_, Deref @ Expression::ARRAY { .. }) => Expression::makeArray(var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b0 = exp1.clone(); move |__pe_a1| evalBinarySub(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, var_field!((*exp2).literal, Expression::NFExpression::ARRAY).clone()),
        (Deref @ Expression::EMPTY { .. }, _) => evalBinarySub(Expression::makeZero(Expression::typeOf(exp2.clone()))?, exp2.clone())?,
        (_, Deref @ Expression::EMPTY { .. }) => exp1.clone(),
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeSub(crate::NFType::interned_UNKNOWN()), exp2: exp2.clone() });
            printFailedEvalError(literal!("NFCeval.evalBinarySub"), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalMultaryAddSub(mut arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut inv_arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut operator_ty: Arc<Type::NFType>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::EMPTY { ty: operator_ty.clone() });
    let mut isNeutral: bool;
    for mut arg in &*arguments {
        let mut arg = arg.clone();
        exp = evalBinaryAdd(exp.clone(), arg.clone())?;
    }
    for mut arg in &*inv_arguments {
        let mut arg = arg.clone();
        exp = evalBinarySub(exp.clone(), arg.clone())?;
    }
    isNeutral = Expression::isEmpty(exp.clone()) || Expression::isZero(exp.clone())?;
    Ok((exp, isNeutral))
}

pub(crate) fn evalBinaryMul(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::REAL { .. }, Deref @ Expression::INTEGER { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() * metamodelica::OrderedFloat((var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) as f64) }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) as f64) * var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() * var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone() }),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() * var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if (metamodelica::arrayLength(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone()) == metamodelica::arrayLength(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone())) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new(evalBinaryMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, true),
        (Deref @ Expression::ARRAY { .. }, _) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1 = exp2.clone(); move |__pe_a0| evalBinaryMul(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, var_field!((*exp1).literal, Expression::NFExpression::ARRAY).clone()),
        (_, Deref @ Expression::ARRAY { .. }) => Expression::makeArray(var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b0 = exp1.clone(); move |__pe_a1| evalBinaryMul(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, var_field!((*exp2).literal, Expression::NFExpression::ARRAY).clone()),
        (Deref @ Expression::EMPTY { .. }, _) => exp2.clone(),
        (_, Deref @ Expression::EMPTY { .. }) => exp1.clone(),
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeMul(crate::NFType::interned_UNKNOWN()), exp2: exp2.clone() });
            printFailedEvalError(literal!("NFCeval.evalBinaryMul"), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinaryDiv(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (_, _) if (Expression::isZero(exp2.clone())?) => {
            if EvalTarget::hasInfo(target.clone()) {
                Error::addSourceMessage(Error::DIVISION_BY_ZERO.clone(), list![(Expression::toString(exp1.clone())?).clone(), (Expression::toString(exp2.clone())?).clone()], EvalTarget::getInfo(target))?;
                bail!("fail");
            } else {
                exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeDiv(crate::NFType::interned_REAL()), exp2: exp2.clone() });
            }
            exp
        },
        (_, Deref @ Expression::INTEGER { value: 1 }) => exp1.clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::INTEGER { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() / metamodelica::OrderedFloat((var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) as f64) }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) as f64) / var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => if (intMod(var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone(), var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) == 0) {Arc::new(Expression::NFExpression::INTEGER { value: intDiv(var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone(), var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) })} else {Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) as f64) / metamodelica::OrderedFloat((var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) as f64) })},
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: var_field!((*exp1).value, Expression::NFExpression::REAL).clone() / var_field!((*exp2).value, Expression::NFExpression::REAL).clone() }),
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if (metamodelica::arrayLength(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone()) == metamodelica::arrayLength(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone())) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b2 = target; move |__pe_a0, __pe_a1| evalBinaryDiv(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, true),
        (Deref @ Expression::ARRAY { .. }, _) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1 = exp2.clone(); let __pe_b2 = target; move |__pe_a0| evalBinaryDiv(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, var_field!((*exp1).literal, Expression::NFExpression::ARRAY).clone()),
        (_, Deref @ Expression::ARRAY { .. }) => Expression::makeArray(var_field!((*exp2).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b0 = exp1.clone(); let __pe_b2 = target; move |__pe_a1| evalBinaryDiv(__pe_b0.clone(), __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, var_field!((*exp2).literal, Expression::NFExpression::ARRAY).clone()),
        (Deref @ Expression::EMPTY { .. }, _) => evalBinaryDiv(Expression::makeOne(Expression::typeOf(exp2.clone()))?, exp2.clone(), target)?,
        (_, Deref @ Expression::EMPTY { .. }) => exp1.clone(),
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeDiv(crate::NFType::interned_UNKNOWN()), exp2: exp2.clone() });
            printFailedEvalError(literal!("NFCeval.evalBinaryDiv"), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalMultaryMulDiv(mut arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut inv_arguments: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut operator_ty: Arc<Type::NFType>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::EMPTY { ty: operator_ty.clone() });
    let mut isNeutral: bool;
    for mut arg in &*arguments {
        let mut arg = arg.clone();
        exp = evalBinaryMul(exp.clone(), arg.clone())?;
    }
    for mut arg in &*inv_arguments {
        let mut arg = arg.clone();
        exp = evalBinaryDiv(exp.clone(), arg.clone(), noTarget().clone())?;
    }
    isNeutral = Expression::isEmpty(exp.clone()) || Expression::isOne(exp.clone())?;
    Ok((exp, isNeutral))
}

pub(crate) fn evalBinaryPow(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) if (var_field!((*exp1).value, Expression::NFExpression::REAL).clone() < metamodelica::OrderedFloat((0) as f64) && metamodelica::OrderedFloat((((var_field!((*exp2).value, Expression::NFExpression::REAL).clone()).0.floor() as i32)) as f64) != var_field!((*exp2).value, Expression::NFExpression::REAL).clone()) => {
            if EvalTarget::hasInfo(target.clone()) {
                Error::addSourceMessage(Error::INVALID_NEGATIVE_POW.clone(), list![(Expression::toString(exp1.clone())?).clone(), (Expression::toString(exp2.clone())?).clone()], EvalTarget::getInfo(target))?;
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makePow(crate::NFType::interned_REAL()), exp2: exp2.clone() })
        },
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*exp1).value, Expression::NFExpression::REAL).clone()).powf(var_field!((*exp2).value, Expression::NFExpression::REAL).clone()) }),
        (Deref @ Expression::ARRAY { .. }, Deref @ Expression::ARRAY { .. }) if (metamodelica::arrayLength(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone()) == metamodelica::arrayLength(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone())) => Expression::makeArray(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b2 = target; move |__pe_a0, __pe_a1| evalBinaryPow(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, true),
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makePow(crate::NFType::interned_UNKNOWN()), exp2: exp2.clone() });
            printFailedEvalError(literal!("NFCeval.evalBinaryPow"), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinaryScalarArray(mut scalarExp: Arc<Expression::NFExpression>, mut arrayExp: Arc<Expression::NFExpression>, mut opFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> {
    pub type FuncT = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

    let mut exp: Arc<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &(arrayExp.clone()) {
        Deref @ Expression::ARRAY { .. } => Expression::makeArray(var_field!((*arrayExp).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b0 = scalarExp; let __pe_b2: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = opFunc.clone(); move |__pe_a1| evalBinaryScalarArray(__pe_b0.clone(), __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, true),
        _ => opFunc(scalarExp, arrayExp)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinaryArrayScalar(mut arrayExp: Arc<Expression::NFExpression>, mut scalarExp: Arc<Expression::NFExpression>, mut opFunc: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> {
    pub type FuncT = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

    let mut exp: Arc<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &(arrayExp.clone()) {
        Deref @ Expression::ARRAY { .. } => Expression::makeArray(var_field!((*arrayExp).ty, Expression::NFExpression::ARRAY).clone(), Array::map(var_field!((*arrayExp).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1 = scalarExp; let __pe_b2: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = opFunc.clone(); move |__pe_a0| evalBinaryArrayScalar(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, true),
        _ => opFunc(arrayExp, scalarExp)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinaryMulVectorMatrix(mut vectorExp: Arc<Expression::NFExpression>, mut matrixExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut m: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>> = Default::default();
    exp = (::match_deref::match_deref! { match &(Expression::transposeArray(matrixExp.clone())?) {
        Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: __esc_ty, dimensions: Deref @ metamodelica::List::Cons { head: __esc_m, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, elements: __esc_arr, .. } => {
            ty = (*__esc_ty).clone();
            m = (*__esc_m).clone();
            arr = (*__esc_arr).clone();
            arr = Array::map(arr.clone(), (std::sync::Arc::new({ let __pe_b0 = vectorExp; move |__pe_a1| evalBinaryScalarProduct(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            Expression::makeArray(Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: list![m.clone()] }), arr.clone(), true)
        },
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: vectorExp, operator: Operator::makeMul(crate::NFType::interned_UNKNOWN()), exp2: matrixExp });
            printFailedEvalError(literal!("NFCeval.evalBinaryMulVectorMatrix"), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinaryMulMatrixVector(mut matrixExp: Arc<Expression::NFExpression>, mut vectorExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>> = Default::default();
    exp = (::match_deref::match_deref! { match &(matrixExp.clone()) {
        Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: __esc_ty, dimensions: Deref @ metamodelica::List::Cons { head: __esc_n, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, elements: __esc_arr, .. } => {
            ty = (*__esc_ty).clone();
            n = (*__esc_n).clone();
            arr = (*__esc_arr).clone();
            arr = Array::map(arr.clone(), (std::sync::Arc::new({ let __pe_b1 = vectorExp; move |__pe_a0| evalBinaryScalarProduct(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            Expression::makeArray(Arc::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: list![n.clone()] }), arr.clone(), true)
        },
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: matrixExp, operator: Operator::makeMul(crate::NFType::interned_UNKNOWN()), exp2: vectorExp });
            printFailedEvalError(literal!("NFCeval.evalBinaryMulMatrixVector"), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinaryScalarProduct(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: elem_ty, .. }, .. }, Deref @ Expression::ARRAY { .. }) if (metamodelica::arrayLength(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone()) == metamodelica::arrayLength(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone())) => {
            exp = Expression::makeZero(elem_ty.clone())?;
            for mut i in 1..=metamodelica::arrayLength(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone()) {
                exp = evalBinaryAdd(exp.clone(), evalBinaryMul(metamodelica::Dangerous::arrayGetNoBoundsChecking(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), i.clone()), metamodelica::Dangerous::arrayGetNoBoundsChecking(var_field!((*exp2).elements, Expression::NFExpression::ARRAY).clone(), i.clone()))?)?;
            }
            exp
        },
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1.clone(), operator: Operator::makeMul(crate::NFType::interned_UNKNOWN()), exp2: exp2.clone() });
            printFailedEvalError(literal!("NFCeval.evalBinaryScalarProduct"), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinaryMatrixProduct(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression>;
    let mut elem_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut row_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut mat_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut n: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut p: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut arr1: metamodelica::Array<Arc<Expression::NFExpression>> = Default::default();
    let mut arr2: metamodelica::Array<Arc<Expression::NFExpression>> = Default::default();
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>> = Default::default();
    e2 = Expression::transposeArray(exp2.clone())?;
    exp = (::match_deref::match_deref! { match &((exp1.clone(), e2)) {
        (Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: __esc_elem_ty, dimensions: Deref @ metamodelica::List::Cons { head: __esc_n, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, elements: __esc_arr1, .. }, Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { elementType: _, dimensions: Deref @ metamodelica::List::Cons { head: __esc_p, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, elements: __esc_arr2, .. }) => {
            elem_ty = (*__esc_elem_ty).clone();
            n = (*__esc_n).clone();
            arr1 = (*__esc_arr1).clone();
            p = (*__esc_p).clone();
            arr2 = (*__esc_arr2).clone();
            mat_ty = Arc::new(Type::NFType::ARRAY { elementType: elem_ty.clone(), dimensions: list![n.clone(), p.clone()] });
            if arr2.clone().borrow().is_empty() {
                exp = Expression::makeZero(mat_ty)?;
            } else {
                row_ty = Arc::new(Type::NFType::ARRAY { elementType: elem_ty.clone(), dimensions: list![p.clone()] });
                arr = metamodelica::arrayCreate(metamodelica::arrayLength(arr1.clone()), exp1);
                for mut i in 1..=metamodelica::arrayLength(arr1.clone()) {
                    unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), i.clone(), Expression::makeArray(row_ty.clone(), Array::map(arr2.clone(), (std::sync::Arc::new({ let __pe_b0 = metamodelica::Dangerous::arrayGetNoBoundsChecking(arr1.clone(), i.clone()); move |__pe_a1| evalBinaryScalarProduct(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, true)) };
                }
                exp = Expression::makeArray(mat_ty, arr.clone(), true);
            }
            exp
        },
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: exp1, operator: Operator::makeMul(crate::NFType::interned_UNKNOWN()), exp2: exp2 });
            printFailedEvalError(literal!("NFCeval.evalBinaryMatrixProduct"), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinaryPowMatrix(mut matrixExp: Arc<Expression::NFExpression>, mut nExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: i32 = 0;
    exp = (::match_deref::match_deref! { match &(nExp.clone()) {
        Deref @ Expression::INTEGER { value: 0 } => {
            n = Dimension::size(listHead(Type::arrayDims(Expression::typeOf(matrixExp)))?, false)?;
            Expression::makeIdentityMatrix(n, crate::NFType::interned_REAL())?
        },
        Deref @ Expression::INTEGER { value: __esc_n } => {
            n = (*__esc_n).clone();
            evalBinaryPowMatrix2(matrixExp, n.clone())?
        },
        _ => {
            exp = Arc::new(Expression::NFExpression::BINARY { exp1: matrixExp, operator: Operator::makePow(crate::NFType::interned_UNKNOWN()), exp2: nExp });
            printFailedEvalError(literal!("NFCeval.evalBinaryPowMatrix"), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBinaryPowMatrix2(mut matrix: Arc<Expression::NFExpression>, mut n: i32) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (match n {
        1 => matrix,
        2 => evalBinaryMatrixProduct(matrix.clone(), matrix)?,
        _ if (intMod(n, 2) == 0) => {
            exp = evalBinaryPowMatrix2(matrix, intDiv(n, 2))?;
            evalBinaryMatrixProduct(exp.clone(), exp)?
        },
        _ => {
            exp = evalBinaryPowMatrix2(matrix.clone(), n - 1)?;
            evalBinaryMatrixProduct(matrix, exp)?
        },
    });
    Ok(exp)
}

pub(crate) fn evalUnaryOp(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    exp = (match op.op.clone() {
        Operator::Op::UMINUS if (Expression::isZero(exp1.clone())?) => exp1.clone(),
        Operator::Op::UMINUS => Expression::mapSplitExpressions(exp1.clone(), (std::sync::Arc::new(evalUnaryMinus) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalUnaryOp")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*Expression::toString(Arc::new(Expression::NFExpression::UNARY { operator: op, exp: exp1.clone() }))?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
    });
    Ok(exp)
}

pub(crate) fn evalUnaryMinus(mut exp1: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ Expression::INTEGER { .. } => Arc::new(Expression::NFExpression::INTEGER { value: -(var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone()) }),
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: -(var_field!((*exp1).value, Expression::NFExpression::REAL).clone()) }),
        Deref @ Expression::ARRAY { .. } => {
            assign_variant_field!(exp1 => Expression::NFExpression::ARRAY; elements = Array::map(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new(evalUnaryMinus) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?);
            exp1
        },
        _ => {
            exp = Arc::new(Expression::NFExpression::UNARY { operator: Operator::makeUMinus(crate::NFType::interned_UNKNOWN()), exp: exp1 });
            printFailedEvalError(literal!("NFCeval.evalUnaryMinus"), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalLogicBinaryOp(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    exp = Expression::mapSplitExpressions(Arc::new(Expression::NFExpression::LBINARY { exp1: exp1, operator: op, exp2: exp2 }), (std::sync::Arc::new({ let __pe_b1 = target; move |__pe_a0| evalLogicBinaryExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(exp)
}

pub(crate) fn evalLogicBinaryExp(mut binaryExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut op: Arc<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(binaryExp) {
        Deref @ Expression::LBINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    op = __pa1.clone();
    e2 = __pa2.clone();
    result = evalLogicBinaryOp_dispatch(e1, op, e2, target)?;
    Ok(result)
}

pub(crate) fn evalLogicBinaryOp_dispatch(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    exp = (match op.op.clone() {
        Operator::Op::AND => evalLogicBinaryAnd(evalExp(exp1, target.clone())?, exp2, target)?,
        Operator::Op::OR => evalLogicBinaryOr(evalExp(exp1, target.clone())?, exp2, target)?,
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalLogicBinaryOp_dispatch")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*Expression::toString(Arc::new(Expression::NFExpression::LBINARY { exp1: exp1, operator: op, exp2: exp2 }))?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
    });
    Ok(exp)
}

pub(crate) fn evalLogicBinaryAnd(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = 'mc: {
        let __mc_input = exp1.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::BOOLEAN { .. } => {
                    Ok(if (var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone()) {evalExp(exp2.clone(), target.clone())?} else {exp1.clone()})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Expression::ARRAY { .. } => {
                    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
                    let __pa0 = ::match_deref::match_deref! { match &(evalExp(exp2.clone(), target.clone())?) {
                        Deref @ Expression::ARRAY { elements: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    arr = __pa0.clone();
                    arr = Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), arr.clone(), (std::sync::Arc::new({ let __pe_b2 = target.clone(); move |__pe_a0, __pe_a1| evalLogicBinaryAnd(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
                    Ok(Expression::makeArray(Type::setArrayElementType(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), crate::NFType::interned_BOOLEAN()), arr.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut exp: Arc<Expression::NFExpression> = exp.clone();
                    exp = Arc::new(Expression::NFExpression::LBINARY { exp1: exp1.clone(), operator: Operator::makeAnd(crate::NFType::interned_UNKNOWN()), exp2: exp2.clone() });
                    printFailedEvalError(literal!("NFCeval.evalLogicBinaryAnd"), exp.clone(), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
                    Ok((bail!("fail"), exp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { exp = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(exp)
}

pub(crate) fn evalLogicBinaryOr(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ Expression::BOOLEAN { .. } => {
            if (var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone()) {exp1} else {evalExp(exp2, target)?}
        },
        Deref @ Expression::ARRAY { .. } => {
            let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
            let __pa0 = ::match_deref::match_deref! { match &(evalExp(exp2, target.clone())?) {
                Deref @ Expression::ARRAY { elements: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            arr = __pa0.clone();
            arr = Array::threadMap(var_field!((*exp1).elements, Expression::NFExpression::ARRAY).clone(), arr.clone(), (std::sync::Arc::new({ let __pe_b2 = target; move |__pe_a0, __pe_a1| evalLogicBinaryOr(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            Expression::makeArray(Type::setArrayElementType(var_field!((*exp1).ty, Expression::NFExpression::ARRAY).clone(), crate::NFType::interned_BOOLEAN()), arr.clone(), true)
        },
        _ => {
            exp = Arc::new(Expression::NFExpression::LBINARY { exp1: exp1, operator: Operator::makeOr(crate::NFType::interned_UNKNOWN()), exp2: exp2 });
            printFailedEvalError(literal!("NFCeval.evalLogicBinaryOr"), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalLogicUnaryOp(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    exp = (match op.op.clone() {
        Operator::Op::NOT => Expression::mapSplitExpressions(exp1, (std::sync::Arc::new(evalLogicUnaryNot) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalLogicUnaryOp")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*Expression::toString(Arc::new(Expression::NFExpression::LUNARY { operator: op, exp: exp1 }))?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
    });
    Ok(exp)
}

pub(crate) fn evalLogicUnaryNot(mut exp1: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    exp = (::match_deref::match_deref! { match &(exp1.clone()) {
        Deref @ Expression::BOOLEAN { .. } => Arc::new(Expression::NFExpression::BOOLEAN { value: !(var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone()) }),
        Deref @ Expression::ARRAY { .. } => Expression::mapArrayElements(exp1, (std::sync::Arc::new(evalLogicUnaryNot) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
        _ => {
            exp = Arc::new(Expression::NFExpression::LUNARY { operator: Operator::makeNot(crate::NFType::interned_UNKNOWN()), exp: exp1 });
            printFailedEvalError(literal!("NFCeval.evalLogicUnaryNot"), exp, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalRelationOp(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    exp = Expression::mapSplitExpressions(Arc::new(Expression::NFExpression::RELATION { exp1: exp1, operator: op, exp2: exp2, index: -1 }), (std::sync::Arc::new(evalRelationExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(exp)
}

pub(crate) fn evalRelationExp(mut relationExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    let mut op: Arc<Operator::NFOperator>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(relationExp) {
        Deref @ Expression::RELATION { exp1: __pa0, operator: __pa1, exp2: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e1 = __pa0.clone();
    op = __pa1.clone();
    e2 = __pa2.clone();
    result = evalRelationOp_dispatch(e1, op, e2)?;
    Ok(result)
}

pub(crate) fn evalRelationOp_dispatch(mut exp1: Arc<Expression::NFExpression>, mut op: Arc<Operator::NFOperator>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    let mut res: bool;
    res = (match op.op.clone() {
        Operator::Op::LESS => evalRelationLess(exp1, exp2)?,
        Operator::Op::LESSEQ => evalRelationLessEq(exp1, exp2)?,
        Operator::Op::GREATER => evalRelationGreater(exp1, exp2)?,
        Operator::Op::GREATEREQ => evalRelationGreaterEq(exp1, exp2)?,
        Operator::Op::EQUAL => evalRelationEqual(exp1, exp2)?,
        Operator::Op::NEQUAL => evalRelationNotEqual(exp1, exp2)?,
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalRelationOp_dispatch")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*Expression::toString(Arc::new(Expression::NFExpression::RELATION { exp1: exp1, operator: op, exp2: exp2, index: -1 }))?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
    });
    exp = Arc::new(Expression::NFExpression::BOOLEAN { value: res });
    Ok(exp)
}

pub(crate) fn evalRelationLess(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() < var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() < var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() < var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) < 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) < 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) < 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) < 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() < var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError(literal!("NFCeval.evalRelationLess"), Arc::new(Expression::NFExpression::RELATION { exp1: exp1, operator: Operator::makeLess(crate::NFType::interned_UNKNOWN()), exp2: exp2, index: -1 }), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn evalRelationLessEq(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() <= var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() <= var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() <= var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) <= 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) <= 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) <= 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) <= 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() <= var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError(literal!("NFCeval.evalRelationLessEq"), Arc::new(Expression::NFExpression::RELATION { exp1: exp1, operator: Operator::makeLessEq(crate::NFType::interned_UNKNOWN()), exp2: exp2, index: -1 }), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn evalRelationGreater(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() > var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() > var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() > var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) > 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) > 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) > 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) > 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() > var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError(literal!("NFCeval.evalRelationGreater"), Arc::new(Expression::NFExpression::RELATION { exp1: exp1, operator: Operator::makeGreater(crate::NFType::interned_UNKNOWN()), exp2: exp2, index: -1 }), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn evalRelationGreaterEq(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() >= var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() >= var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() >= var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) >= 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) >= 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) >= 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) >= 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() >= var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError(literal!("NFCeval.evalRelationGreaterEq"), Arc::new(Expression::NFExpression::RELATION { exp1: exp1, operator: Operator::makeGreaterEq(crate::NFType::interned_UNKNOWN()), exp2: exp2, index: -1 }), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn evalRelationEqual(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() == var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() == var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() == var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) == 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) == 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) == 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) == 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() == var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError(literal!("NFCeval.evalRelationEqual"), Arc::new(Expression::NFExpression::RELATION { exp1: exp1, operator: Operator::makeEqual(crate::NFType::interned_UNKNOWN()), exp2: exp2, index: -1 }), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn evalRelationNotEqual(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() != var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone(),
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => var_field!((*exp1).value, Expression::NFExpression::REAL).clone() != var_field!((*exp2).value, Expression::NFExpression::REAL).clone(),
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() != var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone(),
        (Deref @ Expression::STRING { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) != 0,
        (Deref @ Expression::STRING { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).value, Expression::NFExpression::STRING).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) != 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::STRING { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).value, Expression::NFExpression::STRING).clone()).clone()) != 0,
        (Deref @ Expression::FILENAME { .. }, Deref @ Expression::FILENAME { .. }) => stringCompare((var_field!((*exp1).filename, Expression::NFExpression::FILENAME).clone()).clone(), (var_field!((*exp2).filename, Expression::NFExpression::FILENAME).clone()).clone()) != 0,
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() != var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printFailedEvalError(literal!("NFCeval.evalRelationNotEqual"), Arc::new(Expression::NFExpression::RELATION { exp1: exp1, operator: Operator::makeNotEqual(crate::NFType::interned_UNKNOWN()), exp2: exp2, index: -1 }), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn evalIfExp(mut ifExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut ty: Arc<Type::NFType>;
    let mut cond: Arc<Expression::NFExpression>;
    let mut btrue: Arc<Expression::NFExpression>;
    let mut bfalse: Arc<Expression::NFExpression>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(ifExp) {
        Deref @ Expression::IF { ty: __pa0, condition: __pa1, trueBranch: __pa2, falseBranch: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    cond = __pa1.clone();
    btrue = __pa2.clone();
    bfalse = __pa3.clone();
    result = Arc::new(Expression::NFExpression::IF { ty: ty, condition: evalExp(cond, target.clone())?, trueBranch: btrue, falseBranch: bfalse });
    result = Expression::mapSplitExpressions(result, (std::sync::Arc::new({ let __pe_b1 = target; move |__pe_a0| evalIfExp2(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(result)
}

pub(crate) fn evalIfExp2(mut ifExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut ty: Arc<Type::NFType>;
    let mut cond: Arc<Expression::NFExpression>;
    let mut tb: Arc<Expression::NFExpression>;
    let mut fb: Arc<Expression::NFExpression>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(ifExp.clone()) {
        Deref @ Expression::IF { ty: __pa0, condition: __pa1, trueBranch: __pa2, falseBranch: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    cond = __pa1.clone();
    tb = __pa2.clone();
    fb = __pa3.clone();
    result = (::match_deref::match_deref! { match &(cond.clone()) {
        Deref @ Expression::BOOLEAN { .. } => {
            if Type::isConditionalArray(ty.clone()) && !(Type::isMatchedBranch(var_field!((*cond).value, Expression::NFExpression::BOOLEAN).clone(), ty)?) {
                (tb, fb) = Util::swap(var_field!((*cond).value, Expression::NFExpression::BOOLEAN).clone(), fb, tb);
                Error::addSourceMessage(Error::ARRAY_DIMENSION_MISMATCH.clone(), list![(Expression::toString(tb.clone())?).clone(), (Type::toString(Expression::typeOf(tb.clone()))?).clone(), (Dimension::toStringList(Type::arrayDims(Expression::typeOf(fb.clone())), false)?).clone()], EvalTarget::getInfo(target.clone()))?;
                bail!("fail");
            }
            evalExp(if (var_field!((*cond).value, Expression::NFExpression::BOOLEAN).clone()) {tb} else {fb}, target)?
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalIfExp2")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*Expression::toString(ifExp)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalCast(mut castExp: Arc<Expression::NFExpression>, mut castTy: Arc<Type::NFType>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    exp = Expression::typeCast(castExp.clone(), castTy.clone())?;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CAST { .. } => {
            exp = Arc::new(Expression::NFExpression::CAST { ty: castTy, exp: castExp });
            printFailedEvalError(literal!("NFCeval.evalCast"), exp.clone(), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalCall(mut call: Arc<Call::NFCall>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    let mut c: Arc<Call::NFCall> = call.clone();
    exp = (::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Call::TYPED_CALL { .. } => {
            assign_variant_field!(c => Call::NFCall::TYPED_CALL; arguments = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut arg in (var_field!((*c).arguments, Call::NFCall::TYPED_CALL).clone()).into_iter().cloned() {
            let __x = evalExp(arg.clone(), target.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            if (Function::isBuiltin(var_field!((*c).r#fn, Call::NFCall::TYPED_CALL).clone())) {Expression::mapSplitExpressions(Arc::new(Expression::NFExpression::CALL { call: c }), (std::sync::Arc::new({ let __pe_b1 = target; move |__pe_a0| evalBuiltinCallExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?} else {Expression::mapSplitExpressions(Arc::new(Expression::NFExpression::CALL { call: c }), (std::sync::Arc::new({ let __pe_b1 = target; move |__pe_a0| evalNormalCallExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?}
        },
        Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } => {
            assign_variant_field!(c => Call::NFCall::TYPED_ARRAY_CONSTRUCTOR;
                exp = evalExpPartial(var_field!((*c).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), noTarget().clone(), true)?.0,
                iters = Call::mapIteratorsExpShallow(var_field!((*c).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), (std::sync::Arc::new(evalExpPartialDefault) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?
            );
            Expression::mapSplitExpressions(Arc::new(Expression::NFExpression::CALL { call: c }), (std::sync::Arc::new(evalArrayConstructor) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?
        },
        Deref @ Call::TYPED_REDUCTION { .. } => {
            assign_variant_field!(c => Call::NFCall::TYPED_REDUCTION;
                exp = evalExpPartial(var_field!((*c).exp, Call::NFCall::TYPED_REDUCTION).clone(), noTarget().clone(), true)?.0,
                iters = Call::mapIteratorsExpShallow(var_field!((*c).iters, Call::NFCall::TYPED_REDUCTION).clone(), (std::sync::Arc::new(evalExpPartialDefault) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?
            );
            Expression::mapSplitExpressions(Arc::new(Expression::NFExpression::CALL { call: c }), (std::sync::Arc::new(evalReduction) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalCall")); __mm_s.push_str(&*literal!(" got untyped call")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn evalBuiltinCallExp(mut callExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut r#fn: Arc<Function::Function>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(callExp) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { r#fn: __pa0, arguments: __pa1, .. } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa0.clone();
    args = __pa1.clone();
    result = evalBuiltinCall(r#fn, args, target)?;
    Ok(result)
}

pub(crate) fn evalBuiltinCall(mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut fn_path: Arc<Absyn::Path> = Function::nameConsiderBuiltin(r#fn.clone())?;
    result = (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(fn_path.clone())?) {
        Deref @ "abs" => evalBuiltinAbs(listHead(args)?)?,
        Deref @ "acos" => evalBuiltinAcos(listHead(args)?, target)?,
        Deref @ "array" => evalBuiltinArray(args)?,
        Deref @ "asin" => evalBuiltinAsin(listHead(args)?, target)?,
        Deref @ "atan2" => evalBuiltinAtan2(args)?,
        Deref @ "atan" => evalBuiltinAtan(listHead(args)?)?,
        Deref @ "cat" => evalBuiltinCat(listHead(args.clone())?, listRest(args)?, target)?,
        Deref @ "ceil" => evalBuiltinCeil(listHead(args)?)?,
        Deref @ "cosh" => evalBuiltinCosh(listHead(args)?)?,
        Deref @ "cos" => evalBuiltinCos(listHead(args)?)?,
        Deref @ "der" => evalBuiltinDer(listHead(args)?)?,
        Deref @ "diagonal" => evalBuiltinDiagonal(Expression::unbox(listHead(args)?))?,
        Deref @ "div" => evalBuiltinDiv(args, target)?,
        Deref @ "exp" => evalBuiltinExp(listHead(args)?)?,
        Deref @ "fill" => evalBuiltinFill(args)?,
        Deref @ "floor" => evalBuiltinFloor(listHead(args)?)?,
        Deref @ "identity" => evalBuiltinIdentity(listHead(args)?)?,
        Deref @ "integer" => evalBuiltinInteger(listHead(args)?)?,
        Deref @ "Integer" => evalBuiltinIntegerEnum(listHead(args)?)?,
        Deref @ "log10" => evalBuiltinLog10(listHead(args)?, target)?,
        Deref @ "log" => evalBuiltinLog(listHead(args)?, target)?,
        Deref @ "matrix" => evalBuiltinMatrix(listHead(args)?)?,
        Deref @ "max" => evalBuiltinMax(args, r#fn)?,
        Deref @ "min" => evalBuiltinMin(args, r#fn)?,
        Deref @ "mod" => evalBuiltinMod(args, target)?,
        Deref @ "noEvent" => listHead(args)?,
        Deref @ "ones" => evalBuiltinOnes(args)?,
        Deref @ "pre" => listHead(args)?,
        Deref @ "product" => evalBuiltinProduct(listHead(args)?)?,
        Deref @ "promote" => evalBuiltinPromote((args.clone()).get(1)?, (args).get(2)?)?,
        Deref @ "rem" => evalBuiltinRem(args, target)?,
        Deref @ "scalar" => evalBuiltinScalar(listHead(args)?)?,
        Deref @ "sign" => evalBuiltinSign(listHead(args)?)?,
        Deref @ "sinh" => evalBuiltinSinh(listHead(args)?)?,
        Deref @ "sin" => evalBuiltinSin(listHead(args)?)?,
        Deref @ "skew" => evalBuiltinSkew(listHead(args)?)?,
        Deref @ "smooth" => (args).get(2)?,
        Deref @ "sqrt" => evalBuiltinSqrt(listHead(args)?)?,
        Deref @ "String" => evalBuiltinString(args)?,
        Deref @ "sum" => evalBuiltinSum(listHead(args)?)?,
        Deref @ "symmetric" => evalBuiltinSymmetric(listHead(args)?)?,
        Deref @ "tanh" => evalBuiltinTanh(listHead(args)?)?,
        Deref @ "tan" => evalBuiltinTan(listHead(args)?)?,
        Deref @ "transpose" => evalBuiltinTranspose(listHead(args)?)?,
        Deref @ "vector" => evalBuiltinVector(listHead(args)?),
        Deref @ "zeros" => evalBuiltinZeros(args)?,
        Deref @ "OpenModelica_uriToFilename" => evalUriToFilename(r#fn, listHead(args)?, target)?,
        Deref @ "intBitAnd" => evalIntBitAnd(args)?,
        Deref @ "intBitOr" => evalIntBitOr(args)?,
        Deref @ "intBitXor" => evalIntBitXor(args)?,
        Deref @ "intBitLShift" => evalIntBitLShift(args)?,
        Deref @ "intBitRShift" => evalIntBitRShift(args)?,
        Deref @ "intMaxLit" => Arc::new(Expression::NFExpression::INTEGER { value: System::intMaxLit() }),
        Deref @ "inferredClock" => evalInferredClock(args)?,
        Deref @ "rationalClock" => evalRationalClock(args)?,
        Deref @ "realClock" => evalRealClock(args)?,
        Deref @ "booleanClock" => evalBooleanClock(args)?,
        Deref @ "solverClock" => evalSolverClock(args)?,
        Deref @ "$OMC$PositiveMax" => evalPositiveMax((args.clone()).get(1)?, (args).get(2)?)?,
        Deref @ "$OMC$inStreamDiv" => listHead(args)?,
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalBuiltinCall")); __mm_s.push_str(&*literal!(": unimplemented case for ")); __mm_s.push_str(&*AbsynUtil::pathString(fn_path, (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalNormalCallExp(mut callExp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut r#fn: Arc<Function::Function>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(callExp) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { r#fn: __pa0, arguments: __pa1, .. } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa0.clone();
    args = __pa1.clone();
    result = evalNormalCall(r#fn, args, target)?;
    Ok(result)
}

pub(crate) fn evalNormalCall(mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = EvalFunction::evaluate(r#fn.clone(), args.clone(), target.clone())?;
    Ok(result)
}

pub(crate) fn evalBuiltinAbs(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::INTEGER { .. } => Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*arg).value, Expression::NFExpression::INTEGER).clone().abs() }),
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: var_field!((*arg).value, Expression::NFExpression::REAL).clone().abs() }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinAbs"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinAcos(mut arg: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut x: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { value: __esc_x } => {
            x = (*__esc_x).clone();
            if x.clone() < metamodelica::OrderedFloat(-1.0_f64) || x.clone() > metamodelica::OrderedFloat(1.0_f64) {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::ARGUMENT_OUT_OF_RANGE.clone(), list![ArcStr::from(::std::format!("{}", x.clone())), (literal!("acos")).clone(), (literal!("-1 <= x <= 1")).clone()], EvalTarget::getInfo(target))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::REAL { value: (x.clone()).acos() })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinAcos"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinArray(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut ty: Arc<Type::NFType>;
    ty = Expression::typeOf(listHead(args.clone())?);
    ty = Type::liftArrayLeft(ty, Dimension::fromInteger((args.clone().len() as i32), Prefixes::Variability::CONSTANT.clone()));
    result = Expression::makeArray(ty, metamodelica::arrayFromVec(args.into_iter().cloned().collect()), true);
    Ok(result)
}

pub(crate) fn evalBuiltinAsin(mut arg: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut x: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { value: __esc_x } => {
            x = (*__esc_x).clone();
            if x.clone() < metamodelica::OrderedFloat(-1.0_f64) || x.clone() > metamodelica::OrderedFloat(1.0_f64) {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::ARGUMENT_OUT_OF_RANGE.clone(), list![ArcStr::from(::std::format!("{}", x.clone())), (literal!("asin")).clone(), (literal!("-1 <= x <= 1")).clone()], EvalTarget::getInfo(target))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::REAL { value: (x.clone()).asin() })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinAsin"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinAtan2(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut y: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut x: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::REAL { value: __esc_y }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::REAL { value: __esc_x }, tail: Deref @ metamodelica::List::Nil } } => {
            y = (*__esc_y).clone();
            x = (*__esc_x).clone();
            Arc::new(Expression::NFExpression::REAL { value: (y.clone()).atan2(x.clone()) })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinAtan2"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinAtan(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).atan() }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinAtan"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinCat(mut argN: Arc<Expression::NFExpression>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut n: i32;
    let mut nd: i32;
    let mut sz: i32;
    let mut ty: Arc<Type::NFType>;
    let mut es: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut dims: Arc<metamodelica::List<i32>>;
    let __pa0 = ::match_deref::match_deref! { match &(argN) {
        Deref @ Expression::INTEGER { value: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    n = __pa0.clone();
    ty = Expression::typeOf(listHead(args.clone())?);
    nd = Type::dimensionCount(ty);
    if n > nd || n < 1 {
        if EvalTarget::hasInfo(target.clone()) {
            Error::addSourceMessage(Error::ARGUMENT_OUT_OF_RANGE.clone(), list![ArcStr::from(::std::format!("{}", n)), (literal!("cat")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1 <= x <= ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", nd))); ArcStr::from(__mm_s) }).clone()], EvalTarget::getInfo(target))?;
        }
        bail!("fail");
    }
    es = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (args.clone()).into_iter().cloned() {
            if !(!(Expression::isEmptyArray(e.clone()))) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    sz = (es.clone().len() as i32);
    if sz == 0 {
        result = listHead(args)?;
    } else if sz == 1 {
        result = listHead(es)?;
    } else {
        (es, dims) = ExpressionBasics::evalCat(n, es, (std::sync::Arc::new(Expression::arrayElementList) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<metamodelica::List<Arc<Expression::NFExpression>>>> + 'static>), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>))?;
        result = Expression::arrayFromList(es.clone(), Expression::typeOf(listHead(es)?), ({
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut d in (dims).into_iter().cloned() {
            let __x = Dimension::fromInteger(d.clone(), Prefixes::Variability::CONSTANT.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
    }
    Ok(result)
}

pub(crate) fn evalBuiltinCeil(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).ceil() }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinCeil"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinCosh(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).cosh() }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinCosh"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinCos(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).cos() }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinCos"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinDer(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = Expression::fillType(Expression::typeOf(arg), Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }))?;
    Ok(result)
}

pub(crate) fn evalBuiltinDiagonal(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut elem_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut row_ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut zero: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut n: i32 = 0;
    let mut i: i32 = 1;
    let mut e_lit: bool = false;
    let mut arg_lit: bool = true;
    let mut arr_zero: metamodelica::Array<Arc<Expression::NFExpression>> = Default::default();
    let mut arr_row: metamodelica::Array<Arc<Expression::NFExpression>> = Default::default();
    let mut arr_rows: metamodelica::Array<Arc<Expression::NFExpression>> = Default::default();
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::ARRAY { .. } if (var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone().borrow().is_empty()) => arg.clone(),
        Deref @ Expression::ARRAY { .. } => {
            n = metamodelica::arrayLength(var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone());
            elem_ty = Type::unliftArray(var_field!((*arg).ty, Expression::NFExpression::ARRAY).clone())?;
            row_ty = Type::liftArrayLeft(elem_ty.clone(), Dimension::fromInteger(n, Prefixes::Variability::CONSTANT.clone()));
            zero = Expression::makeZero(elem_ty)?;
            arr_zero = arrayCreate(n, zero.clone());
            arr_rows = metamodelica::arrayCreate(n, zero);
            for mut i in 1..=n {
                arr_row = metamodelica::arrayFromVec(arr_zero.clone().borrow().clone());
                exp = metamodelica::Dangerous::arrayGetNoBoundsChecking(var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(), i);
                e_lit = Expression::isLiteral(exp.clone())?;
                arg_lit = arg_lit && e_lit;
                metamodelica::Dangerous::arrayUpdateNoBoundsChecking(arr_row.clone(), i, exp.clone());
                exp = Expression::makeArray(row_ty.clone(), arr_row.clone(), e_lit);
                unsafe { metamodelica::Dangerous::arrayInitSlot(arr_rows.clone(), i, exp.clone()) };
            }
            Expression::makeArray(Type::liftArrayLeft(row_ty, Dimension::fromInteger(n, Prefixes::Variability::CONSTANT.clone())), arr_rows.clone(), arg_lit)
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinDiagonal"), list![arg.clone()], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinDiv(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut rx: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut ry: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __esc_ix }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __esc_iy }, tail: Deref @ metamodelica::List::Nil } } => {
            ix = (*__esc_ix).clone();
            iy = (*__esc_iy).clone();
            if iy.clone() == 0 {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::DIVISION_BY_ZERO.clone(), list![ArcStr::from(::std::format!("{}", ix.clone())), ArcStr::from(::std::format!("{}", iy.clone()))], EvalTarget::getInfo(target))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::INTEGER { value: intDiv(ix.clone(), iy.clone()) })
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::REAL { value: __esc_rx }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::REAL { value: __esc_ry }, tail: Deref @ metamodelica::List::Nil } } => {
            rx = (*__esc_rx).clone();
            ry = (*__esc_ry).clone();
            if ry.clone() == metamodelica::OrderedFloat(0.0_f64) {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::DIVISION_BY_ZERO.clone(), list![ArcStr::from(::std::format!("{}", rx.clone())), ArcStr::from(::std::format!("{}", ry.clone()))], EvalTarget::getInfo(target))?;
                }
                bail!("fail");
            }
            rx = rx.clone() / ry.clone();
            Arc::new(Expression::NFExpression::REAL { value: if (rx.clone() < metamodelica::OrderedFloat(0.0_f64)) {(rx.clone()).ceil()} else {(rx.clone()).floor()} })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinDiv"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinExp(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).exp() }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinExp"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinFill(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut fill_exp: Arc<Expression::NFExpression>;
    let mut dims: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        fill_exp = __pa1.clone();
        dims = __pa2.clone();
        result = unwrap_break_err!(Expression::fillArgs(fill_exp.clone(), dims.clone()), '__try0);
        Ok::<_, anyhow::Error>((dims.clone(), fill_exp.clone(), result.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            dims = __try0_o0;
            fill_exp = __try0_o1;
            result = __try0_o2;
        }
        Err(__try0_err) => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinFill"), args.clone(), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            return Err(__try0_err);
        }
    }
    Ok(result)
}

fn evalBuiltinFloor(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).floor() }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinFloor"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinIdentity(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::INTEGER { .. } => Expression::makeIdentityMatrix(var_field!((*arg).value, Expression::NFExpression::INTEGER).clone(), crate::NFType::interned_INTEGER())?,
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinIdentity"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinInteger(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::INTEGER { .. } => arg,
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::INTEGER { value: ((var_field!((*arg).value, Expression::NFExpression::REAL).clone()).0.floor() as i32) }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinInteger"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinIntegerEnum(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::ENUM_LITERAL { .. } => Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*arg).index, Expression::NFExpression::ENUM_LITERAL).clone() }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinIntegerEnum"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinLog10(mut arg: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut x: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { value: __esc_x } => {
            x = (*__esc_x).clone();
            if x.clone() <= metamodelica::OrderedFloat(0.0_f64) {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::ARGUMENT_OUT_OF_RANGE.clone(), list![ArcStr::from(::std::format!("{}", x.clone())), (literal!("log10")).clone(), (literal!("x > 0")).clone()], EvalTarget::getInfo(target))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::REAL { value: (x.clone()).log10() })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinLog10"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinLog(mut arg: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut x: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { value: __esc_x } => {
            x = (*__esc_x).clone();
            if x.clone() <= metamodelica::OrderedFloat(0.0_f64) {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::ARGUMENT_OUT_OF_RANGE.clone(), list![ArcStr::from(::std::format!("{}", x.clone())), (literal!("log")).clone(), (literal!("x > 0")).clone()], EvalTarget::getInfo(target))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::REAL { value: (x.clone()).ln() })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinLog"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinMatrix(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::ARRAY { ty, .. } => {
            let mut dim_count: i32;
            let mut dim1: Arc<Dimension::NFDimension>;
            let mut dim2: Arc<Dimension::NFDimension>;
            let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
            let mut ty = (*ty).clone();
            dim_count = Type::dimensionCount(ty.clone());
            if dim_count.clone() < 2 {
                (result, _) = Expression::promote(arg, ty.clone(), 2)?;
            } else if dim_count.clone() == 2 {
                result = arg;
            } else {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Type::arrayDims(ty.clone())) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: _ } } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                dim1 = __pa0.clone();
                dim2 = __pa1.clone();
                ty = Type::liftArrayLeft(Type::arrayElementType(ty.clone()), dim2.clone());
                arr = Array::map(var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1 = ty.clone(); move |__pe_a0| evalBuiltinMatrix2(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
                ty = Type::liftArrayLeft(ty.clone(), dim1.clone());
                result = Expression::makeArray(ty.clone(), arr.clone(), false);
            }
            result
        },
        _ => {
            let mut ty: Arc<Type::NFType>;
            ty = Expression::typeOf(arg.clone());
            if Type::isScalar(ty.clone()) {
                (result, _) = Expression::promote(arg, ty.clone(), 2)?;
            } else {
                printWrongArgsError(literal!("NFCeval.evalBuiltinMatrix"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
                bail!("fail");
            }
            result
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinMatrix2(mut arg: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::ARRAY { .. } => Expression::makeArray(ty, Array::map(var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(), (std::sync::Arc::new(Expression::toScalar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, var_field!((*arg).literal, Expression::NFExpression::ARRAY).clone()),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinMatrix2"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinMax(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut r#fn: Arc<Function::Function>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __esc_e1, tail: Deref @ metamodelica::List::Cons { head: __esc_e2, tail: Deref @ metamodelica::List::Nil } } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            evalBuiltinMax2(e1.clone(), e2.clone())?
        },
        Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } if (Expression::isArray(e1.clone())) => {
            ty = Expression::typeOf(e1.clone());
            result = Expression::fold(e1.clone(), (std::sync::Arc::new(evalBuiltinMax2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), Arc::new(Expression::NFExpression::EMPTY { ty: ty.clone() }))?;
            if Expression::isEmpty(result.clone()) {
                result = Expression::makeMinValue(Type::arrayElementType(ty))?;
            }
            result
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinMax"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinMax2(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() < var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) {exp2} else {exp1},
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::REAL).clone() < var_field!((*exp2).value, Expression::NFExpression::REAL).clone()) {exp2} else {exp1},
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() < var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone()) {exp2} else {exp1},
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => if (var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() < var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone()) {exp2} else {exp1},
        (Deref @ Expression::ARRAY { .. }, _) => exp2,
        (_, Deref @ Expression::EMPTY { .. }) => exp1,
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinMax2"), list![exp1, exp2], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalPositiveMax(mut flow_exp: Arc<Expression::NFExpression>, mut eps: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = if (Expression::isNonPositive(flow_exp.clone())?) {Expression::makeZero(Expression::typeOf(flow_exp))?} else {evalBuiltinMax2(flow_exp, eps)?};
    Ok(result)
}

fn evalBuiltinMin(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut r#fn: Arc<Function::Function>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __esc_e1, tail: Deref @ metamodelica::List::Cons { head: __esc_e2, tail: Deref @ metamodelica::List::Nil } } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            evalBuiltinMin2(e1.clone(), e2.clone())?
        },
        Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } if (Expression::isArray(e1.clone())) => {
            ty = Expression::typeOf(e1.clone());
            result = Expression::fold(e1.clone(), (std::sync::Arc::new(evalBuiltinMin2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), Arc::new(Expression::NFExpression::EMPTY { ty: ty.clone() }))?;
            if Expression::isEmpty(result.clone()) {
                result = Expression::makeMaxValue(Type::arrayElementType(ty))?;
            }
            result
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinMin"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalBuiltinMin2(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::INTEGER).clone() > var_field!((*exp2).value, Expression::NFExpression::INTEGER).clone()) {exp2} else {exp1},
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::REAL).clone() > var_field!((*exp2).value, Expression::NFExpression::REAL).clone()) {exp2} else {exp1},
        (Deref @ Expression::BOOLEAN { .. }, Deref @ Expression::BOOLEAN { .. }) => if (var_field!((*exp1).value, Expression::NFExpression::BOOLEAN).clone() > var_field!((*exp2).value, Expression::NFExpression::BOOLEAN).clone()) {exp2} else {exp1},
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ Expression::ENUM_LITERAL { .. }) => if (var_field!((*exp1).index, Expression::NFExpression::ENUM_LITERAL).clone() > var_field!((*exp2).index, Expression::NFExpression::ENUM_LITERAL).clone()) {exp2} else {exp1},
        (Deref @ Expression::ARRAY { .. }, _) => exp2,
        (_, Deref @ Expression::EMPTY { .. }) => exp1,
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinMin2"), list![exp1, exp2], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinMod(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut x: Arc<Expression::NFExpression>;
    let mut y: Arc<Expression::NFExpression>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    x = __pa0.clone();
    y = __pa1.clone();
    result = (::match_deref::match_deref! { match &((x.clone(), y.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => {
            if var_field!((*y).value, Expression::NFExpression::INTEGER).clone() == 0 {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::MODULO_BY_ZERO.clone(), list![ArcStr::from(::std::format!("{}", var_field!((*x).value, Expression::NFExpression::INTEGER).clone())), ArcStr::from(::std::format!("{}", var_field!((*y).value, Expression::NFExpression::INTEGER).clone()))], EvalTarget::getInfo(target))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::INTEGER { value: intMod(var_field!((*x).value, Expression::NFExpression::INTEGER).clone(), var_field!((*y).value, Expression::NFExpression::INTEGER).clone()) })
        },
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => {
            if var_field!((*y).value, Expression::NFExpression::REAL).clone() == metamodelica::OrderedFloat(0.0_f64) {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::MODULO_BY_ZERO.clone(), list![ArcStr::from(::std::format!("{}", var_field!((*x).value, Expression::NFExpression::REAL).clone())), ArcStr::from(::std::format!("{}", var_field!((*y).value, Expression::NFExpression::REAL).clone()))], EvalTarget::getInfo(target))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::REAL { value: realMod(var_field!((*x).value, Expression::NFExpression::REAL).clone(), var_field!((*y).value, Expression::NFExpression::REAL).clone()) })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinMod"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinOnes(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = evalBuiltinFill(metamodelica::cons(Arc::new(Expression::NFExpression::INTEGER { value: 1 }), args))?;
    Ok(result)
}

fn evalBuiltinProduct(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        _ if (Expression::isArray(arg.clone())) => (::match_deref::match_deref! { match &(Type::arrayElementType(Expression::typeOf(arg.clone()))) {
        Deref @ Type::INTEGER => Arc::new(Expression::NFExpression::INTEGER { value: Expression::fold(arg.clone(), (std::sync::Arc::new(evalBuiltinProductInt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, i32) -> Result<i32> + 'static>), 1)? }),
        Deref @ Type::REAL => Arc::new(Expression::NFExpression::REAL { value: Expression::fold(arg.clone(), (std::sync::Arc::new(evalBuiltinProductReal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(1.0_f64))? }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinProduct"), list![arg.clone()], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinProduct"), list![arg.clone()], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinProductInt(mut exp: Arc<Expression::NFExpression>, mut result: i32) -> Result<i32> {
    let mut result: i32 = result;
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => result * var_field!((*exp).value, Expression::NFExpression::INTEGER).clone(),
        Deref @ Expression::ARRAY { .. } => result,
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinProductReal(mut exp: Arc<Expression::NFExpression>, mut result: metamodelica::Real) -> Result<metamodelica::Real> {
    let mut result: metamodelica::Real = result;
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::REAL { .. } => result * var_field!((*exp).value, Expression::NFExpression::REAL).clone(),
        Deref @ Expression::ARRAY { .. } => result,
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinPromote(mut arg: Arc<Expression::NFExpression>, mut argN: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut n: i32;
    if Expression::isInteger(argN.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(argN) {
            Deref @ Expression::INTEGER { value: __pa0 } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        n = __pa0.clone();
        (result, _) = Expression::promote(arg.clone(), Expression::typeOf(arg), n)?;
    } else {
        printWrongArgsError(literal!("NFCeval.evalBuiltinPromote"), list![arg, argN], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
        bail!("fail");
    }
    Ok(result)
}

fn evalBuiltinRem(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut x: Arc<Expression::NFExpression>;
    let mut y: Arc<Expression::NFExpression>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    x = __pa0.clone();
    y = __pa1.clone();
    result = (::match_deref::match_deref! { match &((x.clone(), y.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => {
            if var_field!((*y).value, Expression::NFExpression::INTEGER).clone() == 0 {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::REM_ARG_ZERO.clone(), list![ArcStr::from(::std::format!("{}", var_field!((*x).value, Expression::NFExpression::INTEGER).clone())), ArcStr::from(::std::format!("{}", var_field!((*y).value, Expression::NFExpression::INTEGER).clone()))], EvalTarget::getInfo(target))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*x).value, Expression::NFExpression::INTEGER).clone() - intDiv(var_field!((*x).value, Expression::NFExpression::INTEGER).clone(), var_field!((*y).value, Expression::NFExpression::INTEGER).clone()) * var_field!((*y).value, Expression::NFExpression::INTEGER).clone() })
        },
        (Deref @ Expression::REAL { .. }, Deref @ Expression::REAL { .. }) => {
            if var_field!((*y).value, Expression::NFExpression::REAL).clone() == metamodelica::OrderedFloat(0.0_f64) {
                if EvalTarget::hasInfo(target.clone()) {
                    Error::addSourceMessage(Error::REM_ARG_ZERO.clone(), list![ArcStr::from(::std::format!("{}", var_field!((*x).value, Expression::NFExpression::REAL).clone())), ArcStr::from(::std::format!("{}", var_field!((*y).value, Expression::NFExpression::REAL).clone()))], EvalTarget::getInfo(target))?;
                }
                bail!("fail");
            }
            Arc::new(Expression::NFExpression::REAL { value: var_field!((*x).value, Expression::NFExpression::REAL).clone() - realDiv(var_field!((*x).value, Expression::NFExpression::REAL).clone(), var_field!((*y).value, Expression::NFExpression::REAL).clone()) * var_field!((*y).value, Expression::NFExpression::REAL).clone() })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinRem"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinScalar(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = arg.clone();
    while Expression::isArray(result.clone()) {
        result = Expression::arrayScalarElement(result.clone())?;
    }
    Ok(result)
}

fn evalBuiltinSign(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::INTEGER { value: if (var_field!((*arg).value, Expression::NFExpression::REAL).clone() > metamodelica::OrderedFloat((0) as f64)) {1} else {if (var_field!((*arg).value, Expression::NFExpression::REAL).clone() < metamodelica::OrderedFloat((0) as f64)) {-1} else {0}} }),
        Deref @ Expression::INTEGER { .. } => Arc::new(Expression::NFExpression::INTEGER { value: if (var_field!((*arg).value, Expression::NFExpression::INTEGER).clone() > 0) {1} else {if (var_field!((*arg).value, Expression::NFExpression::INTEGER).clone() < 0) {-1} else {0}} }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinSign"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinSinh(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).sinh() }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinSinh"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinSin(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).sin() }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinSin"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinSkew(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut x1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut x2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut x3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut y1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut y2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut y3: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut zero: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut literal: bool = false;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::ARRAY { ty: __esc_ty, literal: __esc_literal, .. } => {
            ty = (*__esc_ty).clone();
            literal = (*__esc_literal).clone();
            x1 = metamodelica::arrayGet(var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(), 1)?;
            x2 = metamodelica::arrayGet(var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(), 2)?;
            x3 = metamodelica::arrayGet(var_field!((*arg).elements, Expression::NFExpression::ARRAY).clone(), 3)?;
            zero = Expression::makeZero(Type::arrayElementType(ty.clone()))?;
            y1 = Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(list![zero.clone(), Expression::negate(x3.clone()), x2.clone()].into_iter().cloned().collect()), literal.clone());
            y2 = Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(list![x3, zero.clone(), Expression::negate(x1.clone())].into_iter().cloned().collect()), literal.clone());
            y3 = Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(list![Expression::negate(x2), x1, zero].into_iter().cloned().collect()), literal.clone());
            ty = Type::liftArrayLeft(ty.clone(), Dimension::fromInteger(3, Prefixes::Variability::CONSTANT.clone()));
            Expression::makeArray(ty.clone(), metamodelica::arrayFromVec(list![y1, y2, y3].into_iter().cloned().collect()), literal.clone())
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinSkew"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinSqrt(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).sqrt() }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinSqrt"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinString(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: arg, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: min_len }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::BOOLEAN { value: left_justified }, tail: Deref @ metamodelica::List::Nil } } } => {
            let mut str_len: i32;
            let mut r#str: ArcStr;
            r#str = ((::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::INTEGER { .. } => intString(var_field!((**arg).value, Expression::NFExpression::INTEGER).clone()),
        Deref @ Expression::BOOLEAN { .. } => boolString(var_field!((**arg).value, Expression::NFExpression::BOOLEAN).clone()),
        Deref @ Expression::ENUM_LITERAL { .. } => var_field!((**arg).name, Expression::NFExpression::ENUM_LITERAL).clone(),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinString"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
            str_len = ((r#str.clone()).clone().len() as i32);
            if str_len.clone() < min_len.clone() {
                if left_justified.clone() {
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*stringAppendList(List::fill((literal!(" ")).clone(), min_len.clone() - str_len.clone()))); ArcStr::from(__mm_s) }).clone();
                } else {
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*stringAppendList(List::fill((literal!(" ")).clone(), min_len.clone() - str_len.clone()))); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
                }
            }
            Arc::new(Expression::NFExpression::STRING { value: (r#str.clone()).clone() })
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::REAL { value: r }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: significant_digits }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: min_len }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::BOOLEAN { value: left_justified }, tail: Deref @ metamodelica::List::Nil } } } } => {
            let mut r#str: ArcStr;
            let mut format: ArcStr;
            format = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("%")); __mm_s.push_str(&*if (left_justified.clone()) {literal!("-")} else {literal!("")}); __mm_s.push_str(&*intString(min_len.clone())); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*intString(significant_digits.clone())); __mm_s.push_str(&*literal!("g")); ArcStr::from(__mm_s) }).clone();
            r#str = (System::sprintff((format.clone()).clone(), r.clone())?).clone();
            Arc::new(Expression::NFExpression::STRING { value: (r#str.clone()).clone() })
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::REAL { value: r }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::STRING { value: format }, tail: Deref @ metamodelica::List::Nil } } => {
            let mut r#str: ArcStr;
            r#str = (System::sprintff(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("%")); __mm_s.push_str(&*format.clone()); ArcStr::from(__mm_s) }).clone(), r.clone())?).clone();
            Arc::new(Expression::NFExpression::STRING { value: (r#str.clone()).clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(result)
}

fn evalBuiltinSum(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        _ if (Expression::isArray(arg.clone())) => (::match_deref::match_deref! { match &(Type::arrayElementType(Expression::typeOf(arg.clone()))) {
        Deref @ Type::INTEGER => Arc::new(Expression::NFExpression::INTEGER { value: Expression::fold(arg.clone(), (std::sync::Arc::new(evalBuiltinSumInt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, i32) -> Result<i32> + 'static>), 0)? }),
        Deref @ Type::REAL => Arc::new(Expression::NFExpression::REAL { value: Expression::fold(arg.clone(), (std::sync::Arc::new(evalBuiltinSumReal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, metamodelica::Real) -> Result<metamodelica::Real> + 'static>), metamodelica::OrderedFloat(0.0_f64))? }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinSum"), list![arg.clone()], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinSum"), list![arg.clone()], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinSumInt(mut exp: Arc<Expression::NFExpression>, mut result: i32) -> Result<i32> {
    let mut result: i32 = result;
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => result + var_field!((*exp).value, Expression::NFExpression::INTEGER).clone(),
        Deref @ Expression::ARRAY { .. } => result,
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinSumReal(mut exp: Arc<Expression::NFExpression>, mut result: metamodelica::Real) -> Result<metamodelica::Real> {
    let mut result: metamodelica::Real = result;
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::REAL { .. } => result + var_field!((*exp).value, Expression::NFExpression::REAL).clone(),
        Deref @ Expression::ARRAY { .. } => result,
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinSymmetric(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut mat: metamodelica::Array<metamodelica::Array<Arc<Expression::NFExpression>>>;
    let mut n: i32;
    let mut ty: Arc<Type::NFType>;
    let mut row_ty: Arc<Type::NFType>;
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut accum: metamodelica::Array<Arc<Expression::NFExpression>>;
    ty = Expression::typeOf(arg.clone());
    if Expression::isArray(arg.clone()) && Type::isSquareMatrix(ty.clone())? {
        mat = Array::map(Expression::arrayElements(arg.clone())?, (std::sync::Arc::new(Expression::arrayElements) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<metamodelica::Array<Arc<Expression::NFExpression>>> + 'static>))?;
        n = metamodelica::arrayLength(mat.clone());
        row_ty = Type::unliftArray(Expression::typeOf(arg.clone()))?;
        accum = metamodelica::arrayCreate(n, arg.clone());
        for mut i in 1..=n {
            arr = metamodelica::arrayCreate(n, arg.clone());
            for mut j in 1..=n {
                unsafe { metamodelica::Dangerous::arrayInitSlot(arr.clone(), j.clone(), if (i.clone() > j.clone()) {metamodelica::arrayGet(({let __elt = mat.borrow()[(j.clone()-1) as usize].clone(); __elt}), i.clone())?} else {metamodelica::arrayGet(({let __elt = mat.borrow()[(i.clone()-1) as usize].clone(); __elt}), j.clone())?}) };
            }
            unsafe { metamodelica::Dangerous::arrayInitSlot(accum.clone(), i.clone(), Expression::makeArray(row_ty.clone(), arr.clone(), true)) };
        }
        result = Expression::makeArray(ty, accum.clone(), true);
    } else {
        printWrongArgsError(literal!("NFCeval.evalBuiltinSymmetric"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
        bail!("fail");
    }
    Ok(result)
}

fn evalBuiltinTanh(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).tanh() }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinTanh"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinTan(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::REAL { .. } => Arc::new(Expression::NFExpression::REAL { value: (var_field!((*arg).value, Expression::NFExpression::REAL).clone()).tan() }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBuiltinTan"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBuiltinTranspose(mut arg: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut ty: Arc<Type::NFType>;
    ty = Expression::typeOf(arg.clone());
    if Expression::isArray(arg.clone()) && Type::dimensionCount(ty) >= 2 {
        result = Expression::transposeArray(arg)?;
    } else {
        printWrongArgsError(literal!("NFCeval.evalBuiltinTranspose"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
        bail!("fail");
    }
    Ok(result)
}

fn evalBuiltinVector(mut arg: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression>;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    expl = Expression::arrayScalarElements(arg.clone());
    result = Expression::makeExpArray(metamodelica::arrayFromVec(expl.into_iter().cloned().collect()), Type::arrayElementType(Expression::typeOf(arg)), true);
    result
}

fn evalBuiltinZeros(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = evalBuiltinFill(metamodelica::cons(Arc::new(Expression::NFExpression::INTEGER { value: 0 }), args))?;
    Ok(result)
}

fn evalUriToFilename(mut r#fn: Arc<Function::Function>, mut arg: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::STRING { .. } => Arc::new(Expression::NFExpression::FILENAME { filename: uriToFilename((var_field!((*arg).value, Expression::NFExpression::STRING).clone()).clone())? }),
        Deref @ Expression::FILENAME { .. } => Arc::new(Expression::NFExpression::FILENAME { filename: uriToFilename((var_field!((*arg).filename, Expression::NFExpression::FILENAME).clone()).clone())? }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalUriToFilename"), list![arg], metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalIntBitAnd(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __esc_i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __esc_i2 }, tail: Deref @ metamodelica::List::Nil } } => {
            i1 = (*__esc_i1).clone();
            i2 = (*__esc_i2).clone();
            Arc::new(Expression::NFExpression::INTEGER { value: intBitAnd(i1.clone(), i2.clone()) })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalIntBitAnd"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalIntBitOr(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __esc_i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __esc_i2 }, tail: Deref @ metamodelica::List::Nil } } => {
            i1 = (*__esc_i1).clone();
            i2 = (*__esc_i2).clone();
            Arc::new(Expression::NFExpression::INTEGER { value: intBitOr(i1.clone(), i2.clone()) })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalIntBitOr"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalIntBitXor(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __esc_i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __esc_i2 }, tail: Deref @ metamodelica::List::Nil } } => {
            i1 = (*__esc_i1).clone();
            i2 = (*__esc_i2).clone();
            Arc::new(Expression::NFExpression::INTEGER { value: intBitXor(i1.clone(), i2.clone()) })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalIntBitXor"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalIntBitLShift(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __esc_i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __esc_i2 }, tail: Deref @ metamodelica::List::Nil } } => {
            i1 = (*__esc_i1).clone();
            i2 = (*__esc_i2).clone();
            Arc::new(Expression::NFExpression::INTEGER { value: intBitLShift(i1.clone(), i2.clone()) })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalIntBitLShift"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalIntBitRShift(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __esc_i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __esc_i2 }, tail: Deref @ metamodelica::List::Nil } } => {
            i1 = (*__esc_i1).clone();
            i2 = (*__esc_i2).clone();
            Arc::new(Expression::NFExpression::INTEGER { value: intBitRShift(i1.clone(), i2.clone()) })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalIntBitRShift"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalInferredClock(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Nil => Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(NFClockKind::NFClockKind::INFERRED_CLOCK { idx: System::tmpTickIndex(Global::inferredClock_index.clone()) }) }),
        _ => {
            printWrongArgsError(literal!("NFCeval.evalInferredClock"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalRationalClock(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: interval @ Deref @ Expression::INTEGER { .. }, tail: Deref @ metamodelica::List::Cons { head: resolution @ Deref @ Expression::INTEGER { .. }, tail: Deref @ metamodelica::List::Nil } } => {
            Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(NFClockKind::NFClockKind::RATIONAL_CLOCK { intervalCounter: interval.clone(), resolution: resolution.clone() }) })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalRationalClock"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalRealClock(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: interval @ Deref @ Expression::REAL { .. }, tail: Deref @ metamodelica::List::Nil } => {
            Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(NFClockKind::NFClockKind::REAL_CLOCK { interval: interval.clone() }) })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalRealClock"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalBooleanClock(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: condition @ Deref @ Expression::BOOLEAN { .. }, tail: Deref @ metamodelica::List::Cons { head: interval @ Deref @ Expression::REAL { .. }, tail: Deref @ metamodelica::List::Nil } } => {
            Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(NFClockKind::NFClockKind::EVENT_CLOCK { condition: condition.clone(), startInterval: interval.clone() }) })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalBooleanClock"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn evalSolverClock(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: c @ Deref @ Expression::CLKCONST { .. }, tail: Deref @ metamodelica::List::Cons { head: solver @ Deref @ Expression::STRING { .. }, tail: Deref @ metamodelica::List::Nil } } => {
            Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(NFClockKind::NFClockKind::SOLVER_CLOCK { c: c.clone(), solverMethod: solver.clone() }) })
        },
        _ => {
            printWrongArgsError(literal!("NFCeval.evalSolverClock"), args, metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn evalGetInstanceName(mut scope: Arc<InstNode::InstNode>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = Arc::new(Expression::NFExpression::STRING { value: (AbsynUtil::pathString(InstNode::rootPath(scope, false)?, (literal!(".")).clone(), true, false)?).clone() });
    Ok(result)
}

fn evalArrayConstructor(mut callExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut exp: Arc<Expression::NFExpression>;
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>;
    let mut iter_exps: Arc<metamodelica::List<Mutable::Mutable<Arc<Expression::NFExpression>>>>;
    let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(callExp) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { exp: __pa0, iters: __pa1, .. } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp = __pa0.clone();
    iters = __pa1.clone();
    (exp, ranges, iter_exps) = Expression::createIterationRanges(exp, iters)?;
    result = evalArrayConstructor2(exp, ranges, iter_exps)?;
    Ok(result)
}

fn evalArrayConstructor2(mut exp: Arc<Expression::NFExpression>, mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut iterators: Arc<metamodelica::List<Mutable::Mutable<Arc<Expression::NFExpression>>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut range: Arc<Expression::NFExpression>;
    let mut ranges_rest: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
    let mut iter: Mutable::Mutable<Arc<Expression::NFExpression>>;
    let mut iters_rest: Arc<metamodelica::List<Mutable::Mutable<Arc<Expression::NFExpression>>>>;
    let mut range_iter: Arc<ExpressionIterator::NFExpressionIterator>;
    let mut value: Arc<Expression::NFExpression>;
    let mut ty: Arc<Type::NFType>;
    if ranges.clone().is_empty() {
        result = evalExp(exp, noTarget().clone())?;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ranges) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        range = __pa0.clone();
        ranges_rest = __pa1.clone();
        range = evalExp(range, noTarget().clone())?;
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(iterators) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        iter = __pa2.clone();
        iters_rest = __pa3.clone();
        range_iter = ExpressionIterator::fromExp(range, false, false)?;
        while ExpressionIterator::hasNext(range_iter.clone())? {
            (range_iter, value) = ExpressionIterator::next(range_iter.clone())?;
            Mutable::update(iter.clone(), value.clone());
            expl = metamodelica::cons(evalArrayConstructor2(exp.clone(), ranges_rest.clone(), iters_rest.clone())?, expl.clone());
        }
        arr = metamodelica::arrayFromVec(metamodelica::Dangerous::listReverseInPlace(expl.clone()).into_iter().cloned().collect());
        ty = if (arr.clone().borrow().is_empty()) {Type::liftArrayLeftList(Expression::typeOf(exp), List::mapFlat(ranges_rest, (std::sync::Arc::new(fnptr!(Expression::dimensions, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<metamodelica::List<Arc<Dimension::NFDimension>>>> + 'static>))?)} else {Expression::typeOf(listHead(expl)?)};
        ty = Type::liftArrayLeft(ty, Dimension::fromInteger(metamodelica::arrayLength(arr.clone()), Prefixes::Variability::CONSTANT.clone()));
        result = Expression::makeArray(ty, arr.clone(), true);
    }
    Ok(result)
}

type ReductionFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

fn evalReduction(mut callExp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    fn reductionFn(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> {
        let mut result: Arc<Expression::NFExpression> = r#fn(exp1.clone(), evalExp(exp2.clone(), target.clone())?)?;
        Ok(result)
    }

    let mut result: Arc<Expression::NFExpression>;
    let mut r#fn: Arc<Function::Function>;
    let mut exp: Arc<Expression::NFExpression>;
    let mut default_exp: Arc<Expression::NFExpression>;
    let mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>;
    let mut ty: Arc<Type::NFType>;
    let mut red_fn: ReductionFn;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_REDUCTION { r#fn: __pa0, exp: __pa1, iters: __pa2, .. } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa0.clone();
    exp = __pa1.clone();
    iters = __pa2.clone();
    ty = Expression::typeOf(exp.clone());
    (red_fn, default_exp) = (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::name(r#fn.clone()), (literal!(".")).clone(), true, false)?) {
        Deref @ "sum" => ((std::sync::Arc::new(evalBinaryAdd) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), Expression::makeZero(ty.clone())?),
        Deref @ "product" => ((std::sync::Arc::new(evalBinaryMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), Expression::makeOne(ty.clone())?),
        Deref @ "min" => ((std::sync::Arc::new(evalBuiltinMin2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), Expression::makeMaxValue(ty.clone())?),
        Deref @ "max" => ((std::sync::Arc::new(evalBuiltinMax2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), Expression::makeMinValue(ty.clone())?),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalReduction")); __mm_s.push_str(&*literal!(" got unknown reduction function ")); __mm_s.push_str(&*AbsynUtil::pathString(Function::name(r#fn), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result = Expression::foldReduction(exp.clone(), iters.clone(), default_exp.clone(), (std::sync::Arc::new({ let __pe_b1 = noTarget().clone(); move |__pe_a0| evalExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), red_fn.clone())?;
    Ok(result)
}

fn evalSize(mut exp: Arc<Expression::NFExpression>, mut optIndex: Option<Arc<Expression::NFExpression>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut index_exp: Arc<Expression::NFExpression>;
    let mut index: i32;
    let mut ty_err: Arc<TypingError::TypingError>;
    let mut dim: Arc<Dimension::NFDimension>;
    let mut ty: Arc<Type::NFType>;
    let mut info: SourceInfo;
    let mut arr: metamodelica::Array<Arc<Expression::NFExpression>>;
    info = EvalTarget::getInfo(target.clone());
    if isSome(optIndex.clone()) {
        index_exp = evalExp(Util::getOption(optIndex)?, target.clone())?;
        index = Expression::toInteger(index_exp)?;
        (dim, _, ty_err) = Typing::typeExpDim(exp.clone(), index, InstContext::CLASS.clone(), info.clone())?;
        Typing::checkSizeTypingError(ty_err, exp, index, info)?;
        outExp = Dimension::sizeExp(dim)?;
        outExp = evalExp(outExp, target)?;
    } else {
        (outExp, ty, _, _) = Typing::typeExp(exp, InstContext::CLASS.clone(), info, false)?;
        arr = Array::mapList(Type::arrayDims(ty), (std::sync::Arc::new(Dimension::sizeExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
        Array::mapNoCopy(arr.clone(), (std::sync::Arc::new({ let __pe_b1 = target; move |__pe_a0| evalExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
        dim = Dimension::fromInteger(metamodelica::arrayLength(arr.clone()), Variability::PARAMETER.clone());
        outExp = Expression::makeArray(Arc::new(Type::NFType::ARRAY { elementType: crate::NFType::interned_INTEGER(), dimensions: list![dim] }), arr.clone(), false);
    }
    Ok(outExp)
}

fn evalSubscriptedExp(mut exp: Arc<Expression::NFExpression>, mut subscripts: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RANGE { .. } => Arc::new(Expression::NFExpression::RANGE { ty: var_field!((*exp).ty, Expression::NFExpression::RANGE).clone(), start: evalExp(var_field!((*exp).start, Expression::NFExpression::RANGE).clone(), target.clone())?, step: Util::applyOption(var_field!((*exp).step, Expression::NFExpression::RANGE).clone(), (std::sync::Arc::new({ let __pe_b1 = target.clone(); move |__pe_a0| evalExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?, stop: evalExp(var_field!((*exp).stop, Expression::NFExpression::RANGE).clone(), target.clone())? }),
        _ => evalExp(exp, target.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    subs = ({
        let mut __acc: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        for mut s in (subscripts).into_iter().cloned() {
            let __x = Subscript::mapShallowExp(s.clone(), (std::sync::Arc::new({ let __pe_b1 = target.clone(); move |__pe_a0| evalExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    result = Expression::applySubscripts(subs, result, false)?;
    Ok(result)
}

fn evalRecordElement(mut exp: Arc<Expression::NFExpression>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e: Arc<Expression::NFExpression>;
    let mut index: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RECORD_ELEMENT { recordExp: __pa0, index: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    e = __pa0.clone();
    index = __pa1.clone();
    e = evalExp(e, target)?;
    if '__try2: {
        result = unwrap_break_err!(Expression::mapSplitExpressions(e.clone(), (std::sync::Arc::new({ let __pe_b0 = index; move |__pe_a1| Expression::nthRecordElement(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>)), '__try2);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFCeval.evalRecordElement")); __mm_s.push_str(&*literal!(" could not evaluate ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFCeval.mo"))?;
    }
    Ok(result)
}

fn evalRecordElement2(mut exp: Arc<Expression::NFExpression>, mut index: i32) -> Result<Arc<Expression::NFExpression>> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RECORD { .. } => (var_field!((*exp).elements, Expression::NFExpression::RECORD).clone()).get(index)?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(result)
}

fn printUnboundError(mut component: Arc<Component::NFComponent>, mut target: Arc<EvalTarget::EvalTarget>, mut exp: Arc<Expression::NFExpression>) -> Result<()> {
    let mut extra: Arc<EvalTargetData> = Arc::new(<EvalTargetData as ::std::default::Default>::default());
    if !(EvalTarget::hasInfo(target.clone())) {
        return Ok(());
    }
    let () = (::match_deref::match_deref! { match &(target.extra.clone()) {
        Some(__esc_extra @ Deref @ EvalTargetData { .. }) => {
            extra = (*__esc_extra).clone();
            Error::addSourceMessage(Error::STRUCTURAL_PARAMETER_OR_CONSTANT_WITH_NO_BINDING.clone(), list![(Expression::toString(extra.exp.clone())?).clone(), (InstNode::name(extra.component.clone())?).clone()], target.info.clone())?;
            bail!("fail")
        },
        _ if (InstContext::inCondition(target.context.clone())) => {
            Error::addSourceMessage(Error::CONDITIONAL_EXP_WITHOUT_VALUE.clone(), list![(Expression::toString(exp)?).clone()], target.info.clone())?;
            bail!("fail")
        },
        _ => {
            if listMember(Component::variability(component.clone())?, list![Variability::STRUCTURAL_PARAMETER.clone(), Variability::PARAMETER.clone()]) && Util::getOptionOrDefault(Component::getEvaluateAnnotation(component.clone())?, false) {
                if Component::isFixed(component)? {
                    Error::addMultiSourceMessage(Error::UNBOUND_PARAMETER_EVALUATE_TRUE.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!("(fixed = true)")); ArcStr::from(__mm_s) }).clone()], list![InstNode::info(ComponentRef::node(Expression::toCref(exp)?)?), EvalTarget::getInfo(target.clone())])?;
                }
            } else {
                Error::addMultiSourceMessage(Error::UNBOUND_CONSTANT.clone(), list![(Expression::toString(exp.clone())?).clone()], list![InstNode::info(ComponentRef::node(Expression::toCref(exp)?)?), EvalTarget::getInfo(target.clone())])?;
                bail!("fail");
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printWrongArgsError(mut evalFunc: ArcStr, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut info: SourceInfo) -> Result<()> {
    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*evalFunc); __mm_s.push_str(&*literal!(" got invalid arguments ")); __mm_s.push_str(&*List::toString(args, (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone(), info)?;
    Ok(())
}

