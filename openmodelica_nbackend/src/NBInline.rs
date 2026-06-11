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

use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBModule as Module;
use crate::NBReplacements as Replacements;
use crate::NBSlice as Slice;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFlatten::FunctionTree;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:         NBInline.mo
///  package:      NBInline
///  description:  This file contains functions for inlining operations.
pub struct NBInline<T>(std::marker::PhantomData<T>);
pub(crate) fn main(mut bdae: Arc<BackendDAE::NBackendDAE>, mut inline_types: Arc<metamodelica::List<DAE::InlineType>>, mut init: bool) -> Result<Arc<BackendDAE::NBackendDAE>> {
    let mut bdae: Arc<BackendDAE::NBackendDAE> = bdae;
    bdae = (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ BackendDAE::MAIN { .. } => {
            let mut eqData: Arc<EqData::EqData>;
            let mut varData: Arc<VarData::VarData>;
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                metamodelica::print((StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dumpBackendInline] Inlining operatations for: ")); __mm_s.push_str(&*List::toString(inline_types.clone(), (std::sync::Arc::new(fnptr!(DAEDump::dumpInlineTypeBackendStr, DAE::InlineType)) as std::sync::Arc<dyn ::std::ops::Fn(DAE::InlineType) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone())?).clone());
            }
            (eqData, varData) = inline(var_field!((*bdae).eqData, BackendDAE::NBackendDAE::MAIN).clone(), var_field!((*bdae).varData, BackendDAE::NBackendDAE::MAIN).clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), inline_types, init)?;
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                eqData = eqData.clone(),
                varData = varData.clone()
            );
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                metamodelica::print((literal!("\n")).clone());
            }
            bdae
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBInline.main")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bdae)
}

pub(crate) fn inlineForEquation(mut eqn: Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: new_eqn, tail: Deref @ metamodelica::List::Nil }, .. } if (BEquation::Iterator::size(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone(), false)? == 1 && !(BEquation::Iterator::isResizable(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone())?)) => {
            let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
            let mut names: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
            let mut ranges: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
            let mut name: Arc<ComponentRef::NFComponentRef>;
            let mut range: Arc<Expression::NFExpression>;
            let mut start: i32;
            let mut new_eqn = (*new_eqn).clone();
            replacements = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            (names, ranges, _) = BEquation::Iterator::getFrames(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone())?;
            for mut tpl in &*List::zip(names.clone(), ranges.clone()) {
                let mut tpl = tpl.clone();
                (name, range) = tpl.clone();
                (start, _, _) = Expression::getIntegerRange(range.clone(), true)?;
                UnorderedMap::add(name.clone(), Arc::new(Expression::NFExpression::INTEGER { value: start.clone() }), replacements.clone())?;
            }
            new_eqn = BEquation::Equation::map(new_eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = replacements.clone(); move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*literal!("NBInline.inlineForEquation")); __mm_s.push_str(&*literal!("] Inlining: ")); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-- Result: ")); __mm_s.push_str(&*BEquation::Equation::toString(new_eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            new_eqn.clone()
        },
        _ => {
            eqn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

pub(crate) fn functionInlineable(mut r#fn: Arc<Function::Function>) -> Result<bool> {
    let mut b: bool = false;
    if Function::hasSingleOrEmptyBody(r#fn.clone()) {
        b = (::match_deref::match_deref! { match &(Function::getBody(r#fn)?) {
        Deref @ metamodelica::List::Cons { head: Deref @ Statement::ASSIGNMENT { .. }, tail: Deref @ metamodelica::List::Nil } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(b)
}

pub(crate) fn inlineRecordSliceEquation(mut slice: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>, mut variables: Arc<VariablePointers::VariablePointers>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>, mut inlineSimple: bool) -> Result<Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>> {
    let mut slices: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>;
    let mut record_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> = Pointer::create(metamodelica::nil());
    inlineRecordTupleArrayEquation(Pointer::access(Slice::getT(slice.clone())), crate::NBEquation::Iterator::interned_EMPTY(), variables, record_eqns.clone(), set, index, inlineSimple)?;
    slices = ({
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
        for mut eqn in (Pointer::access(record_eqns)).into_iter().cloned() {
            let __x = Arc::new(Slice::NBSlice { t: eqn.clone(), indices: metamodelica::nil() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if slices.clone().is_empty() {
        slices = list![slice];
    }
    Ok(slices)
}

pub(crate) fn inlineArrayConstructorSingle(mut eqn: Arc<Equation::Equation>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>) -> Result<(Arc<Equation::Equation>, bool)> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut changed: bool = false;
    match '__try0: {
        (eqn, changed) = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: lhs @ Deref @ Expression::CREF { .. }, rhs: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } }, .. } => {
            (unwrap_break_err!(inlineArrayConstructor(eqn.clone(), var_field!((**lhs).cref, Expression::NFExpression::CREF).clone(), var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0), true)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } }, rhs: rhs @ Deref @ Expression::CREF { .. }, .. } => {
            (unwrap_break_err!(inlineArrayConstructor(eqn.clone(), var_field!((**rhs).cref, Expression::NFExpression::CREF).clone(), var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0), true)
        },
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: body, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut new_eqn: Arc<Equation::Equation>;
            (new_eqn, changed) = unwrap_break_err!(inlineArrayConstructorSingle(body.clone(), var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone(), variables.clone(), set.clone(), index.clone(), new_eqns.clone()), '__try0);
            new_eqn = if (changed) {new_eqn.clone()} else {eqn.clone()};
            (new_eqn.clone(), changed)
        },
        _ => {
            (eqn.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        eqn = if (BEquation::Equation::isDummy(eqn.clone())) {Pointer::access(unwrap_break_err!(listHead(Pointer::access(new_eqns.clone())), '__try0))} else {eqn.clone()};
        Ok::<_, anyhow::Error>((changed.clone(),))
    } {
        Ok((__try0_o0,)) => {
            changed = __try0_o0;
        }
        Err(_) => {
            changed = false;
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to inline following equation:\n")); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone())?;
            }
        }
    }
    Ok((eqn, changed))
}

fn inline(mut eqData: Arc<EqData::EqData>, mut varData: Arc<VarData::VarData>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Function::Function>>>, mut inline_types: Arc<metamodelica::List<DAE::InlineType>>, mut init: bool) -> Result<(Arc<EqData::EqData>, Arc<VarData::VarData>)> {
    let mut eqData: Arc<EqData::EqData> = eqData;
    let mut varData: Arc<VarData::VarData> = varData;
    let mut replacements: Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<Function::Function>>>;
    let mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut variables: Arc<VariablePointers::VariablePointers> = BVariable::VarData::getVariables(varData.clone())?;
    let mut key: Arc<Absyn::Path>;
    let mut value: Arc<Function::Function>;
    let mut func_map: Arc<UnorderedMap::UnorderedMap<Arc<Function::Function>, Arc<InlineRating::InlineRating>>> = UnorderedMap::new((std::sync::Arc::new(Function::nameHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Function::Function>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(Function::nameEqual, Arc<Function::Function>, Arc<Function::Function>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Function::Function>, Arc<Function::Function>) -> Result<bool> + 'static>), 1);
    replacements = UnorderedMap::new((std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>), 1);
    for mut tpl in &*UnorderedMap::toList(funcMap) {
        let mut tpl = tpl.clone();
        (key, value) = tpl.clone();
        if checkInline(value.clone(), inline_types.clone(), func_map.clone())? {
            UnorderedMap::add(key.clone(), value.clone(), replacements.clone())?;
        }
    }
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? && List::contains(inline_types.clone(), openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE, (std::sync::Arc::new(fnptr!(DAEUtil::inlineTypeEqual, DAE::InlineType, DAE::InlineType)) as std::sync::Arc<dyn ::std::ops::Fn(DAE::InlineType, DAE::InlineType) -> Result<bool> + 'static>))? && !(init) {
        metamodelica::print((StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Heuristic results for Inline=default functions. Threshold = ")); __mm_s.push_str(&*intString(HEURISTIC_THRESHOLD.clone())); ArcStr::from(__mm_s) }).clone())?).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*UnorderedMap::toString(func_map, (std::sync::Arc::new({ let __pe_b1 = false; move |__pe_a0| Function::signatureString(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Function::Function>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(InlineRating::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<InlineRating::InlineRating>) -> Result<ArcStr> + 'static>), (literal!("\n")).clone(), (literal!(", ")).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
    }
    eqData = Replacements::replaceFunctions(eqData, variables.clone(), replacements)?;
    set = UnorderedSet::new((std::sync::Arc::new(BVariable::hash) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(BVariable::equalName) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), 13);
    if !(List::any(inline_types, (std::sync::Arc::new({ let __pe_b1 = openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE; move |__pe_a0| Ok(DAEUtil::inlineTypeEqual(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(DAE::InlineType) -> Result<bool> + 'static>))?) {
        eqData = inlineRecordsTuplesArrays(eqData, variables.clone(), set.clone(), init)?;
    }
    eqData = BEquation::EqData::map(eqData, (std::sync::Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = set.clone(); move |__pe_a0| BackendDAE::lowerEquationIterators(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> + 'static>))?;
    varData = BVariable::VarData::addTypedList(varData, UnorderedSet::toList(set), BVariable::VarData::VarType::ITERATOR.clone())?;
    eqData = BEquation::EqData::mapExp(eqData, (std::sync::Arc::new({ let __pe_b1 = variables; let __pe_b2 = true; move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok((eqData, varData))
}

fn inlineRecordsTuplesArrays(mut eqData: Arc<EqData::EqData>, mut variables: Arc<VariablePointers::VariablePointers>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut init: bool) -> Result<Arc<EqData::EqData>> {
    let mut eqData: Arc<EqData::EqData> = eqData;
    let mut index: Pointer::Pointer<i32> = BEquation::EqData::getUniqueIndex(eqData.clone())?;
    let mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> = Pointer::create(metamodelica::nil());
    if init {
        eqData = (::match_deref::match_deref! { match &(eqData.clone()) {
        Deref @ BEquation::EqData::EQ_DATA_SIM { .. } => {
            assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; initials = BEquation::EquationPointers::map(var_field!((*eqData).initials, EqData::EqData::EQ_DATA_SIM).clone(), (std::sync::Arc::new({ let __pe_b1 = crate::NBEquation::Iterator::interned_EMPTY(); let __pe_b2 = variables; let __pe_b3 = new_eqns.clone(); let __pe_b4 = set; let __pe_b5 = index; let __pe_b6 = false; move |__pe_a0| inlineRecordTupleArrayEquation(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> + 'static>))?);
            assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; initials = BEquation::EquationPointers::addList(Pointer::access(new_eqns), var_field!((*eqData).initials, EqData::EqData::EQ_DATA_SIM).clone())?);
            assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; initials = BEquation::EquationPointers::compress(var_field!((*eqData).initials, EqData::EqData::EQ_DATA_SIM).clone())?);
            eqData
        },
        _ => eqData,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    } else {
        eqData = BEquation::EqData::map(eqData, (std::sync::Arc::new({ let __pe_b1 = crate::NBEquation::Iterator::interned_EMPTY(); let __pe_b2 = variables; let __pe_b3 = new_eqns.clone(); let __pe_b4 = set; let __pe_b5 = index; let __pe_b6 = false; move |__pe_a0| inlineRecordTupleArrayEquation(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> + 'static>))?;
        eqData = BEquation::EqData::addUntypedList(eqData, Pointer::access(new_eqns), false)?;
        eqData = BEquation::EqData::compress(eqData)?;
    }
    Ok(eqData)
}

pub(crate) fn inlineRecordTupleArrayEquation(mut eqn: Arc<Equation::Equation>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>, mut inlineSimple: bool) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    if '__try0: {
        eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::RECORD_EQUATION { lhs: Deref @ Expression::CREF { .. }, rhs: Deref @ Expression::CREF { .. }, .. } if (!(inlineSimple)) => {
            eqn.clone()
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CREF { .. }, rhs: Deref @ Expression::CREF { .. }, .. } if (!(inlineSimple)) => {
            eqn.clone()
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { ty: Deref @ Type::COMPLEX { .. }, .. } => {
            unwrap_break_err!(inlineRecordEquation(eqn.clone(), var_field!((*eqn).lhs, Equation::Equation::RECORD_EQUATION).clone(), var_field!((*eqn).rhs, Equation::Equation::RECORD_EQUATION).clone(), iter.clone(), var_field!((*eqn).attr, Equation::Equation::RECORD_EQUATION).clone(), var_field!((*eqn).recordSize, Equation::Equation::RECORD_EQUATION).clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone(), inlineSimple), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { recordSize: Some(size), .. } => {
            unwrap_break_err!(inlineRecordEquation(eqn.clone(), var_field!((*eqn).lhs, Equation::Equation::ARRAY_EQUATION).clone(), var_field!((*eqn).rhs, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), size.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone(), inlineSimple), '__try0)
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { .. } => {
            unwrap_break_err!(inlineTupleEquation(eqn.clone(), var_field!((*eqn).lhs, Equation::Equation::RECORD_EQUATION).clone(), var_field!((*eqn).rhs, Equation::Equation::RECORD_EQUATION).clone(), var_field!((*eqn).attr, Equation::Equation::RECORD_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: lhs @ Deref @ Expression::ARRAY { .. }, rhs: rhs @ Deref @ Expression::ARRAY { .. }, .. } => {
            unwrap_break_err!(inlineArrayEquation(eqn.clone(), var_field!((**lhs).elements, Expression::NFExpression::ARRAY).clone(), var_field!((**rhs).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: lhs @ Deref @ Expression::CREF { .. }, rhs: rhs @ Deref @ Expression::ARRAY { .. }, .. } => {
            let mut dim: Arc<Dimension::NFDimension>;
            let mut elements: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
            dim = unwrap_break_err!(listHead(Type::arrayDims(var_field!((**lhs).ty, Expression::NFExpression::CREF).clone())), '__try0);
            elements = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (1..=metamodelica::arrayLength(var_field!((**rhs).elements, Expression::NFExpression::ARRAY).clone())).into_iter() {
            let __x = unwrap_break_err!(Expression::applySubscripts(list![unwrap_break_err!(Subscript::nth(dim.clone(), i.clone()), '__try0)], lhs.clone(), true), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            unwrap_break_err!(inlineArrayEquation(eqn.clone(), metamodelica::arrayFromVec(elements.clone().into_iter().cloned().collect()), var_field!((**rhs).elements, Expression::NFExpression::ARRAY).clone(), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: lhs @ Deref @ Expression::ARRAY { .. }, rhs: rhs @ Deref @ Expression::CREF { .. }, .. } => {
            let mut dim: Arc<Dimension::NFDimension>;
            let mut elements: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
            dim = unwrap_break_err!(listHead(Type::arrayDims(var_field!((**rhs).ty, Expression::NFExpression::CREF).clone())), '__try0);
            elements = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut i in (1..=metamodelica::arrayLength(var_field!((**lhs).elements, Expression::NFExpression::ARRAY).clone())).into_iter() {
            let __x = unwrap_break_err!(Expression::applySubscripts(list![unwrap_break_err!(Subscript::nth(dim.clone(), i.clone()), '__try0)], rhs.clone(), true), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            unwrap_break_err!(inlineArrayEquation(eqn.clone(), var_field!((**lhs).elements, Expression::NFExpression::ARRAY).clone(), metamodelica::arrayFromVec(elements.clone().into_iter().cloned().collect()), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: lhs @ Deref @ Expression::CREF { .. }, rhs: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } }, .. } => {
            unwrap_break_err!(inlineArrayConstructor(eqn.clone(), var_field!((**lhs).cref, Expression::NFExpression::CREF).clone(), var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } }, rhs: rhs @ Deref @ Expression::CREF { .. }, .. } => {
            unwrap_break_err!(inlineArrayConstructor(eqn.clone(), var_field!((**rhs).cref, Expression::NFExpression::CREF).clone(), var_field!((**call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: lhs @ Deref @ Expression::CREF { .. }, rhs: Deref @ Expression::CALL { call }, .. } if (unwrap_break_err!(AbsynUtil::pathString(unwrap_break_err!(Function::nameConsiderBuiltin(unwrap_break_err!(Call::typedFunction(call.clone()), '__try0)), '__try0), (literal!(".")).clone(), true, false), '__try0) == literal!("cat")) => {
            unwrap_break_err!(inlineCatCall(eqn.clone(), var_field!((**lhs).cref, Expression::NFExpression::CREF).clone(), unwrap_break_err!(Call::arguments(call.clone()), '__try0), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CALL { call }, rhs: rhs @ Deref @ Expression::CREF { .. }, .. } if (unwrap_break_err!(AbsynUtil::pathString(unwrap_break_err!(Function::nameConsiderBuiltin(unwrap_break_err!(Call::typedFunction(call.clone()), '__try0)), '__try0), (literal!(".")).clone(), true, false), '__try0) == literal!("cat")) => {
            unwrap_break_err!(inlineCatCall(eqn.clone(), var_field!((**rhs).cref, Expression::NFExpression::CREF).clone(), unwrap_break_err!(Call::arguments(call.clone()), '__try0), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: lhs @ Deref @ Expression::CREF { .. }, rhs: Deref @ Expression::CALL { call }, .. } if (unwrap_break_err!(AbsynUtil::pathString(unwrap_break_err!(Function::nameConsiderBuiltin(unwrap_break_err!(Call::typedFunction(call.clone()), '__try0)), '__try0), (literal!(".")).clone(), true, false), '__try0) == literal!("promote")) => {
            unwrap_break_err!(inlinePromoteCall(eqn.clone(), var_field!((**lhs).cref, Expression::NFExpression::CREF).clone(), unwrap_break_err!(Call::arguments(call.clone()), '__try0), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CALL { call }, rhs: rhs @ Deref @ Expression::CREF { .. }, .. } if (unwrap_break_err!(AbsynUtil::pathString(unwrap_break_err!(Function::nameConsiderBuiltin(unwrap_break_err!(Call::typedFunction(call.clone()), '__try0)), '__try0), (literal!(".")).clone(), true, false), '__try0) == literal!("promote")) => {
            unwrap_break_err!(inlinePromoteCall(eqn.clone(), var_field!((**rhs).cref, Expression::NFExpression::CREF).clone(), unwrap_break_err!(Call::arguments(call.clone()), '__try0), var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone()), '__try0)
        },
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::List::Cons { head: body, tail: Deref @ metamodelica::List::Nil }, .. } => {
            let mut new_eqn: Arc<Equation::Equation>;
            new_eqn = unwrap_break_err!(inlineRecordTupleArrayEquation(body.clone(), var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone(), true), '__try0);
            new_eqn = if (BEquation::Equation::isDummy(new_eqn.clone())) {new_eqn.clone()} else {eqn.clone()};
            new_eqn.clone()
        },
        Deref @ BEquation::Equation::IF_EQUATION { .. } if (unwrap_break_err!(BEquation::IfEquationBody::isRecordOrTupleEquation(var_field!((*eqn).body, Equation::Equation::IF_EQUATION).clone()), '__try0)) => {
            let mut new_eqn: Arc<Equation::Equation>;
            new_eqn = unwrap_break_err!(inlineRecordTupleArrayIfEquation(eqn.clone(), var_field!((*eqn).body, Equation::Equation::IF_EQUATION).clone(), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone(), inlineSimple), '__try0);
            new_eqn = if (BEquation::Equation::isDummy(new_eqn.clone())) {new_eqn.clone()} else {eqn.clone()};
            new_eqn.clone()
        },
        _ => {
            eqn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to inline following equation:\n")); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone())?;
        }
    }
    Ok(eqn)
}

fn inlineRecordTupleArrayIfEquation(mut eqn: Arc<Equation::Equation>, mut body: Arc<IfEquationBody::IfEquationBody>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>, mut inlineSimple: bool) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>;
    let mut new_body: Arc<IfEquationBody::IfEquationBody>;
    let mut new_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    eqns = Pointer::access(new_eqns.clone());
    new_body = inlineRecordTupleArrayIfBody(body, iter.clone(), variables, set, index.clone(), inlineSimple)?;
    for mut b in &*BEquation::IfEquationBody::split(new_body)? {
        let mut b = b.clone();
        new_eqn = BEquation::IfEquationBody::makeIfEquation(b.clone(), index.clone(), (arcstr::literal!(BEquation::SIMULATION_STR)).clone(), iter.clone(), BEquation::Equation::getSource(eqn.clone()), BEquation::Equation::getAttributes(eqn.clone()))?;
        eqns = metamodelica::cons(new_eqn.clone(), eqns.clone());
    }
    Pointer::update(new_eqns, eqns);
    eqn = crate::NBEquation::Equation::interned_DUMMY_EQUATION();
    Ok(eqn)
}

fn inlineRecordTupleArrayIfBody(mut body: Arc<IfEquationBody::IfEquationBody>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>, mut inlineSimple: bool) -> Result<Arc<IfEquationBody::IfEquationBody>> {
    let mut body: Arc<IfEquationBody::IfEquationBody> = body;
    let mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> = Pointer::create(metamodelica::nil());
    assign_field!(
        body.then_eqns = List::flatten(({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
        for mut e in (body.then_eqns.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(inlineRecordTupleArrayEquation(Pointer::access(e.clone()), iter.clone(), variables.clone(), new_eqns.clone(), set.clone(), index.clone(), inlineSimple)?) {
        Deref @ BEquation::Equation::DUMMY_EQUATION => Pointer::access(new_eqns.clone()),
        _ => list![e.clone()],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?,
        body.else_if = Util::applyOption(body.else_if.clone(), (std::sync::Arc::new({ let __pe_b1 = iter; let __pe_b2 = variables; let __pe_b3 = set; let __pe_b4 = index; let __pe_b5 = inlineSimple; move |__pe_a0| inlineRecordTupleArrayIfBody(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<IfEquationBody::IfEquationBody>) -> Result<Arc<IfEquationBody::IfEquationBody>> + 'static>))?
    );
    Ok(body)
}

fn inlineRecordEquation(mut eqn: Arc<Equation::Equation>, mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>, mut iter: Arc<Iterator::Iterator>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut recordSize: i32, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>, mut inlineSimple: bool) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut new_lhs: Arc<Expression::NFExpression>;
    let mut new_rhs: Arc<Expression::NFExpression>;
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>;
    if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*literal!("NBInline.inlineRecordEquation")); __mm_s.push_str(&*literal!("] Inlining: ")); ArcStr::from(__mm_s) }).clone());
        if !(BEquation::Iterator::isEmpty(iter.clone())) {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*BEquation::Iterator::toString(iter.clone())?); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
        }
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*BEquation::Equation::toString(eqn, (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    eqns = Pointer::access(new_eqns.clone());
    for mut i in 1..=recordSize {
        new_lhs = inlineRecordConstructorExp(lhs.clone(), i.clone(), variables.clone())?;
        new_rhs = inlineRecordConstructorExp(rhs.clone(), i.clone(), variables.clone())?;
        eqns = createInlinedEquation(eqns.clone(), new_lhs.clone(), new_rhs.clone(), attr.clone(), iter.clone(), variables.clone(), set.clone(), index.clone())?;
    }
    Pointer::update(new_eqns, eqns);
    eqn = crate::NBEquation::Equation::interned_DUMMY_EQUATION();
    Ok(eqn)
}

fn inlineTupleEquation(mut eqn: Arc<Equation::Equation>, mut LHS: Arc<Expression::NFExpression>, mut RHS: Arc<Expression::NFExpression>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>;
    let mut lhs_elems: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut rhs_elems: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut lhs: Arc<Expression::NFExpression>;
    let mut rhs: Arc<Expression::NFExpression>;
    lhs_elems = getElementList(LHS)?;
    rhs_elems = getElementList(RHS)?;
    if !(lhs_elems.clone().is_empty()) && List::compareLength(lhs_elems.clone(), rhs_elems.clone())? == 0 {
        if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*literal!("NBInline.inlineTupleEquation")); __mm_s.push_str(&*literal!("] Inlining: ")); ArcStr::from(__mm_s) }).clone());
            if !(BEquation::Iterator::isEmpty(iter.clone())) {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*BEquation::Iterator::toString(iter.clone())?); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
            }
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*BEquation::Equation::toString(eqn, (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        eqns = Pointer::access(new_eqns.clone());
        for mut tpl in &*List::zip(lhs_elems, rhs_elems) {
            let mut tpl = tpl.clone();
            (lhs, rhs) = tpl.clone();
            if !(Expression::isWildCref(lhs.clone()) || Expression::isWildCref(rhs.clone())) {
                eqns = createInlinedEquation(eqns.clone(), lhs.clone(), rhs.clone(), attr.clone(), iter.clone(), variables.clone(), set.clone(), index.clone())?;
            }
        }
        Pointer::update(new_eqns, eqns);
        eqn = crate::NBEquation::Equation::interned_DUMMY_EQUATION();
    }
    Ok(eqn)
}

fn inlineArrayEquation(mut eqn: Arc<Equation::Equation>, mut lhs_elements: metamodelica::Array<Arc<Expression::NFExpression>>, mut rhs_elements: metamodelica::Array<Arc<Expression::NFExpression>>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>;
    if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*literal!("NBInline.inlineArrayEquation")); __mm_s.push_str(&*literal!("] Inlining: ")); ArcStr::from(__mm_s) }).clone());
        if !(BEquation::Iterator::isEmpty(iter.clone())) {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*BEquation::Iterator::toString(iter.clone())?); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
        }
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*BEquation::Equation::toString(eqn, (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    eqns = Pointer::access(new_eqns.clone());
    for mut i in 1..=metamodelica::arrayLength(lhs_elements.clone()) {
        eqns = createInlinedEquation(eqns.clone(), ({let __elt = lhs_elements.borrow()[(i.clone()-1) as usize].clone(); __elt}), ({let __elt = rhs_elements.borrow()[(i.clone()-1) as usize].clone(); __elt}), attr.clone(), iter.clone(), variables.clone(), set.clone(), index.clone())?;
    }
    Pointer::update(new_eqns, eqns);
    eqn = crate::NBEquation::Equation::interned_DUMMY_EQUATION();
    Ok(eqn)
}

fn inlineArrayConstructor(mut eqn: Arc<Equation::Equation>, mut cref: Arc<ComponentRef::NFComponentRef>, mut rhs: Arc<Expression::NFExpression>, mut iters: Arc<metamodelica::List<(Arc<InstNode::InstNode>, Arc<Expression::NFExpression>)>>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut frames: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>>;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>;
    let mut cref_exp: Arc<Expression::NFExpression>;
    let mut new_rhs: Arc<Expression::NFExpression>;
    let mut local_set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>> = UnorderedSet::new((std::sync::Arc::new(BVariable::hash) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(BVariable::equalName) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), 13);
    let mut local_it: Arc<VariablePointers::VariablePointers>;
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>;
    if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*literal!("NBInline.inlineArrayConstructor")); __mm_s.push_str(&*literal!("] Inlining: ")); ArcStr::from(__mm_s) }).clone());
        if !(BEquation::Iterator::isEmpty(iter.clone())) {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*BEquation::Iterator::toString(iter.clone())?); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
        }
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*BEquation::Equation::toString(eqn, (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    eqns = Pointer::access(new_eqns.clone());
    frames = ({
        let mut __acc: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>, Option<Arc<Iterator::Iterator>>)>> = metamodelica::nil();
        for mut iter in (iters).into_iter().cloned() {
            let __x = BEquation::Iterator::createFrame(iter.clone(), local_set.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    UnorderedSet::merge(set.clone(), local_set.clone())?;
    subs = BEquation::Iterator::normalizedSubscripts(BEquation::Iterator::fromFrames(frames.clone()), UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1))?;
    cref_exp = Expression::fromCref(ComponentRef::mergeSubscripts(subs, cref, true, false, false)?, false)?;
    local_it = BVariable::VariablePointers::fromList(UnorderedSet::toList(local_set), false)?;
    cref_exp = Expression::map(cref_exp, (std::sync::Arc::new({ let __pe_b1 = local_it.clone(); let __pe_b2 = false; move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    new_rhs = Expression::map(rhs, (std::sync::Arc::new({ let __pe_b1 = local_it; let __pe_b2 = false; move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    eqns = createInlinedEquation(eqns, cref_exp, new_rhs, attr, BEquation::Iterator::addFrames(iter, frames)?, variables, set, index)?;
    Pointer::update(new_eqns, eqns);
    eqn = crate::NBEquation::Equation::interned_DUMMY_EQUATION();
    Ok(eqn)
}

fn inlinePromoteCall(mut eqn: Arc<Equation::Equation>, mut cref: Arc<ComponentRef::NFComponentRef>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut arg: Arc<Expression::NFExpression>;
    let mut n: i32;
    let mut dim_count: i32 = 0;
    let mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>> = metamodelica::nil();
    let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut new_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*literal!("NBInline.inlinePromoteCall")); __mm_s.push_str(&*literal!("] Inlining: ")); ArcStr::from(__mm_s) }).clone());
        if !(BEquation::Iterator::isEmpty(iter.clone())) {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*BEquation::Iterator::toString(iter.clone())?); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
        }
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __pa1 }, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    arg = __pa0.clone();
    n = __pa1.clone();
    eqn = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::CREF { .. } => {
            dim_count = Type::dimensionCount(ComponentRef::getSubscriptedType(var_field!((*arg).cref, Expression::NFExpression::CREF).clone(), false)?);
            if n == dim_count {
                lhs = Expression::fromCref(cref, false)?;
            } else {
                subs = Subscript::fillWithWholeLeft(List::fill(Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) }), n - dim_count), n);
                lhs = Expression::fromCref(ComponentRef::mergeSubscripts(subs, cref, false, false, false)?, false)?;
            }
            new_eqn = BEquation::Equation::makeAssignment(lhs, arg, index, (arcstr::literal!(BEquation::SIMULATION_STR)).clone(), iter, attr)?;
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-- Result: ")); __mm_s.push_str(&*BEquation::Equation::pointerToString(new_eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            Pointer::access(new_eqn)
        },
        _ => eqn,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

fn inlineCatCall(mut eqn: Arc<Equation::Equation>, mut cref: Arc<ComponentRef::NFComponentRef>, mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut new_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut n: i32;
    let mut sz: i32 = 0;
    let mut rest: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>;
    let mut ty: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut dim: Arc<Dimension::NFDimension> = Arc::new(Dimension::BOOLEAN);
    let mut iterator_name: Arc<ComponentRef::NFComponentRef>;
    let mut lhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut rhs: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut iterator_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut update_vars: Arc<VariablePointers::VariablePointers>;
    let mut range: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut subscript_exp: Arc<Expression::NFExpression>;
    let mut lhs_sub: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut lhs_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut shift: Arc<Expression::NFExpression>;
    let mut new_size: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut local_iter: Arc<Iterator::Iterator> = Arc::new(Iterator::EMPTY);
    let mut new_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut failed: bool = false;
    if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*literal!("NBInline.inlineCatCall")); __mm_s.push_str(&*literal!("] Inlining: ")); ArcStr::from(__mm_s) }).clone());
        if !(BEquation::Iterator::isEmpty(iter.clone())) {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*BEquation::Iterator::toString(iter.clone())?); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
        }
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    eqns = Pointer::access(new_eqns.clone());
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args) {
        Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value: __pa0 }, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    n = __pa0.clone();
    rest = __pa1.clone();
    iterator_name = ComponentRef::makeIterator(InstNode::newUniqueIterator(Absyn::dummyInfo.clone(), openmodelica_nf_frontend::NFType::interned_INTEGER()), openmodelica_nf_frontend::NFType::interned_INTEGER())?;
    iterator_var = BackendDAE::lowerIterator(iterator_name)?;
    iterator_name = BVariable::getVarName(iterator_var.clone());
    update_vars = BVariable::VariablePointers::fromList(list![iterator_var.clone()], false)?;
    UnorderedSet::add(iterator_var, set)?;
    subscript_exp = Expression::fromCref(iterator_name.clone(), false)?;
    shift = Arc::new(Expression::NFExpression::INTEGER { value: 0 });
    for mut arg in &*rest {
        let mut arg = arg.clone();
        failed = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Expression::CREF { cref: __esc_rhs, .. } if (!(failed)) => {
            rhs = (*__esc_rhs).clone();
            ty = Expression::typeOf(arg.clone());
            if Type::isArray(ty.clone()) {
                dim = Type::nthDimension(ty.clone(), n)?;
                sz = Dimension::size(dim.clone(), false)?;
                if sz != 1 || Dimension::isResizable(dim.clone()) {
                    new_size = Dimension::sizeExp(dim.clone())?;
                    range = Expression::makeRange(Arc::new(Expression::NFExpression::INTEGER { value: 1 }), None, new_size.clone())?;
                    local_iter = BEquation::Iterator::addFrames(iter.clone(), list![(iterator_name.clone(), range.clone(), None)])?;
                    lhs_sub = if (Expression::isZero(shift.clone())?) {subscript_exp.clone()} else {Arc::new(Expression::NFExpression::MULTARY { arguments: list![shift.clone(), subscript_exp.clone()], inv_arguments: metamodelica::nil(), operator: Operator::makeAdd(openmodelica_nf_frontend::NFType::interned_INTEGER()) })};
                    lhs = ComponentRef::mergeSubscripts(Subscript::fillWithWholeLeft(list![Arc::new(Subscript::NFSubscript::INDEX { index: lhs_sub.clone() })], n), cref.clone(), false, false, false)?;
                    rhs = ComponentRef::mergeSubscripts(Subscript::fillWithWholeLeft(list![Arc::new(Subscript::NFSubscript::INDEX { index: subscript_exp.clone() })], n), rhs.clone(), false, false, false)?;
                    lhs_exp = Expression::map(Expression::fromCref(lhs.clone(), false)?, (std::sync::Arc::new({ let __pe_b1 = update_vars.clone(); let __pe_b2 = false; move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
                    rhs_exp = Expression::map(Expression::fromCref(rhs.clone(), false)?, (std::sync::Arc::new({ let __pe_b1 = update_vars.clone(); let __pe_b2 = false; move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
                } else {
                    new_size = Arc::new(Expression::NFExpression::INTEGER { value: 1 });
                    lhs_sub = bumpShift(shift.clone(), new_size.clone())?;
                    lhs = ComponentRef::mergeSubscripts(Subscript::fillWithWholeLeft(list![Arc::new(Subscript::NFSubscript::INDEX { index: lhs_sub.clone() })], n), cref.clone(), false, false, false)?;
                    lhs_exp = Expression::fromCref(lhs.clone(), false)?;
                    rhs = ComponentRef::mergeSubscripts(Subscript::fillWithWholeLeft(list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) })], n), rhs.clone(), false, false, false)?;
                    rhs_exp = Expression::fromCref(rhs.clone(), false)?;
                    local_iter = iter.clone();
                }
            } else {
                new_size = Arc::new(Expression::NFExpression::INTEGER { value: 1 });
                lhs_sub = bumpShift(shift.clone(), new_size.clone())?;
                lhs = ComponentRef::mergeSubscripts(Subscript::fillWithWholeLeft(list![Arc::new(Subscript::NFSubscript::INDEX { index: lhs_sub.clone() })], n), cref.clone(), false, false, false)?;
                lhs_exp = Expression::fromCref(lhs.clone(), false)?;
                rhs_exp = Expression::fromCref(rhs.clone(), false)?;
                local_iter = iter.clone();
            }
            new_eqn = BEquation::Equation::makeAssignment(lhs_exp.clone(), rhs_exp.clone(), index.clone(), (arcstr::literal!(BEquation::SIMULATION_STR)).clone(), local_iter.clone(), attr.clone())?;
            shift = bumpShift(shift.clone(), new_size.clone())?;
            eqns = metamodelica::cons(new_eqn.clone(), eqns.clone());
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-- Result: ")); __mm_s.push_str(&*BEquation::Equation::pointerToString(new_eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            false
        },
        Deref @ Expression::ARRAY { .. } if (!(failed) && Expression::isLiteral(arg.clone())?) => {
            (eqns, shift) = inlineCatCallLiterals(arg.clone(), cref.clone(), iter.clone(), attr.clone(), n, index.clone(), eqns.clone(), shift.clone(), metamodelica::nil())?;
            false
        },
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    if !(failed) {
        Pointer::update(new_eqns, eqns);
        eqn = crate::NBEquation::Equation::interned_DUMMY_EQUATION();
    }
    Ok(eqn)
}

fn inlineCatCallLiterals(mut exp: Arc<Expression::NFExpression>, mut cref: Arc<ComponentRef::NFComponentRef>, mut iter: Arc<Iterator::Iterator>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut n: i32, mut index: Pointer::Pointer<i32>, mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut shift: Arc<Expression::NFExpression>, mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>) -> Result<(Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, Arc<Expression::NFExpression>)> {
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = eqns;
    let mut shift: Arc<Expression::NFExpression> = shift;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::ARRAY { .. } => {
            let mut sub_idx: Arc<Expression::NFExpression>;
            let mut is_cat_dim: bool;
            let mut sub: Arc<Subscript::NFSubscript>;
            is_cat_dim = n == (subs.clone().len() as i32) + 1;
            sub_idx = if (is_cat_dim.clone()) {bumpShift(shift.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 1 }))?} else {Arc::new(Expression::NFExpression::INTEGER { value: 1 })};
            let __range0 = var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut elem in __range0 {
                sub = Arc::new(Subscript::NFSubscript::INDEX { index: sub_idx.clone() });
                (eqns, shift) = inlineCatCallLiterals(elem.clone(), cref.clone(), iter.clone(), attr.clone(), n, index.clone(), eqns.clone(), shift.clone(), metamodelica::cons(sub.clone(), subs.clone()))?;
                sub_idx = bumpShift(sub_idx.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 1 }))?;
            }
            if is_cat_dim.clone() {
                shift = bumpShift(shift, Arc::new(Expression::NFExpression::INTEGER { value: metamodelica::arrayLength(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone()) }))?;
            }
            ()
        },
        _ => {
            let mut lhs: Arc<ComponentRef::NFComponentRef>;
            let mut lhs_exp: Arc<Expression::NFExpression>;
            let mut new_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
            lhs = ComponentRef::mergeSubscripts(subs.reverse(), cref, false, false, false)?;
            lhs_exp = Expression::fromCref(lhs.clone(), false)?;
            new_eqn = BEquation::Equation::makeAssignment(lhs_exp.clone(), exp, index, (arcstr::literal!(BEquation::SIMULATION_STR)).clone(), iter, attr)?;
            eqns = metamodelica::cons(new_eqn.clone(), eqns);
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-- Result: ")); __mm_s.push_str(&*BEquation::Equation::pointerToString(new_eqn.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqns, shift))
}

fn bumpShift(mut shift: Arc<Expression::NFExpression>, mut new_size: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut shift: Arc<Expression::NFExpression> = shift;
    shift = (::match_deref::match_deref! { match &((shift.clone(), new_size.clone())) {
        (Deref @ Expression::INTEGER { .. }, Deref @ Expression::INTEGER { .. }) => {
            Arc::new(Expression::NFExpression::INTEGER { value: var_field!((*shift).value, Expression::NFExpression::INTEGER).clone() + var_field!((*new_size).value, Expression::NFExpression::INTEGER).clone() })
        },
        (Deref @ Expression::MULTARY { arguments: Deref @ metamodelica::List::Cons { head: Deref @ Expression::INTEGER { value }, tail: args }, .. }, Deref @ Expression::INTEGER { .. }) if (Operator::getMathClassification(var_field!((*shift).operator, Expression::NFExpression::MULTARY).clone())? == Operator::MathClassification::ADDITION.clone()) => {
            assign_variant_field!(shift => Expression::NFExpression::MULTARY; arguments = metamodelica::cons(Arc::new(Expression::NFExpression::INTEGER { value: value.clone() + var_field!((*new_size).value, Expression::NFExpression::INTEGER).clone() }), args.clone()));
            shift.clone()
        },
        (Deref @ Expression::MULTARY { arguments: Deref @ metamodelica::List::Cons { head: arg, tail: args }, .. }, _) if (Operator::getMathClassification(var_field!((*shift).operator, Expression::NFExpression::MULTARY).clone())? == Operator::MathClassification::ADDITION.clone()) => {
            assign_variant_field!(shift => Expression::NFExpression::MULTARY; arguments = metamodelica::cons(arg.clone(), metamodelica::cons(new_size, args.clone())));
            shift.clone()
        },
        _ => {
            Arc::new(Expression::NFExpression::MULTARY { arguments: list![shift.clone(), new_size], inv_arguments: metamodelica::nil(), operator: Operator::makeAdd(openmodelica_nf_frontend::NFType::interned_INTEGER()) })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(shift)
}

fn createInlinedEquation(mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>, mut attr: Arc<EquationAttributes::EquationAttributes>, mut iter: Arc<Iterator::Iterator>, mut variables: Arc<VariablePointers::VariablePointers>, mut set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut index: Pointer::Pointer<i32>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> {
    let mut eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = eqns;
    let mut tmp_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> = Pointer::create(metamodelica::nil());
    let mut inlined: Arc<Equation::Equation>;
    let mut new_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    new_eqn = BEquation::Equation::makeAssignment(lhs, rhs, index.clone(), (arcstr::literal!(BEquation::SIMULATION_STR)).clone(), iter.clone(), attr)?;
    inlined = inlineRecordTupleArrayEquation(Pointer::access(new_eqn.clone()), iter, variables, tmp_eqns.clone(), set, index, false)?;
    eqns = (::match_deref::match_deref! { match &(inlined.clone()) {
        Deref @ BEquation::Equation::DUMMY_EQUATION => listAppend(eqns, Pointer::access(tmp_eqns)),
        _ => {
            if Flags::isSet(Flags::DUMPBACKENDINLINE.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-- Result: ")); __mm_s.push_str(&*BEquation::Equation::toString(inlined, (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            metamodelica::cons(new_eqn, eqns)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqns)
}

fn inlineRecordConstructorExp(mut exp: Arc<Expression::NFExpression>, mut index: i32, mut variables: Arc<VariablePointers::VariablePointers>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::nthRecordElement(index, exp)?;
    exp = Expression::map(exp, (std::sync::Arc::new(inlineRecordConstructorElements) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    exp = Expression::map(exp, (std::sync::Arc::new({ let __pe_b1 = variables; let __pe_b2 = true; move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(exp)
}

fn inlineRecordConstructorElements(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::RECORD_ELEMENT { recordExp: Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { r#fn, .. } }, .. } => {
            let mut new_exp: Arc<Expression::NFExpression>;
            if Function::isDefaultRecordConstructor(r#fn.clone()) {
                new_exp = (var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone()).get(var_field!((*exp).index, Expression::NFExpression::RECORD_ELEMENT).clone())?;
            } else if Function::isNonDefaultRecordConstructor(r#fn.clone()) {
                new_exp = (var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone()).get(var_field!((*exp).index, Expression::NFExpression::RECORD_ELEMENT).clone())?;
            } else {
                new_exp = exp;
            }
            new_exp.clone()
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn getElementList(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<metamodelica::List<Arc<Expression::NFExpression>>>> {
    let mut elements: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    elements = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::TUPLE { .. } => {
            var_field!((*exp).elements, Expression::NFExpression::TUPLE).clone()
        },
        Deref @ Expression::TUPLE_ELEMENT { tupleExp: sub_exp @ Deref @ Expression::TUPLE { .. }, .. } => {
            let mut elem: Arc<Expression::NFExpression>;
            if var_field!((*exp).index, Expression::NFExpression::TUPLE_ELEMENT).clone() > (var_field!((**sub_exp).elements, Expression::NFExpression::TUPLE).clone().len() as i32) {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBInline.getElementList")); __mm_s.push_str(&*literal!(" failed to get subscripted tuple element: ")); __mm_s.push_str(&*Expression::toString(exp)?); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            } else {
                elem = (var_field!((**sub_exp).elements, Expression::NFExpression::TUPLE).clone()).get(var_field!((*exp).index, Expression::NFExpression::TUPLE_ELEMENT).clone())?;
            }
            list![elem.clone()]
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elements)
}

fn checkInline(mut func: Arc<Function::Function>, mut inline_types: Arc<metamodelica::List<DAE::InlineType>>, mut func_map: Arc<UnorderedMap::UnorderedMap<Arc<Function::Function>, Arc<InlineRating::InlineRating>>>) -> Result<bool> {
    let mut b: bool;
    let mut it: DAE::InlineType = Function::inlineBuiltin(func.clone());
    b = List::contains(inline_types, it.clone(), (std::sync::Arc::new(fnptr!(DAEUtil::inlineTypeEqual, DAE::InlineType, DAE::InlineType)) as std::sync::Arc<dyn ::std::ops::Fn(DAE::InlineType, DAE::InlineType) -> Result<bool> + 'static>))? && functionInlineable(func.clone())?;
    if b && DAEUtil::inlineTypeEqual(it, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE) {
        b = defaultHeuristic(func, func_map)?;
    }
    Ok(b)
}

pub(crate) const HEURISTIC_THRESHOLD: i32 = 10;

fn defaultHeuristic(mut r#fn: Arc<Function::Function>, mut func_map: Arc<UnorderedMap::UnorderedMap<Arc<Function::Function>, Arc<InlineRating::InlineRating>>>) -> Result<bool> {
    let mut b: bool;
    b = InlineRating::resolve(InlineRating::fromFunction(r#fn, func_map)?) < metamodelica::OrderedFloat((HEURISTIC_THRESHOLD.clone()) as f64);
    Ok(b)
}

pub mod InlineRating {
    use super::*;
    /// used to rate a function by how much it grows when inlining.
    ///    collects data about how often the inputs will occur and how much constant bloating inlining would cause.
    /// factors for each input with an additional constant overhead.
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct InlineRating {
        pub input_rating: metamodelica::Array<i32>,
        pub constant_rating: i32,
    }

    impl metamodelica::gc::MMTrace for InlineRating {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.input_rating, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.constant_rating, __mmv)?;
            Ok(())
        }
    }
    impl Default for InlineRating {
        fn default() -> Self {
            Self {
                input_rating: Default::default(),
                constant_rating: Default::default(),
            }
        }
    }

    pub type INLINE_RATING = InlineRating;

    pub(crate) fn toString(mut ir: Arc<InlineRating>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{resolved: ")); __mm_s.push_str(&*realString(resolve(ir.clone()))); __mm_s.push_str(&*literal!(" | input: ")); __mm_s.push_str(&*Array::toString(ir.input_rating.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("[")).clone(), (literal!(", ")).clone(), (literal!("]")).clone(), true, 0)?); __mm_s.push_str(&*literal!(" | constant: ")); __mm_s.push_str(&*intString(ir.constant_rating.clone())); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }).clone();
        Ok(r#str)
    }

    pub(crate) fn resolve(mut ir: Arc<InlineRating>) -> metamodelica::Real {
        let mut r: metamodelica::Real = metamodelica::OrderedFloat((({
        let mut __acc: i32 = 0;
        for mut v in (ir.input_rating.clone()).borrow().iter() {
            let __x = v.clone();
            __acc += __x;
        }
        __acc
    })) as f64) / metamodelica::OrderedFloat((metamodelica::arrayLength(ir.input_rating.clone())) as f64) + intReal(ir.constant_rating.clone());
        r
    }

    pub(crate) fn add(mut dst: Arc<InlineRating>, mut src: Arc<InlineRating>) -> Result<Arc<InlineRating>> {
        let mut dst: Arc<InlineRating> = dst;
        if metamodelica::arrayLength(dst.input_rating.clone()) == metamodelica::arrayLength(src.input_rating.clone()) {
            for mut i in 1..=metamodelica::arrayLength(dst.input_rating.clone()) {
                {
                    let __cell0 = ({let __elt = dst.input_rating.borrow()[(i.clone()-1) as usize].clone(); __elt}) + ({let __elt = src.input_rating.borrow()[(i.clone()-1) as usize].clone(); __elt});
                    let __idx0 = i.clone();
                    dst.input_rating.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
                }
            }
            assign_field!(dst.constant_rating = dst.constant_rating.clone() + src.constant_rating.clone());
        } else {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBInline.InlineRating.add")); __mm_s.push_str(&*literal!(" failed because dst and src input arrays are of different length.\n")); __mm_s.push_str(&*literal!("dst: ")); __mm_s.push_str(&*toString(dst.clone())?); __mm_s.push_str(&*literal!("\nsrc: ")); __mm_s.push_str(&*toString(src)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        Ok(dst)
    }

    pub(crate) fn multiply(mut ir: Arc<InlineRating>, mut i: i32) -> Arc<InlineRating> {
        let mut ir: Arc<InlineRating> = ir;
        for mut i in 1..=metamodelica::arrayLength(ir.input_rating.clone()) {
            {
                let __cell0 = i * ({let __elt = ir.input_rating.borrow()[(i-1) as usize].clone(); __elt});
                let __idx0 = i;
                ir.input_rating.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
            }
        }
        assign_field!(ir.constant_rating = i * ir.constant_rating.clone());
        ir
    }

    pub(crate) fn addConst(mut ir: Arc<InlineRating>) -> Arc<InlineRating> {
        let mut ir: Arc<InlineRating> = ir;
        assign_field!(ir.constant_rating = ir.constant_rating.clone() + 1);
        ir
    }

    pub(crate) fn addMapped(mut dst: Arc<InlineRating>, mut src: Arc<InlineRating>, mut args: metamodelica::Array<Arc<Expression::NFExpression>>, mut local_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<InlineRating>>>) -> Result<Arc<InlineRating>> {
        let mut dst: Arc<InlineRating> = dst;
        let mut irp: Pointer::Pointer<Arc<InlineRating>> = Pointer::create(Arc::new(InlineRating { input_rating: arrayCreate(metamodelica::arrayLength(dst.input_rating.clone()), 0), constant_rating: src.constant_rating.clone() }));
        if metamodelica::arrayLength(src.input_rating.clone()) == metamodelica::arrayLength(args.clone()) {
            for mut i in 1..=metamodelica::arrayLength(src.input_rating.clone()) {
                if ({let __elt = src.input_rating.borrow()[(i.clone()-1) as usize].clone(); __elt}) != 0 {
                    Expression::map(({let __elt = args.borrow()[(i.clone()-1) as usize].clone(); __elt}), (std::sync::Arc::new({ let __pe_b1 = ({let __elt = src.input_rating.borrow()[(i.clone()-1) as usize].clone(); __elt}); let __pe_b2 = irp.clone(); let __pe_b3 = local_map.clone(); move |__pe_a0| addMappedExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
                }
            }
            dst = add(dst, Pointer::access(irp))?;
        } else {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBInline.InlineRating.addMapped")); __mm_s.push_str(&*literal!(" failed because src input array and arguments are of different length.\n")); __mm_s.push_str(&*literal!("src: ")); __mm_s.push_str(&*toString(src)?); __mm_s.push_str(&*literal!("\nargs: ")); __mm_s.push_str(&*Array::toString(args.clone(), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("[")).clone(), (literal!(", ")).clone(), (literal!("]")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        Ok(dst)
    }

    pub(crate) fn addMappedExp(mut exp: Arc<Expression::NFExpression>, mut i: i32, mut irp: Pointer::Pointer<Arc<InlineRating>>, mut local_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<InlineRating>>>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            let mut iro: Option<Arc<InlineRating>>;
            iro = UnorderedMap::get(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), local_map)?;
            if isSome(iro.clone()) {
                Pointer::update(irp.clone(), add(Pointer::access(irp), multiply(Util::getOption(iro.clone())?, i))?);
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(exp)
    }

    pub(crate) fn fromFunction(mut r#fn: Arc<Function::Function>, mut func_map: Arc<UnorderedMap::UnorderedMap<Arc<Function::Function>, Arc<InlineRating>>>) -> Result<Arc<InlineRating>> {
        let mut ir: Arc<InlineRating>;
        let mut irp: Pointer::Pointer<Arc<InlineRating>>;
        let mut lir: Arc<InlineRating>;
        let mut idx: i32 = 1;
        let mut num_inp: i32 = (r#fn.inputs.clone().len() as i32);
        let mut tmp: Arc<InlineRating>;
        let mut local_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<InlineRating>>> = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        for mut inp in &*r#fn.inputs.clone() {
            let mut inp = inp.clone();
            tmp = Arc::new(InlineRating { input_rating: arrayCreate(num_inp, 0), constant_rating: 0 });
            {
                let __cell0 = 1;
                let __idx0 = idx;
                tmp.input_rating.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
            }
            idx = idx + 1;
            UnorderedMap::add(ComponentRef::fromNode(inp.clone(), InstNode::getType(inp.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()), tmp.clone(), local_map.clone())?;
        }
        for mut loc in &*r#fn.locals.clone() {
            let mut loc = loc.clone();
            irp = Pointer::create(Arc::new(InlineRating { input_rating: arrayCreate(num_inp, 0), constant_rating: 0 }));
            lir = (::match_deref::match_deref! { match &(InstNode::getBindingExpOpt(loc.clone())?) {
        Some(bind) => {
            Expression::fakeMap(bind.clone(), (std::sync::Arc::new({ let __pe_b1 = func_map.clone(); let __pe_b2 = local_map.clone(); let __pe_b3 = irp.clone(); move |__pe_a0| rateExpression(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            Pointer::access(irp.clone())
        },
        _ => {
            Pointer::access(irp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            UnorderedMap::add(ComponentRef::fromNode(loc.clone(), InstNode::getType(loc.clone())?, metamodelica::nil(), ComponentRef::Origin::CREF.clone()), lir.clone(), local_map.clone())?;
        }
        irp = Pointer::create(Arc::new(InlineRating { input_rating: arrayCreate(num_inp, 0), constant_rating: 0 }));
        Expression::fakeMap(Function::getSingleBodyExp(r#fn.clone())?, (std::sync::Arc::new({ let __pe_b1 = func_map.clone(); let __pe_b2 = local_map; let __pe_b3 = irp.clone(); move |__pe_a0| rateExpression(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
        ir = Pointer::access(irp);
        UnorderedMap::add(r#fn, ir.clone(), func_map)?;
        Ok(ir)
    }

    pub(crate) fn rateExpression(mut exp: Arc<Expression::NFExpression>, mut func_map: Arc<UnorderedMap::UnorderedMap<Arc<Function::Function>, Arc<InlineRating>>>, mut local_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<InlineRating>>>, mut irp: Pointer::Pointer<Arc<InlineRating>>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut cont: bool = false;
        if Expression::isLiteral(exp.clone())? {
            Pointer::update(irp.clone(), addConst(Pointer::access(irp)));
        } else {
            cont = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { .. } if (functionInlineable(Call::typedFunction(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?)?) => {
            let mut r#fn: Arc<Function::Function>;
            let mut lir: Option<Arc<InlineRating>>;
            r#fn = Call::typedFunction(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?;
            lir = UnorderedMap::get(r#fn.clone(), func_map.clone())?;
            if isSome(lir.clone()) {
                Pointer::update(irp.clone(), addMapped(Pointer::access(irp.clone()), Util::getOption(lir.clone())?, metamodelica::arrayFromVec(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?.into_iter().cloned().collect()), local_map.clone())?);
                cont = false;
            } else if DAEUtil::inlineTypeEqual(Function::inlineBuiltin(r#fn.clone()), openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE) {
                Pointer::update(irp.clone(), addMapped(Pointer::access(irp.clone()), fromFunction(r#fn.clone(), func_map.clone())?, metamodelica::arrayFromVec(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL).clone())?.into_iter().cloned().collect()), local_map.clone())?);
                cont = false;
            } else {
                cont = true;
            }
            cont
        },
        Deref @ Expression::CREF { .. } => {
            let mut lir: Option<Arc<InlineRating>> = None;
            (::match_deref::match_deref! { match &(UnorderedMap::get(ComponentRef::stripSubscriptsAll(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()), local_map.clone())?) {
        __esc_lir @ Some(_) => {
            lir = (*__esc_lir).clone();
            Pointer::update(irp.clone(), add(Pointer::access(irp.clone()), Util::getOption(lir.clone())?)?);
            false
        },
        _ => {
            Pointer::update(irp.clone(), addConst(Pointer::access(irp.clone())));
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if cont {
                exp = Expression::mapShallow(exp, (std::sync::Arc::new({ let __pe_b1 = func_map; let __pe_b2 = local_map; let __pe_b3 = irp; move |__pe_a0| rateExpression(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            }
        }
        Ok(exp)
    }

}


