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

use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFSubscript as Subscript;
use openmodelica_util::Error;
use openmodelica_util::Util;

pub(crate) fn isStructuralComponent(mut component: Arc<Component::NFComponent>, mut compAttrs: Arc<Attributes::NFAttributes>, mut compBinding: Arc<Binding::NFBinding>, mut compNode: Arc<InstNode::InstNode>, mut compEval: bool, mut parentEval: bool, mut context: i32) -> Result<bool> {
    let mut isStructural: bool;
    let mut binding: Arc<Binding::NFBinding>;
    if compAttrs.variability.clone() != Variability::PARAMETER.clone() {
        isStructural = false;
    } else if compEval || parentEval {
        binding = if (Binding::isBound(compBinding.clone())) {compBinding} else {Component::getTypeAttributeBinding(component.clone(), (literal!("start")).clone())};
        if !(Component::isFixed(component.clone())?) {
            isStructural = false;
        } else if Component::isExternalObject(component)? {
            isStructural = false;
        } else if !(Binding::isBound(binding.clone()) || InstNode::hasBinding(compNode.clone())?) {
            if !(parentEval) && !(InstContext::inRelaxed(context)) {
                Error::addSourceMessage(Error::UNBOUND_PARAMETER_EVALUATE_TRUE.clone(), list![(InstNode::name(compNode.clone())?).clone()], InstNode::info(compNode))?;
            }
            isStructural = false;
        } else if isBindingNotFixed(binding, false, 4)? {
            isStructural = false;
        } else {
            isStructural = true;
        }
    } else {
        isStructural = false;
    }
    Ok(isStructural)
}

pub(crate) fn isBindingNotFixed(mut binding: Arc<Binding::NFBinding>, mut requireFinal: bool, mut maxDepth: i32) -> Result<bool> {
    let mut isNotFixed: bool;
    if maxDepth == 0 {
        isNotFixed = true;
        return Ok(isNotFixed.clone());
    }
    if Binding::hasExp(binding.clone()) {
        isNotFixed = isExpressionNotFixed(Binding::getExp(binding)?, requireFinal, maxDepth)?;
    } else {
        isNotFixed = true;
    }
    Ok(isNotFixed)
}

pub(crate) fn isComponentBindingNotFixed(mut component: Arc<Component::NFComponent>, mut node: Arc<InstNode::InstNode>, mut requireFinal: bool, mut maxDepth: i32, mut isRecord: bool) -> Result<bool> {
    '__tco: loop {
        let mut binding: Arc<Binding::NFBinding>;
        let mut parent: Arc<InstNode::InstNode>;
        binding = Component::getBinding(component.clone());
        if Binding::isUnbound(binding.clone()) {
            if isRecord || InstNode::isRecord(node.clone()) {
                return Ok(false)
            } else {
                parent = InstNode::parent(node);
                if InstNode::isComponent(parent.clone())? && InstNode::isRecord(parent.clone()) {
                    { (component, node, requireFinal, maxDepth, isRecord) = (InstNode::component(parent.clone())?, parent, requireFinal, maxDepth, true); continue '__tco; }
                } else {
                    binding = Component::getTypeAttributeBinding(component, (literal!("start")).clone());
                    return Ok(isBindingNotFixed(binding, requireFinal, maxDepth)?)
                }
            }
        } else {
            return Ok(isBindingNotFixed(binding, requireFinal, maxDepth)?)
        }
    }
}

pub(crate) fn isExpressionNotFixed(mut exp: Arc<Expression::NFExpression>, mut requireFinal: bool, mut maxDepth: i32) -> Result<bool> {
    let mut isNotFixed: bool = false;
    isNotFixed = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (ComponentRef::isCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()) && !(ComponentRef::isIterator(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()))) => {
            let mut node: Arc<InstNode::InstNode>;
            let mut c: Arc<Component::NFComponent>;
            let mut var: Variability;
            node = ComponentRef::node(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())?;
            if InstNode::isComponent(node.clone())? {
                c = InstNode::component(node.clone())?;
                var = Component::variability(c.clone())?;
                if var <= Variability::STRUCTURAL_PARAMETER.clone() {
                    isNotFixed = false;
                } else if var == Variability::PARAMETER.clone() && (!(requireFinal) || Component::isFinal(c.clone())?) && !(Component::isExternalObject(c.clone())?) && Component::isFixed(c.clone())? {
                    isNotFixed = isComponentBindingNotFixed(c, node, requireFinal, maxDepth - 1, false)?;
                } else {
                    isNotFixed = true;
                }
            } else {
                isNotFixed = true;
            }
            isNotFixed || Expression::containsShallow(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = requireFinal; let __pe_b2 = maxDepth; move |__pe_a0| isExpressionNotFixed(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?
        },
        Deref @ Expression::SIZE { .. } => {
            if isSome(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone()) {
                isNotFixed = isExpressionNotFixed(Util::getOption(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone())?, requireFinal, maxDepth)?;
            } else {
                isNotFixed = false;
            }
            isNotFixed
        },
        Deref @ Expression::CALL { .. } => {
            if Call::isImpure(var_field!((*exp).call, Expression::NFExpression::CALL).clone())? || Call::isExternal(var_field!((*exp).call, Expression::NFExpression::CALL).clone())? {
                isNotFixed = true;
            } else {
                isNotFixed = Expression::containsShallow(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = requireFinal; let __pe_b2 = maxDepth; move |__pe_a0| isExpressionNotFixed(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?;
            }
            isNotFixed
        },
        _ => {
            Expression::containsShallow(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = requireFinal; let __pe_b2 = maxDepth; move |__pe_a0| isExpressionNotFixed(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isNotFixed)
}

pub(crate) fn markDimension(mut dimension: Arc<Dimension::NFDimension>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(dimension.clone()) {
        Deref @ Dimension::UNTYPED { .. } => {
            markExp(var_field!((*dimension).dimension, Dimension::NFDimension::UNTYPED).clone())?;
            ()
        },
        Deref @ Dimension::EXP { .. } => {
            markExp(var_field!((*dimension).exp, Dimension::NFDimension::EXP).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn markExp(mut exp: Arc<Expression::NFExpression>) -> Result<()> {
    use crate::NFComponentRef::Origin;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { node, origin: ComponentRef::Origin::CREF { .. }, .. }, .. } => {
            let mut comp: Arc<Component::NFComponent>;
            if InstNode::isComponent(node.clone())? {
                comp = InstNode::component(node.clone())?;
                if Component::variability(comp.clone())? == Variability::PARAMETER.clone() {
                    markComponent(comp, node.clone())?;
                }
            }
            Expression::applyShallow(exp, (std::sync::Arc::new(markExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>))?;
            ()
        },
        Deref @ Expression::SIZE { .. } => {
            let mut e: Arc<Expression::NFExpression>;
            markSubscriptsInExp(var_field!((*exp).exp, Expression::NFExpression::SIZE).clone())?;
            if isSome(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                e = __pa0.clone();
                markExp(e)?;
            }
            ()
        },
        _ => {
            Expression::applyShallow(exp, (std::sync::Arc::new(markExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn markSubscriptsInExp(mut exp: Arc<Expression::NFExpression>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            ComponentRef::applySubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), (std::sync::Arc::new(markSubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<()> + 'static>), false)?;
            ()
        },
        _ => {
            Expression::applyShallow(exp, (std::sync::Arc::new(markSubscriptsInExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn markComponent(mut component: Arc<Component::NFComponent>, mut node: Arc<InstNode::InstNode>) -> Result<()> {
    let mut comp: Arc<Component::NFComponent>;
    let mut binding: Option<Arc<Expression::NFExpression>>;
    comp = Component::setVariability(Variability::STRUCTURAL_PARAMETER.clone(), component);
    comp = Component::setFinal(comp, true);
    InstNode::updateComponent(comp.clone(), node)?;
    binding = Binding::getExpOpt(Component::getBinding(comp));
    if isSome(binding.clone()) {
        markExp(Util::getOption(binding)?)?;
    }
    Ok(())
}

pub(crate) fn markExpSize(mut exp: Arc<Expression::NFExpression>) -> Result<()> {
    Expression::apply(exp, (std::sync::Arc::new(markExpSize_traverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>))?;
    Ok(())
}

pub(crate) fn markExpSize_traverser(mut exp: Arc<Expression::NFExpression>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(exp) {
        Deref @ Expression::CALL { call: Deref @ Call::UNTYPED_ARRAY_CONSTRUCTOR { iters, .. } } => {
            for mut iter in &*iters.clone() {
                let mut iter = iter.clone();
                markExp(Util::tuple22(iter.clone()))?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn markSubscripts(mut exp: Arc<Expression::NFExpression>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            ComponentRef::applySubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), (std::sync::Arc::new(markSubscript) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Subscript::NFSubscript>) -> Result<()> + 'static>), false)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn markSubscript(mut sub: Arc<Subscript::NFSubscript>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Subscript::UNTYPED { .. } => {
            markExp(var_field!((*sub).exp, Subscript::NFSubscript::UNTYPED).clone())?;
            ()
        },
        Deref @ Subscript::INDEX { .. } => {
            markExp(var_field!((*sub).index, Subscript::NFSubscript::INDEX).clone())?;
            ()
        },
        Deref @ Subscript::SLICE { .. } => {
            markExp(var_field!((*sub).slice, Subscript::NFSubscript::SLICE).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

