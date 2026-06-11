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
use crate::NBAdjacency::Mapping;
use crate::NBDifferentiate as Differentiate;
use crate::NBDifferentiate::DifferentiationArguments;
use crate::NBDifferentiate::DifferentiationType;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointers;
use crate::NBEquation;
use crate::NBMatching as Matching;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBReplacements as Replacements;
use crate::NBSlice as Slice;
use crate::NBSolve;
use crate::NBSorting as Sorting;
use crate::NBStrongComponent as StrongComponent;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBVariable;
use crate::NBackendDAE as BackendDAE;
use crate::NBackendDAE as Jacobian;
use openmodelica_ast::Absyn::Path;
use openmodelica_backend_util::Coloring;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFOperator::MathClassification;
use openmodelica_nf_frontend::NFOperator::SizeClassification;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// Backend imports
// Sparsity-pattern graph coloring, shared with the old backend.
// Util imports
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum JacobianType {
    ODE = 1,
    DAE = 2,
    LS = 3,
    NLS = 4,
    OPT_LFG = 5,
    OPT_MRF = 6,
    OPT_R0 = 7,
}
impl PartialOrd for JacobianType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for JacobianType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl metamodelica::gc::MMTrace for JacobianType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> { Ok(()) }
}

pub(crate) fn isDynamic(mut jacType: JacobianType) -> bool {
    let mut b: bool;
    b = (match jacType {
        JacobianType::ODE => true,
        JacobianType::DAE { .. } => true,
        JacobianType::OPT_LFG => true,
        JacobianType::OPT_MRF => true,
        JacobianType::OPT_R0 => true,
        _ => false,
    });
    b
}

pub(crate) fn main(mut bdae: Arc<Jacobian::NBackendDAE>, mut kind: Partition::Kind) -> Result<Arc<Jacobian::NBackendDAE>> {
    let mut bdae: Arc<Jacobian::NBackendDAE> = bdae;
    let func: Module::jacobianInterface = getModule()?;
    bdae = (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ Jacobian::MAIN { varData: Deref @ NBVariable::VarData::VAR_DATA_SIM { knowns, .. }, .. } => {
            let mut name: ArcStr = arcstr::literal!("");
            if Flags::isSet(Flags::JAC_DUMP.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1((literal!("[symjacdump] Creating symbolic Jacobians:")).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            name = ((match kind {
        Partition::Kind::ODE => {
            name = (literal!("ODE_JAC")).clone();
            assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN; ode = applyToPartitions(var_field!((*bdae).ode, Jacobian::NBackendDAE::MAIN).clone(), var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), knowns.clone(), (name.clone()).clone(), func.clone())?.0);
            name.clone()
        },
        Partition::Kind::DAE => {
            name = (literal!("DAE_JAC")).clone();
            assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN; dae = Some((applyToPartitions(Util::getOption(var_field!((*bdae).dae, Jacobian::NBackendDAE::MAIN).clone())?, var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), knowns.clone(), (name.clone()).clone(), func.clone())?).0));
            name.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.main")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Partition::Partition::kindToString(kind)?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    })).clone();
            assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN;
                ode_event = applyToPartitions(var_field!((*bdae).ode_event, Jacobian::NBackendDAE::MAIN).clone(), var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), knowns.clone(), (name.clone()).clone(), func.clone())?.0,
                algebraic = applyToPartitions(var_field!((*bdae).algebraic, Jacobian::NBackendDAE::MAIN).clone(), var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), knowns.clone(), (name.clone()).clone(), func.clone())?.0,
                alg_event = applyToPartitions(var_field!((*bdae).alg_event, Jacobian::NBackendDAE::MAIN).clone(), var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), knowns.clone(), (name.clone()).clone(), func.clone())?.0,
                init = applyToPartitions(var_field!((*bdae).init, Jacobian::NBackendDAE::MAIN).clone(), var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), knowns.clone(), (name.clone()).clone(), func.clone())?.0
            );
            if isSome(var_field!((*bdae).init_0, Jacobian::NBackendDAE::MAIN).clone()) {
                assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN; init_0 = Some((applyToPartitions(Util::getOption(var_field!((*bdae).init_0, Jacobian::NBackendDAE::MAIN).clone())?, var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), knowns.clone(), (name.clone()).clone(), func.clone())?).0));
            }
            bdae
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.main")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Jacobian::toString(bdae, (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bdae)
}

pub(crate) fn applyToPartitions(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut knowns: Arc<VariablePointers::VariablePointers>, mut name: ArcStr, mut func: Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> + 'static>) -> Result<(Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>)> {
    let mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = partitions;
    let mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>> = funcMap;
    partitions = ({
        let mut __acc: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = metamodelica::nil();
        for mut part in (partitions).into_iter().cloned() {
            let __x = partJacobian(part.clone(), funcMap.clone(), knowns.clone(), (name.clone()).clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((partitions, funcMap))
}

pub(crate) fn nonlinear(mut seedCandidates: Arc<VariablePointers::VariablePointers>, mut partialCandidates: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut comps: metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>, mut full: Option<Arc<Adjacency::Matrix::Matrix>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut name: ArcStr, mut staticAsContinuous: bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> {
    let mut jacobian: Option<Arc<Jacobian::NBackendDAE>>;
    let func: Module::jacobianInterface = if (Flags::isSet(Flags::NLS_ANALYTIC_JACOBIAN.clone())?) {(std::sync::Arc::new(jacobianSymbolic) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> + 'static>)} else {(std::sync::Arc::new(jacobianNumeric) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> + 'static>)};
    jacobian = func((name).clone(), JacobianType::NLS.clone(), seedCandidates, partialCandidates, equations, Some(comps.clone()), full, funcMap, staticAsContinuous)?;
    Ok(jacobian)
}

pub(crate) fn combine(mut jacobians: Arc<metamodelica::List<Arc<Jacobian::NBackendDAE>>>, mut name: ArcStr) -> Result<Arc<Jacobian::NBackendDAE>> {
    let mut jacobian: Arc<Jacobian::NBackendDAE>;
    let mut jacType: JacobianType = JacobianType::ODE;
    let mut variables: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut unknowns: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut auxiliaryVars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut aliasVars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut diffVars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut dependencies: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut resultVars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut tmpVars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut seedVars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut col_wise_pattern: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
    let mut row_wise_pattern: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
    let mut seed_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut partial_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut nnz: i32 = 0;
    let mut varData: Arc<VarData::VarData>;
    let mut sparsityPattern: Arc<SparsityPattern::SparsityPattern>;
    let mut sparsityColoring: Arc<SparsityColoring::SparsityColoring> = SparsityColoring::lazy(EMPTY_SPARSITY_PATTERN().clone());
    if List::hasOneElement(jacobians.clone()) {
        jacobian = listHead(jacobians)?;
        jacobian = (::match_deref::match_deref! { match &(jacobian.clone()) {
        Deref @ Jacobian::JACOBIAN { .. } => {
            assign_variant_field!(jacobian => Jacobian::NBackendDAE::JACOBIAN; name = name);
            jacobian
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.combine")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*Jacobian::toString(jacobian, (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    } else {
        for mut jac in &*jacobians {
            let mut jac = jac.clone();
            let () = (::match_deref::match_deref! { match &(jac.clone()) {
        Deref @ Jacobian::JACOBIAN { varData: tmpVarData @ Deref @ NBVariable::VarData::VAR_DATA_JAC { .. }, sparsityPattern: tmpPattern, .. } => {
            jacType = var_field!((*jac).jacType, Jacobian::NBackendDAE::JACOBIAN).clone();
            variables = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).variables, VarData::VarData::VAR_DATA_JAC).clone())?, variables.clone());
            unknowns = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).unknowns, VarData::VarData::VAR_DATA_JAC).clone())?, unknowns.clone());
            auxiliaryVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).auxiliaries, VarData::VarData::VAR_DATA_JAC).clone())?, auxiliaryVars.clone());
            aliasVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).aliasVars, VarData::VarData::VAR_DATA_JAC).clone())?, aliasVars.clone());
            diffVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).diffVars, VarData::VarData::VAR_DATA_JAC).clone())?, diffVars.clone());
            dependencies = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).dependencies, VarData::VarData::VAR_DATA_JAC).clone())?, dependencies.clone());
            resultVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).resultVars, VarData::VarData::VAR_DATA_JAC).clone())?, resultVars.clone());
            tmpVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).tmpVars, VarData::VarData::VAR_DATA_JAC).clone())?, tmpVars.clone());
            seedVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).seedVars, VarData::VarData::VAR_DATA_JAC).clone())?, seedVars.clone());
            comps = listAppend(Arc::new(var_field!((*jac).comps, Jacobian::NBackendDAE::JACOBIAN).clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), comps.clone());
            col_wise_pattern = listAppend(tmpPattern.col_wise_pattern.clone(), col_wise_pattern.clone());
            row_wise_pattern = listAppend(tmpPattern.row_wise_pattern.clone(), row_wise_pattern.clone());
            seed_vars = listAppend(tmpPattern.seed_vars.clone(), seed_vars.clone());
            partial_vars = listAppend(tmpPattern.partial_vars.clone(), partial_vars.clone());
            nnz = nnz + tmpPattern.nnz.clone();
            sparsityColoring = SparsityColoring::combine(sparsityColoring.clone(), var_field!((*jac).sparsityColoring, Jacobian::NBackendDAE::JACOBIAN).clone());
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.combine")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*Jacobian::toString(jac.clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        varData = Arc::new(VarData::VarData::VAR_DATA_JAC { variables: NBVariable::VariablePointers::fromList(variables, false)?, unknowns: NBVariable::VariablePointers::fromList(unknowns, false)?, auxiliaries: NBVariable::VariablePointers::fromList(auxiliaryVars, false)?, aliasVars: NBVariable::VariablePointers::fromList(aliasVars, false)?, diffVars: NBVariable::VariablePointers::fromList(diffVars, false)?, dependencies: NBVariable::VariablePointers::fromList(dependencies, false)?, resultVars: NBVariable::VariablePointers::fromList(resultVars, false)?, tmpVars: NBVariable::VariablePointers::fromList(tmpVars, false)?, seedVars: NBVariable::VariablePointers::fromList(seedVars, false)? });
        sparsityPattern = Arc::new(SparsityPattern::SparsityPattern { col_wise_pattern: col_wise_pattern, row_wise_pattern: row_wise_pattern, seed_vars: seed_vars, partial_vars: partial_vars, nnz: nnz });
        jacobian = Arc::new(Jacobian::NBackendDAE::JACOBIAN { name: (name.clone()).clone(), jacType: jacType, varData: varData, comps: metamodelica::arrayFromVec(comps.into_iter().cloned().collect()), sparsityPattern: sparsityPattern, sparsityColoring: sparsityColoring, isAdjoint: name == literal!("ADJ") });
    }
    Ok(jacobian)
}

pub(crate) fn getModule() -> Result<Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> + 'static>> {
    let mut func: Module::jacobianInterface;
    func = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::GENERATE_DYNAMIC_JACOBIAN.clone())?) {
        Deref @ "symbolic" => (std::sync::Arc::new(jacobianSymbolic) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> + 'static>),
        Deref @ "symbolicadjoint" => (std::sync::Arc::new(jacobianSymbolicAdjoint) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> + 'static>),
        Deref @ "numeric" => (std::sync::Arc::new(jacobianNumeric) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> + 'static>),
        Deref @ "none" => (std::sync::Arc::new(fnptr!(jacobianNone, ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> + 'static>),
        _ => bail!("match: no arm matched"),
    } });
    Ok(func)
}

pub(crate) fn toString(mut jacobian: Arc<Jacobian::NBackendDAE>, mut r#str: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    r#str = (Jacobian::toString(jacobian, (r#str).clone())?).clone();
    Ok(r#str)
}

pub(crate) fn jacobianTypeString(mut jacType: JacobianType) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = ((match jacType {
        JacobianType::ODE => literal!("[ODE]"),
        JacobianType::DAE { .. } => literal!("[DAE]"),
        JacobianType::LS => literal!("[LS-]"),
        JacobianType::NLS => literal!("[NLS]"),
        JacobianType::OPT_LFG => literal!("[OPT-LFG]"),
        JacobianType::OPT_MRF => literal!("[OPT-MRF]"),
        JacobianType::OPT_R0 => literal!("[OPT-R0]"),
        _ => literal!("[ERR]"),
    })).clone();
    r#str
}

// necessary as wrapping value type for UnorderedMap
pub type CrefLst = Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;

/// partial_vars, {seed_vars}
pub type SparsityPatternCol = (Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>);

/// seed_vars, {partial_vars}
pub type SparsityPatternRow = (Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>);

pub mod SparsityPattern {
    use super::*;
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct SparsityPattern {
        /// colum-wise sparsity pattern
        pub col_wise_pattern: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>>,
        /// row-wise sparsity pattern
        pub row_wise_pattern: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>>,
        /// independent variables solved here ($SEED)
        pub seed_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>,
        /// LHS variables of the jacobian ($pDER)
        pub partial_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>,
        /// number of nonzero elements
        pub nnz: i32,
    }

    impl metamodelica::gc::MMTrace for SparsityPattern {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.col_wise_pattern, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.row_wise_pattern, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.seed_vars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.partial_vars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.nnz, __mmv)?;
            Ok(())
        }
    }
    impl Default for SparsityPattern {
        fn default() -> Self {
            Self {
                col_wise_pattern: Default::default(),
                row_wise_pattern: Default::default(),
                seed_vars: Default::default(),
                partial_vars: Default::default(),
                nnz: Default::default(),
            }
        }
    }

    pub type SPARSITY_PATTERN = SparsityPattern;

    pub(crate) fn toString(mut pattern: Arc<SparsityPattern>) -> Result<ArcStr> {
        let mut r#str: ArcStr = StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Sparsity Pattern (nnz: ")); __mm_s.push_str(&*intString(pattern.nnz.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone());
        let mut cref: Arc<ComponentRef::NFComponentRef>;
        let mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
        let mut colEmpty: bool = pattern.col_wise_pattern.clone().is_empty();
        let mut rowEmpty: bool = pattern.row_wise_pattern.clone().is_empty();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3((literal!("### Seeds (col vars) ###")).clone())); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*List::toString(pattern.seed_vars.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3((literal!("### Partials (row vars) ###")).clone())); ArcStr::from(__mm_s) }).clone();
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*List::toString(pattern.partial_vars.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        if !(colEmpty) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3((literal!("### Columns ###")).clone())); ArcStr::from(__mm_s) }).clone();
            for mut col in &*pattern.col_wise_pattern.clone() {
                let mut col = col.clone();
                (cref, dependencies) = col.clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(")\t affects:\t")); __mm_s.push_str(&*ComponentRef::listToString(dependencies.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
        }
        if !(rowEmpty) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3((literal!("##### Rows #####")).clone())); ArcStr::from(__mm_s) }).clone();
            for mut row in &*pattern.row_wise_pattern.clone() {
                let mut row = row.clone();
                (cref, dependencies) = row.clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!(")\t depends on:\t")); __mm_s.push_str(&*ComponentRef::listToString(dependencies.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
        }
        Ok(r#str)
    }

    pub(crate) fn lazy(mut seedCandidates: Arc<VariablePointers::VariablePointers>, mut partialCandidates: Arc<VariablePointers::VariablePointers>, mut strongComponents: Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, mut jacType: JacobianType) -> Result<(Arc<SparsityPattern>, Arc<SparsityColoring::SparsityColoring>)> {
        let mut sparsityPattern: Arc<SparsityPattern>;
        let mut sparsityColoring: Arc<SparsityColoring::SparsityColoring>;
        let mut seed_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
        let mut partial_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
        let mut cols: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
        let mut rows: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
        let mut nnz: i32;
        seed_vars = NBVariable::VariablePointers::getScalarVarNames(seedCandidates, false)?;
        partial_vars = NBVariable::VariablePointers::getScalarVarNames(partialCandidates, false)?;
        cols = ({
        let mut __acc: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
        for mut s in (seed_vars.clone()).into_iter().cloned() {
            let __x = (s.clone(), partial_vars.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        rows = ({
        let mut __acc: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
        for mut p in (partial_vars.clone()).into_iter().cloned() {
            let __x = (p.clone(), seed_vars.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        nnz = (partial_vars.clone().len() as i32) * (seed_vars.clone().len() as i32);
        sparsityPattern = Arc::new(SparsityPattern { col_wise_pattern: cols, row_wise_pattern: rows, seed_vars: seed_vars, partial_vars: partial_vars, nnz: nnz });
        sparsityColoring = SparsityColoring::lazy(sparsityPattern.clone());
        Ok((sparsityPattern, sparsityColoring))
    }

    pub(crate) fn adjacencyMapToString(mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<ArcStr> {
        let mut s: ArcStr;
        let mut keys: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
        let mut k: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut neighs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
        let mut lines: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        keys = UnorderedMap::keyList(map.clone());
        for mut k in &*keys.clone() {
            let mut k = k.clone();
            neighs = UnorderedMap::getOrFail(k.clone(), map.clone())?;
            lines = metamodelica::cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*ComponentRef::toString(k.clone())?); __mm_s.push_str(&*literal!(" -> ")); __mm_s.push_str(&*ComponentRef::listToString(neighs.clone())?); ArcStr::from(__mm_s) }).clone(), lines.clone());
        }
        lines = lines.reverse();
        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Adjacency map (")); __mm_s.push_str(&*intString((keys.len() as i32))); __mm_s.push_str(&*literal!(" keys):\n")); __mm_s.push_str(&*stringDelimitList(lines, (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone();
        Ok(s)
    }

    pub(crate) fn create(mut seedCandidates: Arc<VariablePointers::VariablePointers>, mut partialCandidates: Arc<VariablePointers::VariablePointers>, mut strongComponents: Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, mut jacType: JacobianType) -> Result<(Arc<SparsityPattern>, Arc<SparsityColoring::SparsityColoring>)> {
        let mut sparsityPattern: Arc<SparsityPattern>;
        let mut sparsityColoring: Arc<SparsityColoring::SparsityColoring>;
        let mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = <Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> as ::std::default::Default>::default();
        (sparsityPattern, map) = ({
        let mut row_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut col_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut cols: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
        let mut rows: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>> = metamodelica::nil();
        let mut nnz: i32 = 0;
        (match strongComponents {
        Some(mut comps) if (comps.clone().borrow().is_empty()) => {
            (EMPTY_SPARSITY_PATTERN().clone(), UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1))
        },
        Some(mut comps) => {
            let mut seed_mapping: Arc<Mapping::Mapping>;
            let mut partial_mapping: Arc<Mapping::Mapping>;
            let mut seed_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
            let mut seed_vars_array: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
            let mut partial_vars: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
            let mut partial_vars_array: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
            let mut tmp: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
            let mut set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>;
            seed_mapping = Adjacency::Mapping::create(NBEquation::EquationPointers::empty(BaseHashTable::bigBucketSize.clone()), seedCandidates.clone())?;
            partial_mapping = Adjacency::Mapping::create(NBEquation::EquationPointers::empty(BaseHashTable::bigBucketSize.clone()), partialCandidates.clone())?;
            partial_vars = NBVariable::VariablePointers::getScalarVarNames(partialCandidates.clone(), false)?;
            seed_vars = NBVariable::VariablePointers::getScalarVarNames(seedCandidates.clone(), false)?;
            seed_vars_array = NBVariable::VariablePointers::getVarNames(seedCandidates.clone())?;
            partial_vars_array = NBVariable::VariablePointers::getVarNames(partialCandidates.clone())?;
            map = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), Util::nextPrime((seed_vars.clone().len() as i32) + (partial_vars.clone().len() as i32)));
            set = UnorderedSet::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), Util::nextPrime((seed_vars_array.clone().len() as i32)));
            for mut cref in &*seed_vars.clone() {
                let mut cref = cref.clone();
                UnorderedMap::add(cref.clone(), metamodelica::nil(), map.clone())?;
            }
            for mut cref in &*partial_vars.clone() {
                let mut cref = cref.clone();
                UnorderedMap::add(cref.clone(), metamodelica::nil(), map.clone())?;
            }
            for mut cref in &*seed_vars_array.clone() {
                let mut cref = cref.clone();
                UnorderedSet::add(cref.clone(), set.clone())?;
            }
            for mut cref in &*partial_vars_array.clone() {
                let mut cref = cref.clone();
                UnorderedSet::add(cref.clone(), set.clone())?;
            }
            for mut i in 1..=metamodelica::arrayLength(comps.clone()) {
                if !(StrongComponent::isDiscrete(({let __elt = comps.borrow()[(i.clone()-1) as usize].clone(); __elt}))?) {
                    StrongComponent::collectCrefs(({let __elt = comps.borrow()[(i.clone()-1) as usize].clone(); __elt}), seedCandidates.clone(), partialCandidates.clone(), seed_mapping.clone(), partial_mapping.clone(), map.clone(), set.clone(), jacType)?;
                }
            }
            for mut cref in &*partial_vars.clone().reverse() {
                let mut cref = cref.clone();
                if jacType == JacobianType::NLS.clone() || isRowInJacobian(cref.clone(), jacType)? {
                    if UnorderedMap::contains(cref.clone(), map.clone())? {
                        tmp = UnorderedSet::unique_list(UnorderedMap::getOrFail(cref.clone(), map.clone())?, (std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
                        rows = metamodelica::cons((cref.clone(), tmp.clone()), rows.clone());
                        row_vars = metamodelica::cons(cref.clone(), row_vars.clone());
                        for mut dep in &*tmp.clone() {
                            let mut dep = dep.clone();
                            UnorderedMap::add(dep.clone(), metamodelica::cons(cref.clone(), UnorderedMap::getSafe(dep.clone(), map.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?), map.clone())?;
                        }
                    }
                }
            }
            for mut cref in &*seed_vars.clone().reverse() {
                let mut cref = cref.clone();
                if jacType == JacobianType::NLS.clone() || NBVariable::checkCref(cref.clone(), (std::sync::Arc::new(fnptr!(NBVariable::isState, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))? || (jacType == JacobianType::OPT_LFG.clone() || jacType == JacobianType::OPT_MRF.clone() || jacType == JacobianType::OPT_R0.clone()) {
                    tmp = UnorderedSet::unique_list(UnorderedMap::getSafe(cref.clone(), map.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?, (std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
                    cols = metamodelica::cons((cref.clone(), tmp.clone()), cols.clone());
                    col_vars = metamodelica::cons(cref.clone(), col_vars.clone());
                }
            }
            for mut col in &*cols.clone() {
                let mut col = col.clone();
                (_, tmp) = col.clone();
                nnz = nnz.clone() + (tmp.clone().len() as i32);
            }
            (Arc::new(SparsityPattern { col_wise_pattern: cols.clone(), row_wise_pattern: rows.clone(), seed_vars: col_vars.clone(), partial_vars: row_vars.clone(), nnz: nnz.clone() }), map)
        },
        None => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.SparsityPattern.create")); __mm_s.push_str(&*literal!(" failed because of missing strong components.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.SparsityPattern.create")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    })
    });
        sparsityColoring = SparsityColoring::PartialD2ColoringAlgC(sparsityPattern.clone(), jacType)?;
        if Flags::isSet(Flags::DUMP_SPARSE.clone())? {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*toString(sparsityPattern.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*SparsityColoring::toString(sparsityColoring.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        Ok((sparsityPattern, sparsityColoring))
    }

    pub(crate) fn createEmpty() -> (Arc<SparsityPattern>, Arc<SparsityColoring::SparsityColoring>) {
        let mut sparsityPattern: Arc<SparsityPattern> = EMPTY_SPARSITY_PATTERN().clone();
        let mut sparsityColoring: Arc<SparsityColoring::SparsityColoring> = EMPTY_SPARSITY_COLORING().clone();
        (sparsityPattern, sparsityColoring)
    }

}

thread_local! { static __EMPTY_SPARSITY_PATTERN_TLS: Arc<SparsityPattern::SparsityPattern> = Arc::new(SparsityPattern::SparsityPattern { col_wise_pattern: metamodelica::nil(), row_wise_pattern: metamodelica::nil(), seed_vars: metamodelica::nil(), partial_vars: metamodelica::nil(), nnz: 0 }); }
pub(crate) fn EMPTY_SPARSITY_PATTERN() -> Arc<SparsityPattern::SparsityPattern> { __EMPTY_SPARSITY_PATTERN_TLS.with(|__t| __t.clone()) }

thread_local! { static __EMPTY_SPARSITY_COLORING_TLS: Arc<SparsityColoring::SparsityColoring> = Arc::new(SparsityColoring::SparsityColoring { cols: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), rows: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()) }); }
pub(crate) fn EMPTY_SPARSITY_COLORING() -> Arc<SparsityColoring::SparsityColoring> { __EMPTY_SPARSITY_COLORING_TLS.with(|__t| __t.clone()) }

/// seed variable lists belonging to the same color
pub type SparsityColoringCol = Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;

/// partial variable lists for each color (multiples allowed!)
pub type SparsityColoringRow = Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;

pub mod SparsityColoring {
    use super::*;
    /// column wise coloring with extra row sparsity information
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct SparsityColoring {
        pub cols: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>,
        pub rows: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>,
    }

    impl metamodelica::gc::MMTrace for SparsityColoring {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.cols, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.rows, __mmv)?;
            Ok(())
        }
    }
    impl Default for SparsityColoring {
        fn default() -> Self {
            Self {
                cols: Default::default(),
                rows: Default::default(),
            }
        }
    }

    pub type SPARSITY_COLORING = SparsityColoring;

    pub(crate) fn toString(mut sparsityColoring: Arc<SparsityColoring>) -> Result<ArcStr> {
        let mut r#str: ArcStr = StringUtil::headline_2((literal!("Sparsity Coloring")).clone());
        let mut empty: bool = metamodelica::arrayLength(sparsityColoring.cols.clone()) == 0;
        if empty {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n<empty sparsity pattern>\n")); ArcStr::from(__mm_s) }).clone();
        }
        for mut i in 1..=metamodelica::arrayLength(sparsityColoring.cols.clone()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("Column Color (")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*literal!("  - Column: ")); __mm_s.push_str(&*ComponentRef::listToString(({let __elt = sparsityColoring.cols.borrow()[(i.clone()-1) as usize].clone(); __elt}))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        for mut i in 1..=metamodelica::arrayLength(sparsityColoring.rows.clone()) {
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("Row Color (")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(")\n")); __mm_s.push_str(&*literal!("  - Row:    ")); __mm_s.push_str(&*ComponentRef::listToString(({let __elt = sparsityColoring.rows.borrow()[(i.clone()-1) as usize].clone(); __elt}))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
        }
        Ok(r#str)
    }

    pub(crate) fn lazy(mut sparsityPattern: Arc<SparsityPattern::SparsityPattern>) -> Arc<SparsityColoring> {
        let mut sparsityColoring: Arc<SparsityColoring>;
        let mut cols: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut rows: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        cols = metamodelica::arrayFromVec(({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        for mut cref in (sparsityPattern.seed_vars.clone()).into_iter().cloned() {
            let __x = list![cref.clone()];
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect());
        rows = arrayCreate(metamodelica::arrayLength(cols.clone()), sparsityPattern.partial_vars.clone());
        sparsityColoring = Arc::new(SparsityColoring { cols: cols.clone(), rows: rows.clone() });
        sparsityColoring
    }

    pub(crate) fn PartialD2ColoringAlgC(mut sparsityPattern: Arc<SparsityPattern::SparsityPattern>, mut jacType: JacobianType) -> Result<Arc<SparsityColoring>> {
        fn getIndices(mut cref: Arc<ComponentRef::NFComponentRef>, mut seed_indices: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut partial_indices: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut rows: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Arc<metamodelica::List<i32>>> {
            let mut indices: Arc<metamodelica::List<i32>>;
            if UnorderedMap::contains(cref.clone(), seed_indices.clone())? {
                indices = list![UnorderedMap::getSafe(cref, seed_indices, metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?];
            } else if UnorderedMap::contains(cref.clone(), partial_indices.clone())? {
                indices = ({let __elt = rows.borrow()[(UnorderedMap::getSafe(cref, partial_indices, metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?-1) as usize].clone(); __elt});
            } else {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.SparsityColoring.PartialD2ColoringAlgC.getIndices")); __mm_s.push_str(&*literal!(" failed because cref ")); __mm_s.push_str(&*ComponentRef::toString(cref)?); __mm_s.push_str(&*literal!(" is neither a seed nor a partial candidate!")); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
            Ok(indices)
        }

        let mut sparsityColoring: Arc<SparsityColoring>;
        let mut seeds: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>;
        let mut partials: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>;
        let mut seed_indices: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
        let mut partial_indices: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
        let mut sizeCols: i32;
        let mut sizeRows: i32;
        let mut idx_cref: Arc<ComponentRef::NFComponentRef>;
        let mut deps: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
        let mut cols: metamodelica::Array<Arc<metamodelica::List<i32>>>;
        let mut rows: metamodelica::Array<Arc<metamodelica::List<i32>>>;
        let mut colored_cols: metamodelica::Array<Arc<metamodelica::List<i32>>>;
        let mut colored_rows: metamodelica::Array<Arc<metamodelica::List<i32>>>;
        let mut cref_colored_cols: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut cref_colored_rows: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        seeds = metamodelica::arrayFromVec(sparsityPattern.seed_vars.clone().into_iter().cloned().collect());
        if jacType.clone() == JacobianType::NLS.clone() {
            partials = metamodelica::arrayFromVec(sparsityPattern.partial_vars.clone().into_iter().cloned().collect());
        } else {
            partials = metamodelica::arrayFromVec(({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut cref in (sparsityPattern.partial_vars.clone()).into_iter().cloned() {
            if !(isRowInJacobian(cref.clone(), jacType.clone())?) { continue; }
            let __x = cref.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect());
        }
        sizeCols = metamodelica::arrayLength(seeds.clone());
        sizeRows = metamodelica::arrayLength(partials.clone());
        seed_indices = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), Util::nextPrime(sizeCols.clone()));
        partial_indices = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), Util::nextPrime(sizeRows.clone()));
        for mut i in 1..=sizeCols.clone() {
            UnorderedMap::add(({let __elt = seeds.borrow()[(i.clone()-1) as usize].clone(); __elt}), i.clone(), seed_indices.clone())?;
        }
        for mut i in 1..=sizeRows.clone() {
            UnorderedMap::add(({let __elt = partials.borrow()[(i.clone()-1) as usize].clone(); __elt}), i.clone(), partial_indices.clone())?;
        }
        cols = arrayCreate(sizeCols.clone(), metamodelica::nil());
        rows = arrayCreate(sizeRows.clone(), metamodelica::nil());
        for mut tpl in &*sparsityPattern.col_wise_pattern.clone() {
            let mut tpl = tpl.clone();
            (idx_cref, deps) = tpl.clone();
            {
                let __cell0 = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut dep in (deps.clone()).into_iter().cloned() {
            let __x = UnorderedMap::getSafe(dep.clone(), partial_indices.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                let __idx0 = UnorderedMap::getSafe(idx_cref.clone(), seed_indices.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?;
                cols.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
            }
        }
        for mut tpl in &*sparsityPattern.row_wise_pattern.clone() {
            let mut tpl = tpl.clone();
            (idx_cref, deps) = tpl.clone();
            {
                let __cell1 = ({
        let mut __acc: Arc<metamodelica::List<i32>> = metamodelica::nil();
        for mut dep in (deps.clone()).into_iter().cloned() {
            let __x = getIndices(dep.clone(), seed_indices.clone(), partial_indices.clone(), rows.clone())?;
            __acc = __x.append(&__acc);
        }
        __acc
    });
                let __idx1 = UnorderedMap::getSafe(idx_cref.clone(), partial_indices.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?;
                rows.clone().borrow_mut()[(__idx1-1) as usize] = __cell1;
            }
        }
        colored_cols = Coloring::createColoring(rows.clone(), cols.clone(), sizeCols.clone(), sizeRows.clone())?;
        cref_colored_cols = arrayCreate(metamodelica::arrayLength(colored_cols.clone()), metamodelica::nil());
        for mut i in 1..=metamodelica::arrayLength(colored_cols.clone()) {
            {
                let __cell2 = ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut idx in (({let __elt = colored_cols.borrow()[(i.clone()-1) as usize].clone(); __elt})).into_iter().cloned() {
            let __x = ({let __elt = seeds.borrow()[(idx.clone()-1) as usize].clone(); __elt});
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                let __idx2 = i.clone();
                cref_colored_cols.clone().borrow_mut()[(__idx2-1) as usize] = __cell2;
            }
        }
        colored_rows = Coloring::createColoring(cols.clone(), rows.clone(), sizeRows.clone(), sizeCols.clone())?;
        cref_colored_rows = arrayCreate(metamodelica::arrayLength(colored_rows.clone()), metamodelica::nil());
        for mut i in 1..=metamodelica::arrayLength(colored_rows.clone()) {
            {
                let __cell3 = ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut idx in (({let __elt = colored_rows.borrow()[(i.clone()-1) as usize].clone(); __elt})).into_iter().cloned() {
            let __x = ({let __elt = partials.borrow()[(idx.clone()-1) as usize].clone(); __elt});
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                let __idx3 = i.clone();
                cref_colored_rows.clone().borrow_mut()[(__idx3-1) as usize] = __cell3;
            }
        }
        sparsityColoring = Arc::new(SparsityColoring { cols: cref_colored_cols.clone(), rows: cref_colored_rows.clone() });
        Ok(sparsityColoring)
    }

    pub(crate) fn PartialD2ColoringAlgColumnAndRow(mut sparsityPattern: Arc<SparsityPattern::SparsityPattern>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<Arc<SparsityColoring>> {
        let mut sparsityColoring: Arc<SparsityColoring>;
        let mut seed_nodes: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>;
        let mut partial_nodes: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>;
        let mut col_groups: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        let mut row_groups: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        let mut cols_arr: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut rows_arr: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        seed_nodes = metamodelica::arrayFromVec(sparsityPattern.seed_vars.clone().into_iter().cloned().collect());
        partial_nodes = metamodelica::arrayFromVec(sparsityPattern.partial_vars.clone().into_iter().cloned().collect());
        col_groups = GreedyPartialD2Color(seed_nodes.clone(), map.clone())?;
        row_groups = GreedyPartialD2Color(partial_nodes.clone(), map)?;
        cols_arr = metamodelica::arrayFromVec(col_groups.into_iter().cloned().collect());
        rows_arr = metamodelica::arrayFromVec(row_groups.into_iter().cloned().collect());
        sparsityColoring = Arc::new(SparsityColoring { cols: cols_arr.clone(), rows: rows_arr.clone() });
        Ok(sparsityColoring)
    }

    pub(crate) fn GreedyPartialD2Color(mut nodes: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>> {
        let mut groups_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>;
        let mut index_lookup: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
        let mut coloring: metamodelica::Array<i32>;
        let mut forbidden_colors: metamodelica::Array<i32>;
        let mut color_exists: metamodelica::Array<bool>;
        let mut groups: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut i: i32 = 0;
        let mut color: i32;
        let mut n: i32 = metamodelica::arrayLength(nodes.clone());
        let mut node: Arc<ComponentRef::NFComponentRef>;
        let mut mid: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        let mut neigh: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
        index_lookup = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), Util::nextPrime(n));
        for mut i in 1..=n {
            UnorderedMap::add(({let __elt = nodes.borrow()[(i-1) as usize].clone(); __elt}), i, index_lookup.clone())?;
        }
        coloring = arrayCreate(n, 0);
        forbidden_colors = arrayCreate(n, 0);
        color_exists = arrayCreate(n, false);
        groups = arrayCreate(n, metamodelica::nil());
        for mut i in 1..=n {
            node = ({let __elt = nodes.borrow()[(i-1) as usize].clone(); __elt});
            for mut mid in &*UnorderedMap::getSafe(node.clone(), map.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))? {
                let mut mid = mid.clone();
                for mut neigh in &*UnorderedMap::getSafe(mid.clone(), map.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))? {
                    let mut neigh = neigh.clone();
                    color = ({let __elt = coloring.borrow()[(UnorderedMap::getSafe(neigh.clone(), index_lookup.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?-1) as usize].clone(); __elt});
                    if color > 0 {
                        {
                            let __cell0 = i;
                            let __idx0 = color;
                            forbidden_colors.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
                        }
                    }
                }
            }
            color = 1;
            while ({let __elt = forbidden_colors.borrow()[(color-1) as usize].clone(); __elt}) == i {
                color = color + 1;
            }
            {
                let __cell1 = color;
                let __idx1 = i;
                coloring.clone().borrow_mut()[(__idx1-1) as usize] = __cell1;
            }
            {
                let __cell2 = true;
                let __idx2 = color;
                color_exists.clone().borrow_mut()[(__idx2-1) as usize] = __cell2;
            }
            {
                let __cell3 = metamodelica::cons(node.clone(), ({let __elt = groups.borrow()[(color-1) as usize].clone(); __elt}));
                let __idx3 = color;
                groups.clone().borrow_mut()[(__idx3-1) as usize] = __cell3;
            }
        }
        groups_lst = metamodelica::nil();
        for mut i in ({let __s=metamodelica::arrayLength(color_exists.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
            if ({let __elt = color_exists.borrow()[(i-1) as usize].clone(); __elt}) {
                groups_lst = metamodelica::cons(({let __elt = groups.borrow()[(i-1) as usize].clone(); __elt}), groups_lst.clone());
            }
        }
        Ok(groups_lst)
    }

    pub(crate) fn PartialD2ColoringAlg(mut sparsityPattern: Arc<SparsityPattern::SparsityPattern>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>) -> Result<Arc<SparsityColoring>> {
        let mut sparsityColoring: Arc<SparsityColoring>;
        let mut cref_lookup: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>;
        let mut index_lookup: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
        let mut color_exists: metamodelica::Array<bool>;
        let mut coloring: metamodelica::Array<i32>;
        let mut forbidden_colors: metamodelica::Array<i32>;
        let mut col_coloring: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut row_coloring: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut color: i32;
        let mut cols_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        let mut rows_lst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
        cref_lookup = metamodelica::arrayFromVec(sparsityPattern.seed_vars.clone().into_iter().cloned().collect());
        index_lookup = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), Util::nextPrime((sparsityPattern.seed_vars.clone().len() as i32)));
        for mut i in 1..=metamodelica::arrayLength(cref_lookup.clone()) {
            UnorderedMap::add(({let __elt = cref_lookup.borrow()[(i.clone()-1) as usize].clone(); __elt}), i.clone(), index_lookup.clone())?;
        }
        coloring = arrayCreate(metamodelica::arrayLength(cref_lookup.clone()), 0);
        forbidden_colors = arrayCreate(metamodelica::arrayLength(cref_lookup.clone()), 0);
        color_exists = arrayCreate(metamodelica::arrayLength(cref_lookup.clone()), false);
        col_coloring = arrayCreate(metamodelica::arrayLength(cref_lookup.clone()), metamodelica::nil());
        row_coloring = arrayCreate(metamodelica::arrayLength(cref_lookup.clone()), metamodelica::nil());
        for mut i in 1..=metamodelica::arrayLength(cref_lookup.clone()) {
            let __range0 = &*UnorderedMap::getSafe(({let __elt = cref_lookup.borrow()[(i.clone()-1) as usize].clone(); __elt}), map.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?;
            for mut row_var in __range0 {
                let mut row_var = row_var.clone();
                for mut col_var in &*UnorderedMap::getSafe(row_var.clone(), map.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))? {
                    let mut col_var = col_var.clone();
                    color = ({let __elt = coloring.borrow()[(UnorderedMap::getSafe(col_var.clone(), index_lookup.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?-1) as usize].clone(); __elt});
                    if color > 0 {
                        {
                            let __cell1 = i.clone();
                            let __idx1 = color;
                            forbidden_colors.clone().borrow_mut()[(__idx1-1) as usize] = __cell1;
                        }
                    }
                }
            }
            color = 1;
            while ({let __elt = forbidden_colors.borrow()[(color-1) as usize].clone(); __elt}) == i.clone() {
                color = color + 1;
            }
            {
                let __cell2 = color;
                let __idx2 = i.clone();
                coloring.clone().borrow_mut()[(__idx2-1) as usize] = __cell2;
            }
            {
                let __cell3 = listAppend(({let __elt = row_coloring.borrow()[(color-1) as usize].clone(); __elt}), UnorderedMap::getSafe(({let __elt = cref_lookup.borrow()[(i.clone()-1) as usize].clone(); __elt}), map.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?);
                let __idx3 = color;
                row_coloring.clone().borrow_mut()[(__idx3-1) as usize] = __cell3;
            }
            {
                let __cell4 = true;
                let __idx4 = color;
                color_exists.clone().borrow_mut()[(__idx4-1) as usize] = __cell4;
            }
        }
        for mut i in 1..=metamodelica::arrayLength(coloring.clone()) {
            {
                let __cell5 = metamodelica::cons(({let __elt = cref_lookup.borrow()[(i.clone()-1) as usize].clone(); __elt}), ({let __elt = col_coloring.borrow()[(({let __elt = coloring.borrow()[(i.clone()-1) as usize].clone(); __elt})-1) as usize].clone(); __elt}));
                let __idx5 = ({let __elt = coloring.borrow()[(i.clone()-1) as usize].clone(); __elt});
                col_coloring.clone().borrow_mut()[(__idx5-1) as usize] = __cell5;
            }
        }
        for mut i in ({let __s=metamodelica::arrayLength(color_exists.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
            if ({let __elt = color_exists.borrow()[(i.clone()-1) as usize].clone(); __elt}) {
                cols_lst = metamodelica::cons(({let __elt = col_coloring.borrow()[(i.clone()-1) as usize].clone(); __elt}), cols_lst.clone());
                rows_lst = metamodelica::cons(({let __elt = row_coloring.borrow()[(i.clone()-1) as usize].clone(); __elt}), rows_lst.clone());
            }
        }
        sparsityColoring = Arc::new(SparsityColoring { cols: metamodelica::arrayFromVec(cols_lst.into_iter().cloned().collect()), rows: metamodelica::arrayFromVec(rows_lst.into_iter().cloned().collect()) });
        Ok(sparsityColoring)
    }

    pub(crate) fn combine(mut coloring1: Arc<SparsityColoring>, mut coloring2: Arc<SparsityColoring>) -> Arc<SparsityColoring> {
        let mut coloring_out: Arc<SparsityColoring>;
        let mut cols_big: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut cols_small: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut rows_big: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        let mut rows_small: metamodelica::Array<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>;
        (cols_big, cols_small) = if (metamodelica::arrayLength(coloring2.cols.clone()) > metamodelica::arrayLength(coloring1.cols.clone())) {(coloring2.cols.clone(), coloring1.cols.clone())} else {(coloring1.cols.clone(), coloring2.cols.clone())};
        (rows_big, rows_small) = if (metamodelica::arrayLength(coloring2.rows.clone()) > metamodelica::arrayLength(coloring1.rows.clone())) {(coloring2.rows.clone(), coloring1.rows.clone())} else {(coloring1.rows.clone(), coloring2.rows.clone())};
        coloring_out = Arc::new(SparsityColoring { cols: cols_big.clone(), rows: rows_big.clone() });
        for mut i in 1..=metamodelica::arrayLength(cols_small.clone()) {
            {
                let __cell0 = listAppend(({let __elt = coloring_out.cols.borrow()[(i.clone()-1) as usize].clone(); __elt}), ({let __elt = cols_small.borrow()[(i.clone()-1) as usize].clone(); __elt}));
                let __idx0 = i.clone();
                coloring_out.cols.clone().borrow_mut()[(__idx0-1) as usize] = __cell0;
            }
        }
        for mut i in 1..=metamodelica::arrayLength(rows_small.clone()) {
            {
                let __cell1 = listAppend(({let __elt = coloring_out.rows.borrow()[(i.clone()-1) as usize].clone(); __elt}), ({let __elt = rows_small.borrow()[(i.clone()-1) as usize].clone(); __elt}));
                let __idx1 = i.clone();
                coloring_out.rows.clone().borrow_mut()[(__idx1-1) as usize] = __cell1;
            }
        }
        coloring_out
    }

}

// ToDo: all the DAEMode stuff is probably incorrect!
fn isRowInJacobian(mut cref: Arc<ComponentRef::NFComponentRef>, mut jacType: JacobianType) -> Result<bool> {
    let mut b: bool;
    b = NBVariable::checkCref(cref.clone(), (std::sync::Arc::new(fnptr!(NBVariable::isResidual, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))? || NBVariable::checkCref(cref.clone(), (std::sync::Arc::new(fnptr!(NBVariable::isStateDerivative, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))? && jacType != JacobianType::OPT_MRF.clone() && jacType != JacobianType::OPT_R0.clone() || jacType == JacobianType::OPT_LFG.clone() && NBVariable::checkCref(cref.clone(), (std::sync::Arc::new(fnptr!(NBVariable::isLfgFunction, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))? || jacType == JacobianType::OPT_MRF.clone() && NBVariable::checkCref(cref.clone(), (std::sync::Arc::new(fnptr!(NBVariable::isMrfFunction, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))? || jacType == JacobianType::OPT_R0.clone() && NBVariable::checkCref(cref, (std::sync::Arc::new(fnptr!(NBVariable::isInitialConstraint, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?;
    Ok(b)
}

// TODO: refactor with map
fn getOptimizableVars(mut variables: Arc<VariablePointers::VariablePointers>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut optimizable_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    for mut var_ptr in &*NBVariable::VariablePointers::toList(variables).unwrap() {
        let mut var_ptr = var_ptr.clone();
        if NBVariable::isOptimizable(var_ptr.clone()) {
            optimizable_vars = metamodelica::cons(var_ptr.clone(), optimizable_vars.clone());
        }
    }
    optimizable_vars
}

fn getSeedCandidatesDynamicOptimization(mut part: Arc<Partition::Partition::Partition>, mut all_knowns: Arc<VariablePointers::VariablePointers>, mut filter: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
    let mut unknowns: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut derivative_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut unknown_states: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    unknowns = getOptimizableVars(all_knowns);
    derivative_vars = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (NBVariable::VariablePointers::toList(part.unknowns.clone())?).into_iter().cloned() {
            if !(NBVariable::isStateDerivative(var.clone())) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    unknown_states = ({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut var in (derivative_vars).into_iter().cloned() {
            let __x = Util::getOption((NBVariable::getVarState(var.clone())).0)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    unknowns = listAppend(unknown_states, unknowns);
    unknowns = List::filterOnTrue(unknowns, filter.clone())?;
    Ok(unknowns)
}

fn getLfgPartialCandidates(mut part: Arc<Partition::Partition::Partition>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut partialCandidates: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut lagrange_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut derivative_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut path_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    for mut var_ptr in &*NBVariable::VariablePointers::toList(part.unknowns.clone()).unwrap() {
        let mut var_ptr = var_ptr.clone();
        if NBVariable::isLagrange(var_ptr.clone()) {
            lagrange_vars = metamodelica::cons(var_ptr.clone(), lagrange_vars.clone());
        } else if NBVariable::isStateDerivative(var_ptr.clone()) {
            derivative_vars = metamodelica::cons(var_ptr.clone(), derivative_vars.clone());
        } else if NBVariable::isPathConstraint(var_ptr.clone()) {
            path_vars = metamodelica::cons(var_ptr.clone(), path_vars.clone());
        }
    }
    partialCandidates = listAppend(lagrange_vars, listAppend(derivative_vars, path_vars)).reverse();
    partialCandidates
}

fn getMrfPartialCandidates(mut part: Arc<Partition::Partition::Partition>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut partialCandidates: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut mayer_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut final_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    for mut var_ptr in &*NBVariable::VariablePointers::toList(part.unknowns.clone()).unwrap() {
        let mut var_ptr = var_ptr.clone();
        if NBVariable::isMayer(var_ptr.clone()) {
            mayer_vars = metamodelica::cons(var_ptr.clone(), mayer_vars.clone());
        } else if NBVariable::isFinalConstraint(var_ptr.clone()) {
            final_vars = metamodelica::cons(var_ptr.clone(), final_vars.clone());
        }
    }
    partialCandidates = listAppend(mayer_vars, final_vars).reverse();
    partialCandidates
}

fn getR0PartialCandidates(mut part: Arc<Partition::Partition::Partition>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut partialCandidates: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    for mut var_ptr in &*NBVariable::VariablePointers::toList(part.unknowns.clone()).unwrap() {
        let mut var_ptr = var_ptr.clone();
        if NBVariable::isInitialConstraint(var_ptr.clone()) {
            partialCandidates = metamodelica::cons(var_ptr.clone(), partialCandidates.clone());
        }
    }
    partialCandidates = partialCandidates.reverse();
    partialCandidates
}

// TODO: before this is ever called, we should check if the variable / annotation pairs are even valid: e.g. path constraint with final time or so!
// add a module for optimization? where we check the model, may do some transformations etc?
fn partJacobianDynamicOptimization(mut part: Arc<Partition::Partition::Partition>, mut all_knowns: Arc<VariablePointers::VariablePointers>, mut name: ArcStr, mut func: Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> + 'static>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>) -> Result<(Option<Arc<Jacobian::NBackendDAE>>, Option<Arc<Jacobian::NBackendDAE>>, Option<Arc<Jacobian::NBackendDAE>>)> {
    let mut LFG_jacobian: Option<Arc<Jacobian::NBackendDAE>>;
    let mut MRF_jacobian: Option<Arc<Jacobian::NBackendDAE>>;
    let mut R0_jacobian: Option<Arc<Jacobian::NBackendDAE>>;
    let mut kind: Partition::Kind = Partition::Partition::getKind(part.clone());
    let mut staticAsContinuous: bool = true;
    let mut seedCandidates: Arc<VariablePointers::VariablePointers>;
    let mut partialCandidates: Arc<VariablePointers::VariablePointers>;
    partialCandidates = NBVariable::VariablePointers::fromList(listAppend(getLfgPartialCandidates(part.clone()), NBVariable::VariablePointers::toList(part.unknowns.clone())?), part.unknowns.scalarized.clone())?;
    seedCandidates = NBVariable::VariablePointers::fromList(getSeedCandidatesDynamicOptimization(part.clone(), all_knowns.clone(), (std::sync::Arc::new(fnptr!(NBVariable::isLfgVariable, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?, partialCandidates.scalarized.clone())?;
    LFG_jacobian = func((name.clone()).clone(), JacobianType::OPT_LFG.clone(), seedCandidates, partialCandidates, part.equations.clone(), part.strongComponents.clone(), part.adjacencyMatrix.clone(), funcMap.clone(), staticAsContinuous)?;
    partialCandidates = NBVariable::VariablePointers::fromList(listAppend(getMrfPartialCandidates(part.clone()), NBVariable::VariablePointers::toList(part.unknowns.clone())?), part.unknowns.scalarized.clone())?;
    seedCandidates = NBVariable::VariablePointers::fromList(getSeedCandidatesDynamicOptimization(part.clone(), all_knowns.clone(), (std::sync::Arc::new(fnptr!(NBVariable::isMrfVariable, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?, partialCandidates.scalarized.clone())?;
    MRF_jacobian = func((name.clone()).clone(), JacobianType::OPT_MRF.clone(), seedCandidates, partialCandidates, part.equations.clone(), part.strongComponents.clone(), part.adjacencyMatrix.clone(), funcMap.clone(), staticAsContinuous)?;
    partialCandidates = NBVariable::VariablePointers::fromList(listAppend(getR0PartialCandidates(part.clone()), NBVariable::VariablePointers::toList(part.unknowns.clone())?), part.unknowns.scalarized.clone())?;
    seedCandidates = NBVariable::VariablePointers::fromList(getSeedCandidatesDynamicOptimization(part.clone(), all_knowns, (std::sync::Arc::new(fnptr!(NBVariable::isR0Variable, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?, partialCandidates.scalarized.clone())?;
    R0_jacobian = func((name).clone(), JacobianType::OPT_R0.clone(), seedCandidates, partialCandidates, part.equations.clone(), part.strongComponents.clone(), part.adjacencyMatrix.clone(), funcMap, staticAsContinuous)?;
    Ok((LFG_jacobian, MRF_jacobian, R0_jacobian))
}

fn partJacobian(mut part: Arc<Partition::Partition::Partition>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut knowns: Arc<VariablePointers::VariablePointers>, mut name: ArcStr, mut func: Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, Option<Arc<Adjacency::Matrix::Matrix>>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> + 'static>) -> Result<Arc<Partition::Partition::Partition>> {
    let mut part: Arc<Partition::Partition::Partition> = part;
    let mut jacType: JacobianType;
    let mut unknowns: Arc<VariablePointers::VariablePointers>;
    let mut derivative_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut state_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut seedCandidates: Arc<VariablePointers::VariablePointers>;
    let mut partialCandidates: Arc<VariablePointers::VariablePointers>;
    let mut jacobian: Option<Arc<Jacobian::NBackendDAE>>;
    let mut LFG_jacobian: Option<Arc<Jacobian::NBackendDAE>> = None;
    let mut MRF_jacobian: Option<Arc<Jacobian::NBackendDAE>> = None;
    let mut R0_jacobian: Option<Arc<Jacobian::NBackendDAE>> = None;
    let mut kind: Partition::Kind = Partition::Partition::getKind(part.clone());
    let mut updated: bool = false;
    assign_field!(part.strongComponents = (match part.strongComponents.clone() {
        Some(mut comps) => {
            let mut tmp: Arc<StrongComponent::NBStrongComponent>;
            for mut i in 1..=metamodelica::arrayLength(comps.clone()) {
                (tmp, updated) = compJacobian(({let __elt = comps.borrow()[(i.clone()-1) as usize].clone(); __elt}), part.adjacencyMatrix.clone(), funcMap.clone(), kind)?;
                if updated {
                    metamodelica::arrayUpdate(comps.clone(), i.clone(), tmp.clone())?;
                }
            }
            Some(comps.clone())
        },
        _ => {
            part.strongComponents.clone()
        },
    }));
    if Partition::Partition::isODEorDAE(part.clone()) {
        partialCandidates = part.unknowns.clone();
        unknowns = if (Partition::Partition::getKind(part.clone()) == Partition::Kind::DAE.clone()) {Util::getOption(part.daeUnknowns.clone())?} else {part.unknowns.clone()};
        jacType = if (Partition::Partition::getKind(part.clone()) == Partition::Kind::DAE.clone()) {JacobianType::DAE.clone()} else {JacobianType::ODE.clone()};
        derivative_vars = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (NBVariable::VariablePointers::toList(unknowns)?).into_iter().cloned() {
            if !(NBVariable::isStateDerivative(var.clone())) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        state_vars = ({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut var in (derivative_vars).into_iter().cloned() {
            let __x = Util::getOption((NBVariable::getVarState(var.clone())).0)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        seedCandidates = NBVariable::VariablePointers::fromList(state_vars, partialCandidates.scalarized.clone())?;
        jacobian = func((name.clone()).clone(), jacType, seedCandidates, partialCandidates, part.equations.clone(), part.strongComponents.clone(), part.adjacencyMatrix.clone(), funcMap.clone(), Partition::kindIsInitial(kind))?;
        if Flags::getConfigBool(Flags::MOO_DYNAMIC_OPTIMIZATION.clone())? {
            (LFG_jacobian, MRF_jacobian, R0_jacobian) = partJacobianDynamicOptimization(part.clone(), knowns, (name).clone(), func.clone(), funcMap)?;
        }
        if isSome(jacobian.clone()) {
            if Jacobian::getIsAdjoint(Util::getOption(jacobian.clone())?)? {
                assign_field!(part.association = Arc::new(Partition::Association::Association::CONTINUOUS { kind: kind, jacobian: None, jacobianAdjoint: jacobian, LFG_jacobian: LFG_jacobian, MRF_jacobian: MRF_jacobian, R0_jacobian: R0_jacobian }));
            } else {
                assign_field!(part.association = Arc::new(Partition::Association::Association::CONTINUOUS { kind: kind, jacobian: jacobian, jacobianAdjoint: None, LFG_jacobian: LFG_jacobian, MRF_jacobian: MRF_jacobian, R0_jacobian: R0_jacobian }));
            }
        } else {
            assign_field!(part.association = Arc::new(Partition::Association::Association::CONTINUOUS { kind: kind, jacobian: None, jacobianAdjoint: None, LFG_jacobian: LFG_jacobian, MRF_jacobian: MRF_jacobian, R0_jacobian: R0_jacobian }));
        }
        if Flags::isSet(Flags::JAC_DUMP.clone())? {
            metamodelica::print((Partition::Partition::toString(part.clone(), 2)?).clone());
        }
    }
    Ok(part)
}

fn compJacobian(mut comp: Arc<StrongComponent::NBStrongComponent>, mut full: Option<Arc<Adjacency::Matrix::Matrix>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut kind: Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, bool)> {
    let mut comp: Arc<StrongComponent::NBStrongComponent> = comp;
    let mut updated: bool;
    let mut strict: Arc<Tearing::NBTearing> = Arc::new(<Tearing::NBTearing as ::std::default::Default>::default());
    let mut residual_comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut seed_candidates: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut residual_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut inner_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let staticAsContinuous: bool = Partition::kindIsInitial(kind);
    (comp, updated) = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::ALGEBRAIC_LOOP { strict: __esc_strict, .. } => {
            strict = (*__esc_strict).clone();
            residual_comps = ({
        let mut __acc: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
        for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
            let __x = StrongComponent::fromSolvedEquationSlice(eqn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            seed_candidates = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (strict.iteration_vars.clone()).into_iter().cloned() {
            let __x = Slice::getT(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            residual_vars = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
            let __x = NBEquation::Equation::getResidualVar(Slice::getT(eqn.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            inner_vars = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut comp in (strict.innerEquations.clone()).borrow().iter() {
            let __x = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (StrongComponent::getVariables(comp.clone())?).into_iter().cloned() {
            if !(NBVariable::isContinuous(var.clone(), staticAsContinuous)?) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = __x.append(&__acc);
        }
        __acc
    });
            assign_field!(strict.jac = nonlinear(NBVariable::VariablePointers::fromList(seed_candidates, false)?, NBVariable::VariablePointers::fromList(listAppend(residual_vars, inner_vars), false)?, NBEquation::EquationPointers::fromList(({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
            let __x = Slice::getT(eqn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, Array::appendList(strict.innerEquations.clone(), residual_comps)?, full, funcMap, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Partition::Partition::kindToString(kind)?); __mm_s.push_str(&*if (var_field!((*comp).linear, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone()) {literal!("_LS_JAC_")} else {literal!("_NLS_JAC_")}); __mm_s.push_str(&*intString(var_field!((*comp).idx, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone())); ArcStr::from(__mm_s) }).clone(), staticAsContinuous)?);
            assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; strict = strict.clone());
            if Flags::isSet(Flags::JAC_DUMP.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            (comp, true)
        },
        _ => (comp, false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((comp, updated))
}

fn jacobianSymbolic(mut name: ArcStr, mut jacType: JacobianType, mut seedCandidates: Arc<VariablePointers::VariablePointers>, mut partialCandidates: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut strongComponents: Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, mut full: Option<Arc<Adjacency::Matrix::Matrix>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut staticAsContinuous: bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> {
    let mut jacobian: Option<Arc<Jacobian::NBackendDAE>>;
    let mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut diffed_comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>;
    let mut seed_vars_ptr: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    let mut pDer_vars_ptr: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    let mut diff_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>> = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>;
    let mut idx: Pointer::Pointer<i32> = Pointer::create(0);
    let mut all_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut unknown_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut aux_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut alias_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut depend_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut res_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut tmp_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut seed_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut varDataJac: Arc<VarData::VarData>;
    let mut sparsityPattern: Arc<SparsityPattern::SparsityPattern>;
    let mut sparsityColoring: Arc<SparsityColoring::SparsityColoring>;
    let mut func: NBVariable::checkVar = getTmpFilterFunction(jacType)?;
    if isSome(strongComponents.clone()) {
        comps = ({
        let mut __acc: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
        for mut comp in (Util::getOption(strongComponents.clone())?).borrow().iter() {
            if !(!(StrongComponent::isDiscrete(comp.clone())?)) { continue; }
            let __x = comp.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    } else {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.jacobianSymbolic")); __mm_s.push_str(&*literal!(" failed because no strong components were given!")); ArcStr::from(__mm_s) }).clone()])?;
    }
    NBVariable::VariablePointers::mapPtr(seedCandidates.clone(), (std::sync::Arc::new({ let __pe_b1 = (name.clone()).clone(); let __pe_b2 = seed_vars_ptr.clone(); let __pe_b3 = diff_map.clone(); let __pe_b4: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, ArcStr) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> + 'static> = (std::sync::Arc::new(NBVariable::makeSeedVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, ArcStr) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> + 'static>); let __pe_b5 = staticAsContinuous; move |__pe_a0| makeVarTraverse(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<()> + 'static>))?;
    (res_vars, tmp_vars) = List::splitOnTrue(NBVariable::VariablePointers::toList(partialCandidates.clone())?, func.clone())?;
    (tmp_vars, _) = List::splitOnTrue(tmp_vars, (std::sync::Arc::new({ let __pe_b1 = staticAsContinuous; move |__pe_a0| NBVariable::isContinuous(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?;
    for mut v in &*res_vars {
        let mut v = v.clone();
        makeVarTraverse(v.clone(), (name.clone()).clone(), pDer_vars_ptr.clone(), diff_map.clone(), (std::sync::Arc::new({ let __pe_b2 = false; move |__pe_a0, __pe_a1| NBVariable::makePDerVar(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, ArcStr) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> + 'static>), staticAsContinuous)?;
    }
    res_vars = Pointer::access(pDer_vars_ptr);
    pDer_vars_ptr = Pointer::create(metamodelica::nil());
    for mut v in &*tmp_vars {
        let mut v = v.clone();
        makeVarTraverse(v.clone(), (name.clone()).clone(), pDer_vars_ptr.clone(), diff_map.clone(), (std::sync::Arc::new({ let __pe_b2 = true; move |__pe_a0, __pe_a1| NBVariable::makePDerVar(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, ArcStr) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> + 'static>), staticAsContinuous)?;
    }
    tmp_vars = Pointer::access(pDer_vars_ptr);
    diffArguments = Arc::new(DifferentiationArguments::DifferentiationArguments { diffCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(), new_vars: metamodelica::nil(), diff_map: Some(diff_map), diffType: Differentiate::DifferentiationType::JACOBIAN.clone(), funcMap: funcMap, scalarized: seedCandidates.scalarized.clone(), adjoint_map: None, current_grad: Arc::new(Expression::NFExpression::EMPTY { ty: openmodelica_nf_frontend::NFType::interned_REAL() }), collectAdjoints: false });
    (diffed_comps, diffArguments) = Differentiate::differentiateStrongComponentList(comps, diffArguments, idx, (name.clone()).clone(), literal!("NBJacobian.jacobianSymbolic"))?;
    unknown_vars = listAppend(res_vars.clone(), tmp_vars.clone());
    all_vars = unknown_vars.clone();
    seed_vars = Pointer::access(seed_vars_ptr);
    aux_vars = seed_vars.clone();
    alias_vars = metamodelica::nil();
    depend_vars = metamodelica::nil();
    varDataJac = Arc::new(VarData::VarData::VAR_DATA_JAC { variables: NBVariable::VariablePointers::fromList(all_vars, false)?, unknowns: NBVariable::VariablePointers::fromList(unknown_vars, false)?, auxiliaries: NBVariable::VariablePointers::fromList(aux_vars, false)?, aliasVars: NBVariable::VariablePointers::fromList(alias_vars, false)?, diffVars: partialCandidates.clone(), dependencies: NBVariable::VariablePointers::fromList(depend_vars, false)?, resultVars: NBVariable::VariablePointers::fromList(res_vars, false)?, tmpVars: NBVariable::VariablePointers::fromList(tmp_vars, false)?, seedVars: NBVariable::VariablePointers::fromList(seed_vars, false)? });
    if isSome(full) {
    } else {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.jacobianSymbolic")); __mm_s.push_str(&*literal!(" failed because full adjacency matrix to create sparsity pattern is missing.")); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    (sparsityPattern, sparsityColoring) = SparsityPattern::create(seedCandidates, partialCandidates, strongComponents, jacType)?;
    jacobian = Some(Arc::new(Jacobian::NBackendDAE::JACOBIAN { name: (name).clone(), jacType: jacType, varData: varDataJac, comps: metamodelica::arrayFromVec(diffed_comps.into_iter().cloned().collect()), sparsityPattern: sparsityPattern, sparsityColoring: sparsityColoring, isAdjoint: false }));
    Ok(jacobian)
}

fn sizeClassificationFromType(mut ty: Arc<Type::NFType>) -> SizeClassification {
    let mut sc: SizeClassification;
    sc = (match Type::dimensionCount(ty) {
        0 => SizeClassification::SCALAR.clone(),
        1 => SizeClassification::ELEMENT_WISE.clone(),
        2 => SizeClassification::MATRIX.clone(),
        _ => SizeClassification::ELEMENT_WISE.clone(),
    });
    sc
}

// Helper: build addition (or single term) expression from a list of terms for a given LHS cref.
fn buildAdjointRhs(mut lhsCref: Arc<ComponentRef::NFComponentRef>, mut terms: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<Arc<Expression::NFExpression>> {
    let mut rhs: Arc<Expression::NFExpression>;
    let mut vty: Arc<Type::NFType>;
    let mut sc: SizeClassification;
    let mut addOp: Arc<Operator::NFOperator>;
    vty = ComponentRef::getComponentType(lhsCref);
    if terms.clone().is_empty() {
        rhs = Expression::makeZero(vty)?;
        return Ok(rhs.clone());
    }
    if List::hasOneElement(terms.clone()) {
        rhs = listHead(terms)?;
        return Ok(rhs.clone());
    }
    sc = sizeClassificationFromType(vty.clone());
    addOp = Operator::fromClassification((MathClassification::ADDITION.clone(), sc), vty)?;
    rhs = SimplifyExp::simplify(Arc::new(Expression::NFExpression::MULTARY { arguments: terms, inv_arguments: metamodelica::nil(), operator: addOp }), false)?;
    rhs = Expression::map(rhs, (std::sync::Arc::new(Expression::repairOperator) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(rhs)
}

// Helper: run reverse-mode on a residual expression with a given seed (current_grad),
// accumulating into the provided adjoint_map. Returns updated DifferentiationArguments.
fn accumulateAdjointForResidual(mut residual: Arc<Expression::NFExpression>, mut seed: Arc<Expression::NFExpression>, mut diff_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>, mut funcMapIn: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut scalarized: bool, mut adjoint_map_in: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>) -> Result<Arc<DifferentiationArguments::DifferentiationArguments>> {
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments>;
    diffArguments = Arc::new(DifferentiationArguments::DifferentiationArguments { diffCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(), new_vars: metamodelica::nil(), diff_map: Some(diff_map), diffType: Differentiate::DifferentiationType::JACOBIAN.clone(), funcMap: funcMapIn, scalarized: scalarized, adjoint_map: Some(adjoint_map_in), current_grad: seed, collectAdjoints: true });
    (_, diffArguments) = Differentiate::differentiateExpression(residual, diffArguments)?;
    Ok(diffArguments)
}

// Reusable builder for a SINGLE_COMPONENT adjoint assignment (tmp or result var).
fn makeAdjointComponent(mut lhsKey: Arc<ComponentRef::NFComponentRef>, mut adjoint_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>, mut contextName: ArcStr, mut eqIndex: i32) -> Result<Arc<StrongComponent::NBStrongComponent>> {
    let mut diffed_comp: Arc<StrongComponent::NBStrongComponent> = Arc::new(<StrongComponent::NBStrongComponent as ::std::default::Default>::default());
    let mut terms: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut rhsExpr: Arc<Expression::NFExpression>;
    let mut eqPtr: Pointer::Pointer<Arc<Equation::Equation>>;
    let mut eq: Arc<Equation::Equation>;
    let mut lhsVarPtr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    terms = UnorderedMap::getOrFail(lhsKey.clone(), adjoint_map)?;
    rhsExpr = buildAdjointRhs(lhsKey.clone(), terms)?;
    eqPtr = NBEquation::Equation::makeAssignment(Expression::fromCref(lhsKey.clone(), false)?, rhsExpr, Pointer::create(eqIndex), (contextName).clone(), crate::NBEquation::Iterator::interned_EMPTY(), NBEquation::default(NBEquation::EquationKind::CONTINUOUS.clone(), false, None, None))?;
    lhsVarPtr = NBVariable::getVarPointer(lhsKey.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?;
    eq = Pointer::access(eqPtr.clone());
    diffed_comp = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ NBEquation::Equation::SCALAR_EQUATION { .. } => {
            if !(ComponentRef::subscriptsAllFlat(lhsKey.clone())?.is_empty()) {
                diffed_comp = Arc::new(StrongComponent::NBStrongComponent::SLICED_COMPONENT { var_cref: lhsKey, var: Arc::new(Slice::NBSlice { t: lhsVarPtr, indices: metamodelica::nil() }), eqn: Arc::new(Slice::NBSlice { t: eqPtr, indices: metamodelica::nil() }), status: NBSolve::Status::EXPLICIT.clone() });
            } else {
                diffed_comp = Arc::new(StrongComponent::NBStrongComponent::SINGLE_COMPONENT { var: lhsVarPtr, eqn: eqPtr, status: NBSolve::Status::EXPLICIT.clone() });
            }
            diffed_comp
        },
        Deref @ NBEquation::Equation::ARRAY_EQUATION { .. } => Arc::new(StrongComponent::NBStrongComponent::SINGLE_COMPONENT { var: lhsVarPtr, eqn: eqPtr, status: NBSolve::Status::EXPLICIT.clone() }),
        Deref @ NBEquation::Equation::RECORD_EQUATION { .. } => Arc::new(StrongComponent::NBStrongComponent::SINGLE_COMPONENT { var: lhsVarPtr, eqn: eqPtr, status: NBSolve::Status::EXPLICIT.clone() }),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.makeAdjointComponent")); __mm_s.push_str(&*literal!(" cannot create adjoint strong component for equation ")); __mm_s.push_str(&*NBEquation::Equation::toString(eq, (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(diffed_comp)
}

fn addEntryToLPAMap(mut vptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut diff_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>, mut loop_product_adjoint_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>) -> Result<()> {
    let mut mappedSeed: Option<Arc<ComponentRef::NFComponentRef>>;
    mappedSeed = UnorderedMap::get(NBVariable::getVarName(vptr), diff_map)?;
    if isSome(mappedSeed.clone()) {
        UnorderedMap::tryAdd(Util::getOption(mappedSeed)?, metamodelica::nil(), loop_product_adjoint_map)?;
    }
    Ok(())
}

// Build a filtered diff map for a given variable list.
// For each variable pointer v in 'vars', if there exists a mapping
//   base = BVariable.getVarName(v) -> mapped in 'globalDiffMap'
// then add (base -> mapped) to the returned map.
fn populateDiffMap(mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut globalDiffMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>> {
    let mut outMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>;
    let mut baseCref: Arc<ComponentRef::NFComponentRef>;
    let mut o_mappedCref: Option<Arc<ComponentRef::NFComponentRef>>;
    outMap = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), Util::nextPrime((vars.clone().len() as i32)));
    for mut vp in &*vars {
        let mut vp = vp.clone();
        baseCref = NBVariable::getVarName(vp.clone());
        o_mappedCref = UnorderedMap::get(baseCref.clone(), globalDiffMap.clone())?;
        if isSome(o_mappedCref.clone()) {
            UnorderedMap::add(baseCref.clone(), Util::getOption(o_mappedCref.clone())?, outMap.clone())?;
        }
    }
    Ok(outMap)
}

// Flattened across all components: preserve component order and in-component order
fn getAllAlgVars(mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    for mut c in &*comps {
        let mut c = c.clone();
        for mut v in &*({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut v in (StrongComponent::getVariables(c.clone()).unwrap()).into_iter().cloned() {
            if !(NBVariable::isAlgebraic(v.clone())) { continue; }
            let __x = v.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) {
            let mut v = v.clone();
            vars = metamodelica::cons(v.clone(), vars.clone());
        }
    }
    vars
}

fn jacobianSymbolicAdjoint(mut name: ArcStr, mut jacType: JacobianType, mut seedCandidates: Arc<VariablePointers::VariablePointers>, mut partialCandidates: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut strongComponents: Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, mut full: Option<Arc<Adjacency::Matrix::Matrix>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut staticAsContinuous: bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> {
    pub(crate) type ExpressionList = Arc<metamodelica::List<Arc<Expression::NFExpression>>>;

    let mut jacobian: Option<Arc<Jacobian::NBackendDAE>>;
    let mut comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>;
    let mut diffed_comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>;
    let mut comps_non_alg: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>>;
    let mut c_noalias: Arc<StrongComponent::NBStrongComponent>;
    let mut seed_vars_ptr: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    let mut pDer_vars_ptr: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    let mut diff_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>> = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    let mut diffArguments: Arc<DifferentiationArguments::DifferentiationArguments> = Arc::new(<DifferentiationArguments::DifferentiationArguments as ::std::default::Default>::default());
    let mut idx: Pointer::Pointer<i32> = Pointer::create(0);
    let mut all_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut unknown_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut aux_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut alias_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut depend_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut res_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut tmp_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut seed_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut old_res_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut varDataJac: Arc<VarData::VarData>;
    let mut sparsityPattern: Arc<SparsityPattern::SparsityPattern>;
    let mut sparsityColoring: Arc<SparsityColoring::SparsityColoring>;
    let mut i: i32;
    let mut newName: ArcStr;
    let mut newC: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut c: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut func: NBVariable::checkVar = getTmpFilterFunction(jacType)?;
    let mut adjoint_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>;
    let mut lhsVarPtr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut algebraicLoopComps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut tmpComps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut resComps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut orderedTmpCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut baseCref: Arc<ComponentRef::NFComponentRef>;
    let mut pDerCref: Arc<ComponentRef::NFComponentRef>;
    let mut o_pDerCref: Option<Arc<ComponentRef::NFComponentRef>>;
    newName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("_ADJ")); ArcStr::from(__mm_s) }).clone();
    if isSome(strongComponents.clone()) {
        comps = ({
        let mut __acc: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
        for mut comp in (Util::getOption(strongComponents.clone())?).borrow().iter() {
            if !(!(StrongComponent::isDiscrete(comp.clone())?)) { continue; }
            let __x = comp.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        for mut c in &*comps.clone() {
            let mut c = c.clone();
            if !(StrongComponent::isSingleComponent(c.clone())) && !(StrongComponent::isAlgebraicLoop(c.clone())) {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.jacobianSymbolicAdjoint")); __mm_s.push_str(&*literal!(" only supports SINGLE_COMPONENT and ALGEBRAIC_LOOP!")); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
        }
    } else {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.jacobianSymbolicAdjoint")); __mm_s.push_str(&*literal!(" failed because no strong components were given!")); ArcStr::from(__mm_s) }).clone()])?;
        bail!("fail");
    }
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Seed candidates before pDer creation:\n")); __mm_s.push_str(&*NBVariable::VariablePointers::toString(seedCandidates.clone(), (literal!("Seed Candidates")).clone(), None, true)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Partial candidates before pDer creation:\n")); __mm_s.push_str(&*NBVariable::VariablePointers::toString(partialCandidates.clone(), (literal!("Partial Candidates")).clone(), None, true)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    for mut v in &*NBVariable::VariablePointers::toList(seedCandidates.clone())? {
        let mut v = v.clone();
        makeVarTraverse(v.clone(), (newName.clone()).clone(), pDer_vars_ptr.clone(), diff_map.clone(), (std::sync::Arc::new({ let __pe_b2 = false; move |__pe_a0, __pe_a1| NBVariable::makePDerVar(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, ArcStr) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> + 'static>), staticAsContinuous)?;
    }
    res_vars = Pointer::access(pDer_vars_ptr);
    (old_res_vars, tmp_vars) = List::splitOnTrue(NBVariable::VariablePointers::toList(partialCandidates.clone())?, func.clone())?;
    (tmp_vars, _) = List::splitOnTrue(tmp_vars, (std::sync::Arc::new({ let __pe_b1 = staticAsContinuous; move |__pe_a0| NBVariable::isContinuous(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?;
    for mut v in &*old_res_vars {
        let mut v = v.clone();
        makeVarTraverse(v.clone(), (newName.clone()).clone(), seed_vars_ptr.clone(), diff_map.clone(), (std::sync::Arc::new(NBVariable::makeSeedVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, ArcStr) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> + 'static>), staticAsContinuous)?;
    }
    seed_vars = Pointer::access(seed_vars_ptr.clone());
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("seed vars after seed creation:\n")); __mm_s.push_str(&*NBVariable::VariablePointers::toString(NBVariable::VariablePointers::fromList(seed_vars, false)?, (literal!("Seed Vars")).clone(), None, true)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("res vars after pDer creation:\n")); __mm_s.push_str(&*NBVariable::VariablePointers::toString(NBVariable::VariablePointers::fromList(res_vars.clone(), false)?, (literal!("Res Vars")).clone(), None, true)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("tmp vars after pDer creation:\n")); __mm_s.push_str(&*NBVariable::VariablePointers::toString(NBVariable::VariablePointers::fromList(tmp_vars.clone(), false)?, (literal!("Tmp Vars")).clone(), None, true)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    pDer_vars_ptr = Pointer::create(metamodelica::nil());
    for mut v in &*tmp_vars {
        let mut v = v.clone();
        makeVarTraverse(v.clone(), (newName.clone()).clone(), pDer_vars_ptr.clone(), diff_map.clone(), (std::sync::Arc::new({ let __pe_b2 = true; move |__pe_a0, __pe_a1| NBVariable::makePDerVar(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, ArcStr) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> + 'static>), staticAsContinuous)?;
    }
    tmp_vars = Pointer::access(pDer_vars_ptr);
    adjoint_map = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), (res_vars.clone().len() as i32) + (tmp_vars.clone().len() as i32));
    for mut v in &*res_vars.clone() {
        let mut v = v.clone();
        UnorderedMap::tryAdd(NBVariable::getVarName(v.clone()), metamodelica::nil(), adjoint_map.clone())?;
    }
    for mut v in &*tmp_vars.clone() {
        let mut v = v.clone();
        UnorderedMap::tryAdd(NBVariable::getVarName(v.clone()), metamodelica::nil(), adjoint_map.clone())?;
    }
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Adjoint map before:\n")); __mm_s.push_str(&*adjointMapToString(Some(adjoint_map.clone()))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Diff map before:\n")); __mm_s.push_str(&*diffMapToString(diff_map.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    comps_non_alg = metamodelica::nil();
    for mut c in &*comps {
        let mut c = c.clone();
        c_noalias = StrongComponent::removeAlias(c.clone());
        let () = ({
        let mut itVarPtrs: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        let mut lambdaPtrs: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        let mut lambdaCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        let mut diff_map_y: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>> = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        let mut diff_map_x: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>> = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        let mut diff_map_union: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>> = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        let mut loop_product_adjoint_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>> = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        let mut linResEqnPtrs: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        (::match_deref::match_deref! { match &(c_noalias.clone()) {
        Deref @ StrongComponent::ALGEBRAIC_LOOP { strict: tearing, .. } => {
            let mut residuals: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
            let mut iRes: i32;
            let mut terms_x: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
            let mut rhs_x: Arc<Expression::NFExpression>;
            let mut baseX: Arc<ComponentRef::NFComponentRef>;
            let mut pDerX: Arc<ComponentRef::NFComponentRef>;
            let mut o_pDerX: Option<Arc<ComponentRef::NFComponentRef>>;
            let mut seedPtrListX: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
            let mut terms_j: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
            let mut lhs_j: Arc<Expression::NFExpression>;
            let mut rhs_j: Arc<Expression::NFExpression>;
            let mut resid_j: Pointer::Pointer<Arc<Equation::Equation>>;
            let mut o_ySeedCref: Option<Arc<ComponentRef::NFComponentRef>>;
            let mut ySeedCref: Arc<ComponentRef::NFComponentRef>;
            itVarPtrs = Tearing::getIterationVars(tearing.clone());
            residuals = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut e in (Tearing::getResidualEqns(tearing.clone())).into_iter().cloned() {
            let __x = NBEquation::Equation::getResidualExp(Pointer::access(e.clone()), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            for mut iIdx in 1..=(residuals.clone().len() as i32) {
                (lhsVarPtr, newC) = NBVariable::makeAuxVar((arcstr::literal!(NBVariable::TEMPORARY_STR)).clone(), Pointer::access(idx.clone()) + 1, openmodelica_nf_frontend::NFType::interned_REAL(), false)?;
                Pointer::update(idx.clone(), Pointer::access(idx.clone()) + 1);
                (newC, lhsVarPtr) = NBVariable::makePDerVar(newC.clone(), (newName.clone()).clone(), true)?;
                lambdaPtrs = metamodelica::cons(lhsVarPtr.clone(), lambdaPtrs.clone());
                lambdaCrefs = metamodelica::cons(newC.clone(), lambdaCrefs.clone());
                if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
                    metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[adjoint] created lambda_")); __mm_s.push_str(&*intString(iIdx.clone())); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ComponentRef::toString(newC.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
            }
            tmp_vars = List::append_reverse(lambdaPtrs.clone(), tmp_vars.clone());
            lambdaPtrs = lambdaPtrs.clone().reverse();
            lambdaCrefs = lambdaCrefs.clone().reverse();
            diff_map_y = populateDiffMap(itVarPtrs.clone(), diff_map.clone())?;
            seedPtrListX = NBVariable::VariablePointers::toList(seedCandidates.clone())?;
            diff_map_x = populateDiffMap(seedPtrListX.clone(), diff_map.clone())?;
            diff_map_union = UnorderedMap::merge(diff_map_y.clone(), diff_map_x.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?;
            for mut itVarPtr in &*itVarPtrs.clone() {
                let mut itVarPtr = itVarPtr.clone();
                addEntryToLPAMap(itVarPtr.clone(), diff_map_y.clone(), loop_product_adjoint_map.clone())?;
            }
            for mut seedVarPtr in &*seedPtrListX.clone() {
                let mut seedVarPtr = seedVarPtr.clone();
                addEntryToLPAMap(seedVarPtr.clone(), diff_map_x.clone(), loop_product_adjoint_map.clone())?;
            }
            iRes = 1;
            for mut residual_i in &*residuals.clone() {
                let mut residual_i = residual_i.clone();
                if iRes.clone() > (lambdaCrefs.clone().len() as i32) {
                    break;
                }
                diffArguments = accumulateAdjointForResidual(residual_i.clone(), Expression::fromCref((lambdaCrefs.clone()).get(iRes.clone())?, false)?, diff_map_union.clone(), funcMap.clone(), seedCandidates.scalarized.clone(), loop_product_adjoint_map.clone())?;
                loop_product_adjoint_map = Util::getOption(diffArguments.adjoint_map.clone())?;
                iRes = iRes.clone() + 1;
            }
            if Flags::isSet(Flags::DEBUG_DIFFERENTIATION.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[adjoint] loop_product_adjoint_map after: \n")); __mm_s.push_str(&*adjointMapToString(Some(loop_product_adjoint_map.clone()))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            for mut vptr in &*itVarPtrs.clone() {
                let mut vptr = vptr.clone();
                o_ySeedCref = UnorderedMap::get(NBVariable::getVarName(vptr.clone()), diff_map_y.clone())?;
                if isSome(o_ySeedCref.clone()) {
                    ySeedCref = Util::getOption(o_ySeedCref.clone())?;
                    terms_j = UnorderedMap::getOrDefault(ySeedCref.clone(), loop_product_adjoint_map.clone(), metamodelica::nil())?;
                    lhs_j = buildAdjointRhs(ySeedCref.clone(), terms_j.clone())?;
                    rhs_j = Expression::fromCref(ySeedCref.clone(), false)?;
                    resid_j = NBEquation::Equation::makeAssignment(lhs_j.clone(), rhs_j.clone(), idx.clone(), (newName.clone()).clone(), crate::NBEquation::Iterator::interned_EMPTY(), NBEquation::default(NBEquation::EquationKind::CONTINUOUS.clone(), false, None, None))?;
                    linResEqnPtrs = metamodelica::cons(NBEquation::Equation::createResidual(resid_j.clone(), None, false, false)?, linResEqnPtrs.clone());
                } else {
                    continue;
                }
            }
            linResEqnPtrs = linResEqnPtrs.clone().reverse();
            if !(linResEqnPtrs.clone().is_empty()) {
                algebraicLoopComps = metamodelica::cons(makeLinearAlgebraicLoop(lambdaPtrs.clone(), linResEqnPtrs.clone(), None, false, false)?, algebraicLoopComps.clone());
            }
            for mut seedVarPtrX in &*seedPtrListX.clone() {
                let mut seedVarPtrX = seedVarPtrX.clone();
                baseX = NBVariable::getVarName(seedVarPtrX.clone());
                o_pDerX = UnorderedMap::get(baseX.clone(), diff_map_x.clone())?;
                if isSome(o_pDerX.clone()) {
                    pDerX = Util::getOption(o_pDerX.clone())?;
                    terms_x = UnorderedMap::getOrDefault(pDerX.clone(), loop_product_adjoint_map.clone(), metamodelica::nil())?;
                    if terms_x.clone().is_empty() {
                        continue;
                    }
                    rhs_x = Expression::negate(buildAdjointRhs(pDerX.clone(), terms_x.clone())?);
                    UnorderedMap::add(pDerX.clone(), metamodelica::cons(rhs_x.clone(), UnorderedMap::getOrDefault(pDerX.clone(), adjoint_map.clone(), metamodelica::nil())?), adjoint_map.clone())?;
                }
            }
            ()
        },
        _ => {
            comps_non_alg = metamodelica::cons(c_noalias.clone(), comps_non_alg.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    }
    comps = comps_non_alg.reverse();
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Adjoint map after loop adding:\n")); __mm_s.push_str(&*adjointMapToString(Some(adjoint_map.clone()))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    diffArguments = Arc::new(DifferentiationArguments::DifferentiationArguments { diffCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(), new_vars: metamodelica::nil(), diff_map: Some(diff_map.clone()), diffType: Differentiate::DifferentiationType::JACOBIAN.clone(), funcMap: funcMap, scalarized: seedCandidates.scalarized.clone(), adjoint_map: Some(adjoint_map), current_grad: Arc::new(Expression::NFExpression::EMPTY { ty: openmodelica_nf_frontend::NFType::interned_REAL() }), collectAdjoints: true });
    (_, diffArguments) = Differentiate::differentiateStrongComponentListAdjoint(comps.clone(), diffArguments, idx, (newName.clone()).clone(), literal!("NBJacobian.jacobianSymbolicAdjoint"))?;
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Adjoint map after differentiation:\n")); __mm_s.push_str(&*adjointMapToString(diffArguments.adjoint_map.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    adjoint_map = Util::getOption(diffArguments.adjoint_map.clone())?;
    diffed_comps = metamodelica::nil();
    i = 1;
    for mut v in &*getAllAlgVars(comps) {
        let mut v = v.clone();
        baseCref = NBVariable::getVarName(v.clone());
        o_pDerCref = UnorderedMap::get(baseCref.clone(), diff_map.clone())?;
        if isSome(o_pDerCref.clone()) {
            pDerCref = Util::getOption(o_pDerCref.clone())?;
            if UnorderedMap::contains(pDerCref.clone(), adjoint_map.clone())? {
                orderedTmpCrefs = metamodelica::cons(pDerCref.clone(), orderedTmpCrefs.clone());
            }
        }
    }
    for mut lhsKey in &*orderedTmpCrefs.clone() {
        let mut lhsKey = lhsKey.clone();
        tmpComps = metamodelica::cons(makeAdjointComponent(lhsKey.clone(), adjoint_map.clone(), (newName.clone()).clone(), i)?, tmpComps.clone());
        i = i + 1;
    }
    for mut v in &*tmp_vars.clone() {
        let mut v = v.clone();
        baseCref = NBVariable::getVarName(v.clone());
        if !(List::contains(orderedTmpCrefs.clone(), baseCref.clone(), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?) && UnorderedMap::contains(baseCref.clone(), adjoint_map.clone())? {
            tmpComps = metamodelica::cons(makeAdjointComponent(baseCref.clone(), adjoint_map.clone(), (newName.clone()).clone(), i)?, tmpComps.clone());
            i = i + 1;
        }
    }
    resComps = metamodelica::nil();
    for mut v in &*res_vars.clone() {
        let mut v = v.clone();
        c = NBVariable::getVarName(v.clone());
        if UnorderedMap::contains(c.clone(), adjoint_map.clone())? {
            resComps = metamodelica::cons(makeAdjointComponent(c.clone(), adjoint_map.clone(), (newName.clone()).clone(), i)?, resComps.clone());
            i = i + 1;
        }
    }
    diffed_comps = listAppend(tmpComps, listAppend(algebraicLoopComps, resComps));
    unknown_vars = listAppend(res_vars.clone(), tmp_vars.clone());
    all_vars = unknown_vars.clone();
    seed_vars = Pointer::access(seed_vars_ptr);
    aux_vars = seed_vars.clone();
    alias_vars = metamodelica::nil();
    depend_vars = metamodelica::nil();
    varDataJac = Arc::new(VarData::VarData::VAR_DATA_JAC { variables: NBVariable::VariablePointers::fromList(all_vars, false)?, unknowns: NBVariable::VariablePointers::fromList(unknown_vars, false)?, auxiliaries: NBVariable::VariablePointers::fromList(aux_vars, false)?, aliasVars: NBVariable::VariablePointers::fromList(alias_vars, false)?, diffVars: partialCandidates.clone(), dependencies: NBVariable::VariablePointers::fromList(depend_vars, false)?, resultVars: NBVariable::VariablePointers::fromList(res_vars, false)?, tmpVars: NBVariable::VariablePointers::fromList(tmp_vars, false)?, seedVars: NBVariable::VariablePointers::fromList(seed_vars, false)? });
    (sparsityPattern, sparsityColoring) = SparsityPattern::create(seedCandidates, partialCandidates, strongComponents, jacType)?;
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        metamodelica::print((literal!("Adjoint sparsity pattern and coloring:\n")).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*SparsityPattern::toString(sparsityPattern.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*SparsityColoring::toString(sparsityColoring.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    jacobian = Some(Arc::new(Jacobian::NBackendDAE::JACOBIAN { name: (newName).clone(), jacType: jacType, varData: varDataJac, comps: metamodelica::arrayFromVec(diffed_comps.into_iter().cloned().collect()), sparsityPattern: sparsityPattern, sparsityColoring: sparsityColoring, isAdjoint: true }));
    Ok(jacobian)
}

fn jacobianNumeric(mut name: ArcStr, mut jacType: JacobianType, mut seedCandidates: Arc<VariablePointers::VariablePointers>, mut partialCandidates: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut strongComponents: Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, mut full: Option<Arc<Adjacency::Matrix::Matrix>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut staticAsContinuous: bool) -> Result<Option<Arc<Jacobian::NBackendDAE>>> {
    let mut jacobian: Option<Arc<Jacobian::NBackendDAE>>;
    let mut varDataJac: Arc<VarData::VarData>;
    let mut sparsityPattern: Arc<SparsityPattern::SparsityPattern>;
    let mut sparsityColoring: Arc<SparsityColoring::SparsityColoring>;
    let mut res_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut tmp_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut func: NBVariable::checkVar = getTmpFilterFunction(jacType)?;
    (res_vars, tmp_vars) = List::splitOnTrue(NBVariable::VariablePointers::toList(partialCandidates.clone())?, func.clone())?;
    (tmp_vars, _) = List::splitOnTrue(tmp_vars, (std::sync::Arc::new({ let __pe_b1 = staticAsContinuous; move |__pe_a0| NBVariable::isContinuous(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?;
    varDataJac = Arc::new(VarData::VarData::VAR_DATA_JAC { variables: NBVariable::VariablePointers::fromList(metamodelica::nil(), false)?, unknowns: partialCandidates.clone(), auxiliaries: NBVariable::VariablePointers::fromList(metamodelica::nil(), false)?, aliasVars: NBVariable::VariablePointers::fromList(metamodelica::nil(), false)?, diffVars: NBVariable::VariablePointers::fromList(metamodelica::nil(), false)?, dependencies: NBVariable::VariablePointers::fromList(metamodelica::nil(), false)?, resultVars: NBVariable::VariablePointers::fromList(res_vars, false)?, tmpVars: NBVariable::VariablePointers::fromList(tmp_vars, false)?, seedVars: seedCandidates.clone() });
    (sparsityPattern, sparsityColoring) = SparsityPattern::create(seedCandidates, partialCandidates, strongComponents, jacType)?;
    jacobian = Some(Arc::new(Jacobian::NBackendDAE::JACOBIAN { name: (name).clone(), jacType: jacType, varData: varDataJac, comps: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), sparsityPattern: sparsityPattern, sparsityColoring: sparsityColoring, isAdjoint: false }));
    Ok(jacobian)
}

fn jacobianNone(mut name: ArcStr, mut jacType: JacobianType, mut seedCandidates: Arc<VariablePointers::VariablePointers>, mut partialCandidates: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut strongComponents: Option<metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>>, mut full: Option<Arc<Adjacency::Matrix::Matrix>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut staticAsContinuous: bool) -> Option<Arc<Jacobian::NBackendDAE>> {
    let mut jacobian: Option<Arc<Jacobian::NBackendDAE>>;
    jacobian = None;
    jacobian
}

fn getTmpFilterFunction(mut jacType: JacobianType) -> Result<Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>> {
    let mut func: NBVariable::checkVar;
    func = (match jacType {
        JacobianType::ODE => (std::sync::Arc::new(fnptr!(NBVariable::isStateDerivative, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>),
        JacobianType::DAE { .. } => (std::sync::Arc::new(fnptr!(NBVariable::isResidual, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>),
        JacobianType::LS => (std::sync::Arc::new(fnptr!(NBVariable::isResidual, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>),
        JacobianType::NLS => (std::sync::Arc::new(fnptr!(NBVariable::isResidual, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>),
        JacobianType::OPT_LFG => (std::sync::Arc::new(fnptr!(NBVariable::isLfgFunction, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>),
        JacobianType::OPT_MRF => (std::sync::Arc::new(fnptr!(NBVariable::isMrfFunction, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>),
        JacobianType::OPT_R0 => (std::sync::Arc::new(fnptr!(NBVariable::isInitialConstraint, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.getTmpFilterFunction")); __mm_s.push_str(&*literal!(" failed because jacobian type is not known: ")); __mm_s.push_str(&*jacobianTypeString(jacType)); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
    });
    Ok(func)
}

fn makeVarTraverse(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut name: ArcStr, mut vars_ptr: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>, mut makeVar: Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, ArcStr) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> + 'static>, mut staticAsContinuous: bool) -> Result<()> {
    pub type Func = std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, ArcStr) -> Result<(Arc<ComponentRef::NFComponentRef>, Pointer::Pointer<Arc<Variable::NFVariable>>)> + 'static>;

    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut diff: Arc<ComponentRef::NFComponentRef>;
    let mut parent_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut diff_parent_name: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut diff_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut parent: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut diff_parent: Pointer::Pointer<Arc<Variable::NFVariable>>;
    if NBVariable::isContinuous(var_ptr.clone(), staticAsContinuous)? {
        (diff, diff_ptr) = makeVar(var.name.clone(), (name.clone()).clone())?;
        Pointer::update(vars_ptr.clone(), metamodelica::cons(diff_ptr.clone(), Pointer::access(vars_ptr)));
        UnorderedMap::add(var.name.clone(), diff, map.clone())?;
        let () = (match NBVariable::getParent(var_ptr) {
        Some(mut __esc_parent) => {
            parent = __esc_parent.clone();
            parent_name = NBVariable::getVarName(parent.clone());
            diff_parent = (::match_deref::match_deref! { match &(UnorderedMap::get(parent_name.clone(), map.clone())?) {
        Some(__esc_diff_parent_name) => {
            diff_parent_name = (*__esc_diff_parent_name).clone();
            NBVariable::getVarPointer(diff_parent_name.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?
        },
        _ => {
            (diff_parent_name, _) = makeVar(parent_name.clone(), (name).clone())?;
            UnorderedMap::add(parent_name, diff_parent_name.clone(), map)?;
            NBVariable::getVarPointer(diff_parent_name.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            NBVariable::addRecordChild(diff_parent.clone(), diff_ptr.clone())?;
            diff_ptr = NBVariable::setParent(diff_ptr, diff_parent);
            ()
        },
        _ => (),
    });
    }
    Ok(())
}

fn adjointMapToString(mut adjoint_map: Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>>) -> Result<ArcStr> {
    fn valueToString(mut elst: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<ArcStr> {
        let mut vstr: ArcStr;
        vstr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (elst).into_iter().cloned() {
            let __x = Expression::toString(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(", ")).clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
        Ok(vstr)
    }

    let mut r#str: ArcStr;
    let mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>>>;
    if isNone(adjoint_map.clone()) {
        r#str = (literal!("{}")).clone();
        return Ok(r#str.clone());
    }
    let __pa0 = ::match_deref::match_deref! { match &(adjoint_map.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    map = __pa0.clone();
    r#str = (UnorderedMap::toString(map.clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(valueToString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Result<ArcStr> + 'static>), (literal!("\n  ")).clone(), (literal!(" -> ")).clone())?).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{\n  ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n}")); ArcStr::from(__mm_s) }).clone();
    Ok(r#str)
}

fn diffMapToString(mut map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = (UnorderedMap::toString(map, (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("\n  ")).clone(), (literal!(" -> ")).clone())?).clone();
    s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{\n  ")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!("\n}")); ArcStr::from(__mm_s) }).clone();
    Ok(s)
}

fn makeLinearAlgebraicLoop(mut itVarPtrs: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut resEqnPtrs: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>, mut jac: Option<Arc<Jacobian::NBackendDAE>>, mut mixed: bool, mut homotopy: bool) -> Result<Arc<StrongComponent::NBStrongComponent>> {
    let mut comp: Arc<StrongComponent::NBStrongComponent>;
    let mut m1: i32 = (itVarPtrs.clone().len() as i32);
    let mut m2: i32 = (resEqnPtrs.clone().len() as i32);
    let mut itVars_s: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>;
    let mut res_s: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>;
    let mut tearingSet: Arc<Tearing::NBTearing>;
    if m1 != m2 {
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("makeLinearAlgebraicLoop: |vars| != |eqns|")).clone()])?;
        bail!("fail");
    }
    itVars_s = ({
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
        for mut vp in (itVarPtrs).into_iter().cloned() {
            let __x = Arc::new(Slice::NBSlice { t: vp.clone(), indices: metamodelica::nil() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res_s = ({
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
        for mut ep in (resEqnPtrs).into_iter().cloned() {
            let __x = Arc::new(Slice::NBSlice { t: ep.clone(), indices: metamodelica::nil() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    tearingSet = Arc::new(Tearing::NBTearing { iteration_vars: itVars_s, residual_eqns: res_s, innerEquations: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), jac: jac });
    comp = Arc::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP { idx: -1, strict: tearingSet, casual: None, linear: true, mixed: mixed, homotopy: homotopy, status: NBSolve::Status::IMPLICIT.clone() });
    Ok(comp)
}

