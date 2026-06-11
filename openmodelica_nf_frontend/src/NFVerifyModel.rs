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

use crate::NFAlgorithm as Algorithm;
use crate::NFBinding as Binding;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_error::ErrorTypes;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub struct NFVerifyModel;
pub fn verify(mut flatModel: Arc<FlatModel::NFFlatModel>, mut isPartial: bool) -> Result<()> {
    for mut var in &*flatModel.variables.clone() {
        let mut var = var.clone();
        verifyVariable(var.clone(), isPartial)?;
    }
    for mut eq in &*flatModel.equations.clone() {
        let mut eq = eq.clone();
        verifyEquation(eq.clone(), isPartial)?;
    }
    for mut ieq in &*flatModel.initialEquations.clone() {
        let mut ieq = ieq.clone();
        verifyEquation(ieq.clone(), isPartial)?;
    }
    for mut alg in &*flatModel.algorithms.clone() {
        let mut alg = alg.clone();
        verifyAlgorithm(alg.clone(), isPartial)?;
    }
    for mut ialg in &*flatModel.initialAlgorithms.clone() {
        let mut ialg = ialg.clone();
        verifyAlgorithm(ialg.clone(), isPartial)?;
    }
    if !(isPartial) {
        checkDiscreteReal(flatModel)?;
    }
    execStat(literal!("NFVerifyModel.verify"))?;
    Ok(())
}

fn verifyVariable(mut var: Arc<Variable::NFVariable>, mut isPartial: bool) -> Result<()> {
    verifyBinding(var.binding.clone(), isPartial)?;
    for mut attr in &*var.typeAttributes.clone() {
        let mut attr = attr.clone();
        verifyBinding(Util::tuple22(attr.clone()), isPartial)?;
    }
    for mut v in &*var.children.clone() {
        let mut v = v.clone();
        verifyVariable(v.clone(), isPartial)?;
    }
    Ok(())
}

fn verifyBinding(mut binding: Arc<Binding::NFBinding>, mut isPartial: bool) -> Result<()> {
    if Binding::isBound(binding.clone()) {
        checkSubscriptBounds(Binding::getTypedExp(binding.clone())?, isPartial, Binding::getInfo(binding))?;
    }
    Ok(())
}

fn verifyEquation(mut eq: Arc<Equation::NFEquation>, mut isPartial: bool) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::WHEN { .. } if (!(isPartial)) => {
            verifyWhenEquation(var_field!((*eq).branches, Equation::NFEquation::WHEN).clone(), var_field!((*eq).source, Equation::NFEquation::WHEN).clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Equation::applyExpShallow(eq.clone(), (std::sync::Arc::new({ let __pe_b1 = isPartial; let __pe_b2 = Equation::info(eq)?; move |__pe_a0| checkSubscriptBounds(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>))?;
    Ok(())
}

fn verifyWhenEquation(mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>, mut source: Arc<DAE::ElementSource>) -> Result<()> {
    let mut crefs1: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut crefs2: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut rest_branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>;
    let mut body: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    if List::hasOneElement(branches.clone()) {
        return Ok(());
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(branches) {
        Deref @ metamodelica::List::Cons { head: Deref @ Equation::Branch::BRANCH { body: __pa0, .. }, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    body = __pa0.clone();
    rest_branches = __pa1.clone();
    crefs1 = whenEquationBranchCrefs(body.clone())?;
    for mut branch in &*rest_branches {
        let mut branch = branch.clone();
        let __pa2 = ::match_deref::match_deref! { match &(branch.clone()) {
            Deref @ Equation::Branch::BRANCH { body: __pa2, .. } => __pa2.clone(),
            _ => bail!("pattern mismatch"),
        } };
        body = __pa2.clone();
        crefs2 = whenEquationBranchCrefs(body.clone())?;
        checkCrefSetEquality(crefs1.clone(), crefs2.clone(), Error::DIFFERENT_VARIABLES_SOLVED_IN_ELSEWHEN.clone(), source.clone())?;
    }
    Ok(())
}

fn whenEquationBranchCrefs(mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    for mut eq in &*eql {
        let mut eq = eq.clone();
        crefs = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { .. } => whenEquationEqualityCrefs(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone(), crefs.clone())?,
        Deref @ Equation::IF { .. } => whenEquationIfCrefs(var_field!((*eq).branches, Equation::NFEquation::IF).clone(), var_field!((*eq).source, Equation::NFEquation::IF).clone(), crefs.clone())?,
        _ => crefs.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    crefs = List::sort(crefs, (std::sync::Arc::new(ComponentRef::isGreater) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
    crefs = List::sortedUnique(crefs, (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
    Ok(crefs)
}

fn whenEquationEqualityCrefs(mut lhsExp: Arc<Expression::NFExpression>, mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = crefs;
    crefs = (::match_deref::match_deref! { match &(lhsExp.clone()) {
        Deref @ Expression::CREF { .. } => metamodelica::cons(var_field!((*lhsExp).cref, Expression::NFExpression::CREF).clone(), crefs),
        Deref @ Expression::TUPLE { .. } => List::fold(var_field!((*lhsExp).elements, Expression::NFExpression::TUPLE).clone(), (std::sync::Arc::new(whenEquationEqualityCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> + 'static>), crefs)?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(crefs)
}

fn whenEquationIfCrefs(mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>, mut source: Arc<DAE::ElementSource>, mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = crefs;
    let mut crefs1: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut crefs2: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut rest_branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>;
    let mut body: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(branches) {
        Deref @ metamodelica::List::Cons { head: Deref @ Equation::Branch::BRANCH { body: __pa0, .. }, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    body = __pa0.clone();
    rest_branches = __pa1.clone();
    crefs1 = whenEquationBranchCrefs(body.clone())?;
    for mut branch in &*rest_branches {
        let mut branch = branch.clone();
        let __pa2 = ::match_deref::match_deref! { match &(branch.clone()) {
            Deref @ Equation::Branch::BRANCH { body: __pa2, .. } => __pa2.clone(),
            _ => bail!("pattern mismatch"),
        } };
        body = __pa2.clone();
        crefs2 = whenEquationBranchCrefs(body.clone())?;
        checkCrefSetEquality(crefs1.clone(), crefs2.clone(), Error::WHEN_IF_VARIABLE_MISMATCH.clone(), source.clone())?;
    }
    crefs = listAppend(crefs1, crefs);
    Ok(crefs)
}

fn checkCrefSetEquality(mut crefs1: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut crefs2: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut errMsg: ErrorTypes::Message, mut source: Arc<DAE::ElementSource>) -> Result<()> {
    if List::isEqualOnTrue(crefs1.clone(), crefs2.clone(), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))? {
        return Ok(());
    }
    if List::isEqualOnTrue(expandCrefSet(crefs1)?, expandCrefSet(crefs2)?, (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))? {
        return Ok(());
    }
    Error::addSourceMessage(errMsg, metamodelica::nil(), ElementSource::getInfo(source))?;
    bail!("fail");
    Ok(())
}

fn expandCrefSet(mut crefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
    let mut outCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut exp: Arc<Expression::NFExpression>;
    let mut expl: metamodelica::Array<Arc<Expression::NFExpression>>;
    for mut cref in &*crefs {
        let mut cref = cref.clone();
        exp = Expression::fromCref(cref.clone(), false)?;
        (exp, _) = ExpandExp::expandCref(exp.clone(), false, false)?;
        if Expression::isArray(exp.clone()) {
            expl = Expression::arrayElements(exp.clone())?;
            outCrefs = listAppend(({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut e in (expl.clone()).borrow().iter() {
            let __x = Expression::toCref(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), outCrefs.clone());
        } else {
            outCrefs = metamodelica::cons(cref.clone(), outCrefs.clone());
        }
    }
    outCrefs = List::sort(outCrefs, (std::sync::Arc::new(ComponentRef::isGreater) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
    outCrefs = List::sortedUnique(outCrefs, (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
    Ok(outCrefs)
}

fn verifyAlgorithm(mut alg: Arc<Algorithm::NFAlgorithm>, mut isPartial: bool) -> Result<()> {
    Algorithm::apply(alg, (std::sync::Arc::new({ let __pe_b1 = isPartial; move |__pe_a0| verifyStatement(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>) -> Result<()> + 'static>))?;
    Ok(())
}

fn verifyStatement(mut stmt: Arc<Statement::NFStatement>, mut isPartial: bool) -> Result<()> {
    Statement::applyExp(stmt.clone(), (std::sync::Arc::new({ let __pe_b1 = isPartial; let __pe_b2 = Statement::info(stmt)?; move |__pe_a0| checkSubscriptBounds(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>))?;
    Ok(())
}

fn checkSubscriptBounds(mut exp: Arc<Expression::NFExpression>, mut isPartial: bool, mut info: SourceInfo) -> Result<()> {
    Expression::apply(exp, (std::sync::Arc::new({ let __pe_b1 = isPartial; let __pe_b2 = info; move |__pe_a0| checkSubscriptBounds_traverser(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<()> + 'static>))?;
    Ok(())
}

fn checkSubscriptBounds_traverser(mut exp: Arc<Expression::NFExpression>, mut isPartial: bool, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            checkSubscriptBoundsCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), isPartial, info)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn checkSubscriptBoundsCref(mut cref: Arc<ComponentRef::NFComponentRef>, mut isPartial: bool, mut info: SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { subscripts: subs @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, ty: Deref @ Type::ARRAY { dimensions: dims, .. }, .. } => {
            let mut d: Arc<Dimension::NFDimension>;
            let mut int_sub: i32;
            let mut index: i32;
            let mut dims = (*dims).clone();
            index = 1;
            for mut s in &*subs.clone() {
                let mut s = s.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dims.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                d = __pa0.clone();
                dims = __pa1.clone();
                if Subscript::isScalarLiteral(s.clone()) && Dimension::isKnown(d.clone(), false) {
                    int_sub = Subscript::toInteger(s.clone())?;
                    if int_sub < 1 || int_sub > Dimension::size(d.clone(), false)? {
                        Error::addSourceMessage(Error::ARRAY_INDEX_OUT_OF_BOUNDS.clone(), list![(Subscript::toString(s.clone())?).clone(), ArcStr::from(::std::format!("{}", index)), (Dimension::toString(d.clone())?).clone(), (ComponentRef::firstName(cref.clone(), false)?).clone()], info.clone())?;
                        if !(isPartial) {
                            bail!("fail");
                        }
                    }
                }
                index = index + 1;
            }
            checkSubscriptBoundsCref(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), isPartial, info)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn checkDiscreteReal(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<()> {
    let mut discrete_reals: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
    let mut illegal_discrete_vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    discrete_reals = UnorderedSet::new((std::sync::Arc::new(ComponentRef::hashStrip) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqualStrip) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    for mut eqn in &*flatModel.equations.clone() {
        let mut eqn = eqn.clone();
        checkDiscreteRealEquation(eqn.clone(), discrete_reals.clone(), false)?;
    }
    for mut alg in &*flatModel.algorithms.clone() {
        let mut alg = alg.clone();
        for mut statement in &*alg.statements.clone() {
            let mut statement = statement.clone();
            checkDiscreteRealStatement(statement.clone(), discrete_reals.clone(), false)?;
        }
    }
    for mut variable in &*flatModel.variables.clone() {
        let mut variable = variable.clone();
        if Variable::variability(variable.clone()) == Variability::DISCRETE.clone() && Type::isReal(Type::arrayElementType(variable.ty.clone()))? && !(UnorderedSet::contains(variable.name.clone(), discrete_reals.clone())?) {
            illegal_discrete_vars = metamodelica::cons(variable.clone(), illegal_discrete_vars.clone());
        }
    }
    if !(illegal_discrete_vars.clone().is_empty()) {
        for mut var in &*illegal_discrete_vars {
            let mut var = var.clone();
            Error::addSourceMessage(Error::DISCRETE_REAL_UNDEFINED.clone(), list![(ComponentRef::toString(ComponentRef::stripSubscriptsAll(var.name.clone()))?).clone()], var.info.clone())?;
        }
        bail!("fail");
    }
    Ok(())
}

fn checkDiscreteRealBranch(mut branch: Arc<Equation::Branch::Branch>, mut discreteReals: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut when_found: bool) -> Result<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>> {
    let mut discreteReals: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = discreteReals;
    let () = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Equation::Branch::BRANCH { .. } if (when_found) => {
            for mut eqn in &*var_field!((*branch).body, Equation::Branch::Branch::BRANCH).clone() {
                let mut eqn = eqn.clone();
                checkDiscreteRealEquation(eqn.clone(), discreteReals.clone(), when_found)?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(discreteReals)
}

fn checkDiscreteRealEquation(mut body_eqn: Arc<Equation::NFEquation>, mut discreteReals: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut when_found: bool) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(body_eqn) {
        Deref @ Equation::EQUALITY { lhs, .. } if (when_found) => {
            checkDiscreteRealExp(lhs.clone(), discreteReals)?;
            ()
        },
        Deref @ Equation::IF { branches, .. } => {
            for mut branch in &*branches.clone() {
                let mut branch = branch.clone();
                checkDiscreteRealBranch(branch.clone(), discreteReals.clone(), when_found)?;
            }
            ()
        },
        Deref @ Equation::WHEN { branches, .. } => {
            for mut branch in &*branches.clone() {
                let mut branch = branch.clone();
                checkDiscreteRealBranch(branch.clone(), discreteReals.clone(), true)?;
            }
            ()
        },
        Deref @ Equation::FOR { body, .. } => {
            for mut eqn in &*body.clone() {
                let mut eqn = eqn.clone();
                checkDiscreteRealEquation(eqn.clone(), discreteReals.clone(), when_found)?;
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

fn checkDiscreteRealStatement(mut statement: Arc<Statement::NFStatement>, mut discreteReals: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut when_found: bool) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(statement.clone()) {
        Deref @ Statement::WHEN { branches, .. } => {
            let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>>;
            for mut branch in &*branches.clone() {
                let mut branch = branch.clone();
                (_, body) = branch.clone();
                for mut statement in &*body.clone() {
                    let mut statement = statement.clone();
                    checkDiscreteRealStatement(statement.clone(), discreteReals.clone(), true)?;
                }
            }
            ()
        },
        Deref @ Statement::ASSIGNMENT { lhs, .. } if (when_found) => {
            checkDiscreteRealExp(lhs.clone(), discreteReals)?;
            ()
        },
        Deref @ Statement::IF { branches, .. } => {
            let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>>;
            for mut branch in &*branches.clone() {
                let mut branch = branch.clone();
                (_, body) = branch.clone();
                for mut stmt in &*body.clone() {
                    let mut stmt = stmt.clone();
                    checkDiscreteRealStatement(stmt.clone(), discreteReals.clone(), when_found)?;
                }
            }
            ()
        },
        Deref @ Statement::FOR { body, .. } => {
            for mut statement in &*body.clone() {
                let mut statement = statement.clone();
                checkDiscreteRealStatement(statement.clone(), discreteReals.clone(), when_found)?;
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

fn checkDiscreteRealExp(mut exp: Arc<Expression::NFExpression>, mut discreteReals: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(exp) {
        Deref @ Expression::CREF { ty, cref } if (Type::isReal(Type::arrayElementType(ty.clone()))?) => {
            UnorderedSet::add(cref.clone(), discreteReals)?;
            ()
        },
        Deref @ Expression::CREF { ty: ty @ Deref @ Type::COMPLEX { cls, .. }, cref } if (Type::isRecord(ty.clone())) => {
            checkDiscreteRealRecord(cref.clone(), cls.clone(), discreteReals)?;
            ()
        },
        Deref @ Expression::TUPLE { elements, .. } => {
            for mut element in &*elements.clone() {
                let mut element = element.clone();
                checkDiscreteRealExp(element.clone(), discreteReals.clone())?;
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

fn checkDiscreteRealRecord(mut cref: Arc<ComponentRef::NFComponentRef>, mut cls: Arc<InstNode::InstNode>, mut discreteReals: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
    let mut element: Arc<ComponentRef::NFComponentRef>;
    let mut inputs: Arc<metamodelica::List<Arc<InstNode::InstNode>>>;
    UnorderedSet::add(cref.clone(), discreteReals.clone())?;
    (inputs, _, _) = Record::collectRecordParams(cls)?;
    for mut node in &*inputs {
        let mut node = node.clone();
        element = ComponentRef::prefixCref(node.clone(), InstNode::getType(node.clone())?, metamodelica::nil(), cref.clone());
        UnorderedSet::add(element.clone(), discreteReals.clone())?;
    }
    Ok(())
}


