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

use crate::BaseModelica;
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFClass as Class;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFInst as Inst;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Variability;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFType as Type;
use openmodelica_ast::Absyn::Exp;
use openmodelica_ast::Absyn::Path;
use openmodelica_ast::Absyn::Subscript;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFDimension {
    RAW_DIM {
        dim: Arc<Subscript>,
        scope: Arc<InstNode::InstNode>,
    },
    UNTYPED {
        dimension: Arc<Expression::NFExpression>,
        isProcessing: bool,
    },
    INTEGER {
        size: i32,
        var: Variability,
    },
    BOOLEAN,
    ENUM {
        enumType: Arc<Type::NFType>,
    },
    EXP {
        exp: Arc<Expression::NFExpression>,
        var: Variability,
    },
    /// for all symbolic purposes this is INTEGER() for codegeneration it is EXP()
    ///    invoked by using annotation(__OpenModelica_resizable=true) on a parameter
    RESIZABLE {
        /// the actual size defined by the user
        size: i32,
        /// the optimal size determined by the backend
        opt_size: Option<i32>,
        /// the full expression (parameter)
        exp: Arc<Expression::NFExpression>,
        var: Variability,
    },
    UNKNOWN,
}
impl metamodelica::gc::MMTrace for NFDimension {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFDimension::RAW_DIM { dim, scope } => {
                metamodelica::gc::MMTrace::mm_accept(dim, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                Ok(())
            }
            NFDimension::UNTYPED { dimension, isProcessing } => {
                metamodelica::gc::MMTrace::mm_accept(dimension, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isProcessing, __mmv)?;
                Ok(())
            }
            NFDimension::INTEGER { size, var } => {
                metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                Ok(())
            }
            NFDimension::BOOLEAN => Ok(()),
            NFDimension::ENUM { enumType } => {
                metamodelica::gc::MMTrace::mm_accept(enumType, __mmv)?;
                Ok(())
            }
            NFDimension::EXP { exp, var } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                Ok(())
            }
            NFDimension::RESIZABLE { size, opt_size, exp, var } => {
                metamodelica::gc::MMTrace::mm_accept(size, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(opt_size, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                Ok(())
            }
            NFDimension::UNKNOWN => Ok(()),
        }
    }
}
impl NFDimension {
    pub fn interned_BOOLEAN() -> Arc<NFDimension> {
        thread_local! {
            static INTERNED: Arc<NFDimension> = Arc::new(NFDimension::BOOLEAN);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_UNKNOWN() -> Arc<NFDimension> {
        thread_local! {
            static INTERNED: Arc<NFDimension> = Arc::new(NFDimension::UNKNOWN);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_BOOLEAN() -> Arc<NFDimension> { NFDimension::interned_BOOLEAN() }
pub fn interned_UNKNOWN() -> Arc<NFDimension> { NFDimension::interned_UNKNOWN() }
impl Default for NFDimension {
    fn default() -> Self { Self::BOOLEAN }
}
pub use self::NFDimension::{RAW_DIM,UNTYPED,INTEGER,BOOLEAN,ENUM,EXP,RESIZABLE,UNKNOWN};
pub fn fromExp(mut exp: Arc<Expression::NFExpression>, mut var: Variability) -> Result<Arc<NFDimension>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => {
            return Ok(Arc::new(NFDimension::INTEGER { size: var_field!((*exp).value, Expression::NFExpression::INTEGER).clone(), var: var }))
        },
        Deref @ Expression::TYPENAME { ty: Deref @ Type::ARRAY { elementType: ty, .. } } => {
            ::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::BOOLEAN => return Ok(crate::NFDimension::interned_BOOLEAN()),
        Deref @ Type::ENUMERATION { .. } => return Ok(Arc::new(NFDimension::ENUM { enumType: ty.clone() })),
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFDimension.fromExp")); __mm_s.push_str(&*literal!(" got invalid typename")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFDimension.mo"))?;
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
        },
        Deref @ Expression::ARRAY { .. } if (Expression::arrayAllEqual(exp.clone())) => {
            { (exp, var) = (Expression::arrayFirstScalar(exp.clone())?, var); continue '__tco; }
        },
        Deref @ Expression::SUBSCRIPTED_EXP { split: true, .. } if (Expression::isArray(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone()) && Expression::arrayAllEqual(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone())) => {
            { (exp, var) = (Expression::arrayFirstScalar(var_field!((*exp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone())?, var); continue '__tco; }
        },
        _ => {
            let mut exp_simple: Arc<Expression::NFExpression>;
            let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut value: i32 = 0;
            let mut value_original: i32 = 0;
            exp_simple = SimplifyExp::simplify(exp.clone(), false)?;
            ::match_deref::match_deref! { match &(exp_simple.clone()) {
        Deref @ Expression::INTEGER { value: __esc_value } => {
            value = (*__esc_value).clone();
            return Ok(Arc::new(NFDimension::INTEGER { size: value.clone(), var: var }))
        },
        _ => {
            e1 = Expression::map(exp_simple.clone(), (std::sync::Arc::new(Expression::replaceResizableParameter) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            e1 = SimplifyExp::simplify(e1.clone(), false)?;
            ::match_deref::match_deref! { match &(e1.clone()) {
        Deref @ Expression::INTEGER { value: __esc_value } => {
            value = (*__esc_value).clone();
            e2 = Expression::map(exp_simple.clone(), (std::sync::Arc::new(Expression::replaceResizableParameterWithOriginal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            e2 = SimplifyExp::simplify(e2.clone(), false)?;
            ::match_deref::match_deref! { match &(e2.clone()) {
        Deref @ Expression::INTEGER { value: value_original } if (value.clone() != value_original.clone()) => return Ok(Arc::new(NFDimension::RESIZABLE { size: value_original.clone(), opt_size: Some(value.clone()), exp: exp.clone(), var: var })),
        _ => return Ok(Arc::new(NFDimension::RESIZABLE { size: value.clone(), opt_size: None, exp: exp.clone(), var: var })),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
        },
        _ => return Ok(Arc::new(NFDimension::EXP { exp: exp.clone(), var: var })),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn fromRange(mut range: Arc<Expression::NFExpression>) -> Result<Arc<NFDimension>> {
    let mut dim: Arc<NFDimension>;
    let mut start: i32 = 0;
    let mut step: i32 = 0;
    let mut stop: i32 = 0;
    (start, step, stop) = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ Expression::RANGE { start: Deref @ Expression::INTEGER { value: __esc_start }, step: None, stop: Deref @ Expression::INTEGER { value: __esc_stop }, .. } => {
            start = (*__esc_start).clone();
            stop = (*__esc_stop).clone();
            (start.clone(), 1, stop.clone())
        },
        Deref @ Expression::RANGE { start: Deref @ Expression::INTEGER { value: __esc_start }, step: Some(Deref @ Expression::INTEGER { value: __esc_step }), stop: Deref @ Expression::INTEGER { value: __esc_stop }, .. } => {
            start = (*__esc_start).clone();
            step = (*__esc_step).clone();
            stop = (*__esc_stop).clone();
            (start.clone(), step.clone(), stop.clone())
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFDimension.fromRange")); __mm_s.push_str(&*literal!(" got non-range expression: ")); __mm_s.push_str(&*Expression::toString(range)?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("NFFrontEnd/NFDimension.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dim = Arc::new(NFDimension::INTEGER { size: intDiv(stop - start, step) + 1, var: Prefixes::Variability::CONSTANT.clone() });
    Ok(dim)
}

pub fn fromInteger(mut n: i32, mut var: Variability) -> Arc<NFDimension> {
    let mut dim: Arc<NFDimension> = Arc::new(NFDimension::INTEGER { size: n, var: var });
    dim
}

pub(crate) fn fromExpArray(mut expl: metamodelica::Array<Arc<Expression::NFExpression>>) -> Arc<NFDimension> {
    let mut dim: Arc<NFDimension> = Arc::new(NFDimension::INTEGER { size: metamodelica::arrayLength(expl.clone()), var: Variability::CONSTANT.clone() });
    dim
}

pub(crate) fn fromExpList(mut expl: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Arc<NFDimension> {
    let mut dim: Arc<NFDimension> = Arc::new(NFDimension::INTEGER { size: (expl.clone().len() as i32), var: Variability::CONSTANT.clone() });
    dim
}

pub(crate) fn toRange(mut dim: Arc<NFDimension>) -> Result<Arc<Expression::NFExpression>> {
    let mut range: Arc<Expression::NFExpression>;
    range = Arc::new(Expression::NFExpression::RANGE { ty: Type::liftArrayLeft(typeOf(dim.clone()), dim.clone()), start: lowerBoundExp(dim.clone())?, step: None, stop: upperBoundExp(dim)? });
    Ok(range)
}

pub(crate) fn toDAE(mut dim: Arc<NFDimension>) -> Result<Arc<DAE::Dimension>> {
    let mut daeDim: Arc<DAE::Dimension>;
    daeDim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => {
            Arc::new(DAE::Dimension::DIM_INTEGER { integer: var_field!((*dim).size, NFDimension::INTEGER).clone() })
        },
        Deref @ BOOLEAN { .. } => {
            openmodelica_frontend_types::DAE::Dimension::interned_DIM_BOOLEAN()
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            Arc::new(DAE::Dimension::DIM_ENUM { enumTypeName: var_field!((**ty).typePath, Type::NFType::ENUMERATION).clone(), literals: var_field!((**ty).literals, Type::NFType::ENUMERATION).clone(), size: (var_field!((**ty).literals, Type::NFType::ENUMERATION).clone().len() as i32) })
        },
        Deref @ EXP { .. } => {
            Arc::new(DAE::Dimension::DIM_EXP { exp: Expression::toDAE(var_field!((*dim).exp, NFDimension::EXP).clone(), false)? })
        },
        Deref @ RESIZABLE { .. } => {
            Arc::new(DAE::Dimension::DIM_EXP { exp: Expression::toDAE(var_field!((*dim).exp, NFDimension::RESIZABLE).clone(), false)? })
        },
        Deref @ UNKNOWN { .. } => {
            openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(daeDim)
}

pub(crate) fn add(mut a: Arc<NFDimension>, mut b: Arc<NFDimension>) -> Arc<NFDimension> {
    fn addExp(mut e1: Arc<Expression::NFExpression>, mut e2: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
        let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::NFExpression::BINARY { exp1: e1.clone(), operator: Arc::new(Operator::NFOperator { ty: crate::NFType::interned_INTEGER(), op: Operator::Op::ADD.clone() }), exp2: e2.clone() });
        res
    }

    fn addOpt(mut s1: Option<i32>, mut s2: Option<i32>) -> Option<i32> {
        let mut res: Option<i32>;
        res = (match (s1, s2) {
        (Some(mut i1), Some(mut i2)) => {
            Some(i1.clone() + i2.clone())
        },
        _ => {
            None
        },
    });
        res
    }

    let mut c: Arc<NFDimension>;
    c = (::match_deref::match_deref! { match &((a.clone(), b.clone())) {
        (Deref @ UNKNOWN { .. }, _) => crate::NFDimension::interned_UNKNOWN(),
        (_, Deref @ UNKNOWN { .. }) => crate::NFDimension::interned_UNKNOWN(),
        (Deref @ INTEGER { .. }, Deref @ INTEGER { .. }) => Arc::new(NFDimension::INTEGER { size: var_field!((*a).size, NFDimension::INTEGER).clone() + var_field!((*b).size, NFDimension::INTEGER).clone(), var: Prefixes::variabilityMax(var_field!((*a).var, NFDimension::INTEGER).clone(), var_field!((*b).var, NFDimension::INTEGER).clone()) }),
        (Deref @ INTEGER { .. }, Deref @ EXP { .. }) => Arc::new(NFDimension::EXP { exp: addExp(var_field!((*b).exp, NFDimension::EXP).clone(), Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*a).size, NFDimension::INTEGER).clone() })), var: var_field!((*b).var, NFDimension::EXP).clone() }),
        (Deref @ EXP { .. }, Deref @ INTEGER { .. }) => Arc::new(NFDimension::EXP { exp: addExp(var_field!((*a).exp, NFDimension::EXP).clone(), Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*b).size, NFDimension::INTEGER).clone() })), var: var_field!((*a).var, NFDimension::EXP).clone() }),
        (Deref @ EXP { .. }, Deref @ EXP { .. }) => Arc::new(NFDimension::EXP { exp: addExp(var_field!((*a).exp, NFDimension::EXP).clone(), var_field!((*b).exp, NFDimension::EXP).clone()), var: Prefixes::variabilityMax(var_field!((*a).var, NFDimension::EXP).clone(), var_field!((*b).var, NFDimension::EXP).clone()) }),
        (Deref @ INTEGER { .. }, Deref @ RESIZABLE { .. }) => Arc::new(NFDimension::RESIZABLE { size: var_field!((*a).size, NFDimension::INTEGER).clone() + var_field!((*b).size, NFDimension::RESIZABLE).clone(), opt_size: addOpt(Some(var_field!((*a).size, NFDimension::INTEGER).clone()), var_field!((*b).opt_size, NFDimension::RESIZABLE).clone()), exp: addExp(var_field!((*b).exp, NFDimension::RESIZABLE).clone(), Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*a).size, NFDimension::INTEGER).clone() })), var: var_field!((*b).var, NFDimension::RESIZABLE).clone() }),
        (Deref @ RESIZABLE { .. }, Deref @ INTEGER { .. }) => Arc::new(NFDimension::RESIZABLE { size: var_field!((*a).size, NFDimension::RESIZABLE).clone() + var_field!((*b).size, NFDimension::INTEGER).clone(), opt_size: addOpt(var_field!((*a).opt_size, NFDimension::RESIZABLE).clone(), Some(var_field!((*b).size, NFDimension::INTEGER).clone())), exp: addExp(var_field!((*a).exp, NFDimension::RESIZABLE).clone(), Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*b).size, NFDimension::INTEGER).clone() })), var: var_field!((*a).var, NFDimension::RESIZABLE).clone() }),
        (Deref @ EXP { .. }, Deref @ RESIZABLE { .. }) => Arc::new(NFDimension::EXP { exp: addExp(var_field!((*a).exp, NFDimension::EXP).clone(), var_field!((*b).exp, NFDimension::RESIZABLE).clone()), var: Prefixes::variabilityMax(var_field!((*a).var, NFDimension::EXP).clone(), var_field!((*b).var, NFDimension::RESIZABLE).clone()) }),
        (Deref @ RESIZABLE { .. }, Deref @ EXP { .. }) => Arc::new(NFDimension::EXP { exp: addExp(var_field!((*a).exp, NFDimension::RESIZABLE).clone(), var_field!((*b).exp, NFDimension::EXP).clone()), var: Prefixes::variabilityMax(var_field!((*a).var, NFDimension::RESIZABLE).clone(), var_field!((*b).var, NFDimension::EXP).clone()) }),
        (Deref @ RESIZABLE { .. }, Deref @ RESIZABLE { .. }) => Arc::new(NFDimension::RESIZABLE { size: var_field!((*a).size, NFDimension::RESIZABLE).clone() + var_field!((*b).size, NFDimension::RESIZABLE).clone(), opt_size: addOpt(var_field!((*a).opt_size, NFDimension::RESIZABLE).clone(), var_field!((*b).opt_size, NFDimension::RESIZABLE).clone()), exp: addExp(var_field!((*a).exp, NFDimension::RESIZABLE).clone(), var_field!((*b).exp, NFDimension::RESIZABLE).clone()), var: Prefixes::variabilityMax(var_field!((*a).var, NFDimension::RESIZABLE).clone(), var_field!((*b).var, NFDimension::RESIZABLE).clone()) }),
        _ => crate::NFDimension::interned_UNKNOWN(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    c
}

pub fn size(mut dim: Arc<NFDimension>, mut resize: bool) -> Result<i32> {
    let mut size: i32;
    size = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => {
            var_field!((*dim).size, NFDimension::INTEGER).clone()
        },
        Deref @ RESIZABLE { .. } => {
            if (resize) {Util::getOptionOrDefault(var_field!((*dim).opt_size, NFDimension::RESIZABLE).clone(), var_field!((*dim).size, NFDimension::RESIZABLE).clone())} else {var_field!((*dim).size, NFDimension::RESIZABLE).clone()}
        },
        Deref @ BOOLEAN { .. } => {
            2
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            (var_field!((**ty).literals, Type::NFType::ENUMERATION).clone().len() as i32)
        },
        _ => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFDimension.size")); __mm_s.push_str(&*literal!(" could not get size of: ")); __mm_s.push_str(&*toString(dim)?); ArcStr::from(__mm_s) }).clone())?;
            }
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(size)
}

pub fn sizes(mut dims: Arc<metamodelica::List<Arc<NFDimension>>>, mut resize: bool) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outSizes: Arc<metamodelica::List<i32>> = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = size(d.clone(), resize)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outSizes)
}

pub fn sizesProduct(mut dims: Arc<metamodelica::List<Arc<NFDimension>>>, mut resize: bool) -> Result<i32> {
    let mut outSize: i32 = ({
        let mut __acc: i32 = 1;
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = size(d.clone(), resize)?;
            __acc *= __x;
        }
        __acc
    });
    Ok(outSize)
}

pub fn isEqual(mut dim1: Arc<NFDimension>, mut dim2: Arc<NFDimension>) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = (::match_deref::match_deref! { match &((dim1.clone(), dim2.clone())) {
        (Deref @ UNKNOWN { .. }, _) => true,
        (_, Deref @ UNKNOWN { .. }) => true,
        (Deref @ EXP { .. }, _) => true,
        (_, Deref @ EXP { .. }) => true,
        (Deref @ RESIZABLE { .. }, Deref @ RESIZABLE { .. }) => Expression::isEqual(var_field!((*dim1).exp, NFDimension::RESIZABLE).clone(), var_field!((*dim2).exp, NFDimension::RESIZABLE).clone())?,
        _ => size(dim1, false)? == size(dim2, false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

pub(crate) fn isEqualKnown(mut dim1: Arc<NFDimension>, mut dim2: Arc<NFDimension>) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = (::match_deref::match_deref! { match &((dim1.clone(), dim2.clone())) {
        (Deref @ UNKNOWN { .. }, _) => false,
        (_, Deref @ UNKNOWN { .. }) => false,
        (Deref @ EXP { .. }, Deref @ EXP { .. }) => Expression::isEqual(var_field!((*dim1).exp, NFDimension::EXP).clone(), var_field!((*dim2).exp, NFDimension::EXP).clone())?,
        (Deref @ RESIZABLE { .. }, Deref @ RESIZABLE { .. }) => Expression::isEqual(var_field!((*dim1).exp, NFDimension::RESIZABLE).clone(), var_field!((*dim2).exp, NFDimension::RESIZABLE).clone())?,
        (Deref @ EXP { .. }, _) => false,
        (_, Deref @ EXP { .. }) => false,
        _ => size(dim1, false)? == size(dim2, false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

pub(crate) fn isEqualKnownSize(mut dim1: Arc<NFDimension>, mut node1: Arc<InstNode::InstNode>, mut index1: i32, mut dim2: Arc<NFDimension>, mut node2: Arc<InstNode::InstNode>, mut index2: i32) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = (::match_deref::match_deref! { match &((dim1.clone(), dim2.clone())) {
        (Deref @ EXP { .. }, _) if (isSizeOf(dim1.clone(), node2.clone(), index2)?) => true,
        (_, Deref @ EXP { .. }) if (isSizeOf(dim2.clone(), node1.clone(), index1)?) => true,
        (Deref @ EXP { .. }, Deref @ EXP { .. }) => Expression::isEqual(var_field!((*dim1).exp, NFDimension::EXP).clone(), var_field!((*dim2).exp, NFDimension::EXP).clone())?,
        (Deref @ RESIZABLE { .. }, Deref @ RESIZABLE { .. }) => Expression::isEqual(var_field!((*dim1).exp, NFDimension::RESIZABLE).clone(), var_field!((*dim2).exp, NFDimension::RESIZABLE).clone())?,
        (Deref @ UNKNOWN { .. }, _) => false,
        (_, Deref @ UNKNOWN { .. }) => false,
        _ => size(dim1.clone(), false)? == size(dim2.clone(), false)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

pub(crate) fn isSizeOf(mut dim: Arc<NFDimension>, mut node: Arc<InstNode::InstNode>, mut index: i32) -> Result<bool> {
    let mut res: bool;
    let mut cref_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut index_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    res = (::match_deref::match_deref! { match &(dim) {
        Deref @ EXP { exp: Deref @ Expression::SIZE { exp: __esc_cref_exp @ Deref @ Expression::CREF { .. }, dimIndex: Some(__esc_index_exp) }, .. } => {
            cref_exp = (*__esc_cref_exp).clone();
            index_exp = (*__esc_index_exp).clone();
            InstNode::refEqual(ComponentRef::node(var_field!((*cref_exp).cref, Expression::NFExpression::CREF).clone())?, node) && Expression::isEqual(index_exp.clone(), Arc::new(Expression::NFExpression::INTEGER { value: index }))?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn isResizable(mut dim: Arc<NFDimension>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(dim) {
        Deref @ RESIZABLE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn allEqualKnown(mut dims1: Arc<metamodelica::List<Arc<NFDimension>>>, mut dims2: Arc<metamodelica::List<Arc<NFDimension>>>) -> Result<bool> {
    let mut allEqual: bool = List::isEqualOnTrue(dims1.clone(), dims2.clone(), (std::sync::Arc::new(isEqualKnown) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFDimension>, Arc<NFDimension>) -> Result<bool> + 'static>))?;
    Ok(allEqual)
}

pub(crate) fn isKnown(mut dim: Arc<NFDimension>, mut allowExp: bool) -> bool {
    let mut known: bool;
    known = (::match_deref::match_deref! { match &(dim) {
        Deref @ INTEGER { .. } => true,
        Deref @ BOOLEAN { .. } => true,
        Deref @ ENUM { .. } => true,
        Deref @ RESIZABLE { .. } => true,
        Deref @ EXP { .. } => allowExp,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    known
}

pub(crate) fn isUnknown(mut dim: Arc<NFDimension>) -> bool {
    let mut isUnknown: bool;
    isUnknown = (::match_deref::match_deref! { match &(dim) {
        Deref @ UNKNOWN { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isUnknown
}

pub(crate) fn isZero(mut dim: Arc<NFDimension>) -> Result<bool> {
    let mut isZero: bool;
    isZero = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => var_field!((*dim).size, NFDimension::INTEGER).clone() == 0,
        Deref @ ENUM { .. } => Type::enumSize(var_field!((*dim).enumType, NFDimension::ENUM).clone())? == 0,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isZero)
}

pub fn isOne(mut dim: Arc<NFDimension>) -> Result<bool> {
    let mut isOne: bool;
    isOne = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => var_field!((*dim).size, NFDimension::INTEGER).clone() == 1,
        Deref @ ENUM { .. } => Type::enumSize(var_field!((*dim).enumType, NFDimension::ENUM).clone())? == 1,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isOne)
}

pub(crate) fn subscriptType(mut dim: Arc<NFDimension>) -> Arc<Type::NFType> {
    let mut ty: Arc<Type::NFType>;
    ty = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => crate::NFType::interned_INTEGER(),
        Deref @ BOOLEAN { .. } => crate::NFType::interned_BOOLEAN(),
        Deref @ ENUM { .. } => var_field!((*dim).enumType, NFDimension::ENUM).clone(),
        Deref @ EXP { .. } => Expression::typeOf(var_field!((*dim).exp, NFDimension::EXP).clone()),
        Deref @ RESIZABLE { .. } => Expression::typeOf(var_field!((*dim).exp, NFDimension::RESIZABLE).clone()),
        _ => crate::NFType::interned_UNKNOWN(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty
}

pub fn toString(mut dim: Arc<NFDimension>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ RAW_DIM { .. } => {
            Dump::printSubscriptStr(var_field!((*dim).dim, NFDimension::RAW_DIM).clone())?
        },
        Deref @ INTEGER { .. } => {
            ArcStr::from(::std::format!("{}", var_field!((*dim).size, NFDimension::INTEGER).clone()))
        },
        Deref @ BOOLEAN { .. } => {
            literal!("Boolean")
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            AbsynUtil::pathString(var_field!((**ty).typePath, Type::NFType::ENUMERATION).clone(), (literal!(".")).clone(), true, false)?
        },
        Deref @ EXP { .. } => {
            Expression::toString(var_field!((*dim).exp, NFDimension::EXP).clone())?
        },
        Deref @ RESIZABLE { .. } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toString(var_field!((*dim).exp, NFDimension::RESIZABLE).clone())?); __mm_s.push_str(&*literal!("(R)")); ArcStr::from(__mm_s) }
        },
        Deref @ UNKNOWN { .. } => {
            literal!(":")
        },
        Deref @ UNTYPED { .. } => {
            Expression::toString(var_field!((*dim).dimension, NFDimension::UNTYPED).clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn hashList(mut dims: Arc<metamodelica::List<Arc<NFDimension>>>) -> Result<i32> {
    let mut hash: i32 = Util::HASH_SEED.clone();
    for mut dim in &*dims {
        let mut dim = dim.clone();
        hash = stringHashDjb2Continue((toString(dim.clone())?).clone(), hash);
    }
    Ok(hash)
}

pub(crate) fn toStringList(mut dims: Arc<metamodelica::List<Arc<NFDimension>>>, mut brackets: bool) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut d in (dims).into_iter().cloned() {
            let __x = toString(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone());
    if brackets {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub(crate) fn toFlatString(mut dim: Arc<NFDimension>, mut format: BaseModelica::OutputFormat) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => ArcStr::from(::std::format!("{}", var_field!((*dim).size, NFDimension::INTEGER).clone())),
        Deref @ BOOLEAN { .. } => literal!("Boolean"),
        Deref @ ENUM { .. } => Type::toFlatString(var_field!((*dim).enumType, NFDimension::ENUM).clone(), format)?,
        Deref @ EXP { .. } => Expression::toFlatString(var_field!((*dim).exp, NFDimension::EXP).clone(), format)?,
        Deref @ RESIZABLE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*Expression::toFlatString(var_field!((*dim).exp, NFDimension::RESIZABLE).clone(), format)?); __mm_s.push_str(&*literal!("(R)")); ArcStr::from(__mm_s) },
        Deref @ UNKNOWN { .. } => literal!(":"),
        Deref @ UNTYPED { .. } => Expression::toFlatString(var_field!((*dim).dimension, NFDimension::UNTYPED).clone(), format)?,
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub(crate) fn toFlatStringList(mut dims: Arc<metamodelica::List<Arc<NFDimension>>>, mut format: BaseModelica::OutputFormat, mut name: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (List::toString(dims, (std::sync::Arc::new({ let __pe_b1 = format; move |__pe_a0| toFlatString(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFDimension>) -> Result<ArcStr> + 'static>), (name).clone(), (literal!("[")).clone(), (literal!(", ")).clone(), (literal!("]")).clone(), false, 0)?).clone();
    Ok(r#str)
}

pub(crate) fn endExp(mut dim: Arc<NFDimension>, mut subscriptedExp: Arc<Expression::NFExpression>, mut index: i32) -> Result<Arc<Expression::NFExpression>> {
    let mut sizeExp: Arc<Expression::NFExpression>;
    sizeExp = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => {
            Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*dim).size, NFDimension::INTEGER).clone() })
        },
        Deref @ BOOLEAN { .. } => {
            Arc::new(Expression::NFExpression::BOOLEAN { value: true })
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            Expression::makeEnumLiteral(ty.clone(), (var_field!((**ty).literals, Type::NFType::ENUMERATION).clone().len() as i32))?
        },
        Deref @ EXP { .. } => {
            var_field!((*dim).exp, NFDimension::EXP).clone()
        },
        Deref @ RESIZABLE { .. } => {
            var_field!((*dim).exp, NFDimension::RESIZABLE).clone()
        },
        Deref @ UNKNOWN { .. } => {
            (::match_deref::match_deref! { match &(subscriptedExp.clone()) {
        Deref @ Expression::CREF { .. } => Arc::new(Expression::NFExpression::SIZE { exp: Expression::fromCref((ComponentRef::stripSubscripts(var_field!((*subscriptedExp).cref, Expression::NFExpression::CREF).clone())).0, false)?, dimIndex: Some(Arc::new(Expression::NFExpression::INTEGER { value: index })) }),
        Deref @ Expression::SUBSCRIPTED_EXP { .. } => Arc::new(Expression::NFExpression::SIZE { exp: var_field!((*subscriptedExp).exp, Expression::NFExpression::SUBSCRIPTED_EXP).clone(), dimIndex: Some(Arc::new(Expression::NFExpression::INTEGER { value: index })) }),
        _ => bail!("match: no arm matched"),
    } })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(sizeExp)
}

pub fn sizeExp(mut dim: Arc<NFDimension>) -> Result<Arc<Expression::NFExpression>> {
    let mut sizeExp: Arc<Expression::NFExpression>;
    sizeExp = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => {
            Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*dim).size, NFDimension::INTEGER).clone() })
        },
        Deref @ BOOLEAN { .. } => {
            Arc::new(Expression::NFExpression::INTEGER { value: 2 })
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            Arc::new(Expression::NFExpression::INTEGER { value: (var_field!((**ty).literals, Type::NFType::ENUMERATION).clone().len() as i32) })
        },
        Deref @ EXP { .. } => {
            var_field!((*dim).exp, NFDimension::EXP).clone()
        },
        Deref @ RESIZABLE { .. } => {
            var_field!((*dim).exp, NFDimension::RESIZABLE).clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(sizeExp)
}

pub(crate) fn lowerBoundExp(mut dim: Arc<NFDimension>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ BOOLEAN { .. } => Arc::new(Expression::NFExpression::BOOLEAN { value: false }),
        Deref @ ENUM { .. } => Expression::makeEnumLiteral(var_field!((*dim).enumType, NFDimension::ENUM).clone(), 1)?,
        _ => Arc::new(Expression::NFExpression::INTEGER { value: 1 }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn expIsLowerBound(mut exp: Arc<Expression::NFExpression>) -> bool {
    let mut isStart: bool;
    isStart = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::INTEGER { .. } => var_field!((*exp).value, Expression::NFExpression::INTEGER).clone() == 1,
        Deref @ Expression::BOOLEAN { .. } => var_field!((*exp).value, Expression::NFExpression::BOOLEAN).clone() == false,
        Deref @ Expression::ENUM_LITERAL { .. } => var_field!((*exp).index, Expression::NFExpression::ENUM_LITERAL).clone() == 1,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isStart
}

pub(crate) fn upperBoundExp(mut dim: Arc<NFDimension>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => {
            Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*dim).size, NFDimension::INTEGER).clone() })
        },
        Deref @ BOOLEAN { .. } => {
            Arc::new(Expression::NFExpression::BOOLEAN { value: true })
        },
        Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } } => {
            Expression::makeEnumLiteral(ty.clone(), (var_field!((**ty).literals, Type::NFType::ENUMERATION).clone().len() as i32))?
        },
        Deref @ EXP { .. } => {
            var_field!((*dim).exp, NFDimension::EXP).clone()
        },
        Deref @ RESIZABLE { .. } => {
            var_field!((*dim).exp, NFDimension::RESIZABLE).clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(exp)
}

pub(crate) fn expIsUpperBound(mut exp: Arc<Expression::NFExpression>, mut dim: Arc<NFDimension>) -> bool {
    let mut isEnd: bool;
    isEnd = (::match_deref::match_deref! { match &((exp.clone(), dim.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ INTEGER { .. }) => {
            var_field!((*exp).value, Expression::NFExpression::INTEGER).clone() == var_field!((*dim).size, NFDimension::INTEGER).clone()
        },
        (Deref @ Expression::BOOLEAN { .. }, _) => {
            var_field!((*exp).value, Expression::NFExpression::BOOLEAN).clone() == true
        },
        (Deref @ Expression::ENUM_LITERAL { .. }, Deref @ ENUM { enumType: ty @ Deref @ Type::ENUMERATION { .. } }) => {
            var_field!((*exp).index, Expression::NFExpression::ENUM_LITERAL).clone() == (var_field!((**ty).literals, Type::NFType::ENUMERATION).clone().len() as i32)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEnd
}

pub(crate) fn variability(mut dim: Arc<NFDimension>) -> Result<Variability> {
    let mut var: Variability;
    var = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => var_field!((*dim).var, NFDimension::INTEGER).clone(),
        Deref @ BOOLEAN { .. } => Variability::CONSTANT.clone(),
        Deref @ ENUM { .. } => Variability::CONSTANT.clone(),
        Deref @ EXP { .. } => var_field!((*dim).var, NFDimension::EXP).clone(),
        Deref @ RESIZABLE { .. } => var_field!((*dim).var, NFDimension::RESIZABLE).clone(),
        Deref @ UNKNOWN { .. } => Variability::CONTINUOUS.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(var)
}

pub(crate) fn mapExp(mut dim: Arc<NFDimension>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<NFDimension>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

    let mut outDim: Arc<NFDimension>;
    outDim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ UNTYPED { dimension: e1, .. } => {
            let mut e2: Arc<Expression::NFExpression>;
            e2 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&*(e1.clone()),&*(e2.clone()))) {dim} else {Arc::new(NFDimension::UNTYPED { dimension: e2.clone(), isProcessing: var_field!((*dim).isProcessing, NFDimension::UNTYPED).clone() })}
        },
        Deref @ EXP { exp: e1, .. } => {
            let mut e2: Arc<Expression::NFExpression>;
            e2 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&*(e1.clone()),&*(e2.clone()))) {dim} else {fromExp(e2.clone(), var_field!((*dim).var, NFDimension::EXP).clone())?}
        },
        Deref @ RESIZABLE { exp: e1, .. } => {
            let mut e2: Arc<Expression::NFExpression>;
            e2 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&*(e1.clone()),&*(e2.clone()))) {dim} else {fromExp(e2.clone(), var_field!((*dim).var, NFDimension::RESIZABLE).clone())?}
        },
        _ => {
            dim
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDim)
}

pub(crate) fn foldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut dim: Arc<NFDimension>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut outArg: ArgT;
    outArg = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ UNTYPED { .. } => Expression::fold(var_field!((*dim).dimension, NFDimension::UNTYPED).clone(), func.clone(), arg)?,
        Deref @ EXP { .. } => Expression::fold(var_field!((*dim).exp, NFDimension::EXP).clone(), func.clone(), arg)?,
        Deref @ RESIZABLE { .. } => Expression::fold(var_field!((*dim).exp, NFDimension::RESIZABLE).clone(), func.clone(), arg)?,
        _ => arg,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArg)
}

pub(crate) fn foldExpList<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut dims: Arc<metamodelica::List<Arc<NFDimension>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    for mut dim in &*dims {
        let mut dim = dim.clone();
        arg = foldExp(dim.clone(), func.clone(), arg.clone())?;
    }
    Ok(arg)
}

pub(crate) fn eval(mut dim: Arc<NFDimension>, mut target: Arc<EvalTarget::EvalTarget>) -> Result<Arc<NFDimension>> {
    let mut outDim: Arc<NFDimension>;
    outDim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ EXP { .. } => fromExp(Ceval::evalExp(var_field!((*dim).exp, NFDimension::EXP).clone(), target)?, var_field!((*dim).var, NFDimension::EXP).clone())?,
        Deref @ RESIZABLE { .. } => {
            assign_variant_field!(dim => NFDimension::RESIZABLE; exp = Ceval::evalExp(var_field!((*dim).exp, NFDimension::RESIZABLE).clone(), target)?);
            dim
        },
        _ => dim,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDim)
}

pub(crate) fn simplify(mut dim: Arc<NFDimension>) -> Result<Arc<NFDimension>> {
    let mut dim: Arc<NFDimension> = dim;
    dim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ EXP { .. } => {
            let mut simple: Arc<Expression::NFExpression>;
            simple = SimplifyExp::simplify(var_field!((*dim).exp, NFDimension::EXP).clone(), false)?;
            fromExp(simple.clone(), Expression::variability(simple.clone())?)?
        },
        Deref @ RESIZABLE { .. } => {
            assign_variant_field!(dim => NFDimension::RESIZABLE; exp = SimplifyExp::simplify(var_field!((*dim).exp, NFDimension::RESIZABLE).clone(), false)?);
            dim
        },
        _ => {
            dim
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(dim)
}

pub(crate) fn typeOf(mut dim: Arc<NFDimension>) -> Arc<Type::NFType> {
    let mut ty: Arc<Type::NFType>;
    ty = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ INTEGER { .. } => crate::NFType::interned_INTEGER(),
        Deref @ BOOLEAN { .. } => crate::NFType::interned_BOOLEAN(),
        Deref @ ENUM { .. } => var_field!((*dim).enumType, NFDimension::ENUM).clone(),
        Deref @ EXP { .. } => Expression::typeOf(var_field!((*dim).exp, NFDimension::EXP).clone()),
        Deref @ RESIZABLE { .. } => Expression::typeOf(var_field!((*dim).exp, NFDimension::RESIZABLE).clone()),
        _ => crate::NFType::interned_UNKNOWN(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ty
}


