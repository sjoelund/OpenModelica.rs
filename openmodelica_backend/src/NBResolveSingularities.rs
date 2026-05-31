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

use crate::NBAdjacency as Adjacency;
use crate::NBBackendUtil as BackendUtil;
use crate::NBDifferentiate as Differentiate;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBEquation::SlicingStatus;
use crate::NBFunctionAlias::Call_Aux;
use crate::NBInitialization as Initialization;
use crate::NBMatching as Matching;
use crate::NBModule as Module;
use crate::NBPartition;
use crate::NBSlice as Slice;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use openmodelica_ast::Absyn;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::StateSelect;
use openmodelica_nf_frontend::NFBackendExtension::VariableAttributes;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// NF imports
// NB imports
// util imports
pub fn indexReduction(mut adj: Arc<Adjacency::Matrix::Matrix>, mut full: Arc<Adjacency::Matrix::Matrix>, mut variables: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>, mut kind: NBPartition::Kind, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<NFFunction::Function::Function>>>, mut matching: Arc<Matching::NBMatching>, mut mapping_opt: Option<Arc<Adjacency::Mapping::Mapping>>) -> Result<(Arc<Adjacency::Matrix::Matrix>, Arc<Adjacency::Matrix::Matrix>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VarData::VarData>, Arc<EqData::EqData>, bool)> {
    pub type SliceSet = Arc<UnorderedSet::UnorderedSet<i32>>;

    let mut adj: Arc<Adjacency::Matrix::Matrix> = adj;
    let mut full: Arc<Adjacency::Matrix::Matrix> = full;
    let mut variables: Arc<VariablePointers::VariablePointers> = variables;
    let mut equations: Arc<EquationPointers::EquationPointers> = equations;
    let mut varData: Arc<VarData::VarData> = varData;
    let mut eqData: Arc<EqData::EqData> = eqData;
    let mut changed: bool = false;
    let mut mapping: Arc<Adjacency::Mapping::Mapping> = Arc::new(<Adjacency::Mapping::Mapping as ::std::default::Default>::default());
    let mut excluded_eqns: metamodelica::Array<bool> = Default::default();
    let mut msss: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut marked_eqns: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut constraint: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut diffed_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut states: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
    let mut dummy_states: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
    let mut sliced_dummies: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
    let mut sliced_states: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut sliced_dummy_states: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut state_derivatives: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut dummy_derivatives: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut dummy_slice_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut current_candidates: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut rest_candidates: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut constraint_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
    let mut matched_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
    let mut unmatched_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
    let mut new_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut diffArguments: Arc<Differentiate::DifferentiationArguments::DifferentiationArguments> = Arc::new(<Differentiate::DifferentiationArguments::DifferentiationArguments as ::std::default::Default>::default());
    let mut diffArguments_ptr: Pointer::Pointer<Arc<Differentiate::DifferentiationArguments::DifferentiationArguments>>;
    let mut candidate_ptrs: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut constraint_ptrs: Arc<EquationPointers::EquationPointers> = Arc::new(<EquationPointers::EquationPointers as ::std::default::Default>::default());
    let mut set_adj: Arc<Adjacency::Matrix::Matrix> = Arc::new(<Adjacency::Matrix::Matrix as ::std::default::Default>::default());
    let mut full_local: Arc<Adjacency::Matrix::Matrix> = Arc::new(<Adjacency::Matrix::Matrix as ::std::default::Default>::default());
    let mut set_matching: Arc<Matching::NBMatching> = Arc::new(<Matching::NBMatching as ::std::default::Default>::default());
    let mut vo: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> = <Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> as ::std::default::Default>::default();
    let mut vn: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> = <Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> as ::std::default::Default>::default();
    let mut eo: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> = <Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> as ::std::default::Default>::default();
    let mut en: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> = <Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> as ::std::default::Default>::default();
    let mut stages: Arc<metamodelica::List<(ArcStr, BVariable::checkVar)>> = metamodelica::nil();
    let mut stageFunc: BVariable::checkVar;
    let mut stageStr: ArcStr = arcstr::literal!("");
    let mut slice_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<i32>>>> = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut dummy_slice_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    let mut debug: bool = false;
    mapping = (::match_deref::match_deref! { match &(mapping_opt.clone()) {
        Some(mapping) => mapping.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBResolveSingularities.indexReduction")); __mm_s.push_str(&*literal!(" failed because no mapping was provided.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    excluded_eqns = metamodelica::arrayFromVec(({
        let mut __acc: Arc<metamodelica::List<bool>> = metamodelica::nil();
        for mut eqn in (EquationPointers::toList(equations.clone())?).into_iter().cloned() {
            let __x = Equation::isDiscrete(eqn.clone()) || Equation::hasDerivative(eqn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect());
    msss = (::match_deref::match_deref! { match &(adj.clone()) {
        Deref @ Adjacency::Matrix::FINAL { .. } => getMSSS(var_field!((*adj).m, Adjacency::Matrix::Matrix::FINAL).clone(), var_field!((*adj).mT, Adjacency::Matrix::Matrix::FINAL).clone(), matching.clone(), excluded_eqns.clone(), mapping.clone())?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBResolveSingularities.indexReduction")); __mm_s.push_str(&*literal!(" expected final matrix as adj input but got :\n")); __mm_s.push_str(&*Adjacency::Matrix::toString(adj.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !((msss.clone().borrow().len() as i32) == 0) {
        changed = true;
        marked_eqns = UnorderedSet::unique_list(List::flatten(Arc::new(msss.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())), std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>));
        (constraint_ptrs, candidate_ptrs, constraint_eqns) = getConstraintsAndCandidates(equations.clone(), marked_eqns.clone(), mapping.clone())?;
        for mut eq in &*constraint_eqns.clone() {
            let mut eq = eq.clone();
            UnorderedMap::add(Equation::getEqnName(Slice::getT(eq.clone()))?, UnorderedSet::fromList(eq.indices.clone(), std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?, slice_map.clone())?;
        }
        if BVariable::VariablePointers::scalarSize(candidate_ptrs.clone(), false) < ({
        let mut __acc: i32 = 0;
        for mut eq in (constraint_eqns.clone()).into_iter().cloned() {
            let __x = Slice::size(eq.clone(), (std::sync::Arc::new({ let __pe_b1 = true; move |__pe_a0| Equation::size(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<i32> + 'static>));
            __acc += __x;
        }
        __acc
    }) {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBResolveSingularities.indexReduction")); __mm_s.push_str(&*literal!(" failed because there was not enough state candidates to balance out the constraint equations.\n")); __mm_s.push_str(&*EquationPointers::toString(constraint_ptrs.clone(), (literal!("Constraint")).clone(), None, true, None)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*BVariable::VariablePointers::toString(candidate_ptrs.clone(), (literal!("State Candidate")).clone(), None, true)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1((literal!("Index Reduction")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*BVariable::VariablePointers::toString(candidate_ptrs.clone(), (literal!("State Candidate")).clone(), None, true)?); __mm_s.push_str(&*EquationPointers::toString(constraint_ptrs.clone(), (literal!("Constraint")).clone(), None, true, None)?); ArcStr::from(__mm_s) }).clone());
        }
        full_local = Adjacency::Matrix::createFull(candidate_ptrs.clone(), constraint_ptrs.clone(), kind.clone())?;
        set_adj = Arc::new(Adjacency::Matrix::Matrix::EMPTY { st: Adjacency::MatrixStrictness::LINEAR.clone() });
        rest_candidates = BVariable::VariablePointers::toList(candidate_ptrs.clone())?;
        eo = constraint_ptrs.map.clone();
        en = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        vo = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        vn = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        set_matching = Matching::EMPTY_MATCHING().clone();
        stages = list![(literal!("1. StateSelect.NEVER"), (std::sync::Arc::new({ let __pe_b1 = StateSelect::NEVER.clone(); move |__pe_a0| Ok(BVariable::isStateSelect(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>)), (literal!("2. StateSelect.AVOID"), (std::sync::Arc::new({ let __pe_b1 = StateSelect::AVOID.clone(); move |__pe_a0| Ok(BVariable::isStateSelect(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>)), (literal!("3. Artificial Variables"), (std::sync::Arc::new(BVariable::isArtificial) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>)), (literal!("4. StateSelect.DEFAULT"), (std::sync::Arc::new({ let __pe_b1 = StateSelect::DEFAULT.clone(); move |__pe_a0| Ok(BVariable::isStateSelect(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>)), (literal!("5. StateSelect.PREFER"), (std::sync::Arc::new({ let __pe_b1 = StateSelect::PREFER.clone(); move |__pe_a0| Ok(BVariable::isStateSelect(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))];
        for mut stage in &*stages.clone() {
            let mut stage = stage.clone();
            (stageStr, stageFunc) = stage.clone();
            (current_candidates, rest_candidates) = List::splitOnTrue(rest_candidates.clone(), stageFunc.clone());
            if current_candidates.clone().is_empty() {
                if debug.clone() {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Nothing done for (")); __mm_s.push_str(&*stageStr.clone()); __mm_s.push_str(&*literal!(") Index Reduction")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
            } else {
                vo = UnorderedMap::merge(vo.clone(), UnorderedMap::copy(vn.clone()), metamodelica::sourceInfo!())?;
                vn = UnorderedMap::subMap(candidate_ptrs.map.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut var in (current_candidates.clone()).into_iter().cloned() {
            let __x = BVariable::getVarName(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
                (set_adj, full_local) = Adjacency::Matrix::expand(set_adj.clone(), full_local.clone(), vo.clone(), vn.clone(), eo.clone(), en.clone(), candidate_ptrs.clone(), constraint_ptrs.clone(), kind.clone())?;
                set_matching = Matching::regular(set_matching.clone(), set_adj.clone(), false, true, false)?;
                if debug.clone() {
                    println!("{}", (Adjacency::Matrix::toString(set_adj.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stageStr.clone()); __mm_s.push_str(&*literal!(") Index Reduction")); ArcStr::from(__mm_s) }).clone())?).clone());
                    println!("{}", (Matching::toString(set_matching.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stageStr.clone()); __mm_s.push_str(&*literal!(") Index Reduction")); ArcStr::from(__mm_s) }).clone())).clone());
                }
                if Matching::isEmpty(set_matching.clone()) && Matching::isPerfect(set_matching.clone()) {
                    if debug.clone() {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Finished with perfect matching in stage ")); __mm_s.push_str(&*stageStr.clone()); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    break;
                }
            }
        }
        (dummy_states, states, matched_eqns, unmatched_eqns) = Matching::getMatches(set_matching.clone(), Adjacency::Matrix::getMappingOpt(set_adj.clone()), candidate_ptrs.clone(), constraint_ptrs.clone())?;
        unmatched_eqns = resolveSlicedUnmatched(unmatched_eqns.clone(), slice_map.clone())?;
        diffArguments = Differentiate::DifferentiationArguments::default(Differentiate::DifferentiationType::TIME.clone(), funcMap.clone());
        assign_field!(diffArguments.diff_map = Some(BVariable::VarData::getStateOrder(varData.clone())?));
        diffArguments_ptr = Pointer::create(diffArguments.clone());
        if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
            println!("{}", (StringUtil::headline_3((literal!("[dummyselect] 1. Differentiate the constraint equations")).clone())).clone());
        }
        for mut constraint in &*EquationPointers::toList(constraint_ptrs.clone())? {
            let mut constraint = constraint.clone();
            diffed_eqn = Differentiate::differentiateEquationPointer(constraint.clone(), diffArguments_ptr.clone(), (literal!("")).clone())?;
            diffed_eqn = removeSlicedDerivatives(diffed_eqn.clone(), UnorderedMap::getSafe(Equation::getEqnName(constraint.clone())?, slice_map.clone(), metamodelica::sourceInfo!())?, dummy_slice_set.clone(), BVariable::VarData::getUniqueIndex(varData.clone())?)?;
            new_eqns = metamodelica::cons(diffed_eqn.clone(), new_eqns.clone());
            if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dummyselect] constraint eqn:\t\t")); __mm_s.push_str(&*Equation::toString(Pointer::access(constraint.clone()), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dummyselect] differentiated eqn:\t")); __mm_s.push_str(&*Equation::toString(Pointer::access(diffed_eqn.clone()), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
        }
        diffArguments = Pointer::access(diffArguments_ptr.clone());
        for mut dummy in &*dummy_states.clone() {
            let mut dummy = dummy.clone();
            if dummy.indices.clone().is_empty() {
                dummy_derivatives = metamodelica::cons(BVariable::makeDummyState(Slice::getT(dummy.clone()))?, dummy_derivatives.clone());
            } else {
                sliced_dummies = metamodelica::cons(dummy.clone(), sliced_dummies.clone());
            }
        }
        if !(sliced_dummies.clone().is_empty()) {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBResolveSingularities.indexReduction")); __mm_s.push_str(&*literal!(" failed because slicing during index reduction is not yet supported.\n")); __mm_s.push_str(&*List::toString(sliced_dummies.clone(), (std::sync::Arc::new({ let __pe_b1 = (std::sync::Arc::new(fnptr!(BVariable::pointerToString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>); let __pe_b2 = 10; move |__pe_a0| Slice::toString(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<ArcStr> + 'static>), (literal!("Sliced Dummies:")).clone(), (literal!("\n  ")).clone(), (literal!("\n  ")).clone(), (literal!("\n")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
            println!("{}", (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dummyselect] (")); __mm_s.push_str(&*intString((states.clone().len() as i32))); __mm_s.push_str(&*literal!(") Selected States")); ArcStr::from(__mm_s) }).clone())).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Slice::lstToString(states.clone(), (std::sync::Arc::new(fnptr!(BVariable::pointerToString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), 10)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        }
        if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? {
            println!("{}", (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[stateselection] (")); __mm_s.push_str(&*intString((diffArguments.new_vars.clone().len() as i32))); __mm_s.push_str(&*literal!(") State Derivatives Created by Differentiation")); ArcStr::from(__mm_s) }).clone())).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(diffArguments.new_vars.clone(), (std::sync::Arc::new(fnptr!(BVariable::pointerToString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("\t")).clone(), (literal!("\n\t")).clone(), (literal!("")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[stateselection] (")); __mm_s.push_str(&*intString((dummy_states.clone().len() as i32))); __mm_s.push_str(&*literal!(") Selected Dummy States")); ArcStr::from(__mm_s) }).clone())).clone());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Slice::lstToString(dummy_states.clone(), (std::sync::Arc::new(fnptr!(BVariable::pointerToString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), 10)); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
        }
        if unmatched_eqns.clone().is_empty() {
            if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((literal!("\t STATIC STATE SELECTION\n\t(no unmatched equations)")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
        } else {
            if Flags::isSet(Flags::DUMMY_SELECT.clone())? {
                println!("{}", (toStringDynamicSelect(dummy_states.clone(), unmatched_eqns.clone())).clone());
            }
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBResolveSingularities.indexReduction")); __mm_s.push_str(&*literal!(" failed because dynamic state selection is not yet supported.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail");
        }
        (state_derivatives, _) = List::extractOnTrue(diffArguments.new_vars.clone(), (std::sync::Arc::new(fnptr!(BVariable::isStateDerivative, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>));
        sliced_states = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut slice in (states.clone()).into_iter().cloned() {
            let __x = Slice::getT(slice.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        varData = BVariable::VarData::addTypedList(varData.clone(), sliced_states.clone(), BVariable::VarData::VarType::STATE.clone())?;
        varData = BVariable::VarData::addTypedList(varData.clone(), state_derivatives.clone(), BVariable::VarData::VarType::STATE_DER.clone())?;
        sliced_dummy_states = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut slice in (dummy_states.clone()).into_iter().cloned() {
            let __x = Slice::getT(slice.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        varData = BVariable::VarData::addTypedList(varData.clone(), sliced_dummy_states.clone(), BVariable::VarData::VarType::ALGEBRAIC.clone())?;
        varData = BVariable::VarData::addTypedList(varData.clone(), dummy_derivatives.clone(), BVariable::VarData::VarType::ALGEBRAIC.clone())?;
        eqData = EqData::addTypedList(eqData.clone(), new_eqns.clone(), EqData::EqType::CONTINUOUS.clone(), true)?;
        variables = BVariable::VariablePointers::addList(diffArguments.new_vars.clone(), variables.clone());
        variables = BVariable::VariablePointers::addList(sliced_dummy_states.clone(), variables.clone());
        variables = BVariable::VariablePointers::removeList(sliced_states.clone(), variables.clone())?;
        equations = EquationPointers::addList(new_eqns.clone(), equations.clone());
        dummy_slice_vars = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut cref in (UnorderedSet::toList(dummy_slice_set.clone())).into_iter().cloned() {
            let __x = BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        varData = BVariable::VarData::addTypedList(varData.clone(), dummy_slice_vars.clone(), BVariable::VarData::VarType::ALGEBRAIC.clone())?;
        variables = BVariable::VariablePointers::addList(dummy_slice_vars.clone(), variables.clone());
    } else {
        changed = false;
    }
    Ok((adj, full, variables, equations, varData, eqData, changed))
}

pub fn balanceInitialization(mut adj: Arc<Adjacency::Matrix::Matrix>, mut full: Arc<Adjacency::Matrix::Matrix>, mut variables: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>, mut kind: NBPartition::Kind, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Absyn::Path>, Arc<NFFunction::Function::Function>>>, mut matching: Arc<Matching::NBMatching>, mut mapping_opt: Option<Arc<Adjacency::Mapping::Mapping>>) -> Result<(Arc<Adjacency::Matrix::Matrix>, Arc<Adjacency::Matrix::Matrix>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VarData::VarData>, Arc<EqData::EqData>, bool)> {
    let mut adj: Arc<Adjacency::Matrix::Matrix> = adj;
    let mut full: Arc<Adjacency::Matrix::Matrix> = full;
    let mut variables: Arc<VariablePointers::VariablePointers> = variables;
    let mut equations: Arc<EquationPointers::EquationPointers> = equations;
    let mut varData: Arc<VarData::VarData> = varData;
    let mut eqData: Arc<EqData::EqData> = eqData;
    let mut changed: bool = false;
    let mut unmatched_vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
    let mut unmatched_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
    let mut start_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut failed_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut sliced_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut start_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut ptr_start_vars: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    let mut ptr_start_eqns: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> = Pointer::create(metamodelica::nil());
    let mut idx: Pointer::Pointer<i32>;
    let mut error_msg: ArcStr = arcstr::literal!("");
    let mut vo: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> = <Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> as ::std::default::Default>::default();
    let mut vn: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> = <Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> as ::std::default::Default>::default();
    let mut eo: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> = <Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> as ::std::default::Default>::default();
    let mut en: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> = <Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> as ::std::default::Default>::default();
    (_, unmatched_vars, _, unmatched_eqns) = Matching::getMatches(matching.clone(), mapping_opt.clone(), variables.clone(), equations.clone())?;
    if Flags::isSet(Flags::INITIALIZATION.clone())? {
        println!("{}", (toStringUnmatched(unmatched_vars.clone(), unmatched_eqns.clone())).clone());
    }
    if !(unmatched_vars.clone().is_empty() && unmatched_eqns.clone().is_empty()) {
        changed = true;
        if !(unmatched_eqns.clone().is_empty()) {
            Error::addMessage(Error::COMPILER_WARNING.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBResolveSingularities.balanceInitialization")); __mm_s.push_str(&*literal!(" reports an overdetermined initialization!\nChecking for consistency is not yet supported, following equations had to be removed:\n")); __mm_s.push_str(&*Slice::lstToString(unmatched_eqns.clone(), (std::sync::Arc::new({ let __pe_b1 = (literal!("")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<ArcStr> + 'static>), 10)); ArcStr::from(__mm_s) }).clone()])?;
            eo = UnorderedMap::copy(equations.map.clone());
            sliced_eqns = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut eqn in (unmatched_eqns.clone()).into_iter().cloned() {
            let __x = Slice::getT(eqn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            equations = EquationPointers::removeList(sliced_eqns.clone(), equations.clone())?;
            (adj, full) = Adjacency::Matrix::compress(adj.clone(), full.clone(), equations.clone(), variables.clone(), eo.clone())?;
        }
        idx = EqData::getUniqueIndex(eqData.clone())?;
        for mut var in &*unmatched_vars.clone() {
            let mut var = var.clone();
            var_ptr = Slice::getT(var.clone());
            if BVariable::isFixable(var_ptr.clone()) {
                Initialization::createStartEquationSlice(var.clone(), ptr_start_vars.clone(), ptr_start_eqns.clone(), idx.clone(), true)?;
            } else {
                failed_vars = metamodelica::cons(var_ptr.clone(), failed_vars.clone());
            }
        }
        if failed_vars.clone().is_empty() {
            start_vars = Pointer::access(ptr_start_vars.clone());
            start_eqns = Pointer::access(ptr_start_eqns.clone());
            vo = variables.map.clone();
            eo = UnorderedMap::copy(equations.map.clone());
            varData = BVariable::VarData::addTypedList(varData.clone(), start_vars.clone(), VarData::VarType::START.clone())?;
            eqData = EqData::addTypedList(eqData.clone(), start_eqns.clone(), EqData::EqType::INITIAL.clone(), true)?;
            equations = EquationPointers::addList(start_eqns.clone(), equations.clone());
            vn = UnorderedMap::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            en = UnorderedMap::subMap(equations.map.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut eqn in (start_eqns.clone()).into_iter().cloned() {
            let __x = Equation::getEqnName(eqn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
            (adj, full) = Adjacency::Matrix::expand(adj.clone(), full.clone(), vo.clone(), vn.clone(), eo.clone(), en.clone(), variables.clone(), equations.clone(), kind.clone())?;
            if Flags::isSet(Flags::INITIALIZATION.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(start_eqns.clone(), (std::sync::Arc::new({ let __pe_b1 = (literal!("")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<ArcStr> + 'static>), (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Created Start Equations for balancing the Initialization (")); __mm_s.push_str(&*intString((start_eqns.clone().len() as i32))); __mm_s.push_str(&*literal!("):")); ArcStr::from(__mm_s) }).clone())).clone(), (literal!("\t")).clone(), (literal!("\n\t")).clone(), (literal!("")).clone(), false, 0)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
        } else {
            error_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBResolveSingularities.balanceInitialization")); __mm_s.push_str(&*literal!(" failed because following non-fixable variables could not be solved:\n")); __mm_s.push_str(&*List::toString(failed_vars.clone(), (std::sync::Arc::new(fnptr!(BVariable::pointerToString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("\t")).clone(), (literal!("\n\t")).clone(), (literal!("\n")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
            if Flags::isSet(Flags::INITIALIZATION.clone())? {
                error_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*error_msg.clone()); __mm_s.push_str(&*literal!("\nFollowing equations were created by fixing variables:\n")); __mm_s.push_str(&*List::toString(Pointer::access(ptr_start_eqns.clone()), (std::sync::Arc::new({ let __pe_b1 = (literal!("\t")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("\n")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
            } else {
                error_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*error_msg.clone()); __mm_s.push_str(&*literal!("\nUse -d=initialization for more debug output.")); ArcStr::from(__mm_s) }).clone();
            }
            if Flags::isSet(Flags::BLT_DUMP.clone())? {
                error_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*error_msg.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*BVariable::VariablePointers::toString(variables.clone(), (literal!("All")).clone(), None, true)?); __mm_s.push_str(&*EquationPointers::toString(equations.clone(), (literal!("All")).clone(), None, true, None)?); __mm_s.push_str(&*Adjacency::Mapping::toString(Util::getOptionOrDefault(mapping_opt.clone(), Adjacency::Mapping::empty()))); __mm_s.push_str(&*Adjacency::Matrix::toString(adj.clone(), (literal!("")).clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*Matching::toString(matching.clone(), (literal!("")).clone())); ArcStr::from(__mm_s) }).clone();
            } else {
                error_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*error_msg.clone()); __mm_s.push_str(&*literal!("\nUse -d=bltdump for more verbose debug output.")); ArcStr::from(__mm_s) }).clone();
            }
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(error_msg.clone()).clone()])?;
            bail!("fail");
        }
    } else {
        changed = false;
    }
    Ok((adj, full, variables, equations, varData, eqData, changed))
}

fn getMSSS(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut matching: Arc<Matching::NBMatching>, mut excluded_eqns: metamodelica::Array<bool>, mut mapping: Arc<Adjacency::Mapping::Mapping>) -> Result<metamodelica::Array<Arc<metamodelica::List<i32>>>> {
    let mut msss: metamodelica::Array<Arc<metamodelica::List<i32>>> = Default::default();
    let mut eqn_candidates: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut color_clustering: metamodelica::Array<i32> = Default::default();
    let mut eqn_coloring: metamodelica::Array<i32> = arrayCreate((m.clone().borrow().len() as i32), -1);
    let mut var_coloring: metamodelica::Array<i32> = arrayCreate((mT.clone().borrow().len() as i32), -1);
    let mut color: i32 = 0;
    let __range0 = 1..=(matching.eqn_to_var.clone().borrow().len() as i32);
    for mut eqn in __range0 {
        if matching.eqn_to_var.borrow()[(eqn.clone()-1) as usize].clone() == -1 {
            eqn_candidates = metamodelica::cons(eqn.clone(), eqn_candidates.clone());
        }
    }
    color_clustering = metamodelica::arrayFromVec(({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut i in (1..=(eqn_candidates.clone().len() as i32)).into_iter() {
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect());
    for mut eqn in &*eqn_candidates.clone() {
        let mut eqn = eqn.clone();
        if eqn_coloring.borrow()[(eqn.clone()-1) as usize].clone() == -1 {
            color = color.clone() + 1;
            fillColorEqn(eqn.clone(), color.clone(), eqn_coloring.clone(), var_coloring.clone(), color_clustering.clone(), m.clone(), mT.clone(), matching.clone(), mapping.clone())?;
        }
    }
    resolveClustering(color_clustering.clone())?;
    msss = arrayCreate(color.clone(), metamodelica::nil());
    let __range1 = 1..=(eqn_coloring.clone().borrow().len() as i32);
    for mut eqn in __range1 {
        if eqn_coloring.borrow()[(eqn.clone()-1) as usize].clone() != -1 && !(excluded_eqns.borrow()[(mapping.eqn_StA.borrow()[(eqn.clone()-1) as usize].clone()-1) as usize].clone()) {
            color = color_clustering.borrow()[(eqn_coloring.borrow()[(eqn.clone()-1) as usize].clone()-1) as usize].clone();
            {
                let __cell2 = metamodelica::cons(eqn.clone(), msss.borrow()[(color.clone()-1) as usize].clone());
                msss.clone().borrow_mut()[(color.clone()-1) as usize] = __cell2;
            }
        }
    }
    msss = metamodelica::arrayFromVec(({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        for mut ms in (Arc::new(msss.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())).into_iter().cloned() {
            if !(!(ms.clone().is_empty())) { continue; }
            let __x = ms.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect());
    Ok(msss)
}

fn fillColorEqn(mut eqn: i32, mut color: i32, mut eqn_coloring: metamodelica::Array<i32>, mut var_coloring: metamodelica::Array<i32>, mut color_clustering: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut matching: Arc<Matching::NBMatching>, mut mapping: Arc<Adjacency::Mapping::Mapping>) -> Result<()> {
    {let _arr = eqn_coloring.clone(); _arr.borrow_mut()[(eqn.clone()-1) as usize] = color.clone(); _arr};
    let __range0 = &*m.borrow()[(eqn.clone()-1) as usize].clone();
    for mut var in __range0 {
        let mut var = var.clone();
        fillColorVar(var.clone(), color.clone(), eqn_coloring.clone(), var_coloring.clone(), color_clustering.clone(), m.clone(), mT.clone(), matching.clone(), mapping.clone())?;
    }
    Ok(())
}

fn fillColorVar(mut var: i32, mut color: i32, mut eqn_coloring: metamodelica::Array<i32>, mut var_coloring: metamodelica::Array<i32>, mut color_clustering: metamodelica::Array<i32>, mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut mT: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut matching: Arc<Matching::NBMatching>, mut mapping: Arc<Adjacency::Mapping::Mapping>) -> Result<()> {
    let mut eqn: i32 = matching.var_to_eqn.borrow()[(var.clone()-1) as usize].clone();
    if var_coloring.borrow()[(var.clone()-1) as usize].clone() == -1 {
        {let _arr = var_coloring.clone(); _arr.borrow_mut()[(var.clone()-1) as usize] = color.clone(); _arr};
        if eqn.clone() != -1 {
            if eqn_coloring.borrow()[(eqn.clone()-1) as usize].clone() == -1 {
                fillColorEqn(eqn.clone(), color.clone(), eqn_coloring.clone(), var_coloring.clone(), color_clustering.clone(), m.clone(), mT.clone(), matching.clone(), mapping.clone())?;
            }
        }
    } else {
        colorClustering(var_coloring.borrow()[(var.clone()-1) as usize].clone(), color.clone(), color_clustering.clone())?;
    }
    Ok(())
}

fn colorClustering(mut old_color: i32, mut new_color: i32, mut color_clustering: metamodelica::Array<i32>) -> Result<()> {
    if color_clustering.borrow()[(old_color.clone()-1) as usize].clone() != old_color.clone() {
        colorClustering(color_clustering.borrow()[(old_color.clone()-1) as usize].clone(), new_color.clone(), color_clustering.clone())?;
    }
    {let _arr = color_clustering.clone(); _arr.borrow_mut()[(old_color.clone()-1) as usize] = new_color.clone(); _arr};
    Ok(())
}

fn resolveClustering(mut color_clustering: metamodelica::Array<i32>) -> Result<()> {
    let mut color: i32 = 0;
    let __range0 = 1..=(color_clustering.clone().borrow().len() as i32);
    for mut i in __range0 {
        color = i.clone();
        while color_clustering.borrow()[(color.clone()-1) as usize].clone() != color.clone() {
            color = color_clustering.borrow()[(color.clone()-1) as usize].clone();
        }
        {let _arr = color_clustering.clone(); _arr.borrow_mut()[(i.clone()-1) as usize] = color.clone(); _arr};
    }
    Ok(())
}

fn getConstraintsAndCandidates(mut equations: Arc<EquationPointers::EquationPointers>, mut marked_eqns: Arc<metamodelica::List<i32>>, mut mapping: Arc<Adjacency::Mapping::Mapping>) -> Result<(Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>)> {
    let mut constr: Arc<EquationPointers::EquationPointers> = EquationPointers::empty(BaseHashTable::bigBucketSize.clone());
    let mut states: Arc<VariablePointers::VariablePointers> = BVariable::VariablePointers::empty(BaseHashTable::bigBucketSize.clone(), false);
    let mut sliced_constr: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
    let mut eqn_indices: Arc<UnorderedSet::UnorderedSet<i32>> = UnorderedSet::new(std::sync::Arc::new(fnptr!(Util::id, _)), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 13);
    let mut eqn_slices: metamodelica::Array<Arc<metamodelica::List<i32>>> = arrayCreate(EquationPointers::size(equations.clone()), metamodelica::nil());
    let mut state_candidates: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(ComponentRef::hash, Arc<ComponentRef::NFComponentRef>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    let mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    for mut eqn in &*marked_eqns.clone() {
        let mut eqn = eqn.clone();
        UnorderedSet::add(mapping.eqn_StA.borrow()[(eqn.clone()-1) as usize].clone(), eqn_indices.clone())?;
        {
            let __cell0 = metamodelica::cons(eqn.clone(), eqn_slices.borrow()[(mapping.eqn_StA.borrow()[(eqn.clone()-1) as usize].clone()-1) as usize].clone());
            eqn_slices.clone().borrow_mut()[(mapping.eqn_StA.borrow()[(eqn.clone()-1) as usize].clone()-1) as usize] = __cell0;
        }
    }
    for mut eqn in &*UnorderedSet::toList(eqn_indices.clone()) {
        let mut eqn = eqn.clone();
        eqn_ptr = EquationPointers::getEqnAt(equations.clone(), eqn.clone())?;
        constr = EquationPointers::add(eqn_ptr.clone(), constr.clone())?;
        sliced_constr = metamodelica::cons(Arc::new(Slice::NBSlice { t: eqn_ptr.clone(), indices: eqn_slices.borrow()[(eqn.clone()-1) as usize].clone() }), sliced_constr.clone());
        for mut candidate in &*Equation::collectCrefs(Pointer::access(eqn_ptr.clone()), (std::sync::Arc::new(getStateCandidate) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<ComponentRef::NFComponentRef>> + 'static>), (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))? {
            let mut candidate = candidate.clone();
            UnorderedSet::add(candidate.clone(), state_candidates.clone())?;
        }
    }
    for mut candidate in &*UnorderedSet::toList(state_candidates.clone()) {
        let mut candidate = candidate.clone();
        var_ptr = BVariable::getVarPointer(candidate.clone(), metamodelica::sourceInfo!())?;
        states = BVariable::VariablePointers::add(var_ptr.clone(), states.clone())?;
    }
    Ok((constr, states, sliced_constr))
}

fn getStateCandidate(mut cref: Arc<ComponentRef::NFComponentRef>, mut acc: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    fn getStateCandidateVar(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut acc: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
        if BVariable::isContinuous(var.clone(), false)? && !(BVariable::isTime(var.clone()) || BVariable::isDummyVariable(var.clone()) || BVariable::isDummyState(var.clone()) || BVariable::isForcedState(var.clone())) {
            UnorderedSet::add(BVariable::getVarName(var.clone()), acc.clone())?;
        }
        Ok(())
    }

    let mut cref: Arc<ComponentRef::NFComponentRef> = cref;
    let mut var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    var = BVariable::getVarPointer(cref.clone(), metamodelica::sourceInfo!())?;
    if BVariable::isRecord(var.clone()) {
        for mut child in &*BVariable::getRecordChildren(var.clone()) {
            let mut child = child.clone();
            getStateCandidateVar(child.clone(), acc.clone())?;
        }
    } else {
        getStateCandidateVar(var.clone(), acc.clone())?;
    }
    Ok(cref)
}

fn candidatePriority(mut candidate: Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> {
    let mut prio: i32 = 0;
    prio = (::match_deref::match_deref! { match &(Pointer::access(candidate.clone())) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { attributes, .. }, .. } => {
            (match VariableAttributes::getStateSelect(attributes.clone()) {
        StateSelect::NEVER => -200,
        StateSelect::AVOID => -100,
        StateSelect::DEFAULT => 0,
        StateSelect::PREFER => 100,
        StateSelect::ALWAYS => 200,
        _ => 0,
    })
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(prio)
}

fn sortCandidates(mut candidates: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
    let mut candidates: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = candidates;
    let mut priorities: Arc<metamodelica::List<(i32, Pointer::Pointer<Arc<Variable::NFVariable>>)>> = metamodelica::nil();
    for mut candidate in &*candidates.clone() {
        let mut candidate = candidate.clone();
        priorities = metamodelica::cons((candidatePriority(candidate.clone())?, candidate.clone()), priorities.clone());
    }
    priorities = List::sort(priorities.clone(), std::sync::Arc::new(fnptr!(BackendUtil::indexTplGt, _, _)))?;
    candidates = List::unzipSecond(priorities.clone());
    Ok(candidates)
}

fn resolveSlicedUnmatched(mut old_unmatched: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>, mut slice_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<i32>>>>) -> Result<Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>> {
    pub fn resolveSlicedUnmatchedSingle(mut eq: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>, mut acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>, mut slice_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<UnorderedSet::UnorderedSet<i32>>>>) -> Result<Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>> {
        let mut acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = acc;
        let mut relevant_indices: Arc<UnorderedSet::UnorderedSet<i32>> = <Arc<UnorderedSet::UnorderedSet<i32>> as ::std::default::Default>::default();
        relevant_indices = UnorderedMap::getSafe(Equation::getEqnName(Slice::getT(eq.clone()))?, slice_map.clone(), metamodelica::sourceInfo!())?;
        if UnorderedSet::isEmpty(relevant_indices.clone()) {
            acc = metamodelica::cons(eq.clone(), acc.clone());
        } else {
            assign_field!(eq.indices = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut ind in (eq.indices.clone()).into_iter().cloned() {
            if !(UnorderedSet::contains(ind.clone(), relevant_indices.clone())?) { continue; }
            let __x = ind.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            if !(eq.indices.clone().is_empty()) {
                acc = metamodelica::cons(eq.clone(), acc.clone());
            }
        }
        Ok(acc)
    }

    let mut filtered_unmatched: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
    for mut eq in &*old_unmatched.clone() {
        let mut eq = eq.clone();
        filtered_unmatched = resolveSlicedUnmatchedSingle(eq.clone(), filtered_unmatched.clone(), slice_map.clone())?;
    }
    Ok(filtered_unmatched)
}

fn removeSlicedDerivatives(mut derivative: Pointer::Pointer<Arc<Equation::Equation>>, mut slice_set: Arc<UnorderedSet::UnorderedSet<i32>>, mut dummy_slice_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut aux_index: Pointer::Pointer<i32>) -> Result<Pointer::Pointer<Arc<Equation::Equation>>> {
    let mut derivative: Pointer::Pointer<Arc<Equation::Equation>> = derivative;
    let mut eqn: Arc<Equation::Equation> = Arc::new(Equation::DUMMY_EQUATION);
    if !(UnorderedSet::isEmpty(slice_set.clone())) {
        eqn = removeSlicedDerivateEqn(Pointer::access(derivative.clone()), Arc::new(crate::NBEquation::Iterator::EMPTY), dummy_slice_set.clone(), aux_index.clone())?;
        Pointer::update(derivative.clone(), eqn.clone());
    }
    Ok(derivative)
}

fn removeSlicedDerivateEqn(mut eqn: Arc<Equation::Equation>, mut iter: Arc<Iterator::Iterator>, mut dummy_slice_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut aux_index: Pointer::Pointer<i32>) -> Result<Arc<Equation::Equation>> {
    fn replaceTupleLiterals(mut exp: Arc<Expression::NFExpression>, mut iter: Arc<Iterator::Iterator>, mut dummy_slice_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut aux_index: Pointer::Pointer<i32>) -> Result<Arc<Expression::NFExpression>> {
        let mut exp: Arc<Expression::NFExpression> = exp;
        let mut aux: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        if Expression::isLiteral(exp.clone()) {
            aux = Call_Aux::createName(Expression::typeOf(exp.clone()), iter.clone(), aux_index.clone(), (arcstr::literal!(BVariable::DERIVATIVE_STR)).clone(), false)?;
            exp = Arc::new(Expression::NFExpression::CREF { ty: ComponentRef::getSubscriptedType(aux.clone(), false)?, cref: aux.clone() });
            UnorderedSet::add(aux.clone(), dummy_slice_set.clone())?;
        }
        Ok(exp)
    }

    let mut eqn: Arc<Equation::Equation> = eqn;
    eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ Equation::FOR_EQUATION { .. } => {
            assign_variant_field!(eqn => Equation::Equation::FOR_EQUATION; body = ({
        let mut __acc: Arc<metamodelica::List<Arc<Equation::Equation>>> = metamodelica::nil();
        for mut b in (var_field!((*eqn).body, Equation::Equation::FOR_EQUATION).clone()).into_iter().cloned() {
            let __x = removeSlicedDerivateEqn(b.clone(), var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION).clone(), dummy_slice_set.clone(), aux_index.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            eqn.clone()
        },
        Deref @ Equation::RECORD_EQUATION { lhs: lhs @ Deref @ Expression::TUPLE { .. }, .. } => {
            let mut lhs = (*lhs).clone();
            assign_variant_field!(lhs => Expression::NFExpression::TUPLE; elements = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (var_field!((*lhs).elements, Expression::NFExpression::TUPLE).clone()).borrow().iter() {
            let __x = replaceTupleLiterals(e.clone(), iter.clone(), dummy_slice_set.clone(), aux_index.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            assign_variant_field!(eqn => Equation::Equation::RECORD_EQUATION; lhs = lhs.clone());
            eqn.clone()
        },
        _ => {
            eqn.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

fn toStringCandidatesConstraints(mut state_candidates: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>, mut constraint_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1((literal!("Index Reduction")).clone())); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString((state_candidates.clone().len() as i32))); __mm_s.push_str(&*literal!(") Sorted State Candidates")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*Slice::lstToString(state_candidates.clone(), (std::sync::Arc::new(fnptr!(BVariable::pointerToString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), 10)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString((constraint_eqns.clone().len() as i32))); __mm_s.push_str(&*literal!(") Constraint Equations")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*Slice::lstToString(constraint_eqns.clone(), (std::sync::Arc::new({ let __pe_b1 = (literal!("")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<ArcStr> + 'static>), 10)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    r#str
}

fn toStringDynamicSelect(mut dummy_states: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>, mut unmatched_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2((literal!("\t  DYNAMIC STATE SELECTION\n\t(some unmatched equations)")).clone())); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString((dummy_states.clone().len() as i32))); __mm_s.push_str(&*literal!(") Remaining State Candidates")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*Slice::lstToString(dummy_states.clone(), (std::sync::Arc::new(fnptr!(BVariable::pointerToString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), 10)); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString((unmatched_eqns.clone().len() as i32))); __mm_s.push_str(&*literal!(") Remaining Equations")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*Slice::lstToString(unmatched_eqns.clone(), (std::sync::Arc::new({ let __pe_b1 = (literal!("")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<ArcStr> + 'static>), 10)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    r#str
}

fn toStringUnmatched(mut unmatched_vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>, mut unmatched_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let mut s3: ArcStr = arcstr::literal!("");
    let mut s4: ArcStr = arcstr::literal!("");
    if unmatched_vars.clone().is_empty() {
        s1 = (StringUtil::headline_4((literal!("Not underdetermined.")).clone())).clone();
        s3 = (literal!("")).clone();
    } else {
        s1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Stage ")); __mm_s.push_str(&*intString((unmatched_vars.clone().len() as i32))); __mm_s.push_str(&*literal!(" underdetermined.\n")); ArcStr::from(__mm_s) }).clone();
        s3 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString((unmatched_vars.clone().len() as i32))); __mm_s.push_str(&*literal!(") Unmatched variables:")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*Slice::lstToString(unmatched_vars.clone(), (std::sync::Arc::new(fnptr!(BVariable::pointerToString, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), 10)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    }
    if unmatched_eqns.clone().is_empty() {
        s2 = (StringUtil::headline_4((literal!("Not overdetermined.")).clone())).clone();
        s4 = (literal!("")).clone();
    } else {
        s2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Stage ")); __mm_s.push_str(&*intString((unmatched_eqns.clone().len() as i32))); __mm_s.push_str(&*literal!(" overdetermined.\n")); ArcStr::from(__mm_s) }).clone();
        s4 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*intString((unmatched_eqns.clone().len() as i32))); __mm_s.push_str(&*literal!(") Unmatched equations:")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*Slice::lstToString(unmatched_eqns.clone(), (std::sync::Arc::new({ let __pe_b1 = (literal!("")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<ArcStr> + 'static>), 10)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    }
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*s4.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
    r#str
}

