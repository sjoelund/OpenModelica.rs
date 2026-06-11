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

use crate::NBASSC as ASSC;
use crate::NBAdjacency as Adjacency;
use crate::NBBackendUtil as BackendUtil;
use crate::NBDifferentiate as Differentiate;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBMatching as Matching;
use crate::NBModule as Module;
use crate::NBPartition as BPartition;
use crate::NBPartition::Partition;
use crate::NBSorting as Sorting;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn::Path;
use openmodelica_nf_frontend::NFArrayConnections::NameVertexTable;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFTypeCheck as TypeCheck;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// Backend imports
// util imports
// ############################################################
//                      Main Functions
// ############################################################
pub(crate) fn main(mut bdae: Arc<BackendDAE::NBackendDAE>, mut kind: BPartition::Kind) -> Result<Arc<BackendDAE::NBackendDAE>> {
    let mut bdae: Arc<BackendDAE::NBackendDAE> = bdae;
    let mut func: Module::causalizeInterface = getModule()?;
    bdae = (::match_deref::match_deref! { match &((kind, bdae.clone())) {
        (BPartition::Kind::ODE, Deref @ BackendDAE::MAIN { ode: partitions, clocked, varData, eqData, .. }) => {
            let mut partitions = (*partitions).clone();
            let mut clocked = (*clocked).clone();
            let mut varData = (*varData).clone();
            let mut eqData = (*eqData).clone();
            (partitions, varData, eqData) = applyModule(partitions.clone(), kind, varData.clone(), eqData.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), func.clone())?;
            (clocked, varData, eqData) = applyModule(clocked.clone(), kind, varData.clone(), eqData.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), func.clone())?;
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                ode = partitions.clone(),
                clocked = clocked.clone(),
                varData = varData.clone(),
                eqData = eqData.clone()
            );
            bdae
        },
        (_, Deref @ BackendDAE::MAIN { init: partitions, varData, eqData, .. }) if (BPartition::kindIsInitial(kind)) => {
            let mut partitions = (*partitions).clone();
            let mut varData = (*varData).clone();
            let mut eqData = (*eqData).clone();
            if Flags::isSet(Flags::INITIALIZATION.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1((literal!("Balance Initialization")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (partitions, varData, eqData) = applyModule(partitions.clone(), kind, varData.clone(), eqData.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), func.clone())?;
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; init = partitions.clone());
            if isSome(var_field!((*bdae).init_0, BackendDAE::NBackendDAE::MAIN).clone()) {
                (partitions, varData, eqData) = applyModule(Util::getOption(var_field!((*bdae).init_0, BackendDAE::NBackendDAE::MAIN).clone())?, kind, varData.clone(), eqData.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), func.clone())?;
                assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; init_0 = Some(partitions.clone()));
            }
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                varData = varData.clone(),
                eqData = eqData.clone()
            );
            bdae
        },
        (BPartition::Kind::DAE, Deref @ BackendDAE::MAIN { dae: Some(partitions), varData, eqData, .. }) => {
            let mut partitions = (*partitions).clone();
            let mut varData = (*varData).clone();
            let mut eqData = (*eqData).clone();
            (partitions, varData, eqData) = applyModule(partitions.clone(), kind, varData.clone(), eqData.clone(), var_field!((*bdae).funcMap, BackendDAE::NBackendDAE::MAIN).clone(), (std::sync::Arc::new(fnptr!(causalizeDAEMode, Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>)> + 'static>))?;
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                dae = Some(partitions.clone()),
                varData = varData.clone(),
                eqData = eqData.clone()
            );
            bdae
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBCausalize.main")); __mm_s.push_str(&*literal!(" failed with partition type ")); __mm_s.push_str(&*BPartition::Partition::kindToString(kind)?); __mm_s.push_str(&*literal!("!")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bdae)
}

pub(crate) fn applyModule(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition>>>, mut kind: BPartition::Kind, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut func: Arc<dyn ::std::ops::Fn(Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>)> + 'static>) -> Result<(Arc<metamodelica::List<Arc<Partition::Partition>>>, Arc<VarData::VarData>, Arc<EqData::EqData>)> {
    let mut new_partitions: Arc<metamodelica::List<Arc<Partition::Partition>>> = metamodelica::nil();
    let mut varData: Arc<VarData::VarData> = varData;
    let mut eqData: Arc<EqData::EqData> = eqData;
    let mut new_partition: Arc<Partition::Partition>;
    let mut violated: bool = false;
    for mut partition in &*partitions {
        let mut partition = partition.clone();
        (new_partition, varData, eqData) = func(partition.clone(), varData.clone(), eqData.clone(), funcMap.clone())?;
        new_partitions = if (BPartition::Partition::isEmpty(new_partition.clone())?) {new_partitions.clone()} else {metamodelica::cons(new_partition.clone(), new_partitions.clone())};
    }
    new_partitions = new_partitions.reverse();
    if !(BPartition::kindIsInitial(kind)) {
        for mut partition in &*new_partitions.clone() {
            let mut partition = partition.clone();
            violated = checkSystemVariabilities(partition.clone())? || violated;
        }
        if violated {
            bail!("fail");
        }
    }
    Ok((new_partitions, varData, eqData))
}

pub(crate) fn checkSystemVariabilities(mut partition: Arc<Partition::Partition>) -> Result<bool> {
    let mut violated: bool = false;
    let mut err: ArcStr = arcstr::literal!("");
    if isSome(partition.strongComponents.clone()) {
        let __range0 = Util::getOption(partition.strongComponents.clone())?.borrow().iter().cloned().collect::<Vec<_>>();
        for mut scc in __range0 {
            let () = (::match_deref::match_deref! { match &(scc.clone()) {
        Deref @ StrongComponent::SINGLE_COMPONENT { .. } => {
            let mut ty1: Arc<Type::NFType>;
            let mut ty2: Arc<Type::NFType>;
            let mut kind: TypeCheck::MatchKind;
            ty1 = Type::removeSizeOneArraysAndRecords(Variable::typeOf(Pointer::access(var_field!((*scc).var, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone())))?;
            ty2 = Type::removeSizeOneArraysAndRecords(BEquation::Equation::getType(Pointer::access(var_field!((*scc).eqn, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone()), false)?)?;
            (_, _, kind) = TypeCheck::matchTypes(ty1.clone(), ty2.clone(), Expression::fromCref(BVariable::getVarName(var_field!((*scc).var, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone()), false)?, TypeCheck::DEFAULT_OPTIONS.clone())?;
            if kind.clone() != TypeCheck::MatchKind::EXACT.clone() {
                err = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBCausalize.checkSystemVariabilities")); __mm_s.push_str(&*literal!(" failed. The following strong component has conflicting types: ")); __mm_s.push_str(&*Type::toString(ty1.clone())?); __mm_s.push_str(&*literal!(" != ")); __mm_s.push_str(&*Type::toString(ty2.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StrongComponent::toString(scc.clone(), -1)?); ArcStr::from(__mm_s) }).clone();
                if Flags::isSet(Flags::BLT_DUMP.clone())? {
                    err = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*err.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*BPartition::Partition::toString(partition.clone(), 0)?); ArcStr::from(__mm_s) }).clone();
                }
                Error::addMessage(Error::COMPILER_ERROR.clone(), list![(err.clone()).clone()])?;
                violated = true;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
    }
    Ok(violated)
}

pub(crate) fn simple(mut vars: Arc<VariablePointers::VariablePointers>, mut eqns: Arc<EquationPointers::EquationPointers>, mut kind: BPartition::Kind, mut st: Adjacency::MatrixStrictness, mut iter: Arc<Iterator::Iterator>) -> Result<(Arc<Matching::NBMatching>, Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>)> {
    let mut matching: Arc<Matching::NBMatching>;
    let mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>;
    let mut full: Arc<Adjacency::Matrix::Matrix>;
    let mut adj: Arc<Adjacency::Matrix::Matrix>;
    full = Adjacency::Matrix::createFull(vars.clone(), eqns.clone(), kind)?;
    adj = Adjacency::Matrix::fullToFinal(full.clone(), vars.map.clone(), eqns.map.clone(), eqns.clone(), st, iter)?;
    matching = Matching::regular(Matching::EMPTY_MATCHING().clone(), adj.clone(), false, false, true)?;
    adj = Adjacency::Matrix::upgrade(adj, full, vars.map.clone(), eqns.map.clone(), eqns.clone(), Adjacency::MatrixStrictness::SORTING.clone(), crate::NBEquation::Iterator::interned_EMPTY())?;
    comps = Sorting::tarjan(adj, matching.clone(), vars, eqns)?;
    Ok((matching, comps))
}

pub(crate) fn getModule() -> Result<Arc<dyn ::std::ops::Fn(Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>)> + 'static>> {
    let mut func: Module::causalizeInterface;
    let mut flag: ArcStr = Flags::getConfigString(Flags::MATCHING_ALGORITHM.clone())?;
    func = (::match_deref::match_deref! { match &(flag.clone()) {
        Deref @ "PFPlusExt" => (std::sync::Arc::new(causalizePseudoArray) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>)> + 'static>),
        Deref @ "pseudo" => (std::sync::Arc::new(causalizePseudoArray) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>)> + 'static>),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBCausalize.getModule")); __mm_s.push_str(&*literal!(" failed for unknown option: ")); __mm_s.push_str(&*flag); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

// ############################################################
//                Protected Functions and Types
// ############################################################
fn causalizePseudoArray(mut partition: Arc<Partition::Partition>, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>)> {
    let mut partition: Arc<Partition::Partition> = partition;
    let mut varData: Arc<VarData::VarData> = varData;
    let mut eqData: Arc<EqData::EqData> = eqData;
    let mut kind: BPartition::Kind = BPartition::Partition::getKind(partition.clone());
    let mut variables: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut equations: Arc<EquationPointers::EquationPointers> = Arc::new(<EquationPointers::EquationPointers as ::std::default::Default>::default());
    let mut full: Arc<Adjacency::Matrix::Matrix> = Arc::new(<Adjacency::Matrix::Matrix as ::std::default::Default>::default());
    let mut adj_matching: Arc<Adjacency::Matrix::Matrix> = Arc::new(<Adjacency::Matrix::Matrix as ::std::default::Default>::default());
    let mut adj_sorting: Arc<Adjacency::Matrix::Matrix> = Arc::new(<Adjacency::Matrix::Matrix as ::std::default::Default>::default());
    let mut matching: Arc<Matching::NBMatching> = Arc::new(<Matching::NBMatching as ::std::default::Default>::default());
    let mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    (variables, equations, full, matching, comps) = (match kind {
        mut kind if (BPartition::kindIsInitial(kind)) => {
            let mut fixable: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
            let mut unfixable: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
            let mut initials: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>;
            let mut simulation: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>;
            let mut vo: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
            let mut vn: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
            let mut eo: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
            let mut en: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
            assign_field!(
                partition.unknowns = BVariable::VariablePointers::compress(partition.unknowns.clone())?,
                partition.equations = BEquation::EquationPointers::compress(partition.equations.clone())?
            );
            (fixable, unfixable) = List::splitOnTrue(BVariable::VariablePointers::toList(partition.unknowns.clone())?, (std::sync::Arc::new(BVariable::isFixable) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?;
            (initials, simulation) = List::splitOnTrue(BEquation::EquationPointers::toList(partition.equations.clone())?, (std::sync::Arc::new(fnptr!(BEquation::Equation::isInitial, Pointer::Pointer<Arc<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>))?;
            full = Adjacency::Matrix::createFull(partition.unknowns.clone(), partition.equations.clone(), kind)?;
            vn = UnorderedMap::subMap(partition.unknowns.map.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut var in (unfixable.clone()).into_iter().cloned() {
            let __x = BVariable::getVarName(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
            en = UnorderedMap::subMap(partition.equations.map.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut eqn in (initials.clone()).into_iter().cloned() {
            let __x = BEquation::Equation::getEqnName(eqn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
            adj_matching = Adjacency::Matrix::fullToFinal(full.clone(), vn.clone(), en.clone(), partition.equations.clone(), Adjacency::MatrixStrictness::MATCHING.clone(), crate::NBEquation::Iterator::interned_EMPTY())?;
            matching = Matching::regular(Matching::EMPTY_MATCHING().clone(), adj_matching.clone(), true, true, true)?;
            vo = vn.clone();
            eo = en.clone();
            vn = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            en = UnorderedMap::subMap(partition.equations.map.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut eqn in (simulation.clone()).into_iter().cloned() {
            let __x = BEquation::Equation::getEqnName(eqn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
            (adj_matching, full) = Adjacency::Matrix::expand(adj_matching, full, vo.clone(), vn.clone(), eo.clone(), en.clone(), partition.unknowns.clone(), partition.equations.clone(), BPartition::Partition::getKind(partition.clone()))?;
            matching = Matching::regular(matching, adj_matching.clone(), true, true, true)?;
            vo = UnorderedMap::merge(vo.clone(), vn.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBCausalize.mo"))?;
            eo = UnorderedMap::merge(eo.clone(), en.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBCausalize.mo"))?;
            vn = UnorderedMap::subMap(partition.unknowns.map.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut var in (fixable.clone()).into_iter().cloned() {
            let __x = BVariable::getVarName(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
            en = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            (adj_matching, full) = Adjacency::Matrix::expand(adj_matching, full, vo.clone(), vn.clone(), eo.clone(), en.clone(), partition.unknowns.clone(), partition.equations.clone(), BPartition::Partition::getKind(partition.clone()))?;
            (matching, adj_matching, full, variables, equations, varData, eqData) = Matching::singular(matching, adj_matching, full, partition.unknowns.clone(), partition.equations.clone(), funcMap, varData, eqData, kind, false, false)?;
            adj_sorting = Adjacency::Matrix::upgrade(adj_matching, full.clone(), variables.map.clone(), equations.map.clone(), equations.clone(), Adjacency::MatrixStrictness::SORTING.clone(), crate::NBEquation::Iterator::interned_EMPTY())?;
            comps = Sorting::tarjan(adj_sorting, matching.clone(), variables.clone(), equations.clone())?;
            (variables, equations, full, matching, comps)
        },
        _ => {
            variables = BVariable::VariablePointers::compress(partition.unknowns.clone())?;
            equations = BEquation::EquationPointers::compress(partition.equations.clone())?;
            ASSC::main(equations.clone(), variables.clone());
            full = Adjacency::Matrix::createFull(variables.clone(), equations.clone(), kind)?;
            adj_matching = Adjacency::Matrix::fullToFinal(full.clone(), variables.map.clone(), equations.map.clone(), equations.clone(), Adjacency::MatrixStrictness::MATCHING.clone(), crate::NBEquation::Iterator::interned_EMPTY())?;
            (matching, adj_matching, full, variables, equations, varData, eqData) = Matching::singular(Matching::EMPTY_MATCHING().clone(), adj_matching, full, variables, equations, funcMap, varData, eqData, kind, false, true)?;
            adj_sorting = Adjacency::Matrix::upgrade(adj_matching, full.clone(), variables.map.clone(), equations.map.clone(), equations.clone(), Adjacency::MatrixStrictness::SORTING.clone(), crate::NBEquation::Iterator::interned_EMPTY())?;
            comps = Sorting::tarjan(adj_sorting, matching.clone(), variables.clone(), equations.clone())?;
            (variables, equations, full, matching, comps)
        },
    });
    assign_field!(
        partition.unknowns = variables,
        partition.equations = equations,
        partition.adjacencyMatrix = Some(full),
        partition.matching = Some(matching),
        partition.strongComponents = Some(metamodelica::arrayFromVec(comps.into_iter().cloned().collect()))
    );
    Ok((partition, varData, eqData))
}

fn causalizeDAEMode(mut partition: Arc<Partition::Partition>, mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> (Arc<Partition::Partition>, Arc<VarData::VarData>, Arc<EqData::EqData>) {
    let mut partition: Arc<Partition::Partition> = partition;
    let mut varData: Arc<VarData::VarData> = varData;
    let mut eqData: Arc<EqData::EqData> = eqData;
    (partition, varData, eqData)
}

