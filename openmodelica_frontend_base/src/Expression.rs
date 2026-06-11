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

use crate::ComponentReference;
use crate::DAEUtil;
use crate::ExpressionDump;
use crate::ExpressionSimplify;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics::printExpStr;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

// public imports
pub type ComponentRef = Arc<DAE::ComponentRef>;

pub type Exp = Arc<DAE::Exp>;

pub type Operator = DAE::Operator;

pub type Type = Arc<DAE::Type>;

pub type Subscript = Arc<DAE::Subscript>;

pub type Var = Arc<DAE::Var>;

// protected imports
// stringReal
/* **************************************************/
/* transform to other types */
/* **************************************************/
pub fn intSubscript(mut inInteger: i32) -> Arc<DAE::Subscript> {
    let mut outSubscript: Arc<DAE::Subscript>;
    outSubscript = Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: inInteger }) });
    outSubscript
}

pub(crate) fn intSubscripts(mut inIntegers: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> {
    let mut outSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    outSubscripts = List::map(inIntegers, (std::sync::Arc::new(fnptr!(intSubscript, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<DAE::Subscript>> + 'static>))?;
    Ok(outSubscripts)
}

pub fn dimensionIsZero(mut inDimension: Arc<DAE::Dimension>) -> Result<bool> {
    let mut outIsZero: bool;
    outIsZero = 0 == dimensionSize(inDimension)?;
    Ok(outIsZero)
}

pub fn unelabExp(mut inExp: Arc<DAE::Exp>) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ICONST { integer: i } => {
                    Ok(Arc::new(Absyn::Exp::INTEGER { value: i.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RCONST { real: r } => {
                    let mut s: ArcStr;
                    s = (realString(r.clone())).clone();
                    Ok(Arc::new(Absyn::Exp::REAL { value: (s.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SCONST { string: s } => {
                    Ok(Arc::new(Absyn::Exp::STRING { value: (s.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BCONST { bool: b } => {
                    Ok(Arc::new(Absyn::Exp::BOOL { value: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ENUM_LITERAL { name: path, .. } => {
                    let mut cr_1: Arc<Absyn::ComponentRef>;
                    cr_1 = AbsynUtil::pathToCref(path.clone())?;
                    Ok(Arc::new(Absyn::Exp::CREF { componentRef: cr_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
                    let mut cr_1: Arc<Absyn::ComponentRef>;
                    cr_1 = ComponentReference::unelabCref(cr.clone())?;
                    Ok(Arc::new(Absyn::Exp::CREF { componentRef: cr_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
                    let mut ae1: Arc<Absyn::Exp>;
                    let mut ae2: Arc<Absyn::Exp>;
                    let mut aop: Absyn::Operator;
                    aop = unelabOperator(op.clone())?;
                    ae1 = unelabExp(e1.clone())?;
                    ae2 = unelabExp(e2.clone())?;
                    Ok(Arc::new(Absyn::Exp::BINARY { exp1: ae1.clone(), op: aop.clone(), exp2: ae2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: op, exp: e1 } => {
                    let mut ae1: Arc<Absyn::Exp>;
                    let mut aop: Absyn::Operator;
                    aop = unelabOperator(op.clone())?;
                    ae1 = unelabExp(e1.clone())?;
                    Ok(Arc::new(Absyn::Exp::UNARY { op: aop.clone(), exp: ae1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 } => {
                    let mut ae1: Arc<Absyn::Exp>;
                    let mut ae2: Arc<Absyn::Exp>;
                    let mut aop: Absyn::Operator;
                    aop = unelabOperator(op.clone())?;
                    ae1 = unelabExp(e1.clone())?;
                    ae2 = unelabExp(e2.clone())?;
                    Ok(Arc::new(Absyn::Exp::LBINARY { exp1: ae1.clone(), op: aop.clone(), exp2: ae2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 } => {
                    let mut ae1: Arc<Absyn::Exp>;
                    let mut aop: Absyn::Operator;
                    aop = unelabOperator(op.clone())?;
                    ae1 = unelabExp(e1.clone())?;
                    Ok(Arc::new(Absyn::Exp::LUNARY { op: aop.clone(), exp: ae1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, .. } => {
                    let mut ae1: Arc<Absyn::Exp>;
                    let mut ae2: Arc<Absyn::Exp>;
                    let mut aop: Absyn::Operator;
                    aop = unelabOperator(op.clone())?;
                    ae1 = unelabExp(e1.clone())?;
                    ae2 = unelabExp(e2.clone())?;
                    Ok(Arc::new(Absyn::Exp::RELATION { exp1: ae1.clone(), op: aop.clone(), exp2: ae2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 } => {
                    let mut ae1: Arc<Absyn::Exp>;
                    let mut ae2: Arc<Absyn::Exp>;
                    let mut ae3: Arc<Absyn::Exp>;
                    ae1 = unelabExp(e1.clone())?;
                    ae2 = unelabExp(e2.clone())?;
                    ae3 = unelabExp(e3.clone())?;
                    Ok(Arc::new(Absyn::Exp::IFEXP { ifExp: ae1.clone(), trueBranch: ae2.clone(), elseBranch: ae3.clone(), elseIfBranch: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path, expLst: expl, attr: _ } => {
                    let mut aexpl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut acref: Arc<Absyn::ComponentRef>;
                    aexpl = List::map(expl.clone(), (std::sync::Arc::new(unelabExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>))?;
                    acref = AbsynUtil::pathToCref(path.clone())?;
                    Ok(Arc::new(Absyn::Exp::CALL { function_: acref.clone(), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: aexpl.clone(), argNames: metamodelica::nil() }), typeVars: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RECORD { path, exps: expl, .. } => {
                    let mut aexpl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut acref: Arc<Absyn::ComponentRef>;
                    aexpl = List::map(expl.clone(), (std::sync::Arc::new(unelabExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>))?;
                    acref = AbsynUtil::pathToCref(path.clone())?;
                    Ok(Arc::new(Absyn::Exp::CALL { function_: acref.clone(), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: aexpl.clone(), argNames: metamodelica::nil() }), typeVars: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::PARTEVALFUNCTION { path, expList: expl, ty: _, origType: _ } => {
                    let mut aexpl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut acref: Arc<Absyn::ComponentRef>;
                    aexpl = List::map(expl.clone(), (std::sync::Arc::new(unelabExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>))?;
                    acref = AbsynUtil::pathToCref(path.clone())?;
                    Ok(Arc::new(Absyn::Exp::PARTEVALFUNCTION { function_: acref.clone(), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: aexpl.clone(), argNames: metamodelica::nil() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, ty, .. } => {
                    let mut expl_1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    let mut ae1: Arc<Absyn::Exp>;
                    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let mut ty = (*ty).clone();
                    (ty, dims) = TypesDump::flattenArrayType(ty.clone());
                    ae1 = unleabZeroExpFromType(ty.clone())?;
                    expl_1 = List::map(dims.clone(), (std::sync::Arc::new(unelabDimensionToFillExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<Arc<Absyn::Exp>> + 'static>))?;
                    Ok(Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("fill")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::cons(ae1.clone(), expl_1.clone()), argNames: metamodelica::nil() }), typeVars: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
                    let mut expl_1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    expl_1 = List::map(expl.clone(), (std::sync::Arc::new(unelabExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>))?;
                    Ok(Arc::new(Absyn::Exp::ARRAY { arrayExp: expl_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { matrix: mexpl2, .. } => {
                    let mut amexpl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
                    amexpl = List::mapList(mexpl2.clone(), (std::sync::Arc::new(unelabExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>))?;
                    Ok(Arc::new(Absyn::Exp::MATRIX { matrix: amexpl.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { ty: _, start: e1, step: Some(e2), stop: e3 } => {
                    let mut ae1: Arc<Absyn::Exp>;
                    let mut ae2: Arc<Absyn::Exp>;
                    let mut ae3: Arc<Absyn::Exp>;
                    ae1 = unelabExp(e1.clone())?;
                    ae2 = unelabExp(e2.clone())?;
                    ae3 = unelabExp(e3.clone())?;
                    Ok(Arc::new(Absyn::Exp::RANGE { start: ae1.clone(), step: Some(ae2.clone()), stop: ae3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { ty: _, start: e1, step: None, stop: e3 } => {
                    let mut ae1: Arc<Absyn::Exp>;
                    let mut ae3: Arc<Absyn::Exp>;
                    ae1 = unelabExp(e1.clone())?;
                    ae3 = unelabExp(e3.clone())?;
                    Ok(Arc::new(Absyn::Exp::RANGE { start: ae1.clone(), step: None, stop: ae3.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: expl } => {
                    let mut expl_1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
                    expl_1 = List::map(expl.clone(), (std::sync::Arc::new(unelabExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>))?;
                    Ok(Arc::new(Absyn::Exp::TUPLE { expressions: expl_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { ty: _, exp: e1 } => {
                    let mut ae1: Arc<Absyn::Exp>;
                    ae1 = unelabExp(e1.clone())?;
                    Ok(ae1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp: _, sub: _ } => {
                    metamodelica::print((literal!("Internal Error, can not unelab ASUB\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TSUB { exp: e1, ix: _, ty: _ } => {
                    let mut ae1: Arc<Absyn::Exp>;
                    ae1 = unelabExp(e1.clone())?;
                    Ok(ae1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { exp: e1, sz: Some(e2) } => {
                    let mut ae1: Arc<Absyn::Exp>;
                    let mut ae2: Arc<Absyn::Exp>;
                    ae1 = unelabExp(e1.clone())?;
                    ae2 = unelabExp(e2.clone())?;
                    Ok(Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("size")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![ae1.clone(), ae2.clone()], argNames: metamodelica::nil() }), typeVars: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CODE { code, ty: _ } => {
                    Ok(Arc::new(Absyn::Exp::CODE { code: code.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { iterType, path, .. }, expr: e1, iterators: riters } => {
                    let mut ae1: Arc<Absyn::Exp>;
                    let mut acref: Arc<Absyn::ComponentRef>;
                    let mut aiters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
                    acref = AbsynUtil::pathToCref(path.clone())?;
                    ae1 = unelabExp(e1.clone())?;
                    aiters = List::map(riters.clone(), (std::sync::Arc::new(unelabReductionIterator) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ReductionIterator>) -> Result<Arc<Absyn::ForIterator>> + 'static>))?;
                    Ok(Arc::new(Absyn::Exp::CALL { function_: acref.clone(), functionArgs: Arc::new(Absyn::FunctionArgs::FOR_ITER_FARG { exp: ae1.clone(), iterType: iterType.clone(), iterators: aiters.clone() }), typeVars: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.unelabExp failed on: ")); __mm_s.push_str(&*printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

pub fn unelabDimension(mut inDim: Arc<DAE::Dimension>) -> Result<Arc<Absyn::Subscript>> {
    let mut outDim: Arc<Absyn::Subscript>;
    outDim = (::match_deref::match_deref! { match &(inDim) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: i } => {
            Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: i.clone() }) })
        },
        Deref @ DAE::Dimension::DIM_BOOLEAN { .. } => {
            Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("Boolean")).clone(), subscripts: metamodelica::nil() }) }) })
        },
        Deref @ DAE::Dimension::DIM_ENUM { enumTypeName: p, .. } => {
            let mut c: Arc<Absyn::ComponentRef>;
            c = AbsynUtil::pathToCref(p.clone())?;
            Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::CREF { componentRef: c.clone() }) })
        },
        Deref @ DAE::Dimension::DIM_EXP { exp: e } => {
            let mut ae: Arc<Absyn::Exp>;
            ae = unelabExp(e.clone())?;
            Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: ae.clone() })
        },
        Deref @ DAE::Dimension::DIM_UNKNOWN { .. } => {
            openmodelica_ast::Absyn::Subscript::interned_NOSUB()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDim)
}

fn unleabZeroExpFromType(mut ty: Arc<DAE::Type>) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp>;
    outExp = (::match_deref::match_deref! { match &(ty) {
        Deref @ DAE::Type::T_BOOL { .. } => Arc::new(Absyn::Exp::BOOL { value: false }),
        Deref @ DAE::Type::T_STRING { .. } => Arc::new(Absyn::Exp::STRING { value: (literal!("")).clone() }),
        Deref @ DAE::Type::T_INTEGER { .. } => Arc::new(Absyn::Exp::INTEGER { value: 0 }),
        Deref @ DAE::Type::T_REAL { .. } => Arc::new(Absyn::Exp::REAL { value: (literal!("0.0")).clone() }),
        Deref @ DAE::Type::T_UNKNOWN { .. } => Arc::new(Absyn::Exp::REAL { value: (literal!("0.0")).clone() }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

fn unelabDimensionToFillExp(mut inDim: Arc<DAE::Dimension>) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp>;
    outExp = (::match_deref::match_deref! { match &(inDim) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: i } => {
            Arc::new(Absyn::Exp::INTEGER { value: i.clone() })
        },
        Deref @ DAE::Dimension::DIM_EXP { exp: e } => {
            unelabExp(e.clone())?
        },
        _ => {
            Arc::new(Absyn::Exp::INTEGER { value: 1 })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn unelabReductionIterator(mut riter: Arc<DAE::ReductionIterator>) -> Result<Arc<Absyn::ForIterator>> {
    let mut aiter: Arc<Absyn::ForIterator>;
    aiter = (::match_deref::match_deref! { match &(riter) {
        Deref @ DAE::ReductionIterator { id, exp, guardExp: gexp, .. } => {
            let mut aexp: Arc<Absyn::Exp>;
            let mut agexp: Option<Arc<Absyn::Exp>>;
            aexp = unelabExp(exp.clone())?;
            agexp = Util::applyOption(gexp.clone(), (std::sync::Arc::new(unelabExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>))?;
            Arc::new(Absyn::ForIterator { name: (id.clone()).clone(), guardExp: agexp.clone(), range: Some(aexp.clone()) })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(aiter)
}

fn unelabOperator(mut op: DAE::Operator) -> Result<Absyn::Operator> {
    let mut aop: Absyn::Operator;
    aop = (match op {
        DAE::Operator::ADD { ty: _ } => openmodelica_ast::Absyn::Operator::ADD,
        DAE::Operator::SUB { ty: _ } => openmodelica_ast::Absyn::Operator::SUB,
        DAE::Operator::MUL { ty: _ } => openmodelica_ast::Absyn::Operator::MUL,
        DAE::Operator::DIV { ty: _ } => openmodelica_ast::Absyn::Operator::DIV,
        DAE::Operator::POW { ty: _ } => openmodelica_ast::Absyn::Operator::POW,
        DAE::Operator::UMINUS { ty: _ } => openmodelica_ast::Absyn::Operator::UMINUS,
        DAE::Operator::UMINUS_ARR { ty: _ } => openmodelica_ast::Absyn::Operator::UMINUS,
        DAE::Operator::ADD_ARR { ty: _ } => openmodelica_ast::Absyn::Operator::ADD,
        DAE::Operator::SUB_ARR { ty: _ } => openmodelica_ast::Absyn::Operator::SUB,
        DAE::Operator::MUL_ARR { ty: _ } => openmodelica_ast::Absyn::Operator::MUL,
        DAE::Operator::DIV_ARR { ty: _ } => openmodelica_ast::Absyn::Operator::DIV,
        DAE::Operator::MUL_ARRAY_SCALAR { ty: _ } => openmodelica_ast::Absyn::Operator::MUL,
        DAE::Operator::ADD_ARRAY_SCALAR { ty: _ } => openmodelica_ast::Absyn::Operator::ADD,
        DAE::Operator::SUB_SCALAR_ARRAY { ty: _ } => openmodelica_ast::Absyn::Operator::SUB,
        DAE::Operator::MUL_SCALAR_PRODUCT { ty: _ } => openmodelica_ast::Absyn::Operator::MUL,
        DAE::Operator::MUL_MATRIX_PRODUCT { ty: _ } => openmodelica_ast::Absyn::Operator::MUL,
        DAE::Operator::DIV_SCALAR_ARRAY { ty: _ } => openmodelica_ast::Absyn::Operator::DIV,
        DAE::Operator::DIV_ARRAY_SCALAR { ty: _ } => openmodelica_ast::Absyn::Operator::DIV,
        DAE::Operator::POW_SCALAR_ARRAY { ty: _ } => openmodelica_ast::Absyn::Operator::POW,
        DAE::Operator::POW_ARRAY_SCALAR { ty: _ } => openmodelica_ast::Absyn::Operator::POW,
        DAE::Operator::POW_ARR { ty: _ } => openmodelica_ast::Absyn::Operator::POW,
        DAE::Operator::POW_ARR2 { ty: _ } => openmodelica_ast::Absyn::Operator::POW,
        DAE::Operator::AND { ty: _ } => openmodelica_ast::Absyn::Operator::AND,
        DAE::Operator::OR { ty: _ } => openmodelica_ast::Absyn::Operator::OR,
        DAE::Operator::NOT { ty: _ } => openmodelica_ast::Absyn::Operator::NOT,
        DAE::Operator::LESS { ty: _ } => openmodelica_ast::Absyn::Operator::LESS,
        DAE::Operator::LESSEQ { ty: _ } => openmodelica_ast::Absyn::Operator::LESSEQ,
        DAE::Operator::GREATER { ty: _ } => openmodelica_ast::Absyn::Operator::GREATER,
        DAE::Operator::GREATEREQ { ty: _ } => openmodelica_ast::Absyn::Operator::GREATEREQ,
        DAE::Operator::EQUAL { ty: _ } => openmodelica_ast::Absyn::Operator::EQUAL,
        DAE::Operator::NEQUAL { ty: _ } => openmodelica_ast::Absyn::Operator::NEQUAL,
        _ => bail!("match: no arm matched"),
    });
    Ok(aop)
}

pub(crate) fn stringifyCrefs(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = traverseExpDummy(inExp, (std::sync::Arc::new(traversingstringifyCrefFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
    Ok(outExp)
}

pub(crate) fn traversingstringifyCrefFinder(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { .. }, .. } => {
            inExp
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. }, .. } => {
            inExp
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, ty } => {
            let mut crs: ComponentRef;
            crs = ComponentReference::stringifyComponentRef(cr.clone())?;
            makeCrefExp(crs.clone(), ty.clone())?
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn realToIntIfPossible(mut inVal: metamodelica::Real) -> Arc<DAE::Exp> {
    let mut outVal: Arc<DAE::Exp>;
    match '__try0: {
        outVal = Arc::new(DAE::Exp::ICONST { integer: ((inVal).0.floor() as i32) });
        Ok::<_, anyhow::Error>((outVal.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outVal = __try0_o0;
        }
        Err(_) => {
            outVal = Arc::new(DAE::Exp::RCONST { real: inVal });
        }
    }
    outVal
}

pub(crate) fn liftArrayR(mut tp: Arc<DAE::Type>, mut n: Arc<DAE::Dimension>) -> Arc<DAE::Type> {
    let mut outTp: Arc<DAE::Type>;
    outTp = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty: elt_tp, dims } => {
            let mut dims = (*dims).clone();
            dims = metamodelica::cons(n, dims.clone());
            Arc::new(DAE::Type::T_ARRAY { ty: elt_tp.clone(), dims: dims.clone() })
        },
        _ => {
            Arc::new(DAE::Type::T_ARRAY { ty: tp, dims: list![n] })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTp
}

pub fn dimensionSizeConstantExp(mut dim: Arc<DAE::Dimension>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &(dim) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: i } => {
            Arc::new(DAE::Exp::ICONST { integer: i.clone() })
        },
        Deref @ DAE::Dimension::DIM_ENUM { size: i, .. } => {
            Arc::new(DAE::Exp::ICONST { integer: i.clone() })
        },
        Deref @ DAE::Dimension::DIM_BOOLEAN { .. } => {
            Arc::new(DAE::Exp::ICONST { integer: 2 })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(exp)
}

pub fn dimensionSizeExp(mut dim: Arc<DAE::Dimension>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &(dim) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: i } => {
            Arc::new(DAE::Exp::ICONST { integer: i.clone() })
        },
        Deref @ DAE::Dimension::DIM_ENUM { size: i, .. } => {
            Arc::new(DAE::Exp::ICONST { integer: i.clone() })
        },
        Deref @ DAE::Dimension::DIM_BOOLEAN { .. } => {
            Arc::new(DAE::Exp::ICONST { integer: 2 })
        },
        Deref @ DAE::Dimension::DIM_EXP { exp: e } => {
            e.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(exp)
}

pub fn dimensionSizeExpHandleUnkown(mut dim: Arc<DAE::Dimension>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ DAE::Dimension::DIM_UNKNOWN { .. } => Arc::new(DAE::Exp::ICONST { integer: -1 }),
        _ => dimensionSizeExp(dim)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn intDimension(mut value: i32) -> Arc<DAE::Dimension> {
    let mut dim: Arc<DAE::Dimension>;
    dim = Arc::new(DAE::Dimension::DIM_INTEGER { integer: value });
    dim
}

pub fn dimensionSubscript(mut dim: Arc<DAE::Dimension>) -> Result<Arc<DAE::Subscript>> {
    let mut sub: Arc<DAE::Subscript>;
    sub = (::match_deref::match_deref! { match &(dim) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: i } => {
            Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })
        },
        Deref @ DAE::Dimension::DIM_ENUM { size: i, .. } => {
            Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) })
        },
        Deref @ DAE::Dimension::DIM_BOOLEAN { .. } => {
            Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: 2 }) })
        },
        Deref @ DAE::Dimension::DIM_UNKNOWN { .. } => {
            openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(sub)
}

/* **************************************************/
/* Change  */
/* **************************************************/
pub fn negate(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    outExp = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e } => {
            e.clone()
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e } => {
            e.clone()
        },
        Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e } => {
            e.clone()
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } if (isMulOrDiv(op.clone())) => {
            Arc::new(DAE::Exp::BINARY { exp1: negate(e1.clone())?, operator: op.clone(), exp2: e2.clone() })
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } if (isSub(op.clone())) => {
            Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op.clone(), exp2: e1.clone() })
        },
        e if (isZero(e.clone())?) => {
            e.clone()
        },
        Deref @ DAE::Exp::ICONST { integer: i } => {
            let mut i_1: i32;
            i_1 = 0 - i.clone();
            Arc::new(DAE::Exp::ICONST { integer: i_1.clone() })
        },
        Deref @ DAE::Exp::RCONST { real: r } => {
            let mut r_1: metamodelica::Real;
            r_1 = metamodelica::OrderedFloat(0.0_f64) - r.clone();
            Arc::new(DAE::Exp::RCONST { real: r_1.clone() })
        },
        Deref @ DAE::Exp::BCONST { bool: b } => {
            let mut b_1: bool;
            b_1 = !(b.clone());
            Arc::new(DAE::Exp::BCONST { bool: b_1.clone() })
        },
        e => {
            let mut t: Type;
            let mut op: Operator = <DAE::Operator as ::std::default::Default>::default();
            let mut b: bool = false;
            t = r#typeof(e.clone())?;
            outExp = (::match_deref::match_deref! { match &(t.clone()) {
        Deref @ DAE::Type::T_BOOL { .. } => Arc::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: t.clone() }, exp: e.clone() }),
        _ => {
            b = DAEUtil::expTypeArray(t.clone());
            op = if (b.clone()) {DAE::Operator::UMINUS_ARR { ty: t.clone() }} else {DAE::Operator::UMINUS { ty: t.clone() }};
            Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            outExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn negateReal(mut inReal: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outNegatedReal: Arc<DAE::Exp>;
    outNegatedReal = Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: DAE::T_REAL_DEFAULT().clone() }, exp: inReal });
    outNegatedReal
}

pub fn expand(mut e: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outE: Arc<DAE::Exp>;
    outE = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: tp }, exp2: e2 @ Deref @ DAE::Exp::BINARY { exp1: e21, operator: op, exp2: e22 } } if (isAddOrSub(op.clone())) => {
            let mut e21 = (*e21).clone();
            let mut op = (*op).clone();
            let mut e22 = (*e22).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(expand(e2.clone())?) {
                Deref @ DAE::Exp::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e21 = __pa0.clone();
            op = __pa1.clone();
            e22 = __pa2.clone();
            Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e21.clone() }), operator: op.clone(), exp2: Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e22.clone() }) })
        },
        _ => {
            e
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outE)
}

pub fn expDer(mut inExp: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("der")).clone() }), expLst: list![inExp], attr: DAE::callAttrBuiltinReal().clone() });
    outExp
}

pub(crate) fn expAbs(mut inExp: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ICONST { integer: i } => {
            let mut i2: i32;
            i2 = intAbs(i.clone());
            return Arc::new(DAE::Exp::ICONST { integer: i2.clone() })
        },
        Deref @ DAE::Exp::RCONST { real: r } => {
            let mut r2: metamodelica::Real;
            r2 = realAbs(r.clone());
            return Arc::new(DAE::Exp::RCONST { real: r2.clone() })
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e } => {
            let mut e_1: Arc<DAE::Exp>;
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            e1_1 = expAbs(e1.clone());
            e2_1 = expAbs(e2.clone());
            return Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() })
        },
        _ => {
            return inExp
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub(crate) fn stripNoEvent(mut e: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outE: Arc<DAE::Exp>;
    outE = traverseExpDummy(e, (std::sync::Arc::new(fnptr!(stripNoEventExp, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
    Ok(outE)
}

fn stripNoEventExp(mut e: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    outExp = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, expLst: Deref @ metamodelica::List::Cons { head: __esc_outExp, tail: Deref @ metamodelica::List::Nil }, .. } => {
            outExp = (*__esc_outExp).clone();
            outExp.clone()
        },
        _ => e,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub fn addNoEventToRelations(mut e: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outE: Arc<DAE::Exp>;
    outE = traverseExpDummy(e, (std::sync::Arc::new(fnptr!(addNoEventToRelationExp, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
    Ok(outE)
}

fn addNoEventToRelationExp(mut e: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::RELATION { .. } => makeNoEvent(e),
        _ => e,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub fn addNoEventToRelationsAndConds(mut e: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outE: Arc<DAE::Exp>;
    outE = traverseExpDummy(e, (std::sync::Arc::new(fnptr!(addNoEventToRelationandCondExp, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
    Ok(outE)
}

fn addNoEventToRelationandCondExp(mut e: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::RELATION { .. } => {
            makeNoEvent(e)
        },
        Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 } => {
            Arc::new(DAE::Exp::IFEXP { expCond: makeNoEvent(e1.clone()), expThen: e2.clone(), expElse: e3.clone() })
        },
        _ => {
            e
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub fn addNoEventToEventTriggeringFunctions(mut e: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outE: Arc<DAE::Exp>;
    outE = traverseExpDummy(e, (std::sync::Arc::new(fnptr!(addNoEventToEventTriggeringFunctionsExp, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
    Ok(outE)
}

fn addNoEventToEventTriggeringFunctionsExp(mut e: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CALL { .. } if (isEventTriggeringFunctionExp(e.clone())) => makeNoEvent(e.clone()),
        _ => e.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub fn expStripLastSubs(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut cr_1: ComponentRef;
            let mut ty: Type;
            let mut e: Arc<DAE::Exp>;
            ty = ComponentReference::crefLastType(cr.clone())?;
            cr_1 = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
            e = makeCrefExp(cr_1.clone(), ty.clone())?;
            e.clone()
        },
        Deref @ DAE::Exp::UNARY { exp: e, .. } => {
            let mut ty: Type;
            let mut op1: Operator;
            let mut e_1: Arc<DAE::Exp>;
            let mut b: bool;
            e_1 = expStripLastSubs(e.clone())?;
            ty = r#typeof(e_1.clone())?;
            b = DAEUtil::expTypeArray(ty.clone());
            op1 = if (b.clone()) {DAE::Operator::UMINUS_ARR { ty: ty.clone() }} else {DAE::Operator::UMINUS { ty: ty.clone() }};
            Arc::new(DAE::Exp::UNARY { operator: op1.clone(), exp: e_1.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub(crate) fn expStripLastIdent(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut cr_1: ComponentRef;
            let mut ty: Type;
            let mut e: Arc<DAE::Exp>;
            cr_1 = ComponentReference::crefStripLastIdent(cr.clone())?;
            ty = ComponentReference::crefLastType(cr_1.clone())?;
            e = makeCrefExp(cr_1.clone(), ty.clone())?;
            e.clone()
        },
        Deref @ DAE::Exp::UNARY { exp: e, .. } => {
            let mut ty: Type;
            let mut op1: Operator;
            let mut e_1: Arc<DAE::Exp>;
            let mut b: bool;
            e_1 = expStripLastIdent(e.clone())?;
            ty = r#typeof(e_1.clone())?;
            b = DAEUtil::expTypeArray(ty.clone());
            op1 = if (b.clone()) {DAE::Operator::UMINUS_ARR { ty: ty.clone() }} else {DAE::Operator::UMINUS { ty: ty.clone() }};
            Arc::new(DAE::Exp::UNARY { operator: op1.clone(), exp: e_1.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub(crate) fn prependSubscriptExp(mut exp: Arc<DAE::Exp>, mut subscr: Arc<DAE::Subscript>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(exp) {
        Deref @ DAE::Exp::CREF { componentRef: cr, ty: t } => {
            let mut cr1: ComponentRef;
            let mut cr2: ComponentRef;
            let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut e: Arc<DAE::Exp>;
            cr1 = ComponentReferenceBasics::crefStripLastSubs(cr.clone())?;
            subs = ComponentReference::crefLastSubs(cr.clone())?;
            cr2 = ComponentReference::subscriptCref(cr1.clone(), metamodelica::cons(subscr, subs.clone()))?;
            e = makeCrefExp(cr2.clone(), t.clone())?;
            e.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn applyExpSubscripts(mut exp: Arc<DAE::Exp>, mut inSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut r#str: ArcStr;
    if '__try0: {
        exp = applyExpSubscripts2(exp.clone(), inSubs.clone());
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.applyExpSubscripts failed applying subs: [")); __mm_s.push_str(&*ExpressionDump::printSubscriptLstStr(inSubs.clone())?); __mm_s.push_str(&*literal!("] on expression:")); __mm_s.push_str(&*printExpStr(exp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
    }
    Ok(exp)
}

pub fn applyExpSubscriptsFoldCheckSimplify(mut exp: Arc<DAE::Exp>, mut inSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut checkSimplify: bool) -> (Arc<DAE::Exp>, bool) {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut checkSimplify: bool = checkSimplify;
    let mut b: bool;
    let mut s: Arc<DAE::Exp>;
    for mut sub in &*inSubs {
        let mut sub = sub.clone();
        if '__try0: {
            s = unwrap_break_err!(getSubscriptExp(sub.clone()), '__try0);
            (exp, b) = unwrap_break_err!(ExpressionSimplify::simplify(unwrap_break_err!(makeASUB(exp.clone(), list![s.clone()]), '__try0)), '__try0);
            checkSimplify = b || checkSimplify;
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    (exp, checkSimplify)
}

pub(crate) fn applyExpSubscripts2(mut inExp: Arc<DAE::Exp>, mut inSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Arc<DAE::Exp> {
    let mut outArg: Arc<DAE::Exp>;
    outArg = (::match_deref::match_deref! { match &((inExp.clone(), inSubs.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            inExp
        },
        (Deref @ DAE::Exp::CREF { componentRef: cref, ty }, _) => {
            let mut exp: Arc<DAE::Exp>;
            let mut cref = (*cref).clone();
            let mut ty = (*ty).clone();
            match '__try0: {
                cref = unwrap_break_err!(ComponentReference::subscriptCref(cref.clone(), inSubs.clone()), '__try0);
                ty = unwrap_break_err!(ComponentReference::crefTypeFull(cref.clone()), '__try0);
                exp = Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ty.clone() });
                Ok::<_, anyhow::Error>((exp.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    exp = __try0_o0;
                }
                Err(_) => {
                    (exp, _) = applyExpSubscriptsFoldCheckSimplify(inExp.clone(), inSubs.clone(), false);
                }
            }
            exp.clone()
        },
        _ => {
            let mut exp: Arc<DAE::Exp>;
            (exp, _) = applyExpSubscriptsFoldCheckSimplify(inExp, inSubs, false);
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outArg
}

pub fn unliftArray(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type>;
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty: tp, dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } => {
            tp.clone()
        },
        Deref @ DAE::Type::T_ARRAY { ty: tp, dims: Deref @ metamodelica::List::Cons { head: _, tail: ds } } => {
            Arc::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: ds.clone() })
        },
        Deref @ DAE::Type::T_METATYPE { ty: tp } => {
            Types::simplifyType(unliftArray(tp.clone())?)?
        },
        Deref @ DAE::Type::T_METAARRAY { ty: tp } => {
            tp.clone()
        },
        _ => {
            inType
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

pub(crate) fn unliftArrayIgnoreFirst<A: Clone + 'static + metamodelica::gc::MMTrace>(mut a: A, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type>;
    outType = unliftArray(inType)?;
    Ok(outType)
}

pub fn unliftExp(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cr, ty } => {
            let mut expCref: Arc<DAE::Exp>;
            let mut ty = (*ty).clone();
            ty = unliftArray(ty.clone())?;
            expCref = makeCrefExp(cr.clone(), ty.clone())?;
            expCref.clone()
        },
        Deref @ DAE::Exp::ARRAY { ty, scalar: s, array: a } => {
            let mut ty = (*ty).clone();
            ty = unliftArray(ty.clone())?;
            Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: s.clone(), array: a.clone() })
        },
        Deref @ DAE::Exp::MATRIX { ty, integer: i, matrix: mat } => {
            let mut ty = (*ty).clone();
            ty = unliftArray(ty.clone())?;
            Arc::new(DAE::Exp::MATRIX { ty: ty.clone(), integer: i.clone(), matrix: mat.clone() })
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn liftExp(mut inExp: Arc<DAE::Exp>, mut inDimension: Arc<DAE::Dimension>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = Arc::new(DAE::Exp::ARRAY { ty: Types::liftArray(r#typeof(inExp.clone())?, inDimension.clone()), scalar: false, array: List::fill(inExp, dimensionSize(inDimension)?) });
    Ok(outExp)
}

pub fn liftExpList(mut inExp: Arc<DAE::Exp>, mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    for mut dim in &*inDimensions.reverse() {
        let mut dim = dim.clone();
        outExp = liftExp(outExp.clone(), dim.clone())?;
    }
    Ok(outExp)
}

pub fn liftArrayRight(mut inType: Arc<DAE::Type>, mut inDimension: Arc<DAE::Dimension>) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type>;
    outType = (::match_deref::match_deref! { match &((inType.clone(), inDimension.clone())) {
        (Deref @ DAE::Type::T_ARRAY { ty, dims }, dim) => {
            let mut ty_1: Type;
            ty_1 = liftArrayRight(ty.clone(), dim.clone());
            Arc::new(DAE::Type::T_ARRAY { ty: ty_1.clone(), dims: dims.clone() })
        },
        _ => {
            Arc::new(DAE::Type::T_ARRAY { ty: inType, dims: list![inDimension] })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

pub fn liftArrayLeft(mut inType: Arc<DAE::Type>, mut inDimension: Arc<DAE::Dimension>) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type>;
    outType = (::match_deref::match_deref! { match &((inType.clone(), inDimension.clone())) {
        (Deref @ DAE::Type::T_ARRAY { ty, dims }, dim) => {
            Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: metamodelica::cons(dim.clone(), dims.clone()) })
        },
        _ => {
            Arc::new(DAE::Type::T_ARRAY { ty: inType, dims: list![inDimension] })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

pub fn liftArrayLeftList(mut inType: Arc<DAE::Type>, mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type>;
    outType = (::match_deref::match_deref! { match &((inType.clone(), inDimensions.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            inType
        },
        (Deref @ DAE::Type::T_ARRAY { ty, dims }, _) => {
            let mut dims = (*dims).clone();
            dims = listAppend(inDimensions, dims.clone());
            Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: dims.clone() })
        },
        _ => {
            Arc::new(DAE::Type::T_ARRAY { ty: inType, dims: inDimensions })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

pub fn setOpType(mut inOp: DAE::Operator, mut inType: Arc<DAE::Type>) -> Result<DAE::Operator> {
    let mut outOp: DAE::Operator;
    outOp = (match inOp.clone() {
        DAE::Operator::ADD { .. } => DAE::Operator::ADD { ty: inType },
        DAE::Operator::SUB { .. } => DAE::Operator::SUB { ty: inType },
        DAE::Operator::MUL { .. } => DAE::Operator::MUL { ty: inType },
        DAE::Operator::DIV { .. } => DAE::Operator::DIV { ty: inType },
        DAE::Operator::POW { .. } => DAE::Operator::POW { ty: inType },
        DAE::Operator::UMINUS { .. } => DAE::Operator::UMINUS { ty: inType },
        DAE::Operator::UMINUS_ARR { .. } => DAE::Operator::UMINUS_ARR { ty: inType },
        DAE::Operator::ADD_ARR { .. } => DAE::Operator::ADD_ARR { ty: inType },
        DAE::Operator::SUB_ARR { .. } => DAE::Operator::SUB_ARR { ty: inType },
        DAE::Operator::MUL_ARR { .. } => DAE::Operator::MUL_ARR { ty: inType },
        DAE::Operator::DIV_ARR { .. } => DAE::Operator::DIV_ARR { ty: inType },
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => DAE::Operator::MUL_ARRAY_SCALAR { ty: inType },
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => DAE::Operator::ADD_ARRAY_SCALAR { ty: inType },
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => DAE::Operator::SUB_SCALAR_ARRAY { ty: inType },
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => DAE::Operator::MUL_SCALAR_PRODUCT { ty: inType },
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => DAE::Operator::MUL_MATRIX_PRODUCT { ty: inType },
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => DAE::Operator::DIV_ARRAY_SCALAR { ty: inType },
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => DAE::Operator::DIV_SCALAR_ARRAY { ty: inType },
        DAE::Operator::POW_ARRAY_SCALAR { .. } => DAE::Operator::POW_ARRAY_SCALAR { ty: inType },
        DAE::Operator::POW_SCALAR_ARRAY { .. } => DAE::Operator::POW_SCALAR_ARRAY { ty: inType },
        DAE::Operator::POW_ARR { .. } => DAE::Operator::POW_ARR { ty: inType },
        DAE::Operator::POW_ARR2 { .. } => DAE::Operator::POW_ARR2 { ty: inType },
        DAE::Operator::AND { .. } => DAE::Operator::AND { ty: inType },
        DAE::Operator::OR { .. } => DAE::Operator::OR { ty: inType },
        DAE::Operator::NOT { .. } => DAE::Operator::NOT { ty: inType },
        DAE::Operator::LESS { .. } => inOp,
        DAE::Operator::LESSEQ { .. } => inOp,
        DAE::Operator::GREATER { .. } => inOp,
        DAE::Operator::GREATEREQ { .. } => inOp,
        DAE::Operator::EQUAL { .. } => inOp,
        DAE::Operator::NEQUAL { .. } => inOp,
        DAE::Operator::USERDEFINED { .. } => inOp,
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln((literal!("- Expression.setOpType failed on unknown operator")).clone())?;
            bail!("fail")
        },
    });
    Ok(outOp)
}

pub(crate) fn unliftOperator(mut inOperator: DAE::Operator) -> Result<DAE::Operator> {
    let mut outOperator: DAE::Operator;
    let mut ty: Type;
    ty = typeofOp(inOperator.clone())?;
    ty = unliftArray(ty)?;
    outOperator = unliftOperator2(inOperator, ty)?;
    Ok(outOperator)
}

pub(crate) fn unliftOperatorX(mut inOperator: DAE::Operator, mut inX: i32) -> Result<DAE::Operator> {
    let mut outOperator: DAE::Operator;
    let mut ty: Type;
    ty = typeofOp(inOperator.clone())?;
    ty = unliftArrayX(ty, inX)?;
    outOperator = unliftOperator2(inOperator, ty)?;
    Ok(outOperator)
}

fn unliftOperator2(mut inOperator: DAE::Operator, mut inType: Arc<DAE::Type>) -> Result<DAE::Operator> {
    let mut outOperator: DAE::Operator;
    outOperator = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { .. } => setOpType(inOperator, inType)?,
        _ => makeScalarOpFromArrayOp(inOperator, inType),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outOperator)
}

fn makeScalarOpFromArrayOp(mut inOperator: DAE::Operator, mut inType: Arc<DAE::Type>) -> DAE::Operator {
    let mut outOperator: DAE::Operator;
    outOperator = (match inOperator.clone() {
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => DAE::Operator::MUL { ty: inType },
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => DAE::Operator::ADD { ty: inType },
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => DAE::Operator::SUB { ty: inType },
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => DAE::Operator::DIV { ty: inType },
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => DAE::Operator::DIV { ty: inType },
        DAE::Operator::POW_ARRAY_SCALAR { .. } => DAE::Operator::POW { ty: inType },
        DAE::Operator::POW_SCALAR_ARRAY { .. } => DAE::Operator::POW { ty: inType },
        DAE::Operator::UMINUS_ARR { .. } => DAE::Operator::UMINUS { ty: inType },
        DAE::Operator::ADD_ARR { .. } => DAE::Operator::ADD { ty: inType },
        DAE::Operator::SUB_ARR { .. } => DAE::Operator::SUB { ty: inType },
        DAE::Operator::MUL_ARR { .. } => DAE::Operator::MUL { ty: inType },
        DAE::Operator::DIV_ARR { .. } => DAE::Operator::DIV { ty: inType },
        _ => inOperator,
    });
    outOperator
}

pub(crate) fn isScalarArrayOp(mut inOperator: DAE::Operator) -> bool {
    let mut outIsScalarArrayOp: bool;
    outIsScalarArrayOp = (match inOperator {
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => true,
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => true,
        DAE::Operator::POW_SCALAR_ARRAY { .. } => true,
        _ => false,
    });
    outIsScalarArrayOp
}

pub(crate) fn isArrayScalarOp(mut inOperator: DAE::Operator) -> bool {
    let mut outIsArrayScalarOp: bool;
    outIsArrayScalarOp = (match inOperator {
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => true,
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => true,
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => true,
        DAE::Operator::POW_ARRAY_SCALAR { .. } => true,
        _ => false,
    });
    outIsArrayScalarOp
}

pub(crate) fn subscriptsAppend(mut inSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inSubscript: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> {
    let mut outSubscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    outSubscriptLst = (::match_deref::match_deref! { match &(inSubscriptLst) {
        Deref @ metamodelica::List::Nil => {
            list![Arc::new(DAE::Subscript::INDEX { exp: inSubscript })]
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: ss } => {
            metamodelica::cons(Arc::new(DAE::Subscript::INDEX { exp: inSubscript }), ss.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: e }, tail: Deref @ metamodelica::List::Nil } => {
            let mut e_1: Arc<DAE::Exp>;
            (e_1, _) = ExpressionSimplify::simplify1(makeASUB(e.clone(), list![inSubscript])?)?;
            list![Arc::new(DAE::Subscript::INDEX { exp: e_1.clone() })]
        },
        Deref @ metamodelica::List::Cons { head: s @ Deref @ DAE::Subscript::INDEX { .. }, tail: Deref @ metamodelica::List::Nil } => {
            list![s.clone(), Arc::new(DAE::Subscript::INDEX { exp: inSubscript })]
        },
        Deref @ metamodelica::List::Cons { head: s, tail: ss } => {
            let mut ss_1: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            ss_1 = subscriptsAppend(ss.clone(), inSubscript)?;
            metamodelica::cons(s.clone(), ss_1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSubscriptLst)
}

pub(crate) fn subscriptsReplaceSlice(mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut inSubscript: Arc<DAE::Subscript>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> {
    let mut outSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    outSubscripts = (::match_deref::match_deref! { match &(inSubscripts) {
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: rest_subs } => {
            metamodelica::cons(inSubscript, rest_subs.clone())
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { .. }, tail: rest_subs } => {
            metamodelica::cons(inSubscript, rest_subs.clone())
        },
        Deref @ metamodelica::List::Cons { head: sub, tail: rest_subs } => {
            let mut rest_subs = (*rest_subs).clone();
            rest_subs = subscriptsReplaceSlice(rest_subs.clone(), inSubscript)?;
            metamodelica::cons(sub.clone(), rest_subs.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubscripts)
}

pub fn unliftArrayTypeWithSubs(mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut ity: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((subs, ity)) {
        (Deref @ metamodelica::List::Nil, ty) => {
            return Ok(ty.clone())
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, ty) => {
            let mut ty = (*ty).clone();
            ty = unliftArray(ty.clone())?;
            { (subs, ity) = (rest.clone(), ty.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn unliftArrayX(mut inType: Arc<DAE::Type>, mut x: i32) -> Result<Arc<DAE::Type>> {
    '__tco: loop {
        match x {
        0 => {
            return Ok(inType)
        },
        _ => {
            let mut ty: Type;
            ty = unliftArray(inType)?;
            { (inType, x) = (ty.clone(), x - 1); continue '__tco; }
        },
    }
    }
}

pub fn arrayAppend(mut head: Arc<DAE::Exp>, mut rest: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut array: Arc<DAE::Exp>;
    array = (::match_deref::match_deref! { match &(rest) {
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: dim }, tail: dims } }, scalar, array: expl } => {
            let mut dim = (*dim).clone();
            let mut dims = (*dims).clone();
            dim = dim.clone() + 1;
            dims = metamodelica::cons(Arc::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() }), dims.clone());
            Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: dims.clone() }), scalar: scalar.clone(), array: metamodelica::cons(head, expl.clone()) })
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::traceln((literal!("- Expression.arrayAppend failed.")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(array)
}

pub(crate) fn arrayDimensionSetFirst(mut inArrayType: Arc<DAE::Type>, mut dimension: Arc<DAE::Dimension>) -> Result<Arc<DAE::Type>> {
    let mut outArrayType: Arc<DAE::Type>;
    outArrayType = (::match_deref::match_deref! { match &(inArrayType) {
        Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: _, tail: rest_dims } } => {
            Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: metamodelica::cons(dimension, rest_dims.clone()) })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outArrayType)
}

/* **************************************************/
/* Getter  */
/* **************************************************/
pub fn toReal(mut inExp: Arc<DAE::Exp>) -> Result<metamodelica::Real> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::RCONST { .. } => return Ok(var_field!((*inExp).real, DAE::Exp::RCONST).clone()),
        Deref @ DAE::Exp::ICONST { .. } => return Ok(intReal(var_field!((*inExp).integer, DAE::Exp::ICONST).clone())),
        Deref @ DAE::Exp::CAST { .. } => { inExp = var_field!((*inExp).exp, DAE::Exp::CAST).clone(); continue '__tco; },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => return Ok(intReal(var_field!((*inExp).index, DAE::Exp::ENUM_LITERAL).clone())),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn toBool(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut outBool: bool;
    let __pa0 = ::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::BCONST { bool: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outBool = __pa0.clone();
    Ok(outBool)
}

pub fn realExpIntLit(mut exp: Arc<DAE::Exp>) -> Option<i32> {
    let mut oi: Option<i32>;
    oi = (::match_deref::match_deref! { match &(exp) {
        Deref @ DAE::Exp::RCONST { real: r } => {
            let mut i: i32;
            let mut op: Option<i32>;
            i = ((r.clone()).0.floor() as i32);
            op = if (realEq(r.clone(), intReal(i.clone()))) {Some(i.clone())} else {None};
            op.clone()
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oi
}

pub fn expInt(mut exp: Arc<DAE::Exp>) -> Result<i32> {
    let mut i: i32;
    i = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::ICONST { .. } => var_field!((*exp).integer, DAE::Exp::ICONST).clone(),
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => var_field!((*exp).index, DAE::Exp::ENUM_LITERAL).clone(),
        Deref @ DAE::Exp::BCONST { .. } => if (var_field!((*exp).bool, DAE::Exp::BCONST).clone()) {1} else {0},
        _ => bail!("match: no arm matched"),
    } });
    Ok(i)
}

pub fn getClockInterval(mut inClk: Arc<DAE::ClockKind>) -> Arc<DAE::Exp> {
    let mut outIntvl: Arc<DAE::Exp>;
    let mut e: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut e2: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    outIntvl = (::match_deref::match_deref! { match &(inClk) {
        Deref @ DAE::ClockKind::REAL_CLOCK { interval: __esc_e } => {
            e = (*__esc_e).clone();
            e.clone()
        },
        Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: __esc_e, resolution: __esc_e2 } => {
            e = (*__esc_e).clone();
            e2 = (*__esc_e2).clone();
            Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::CAST { ty: DAE::T_REAL_DEFAULT().clone(), exp: e.clone() }), operator: DAE::Operator::DIV { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::CAST { ty: DAE::T_REAL_DEFAULT().clone(), exp: e2.clone() }) })
        },
        Deref @ DAE::ClockKind::EVENT_CLOCK { condition: __esc_e, startInterval: __esc_e2 } => {
            e = (*__esc_e).clone();
            e2 = (*__esc_e2).clone();
            e2.clone()
        },
        _ => Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIntvl
}

pub fn sconstEnumNameString(mut exp: Arc<DAE::Exp>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((::match_deref::match_deref! { match &(exp) {
        Deref @ DAE::Exp::SCONST { string: s } => {
            s.clone()
        },
        Deref @ DAE::Exp::ENUM_LITERAL { name, .. } => {
            AbsynUtil::pathString(name.clone(), (literal!(".")).clone(), true, false)?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn varName(mut v: Arc<DAE::Var>) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    name = ((::match_deref::match_deref! { match &(v) {
        Deref @ DAE::Var { name: __esc_name, .. } => {
            name = (*__esc_name).clone();
            name.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(name)
}

pub(crate) fn varType(mut v: Arc<DAE::Var>) -> Result<Arc<DAE::Type>> {
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    tp = (::match_deref::match_deref! { match &(v) {
        Deref @ DAE::Var { ty: __esc_tp, .. } => {
            tp = (*__esc_tp).clone();
            tp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(tp)
}

pub fn expOrDerCref(mut inExp: Arc<DAE::Exp>) -> Result<(Arc<DAE::ComponentRef>, bool)> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    let mut isDer: bool;
    (outComponentRef, isDer) = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            (cr.clone(), false)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            (cr.clone(), true)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outComponentRef, isDer))
}

pub fn expCref(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            cr.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub fn expCrefNegCref(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            cr.clone()
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } } => {
            cr.clone()
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } } => {
            cr.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub(crate) fn expCrefTuple(mut inTuple: (Arc<DAE::Exp>, bool)) -> Result<Arc<DAE::ComponentRef>> {
    let mut outComponentRef: Arc<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &(inTuple) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _) => {
            cr.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub(crate) fn expCrefInclIfExpFactors(mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outComponentRefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    outComponentRefs = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            list![cr.clone()]
        },
        Deref @ DAE::Exp::IFEXP { expCond: _, expThen: tb, expElse: fb } => {
            let mut f: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            f = List::select(listAppend(factors(tb.clone())?, factors(fb.clone())?), (std::sync::Arc::new(fnptr!(isCref, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?;
            crefs = List::map(f.clone(), (std::sync::Arc::new(expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
            crefs.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRefs)
}

pub(crate) fn getArrayContents(mut e: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut es: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let __pa0 = ::match_deref::match_deref! { match &(e) {
        Deref @ DAE::Exp::ARRAY { array: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    es = __pa0.clone();
    Ok(es)
}

pub fn getArrayOrMatrixContents(mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outContents: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    outContents = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
            expl.clone()
        },
        Deref @ DAE::Exp::MATRIX { ty: Deref @ DAE::Type::T_ARRAY { ty: el_ty, dims: Deref @ metamodelica::List::Cons { head: _, tail: dims } }, matrix: mat, .. } => {
            let mut ty: Arc<DAE::Type>;
            let mut sc: bool;
            ty = Arc::new(DAE::Type::T_ARRAY { ty: el_ty.clone(), dims: dims.clone() });
            sc = Types::basicType(el_ty.clone());
            List::map2(mat.clone(), (std::sync::Arc::new(fnptr!(makeArray, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool) -> Result<Arc<DAE::Exp>> + 'static>), ty.clone(), sc.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outContents)
}

pub fn expandArray(mut exp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut contents: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    contents = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::ARRAY { .. } => ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*exp).array, DAE::Exp::ARRAY).clone().reverse()).into_iter().cloned() {
            let __x = expandArray(e.clone())?;
            __acc = __x.append(&__acc);
        }
        __acc
    }),
        Deref @ DAE::Exp::MATRIX { .. } => getArrayOrMatrixContents(exp)?,
        _ => list![exp],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(contents)
}

fn makeASUBsForDimension(mut eIn: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut eLstOut: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut size: i32;
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    dims = expDimensions(eIn.clone())?;
    if '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(dims.clone()) {
            Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::List::Nil } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        size = __pa1.clone();
        for mut i in ({let __s=size; let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
            eLstOut = metamodelica::cons(unwrap_break_err!(makeASUBSingleSub(eIn.clone(), Arc::new(DAE::Exp::ICONST { integer: i.clone() })), '__try0), eLstOut.clone());
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        eLstOut = metamodelica::nil();
    }
    Ok(eLstOut)
}

pub fn getComplexContents(mut e: Arc<DAE::Exp>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut es: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    es = 'mc: {
        let __mc_input = e.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { .. } => {
                    let mut noArr: bool;
                    let mut exp: Arc<DAE::Exp>;
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    expLst = arrayElements(e.clone())?;
                    noArr = (expLst.clone().len() as i32) == 1;
                    exp = listHead(expLst.clone())?;
                    noArr = noArr.clone() && ExpressionBasics::expEqual(exp.clone(), e.clone())?;
                    expLst = if (noArr.clone()) {metamodelica::nil()} else {expLst.clone()};
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD_ARR { .. }, exp2 } => {
                    let mut ty: Arc<DAE::Type>;
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut expLst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut expLst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    if isArray(exp1.clone()) {
                        expLst1 = getComplexContents(exp1.clone());
                    } else {
                        expLst1 = makeASUBsForDimension(exp1.clone())?;
                    }
                    if isArray(exp2.clone()) {
                        expLst2 = getComplexContents(exp2.clone());
                    } else {
                        expLst2 = makeASUBsForDimension(exp2.clone())?;
                    }
                    ty = r#typeof(listHead(expLst1.clone())?)?;
                    expLst = List::threadMap(expLst1.clone(), expLst2.clone(), (std::sync::Arc::new({ let __pe_b1 = DAE::Operator::ADD { ty: ty.clone() }; move |__pe_a0, __pe_a2| Ok(makeBinaryExp(__pe_a0, __pe_b1.clone(), __pe_a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst, .. } => {
                    let mut expLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
                    let mut expLst = (*expLst).clone();
                    expLstLst = List::map(expLst.clone(), (std::sync::Arc::new(fnptr!(getComplexContentsInCall, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?;
                    expLst = List::flatten(expLstLst.clone())?;
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RECORD { exps: expLst, .. } => {
                    let mut expLstLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
                    let mut expLst = (*expLst).clone();
                    expLstLst = List::map(expLst.clone(), (std::sync::Arc::new(fnptr!(getComplexContentsInCall, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?;
                    expLst = List::flatten(expLstLst.clone())?;
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { .. } => {
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    expLst = arrayElements(e.clone())?;
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { matrix: expLstLst, .. } => {
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    expLst = List::flatten(expLstLst.clone())?;
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: expLst } => {
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { exp, .. } => {
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    expLst = getComplexContents(exp.clone());
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp, .. } => {
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    expLst = getComplexContents(exp.clone());
                    Ok(expLst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    es
}

fn getComplexContentsInCall(mut expIn: Arc<DAE::Exp>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut expsOut: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    expLst = getComplexContents(expIn.clone());
    expsOut = if (expLst.clone().is_empty()) {list![expIn]} else {expLst};
    expsOut
}

pub fn getArrayOrRangeContents(mut e: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut es: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    es = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::ARRAY { array: __esc_es, .. } => {
            es = (*__esc_es).clone();
            es.clone()
        },
        Deref @ DAE::Exp::MATRIX { matrix, ty, .. } => {
            let mut ty = (*ty).clone();
            ty = Types::unliftArray(ty.clone())?;
            es = List::map2(matrix.clone(), (std::sync::Arc::new(fnptr!(makeArray, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool) -> Result<Arc<DAE::Exp>> + 'static>), ty.clone(), !(Types::arrayType(ty.clone())))?;
            es.clone()
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: istop }, tail: _ }, .. }, .. } => {
            es = List::map(ExpressionSimplify::simplifyRange(1, 1, istop.clone())?, (std::sync::Arc::new(fnptr!(makeIntegerExp, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<DAE::Exp>> + 'static>))?;
            es = List::map1r(es, (std::sync::Arc::new(makeASUBSingleSub) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e)?;
            es.clone()
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::BCONST { bool: bstart }, step: None, stop: Deref @ DAE::Exp::BCONST { bool: bstop }, .. } => {
            List::map(ExpressionSimplify::simplifyRangeBool(bstart.clone(), bstop.clone()), (std::sync::Arc::new(fnptr!(makeBoolExp, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool) -> Result<Arc<DAE::Exp>> + 'static>))?
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: istart }, step: None, stop: Deref @ DAE::Exp::ICONST { integer: istop }, .. } => {
            List::map(ExpressionSimplify::simplifyRange(istart.clone(), 1, istop.clone())?, (std::sync::Arc::new(fnptr!(makeIntegerExp, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<DAE::Exp>> + 'static>))?
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: istart }, step: Some(Deref @ DAE::Exp::ICONST { integer: istep }), stop: Deref @ DAE::Exp::ICONST { integer: istop }, .. } => {
            List::map(ExpressionSimplify::simplifyRange(istart.clone(), istep.clone(), istop.clone())?, (std::sync::Arc::new(fnptr!(makeIntegerExp, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<DAE::Exp>> + 'static>))?
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::RCONST { real: rstart }, step: None, stop: Deref @ DAE::Exp::RCONST { real: rstop }, .. } => {
            List::map(ExpressionSimplify::simplifyRangeReal(rstart.clone(), metamodelica::OrderedFloat(1.0_f64), rstop.clone()), (std::sync::Arc::new(fnptr!(makeRealExp, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real) -> Result<Arc<DAE::Exp>> + 'static>))?
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::RCONST { real: rstart }, step: Some(Deref @ DAE::Exp::RCONST { real: rstep }), stop: Deref @ DAE::Exp::RCONST { real: rstop }, .. } => {
            List::map(ExpressionSimplify::simplifyRangeReal(rstart.clone(), rstep.clone(), rstop.clone()), (std::sync::Arc::new(fnptr!(makeRealExp, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real) -> Result<Arc<DAE::Exp>> + 'static>))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(es)
}

pub(crate) fn get2dArrayOrMatrixContent(mut e: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>> {
    let mut outExps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
    outExps = (::match_deref::match_deref! { match &(e) {
        Deref @ DAE::Exp::ARRAY { array: es, .. } => {
            List::map(es.clone(), (std::sync::Arc::new(getArrayContents) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?
        },
        Deref @ DAE::Exp::MATRIX { matrix: ess, .. } => {
            ess.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExps)
}

// stefan
pub(crate) fn unboxExpType(mut inType: Arc<DAE::Type>) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type>;
    outType = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_METABOXED { ty } => {
            ty.clone()
        },
        _ => {
            inType
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

pub fn unboxExp(mut ie: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(ie.clone()) {
        Deref @ DAE::Exp::BOX { exp: e } => {
            { ie = e.clone(); continue '__tco; }
        },
        _ => {
            return ie
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub(crate) fn boxExp(mut e: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::BOX { exp: _ } => e,
        _ => Arc::new(DAE::Exp::BOX { exp: e }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub fn getSubscriptExp(mut inSubscript: Arc<DAE::Subscript>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inSubscript) {
        Deref @ DAE::Subscript::SLICE { exp: e } => {
            e.clone()
        },
        Deref @ DAE::Subscript::INDEX { exp: e } => {
            e.clone()
        },
        Deref @ DAE::Subscript::WHOLE_NONEXP { exp: e } => {
            e.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub(crate) fn subscriptNonExpandedExp(mut inSubscript: Arc<DAE::Subscript>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inSubscript) {
        Deref @ DAE::Subscript::WHOLE_NONEXP { exp: e } => {
            e.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub(crate) fn subscriptIsFirst(mut inSubscript: Arc<DAE::Subscript>) -> Result<bool> {
    let mut outIsFirst: bool;
    outIsFirst = (::match_deref::match_deref! { match &(inSubscript) {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: 1 } } => true,
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::BCONST { bool: false } } => true,
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ENUM_LITERAL { index: 1, .. } } => true,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outIsFirst)
}

pub fn nthArrayExp(mut inExp: Arc<DAE::Exp>, mut inInteger: i32) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { operator: op, exp1: e1, exp2: e2 } => {
                    let mut e_1: Arc<DAE::Exp>;
                    let mut e_2: Arc<DAE::Exp>;
                    let mut ty: Type;
                    ty = typeofOp(op.clone())?;
                    let true = (Types::isArray(ty.clone())) else { bail!("pattern mismatch") };
                    e_1 = nthArrayExp(e1.clone(), inInteger);
                    e_2 = nthArrayExp(e2.clone(), inInteger);
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e_1.clone(), operator: op.clone(), exp2: e_2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
                    let mut e1: Arc<DAE::Exp>;
                    e1 = (expl.clone()).get(inInteger)?;
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outExp
}

pub(crate) fn expLastSubs(mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            return Ok(ComponentReference::crefLastSubs(cr.clone())?)
        },
        Deref @ DAE::Exp::UNARY { exp: e, .. } => {
            let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            { inExp = e.clone(); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn expDimensions(mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::ARRAY { ty: tp, .. } => {
            return Ok(arrayDimension(tp.clone()))
        },
        Deref @ DAE::Exp::MATRIX { ty: tp, .. } => {
            return Ok(arrayDimension(tp.clone()))
        },
        Deref @ DAE::Exp::LUNARY { exp: e, .. } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::LBINARY { exp1: e, .. } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, .. } => {
            return Ok(arrayDimension(tp.clone()))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn arrayDimension(mut tp: Arc<DAE::Type>) -> Arc<metamodelica::List<Arc<DAE::Dimension>>> {
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    dims = (::match_deref::match_deref! { match &(tp) {
        Deref @ DAE::Type::T_ARRAY { dims: __esc_dims, .. } => {
            dims = (*__esc_dims).clone();
            dims.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dims
}

pub(crate) fn arrayTypeDimensions(mut tp: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> {
    let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    dims = (::match_deref::match_deref! { match &(tp) {
        Deref @ DAE::Type::T_ARRAY { dims: __esc_dims, .. } => {
            dims = (*__esc_dims).clone();
            dims.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(dims)
}

pub fn subscriptDimensions(mut inSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Dimension>>>> {
    let mut outDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    outDimensions = List::map(inSubscripts, (std::sync::Arc::new(subscriptDimension) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<Arc<DAE::Dimension>> + 'static>))?;
    Ok(outDimensions)
}

pub(crate) fn subscriptDimension(mut inSubscript: Arc<DAE::Subscript>) -> Result<Arc<DAE::Dimension>> {
    let mut outDimension: Arc<DAE::Dimension>;
    outDimension = (::match_deref::match_deref! { match &(inSubscript.clone()) {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: x } } => {
            Arc::new(DAE::Dimension::DIM_INTEGER { integer: x.clone() })
        },
        Deref @ DAE::Subscript::INDEX { exp: e } => {
            Arc::new(DAE::Dimension::DIM_EXP { exp: e.clone() })
        },
        Deref @ DAE::Subscript::WHOLEDIM { .. } => {
            openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()
        },
        Deref @ DAE::Subscript::WHOLE_NONEXP { exp: Deref @ DAE::Exp::ICONST { integer: x } } if (!(Config::splitArrays()?)) => {
            Arc::new(DAE::Dimension::DIM_INTEGER { integer: x.clone() })
        },
        Deref @ DAE::Subscript::WHOLE_NONEXP { exp: e } if (!(Config::splitArrays()?)) => {
            Arc::new(DAE::Dimension::DIM_EXP { exp: e.clone() })
        },
        _ => {
            let mut sub_str: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            sub_str = (ExpressionDump::subscriptString(inSubscript)?).clone();
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Expression.subscriptDimension failed on ")); __mm_s.push_str(&*sub_str.clone()); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDimension)
}

pub fn arrayEltType(mut inType: Arc<DAE::Type>) -> Arc<DAE::Type> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty: t, .. } => {
            { inType = t.clone(); continue '__tco; }
        },
        _ => {
            return inType
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn sizeOf(mut inType: Arc<DAE::Type>) -> i32 {
    let mut i: i32;
    i = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { .. } => {
                    Ok(sizeOf(var_field!((*inType).ty, DAE::Type::T_ARRAY).clone()) * ({
        let mut __acc: i32 = 1;
        for mut d in (var_field!((*inType).dims, DAE::Type::T_ARRAY).clone()).into_iter().cloned() {
            let __x = dimensionSize(d.clone())?;
            __acc *= __x;
        }
        __acc
    }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. } => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { .. } => {
                    Ok(({
        let mut __acc: i32 = 0;
        for mut v in (var_field!((*inType).varLst, DAE::Type::T_COMPLEX).clone()).into_iter().cloned() {
            let __x = sizeOf(varType(v.clone())?);
            __acc += __x;
        }
        __acc
    }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_TUPLE { .. } => {
                    Ok(({
        let mut __acc: i32 = 0;
        for mut ty in (var_field!((*inType).types, DAE::Type::T_TUPLE).clone()).into_iter().cloned() {
            let __x = sizeOf(ty.clone());
            __acc += __x;
        }
        __acc
    }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_FUNCTION { .. } => {
                    Ok(sizeOf(var_field!((*inType).funcResultType, DAE::Type::T_FUNCTION).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METATYPE { .. } => {
                    Ok(sizeOf(var_field!((*inType).ty, DAE::Type::T_METATYPE).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_UNKNOWN { .. } => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    i
}

pub fn dimensionSize(mut dim: Arc<DAE::Dimension>) -> Result<i32> {
    let mut value: i32;
    value = (::match_deref::match_deref! { match &(dim) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: i } => {
            i.clone()
        },
        Deref @ DAE::Dimension::DIM_ENUM { size: i, .. } => {
            i.clone()
        },
        Deref @ DAE::Dimension::DIM_BOOLEAN { .. } => {
            2
        },
        Deref @ DAE::Dimension::DIM_EXP { exp: Deref @ DAE::Exp::ICONST { integer: i } } => {
            i.clone()
        },
        Deref @ DAE::Dimension::DIM_EXP { exp: Deref @ DAE::Exp::ENUM_LITERAL { index: i, .. } } => {
            i.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(value)
}

pub(crate) fn addDimensions(mut dim1: Arc<DAE::Dimension>, mut dim2: Arc<DAE::Dimension>) -> Arc<DAE::Dimension> {
    let mut dim: Arc<DAE::Dimension>;
    dim = 'mc: {
        let __mc_input = dim2.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut i: i32;
                    i = dimensionSize(dim1.clone())? + dimensionSize(dim2.clone())?;
                    Ok(Arc::new(DAE::Dimension::DIM_INTEGER { integer: i.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    dim
}

pub(crate) fn dimensionSizeAll(mut dim: Arc<DAE::Dimension>) -> Result<i32> {
    let mut value: i32;
    value = 'mc: {
        let __mc_input = dim;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Dimension::DIM_INTEGER { integer: i } => {
                    Ok(i.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Dimension::DIM_ENUM { size: i, .. } => {
                    Ok(i.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Dimension::DIM_BOOLEAN { .. } => {
                    Ok(2)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Dimension::DIM_EXP { exp: e } => {
                    Ok(expInt(e.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Dimension::DIM_EXP { .. } => {
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Dimension::DIM_UNKNOWN { .. } => {
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { bail!("pattern mismatch") };
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(value)
}

pub fn dimensionsSizes(mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<metamodelica::List<i32>>> {
    let mut outValues: Arc<metamodelica::List<i32>>;
    outValues = List::map(inDims, (std::sync::Arc::new(dimensionSizeAll) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<i32> + 'static>))?;
    Ok(outValues)
}

pub fn r#typeof(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type>;
    outType = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ICONST { .. } => {
                    Ok(DAE::T_INTEGER_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RCONST { .. } => {
                    Ok(DAE::T_REAL_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SCONST { .. } => {
                    Ok(DAE::T_STRING_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BCONST { .. } => {
                    Ok(DAE::T_BOOL_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CLKCONST { .. } => {
                    Ok(DAE::T_CLOCK_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ENUM_LITERAL { name: p, index: i } => {
                    Ok(Arc::new(DAE::Type::T_ENUMERATION { index: Some(i.clone()), path: p.clone(), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { operator: op, .. } => {
                    Ok(typeofOp(op.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: op, .. } => {
                    Ok(typeofOp(op.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LBINARY { operator: op, .. } => {
                    Ok(typeofOp(op.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { operator: op, .. } => {
                    Ok(typeofOp(op.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RELATION { operator: op, .. } => {
                    Ok(typeofRelation(typeofOp(op.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expThen: e2, .. } => {
                    Ok(r#typeof(e2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, .. } => {
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RECORD { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::PARTEVALFUNCTION { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp: e, sub: subs } => {
                    let mut tp: Type;
                    let mut explist: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut i: i32;
                    if Config::acceptMetaModelicaGrammar()? {
                        explist = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
                    let __x = getSubscriptExp(sub.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                        i = ({
        let mut __acc: i32 = 0;
        for mut e in (explist.clone()).into_iter().cloned() {
                    if !(isScalar(e.clone())?) { continue; }
                    let __x = 1;
                    __acc += __x;
        }
        __acc
    });
                    } else {
                        i = ({
        let mut __acc: i32 = 0;
        for mut sub in (subs.clone()).into_iter().cloned() {
                    if !(isScalarSubscript(sub.clone())?) { continue; }
                    let __x = 1;
                    __acc += __x;
        }
        __acc
    });
                    }
                    tp = unliftArrayX(r#typeof(e.clone())?, i.clone())?;
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TSUB { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RSUB { .. } => {
                    Ok(var_field!((*inExp).ty, DAE::Exp::RSUB).clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CODE { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { iterators: Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { exp: iterExp, guardExp: None, .. }, tail: Deref @ metamodelica::List::Nil }, expr: operExp, reductionInfo: Deref @ DAE::ReductionInfo { exprType: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: dim, tail: _ }, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, .. } } => {
                    let mut tp: Type;
                    let mut iterTp: Arc<DAE::Type>;
                    let mut operTp: Arc<DAE::Type>;
                    let mut iterdims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
                    let false = (dimensionKnown(dim.clone())) else { bail!("pattern mismatch") };
                    iterTp = r#typeof(iterExp.clone())?;
                    operTp = r#typeof(operExp.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(iterTp.clone()) {
                        Deref @ DAE::Type::T_ARRAY { dims: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    iterdims = __pa0.clone();
                    tp = Types::liftTypeWithDims(operTp.clone(), iterdims.clone())?;
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { exprType: ty, .. }, .. } => {
                    Ok(Types::simplifyType(ty.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { exp: _, sz: None } => {
                    Ok(Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { exp: _, sz: Some(_) } => {
                    Ok(DAE::T_INTEGER_DEFAULT().clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LIST { .. } => {
                    Ok(Arc::new(DAE::Type::T_METATYPE { ty: DAE::T_METALIST_DEFAULT().clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CONS { .. } => {
                    Ok(Arc::new(DAE::Type::T_METATYPE { ty: DAE::T_METALIST_DEFAULT().clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::META_TUPLE { listExp: exps } => {
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>>;
                    tys = List::map(exps.clone(), (std::sync::Arc::new(r#typeof) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Type>> + 'static>))?;
                    Ok(Arc::new(DAE::Type::T_METATYPE { ty: Arc::new(DAE::Type::T_METATUPLE { types: tys.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: exps } => {
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>>;
                    tys = List::map(exps.clone(), (std::sync::Arc::new(r#typeof) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Type>> + 'static>))?;
                    Ok(Arc::new(DAE::Type::T_TUPLE { types: tys.clone(), names: None }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::META_OPTION { .. } => {
                    Ok(Arc::new(DAE::Type::T_METATYPE { ty: DAE::T_NONE_DEFAULT().clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::METARECORDCALL { path: p, index: i, typeVars, .. } => {
                    Ok(Arc::new(DAE::Type::T_METATYPE { ty: Arc::new(DAE::Type::T_METARECORD { path: p.clone(), utPath: AbsynUtil::stripLast(p.clone())?, typeVars: typeVars.clone(), index: i.clone(), fields: metamodelica::nil(), knownSingleton: false }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BOX { exp: e } => {
                    Ok(Arc::new(DAE::Type::T_METATYPE { ty: Arc::new(DAE::Type::T_METABOXED { ty: r#typeof(e.clone())? }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATCHEXPRESSION { et: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNBOX { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SHARED_LITERAL { exp: e, .. } => {
                    Ok(r#typeof(e.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::EMPTY { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e => {
                    let mut msg: ArcStr;
                    msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Expression.typeof failed for ")); __mm_s.push_str(&*printExpStr(e.clone())?); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(msg.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outType)
}

fn typeofRelation(mut inType: Arc<DAE::Type>) -> Arc<DAE::Type> {
    let mut outType: Arc<DAE::Type>;
    outType = (::match_deref::match_deref! { match &(inType) {
        Deref @ DAE::Type::T_ARRAY { ty, dims } => {
            typeofRelation(ty.clone());
            Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: dims.clone() })
        },
        _ => {
            DAE::T_BOOL_DEFAULT().clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

pub(crate) fn typeofOp(mut inOperator: DAE::Operator) -> Result<Arc<DAE::Type>> {
    let mut outType: Arc<DAE::Type>;
    outType = (match inOperator {
        DAE::Operator::ADD { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::SUB { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::MUL { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::DIV { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::POW { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::UMINUS { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::UMINUS_ARR { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::ADD_ARR { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::SUB_ARR { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::MUL_ARR { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::DIV_ARR { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::MUL_ARRAY_SCALAR { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::ADD_ARRAY_SCALAR { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::SUB_SCALAR_ARRAY { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::MUL_SCALAR_PRODUCT { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::MUL_MATRIX_PRODUCT { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::DIV_ARRAY_SCALAR { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::DIV_SCALAR_ARRAY { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::POW_ARRAY_SCALAR { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::POW_SCALAR_ARRAY { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::POW_ARR { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::POW_ARR2 { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::AND { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::OR { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::NOT { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::LESS { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::LESSEQ { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::GREATER { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::GREATEREQ { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::EQUAL { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::NEQUAL { ty: ref t } => {
            t.clone()
        },
        DAE::Operator::USERDEFINED { .. } => {
            DAE::T_UNKNOWN_DEFAULT().clone()
        },
    });
    Ok(outType)
}

pub(crate) fn getRelations(mut inExp: Arc<DAE::Exp>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
        e @ Deref @ DAE::Exp::RELATION { .. } => {
            return list![e.clone()]
        },
        Deref @ DAE::Exp::LBINARY { exp1: e1, exp2: e2, .. } => {
            let mut rellst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut rellst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut rellst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            rellst1 = getRelations(e1.clone());
            rellst2 = getRelations(e2.clone());
            return listAppend(rellst1.clone(), rellst2.clone())
        },
        Deref @ DAE::Exp::LUNARY { exp: e, .. } => {
            let mut rellst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, exp2: e2, .. } => {
            let mut rellst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut rellst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut rellst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            rellst1 = getRelations(e1.clone());
            rellst2 = getRelations(e2.clone());
            return listAppend(rellst1.clone(), rellst2.clone())
        },
        Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: tb, expElse: fb } => {
            let mut rellst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut rellst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut rellst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut rellst3: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut rellst4: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            rellst1 = getRelations(cond.clone());
            rellst2 = getRelations(tb.clone());
            rellst3 = getRelations(fb.clone());
            rellst4 = listAppend(rellst1.clone(), rellst2.clone());
            return listAppend(rellst3.clone(), rellst4.clone())
        },
        Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut rellst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::ARRAY { ty: t, scalar: sc, array: Deref @ metamodelica::List::Cons { head: e, tail: xs } } => {
            let mut rellst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut rellst2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut rellst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            rellst1 = getRelations(Arc::new(DAE::Exp::ARRAY { ty: t.clone(), scalar: sc.clone(), array: xs.clone() }));
            rellst2 = getRelations(e.clone());
            return listAppend(rellst1.clone(), rellst2.clone())
        },
        Deref @ DAE::Exp::UNARY { exp: e, .. } => {
            let mut rellst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            { inExp = e.clone(); continue '__tco; }
        },
        _ => {
            return metamodelica::nil()
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn getAllCrefs(mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    (_, outCrefs) = traverseExpBottomUp(inExp, (std::sync::Arc::new(getAllCrefs2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
    Ok(outCrefs)
}

fn getAllCrefs2(mut inExp: Arc<DAE::Exp>, mut inCrefList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outCrefList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = inCrefList.clone();
    let mut cr: Arc<DAE::ComponentRef>;
    if isCref(inExp.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(inExp) {
            Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        cr = __pa0.clone();
        if !(ComponentReferenceBasics::crefEqual(cr.clone(), DAE::crefTime().clone())?) && !(listMember(cr.clone(), inCrefList)) {
            outCrefList = metamodelica::cons(cr, outCrefList);
        }
    }
    Ok((outExp, outCrefList))
}

pub fn getAllCrefsExpanded(mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    (_, outCrefs) = traverseExpBottomUp(inExp, (std::sync::Arc::new(getAllCrefsExpanded2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
    Ok(outCrefs)
}

fn getAllCrefsExpanded2(mut inExp: Arc<DAE::Exp>, mut inCrefList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outCrefList: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = inCrefList.clone();
    let mut cr: Arc<DAE::ComponentRef>;
    let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    if isCref(inExp.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(inExp) {
            Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        cr = __pa0.clone();
        crlst = ComponentReference::expandCref(cr, true);
        for mut c in &*crlst {
            let mut c = c.clone();
            if !(ComponentReferenceBasics::crefEqual(c.clone(), DAE::crefTime().clone())?) && !(listMember(c.clone(), inCrefList.clone())) {
                outCrefList = metamodelica::cons(c.clone(), outCrefList.clone());
            }
        }
    }
    Ok((outExp, outCrefList))
}

pub fn allTerms(mut inExp: Arc<DAE::Exp>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    outExpLst = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut f2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    f1 = allTerms(e1.clone());
                    f2 = allTerms(e2.clone());
                    res = listAppend(f1.clone(), f2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut f2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut f2_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    f1 = allTerms(e1.clone());
                    f2 = allTerms(e2.clone());
                    f2_1 = List::map(f2.clone(), (std::sync::Arc::new(negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    res = listAppend(f1.clone(), f2_1.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD_ARR { .. }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut f2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    f1 = allTerms(e1.clone());
                    f2 = allTerms(e2.clone());
                    res = listAppend(f1.clone(), f2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB_ARR { .. }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut f2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut f2_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    f1 = allTerms(e1.clone());
                    f2 = allTerms(e2.clone());
                    f2_1 = List::map(f2.clone(), (std::sync::Arc::new(negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    res = listAppend(f1.clone(), f2_1.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: _ }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(e2.clone())) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    f1 = __pa0.clone();
                    f1 = List::map1(f1.clone(), (std::sync::Arc::new(makeProduct) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e1.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), (std::sync::Arc::new(fnptr!(allTerms, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?)?;
                    Ok(f1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_ARR { ty: _ }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(e2.clone())) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    f1 = __pa0.clone();
                    f1 = List::map1(f1.clone(), (std::sync::Arc::new(makeProduct) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e1.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), (std::sync::Arc::new(fnptr!(allTerms, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?)?;
                    Ok(f1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: _ }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(e2.clone())) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    f1 = __pa0.clone();
                    f1 = List::map1(f1.clone(), (std::sync::Arc::new(makeProduct) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e1.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), (std::sync::Arc::new(fnptr!(allTerms, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?)?;
                    Ok(f1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: _ }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(e1.clone())) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    f1 = __pa0.clone();
                    f1 = List::map1(f1.clone(), (std::sync::Arc::new(makeProduct) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), (std::sync::Arc::new(fnptr!(allTerms, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?)?;
                    Ok(f1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_ARR { ty: _ }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(e1.clone())) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    f1 = __pa0.clone();
                    f1 = List::map1(f1.clone(), (std::sync::Arc::new(makeProduct) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), (std::sync::Arc::new(fnptr!(allTerms, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?)?;
                    Ok(f1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: _ }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(e1.clone())) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    f1 = __pa0.clone();
                    f1 = List::map1(f1.clone(), (std::sync::Arc::new(makeProduct) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), (std::sync::Arc::new(fnptr!(allTerms, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?)?;
                    Ok(f1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: _ }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(e1.clone())) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    f1 = __pa0.clone();
                    f1 = List::map1(f1.clone(), (std::sync::Arc::new(expDiv) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), (std::sync::Arc::new(fnptr!(allTerms, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?)?;
                    Ok(f1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_ARR { ty: _ }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(e1.clone())) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    f1 = __pa0.clone();
                    f1 = List::map1(f1.clone(), (std::sync::Arc::new(expDiv) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), (std::sync::Arc::new(fnptr!(allTerms, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?)?;
                    Ok(f1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_ARRAY_SCALAR { ty: _ }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(e1.clone())) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    f1 = __pa0.clone();
                    f1 = List::map1(f1.clone(), (std::sync::Arc::new(expDiv) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), (std::sync::Arc::new(fnptr!(allTerms, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?)?;
                    Ok(f1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_SCALAR_ARRAY { ty: _ }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(e1.clone())) {
                        __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    f1 = __pa0.clone();
                    f1 = List::map1(f1.clone(), (std::sync::Arc::new(expDiv) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), (std::sync::Arc::new(fnptr!(allTerms, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?)?;
                    Ok(f1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    f1 = allTerms(e1.clone());
                    f1 = List::map(f1.clone(), (std::sync::Arc::new(negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    Ok(f1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: e1 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    f1 = allTerms(e1.clone());
                    f1 = List::map(f1.clone(), (std::sync::Arc::new(negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    Ok(f1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { .. }, exp: e1 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    f1 = allTerms(e1.clone());
                    f1 = List::map(f1.clone(), (std::sync::Arc::new(negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    Ok(f1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp: e1, sub: subs } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut f2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    f2 = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
                    let __x = getSubscriptExp(sub.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    f1 = allTerms(e1.clone());
                    f1 = List::map1(f1.clone(), (std::sync::Arc::new(makeASUB) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Exp>> + 'static>), f2.clone())?;
                    Ok(f1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(list![inExp.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outExpLst
}

pub fn allTermsForCref(mut inExp: Arc<DAE::Exp>, mut cr: Arc<DAE::ComponentRef>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>;

    let mut outExpLstWithX: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut outExpLstWithoutX: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    (outExpLstWithX, outExpLstWithoutX) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut f2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut fx1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut fx2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut resx: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    (fx1, f1) = allTermsForCref(e1.clone(), cr.clone(), inFunc.clone())?;
                    (fx2, f2) = allTermsForCref(e2.clone(), cr.clone(), inFunc.clone())?;
                    res = listAppend(f1.clone(), f2.clone());
                    resx = listAppend(fx1.clone(), fx2.clone());
                    Ok((resx.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut f2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut fx1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut fx2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut resx: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    (fx1, f1) = allTermsForCref(e1.clone(), cr.clone(), inFunc.clone())?;
                    (fx2, f2) = allTermsForCref(e2.clone(), cr.clone(), inFunc.clone())?;
                    f2 = List::map(f2.clone(), (std::sync::Arc::new(negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    fx2 = List::map(fx2.clone(), (std::sync::Arc::new(negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    res = listAppend(f1.clone(), f2.clone());
                    resx = listAppend(fx1.clone(), fx2.clone());
                    Ok((resx.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD_ARR { .. }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut f2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut fx1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut fx2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut resx: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    (fx1, f1) = allTermsForCref(e1.clone(), cr.clone(), inFunc.clone())?;
                    (fx2, f2) = allTermsForCref(e2.clone(), cr.clone(), inFunc.clone())?;
                    res = listAppend(f1.clone(), f2.clone());
                    resx = listAppend(fx1.clone(), fx2.clone());
                    Ok((resx.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB_ARR { .. }, exp2: e2 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut f2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut fx1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut fx2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut resx: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    (fx1, f1) = allTermsForCref(e1.clone(), cr.clone(), inFunc.clone())?;
                    (fx2, f2) = allTermsForCref(e2.clone(), cr.clone(), inFunc.clone())?;
                    f2 = List::map(f2.clone(), (std::sync::Arc::new(negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    fx2 = List::map(fx2.clone(), (std::sync::Arc::new(negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    res = listAppend(f1.clone(), f2.clone());
                    resx = listAppend(fx1.clone(), fx2.clone());
                    Ok((resx.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: _ }, exp2: e2 } => {
                    if !((inFunc(e2.clone(), cr.clone())?)) { bail!("guard") }
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut f2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut fx1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut e: Arc<DAE::Exp>;
                    (fx1, f1) = allTermsForCref(e2.clone(), cr.clone(), inFunc.clone())?;
                    (fx1, f2) = List::split1OnTrue(fx1.clone(), inFunc.clone(), cr.clone())?;
                    res = listAppend(f1.clone(), f2.clone());
                    e = makeSum1(res.clone(), false)?;
                    e = expMul(e.clone(), e1.clone())?;
                    fx1 = List::map1(fx1.clone(), (std::sync::Arc::new(expMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e1.clone())?;
                    if !(isZero(e.clone())?) {
                        if expHasCrefNoPreOrStart(e1.clone(), cr.clone())? {
                            fx1 = metamodelica::cons(e.clone(), fx1.clone());
                            f1 = metamodelica::nil();
                        } else {
                            f1 = list![e.clone()];
                        }
                    }
                    Ok((fx1.clone(), f1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: _ }, exp2: e2 } => {
                    if !((inFunc(e1.clone(), cr.clone())?)) { bail!("guard") }
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut f2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut fx1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut e: Arc<DAE::Exp>;
                    (fx1, f1) = allTermsForCref(e1.clone(), cr.clone(), inFunc.clone())?;
                    (fx1, f2) = List::split1OnTrue(fx1.clone(), inFunc.clone(), cr.clone())?;
                    res = listAppend(f1.clone(), f2.clone());
                    e = makeSum1(res.clone(), false)?;
                    e = expMul(e.clone(), e2.clone())?;
                    fx1 = List::map1(fx1.clone(), (std::sync::Arc::new(expMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
                    if !(isZero(e.clone())?) {
                        if expHasCrefNoPreOrStart(e1.clone(), cr.clone())? {
                            fx1 = metamodelica::cons(e.clone(), fx1.clone());
                            f1 = metamodelica::nil();
                        } else {
                            f1 = list![e.clone()];
                        }
                    }
                    Ok((fx1.clone(), f1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: _ }, exp2: e2 } => {
                    if !((inFunc(e1.clone(), cr.clone())?)) { bail!("guard") }
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut f2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut fx1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut e: Arc<DAE::Exp>;
                    (fx1, f1) = allTermsForCref(e1.clone(), cr.clone(), inFunc.clone())?;
                    (fx1, f2) = List::split1OnTrue(fx1.clone(), inFunc.clone(), cr.clone())?;
                    res = listAppend(f1.clone(), f2.clone());
                    e = makeSum1(res.clone(), false)?;
                    e = makeDiv(e.clone(), e2.clone())?;
                    fx1 = List::map1(fx1.clone(), (std::sync::Arc::new(makeDiv) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
                    if !(isZero(e.clone())?) {
                        if expHasCrefNoPreOrStart(e1.clone(), cr.clone())? {
                            fx1 = metamodelica::cons(e.clone(), fx1.clone());
                            f1 = metamodelica::nil();
                        } else {
                            f1 = list![e.clone()];
                        }
                    }
                    Ok((fx1.clone(), f1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 } => {
                    let mut f1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut fx1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    (fx1, f1) = allTermsForCref(e1.clone(), cr.clone(), inFunc.clone())?;
                    f1 = List::map(f1.clone(), (std::sync::Arc::new(negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    fx1 = List::map(fx1.clone(), (std::sync::Arc::new(negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    Ok((fx1.clone(), f1.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut resx: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    if inFunc(inExp.clone(), cr.clone())? {
                        res = metamodelica::nil();
                        resx = list![inExp.clone()];
                    } else {
                        resx = metamodelica::nil();
                        res = list![inExp.clone()];
                    }
                    Ok((resx.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExpLstWithX, outExpLstWithoutX))
}

pub(crate) fn termsExpandUnary(mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    outExpLst = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e } => {
            List::map(terms(e.clone())?, (std::sync::Arc::new(negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?
        },
        _ => {
            terms(inExp)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExpLst)
}

pub fn terms(mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    outExpLst = terms2(inExp, metamodelica::nil(), false)?;
    Ok(outExpLst)
}

fn terms2(mut inExp: Arc<DAE::Exp>, mut inAcc: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut neg: bool) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExp, inAcc, neg)) {
        (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 }, acc, _) => {
            let mut acc = (*acc).clone();
            acc = terms2(e2.clone(), acc.clone(), neg)?;
            { (inExp, inAcc, neg) = (e1.clone(), acc.clone(), neg); continue '__tco; }
        },
        (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 }, acc, _) => {
            let mut acc = (*acc).clone();
            acc = terms2(e2.clone(), acc.clone(), !(neg))?;
            { (inExp, inAcc, neg) = (e1.clone(), acc.clone(), neg); continue '__tco; }
        },
        (e, acc, true) => {
            let mut e = (*e).clone();
            e = negate(e.clone())?;
            return Ok(metamodelica::cons(e.clone(), acc.clone()))
        },
        (e, acc, _) => {
            return Ok(metamodelica::cons(e.clone(), acc.clone()))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn quotient(mut inExp: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>)> {
    let mut num: Arc<DAE::Exp>;
    let mut denom: Arc<DAE::Exp>;
    (num, denom) = 'mc: {
        let __mc_input = inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } => {
                    Ok((e1.clone(), e2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
                    let mut p: Arc<DAE::Exp>;
                    let mut q: Arc<DAE::Exp>;
                    let mut tp: Type;
                    (p, q) = quotient(e1.clone())?;
                    tp = r#typeof(p.clone())?;
                    Ok((Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: p.clone() }), q.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
                    let mut p: Arc<DAE::Exp>;
                    let mut q: Arc<DAE::Exp>;
                    let mut tp: Type;
                    (p, q) = quotient(e2.clone())?;
                    tp = r#typeof(p.clone())?;
                    Ok((Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: p.clone() }), q.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((num, denom))
}

pub fn factors(mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    outExpLst = factorsWork(inExp, metamodelica::nil(), false)?.reverse();
    Ok(outExpLst)
}

fn factorsWork(mut inExp: Arc<DAE::Exp>, mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut doInverseFactors: bool) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = acc;
    acc = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
            acc = factorsWork(e1.clone(), acc, doInverseFactors)?;
            acc = factorsWork(e2.clone(), acc, doInverseFactors)?;
            acc
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: e2 } => {
            acc = factorsWork(e1.clone(), acc, doInverseFactors)?;
            acc = factorsWork(e2.clone(), acc, !(doInverseFactors))?;
            acc
        },
        Deref @ DAE::Exp::ICONST { integer: 1 } => {
            acc
        },
        Deref @ DAE::Exp::RCONST { real: __rlit_0 } if __rlit_0.eq(&metamodelica::OrderedFloat((1.0) as f64)) => {
            acc
        },
        _ => {
            metamodelica::cons(if (doInverseFactors) {inverseFactors(inExp)?} else {inExp}, acc)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(acc)
}

pub fn inverseFactors(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: tp }, exp2: e2 } => {
                    let mut tp2: Type;
                    tp2 = r#typeof(e2.clone())?;
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp2.clone() }, exp: e2.clone() }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::DIV { .. }, exp2: e2 } => {
                    let false = (isZero(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op.clone(), exp2: e1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e => {
                    let mut tp: Type;
                    let mut e = (*e).clone();
                    let false = (isZero(e.clone())?) else { bail!("pattern mismatch") };
                    tp = r#typeof(e.clone())?;
                    e = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ DAE::Type::T_REAL { .. } => Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::DIV { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }),
        Deref @ DAE::Type::T_INTEGER { .. } => Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::ICONST { integer: 1 }), operator: DAE::Operator::DIV { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: e.clone() }),
        _ => bail!("match: no arm matched"),
    } });
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

pub fn expandFactors(mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    outExpLst = expandFactorsWork(inExp, metamodelica::nil(), false)?.reverse();
    Ok(outExpLst)
}

fn expandFactorsWork(mut inExp: Arc<DAE::Exp>, mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut doInverseFactors: bool) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = acc;
    acc = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 }, operator: DAE::Operator::POW { .. }, exp2: e3 } => {
            let mut pow_acc: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut pow_acc2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            pow_acc = expandFactorsWork(e1.clone(), metamodelica::nil(), doInverseFactors)?;
            pow_acc = expPowLst(pow_acc.clone(), e3.clone())?;
            pow_acc2 = expandFactorsWork(e2.clone(), metamodelica::nil(), doInverseFactors)?;
            pow_acc2 = expPowLst(pow_acc2.clone(), e3.clone())?;
            acc = listAppend(pow_acc.clone(), acc);
            acc = listAppend(pow_acc2.clone(), acc);
            acc
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 }, operator: DAE::Operator::POW { .. }, exp2: e3 } => {
            let mut pow_acc: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut pow_acc2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            pow_acc = expandFactorsWork(e1.clone(), metamodelica::nil(), doInverseFactors)?;
            pow_acc = expPowLst(pow_acc.clone(), e3.clone())?;
            pow_acc2 = expandFactorsWork(e2.clone(), metamodelica::nil(), doInverseFactors)?;
            pow_acc2 = expPowLst(pow_acc2.clone(), negate(e3.clone())?)?;
            acc = listAppend(pow_acc.clone(), acc);
            acc = listAppend(pow_acc2.clone(), acc);
            acc
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: e2 }, operator: DAE::Operator::POW { .. }, exp2: e3 } => {
            let mut e: Arc<DAE::Exp>;
            let mut pow_acc: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            e = expMul(e2.clone(), e3.clone())?;
            pow_acc = expandFactorsWork(e1.clone(), metamodelica::nil(), doInverseFactors)?;
            pow_acc = expPowLst(pow_acc.clone(), e.clone())?;
            acc = listAppend(pow_acc.clone(), acc);
            acc
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::DIV { ty: tp }, exp2: e2 } if (isZero(e2.clone())?) => {
            let mut e: Arc<DAE::Exp>;
            if doInverseFactors {
                e = e2.clone();
            } else {
                e = Arc::new(DAE::Exp::BINARY { exp1: makeConstOne(tp.clone()), operator: op.clone(), exp2: e2.clone() });
            }
            acc = expandFactorsWork(e1.clone(), acc, doInverseFactors)?;
            metamodelica::cons(e.clone(), acc)
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp }, exp: e1 } => {
            let mut e: Arc<DAE::Exp>;
            e = makeConstOne(tp.clone());
            acc = expandFactorsWork(e1.clone(), acc, doInverseFactors)?;
            e = negate(e.clone())?;
            metamodelica::cons(e.clone(), acc)
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp }, exp: e1 } => {
            let mut e: Arc<DAE::Exp>;
            e = makeConstOne(tp.clone());
            acc = expandFactorsWork(e1.clone(), acc, doInverseFactors)?;
            e = negate(e.clone())?;
            metamodelica::cons(e.clone(), acc)
        },
        _ => {
            acc = expandFactorsWork3(inExp, acc, doInverseFactors)?;
            expandFactorsWork2(acc, doInverseFactors)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(acc)
}

fn expandFactorsWork3(mut inExp: Arc<DAE::Exp>, mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut doInverseFactors: bool) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = acc;
    let mut e1: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut e2: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut e: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut op: Operator = <DAE::Operator as ::std::default::Default>::default();
    acc = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(factorsWork(inExp.clone(), acc.clone(), doInverseFactors)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
                    let mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = acc.clone();
                    acc = expandFactorsWork(e1.clone(), acc.clone(), doInverseFactors)?;
                    acc = expandFactorsWork(e2.clone(), acc.clone(), doInverseFactors)?;
                    Ok((acc.clone(), acc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { acc = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e, operator: op @ DAE::Operator::DIV { .. }, exp2: e1 }, operator: DAE::Operator::DIV { .. }, exp2: e2 } => {
                    let mut e = (*e).clone();
                    let mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = acc.clone();
                    e = Arc::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op.clone(), exp2: expMul(e1.clone(), e2.clone())? });
                    acc = expandFactorsWork(e.clone(), acc.clone(), doInverseFactors)?;
                    Ok((acc.clone(), acc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { acc = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e, operator: DAE::Operator::MUL { .. }, exp2: e1 }, operator: op @ DAE::Operator::DIV { .. }, exp2: e2 } => {
                    let mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = acc.clone();
                    acc = expandFactorsWork(e.clone(), acc.clone(), doInverseFactors)?;
                    acc = expandFactorsWork(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() }), acc.clone(), doInverseFactors)?;
                    Ok((acc.clone(), acc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { acc = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::cons(inExp.clone(), acc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(acc)
}

fn expandFactorsWork2(mut inAcc: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut doInverseFactors: bool) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut tmpExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    for mut elem in &*inAcc {
        let mut elem = elem.clone();
        tmpExpLst = (::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: _, operator: DAE::Operator::DIV { .. }, exp2: _ }, operator: DAE::Operator::POW { .. }, exp2: _ } => expandFactorsWork(elem.clone(), metamodelica::nil(), doInverseFactors)?,
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: _, operator: DAE::Operator::MUL { .. }, exp2: _ }, operator: DAE::Operator::POW { .. }, exp2: _ } => expandFactorsWork(elem.clone(), metamodelica::nil(), doInverseFactors)?,
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: _, operator: DAE::Operator::POW { .. }, exp2: _ }, operator: DAE::Operator::POW { .. }, exp2: _ } => expandFactorsWork(elem.clone(), metamodelica::nil(), doInverseFactors)?,
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: _ } => expandFactorsWork(elem.clone(), metamodelica::nil(), doInverseFactors)?,
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: _ } => expandFactorsWork(elem.clone(), metamodelica::nil(), doInverseFactors)?,
        _ => list![elem.clone()],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outExpLst = listAppend(tmpExpLst.clone(), outExpLst.clone());
    }
    Ok(outExpLst)
}

pub(crate) fn getTermsContainingX(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>)> {
    let mut outExp1: Arc<DAE::Exp>;
    let mut outExp2: Arc<DAE::Exp>;
    (outExp1, outExp2) = 'mc: {
        let __mc_input = (inExp1, inExp2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { ty }, exp2: e2 }, cr @ Deref @ DAE::Exp::CREF { .. }) => {
                    let mut xt1: Arc<DAE::Exp>;
                    let mut nonxt1: Arc<DAE::Exp>;
                    let mut xt2: Arc<DAE::Exp>;
                    let mut nonxt2: Arc<DAE::Exp>;
                    let mut xt: Arc<DAE::Exp>;
                    let mut nonxt: Arc<DAE::Exp>;
                    (xt1, nonxt1) = getTermsContainingX(e1.clone(), cr.clone())?;
                    (xt2, nonxt2) = getTermsContainingX(e2.clone(), cr.clone())?;
                    xt = Arc::new(DAE::Exp::BINARY { exp1: xt1.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: xt2.clone() });
                    nonxt = Arc::new(DAE::Exp::BINARY { exp1: nonxt1.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: nonxt2.clone() });
                    Ok((xt.clone(), nonxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { ty }, exp2: e2 }, cr @ Deref @ DAE::Exp::CREF { .. }) => {
                    let mut xt1: Arc<DAE::Exp>;
                    let mut nonxt1: Arc<DAE::Exp>;
                    let mut xt2: Arc<DAE::Exp>;
                    let mut nonxt2: Arc<DAE::Exp>;
                    let mut xt: Arc<DAE::Exp>;
                    let mut nonxt: Arc<DAE::Exp>;
                    (xt1, nonxt1) = getTermsContainingX(e1.clone(), cr.clone())?;
                    (xt2, nonxt2) = getTermsContainingX(e2.clone(), cr.clone())?;
                    xt = Arc::new(DAE::Exp::BINARY { exp1: xt1.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: xt2.clone() });
                    nonxt = Arc::new(DAE::Exp::BINARY { exp1: nonxt1.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: nonxt2.clone() });
                    Ok((xt.clone(), nonxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty }, exp: e }, cr @ Deref @ DAE::Exp::CREF { .. }) => {
                    let mut xt1: Arc<DAE::Exp>;
                    let mut nonxt1: Arc<DAE::Exp>;
                    let mut xt: Arc<DAE::Exp>;
                    let mut nonxt: Arc<DAE::Exp>;
                    (xt1, nonxt1) = getTermsContainingX(e.clone(), cr.clone())?;
                    xt = Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: xt1.clone() });
                    nonxt = Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: nonxt1.clone() });
                    Ok((xt.clone(), nonxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD_ARR { ty }, exp2: e2 }, cr @ Deref @ DAE::Exp::CREF { .. }) => {
                    let mut xt1: Arc<DAE::Exp>;
                    let mut nonxt1: Arc<DAE::Exp>;
                    let mut xt2: Arc<DAE::Exp>;
                    let mut nonxt2: Arc<DAE::Exp>;
                    let mut xt: Arc<DAE::Exp>;
                    let mut nonxt: Arc<DAE::Exp>;
                    (xt1, nonxt1) = getTermsContainingX(e1.clone(), cr.clone())?;
                    (xt2, nonxt2) = getTermsContainingX(e2.clone(), cr.clone())?;
                    xt = Arc::new(DAE::Exp::BINARY { exp1: xt1.clone(), operator: DAE::Operator::ADD_ARR { ty: ty.clone() }, exp2: xt2.clone() });
                    nonxt = Arc::new(DAE::Exp::BINARY { exp1: nonxt1.clone(), operator: DAE::Operator::ADD_ARR { ty: ty.clone() }, exp2: nonxt2.clone() });
                    Ok((xt.clone(), nonxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB_ARR { ty }, exp2: e2 }, cr @ Deref @ DAE::Exp::CREF { .. }) => {
                    let mut xt1: Arc<DAE::Exp>;
                    let mut nonxt1: Arc<DAE::Exp>;
                    let mut xt2: Arc<DAE::Exp>;
                    let mut nonxt2: Arc<DAE::Exp>;
                    let mut xt: Arc<DAE::Exp>;
                    let mut nonxt: Arc<DAE::Exp>;
                    (xt1, nonxt1) = getTermsContainingX(e1.clone(), cr.clone())?;
                    (xt2, nonxt2) = getTermsContainingX(e2.clone(), cr.clone())?;
                    xt = Arc::new(DAE::Exp::BINARY { exp1: xt1.clone(), operator: DAE::Operator::SUB_ARR { ty: ty.clone() }, exp2: xt2.clone() });
                    nonxt = Arc::new(DAE::Exp::BINARY { exp1: nonxt1.clone(), operator: DAE::Operator::SUB_ARR { ty: ty.clone() }, exp2: nonxt2.clone() });
                    Ok((xt.clone(), nonxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty }, exp: e }, cr @ Deref @ DAE::Exp::CREF { .. }) => {
                    let mut xt1: Arc<DAE::Exp>;
                    let mut nonxt1: Arc<DAE::Exp>;
                    let mut xt: Arc<DAE::Exp>;
                    let mut nonxt: Arc<DAE::Exp>;
                    (xt1, nonxt1) = getTermsContainingX(e.clone(), cr.clone())?;
                    xt = Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: xt1.clone() });
                    nonxt = Arc::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: nonxt1.clone() });
                    Ok((xt.clone(), nonxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, cr @ Deref @ DAE::Exp::CREF { ty, .. }) => {
                    let mut xt: Arc<DAE::Exp>;
                    let mut nonxt: Arc<DAE::Exp>;
                    let mut zero: Arc<DAE::Exp>;
                    let mut res: bool;
                    res = expContains(e.clone(), cr.clone())?;
                    (zero, _) = makeZeroExpression(arrayDimension(ty.clone()))?;
                    xt = if (res.clone()) {e.clone()} else {zero.clone()};
                    nonxt = if (res.clone()) {zero.clone()} else {e.clone()};
                    Ok((xt.clone(), nonxt.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp1, outExp2))
}

pub fn flattenArrayExpToList(mut e: Arc<DAE::Exp>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    expLst = 'mc: {
        let __mc_input = e.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: Deref @ DAE::Exp::ARRAY { array: expl, .. } } => {
                    let mut expl = (*expl).clone();
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = expLst.clone();
                    expl = List::flatten(List::map(expl.clone(), (std::sync::Arc::new(fnptr!(flattenArrayExpToList, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?)?;
                    expLst = List::map(expl.clone(), (std::sync::Arc::new(negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    Ok((expLst.clone(), expLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { expLst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = expLst.clone();
                    expLst = List::flatten(List::map(expl.clone(), (std::sync::Arc::new(fnptr!(flattenArrayExpToList, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?)?;
                    Ok((expLst.clone(), expLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { expLst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: Deref @ DAE::Exp::MATRIX { matrix: mexpl, .. } } => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = expLst.clone();
                    expl = List::flatten(List::map(List::flatten(mexpl.clone())?, (std::sync::Arc::new(fnptr!(flattenArrayExpToList, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?)?;
                    expLst = List::map(expl.clone(), (std::sync::Arc::new(negate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    Ok((expLst.clone(), expLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { expLst = __wb0; break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { matrix: mexpl, .. } => {
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>> = expLst.clone();
                    expLst = List::flatten(List::map(List::flatten(mexpl.clone())?, (std::sync::Arc::new(fnptr!(flattenArrayExpToList, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?)?;
                    Ok((expLst.clone(), expLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { expLst = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(list![e.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    expLst
}

/* **************************************************/
/* generate  */
/* **************************************************/
pub fn makeNoEvent(mut e1: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut res: Arc<DAE::Exp>;
    res = makePureBuiltinCall((literal!("noEvent")).clone(), list![e1], DAE::T_BOOL_DEFAULT().clone());
    res
}

pub fn makeAbs(mut e1: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut res: Arc<DAE::Exp>;
    res = makePureBuiltinCall((literal!("abs")).clone(), list![e1], DAE::T_REAL_DEFAULT().clone());
    res
}

pub fn makeSign(mut e1: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut res: Arc<DAE::Exp>;
    res = makePureBuiltinCall((literal!("sign")).clone(), list![e1], DAE::T_REAL_DEFAULT().clone());
    res
}

pub fn makeNestedIf(mut inConds: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inTbExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut fExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut ifExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    ifExp = (::match_deref::match_deref! { match &((inConds, inTbExps)) {
        (Deref @ metamodelica::List::Cons { head: c, tail: Deref @ metamodelica::List::Nil }, Deref @ metamodelica::List::Cons { head: tbExp, tail: Deref @ metamodelica::List::Nil }) => {
            Arc::new(DAE::Exp::IFEXP { expCond: c.clone(), expThen: tbExp.clone(), expElse: fExp })
        },
        (Deref @ metamodelica::List::Cons { head: c, tail: conds }, Deref @ metamodelica::List::Cons { head: tbExp, tail: tbExps }) => {
            ifExp = makeNestedIf(conds.clone(), tbExps.clone(), fExp)?;
            Arc::new(DAE::Exp::IFEXP { expCond: c.clone(), expThen: tbExp.clone(), expElse: ifExp })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ifExp)
}

pub fn makeCrefExp(mut inCref: Arc<DAE::ComponentRef>, mut inExpType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inCref, inExpType)) {
        (cref, tGiven) => {
            let mut tExisting: Type;
            if Flags::isSet(Flags::CHECK_DAE_CREF_TYPE.clone())? {
                tExisting = ComponentReference::crefLastType(cref.clone())?;
                if !(tGiven.clone() == tExisting.clone()) {
                    Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Warning: Expression.makeCrefExp: cref ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cref.clone())?); __mm_s.push_str(&*literal!(" was given type DAE.CREF.ty: ")); __mm_s.push_str(&*TypesDump::unparseType(tGiven.clone())?); __mm_s.push_str(&*literal!(" is different from existing DAE.CREF.componentRef.ty: ")); __mm_s.push_str(&*TypesDump::unparseType(tExisting.clone())?); ArcStr::from(__mm_s) }).clone())?;
                }
            }
            Arc::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: tGiven.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn crefToExp(mut cr: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> {
    let mut cref: Arc<DAE::Exp>;
    cref = Arc::new(DAE::Exp::CREF { componentRef: cr.clone(), ty: ComponentReference::crefTypeFull(cr)? });
    Ok(cref)
}

pub fn crefExp(mut cr: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> {
    let mut cref: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    cref = (::match_deref::match_deref! { match &(cr.clone()) {
        Deref @ DAE::ComponentRef::WILD { .. } => {
            Arc::new(DAE::Exp::CREF { componentRef: cr, ty: openmodelica_frontend_types::DAE::Type::interned_T_UNKNOWN() })
        },
        _ => {
            let mut ty1: Type;
            let mut ty2: Type = Arc::new(DAE::Type::T_NORETCALL);
            let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            ty1 = ComponentReference::crefLastType(cr.clone())?;
            cref = (::match_deref::match_deref! { match &(ty1.clone()) {
        Deref @ DAE::Type::T_ARRAY { .. } => {
            subs = ComponentReference::crefLastSubs(cr.clone())?;
            ty2 = unliftArrayTypeWithSubs(subs.clone(), ty1.clone())?;
            Arc::new(DAE::Exp::CREF { componentRef: cr, ty: ty2.clone() })
        },
        _ => Arc::new(DAE::Exp::CREF { componentRef: cr, ty: ty1.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            cref
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub fn makeASUB(mut inExp: Arc<DAE::Exp>, mut inSubs: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut inSubs_: Arc<metamodelica::List<Arc<DAE::Subscript>>> = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
        for mut s in (inSubs.clone()).into_iter().cloned() {
            let __x = makeIndexSubscript(s.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outExp = (::match_deref::match_deref! { match &((inExp.clone(), inSubs_.clone())) {
        (Deref @ DAE::Exp::ASUB { exp, sub: subs1 }, subs2) => {
            let mut subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut exp = (*exp).clone();
            subs = listAppend(subs1.clone(), subs2.clone());
            exp = Arc::new(DAE::Exp::ASUB { exp: exp.clone(), sub: subs.clone() });
            exp.clone()
        },
        (_, _) => {
            let mut exp: Arc<DAE::Exp>;
            if Flags::isSet(Flags::CHECK_ASUB.clone())? {
                let () = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { .. } => {
            Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Warning: makeASUB: given expression: ")); __mm_s.push_str(&*printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!(" contains a component reference!\n")); __mm_s.push_str(&*literal!(" Subscripts exps: [")); __mm_s.push_str(&*stringDelimitList(List::map(inSubs, (std::sync::Arc::new(printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))?, (literal!(",")).clone())); __mm_s.push_str(&*literal!("]\n")); __mm_s.push_str(&*literal!("DAE.ASUB should not be used for component references, instead the subscripts should be added directly to the component reference!")); ArcStr::from(__mm_s) }).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            exp = Arc::new(DAE::Exp::ASUB { exp: inExp, sub: inSubs_ });
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn makeASUBSingleSub(mut exp: Arc<DAE::Exp>, mut sub: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = makeASUB(exp, list![sub])?;
    Ok(outExp)
}

pub fn makeTuple(mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = if ((inExps.clone().len() as i32) > 1) {Arc::new(DAE::Exp::TUPLE { PR: inExps })} else {listHead(inExps)?};
    Ok(outExp)
}

pub fn generateCrefsExpFromExpVar(mut inVar: Arc<DAE::Var>, mut inCrefPrefix: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> {
    let mut outCrefExp: Arc<DAE::Exp>;
    outCrefExp = (::match_deref::match_deref! { match &(inVar) {
        Deref @ DAE::Var { name, ty, .. } => {
            let mut cr: Arc<DAE::ComponentRef>;
            let mut e: Arc<DAE::Exp>;
            cr = ComponentReference::crefPrependIdent(inCrefPrefix, (name.clone()).clone(), metamodelica::nil(), ty.clone())?;
            e = makeCrefExp(cr.clone(), ty.clone())?;
            e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCrefExp)
}

pub(crate) fn generateCrefsFromExpVar(mut inVar: Arc<DAE::Var>, mut inCrefPrefix: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outCref: Arc<DAE::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &(inVar) {
        Deref @ DAE::Var { name, ty, .. } => {
            let mut cr: Arc<DAE::ComponentRef>;
            cr = ComponentReference::crefPrependIdent(inCrefPrefix, (name.clone()).clone(), metamodelica::nil(), ty.clone())?;
            cr.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCref)
}

pub fn generateCrefsExpFromExp(mut inExp: Arc<DAE::Exp>, mut inCrefPrefix: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> {
    let mut outCrefExp: Arc<DAE::Exp>;
    outCrefExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. } => {
            inExp
        },
        Deref @ DAE::Exp::ARRAY { ty, scalar: b, array: explst } => {
            let mut explst = (*explst).clone();
            explst = List::map1(explst.clone(), (std::sync::Arc::new(generateCrefsExpFromExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), inCrefPrefix)?;
            Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: b.clone(), array: explst.clone() })
        },
        Deref @ DAE::Exp::CALL { path: p1, expLst: explst, attr: attr @ Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p2 }, .. }, .. } } => {
            let mut explst = (*explst).clone();
            let true = (AbsynUtil::pathEqual(p1.clone(), p2.clone())) else { bail!("pattern mismatch") };
            explst = List::map1(explst.clone(), (std::sync::Arc::new(generateCrefsExpFromExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), inCrefPrefix)?;
            Arc::new(DAE::Exp::CALL { path: p1.clone(), expLst: explst.clone(), attr: attr.clone() })
        },
        Deref @ DAE::Exp::RECORD { path: p1, exps: explst, comp: fields, ty } => {
            let mut explst = (*explst).clone();
            explst = List::map1(explst.clone(), (std::sync::Arc::new(generateCrefsExpFromExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), inCrefPrefix)?;
            Arc::new(DAE::Exp::RECORD { path: p1.clone(), exps: explst.clone(), comp: fields.clone(), ty: ty.clone() })
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, ty } => {
            let mut name: ArcStr;
            let mut e: Arc<DAE::Exp>;
            let mut cr = (*cr).clone();
            name = (ComponentReference::crefModelicaStr(cr.clone())).clone();
            cr = ComponentReference::crefPrependIdent(inCrefPrefix, (name.clone()).clone(), metamodelica::nil(), ty.clone())?;
            e = makeCrefExp(cr.clone(), ty.clone())?;
            e.clone()
        },
        Deref @ DAE::Exp::UNARY { exp: e, .. } => {
            negate(generateCrefsExpFromExp(e.clone(), inCrefPrefix)?)?
        },
        _ => {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.generateCrefsExpFromExp: fail for")); __mm_s.push_str(&*printExpStr(inExp)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCrefExp)
}

pub fn generateCrefsExpLstFromExp(mut inExp: Arc<DAE::Exp>, mut inCrefPrefix: Option<Arc<DAE::ComponentRef>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExp.clone(), inCrefPrefix.clone())) {
        (Deref @ DAE::Exp::TUPLE { PR: explst }, _) => {
            let mut explst = (*explst).clone();
            return Ok(List::flatten(List::map1(explst.clone(), (std::sync::Arc::new(generateCrefsExpLstFromExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), inCrefPrefix)?)?)
        },
        (Deref @ DAE::Exp::ARRAY { array: explst, .. }, _) => {
            let mut explst = (*explst).clone();
            return Ok(List::flatten(List::map1(explst.clone(), (std::sync::Arc::new(generateCrefsExpLstFromExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), inCrefPrefix)?)?)
        },
        (Deref @ DAE::Exp::CALL { path: p1, expLst: explst, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p2 }, .. }, .. } }, _) if (AbsynUtil::pathEqual(p1.clone(), p2.clone())) => {
            return Ok(List::flatten(List::map1(explst.clone(), (std::sync::Arc::new(generateCrefsExpLstFromExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), inCrefPrefix)?)?)
        },
        (Deref @ DAE::Exp::RECORD { exps: explst, .. }, _) => {
            return Ok(List::flatten(List::map1(explst.clone(), (std::sync::Arc::new(generateCrefsExpLstFromExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>), inCrefPrefix)?)?)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: incref, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, _) => {
            let mut cr: Arc<DAE::ComponentRef>;
            let mut e: Arc<DAE::Exp>;
            cr = ComponentReference::crefPrefixDer(incref.clone());
            e = crefExp(cr.clone())?;
            { (inExp, inCrefPrefix) = (e.clone(), inCrefPrefix); continue '__tco; }
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr, ty }, Some(incref)) => {
            let mut name: ArcStr;
            let mut e: Arc<DAE::Exp>;
            let mut cr = (*cr).clone();
            name = (ComponentReference::crefModelicaStr(cr.clone())).clone();
            cr = ComponentReference::crefPrependIdent(incref.clone(), (name.clone()).clone(), metamodelica::nil(), ty.clone())?;
            e = makeCrefExp(cr.clone(), ty.clone())?;
            return Ok(list![e.clone()])
        },
        (Deref @ DAE::Exp::CREF { .. }, None) => {
            return Ok(list![inExp])
        },
        (Deref @ DAE::Exp::UNARY { exp: e, .. }, _) => {
            { (inExp, inCrefPrefix) = (e.clone(), inCrefPrefix); continue '__tco; }
        },
        _ => {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.generateCrefsExpLstFromExp: fail for ")); __mm_s.push_str(&*printExpStr(inExp)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            return Ok(bail!("fail"))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn makeArray(mut inElements: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inType: Arc<DAE::Type>, mut inScalar: bool) -> Arc<DAE::Exp> {
    let mut outArray: Arc<DAE::Exp>;
    outArray = Arc::new(DAE::Exp::ARRAY { ty: inType, scalar: inScalar, array: inElements });
    outArray
}

pub(crate) fn makeArrayFromList(mut inElements: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Exp>> {
    let mut outArray: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type>;
    ty = r#typeof(listHead(inElements.clone())?)?;
    outArray = Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: (inElements.clone().len() as i32) })] }), scalar: !(Types::isArray(ty)), array: inElements });
    Ok(outArray)
}

pub fn makeScalarArray(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut et: Arc<DAE::Type>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    let mut i: i32;
    i = (inExpLst.clone().len() as i32);
    outExp = Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: et, dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: i })] }), scalar: true, array: inExpLst });
    outExp
}

pub(crate) fn makeRealArray(mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = makeScalarArray(expl, DAE::T_REAL_DEFAULT().clone());
    outExp
}

pub fn makeRealAdd(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = Arc::new(DAE::Exp::BINARY { exp1: inExp1, operator: DAE::Operator::ADD { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: inExp2 });
    outExp
}

pub fn expAdd(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((e1.clone(), e2.clone())) {
        (_, _) if (isZero(e1.clone())?) => {
            e2.clone()
        },
        (_, _) if (isZero(e2.clone())?) => {
            e1.clone()
        },
        (Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::RCONST { real: r2 }) => {
            Arc::new(DAE::Exp::RCONST { real: r1.clone() + r2.clone() })
        },
        (Deref @ DAE::Exp::ICONST { integer: i1 }, Deref @ DAE::Exp::ICONST { integer: i2 }) => {
            Arc::new(DAE::Exp::ICONST { integer: i1.clone() + i2.clone() })
        },
        (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e }) => {
            expSub(e1.clone(), e.clone())?
        },
        (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: e }) => {
            expSub(e1.clone(), e.clone())?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: x }, operator: op @ DAE::Operator::MUL { .. }, exp2: y }) => {
            expSub(e1.clone(), Arc::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: x }, operator: op @ DAE::Operator::MUL_ARR { .. }, exp2: y }) => {
            expSub(e1.clone(), Arc::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: x }, operator: op @ DAE::Operator::DIV { .. }, exp2: y }) => {
            expSub(e1.clone(), Arc::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: x }, operator: op @ DAE::Operator::DIV_ARR { .. }, exp2: y }) => {
            expSub(e1.clone(), Arc::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e }, _) => {
            expSub(e2.clone(), e.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: e }, _) => {
            expSub(e2.clone(), e.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: x }, operator: op @ DAE::Operator::MUL { .. }, exp2: y }, _) => {
            expSub(e2.clone(), Arc::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: x }, operator: op @ DAE::Operator::MUL_ARR { .. }, exp2: y }, _) => {
            expSub(e2.clone(), Arc::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: x }, operator: op @ DAE::Operator::DIV { .. }, exp2: y }, _) => {
            expSub(e2.clone(), Arc::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: x }, operator: op @ DAE::Operator::DIV_ARR { .. }, exp2: y }, _) => {
            expSub(e2.clone(), Arc::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (_, _) if (Types::isIntegerOrRealOrSubTypeOfEither(r#typeof(e1.clone())?)) => {
            let mut tp: Type;
            let mut b: bool;
            let mut op: Operator;
            tp = r#typeof(e1.clone())?;
            b = DAEUtil::expTypeArray(tp.clone());
            op = if (b.clone()) {DAE::Operator::ADD_ARR { ty: tp.clone() }} else {DAE::Operator::ADD { ty: tp.clone() }};
            Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() })
        },
        (_, _) if (Types::isEnumeration(r#typeof(e1.clone())?)) => {
            Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::ADD { ty: r#typeof(e1.clone())? }, exp2: e2.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn expSub(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((e1.clone(), e2.clone())) {
        (_, _) if (isZero(e1.clone())?) => {
            negate(e2.clone())?
        },
        (_, _) if (isZero(e2.clone())?) => {
            e1.clone()
        },
        (Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::RCONST { real: r2 }) => {
            Arc::new(DAE::Exp::RCONST { real: r1.clone() - r2.clone() })
        },
        (Deref @ DAE::Exp::ICONST { integer: i1 }, Deref @ DAE::Exp::ICONST { integer: i2 }) => {
            Arc::new(DAE::Exp::ICONST { integer: i1.clone() - i2.clone() })
        },
        (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e }) => {
            expAdd(e1.clone(), e.clone())?
        },
        (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: e }) => {
            expAdd(e1.clone(), e.clone())?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: x }, operator: op @ DAE::Operator::MUL { .. }, exp2: y }) => {
            expAdd(e1.clone(), Arc::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: x }, operator: op @ DAE::Operator::MUL_ARR { .. }, exp2: y }) => {
            expAdd(e1.clone(), Arc::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: x }, operator: op @ DAE::Operator::DIV { .. }, exp2: y }) => {
            expAdd(e1.clone(), Arc::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: x }, operator: op @ DAE::Operator::DIV_ARR { .. }, exp2: y }) => {
            expAdd(e1.clone(), Arc::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e }, _) => {
            let mut e = (*e).clone();
            e = expAdd(e.clone(), e2.clone())?;
            negate(e.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: e }, _) => {
            let mut e = (*e).clone();
            e = expAdd(e.clone(), e2.clone())?;
            negate(e.clone())?
        },
        (_, _) if (Types::isIntegerOrRealOrSubTypeOfEither(r#typeof(e1.clone())?)) => {
            let mut tp: Type;
            let mut b: bool;
            let mut op: Operator;
            tp = r#typeof(e1.clone())?;
            b = DAEUtil::expTypeArray(tp.clone());
            op = if (b.clone()) {DAE::Operator::SUB_ARR { ty: tp.clone() }} else {DAE::Operator::SUB { ty: tp.clone() }};
            Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() })
        },
        (_, _) if (Types::isEnumeration(r#typeof(e1.clone())?)) => {
            Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: r#typeof(e1.clone())? }, exp2: e2.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub(crate) fn makeLBinary(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut op: DAE::Operator) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inExpLst, op.clone())) {
        (Deref @ metamodelica::List::Nil, DAE::Operator::AND { ty: _ }) => {
            Arc::new(DAE::Exp::BCONST { bool: true })
        },
        (Deref @ metamodelica::List::Nil, DAE::Operator::OR { ty: _ }) => {
            Arc::new(DAE::Exp::BCONST { bool: false })
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, _) => {
            e1.clone()
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } }, _) => {
            Arc::new(DAE::Exp::LBINARY { exp1: e1.clone(), operator: op, exp2: e2.clone() })
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: rest }, _) => {
            let mut res: Arc<DAE::Exp>;
            res = makeLBinary(rest.clone(), op.clone())?;
            res = Arc::new(DAE::Exp::LBINARY { exp1: e1.clone(), operator: op, exp2: res.clone() });
            res.clone()
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.makeLBinary failed for operator ")); __mm_s.push_str(&*ExpressionDump::lbinopSymbol(op)?); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn makeSum1(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut simplify: bool) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut e1: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut e2: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    outExp = 'mc: {
        let __mc_input = inExpLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } => {
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } => {
                    Ok(expAdd(e1.clone(), e2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(makeSumWork(inExpLst.clone(), simplify)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace((literal!("-Expression.makeSum1 failed, DAE.Exp lst:")).clone())?;
                        Debug::trace((ExpressionDump::printExpListStr(inExpLst.clone())?).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn makeSumWork(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut simplify: bool) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut tp: Type;
    let mut rest: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut eLst: Arc<DAE::Exp>;
    let mut op: DAE::Operator;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExpLst) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    eLst = __pa0.clone();
    rest = __pa1.clone();
    tp = r#typeof(eLst.clone())?;
    op = if (DAEUtil::expTypeArray(tp.clone())) {DAE::Operator::ADD_ARR { ty: tp }} else {DAE::Operator::ADD { ty: tp }};
    outExp = eLst;
    for mut elem in &*rest {
        let mut elem = elem.clone();
        outExp = if (isZero(elem.clone())?) {outExp.clone()} else if (isZero(outExp.clone())?) {elem.clone()} else if (simplify) {(ExpressionSimplify::simplify1(Arc::new(DAE::Exp::BINARY { exp1: outExp.clone(), operator: op.clone(), exp2: elem.clone() }))?).0} else {Arc::new(DAE::Exp::BINARY { exp1: outExp.clone(), operator: op.clone(), exp2: elem.clone() })};
    }
    Ok(outExp)
}

pub fn makeSum(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExpLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } => {
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } => {
                    let true = (isZero(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(e2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } => {
                    let true = (isZero(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } => {
                    let mut tp: Type;
                    let mut op: Operator;
                    let mut b: bool;
                    tp = r#typeof(e1.clone())?;
                    b = DAEUtil::expTypeArray(tp.clone());
                    op = if (b.clone()) {DAE::Operator::ADD_ARR { ty: tp.clone() }} else {DAE::Operator::ADD { ty: tp.clone() }};
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e1, tail: rest } => {
                    let mut e2: Arc<DAE::Exp>;
                    let mut res: Arc<DAE::Exp>;
                    let mut b1: bool;
                    let mut tp: Type;
                    let mut op: Operator;
                    let mut b: bool;
                    b1 = isZero(e1.clone())?;
                    e2 = makeSum(rest.clone())?;
                    tp = r#typeof(e2.clone())?;
                    b = DAEUtil::expTypeArray(tp.clone());
                    op = if (b.clone()) {DAE::Operator::ADD_ARR { ty: tp.clone() }} else {DAE::Operator::ADD { ty: tp.clone() }};
                    res = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() });
                    res = if (b1.clone()) {e2.clone()} else {res.clone()};
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                lst => {
                    let mut explst: Arc<metamodelica::List<ArcStr>>;
                    let mut r#str: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-Expression.makeSum failed, DAE.Exp lst:")).clone())?;
                    explst = List::map(lst.clone(), (std::sync::Arc::new(printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))?;
                    r#str = stringDelimitList(explst.clone(), (literal!(", ")).clone());
                    Debug::traceln((r#str.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

pub fn expMul(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (e1.clone(), e2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (isZero(e1.clone())?) else { bail!("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (isZero(e2.clone())?) else { bail!("pattern mismatch") };
                    Ok(e2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: __rlit_1 }, _) => {
                    if !(__rlit_1.eq(&metamodelica::OrderedFloat((1.0) as f64))) { bail!("guard") }
                    Ok(e2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::RCONST { real: __rlit_2 }) => {
                    if !(__rlit_2.eq(&metamodelica::OrderedFloat((1.0) as f64))) { bail!("guard") }
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: 1 }, _) => {
                    Ok(e2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::ICONST { integer: 1 }) => {
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::RCONST { real: r2 }) => {
                    let mut r1 = (*r1).clone();
                    r1 = (r1.clone()) * (r2.clone());
                    Ok(Arc::new(DAE::Exp::RCONST { real: r1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: i1 }, Deref @ DAE::Exp::ICONST { integer: i2 }) => {
                    let mut i1 = (*i1).clone();
                    i1 = intMul(i1.clone(), i2.clone());
                    Ok(Arc::new(DAE::Exp::ICONST { integer: i1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tp: Type;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut op: Operator;
                    let mut e1_1: Arc<DAE::Exp>;
                    let mut e2_1: Arc<DAE::Exp>;
                    tp = r#typeof(e1.clone())?;
                    let true = (Types::isIntegerOrRealOrSubTypeOfEither(tp.clone())) else { bail!("pattern mismatch") };
                    b1 = DAEUtil::expTypeArray(tp.clone());
                    tp = r#typeof(e2.clone())?;
                    let true = (Types::isIntegerOrRealOrSubTypeOfEither(tp.clone())) else { bail!("pattern mismatch") };
                    b2 = DAEUtil::expTypeArray(tp.clone());
                    (e1_1, e2_1) = Util::swap(!(b1.clone()) && b2.clone(), e1.clone(), e2.clone());
                    op = if (b1.clone() && b2.clone()) {DAE::Operator::MUL_ARR { ty: tp.clone() }} else {if (b1.clone() == b2.clone()) {DAE::Operator::MUL { ty: tp.clone() }} else {DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }}};
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

pub fn expPow(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((e1.clone(), e2.clone())) {
        (_, _) if (isOne(e2.clone())) => {
            return Ok(e1.clone())
        },
        (_, _) if (isZero(e2.clone())?) => {
            return Ok(makeConstOne(r#typeof(e1.clone())?))
        },
        (_, _) if (isConstOne(e1.clone())) => {
            return Ok(e1.clone())
        },
        (_, _) if (isZero(e1.clone())? && isPositive(e2.clone())?) => {
            return Ok(makeConstZero(r#typeof(e1.clone())?))
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e }, _) if (isEven(e2.clone())) => {
            { (e1, e2) = (e.clone(), e2.clone()); continue '__tco; }
        },
        (Deref @ DAE::Exp::BINARY { exp1: e3, operator: DAE::Operator::DIV { .. }, exp2: e4 }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e5 }) => {
            let mut e: Arc<DAE::Exp>;
            e = makeDiv(e4.clone(), e3.clone())?;
            { (e1, e2) = (e.clone(), e5.clone()); continue '__tco; }
        },
        (Deref @ DAE::Exp::BINARY { exp1: e3, operator: DAE::Operator::DIV { .. }, exp2: e4 }, _) if (isNegativeOrZero(e2.clone())?) => {
            { (e1, e2) = (makeDiv(e4.clone(), e3.clone())?, negate(e2.clone())?); continue '__tco; }
        },
        (_, _) if (isHalf(e2.clone()) && isPositiveOrZero(e1.clone())?) => {
            return Ok(makePureBuiltinCall((literal!("sqrt")).clone(), list![e1.clone()], DAE::T_REAL_DEFAULT().clone()))
        },
        _ => {
            let mut tp: Type;
            let mut b: bool;
            let mut op: Operator;
            tp = r#typeof(e1.clone())?;
            b = DAEUtil::expTypeArray(tp.clone());
            op = if (b.clone()) {DAE::Operator::POW_ARR { ty: tp.clone() }} else {DAE::Operator::POW { ty: tp.clone() }};
            return Ok(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() }))
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn expPowLst(mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut n: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExp: Arc<metamodelica::List<Arc<DAE::Exp>>> = List::map1(expLst.clone(), (std::sync::Arc::new(expPow) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), n.clone())?;
    Ok(outExp)
}

pub fn expMaxScalar(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut tp: Type;
    tp = r#typeof(e1.clone())?;
    outExp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("max")).clone() }), expLst: list![e1, e2], attr: Arc::new(DAE::CallAttributes { ty: tp, tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) });
    Ok(outExp)
}

pub fn expOptMaxScalar(mut e1: Option<Arc<DAE::Exp>>, mut e2: Option<Arc<DAE::Exp>>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outExp: Option<Arc<DAE::Exp>>;
    outExp = (::match_deref::match_deref! { match &((e1.clone(), e2.clone())) {
        (_, None) => {
            e1
        },
        (None, _) => {
            e2
        },
        (Some(e11), Some(e22)) => {
            Some(expMaxScalar(e11.clone(), e22.clone())?)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn expMinScalar(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut tp: Type;
    tp = r#typeof(e1.clone())?;
    outExp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("min")).clone() }), expLst: list![e1, e2], attr: Arc::new(DAE::CallAttributes { ty: tp, tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) });
    Ok(outExp)
}

pub fn expOptMinScalar(mut e1: Option<Arc<DAE::Exp>>, mut e2: Option<Arc<DAE::Exp>>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outExp: Option<Arc<DAE::Exp>>;
    outExp = (::match_deref::match_deref! { match &((e1.clone(), e2.clone())) {
        (_, None) => {
            e1
        },
        (None, _) => {
            e2
        },
        (Some(e11), Some(e22)) => {
            Some(expMinScalar(e11.clone(), e22.clone())?)
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub(crate) fn makeProductVector(mut e1: Arc<DAE::Exp>, mut v: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    res = List::map1(v, (std::sync::Arc::new(makeProduct) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e1)?;
    Ok(res)
}

pub fn makeScalarProduct(mut v: metamodelica::Array<Arc<DAE::Exp>>, mut w: metamodelica::Array<Arc<DAE::Exp>>) -> Result<Arc<DAE::Exp>> {
    let mut s: Arc<DAE::Exp> = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) });
    let mut size1: i32 = metamodelica::arrayLength(v.clone());
    let mut size2: i32 = metamodelica::arrayLength(w.clone());
    if size1 != size2 {
        metamodelica::print((literal!("makeScalarProduct faili.\n")).clone());
        return Ok(s.clone());
    }
    s = makeSum1(({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut i in (1..=size1).into_iter() {
            let __x = expMul(metamodelica::arrayGet(v.clone(), i.clone())?, metamodelica::arrayGet(w.clone(), i.clone())?)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), false)?;
    (s, _) = ExpressionSimplify::simplify(s)?;
    Ok(s)
}

pub fn lenVec(mut v: metamodelica::Array<Arc<DAE::Exp>>) -> Result<Arc<DAE::Exp>> {
    let mut len: Arc<DAE::Exp> = makeScalarProduct(v.clone(), v.clone())?;
    len = makePureBuiltinCall((literal!("sqrt")).clone(), list![len], DAE::T_REAL_DEFAULT().clone());
    Ok(len)
}

pub fn subVec(mut v: metamodelica::Array<Arc<DAE::Exp>>, mut w: metamodelica::Array<Arc<DAE::Exp>>) -> Result<metamodelica::Array<Arc<DAE::Exp>>> {
    let mut y: metamodelica::Array<Arc<DAE::Exp>>;
    let mut size1: i32 = metamodelica::arrayLength(v.clone());
    let mut size2: i32 = metamodelica::arrayLength(w.clone());
    if size1 != size2 {
        metamodelica::print((literal!("subVec fail.\n")).clone());
        bail!("fail");
    }
    y = arrayCreate(size1, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }));
    for mut i in 1..=size1 {
        metamodelica::arrayUpdate(y.clone(), i.clone(), expSub(metamodelica::arrayGet(v.clone(), i.clone())?, metamodelica::arrayGet(w.clone(), i.clone())?)?)?;
    }
    Ok(y)
}

pub(crate) fn makeProduct(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut product: Arc<DAE::Exp>;
    product = makeProductLst(list![e1, e2])?;
    Ok(product)
}

pub fn makeProductLst(mut inExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExpLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } => {
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e, tail: es } => {
                    let mut res: Arc<DAE::Exp>;
                    let true = (isConstOne(e.clone())) else { bail!("pattern mismatch") };
                    res = makeProductLst(es.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BINARY { operator: DAE::Operator::DIV { .. }, exp2: e, .. }, tail: _ } => {
                    let true = (isZero(e.clone())?) else { bail!("pattern mismatch") };
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BINARY { operator: DAE::Operator::DIV { .. }, exp2: e, .. }, tail: Deref @ metamodelica::List::Nil } } => {
                    let true = (isZero(e.clone())?) else { bail!("pattern mismatch") };
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e, tail: _ } => {
                    let true = (isZero(e.clone())?) else { bail!("pattern mismatch") };
                    Ok(e.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: tp }, exp2: e }, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } => {
                    let true = (isConstOne(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: e.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: tp }, exp2: e }, tail: Deref @ metamodelica::List::Nil } } => {
                    let true = (isConstOne(e1.clone())) else { bail!("pattern mismatch") };
                    Ok(Arc::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: e.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: tp }, exp2: e }, tail: es } => {
                    let mut res: Arc<DAE::Exp>;
                    let mut p1: Arc<DAE::Exp>;
                    let mut b_isZero: bool;
                    let true = (isConstOne(e1.clone())) else { bail!("pattern mismatch") };
                    p1 = makeProductLst(es.clone())?;
                    res = Arc::new(DAE::Exp::BINARY { exp1: p1.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: e.clone() });
                    b_isZero = isZero(p1.clone())?;
                    res = if (b_isZero.clone()) {makeConstZero(r#typeof(e.clone())?)} else {res.clone()};
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } => {
                    let true = (isConstOne(e2.clone())) else { bail!("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } => {
                    let mut res: Arc<DAE::Exp>;
                    let mut tp: Type;
                    let mut b_isZero: bool;
                    let mut b1: bool;
                    let mut b2: bool;
                    b1 = isZero(e1.clone())?;
                    b2 = isZero(e2.clone())?;
                    b_isZero = boolOr(b1.clone(), b2.clone());
                    tp = r#typeof(e1.clone())?;
                    tp = checkIfOther(tp.clone());
                    res = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() });
                    res = if (b_isZero.clone()) {makeConstZero(tp.clone())} else {res.clone()};
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e1, tail: rest } => {
                    let mut res: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let mut tp: Type;
                    let mut b_isZero: bool;
                    let mut b1: bool;
                    let mut b2: bool;
                    e2 = makeProductLst(rest.clone())?;
                    tp = r#typeof(e1.clone())?;
                    tp = checkIfOther(tp.clone());
                    res = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() });
                    b1 = isZero(e1.clone())?;
                    b2 = isZero(e2.clone())?;
                    b_isZero = boolOr(b1.clone(), b2.clone());
                    res = if (b_isZero.clone()) {makeConstZero(r#typeof(e1.clone())?)} else {res.clone()};
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                lst => {
                    let mut explst: Arc<metamodelica::List<ArcStr>>;
                    let mut r#str: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("-Expression.makeProductLst failed, DAE.Exp lst:")).clone())?;
                    explst = List::map(lst.clone(), (std::sync::Arc::new(printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<ArcStr> + 'static>))?;
                    r#str = stringDelimitList(explst.clone(), (literal!(", ")).clone());
                    Debug::traceln((r#str.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn checkIfOther(mut inTp: Arc<DAE::Type>) -> Arc<DAE::Type> {
    let mut outTp: Arc<DAE::Type>;
    outTp = (::match_deref::match_deref! { match &(inTp.clone()) {
        Deref @ DAE::Type::T_UNKNOWN { .. } => DAE::T_REAL_DEFAULT().clone(),
        _ => inTp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTp
}

pub fn expDiv(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut tp: Type;
    let mut b: bool;
    let mut op: Operator;
    tp = r#typeof(e1.clone())?;
    let true = (Types::isIntegerOrRealOrSubTypeOfEither(tp.clone())) else { bail!("pattern mismatch") };
    b = DAEUtil::expTypeArray(tp.clone());
    op = if (b) {DAE::Operator::DIV_ARR { ty: tp }} else {DAE::Operator::DIV { ty: tp }};
    outExp = Arc::new(DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 });
    Ok(outExp)
}

pub fn makeDiv(mut e1: Arc<DAE::Exp>, mut e2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut res: Arc<DAE::Exp>;
    res = (::match_deref::match_deref! { match &(e2.clone()) {
        _ if (isZero(e1.clone())? && !(isZero(e2.clone())?)) => e1.clone(),
        _ if (isOne(e2.clone())) => e1.clone(),
        _ => expDiv(e1.clone(), e2.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn makeDivVector(mut v: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut e1: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut res: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    res = List::map1(v, (std::sync::Arc::new(makeDiv) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e1)?;
    Ok(res)
}

pub fn makeAsubAddIndex(mut e: Arc<DAE::Exp>, mut indx: i32) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp> = e.clone();
    outExp = (::match_deref::match_deref! { match &(outExp.clone()) {
        Deref @ DAE::Exp::ASUB { .. } => {
            assign_variant_field!(outExp => DAE::Exp::ASUB; sub = listAppend(var_field!((*outExp).sub, DAE::Exp::ASUB).clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: indx }) })]));
            outExp
        },
        _ => makeASUB(e, list![Arc::new(DAE::Exp::ICONST { integer: indx })])?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn makeIntegerExp(mut i: i32) -> Arc<DAE::Exp> {
    let mut e: Arc<DAE::Exp>;
    e = Arc::new(DAE::Exp::ICONST { integer: i });
    e
}

pub fn makeRealExp(mut r: metamodelica::Real) -> Arc<DAE::Exp> {
    let mut e: Arc<DAE::Exp>;
    e = Arc::new(DAE::Exp::RCONST { real: r });
    e
}

pub(crate) fn makeBoolExp(mut b: bool) -> Arc<DAE::Exp> {
    let mut e: Arc<DAE::Exp>;
    e = Arc::new(DAE::Exp::BCONST { bool: b });
    e
}

pub fn makeConstOne(mut inType: Arc<DAE::Type>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inType) {
        Deref @ DAE::Type::T_INTEGER { .. } => Arc::new(DAE::Exp::ICONST { integer: 1 }),
        Deref @ DAE::Type::T_REAL { .. } => Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }),
        _ => Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub fn makeConstZero(mut inType: Arc<DAE::Type>) -> Arc<DAE::Exp> {
    let mut r#const: Arc<DAE::Exp>;
    r#const = (::match_deref::match_deref! { match &(inType) {
        Deref @ DAE::Type::T_REAL { .. } => Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }),
        Deref @ DAE::Type::T_INTEGER { .. } => Arc::new(DAE::Exp::ICONST { integer: 0 }),
        _ => Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#const
}

pub(crate) fn makeConstNumber(mut ty: Arc<DAE::Type>, mut n: i32) -> Arc<DAE::Exp> {
    let mut exp: Arc<DAE::Exp>;
    exp = (::match_deref::match_deref! { match &(ty) {
        Deref @ DAE::Type::T_INTEGER { .. } => Arc::new(DAE::Exp::ICONST { integer: n }),
        _ => Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((n) as f64) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn makeConstZeroE(mut iExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut r#const: Arc<DAE::Exp>;
    let mut tp: Arc<DAE::Type> = r#typeof(iExp.clone())?;
    r#const = makeConstZero(tp);
    Ok(r#const)
}

pub(crate) fn makeListOfZeros(mut inDimension: i32) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut outList: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    if inDimension > 0 {
        for mut i in 1..=inDimension {
            outList = metamodelica::cons(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), outList.clone());
        }
    }
    outList
}

pub fn makeRealArrayOfZeros(mut inDimension: i32) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    let mut l: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    l = makeListOfZeros(inDimension);
    outExp = makeRealArray(l);
    outExp
}

pub fn createZeroExpression(mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inType.clone()) {
        _ if (isIntegerOrReal(inType.clone())) => {
            makeConstZero(inType.clone())
        },
        Deref @ DAE::Type::T_TUPLE { types: typeLst, .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            expLst = List::map(typeLst.clone(), (std::sync::Arc::new(createZeroExpression) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>))?;
            e = Arc::new(DAE::Exp::TUPLE { PR: expLst.clone() });
            e.clone()
        },
        Deref @ DAE::Type::T_ARRAY { dims, .. } => {
            let mut e: Arc<DAE::Exp>;
            (e, _) = makeZeroExpression(dims.clone())?;
            e.clone()
        },
        Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path }, .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut typeLst: Arc<metamodelica::List<Arc<DAE::Type>>>;
            let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut varNames: Arc<metamodelica::List<ArcStr>>;
            typeLst = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
        for mut v in (varLst.clone()).into_iter().cloned() {
            let __x = v.ty.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            expLst = List::map(typeLst.clone(), (std::sync::Arc::new(createZeroExpression) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> + 'static>))?;
            varNames = List::map(varLst.clone(), (std::sync::Arc::new(varName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>))?;
            let true = ((varNames.clone().len() as i32) == (expLst.clone().len() as i32)) else { bail!("pattern mismatch") };
            e = Arc::new(DAE::Exp::RECORD { path: path.clone(), exps: expLst.clone(), comp: varNames.clone(), ty: inType.clone() });
            e.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn makeZeroExpression(mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Type>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outType: Arc<DAE::Type>;
    (outExp, outType) = (::match_deref::match_deref! { match &(inDims) {
        Deref @ metamodelica::List::Nil => {
            (Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), DAE::T_REAL_DEFAULT().clone())
        },
        Deref @ metamodelica::List::Cons { head: d, tail: dims } => {
            let mut i: i32;
            let mut e: Arc<DAE::Exp>;
            let mut eLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut ty: Arc<DAE::Type>;
            let mut scalar: bool;
            i = dimensionSize(d.clone())?;
            (e, ty) = makeZeroExpression(dims.clone())?;
            eLst = List::fill(e.clone(), i.clone());
            scalar = dims.clone().is_empty();
            (Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: metamodelica::cons(d.clone(), dims.clone()) }), scalar: scalar.clone(), array: eLst.clone() }), Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![d.clone()] }))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outType))
}

pub fn makeOneExpression(mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Type>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outType: Arc<DAE::Type>;
    (outExp, outType) = (::match_deref::match_deref! { match &(inDims) {
        Deref @ metamodelica::List::Nil => {
            (Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), DAE::T_REAL_DEFAULT().clone())
        },
        Deref @ metamodelica::List::Cons { head: d, tail: dims } => {
            let mut i: i32;
            let mut e: Arc<DAE::Exp>;
            let mut eLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut ty: Arc<DAE::Type>;
            let mut scalar: bool;
            i = dimensionSize(d.clone())?;
            (e, ty) = makeOneExpression(dims.clone())?;
            eLst = List::fill(e.clone(), i.clone());
            scalar = dims.clone().is_empty();
            (Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: metamodelica::cons(d.clone(), dims.clone()) }), scalar: scalar.clone(), array: eLst.clone() }), Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![d.clone()] }))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outType))
}

pub fn listToArray(mut inList: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<DAE::Exp>> {
    let mut oExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    oExp = 'mc: {
        let __mc_input = (inList.clone(), dims.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Expression.listToArray called with empty dimension list.")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Expression.listToArray called with empty list.")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: exp, tail: _ }, _) => {
                    let mut ty: Arc<DAE::Type>;
                    let mut oExp: Arc<DAE::Exp> = oExp.clone();
                    ty = r#typeof(exp.clone())?;
                    oExp = listToArray2(inList.clone(), dims.clone(), ty.clone())?;
                    Ok((oExp.clone(), oExp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { oExp = __wb0; break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oExp)
}

fn listToArray2(mut inList: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut iDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut oExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let () = (::match_deref::match_deref! { match &(iDims.clone()) {
        Deref @ metamodelica::List::Cons { head: d, tail: Deref @ metamodelica::List::Nil } => {
            let mut i: i32;
            let mut is_scalar: bool;
            let mut ty: Arc<DAE::Type>;
            is_scalar = !(Types::isArray(inType.clone()));
            if dimensionKnown(d.clone()) {
                i = dimensionSize(d.clone())?;
                if i.clone() != (inList.clone().len() as i32) {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Expression.listToArray2: Number of elements in the list does not match the dimension size.")).clone()])?;
                    bail!("fail");
                } else {
                    ty = liftArrayR(inType, Arc::new(DAE::Dimension::DIM_INTEGER { integer: (inList.clone().len() as i32) }));
                    oExp = Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: is_scalar.clone(), array: inList });
                }
            } else {
                ty = liftArrayR(inType, d.clone());
                oExp = Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: is_scalar.clone(), array: inList });
            }
            ()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: _ } => {
            let mut d: Arc<DAE::Dimension>;
            let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
            let mut ty: Arc<DAE::Type>;
            (d, dims) = List::splitLast(iDims)?;
            explst = listToArray3(inList, d.clone())?;
            ty = liftArrayR(inType, Arc::new(DAE::Dimension::DIM_INTEGER { integer: (explst.clone().len() as i32) }));
            oExp = listToArray2(explst.clone(), dims.clone(), ty.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(oExp)
}

fn listToArray3(mut inList: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut iDim: Arc<DAE::Dimension>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut oExps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    oExps = (::match_deref::match_deref! { match &((inList.clone(), iDim)) {
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        (_, d) => {
            let mut i: i32;
            let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut restexps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut restarr: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut arrexp: Arc<DAE::Exp>;
            i = dimensionSize(d.clone())?;
            if i.clone() > (inList.clone().len() as i32) {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Expression.listToArray3: Not enough elements left in list to fit dimension.")).clone()])?;
                bail!("fail");
            } else {
                (explst, restexps) = List::split(inList, i.clone())?;
                arrexp = makeArrayFromList(explst.clone())?;
                restarr = listToArray3(restexps.clone(), d.clone())?;
            }
            metamodelica::cons(arrexp.clone(), restarr.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oExps)
}

pub fn arrayFill(mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut oExp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    oExp = (::match_deref::match_deref! { match &(dims.clone()) {
        Deref @ metamodelica::List::Nil => inExp,
        _ => {
            oExp = arrayFill2(dims, inExp)?;
            oExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oExp)
}

fn arrayFill2(mut iDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(iDims) {
        Deref @ metamodelica::List::Cons { head: d, tail: Deref @ metamodelica::List::Nil } => {
            let mut i: i32;
            let mut ty: Type;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            ty = r#typeof(inExp.clone())?;
            i = dimensionSize(d.clone())?;
            expl = List::fill(inExp, i.clone());
            return Ok(Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: i.clone() })] }), scalar: true, array: expl.clone() }))
        },
        Deref @ metamodelica::List::Cons { head: d, tail: dims } => {
            let mut arrexp: Arc<DAE::Exp>;
            arrexp = arrayFill2(list![d.clone()], inExp)?;
            { (iDims, inExp) = (dims.clone(), arrexp.clone()); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn makeIndexSubscript(mut exp: Arc<DAE::Exp>) -> Arc<DAE::Subscript> {
    let mut subscript: Arc<DAE::Subscript>;
    subscript = Arc::new(DAE::Subscript::INDEX { exp: exp });
    subscript
}

pub fn makeVar(mut name: ArcStr, mut tp: Arc<DAE::Type>) -> Arc<DAE::Var> {
    let mut v: Arc<DAE::Var>;
    v = Arc::new(DAE::Var { name: (name).clone(), attributes: DAE::dummyAttrVar().clone(), ty: tp, binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None });
    v
}

pub fn dimensionsAdd(mut dim1: Arc<DAE::Dimension>, mut dim2: Arc<DAE::Dimension>) -> Arc<DAE::Dimension> {
    let mut res: Arc<DAE::Dimension>;
    match '__try0: {
        res = intDimension(unwrap_break_err!(dimensionSize(dim1.clone()), '__try0) + unwrap_break_err!(dimensionSize(dim2.clone()), '__try0));
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN();
        }
    }
    res
}

pub fn concatArrayType(mut arrayType1: Arc<DAE::Type>, mut arrayType2: Arc<DAE::Type>) -> Result<Arc<DAE::Type>> {
    let mut concatType: Arc<DAE::Type>;
    concatType = (::match_deref::match_deref! { match &((arrayType1, arrayType2)) {
        (Deref @ DAE::Type::T_ARRAY { ty: et, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: dims1 } }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: dim2, tail: _ }, .. }) => {
            let mut dim1 = (*dim1).clone();
            dim1 = dimensionsAdd(dim1.clone(), dim2.clone());
            Arc::new(DAE::Type::T_ARRAY { ty: et.clone(), dims: metamodelica::cons(dim1.clone(), dims1.clone()) })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(concatType)
}

pub fn replaceExpTpl(mut inExp: Arc<DAE::Exp>, mut tpl: (Arc<DAE::Exp>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::Exp>, Arc<DAE::Exp>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<DAE::Exp>, Arc<DAE::Exp>);
    (outExp, outTpl) = (::match_deref::match_deref! { match &((inExp, tpl.clone())) {
        (e, (s, t)) => {
            let mut e1: Arc<DAE::Exp>;
            (e1, _) = replaceExp(e.clone(), s.clone(), t.clone())?;
            (e1.clone(), tpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTpl))
}

pub fn replaceExp(mut inExp: Arc<DAE::Exp>, mut inSourceExp: Arc<DAE::Exp>, mut inTargetExp: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, i32)> {
    let mut exp: Arc<DAE::Exp>;
    let mut i: i32;
    let (__pa0, (_, _, __pa1)) = traverseExpTopDown(inExp, (std::sync::Arc::new(replaceExpWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::Exp>, Arc<DAE::Exp>, i32)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::Exp>, Arc<DAE::Exp>, i32))> + 'static>), (inSourceExp, inTargetExp, 0))?;
    exp = __pa0.clone();
    i = __pa1.clone();
    Ok((exp, i))
}

fn replaceExpWork(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<DAE::Exp>, Arc<DAE::Exp>, i32)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::Exp>, Arc<DAE::Exp>, i32))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut otpl: (Arc<DAE::Exp>, Arc<DAE::Exp>, i32);
    (outExp, cont, otpl) = (::match_deref::match_deref! { match &(inTpl.clone()) {
        (source, target, c) if (ExpressionBasics::expEqual(inExp.clone(), source.clone())?) => {
            (target.clone(), false, (source.clone(), target.clone(), c.clone() + 1))
        },
        _ => {
            (inExp.clone(), true, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, otpl))
}

pub(crate) fn expressionCollector(mut exp: Arc<DAE::Exp>, mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> (Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Exp>>>) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outExps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    outExp = exp.clone();
    outExps = metamodelica::cons(exp, acc);
    (outExp, outExps)
}

pub fn replaceCrefBottomUp(mut inExp: Arc<DAE::Exp>, mut inSourceExp: Arc<DAE::ComponentRef>, mut inTargetExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    (exp, _) = traverseExpBottomUp(inExp, (std::sync::Arc::new(replaceCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, Arc<DAE::Exp>))> + 'static>), (inSourceExp, inTargetExp))?;
    Ok(exp)
}

pub fn replaceCref(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<DAE::ComponentRef>, Arc<DAE::Exp>)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, Arc<DAE::Exp>))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut otpl: (Arc<DAE::ComponentRef>, Arc<DAE::Exp>);
    (outExp, otpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (cr1, target)) if (ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cr1.clone())?) => {
            (target.clone(), inTpl)
        },
        _ => {
            (inExp, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, otpl))
}

pub fn containsInitialCall(mut condition: Arc<DAE::Exp>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(condition) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => {
            true
        },
        Deref @ DAE::Exp::ARRAY { array, .. } => {
            List::any(array.clone(), (std::sync::Arc::new(containsInitialCall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

/* **************************************************/
/* traverse DAE.Exp */
/* **************************************************/
pub fn traverseExpBottomUp<T: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inExp: Arc<DAE::Exp>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>, mut inExtArg: T) -> Result<(Arc<DAE::Exp>, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, T) -> Result<(Arc<DAE::Exp>, T)> + 'static>;

    let mut outExp: Arc<DAE::Exp>;
    let mut outExtArg: T;
    (outExp, outExtArg) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::EMPTY { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::ICONST { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::SCONST { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::CLKCONST { clk } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            let mut clk1: Arc<DAE::ClockKind>;
            (clk1, ext_arg) = traverseExpClk(clk.clone(), inFunc.clone(), inExtArg)?;
            e = if (referenceEq(&*(clk1.clone()),&*(clk.clone()))) {inExp} else {Arc::new(DAE::Exp::CLKCONST { clk: clk1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            let mut cr_1: Arc<DAE::ComponentRef>;
            (cr_1, ext_arg) = traverseExpCref(cr.clone(), inFunc.clone(), inExtArg)?;
            e = if (referenceEq(&*(cr.clone()),&*(cr_1.clone()))) {inExp} else {Arc::new(DAE::Exp::CREF { componentRef: cr_1.clone(), ty: tp.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::UNARY { operator: op, exp: e1 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e1_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc.clone(), ext_arg.clone())?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc.clone(), ext_arg.clone())?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::LBINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, index: index_, optionExpisASUB: isExpisASUB } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc.clone(), ext_arg.clone())?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::RELATION { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone(), index: index_.clone(), optionExpisASUB: isExpisASUB.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut e3_1: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc.clone(), ext_arg.clone())?;
            (e3_1, ext_arg) = traverseExpBottomUp(e3.clone(), inFunc.clone(), ext_arg.clone())?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone())) && referenceEq(&*(e3.clone()),&*(e3_1.clone()))) {inExp} else {Arc::new(DAE::Exp::IFEXP { expCond: e1_1.clone(), expThen: e2_1.clone(), expElse: e3_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::CALL { path: r#fn, expLst: expl, attr } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc.clone(), inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::CALL { path: r#fn.clone(), expLst: expl_1.clone(), attr: attr.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::RECORD { path: r#fn, exps: expl, comp: fieldNames, ty: tp } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc.clone(), inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::RECORD { path: r#fn.clone(), exps: expl_1.clone(), comp: fieldNames.clone(), ty: tp.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { path: r#fn, expList: expl, ty: tp, origType: t } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc.clone(), inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::PARTEVALFUNCTION { path: r#fn.clone(), expList: expl_1.clone(), ty: tp.clone(), origType: t.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::ARRAY { ty: tp, scalar, array: expl } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc.clone(), inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: scalar.clone(), array: expl_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::MATRIX { ty: tp, integer: dim, matrix: lstexpl } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            let mut lstexpl_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
            (lstexpl_1, ext_arg) = traverseExpMatrix(lstexpl.clone(), inFunc.clone(), inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&*(lstexpl.clone()), &*(lstexpl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::MATRIX { ty: tp.clone(), integer: dim.clone(), matrix: lstexpl_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::RANGE { ty: tp, start: e1, step: None, stop: e2 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc.clone(), ext_arg.clone())?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::RANGE { ty: tp.clone(), start: e1_1.clone(), step: None, stop: e2_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::RANGE { ty: tp, start: e1, step: Some(e2), stop: e3 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut e3_1: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc.clone(), ext_arg.clone())?;
            (e3_1, ext_arg) = traverseExpBottomUp(e3.clone(), inFunc.clone(), ext_arg.clone())?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone())) && referenceEq(&*(e3.clone()),&*(e3_1.clone()))) {inExp} else {Arc::new(DAE::Exp::RANGE { ty: tp.clone(), start: e1_1.clone(), step: Some(e2_1.clone()), stop: e3_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::TUPLE { PR: expl } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc.clone(), inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::TUPLE { PR: expl_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::CAST { ty: tp, exp: e1 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::CAST { ty: tp.clone(), exp: e1_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::ASUB { exp: e1, sub: subs } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = getSubscriptExp(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc.clone(), ext_arg.clone())?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {makeASUB(e1_1.clone(), expl_1.clone())?};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::TSUB { exp: e1, ix: i, ty: tp } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::TSUB { exp: e1_1.clone(), ix: i.clone(), ty: tp.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        e1 @ Deref @ DAE::Exp::RSUB { .. } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut ext_arg: T;
            let mut e1 = (*e1).clone();
            (e1_1, ext_arg) = traverseExpBottomUp(var_field!((*e1).exp, DAE::Exp::RSUB).clone(), inFunc.clone(), inExtArg)?;
            if !(referenceEq(&*(var_field!((*e1).exp, DAE::Exp::RSUB).clone()),&*(e1_1.clone()))) {
                assign_variant_field!(e1 => DAE::Exp::RSUB; exp = e1_1.clone());
            }
            (e1, ext_arg) = inFunc(e1.clone(), ext_arg.clone())?;
            (e1.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::SIZE { exp: e1, sz: None } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::SIZE { exp: e1_1.clone(), sz: None })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::SIZE { exp: e1, sz: Some(e2) } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc.clone(), ext_arg.clone())?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::SIZE { exp: e1_1.clone(), sz: Some(e2_1.clone()) })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::REDUCTION { reductionInfo, expr: e1, iterators: riters } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            let mut riters_1: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            (riters_1, ext_arg) = traverseReductionIterators(riters.clone(), inFunc.clone(), ext_arg.clone())?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && metamodelica::ReferenceEq::reference_eq(&*(riters.clone()), &*(riters_1.clone()))) {inExp} else {Arc::new(DAE::Exp::REDUCTION { reductionInfo: reductionInfo.clone(), expr: e1_1.clone(), iterators: riters_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::CONS { car: e1, cdr: e2 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc.clone(), ext_arg.clone())?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::CONS { car: e1_1.clone(), cdr: e2_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::LIST { valList: expl } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc.clone(), inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::LIST { valList: expl_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::META_TUPLE { listExp: expl } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc.clone(), inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::META_TUPLE { listExp: expl_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::META_OPTION { exp: None } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::META_OPTION { exp: Some(e1) } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::META_OPTION { exp: Some(e1_1.clone()) })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::BOX { exp: e1 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::BOX { exp: e1_1.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::UNBOX { exp: e1, ty: tp } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc.clone(), inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::UNBOX { exp: e1_1.clone(), ty: tp.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::METARECORDCALL { path: r#fn, args: expl, fieldNames, index: i, typeVars } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc.clone(), inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::METARECORDCALL { path: r#fn.clone(), args: expl_1.clone(), fieldNames: fieldNames.clone(), index: i.clone(), typeVars: typeVars.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { matchType: matchTy, inputs: expl, aliases, localDecls, cases, et: tp } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut cases_1: Arc<metamodelica::List<Arc<DAE::MatchCase>>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc.clone(), inExtArg)?;
            (cases_1, ext_arg) = traverseCases(cases.clone(), inFunc.clone(), ext_arg.clone())?;
            e = if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone())) && metamodelica::ReferenceEq::reference_eq(&*(cases.clone()), &*(cases_1.clone()))) {inExp} else {Arc::new(DAE::Exp::MATCHEXPRESSION { matchType: matchTy.clone(), inputs: expl_1.clone(), aliases: aliases.clone(), localDecls: localDecls.clone(), cases: cases_1.clone(), et: tp.clone() })};
            (e, ext_arg) = inFunc(e.clone(), ext_arg.clone())?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::PATTERN { .. } => {
            let mut e: Arc<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e.clone(), ext_arg.clone())
        },
        Deref @ DAE::Exp::CODE { .. } => {
            (inExp, inExtArg)
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = (printExpStr(inExp)?).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.traverseExpBottomUp or one of the user-defined functions using it is not implemented correctly: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
            Error::addInternalError((r#str.clone()).clone(), metamodelica::sourceInfo!("FrontEnd/Expression.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outExtArg))
}

pub fn traverseExpDummy(mut inExp: Arc<DAE::Exp>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>) -> Result<Arc<DAE::Exp>> {
    pub type FuncExpType = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>;

    let mut outExp: Arc<DAE::Exp>;
    (outExp, _) = traverseExpBottomUp(inExp, (std::sync::Arc::new(traverseExpDummyHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>) -> Result<(Arc<DAE::Exp>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>)> + 'static>), func.clone())?;
    Ok(outExp)
}

pub(crate) fn traverseExpDummyHelper(mut inExp: Arc<DAE::Exp>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>) -> Result<(Arc<DAE::Exp>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>)> {
    pub type FuncExpType = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>;

    let mut outExp: Arc<DAE::Exp>;
    let mut outFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>;
    outExp = func(inExp)?;
    outFunc = func.clone();
    Ok((outExp, outFunc))
}

pub fn traverseSubexpressionsHelper<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inExp: Arc<DAE::Exp>, mut itpl: (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, Type_a)) -> Result<(Arc<DAE::Exp>, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, Type_a))> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outExp: Arc<DAE::Exp>;
    let mut otpl: (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, Type_a);
    let mut rel: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;
    let mut ext_arg: Type_a;
    let mut ext_arg2: Type_a;
    (rel, ext_arg) = itpl.clone();
    (outExp, ext_arg2) = traverseExpBottomUp(inExp, rel.clone(), ext_arg.clone())?;
    otpl = if (metamodelica::ReferenceEq::reference_eq(&(ext_arg), &(ext_arg2.clone()))) {itpl} else {(rel.clone(), ext_arg2)};
    Ok((outExp, otpl))
}

pub fn traverseSubexpressions<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut e: Arc<DAE::Exp>, mut arg: Type_a, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>) -> Result<(Arc<DAE::Exp>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut e: Arc<DAE::Exp> = e;
    let mut arg: Type_a = arg;
    (e, arg) = traverseExpBottomUp(e, func.clone(), arg)?;
    Ok((e, arg))
}

pub fn traverseSubexpressionsDummyHelper(mut inExp: Arc<DAE::Exp>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>) -> Result<(Arc<DAE::Exp>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>)> {
    pub type FuncExpType = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>;

    let mut outExp: Arc<DAE::Exp>;
    let mut outFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>;
    (outExp, outFunc) = traverseExpBottomUp(inExp, (std::sync::Arc::new(traverseExpDummyHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>) -> Result<(Arc<DAE::Exp>, Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>)> + 'static>), inFunc.clone())?;
    Ok((outExp, outFunc))
}

pub(crate) fn traverseSubexpressionsTopDownHelper<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inExp: Arc<DAE::Exp>, mut itpl: (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>, Type_a)) -> Result<(Arc<DAE::Exp>, (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>, Type_a))> {
    pub type FuncExpType2<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>;

    let mut outExp: Arc<DAE::Exp>;
    let mut otpl: (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>, Type_a);
    let mut rel: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>;
    let mut ext_arg: Type_a;
    let mut ext_arg2: Type_a;
    (rel, ext_arg) = itpl.clone();
    (outExp, ext_arg2) = traverseExpTopDown(inExp, rel.clone(), ext_arg.clone())?;
    otpl = if (metamodelica::ReferenceEq::reference_eq(&(ext_arg), &(ext_arg2.clone()))) {itpl} else {(rel.clone(), ext_arg2)};
    Ok((outExp, otpl))
}

fn traverseExpMatrix<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut outTypeA: Type_a = inTypeA.clone();
    let mut row_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut same: bool = true;
    for mut row in &*inMatrix.clone() {
        let mut row = row.clone();
        (row_1, outTypeA) = traverseExpList(row.clone(), func.clone(), outTypeA.clone())?;
        same = if (metamodelica::ReferenceEq::reference_eq(&*(row.clone()), &*(row_1.clone()))) {same} else {false};
        outMatrix = metamodelica::cons(row_1.clone(), outMatrix.clone());
    }
    if same {
        outMatrix = inMatrix;
    } else {
        outMatrix = metamodelica::Dangerous::listReverseInPlace(outMatrix);
    }
    Ok((outMatrix, outTypeA))
}

pub fn traverseExpList<ArgT: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut rel: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut iext_arg: ArgT) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, ArgT)> {
    pub type FuncExpType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>;

    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut ext_arg: ArgT = iext_arg.clone();
    let mut e1: Arc<DAE::Exp>;
    let mut allEq: bool = true;
    let mut delst: DoubleEnded::MutableList<Arc<DAE::Exp>> = <DoubleEnded::MutableList<Arc<DAE::Exp>> as ::std::default::Default>::default();
    let mut nEq: i32 = 0;
    for mut e in &*inExpl.clone() {
        let mut e = e.clone();
        (e1, ext_arg) = traverseExpBottomUp(e.clone(), rel.clone(), ext_arg.clone())?;
        if if (allEq) {!(referenceEq(&*(e.clone()),&*(e1.clone())))} else {false} {
            allEq = false;
            delst = DoubleEnded::empty(e1.clone());
            for mut elt in &*inExpl.clone() {
                let mut elt = elt.clone();
                if nEq < 1 {
                    break;
                }
                DoubleEnded::push_back(delst.clone(), elt.clone())?;
                nEq = nEq - 1;
            }
        }
        if allEq {
            nEq = nEq + 1;
        } else {
            DoubleEnded::push_back(delst.clone(), e1.clone())?;
        }
    }
    expl = if (allEq) {inExpl} else {DoubleEnded::toListAndClear(delst, metamodelica::nil())?};
    Ok((expl, ext_arg))
}

pub fn traverseExpTopDown<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inExp: Arc<DAE::Exp>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>, mut ext_arg: Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>;

    let mut outExp: Arc<DAE::Exp>;
    let mut outArg: Type_a;
    let mut cont: bool;
    (outExp, cont, outArg) = func(inExp, ext_arg)?;
    (outExp, outArg) = traverseExpTopDown1(cont, outExp, func.clone(), outArg)?;
    Ok((outExp, outArg))
}

fn traverseExpClk<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inClk: Arc<DAE::ClockKind>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inArg: Type_a) -> Result<(Arc<DAE::ClockKind>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outClk: Arc<DAE::ClockKind>;
    let mut outArg: Type_a;
    (outClk, outArg) = (::match_deref::match_deref! { match &(inClk.clone()) {
        Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: e1, resolution: e2 } => {
            let mut ea: Arc<DAE::Exp>;
            let mut eb: Arc<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: Arc<DAE::ClockKind>;
            (ea, arg) = traverseExpBottomUp(e1.clone(), func.clone(), inArg.clone())?;
            (eb, arg) = traverseExpBottomUp(e2.clone(), func.clone(), inArg)?;
            clk = if (referenceEq(&*(ea.clone()),&*(e1.clone())) && referenceEq(&*(eb.clone()),&*(e2.clone()))) {inClk} else {Arc::new(DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: ea.clone(), resolution: eb.clone() })};
            (clk.clone(), arg.clone())
        },
        Deref @ DAE::ClockKind::REAL_CLOCK { interval: e } => {
            let mut e1: Arc<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: Arc<DAE::ClockKind>;
            (e1, arg) = traverseExpBottomUp(e.clone(), func.clone(), inArg)?;
            clk = if (referenceEq(&*(e1.clone()),&*(e.clone()))) {inClk} else {Arc::new(DAE::ClockKind::REAL_CLOCK { interval: e1.clone() })};
            (clk.clone(), arg.clone())
        },
        Deref @ DAE::ClockKind::EVENT_CLOCK { condition: e1, startInterval: e2 } => {
            let mut ea: Arc<DAE::Exp>;
            let mut eb: Arc<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: Arc<DAE::ClockKind>;
            (ea, arg) = traverseExpBottomUp(e1.clone(), func.clone(), inArg.clone())?;
            (eb, arg) = traverseExpBottomUp(e2.clone(), func.clone(), inArg)?;
            clk = if (referenceEq(&*(ea.clone()),&*(e1.clone())) && referenceEq(&*(eb.clone()),&*(e2.clone()))) {inClk} else {Arc::new(DAE::ClockKind::EVENT_CLOCK { condition: ea.clone(), startInterval: eb.clone() })};
            (clk.clone(), arg.clone())
        },
        Deref @ DAE::ClockKind::SOLVER_CLOCK { c: e1, solverMethod: e2 } => {
            let mut ea: Arc<DAE::Exp>;
            let mut eb: Arc<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: Arc<DAE::ClockKind>;
            (ea, arg) = traverseExpBottomUp(e1.clone(), func.clone(), inArg.clone())?;
            (eb, arg) = traverseExpBottomUp(e2.clone(), func.clone(), inArg)?;
            clk = if (referenceEq(&*(ea.clone()),&*(e1.clone())) && referenceEq(&*(eb.clone()),&*(e2.clone()))) {inClk} else {Arc::new(DAE::ClockKind::SOLVER_CLOCK { c: ea.clone(), solverMethod: eb.clone() })};
            (clk.clone(), arg.clone())
        },
        _ => {
            (inClk, inArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClk, outArg))
}

fn traverseExpTopDownClockHelper<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inClk: Arc<DAE::ClockKind>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>, mut inArg: Type_a) -> Result<(Arc<DAE::ClockKind>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>;

    let mut outClk: Arc<DAE::ClockKind>;
    let mut outArg: Type_a;
    (outClk, outArg) = (::match_deref::match_deref! { match &(inClk.clone()) {
        Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: e1, resolution: e2 } => {
            let mut ea: Arc<DAE::Exp>;
            let mut eb: Arc<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: Arc<DAE::ClockKind>;
            (ea, arg) = traverseExpTopDown(e1.clone(), func.clone(), inArg.clone())?;
            (eb, arg) = traverseExpTopDown(e2.clone(), func.clone(), inArg)?;
            clk = if (referenceEq(&*(ea.clone()),&*(e1.clone())) && referenceEq(&*(eb.clone()),&*(e2.clone()))) {inClk} else {Arc::new(DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: ea.clone(), resolution: eb.clone() })};
            (clk.clone(), arg.clone())
        },
        Deref @ DAE::ClockKind::REAL_CLOCK { interval: e } => {
            let mut e1: Arc<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: Arc<DAE::ClockKind>;
            (e1, arg) = traverseExpTopDown(e.clone(), func.clone(), inArg)?;
            clk = if (referenceEq(&*(e1.clone()),&*(e.clone()))) {inClk} else {Arc::new(DAE::ClockKind::REAL_CLOCK { interval: e1.clone() })};
            (clk.clone(), arg.clone())
        },
        Deref @ DAE::ClockKind::EVENT_CLOCK { condition: e1, startInterval: e2 } => {
            let mut ea: Arc<DAE::Exp>;
            let mut eb: Arc<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: Arc<DAE::ClockKind>;
            (ea, arg) = traverseExpTopDown(e1.clone(), func.clone(), inArg.clone())?;
            (eb, arg) = traverseExpTopDown(e2.clone(), func.clone(), inArg)?;
            clk = if (referenceEq(&*(ea.clone()),&*(e1.clone())) && referenceEq(&*(eb.clone()),&*(e2.clone()))) {inClk} else {Arc::new(DAE::ClockKind::EVENT_CLOCK { condition: ea.clone(), startInterval: eb.clone() })};
            (clk.clone(), arg.clone())
        },
        Deref @ DAE::ClockKind::SOLVER_CLOCK { c: e1, solverMethod: e2 } => {
            let mut ea: Arc<DAE::Exp>;
            let mut eb: Arc<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: Arc<DAE::ClockKind>;
            (ea, arg) = traverseExpTopDown(e1.clone(), func.clone(), inArg.clone())?;
            (eb, arg) = traverseExpTopDown(e2.clone(), func.clone(), inArg)?;
            clk = if (referenceEq(&*(ea.clone()),&*(e1.clone())) && referenceEq(&*(eb.clone()),&*(e2.clone()))) {inClk} else {Arc::new(DAE::ClockKind::SOLVER_CLOCK { c: ea.clone(), solverMethod: eb.clone() })};
            (clk.clone(), arg.clone())
        },
        _ => {
            (inClk, inArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClk, outArg))
}

fn traverseExpTopDown1<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut cont: bool, mut inExp: Arc<DAE::Exp>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>, mut inArg: Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>;

    let mut outExp: Arc<DAE::Exp>;
    let mut outArg: Type_a;
    (outExp, outArg) = (::match_deref::match_deref! { match &((cont, inExp.clone(), func.clone(), inArg.clone())) {
        (false, _, _, _) => {
            (inExp, inArg)
        },
        (_, Deref @ DAE::Exp::ICONST { integer: _ }, _, ext_arg) => {
            (inExp, ext_arg.clone())
        },
        (_, Deref @ DAE::Exp::RCONST { real: _ }, _, ext_arg) => {
            (inExp, ext_arg.clone())
        },
        (_, Deref @ DAE::Exp::SCONST { string: _ }, _, ext_arg) => {
            (inExp, ext_arg.clone())
        },
        (_, Deref @ DAE::Exp::BCONST { bool: _ }, _, ext_arg) => {
            (inExp, ext_arg.clone())
        },
        (_, Deref @ DAE::Exp::CLKCONST { clk }, _, ext_arg) => {
            let mut e: Arc<DAE::Exp>;
            let mut clk1: Arc<DAE::ClockKind>;
            let mut ext_arg = (*ext_arg).clone();
            (clk1, ext_arg) = traverseExpTopDownClockHelper(clk.clone(), func.clone(), ext_arg.clone())?;
            e = if (referenceEq(&*(clk1.clone()),&*(clk.clone()))) {inExp} else {Arc::new(DAE::Exp::CLKCONST { clk: clk1.clone() })};
            (e.clone(), ext_arg.clone())
        },
        (_, Deref @ DAE::Exp::ENUM_LITERAL { .. }, _, ext_arg) => {
            (inExp, ext_arg.clone())
        },
        (_, Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, rel, ext_arg) => {
            let mut ext_arg_1: Type_a;
            let mut cr_1: ComponentRef;
            (cr_1, ext_arg_1) = traverseExpTopDownCrefHelper(cr.clone(), rel.clone(), ext_arg.clone())?;
            (if (referenceEq(&*(cr.clone()),&*(cr_1.clone()))) {inExp} else {Arc::new(DAE::Exp::CREF { componentRef: cr_1.clone(), ty: tp.clone() })}, ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::UNARY { operator: op, exp: e1 }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e1_1.clone() })}, ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), rel.clone(), ext_arg_1.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() })}, ext_arg_2.clone())
        },
        (_, Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1_1.clone() })}, ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), rel.clone(), ext_arg_1.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::LBINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() })}, ext_arg_2.clone())
        },
        (_, Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, index: index_, optionExpisASUB: isExpisASUB }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), rel.clone(), ext_arg_1.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::RELATION { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone(), index: index_.clone(), optionExpisASUB: isExpisASUB.clone() })}, ext_arg_2.clone())
        },
        (_, Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut e3_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            let mut ext_arg_3: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), rel.clone(), ext_arg_1.clone())?;
            (e3_1, ext_arg_3) = traverseExpTopDown(e3.clone(), rel.clone(), ext_arg_2.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone())) && referenceEq(&*(e3.clone()),&*(e3_1.clone()))) {inExp} else {Arc::new(DAE::Exp::IFEXP { expCond: e1_1.clone(), expThen: e2_1.clone(), expElse: e3_1.clone() })}, ext_arg_3.clone())
        },
        (_, Deref @ DAE::Exp::CALL { path: r#fn, expLst: expl, attr }, rel, ext_arg) => {
            let mut ext_arg_1: Type_a;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::CALL { path: r#fn.clone(), expLst: expl_1.clone(), attr: attr.clone() }), ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::RECORD { path: r#fn, exps: expl, comp: fieldNames, ty: tp }, rel, ext_arg) => {
            let mut ext_arg_1: Type_a;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::RECORD { path: r#fn.clone(), exps: expl_1.clone(), comp: fieldNames.clone(), ty: tp.clone() }), ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::PARTEVALFUNCTION { path: r#fn, expList: expl, ty: tp, origType: t }, rel, ext_arg) => {
            let mut ext_arg_1: Type_a;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::PARTEVALFUNCTION { path: r#fn.clone(), expList: expl_1.clone(), ty: tp.clone(), origType: t.clone() }), ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::ARRAY { ty: tp, scalar, array: expl }, rel, ext_arg) => {
            let mut ext_arg_1: Type_a;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: scalar.clone(), array: expl_1.clone() }), ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::MATRIX { ty: tp, integer: dim, matrix: lstexpl }, rel, ext_arg) => {
            let mut ext_arg_1: Type_a;
            let mut lstexpl_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
            (lstexpl_1, ext_arg_1) = traverseExpMatrixTopDown(lstexpl.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::MATRIX { ty: tp.clone(), integer: dim.clone(), matrix: lstexpl_1.clone() }), ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::RANGE { ty: tp, start: e1, step: None, stop: e2 }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), rel.clone(), ext_arg_1.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::RANGE { ty: tp.clone(), start: e1_1.clone(), step: None, stop: e2_1.clone() })}, ext_arg_2.clone())
        },
        (_, Deref @ DAE::Exp::RANGE { ty: tp, start: e1, step: Some(e2), stop: e3 }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut e3_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            let mut ext_arg_3: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), rel.clone(), ext_arg_1.clone())?;
            (e3_1, ext_arg_3) = traverseExpTopDown(e3.clone(), rel.clone(), ext_arg_2.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone())) && referenceEq(&*(e3.clone()),&*(e3_1.clone()))) {inExp} else {Arc::new(DAE::Exp::RANGE { ty: tp.clone(), start: e1_1.clone(), step: Some(e2_1.clone()), stop: e3_1.clone() })}, ext_arg_3.clone())
        },
        (_, Deref @ DAE::Exp::TUPLE { PR: expl }, rel, ext_arg) => {
            let mut ext_arg_1: Type_a;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::TUPLE { PR: expl_1.clone() }), ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::CAST { ty: tp, exp: e1 }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::CAST { ty: tp.clone(), exp: e1_1.clone() }), ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::ASUB { exp: e1, sub: subs }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            expl_1 = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = getSubscriptExp(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (expl_1, ext_arg_2) = traverseExpListTopDown(expl_1.clone(), rel.clone(), ext_arg_1.clone())?;
            (makeASUB(e1_1.clone(), expl_1.clone())?, ext_arg_2.clone())
        },
        (_, Deref @ DAE::Exp::TSUB { exp: e1, ix: i, ty: tp }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::TSUB { exp: e1_1.clone(), ix: i.clone(), ty: tp.clone() }), ext_arg_1.clone())
        },
        (_, e1 @ Deref @ DAE::Exp::RSUB { .. }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut e1 = (*e1).clone();
            (e1_1, ext_arg_1) = traverseExpTopDown(var_field!((*e1).exp, DAE::Exp::RSUB).clone(), rel.clone(), ext_arg.clone())?;
            if !(referenceEq(&*(var_field!((*e1).exp, DAE::Exp::RSUB).clone()),&*(e1_1.clone()))) {
                assign_variant_field!(e1 => DAE::Exp::RSUB; exp = e1_1.clone());
            }
            (e1.clone(), ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::SIZE { exp: e1, sz: None }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::SIZE { exp: e1_1.clone(), sz: None }), ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::SIZE { exp: e1, sz: Some(e2) }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), rel.clone(), ext_arg_1.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::SIZE { exp: e1_1.clone(), sz: Some(e2_1.clone()) })}, ext_arg_2.clone())
        },
        (_, Deref @ DAE::Exp::CODE { .. }, _, ext_arg) => {
            (inExp, ext_arg.clone())
        },
        (_, Deref @ DAE::Exp::REDUCTION { reductionInfo, expr: e1, iterators: riters }, rel, ext_arg) => {
            let mut e1 = (*e1).clone();
            let mut riters = (*riters).clone();
            let mut ext_arg = (*ext_arg).clone();
            (e1, ext_arg) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (riters, ext_arg) = traverseReductionIteratorsTopDown(riters.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::REDUCTION { reductionInfo: reductionInfo.clone(), expr: e1.clone(), iterators: riters.clone() }), ext_arg.clone())
        },
        (_, Deref @ DAE::Exp::EMPTY { .. }, _, _) => {
            (inExp, inArg)
        },
        (_, Deref @ DAE::Exp::CONS { car: e1, cdr: e2 }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), rel.clone(), ext_arg_1.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::CONS { car: e1_1.clone(), cdr: e2_1.clone() })}, ext_arg_2.clone())
        },
        (_, Deref @ DAE::Exp::LIST { valList: expl }, rel, ext_arg) => {
            let mut ext_arg_1: Type_a;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::LIST { valList: expl_1.clone() }), ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::META_TUPLE { listExp: expl }, rel, ext_arg) => {
            let mut ext_arg_1: Type_a;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::META_TUPLE { listExp: expl_1.clone() }), ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::META_OPTION { exp: oe1 }, rel, ext_arg) => {
            let mut oe1 = (*oe1).clone();
            let mut ext_arg = (*ext_arg).clone();
            (oe1, ext_arg) = traverseExpOptTopDown(oe1.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::META_OPTION { exp: oe1.clone() }), ext_arg.clone())
        },
        (_, Deref @ DAE::Exp::MATCHEXPRESSION { matchType, inputs: expl, aliases, localDecls, cases, et }, rel, ext_arg) => {
            let mut expl = (*expl).clone();
            let mut cases = (*cases).clone();
            let mut ext_arg = (*ext_arg).clone();
            (expl, ext_arg) = traverseExpListTopDown(expl.clone(), rel.clone(), ext_arg.clone())?;
            (cases, ext_arg) = traverseCasesTopDown(cases.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::MATCHEXPRESSION { matchType: matchType.clone(), inputs: expl.clone(), aliases: aliases.clone(), localDecls: localDecls.clone(), cases: cases.clone(), et: et.clone() }), ext_arg.clone())
        },
        (_, Deref @ DAE::Exp::METARECORDCALL { path: r#fn, args: expl, fieldNames, index: i, typeVars }, rel, ext_arg) => {
            let mut ext_arg_1: Type_a;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::METARECORDCALL { path: r#fn.clone(), args: expl_1.clone(), fieldNames: fieldNames.clone(), index: i.clone(), typeVars: typeVars.clone() }), ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::UNBOX { exp: e1, ty: tp }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::UNBOX { exp: e1_1.clone(), ty: tp.clone() }), ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::BOX { exp: e1 }, rel, ext_arg) => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), rel.clone(), ext_arg.clone())?;
            (Arc::new(DAE::Exp::BOX { exp: e1_1.clone() }), ext_arg_1.clone())
        },
        (_, Deref @ DAE::Exp::PATTERN { .. }, _, ext_arg) => {
            (inExp, ext_arg.clone())
        },
        (_, Deref @ DAE::Exp::SHARED_LITERAL { .. }, _, ext_arg) => {
            (inExp, ext_arg.clone())
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = (printExpStr(inExp)?).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.traverseExpTopDown1")); __mm_s.push_str(&*literal!(" or ")); __mm_s.push_str(&*(System::dladdr(func.clone())).0); __mm_s.push_str(&*literal!("not implemented correctly: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outArg))
}

fn traverseExpMatrixTopDown<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>;

    let mut outMatrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
    let mut outTypeA: Type_a = inTypeA.clone();
    let mut row_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut same: bool = true;
    for mut row in &*inMatrix.clone() {
        let mut row = row.clone();
        (row_1, outTypeA) = traverseExpListTopDown(row.clone(), func.clone(), outTypeA.clone())?;
        same = if (metamodelica::ReferenceEq::reference_eq(&*(row.clone()), &*(row_1.clone()))) {same} else {false};
        outMatrix = metamodelica::cons(row_1.clone(), outMatrix.clone());
    }
    if same {
        outMatrix = inMatrix;
    } else {
        outMatrix = metamodelica::Dangerous::listReverseInPlace(outMatrix);
    }
    Ok((outMatrix, outTypeA))
}

pub fn traverseExpListTopDown<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut rel: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>, mut inExt_arg: Type_a) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>;

    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut outA: Type_a = inExt_arg.clone();
    let mut e_1: Arc<DAE::Exp>;
    let mut same: bool = true;
    for mut e in &*inExpl.clone() {
        let mut e = e.clone();
        (e_1, outA) = traverseExpTopDown(e.clone(), rel.clone(), outA.clone())?;
        same = if (referenceEq(&*(e.clone()),&*(e_1.clone()))) {same} else {false};
        outExpl = metamodelica::cons(e_1.clone(), outExpl.clone());
    }
    if same {
        outExpl = inExpl;
    } else {
        outExpl = metamodelica::Dangerous::listReverseInPlace(outExpl);
    }
    Ok((outExpl, outA))
}

pub fn traverseExpOpt<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inExp: Option<Arc<DAE::Exp>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<(Option<Arc<DAE::Exp>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outExp: Option<Arc<DAE::Exp>>;
    let mut outTypeA: Type_a;
    (outExp, outTypeA) = (::match_deref::match_deref! { match &((inExp.clone(), inTypeA)) {
        (None, a) => {
            (inExp, a.clone())
        },
        (oe @ Some(e), a) => {
            let mut e1: Arc<DAE::Exp>;
            let mut oe = (*oe).clone();
            let mut a = (*a).clone();
            (e1, a) = traverseExpBottomUp(e.clone(), func.clone(), a.clone())?;
            oe = if (referenceEq(&*(e.clone()),&*(e1.clone()))) {oe.clone()} else {Some(e1.clone())};
            (oe.clone(), a.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outExp, outTypeA))
}

pub(crate) fn traverseExpOptTopDown<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inExp: Option<Arc<DAE::Exp>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>, mut inTypeA: Type_a) -> Result<(Option<Arc<DAE::Exp>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>;

    let mut outExp: Option<Arc<DAE::Exp>>;
    let mut outA: Type_a;
    (outExp, outA) = (::match_deref::match_deref! { match &((inExp.clone(), inTypeA)) {
        (None, a) => {
            (None, a.clone())
        },
        (Some(e), a) => {
            let mut e1: Arc<DAE::Exp>;
            let mut a = (*a).clone();
            (e1, a) = traverseExpTopDown(e.clone(), func.clone(), a.clone())?;
            (if (referenceEq(&*(e.clone()),&*(e1.clone()))) {inExp} else {Some(e1.clone())}, a.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outExp, outA))
}

pub fn traverseExpCrefDims<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut inCref: Arc<DAE::ComponentRef>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(Arc<DAE::ComponentRef>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>;

    let mut outCref: Arc<DAE::ComponentRef>;
    let mut outArg: ArgT;
    (outCref, outArg) = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, identType: ty, subscriptLst: subs, componentRef: cr } => {
            let mut new_ty: Arc<DAE::Type>;
            let mut new_cr: Arc<DAE::ComponentRef>;
            let mut arg: ArgT;
            let mut cr = (*cr).clone();
            (new_cr, arg) = traverseExpCrefDims(cr.clone(), inFunc.clone(), inArg.clone())?;
            (new_ty, arg) = traverseExpTypeDims(ty.clone(), inFunc.clone(), inArg)?;
            cr = if (referenceEq(&*(new_cr.clone()),&*(cr.clone())) && referenceEq(&*(new_ty.clone()),&*(ty.clone()))) {inCref} else {Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (id.clone()).clone(), identType: new_ty.clone(), subscriptLst: subs.clone(), componentRef: new_cr.clone() })};
            (cr.clone(), arg.clone())
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty, subscriptLst: subs } => {
            let mut new_ty: Arc<DAE::Type>;
            let mut cr: Arc<DAE::ComponentRef>;
            let mut arg: ArgT;
            (new_ty, arg) = traverseExpTypeDims(ty.clone(), inFunc.clone(), inArg)?;
            cr = if (referenceEq(&*(new_ty.clone()),&*(ty.clone()))) {inCref} else {Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (id.clone()).clone(), identType: new_ty.clone(), subscriptLst: subs.clone() })};
            (cr.clone(), arg.clone())
        },
        _ => {
            (inCref, inArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCref, outArg))
}

pub(crate) fn traverseExpTypeDims<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut inType: Arc<DAE::Type>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(Arc<DAE::Type>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>;

    let mut outType: Arc<DAE::Type>;
    let mut outArg: ArgT;
    (outType, outArg) = (::match_deref::match_deref! { match &(inType.clone()) {
        Deref @ DAE::Type::T_ARRAY { ty, dims } => {
            let mut arg: ArgT;
            let mut changed: bool;
            let mut ty = (*ty).clone();
            (_, arg, changed) = traverseExpTypeDims2(dims.clone(), inFunc.clone(), inArg)?;
            ty = if (changed.clone()) {Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: dims.clone() })} else {inType};
            (ty.clone(), arg.clone())
        },
        Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: state, varLst: vars, complexType: ty, equalityConstraint: ec } => {
            let mut new_ty: Arc<DAE::Type>;
            let mut arg: ArgT;
            let mut ty = (*ty).clone();
            (new_ty, arg) = traverseExpTypeDims(ty.clone(), inFunc.clone(), inArg)?;
            ty = if (referenceEq(&*(new_ty.clone()),&*(ty.clone()))) {inType} else {Arc::new(DAE::Type::T_SUBTYPE_BASIC { complexClassType: state.clone(), varLst: vars.clone(), complexType: ty.clone(), equalityConstraint: ec.clone() })};
            (ty.clone(), arg.clone())
        },
        _ => {
            (inType, inArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outType, outArg))
}

fn traverseExpTypeDims2<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(Arc<metamodelica::List<Arc<DAE::Dimension>>>, ArgT, bool)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>;

    let mut outDims: Arc<metamodelica::List<Arc<DAE::Dimension>>> = metamodelica::nil();
    let mut outArg: ArgT = inArg.clone();
    let mut outChanged: bool = false;
    let mut changed: bool = false;
    let mut exp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut new_exp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    for mut dim in &*inDims.clone() {
        let mut dim = dim.clone();
        dim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ DAE::Dimension::DIM_EXP { exp: __esc_exp } => {
            exp = (*__esc_exp).clone();
            (new_exp, outArg) = inFunc(exp.clone(), outArg.clone())?;
            changed = !(referenceEq(&*(new_exp.clone()),&*(exp.clone())));
            outChanged = outChanged || changed;
            if (changed) {Arc::new(DAE::Dimension::DIM_EXP { exp: exp.clone() })} else {dim.clone()}
        },
        _ => dim.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outDims = metamodelica::cons(dim.clone(), outDims.clone());
    }
    outDims = if (outChanged) {outDims.reverse()} else {inDims};
    Ok((outDims, outArg, outChanged))
}

pub(crate) fn extractUniqueCrefsFromExp(mut inExp: Arc<DAE::Exp>, mut expand: bool) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut ocrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    ocrefs = ComponentReference::uniqueList(extractCrefsFromExp(inExp)?)?;
    if expand {
        ocrefs = List::flatten(List::map1(ocrefs, (std::sync::Arc::new(fnptr!(ComponentReference::expandCref, Arc<DAE::ComponentRef>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, bool) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>), true)?)?;
    }
    Ok(ocrefs)
}

pub fn extractCrefsFromExp(mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut ocrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    (_, ocrefs) = traverseExpBottomUp(inExp, (std::sync::Arc::new(traversingComponentRefFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
    Ok(ocrefs)
}

pub fn traversingComponentRefFinder(mut inExp: Arc<DAE::Exp>, mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (outExp, crefs) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefs.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, __esc_crefs) => {
            crefs = (*__esc_crefs).clone();
            crefs = List::unionEltOnTrue(cr.clone(), crefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
            (inExp, crefs.clone())
        },
        _ => {
            (inExp, inCrefs)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, crefs))
}

pub fn extractUniqueCrefsFromExpDerPreStart(mut inExp: Arc<DAE::Exp>, mut expand: bool) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut ocrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    ocrefs = ComponentReference::uniqueList(extractCrefsFromExpDerPreStart(inExp, expand)?)?;
    Ok(ocrefs)
}

pub fn extractCrefsFromExpDerPreStart(mut inExp: Arc<DAE::Exp>, mut expand: bool) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut ocrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    (_, ocrefs) = traverseExpTopDown(inExp, (std::sync::Arc::new(traversingComponentRefFinderDerPreStart) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>), metamodelica::nil())?;
    if expand {
        ocrefs = List::flatten(List::map1(ocrefs, (std::sync::Arc::new(fnptr!(ComponentReference::expandCref, Arc<DAE::ComponentRef>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, bool) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>), true)?)?;
    }
    Ok(ocrefs)
}

pub(crate) fn traversingComponentRefFinderDerPreStart(mut inExp: Arc<DAE::Exp>, mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut e: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (e, cont, crefs) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefs.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, __esc_crefs) => {
            crefs = (*__esc_crefs).clone();
            crefs = List::unionEltOnTrue(cr.clone(), inCrefs, (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
            (inExp, false, crefs.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, _) => {
            let mut cr = (*cr).clone();
            cr = ComponentReference::crefPrefixDer(cr.clone());
            crefs = List::unionEltOnTrue(cr.clone(), inCrefs, (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
            (inExp, false, crefs.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, _) => {
            let mut cr = (*cr).clone();
            cr = ComponentReference::crefPrefixPre(cr.clone());
            crefs = List::unionEltOnTrue(cr.clone(), inCrefs, (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
            (inExp, false, crefs.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, _) => {
            let mut cr = (*cr).clone();
            cr = ComponentReference::crefPrefixPrevious(cr.clone());
            crefs = List::unionEltOnTrue(cr.clone(), inCrefs, (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
            (inExp, false, crefs.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "start" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, _) => {
            let mut cr = (*cr).clone();
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.traversingComponentRefFinderDerPreStart")); __mm_s.push_str(&*literal!(" - Found a start call expression ")); __mm_s.push_str(&*printExpStr(inExp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("FrontEnd/Expression.mo"))?;
            cr = ComponentReference::crefPrefixStart(cr.clone());
            crefs = List::unionEltOnTrue(cr.clone(), inCrefs, (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
            (inExp, false, crefs.clone())
        },
        _ => {
            (inExp, true, inCrefs)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((e, cont, crefs))
}

pub fn extractUniqueCrefsFromStatmentS(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut olhscrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut orhscrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut lhscreflstlst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>;
    let mut rhscreflstlst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>;
    (lhscreflstlst, rhscreflstlst) = List::map_2(inStmts, (std::sync::Arc::new(extractCrefsStatment) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> + 'static>))?;
    orhscrefs = ComponentReference::uniqueList(List::flatten(rhscreflstlst)?)?;
    olhscrefs = ComponentReference::uniqueList(List::flatten(lhscreflstlst)?)?;
    Ok((olhscrefs, orhscrefs))
}

pub fn extractCrefsStatment(mut inStmt: Arc<DAE::Statement>) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut olcrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut orcrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (olcrefs, orcrefs) = (::match_deref::match_deref! { match &(inStmt) {
        Deref @ DAE::Statement::STMT_ASSIGN { exp1, exp: exp2, .. } => {
            olcrefs = extractCrefsFromExpDerPreStart(exp1.clone(), false)?;
            orcrefs = extractCrefsFromExpDerPreStart(exp2.clone(), false)?;
            (olcrefs, orcrefs)
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: expLst, exp: exp2, .. } => {
            olcrefs = List::flatten(List::map1(expLst.clone(), (std::sync::Arc::new(extractCrefsFromExpDerPreStart) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>), false)?)?;
            orcrefs = extractCrefsFromExpDerPreStart(exp2.clone(), false)?;
            (olcrefs, orcrefs)
        },
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: exp1, exp: exp2, .. } => {
            olcrefs = extractCrefsFromExpDerPreStart(exp1.clone(), false)?;
            orcrefs = extractCrefsFromExpDerPreStart(exp2.clone(), false)?;
            (olcrefs, orcrefs)
        },
        Deref @ DAE::Statement::STMT_IF { statementLst: stmtLst, .. } => {
            (olcrefs, orcrefs) = extractUniqueCrefsFromStatmentS(stmtLst.clone())?;
            (olcrefs, orcrefs)
        },
        Deref @ DAE::Statement::STMT_FOR { statementLst: stmtLst, .. } => {
            (olcrefs, orcrefs) = extractUniqueCrefsFromStatmentS(stmtLst.clone())?;
            (olcrefs, orcrefs)
        },
        Deref @ DAE::Statement::STMT_WHILE { statementLst: stmtLst, .. } => {
            (olcrefs, orcrefs) = extractUniqueCrefsFromStatmentS(stmtLst.clone())?;
            (olcrefs, orcrefs)
        },
        Deref @ DAE::Statement::STMT_WHEN { statementLst: stmtLst, .. } => {
            (olcrefs, orcrefs) = extractUniqueCrefsFromStatmentS(stmtLst.clone())?;
            (olcrefs, orcrefs)
        },
        Deref @ DAE::Statement::STMT_ASSERT { cond: exp1, .. } => {
            orcrefs = extractCrefsFromExpDerPreStart(exp1.clone(), false)?;
            (metamodelica::nil(), orcrefs)
        },
        _ => {
            (metamodelica::nil(), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((olcrefs, orcrefs))
}

pub fn getLhsCrefsFromStatements(mut inStmts: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut lhsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut lhsCrefsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>;
    lhsCrefsLst = List::map(inStmts, (std::sync::Arc::new(getLhsCrefsFromStatement) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Statement>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>))?;
    lhsCrefs = List::flatten(lhsCrefsLst)?;
    Ok(lhsCrefs)
}

fn getLhsCrefsFromStatement(mut inStmt: Arc<DAE::Statement>) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> {
    let mut lhsCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    lhsCrefs = (::match_deref::match_deref! { match &(inStmt) {
        Deref @ DAE::Statement::STMT_ASSIGN { exp1, .. } => {
            lhsCrefs = extractCrefsFromExpDerPreStart(exp1.clone(), false)?;
            lhsCrefs
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: expLst, .. } => {
            lhsCrefs = List::flatten(List::map1(expLst.clone(), (std::sync::Arc::new(extractCrefsFromExpDerPreStart) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>> + 'static>), false)?)?;
            lhsCrefs
        },
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: exp1, .. } => {
            lhsCrefs = extractCrefsFromExpDerPreStart(exp1.clone(), false)?;
            lhsCrefs
        },
        Deref @ DAE::Statement::STMT_IF { statementLst: stmtLst, .. } => {
            lhsCrefs = getLhsCrefsFromStatements(stmtLst.clone())?;
            lhsCrefs
        },
        Deref @ DAE::Statement::STMT_FOR { statementLst: stmtLst, .. } => {
            lhsCrefs = getLhsCrefsFromStatements(stmtLst.clone())?;
            lhsCrefs
        },
        Deref @ DAE::Statement::STMT_WHILE { statementLst: stmtLst, .. } => {
            lhsCrefs = getLhsCrefsFromStatements(stmtLst.clone())?;
            lhsCrefs
        },
        Deref @ DAE::Statement::STMT_WHEN { statementLst: stmtLst, .. } => {
            lhsCrefs = getLhsCrefsFromStatements(stmtLst.clone())?;
            lhsCrefs
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(lhsCrefs)
}

pub fn expHasInitial(mut exp: Arc<DAE::Exp>) -> Result<bool> {
    let mut found: bool;
    (_, found) = traverseExpTopDown(exp, (std::sync::Arc::new(fnptr!(traversingexpHasInitial, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
    Ok(found)
}

pub(crate) fn traversingexpHasInitial(mut exp: Arc<DAE::Exp>, mut found: bool) -> (Arc<DAE::Exp>, bool, bool) {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut cont: bool;
    let mut found: bool = found;
    if found {
        cont = false;
        return (exp.clone(), cont.clone(), found.clone());
    }
    (cont, found) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => (false, true),
        _ => (true, found),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, cont, found)
}

pub fn expHasCrefs(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut hasCrefs: bool;
    hasCrefs = (::match_deref::match_deref! { match &(inExp.clone()) {
        _ => {
            let mut b: bool;
            (_, b) = traverseExpTopDown(inExp, (std::sync::Arc::new(fnptr!(traversingComponentRefPresent, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
            b.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hasCrefs)
}

pub fn traversingComponentRefPresent(mut inExp: Arc<DAE::Exp>, mut found: bool) -> (Arc<DAE::Exp>, bool, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut outFound: bool;
    (outExp, cont, outFound) = (::match_deref::match_deref! { match &((inExp.clone(), found)) {
        (_, true) => (inExp, false, true),
        (Deref @ DAE::Exp::CREF { .. }, _) => (inExp, false, true),
        _ => (inExp, true, false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outFound)
}

pub fn traversingComponentRefFinderNoPreDer(mut inExp: Arc<DAE::Exp>, mut inCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<(Arc<DAE::Exp>, bool, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> {
    let mut e: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (e, cont, crefs) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefs.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, __esc_crefs) => {
            crefs = (*__esc_crefs).clone();
            crefs = List::unionEltOnTrue(cr.clone(), crefs.clone(), (std::sync::Arc::new(ComponentReferenceBasics::crefEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
            (inExp, false, crefs.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _) => {
            (inExp, false, inCrefs)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
            (inExp, false, inCrefs)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
            (inExp, false, inCrefs)
        },
        _ => {
            (inExp, true, inCrefs)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((e, cont, crefs))
}

pub fn expHasCref(mut inExp: Arc<DAE::Exp>, mut inCr: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut hasCref: bool;
    let (_, (_, __pa0)) = traverseExpTopDown(inExp, (std::sync::Arc::new(fnptr!(traversingexpHasCref, Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool))> + 'static>), (inCr, false))?;
    hasCref = __pa0.clone();
    Ok(hasCref)
}

pub(crate) fn traversingexpHasCref(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<DAE::ComponentRef>, bool)) -> (Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (Arc<DAE::ComponentRef>, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, (cr, false)) => {
                    let mut b: bool;
                    b = ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cr1.clone())?;
                    Ok((inExp.clone(), !(b.clone()), if (b.clone()) {(cr.clone(), b.clone())} else {inTpl.clone()}))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (_, b)) => {
                    Ok((inExp.clone(), !(b.clone()), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outTpl)
}

pub(crate) fn expHasCrefName(mut inExp: Arc<DAE::Exp>, mut name: ArcStr) -> Result<bool> {
    let mut hasCref: bool;
    let (_, (_, __pa0)) = traverseExpTopDown(inExp, (std::sync::Arc::new(traversingexpHasName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (ArcStr, bool)) -> Result<(Arc<DAE::Exp>, bool, (ArcStr, bool))> + 'static>), (name, false))?;
    hasCref = __pa0.clone();
    Ok(hasCref)
}

pub fn anyExpHasCrefName(mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut name: ArcStr) -> Result<bool> {
    let mut hasCref: bool;
    hasCref = List::applyAndFold1(inExps, (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(expHasCrefName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArcStr) -> Result<bool> + 'static>), (name).clone(), false)?;
    Ok(hasCref)
}

pub(crate) fn traversingexpHasName(mut inExp: Arc<DAE::Exp>, mut inTpl: (ArcStr, bool)) -> Result<(Arc<DAE::Exp>, bool, (ArcStr, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (ArcStr, bool);
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (name, false)) => {
            let mut b: bool;
            b = name.clone() == ComponentReferenceBasics::crefFirstIdent(cr.clone())?;
            (inExp, !(b.clone()), if (b.clone()) {(name.clone(), b.clone())} else {inTpl})
        },
        (_, (_, b)) => {
            (inExp, !(b.clone()), inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

pub fn expHasDerCref(mut inExp: Arc<DAE::Exp>, mut inCr: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut hasCref: bool;
    let (_, (_, __pa0)) = traverseExpTopDown(inExp, (std::sync::Arc::new(fnptr!(traversingexpHasDerCref, Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool))> + 'static>), (inCr, false))?;
    hasCref = __pa0.clone();
    Ok(hasCref)
}

pub(crate) fn traversingexpHasDerCref(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<DAE::ComponentRef>, bool)) -> (Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (Arc<DAE::ComponentRef>, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, (cr, false)) => {
                    let mut b: bool;
                    b = ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cr1.clone())?;
                    Ok((inExp.clone(), !(b.clone()), if (b.clone()) {(cr.clone(), b.clone())} else {inTpl.clone()}))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, (cr, false)) => {
                    let mut b: bool;
                    b = ComponentReferenceBasics::crefPrefixOf(cr.clone(), cr1.clone())?;
                    Ok((inExp.clone(), !(b.clone()), if (b.clone()) {(cr.clone(), b.clone())} else {inTpl.clone()}))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (_, b)) => {
                    Ok((inExp.clone(), !(b.clone()), inTpl.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outTpl)
}

pub fn expHasDer(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut hasCref: bool;
    (_, hasCref) = traverseExpTopDown(inExp, (std::sync::Arc::new(traversingexpHasDer) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
    Ok(hasCref)
}

pub(crate) fn traversingexpHasDer(mut inExp: Arc<DAE::Exp>, mut inTpl: bool) -> Result<(Arc<DAE::Exp>, bool, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: bool;
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl)) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, false) => {
            (inExp, false, true)
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, false) if (intEq(System::strncmp((ComponentReferenceBasics::crefFirstIdent(cr.clone())?).clone(), (literal!("$DERAlias")).clone(), 9), 0)) => {
            (inExp, false, true)
        },
        (_, b) => {
            (inExp, !(b.clone()), inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

pub fn expHasPre(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut hasPre: bool;
    (_, hasPre) = traverseExpTopDown(inExp, (std::sync::Arc::new(fnptr!(traversingexpHasPre, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
    Ok(hasPre)
}

fn traversingexpHasPre(mut inExp: Arc<DAE::Exp>, mut inHasIt: bool) -> (Arc<DAE::Exp>, bool, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut outHasIt: bool;
    (outExp, cont, outHasIt) = (::match_deref::match_deref! { match &((inExp.clone(), inHasIt)) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, false) => {
            (inExp, false, true)
        },
        (_, b) => {
            (inExp, !(b.clone()), inHasIt)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outHasIt)
}

pub fn expHasPrevious(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut hasPre: bool;
    (_, hasPre) = traverseExpTopDown(inExp, (std::sync::Arc::new(fnptr!(traversingexpHasPrevious, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
    Ok(hasPre)
}

fn traversingexpHasPrevious(mut inExp: Arc<DAE::Exp>, mut inHasIt: bool) -> (Arc<DAE::Exp>, bool, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut outHasIt: bool;
    (outExp, cont, outHasIt) = (::match_deref::match_deref! { match &((inExp.clone(), inHasIt)) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, false) => {
            (inExp, false, true)
        },
        (_, b) => {
            (inExp, !(b.clone()), inHasIt)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outHasIt)
}

pub fn expHasCrefNoPreorDer(mut inExp: Arc<DAE::Exp>, mut inCr: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut hasCref: bool;
    let (_, (_, __pa0)) = traverseExpTopDown(inExp, (std::sync::Arc::new(traversingexpHasCrefNoPreorDer) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool))> + 'static>), (inCr, false))?;
    hasCref = __pa0.clone();
    Ok(hasCref)
}

pub(crate) fn traversingexpHasCrefNoPreorDer(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (Arc<DAE::ComponentRef>, bool);
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, (cr, false)) => {
            let mut b: bool;
            b = ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cr1.clone())?;
            (inExp, !(b.clone()), if (b.clone()) {(cr.clone(), b.clone())} else {inTpl})
        },
        (_, (_, b)) => {
            (inExp, !(b.clone()), inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

pub fn expHasCrefsNoPreOrStart(mut inExp: Arc<DAE::Exp>, mut inCr: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<bool> {
    let mut hasCref: bool = false;
    for mut cr in &*inCr {
        let mut cr = cr.clone();
        let (_, (_, __pa0)) = traverseExpTopDown(inExp.clone(), (std::sync::Arc::new(traversingexpHasCrefNoPreOrStart) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool))> + 'static>), (cr.clone(), false))?;
        hasCref = __pa0.clone();
        if hasCref {
            break;
        }
    }
    Ok(hasCref)
}

pub fn expHasCrefNoPreOrStart(mut inExp: Arc<DAE::Exp>, mut inCr: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut hasCref: bool;
    let (_, (_, __pa0)) = traverseExpTopDown(inExp, (std::sync::Arc::new(traversingexpHasCrefNoPreOrStart) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool))> + 'static>), (inCr, false))?;
    hasCref = __pa0.clone();
    Ok(hasCref)
}

fn traversingexpHasCrefNoPreOrStart(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (Arc<DAE::ComponentRef>, bool);
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$_round" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, (cr, false)) => {
            let mut b: bool;
            b = ComponentReferenceBasics::crefEqualNoStringCompare(cr.clone(), cr1.clone())?;
            (inExp, !(b.clone()), if (b.clone()) {(cr.clone(), b.clone())} else {inTpl})
        },
        (_, (_, b)) => {
            (inExp, !(b.clone()), inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

pub fn expHasCrefInIf(mut inExp: Arc<DAE::Exp>, mut inCr: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut hasCref: bool;
    let (_, (_, __pa0)) = traverseExpTopDown(inExp, (std::sync::Arc::new(expHasCrefInIfWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool))> + 'static>), (inCr, false))?;
    hasCref = __pa0.clone();
    Ok(hasCref)
}

pub(crate) fn expHasCrefInIfWork(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (Arc<DAE::ComponentRef>, bool);
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: _, expElse: _ }, (cr, false)) if (!(isFunCall(e1.clone(), (literal!("noEvent")).clone()))) => {
            let mut b: bool;
            b = expHasCref(e1.clone(), cr.clone())?;
            (e1.clone(), true, if (b.clone()) {(cr.clone(), b.clone())} else {inTpl})
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil } }, .. }, (cr, false)) if (i.clone() > 1) => {
            (e1.clone(), true, (cr.clone(), expHasCref(e1.clone(), cr.clone())?))
        },
        (Deref @ DAE::Exp::CALL { .. }, (cr, false)) if (isEventTriggeringFunctionExp(inExp.clone())) => {
            let mut b: bool;
            b = expHasCref(inExp.clone(), cr.clone())?;
            (inExp.clone(), true, if (b.clone()) {(cr.clone(), b.clone())} else {inTpl})
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }, .. }, (cr, false)) => {
            let mut b: bool;
            b = expHasCref(e1.clone(), cr.clone())?;
            (e1.clone(), true, if (b.clone()) {(cr.clone(), b.clone())} else {inTpl})
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. }, (cr, false)) => {
            let mut b: bool;
            b = expHasCref(e1.clone(), cr.clone())?;
            (e1.clone(), !(b.clone()), if (b.clone()) {(cr.clone(), b.clone())} else {inTpl})
        },
        (_, (_, true)) => {
            (inExp.clone(), false, inTpl)
        },
        _ => {
            (inExp.clone(), true, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

pub fn expHasCrefInSmoothZero(mut exp: Arc<DAE::Exp>, mut cr: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut b: bool;
    let (_, (_, __pa0)) = traverseExpBottomUp(exp, (std::sync::Arc::new(expHasCrefInSmoothZeroWork) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, bool))> + 'static>), (cr, false))?;
    b = __pa0.clone();
    Ok(b)
}

fn expHasCrefInSmoothZeroWork(mut exp: Arc<DAE::Exp>, mut tpl: (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, bool))> {
    let mut exp: Arc<DAE::Exp> = exp;
    let mut tpl: (Arc<DAE::ComponentRef>, bool) = tpl;
    tpl = (::match_deref::match_deref! { match &((exp.clone(), tpl.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: 0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: sCr, .. }, tail: Deref @ metamodelica::List::Nil } }, .. }, (cr, false)) => {
            let mut b: bool;
            b = ComponentReferenceBasics::crefEqual(sCr.clone(), cr.clone())?;
            (cr.clone(), b.clone())
        },
        _ => {
            tpl
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, tpl))
}

pub fn traverseCrefsFromExp<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inExp: Arc<DAE::Exp>, mut inFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<Type_a> + 'static>, mut inArg: Type_a) -> Result<Type_a> {
    pub type FuncCrefTypeA<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<Type_a> + 'static>;

    let mut outArg: Type_a;
    outArg = (match inArg.clone() {
        _ => {
            let mut arg: Type_a;
            let (_, (_, __pa0)) = traverseExpBottomUp(inExp, (std::sync::Arc::new(traversingCrefFinder) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), (inFunc.clone(), inArg))?;
            arg = __pa0.clone();
            arg.clone()
        },
    });
    Ok(outArg)
}

fn traversingCrefFinder<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<Type_a> + 'static>, Type_a)) -> Result<(Arc<DAE::Exp>, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<Type_a> + 'static>, Type_a))> {
    pub type FuncCrefTypeA<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<Type_a> + 'static>;

    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Type_a) -> Result<Type_a> + 'static>, Type_a);
    (outExp, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (func, arg)) => {
            let mut arg1: Type_a;
            arg1 = func(cr.clone(), arg.clone())?;
            (inExp, if (metamodelica::ReferenceEq::reference_eq(&(arg.clone()), &(arg1.clone()))) {inTpl} else {(func.clone(), arg1.clone())})
        },
        _ => {
            (inExp, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTpl))
}

pub(crate) fn extractDivExpFromExp(mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    (_, outExps) = traverseExpBottomUp(inExp, (std::sync::Arc::new(fnptr!(traversingDivExpFinder, Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Exp>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> + 'static>), metamodelica::nil())?;
    Ok(outExps)
}

fn traversingDivExpFinder(mut e: Arc<DAE::Exp>, mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> (Arc<DAE::Exp>, Arc<metamodelica::List<Arc<DAE::Exp>>>) {
    let mut outExp: Arc<DAE::Exp>;
    let mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    (outExp, acc) = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::DIV { ty: _ }, exp2: e2, .. } => {
            (e, metamodelica::cons(e2.clone(), exps))
        },
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::DIV_ARR { ty: _ }, exp2: e2, .. } => {
            (e, metamodelica::cons(e2.clone(), exps))
        },
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::DIV_ARRAY_SCALAR { ty: _ }, exp2: e2, .. } => {
            (e, metamodelica::cons(e2.clone(), exps))
        },
        Deref @ DAE::Exp::BINARY { operator: DAE::Operator::DIV_SCALAR_ARRAY { ty: _ }, exp2: e2, .. } => {
            (e, metamodelica::cons(e2.clone(), exps))
        },
        _ => {
            (e, exps)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, acc)
}

pub(crate) fn traverseExpListBidir<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut inExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inEnterFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inExitFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>;

    let mut outExpl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut outArg: ArgT = inArg.clone();
    let mut e1: Arc<DAE::Exp>;
    let mut allEq: bool = true;
    let mut delst: DoubleEnded::MutableList<Arc<DAE::Exp>> = <DoubleEnded::MutableList<Arc<DAE::Exp>> as ::std::default::Default>::default();
    let mut nEq: i32 = 0;
    for mut e in &*inExpl.clone() {
        let mut e = e.clone();
        (e1, outArg) = traverseExpBidir(e.clone(), inEnterFunc.clone(), inExitFunc.clone(), outArg.clone())?;
        if if (allEq) {!(referenceEq(&*(e.clone()),&*(e1.clone())))} else {false} {
            allEq = false;
            delst = DoubleEnded::empty(e1.clone());
            for mut elt in &*inExpl.clone() {
                let mut elt = elt.clone();
                if nEq < 1 {
                    break;
                }
                DoubleEnded::push_back(delst.clone(), elt.clone())?;
                nEq = nEq - 1;
            }
        }
        if allEq {
            nEq = nEq + 1;
        } else {
            DoubleEnded::push_back(delst.clone(), e1.clone())?;
        }
    }
    outExpl = if (allEq) {inExpl} else {DoubleEnded::toListAndClear(delst, metamodelica::nil())?};
    Ok((outExpl, outArg))
}

pub fn traverseExpBidir<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut inExp: Arc<DAE::Exp>, mut inEnterFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inExitFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>;

    let mut outExp: Arc<DAE::Exp>;
    let mut outArg: ArgT;
    (outExp, outArg) = inEnterFunc(inExp, inArg)?;
    (outExp, outArg) = traverseExpBidirSubExps(outExp, inEnterFunc.clone(), inExitFunc.clone(), outArg)?;
    (outExp, outArg) = inExitFunc(outExp, outArg)?;
    Ok((outExp, outArg))
}

pub(crate) fn traverseExpOptBidir<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut inExp: Option<Arc<DAE::Exp>>, mut inEnterFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inExitFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(Option<Arc<DAE::Exp>>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Exp, ArgT) -> Result<Exp> + 'static>;

    let mut outExp: Option<Arc<DAE::Exp>>;
    let mut outArg: ArgT;
    (outExp, outArg) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Some(e) => {
            let mut e1: Arc<DAE::Exp>;
            let mut arg: ArgT;
            (e1, arg) = traverseExpBidir(e.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(e.clone()),&*(e1.clone()))) {inExp} else {Some(e1.clone())}, arg.clone())
        },
        _ => {
            (inExp, inArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outArg))
}

fn traverseExpBidirSubExps<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut inExp: Arc<DAE::Exp>, mut inEnterFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inExitFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>;

    let mut outExp: Arc<DAE::Exp>;
    let mut outArg: ArgT;
    (outExp, outArg) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ICONST { .. } => {
            (inExp, inArg)
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            (inExp, inArg)
        },
        Deref @ DAE::Exp::SCONST { .. } => {
            (inExp, inArg)
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            (inExp, inArg)
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            (inExp, inArg)
        },
        Deref @ DAE::Exp::CREF { componentRef: cref, ty } => {
            let mut cref_1: ComponentRef;
            let mut arg: ArgT;
            (cref_1, arg) = traverseExpBidirCref(cref.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(cref.clone()),&*(cref_1.clone()))) {inExp} else {Arc::new(DAE::Exp::CREF { componentRef: cref_1.clone(), ty: ty.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (e2_1, arg) = traverseExpBidir(e2.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::UNARY { operator: op, exp: e1 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e1.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (e2_1, arg) = traverseExpBidir(e2.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::LBINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, index, optionExpisASUB: opt_exp_asub } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (e2_1, arg) = traverseExpBidir(e2.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::RELATION { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone(), index: index.clone(), optionExpisASUB: opt_exp_asub.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut e3_1: Arc<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (e2_1, arg) = traverseExpBidir(e2.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg.clone())?;
            (e3_1, arg) = traverseExpBidir(e3.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone())) && referenceEq(&*(e3.clone()),&*(e3_1.clone()))) {inExp} else {Arc::new(DAE::Exp::IFEXP { expCond: e1_1.clone(), expThen: e2_1.clone(), expElse: e3_1.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::CALL { path, expLst: expl, attr } => {
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::CALL { path: path.clone(), expLst: expl_1.clone(), attr: attr.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::RECORD { path, exps: expl, comp: strl, ty } => {
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::RECORD { path: path.clone(), exps: expl_1.clone(), comp: strl.clone(), ty: ty.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { path, expList: expl, ty, origType: t } => {
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::PARTEVALFUNCTION { path: path.clone(), expList: expl_1.clone(), ty: ty.clone(), origType: t.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::ARRAY { ty, scalar: b1, array: expl } => {
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: b1.clone(), array: expl_1.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::MATRIX { ty, integer: dim, matrix: mat_expl } => {
            let mut arg: ArgT;
            let mut mat_expl = (*mat_expl).clone();
            (mat_expl, arg) = List::map2Fold(mat_expl.clone(), (std::sync::Arc::new(traverseExpListBidir) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, _, _, _) -> Result<_> + 'static>), inEnterFunc.clone(), inExitFunc.clone(), inArg, metamodelica::nil())?;
            (Arc::new(DAE::Exp::MATRIX { ty: ty.clone(), integer: dim.clone(), matrix: mat_expl.clone() }), arg.clone())
        },
        Deref @ DAE::Exp::RANGE { ty, start: e1, step: oe1, stop: e2 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut oe1_1: Option<Arc<DAE::Exp>>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (oe1_1, arg) = traverseExpOptBidir(oe1.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg.clone())?;
            (e2_1, arg) = traverseExpBidir(e2.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone())) && (match (&(oe1.clone()), &(oe1_1.clone())) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false })) {inExp} else {Arc::new(DAE::Exp::RANGE { ty: ty.clone(), start: e1_1.clone(), step: oe1_1.clone(), stop: e2_1.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::TUPLE { PR: expl } => {
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::TUPLE { PR: expl_1.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::CAST { ty, exp: e1 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::CAST { ty: ty.clone(), exp: e1.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::ASUB { exp: e1, sub: subs } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut arg: ArgT;
            let mut subs = (*subs).clone();
            expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = getSubscriptExp(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg.clone())?;
            subs = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
        for mut sub in (expl.clone()).into_iter().cloned() {
            let __x = makeIndexSubscript(sub.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::ASUB { exp: e1_1.clone(), sub: subs.clone() })}, arg.clone())
        },
        e1 @ Deref @ DAE::Exp::RSUB { .. } => {
            let mut e2: Arc<DAE::Exp>;
            let mut arg: ArgT;
            let mut e1 = (*e1).clone();
            (e2, arg) = traverseExpBidir(var_field!((*e1).exp, DAE::Exp::RSUB).clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            if referenceEq(&*(var_field!((*e1).exp, DAE::Exp::RSUB).clone()),&*(e2.clone())) {
                assign_variant_field!(e1 => DAE::Exp::RSUB; exp = e2.clone());
            }
            (e1.clone(), arg.clone())
        },
        Deref @ DAE::Exp::TSUB { exp: e1, ix: i, ty } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::TSUB { exp: e1_1.clone(), ix: i.clone(), ty: ty.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::SIZE { exp: e1, sz: oe1 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut oe1_1: Option<Arc<DAE::Exp>>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (oe1_1, arg) = traverseExpOptBidir(oe1.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && (match (&(oe1.clone()), &(oe1_1.clone())) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false })) {inExp} else {Arc::new(DAE::Exp::SIZE { exp: e1_1.clone(), sz: oe1_1.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::CODE { .. } => {
            (inExp, inArg)
        },
        Deref @ DAE::Exp::REDUCTION { reductionInfo, expr: e1, iterators: riters } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut riters_1: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (riters_1, arg) = List::map2Fold(riters.clone(), (std::sync::Arc::new(traverseReductionIteratorBidir) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ReductionIterator>, _, _, _) -> Result<_> + 'static>), inEnterFunc.clone(), inExitFunc.clone(), arg.clone(), metamodelica::nil())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && metamodelica::ReferenceEq::reference_eq(&*(riters.clone()), &*(riters_1.clone()))) {inExp} else {Arc::new(DAE::Exp::REDUCTION { reductionInfo: reductionInfo.clone(), expr: e1.clone(), iterators: riters.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::LIST { valList: expl } => {
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::LIST { valList: expl_1.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::CONS { car: e1, cdr: e2 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut e2_1: Arc<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (e2_1, arg) = traverseExpBidir(e2.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg.clone())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone())) && referenceEq(&*(e2.clone()),&*(e2_1.clone()))) {inExp} else {Arc::new(DAE::Exp::CONS { car: e1_1.clone(), cdr: e2_1.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::META_TUPLE { listExp: expl } => {
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::TUPLE { PR: expl_1.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::META_OPTION { exp: oe1 } => {
            let mut oe1_1: Option<Arc<DAE::Exp>>;
            let mut arg: ArgT;
            (oe1_1, arg) = traverseExpOptBidir(oe1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if ((match (&(oe1.clone()), &(oe1_1.clone())) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false })) {inExp} else {Arc::new(DAE::Exp::META_OPTION { exp: oe1_1.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::METARECORDCALL { path, args: expl, fieldNames: strl, index, typeVars } => {
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::METARECORDCALL { path: path.clone(), args: expl_1.clone(), fieldNames: strl.clone(), index: index.clone(), typeVars: typeVars.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { matchType: match_ty, inputs: expl, aliases, localDecls: match_decls, cases: match_cases, et: ty } => {
            let mut expl_1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            Error::addSourceMessage(Error::COMPILER_NOTIFICATION.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.traverseExpBidirSubExps")); __mm_s.push_str(&*literal!(" not yet implemented for match expressions. Called using: ")); __mm_s.push_str(&*(System::dladdr(inEnterFunc.clone())).0); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*(System::dladdr(inExitFunc.clone())).0); ArcStr::from(__mm_s) }).clone()], metamodelica::sourceInfo!("FrontEnd/Expression.mo"))?;
            (if (metamodelica::ReferenceEq::reference_eq(&*(expl.clone()), &*(expl_1.clone()))) {inExp} else {Arc::new(DAE::Exp::MATCHEXPRESSION { matchType: match_ty.clone(), inputs: expl_1.clone(), aliases: aliases.clone(), localDecls: match_decls.clone(), cases: match_cases.clone(), et: ty.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::BOX { exp: e1 } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::BOX { exp: e1_1.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::UNBOX { exp: e1, ty } => {
            let mut e1_1: Arc<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1.clone()))) {inExp} else {Arc::new(DAE::Exp::UNBOX { exp: e1_1.clone(), ty: ty.clone() })}, arg.clone())
        },
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => {
            (inExp, inArg)
        },
        Deref @ DAE::Exp::PATTERN { .. } => {
            (inExp, inArg)
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.traverseExpBidirSubExps")); __mm_s.push_str(&*literal!(" - Unknown expression ")); __mm_s.push_str(&*printExpStr(inExp)?); __mm_s.push_str(&*literal!(". Called using: ")); __mm_s.push_str(&*(System::dladdr(inEnterFunc.clone())).0); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*(System::dladdr(inExitFunc.clone())).0); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!("FrontEnd/Expression.mo"))?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outArg))
}

pub(crate) fn traverseExpBidirCref<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut inCref: Arc<DAE::ComponentRef>, mut inEnterFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inExitFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(Arc<DAE::ComponentRef>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>;

    let mut outCref: Arc<DAE::ComponentRef>;
    let mut outArg: ArgT;
    (outCref, outArg) = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType: ty, subscriptLst: subs, componentRef: cr } => {
            let mut arg: ArgT;
            let mut subs = (*subs).clone();
            let mut cr = (*cr).clone();
            (subs, arg) = List::map2Fold(subs.clone(), (std::sync::Arc::new(traverseExpBidirSubs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>, _, _, _) -> Result<_> + 'static>), inEnterFunc.clone(), inExitFunc.clone(), inArg, metamodelica::nil())?;
            (cr, arg) = traverseExpBidirCref(cr.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg.clone())?;
            (Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: subs.clone(), componentRef: cr.clone() }), arg.clone())
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, identType: ty, subscriptLst: subs } => {
            let mut arg: ArgT;
            let mut subs = (*subs).clone();
            (subs, arg) = List::map2Fold(subs.clone(), (std::sync::Arc::new(traverseExpBidirSubs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>, _, _, _) -> Result<_> + 'static>), inEnterFunc.clone(), inExitFunc.clone(), inArg, metamodelica::nil())?;
            (Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: subs.clone() }), arg.clone())
        },
        _ => {
            (inCref, inArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCref, outArg))
}

pub(crate) fn traverseExpCref<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inCref: Arc<DAE::ComponentRef>, mut rel: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut iarg: Type_a) -> Result<(Arc<DAE::ComponentRef>, Type_a)> {
    pub type FuncType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outCref: Arc<DAE::ComponentRef>;
    let mut outArg: Type_a;
    (outCref, outArg) = (::match_deref::match_deref! { match &((inCref.clone(), iarg)) {
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType: ty, subscriptLst: subs, componentRef: cr }, arg) => {
            let mut cr_1: ComponentRef;
            let mut subs_1: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut cr = (*cr).clone();
            let mut arg = (*arg).clone();
            (subs_1, arg) = traverseExpSubs(subs.clone(), rel.clone(), arg.clone())?;
            (cr_1, arg) = traverseExpCref(cr.clone(), rel.clone(), arg.clone())?;
            cr = if (referenceEq(&*(cr.clone()),&*(cr_1.clone())) && metamodelica::ReferenceEq::reference_eq(&*(subs.clone()), &*(subs_1.clone()))) {inCref} else {Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: subs_1.clone(), componentRef: cr_1.clone() })};
            (cr.clone(), arg.clone())
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, identType: ty, subscriptLst: subs }, arg) => {
            let mut cr: ComponentRef;
            let mut subs_1: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut arg = (*arg).clone();
            (subs_1, arg) = traverseExpSubs(subs.clone(), rel.clone(), arg.clone())?;
            cr = if (metamodelica::ReferenceEq::reference_eq(&*(subs.clone()), &*(subs_1.clone()))) {inCref} else {Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: subs_1.clone() })};
            (cr.clone(), arg.clone())
        },
        (Deref @ DAE::ComponentRef::OPTIMICA_ATTR_INST_CREF { componentRef: cr, instant }, arg) => {
            let mut cr_1: ComponentRef;
            let mut cr = (*cr).clone();
            let mut arg = (*arg).clone();
            (cr_1, arg) = traverseExpCref(cr.clone(), rel.clone(), arg.clone())?;
            cr = if (referenceEq(&*(cr.clone()),&*(cr_1.clone()))) {inCref} else {Arc::new(DAE::ComponentRef::OPTIMICA_ATTR_INST_CREF { componentRef: cr_1.clone(), instant: (instant.clone()).clone() })};
            (cr.clone(), arg.clone())
        },
        (Deref @ DAE::ComponentRef::WILD { .. }, arg) => {
            (inCref, arg.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Expression.traverseExpCref: Unknown cref")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCref, outArg))
}

fn traverseExpSubs<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inSubscript: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut rel: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut iarg: Type_a) -> Result<(Arc<metamodelica::List<Arc<DAE::Subscript>>>, Type_a)> {
    pub type FuncType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outSubscript: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    let mut outArg: Type_a;
    (outSubscript, outArg) = (::match_deref::match_deref! { match &((inSubscript.clone(), iarg)) {
        (Deref @ metamodelica::List::Nil, arg) => {
            (inSubscript, arg.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: rest }, arg) => {
            let mut res: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut arg = (*arg).clone();
            (res, arg) = traverseExpSubs(rest.clone(), rel.clone(), arg.clone())?;
            res = if (metamodelica::ReferenceEq::reference_eq(&*(rest.clone()), &*(res.clone()))) {inSubscript} else {metamodelica::cons(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), res.clone())};
            (res.clone(), arg.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: sub_exp }, tail: rest }, arg) => {
            let mut sub_exp_1: Arc<DAE::Exp>;
            let mut res: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut arg = (*arg).clone();
            (sub_exp_1, arg) = traverseExpBottomUp(sub_exp.clone(), rel.clone(), arg.clone())?;
            (res, arg) = traverseExpSubs(rest.clone(), rel.clone(), arg.clone())?;
            res = if (referenceEq(&*(sub_exp.clone()),&*(sub_exp_1.clone())) && metamodelica::ReferenceEq::reference_eq(&*(rest.clone()), &*(res.clone()))) {inSubscript} else {metamodelica::cons(Arc::new(DAE::Subscript::SLICE { exp: sub_exp_1.clone() }), res.clone())};
            (res.clone(), arg.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: sub_exp }, tail: rest }, arg) => {
            let mut sub_exp_1: Arc<DAE::Exp>;
            let mut res: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut arg = (*arg).clone();
            (sub_exp_1, arg) = traverseExpBottomUp(sub_exp.clone(), rel.clone(), arg.clone())?;
            (res, arg) = traverseExpSubs(rest.clone(), rel.clone(), arg.clone())?;
            res = if (referenceEq(&*(sub_exp.clone()),&*(sub_exp_1.clone())) && metamodelica::ReferenceEq::reference_eq(&*(rest.clone()), &*(res.clone()))) {inSubscript} else {metamodelica::cons(Arc::new(DAE::Subscript::INDEX { exp: sub_exp_1.clone() }), res.clone())};
            (res.clone(), arg.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLE_NONEXP { exp: sub_exp }, tail: rest }, arg) => {
            let mut sub_exp_1: Arc<DAE::Exp>;
            let mut res: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut arg = (*arg).clone();
            (sub_exp_1, arg) = traverseExpBottomUp(sub_exp.clone(), rel.clone(), arg.clone())?;
            (res, arg) = traverseExpSubs(rest.clone(), rel.clone(), arg.clone())?;
            res = if (referenceEq(&*(sub_exp.clone()),&*(sub_exp_1.clone())) && metamodelica::ReferenceEq::reference_eq(&*(rest.clone()), &*(res.clone()))) {inSubscript} else {metamodelica::cons(Arc::new(DAE::Subscript::WHOLE_NONEXP { exp: sub_exp_1.clone() }), res.clone())};
            (res.clone(), arg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outSubscript, outArg))
}

pub fn traverseExpTopDownCrefHelper<Argument: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inCref: Arc<DAE::ComponentRef>, mut rel: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Argument) -> Result<(Arc<DAE::Exp>, bool, Argument)> + 'static>, mut iarg: Argument) -> Result<(Arc<DAE::ComponentRef>, Argument)> {
    pub type FuncType<Argument: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Argument) -> Result<(Arc<DAE::Exp>, bool, Argument)> + 'static>;

    let mut outCref: Arc<DAE::ComponentRef>;
    let mut outArg: Argument;
    (outCref, outArg) = (::match_deref::match_deref! { match &((inCref.clone(), iarg)) {
        (Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, identType: ty, subscriptLst: subs, componentRef: cr }, arg) => {
            let mut cr_1: ComponentRef;
            let mut subs_1: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut arg = (*arg).clone();
            (subs_1, arg) = traverseExpTopDownSubs(subs.clone(), rel.clone(), arg.clone())?;
            (cr_1, arg) = traverseExpTopDownCrefHelper(cr.clone(), rel.clone(), arg.clone())?;
            (if (metamodelica::ReferenceEq::reference_eq(&*(subs.clone()), &*(subs_1.clone())) && referenceEq(&*(cr.clone()),&*(cr_1.clone()))) {inCref} else {Arc::new(DAE::ComponentRef::CREF_QUAL { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: subs_1.clone(), componentRef: cr_1.clone() })}, arg.clone())
        },
        (Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, identType: ty, subscriptLst: subs }, arg) => {
            let mut subs_1: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
            let mut arg = (*arg).clone();
            (subs_1, arg) = traverseExpTopDownSubs(subs.clone(), rel.clone(), arg.clone())?;
            (if (metamodelica::ReferenceEq::reference_eq(&*(subs.clone()), &*(subs_1.clone()))) {inCref} else {Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (name.clone()).clone(), identType: ty.clone(), subscriptLst: subs_1.clone() })}, arg.clone())
        },
        (Deref @ DAE::ComponentRef::WILD { .. }, arg) => {
            (inCref, arg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCref, outArg))
}

fn traverseExpBidirSubs<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut inSubscript: Arc<DAE::Subscript>, mut inEnterFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inExitFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(Arc<DAE::Subscript>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>;

    let mut outSubscript: Arc<DAE::Subscript>;
    let mut outArg: ArgT;
    (outSubscript, outArg) = (::match_deref::match_deref! { match &(inSubscript.clone()) {
        Deref @ DAE::Subscript::WHOLEDIM { .. } => {
            (inSubscript, inArg)
        },
        Deref @ DAE::Subscript::SLICE { exp: sub_exp } => {
            let mut arg: ArgT;
            let mut sub_exp = (*sub_exp).clone();
            (sub_exp, arg) = traverseExpBidir(sub_exp.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (Arc::new(DAE::Subscript::SLICE { exp: sub_exp.clone() }), arg.clone())
        },
        Deref @ DAE::Subscript::INDEX { exp: sub_exp } => {
            let mut arg: ArgT;
            let mut sub_exp = (*sub_exp).clone();
            (sub_exp, arg) = traverseExpBidir(sub_exp.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (Arc::new(DAE::Subscript::INDEX { exp: sub_exp.clone() }), arg.clone())
        },
        Deref @ DAE::Subscript::WHOLE_NONEXP { exp: sub_exp } => {
            let mut arg: ArgT;
            let mut sub_exp = (*sub_exp).clone();
            (sub_exp, arg) = traverseExpBidir(sub_exp.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (Arc::new(DAE::Subscript::WHOLE_NONEXP { exp: sub_exp.clone() }), arg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outSubscript, outArg))
}

pub fn traverseExpTopDownSubs<Argument: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inSubscript: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut rel: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Argument) -> Result<(Arc<DAE::Exp>, bool, Argument)> + 'static>, mut iarg: Argument) -> Result<(Arc<metamodelica::List<Arc<DAE::Subscript>>>, Argument)> {
    pub type FuncType<Argument: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Argument) -> Result<(Arc<DAE::Exp>, bool, Argument)> + 'static>;

    let mut outSubscript: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    let mut arg: Argument = iarg.clone();
    let mut exp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut nsub: Arc<DAE::Subscript>;
    let mut allEq: bool = true;
    let mut delst: DoubleEnded::MutableList<Arc<DAE::Subscript>> = <DoubleEnded::MutableList<Arc<DAE::Subscript>> as ::std::default::Default>::default();
    let mut nEq: i32 = 0;
    for mut sub in &*inSubscript.clone() {
        let mut sub = sub.clone();
        nsub = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ DAE::Subscript::WHOLEDIM { .. } => sub.clone(),
        Deref @ DAE::Subscript::SLICE { .. } => {
            (exp, arg) = traverseExpTopDown(var_field!((*sub).exp, DAE::Subscript::SLICE).clone(), rel.clone(), arg.clone())?;
            if (referenceEq(&*(var_field!((*sub).exp, DAE::Subscript::SLICE).clone()),&*(exp.clone()))) {sub.clone()} else {Arc::new(DAE::Subscript::SLICE { exp: exp.clone() })}
        },
        Deref @ DAE::Subscript::INDEX { .. } => {
            (exp, arg) = traverseExpTopDown(var_field!((*sub).exp, DAE::Subscript::INDEX).clone(), rel.clone(), arg.clone())?;
            if (referenceEq(&*(var_field!((*sub).exp, DAE::Subscript::INDEX).clone()),&*(exp.clone()))) {sub.clone()} else {Arc::new(DAE::Subscript::INDEX { exp: exp.clone() })}
        },
        Deref @ DAE::Subscript::WHOLE_NONEXP { .. } => {
            (exp, arg) = traverseExpTopDown(var_field!((*sub).exp, DAE::Subscript::WHOLE_NONEXP).clone(), rel.clone(), arg.clone())?;
            if (referenceEq(&*(var_field!((*sub).exp, DAE::Subscript::WHOLE_NONEXP).clone()),&*(exp.clone()))) {sub.clone()} else {Arc::new(DAE::Subscript::WHOLE_NONEXP { exp: exp.clone() })}
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if if (allEq) {!(referenceEq(&*(nsub.clone()),&*(sub.clone())))} else {false} {
            allEq = false;
            delst = DoubleEnded::empty(nsub.clone());
            for mut elt in &*inSubscript.clone() {
                let mut elt = elt.clone();
                if nEq < 1 {
                    break;
                }
                DoubleEnded::push_back(delst.clone(), elt.clone())?;
                nEq = nEq - 1;
            }
        }
        if allEq {
            nEq = nEq + 1;
        } else {
            DoubleEnded::push_back(delst.clone(), nsub.clone())?;
        }
    }
    outSubscript = if (allEq) {inSubscript} else {DoubleEnded::toListAndClear(delst, metamodelica::nil())?};
    Ok((outSubscript, arg))
}

/* **************************************************/
/* Compare and Check DAE.Exp */
/* **************************************************/
pub fn operatorDivOrMul(mut op: DAE::Operator) -> bool {
    let mut res: bool;
    res = (match op {
        DAE::Operator::MUL { ty: _ } => true,
        DAE::Operator::DIV { ty: _ } => true,
        _ => false,
    });
    res
}

pub fn isRange(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::RANGE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isReduction(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::REDUCTION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isOne(mut inExp: Arc<DAE::Exp>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::ICONST { integer: ival } => {
            return intEq(ival.clone(), 1)
        },
        Deref @ DAE::Exp::RCONST { real: rval } => {
            return realEq(rval.clone(), metamodelica::OrderedFloat(1.0_f64))
        },
        Deref @ DAE::Exp::CAST { exp: e, .. } => {
            let mut res: bool;
            { inExp = e.clone(); continue '__tco; }
        },
        _ => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn isZero(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::ICONST { integer: ival } => {
            return Ok(intEq(ival.clone(), 0))
        },
        Deref @ DAE::Exp::RCONST { real: rval } => {
            return Ok(realEq(rval.clone(), metamodelica::OrderedFloat(0.0_f64)))
        },
        Deref @ DAE::Exp::CAST { exp: e, .. } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::ARRAY { array: ae, .. } => {
            return Ok(List::all(ae.clone(), (std::sync::Arc::new(isZero) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?)
        },
        Deref @ DAE::Exp::MATRIX { matrix, .. } => {
            return Ok(List::all(matrix.clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static> = (std::sync::Arc::new(isZero) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>); move |__pe_a0| List::all(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static>))?)
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e } => {
            { inExp = e.clone(); continue '__tco; }
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn isZeroOrAlmostZero(mut inExp: Arc<DAE::Exp>, mut nominal: Arc<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExp, nominal.clone())) {
        (Deref @ DAE::Exp::ICONST { integer: ival }, _) => {
            return Ok(intEq(ival.clone(), 0))
        },
        (Deref @ DAE::Exp::RCONST { real: rval }, Deref @ DAE::Exp::RCONST { real: rNom }) => {
            return Ok(realLt(rval.clone().abs(), metamodelica::OrderedFloat(1e-6_f64) * rNom.clone().abs()))
        },
        (Deref @ DAE::Exp::RCONST { real: rval }, _) => {
            return Ok(realLt(rval.clone().abs(), metamodelica::OrderedFloat(1e-6_f64)))
        },
        (Deref @ DAE::Exp::CAST { exp: e, .. }, _) => {
            { (inExp, nominal) = (e.clone(), nominal); continue '__tco; }
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e }, _) => {
            { (inExp, nominal) = (e.clone(), nominal); continue '__tco; }
        },
        (Deref @ DAE::Exp::ARRAY { array: ae, .. }, _) => {
            return Ok(List::all(ae.clone(), (std::sync::Arc::new({ let __pe_b1 = nominal; move |__pe_a0| isZeroOrAlmostZero(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?)
        },
        (Deref @ DAE::Exp::MATRIX { matrix, .. }, _) => {
            return Ok(List::all(matrix.clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static> = (std::sync::Arc::new({ let __pe_b1 = nominal; move |__pe_a0| isZeroOrAlmostZero(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>); move |__pe_a0| List::all(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static>))?)
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e }, _) => {
            { (inExp, nominal) = (e.clone(), nominal); continue '__tco; }
        },
        (Deref @ DAE::Exp::IFEXP { expCond: _, expThen: e, expElse: e1 }, _) => {
            return Ok(isZeroOrAlmostZero(e.clone(), nominal.clone())? || isZeroOrAlmostZero(e1.clone(), nominal)?)
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn isPositiveOrZero(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ICONST { integer: i } => {
            return Ok(i.clone() >= 0)
        },
        Deref @ DAE::Exp::RCONST { real: r } => {
            return Ok(r.clone() >= metamodelica::OrderedFloat(0.0_f64))
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } => {
            return Ok(isPositiveOrZero(e1.clone())? && isPositiveOrZero(e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
            return Ok(isPositiveOrZero(e1.clone())? && isNegativeOrZero(e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
            return Ok(isPositiveOrZero(e1.clone())? && isPositiveOrZero(e2.clone())? || isNegativeOrZero(e1.clone())? && isNegativeOrZero(e2.clone())? || ExpressionBasics::expEqual(e1.clone(), e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } => {
            return Ok(isPositiveOrZero(e1.clone())? && isPositiveOrZero(e2.clone())? || isNegativeOrZero(e1.clone())? && isNegativeOrZero(e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: _ } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::BINARY { exp1: _, operator: DAE::Operator::POW { .. }, exp2: e2 } => {
            return Ok(isEven(e2.clone()))
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 } => {
            return Ok(isNegativeOrZero(e1.clone())?)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, .. } => {
            return Ok(true)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. } => {
            return Ok(true)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. } => {
            return Ok(true)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        _ => {
            return Ok(isZero(inExp)?)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn isNegativeOrZero(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ICONST { integer: i } => {
            return Ok(i.clone() <= 0)
        },
        Deref @ DAE::Exp::RCONST { real: r } => {
            return Ok(r.clone() <= metamodelica::OrderedFloat(0.0_f64))
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } => {
            return Ok(isNegativeOrZero(e1.clone())? && isNegativeOrZero(e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
            return Ok(isNegativeOrZero(e1.clone())? && isPositiveOrZero(e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
            return Ok(isPositiveOrZero(e1.clone())? && isNegativeOrZero(e2.clone())? || isNegativeOrZero(e1.clone())? && isPositiveOrZero(e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } => {
            return Ok(isPositiveOrZero(e1.clone())? && isNegativeOrZero(e2.clone())? || isNegativeOrZero(e1.clone())? && isPositiveOrZero(e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: e2 } => {
            return Ok(isNegativeOrZero(e1.clone())? && isOdd(e2.clone()))
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 } => {
            return Ok(isPositiveOrZero(e1.clone())?)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            return Ok(isZero(e1.clone())?)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. } => {
            return Ok(false)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. } => {
            return Ok(false)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        _ => {
            return Ok(isZero(inExp)?)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn isPositive(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::ICONST { integer: i } => {
            return Ok(i.clone() > 0)
        },
        Deref @ DAE::Exp::RCONST { real: r } => {
            return Ok(r.clone() > metamodelica::OrderedFloat(0.0_f64))
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } => {
            return Ok(isPositive(e1.clone())? && isPositiveOrZero(e2.clone())? || isZero(e1.clone())? && isPositive(e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
            return Ok(isPositive(e1.clone())? && isNegativeOrZero(e2.clone())? || isZero(e1.clone())? && isNegative(e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
            return Ok(isPositive(e1.clone())? && isPositive(e2.clone())? || isNegative(e1.clone())? && isNegative(e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } => {
            return Ok(isPositive(e1.clone())? && isPositive(e2.clone())? || isNegative(e1.clone())? && isNegative(e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: _ } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 } => {
            return Ok(isNegative(e1.clone())?)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            return Ok(isPositive(e1.clone())? || isNegative(e1.clone())?)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. } => {
            return Ok(true)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. } => {
            return Ok(true)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn isNegative(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::ICONST { integer: i } => {
            return Ok(i.clone() < 0)
        },
        Deref @ DAE::Exp::RCONST { real: r } => {
            return Ok(r.clone() < metamodelica::OrderedFloat(0.0_f64))
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } => {
            return Ok(isNegative(e1.clone())? && isNegativeOrZero(e2.clone())? || isZero(e1.clone())? && isNegative(e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
            return Ok(isNegative(e1.clone())? && isPositiveOrZero(e2.clone())? || isZero(e1.clone())? && isPositive(e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
            return Ok(isPositive(e1.clone())? && isNegative(e2.clone())? || isNegative(e1.clone())? && isPositive(e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } => {
            return Ok(isPositive(e1.clone())? && isNegative(e2.clone())? || isNegative(e1.clone())? && isPositive(e2.clone())?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: e2 } => {
            return Ok(isNegative(e1.clone())? && isOdd(e2.clone()))
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, .. } => {
            return Ok(false)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. } => {
            return Ok(false)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. } => {
            return Ok(false)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. } => {
            { inExp = e1.clone(); continue '__tco; }
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn isGreaterOrEqual(mut exp1: Arc<DAE::Exp>, mut exp2: Arc<DAE::Exp>) -> Result<bool> {
    let mut isGreaterOrEqual: bool = isPositiveOrZero((ExpressionSimplify::simplify(expSub(exp1.clone(), exp2.clone())?)?).0)?;
    Ok(isGreaterOrEqual)
}

pub fn isHalf(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::RCONST { real: rval } => {
            realEq(rval.clone(), metamodelica::OrderedFloat(0.5_f64))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isAtomic(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { .. } => true,
        Deref @ DAE::Exp::CALL { .. } => true,
        Deref @ DAE::Exp::ICONST { .. } => var_field!((*inExp).integer, DAE::Exp::ICONST).clone() >= 0,
        Deref @ DAE::Exp::RCONST { .. } => var_field!((*inExp).real, DAE::Exp::RCONST).clone() > metamodelica::OrderedFloat(0.0_f64),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isImpure(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = isConst(inExp.clone())?;
    (_, outBoolean) = traverseExpTopDown(inExp, (std::sync::Arc::new(fnptr!(isImpureWork, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
    Ok(outBoolean)
}

fn isImpureWork(mut inExp: Arc<DAE::Exp>, mut isImpure: bool) -> (Arc<DAE::Exp>, bool, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool;
    let mut outImpure: bool;
    (outExp, cont, outImpure) = (::match_deref::match_deref! { match &((inExp.clone(), isImpure)) {
        (_, true) => (inExp, true, true),
        (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { isImpure: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "alarm" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "compareFilesAndMove" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "print" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "readFile" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "system" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "system_parallel" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "terminal" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "writeFile" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        _ => (inExp, true, false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outImpure)
}

pub fn containsRecordType(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut isRec: bool;
    (_, isRec) = traverseExpTopDown(inExp, (std::sync::Arc::new(fnptr!(containsRecordTypeWork, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
    Ok(isRec)
}

fn containsRecordTypeWork(mut inExp: Arc<DAE::Exp>, mut inRec: bool) -> (Arc<DAE::Exp>, bool, bool) {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut cont: bool = false;
    let mut outRec: bool = inRec;
    if !(inRec) {
        (outExp, cont, outRec) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RECORD { .. } => {
                    Ok((inExp.clone(), false, true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst, attr: Deref @ DAE::CallAttributes { ty, .. }, .. } => {
                    let mut subRec: bool;
                    subRec = isRecordType(ty.clone());
                    if !(subRec.clone()) {
                        for mut exp in &*expLst.clone() {
                            let mut exp = exp.clone();
                            subRec = containsRecordType(exp.clone())?;
                            if subRec.clone() {
                                        break;
                            }
                        }
                    }
                    Ok((inExp.clone(), !(subRec.clone()), subRec.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    }
    (outExp, cont, outRec)
}

pub fn isEvaluatedConst(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::ICONST { .. } => true,
        Deref @ DAE::Exp::RCONST { .. } => true,
        Deref @ DAE::Exp::BCONST { .. } => true,
        Deref @ DAE::Exp::SCONST { .. } => true,
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn getEvaluatedConstInteger(mut inExp: Arc<DAE::Exp>) -> Result<i32> {
    let mut val: i32;
    val = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ICONST { integer } => {
            integer.clone()
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            let mut integer: i32;
            let __pa0 = ::match_deref::match_deref! { match &(realExpIntLit(inExp)) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            integer = __pa0.clone();
            integer.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(val)
}

pub fn getEvaluatedConstReal(mut inExp: Arc<DAE::Exp>) -> Result<metamodelica::Real> {
    let mut val: metamodelica::Real;
    val = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::RCONST { .. } => {
            var_field!((*inExp).real, DAE::Exp::RCONST).clone()
        },
        Deref @ DAE::Exp::ICONST { .. } => {
            intReal(var_field!((*inExp).integer, DAE::Exp::ICONST).clone())
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(val)
}

pub fn isConst(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::ICONST { .. } => {
            return Ok(true)
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            return Ok(true)
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            return Ok(true)
        },
        Deref @ DAE::Exp::SCONST { .. } => {
            return Ok(true)
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            return Ok(true)
        },
        Deref @ DAE::Exp::UNARY { exp: e, .. } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::CAST { exp: e, .. } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: _, exp2: e2 } => {
            let mut res: bool;
            res = isConst(e2.clone())?;
            if (res.clone()) {{ inExp = e1.clone(); continue '__tco; }} else {return Ok(false)}
        },
        Deref @ DAE::Exp::IFEXP { expCond: e, expThen: e1, expElse: e2 } => {
            let mut res: bool;
            res = isConst(e2.clone())?;
            if res.clone() {
                res = isConst(e1.clone())?;
            }
            if (res.clone()) {{ inExp = e.clone(); continue '__tco; }} else {return Ok(false)}
        },
        Deref @ DAE::Exp::LBINARY { exp1: e1, exp2: e2, .. } => {
            let mut res: bool;
            res = isConst(e2.clone())?;
            if (res.clone()) {{ inExp = e1.clone(); continue '__tco; }} else {return Ok(false)}
        },
        Deref @ DAE::Exp::LUNARY { exp: e, .. } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::RELATION { exp1: e1, exp2: e2, .. } => {
            let mut res: bool;
            res = isConst(e2.clone())?;
            if (res.clone()) {{ inExp = e1.clone(); continue '__tco; }} else {return Ok(false)}
        },
        Deref @ DAE::Exp::ARRAY { array: ae, .. } => {
            return Ok(isConstWorkList(ae.clone())?)
        },
        Deref @ DAE::Exp::MATRIX { matrix, .. } => {
            return Ok(isConstWorkListList(matrix.clone())?)
        },
        Deref @ DAE::Exp::RANGE { start: e1, step: None, stop: e2, .. } => {
            let mut res: bool;
            res = isConst(e2.clone())?;
            if (res.clone()) {{ inExp = e1.clone(); continue '__tco; }} else {return Ok(false)}
        },
        Deref @ DAE::Exp::RANGE { start: e, step: Some(e1), stop: e2, .. } => {
            let mut res: bool;
            res = isConst(e2.clone())?;
            if res.clone() {
                res = isConst(e1.clone())?;
            }
            if (res.clone()) {{ inExp = e.clone(); continue '__tco; }} else {return Ok(false)}
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { expList: ae, .. } => {
            return Ok(isConstWorkList(ae.clone())?)
        },
        Deref @ DAE::Exp::TUPLE { PR: ae } => {
            return Ok(isConstWorkList(ae.clone())?)
        },
        Deref @ DAE::Exp::ASUB { exp: e, sub: subs } => {
            let mut res: bool;
            let mut ae: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            ae = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = getSubscriptExp(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            res = isConst(e.clone())?;
            if (res.clone()) {return Ok(isConstWorkList(ae.clone())?)} else {return Ok(false)}
        },
        Deref @ DAE::Exp::TSUB { exp: e, .. } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::SIZE { exp: e, sz: None } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::SIZE { exp: e1, sz: Some(e2) } => {
            let mut res: bool;
            res = isConst(e2.clone())?;
            if (res.clone()) {{ inExp = e1.clone(); continue '__tco; }} else {return Ok(false)}
        },
        Deref @ DAE::Exp::CALL { expLst: ae, attr: Deref @ DAE::CallAttributes { builtin: false, isImpure: false, .. }, .. } => {
            return Ok(isConstWorkList(ae.clone())?)
        },
        Deref @ DAE::Exp::CALL { path, expLst: ae, attr: Deref @ DAE::CallAttributes { builtin: true, .. } } => {
            if (listMember((AbsynUtil::pathFirstIdent(path.clone())?).clone(), list![(literal!("initial")).clone(), (literal!("terminal")).clone(), (literal!("sample")).clone()])) {return Ok(false)} else {return Ok(isConstWorkList(ae.clone())?)}
        },
        Deref @ DAE::Exp::RECORD { exps: ae, .. } => {
            return Ok(isConstWorkList(ae.clone())?)
        },
        Deref @ DAE::Exp::REDUCTION { expr: e1, iterators: Deref @ metamodelica::List::Cons { head: Deref @ DAE::ReductionIterator { exp: e2, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut res: bool;
            res = isConst(e2.clone())?;
            if (res.clone()) {{ inExp = e1.clone(); continue '__tco; }} else {return Ok(false)}
        },
        Deref @ DAE::Exp::BOX { exp: e } => {
            { inExp = e.clone(); continue '__tco; }
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn isConstValueWork(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::ICONST { .. } => {
            true
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            true
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            true
        },
        Deref @ DAE::Exp::SCONST { .. } => {
            true
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            true
        },
        Deref @ DAE::Exp::ARRAY { array: ae, .. } => {
            isConstValueWorkList(ae.clone())?
        },
        Deref @ DAE::Exp::MATRIX { matrix, .. } => {
            isConstValueWorkListList(matrix.clone())?
        },
        Deref @ DAE::Exp::RECORD { .. } => {
            true
        },
        Deref @ DAE::Exp::METARECORDCALL { .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

pub fn isConstValue(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = isConstValueWork(inExp)?;
    Ok(outBoolean)
}

pub fn isConstWorkList(mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<bool> {
    let mut outBoolean: bool;
    let mut e: Arc<DAE::Exp>;
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut b: bool = true;
    exps = inExps;
    while b && !(exps.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exps.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        exps = __pa1.clone();
        b = isConst(e.clone())?;
    }
    outBoolean = b;
    Ok(outBoolean)
}

fn isConstWorkListList(mut inExps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>) -> Result<bool> {
    let mut outIsConst: bool;
    let mut e: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut exps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
    let mut b: bool = true;
    exps = inExps;
    while b && !(exps.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exps.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        exps = __pa1.clone();
        b = isConstWorkList(e.clone())?;
    }
    outIsConst = b;
    Ok(outIsConst)
}

fn isConstValueWorkList(mut inExps: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<bool> {
    let mut outBoolean: bool;
    let mut e: Arc<DAE::Exp>;
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut b: bool = true;
    exps = inExps;
    while b && !(exps.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exps.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        exps = __pa1.clone();
        b = isConstValueWork(e.clone())?;
    }
    outBoolean = b;
    Ok(outBoolean)
}

fn isConstValueWorkListList(mut inExps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>) -> Result<bool> {
    let mut outIsConst: bool;
    let mut e: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut exps: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
    let mut b: bool = true;
    exps = inExps;
    while b && !(exps.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exps.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        exps = __pa1.clone();
        b = isConstValueWorkList(e.clone())?;
    }
    outIsConst = b;
    Ok(outIsConst)
}

pub fn isNotConst(mut e: Arc<DAE::Exp>) -> Result<bool> {
    let mut nb: bool;
    let mut b: bool;
    b = isConst(e)?;
    nb = boolNot(b);
    Ok(nb)
}

pub fn isRelation(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::RELATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn isEventTriggeringFunctionExp(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" }, .. } => true,
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" }, .. } => true,
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "rem" }, .. } => true,
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" }, .. } => true,
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" }, .. } => true,
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub fn isAddOrSub(mut op: DAE::Operator) -> bool {
    let mut res: bool;
    res = isAdd(op.clone()) || isSub(op);
    res
}

pub fn isAdd(mut op: DAE::Operator) -> bool {
    let mut res: bool;
    res = (match op {
        DAE::Operator::ADD { .. } => true,
        DAE::Operator::ADD_ARR { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isSub(mut op: DAE::Operator) -> bool {
    let mut res: bool;
    res = (match op {
        DAE::Operator::SUB { .. } => true,
        DAE::Operator::SUB_ARR { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isAddOrSubBinary(mut iExp: Arc<DAE::Exp>) -> bool {
    let mut res: bool;
    let mut op: DAE::Operator = <DAE::Operator as ::std::default::Default>::default();
    res = (::match_deref::match_deref! { match &(iExp) {
        Deref @ DAE::Exp::BINARY { exp1: _, operator: __esc_op, exp2: _ } => {
            op = (*__esc_op).clone();
            isAddOrSub(op.clone())
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isMulOrDiv(mut op: DAE::Operator) -> bool {
    let mut res: bool = isMul(op.clone()) || isDiv(op.clone());
    res
}

pub(crate) fn isMul(mut op: DAE::Operator) -> bool {
    let mut res: bool;
    res = (match op {
        DAE::Operator::MUL { .. } => true,
        DAE::Operator::MUL_ARR { .. } => true,
        _ => false,
    });
    res
}

pub fn isDiv(mut op: DAE::Operator) -> bool {
    let mut res: bool;
    res = (match op {
        DAE::Operator::DIV { .. } => true,
        DAE::Operator::DIV_ARR { .. } => true,
        _ => false,
    });
    res
}

pub fn isDivBinary(mut iExp: Arc<DAE::Exp>) -> bool {
    let mut res: bool;
    let mut op: DAE::Operator = <DAE::Operator as ::std::default::Default>::default();
    res = (::match_deref::match_deref! { match &(iExp) {
        Deref @ DAE::Exp::BINARY { exp1: _, operator: __esc_op, exp2: _ } => {
            op = (*__esc_op).clone();
            isDiv(op.clone())
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub(crate) fn isMulorDivBinary(mut iExp: Arc<DAE::Exp>) -> bool {
    let mut res: bool;
    let mut op: DAE::Operator = <DAE::Operator as ::std::default::Default>::default();
    res = (::match_deref::match_deref! { match &(iExp) {
        Deref @ DAE::Exp::BINARY { exp1: _, operator: __esc_op, exp2: _ } => {
            op = (*__esc_op).clone();
            isMulOrDiv(op.clone())
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isPow(mut op: DAE::Operator) -> bool {
    let mut res: bool;
    res = (match op {
        DAE::Operator::POW { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isFunCall(mut iExp: Arc<DAE::Exp>, mut name: ArcStr) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(iExp) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: name_ }, .. } => {
            name_.clone() == name
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn equalTypes(mut t1: Arc<DAE::Type>, mut t2: Arc<DAE::Type>) -> bool {
    let mut b: bool;
    b = 'mc: {
        let __mc_input = (t1, t2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_INTEGER { .. }, Deref @ DAE::Type::T_INTEGER { .. }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_REAL { .. }, Deref @ DAE::Type::T_REAL { .. }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_STRING { .. }, Deref @ DAE::Type::T_STRING { .. }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_BOOL { .. }, Deref @ DAE::Type::T_BOOL { .. }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_CLOCK { .. }, Deref @ DAE::Type::T_CLOCK { .. }) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { varLst: vars1, .. }, Deref @ DAE::Type::T_COMPLEX { varLst: vars2, .. }) => {
                    Ok(equalTypesComplexVars(vars1.clone(), vars2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: ty1, dims: ad1 }, Deref @ DAE::Type::T_ARRAY { ty: ty2, dims: ad2 }) => {
                    let mut li1: Arc<metamodelica::List<i32>>;
                    let mut li2: Arc<metamodelica::List<i32>>;
                    li1 = List::map(ad1.clone(), (std::sync::Arc::new(dimensionSize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<i32> + 'static>))?;
                    li2 = List::map(ad2.clone(), (std::sync::Arc::new(dimensionSize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<i32> + 'static>))?;
                    let true = (List::isEqualOnTrue(li1.clone(), li2.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    let true = (equalTypes(ty1.clone(), ty2.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    b
}

fn equalTypesComplexVars(mut inVars1: Arc<metamodelica::List<Arc<DAE::Var>>>, mut inVars2: Arc<metamodelica::List<Arc<DAE::Var>>>) -> bool {
    let mut b: bool;
    b = 'mc: {
        let __mc_input = (inVars1, inVars2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { name: s1, ty: t1, .. }, tail: vars1 }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { name: s2, ty: t2, .. }, tail: vars2 }) => {
                    let true = (stringEq((s1.clone()).clone(), (s2.clone()).clone())) else { bail!("pattern mismatch") };
                    let true = (equalTypes(t1.clone(), t2.clone())) else { bail!("pattern mismatch") };
                    Ok(equalTypesComplexVars(vars1.clone(), vars2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    b
}

pub fn typeBuiltin(mut inType: Arc<DAE::Type>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inType) {
        Deref @ DAE::Type::T_INTEGER { .. } => true,
        Deref @ DAE::Type::T_REAL { .. } => true,
        Deref @ DAE::Type::T_STRING { .. } => true,
        Deref @ DAE::Type::T_BOOL { .. } => true,
        Deref @ DAE::Type::T_CLOCK { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn isWholeDim(mut s: Arc<DAE::Subscript>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(s) {
        Deref @ DAE::Subscript::WHOLEDIM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isInt(mut it: Arc<DAE::Type>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(it) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            return true
        },
        Deref @ DAE::Type::T_ARRAY { ty: t1, .. } => {
            { it = t1.clone(); continue '__tco; }
        },
        _ => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub(crate) fn isReal(mut it: Arc<DAE::Type>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(it) {
        Deref @ DAE::Type::T_REAL { .. } => {
            return true
        },
        Deref @ DAE::Type::T_ARRAY { ty: t1, .. } => {
            { it = t1.clone(); continue '__tco; }
        },
        _ => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub(crate) fn isExpReal(mut e: Arc<DAE::Exp>) -> Result<bool> {
    let mut re: bool;
    re = isReal(r#typeof(e)?);
    Ok(re)
}

pub(crate) fn isConstZeroLength(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::List::Nil, .. } => true,
        Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Nil, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isConstFalse(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::BCONST { bool: false } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isConstTrue(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::BCONST { bool: true } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isConstOne(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::RCONST { real: rval } => {
            realEq(rval.clone(), metamodelica::OrderedFloat(1.0_f64))
        },
        Deref @ DAE::Exp::ICONST { integer: ival } => {
            intEq(ival.clone(), 1)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isConstMinusOne(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::RCONST { real: rval } => {
            realEq(rval.clone(), metamodelica::OrderedFloat(-1.0_f64))
        },
        Deref @ DAE::Exp::ICONST { integer: ival } => {
            intEq(ival.clone(), -1)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn isGreatereqOrLesseq(mut op: DAE::Operator) -> bool {
    let mut b: bool;
    b = (match op {
        DAE::Operator::GREATEREQ { .. } => true,
        DAE::Operator::LESSEQ { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isLesseqOrLess(mut op: DAE::Operator) -> bool {
    let mut b: bool;
    b = (match op {
        DAE::Operator::LESS { .. } => true,
        DAE::Operator::LESSEQ { .. } => true,
        _ => false,
    });
    b
}

pub fn containVectorFunctioncall(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
            return Ok(false)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => {
            return Ok(false)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. } => {
            return Ok(false)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "inStream" }, .. } => {
            return Ok(false)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "actualStream" }, .. } => {
            return Ok(false)
        },
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, .. } => {
            return Ok(true)
        },
        Deref @ DAE::Exp::CALL { .. } => {
            return Ok(false)
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { expList: elst, .. } => {
            return Ok(List::any(elst.clone(), (std::sync::Arc::new(containVectorFunctioncall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, .. } if (containVectorFunctioncall(e1.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, .. } if (containVectorFunctioncall(e2.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::UNARY { exp: e, .. } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::LBINARY { exp1: e1, .. } if (containVectorFunctioncall(e1.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::LBINARY { exp2: e2, .. } if (containVectorFunctioncall(e2.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::LUNARY { exp: e, .. } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::RELATION { exp1: e1, .. } if (containVectorFunctioncall(e1.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::RELATION { exp2: e2, .. } if (containVectorFunctioncall(e2.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::IFEXP { expCond: e1, .. } if (containVectorFunctioncall(e1.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::IFEXP { expThen: e2, .. } if (containVectorFunctioncall(e2.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::IFEXP { expElse: e3, .. } if (containVectorFunctioncall(e3.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::ARRAY { array: elst, .. } => {
            return Ok(List::any(elst.clone(), (std::sync::Arc::new(containVectorFunctioncall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?)
        },
        Deref @ DAE::Exp::MATRIX { matrix: explst, .. } => {
            let mut res: bool;
            let mut flatexplst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            flatexplst = List::flatten(explst.clone())?;
            return Ok(List::any(flatexplst.clone(), (std::sync::Arc::new(containVectorFunctioncall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?)
        },
        Deref @ DAE::Exp::RANGE { start: e1, .. } if (containVectorFunctioncall(e1.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::RANGE { stop: e2, .. } if (containVectorFunctioncall(e2.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::RANGE { step: Some(e), .. } if (containVectorFunctioncall(e.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::TUPLE { PR: elst } => {
            return Ok(List::any(elst.clone(), (std::sync::Arc::new(containVectorFunctioncall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?)
        },
        Deref @ DAE::Exp::CAST { exp: e, .. } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::SIZE { exp: e1, .. } if (containVectorFunctioncall(e1.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::SIZE { sz: Some(e2), .. } if (containVectorFunctioncall(e2.clone())?) => {
            return Ok(true)
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn containFunctioncall(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
            return Ok(false)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => {
            return Ok(false)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. } => {
            return Ok(false)
        },
        Deref @ DAE::Exp::CALL { .. } => {
            return Ok(true)
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { expList: elst, .. } => {
            let mut res: bool;
            return Ok(List::any(elst.clone(), (std::sync::Arc::new(containFunctioncall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, .. } if (containFunctioncall(e1.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::BINARY { exp2: e2, .. } if (containFunctioncall(e2.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::UNARY { exp: e, .. } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::LBINARY { exp1: e1, .. } if (containFunctioncall(e1.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::LBINARY { exp2: e2, .. } if (containFunctioncall(e2.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::LUNARY { exp: e, .. } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::RELATION { exp1: e1, .. } if (containFunctioncall(e1.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::RELATION { exp2: e2, .. } if (containFunctioncall(e2.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::IFEXP { expCond: e1, .. } if (containFunctioncall(e1.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::IFEXP { expThen: e2, .. } if (containFunctioncall(e2.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::IFEXP { expElse: e3, .. } if (containFunctioncall(e3.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::ARRAY { array: elst, .. } => {
            return Ok(List::any(elst.clone(), (std::sync::Arc::new(containFunctioncall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?)
        },
        Deref @ DAE::Exp::MATRIX { matrix: explst, .. } => {
            let mut res: bool;
            let mut flatexplst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            flatexplst = List::flatten(explst.clone())?;
            return Ok(List::any(flatexplst.clone(), (std::sync::Arc::new(containFunctioncall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?)
        },
        Deref @ DAE::Exp::RANGE { start: e1, .. } if (containFunctioncall(e1.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::RANGE { stop: e2, .. } if (containFunctioncall(e2.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::RANGE { step: Some(e), .. } if (containFunctioncall(e.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::TUPLE { PR: elst } => {
            return Ok(List::any(elst.clone(), (std::sync::Arc::new(containVectorFunctioncall) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?)
        },
        Deref @ DAE::Exp::CAST { exp: e, .. } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::ASUB { exp: e, .. } => {
            { inExp = e.clone(); continue '__tco; }
        },
        Deref @ DAE::Exp::SIZE { exp: e1, .. } if (containFunctioncall(e1.clone())?) => {
            return Ok(true)
        },
        Deref @ DAE::Exp::SIZE { sz: Some(e2), .. } if (containFunctioncall(e2.clone())?) => {
            return Ok(true)
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn expIntOrder(mut expectedValue: i32, mut integers: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &((expectedValue, integers)) {
        (_, Deref @ metamodelica::List::Nil) => {
            return true
        },
        (x1, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: x2 }, tail: expl }) if (intEq(x1.clone(), x2.clone())) => {
            { (expectedValue, integers) = (x1.clone() + 1, expl.clone()); continue '__tco; }
        },
        _ => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn isArray(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::ARRAY { .. } => true,
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: Deref @ DAE::Exp::ARRAY { .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub fn isMetaArray(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut outB: bool;
    outB = Types::isMetaArray(r#typeof(inExp)?);
    Ok(outB)
}

pub fn isMatrix(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::MATRIX { .. } => true,
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: Deref @ DAE::Exp::MATRIX { .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub(crate) fn isVector(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outIsVector: bool;
    outIsVector = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, .. } => false,
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsVector
}

pub(crate) fn isUnary(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::UNARY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub fn isBinary(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::BINARY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub fn isNegativeUnary(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub fn isCref(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outIsCref: bool;
    outIsCref = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CREF { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsCref
}

pub fn isUnaryCref(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outIsCref: bool;
    outIsCref = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::CREF { .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsCref
}

pub fn isCall(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outIsCall: bool;
    outIsCall = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CALL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsCall
}

pub fn isTSUB(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outIsCall: bool;
    outIsCall = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::TSUB { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsCall
}

pub(crate) fn isPureCall(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut outIsPureCall: bool;
    outIsPureCall = isCall(inExp.clone()) && !(isImpure(inExp)?);
    Ok(outIsPureCall)
}

pub fn isImpureCall(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut outIsPureCall: bool;
    outIsPureCall = isCall(inExp.clone()) && isImpure(inExp)?;
    Ok(outIsPureCall)
}

pub fn isRecordCall(mut inExp: Arc<DAE::Exp>, mut funcsIn: Arc<AvlTreePathFunction::Tree>) -> Result<bool> {
    let mut outIsCall: bool;
    outIsCall = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CALL { path, .. } => {
            let mut func: DAE::Function;
            let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(funcsIn, path.clone())?) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            func = __pa0.clone();
            DAEUtil::getFunctionElements(func.clone())?.is_empty()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outIsCall)
}

pub(crate) fn isNotCref(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outIsCref: bool;
    outIsCref = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CREF { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsCref
}

pub(crate) fn isCrefArray(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outIsArray: bool;
    outIsArray = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsArray
}

pub(crate) fn isCrefScalar(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut isScalar: bool;
    isScalar = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
                    let mut cr: ComponentRef;
                    let mut b: bool;
                    cr = expCref(inExp.clone())?;
                    b = ComponentReference::crefHasScalarSubscripts(cr.clone());
                    Ok(b.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { .. } => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    isScalar
}

pub fn isTuple(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outIsTuple: bool;
    outIsTuple = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::TUPLE { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsTuple
}

pub fn isRecord(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outIsRecord: bool;
    outIsRecord = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::RECORD { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsRecord
}

pub fn isScalarConst(mut inExp: Arc<DAE::Exp>) -> bool {
    let mut outIsScalar: bool;
    outIsScalar = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::ICONST { .. } => true,
        Deref @ DAE::Exp::RCONST { .. } => true,
        Deref @ DAE::Exp::SCONST { .. } => true,
        Deref @ DAE::Exp::BCONST { .. } => true,
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsScalar
}

pub(crate) fn isEven(mut e: Arc<DAE::Exp>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(e) {
        Deref @ DAE::Exp::ICONST { integer: i } => {
            return intMod(i.clone(), 2) == 0
        },
        Deref @ DAE::Exp::RCONST { real: r } => {
            return realMod(r.clone(), metamodelica::OrderedFloat(2.0_f64)) == metamodelica::OrderedFloat(0.0_f64)
        },
        Deref @ DAE::Exp::CAST { exp, .. } => {
            { e = exp.clone(); continue '__tco; }
        },
        _ => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub(crate) fn isOdd(mut e: Arc<DAE::Exp>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(e) {
        Deref @ DAE::Exp::ICONST { integer: i } => {
            return intMod(i.clone(), 2) == 1
        },
        Deref @ DAE::Exp::RCONST { real: r } => {
            return realMod(r.clone(), metamodelica::OrderedFloat(2.0_f64)) == metamodelica::OrderedFloat(1.0_f64)
        },
        Deref @ DAE::Exp::CAST { exp, .. } => {
            { e = exp.clone(); continue '__tco; }
        },
        _ => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub(crate) fn isIntegerOrReal(mut tp: Arc<DAE::Type>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(tp) {
        Deref @ DAE::Type::T_REAL { .. } => true,
        Deref @ DAE::Type::T_INTEGER { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub(crate) fn expStructuralEqual(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExp1.clone(), inExp2.clone())) {
        (Deref @ DAE::Exp::ICONST { integer: i1 }, Deref @ DAE::Exp::ICONST { integer: i2 }) => {
            return Ok(i1.clone() == i2.clone())
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::ICONST { integer: i1 } }, Deref @ DAE::Exp::ICONST { integer: i2 }) => {
            let mut i1 = (*i1).clone();
            i1 = -(i1.clone());
            return Ok(i1.clone() == i2.clone())
        },
        (Deref @ DAE::Exp::ICONST { integer: i1 }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::ICONST { integer: i2 } }) => {
            let mut i2 = (*i2).clone();
            i2 = -(i2.clone());
            return Ok(i1.clone() == i2.clone())
        },
        (Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::RCONST { real: r2 }) => {
            return Ok(r1.clone() == r2.clone())
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::RCONST { real: r1 } }, Deref @ DAE::Exp::RCONST { real: r2 }) => {
            let mut r1 = (*r1).clone();
            r1 = -(r1.clone());
            return Ok(r1.clone() == r2.clone())
        },
        (Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::RCONST { real: r2 } }) => {
            let mut r2 = (*r2).clone();
            r2 = -(r2.clone());
            return Ok(r1.clone() == r2.clone())
        },
        (Deref @ DAE::Exp::SCONST { string: s1 }, Deref @ DAE::Exp::SCONST { string: s2 }) => {
            return Ok(stringEq((s1.clone()).clone(), (s2.clone()).clone()))
        },
        (Deref @ DAE::Exp::BCONST { bool: b1 }, Deref @ DAE::Exp::BCONST { bool: b2 }) => {
            return Ok(boolEq(b1.clone(), b2.clone()))
        },
        (Deref @ DAE::Exp::ENUM_LITERAL { name: enum1, .. }, Deref @ DAE::Exp::ENUM_LITERAL { name: enum2, .. }) => {
            return Ok(AbsynUtil::pathEqual(enum1.clone(), enum2.clone()))
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::CREF { .. }) => {
            return Ok(true)
        },
        (Deref @ DAE::Exp::BINARY { exp1: e11, operator: op1, exp2: e12 }, Deref @ DAE::Exp::BINARY { exp1: e21, operator: op2, exp2: e22 }) => {
            let mut b: bool;
            b = operatorEqual(op1.clone(), op2.clone())?;
            b = if (b.clone()) {expStructuralEqual(e11.clone(), e21.clone())?} else {b.clone()};
            if (b.clone()) {{ (inExp1, inExp2) = (e12.clone(), e22.clone()); continue '__tco; }} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::LBINARY { exp1: e11, operator: op1, exp2: e12 }, Deref @ DAE::Exp::LBINARY { exp1: e21, operator: op2, exp2: e22 }) => {
            let mut b: bool;
            b = operatorEqual(op1.clone(), op2.clone())?;
            b = if (b.clone()) {expStructuralEqual(e11.clone(), e21.clone())?} else {b.clone()};
            if (b.clone()) {{ (inExp1, inExp2) = (e12.clone(), e22.clone()); continue '__tco; }} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::UNARY { operator: op1, exp: e1 }, Deref @ DAE::Exp::UNARY { operator: op2, exp: e2 }) => {
            let mut b: bool;
            b = operatorEqual(op1.clone(), op2.clone())?;
            if (b.clone()) {{ (inExp1, inExp2) = (e1.clone(), e2.clone()); continue '__tco; }} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::LUNARY { operator: op1, exp: e1 }, Deref @ DAE::Exp::LUNARY { operator: op2, exp: e2 }) => {
            let mut b: bool;
            b = operatorEqual(op1.clone(), op2.clone())?;
            if (b.clone()) {{ (inExp1, inExp2) = (e1.clone(), e2.clone()); continue '__tco; }} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::RELATION { exp1: e11, operator: op1, exp2: e12, .. }, Deref @ DAE::Exp::RELATION { exp1: e21, operator: op2, exp2: e22, .. }) => {
            let mut b: bool;
            b = operatorEqual(op1.clone(), op2.clone())?;
            b = if (b.clone()) {expStructuralEqual(e11.clone(), e21.clone())?} else {b.clone()};
            if (b.clone()) {{ (inExp1, inExp2) = (e12.clone(), e22.clone()); continue '__tco; }} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::IFEXP { expCond: e11, expThen: e12, expElse: e13 }, Deref @ DAE::Exp::IFEXP { expCond: e21, expThen: e22, expElse: e23 }) => {
            let mut b: bool;
            b = expStructuralEqual(e11.clone(), e21.clone())?;
            b = if (b.clone()) {expStructuralEqual(e12.clone(), e22.clone())?} else {b.clone()};
            if (b.clone()) {{ (inExp1, inExp2) = (e13.clone(), e23.clone()); continue '__tco; }} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::CALL { path: path1, expLst: expl1, .. }, Deref @ DAE::Exp::CALL { path: path2, expLst: expl2, .. }) => {
            let mut b: bool;
            b = AbsynUtil::pathEqual(path1.clone(), path2.clone());
            if (b.clone()) {return Ok(expStructuralEqualList(expl1.clone(), expl2.clone())?)} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::RECORD { path: path1, exps: expl1, .. }, Deref @ DAE::Exp::RECORD { path: path2, exps: expl2, .. }) => {
            let mut b: bool;
            b = AbsynUtil::pathEqual(path1.clone(), path2.clone());
            if (b.clone()) {return Ok(expStructuralEqualList(expl1.clone(), expl2.clone())?)} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::PARTEVALFUNCTION { path: path1, expList: expl1, .. }, Deref @ DAE::Exp::PARTEVALFUNCTION { path: path2, expList: expl2, .. }) => {
            let mut b: bool;
            b = AbsynUtil::pathEqual(path1.clone(), path2.clone());
            if (b.clone()) {return Ok(expStructuralEqualList(expl1.clone(), expl2.clone())?)} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::ARRAY { ty: tp1, array: expl1, .. }, Deref @ DAE::Exp::ARRAY { ty: tp2, array: expl2, .. }) => {
            let mut b: bool;
            b = tp1.clone() == tp2.clone();
            if (b.clone()) {return Ok(expStructuralEqualList(expl1.clone(), expl2.clone())?)} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::MATRIX { matrix: explstlst1, .. }, Deref @ DAE::Exp::MATRIX { matrix: explstlst2, .. }) => {
            return Ok(expStructuralEqualListLst(explstlst1.clone(), explstlst2.clone())?)
        },
        (Deref @ DAE::Exp::RANGE { start: e11, step: None, stop: e13, .. }, Deref @ DAE::Exp::RANGE { start: e21, step: None, stop: e23, .. }) => {
            let mut b: bool;
            b = expStructuralEqual(e11.clone(), e21.clone())?;
            if (b.clone()) {{ (inExp1, inExp2) = (e13.clone(), e23.clone()); continue '__tco; }} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::RANGE { start: e11, step: Some(e12), stop: e13, .. }, Deref @ DAE::Exp::RANGE { start: e21, step: Some(e22), stop: e23, .. }) => {
            let mut b: bool;
            b = expStructuralEqual(e11.clone(), e21.clone())?;
            b = if (b.clone()) {expStructuralEqual(e12.clone(), e22.clone())?} else {b.clone()};
            if (b.clone()) {{ (inExp1, inExp2) = (e13.clone(), e23.clone()); continue '__tco; }} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::TUPLE { PR: expl1 }, Deref @ DAE::Exp::TUPLE { PR: expl2 }) => {
            return Ok(expStructuralEqualList(expl1.clone(), expl2.clone())?)
        },
        (Deref @ DAE::Exp::CAST { ty: tp1, exp: e1 }, Deref @ DAE::Exp::CAST { ty: tp2, exp: e2 }) => {
            let mut b: bool;
            b = tp1.clone() == tp2.clone();
            if (b.clone()) {{ (inExp1, inExp2) = (e1.clone(), e2.clone()); continue '__tco; }} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::ASUB { exp: e1, sub: subs1 }, Deref @ DAE::Exp::ASUB { sub: subs2, .. }) => {
            let mut b: bool;
            let mut ae1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut ae2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            ae1 = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs1.clone()).into_iter().cloned() {
            let __x = getSubscriptExp(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ae2 = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs2.clone()).into_iter().cloned() {
            let __x = getSubscriptExp(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            b = expStructuralEqual(e1.clone(), e1.clone())?;
            if (b.clone()) {return Ok(expStructuralEqualList(ae1.clone(), ae2.clone())?)} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::SIZE { exp: e1, sz: None }, Deref @ DAE::Exp::SIZE { exp: e2, sz: None }) => {
            { (inExp1, inExp2) = (e1.clone(), e2.clone()); continue '__tco; }
        },
        (Deref @ DAE::Exp::SIZE { exp: e1, sz: Some(e11) }, Deref @ DAE::Exp::SIZE { exp: e2, sz: Some(e22) }) => {
            let mut b: bool;
            b = expStructuralEqual(e1.clone(), e2.clone())?;
            if (b.clone()) {{ (inExp1, inExp2) = (e11.clone(), e22.clone()); continue '__tco; }} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::CODE { .. }, Deref @ DAE::Exp::CODE { .. }) => {
            Debug::trace((literal!("exp_equal on CODE not impl.\n")).clone())?;
            return Ok(false)
        },
        (Deref @ DAE::Exp::REDUCTION { .. }, Deref @ DAE::Exp::REDUCTION { .. }) => {
            let mut res: bool;
            return Ok(inExp1 == inExp2)
        },
        (Deref @ DAE::Exp::LIST { valList: expl1 }, Deref @ DAE::Exp::LIST { valList: expl2 }) => {
            return Ok(expStructuralEqualList(expl1.clone(), expl2.clone())?)
        },
        (Deref @ DAE::Exp::CONS { car: e11, cdr: e12 }, Deref @ DAE::Exp::CONS { car: e21, cdr: e22 }) => {
            let mut b: bool;
            b = expStructuralEqual(e11.clone(), e21.clone())?;
            if (b.clone()) {{ (inExp1, inExp2) = (e12.clone(), e22.clone()); continue '__tco; }} else {return Ok(b.clone())}
        },
        (Deref @ DAE::Exp::META_TUPLE { listExp: expl1 }, Deref @ DAE::Exp::META_TUPLE { listExp: expl2 }) => {
            return Ok(expStructuralEqualList(expl1.clone(), expl2.clone())?)
        },
        (Deref @ DAE::Exp::META_OPTION { exp: None }, Deref @ DAE::Exp::META_OPTION { exp: None }) => {
            return Ok(true)
        },
        (Deref @ DAE::Exp::META_OPTION { exp: Some(e1) }, Deref @ DAE::Exp::META_OPTION { exp: Some(e2) }) => {
            { (inExp1, inExp2) = (e1.clone(), e2.clone()); continue '__tco; }
        },
        (Deref @ DAE::Exp::METARECORDCALL { path: path1, args: expl1, .. }, Deref @ DAE::Exp::METARECORDCALL { path: path2, args: expl2, .. }) => {
            let mut b: bool;
            b = AbsynUtil::pathEqual(path1.clone(), path2.clone());
            if (b.clone()) {return Ok(expStructuralEqualList(expl1.clone(), expl2.clone())?)} else {return Ok(b.clone())}
        },
        (e1 @ Deref @ DAE::Exp::MATCHEXPRESSION { .. }, e2 @ Deref @ DAE::Exp::MATCHEXPRESSION { .. }) => {
            return Ok(e1.clone() == e2.clone())
        },
        (Deref @ DAE::Exp::BOX { exp: e1 }, Deref @ DAE::Exp::BOX { exp: e2 }) => {
            { (inExp1, inExp2) = (e1.clone(), e2.clone()); continue '__tco; }
        },
        (Deref @ DAE::Exp::UNBOX { exp: e1, .. }, Deref @ DAE::Exp::UNBOX { exp: e2, .. }) => {
            { (inExp1, inExp2) = (e1.clone(), e2.clone()); continue '__tco; }
        },
        (Deref @ DAE::Exp::SHARED_LITERAL { index: i1, .. }, Deref @ DAE::Exp::SHARED_LITERAL { index: i2, .. }) => {
            return Ok(intEq(i1.clone(), i2.clone()))
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn expStructuralEqualList(mut inExp1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut inExp2: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExp1, inExp2)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok(true)
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: es1 }, Deref @ metamodelica::List::Cons { head: e2, tail: es2 }) if (expStructuralEqual(e1.clone(), e2.clone())?) => {
            { (inExp1, inExp2) = (es1.clone(), es2.clone()); continue '__tco; }
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn expStructuralEqualListLst(mut inExp1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut inExp2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExp1, inExp2)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok(true)
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: es1 }, Deref @ metamodelica::List::Cons { head: e2, tail: es2 }) if (expStructuralEqualList(e1.clone(), e2.clone())?) => {
            { (inExp1, inExp2) = (es1.clone(), es2.clone()); continue '__tco; }
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn expContainsList(mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut exp: Arc<DAE::Exp>) -> Result<bool> {
    let mut contains: bool = List::any(expl.clone(), (std::sync::Arc::new({ let __pe_b1 = exp.clone(); move |__pe_a0| expContains(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?;
    Ok(contains)
}

pub fn expContains(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = 'mc: {
        let __mc_input = (inExp1.clone(), inExp2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: i1 }, Deref @ DAE::Exp::ICONST { integer: i2 }) => {
                    Ok(i1.clone() == i2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { .. }, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::RCONST { real: r2 }) => {
                    Ok(r1.clone() == r2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { .. }, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SCONST { string: s1 }, Deref @ DAE::Exp::SCONST { string: s2 }) => {
                    Ok(s1.clone() == s2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SCONST { .. }, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: b1 }, Deref @ DAE::Exp::BCONST { bool: b2 }) => {
                    Ok(b1.clone() == b2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { .. }, _) => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ENUM_LITERAL { .. }, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: expLst, .. }, _) => {
                    Ok(expContainsList(expLst.clone(), inExp2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: expl, .. }, _) => {
                    Ok(List::any(expl.clone(), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static> = (std::sync::Arc::new({ let __pe_b1 = inExp2.clone(); move |__pe_a0| expContains(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>); move |__pe_a0| List::any(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static>))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
                    let mut res: bool;
                    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    res = ComponentReferenceBasics::crefEqual(cr1.clone(), cr2.clone())?;
                    if !(res.clone()) {
                        expLst = List::map(ComponentReferenceBasics::crefSubs(cr1.clone())?, (std::sync::Arc::new(getSubscriptExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Subscript>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                        res = expContainsList(expLst.clone(), inExp2.clone())?;
                    }
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, exp2: e2, .. }, _) => {
                    Ok(expContains(e1.clone(), inExp2.clone())? || expContains(e2.clone(), inExp2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { exp: e, .. }, _) => {
                    Ok(expContains(e.clone(), inExp2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LBINARY { exp1: e1, exp2: e2, .. }, _) => {
                    Ok(expContains(e1.clone(), inExp2.clone())? || expContains(e2.clone(), inExp2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { exp: e, .. }, _) => {
                    Ok(expContains(e.clone(), inExp2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { exp1: e1, exp2: e2, .. }, _) => {
                    Ok(expContains(e1.clone(), inExp2.clone())? || expContains(e2.clone(), inExp2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: c, expThen: t, expElse: f }, _) => {
                    Ok(expContains(c.clone(), inExp2.clone())? || expContains(t.clone(), inExp2.clone())? || expContains(f.clone(), inExp2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    let mut res: bool;
                    res = ComponentReferenceBasics::crefEqual(cr1.clone(), cr2.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Nil, .. }, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst, .. }, _) => {
                    Ok(expContainsList(expLst.clone(), inExp2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RECORD { exps: expLst, .. }, _) => {
                    Ok(expContainsList(expLst.clone(), inExp2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::PARTEVALFUNCTION { expList: expLst, .. }, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok(expContainsList(expLst.clone(), inExp2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_REAL { .. }, exp: Deref @ DAE::Exp::ICONST { .. } }, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_REAL { .. }, exp: e }, _) => {
                    Ok(expContains(e.clone(), inExp2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ASUB { exp: e, sub: subs }, _) => {
                    Ok(expContainsList(({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = getSubscriptExp(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), inExp2.clone())? || expContains(e.clone(), inExp2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { expr: e, .. }, _) => {
                    Ok(expContains(e.clone(), inExp2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- Expression.expContains failed\n")).clone())?;
                    s1 = (printExpStr(inExp1.clone())?).clone();
                    s2 = (printExpStr(inExp2.clone())?).clone();
                    r#str = stringAppendList(list![(literal!("exp = ")).clone(), (s1.clone()).clone(), (literal!(" subexp = ")).clone(), (s2.clone()).clone()]);
                    Debug::traceln((r#str.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBoolean)
}

pub(crate) fn containsExp(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = expContains(inExp2, inExp1)?;
    Ok(outBoolean)
}

pub fn isExpCref(mut e: Arc<DAE::Exp>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(e) {
        Deref @ DAE::Exp::CREF { componentRef: _, ty: _ } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub(crate) fn isExpCrefOrIfExp(mut e: Arc<DAE::Exp>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(e) {
        Deref @ DAE::Exp::CREF { componentRef: _, ty: _ } => true,
        Deref @ DAE::Exp::IFEXP { expCond: _, expThen: _, expElse: _ } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub(crate) fn isExpIfExp(mut e: Arc<DAE::Exp>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(e) {
        Deref @ DAE::Exp::IFEXP { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn operatorEqual(mut inOperator1: DAE::Operator, mut inOperator2: DAE::Operator) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = 0 == ExpressionBasics::operatorCompare(inOperator1, inOperator2)?;
    Ok(outBoolean)
}

pub fn arrayContainZeroDimension(mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inDimensions) {
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 }, tail: _ } => {
            return true
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest_dims } => {
            { inDimensions = rest_dims.clone(); continue '__tco; }
        },
        _ => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn arrayContainWholeDimension(mut inDim: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inDim) {
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: _ } => {
            return true
        },
        Deref @ metamodelica::List::Cons { head: _, tail: rest_dims } => {
            { inDim = rest_dims.clone(); continue '__tco; }
        },
        _ => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn isArrayType(mut inType: Arc<DAE::Type>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inType) {
        Deref @ DAE::Type::T_ARRAY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isRecordType(mut inType: Arc<DAE::Type>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inType) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isNotComplex(mut e: Arc<DAE::Exp>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &(e) {
        Deref @ DAE::Exp::CALL { .. } => {
            return false
        },
        Deref @ DAE::Exp::RECORD { .. } => {
            return false
        },
        Deref @ DAE::Exp::ARRAY { .. } => {
            return false
        },
        Deref @ DAE::Exp::CAST { exp: e2, .. } => {
            let mut b2: bool;
            { e = e2.clone(); continue '__tco; }
        },
        _ => {
            return true
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn isRealType(mut inType: Arc<DAE::Type>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inType) {
        Deref @ DAE::Type::T_REAL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn dimensionsEqual(mut dim1: Arc<DAE::Dimension>, mut dim2: Arc<DAE::Dimension>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &((dim1.clone(), dim2.clone())) {
        (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, _) => {
            true
        },
        (_, Deref @ DAE::Dimension::DIM_UNKNOWN { .. }) => {
            true
        },
        (Deref @ DAE::Dimension::DIM_EXP { .. }, _) => {
            true
        },
        (_, Deref @ DAE::Dimension::DIM_EXP { .. }) => {
            true
        },
        _ => {
            let mut b: bool;
            b = intEq(dimensionSize(dim1)?, dimensionSize(dim2)?);
            b.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn dimsEqual(mut dims1: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut dims2: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((dims1, dims2)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok(true)
        },
        (Deref @ metamodelica::List::Cons { head: d1, tail: dl1 }, Deref @ metamodelica::List::Cons { head: d2, tail: dl2 }) if (dimensionsEqual(d1.clone(), d2.clone())?) => {
            { (dims1, dims2) = (dl1.clone(), dl2.clone()); continue '__tco; }
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn dimsEqualAllowZero(mut dims1: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut dims2: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((dims1, dims2)) {
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Nil) => {
            return Ok(true)
        },
        (Deref @ metamodelica::List::Cons { head: d1, tail: dl1 }, Deref @ metamodelica::List::Cons { head: d2, tail: dl2 }) if (dimensionsEqualAllowZero(d1.clone(), d2.clone())?) => {
            { (dims1, dims2) = (dl1.clone(), dl2.clone()); continue '__tco; }
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub(crate) fn dimensionsEqualAllowZero(mut dim1: Arc<DAE::Dimension>, mut dim2: Arc<DAE::Dimension>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &((dim1.clone(), dim2.clone())) {
        (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, _) => {
            true
        },
        (_, Deref @ DAE::Dimension::DIM_UNKNOWN { .. }) => {
            true
        },
        (Deref @ DAE::Dimension::DIM_EXP { .. }, _) => {
            true
        },
        (_, Deref @ DAE::Dimension::DIM_EXP { .. }) => {
            true
        },
        _ => {
            let mut b: bool;
            let mut d1: i32;
            let mut d2: i32;
            d1 = dimensionSize(dim1)?;
            d2 = dimensionSize(dim2)?;
            b = boolOr(intEq(d1.clone(), d2.clone()), boolOr(boolAnd(intEq(d1.clone(), 0), intNe(d2.clone(), 0)), boolAnd(intEq(d2.clone(), 0), intNe(d1.clone(), 0))));
            b.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn dimensionsKnownAndEqual(mut dim1: Arc<DAE::Dimension>, mut dim2: Arc<DAE::Dimension>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &((dim1.clone(), dim2.clone())) {
        (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, _) => false,
        (_, Deref @ DAE::Dimension::DIM_UNKNOWN { .. }) => false,
        _ => ExpressionBasics::expEqual(dimensionSizeExp(dim1)?, dimensionSizeExp(dim2)?)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn dimensionKnown(mut dim: Arc<DAE::Dimension>) -> bool {
    let mut known: bool;
    known = (::match_deref::match_deref! { match &(dim) {
        Deref @ DAE::Dimension::DIM_UNKNOWN { .. } => false,
        Deref @ DAE::Dimension::DIM_EXP { exp: Deref @ DAE::Exp::ICONST { .. } } => true,
        Deref @ DAE::Dimension::DIM_EXP { exp: Deref @ DAE::Exp::BCONST { .. } } => true,
        Deref @ DAE::Dimension::DIM_EXP { exp: Deref @ DAE::Exp::ENUM_LITERAL { .. } } => true,
        Deref @ DAE::Dimension::DIM_EXP { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    known
}

pub(crate) fn dimensionKnownAndNonZero(mut dim: Arc<DAE::Dimension>) -> bool {
    let mut known: bool;
    known = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ DAE::Dimension::DIM_EXP { exp: Deref @ DAE::Exp::ICONST { integer: 0 } } => false,
        Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 } => false,
        _ => dimensionKnown(dim),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    known
}

pub fn dimensionsKnownAndNonZero(mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<bool> {
    let mut allKnown: bool;
    allKnown = List::all(dims, (std::sync::Arc::new(fnptr!(dimensionKnownAndNonZero, Arc<DAE::Dimension>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<bool> + 'static>))?;
    Ok(allKnown)
}

pub fn dimensionUnknownOrExp(mut dim: Arc<DAE::Dimension>) -> bool {
    let mut known: bool;
    known = (::match_deref::match_deref! { match &(dim) {
        Deref @ DAE::Dimension::DIM_UNKNOWN { .. } => true,
        Deref @ DAE::Dimension::DIM_EXP { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    known
}

pub(crate) fn dimensionUnknown(mut inDimension: Arc<DAE::Dimension>) -> bool {
    let mut outUnknown: bool;
    outUnknown = (::match_deref::match_deref! { match &(inDimension) {
        Deref @ DAE::Dimension::DIM_UNKNOWN { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outUnknown
}

pub fn hasUnknownDims(mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<bool> {
    let mut hasUnkown: bool;
    hasUnkown = List::any(dims, (std::sync::Arc::new(fnptr!(dimensionUnknown, Arc<DAE::Dimension>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<bool> + 'static>))?;
    Ok(hasUnkown)
}

pub(crate) fn subscriptConstant(mut sub: Arc<DAE::Subscript>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(sub) {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { .. } } => true,
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ENUM_LITERAL { .. } } => true,
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::BCONST { .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn subscriptConstants(mut inSubs: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> bool {
    let mut areConstant: bool = true;
    for mut sub in &*inSubs {
        let mut sub = sub.clone();
        areConstant = subscriptConstant(sub.clone());
        if !(areConstant) {
            return areConstant.clone();
        }
    }
    areConstant
}

pub fn isValidSubscript(mut inSub: Arc<DAE::Exp>) -> bool {
    let mut isValid: bool;
    isValid = (::match_deref::match_deref! { match &(inSub) {
        Deref @ DAE::Exp::ICONST { .. } => true,
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => true,
        Deref @ DAE::Exp::BCONST { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isValid
}

pub(crate) fn subscriptContain(mut issl1: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut issl2: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((issl1, issl2)) {
        (Deref @ metamodelica::List::Nil, _) => {
            return Ok(true)
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: ssl1 }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: ssl2 }) => {
            let mut b: bool;
            { (issl1, issl2) = (ssl1.clone(), ssl2.clone()); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: ssl1 }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::WHOLE_NONEXP { exp: _ }, tail: ssl2 }) => {
            let mut b: bool;
            { (issl1, issl2) = (ssl1.clone(), ssl2.clone()); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } }, tail: ssl1 }, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl } }, tail: ssl2 }) => {
            let mut b: bool;
            let true = (subscriptContain2(i.clone(), expl.clone())) else { bail!("pattern mismatch") };
            { (issl1, issl2) = (ssl1.clone(), ssl2.clone()); continue '__tco; }
        },
        (Deref @ metamodelica::List::Cons { head: ss1, tail: ssl1 }, Deref @ metamodelica::List::Cons { head: ss2, tail: ssl2 }) => {
            let mut b: bool;
            let true = (ExpressionBasics::subscriptEqual(list![ss1.clone()], list![ss2.clone()])?) else { bail!("pattern mismatch") };
            { (issl1, issl2) = (ssl1.clone(), ssl2.clone()); continue '__tco; }
        },
        _ => {
            return Ok(false)
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn subscriptContain2(mut inInt: i32, mut inExp2: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inInt, inExp2)) {
        (i, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: j }, tail: _ }) if (i.clone() == j.clone()) => {
            return true
        },
        (i, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: _ }, tail: expl }) if (subscriptContain2(i.clone(), expl.clone())) => {
            return true
        },
        (i, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl2 }, tail: expl }) => {
            let mut b: bool;
            let mut b2: bool;
            b = subscriptContain2(i.clone(), expl2.clone());
            if (b.clone()) {return true} else {{ (inInt, inExp2) = (i.clone(), expl.clone()); continue '__tco; }}
        },
        _ => {
            return false
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

pub fn hasNoSideEffects(mut inExp: Arc<DAE::Exp>, mut ib: bool) -> (Arc<DAE::Exp>, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut ob: bool;
    (outExp, ob) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { .. } => (inExp, false),
        Deref @ DAE::Exp::MATCHEXPRESSION { .. } => (inExp, false),
        _ => (inExp, ib),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, ob)
}

pub(crate) fn isBuiltinFunctionReference(mut exp: Arc<DAE::Exp>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(exp) {
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { builtin: true, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn makeCons(mut car: Arc<DAE::Exp>, mut cdr: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut exp: Arc<DAE::Exp>;
    exp = Arc::new(DAE::Exp::CONS { car: car, cdr: cdr });
    exp
}

pub fn makeBuiltinCall(mut name: ArcStr, mut args: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut result_type: Arc<DAE::Type>, mut isImpure: bool) -> Arc<DAE::Exp> {
    let mut call: Arc<DAE::Exp>;
    call = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (name).clone() }), expLst: args, attr: Arc::new(DAE::CallAttributes { ty: result_type, tuple_: false, builtin: true, isImpure: isImpure, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL }) });
    call
}

pub fn makePureBuiltinCall(mut name: ArcStr, mut args: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut result_type: Arc<DAE::Type>) -> Arc<DAE::Exp> {
    let mut call: Arc<DAE::Exp>;
    call = makeBuiltinCall((name).clone(), args, result_type, false);
    call
}

pub fn makeImpureBuiltinCall(mut name: ArcStr, mut args: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut result_type: Arc<DAE::Type>) -> Arc<DAE::Exp> {
    let mut call: Arc<DAE::Exp>;
    call = makeBuiltinCall((name).clone(), args, result_type, true);
    call
}

pub fn reductionIterName(mut iter: Arc<DAE::ReductionIterator>) -> Result<ArcStr> {
    let mut name: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(iter) {
        Deref @ DAE::ReductionIterator { id: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    Ok(name)
}

fn traverseReductionIteratorBidir<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut inIter: Arc<DAE::ReductionIterator>, mut inEnterFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inExitFunc: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>, mut inArg: ArgT) -> Result<(Arc<DAE::ReductionIterator>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, ArgT) -> Result<(Arc<DAE::Exp>, ArgT)> + 'static>;

    let mut outIter: Arc<DAE::ReductionIterator>;
    let mut outArg: ArgT;
    (outIter, outArg) = (::match_deref::match_deref! { match &(inIter) {
        Deref @ DAE::ReductionIterator { id, exp, guardExp: gexp, ty } => {
            let mut arg: ArgT;
            let mut exp = (*exp).clone();
            let mut gexp = (*gexp).clone();
            (exp, arg) = traverseExpBidir(exp.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (gexp, arg) = traverseExpOptBidir(gexp.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg.clone())?;
            (Arc::new(DAE::ReductionIterator { id: (id.clone()).clone(), exp: exp.clone(), guardExp: gexp.clone(), ty: ty.clone() }), arg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outIter, outArg))
}

fn traverseReductionIteratorTopDown<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut iter: Arc<DAE::ReductionIterator>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>, mut inArg: Type_a) -> Result<(Arc<DAE::ReductionIterator>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>;

    let mut outIter: Arc<DAE::ReductionIterator>;
    let mut outArg: Type_a;
    (outIter, outArg) = (::match_deref::match_deref! { match &((iter, inArg)) {
        (Deref @ DAE::ReductionIterator { id, exp, guardExp: gexp, ty }, arg) => {
            let mut exp = (*exp).clone();
            let mut gexp = (*gexp).clone();
            let mut arg = (*arg).clone();
            (exp, arg) = traverseExpTopDown(exp.clone(), func.clone(), arg.clone())?;
            (gexp, arg) = traverseExpOptTopDown(gexp.clone(), func.clone(), arg.clone())?;
            (Arc::new(DAE::ReductionIterator { id: (id.clone()).clone(), exp: exp.clone(), guardExp: gexp.clone(), ty: ty.clone() }), arg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outIter, outArg))
}

fn traverseReductionIteratorsTopDown<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inIters: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>, mut inArg: Type_a) -> Result<(Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, bool, Type_a)> + 'static>;

    let mut outIters: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>;
    let mut outArg: Type_a;
    (outIters, outArg) = (::match_deref::match_deref! { match &((inIters.clone(), inArg)) {
        (Deref @ metamodelica::List::Nil, arg) => {
            (inIters, arg.clone())
        },
        (Deref @ metamodelica::List::Cons { head: iter, tail: iters }, arg) => {
            let mut iter = (*iter).clone();
            let mut iters = (*iters).clone();
            let mut arg = (*arg).clone();
            (iter, arg) = traverseReductionIteratorTopDown(iter.clone(), func.clone(), arg.clone())?;
            (iters, arg) = traverseReductionIteratorsTopDown(iters.clone(), func.clone(), arg.clone())?;
            (metamodelica::cons(iter.clone(), iters.clone()), arg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outIters, outArg))
}

fn traverseReductionIterator<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut iter: Arc<DAE::ReductionIterator>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut iarg: Type_a) -> Result<(Arc<DAE::ReductionIterator>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut outIter: Arc<DAE::ReductionIterator> = Arc::new(<DAE::ReductionIterator as ::std::default::Default>::default());
    let mut outArg: Type_a;
    (outIter, outArg) = (::match_deref::match_deref! { match &((iter.clone(), iarg)) {
        (Deref @ DAE::ReductionIterator { id, exp, guardExp: gexp, ty }, arg) => {
            let mut exp1: Arc<DAE::Exp>;
            let mut gexp1: Option<Arc<DAE::Exp>>;
            let mut arg = (*arg).clone();
            (exp1, arg) = traverseExpBottomUp(exp.clone(), func.clone(), arg.clone())?;
            (gexp1, arg) = traverseExpOpt(gexp.clone(), func.clone(), arg.clone())?;
            outIter = if (referenceEq(&*(exp.clone()),&*(exp1.clone())) && (match (&(gexp.clone()), &(gexp1.clone())) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false })) {iter} else {Arc::new(DAE::ReductionIterator { id: (id.clone()).clone(), exp: exp1.clone(), guardExp: gexp1.clone(), ty: ty.clone() })};
            (outIter, arg.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outIter, outArg))
}

fn traverseReductionIterators<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut iters: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>, mut arg: Type_a) -> Result<(Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Type_a) -> Result<(Arc<DAE::Exp>, Type_a)> + 'static>;

    let mut iters: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>> = iters;
    let mut arg: Type_a = arg;
    (iters, arg) = (::match_deref::match_deref! { match &(iters.clone()) {
        Deref @ metamodelica::List::Nil => {
            (iters, arg)
        },
        Deref @ metamodelica::List::Cons { head: iter, tail: rest } => {
            let mut iter1: Arc<DAE::ReductionIterator>;
            let mut iters1: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>;
            (iter1, arg) = traverseReductionIterator(iter.clone(), func.clone(), arg)?;
            (iters1, arg) = traverseReductionIterators(rest.clone(), func.clone(), arg)?;
            iters = if (referenceEq(&*(iter.clone()),&*(iter1.clone())) && metamodelica::ReferenceEq::reference_eq(&*(rest.clone()), &*(iters1.clone()))) {iters} else {metamodelica::cons(iter1.clone(), iters1.clone())};
            (iters, arg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((iters, arg))
}

pub fn simpleCrefName(mut exp: Arc<DAE::Exp>) -> Result<ArcStr> {
    let mut name: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(exp) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: __pa0, subscriptLst: Deref @ metamodelica::List::Nil, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    Ok(name)
}

pub fn isTailCall(mut exp: Arc<DAE::Exp>) -> bool {
    let mut isTail: bool;
    isTail = (::match_deref::match_deref! { match &(exp) {
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { tailCall: DAE::TailCall::TAIL { .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTail
}

pub fn complexityTraverse(mut exp: Arc<DAE::Exp>, mut complexity: i32) -> Result<(Arc<DAE::Exp>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outComplexity: i32;
    (outExp, outComplexity) = traverseExpBottomUp(exp, (std::sync::Arc::new(complexityTraverse2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, i32) -> Result<(Arc<DAE::Exp>, i32)> + 'static>), complexity)?;
    Ok((outExp, outComplexity))
}

fn complexityTraverse2(mut exp: Arc<DAE::Exp>, mut complexity_: i32) -> Result<(Arc<DAE::Exp>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outComplexity: i32;
    outComplexity = complexity_ + complexity(exp.clone())?;
    outExp = exp;
    Ok((outExp, outComplexity))
}

pub(crate) const complexityAlloc: i32 = 5;

pub(crate) const complexityVeryBig: i32 = 500000;

pub(crate) const complexityDimLarge: i32 = 1000;

pub(crate) fn complexity(mut exp: Arc<DAE::Exp>) -> Result<i32> {
    let mut i: i32;
    i = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::ICONST { .. } => {
            0
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            0
        },
        Deref @ DAE::Exp::SCONST { .. } => {
            0
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            0
        },
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => {
            0
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            0
        },
        Deref @ DAE::Exp::CREF { ty: tp, .. } => {
            tpComplexity(tp.clone())?
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
            let mut c1: i32;
            let mut c2: i32;
            let mut c3: i32;
            c1 = complexity(e1.clone())?;
            c2 = complexity(e2.clone())?;
            c3 = opComplexity(op.clone())?;
            c1.clone() + c2.clone() + c3.clone()
        },
        Deref @ DAE::Exp::UNARY { exp: e, operator: op } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = complexity(e.clone())?;
            c2 = opComplexity(op.clone())?;
            c1.clone() + c2.clone()
        },
        Deref @ DAE::Exp::LBINARY { exp1: e1, exp2: e2, operator: op } => {
            let mut c1: i32;
            let mut c2: i32;
            let mut c3: i32;
            c1 = complexity(e1.clone())?;
            c2 = complexity(e2.clone())?;
            c3 = opComplexity(op.clone())?;
            c1.clone() + c2.clone() + c3.clone()
        },
        Deref @ DAE::Exp::LUNARY { exp: e, operator: op } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = complexity(e.clone())?;
            c2 = opComplexity(op.clone())?;
            c1.clone() + c2.clone()
        },
        Deref @ DAE::Exp::RELATION { exp1: e1, exp2: e2, operator: op, .. } => {
            let mut c1: i32;
            let mut c2: i32;
            let mut c3: i32;
            c1 = complexity(e1.clone())?;
            c2 = complexity(e2.clone())?;
            c3 = opComplexity(op.clone())?;
            c1.clone() + c2.clone() + c3.clone()
        },
        Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 } => {
            let mut c1: i32;
            let mut c2: i32;
            let mut c3: i32;
            c1 = complexity(e1.clone())?;
            c2 = complexity(e2.clone())?;
            c3 = complexity(e3.clone())?;
            c1.clone() + intMax(c2.clone(), c3.clone())
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, expLst: exps, attr: Deref @ DAE::CallAttributes { ty: tp, builtin: true, .. } } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(exps.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), (std::sync::Arc::new(complexity) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>), 0)?;
            c2 = complexityBuiltin((name.clone()).clone(), tp.clone())?;
            c1.clone() + c2.clone()
        },
        Deref @ DAE::Exp::CALL { expLst: exps, .. } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(exps.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), (std::sync::Arc::new(complexity) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>), 0)?;
            c2 = (exps.clone().len() as i32);
            c1.clone() + c2.clone() + 25
        },
        Deref @ DAE::Exp::RECORD { exps, .. } => {
            let mut c1: i32;
            c1 = List::applyAndFold(exps.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), (std::sync::Arc::new(complexity) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>), 1)?;
            c1.clone()
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { .. } => {
            complexityVeryBig.clone()
        },
        Deref @ DAE::Exp::ARRAY { array: exps, ty: tp, .. } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(exps.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), (std::sync::Arc::new(complexity) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>), if (isArrayType(tp.clone())) {0} else {complexityAlloc.clone()})?;
            c2 = (exps.clone().len() as i32);
            c1.clone() + c2.clone()
        },
        Deref @ DAE::Exp::MATRIX { matrix: matrix @ Deref @ metamodelica::List::Cons { head: exps, tail: _ }, .. } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(List::flatten(matrix.clone())?, (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), (std::sync::Arc::new(complexity) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>), complexityAlloc.clone())?;
            c2 = (exps.clone().len() as i32) * (matrix.clone().len() as i32);
            c1.clone() + c2.clone()
        },
        Deref @ DAE::Exp::RANGE { start: e1, stop: e2, step: None, .. } => {
            complexityDimLarge.clone() + complexity(e1.clone())? + complexity(e2.clone())?
        },
        Deref @ DAE::Exp::RANGE { start: e1, stop: e2, step: Some(e3), .. } => {
            complexityDimLarge.clone() + complexity(e1.clone())? + complexity(e2.clone())? + complexity(e3.clone())?
        },
        Deref @ DAE::Exp::TUPLE { PR: exps } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(exps.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), (std::sync::Arc::new(complexity) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>), complexityAlloc.clone())?;
            c2 = (exps.clone().len() as i32);
            c1.clone() + c2.clone()
        },
        Deref @ DAE::Exp::CAST { exp: e, ty: tp } => {
            tpComplexity(tp.clone())? + complexity(e.clone())?
        },
        Deref @ DAE::Exp::ASUB { exp: e, sub: subs } => {
            let mut c1: i32;
            let mut c2: i32;
            let mut c3: i32;
            let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            exps = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = getSubscriptExp(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            c1 = List::applyAndFold(exps.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), (std::sync::Arc::new(complexity) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>), complexityAlloc.clone())?;
            c2 = (exps.clone().len() as i32);
            c3 = complexity(e.clone())?;
            c1.clone() + c2.clone() + c3.clone()
        },
        Deref @ DAE::Exp::TSUB { exp: e, .. } => {
            complexity(e.clone())? + 1
        },
        Deref @ DAE::Exp::SIZE { exp: e, sz: None } => {
            complexity(e.clone())? + complexityAlloc.clone() + 10
        },
        Deref @ DAE::Exp::SIZE { exp: e1, sz: Some(e2) } => {
            complexity(e1.clone())? + complexity(e2.clone())? + 1
        },
        Deref @ DAE::Exp::CODE { .. } => {
            complexityVeryBig.clone()
        },
        Deref @ DAE::Exp::EMPTY { .. } => {
            complexityVeryBig.clone()
        },
        Deref @ DAE::Exp::REDUCTION { .. } => {
            complexityVeryBig.clone()
        },
        Deref @ DAE::Exp::LIST { valList: exps } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(exps.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), (std::sync::Arc::new(complexity) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>), complexityAlloc.clone())?;
            c2 = (exps.clone().len() as i32);
            c1.clone() + c2.clone() + complexityAlloc.clone()
        },
        Deref @ DAE::Exp::CONS { car: e1, cdr: e2 } => {
            complexityAlloc.clone() + complexity(e1.clone())? + complexity(e2.clone())?
        },
        Deref @ DAE::Exp::META_TUPLE { listExp: exps } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(exps.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), (std::sync::Arc::new(complexity) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>), complexityAlloc.clone())?;
            c2 = (exps.clone().len() as i32);
            complexityAlloc.clone() + c1.clone() + c2.clone()
        },
        Deref @ DAE::Exp::META_OPTION { exp: None } => {
            0
        },
        Deref @ DAE::Exp::META_OPTION { exp: Some(e) } => {
            complexity(e.clone())? + complexityAlloc.clone()
        },
        Deref @ DAE::Exp::METARECORDCALL { args: exps, .. } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(exps.clone(), (std::sync::Arc::new(fnptr!(intAdd, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), (std::sync::Arc::new(complexity) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>), complexityAlloc.clone())?;
            c2 = (exps.clone().len() as i32);
            c1.clone() + c2.clone() + complexityAlloc.clone()
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { .. } => {
            complexityVeryBig.clone()
        },
        Deref @ DAE::Exp::BOX { exp: e } => {
            complexityAlloc.clone() + complexity(e.clone())?
        },
        Deref @ DAE::Exp::UNBOX { exp: e, .. } => {
            1 + complexity(e.clone())?
        },
        Deref @ DAE::Exp::PATTERN { .. } => {
            0
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.complexityWork failed: ")); __mm_s.push_str(&*printExpStr(exp)?); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(i)
}

fn complexityBuiltin(mut name: ArcStr, mut tp: Arc<DAE::Type>) -> Result<i32> {
    let mut complexity: i32;
    complexity = (::match_deref::match_deref! { match &(name) {
        Deref @ "identity" => complexityAlloc.clone() + tpComplexity(tp)?,
        Deref @ "cross" => 3 * 3,
        _ => 25,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(complexity)
}

fn tpComplexity(mut tp: Arc<DAE::Type>) -> Result<i32> {
    let mut i: i32 = 0;
    i = (::match_deref::match_deref! { match &(tp) {
        Deref @ DAE::Type::T_ARRAY { dims, .. } => {
            i = List::applyAndFold(dims.clone(), (std::sync::Arc::new(fnptr!(intMul, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(dimComplexity, Arc<DAE::Dimension>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<i32> + 'static>), 1)?;
            i
        },
        _ => {
            0
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(i)
}

pub(crate) fn dimComplexity(mut dim: Arc<DAE::Dimension>) -> i32 {
    let mut i: i32 = 0;
    i = (::match_deref::match_deref! { match &(dim) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: __esc_i } => {
            i = (*__esc_i).clone();
            i.clone()
        },
        Deref @ DAE::Dimension::DIM_ENUM { size: __esc_i, .. } => {
            i = (*__esc_i).clone();
            i.clone()
        },
        Deref @ DAE::Dimension::DIM_BOOLEAN { .. } => 2,
        _ => complexityDimLarge.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    i
}

fn opComplexity(mut op: DAE::Operator) -> Result<i32> {
    let mut i: i32;
    i = (::match_deref::match_deref! { match &(op) {
        DAE::Operator::ADD { ty: Deref @ DAE::Type::T_STRING { .. } } => {
            100
        },
        DAE::Operator::ADD { .. } => {
            1
        },
        DAE::Operator::SUB { .. } => {
            1
        },
        DAE::Operator::MUL { .. } => {
            1
        },
        DAE::Operator::DIV { .. } => {
            1
        },
        DAE::Operator::POW { .. } => {
            30
        },
        DAE::Operator::UMINUS { .. } => {
            1
        },
        DAE::Operator::UMINUS_ARR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(tp.clone())?
        },
        DAE::Operator::ADD_ARR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(tp.clone())?
        },
        DAE::Operator::SUB_ARR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(tp.clone())?
        },
        DAE::Operator::MUL_ARR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(tp.clone())?
        },
        DAE::Operator::DIV_ARR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(tp.clone())?
        },
        DAE::Operator::MUL_ARRAY_SCALAR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(tp.clone())?
        },
        DAE::Operator::ADD_ARRAY_SCALAR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(tp.clone())?
        },
        DAE::Operator::SUB_SCALAR_ARRAY { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(tp.clone())?
        },
        DAE::Operator::MUL_SCALAR_PRODUCT { ty: tp } => {
            complexityAlloc.clone() + 3 * tpComplexity(tp.clone())?
        },
        DAE::Operator::MUL_MATRIX_PRODUCT { ty: tp } => {
            complexityAlloc.clone() + 3 * tpComplexity(tp.clone())?
        },
        DAE::Operator::DIV_ARRAY_SCALAR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(tp.clone())?
        },
        DAE::Operator::DIV_SCALAR_ARRAY { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(tp.clone())?
        },
        DAE::Operator::POW_ARRAY_SCALAR { ty: tp } => {
            complexityAlloc.clone() + 30 * tpComplexity(tp.clone())?
        },
        DAE::Operator::POW_SCALAR_ARRAY { ty: tp } => {
            complexityAlloc.clone() + 30 * tpComplexity(tp.clone())?
        },
        DAE::Operator::POW_ARR { ty: tp } => {
            complexityAlloc.clone() + 30 * tpComplexity(tp.clone())?
        },
        DAE::Operator::POW_ARR2 { ty: tp } => {
            complexityAlloc.clone() + 30 * tpComplexity(tp.clone())?
        },
        DAE::Operator::AND { .. } => {
            1
        },
        DAE::Operator::OR { .. } => {
            1
        },
        DAE::Operator::NOT { .. } => {
            1
        },
        DAE::Operator::LESS { .. } => {
            1
        },
        DAE::Operator::LESSEQ { .. } => {
            1
        },
        DAE::Operator::GREATER { .. } => {
            1
        },
        DAE::Operator::GREATEREQ { .. } => {
            1
        },
        DAE::Operator::EQUAL { .. } => {
            1
        },
        DAE::Operator::NEQUAL { .. } => {
            1
        },
        DAE::Operator::USERDEFINED { .. } => {
            100
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Expression.opWCET failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(i)
}

pub fn makeEnumLiterals(mut inTypeName: Arc<Absyn::Path>, mut inLiterals: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outLiterals: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut enum_lit_names: Arc<metamodelica::List<Arc<Absyn::Path>>>;
    enum_lit_names = List::map1r(inLiterals, (std::sync::Arc::new(AbsynUtil::suffixPath) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, ArcStr) -> Result<Arc<Absyn::Path>> + 'static>), inTypeName)?;
    (outLiterals, _) = List::mapFold(enum_lit_names, (std::sync::Arc::new(fnptr!(makeEnumLiteral, Arc<Absyn::Path>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, i32) -> Result<(Arc<DAE::Exp>, i32)> + 'static>), 1)?;
    Ok(outLiterals)
}

fn makeEnumLiteral(mut name: Arc<Absyn::Path>, mut index: i32) -> (Arc<DAE::Exp>, i32) {
    let mut enumExp: Arc<DAE::Exp>;
    let mut newIndex: i32;
    enumExp = Arc::new(DAE::Exp::ENUM_LITERAL { name: name, index: index });
    newIndex = index + 1;
    (enumExp, newIndex)
}

pub fn isWild(mut exp: Arc<DAE::Exp>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(exp) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isNotWild(mut exp: Arc<DAE::Exp>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(exp) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn dimensionsToExps(mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Arc<metamodelica::List<Arc<DAE::Exp>>> {
    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    for mut d in &*dims {
        let mut d = d.clone();
        exps = (::match_deref::match_deref! { match &(d.clone()) {
        Deref @ DAE::Dimension::DIM_EXP { exp } => {
            metamodelica::cons(exp.clone(), exps.clone())
        },
        _ => {
            exps.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    exps = exps.reverse();
    exps
}

pub fn splitRecord(mut inExp: Arc<DAE::Exp>, mut ty: Arc<DAE::Type>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExp, ty.clone())) {
        (Deref @ DAE::Exp::CAST { exp, .. }, _) => {
            { (inExp, ty) = (exp.clone(), ty); continue '__tco; }
        },
        (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, varLst: Deref @ metamodelica::List::Nil, .. }) => {
            return Ok(bail!("fail"))
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, Deref @ DAE::Type::T_COMPLEX { varLst: vs, .. }) => {
            return Ok(List::map1(vs.clone(), (std::sync::Arc::new(splitRecord2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), cr.clone())?)
        },
        (Deref @ DAE::Exp::CALL { path: p1, expLst: exps, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p2 }, .. }, .. } }, _) => {
            let true = (AbsynUtil::pathEqual(p1.clone(), p2.clone())) else { bail!("pattern mismatch") };
            return Ok(exps.clone())
        },
        (Deref @ DAE::Exp::RECORD { exps, .. }, _) => {
            return Ok(exps.clone())
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn splitRecord2(mut var: Arc<DAE::Var>, mut cr: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> {
    let mut exp: Arc<DAE::Exp>;
    let mut n: ArcStr;
    let mut tt: Arc<DAE::Type>;
    let mut ty: Arc<DAE::Type>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(var) {
        Deref @ DAE::Var { name: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    n = __pa0.clone();
    tt = __pa1.clone();
    ty = Types::simplifyType(tt)?;
    exp = makeCrefExp(ComponentReference::crefPrependIdent(cr, (n).clone(), metamodelica::nil(), ty.clone())?, ty)?;
    Ok(exp)
}

pub(crate) fn splitArray(mut inExp: Arc<DAE::Exp>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, bool)> {
    let mut outExp: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut didSplit: bool;
    (outExp, didSplit) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
            (expl.clone(), true)
        },
        Deref @ DAE::Exp::MATRIX { matrix: mat, .. } => {
            (List::flatten(mat.clone())?, true)
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: istart }, step, stop: Deref @ DAE::Exp::ICONST { integer: istop }, .. } => {
            let mut istep: i32 = 0;
            (({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut i in (ExpressionSimplify::simplifyRange(istart.clone(), (::match_deref::match_deref! { match &(step.clone()) {
        None => 1,
        Some(Deref @ DAE::Exp::ICONST { integer: __esc_istep }) => {
            istep = (*__esc_istep).clone();
            istep.clone()
        },
        _ => bail!("match: no arm matched"),
    } }), istop.clone())?).into_iter().cloned() {
            let __x = Arc::new(DAE::Exp::ICONST { integer: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), true)
        },
        _ => {
            (list![inExp], false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, didSplit))
}

pub(crate) fn equationExpEqual(mut exp1: Arc<DAE::EquationExp>, mut exp2: Arc<DAE::EquationExp>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &((exp1, exp2)) {
        (Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: e1 }, Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: e2 }) => {
            ExpressionBasics::expEqual(e1.clone(), e2.clone())?
        },
        (Deref @ DAE::EquationExp::RESIDUAL_EXP { exp: e1 }, Deref @ DAE::EquationExp::RESIDUAL_EXP { exp: e2 }) => {
            ExpressionBasics::expEqual(e1.clone(), e2.clone())?
        },
        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: e1, rhs: e2 }, Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: e3, rhs: e4 }) => {
            ExpressionBasics::expEqual(e1.clone(), e3.clone())? && ExpressionBasics::expEqual(e2.clone(), e4.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn promoteExp(mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>, mut inDims: i32) -> (Arc<DAE::Exp>, Arc<DAE::Type>) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outType: Arc<DAE::Type>;
    (outExp, outType) = 'mc: {
        let __mc_input = inDims;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut dims_to_add: i32;
            let mut ty: Arc<DAE::Type>;
            let mut res_ty: Arc<DAE::Type>;
            let mut exp: Arc<DAE::Exp>;
            let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>>;
            let mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
            let mut added_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
            let mut is_array_ty: bool;
            dims_to_add = inDims - Types::numberOfDimensions(inType.clone());
            let true = (dims_to_add.clone() > 0) else { bail!("pattern mismatch") };
            added_dims = List::fill(Arc::new(DAE::Dimension::DIM_INTEGER { integer: 1 }), dims_to_add.clone());
            dims = listAppend(TypesDump::getDimensions(inType.clone()), added_dims.clone());
            ty = Types::arrayElementType(inType.clone());
            res_ty = Types::liftArrayListDims(ty.clone(), dims.clone());
            ty = Types::simplifyType(ty.clone())?;
            tys = makePromotedTypes(dims.clone(), ty.clone(), metamodelica::nil());
            is_array_ty = Types::isArray(inType.clone());
            exp = promoteExp2(inExp.clone(), is_array_ty.clone(), inDims, tys.clone())?;
            Ok((exp.clone(), res_ty.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((inExp.clone(), inType.clone()))
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outType)
}

fn makePromotedTypes(mut inDimensions: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut inElementType: Arc<DAE::Type>, mut inAccumTypes: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Arc<metamodelica::List<Arc<DAE::Type>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inDimensions.clone()) {
        Deref @ metamodelica::List::Cons { head: _, tail: rest_dims } => {
            let mut ty: Arc<DAE::Type>;
            ty = Arc::new(DAE::Type::T_ARRAY { ty: inElementType.clone(), dims: inDimensions });
            { (inDimensions, inElementType, inAccumTypes) = (rest_dims.clone(), inElementType, metamodelica::cons(ty.clone(), inAccumTypes)); continue '__tco; }
        },
        Deref @ metamodelica::List::Nil => {
            return inAccumTypes.reverse()
        },
        _ => unreachable!("tail-call lowered match: no arm matched"),
    } }
    }
}

fn promoteExp2(mut inExp: Arc<DAE::Exp>, mut inIsArray: bool, mut inDims: i32, mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inExp.clone(), inIsArray, inTypes.clone())) {
        (_, _, Deref @ metamodelica::List::Nil) => {
            inExp
        },
        (Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl }, _, Deref @ metamodelica::List::Cons { head: ty, tail: rest_ty }) => {
            let mut expl = (*expl).clone();
            expl = List::map3(expl.clone(), (std::sync::Arc::new(promoteExp2) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool, i32, Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Arc<DAE::Exp>> + 'static>), false, inDims, rest_ty.clone())?;
            Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: false, array: expl.clone() })
        },
        (_, true, Deref @ metamodelica::List::Cons { head: ty, tail: _ }) => {
            makePureBuiltinCall((literal!("promote")).clone(), list![inExp, Arc::new(DAE::Exp::ICONST { integer: inDims })], ty.clone())
        },
        _ => {
            promoteExp3(inExp, inTypes)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn promoteExp3(mut inExp: Arc<DAE::Exp>, mut inTypes: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inTypes) {
        Deref @ metamodelica::List::Nil => {
            inExp
        },
        Deref @ metamodelica::List::Cons { head: ty, tail: Deref @ metamodelica::List::Nil } => {
            makeArray(list![inExp], ty.clone(), true)
        },
        Deref @ metamodelica::List::Cons { head: ty, tail: rest_ty } => {
            let mut exp: Arc<DAE::Exp>;
            exp = promoteExp3(inExp, rest_ty.clone())?;
            makeArray(list![exp.clone()], ty.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn matrixToArray(mut inMatrix: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outArray: Arc<DAE::Exp>;
    outArray = (::match_deref::match_deref! { match &(inMatrix.clone()) {
        Deref @ DAE::Exp::MATRIX { ty, matrix, .. } => {
            let mut row_ty: Arc<DAE::Type>;
            let mut rows: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            row_ty = unliftArray(ty.clone())?;
            rows = List::map2(matrix.clone(), (std::sync::Arc::new(fnptr!(makeArray, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool) -> Result<Arc<DAE::Exp>> + 'static>), row_ty.clone(), true)?;
            Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: false, array: rows.clone() })
        },
        _ => {
            inMatrix
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArray)
}

pub(crate) fn transposeArray(mut inArray: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outArray: Arc<DAE::Exp>;
    let mut outWasTransposed: bool;
    (outArray, outWasTransposed) = (::match_deref::match_deref! { match &(inArray.clone()) {
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Cons { head: dim2, tail: rest_dims } } }, scalar: _, array: Deref @ metamodelica::List::Nil } => {
            (Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: metamodelica::cons(dim2.clone(), metamodelica::cons(dim1.clone(), rest_dims.clone())) }), scalar: false, array: metamodelica::nil() }), true)
        },
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Cons { head: dim2, tail: rest_dims } } }, scalar: _, array: expl } => {
            let mut row_ty: Arc<DAE::Type>;
            let mut matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
            let mut expl = (*expl).clone();
            row_ty = Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: metamodelica::cons(dim1.clone(), rest_dims.clone()) });
            matrix = List::map(expl.clone(), (std::sync::Arc::new(getArrayOrMatrixContents) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?;
            matrix = List::transposeList(matrix.clone())?;
            expl = List::map2(matrix.clone(), (std::sync::Arc::new(fnptr!(makeArray, Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<DAE::Type>, bool) -> Result<Arc<DAE::Exp>> + 'static>), row_ty.clone(), true)?;
            (Arc::new(DAE::Exp::ARRAY { ty: Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: metamodelica::cons(dim2.clone(), metamodelica::cons(dim1.clone(), rest_dims.clone())) }), scalar: false, array: expl.clone() }), true)
        },
        Deref @ DAE::Exp::MATRIX { matrix, ty: Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::List::Cons { head: dim1, tail: Deref @ metamodelica::List::Cons { head: dim2, tail: Deref @ metamodelica::List::Nil } } }, .. } => {
            let mut i: i32;
            let mut matrix = (*matrix).clone();
            let mut ty = (*ty).clone();
            matrix = List::transposeList(matrix.clone())?;
            ty = Arc::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim2.clone(), dim1.clone()] });
            i = (matrix.clone().len() as i32);
            (Arc::new(DAE::Exp::MATRIX { ty: ty.clone(), integer: i.clone(), matrix: matrix.clone() }), true)
        },
        _ => {
            (inArray, false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outArray, outWasTransposed))
}

pub fn getCrefFromCrefOrAsub(mut exp: Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> {
    let mut cr: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    cr = (::match_deref::match_deref! { match &(exp) {
        Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. } => {
            cr = (*__esc_cr).clone();
            cr.clone()
        },
        Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, .. } => {
            cr = (*__esc_cr).clone();
            cr.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cr)
}

pub fn arrayElements(mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExp: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut crl: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            crl = ComponentReference::expandCref(cr.clone(), false);
            expl = List::map(crl.clone(), (std::sync::Arc::new(crefExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>))?;
            expl.clone()
        },
        Deref @ DAE::Exp::ARRAY { array: expl, ty: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            List::mapFlat(expl.clone(), (std::sync::Arc::new(arrayElements) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?
        },
        Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
            expl.clone()
        },
        Deref @ DAE::Exp::MATRIX { matrix: mat, .. } => {
            List::flatten(mat.clone())?
        },
        _ => {
            list![inExp]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn arrayContent(mut inExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outContent: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let __pa0 = ::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::ARRAY { array: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outContent = __pa0.clone();
    Ok(outContent)
}

pub fn fromAbsynExp(mut inAExp: Arc<Absyn::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outDExp: Arc<DAE::Exp>;
    outDExp = (::match_deref::match_deref! { match &(inAExp.clone()) {
        Deref @ Absyn::Exp::INTEGER { value: i } => {
            Arc::new(DAE::Exp::ICONST { integer: i.clone() })
        },
        Deref @ Absyn::Exp::REAL { value: s } => {
            let mut r: metamodelica::Real;
            r = stringReal((s.clone()).clone())?;
            Arc::new(DAE::Exp::RCONST { real: r.clone() })
        },
        Deref @ Absyn::Exp::BOOL { value: b } => {
            Arc::new(DAE::Exp::BCONST { bool: b.clone() })
        },
        Deref @ Absyn::Exp::STRING { value: s } => {
            Arc::new(DAE::Exp::SCONST { string: (s.clone()).clone() })
        },
        Deref @ Absyn::Exp::CREF { componentRef: acr } => {
            let mut cr: Arc<DAE::ComponentRef>;
            let mut e: Arc<DAE::Exp>;
            cr = ComponentReference::toExpCref(acr.clone())?;
            e = makeCrefExp(cr.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
            e.clone()
        },
        Deref @ Absyn::Exp::BINARY { exp1: ae1, op: aop, exp2: ae2 } => {
            let mut e: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            op = fromAbsynOperator(aop.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
            e1 = fromAbsynExp(ae1.clone())?;
            e2 = fromAbsynExp(ae2.clone())?;
            e = Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() });
            e.clone()
        },
        Deref @ Absyn::Exp::UNARY { op: aop, exp: ae } => {
            let mut e: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            op = fromAbsynOperator(aop.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
            e = fromAbsynExp(ae.clone())?;
            e = Arc::new(DAE::Exp::UNARY { operator: op.clone(), exp: e.clone() });
            e.clone()
        },
        Deref @ Absyn::Exp::LBINARY { exp1: ae1, op: aop, exp2: ae2 } => {
            let mut e: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            op = fromAbsynOperator(aop.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
            e1 = fromAbsynExp(ae1.clone())?;
            e2 = fromAbsynExp(ae2.clone())?;
            e = Arc::new(DAE::Exp::LBINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() });
            e.clone()
        },
        Deref @ Absyn::Exp::LUNARY { op: aop, exp: ae } => {
            let mut e: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            op = fromAbsynOperator(aop.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
            e = fromAbsynExp(ae.clone())?;
            e = Arc::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e.clone() });
            e.clone()
        },
        Deref @ Absyn::Exp::RELATION { exp1: ae1, op: aop, exp2: ae2 } => {
            let mut e: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            op = fromAbsynOperator(aop.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
            e1 = fromAbsynExp(ae1.clone())?;
            e2 = fromAbsynExp(ae2.clone())?;
            e = Arc::new(DAE::Exp::RELATION { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone(), index: 0, optionExpisASUB: None });
            e.clone()
        },
        ae @ Deref @ Absyn::Exp::IFEXP { .. } => {
            let mut ae1: Arc<Absyn::Exp>;
            let mut ae2: Arc<Absyn::Exp>;
            let mut cond: Arc<Absyn::Exp>;
            let mut e: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(AbsynUtil::canonIfExp(ae.clone())?) {
                Deref @ Absyn::Exp::IFEXP { ifExp: __pa0, trueBranch: __pa1, elseBranch: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cond = __pa0.clone();
            ae1 = __pa1.clone();
            ae2 = __pa2.clone();
            e = fromAbsynExp(cond.clone())?;
            e1 = fromAbsynExp(ae1.clone())?;
            e2 = fromAbsynExp(ae2.clone())?;
            e = Arc::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: e1.clone(), expElse: e2.clone() });
            e.clone()
        },
        Deref @ Absyn::Exp::CALL { function_: acr, functionArgs: fargs, .. } => {
            let mut p: Arc<Absyn::Path>;
            let mut e: Arc<DAE::Exp>;
            let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            exps = fargsToExps(fargs.clone())?;
            p = AbsynUtil::crefToPath(acr.clone())?;
            e = Arc::new(DAE::Exp::CALL { path: p.clone(), expLst: exps.clone(), attr: DAE::callAttrBuiltinOther().clone() });
            e.clone()
        },
        Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: acr, functionArgs: fargs } => {
            let mut p: Arc<Absyn::Path>;
            let mut e: Arc<DAE::Exp>;
            let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            exps = fargsToExps(fargs.clone())?;
            p = AbsynUtil::crefToPath(acr.clone())?;
            e = Arc::new(DAE::Exp::PARTEVALFUNCTION { path: p.clone(), expList: exps.clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone(), origType: DAE::T_UNKNOWN_DEFAULT().clone() });
            e.clone()
        },
        Deref @ Absyn::Exp::ARRAY { arrayExp: aexps } => {
            let mut e: Arc<DAE::Exp>;
            let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            exps = List::map(aexps.clone(), (std::sync::Arc::new(fromAbsynExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
            e = Arc::new(DAE::Exp::ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), scalar: false, array: exps.clone() });
            e.clone()
        },
        Deref @ Absyn::Exp::MATRIX { matrix: aexpslst } => {
            let mut i: i32;
            let mut e: Arc<DAE::Exp>;
            let mut expslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
            expslst = List::mapList(aexpslst.clone(), (std::sync::Arc::new(fromAbsynExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
            i = (listHead(expslst.clone())?.len() as i32);
            e = Arc::new(DAE::Exp::MATRIX { ty: DAE::T_UNKNOWN_DEFAULT().clone(), integer: i.clone(), matrix: expslst.clone() });
            e.clone()
        },
        Deref @ Absyn::Exp::RANGE { start: ae1, step: aoe, stop: ae2 } => {
            let mut e: Arc<DAE::Exp>;
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut oe: Option<Arc<DAE::Exp>>;
            e1 = fromAbsynExp(ae1.clone())?;
            e2 = fromAbsynExp(ae2.clone())?;
            oe = fromAbsynExpOpt(aoe.clone())?;
            e = Arc::new(DAE::Exp::RANGE { ty: DAE::T_UNKNOWN_DEFAULT().clone(), start: e1.clone(), step: oe.clone(), stop: e2.clone() });
            e.clone()
        },
        Deref @ Absyn::Exp::TUPLE { expressions: aexps } => {
            let mut e: Arc<DAE::Exp>;
            let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            exps = List::map(aexps.clone(), (std::sync::Arc::new(fromAbsynExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
            e = Arc::new(DAE::Exp::TUPLE { PR: exps.clone() });
            e.clone()
        },
        _ => {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.fromAbsynExp: Unhandled expression: ")); __mm_s.push_str(&*Dump::printExpStr(inAExp)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDExp)
}

pub(crate) fn fargsToExps(mut inFargs: Arc<Absyn::FunctionArgs>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    outExps = 'mc: {
        let __mc_input = inFargs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: aexps, argNames: Deref @ metamodelica::List::Nil } => {
                    let mut exps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    exps = List::map(aexps.clone(), (std::sync::Arc::new(fromAbsynExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<DAE::Exp>> + 'static>))?;
                    Ok(exps.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: _, argNames: _ } => {
                    metamodelica::print((literal!("Expression.fargsToExps: Named arguments are not handled!\n")).clone());
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExps)
}

fn fromAbsynExpOpt(mut aoe: Option<Arc<Absyn::Exp>>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut oe: Option<Arc<DAE::Exp>>;
    oe = (::match_deref::match_deref! { match &(aoe) {
        None => {
            None
        },
        Some(ae) => {
            let mut e: Arc<DAE::Exp>;
            e = fromAbsynExp(ae.clone())?;
            Some(e.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oe)
}

fn fromAbsynOperator(mut aop: Absyn::Operator, mut ty: Arc<DAE::Type>) -> Result<DAE::Operator> {
    let mut op: DAE::Operator;
    op = (match aop.clone() {
        Absyn::Operator::ADD { .. } => DAE::Operator::ADD { ty: ty },
        Absyn::Operator::SUB { .. } => DAE::Operator::SUB { ty: ty },
        Absyn::Operator::MUL { .. } => DAE::Operator::MUL { ty: ty },
        Absyn::Operator::DIV { .. } => DAE::Operator::DIV { ty: ty },
        Absyn::Operator::POW { .. } => DAE::Operator::POW { ty: ty },
        Absyn::Operator::UMINUS { .. } => DAE::Operator::UMINUS { ty: ty },
        Absyn::Operator::AND { .. } => DAE::Operator::AND { ty: ty },
        Absyn::Operator::OR { .. } => DAE::Operator::OR { ty: ty },
        Absyn::Operator::NOT { .. } => DAE::Operator::NOT { ty: ty },
        Absyn::Operator::LESS { .. } => DAE::Operator::LESS { ty: ty },
        Absyn::Operator::LESSEQ { .. } => DAE::Operator::LESSEQ { ty: ty },
        Absyn::Operator::GREATER { .. } => DAE::Operator::GREATER { ty: ty },
        Absyn::Operator::GREATEREQ { .. } => DAE::Operator::GREATEREQ { ty: ty },
        Absyn::Operator::EQUAL { .. } => DAE::Operator::EQUAL { ty: ty },
        Absyn::Operator::NEQUAL { .. } => DAE::Operator::NEQUAL { ty: ty },
        _ => {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.fromAbsynOperator: Unhandled operator: ")); __mm_s.push_str(&*Dump::opSymbol(aop)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            bail!("fail")
        },
    });
    Ok(op)
}

pub fn replaceDerOpInExp(mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    (outExp, _) = traverseExpBottomUp(inExp, (std::sync::Arc::new(fnptr!(replaceDerOpInExpTraverser, Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>) -> Result<(Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>)> + 'static>), None)?;
    Ok(outExp)
}

pub fn replaceDerOpInExpCond(mut e: Arc<DAE::Exp>, mut cr: Option<Arc<DAE::ComponentRef>>) -> Result<(Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outCr: Option<Arc<DAE::ComponentRef>>;
    (outExp, outCr) = traverseExpBottomUp(e, (std::sync::Arc::new(fnptr!(replaceDerOpInExpTraverser, Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>) -> Result<(Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>)> + 'static>), cr)?;
    Ok((outExp, outCr))
}

pub(crate) fn replaceDerOpInExpTraverser(mut e: Arc<DAE::Exp>, mut optCr: Option<Arc<DAE::ComponentRef>>) -> (Arc<DAE::Exp>, Option<Arc<DAE::ComponentRef>>) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outCr: Option<Arc<DAE::ComponentRef>>;
    (outExp, outCr) = 'mc: {
        let __mc_input = (e.clone(), optCr.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, Some(cref)) => {
                    let mut derCr: Arc<DAE::ComponentRef>;
                    let mut cref_exp: Arc<DAE::Exp>;
                    derCr = ComponentReference::crefPrefixDer(cr.clone());
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(derCr.clone(), cref.clone())?) else { bail!("pattern mismatch") };
                    cref_exp = crefExp(derCr.clone())?;
                    Ok((cref_exp.clone(), optCr.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, None) => {
                    let mut cref_exp: Arc<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    cr = ComponentReference::crefPrefixDer(cr.clone());
                    cref_exp = crefExp(cr.clone())?;
                    Ok((cref_exp.clone(), None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((e.clone(), optCr.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outCr)
}

pub fn makeBinaryExp(mut inLhs: Arc<DAE::Exp>, mut inOp: DAE::Operator, mut inRhs: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = Arc::new(DAE::Exp::BINARY { exp1: inLhs, operator: inOp, exp2: inRhs });
    outExp
}

pub(crate) fn checkExpDimensionSizes(mut dim: Arc<DAE::Exp>) -> bool {
    let mut value: bool;
    value = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ DAE::Exp::ICONST { .. } => var_field!((*dim).integer, DAE::Exp::ICONST).clone() > 0,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    value
}

pub(crate) fn checkDimensionSizes(mut dim: Arc<DAE::Dimension>) -> Result<bool> {
    let mut value: bool;
    value = (::match_deref::match_deref! { match &(dim) {
        Deref @ DAE::Dimension::DIM_INTEGER { .. } => true,
        Deref @ DAE::Dimension::DIM_ENUM { .. } => true,
        Deref @ DAE::Dimension::DIM_BOOLEAN { .. } => true,
        Deref @ DAE::Dimension::DIM_EXP { .. } => true,
        Deref @ DAE::Dimension::DIM_UNKNOWN { .. } => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(value)
}

pub fn dimensionsList(mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Arc<metamodelica::List<i32>> {
    let mut outValues: Arc<metamodelica::List<i32>>;
    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outValues = 'mc: {
        let __mc_input = inDims.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut dims: Arc<metamodelica::List<i32>> = dims.clone();
                    let true = (List::all(inDims.clone(), (std::sync::Arc::new(checkDimensionSizes) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    dims = List::map(inDims.clone(), (std::sync::Arc::new(dimensionSizeAll) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Dimension>) -> Result<i32> + 'static>))?;
                    Ok((dims.clone(), dims.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { dims = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outValues
}

pub fn hasZeroDimension(mut inDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> bool {
    let mut hasZeroDimension: bool = false;
    let mut intDims: Arc<metamodelica::List<i32>>;
    if inDims.clone().is_empty() {
        hasZeroDimension = true;
        return hasZeroDimension.clone();
    }
    intDims = dimensionsList(inDims);
    for mut dim in &*intDims {
        let mut dim = dim.clone();
        if dim.clone() == 0 {
            hasZeroDimension = true;
            break;
        }
    }
    hasZeroDimension
}

pub fn expDimensionsList(mut inDims: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Arc<metamodelica::List<i32>> {
    let mut outValues: Arc<metamodelica::List<i32>>;
    let mut dims: Arc<metamodelica::List<i32>> = metamodelica::nil();
    outValues = 'mc: {
        let __mc_input = inDims.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut dims: Arc<metamodelica::List<i32>> = dims.clone();
                    let true = (List::all(inDims.clone(), (std::sync::Arc::new(fnptr!(checkExpDimensionSizes, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    dims = List::map(inDims.clone(), (std::sync::Arc::new(expInt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<i32> + 'static>))?;
                    Ok((dims.clone(), dims.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { dims = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    outValues
}

pub fn isCrefListWithEqualIdents(mut iExpressions: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> bool {
    let mut oCrefWithEqualIdents: bool;
    let mut tmpCrefWithEqualIdents: bool = false;
    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut head: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut headCref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    oCrefWithEqualIdents = 'mc: {
        let __mc_input = iExpressions.clone();
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: head, tail: _ } => {
                    let mut crefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = crefs.clone();
                    let mut headCref: Arc<DAE::ComponentRef> = headCref.clone();
                    let mut tmpCrefWithEqualIdents: bool = tmpCrefWithEqualIdents.clone();
                    let true = (List::all(iExpressions.clone(), (std::sync::Arc::new(fnptr!(isCref, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?) else { bail!("pattern mismatch") };
                    crefs = List::map(iExpressions.clone(), (std::sync::Arc::new(expCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
                    headCref = expCref(head.clone())?;
                    tmpCrefWithEqualIdents = List::all(crefs.clone(), (std::sync::Arc::new({ let __pe_b1 = headCref.clone(); move |__pe_a0| ComponentReferenceBasics::crefEqualWithoutLastSubs(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
                    Ok((tmpCrefWithEqualIdents, crefs.clone(), headCref.clone(), tmpCrefWithEqualIdents.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { crefs = __wb0; headCref = __wb1; tmpCrefWithEqualIdents = __wb2; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    oCrefWithEqualIdents
}

pub(crate) fn renameExpCrefIdent(mut inExp: Arc<DAE::Exp>, mut inTpl: (ArcStr, ArcStr)) -> (Arc<DAE::Exp>, (ArcStr, ArcStr)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outTpl: (ArcStr, ArcStr);
    (outExp, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, identType: ty1, subscriptLst: Deref @ metamodelica::List::Nil }, ty: ty2 }, (from, to)) => {
            let mut exp: Arc<DAE::Exp>;
            exp = if (stringEq((name.clone()).clone(), (from.clone()).clone())) {Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (to.clone()).clone(), identType: ty1.clone(), subscriptLst: metamodelica::nil() }), ty: ty2.clone() })} else {inExp};
            (exp.clone(), inTpl)
        },
        _ => {
            (inExp, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outTpl)
}

pub fn emptyToWild(mut exp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type>;
    ty = r#typeof(exp.clone())?;
    outExp = if (Types::isZeroLengthArray(ty.clone())?) {Arc::new(DAE::Exp::CREF { componentRef: openmodelica_frontend_types::DAE::ComponentRef::interned_WILD(), ty: ty })} else {exp};
    Ok(outExp)
}

pub(crate) fn makeVectorCall(mut exp: Arc<DAE::Exp>, mut tp: Arc<DAE::Type>) -> Arc<DAE::Exp> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = makePureBuiltinCall((literal!("vector")).clone(), list![exp], tp);
    outExp
}

pub fn expandCrefs(mut inExp: Arc<DAE::Exp>, mut expandRecord: bool, mut dummy: i32) -> Result<(Arc<DAE::Exp>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut dummy: i32 = dummy;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { ty: arr_ty @ Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            let mut exp_lst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut exp: Arc<DAE::Exp>;
            exp_lst = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut cr in (ComponentReference::expandCref(var_field!((*inExp).componentRef, DAE::Exp::CREF).clone(), expandRecord)).into_iter().cloned() {
            let __x = makeCrefExp(cr.clone(), var_field!((**arr_ty).ty, DAE::Type::T_ARRAY).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            exp = listToArray(exp_lst.clone(), var_field!((**arr_ty).dims, DAE::Type::T_ARRAY).clone())?;
            exp.clone()
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, dummy))
}

pub fn expandExpression(mut inExp: Arc<DAE::Exp>, mut expandRecord: bool) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outExps: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    outExps = ({
        let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ } => {
            let mut crlst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
            crlst = ComponentReference::expandCref(cr.clone(), expandRecord);
            outExps = List::map(crlst.clone(), (std::sync::Arc::new(crefToExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>))?;
            outExps
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, .. } => {
            expl = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut exp in (expandExpression(var_field!((*inExp).exp, DAE::Exp::UNARY).clone(), expandRecord)?).into_iter().cloned() {
            let __x = Arc::new(DAE::Exp::UNARY { operator: var_field!((*inExp).operator, DAE::Exp::UNARY).clone(), exp: exp.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            expl.clone()
        },
        Deref @ DAE::Exp::BINARY { .. } => {
            let mut expl1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut e1: Arc<DAE::Exp>;
            let mut e2: Arc<DAE::Exp>;
            let mut op: DAE::Operator;
            op = var_field!((*inExp).operator, DAE::Exp::BINARY).clone();
            expl1 = expandExpression(var_field!((*inExp).exp1, DAE::Exp::BINARY).clone(), expandRecord)?;
            expl2 = expandExpression(var_field!((*inExp).exp2, DAE::Exp::BINARY).clone(), expandRecord)?;
            if (expl1.clone().len() as i32) != (expl2.clone().len() as i32) {
                bail!("fail");
            }
            e1 = (expl1.clone()).get(1)?;
            e2 = (expl1.clone()).get(2)?;
            for mut i in 1..=(expl1.clone().len() as i32) {
                e1 = (expl1.clone()).get(i.clone())?;
                e2 = (expl2.clone()).get(i.clone())?;
                expl = metamodelica::cons(Arc::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() }), expl.clone());
            }
            expl = expl.clone().reverse();
            expl.clone()
        },
        Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl } => {
            let mut expl = (*expl).clone();
            expl = List::mapFlat(expl.clone(), (std::sync::Arc::new({ let __pe_b1 = expandRecord; move |__pe_a0| expandExpression(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> + 'static>))?;
            expl.clone()
        },
        _ => {
            let mut msg: ArcStr;
            msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Expression.expandExpression failed for ")); __mm_s.push_str(&*printExpStr(inExp)?); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(msg.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok(outExps)
}

pub fn extendArrExp(mut inExp: Arc<DAE::Exp>, mut inExpanded: bool) -> (Arc<DAE::Exp>, bool) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outExpanded: bool;
    (outExp, outExpanded) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                outExp => {
                    let mut exp: Arc<DAE::Exp>;
                    let mut b: bool;
                    (exp, b) = traverseExpBottomUp(inExp.clone(), (std::sync::Arc::new(traversingextendArrExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), false)?;
                    Ok((exp.clone(), b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inExpanded))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outExpanded)
}

fn traversingextendArrExp(mut inExp: Arc<DAE::Exp>, mut inExpanded: bool) -> Result<(Arc<DAE::Exp>, bool)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outExpanded: bool;
    (outExp, outExpanded) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { ty: ty @ Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: id, tail: Deref @ metamodelica::List::Cons { head: jd, tail: Deref @ metamodelica::List::Nil } }, .. }, .. } => {
            let mut i: i32;
            let mut j: i32;
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut e: Arc<DAE::Exp>;
            let mut mat: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
            i = dimensionSize(id.clone())?;
            j = dimensionSize(jd.clone())?;
            expl = expandExpression(inExp, false)?;
            mat = makeMatrix(expl.clone(), j.clone())?;
            e = Arc::new(DAE::Exp::MATRIX { ty: ty.clone(), integer: i.clone(), matrix: mat.clone() });
            (e.clone(), true)
        },
        Deref @ DAE::Exp::CREF { ty: ty @ Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut e: Arc<DAE::Exp>;
            expl = expandExpression(inExp, false)?;
            e = Arc::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: true, array: expl.clone() });
            (e.clone(), true)
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, ty: ty @ Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: name }, .. } } => {
            let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut e: Arc<DAE::Exp>;
            let mut field_names: Arc<metamodelica::List<ArcStr>>;
            expl = List::map1(varLst.clone(), (std::sync::Arc::new(generateCrefsExpFromExpVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Exp>> + 'static>), cr.clone())?;
            let true = (!(expl.clone().is_empty())) else { bail!("pattern mismatch") };
            field_names = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (varLst.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            e = Arc::new(DAE::Exp::RECORD { path: name.clone(), exps: expl.clone(), comp: field_names.clone(), ty: ty.clone() });
            (e, _) = traverseExpBottomUp(e.clone(), (std::sync::Arc::new(traversingextendArrExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool)> + 'static>), true)?;
            (e.clone(), true)
        },
        _ => {
            (inExp, inExpanded)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outExpanded))
}

fn makeMatrix(mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut n: i32) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>> {
    let mut res: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>;
    let mut col: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut r: i32;
    res = metamodelica::nil();
    col = metamodelica::nil();
    r = n;
    for mut e in &*expl {
        let mut e = e.clone();
        r = r - 1;
        col = metamodelica::cons(e.clone(), col.clone());
        if r == 0 {
            res = metamodelica::cons(col.clone().reverse(), res.clone());
            col = metamodelica::nil();
            r = n;
        }
    }
    Error::assertionOrAddSourceMessage(col.is_empty(), Error::INTERNAL_ERROR.clone(), list![(literal!("Expression.makeMatrix failed")).clone()], metamodelica::sourceInfo!("FrontEnd/Expression.mo"))?;
    res = res.reverse();
    Ok(res)
}

pub fn rangesToSubscripts(mut inRangelist: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>> {
    let mut outSubslst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>;
    outSubslst = List::allCombinations(inRangelist, None, Absyn::dummyInfo.clone())?;
    Ok(outSubslst)
}

pub(crate) fn expandSubscript(mut inSubscript: Arc<DAE::Subscript>, mut inDimension: Arc<DAE::Dimension>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> {
    let mut outSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    outSubscripts = (::match_deref::match_deref! { match &(inSubscript.clone()) {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::RANGE { .. } } => {
            ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
        for mut e in (expandRange(var_field!((*inSubscript).exp, DAE::Subscript::INDEX).clone())?).into_iter().cloned() {
            let __x = Arc::new(DAE::Subscript::INDEX { exp: e.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ARRAY { .. } } => {
            expandSliceExp(var_field!((*inSubscript).exp, DAE::Subscript::INDEX).clone())?
        },
        Deref @ DAE::Subscript::INDEX { .. } => {
            list![inSubscript]
        },
        Deref @ DAE::Subscript::WHOLEDIM { .. } => {
            expandDimension(inDimension)?
        },
        Deref @ DAE::Subscript::SLICE { .. } => {
            expandSliceExp(var_field!((*inSubscript).exp, DAE::Subscript::SLICE).clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubscripts)
}

pub(crate) fn expandDimension(mut inDimension: Arc<DAE::Dimension>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> {
    let mut outSubscript: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    outSubscript = (::match_deref::match_deref! { match &(inDimension) {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: dim_int } => {
            dimensionSizeSubscripts(dim_int.clone())
        },
        Deref @ DAE::Dimension::DIM_ENUM { enumTypeName: enum_ty, literals: enum_lits, .. } => {
            let mut enum_expl: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            enum_expl = makeEnumLiterals(enum_ty.clone(), enum_lits.clone())?;
            List::map(enum_expl.clone(), (std::sync::Arc::new(fnptr!(makeIndexSubscript, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Subscript>> + 'static>))?
        },
        Deref @ DAE::Dimension::DIM_BOOLEAN { .. } => {
            metamodelica::cons(Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::BCONST { bool: false }) }), metamodelica::cons(Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::BCONST { bool: true }) }), metamodelica::nil()))
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSubscript)
}

pub(crate) fn expandSliceExp(mut inSliceExp: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> {
    let mut outSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    outSubscripts = (::match_deref::match_deref! { match &(inSliceExp.clone()) {
        Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
            List::map(expl.clone(), (std::sync::Arc::new(fnptr!(makeIndexSubscript, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Subscript>> + 'static>))?
        },
        Deref @ DAE::Exp::RANGE { .. } => {
            List::map(expandRange(inSliceExp)?, (std::sync::Arc::new(fnptr!(makeIndexSubscript, Arc<DAE::Exp>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<DAE::Subscript>> + 'static>))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSubscripts)
}

pub fn dimensionSizesSubscripts(mut inDimSizes: Arc<metamodelica::List<i32>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>> {
    let mut outSubscripts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>;
    outSubscripts = List::map(inDimSizes, (std::sync::Arc::new(fnptr!(dimensionSizeSubscripts, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> + 'static>))?;
    Ok(outSubscripts)
}

pub fn dimensionSizesSubcriptsOpt(mut inDimSizes: Arc<metamodelica::List<Option<i32>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>> {
    let mut outSubscripts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Subscript>>>>>;
    outSubscripts = List::mapOption(inDimSizes, (std::sync::Arc::new(fnptr!(dimensionSizeSubscripts, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<Arc<metamodelica::List<Arc<DAE::Subscript>>>> + 'static>))?;
    Ok(outSubscripts)
}

pub fn dimensionSizeSubscripts(mut inDimSize: i32) -> Arc<metamodelica::List<Arc<DAE::Subscript>>> {
    let mut outSubscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>;
    outSubscripts = ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
        for mut i in (1..=inDimSize).into_iter() {
            let __x = Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i.clone() }) });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outSubscripts
}

pub fn createResidualExp(mut inExp1: Arc<DAE::Exp>, mut inExp2: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    let mut resExp: Arc<DAE::Exp>;
    let mut iExp1: Arc<DAE::Exp>;
    let mut iExp2: Arc<DAE::Exp>;
    (iExp1, iExp2) = createResidualExp2(inExp1, inExp2)?;
    resExp = 'mc: {
        let __mc_input = (iExp1.clone(), iExp2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::RCONST { real: __rlit_3 }) => {
                    if !(__rlit_3.eq(&metamodelica::OrderedFloat((0.0) as f64))) { bail!("guard") }
                    Ok(iExp1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::ICONST { integer: 0 }) => {
                    Ok(iExp1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: __rlit_4 }, _) => {
                    if !(__rlit_4.eq(&metamodelica::OrderedFloat((0.0) as f64))) { bail!("guard") }
                    Ok(iExp2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: 0 }, _) => {
                    Ok(iExp2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut res1: Arc<DAE::Exp>;
                    let mut res2: Arc<DAE::Exp>;
                    let mut N1: Arc<DAE::Exp>;
                    let mut D1: Arc<DAE::Exp>;
                    let mut N2: Arc<DAE::Exp>;
                    let mut D2: Arc<DAE::Exp>;
                    let mut explst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut explst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
                    let mut ty: Arc<DAE::Type>;
                    ty = r#typeof(iExp1.clone())?;
                    let true = (Types::isIntegerOrRealOrSubTypeOfEither(ty.clone())) else { bail!("pattern mismatch") };
                    (N1, D1) = makeFraction(iExp1.clone())?;
                    (N2, D2) = makeFraction(iExp2.clone())?;
                    res1 = ExpressionSimplify::simplifySumOperatorExpression(N1.clone(), DAE::Operator::MUL { ty: ty.clone() }, D2.clone())?;
                    res2 = ExpressionSimplify::simplifySumOperatorExpression(N2.clone(), DAE::Operator::MUL { ty: ty.clone() }, D1.clone())?;
                    explst = terms(iExp1.clone())?;
                    explst1 = terms(iExp2.clone())?;
                    if isConst(res1.clone())? || (explst1.clone().len() as i32) + 1 > (explst.clone().len() as i32) {
                        res = expSub(res2.clone(), res1.clone())?;
                    } else {
                        res = expSub(res1.clone(), res2.clone())?;
                    }
                    (res, _) = ExpressionSimplify::simplify(res.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut ty: Arc<DAE::Type>;
                    ty = r#typeof(iExp1.clone())?;
                    let true = (Types::isEnumeration(ty.clone())) else { bail!("pattern mismatch") };
                    res = expSub(iExp1.clone(), iExp2.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut ty: Arc<DAE::Type>;
                    ty = r#typeof(iExp1.clone())?;
                    let true = (Types::isBooleanOrSubTypeBoolean(ty.clone())) else { bail!("pattern mismatch") };
                    res = Arc::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: ty.clone() }, exp: Arc::new(DAE::Exp::RELATION { exp1: iExp1.clone(), operator: DAE::Operator::EQUAL { ty: ty.clone() }, exp2: iExp2.clone(), index: -1, optionExpisASUB: None }) });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut res: Arc<DAE::Exp>;
                    let mut ty: Arc<DAE::Type>;
                    ty = r#typeof(iExp1.clone())?;
                    let true = (Types::isStringOrSubTypeString(ty.clone())) else { bail!("pattern mismatch") };
                    res = Arc::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: ty.clone() }, exp: Arc::new(DAE::Exp::RELATION { exp1: iExp1.clone(), operator: DAE::Operator::EQUAL { ty: ty.clone() }, exp2: iExp2.clone(), index: -1, optionExpisASUB: None }) });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut res: Arc<DAE::Exp>;
                    res = expSub(iExp1.clone(), iExp2.clone())?;
                    (res, _) = ExpressionSimplify::simplify(res.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(resExp)
}

pub fn makeFraction(mut iExp: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>)> {
    let mut n: Arc<DAE::Exp>;
    let mut d: Arc<DAE::Exp>;
    let mut N: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut D: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut T: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    T = terms(iExp)?;
    T = ExpressionSimplify::simplifyList(T)?;
    (N, D) = moveDivToMul(T, metamodelica::nil(), metamodelica::nil())?;
    N = ExpressionSimplify::simplifyList(N)?;
    D = ExpressionSimplify::simplifyList(D)?;
    n = makeSum1(N, false)?;
    d = makeProductLst(D)?;
    (n, _) = ExpressionSimplify::simplify1(n)?;
    (d, _) = ExpressionSimplify::simplify1(d)?;
    Ok((n, d))
}

fn moveDivToMul(mut iExpLst: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut iExpLstAcc: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut iExpMuls: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<(Arc<metamodelica::List<Arc<DAE::Exp>>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(iExpLst) {
        Deref @ metamodelica::List::Nil => {
            return Ok((iExpLstAcc, iExpMuls))
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { operator: _, exp: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } }, tail: rest } => {
            let mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut elst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut elst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut rest = (*rest).clone();
            acc = List::map1(iExpLstAcc, (std::sync::Arc::new(expMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
            rest = List::map1(rest.clone(), (std::sync::Arc::new(expMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
            rest = ExpressionSimplify::simplifyList(rest.clone())?;
            { (iExpLst, iExpLstAcc, iExpMuls) = (rest.clone(), metamodelica::cons(negate(e1.clone())?, acc.clone()), metamodelica::cons(e2.clone(), iExpMuls)); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::UNARY { operator: _, exp: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_ARRAY_SCALAR { .. }, exp2: e2 } }, tail: rest } => {
            let mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut elst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut elst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut rest = (*rest).clone();
            acc = List::map1(iExpLstAcc, (std::sync::Arc::new(expMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
            rest = List::map1(rest.clone(), (std::sync::Arc::new(expMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
            rest = ExpressionSimplify::simplifyList(rest.clone())?;
            { (iExpLst, iExpLstAcc, iExpMuls) = (rest.clone(), metamodelica::cons(negate(e1.clone())?, acc.clone()), metamodelica::cons(e2.clone(), iExpMuls)); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 }, tail: rest } => {
            let mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut elst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut elst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut rest = (*rest).clone();
            acc = List::map1(iExpLstAcc, (std::sync::Arc::new(expMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
            rest = List::map1(rest.clone(), (std::sync::Arc::new(expMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
            rest = ExpressionSimplify::simplifyList(rest.clone())?;
            { (iExpLst, iExpLstAcc, iExpMuls) = (rest.clone(), metamodelica::cons(e1.clone(), acc.clone()), metamodelica::cons(e2.clone(), iExpMuls)); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_ARRAY_SCALAR { .. }, exp2: e2 }, tail: rest } => {
            let mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut elst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut elst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut rest = (*rest).clone();
            acc = List::map1(iExpLstAcc, (std::sync::Arc::new(expMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
            rest = List::map1(rest.clone(), (std::sync::Arc::new(expMul) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> + 'static>), e2.clone())?;
            rest = ExpressionSimplify::simplifyList(rest.clone())?;
            { (iExpLst, iExpLstAcc, iExpMuls) = (rest.clone(), metamodelica::cons(e1.clone(), acc.clone()), metamodelica::cons(e2.clone(), iExpMuls)); continue '__tco; }
        },
        Deref @ metamodelica::List::Cons { head: e, tail: rest } => {
            let mut elst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            let mut elst1: Arc<metamodelica::List<Arc<DAE::Exp>>>;
            { (iExpLst, iExpLstAcc, iExpMuls) = (rest.clone(), metamodelica::cons(e.clone(), iExpLstAcc), iExpMuls); continue '__tco; }
        },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

fn createResidualExp2(mut iExp1: Arc<DAE::Exp>, mut iExp2: Arc<DAE::Exp>) -> Result<(Arc<DAE::Exp>, Arc<DAE::Exp>)> {
    let mut oExp1: Arc<DAE::Exp> = iExp1.clone();
    let mut oExp2: Arc<DAE::Exp> = iExp2.clone();
    let mut con: bool = true;
    let mut con1: bool;
    let mut ii: i32 = 1;
    while con && ii < 15 {
        (oExp1, oExp2, con) = 'mc: {
        let __mc_input = oExp2.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(createResidualExp3(oExp1.clone(), oExp2.clone())) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e1 = __pa0.clone();
                    e2 = __pa1.clone();
                    (e1, _) = ExpressionSimplify::simplify1(e1.clone())?;
                    (e2, _) = ExpressionSimplify::simplify1(e2.clone())?;
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(createResidualExp3(oExp2.clone(), oExp1.clone())) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    e2 = __pa0.clone();
                    e1 = __pa1.clone();
                    (e1, _) = ExpressionSimplify::simplify1(e1.clone())?;
                    (e2, _) = ExpressionSimplify::simplify1(e2.clone())?;
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((oExp1.clone(), oExp2.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        (oExp1, oExp2, con1) = 'mc: {
        let __mc_input = oExp2.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let true = (isZero(oExp1.clone())?) else { bail!("pattern mismatch") };
                    (e1, e2) = makeFraction(oExp2.clone())?;
                    Ok((e1.clone(), oExp1.clone(), !(isOne(e2.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let true = (isZero(oExp2.clone())?) else { bail!("pattern mismatch") };
                    (e1, e2) = makeFraction(oExp1.clone())?;
                    Ok((e1.clone(), oExp2.clone(), !(isOne(e2.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let true = (isOne(oExp1.clone())) else { bail!("pattern mismatch") };
                    (e1, e2) = makeFraction(oExp2.clone())?;
                    Ok((e1.clone(), e2.clone(), !(isOne(e2.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut e1: Arc<DAE::Exp>;
                    let mut e2: Arc<DAE::Exp>;
                    let true = (isOne(oExp2.clone())) else { bail!("pattern mismatch") };
                    (e1, e2) = makeFraction(oExp1.clone())?;
                    Ok((e1.clone(), e2.clone(), !(isOne(e2.clone()))))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((oExp1.clone(), oExp2.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        con = con || con1;
        ii = ii + 1;
        if !(con) {
            (oExp1, con) = ExpressionSimplify::simplify1(oExp1.clone())?;
            (oExp2, con1) = ExpressionSimplify::simplify1(oExp2.clone())?;
            con = con || con1;
            ii = ii + 3;
        }
    }
    (oExp1, _) = ExpressionSimplify::simplify1(oExp1)?;
    (oExp2, _) = ExpressionSimplify::simplify1(oExp2)?;
    Ok((oExp1, oExp2))
}

pub fn createResidualExp3(mut iExp1: Arc<DAE::Exp>, mut iExp2: Arc<DAE::Exp>) -> (Arc<DAE::Exp>, Arc<DAE::Exp>, bool) {
    let mut oExp1: Arc<DAE::Exp>;
    let mut oExp2: Arc<DAE::Exp>;
    let mut con: bool;
    (oExp1, oExp2, con) = 'mc: {
        let __mc_input = (iExp1.clone(), iExp2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: s1 }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: s2 }, expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, .. }) => {
                    if !((s1.clone() == s2.clone() && createResidualExp4((s1.clone()).clone()))) { bail!("guard") }
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. }, Deref @ DAE::Exp::RCONST { real: __rlit_5 }) => {
                    if !(__rlit_5.eq(&metamodelica::OrderedFloat((0.0) as f64))) { bail!("guard") }
                    Ok((e1.clone(), iExp2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. }, e2) => {
                    if !((isConst(e2.clone())?)) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    e = expPow(iExp2.clone(), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
                    Ok((e1.clone(), e.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. }, e2) => {
                    if !((isConst(e2.clone())?)) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    let mut tp: Arc<DAE::Type>;
                    tp = r#typeof(iExp2.clone())?;
                    e = makePureBuiltinCall((literal!("exp")).clone(), list![iExp2.clone()], tp.clone());
                    Ok((e1.clone(), e.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. }, e2) => {
                    if !((isConst(e2.clone())?)) { bail!("guard") }
                    let mut e: Arc<DAE::Exp>;
                    e = expPow(Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(10.0_f64) }), iExp2.clone())?;
                    Ok((e1.clone(), e.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::RCONST { real: __rlit_6 }, tail: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } }, .. }, Deref @ DAE::Exp::RCONST { real: __rlit_7 }) => {
                    if !(__rlit_6.eq(&metamodelica::OrderedFloat((0.0) as f64)) && __rlit_7.eq(&metamodelica::OrderedFloat((0.0) as f64))) { bail!("guard") }
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 }, e2 @ Deref @ DAE::Exp::RCONST { real: __rlit_8 }) => {
                    if !(__rlit_8.eq(&metamodelica::OrderedFloat((0.0) as f64))) { bail!("guard") }
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: s1 }, expLst: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Nil }, .. }, operator: DAE::Operator::SUB { .. }, exp2: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: s2 }, expLst: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_9 }) => {
                    if !(__rlit_9.eq(&metamodelica::OrderedFloat((0.0) as f64)) && (s1.clone() == s2.clone() && createResidualExp4((s1.clone()).clone()))) { bail!("guard") }
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((iExp1.clone(), iExp2.clone(), false))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    (oExp1, oExp2, con)
}

fn createResidualExp4(mut f: ArcStr) -> bool {
    let mut resB: bool;
    resB = (::match_deref::match_deref! { match &(f) {
        Deref @ "sqrt" => true,
        Deref @ "exp" => true,
        Deref @ "log" => true,
        Deref @ "log10" => true,
        Deref @ "tanh" => true,
        Deref @ "sinh" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    resB
}

pub fn isAsubExp(mut expIn: Arc<DAE::Exp>) -> bool {
    let mut isAsub: bool;
    isAsub = (::match_deref::match_deref! { match &(expIn) {
        Deref @ DAE::Exp::ASUB { exp: _, sub: _ } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isAsub
}

pub(crate) fn typeCast(mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = Arc::new(DAE::Exp::CAST { ty: inType, exp: inExp });
    (outExp, _) = ExpressionSimplify::simplify1(outExp)?;
    Ok(outExp)
}

pub(crate) fn typeCastElements(mut inExp: Arc<DAE::Exp>, mut inType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type>;
    ty = r#typeof(inExp.clone())?;
    ty = Types::setArrayElementType(ty, inType);
    outExp = typeCast(inExp, ty)?;
    Ok(outExp)
}

pub fn expandRange(mut inRange: Arc<DAE::Exp>) -> Result<Arc<metamodelica::List<Arc<DAE::Exp>>>> {
    let mut outValues: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut start_exp: Arc<DAE::Exp>;
    let mut stop_exp: Arc<DAE::Exp>;
    let mut ostep_exp: Option<Arc<DAE::Exp>>;
    let mut istep: i32 = 0;
    let mut rstep: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut vals: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut enum_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut enum_type: Arc<Absyn::Path> = Arc::new(<Absyn::Path as ::std::default::Default>::default());
    let mut range_ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inRange.clone()) {
        Deref @ DAE::Exp::RANGE { start: __pa0, step: __pa1, stop: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    start_exp = __pa0.clone();
    ostep_exp = __pa1.clone();
    stop_exp = __pa2.clone();
    outValues = (::match_deref::match_deref! { match &((start_exp.clone(), stop_exp.clone())) {
        (Deref @ DAE::Exp::ICONST { .. }, Deref @ DAE::Exp::ICONST { .. }) => {
            let __pa0 = ::match_deref::match_deref! { match &(Util::getOptionOrDefault(ostep_exp, Arc::new(DAE::Exp::ICONST { integer: 1 }))) {
                Deref @ DAE::Exp::ICONST { integer: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            istep = __pa0.clone();
            ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut i in (List::intRange3(var_field!((*start_exp).integer, DAE::Exp::ICONST).clone(), istep, var_field!((*stop_exp).integer, DAE::Exp::ICONST).clone())?).into_iter().cloned() {
            let __x = Arc::new(DAE::Exp::ICONST { integer: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        (Deref @ DAE::Exp::RCONST { .. }, Deref @ DAE::Exp::RCONST { .. }) => {
            let __pa0 = ::match_deref::match_deref! { match &(Util::getOptionOrDefault(ostep_exp, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))) {
                Deref @ DAE::Exp::RCONST { real: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            rstep = __pa0.clone();
            ({
        let mut __acc: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
        for mut r in (ExpressionSimplify::simplifyRangeReal(var_field!((*start_exp).real, DAE::Exp::RCONST).clone(), rstep, var_field!((*stop_exp).real, DAE::Exp::RCONST).clone())).into_iter().cloned() {
            let __x = Arc::new(DAE::Exp::RCONST { real: r.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        (Deref @ DAE::Exp::BCONST { bool: false }, Deref @ DAE::Exp::BCONST { bool: true }) => list![start_exp, stop_exp],
        (Deref @ DAE::Exp::BCONST { bool: true }, Deref @ DAE::Exp::BCONST { bool: false }) => metamodelica::nil(),
        (Deref @ DAE::Exp::BCONST { .. }, Deref @ DAE::Exp::BCONST { .. }) => list![start_exp],
        (Deref @ DAE::Exp::ENUM_LITERAL { .. }, Deref @ DAE::Exp::ENUM_LITERAL { .. }) => {
            if var_field!((*start_exp).index, DAE::Exp::ENUM_LITERAL).clone() > var_field!((*stop_exp).index, DAE::Exp::ENUM_LITERAL).clone() {
                vals = metamodelica::nil();
            } else if var_field!((*start_exp).index, DAE::Exp::ENUM_LITERAL).clone() == var_field!((*stop_exp).index, DAE::Exp::ENUM_LITERAL).clone() {
                vals = list![start_exp];
            } else {
                let __pa0 = ::match_deref::match_deref! { match &(inRange) {
                    Deref @ DAE::Exp::RANGE { ty: __pa0, .. } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                range_ty = __pa0.clone();
                let (__pa1, __pa2) = ::match_deref::match_deref! { match &(Types::arrayElementType(range_ty)) {
                    Deref @ DAE::Type::T_ENUMERATION { path: __pa1, names: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                enum_type = __pa1.clone();
                enum_names = __pa2.clone();
                enum_names = List::sublist(enum_names, var_field!((*start_exp).index, DAE::Exp::ENUM_LITERAL).clone(), var_field!((*stop_exp).index, DAE::Exp::ENUM_LITERAL).clone() - var_field!((*start_exp).index, DAE::Exp::ENUM_LITERAL).clone() + 1)?;
                vals = makeEnumLiterals(enum_type, enum_names)?;
            }
            vals
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValues)
}

pub(crate) fn isScalarSubscript(mut sub: Arc<DAE::Subscript>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ DAE::Subscript::SLICE { .. } => isScalar(var_field!((*sub).exp, DAE::Subscript::SLICE).clone())?,
        Deref @ DAE::Subscript::INDEX { .. } => isScalar(var_field!((*sub).exp, DAE::Subscript::INDEX).clone())?,
        Deref @ DAE::Subscript::WHOLE_NONEXP { .. } => isScalar(var_field!((*sub).exp, DAE::Subscript::WHOLE_NONEXP).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isScalar(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ICONST { .. } => return Ok(true),
        Deref @ DAE::Exp::RCONST { .. } => return Ok(true),
        Deref @ DAE::Exp::SCONST { .. } => return Ok(true),
        Deref @ DAE::Exp::BCONST { .. } => return Ok(true),
        Deref @ DAE::Exp::CLKCONST { .. } => return Ok(true),
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => return Ok(true),
        Deref @ DAE::Exp::UNARY { .. } => { inExp = var_field!((*inExp).exp, DAE::Exp::UNARY).clone(); continue '__tco; },
        Deref @ DAE::Exp::LUNARY { .. } => { inExp = var_field!((*inExp).exp, DAE::Exp::LUNARY).clone(); continue '__tco; },
        Deref @ DAE::Exp::RELATION { .. } => return Ok(true),
        Deref @ DAE::Exp::ARRAY { .. } => return Ok(false),
        Deref @ DAE::Exp::MATRIX { .. } => return Ok(false),
        Deref @ DAE::Exp::RANGE { .. } => return Ok(false),
        Deref @ DAE::Exp::CAST { .. } => { inExp = var_field!((*inExp).exp, DAE::Exp::CAST).clone(); continue '__tco; },
        Deref @ DAE::Exp::SIZE { .. } => return Ok(isSome(var_field!((*inExp).sz, DAE::Exp::SIZE).clone())),
        _ => return Ok(Types::isSimpleType(r#typeof(inExp)?)),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn containsAnyCall(mut inExp: Arc<DAE::Exp>) -> Result<bool> {
    let mut outContainsCall: bool;
    (_, outContainsCall) = traverseExpTopDown(inExp, (std::sync::Arc::new(fnptr!(containsAnyCall_traverser, Arc<DAE::Exp>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, bool) -> Result<(Arc<DAE::Exp>, bool, bool)> + 'static>), false)?;
    Ok(outContainsCall)
}

fn containsAnyCall_traverser(mut inExp: Arc<DAE::Exp>, mut inContainsCall: bool) -> (Arc<DAE::Exp>, bool, bool) {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outContinue: bool;
    let mut outContainsCall: bool;
    outContainsCall = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CALL { .. } => true,
        _ => inContainsCall,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outContinue = !(outContainsCall);
    (outExp, outContinue, outContainsCall)
}

pub(crate) fn containsCallTo(mut inExp: Arc<DAE::Exp>, mut path: Arc<Absyn::Path>) -> Result<bool> {
    let mut outContainsCall: bool;
    let (_, (_, __pa0)) = traverseExpTopDown(inExp, (std::sync::Arc::new(fnptr!(containsCallTo_traverser, Arc<DAE::Exp>, (Arc<Absyn::Path>, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<Absyn::Path>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<Absyn::Path>, bool))> + 'static>), (path, false))?;
    outContainsCall = __pa0.clone();
    Ok(outContainsCall)
}

fn containsCallTo_traverser(mut inExp: Arc<DAE::Exp>, mut inTpl: (Arc<Absyn::Path>, bool)) -> (Arc<DAE::Exp>, bool, (Arc<Absyn::Path>, bool)) {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outContinue: bool = false;
    let mut outTpl: (Arc<Absyn::Path>, bool) = inTpl.clone();
    let mut containsCall: bool;
    let mut path: Arc<Absyn::Path>;
    (path, containsCall) = outTpl.clone();
    if containsCall {
        return (outExp.clone(), outContinue.clone(), outTpl.clone());
    }
    outContinue = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { .. } => AbsynUtil::pathEqual(path.clone(), var_field!((*inExp).path, DAE::Exp::CALL).clone()),
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(outContinue) {
        outTpl = (path, false);
    }
    (outExp, outContinue, outTpl)
}

pub fn rangeSize(mut inRange: Arc<DAE::Exp>) -> Result<i32> {
    let mut outSize: i32 = 0;
    outSize = (::match_deref::match_deref! { match &(inRange) {
        Deref @ DAE::Exp::RANGE { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: __esc_outSize }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => {
            outSize = (*__esc_outSize).clone();
            outSize.clone()
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: start }, step: None, stop: Deref @ DAE::Exp::ICONST { integer: stop }, .. } => {
            std::cmp::max(stop.clone() - start.clone(), 0)
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: start }, step: Some(Deref @ DAE::Exp::ICONST { integer: step }), stop: Deref @ DAE::Exp::ICONST { integer: stop }, .. } => {
            if step.clone() != 0 {
                outSize = std::cmp::max((((realDiv(metamodelica::OrderedFloat((stop.clone() - start.clone()) as f64), metamodelica::OrderedFloat((step.clone()) as f64))).floor()).0.floor() as i32) + 1, 0);
            } else {
                bail!("fail");
            }
            outSize
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outSize)
}

pub fn isInvariantExpNoTraverse(mut e: Arc<DAE::Exp>, mut b: bool) -> (Arc<DAE::Exp>, bool) {
    let mut e: Arc<DAE::Exp> = e;
    let mut b: bool = b;
    if !(b) {
        return (e.clone(), b.clone());
    }
    b = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::ICONST { .. } => true,
        Deref @ DAE::Exp::RCONST { .. } => true,
        Deref @ DAE::Exp::SCONST { .. } => true,
        Deref @ DAE::Exp::BCONST { .. } => true,
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => true,
        Deref @ DAE::Exp::BINARY { .. } => true,
        Deref @ DAE::Exp::UNARY { .. } => true,
        Deref @ DAE::Exp::LBINARY { .. } => true,
        Deref @ DAE::Exp::LUNARY { .. } => true,
        Deref @ DAE::Exp::RELATION { .. } => true,
        Deref @ DAE::Exp::IFEXP { .. } => true,
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::FULLYQUALIFIED { .. }, .. } => true,
        Deref @ DAE::Exp::PARTEVALFUNCTION { path: Deref @ Absyn::Path::FULLYQUALIFIED { .. }, .. } => true,
        Deref @ DAE::Exp::ARRAY { .. } => true,
        Deref @ DAE::Exp::MATRIX { .. } => true,
        Deref @ DAE::Exp::RANGE { .. } => true,
        Deref @ DAE::Exp::CONS { .. } => true,
        Deref @ DAE::Exp::LIST { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (e, b)
}

pub fn findCallIsInlineAfterIndexReduction(mut e: Arc<DAE::Exp>, mut res: bool) -> (Arc<DAE::Exp>, bool, bool) {
    let mut e: Arc<DAE::Exp> = e;
    let mut cont: bool;
    let mut res: bool = res;
    if !(res) {
        res = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { inlineType: DAE::InlineType::AFTER_INDEX_RED_INLINE { .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    cont = !(res);
    (e, cont, res)
}

pub fn tupleHead(mut exp: Arc<DAE::Exp>, mut prop: DAE::Properties) -> Result<(Arc<DAE::Exp>, DAE::Properties)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outProp: DAE::Properties;
    (outExp, outProp) = (::match_deref::match_deref! { match &((exp.clone(), prop.clone())) {
        (Deref @ DAE::Exp::TUPLE { PR: Deref @ metamodelica::List::Cons { head: _, tail: _ } }, DAE::Properties::PROP_TUPLE { .. }) => {
            (listHead(var_field!((*exp).PR, DAE::Exp::TUPLE).clone())?, Types::propTupleFirstProp(prop)?)
        },
        (_, DAE::Properties::PROP_TUPLE { type_: Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::List::Cons { head: ty, tail: _ }, .. }, .. }) => {
            (Arc::new(DAE::Exp::TSUB { exp: exp, ix: 1, ty: ty.clone() }), Types::propTupleFirstProp(prop)?)
        },
        _ => {
            (exp, prop)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outProp))
}

pub fn isSimpleLiteralValue(mut exp: Arc<DAE::Exp>, mut allow_arrays: bool) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::SCONST { .. } => allow_arrays,
        Deref @ DAE::Exp::ICONST { .. } => true,
        Deref @ DAE::Exp::RCONST { .. } => true,
        Deref @ DAE::Exp::BCONST { .. } => true,
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => true,
        Deref @ DAE::Exp::ARRAY { .. } if (allow_arrays) => List::all(var_field!((*exp).array, DAE::Exp::ARRAY).clone(), (std::sync::Arc::new({ let __pe_b1 = true; move |__pe_a0| isSimpleLiteralValue(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>))?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn consToListIgnoreSharedLiteral(mut e: Arc<DAE::Exp>) -> Arc<DAE::Exp> {
    let mut e: Arc<DAE::Exp> = e;
    if (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => true,
        Deref @ DAE::Exp::LIST { .. } => true,
        Deref @ DAE::Exp::CONS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }) {
        if '__try0: {
            e = unwrap_break_err!(consToListIgnoreSharedLiteralWork(e.clone(), metamodelica::nil()), '__try0);
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    e
}

fn consToListIgnoreSharedLiteralWork(mut e: Arc<DAE::Exp>, mut acc: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Arc<DAE::Exp>> {
    let mut e: Arc<DAE::Exp> = e;
    e = (::match_deref::match_deref! { match &((e.clone(), acc.clone())) {
        (Deref @ DAE::Exp::SHARED_LITERAL { .. }, _) => consToListIgnoreSharedLiteralWork(var_field!((*e).exp, DAE::Exp::SHARED_LITERAL).clone(), acc)?,
        (Deref @ DAE::Exp::LIST { .. }, Deref @ metamodelica::List::Nil) => e,
        (Deref @ DAE::Exp::LIST { .. }, _) => Arc::new(DAE::Exp::LIST { valList: List::append_reverse(acc, var_field!((*e).valList, DAE::Exp::LIST).clone()) }),
        (Deref @ DAE::Exp::CONS { .. }, _) => consToListIgnoreSharedLiteralWork(var_field!((*e).cdr, DAE::Exp::CONS).clone(), metamodelica::cons(var_field!((*e).car, DAE::Exp::CONS).clone(), acc))?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(e)
}

pub fn arrayFirstScalar(mut exp: Arc<DAE::Exp>) -> Result<Arc<DAE::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::ARRAY { .. } => { exp = listHead(var_field!((*exp).array, DAE::Exp::ARRAY).clone())?; continue '__tco; },
        _ => return Ok(exp),
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
    }
}

pub fn traverseCases<A: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, A) -> Result<(Arc<DAE::Exp>, A)> + 'static>, mut inA: A) -> Result<(Arc<metamodelica::List<Arc<DAE::MatchCase>>>, A)> {
    pub type FuncExpType<A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, A) -> Result<(Arc<DAE::Exp>, A)> + 'static>;

    let mut outCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>;
    let mut oa: A;
    (outCases, oa) = (::match_deref::match_deref! { match &((inCases.clone(), inA)) {
        (Deref @ metamodelica::List::Nil, a) => {
            (metamodelica::nil(), a.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ DAE::MatchCase { patterns, patternGuard, localDecls: decls, body, result, resultInfo, jump, info }, tail: cases }, a) => {
            let mut body1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
            let mut result1: Option<Arc<DAE::Exp>>;
            let mut patternGuard1: Option<Arc<DAE::Exp>>;
            let mut cases1: Arc<metamodelica::List<Arc<DAE::MatchCase>>>;
            let mut cases = (*cases).clone();
            let mut a = (*a).clone();
            let (__pa0, (_, __pa1)) = DAEUtil::traverseDAEEquationsStmts(body.clone(), (std::sync::Arc::new(traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), (func.clone(), a.clone()))?;
            body1 = __pa0.clone();
            a = __pa1.clone();
            (patternGuard1, a) = traverseExpOpt(patternGuard.clone(), func.clone(), a.clone())?;
            (result1, a) = traverseExpOpt(result.clone(), func.clone(), a.clone())?;
            (cases1, a) = traverseCases(cases.clone(), func.clone(), a.clone())?;
            cases = if (metamodelica::ReferenceEq::reference_eq(&*(cases.clone()), &*(cases1.clone())) && (match (&(patternGuard.clone()), &(patternGuard1.clone())) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && (match (&(result.clone()), &(result1.clone())) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && metamodelica::ReferenceEq::reference_eq(&*(body.clone()), &*(body1.clone()))) {inCases} else {metamodelica::cons(Arc::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: patternGuard1.clone(), localDecls: decls.clone(), body: body1.clone(), result: result1.clone(), resultInfo: resultInfo.clone(), jump: jump.clone(), info: info.clone() }), cases1.clone())};
            (cases.clone(), a.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outCases, oa))
}

pub(crate) fn traverseCasesTopDown<A: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(mut inCases: Arc<metamodelica::List<Arc<DAE::MatchCase>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, A) -> Result<(Arc<DAE::Exp>, bool, A)> + 'static>, mut inA: A) -> Result<(Arc<metamodelica::List<Arc<DAE::MatchCase>>>, A)> {
    pub type FuncExpType<A: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, A) -> Result<(Arc<DAE::Exp>, bool, A)> + 'static>;

    let mut cases: Arc<metamodelica::List<Arc<DAE::MatchCase>>> = metamodelica::nil();
    let mut a: A = inA.clone();
    let mut patterns: Arc<metamodelica::List<Arc<DAE::Pattern>>>;
    let mut decls: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut body: Arc<metamodelica::List<Arc<DAE::Statement>>>;
    let mut body1: Arc<metamodelica::List<Arc<DAE::Statement>>>;
    let mut result: Option<Arc<DAE::Exp>>;
    let mut result1: Option<Arc<DAE::Exp>>;
    let mut patternGuard: Option<Arc<DAE::Exp>>;
    let mut patternGuard1: Option<Arc<DAE::Exp>>;
    let mut jump: i32;
    let mut resultInfo: SourceInfo;
    let mut info: SourceInfo;
    let mut tpl: (Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, A) -> Result<(Arc<DAE::Exp>, bool, A)> + 'static>, A);
    for mut c in &*inCases {
        let mut c = c.clone();
        let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(c.clone()) {
            Deref @ DAE::MatchCase { patterns: __pa0, patternGuard: __pa1, localDecls: __pa2, body: __pa3, result: __pa4, resultInfo: __pa5, jump: __pa6, info: __pa7 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
            _ => bail!("pattern mismatch"),
        } };
        patterns = __pa0.clone();
        patternGuard = __pa1.clone();
        decls = __pa2.clone();
        body = __pa3.clone();
        result = __pa4.clone();
        resultInfo = __pa5.clone();
        jump = __pa6.clone();
        info = __pa7.clone();
        tpl = (func.clone(), a.clone());
        let (__pa8, (_, __pa9)) = DAEUtil::traverseDAEEquationsStmts(body.clone(), (std::sync::Arc::new(traverseSubexpressionsTopDownHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), tpl.clone())?;
        body1 = __pa8.clone();
        a = __pa9.clone();
        (patternGuard1, a) = traverseExpOptTopDown(patternGuard.clone(), func.clone(), a.clone())?;
        (result1, a) = traverseExpOptTopDown(result.clone(), func.clone(), a.clone())?;
        cases = metamodelica::cons(Arc::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: patternGuard1.clone(), localDecls: decls.clone(), body: body1.clone(), result: result1.clone(), resultInfo: resultInfo.clone(), jump: jump, info: info.clone() }), cases.clone());
    }
    cases = cases.reverse();
    Ok((cases, a))
}

