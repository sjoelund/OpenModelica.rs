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
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFEquation::Branch;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten as Flatten;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFPackage as Package;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFSections as Sections;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_error::ErrorExt;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::Util;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct EvalSettings {
    pub scalarize: bool,
}

impl metamodelica::gc::MMTrace for EvalSettings {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.scalarize, __mmv)?;
        Ok(())
    }
}
impl Default for EvalSettings {
    fn default() -> Self {
        Self {
            scalarize: Default::default(),
        }
    }
}

pub type SETTINGS = EvalSettings;


pub fn evaluate(mut flatModel: Arc<FlatModel::NFFlatModel>, mut context: i32) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut settings: EvalSettings;
    settings = EvalSettings { scalarize: Flags::isSet(Flags::NF_SCALARIZE.clone())? };
    assign_field!(
        flatModel.variables = ({
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = evaluateVariable(v.clone(), context, settings.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        flatModel.equations = evaluateEquations(flatModel.equations.clone())?,
        flatModel.initialEquations = evaluateEquations(flatModel.initialEquations.clone())?,
        flatModel.algorithms = evaluateAlgorithms(flatModel.algorithms.clone())?,
        flatModel.initialAlgorithms = evaluateAlgorithms(flatModel.initialAlgorithms.clone())?
    );
    execStat(literal!("NFEvalConstants.evaluate"))?;
    Ok(flatModel)
}

pub(crate) fn evaluateVariable(mut var: Arc<Variable::NFVariable>, mut context: i32, mut settings: EvalSettings) -> Result<Arc<Variable::NFVariable>> {
    let mut var: Arc<Variable::NFVariable> = var;
    let mut binding: Arc<Binding::NFBinding>;
    let mut structural: bool;
    let mut variability: Variability;
    variability = Variable::variability(var.clone());
    structural = variability <= Variability::STRUCTURAL_PARAMETER.clone() && !(Type::isExternalObject(var.ty.clone()));
    binding = evaluateBinding(var.binding.clone(), var.name.clone(), structural, variability, context)?;
    if !(referenceEq(&*(binding.clone()),&*(var.binding.clone()))) {
        assign_field!(var.binding = binding);
    }
    assign_field!(
        var.typeAttributes = ({
        let mut __acc: Arc<metamodelica::List<(ArcStr, Arc<Binding::NFBinding>)>> = metamodelica::nil();
        for mut a in (var.typeAttributes.clone()).into_iter().cloned() {
            let __x = evaluateTypeAttribute(a.clone(), var.name.clone(), context)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        var.children = ({
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (var.children.clone()).into_iter().cloned() {
            let __x = evaluateVariable(v.clone(), context, settings.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
    );
    Ok(var)
}

pub(crate) fn evaluateBinding(mut binding: Arc<Binding::NFBinding>, mut prefix: Arc<ComponentRef::NFComponentRef>, mut structural: bool, mut variability: Variability, mut context: i32) -> Result<Arc<Binding::NFBinding>> {
    let mut binding: Arc<Binding::NFBinding> = binding;
    let mut exp: Arc<Expression::NFExpression>;
    let mut eexp: Arc<Expression::NFExpression>;
    let mut info: SourceInfo;
    if Binding::isBound(binding.clone()) {
        exp = Binding::getTypedExp(binding.clone())?;
        if structural {
            info = Binding::getInfo(binding.clone());
            eexp = evaluateExp(exp.clone(), info.clone())?;
            eexp = SimplifyExp::simplify(eexp, false)?;
            if !(Expression::isLiteral(eexp.clone())? || Expression::isKnownSizeFill(eexp.clone())?) {
                if variability > Variability::CONSTANT.clone() || InstContext::inRelaxed(context) {
                    eexp = Ceval::tryEvalExp(eexp, Ceval::noTarget().clone());
                } else {
                    eexp = Ceval::evalExp(eexp, Ceval::EvalTarget::new(info.clone(), context, None))?;
                }
            }
            eexp = Flatten::flattenExp(eexp, Arc::new(Flatten::Prefix::Prefix::PREFIX { root: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), prefix: prefix }), info)?;
        } else {
            eexp = evaluateExp(exp.clone(), Binding::getInfo(binding.clone()))?;
        }
        if !(referenceEq(&*(exp),&*(eexp.clone()))) {
            binding = Binding::setTypedExp(eexp, binding)?;
        }
    }
    Ok(binding)
}

pub(crate) fn evaluateTypeAttribute(mut attribute: (ArcStr, Arc<Binding::NFBinding>), mut prefix: Arc<ComponentRef::NFComponentRef>, mut context: i32) -> Result<(ArcStr, Arc<Binding::NFBinding>)> {
    let mut attribute: (ArcStr, Arc<Binding::NFBinding>) = attribute;
    let mut name: ArcStr;
    let mut binding: Arc<Binding::NFBinding>;
    let mut sbinding: Arc<Binding::NFBinding>;
    let mut structural: bool;
    (name, binding) = attribute.clone();
    structural = name.clone() == literal!("fixed") || name.clone() == literal!("stateSelect");
    sbinding = evaluateBinding(binding.clone(), prefix, structural, Variability::PARAMETER.clone(), context)?;
    if !(referenceEq(&*(binding),&*(sbinding.clone()))) {
        attribute = (name, sbinding);
    }
    Ok(attribute)
}

pub fn evaluateExp(mut exp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression>;
    (outExp, _) = evaluateExpTraverser(exp, info, false)?;
    Ok(outExp)
}

pub(crate) fn evaluateExpTraverser(mut exp: Arc<Expression::NFExpression>, mut info: SourceInfo, mut changed: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outChanged: bool = false;
    let mut e: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut ty2: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut var: Variability = Variability::CONSTANT;
    (outExp, outChanged) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            let (__pa2, __pa0, __pa1, __pa3) = ::match_deref::match_deref! { match &(Expression::mapFoldShallow(exp, (std::sync::Arc::new({ let __pe_b1 = info.clone(); move |__pe_a0, __pe_a2| evaluateExpTraverser(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, bool) -> Result<(Arc<Expression::NFExpression>, bool)> + 'static>), false)?) {
                (__pa2 @ Deref @ Expression::CREF { cref: __pa0, ty: __pa1 }, __pa3) => (__pa2.clone(), __pa0.clone(), __pa1.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cref = __pa0.clone();
            ty = __pa1.clone();
            outExp = __pa2.clone();
            outChanged = __pa3.clone();
            var = ComponentRef::nodeVariability(cref.clone())?;
            if var <= Variability::STRUCTURAL_PARAMETER.clone() && !(Type::isExternalObject(ty.clone())) {
                if var > Variability::CONSTANT.clone() {
                    ErrorExt::setCheckpoint(literal!("NFEvalConstants.evaluateExpTraverser"));
                    if '__try4: {
                        e = unwrap_break_err!(Ceval::evalCref(cref.clone(), outExp.clone(), Ceval::noTarget().clone(), false, true), '__try4);
                        e = unwrap_break_err!(Flatten::flattenExp(e.clone(), Arc::new(Flatten::Prefix::Prefix::PREFIX { root: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), prefix: cref.clone() }), info.clone()), '__try4);
                        outExp = e.clone();
                        outChanged = true;
                        Ok::<(), anyhow::Error>(())
                    }.is_err() {
                    }
                    ErrorExt::rollBack(literal!("NFEvalConstants.evaluateExpTraverser"));
                } else {
                    outExp = Ceval::evalCref(cref.clone(), outExp, Ceval::EvalTarget::new(info.clone(), InstContext::NO_CONTEXT.clone(), None), false, true)?;
                    outExp = Flatten::flattenExp(outExp, Arc::new(Flatten::Prefix::Prefix::PREFIX { root: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), prefix: cref }), info.clone())?;
                    outChanged = true;
                }
            } else if outChanged {
                ty = ComponentRef::getSubscriptedType(cref, false)?;
            }
            ty2 = evaluateType(ty.clone(), info)?;
            if !(referenceEq(&*(ty),&*(ty2.clone()))) {
                outExp = Expression::setType(ty2, outExp)?;
            }
            (outExp, outChanged)
        },
        Deref @ Expression::ARRAY { literal: true, .. } => (exp, false),
        Deref @ Expression::IF { .. } => evaluateIfExp(exp, info)?,
        Deref @ Expression::SIZE { .. } => {
            if isSome(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*exp).dimIndex, Expression::NFExpression::SIZE).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                e = __pa0.clone();
                (e, outChanged) = Expression::mapFoldShallow(e, (std::sync::Arc::new({ let __pe_b1 = info; move |__pe_a0, __pe_a2| evaluateExpTraverser(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, bool) -> Result<(Arc<Expression::NFExpression>, bool)> + 'static>), false)?;
                if outChanged {
                    assign_variant_field!(exp => Expression::NFExpression::SIZE; dimIndex = Some(e));
                }
            }
            (exp, outChanged)
        },
        Deref @ Expression::RANGE { .. } => {
            (outExp, outChanged) = Expression::mapFoldShallow(exp, (std::sync::Arc::new({ let __pe_b1 = info; move |__pe_a0, __pe_a2| evaluateExpTraverser(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, bool) -> Result<(Arc<Expression::NFExpression>, bool)> + 'static>), false)?;
            if outChanged {
                outExp = Expression::retype(outExp)?;
            }
            (outExp, outChanged)
        },
        _ => {
            (outExp, outChanged) = Expression::mapFoldShallow(exp, (std::sync::Arc::new({ let __pe_b1 = info.clone(); move |__pe_a0, __pe_a2| evaluateExpTraverser(__pe_a0, __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, bool) -> Result<(Arc<Expression::NFExpression>, bool)> + 'static>), false)?;
            ty = Expression::typeOf(outExp.clone());
            ty2 = evaluateType(ty.clone(), info)?;
            (if (referenceEq(&*(ty),&*(ty2.clone()))) {outExp} else {Expression::setType(ty2, outExp)?}, outChanged)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outChanged = changed || outChanged;
    Ok((outExp, outChanged))
}

pub(crate) fn evaluateType(mut ty: Arc<Type::NFType>, mut info: SourceInfo) -> Result<Arc<Type::NFType>> {
    let mut ty: Arc<Type::NFType> = ty;
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ Type::ARRAY { .. } => {
            assign_variant_field!(ty => Type::NFType::ARRAY; dimensions = ({
        let mut __acc: Arc<metamodelica::List<Arc<Dimension::NFDimension>>> = metamodelica::nil();
        for mut d in (var_field!((*ty).dimensions, Type::NFType::ARRAY).clone()).into_iter().cloned() {
            let __x = evaluateDimension(d.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            ty
        },
        Deref @ Type::CONDITIONAL_ARRAY { .. } => Type::simplifyConditionalArray(ty),
        _ => ty,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

pub(crate) fn evaluateDimension(mut dim: Arc<Dimension::NFDimension>, mut info: SourceInfo) -> Result<Arc<Dimension::NFDimension>> {
    let mut outDim: Arc<Dimension::NFDimension>;
    outDim = (::match_deref::match_deref! { match &(dim.clone()) {
        Deref @ Dimension::EXP { .. } => {
            let mut e: Arc<Expression::NFExpression>;
            e = evaluateExp(var_field!((*dim).exp, Dimension::NFDimension::EXP).clone(), info)?;
            if (referenceEq(&*(e.clone()),&*(var_field!((*dim).exp, Dimension::NFDimension::EXP).clone()))) {dim} else {Dimension::fromExp(e.clone(), var_field!((*dim).var, Dimension::NFDimension::EXP).clone())?}
        },
        _ => {
            dim
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDim)
}

pub(crate) fn evaluateIfExp(mut exp: Arc<Expression::NFExpression>, mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outChanged: bool;
    let mut ty: Arc<Type::NFType>;
    let mut cond: Arc<Expression::NFExpression>;
    let mut tb: Arc<Expression::NFExpression>;
    let mut fb: Arc<Expression::NFExpression>;
    let mut c1: bool = false;
    let mut c2: bool = false;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(exp) {
        Deref @ Expression::IF { ty: __pa0, condition: __pa1, trueBranch: __pa2, falseBranch: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    cond = __pa1.clone();
    tb = __pa2.clone();
    fb = __pa3.clone();
    (cond, outChanged) = evaluateExpTraverser(cond, info.clone(), false)?;
    cond = SimplifyExp::simplify(cond, false)?;
    (outExp, outChanged) = (::match_deref::match_deref! { match &(cond.clone()) {
        Deref @ Expression::BOOLEAN { .. } => {
            (outExp, _) = evaluateExpTraverser(if (var_field!((*cond).value, Expression::NFExpression::BOOLEAN).clone()) {tb} else {fb}, info, false)?;
            (outExp, true)
        },
        _ => {
            (tb, c1) = evaluateExpTraverser(tb, info.clone(), false)?;
            (fb, c2) = evaluateExpTraverser(fb, info, false)?;
            (Arc::new(Expression::NFExpression::IF { ty: ty, condition: cond, trueBranch: tb, falseBranch: fb }), outChanged || c1 || c2)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outChanged))
}

pub(crate) fn evaluateEquations(mut eql: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<Arc<metamodelica::List<Arc<Equation::NFEquation>>>> {
    let mut outEql: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = ({
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut e in (eql.clone()).into_iter().cloned() {
            let __x = evaluateEquation(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outEql)
}

pub(crate) fn evaluateEquation(mut eq: Arc<Equation::NFEquation>) -> Result<Arc<Equation::NFEquation>> {
    let mut eq: Arc<Equation::NFEquation> = eq;
    let mut info: SourceInfo = Equation::info(eq.clone())?;
    eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            let mut ty: Arc<Type::NFType>;
            ty = Type::mapDims(var_field!((*eq).ty, Equation::NFEquation::EQUALITY).clone(), (std::sync::Arc::new({ let __pe_b1 = info.clone(); move |__pe_a0| evaluateDimension(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<Arc<Dimension::NFDimension>> + 'static>))?;
            e1 = evaluateExp(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone(), info.clone())?;
            e2 = evaluateExp(var_field!((*eq).rhs, Equation::NFEquation::EQUALITY).clone(), info)?;
            Arc::new(Equation::NFEquation::EQUALITY { lhs: e1.clone(), rhs: e2.clone(), ty: ty.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::EQUALITY).clone(), source: var_field!((*eq).source, Equation::NFEquation::EQUALITY).clone(), scalarizeMode: var_field!((*eq).scalarizeMode, Equation::NFEquation::EQUALITY).clone() })
        },
        Deref @ Equation::FOR { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::FOR;
                range = Util::applyOption(var_field!((*eq).range, Equation::NFEquation::FOR).clone(), (std::sync::Arc::new({ let __pe_b1 = info; move |__pe_a0| evaluateExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
                body = evaluateEquations(var_field!((*eq).body, Equation::NFEquation::FOR).clone())?
            );
            eq
        },
        Deref @ Equation::IF { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::IF; branches = ({
        let mut __acc: Arc<metamodelica::List<Arc<Branch::Branch>>> = metamodelica::nil();
        for mut b in (var_field!((*eq).branches, Equation::NFEquation::IF).clone()).into_iter().cloned() {
            let __x = evaluateEqBranch(b.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            eq
        },
        Deref @ Equation::WHEN { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::WHEN; branches = ({
        let mut __acc: Arc<metamodelica::List<Arc<Branch::Branch>>> = metamodelica::nil();
        for mut b in (var_field!((*eq).branches, Equation::NFEquation::WHEN).clone()).into_iter().cloned() {
            let __x = evaluateEqBranch(b.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            eq
        },
        Deref @ Equation::ASSERT { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            let mut e3: Arc<Expression::NFExpression>;
            e1 = evaluateExp(var_field!((*eq).condition, Equation::NFEquation::ASSERT).clone(), info.clone())?;
            e2 = evaluateExp(var_field!((*eq).message, Equation::NFEquation::ASSERT).clone(), info.clone())?;
            e3 = evaluateExp(var_field!((*eq).level, Equation::NFEquation::ASSERT).clone(), info)?;
            Arc::new(Equation::NFEquation::ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), scope: var_field!((*eq).scope, Equation::NFEquation::ASSERT).clone(), source: var_field!((*eq).source, Equation::NFEquation::ASSERT).clone() })
        },
        Deref @ Equation::TERMINATE { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::TERMINATE; message = evaluateExp(var_field!((*eq).message, Equation::NFEquation::TERMINATE).clone(), info)?);
            eq
        },
        Deref @ Equation::REINIT { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::REINIT; reinitExp = evaluateExp(var_field!((*eq).reinitExp, Equation::NFEquation::REINIT).clone(), info)?);
            eq
        },
        Deref @ Equation::NORETCALL { .. } => {
            assign_variant_field!(eq => Equation::NFEquation::NORETCALL; exp = evaluateExp(var_field!((*eq).exp, Equation::NFEquation::NORETCALL).clone(), info)?);
            eq
        },
        _ => {
            eq
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eq)
}

pub(crate) fn evaluateEqBranch(mut branch: Arc<Branch::Branch>, mut info: SourceInfo) -> Result<Arc<Branch::Branch>> {
    let mut outBranch: Arc<Branch::Branch>;
    outBranch = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Equation::Branch::BRANCH { condition, body, .. } => {
            let mut condition = (*condition).clone();
            let mut body = (*body).clone();
            condition = evaluateExp(condition.clone(), info)?;
            body = evaluateEquations(body.clone())?;
            Arc::new(Branch::Branch::BRANCH { condition: condition.clone(), conditionVar: var_field!((*branch).conditionVar, Branch::Branch::BRANCH).clone(), body: body.clone() })
        },
        _ => {
            branch
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBranch)
}

pub(crate) fn evaluateAlgorithms(mut algs: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>) -> Result<Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>>> {
    let mut outAlgs: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = ({
        let mut __acc: Arc<metamodelica::List<Arc<Algorithm::NFAlgorithm>>> = metamodelica::nil();
        for mut a in (algs.clone()).into_iter().cloned() {
            let __x = evaluateAlgorithm(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outAlgs)
}

pub(crate) fn evaluateAlgorithm(mut alg: Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>> {
    let mut alg: Arc<Algorithm::NFAlgorithm> = alg;
    assign_field!(alg.statements = evaluateStatements(alg.statements.clone())?);
    Ok(alg)
}

pub(crate) fn evaluateStatements(mut stmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>>) -> Result<Arc<metamodelica::List<Arc<Statement::NFStatement>>>> {
    let mut outStmts: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = ({
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut s in (stmts.clone()).into_iter().cloned() {
            let __x = evaluateStatement(s.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outStmts)
}

pub(crate) fn evaluateStatement(mut stmt: Arc<Statement::NFStatement>) -> Result<Arc<Statement::NFStatement>> {
    let mut stmt: Arc<Statement::NFStatement> = stmt;
    let mut info: SourceInfo = Statement::info(stmt.clone())?;
    stmt = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            let mut ty: Arc<Type::NFType>;
            ty = Type::mapDims(var_field!((*stmt).ty, Statement::NFStatement::ASSIGNMENT).clone(), (std::sync::Arc::new({ let __pe_b1 = info.clone(); move |__pe_a0| evaluateDimension(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Dimension::NFDimension>) -> Result<Arc<Dimension::NFDimension>> + 'static>))?;
            e1 = evaluateExp(var_field!((*stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone(), info.clone())?;
            e2 = evaluateExp(var_field!((*stmt).rhs, Statement::NFStatement::ASSIGNMENT).clone(), info)?;
            Arc::new(Statement::NFStatement::ASSIGNMENT { lhs: e1.clone(), rhs: e2.clone(), ty: ty.clone(), source: var_field!((*stmt).source, Statement::NFStatement::ASSIGNMENT).clone() })
        },
        Deref @ Statement::FOR { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::FOR;
                range = Util::applyOption(var_field!((*stmt).range, Statement::NFStatement::FOR).clone(), (std::sync::Arc::new({ let __pe_b1 = info; move |__pe_a0| evaluateExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?,
                body = evaluateStatements(var_field!((*stmt).body, Statement::NFStatement::FOR).clone())?
            );
            stmt
        },
        Deref @ Statement::IF { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::IF; branches = ({
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, Statement::NFStatement::IF).clone()).into_iter().cloned() {
            let __x = evaluateStmtBranch(b.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            stmt
        },
        Deref @ Statement::WHEN { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::WHEN; branches = ({
        let mut __acc: Arc<metamodelica::List<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*stmt).branches, Statement::NFStatement::WHEN).clone()).into_iter().cloned() {
            let __x = evaluateStmtBranch(b.clone(), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            stmt
        },
        Deref @ Statement::ASSERT { .. } => {
            let mut e1: Arc<Expression::NFExpression>;
            let mut e2: Arc<Expression::NFExpression>;
            let mut e3: Arc<Expression::NFExpression>;
            e1 = evaluateExp(var_field!((*stmt).condition, Statement::NFStatement::ASSERT).clone(), info.clone())?;
            e2 = evaluateExp(var_field!((*stmt).message, Statement::NFStatement::ASSERT).clone(), info.clone())?;
            e3 = evaluateExp(var_field!((*stmt).level, Statement::NFStatement::ASSERT).clone(), info)?;
            Arc::new(Statement::NFStatement::ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), source: var_field!((*stmt).source, Statement::NFStatement::ASSERT).clone() })
        },
        Deref @ Statement::TERMINATE { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::TERMINATE; message = evaluateExp(var_field!((*stmt).message, Statement::NFStatement::TERMINATE).clone(), info)?);
            stmt
        },
        Deref @ Statement::REINIT { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::REINIT; reinitExp = evaluateExp(var_field!((*stmt).reinitExp, Statement::NFStatement::REINIT).clone(), info)?);
            stmt
        },
        Deref @ Statement::NORETCALL { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::NORETCALL; exp = evaluateExp(var_field!((*stmt).exp, Statement::NFStatement::NORETCALL).clone(), info)?);
            stmt
        },
        Deref @ Statement::WHILE { .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::WHILE;
                condition = evaluateExp(var_field!((*stmt).condition, Statement::NFStatement::WHILE).clone(), info)?,
                body = evaluateStatements(var_field!((*stmt).body, Statement::NFStatement::WHILE).clone())?
            );
            stmt
        },
        _ => {
            stmt
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stmt)
}

pub(crate) fn evaluateStmtBranch(mut branch: (Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>), mut info: SourceInfo) -> Result<(Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>)> {
    let mut outBranch: (Arc<Expression::NFExpression>, Arc<metamodelica::List<Arc<Statement::NFStatement>>>);
    let mut cond: Arc<Expression::NFExpression>;
    let mut body: Arc<metamodelica::List<Arc<Statement::NFStatement>>>;
    (cond, body) = branch;
    cond = evaluateExp(cond, info)?;
    body = evaluateStatements(body)?;
    outBranch = (cond, body);
    Ok(outBranch)
}

pub(crate) fn evaluateFunction(mut func: Arc<Function::Function>) -> Result<Arc<Function::Function>> {
    let mut func: Arc<Function::Function> = func;
    let mut is_con: bool;
    if !(Function::isEvaluated(func.clone())) {
        Function::markEvaluated(func.clone());
        is_con = Function::isDefaultRecordConstructor(func.clone());
        func = Function::mapExp(func.clone(), (std::sync::Arc::new({ let __pe_b1 = func.node.clone(); let __pe_b2 = is_con; move |__pe_a0| evaluateFuncExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), (std::sync::Arc::new({ let __pe_b1 = func.node.clone(); let __pe_b2 = true; move |__pe_a0| evaluateFuncExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), true, true)?;
        if is_con {
            Record::checkLocalFieldOrder(func.locals.clone(), func.node.clone(), InstNode::info(func.node.clone()))?;
        }
        for mut fn_der in &*func.derivatives.clone() {
            let mut fn_der = fn_der.clone();
            for mut der_fn in &*Function::getCachedFuncs(fn_der.derivativeFn.clone())? {
                let mut der_fn = der_fn.clone();
                evaluateFunction(der_fn.clone())?;
            }
        }
    }
    Ok(func)
}

pub(crate) fn evaluateFuncExp(mut exp: Arc<Expression::NFExpression>, mut fnNode: Arc<InstNode::InstNode>, mut evaluateAll: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression>;
    (outExp, _) = evaluateFuncExpTraverser(exp, fnNode, evaluateAll, false)?;
    Ok(outExp)
}

pub(crate) fn evaluateFuncExpTraverser(mut exp: Arc<Expression::NFExpression>, mut fnNode: Arc<InstNode::InstNode>, mut evaluateAll: bool, mut changed: bool) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut outChanged: bool;
    let mut e: Arc<Expression::NFExpression>;
    (e, outChanged) = Expression::mapFoldShallow(exp, (std::sync::Arc::new({ let __pe_b1 = fnNode.clone(); let __pe_b2 = evaluateAll; move |__pe_a0, __pe_a3| evaluateFuncExpTraverser(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, bool) -> Result<(Arc<Expression::NFExpression>, bool)> + 'static>), false)?;
    outExp = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Expression::CREF { .. } => {
            if evaluateAll || !(isLocalFunctionVariable(var_field!((*e).cref, Expression::NFExpression::CREF).clone(), fnNode)?) {
                ErrorExt::setCheckpoint(literal!("NFEvalConstants.evaluateFuncExpTraverser"));
                match '__try0: {
                    outExp = unwrap_break_err!(Ceval::evalCref(var_field!((*e).cref, Expression::NFExpression::CREF).clone(), e.clone(), Ceval::noTarget().clone(), false, true), '__try0);
                    Ok::<_, anyhow::Error>((outExp.clone(),))
                } {
                    Ok((__try0_o0,)) => {
                        outExp = __try0_o0;
                    }
                    Err(_) => {
                        outExp = e.clone();
                    }
                }
                ErrorExt::rollBack(literal!("NFEvalConstants.evaluateFuncExpTraverser"));
                outChanged = true;
            } else if outChanged {
                outExp = Arc::new(Expression::NFExpression::CREF { ty: ComponentRef::getSubscriptedType(var_field!((*e).cref, Expression::NFExpression::CREF).clone(), false)?, cref: var_field!((*e).cref, Expression::NFExpression::CREF).clone() });
            } else {
                outExp = e;
            }
            outExp
        },
        _ => if (outChanged) {Expression::retype(e)?} else {e},
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outChanged = changed || outChanged;
    Ok((outExp, outChanged))
}

pub(crate) fn isLocalFunctionVariable(mut cref: Arc<ComponentRef::NFComponentRef>, mut fnNode: Arc<InstNode::InstNode>) -> Result<bool> {
    let mut res: bool;
    let mut node: Arc<InstNode::InstNode>;
    let mut fnl: Arc<metamodelica::List<Arc<Function::Function>>>;
    let mut r#fn: Arc<Function::Function>;
    if ComponentRef::isPackageConstant(cref.clone())? {
        res = false;
    } else if ComponentRef::nodeVariability(cref.clone())? <= Variability::PARAMETER.clone() && ComponentRef::isCref(cref.clone()) {
        node = InstNode::instanceParent(ComponentRef::node(ComponentRef::last(cref))?)?;
        if InstNode::isClass(node.clone())? {
            fnl = Function::getCachedFuncs(node)?;
            if fnl.clone().is_empty() {
                res = false;
            } else {
                r#fn = listHead(fnl)?;
                res = InstNode::refEqual(fnNode, r#fn.node.clone());
            }
        } else {
            res = false;
        }
    } else {
        res = true;
    }
    Ok(res)
}

pub(crate) fn evaluateRecordDeclaration(mut recordNode: Arc<InstNode::InstNode>) -> Result<()> {
    ClassTree::applyComponents(Class::classTree(InstNode::getClass(recordNode.clone())?)?, (std::sync::Arc::new({ let __pe_b1 = recordNode; move |__pe_a0| evaluateRecordDeclarationField(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<()> + 'static>))?;
    Ok(())
}

pub(crate) fn evaluateRecordDeclarationField(mut fieldNode: Arc<InstNode::InstNode>, mut recordNode: Arc<InstNode::InstNode>) -> Result<()> {
    let mut comp: Arc<Component::NFComponent>;
    let mut binding: Arc<Binding::NFBinding>;
    let mut cls_inst: Arc<InstNode::InstNode>;
    comp = InstNode::component(fieldNode.clone())?;
    binding = Component::getBinding(comp.clone());
    if Binding::isBound(binding.clone()) {
        binding = Binding::mapExp(binding, (std::sync::Arc::new({ let __pe_b1 = fieldNode.clone(); let __pe_b2 = false; move |__pe_a0| evaluateFuncExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
        comp = Component::setBinding(binding, comp)?;
    }
    cls_inst = Component::classInstance(comp.clone());
    if !(InstNode::isEmpty(cls_inst.clone())) {
        ClassTree::applyComponents(Class::classTree(InstNode::getClass(cls_inst)?)?, (std::sync::Arc::new({ let __pe_b1 = recordNode; move |__pe_a0| evaluateRecordDeclarationField(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<()> + 'static>))?;
    }
    InstNode::updateComponent(comp, fieldNode)?;
    Ok(())
}

