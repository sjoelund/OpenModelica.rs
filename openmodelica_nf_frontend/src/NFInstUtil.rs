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
use crate::NFAlgorithm as Algorithm;
use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten::FunctionTree;
use crate::NFFlatten;
use crate::NFFunction::Function;
use crate::NFInstContext;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn dumpFlatModelDebug(mut stage: ArcStr, mut flatModel: Arc<FlatModel::NFFlatModel>, mut functions: Arc<NFFlatten::FunctionTreeImpl::Tree>) -> Result<()> {
    let mut flat_model: Arc<FlatModel::NFFlatModel> = flatModel.clone();
    if Flags::getConfigString(Flags::OBFUSCATE.clone())? == literal!("protected") || Flags::getConfigString(Flags::OBFUSCATE.clone())? == literal!("encrypted") {
        flat_model = FlatModel::obfuscate(flat_model)?;
    }
    if Flags::isConfigFlagSet(Flags::DUMP_FLAT_MODEL.clone(), (stage.clone()).clone())? || Flags::getConfigStringList(Flags::DUMP_FLAT_MODEL.clone())?.is_empty() {
        flat_model = combineSubscripts(flatModel)?;
        metamodelica::print((literal!("########################################\n")).clone());
        metamodelica::print((stage).clone());
        metamodelica::print((literal!("\n########################################\n\n")).clone());
        if Flags::getConfigBool(Flags::BASE_MODELICA.clone())? {
            FlatModel::printFlatString(flat_model, functions, false)?;
        } else {
            FlatModel::printString(flat_model, functions, false)?;
        }
        metamodelica::print((literal!("\n")).clone());
    }
    Ok(())
}

pub(crate) fn combineSubscripts(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    if Flags::isSet(Flags::COMBINE_SUBSCRIPTS.clone())? {
        flatModel = FlatModel::mapExp(flatModel, (std::sync::Arc::new(combineSubscriptsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    }
    Ok(flatModel)
}

pub(crate) fn combineSubscriptsExp(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    fn traverser(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            assign_variant_field!(exp => Expression::NFExpression::CREF; cref = ComponentRef::combineSubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(exp)
    }

    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::map(exp, (std::sync::Arc::new(traverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(exp)
}

pub(crate) fn printStructuralParameters(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<()> {
    let mut params: Arc<metamodelica::List<Arc<Variable::NFVariable>>>;
    let mut names: Arc<metamodelica::List<ArcStr>>;
    if Flags::isSet(Flags::PRINT_STRUCTURAL.clone())? {
        params = ({
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            if !(Variable::isStructural(v.clone())) { continue; }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        if !(params.clone().is_empty()) {
            names = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (params).into_iter().cloned() {
            let __x = ComponentRef::toString(v.name.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            Error::addMessage(Error::NOTIFY_FRONTEND_STRUCTURAL_PARAMETERS.clone(), list![stringDelimitList(names, (literal!(", ")).clone())])?;
        }
    }
    Ok(())
}

pub(crate) fn dumpFlatModel(mut flatModel: Arc<FlatModel::NFFlatModel>, mut functions: Arc<NFFlatten::FunctionTreeImpl::Tree>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut flat_model: Arc<FlatModel::NFFlatModel>;
    flat_model = combineSubscripts(flatModel)?;
    r#str = (FlatModel::toFlatString(flat_model, functions, false)?).clone();
    Ok(r#str)
}

pub(crate) fn replaceEmptyArrays(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<Arc<FlatModel::NFFlatModel>> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    flatModel = FlatModel::mapExp(flatModel, (std::sync::Arc::new(replaceEmptyArraysExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(flatModel)
}

pub(crate) fn replaceEmptyArraysExp(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    fn traverser(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
        let mut outExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
        let mut cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
        let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
        outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref, .. } if (ComponentRef::isEmptyArray(cref.clone())?) => {
            let mut cref = (*cref).clone();
            if ComponentRef::hasSubscripts(cref.clone())? {
                cref = ComponentRef::fillSubscripts(cref.clone());
                cref = ComponentRef::replaceWholeSubscripts(cref.clone())?;
                subs = ComponentRef::subscriptsAllFlat(cref.clone())?;
                cref = ComponentRef::stripSubscriptsAll(cref.clone());
                ty = ComponentRef::getSubscriptedType(cref.clone(), false)?;
            } else {
                subs = metamodelica::nil();
                ty = var_field!((*exp).ty, Expression::NFExpression::CREF).clone();
            }
            outExp = Expression::makeDefaultValue(ty, None, None)?;
            if !(subs.clone().is_empty()) {
                outExp = Arc::new(Expression::NFExpression::SUBSCRIPTED_EXP { exp: outExp, subscripts: subs, ty: var_field!((*exp).ty, Expression::NFExpression::CREF).clone(), split: false });
            }
            outExp
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(outExp)
    }

    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::map(exp, (std::sync::Arc::new(traverser) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(exp)
}

pub(crate) fn expandSlicedCrefs(mut flatModel: Arc<FlatModel::NFFlatModel>, mut functions: Arc<NFFlatten::FunctionTreeImpl::Tree>) -> Result<(Arc<FlatModel::NFFlatModel>, Arc<NFFlatten::FunctionTreeImpl::Tree>)> {
    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut functions: Arc<NFFlatten::FunctionTreeImpl::Tree> = functions;
    if Flags::isSet(Flags::COMBINE_SUBSCRIPTS.clone())? || !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) {
        return Ok((flatModel.clone(), functions.clone()));
    }
    assign_field!(flatModel.variables = ({
        let mut __acc: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            let __x = Variable::mapExpShallow(v.clone(), (std::sync::Arc::new(expandSlicedCrefsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
    flatModel = FlatModel::mapEquations(flatModel, (std::sync::Arc::new(expandSlicedCrefsEq) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<Arc<Equation::NFEquation>> + 'static>))?;
    flatModel = FlatModel::mapAlgorithms(flatModel, (std::sync::Arc::new(expandSlicedCrefsAlg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>> + 'static>))?;
    functions = NFFlatten::FunctionTreeImpl::map(functions, (std::sync::Arc::new(expandSlicedCrefsFunction) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Function::Function>) -> Result<Arc<Function::Function>> + 'static>))?;
    Ok((flatModel, functions))
}

pub(crate) fn addTrailingWholeIndices(mut exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (ComponentRef::hasImplicitTrailingIndex(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())) => {
            assign_variant_field!(exp => Expression::NFExpression::CREF; cref = ComponentRef::fillSubscripts(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()));
            exp.clone()
        },
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub(crate) fn expandSlicedCrefsExp(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (ComponentRef::isSliced(var_field!((*exp).cref, Expression::NFExpression::CREF).clone())?) => expandSlicedCrefsExp2(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), var_field!((*exp).ty, Expression::NFExpression::CREF).clone())?,
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn expandSlicedCrefsExp2(mut cref: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>) -> Result<Arc<Expression::NFExpression>> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut cr: Arc<ComponentRef::NFComponentRef>;
    let mut iterators: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>;
    (cr, iterators) = ComponentRef::iterate(cref.clone())?;
    outExp = Arc::new(Expression::NFExpression::CALL { call: Arc::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR { ty: ty, var: ComponentRef::variability(cref.clone())?, purity: ComponentRef::purity(cref)?, exp: Expression::fromCref(cr, false)?, iters: iterators }) });
    Ok(outExp)
}

pub(crate) fn expandSlicedCrefsEq(mut eq: Arc<Equation::NFEquation>) -> Result<Arc<Equation::NFEquation>> {
    let mut eq: Arc<Equation::NFEquation> = eq;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut eq2: Arc<Equation::NFEquation> = Arc::new(<Equation::NFEquation as ::std::default::Default>::default());
    eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { rhs: __esc_e1, .. } => {
            e1 = (*__esc_e1).clone();
            e1 = Expression::map(e1.clone(), (std::sync::Arc::new(fnptr!(addTrailingWholeIndices, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            e2 = Expression::map(e1.clone(), (std::sync::Arc::new(expandSlicedCrefsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            if !(referenceEq(&*(e1.clone()),&*(e2.clone()))) {
                assign_variant_field!(eq => Equation::NFEquation::EQUALITY; rhs = e2);
            }
            eq
        },
        _ => {
            eq2 = Equation::mapExpShallow(eq, (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = (std::sync::Arc::new(fnptr!(addTrailingWholeIndices, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            Equation::mapExpShallow(eq2, (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = (std::sync::Arc::new(expandSlicedCrefsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eq)
}

pub(crate) fn expandSlicedCrefsAlg(mut alg: Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>> {
    let mut alg: Arc<Algorithm::NFAlgorithm> = alg;
    assign_field!(alg.statements = ({
        let mut __acc: Arc<metamodelica::List<Arc<Statement::NFStatement>>> = metamodelica::nil();
        for mut s in (alg.statements.clone()).into_iter().cloned() {
            let __x = Statement::map(s.clone(), (std::sync::Arc::new(expandSlicedCrefsStmt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Statement::NFStatement>) -> Result<Arc<Statement::NFStatement>> + 'static>))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
    Ok(alg)
}

pub(crate) fn expandSlicedCrefsStmt(mut stmt: Arc<Statement::NFStatement>) -> Result<Arc<Statement::NFStatement>> {
    let mut stmt: Arc<Statement::NFStatement> = stmt;
    let mut e1: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut e2: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    stmt = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { rhs: __esc_e1, .. } => {
            e1 = (*__esc_e1).clone();
            assign_variant_field!(stmt => Statement::NFStatement::ASSIGNMENT; lhs = Expression::map(var_field!((*stmt).lhs, Statement::NFStatement::ASSIGNMENT).clone(), (std::sync::Arc::new(fnptr!(addTrailingWholeIndices, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?);
            e1 = Expression::map(e1.clone(), (std::sync::Arc::new(fnptr!(addTrailingWholeIndices, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            e2 = Expression::map(e1.clone(), (std::sync::Arc::new(expandSlicedCrefsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            if !(referenceEq(&*(e1.clone()),&*(e2.clone()))) {
                assign_variant_field!(stmt => Statement::NFStatement::ASSIGNMENT; rhs = e2);
            }
            stmt
        },
        _ => {
            let mut stmt2: Arc<Statement::NFStatement>;
            stmt2 = Statement::mapExpShallow(stmt, (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = (std::sync::Arc::new(fnptr!(addTrailingWholeIndices, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            Statement::mapExpShallow(stmt2, (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = (std::sync::Arc::new(expandSlicedCrefsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stmt)
}

pub(crate) fn expandSlicedCrefsFunction(mut fnPath: Arc<Absyn::Path>, mut r#fn: Arc<Function::Function>) -> Result<Arc<Function::Function>> {
    let mut r#fn: Arc<Function::Function> = r#fn;
    r#fn = Function::mapExp(r#fn, (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = (std::sync::Arc::new(expandSlicedCrefsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static> = (std::sync::Arc::new(expandSlicedCrefsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), true, false)?;
    r#fn = Function::mapBody(r#fn, (std::sync::Arc::new(expandSlicedCrefsAlg) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Algorithm::NFAlgorithm>) -> Result<Arc<Algorithm::NFAlgorithm>> + 'static>))?;
    Ok(r#fn)
}

pub type MergeNameMap = Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<Absyn::ComponentRef>>>;

pub fn makeMergeNameMap() -> MergeNameMap {
    let mut nameMap: MergeNameMap = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1);
    nameMap
}

pub(crate) fn mergeScalars(mut node: Arc<InstNode::InstNode>, mut classPath: Arc<Absyn::Path>, mut isRootClass: bool, mut nameMap: MergeNameMap) -> Result<Arc<InstNode::InstNode>> {
    let mut node: Arc<InstNode::InstNode> = node;
    let mut elem: Arc<SCode::Element>;
    if !(Flags::isSet(Flags::MERGE_COMPONENTS.clone())?) {
        return Ok(node.clone());
    }
    elem = InstNode::definition(node.clone())?;
    elem = mergeScalars2(elem, classPath, isRootClass, nameMap)?;
    node = InstNode::setDefinition(elem, node)?;
    execStat(literal!("NFInstUtil.mergeScalars"))?;
    Ok(node)
}

pub(crate) fn mergeScalars2(mut cls: Arc<SCode::Element>, mut classPath: Arc<Absyn::Path>, mut isRootClass: bool, mut nameMap: MergeNameMap) -> Result<Arc<SCode::Element>> {
    let mut cls: Arc<SCode::Element> = cls;
    let mut cdef: Arc<SCode::ClassDef> = Arc::new(<SCode::ClassDef as ::std::default::Default>::default());
    let mut elems: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ SCode::Element::CLASS { classDef: __esc_cdef @ Deref @ SCode::ClassDef::PARTS { .. }, .. } => {
            cdef = (*__esc_cdef).clone();
            elems = mergeScalars3(var_field!((*cdef).elementLst, SCode::ClassDef::PARTS).clone(), nameMap.clone())?;
            elems = ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (elems).into_iter().cloned() {
            let __x = mergeScalarsElement(e.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            assign_variant_field!(cdef => SCode::ClassDef::PARTS;
                elementLst = elems,
                normalEquationLst = mergeScalarsEql(var_field!((*cdef).normalEquationLst, SCode::ClassDef::PARTS).clone(), nameMap.clone())?,
                initialEquationLst = mergeScalarsEql(var_field!((*cdef).initialEquationLst, SCode::ClassDef::PARTS).clone(), nameMap.clone())?,
                normalAlgorithmLst = mergeScalarsAlgs(var_field!((*cdef).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), nameMap.clone())?,
                initialAlgorithmLst = mergeScalarsAlgs(var_field!((*cdef).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), nameMap.clone())?
            );
            assign_variant_field!(cls => SCode::Element::CLASS; classDef = cdef.clone());
            if isRootClass {
                System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(classPath, (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("_merged_table.json")); ArcStr::from(__mm_s) }).clone(), (UnorderedMap::toJSON(nameMap, std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(Dump::printComponentRefStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>) -> Result<ArcStr> + 'static>))?).clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cls)
}

pub(crate) fn mergeScalars3(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut nameMap: MergeNameMap) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut outElements: Arc<metamodelica::List<Arc<SCode::Element>>>;
    let mut mergeable: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Element>>>>>;
    let mut merged_e: Arc<SCode::Element>;
    let mut i: i32 = UnorderedMap::size(nameMap.clone()) + 1;
    let mut prefix: ArcStr;
    (mergeable, outElements) = makeMergeMap(elements)?;
    for mut el in &*mergeable {
        let mut el = el.clone();
        prefix = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*AbsynUtil::pathLastIdent(SCodeUtil::getElementTypePath(listHead(el.clone())?)?)?); ArcStr::from(__mm_s) }).clone();
        merged_e = mergeComponents(el.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", i))); ArcStr::from(__mm_s) }).clone(), nameMap.clone())?;
        i = i + 1;
        outElements = metamodelica::cons(merged_e.clone(), outElements.clone());
    }
    outElements = metamodelica::Dangerous::listReverseInPlace(outElements);
    Ok(outElements)
}

pub(crate) fn makeMergeMap(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Element>>>>>, Arc<metamodelica::List<Arc<SCode::Element>>>)> {
    pub(crate) type ElementList = Arc<metamodelica::List<Arc<SCode::Element>>>;

    fn append_merge(mut oldValue: Option<Arc<metamodelica::List<Arc<SCode::Element>>>>, mut elem: Arc<SCode::Element>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
        let mut newValue: Arc<metamodelica::List<Arc<SCode::Element>>>;
        if isSome(oldValue.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(oldValue) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            newValue = __pa0.clone();
        } else {
            newValue = metamodelica::nil();
        }
        newValue = metamodelica::cons(elem, newValue);
        Ok(newValue)
    }

    let mut mergeable: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Element>>>>> = metamodelica::nil();
    let mut unmergeable: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut merge_map: Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<metamodelica::List<Arc<SCode::Element>>>>>;
    let mut grouped_elems: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Element>>>>>;
    merge_map = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1);
    for mut e in &*elements.clone() {
        let mut e = e.clone();
        let () = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ SCode::Element::COMPONENT { .. } if (isMergeableComponent(e.clone())) => {
            UnorderedMap::addUpdate((getComponentSignature(e.clone())?).clone(), (std::sync::Arc::new({ let __pe_b1 = e.clone(); move |__pe_a0| append_merge(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<metamodelica::List<Arc<SCode::Element>>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> + 'static>), merge_map.clone())?;
            ()
        },
        _ => {
            unmergeable = metamodelica::cons(e.clone(), unmergeable.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    grouped_elems = UnorderedMap::valueList(merge_map.clone());
    for mut el in &*grouped_elems.clone() {
        let mut el = el.clone();
        if (el.clone().len() as i32) == 1 {
            unmergeable = metamodelica::cons(listHead(el.clone())?, unmergeable.clone());
        } else {
            mergeable = metamodelica::cons(metamodelica::Dangerous::listReverseInPlace(el.clone()), mergeable.clone());
        }
    }
    Ok((mergeable, unmergeable))
}

pub(crate) fn isMergeableComponent(mut element: Arc<SCode::Element>) -> bool {
    let mut isMergeable: bool;
    isMergeable = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { arrayDims: Deref @ metamodelica::List::Nil, .. }, prefixes: Deref @ SCode::Prefixes { redeclarePrefix: SCode::Redeclare::NOT_REDECLARE { .. }, innerOuter: Absyn::InnerOuter::NOT_INNER_OUTER { .. }, replaceablePrefix: Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. }, .. }, condition: None, .. } => isMergeableType(var_field!((*element).typeSpec, SCode::Element::COMPONENT).clone()) && isMergeableMod(var_field!((*element).modifications, SCode::Element::COMPONENT).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isMergeable
}

pub(crate) fn isMergeableMod(mut r#mod: Arc<SCode::Mod>) -> bool {
    let mut mergeable: bool = false;
    mergeable = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { eachPrefix: SCode::Each::NOT_EACH { .. }, .. } => {
            for mut m in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                let mut m = m.clone();
                if !(isMergeableMod(m.r#mod.clone())) {
                    mergeable = false;
                    return mergeable.clone();
                }
            }
            true
        },
        Deref @ SCode::Mod::NOMOD { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    mergeable
}

pub(crate) fn isMergeableType(mut ty: Arc<Absyn::TypeSpec>) -> bool {
    let mut mergeable: bool;
    mergeable = (::match_deref::match_deref! { match &(ty) {
        Deref @ Absyn::TypeSpec::TPATH { arrayDim: None, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    mergeable
}

pub(crate) fn getComponentSignature(mut element: Arc<SCode::Element>) -> Result<ArcStr> {
    let mut signature: ArcStr;
    let mut prefs: Arc<SCode::Prefixes>;
    let mut attrs: SCode::Attributes;
    let mut ty: Arc<Absyn::TypeSpec>;
    let mut r#mod: Arc<SCode::Mod>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(element) {
        Deref @ SCode::Element::COMPONENT { prefixes: __pa0, attributes: __pa1, typeSpec: __pa2, modifications: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    prefs = __pa0.clone();
    attrs = __pa1.clone();
    ty = __pa2.clone();
    r#mod = __pa3.clone();
    signature = stringAppendList(list![(SCodeDump::visibilityStr(prefs.visibility.clone())?).clone(), (SCodeDump::finalStr(prefs.finalPrefix.clone())?).clone(), (SCodeDump::connectorTypeStr(attrs.connectorType.clone())?).clone(), (SCodeDump::variabilityString(attrs.variability.clone())?).clone(), (Dump::unparseDirectionSymbolStr(attrs.direction.clone())?).clone(), (Dump::unparseTypeSpec(ty)?).clone(), (getModSignature(r#mod, (literal!("")).clone())?).clone()]);
    Ok(signature)
}

pub(crate) fn getModSignature(mut r#mod: Arc<SCode::Mod>, mut name: ArcStr) -> Result<ArcStr> {
    fn sub_mod_lt(mut m1: Arc<SCode::SubMod>, mut m2: Arc<SCode::SubMod>) -> bool {
        let mut res: bool = m1.ident.clone() < m2.ident.clone();
        res
    }

    let mut signature: ArcStr;
    let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut has_binding: bool = false;
    let mut has_submods: bool = false;
    signature = ((::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            has_binding = isSome(var_field!((*r#mod).binding, SCode::Mod::MOD).clone());
            has_submods = !(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone().is_empty());
            if has_binding.clone() {
                strl = metamodelica::cons((literal!("=")).clone(), strl.clone());
            }
            if has_submods.clone() {
                strl = metamodelica::cons((literal!(")")).clone(), strl.clone());
                for mut m in &*List::sort(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone(), (std::sync::Arc::new(fnptr!(sub_mod_lt, Arc<SCode::SubMod>, Arc<SCode::SubMod>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::SubMod>, Arc<SCode::SubMod>) -> Result<bool> + 'static>))? {
                    let mut m = m.clone();
                    strl = metamodelica::cons((literal!(",")).clone(), strl.clone());
                    strl = metamodelica::cons((getModSignature(m.r#mod.clone(), (m.ident.clone()).clone())?).clone(), strl.clone());
                }
                strl = metamodelica::cons((literal!("(")).clone(), strl.clone());
            }
            if has_binding.clone() || has_submods.clone() {
                strl = metamodelica::cons((name.clone()).clone(), strl.clone());
            }
            if SCodeUtil::finalBool(var_field!((*r#mod).finalPrefix, SCode::Mod::MOD).clone())? {
                strl = metamodelica::cons((literal!("final ")).clone(), strl.clone());
            }
            if SCodeUtil::eachBool(var_field!((*r#mod).eachPrefix, SCode::Mod::MOD).clone())? {
                strl = metamodelica::cons((literal!("each ")).clone(), strl.clone());
            }
            stringAppendList(strl.clone())
        },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(signature)
}

pub(crate) fn mergeComponents(mut components: Arc<metamodelica::List<Arc<SCode::Element>>>, mut prefix: ArcStr, mut nameMap: MergeNameMap) -> Result<Arc<SCode::Element>> {
    let mut mergedComponent: Arc<SCode::Element>;
    let mut ty: Arc<Absyn::TypeSpec>;
    let mut prefs: Arc<SCode::Prefixes>;
    let mut attrs: SCode::Attributes;
    let mut r#mod: Arc<SCode::Mod>;
    let mut i: i32 = 1;
    let mut name: ArcStr;
    let mut cref: Arc<Absyn::ComponentRef>;
    let mut mods: Arc<metamodelica::List<Arc<SCode::Mod>>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(listHead(components.clone())?) {
        Deref @ SCode::Element::COMPONENT { typeSpec: __pa0, prefixes: __pa1, attributes: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa0.clone();
    prefs = __pa1.clone();
    attrs = __pa2.clone();
    attrs.arrayDims = list![AbsynUtil::makeIntegerSubscript((components.clone().len() as i32))];
    mods = ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Mod>>> = metamodelica::nil();
        for mut c in (components.clone()).into_iter().cloned() {
            let __x = SCodeUtil::componentMod(c.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    r#mod = mergeMods(mods)?;
    mergedComponent = Arc::new(SCode::Element::COMPONENT { name: (prefix.clone()).clone(), prefixes: prefs, attributes: attrs, typeSpec: ty, modifications: r#mod, comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() });
    for mut c in &*components {
        let mut c = c.clone();
        let __pa3 = ::match_deref::match_deref! { match &(c.clone()) {
            Deref @ SCode::Element::COMPONENT { name: __pa3, .. } => __pa3.clone(),
            _ => bail!("pattern mismatch"),
        } };
        name = __pa3.clone();
        cref = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (prefix.clone()).clone(), subscripts: list![AbsynUtil::makeIntegerSubscript(i)] });
        i = i + 1;
        UnorderedMap::addUnique((name.clone()).clone(), cref.clone(), nameMap.clone())?;
    }
    Ok(mergedComponent)
}

pub(crate) fn mergeMods(mut mods: Arc<metamodelica::List<Arc<SCode::Mod>>>) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod>;
    let mut names: Arc<metamodelica::List<Arc<Absyn::Path>>>;
    let mut bindings: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
    let mut binding_map: Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Absyn::Exp>>>;
    if mods.clone().is_empty() {
        r#mod = openmodelica_frontend_types::SCode::Mod::interned_NOMOD();
        return Ok(r#mod.clone());
    }
    r#mod = listHead(mods.clone())?;
    names = getModNames(r#mod.clone(), metamodelica::nil(), metamodelica::nil())?;
    bindings = List::fill(metamodelica::nil(), (names.clone().len() as i32));
    for mut m in &*mods.reverse() {
        let mut m = m.clone();
        bindings = getModBindings(m.clone(), names.clone(), bindings.clone())?;
    }
    binding_map = UnorderedMap::fromLists(names, ({
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut b in (bindings).into_iter().cloned() {
            let __x = Arc::new(Absyn::Exp::ARRAY { arrayExp: b.clone() });
            __acc = cons(__x, __acc);
        }
        __acc
    }), (std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>))?;
    r#mod = mergeMods2(r#mod, binding_map, metamodelica::nil())?;
    Ok(r#mod)
}

pub(crate) fn getModNames(mut r#mod: Arc<SCode::Mod>, mut name: Arc<metamodelica::List<ArcStr>>, mut names: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut names: Arc<metamodelica::List<Arc<Absyn::Path>>> = names;
    names = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            if isSome(var_field!((*r#mod).binding, SCode::Mod::MOD).clone()) {
                names = metamodelica::cons(makeModPath(name.clone())?, names);
            }
            for mut m in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                let mut m = m.clone();
                names = getModNames(m.r#mod.clone(), metamodelica::cons((m.ident.clone()).clone(), name.clone()), names.clone())?;
            }
            names
        },
        _ => names,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(names)
}

pub(crate) fn makeModPath(mut name: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path>;
    if name.clone().is_empty() {
        path = Arc::new(Absyn::Path::IDENT { name: (literal!("$")).clone() });
    } else {
        path = AbsynUtil::stringListPathReversed(name)?;
    }
    Ok(path)
}

pub(crate) fn mergeMods2(mut r#mod: Arc<SCode::Mod>, mut bindingMap: Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Absyn::Exp>>>, mut name: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    let mut new_binding: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut submods: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            if isSome(var_field!((*r#mod).binding, SCode::Mod::MOD).clone()) {
                new_binding = UnorderedMap::getOrFail(makeModPath(name.clone())?, bindingMap.clone())?;
                assign_variant_field!(r#mod => SCode::Mod::MOD; binding = Some(new_binding));
            }
            if !(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone().is_empty()) {
                for mut m in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                    let mut m = m.clone();
                    assign_field!(m.r#mod = mergeMods2(m.r#mod.clone(), bindingMap.clone(), metamodelica::cons((m.ident.clone()).clone(), name.clone()))?);
                    submods = metamodelica::cons(m.clone(), submods.clone());
                }
                assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = metamodelica::Dangerous::listReverseInPlace(submods));
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#mod)
}

pub(crate) fn getModBindings(mut r#mod: Arc<SCode::Mod>, mut names: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut bindings: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>> {
    let mut bindings: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = bindings;
    let mut mod_bindings: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    for mut name in &*names {
        let mut name = name.clone();
        mod_bindings = metamodelica::cons(lookupModBinding(name.clone(), r#mod.clone())?, mod_bindings.clone());
    }
    bindings = ({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        let __thr_src0 = mod_bindings;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = bindings;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(e1), Some(e2)) => {
                    let __x = cons(e1.clone(), e2.clone());
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => bail!("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(bindings)
}

pub(crate) fn lookupModBinding(mut name: Arc<Absyn::Path>, mut r#mod: Arc<SCode::Mod>) -> Result<Arc<Absyn::Exp>> {
    let mut binding: Arc<Absyn::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(lookupMod(name, r#mod)?) {
        Deref @ SCode::Mod::MOD { binding: Some(__pa0), .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    binding = __pa0.clone();
    Ok(binding)
}

pub(crate) fn lookupMod(mut name: Arc<Absyn::Path>, mut r#mod: Arc<SCode::Mod>) -> Result<Arc<SCode::Mod>> {
    let mut outMod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    outMod = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => if (var_field!((*name).name, Absyn::Path::IDENT).clone() == literal!("$")) {r#mod} else {SCodeUtil::lookupModInMod((var_field!((*name).name, Absyn::Path::IDENT).clone()).clone(), r#mod)},
        Deref @ Absyn::Path::QUALIFIED { .. } => {
            outMod = SCodeUtil::lookupModInMod((var_field!((*name).name, Absyn::Path::QUALIFIED).clone()).clone(), r#mod);
            lookupMod(var_field!((*name).path, Absyn::Path::QUALIFIED).clone(), outMod)?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

pub(crate) fn mergeScalarsElement(mut element: Arc<SCode::Element>, mut nameMap: MergeNameMap) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::EXTENDS { .. } => {
            assign_variant_field!(element => SCode::Element::EXTENDS; modifications = mergeScalarsMod(var_field!((*element).modifications, SCode::Element::EXTENDS).clone(), nameMap)?);
            ()
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; modifications = mergeScalarsMod(var_field!((*element).modifications, SCode::Element::COMPONENT).clone(), nameMap)?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(element)
}

pub(crate) fn mergeScalarsEql(mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>>, mut nameMap: MergeNameMap) -> Result<Arc<metamodelica::List<Arc<SCode::Equation>>>> {
    let mut eql: Arc<metamodelica::List<Arc<SCode::Equation>>> = eql;
    eql = SCodeUtil::mapEquationsList(eql, (std::sync::Arc::new({ let __pe_b1 = nameMap; move |__pe_a0| mergeScalarsEq(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Equation>) -> Result<Arc<SCode::Equation>> + 'static>))?;
    Ok(eql)
}

pub(crate) fn mergeScalarsEq(mut eq: Arc<SCode::Equation>, mut nameMap: MergeNameMap) -> Result<Arc<SCode::Equation>> {
    let mut eq: Arc<SCode::Equation> = eq;
    eq = SCodeUtil::mapEquationExps(eq, (std::sync::Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| mergeScalarsExps(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>))?;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_CONNECT { .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_CONNECT;
                crefLeft = mergeScalarsCref(var_field!((*eq).crefLeft, SCode::Equation::EQ_CONNECT).clone(), nameMap.clone())?,
                crefRight = mergeScalarsCref(var_field!((*eq).crefRight, SCode::Equation::EQ_CONNECT).clone(), nameMap)?
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eq)
}

pub(crate) fn mergeScalarsMod(mut r#mod: Arc<SCode::Mod>, mut nameMap: MergeNameMap) -> Result<Arc<SCode::Mod>> {
    let mut r#mod: Arc<SCode::Mod> = r#mod;
    let () = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD;
                binding = Util::applyOption(var_field!((*r#mod).binding, SCode::Mod::MOD).clone(), (std::sync::Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| mergeScalarsExps(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>))?,
                subModLst = ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::SubMod>>> = metamodelica::nil();
        for mut m in (var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()).into_iter().cloned() {
            let __x = mergeScalarsSubMod(m.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#mod)
}

pub(crate) fn mergeScalarsSubMod(mut r#mod: Arc<SCode::SubMod>, mut nameMap: MergeNameMap) -> Result<Arc<SCode::SubMod>> {
    let mut r#mod: Arc<SCode::SubMod> = r#mod;
    assign_field!(r#mod.r#mod = mergeScalarsMod(r#mod.r#mod.clone(), nameMap)?);
    Ok(r#mod)
}

pub(crate) fn mergeScalarsExps(mut exp: Arc<Absyn::Exp>, mut nameMap: MergeNameMap) -> Result<Arc<Absyn::Exp>> {
    let mut exp: Arc<Absyn::Exp> = exp;
    (exp, _) = AbsynUtil::traverseExp(exp, (std::sync::Arc::new(mergeScalarsExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<Absyn::ComponentRef>>>) -> Result<(Arc<Absyn::Exp>, Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<Absyn::ComponentRef>>>)> + 'static>), nameMap)?;
    Ok(exp)
}

pub(crate) fn mergeScalarsExp(mut exp: Arc<Absyn::Exp>, mut nameMap: MergeNameMap) -> Result<(Arc<Absyn::Exp>, MergeNameMap)> {
    let mut exp: Arc<Absyn::Exp> = exp;
    let mut nameMap: MergeNameMap = nameMap;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { .. } if (!(AbsynUtil::crefIsWild(var_field!((*exp).componentRef, Absyn::Exp::CREF).clone()))) => {
            assign_variant_field!(exp => Absyn::Exp::CREF; componentRef = mergeScalarsCref(var_field!((*exp).componentRef, Absyn::Exp::CREF).clone(), nameMap.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, nameMap))
}

pub(crate) fn mergeScalarsCref(mut cref: Arc<Absyn::ComponentRef>, mut nameMap: MergeNameMap) -> Result<Arc<Absyn::ComponentRef>> {
    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let mut repl_ocr: Option<Arc<Absyn::ComponentRef>>;
    let mut repl_cr: Arc<Absyn::ComponentRef>;
    let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    repl_ocr = UnorderedMap::get((AbsynUtil::crefFirstIdent(cref.clone())?).clone(), nameMap)?;
    if isSome(repl_ocr.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(repl_ocr) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        repl_cr = __pa0.clone();
        subs = AbsynUtil::crefFirstSubs(cref.clone())?;
        if !(subs.clone().is_empty()) {
            subs = listAppend(AbsynUtil::crefFirstSubs(repl_cr.clone())?, subs);
            repl_cr = AbsynUtil::crefSetLastSubs(repl_cr, subs)?;
        }
        cref = AbsynUtil::crefReplaceFirst(cref, repl_cr)?;
    }
    Ok(cref)
}

pub(crate) fn mergeScalarsAlgs(mut algs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, mut nameMap: MergeNameMap) -> Result<Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>> {
    let mut algs: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = algs;
    algs = ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>> = metamodelica::nil();
        for mut a in (algs).into_iter().cloned() {
            let __x = SCodeUtil::mapAlgorithmStatements(a.clone(), (std::sync::Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| mergeScalarsStmt(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Statement>) -> Result<Arc<SCode::Statement>> + 'static>))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(algs)
}

pub(crate) fn mergeScalarsStmt(mut stmt: Arc<SCode::Statement>, mut nameMap: MergeNameMap) -> Result<Arc<SCode::Statement>> {
    let mut stmt: Arc<SCode::Statement> = stmt;
    stmt = SCodeUtil::mapStatementExps(stmt, (std::sync::Arc::new({ let __pe_b1 = nameMap; move |__pe_a0| mergeScalarsExps(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>))?;
    Ok(stmt)
}

pub(crate) fn mergeScalarsComponentBindings(mut node: Arc<InstNode::InstNode>, mut nameMap: MergeNameMap) -> Result<()> {
    let mut cls: Arc<Class::NFClass>;
    let mut cls_tree: Arc<ClassTree::ClassTree>;
    cls = InstNode::getClass(node.clone())?;
    cls_tree = Class::classTree(cls.clone())?;
    ClassTree::applyComponents(cls_tree.clone(), (std::sync::Arc::new({ let __pe_b1 = nameMap; move |__pe_a0| mergeScalarsComponentBinding(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InstNode::InstNode>) -> Result<()> + 'static>))?;
    cls = Class::setClassTree(cls_tree, cls)?;
    InstNode::updateClass(cls, node)?;
    Ok(())
}

pub(crate) fn mergeScalarsComponentBinding(mut node: Arc<InstNode::InstNode>, mut nameMap: MergeNameMap) -> Result<()> {
    let mut comp: Arc<Component::NFComponent>;
    if !(InstNode::isComponent(node.clone())?) {
        return Ok(());
    }
    comp = InstNode::component(node.clone())?;
    let () = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Component::COMPONENT_DEF { .. } => {
            assign_variant_field!(comp => Component::NFComponent::COMPONENT_DEF; definition = mergeScalarsElement(var_field!((*comp).definition, Component::NFComponent::COMPONENT_DEF).clone(), nameMap)?);
            InstNode::updateComponent(comp, node)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn createExtractorModel(mut flatModel: Arc<FlatModel::NFFlatModel>, mut funcs: Arc<NFFlatten::FunctionTreeImpl::Tree>) -> Result<(Arc<FlatModel::NFFlatModel>, Arc<NFFlatten::FunctionTreeImpl::Tree>)> {
    let mut extractorModel: Arc<FlatModel::NFFlatModel> = flatModel.clone();
    let mut outFuncs: Arc<NFFlatten::FunctionTreeImpl::Tree> = funcs.clone();
    let mut top_level_connectors: Arc<metamodelica::List<Arc<Variable::NFVariable>>>;
    let mut flows: Arc<metamodelica::List<Arc<Variable::NFVariable>>>;
    let mut inputs: Arc<metamodelica::List<Arc<Variable::NFVariable>>>;
    let mut fn_template: Arc<Function::Function>;
    let mut index: i32 = 0;
    let mut eqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut eq: Arc<Equation::NFEquation>;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    (top_level_connectors, flows, inputs) = collectExtractorModelVariables(flatModel.variables.clone())?;
    fn_template = createExtractorModelDummyFn(top_level_connectors.clone())?;
    args = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut c in (top_level_connectors).into_iter().cloned() {
            let __x = Expression::fromCref(Variable::name(c.clone()), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    for mut f in &*flows {
        let mut f = f.clone();
        (eq, outFuncs, index) = createExtractorModelDummyEq(f.clone(), (literal!("flow")).clone(), fn_template.clone(), args.clone(), outFuncs.clone(), index)?;
        eqs = metamodelica::cons(eq.clone(), eqs.clone());
    }
    for mut i in &*inputs {
        let mut i = i.clone();
        (eq, outFuncs, index) = createExtractorModelDummyEq(i.clone(), (literal!("input")).clone(), fn_template.clone(), args.clone(), outFuncs.clone(), index)?;
        eqs = metamodelica::cons(eq.clone(), eqs.clone());
    }
    eqs = metamodelica::Dangerous::listReverseInPlace(eqs);
    assign_field!(extractorModel.equations = listAppend(extractorModel.equations.clone(), eqs));
    Ok((extractorModel, outFuncs))
}

pub(crate) fn collectExtractorModelVariables(mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<(Arc<metamodelica::List<Arc<Variable::NFVariable>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut topLevelConnectorVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut flowVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut inputVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut top_node: Arc<InstNode::InstNode>;
    for mut var in &*vars.reverse() {
        let mut var = var.clone();
        if !(ComponentRef::isSimple(var.name.clone())) {
            top_node = ComponentRef::node(ComponentRef::last(var.name.clone()))?;
            if InstNode::isConnector(top_node.clone())? && InstNode::isPublic(top_node.clone()) {
                topLevelConnectorVars = metamodelica::cons(var.clone(), topLevelConnectorVars.clone());
                if Variable::isFlow(var.clone()) {
                    flowVars = metamodelica::cons(var.clone(), flowVars.clone());
                } else if Variable::isInput(var.clone()) {
                    inputVars = metamodelica::cons(var.clone(), flowVars.clone());
                }
            }
        }
    }
    Ok((topLevelConnectorVars, flowVars, inputVars))
}

pub(crate) static REAL_TYPE_SPEC: std::sync::LazyLock<Arc<Absyn::TypeSpec>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::TypeSpec::TPATH { path: Arc::new(Absyn::Path::IDENT { name: (literal!("Real")).clone() }), arrayDim: None }) });

pub(crate) fn createExtractorModelDummyFn(mut connectors: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<Arc<Function::Function>> {
    let mut r#fn: Arc<Function::Function>;
    let mut cdef: Arc<SCode::ClassDef>;
    let mut output_param: Arc<SCode::Element>;
    let mut elem: Arc<SCode::Element>;
    let mut fn_node: Arc<InstNode::InstNode>;
    let mut params: Arc<metamodelica::List<Arc<SCode::Element>>>;
    let mut output_binding: Arc<SCode::Mod>;
    let mut cmt: Arc<SCode::Comment>;
    output_binding = SCodeUtil::makeMod(false, false, metamodelica::nil(), Some(Arc::new(Absyn::Exp::INTEGER { value: 0 })), None, Absyn::dummyInfo.clone());
    output_param = Arc::new(SCode::Element::COMPONENT { name: (literal!("dummy")).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::defaultOutputAttr.clone(), typeSpec: REAL_TYPE_SPEC.clone(), modifications: output_binding, comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() });
    params = listAppend(({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut c in (connectors).into_iter().cloned() {
            let __x = createExtractorModelDummyFnInput(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), list![output_param]);
    cdef = Arc::new(SCode::ClassDef::PARTS { elementLst: params, normalEquationLst: metamodelica::nil(), initialEquationLst: metamodelica::nil(), normalAlgorithmLst: metamodelica::nil(), initialAlgorithmLst: metamodelica::nil(), constraintLst: metamodelica::nil(), clsattrs: metamodelica::nil(), externalDecl: None });
    cmt = Arc::new(SCode::Comment { annotation_: Some(Arc::new(SCode::Annotation { modification: SCodeUtil::makeMod(false, false, list![Arc::new(SCode::SubMod { ident: (literal!("Inline")).clone(), r#mod: SCodeUtil::makeMod(false, false, metamodelica::nil(), Some(Arc::new(Absyn::Exp::BOOL { value: false })), None, Absyn::dummyInfo.clone()) })], None, None, Absyn::dummyInfo.clone()) })), comment: None });
    elem = Arc::new(SCode::Element::CLASS { name: (literal!("dummy")).clone(), prefixes: SCode::defaultPrefixes.clone(), encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: openmodelica_ast::Absyn::FunctionPurity::PURE } }, classDef: cdef, cmt: cmt, info: Absyn::dummyInfo.clone() });
    fn_node = InstNode::new(elem, crate::NFInstNode::InstNode::interned_EMPTY_NODE())?;
    fn_node = Function::instFunctionNode(fn_node, NFInstContext::FUNCTION.clone(), Absyn::dummyInfo.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(Function::typeNodeCache(fn_node, NFInstContext::FUNCTION.clone())?) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#fn = __pa0.clone();
    Ok(r#fn)
}

pub(crate) fn createExtractorModelDummyFnInput(mut var: Arc<Variable::NFVariable>) -> Result<Arc<SCode::Element>> {
    let mut inputElem: Arc<SCode::Element>;
    inputElem = Arc::new(SCode::Element::COMPONENT { name: (ComponentRef::toFlatString(var.name.clone(), BaseModelica::defaultFormat.clone())?).clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::defaultInputAttr.clone(), typeSpec: REAL_TYPE_SPEC.clone(), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() });
    Ok(inputElem)
}

pub(crate) fn createExtractorModelDummyEq(mut var: Arc<Variable::NFVariable>, mut varType: ArcStr, mut r#fn: Arc<Function::Function>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut funcs: Arc<NFFlatten::FunctionTreeImpl::Tree>, mut index: i32) -> Result<(Arc<Equation::NFEquation>, Arc<NFFlatten::FunctionTreeImpl::Tree>, i32)> {
    let mut eq: Arc<Equation::NFEquation>;
    let mut funcs: Arc<NFFlatten::FunctionTreeImpl::Tree> = funcs;
    let mut index: i32 = index;
    let mut indexed_fn: Arc<Function::Function>;
    let mut fn_name: Arc<Absyn::Path> = Arc::new(<Absyn::Path as ::std::default::Default>::default());
    let mut src: Arc<DAE::ElementSource> = DAE::emptyElementSource().clone();
    let mut var_name: ArcStr;
    loop {
        index = index + 1;
        fn_name = Arc::new(Absyn::Path::IDENT { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("f")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index))); ArcStr::from(__mm_s) }).clone() });
        if !(NFFlatten::FunctionTreeImpl::hasKey(funcs.clone(), fn_name.clone())?) {
            break;
        }
    }
    indexed_fn = Function::setName(fn_name.clone(), r#fn.clone());
    var_name = (ComponentRef::toString(Variable::name(var))?).clone();
    src = ElementSource::addCommentToSource(src, Some(Arc::new(SCode::Comment { annotation_: None, comment: Some(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Dummy equation for ")); __mm_s.push_str(&*var_name); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*varType); __mm_s.push_str(&*literal!(" variable")); ArcStr::from(__mm_s) }).clone()) })));
    eq = Equation::makeEquality(Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat((0) as f64) }), Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(indexed_fn.clone(), args, Variability::CONTINUOUS.clone(), Purity::PURE.clone(), indexed_fn.returnType.clone()) }), crate::NFType::interned_REAL(), src, r#fn.node.clone(), Equation::ScalarizeMode::NO_PREFERENCE.clone());
    funcs = NFFlatten::FunctionTreeImpl::add(funcs, fn_name, indexed_fn, (std::sync::Arc::new(fnptr!(NFFlatten::FunctionTreeImpl::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))?;
    Ok((eq, funcs, index))
}

