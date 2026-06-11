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
use crate::NFCall as Call;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModelicaUtil as FlatModelicaUtil;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFType as Type;
use openmodelica_error::ErrorTypes;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::IOStream;
use openmodelica_util::Util;

#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFEquation {
    EQUALITY {
        /// The left hand side expression.
        lhs: Arc<Expression::NFExpression>,
        /// The right hand side expression.
        rhs: Arc<Expression::NFExpression>,
        ty: Arc<Type::NFType>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
        scalarizeMode: ScalarizeMode,
    },
    CONNECT {
        lhs: Arc<Expression::NFExpression>,
        rhs: Arc<Expression::NFExpression>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
    FOR {
        iterator: Arc<InstNode::InstNode>,
        range: Option<Arc<Expression::NFExpression>>,
        /// The body of the for loop.
        body: Arc<metamodelica::List<Arc<NFEquation>>>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
    IF {
        branches: Arc<metamodelica::List<Arc<Branch::Branch>>>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
    WHEN {
        branches: Arc<metamodelica::List<Arc<Branch::Branch>>>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
    ASSERT {
        /// The assert condition.
        condition: Arc<Expression::NFExpression>,
        /// The message to display if the assert fails.
        message: Arc<Expression::NFExpression>,
        /// Error or warning
        level: Arc<Expression::NFExpression>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
    TERMINATE {
        /// The message to display if the terminate triggers.
        message: Arc<Expression::NFExpression>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
    REINIT {
        /// The variable to reinitialize.
        cref: Arc<Expression::NFExpression>,
        /// The new value of the variable.
        reinitExp: Arc<Expression::NFExpression>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
    NORETCALL {
        exp: Arc<Expression::NFExpression>,
        scope: Arc<InstNode::InstNode>,
        source: Arc<DAE::ElementSource>,
    },
}
impl metamodelica::gc::MMTrace for NFEquation {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFEquation::EQUALITY { lhs, rhs, ty, scope, source, scalarizeMode } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scalarizeMode, __mmv)?;
                Ok(())
            }
            NFEquation::CONNECT { lhs, rhs, scope, source } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFEquation::FOR { iterator, range, body, scope, source } => {
                metamodelica::gc::MMTrace::mm_accept(iterator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(range, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFEquation::IF { branches, scope, source } => {
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFEquation::WHEN { branches, scope, source } => {
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFEquation::ASSERT { condition, message, level, scope, source } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(level, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFEquation::TERMINATE { message, scope, source } => {
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFEquation::REINIT { cref, reinitExp, scope, source } => {
                metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(reinitExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFEquation::NORETCALL { exp, scope, source } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for NFEquation {
    fn default() -> Self {
        Self::IF {
            branches: Default::default(),
            scope: Default::default(),
            source: Default::default(),
        }
    }
}
pub use self::NFEquation::{EQUALITY,CONNECT,FOR,IF,WHEN,ASSERT,TERMINATE,REINIT,NORETCALL};
pub mod Branch {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Branch {
        BRANCH {
            condition: Arc<Expression::NFExpression>,
            conditionVar: Variability,
            body: Arc<metamodelica::List<Arc<NFEquation>>>,
        },
        INVALID_BRANCH {
            branch: Arc<Branch>,
            errors: Arc<metamodelica::List<ErrorTypes::TotalMessage>>,
        },
    }
    impl metamodelica::gc::MMTrace for Branch {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Branch::BRANCH { condition, conditionVar, body } => {
                    metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(conditionVar, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                    Ok(())
                }
                Branch::INVALID_BRANCH { branch, errors } => {
                    metamodelica::gc::MMTrace::mm_accept(branch, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(errors, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for Branch {
        fn default() -> Self {
            Self::BRANCH {
                condition: Default::default(),
                conditionVar: Default::default(),
                body: Default::default(),
            }
        }
    }
    pub use self::Branch::{BRANCH,INVALID_BRANCH};
    pub(crate) fn mapExp(mut branch: Arc<Branch>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>, mut mapBody: bool) -> Result<Arc<Branch>> {
        let mut branch: Arc<Branch> = branch;
        let mut cond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut eql: Arc<metamodelica::List<Arc<NFEquation>>> = metamodelica::nil();
        branch = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ BRANCH { .. } => {
            cond = func(var_field!((*branch).condition, Branch::BRANCH).clone())?;
            if mapBody {
                eql = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFEquation>>> = metamodelica::nil();
        for mut e in (var_field!((*branch).body, Branch::BRANCH).clone()).into_iter().cloned() {
            let __x = super::mapExp(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            } else {
                eql = var_field!((*branch).body, Branch::BRANCH).clone();
            }
            Arc::new(Branch::BRANCH { condition: cond, conditionVar: var_field!((*branch).conditionVar, Branch::BRANCH).clone(), body: eql })
        },
        Deref @ INVALID_BRANCH { .. } => {
            assign_variant_field!(branch => Branch::INVALID_BRANCH; branch = mapExp(var_field!((*branch).branch, Branch::INVALID_BRANCH).clone(), func.clone(), false)?);
            branch
        },
        _ => branch,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(branch)
    }

    pub(crate) fn isEmpty(mut branch: Arc<Branch>) -> Result<bool> {
        '__tco: loop {
            ::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ BRANCH { .. } => return Ok(var_field!((*branch).body, Branch::BRANCH).clone().is_empty()),
        Deref @ INVALID_BRANCH { .. } => { branch = var_field!((*branch).branch, Branch::INVALID_BRANCH).clone(); continue '__tco; },
        _ => return Err(anyhow::anyhow!("match: no arm matched")),
    } }
        }
    }

    pub(crate) fn sizeOf(mut branch: Arc<Branch>) -> i32 {
        let mut size: i32;
        size = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ BRANCH { .. } => sizeOfList(var_field!((*branch).body, Branch::BRANCH).clone()),
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        size
    }

    pub(crate) fn toStream(mut branch: Arc<Branch>, mut header: ArcStr, mut potentialElse: bool, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
        let mut s: IOStream::IOStream = s;
        s = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ BRANCH { .. } => {
            if potentialElse && Expression::isTrue(var_field!((*branch).condition, Branch::BRANCH).clone()) {
                s = IOStream::append(s, (literal!("else\n")).clone())?;
            } else {
                s = IOStream::append(s, (header).clone())?;
                s = IOStream::append(s, (Expression::toString(var_field!((*branch).condition, Branch::BRANCH).clone())?).clone())?;
                s = IOStream::append(s, (literal!(" then\n")).clone())?;
            }
            s = toStreamList(var_field!((*branch).body, Branch::BRANCH).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s)?;
            s
        },
        Deref @ INVALID_BRANCH { .. } => toStream(var_field!((*branch).branch, Branch::INVALID_BRANCH).clone(), (header).clone(), potentialElse, (indent).clone(), s)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(s)
    }

    pub(crate) fn toFlatStream(mut branch: Arc<Branch>, mut header: ArcStr, mut format: BaseModelica::OutputFormat, mut potentialElse: bool, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
        let mut s: IOStream::IOStream = s;
        s = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ BRANCH { .. } => {
            if potentialElse && Expression::isTrue(var_field!((*branch).condition, Branch::BRANCH).clone()) {
                s = IOStream::append(s, (literal!("else\n")).clone())?;
            } else {
                s = IOStream::append(s, (header).clone())?;
                s = IOStream::append(s, (Expression::toFlatString(var_field!((*branch).condition, Branch::BRANCH).clone(), format.clone())?).clone())?;
                s = IOStream::append(s, (literal!(" then\n")).clone())?;
            }
            s = toFlatStreamList(var_field!((*branch).body, Branch::BRANCH).clone(), format, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s)?;
            s
        },
        Deref @ INVALID_BRANCH { .. } => toFlatStream(var_field!((*branch).branch, Branch::INVALID_BRANCH).clone(), (header).clone(), format, potentialElse, (indent).clone(), s)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(s)
    }

    pub fn toString(mut branch: Arc<Branch>, mut indent: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        let mut s: IOStream::IOStream;
        s = IOStream::create(literal!("NFEquation.Branch.toString"), openmodelica_util::IOStream::IOStreamType::LIST)?;
        s = toStream(branch, (literal!("")).clone(), false, (indent).clone(), s)?;
        r#str = (IOStream::string(s.clone())?).clone();
        IOStream::delete(s)?;
        Ok(r#str)
    }

    pub(crate) fn triggerErrors(mut branch: Arc<Branch>) -> Result<()> {
        let () = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ INVALID_BRANCH { .. } => {
            Error::addTotalMessages(var_field!((*branch).errors, Branch::INVALID_BRANCH).clone())?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(())
    }

}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum ScalarizeMode {
    DONT_SCALARIZE = 1,
    SCALARIZE = 2,
    NO_PREFERENCE = 3,
}
impl PartialOrd for ScalarizeMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for ScalarizeMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl metamodelica::gc::MMTrace for ScalarizeMode {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> { Ok(()) }
}

pub(crate) fn makeEquality(mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>, mut src: Arc<DAE::ElementSource>, mut scope: Arc<InstNode::InstNode>, mut scalarizeMode: ScalarizeMode) -> Arc<NFEquation> {
    let mut eq: Arc<NFEquation>;
    eq = Arc::new(NFEquation::EQUALITY { lhs: lhs, rhs: rhs, ty: ty, scope: scope, source: src, scalarizeMode: scalarizeMode });
    eq
}

pub(crate) fn makeCrefEquality(mut lhsCref: Arc<ComponentRef::NFComponentRef>, mut rhsCref: Arc<ComponentRef::NFComponentRef>, mut scope: Arc<InstNode::InstNode>, mut src: Arc<DAE::ElementSource>) -> Result<Arc<NFEquation>> {
    let mut eq: Arc<NFEquation>;
    let mut e1: Arc<Expression::NFExpression>;
    let mut e2: Arc<Expression::NFExpression>;
    e1 = Expression::fromCref(lhsCref, false)?;
    e2 = Expression::fromCref(rhsCref, false)?;
    eq = makeEquality(e1.clone(), e2, Expression::typeOf(e1), src, scope, ScalarizeMode::NO_PREFERENCE.clone());
    Ok(eq)
}

pub(crate) fn makeBranch(mut condition: Arc<Expression::NFExpression>, mut body: Arc<metamodelica::List<Arc<NFEquation>>>, mut condVar: Variability) -> Arc<Branch::Branch> {
    let mut branch: Arc<Branch::Branch>;
    branch = Arc::new(Branch::Branch::BRANCH { condition: condition, conditionVar: condVar, body: body });
    branch
}

pub(crate) fn makeIf(mut branches: Arc<metamodelica::List<Arc<Branch::Branch>>>, mut scope: Arc<InstNode::InstNode>, mut src: Arc<DAE::ElementSource>) -> Arc<NFEquation> {
    let mut eq: Arc<NFEquation>;
    eq = Arc::new(NFEquation::IF { branches: branches, scope: scope, source: src });
    eq
}

pub fn source(mut eq: Arc<NFEquation>) -> Result<Arc<DAE::ElementSource>> {
    let mut source: Arc<DAE::ElementSource>;
    source = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ EQUALITY { .. } => var_field!((*eq).source, NFEquation::EQUALITY).clone(),
        Deref @ CONNECT { .. } => var_field!((*eq).source, NFEquation::CONNECT).clone(),
        Deref @ FOR { .. } => var_field!((*eq).source, NFEquation::FOR).clone(),
        Deref @ IF { .. } => var_field!((*eq).source, NFEquation::IF).clone(),
        Deref @ WHEN { .. } => var_field!((*eq).source, NFEquation::WHEN).clone(),
        Deref @ ASSERT { .. } => var_field!((*eq).source, NFEquation::ASSERT).clone(),
        Deref @ TERMINATE { .. } => var_field!((*eq).source, NFEquation::TERMINATE).clone(),
        Deref @ REINIT { .. } => var_field!((*eq).source, NFEquation::REINIT).clone(),
        Deref @ NORETCALL { .. } => var_field!((*eq).source, NFEquation::NORETCALL).clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(source)
}

pub(crate) fn setSource(mut source: Arc<DAE::ElementSource>, mut eq: Arc<NFEquation>) -> Result<Arc<NFEquation>> {
    let mut eq: Arc<NFEquation> = eq;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ EQUALITY { .. } => {
            assign_variant_field!(eq => NFEquation::EQUALITY; source = source);
            ()
        },
        Deref @ CONNECT { .. } => {
            assign_variant_field!(eq => NFEquation::CONNECT; source = source);
            ()
        },
        Deref @ FOR { .. } => {
            assign_variant_field!(eq => NFEquation::FOR; source = source);
            ()
        },
        Deref @ IF { .. } => {
            assign_variant_field!(eq => NFEquation::IF; source = source);
            ()
        },
        Deref @ WHEN { .. } => {
            assign_variant_field!(eq => NFEquation::WHEN; source = source);
            ()
        },
        Deref @ ASSERT { .. } => {
            assign_variant_field!(eq => NFEquation::ASSERT; source = source);
            ()
        },
        Deref @ TERMINATE { .. } => {
            assign_variant_field!(eq => NFEquation::TERMINATE; source = source);
            ()
        },
        Deref @ REINIT { .. } => {
            assign_variant_field!(eq => NFEquation::REINIT; source = source);
            ()
        },
        Deref @ NORETCALL { .. } => {
            assign_variant_field!(eq => NFEquation::NORETCALL; source = source);
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eq)
}

pub fn scope(mut eq: Arc<NFEquation>) -> Result<Arc<InstNode::InstNode>> {
    let mut scope: Arc<InstNode::InstNode>;
    scope = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ EQUALITY { .. } => var_field!((*eq).scope, NFEquation::EQUALITY).clone(),
        Deref @ CONNECT { .. } => var_field!((*eq).scope, NFEquation::CONNECT).clone(),
        Deref @ FOR { .. } => var_field!((*eq).scope, NFEquation::FOR).clone(),
        Deref @ IF { .. } => var_field!((*eq).scope, NFEquation::IF).clone(),
        Deref @ WHEN { .. } => var_field!((*eq).scope, NFEquation::WHEN).clone(),
        Deref @ ASSERT { .. } => var_field!((*eq).scope, NFEquation::ASSERT).clone(),
        Deref @ TERMINATE { .. } => var_field!((*eq).scope, NFEquation::TERMINATE).clone(),
        Deref @ REINIT { .. } => var_field!((*eq).scope, NFEquation::REINIT).clone(),
        Deref @ NORETCALL { .. } => var_field!((*eq).scope, NFEquation::NORETCALL).clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(scope)
}

pub(crate) fn info(mut eq: Arc<NFEquation>) -> Result<SourceInfo> {
    let mut info: SourceInfo = ElementSource::getInfo(source(eq.clone())?);
    Ok(info)
}

pub type ApplyFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFEquation>) -> Result<()> + 'static>;

pub(crate) fn applyList(mut eql: Arc<metamodelica::List<Arc<NFEquation>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFEquation>) -> Result<()> + 'static>) -> Result<()> {
    for mut eq in &*eql {
        let mut eq = eq.clone();
        apply(eq.clone(), func.clone())?;
    }
    Ok(())
}

pub(crate) fn apply(mut eq: Arc<NFEquation>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFEquation>) -> Result<()> + 'static>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ FOR { .. } => {
            for mut e in &*var_field!((*eq).body, NFEquation::FOR).clone() {
                let mut e = e.clone();
                apply(e.clone(), func.clone())?;
            }
            ()
        },
        Deref @ IF { .. } => {
            for mut b in &*var_field!((*eq).branches, NFEquation::IF).clone() {
                let mut b = b.clone();
                let () = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Branch::BRANCH { .. } => {
            for mut e in &*var_field!((*b).body, Branch::Branch::BRANCH).clone() {
                let mut e = e.clone();
                apply(e.clone(), func.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        Deref @ WHEN { .. } => {
            for mut b in &*var_field!((*eq).branches, NFEquation::WHEN).clone() {
                let mut b = b.clone();
                let () = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Branch::BRANCH { .. } => {
            for mut e in &*var_field!((*b).body, Branch::Branch::BRANCH).clone() {
                let mut e = e.clone();
                apply(e.clone(), func.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    func(eq)?;
    Ok(())
}

pub type MapFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFEquation>) -> Result<Arc<NFEquation>> + 'static>;

pub(crate) fn map(mut eq: Arc<NFEquation>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFEquation>) -> Result<Arc<NFEquation>> + 'static>) -> Result<Arc<NFEquation>> {
    let mut eq: Arc<NFEquation> = eq;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ FOR { .. } => {
            assign_variant_field!(eq => NFEquation::FOR; body = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFEquation>>> = metamodelica::nil();
        for mut e in (var_field!((*eq).body, NFEquation::FOR).clone()).into_iter().cloned() {
            let __x = map(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            ()
        },
        Deref @ IF { .. } => {
            assign_variant_field!(eq => NFEquation::IF; branches = ({
        let mut __acc: Arc<metamodelica::List<Arc<Branch::Branch>>> = metamodelica::nil();
        for mut b in (var_field!((*eq).branches, NFEquation::IF).clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Branch::BRANCH { .. } => {
            assign_variant_field!(b => Branch::Branch::BRANCH; body = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFEquation>>> = metamodelica::nil();
        for mut e in (var_field!((*b).body, Branch::Branch::BRANCH).clone()).into_iter().cloned() {
            let __x = map(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            b.clone()
        },
        _ => b.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            ()
        },
        Deref @ WHEN { .. } => {
            assign_variant_field!(eq => NFEquation::WHEN; branches = ({
        let mut __acc: Arc<metamodelica::List<Arc<Branch::Branch>>> = metamodelica::nil();
        for mut b in (var_field!((*eq).branches, NFEquation::WHEN).clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Branch::BRANCH { .. } => {
            assign_variant_field!(b => Branch::Branch::BRANCH; body = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFEquation>>> = metamodelica::nil();
        for mut e in (var_field!((*b).body, Branch::Branch::BRANCH).clone()).into_iter().cloned() {
            let __x = map(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            b.clone()
        },
        _ => b.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    eq = func(eq)?;
    Ok(eq)
}

pub(crate) fn applyExpList(mut eq: Arc<metamodelica::List<Arc<NFEquation>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>;

    for mut e in &*eq {
        let mut e = e.clone();
        applyExp(e.clone(), func.clone())?;
    }
    Ok(())
}

pub(crate) fn applyExp(mut eq: Arc<NFEquation>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ EQUALITY { .. } => {
            func(var_field!((*eq).lhs, NFEquation::EQUALITY).clone())?;
            func(var_field!((*eq).rhs, NFEquation::EQUALITY).clone())?;
            ()
        },
        Deref @ CONNECT { .. } => {
            func(var_field!((*eq).lhs, NFEquation::CONNECT).clone())?;
            func(var_field!((*eq).rhs, NFEquation::CONNECT).clone())?;
            ()
        },
        Deref @ FOR { .. } => {
            applyExpList(var_field!((*eq).body, NFEquation::FOR).clone(), func.clone())?;
            if isSome(var_field!((*eq).range, NFEquation::FOR).clone()) {
                func(Util::getOption(var_field!((*eq).range, NFEquation::FOR).clone())?)?;
            }
            ()
        },
        Deref @ IF { .. } => {
            for mut b in &*var_field!((*eq).branches, NFEquation::IF).clone() {
                let mut b = b.clone();
                let () = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Branch::BRANCH { .. } => {
            func(var_field!((*b).condition, Branch::Branch::BRANCH).clone())?;
            applyExpList(var_field!((*b).body, Branch::Branch::BRANCH).clone(), func.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        Deref @ WHEN { .. } => {
            for mut b in &*var_field!((*eq).branches, NFEquation::WHEN).clone() {
                let mut b = b.clone();
                let () = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Branch::BRANCH { .. } => {
            func(var_field!((*b).condition, Branch::Branch::BRANCH).clone())?;
            applyExpList(var_field!((*b).body, Branch::Branch::BRANCH).clone(), func.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        Deref @ ASSERT { .. } => {
            func(var_field!((*eq).condition, NFEquation::ASSERT).clone())?;
            func(var_field!((*eq).message, NFEquation::ASSERT).clone())?;
            func(var_field!((*eq).level, NFEquation::ASSERT).clone())?;
            ()
        },
        Deref @ TERMINATE { .. } => {
            func(var_field!((*eq).message, NFEquation::TERMINATE).clone())?;
            ()
        },
        Deref @ REINIT { .. } => {
            func(var_field!((*eq).cref, NFEquation::REINIT).clone())?;
            func(var_field!((*eq).reinitExp, NFEquation::REINIT).clone())?;
            ()
        },
        Deref @ NORETCALL { .. } => {
            func(var_field!((*eq).exp, NFEquation::NORETCALL).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn applyExpShallow(mut eq: Arc<NFEquation>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ EQUALITY { .. } => {
            func(var_field!((*eq).lhs, NFEquation::EQUALITY).clone())?;
            func(var_field!((*eq).rhs, NFEquation::EQUALITY).clone())?;
            ()
        },
        Deref @ CONNECT { .. } => {
            func(var_field!((*eq).lhs, NFEquation::CONNECT).clone())?;
            func(var_field!((*eq).rhs, NFEquation::CONNECT).clone())?;
            ()
        },
        Deref @ FOR { .. } => {
            if isSome(var_field!((*eq).range, NFEquation::FOR).clone()) {
                func(Util::getOption(var_field!((*eq).range, NFEquation::FOR).clone())?)?;
            }
            ()
        },
        Deref @ IF { .. } => {
            for mut b in &*var_field!((*eq).branches, NFEquation::IF).clone() {
                let mut b = b.clone();
                let () = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Branch::BRANCH { .. } => {
            func(var_field!((*b).condition, Branch::Branch::BRANCH).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        Deref @ WHEN { .. } => {
            for mut b in &*var_field!((*eq).branches, NFEquation::WHEN).clone() {
                let mut b = b.clone();
                let () = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Branch::BRANCH { .. } => {
            func(var_field!((*b).condition, Branch::Branch::BRANCH).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        Deref @ ASSERT { .. } => {
            func(var_field!((*eq).condition, NFEquation::ASSERT).clone())?;
            func(var_field!((*eq).message, NFEquation::ASSERT).clone())?;
            func(var_field!((*eq).level, NFEquation::ASSERT).clone())?;
            ()
        },
        Deref @ TERMINATE { .. } => {
            func(var_field!((*eq).message, NFEquation::TERMINATE).clone())?;
            ()
        },
        Deref @ REINIT { .. } => {
            func(var_field!((*eq).cref, NFEquation::REINIT).clone())?;
            func(var_field!((*eq).reinitExp, NFEquation::REINIT).clone())?;
            ()
        },
        Deref @ NORETCALL { .. } => {
            func(var_field!((*eq).exp, NFEquation::NORETCALL).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub type MapExpFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>;

pub(crate) fn mapExpList(mut eql: Arc<metamodelica::List<Arc<NFEquation>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<metamodelica::List<Arc<NFEquation>>>> {
    let mut eql: Arc<metamodelica::List<Arc<NFEquation>>> = eql;
    eql = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFEquation>>> = metamodelica::nil();
        for mut eq in (eql).into_iter().cloned() {
            let __x = mapExp(eq.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(eql)
}

pub(crate) fn mapExp(mut eq: Arc<NFEquation>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<NFEquation>> {
    let mut eq: Arc<NFEquation> = eq;
    eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ EQUALITY { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            e1 = func(var_field!((*eq).lhs, NFEquation::EQUALITY).clone())?;
            e2 = func(var_field!((*eq).rhs, NFEquation::EQUALITY).clone())?;
            if (referenceEq(&*(e1.clone()),&*(var_field!((*eq).lhs, NFEquation::EQUALITY).clone())) && referenceEq(&*(e2.clone()),&*(var_field!((*eq).rhs, NFEquation::EQUALITY).clone()))) {eq} else {Arc::new(NFEquation::EQUALITY { lhs: e1.clone(), rhs: e2.clone(), ty: var_field!((*eq).ty, NFEquation::EQUALITY).clone(), scope: var_field!((*eq).scope, NFEquation::EQUALITY).clone(), source: var_field!((*eq).source, NFEquation::EQUALITY).clone(), scalarizeMode: var_field!((*eq).scalarizeMode, NFEquation::EQUALITY).clone() })}
        },
        Deref @ CONNECT { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            e1 = func(var_field!((*eq).lhs, NFEquation::CONNECT).clone())?;
            e2 = func(var_field!((*eq).rhs, NFEquation::CONNECT).clone())?;
            if (referenceEq(&*(e1.clone()),&*(var_field!((*eq).lhs, NFEquation::CONNECT).clone())) && referenceEq(&*(e2.clone()),&*(var_field!((*eq).rhs, NFEquation::CONNECT).clone()))) {eq} else {Arc::new(NFEquation::CONNECT { lhs: e1.clone(), rhs: e2.clone(), scope: var_field!((*eq).scope, NFEquation::CONNECT).clone(), source: var_field!((*eq).source, NFEquation::CONNECT).clone() })}
        },
        Deref @ FOR { .. } => {
            assign_variant_field!(eq => NFEquation::FOR;
                body = ({
        let mut __acc: Arc<metamodelica::List<Arc<NFEquation>>> = metamodelica::nil();
        for mut e in (var_field!((*eq).body, NFEquation::FOR).clone()).into_iter().cloned() {
            let __x = mapExp(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                range = Util::applyOption(var_field!((*eq).range, NFEquation::FOR).clone(), func.clone())?
            );
            eq
        },
        Deref @ IF { .. } => {
            assign_variant_field!(eq => NFEquation::IF; branches = ({
        let mut __acc: Arc<metamodelica::List<Arc<Branch::Branch>>> = metamodelica::nil();
        for mut b in (var_field!((*eq).branches, NFEquation::IF).clone()).into_iter().cloned() {
            let __x = Branch::mapExp(b.clone(), func.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            eq
        },
        Deref @ WHEN { .. } => {
            assign_variant_field!(eq => NFEquation::WHEN; branches = ({
        let mut __acc: Arc<metamodelica::List<Arc<Branch::Branch>>> = metamodelica::nil();
        for mut b in (var_field!((*eq).branches, NFEquation::WHEN).clone()).into_iter().cloned() {
            let __x = Branch::mapExp(b.clone(), func.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            eq
        },
        Deref @ ASSERT { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            let mut e3: Arc<Expression::NFExpression>;
            e1 = func(var_field!((*eq).condition, NFEquation::ASSERT).clone())?;
            e2 = func(var_field!((*eq).message, NFEquation::ASSERT).clone())?;
            e3 = func(var_field!((*eq).level, NFEquation::ASSERT).clone())?;
            if (referenceEq(&*(e1.clone()),&*(var_field!((*eq).condition, NFEquation::ASSERT).clone())) && referenceEq(&*(e2.clone()),&*(var_field!((*eq).message, NFEquation::ASSERT).clone())) && referenceEq(&*(e3.clone()),&*(var_field!((*eq).level, NFEquation::ASSERT).clone()))) {eq} else {Arc::new(NFEquation::ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), scope: var_field!((*eq).scope, NFEquation::ASSERT).clone(), source: var_field!((*eq).source, NFEquation::ASSERT).clone() })}
        },
        Deref @ TERMINATE { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            e1 = func(var_field!((*eq).message, NFEquation::TERMINATE).clone())?;
            if (referenceEq(&*(e1.clone()),&*(var_field!((*eq).message, NFEquation::TERMINATE).clone()))) {eq} else {Arc::new(NFEquation::TERMINATE { message: e1.clone(), scope: var_field!((*eq).scope, NFEquation::TERMINATE).clone(), source: var_field!((*eq).source, NFEquation::TERMINATE).clone() })}
        },
        Deref @ REINIT { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            e1 = func(var_field!((*eq).cref, NFEquation::REINIT).clone())?;
            e2 = func(var_field!((*eq).reinitExp, NFEquation::REINIT).clone())?;
            if (referenceEq(&*(e1.clone()),&*(var_field!((*eq).cref, NFEquation::REINIT).clone())) && referenceEq(&*(e2.clone()),&*(var_field!((*eq).reinitExp, NFEquation::REINIT).clone()))) {eq} else {Arc::new(NFEquation::REINIT { cref: e1.clone(), reinitExp: e2.clone(), scope: var_field!((*eq).scope, NFEquation::REINIT).clone(), source: var_field!((*eq).source, NFEquation::REINIT).clone() })}
        },
        Deref @ NORETCALL { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            e1 = func(var_field!((*eq).exp, NFEquation::NORETCALL).clone())?;
            if (referenceEq(&*(e1.clone()),&*(var_field!((*eq).exp, NFEquation::NORETCALL).clone()))) {eq} else {Arc::new(NFEquation::NORETCALL { exp: e1.clone(), scope: var_field!((*eq).scope, NFEquation::NORETCALL).clone(), source: var_field!((*eq).source, NFEquation::NORETCALL).clone() })}
        },
        _ => {
            eq
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eq)
}

pub(crate) fn mapExpShallow(mut eq: Arc<NFEquation>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<NFEquation>> {
    let mut eq: Arc<NFEquation> = eq;
    eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ EQUALITY { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            e1 = func(var_field!((*eq).lhs, NFEquation::EQUALITY).clone())?;
            e2 = func(var_field!((*eq).rhs, NFEquation::EQUALITY).clone())?;
            if (referenceEq(&*(e1.clone()),&*(var_field!((*eq).lhs, NFEquation::EQUALITY).clone())) && referenceEq(&*(e2.clone()),&*(var_field!((*eq).rhs, NFEquation::EQUALITY).clone()))) {eq} else {Arc::new(NFEquation::EQUALITY { lhs: e1.clone(), rhs: e2.clone(), ty: var_field!((*eq).ty, NFEquation::EQUALITY).clone(), scope: var_field!((*eq).scope, NFEquation::EQUALITY).clone(), source: var_field!((*eq).source, NFEquation::EQUALITY).clone(), scalarizeMode: var_field!((*eq).scalarizeMode, NFEquation::EQUALITY).clone() })}
        },
        Deref @ CONNECT { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            e1 = func(var_field!((*eq).lhs, NFEquation::CONNECT).clone())?;
            e2 = func(var_field!((*eq).rhs, NFEquation::CONNECT).clone())?;
            if (referenceEq(&*(e1.clone()),&*(var_field!((*eq).lhs, NFEquation::CONNECT).clone())) && referenceEq(&*(e2.clone()),&*(var_field!((*eq).rhs, NFEquation::CONNECT).clone()))) {eq} else {Arc::new(NFEquation::CONNECT { lhs: e1.clone(), rhs: e2.clone(), scope: var_field!((*eq).scope, NFEquation::CONNECT).clone(), source: var_field!((*eq).source, NFEquation::CONNECT).clone() })}
        },
        Deref @ FOR { .. } => {
            assign_variant_field!(eq => NFEquation::FOR; range = Util::applyOption(var_field!((*eq).range, NFEquation::FOR).clone(), func.clone())?);
            eq
        },
        Deref @ IF { .. } => {
            assign_variant_field!(eq => NFEquation::IF; branches = ({
        let mut __acc: Arc<metamodelica::List<Arc<Branch::Branch>>> = metamodelica::nil();
        for mut b in (var_field!((*eq).branches, NFEquation::IF).clone()).into_iter().cloned() {
            let __x = Branch::mapExp(b.clone(), func.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            eq
        },
        Deref @ WHEN { .. } => {
            assign_variant_field!(eq => NFEquation::WHEN; branches = ({
        let mut __acc: Arc<metamodelica::List<Arc<Branch::Branch>>> = metamodelica::nil();
        for mut b in (var_field!((*eq).branches, NFEquation::WHEN).clone()).into_iter().cloned() {
            let __x = Branch::mapExp(b.clone(), func.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            eq
        },
        Deref @ ASSERT { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            let mut e3: Arc<Expression::NFExpression>;
            e1 = func(var_field!((*eq).condition, NFEquation::ASSERT).clone())?;
            e2 = func(var_field!((*eq).message, NFEquation::ASSERT).clone())?;
            e3 = func(var_field!((*eq).level, NFEquation::ASSERT).clone())?;
            if (referenceEq(&*(e1.clone()),&*(var_field!((*eq).condition, NFEquation::ASSERT).clone())) && referenceEq(&*(e2.clone()),&*(var_field!((*eq).message, NFEquation::ASSERT).clone())) && referenceEq(&*(e3.clone()),&*(var_field!((*eq).level, NFEquation::ASSERT).clone()))) {eq} else {Arc::new(NFEquation::ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), scope: var_field!((*eq).scope, NFEquation::ASSERT).clone(), source: var_field!((*eq).source, NFEquation::ASSERT).clone() })}
        },
        Deref @ TERMINATE { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            e1 = func(var_field!((*eq).message, NFEquation::TERMINATE).clone())?;
            if (referenceEq(&*(e1.clone()),&*(var_field!((*eq).message, NFEquation::TERMINATE).clone()))) {eq} else {Arc::new(NFEquation::TERMINATE { message: e1.clone(), scope: var_field!((*eq).scope, NFEquation::TERMINATE).clone(), source: var_field!((*eq).source, NFEquation::TERMINATE).clone() })}
        },
        Deref @ REINIT { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            e1 = func(var_field!((*eq).cref, NFEquation::REINIT).clone())?;
            e2 = func(var_field!((*eq).reinitExp, NFEquation::REINIT).clone())?;
            if (referenceEq(&*(e1.clone()),&*(var_field!((*eq).cref, NFEquation::REINIT).clone())) && referenceEq(&*(e2.clone()),&*(var_field!((*eq).reinitExp, NFEquation::REINIT).clone()))) {eq} else {Arc::new(NFEquation::REINIT { cref: e1.clone(), reinitExp: e2.clone(), scope: var_field!((*eq).scope, NFEquation::REINIT).clone(), source: var_field!((*eq).source, NFEquation::REINIT).clone() })}
        },
        Deref @ NORETCALL { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            e1 = func(var_field!((*eq).exp, NFEquation::NORETCALL).clone())?;
            if (referenceEq(&*(e1.clone()),&*(var_field!((*eq).exp, NFEquation::NORETCALL).clone()))) {eq} else {Arc::new(NFEquation::NORETCALL { exp: e1.clone(), scope: var_field!((*eq).scope, NFEquation::NORETCALL).clone(), source: var_field!((*eq).source, NFEquation::NORETCALL).clone() })}
        },
        _ => {
            eq
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eq)
}

pub(crate) fn foldExpList<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut eq: Arc<metamodelica::List<Arc<NFEquation>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    for mut e in &*eq {
        let mut e = e.clone();
        arg = foldExp(e.clone(), func.clone(), arg.clone())?;
    }
    Ok(arg)
}

pub(crate) fn foldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(mut eq: Arc<NFEquation>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>, mut arg: ArgT) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ EQUALITY { .. } => {
            arg = func(var_field!((*eq).lhs, NFEquation::EQUALITY).clone(), arg)?;
            arg = func(var_field!((*eq).rhs, NFEquation::EQUALITY).clone(), arg)?;
            ()
        },
        Deref @ CONNECT { .. } => {
            arg = func(var_field!((*eq).lhs, NFEquation::CONNECT).clone(), arg)?;
            arg = func(var_field!((*eq).rhs, NFEquation::CONNECT).clone(), arg)?;
            ()
        },
        Deref @ FOR { .. } => {
            arg = foldExpList(var_field!((*eq).body, NFEquation::FOR).clone(), func.clone(), arg)?;
            if isSome(var_field!((*eq).range, NFEquation::FOR).clone()) {
                arg = func(Util::getOption(var_field!((*eq).range, NFEquation::FOR).clone())?, arg)?;
            }
            ()
        },
        Deref @ IF { .. } => {
            for mut b in &*var_field!((*eq).branches, NFEquation::IF).clone() {
                let mut b = b.clone();
                let () = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Branch::BRANCH { .. } => {
            arg = func(var_field!((*b).condition, Branch::Branch::BRANCH).clone(), arg.clone())?;
            arg = foldExpList(var_field!((*b).body, Branch::Branch::BRANCH).clone(), func.clone(), arg.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        Deref @ WHEN { .. } => {
            for mut b in &*var_field!((*eq).branches, NFEquation::WHEN).clone() {
                let mut b = b.clone();
                let () = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Branch::BRANCH { .. } => {
            arg = func(var_field!((*b).condition, Branch::Branch::BRANCH).clone(), arg.clone())?;
            arg = foldExpList(var_field!((*b).body, Branch::Branch::BRANCH).clone(), func.clone(), arg.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            ()
        },
        Deref @ ASSERT { .. } => {
            arg = func(var_field!((*eq).condition, NFEquation::ASSERT).clone(), arg)?;
            arg = func(var_field!((*eq).message, NFEquation::ASSERT).clone(), arg)?;
            arg = func(var_field!((*eq).level, NFEquation::ASSERT).clone(), arg)?;
            ()
        },
        Deref @ TERMINATE { .. } => {
            arg = func(var_field!((*eq).message, NFEquation::TERMINATE).clone(), arg)?;
            ()
        },
        Deref @ REINIT { .. } => {
            arg = func(var_field!((*eq).cref, NFEquation::REINIT).clone(), arg)?;
            arg = func(var_field!((*eq).reinitExp, NFEquation::REINIT).clone(), arg)?;
            ()
        },
        Deref @ NORETCALL { .. } => {
            arg = func(var_field!((*eq).exp, NFEquation::NORETCALL).clone(), arg)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arg)
}

pub(crate) fn contains(mut eq: Arc<NFEquation>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFEquation>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type PredFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFEquation>) -> Result<bool> + 'static>;

    let mut res: bool = false;
    if func(eq.clone())? {
        res = true;
        return Ok(res.clone());
    }
    res = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ FOR { .. } => containsList(var_field!((*eq).body, NFEquation::FOR).clone(), func.clone())?,
        Deref @ IF { .. } => {
            for mut b in &*var_field!((*eq).branches, NFEquation::IF).clone() {
                let mut b = b.clone();
                let () = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Branch::BRANCH { .. } => {
            if containsList(var_field!((*b).body, Branch::Branch::BRANCH).clone(), func.clone())? {
                res = true;
                return Ok(res.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            false
        },
        Deref @ WHEN { .. } => {
            for mut b in &*var_field!((*eq).branches, NFEquation::WHEN).clone() {
                let mut b = b.clone();
                let () = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Branch::BRANCH { .. } => {
            if containsList(var_field!((*b).body, Branch::Branch::BRANCH).clone(), func.clone())? {
                res = true;
                return Ok(res.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            false
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn containsList(mut eql: Arc<metamodelica::List<Arc<NFEquation>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<NFEquation>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type PredFn = std::sync::Arc<dyn ::std::ops::Fn(Arc<NFEquation>) -> Result<bool> + 'static>;

    let mut res: bool;
    for mut eq in &*eql {
        let mut eq = eq.clone();
        if contains(eq.clone(), func.clone())? {
            res = true;
            return Ok(res.clone());
        }
    }
    res = false;
    Ok(res)
}

pub(crate) fn containsExp(mut eq: Arc<NFEquation>, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type Predicate = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool = false;
    res = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ EQUALITY { .. } => r#fn(var_field!((*eq).lhs, NFEquation::EQUALITY).clone())? || r#fn(var_field!((*eq).rhs, NFEquation::EQUALITY).clone())?,
        Deref @ CONNECT { .. } => r#fn(var_field!((*eq).lhs, NFEquation::CONNECT).clone())? || r#fn(var_field!((*eq).rhs, NFEquation::CONNECT).clone())?,
        Deref @ FOR { .. } => {
            res = if (isSome(var_field!((*eq).range, NFEquation::FOR).clone())) {r#fn(Util::getOption(var_field!((*eq).range, NFEquation::FOR).clone())?)?} else {false};
            if !(res) {
                res = containsExpList(var_field!((*eq).body, NFEquation::FOR).clone(), r#fn.clone())?;
            }
            res
        },
        Deref @ IF { .. } => {
            res = false;
            for mut b in &*var_field!((*eq).branches, NFEquation::IF).clone() {
                let mut b = b.clone();
                let () = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Branch::BRANCH { .. } => {
            if r#fn(var_field!((*b).condition, Branch::Branch::BRANCH).clone())? {
                res = true;
                return Ok(res.clone());
            }
            if containsExpList(var_field!((*b).body, Branch::Branch::BRANCH).clone(), r#fn.clone())? {
                res = true;
                return Ok(res.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            res
        },
        Deref @ WHEN { .. } => {
            res = false;
            for mut b in &*var_field!((*eq).branches, NFEquation::WHEN).clone() {
                let mut b = b.clone();
                let () = (::match_deref::match_deref! { match &(b.clone()) {
        Deref @ Branch::BRANCH { .. } => {
            if r#fn(var_field!((*b).condition, Branch::Branch::BRANCH).clone())? {
                res = true;
                return Ok(res.clone());
            }
            if containsExpList(var_field!((*b).body, Branch::Branch::BRANCH).clone(), r#fn.clone())? {
                res = true;
                return Ok(res.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            res
        },
        Deref @ ASSERT { .. } => r#fn(var_field!((*eq).condition, NFEquation::ASSERT).clone())? || r#fn(var_field!((*eq).message, NFEquation::ASSERT).clone())? || r#fn(var_field!((*eq).level, NFEquation::ASSERT).clone())?,
        Deref @ TERMINATE { .. } => r#fn(var_field!((*eq).message, NFEquation::TERMINATE).clone())?,
        Deref @ REINIT { .. } => r#fn(var_field!((*eq).cref, NFEquation::REINIT).clone())? || r#fn(var_field!((*eq).reinitExp, NFEquation::REINIT).clone())?,
        Deref @ NORETCALL { .. } => r#fn(var_field!((*eq).exp, NFEquation::NORETCALL).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn containsExpList(mut eql: Arc<metamodelica::List<Arc<NFEquation>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>) -> Result<bool> {
    pub type Predicate = std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    for mut eq in &*eql {
        let mut eq = eq.clone();
        if containsExp(eq.clone(), func.clone())? {
            res = true;
            return Ok(res.clone());
        }
    }
    res = false;
    Ok(res)
}

pub(crate) fn replaceIteratorList(mut eql: Arc<metamodelica::List<Arc<NFEquation>>>, mut iterator: Arc<InstNode::InstNode>, mut value: Arc<Expression::NFExpression>) -> Result<Arc<metamodelica::List<Arc<NFEquation>>>> {
    let mut eql: Arc<metamodelica::List<Arc<NFEquation>>> = eql;
    eql = mapExpList(eql, (std::sync::Arc::new({ let __pe_b1 = iterator; let __pe_b2 = value; move |__pe_a0| Expression::replaceIterator(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(eql)
}

pub(crate) fn isArrayEquality(mut eq: Arc<NFEquation>) -> bool {
    let mut isArray: bool;
    isArray = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ EQUALITY { .. } => Type::isArray(var_field!((*eq).ty, NFEquation::EQUALITY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isArray
}

pub(crate) fn isConnect(mut eq: Arc<NFEquation>) -> bool {
    let mut isConnect: bool;
    isConnect = (::match_deref::match_deref! { match &(eq) {
        Deref @ CONNECT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isConnect
}

pub(crate) fn isConnection(mut eq: Arc<NFEquation>) -> Result<bool> {
    let mut res: bool;
    let mut call: Arc<Call::NFCall> = Arc::new(<Call::NFCall as ::std::default::Default>::default());
    res = (::match_deref::match_deref! { match &(eq) {
        Deref @ CONNECT { .. } => true,
        Deref @ NORETCALL { exp: Deref @ Expression::CALL { call: __esc_call }, .. } => {
            call = (*__esc_call).clone();
            Call::isConnectionsOperator(call.clone())?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub(crate) fn sizeOfList(mut eqs: Arc<metamodelica::List<Arc<NFEquation>>>) -> i32 {
    let mut size: i32 = 0;
    for mut eq in &*eqs {
        let mut eq = eq.clone();
        size = size + sizeOf(eq.clone());
    }
    size
}

pub fn sizeOf(mut eq: Arc<NFEquation>) -> i32 {
    let mut size: i32 = 0;
    size = 'mc: {
        let __mc_input = eq.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ EQUALITY { .. } => {
                    Ok(Type::sizeOf(var_field!((*eq).ty, NFEquation::EQUALITY).clone(), false)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ CONNECT { .. } => {
                    Ok(Type::sizeOf(Expression::typeOf(var_field!((*eq).lhs, NFEquation::CONNECT).clone()), false)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ FOR { .. } => {
                    let mut size: i32 = size.clone();
                    size = Type::sizeOf(Expression::typeOf(Util::getOption(var_field!((*eq).range, NFEquation::FOR).clone())?), false)?;
                    Ok((size * sizeOfList(var_field!((*eq).body, NFEquation::FOR).clone()), size.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { size = __wb0; break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ IF { .. } => {
                    Ok(Branch::sizeOf(listHead(var_field!((*eq).branches, NFEquation::IF).clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ WHEN { .. } => {
                    Ok(Branch::sizeOf(listHead(var_field!((*eq).branches, NFEquation::WHEN).clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        panic!("matchcontinue: no arm matched")
    };
    size
}

pub fn toString(mut eq: Arc<NFEquation>, mut indent: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: IOStream::IOStream;
    s = IOStream::create(literal!("NFEquation.toString"), openmodelica_util::IOStream::IOStreamType::LIST)?;
    s = toStream(eq, (indent).clone(), s)?;
    r#str = (IOStream::string(s.clone())?).clone();
    IOStream::delete(s)?;
    Ok(r#str)
}

pub(crate) fn toStringList(mut eql: Arc<metamodelica::List<Arc<NFEquation>>>, mut indent: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: IOStream::IOStream;
    s = IOStream::create(literal!("NFEquation.toStringList"), openmodelica_util::IOStream::IOStreamType::LIST)?;
    s = toStreamList(eql, (indent).clone(), s)?;
    r#str = (IOStream::string(s.clone())?).clone();
    IOStream::delete(s)?;
    Ok(r#str)
}

pub(crate) fn toStream(mut eq: Arc<NFEquation>, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut branches: Arc<metamodelica::List<Arc<Branch::Branch>>> = metamodelica::nil();
    let mut branch: Arc<Branch::Branch> = Arc::new(<Branch::Branch as ::std::default::Default>::default());
    s = IOStream::append(s, (indent.clone()).clone())?;
    s = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ EQUALITY { .. } => {
            s = IOStream::append(s, (Expression::toString(var_field!((*eq).lhs, NFEquation::EQUALITY).clone())?).clone())?;
            s = IOStream::append(s, (literal!(" = ")).clone())?;
            s = IOStream::append(s, (Expression::toString(var_field!((*eq).rhs, NFEquation::EQUALITY).clone())?).clone())?;
            s
        },
        Deref @ CONNECT { .. } => {
            s = IOStream::append(s, (literal!("connect(")).clone())?;
            s = IOStream::append(s, (Expression::toString(var_field!((*eq).lhs, NFEquation::CONNECT).clone())?).clone())?;
            s = IOStream::append(s, (literal!(", ")).clone())?;
            s = IOStream::append(s, (Expression::toString(var_field!((*eq).rhs, NFEquation::CONNECT).clone())?).clone())?;
            s = IOStream::append(s, (literal!(")")).clone())?;
            s
        },
        Deref @ FOR { .. } => {
            s = IOStream::append(s, (literal!("for ")).clone())?;
            s = IOStream::append(s, (InstNode::name(var_field!((*eq).iterator, NFEquation::FOR).clone())?).clone())?;
            if isSome(var_field!((*eq).range, NFEquation::FOR).clone()) {
                s = IOStream::append(s, (literal!(" in ")).clone())?;
                s = IOStream::append(s, (Expression::toString(Util::getOption(var_field!((*eq).range, NFEquation::FOR).clone())?)?).clone())?;
            }
            s = IOStream::append(s, (literal!(" loop\n")).clone())?;
            s = toStreamList(var_field!((*eq).body, NFEquation::FOR).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s)?;
            s = IOStream::append(s, (indent).clone())?;
            s = IOStream::append(s, (literal!("end for")).clone())?;
            s
        },
        Deref @ IF { .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(var_field!((*eq).branches, NFEquation::IF).clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            branch = __pa0.clone();
            branches = __pa1.clone();
            s = Branch::toStream(listHead(var_field!((*eq).branches, NFEquation::IF).clone())?, (literal!("if ")).clone(), false, (indent.clone()).clone(), s)?;
            while !(branches.clone().is_empty()) {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(branches.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                branch = __pa2.clone();
                branches = __pa3.clone();
                s = IOStream::append(s.clone(), (indent.clone()).clone())?;
                s = Branch::toStream(branch.clone(), (literal!("elseif ")).clone(), branches.clone().is_empty(), (indent.clone()).clone(), s.clone())?;
            }
            s = IOStream::append(s, (indent).clone())?;
            s = IOStream::append(s, (literal!("end if")).clone())?;
            s
        },
        Deref @ WHEN { .. } => {
            s = Branch::toStream(listHead(var_field!((*eq).branches, NFEquation::WHEN).clone())?, (literal!("when ")).clone(), false, (indent.clone()).clone(), s)?;
            for mut b in &*listRest(var_field!((*eq).branches, NFEquation::WHEN).clone())? {
                let mut b = b.clone();
                s = IOStream::append(s.clone(), (indent.clone()).clone())?;
                s = Branch::toStream(b.clone(), (literal!("elsewhen ")).clone(), false, (indent.clone()).clone(), s.clone())?;
            }
            s = IOStream::append(s, (indent).clone())?;
            s = IOStream::append(s, (literal!("end when")).clone())?;
            s
        },
        Deref @ ASSERT { .. } => {
            s = IOStream::append(s, (literal!("assert(")).clone())?;
            s = IOStream::append(s, (Expression::toString(var_field!((*eq).condition, NFEquation::ASSERT).clone())?).clone())?;
            s = IOStream::append(s, (literal!(", ")).clone())?;
            s = IOStream::append(s, (Expression::toString(var_field!((*eq).message, NFEquation::ASSERT).clone())?).clone())?;
            s = IOStream::append(s, (literal!(", ")).clone())?;
            s = IOStream::append(s, (Expression::toString(var_field!((*eq).level, NFEquation::ASSERT).clone())?).clone())?;
            s = IOStream::append(s, (literal!(")")).clone())?;
            s
        },
        Deref @ TERMINATE { .. } => {
            s = IOStream::append(s, (literal!("terminate(")).clone())?;
            s = IOStream::append(s, (Expression::toString(var_field!((*eq).message, NFEquation::TERMINATE).clone())?).clone())?;
            s = IOStream::append(s, (literal!(")")).clone())?;
            s
        },
        Deref @ REINIT { .. } => {
            s = IOStream::append(s, (literal!("reinit(")).clone())?;
            s = IOStream::append(s, (Expression::toString(var_field!((*eq).cref, NFEquation::REINIT).clone())?).clone())?;
            s = IOStream::append(s, (literal!(", ")).clone())?;
            s = IOStream::append(s, (Expression::toString(var_field!((*eq).reinitExp, NFEquation::REINIT).clone())?).clone())?;
            s = IOStream::append(s, (literal!(")")).clone())?;
            s
        },
        Deref @ NORETCALL { .. } => IOStream::append(s, (Expression::toString(var_field!((*eq).exp, NFEquation::NORETCALL).clone())?).clone())?,
        _ => IOStream::append(s, (literal!("#UNKNOWN EQUATION#")).clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s)
}

pub(crate) fn toStreamList(mut eql: Arc<metamodelica::List<Arc<NFEquation>>>, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut prev_multi_line: bool = false;
    let mut multi_line: bool;
    let mut first: bool = true;
    for mut eq in &*eql {
        let mut eq = eq.clone();
        multi_line = isMultiLine(eq.clone());
        if first {
            first = false;
        } else if prev_multi_line || multi_line {
            s = IOStream::append(s.clone(), (literal!("\n")).clone())?;
        }
        prev_multi_line = multi_line;
        s = toStream(eq.clone(), (indent.clone()).clone(), s.clone())?;
        s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
    }
    Ok(s)
}

pub(crate) fn toFlatStream(mut eq: Arc<NFEquation>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut branches: Arc<metamodelica::List<Arc<Branch::Branch>>> = metamodelica::nil();
    let mut branch: Arc<Branch::Branch> = Arc::new(<Branch::Branch as ::std::default::Default>::default());
    s = IOStream::append(s, (indent.clone()).clone())?;
    s = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ EQUALITY { .. } => {
            s = IOStream::append(s, (Expression::toFlatString(var_field!((*eq).lhs, NFEquation::EQUALITY).clone(), format.clone())?).clone())?;
            s = IOStream::append(s, (literal!(" = ")).clone())?;
            s = IOStream::append(s, (Expression::toFlatString(var_field!((*eq).rhs, NFEquation::EQUALITY).clone(), format)?).clone())?;
            s
        },
        Deref @ CONNECT { .. } => {
            s = IOStream::append(s, (literal!("connect(")).clone())?;
            s = IOStream::append(s, (Expression::toFlatString(var_field!((*eq).lhs, NFEquation::CONNECT).clone(), format.clone())?).clone())?;
            s = IOStream::append(s, (literal!(", ")).clone())?;
            s = IOStream::append(s, (Expression::toFlatString(var_field!((*eq).rhs, NFEquation::CONNECT).clone(), format)?).clone())?;
            s = IOStream::append(s, (literal!(")")).clone())?;
            s
        },
        Deref @ FOR { .. } => {
            s = IOStream::append(s, (literal!("for ")).clone())?;
            s = IOStream::append(s, (Util::makeQuotedIdentifier((InstNode::name(var_field!((*eq).iterator, NFEquation::FOR).clone())?).clone())?).clone())?;
            if isSome(var_field!((*eq).range, NFEquation::FOR).clone()) {
                s = IOStream::append(s, (literal!(" in ")).clone())?;
                s = IOStream::append(s, (Expression::toFlatString(Util::getOption(var_field!((*eq).range, NFEquation::FOR).clone())?, format.clone())?).clone())?;
            }
            s = IOStream::append(s, (literal!(" loop\n")).clone())?;
            s = toFlatStreamList(var_field!((*eq).body, NFEquation::FOR).clone(), format, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*indent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone(), s)?;
            s = IOStream::append(s, (indent).clone())?;
            s = IOStream::append(s, (literal!("end for")).clone())?;
            s
        },
        Deref @ IF { .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(var_field!((*eq).branches, NFEquation::IF).clone()) {
                Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            branch = __pa0.clone();
            branches = __pa1.clone();
            s = Branch::toFlatStream(branch.clone(), (literal!("if ")).clone(), format.clone(), false, (indent.clone()).clone(), s)?;
            while !(branches.clone().is_empty()) {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(branches.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                branch = __pa2.clone();
                branches = __pa3.clone();
                s = IOStream::append(s.clone(), (indent.clone()).clone())?;
                s = Branch::toFlatStream(branch.clone(), (literal!("elseif ")).clone(), format.clone(), branches.clone().is_empty(), (indent.clone()).clone(), s.clone())?;
            }
            s = IOStream::append(s, (indent).clone())?;
            s = IOStream::append(s, (literal!("end if")).clone())?;
            s
        },
        Deref @ WHEN { .. } => {
            s = Branch::toFlatStream(listHead(var_field!((*eq).branches, NFEquation::WHEN).clone())?, (literal!("when ")).clone(), format.clone(), false, (indent.clone()).clone(), s)?;
            for mut b in &*listRest(var_field!((*eq).branches, NFEquation::WHEN).clone())? {
                let mut b = b.clone();
                s = IOStream::append(s.clone(), (indent.clone()).clone())?;
                s = Branch::toFlatStream(b.clone(), (literal!("elsewhen ")).clone(), format.clone(), false, (indent.clone()).clone(), s.clone())?;
            }
            s = IOStream::append(s, (indent).clone())?;
            s = IOStream::append(s, (literal!("end when")).clone())?;
            s
        },
        Deref @ ASSERT { .. } => {
            s = IOStream::append(s, (literal!("assert(")).clone())?;
            s = IOStream::append(s, (Expression::toFlatString(var_field!((*eq).condition, NFEquation::ASSERT).clone(), format.clone())?).clone())?;
            s = IOStream::append(s, (literal!(", ")).clone())?;
            s = IOStream::append(s, (Expression::toFlatString(var_field!((*eq).message, NFEquation::ASSERT).clone(), format.clone())?).clone())?;
            s = IOStream::append(s, (literal!(", ")).clone())?;
            s = IOStream::append(s, (Expression::toFlatString(var_field!((*eq).level, NFEquation::ASSERT).clone(), format)?).clone())?;
            s = IOStream::append(s, (literal!(")")).clone())?;
            s
        },
        Deref @ TERMINATE { .. } => {
            s = IOStream::append(s, (literal!("terminate(")).clone())?;
            s = IOStream::append(s, (Expression::toFlatString(var_field!((*eq).message, NFEquation::TERMINATE).clone(), format)?).clone())?;
            s = IOStream::append(s, (literal!(")")).clone())?;
            s
        },
        Deref @ REINIT { .. } => {
            s = IOStream::append(s, (literal!("reinit(")).clone())?;
            s = IOStream::append(s, (Expression::toFlatString(var_field!((*eq).cref, NFEquation::REINIT).clone(), format.clone())?).clone())?;
            s = IOStream::append(s, (literal!(", ")).clone())?;
            s = IOStream::append(s, (Expression::toFlatString(var_field!((*eq).reinitExp, NFEquation::REINIT).clone(), format)?).clone())?;
            s = IOStream::append(s, (literal!(")")).clone())?;
            s
        },
        Deref @ NORETCALL { .. } => IOStream::append(s, (Expression::toFlatString(var_field!((*eq).exp, NFEquation::NORETCALL).clone(), format)?).clone())?,
        _ => IOStream::append(s, (literal!("#UNKNOWN EQUATION#")).clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    s = FlatModelicaUtil::appendElementSourceComment(source(eq)?, FlatModelicaUtil::ElementType::EQUATION.clone(), s)?;
    Ok(s)
}

pub(crate) fn toFlatStreamList(mut eql: Arc<metamodelica::List<Arc<NFEquation>>>, mut format: BaseModelica::OutputFormat, mut indent: ArcStr, mut s: IOStream::IOStream) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut prev_multi_line: bool = false;
    let mut multi_line: bool;
    let mut first: bool = true;
    for mut eq in &*eql {
        let mut eq = eq.clone();
        multi_line = isMultiLine(eq.clone());
        if first {
            first = false;
        } else if prev_multi_line || multi_line {
            s = IOStream::append(s.clone(), (literal!("\n")).clone())?;
        }
        prev_multi_line = multi_line;
        s = toFlatStream(eq.clone(), format.clone(), (indent.clone()).clone(), s.clone())?;
        s = IOStream::append(s.clone(), (literal!(";\n")).clone())?;
    }
    Ok(s)
}

pub(crate) fn isMultiLine(mut eq: Arc<NFEquation>) -> bool {
    let mut singleLine: bool;
    singleLine = (::match_deref::match_deref! { match &(eq) {
        Deref @ FOR { .. } => true,
        Deref @ IF { .. } => true,
        Deref @ WHEN { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    singleLine
}

pub(crate) fn splitRecordEquations(mut equations: Arc<metamodelica::List<Arc<NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<NFEquation>>>> {
    let mut outEquations: Arc<metamodelica::List<Arc<NFEquation>>> = metamodelica::nil();
    for mut eq in &*equations {
        let mut eq = eq.clone();
        outEquations = splitRecordEquation(eq.clone(), outEquations.clone())?;
    }
    outEquations = metamodelica::Dangerous::listReverseInPlace(outEquations);
    Ok(outEquations)
}

pub(crate) fn splitRecordEquation(mut eq: Arc<NFEquation>, mut equations: Arc<metamodelica::List<Arc<NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<NFEquation>>>> {
    let mut equations: Arc<metamodelica::List<Arc<NFEquation>>> = equations;
    let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    equations = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ EQUALITY { .. } if (Type::isRecord(Type::arrayElementType(var_field!((*eq).ty, NFEquation::EQUALITY).clone()))) => {
            assign_variant_field!(eq => NFEquation::EQUALITY;
                lhs = ExpandExp::expand(var_field!((*eq).lhs, NFEquation::EQUALITY).clone(), false, false)?.0,
                rhs = ExpandExp::expand(var_field!((*eq).rhs, NFEquation::EQUALITY).clone(), false, false)?.0
            );
            for mut i in 1..=Type::recordFieldCount(Type::arrayElementType(var_field!((*eq).ty, NFEquation::EQUALITY).clone())) {
                lhs = Expression::nthRecordElement(i.clone(), var_field!((*eq).lhs, NFEquation::EQUALITY).clone())?;
                rhs = Expression::nthRecordElement(i.clone(), var_field!((*eq).rhs, NFEquation::EQUALITY).clone())?;
                equations = metamodelica::cons(Arc::new(NFEquation::EQUALITY { lhs: lhs.clone(), rhs: rhs.clone(), ty: Expression::typeOf(lhs.clone()), scope: var_field!((*eq).scope, NFEquation::EQUALITY).clone(), source: var_field!((*eq).source, NFEquation::EQUALITY).clone(), scalarizeMode: var_field!((*eq).scalarizeMode, NFEquation::EQUALITY).clone() }), equations.clone());
            }
            equations
        },
        Deref @ FOR { .. } => {
            assign_variant_field!(eq => NFEquation::FOR; body = splitRecordEquations(var_field!((*eq).body, NFEquation::FOR).clone())?);
            metamodelica::cons(eq.clone(), equations)
        },
        Deref @ IF { .. } => {
            assign_variant_field!(eq => NFEquation::IF; branches = ({
        let mut __acc: Arc<metamodelica::List<Arc<Branch::Branch>>> = metamodelica::nil();
        for mut b in (var_field!((*eq).branches, NFEquation::IF).clone()).into_iter().cloned() {
            let __x = splitRecordEquationBranch(b.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            metamodelica::cons(eq.clone(), equations)
        },
        Deref @ WHEN { .. } => {
            assign_variant_field!(eq => NFEquation::WHEN; branches = ({
        let mut __acc: Arc<metamodelica::List<Arc<Branch::Branch>>> = metamodelica::nil();
        for mut b in (var_field!((*eq).branches, NFEquation::WHEN).clone()).into_iter().cloned() {
            let __x = splitRecordEquationBranch(b.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            metamodelica::cons(eq.clone(), equations)
        },
        _ => metamodelica::cons(eq.clone(), equations),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equations)
}

pub(crate) fn splitRecordEquationBranch(mut branch: Arc<Branch::Branch>) -> Result<Arc<Branch::Branch>> {
    let mut branch: Arc<Branch::Branch> = branch;
    let () = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Branch::BRANCH { .. } => {
            assign_variant_field!(branch => Branch::Branch::BRANCH; body = splitRecordEquations(var_field!((*branch).body, Branch::Branch::BRANCH).clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(branch)
}


