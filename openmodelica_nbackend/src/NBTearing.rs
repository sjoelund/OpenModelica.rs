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
use crate::NBAdjacency::Solvability;
use crate::NBBackendUtil as BackendUtil;
use crate::NBCausalize as Causalize;
use crate::NBDifferentiate as Differentiate;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::EqnSlice;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBInitialization as Initialization;
use crate::NBInline as Inline;
use crate::NBJacobian as BJacobian;
use crate::NBMatching as Matching;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBSlice as Slice;
use crate::NBSolve as Solve;
use crate::NBSorting as Sorting;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VarSlice;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use crate::NBackendDAE as Jacobian;
use openmodelica_ast::Absyn::Path;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFBackendExtension;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
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

/// file:        NBTearing.mo
/// package:     NBTearing
/// description: This file contains the data-types used for tearing. It is a
///              uniontype and therefore also contains some structures for tearing.
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NBTearing {
    /// the variables used for iteration
    pub iteration_vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>>,
    /// implicitely solved residual equations
    pub residual_eqns: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>,
    /// array of matched equations and variables
    pub innerEquations: metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>,
    /// optional jacobian
    pub jac: Option<Arc<Jacobian::NBackendDAE>>,
}

impl metamodelica::gc::MMTrace for NBTearing {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.iteration_vars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.residual_eqns, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.innerEquations, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.jac, __mmv)?;
        Ok(())
    }
}
impl Default for NBTearing {
    fn default() -> Self {
        Self {
            iteration_vars: Default::default(),
            residual_eqns: Default::default(),
            innerEquations: Default::default(),
            jac: Default::default(),
        }
    }
}

pub type TEARING_SET = NBTearing;

pub(crate) fn hash(mut set: Arc<NBTearing>) -> Result<i32> {
    let mut h: i32 = ({
        let mut __acc: i32 = 0;
        for mut var in (set.iteration_vars.clone()).into_iter().cloned() {
            let __x = Slice::hash(var.clone(), (std::sync::Arc::new(BVariable::hash) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>))?;
            __acc += __x;
        }
        __acc
    });
    Ok(h)
}

pub(crate) fn isEqual(mut set1: Arc<NBTearing>, mut set2: Arc<NBTearing>) -> Result<bool> {
    let mut b: bool;
    b = UnorderedSet::equal_list(set1.residual_eqns.clone(), set2.residual_eqns.clone(), (std::sync::Arc::new({ let __pe_b1 = (std::sync::Arc::new(BEquation::Equation::hash) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<i32> + 'static>); move |__pe_a0| Slice::hash(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<i32> + 'static>), (std::sync::Arc::new({ let __pe_b2 = (std::sync::Arc::new(BEquation::Equation::isEqualPtr) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>, Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| Slice::isEqual(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?;
    b = if (b) {Array::isEqualOnTrue(set1.innerEquations.clone(), set2.innerEquations.clone(), (std::sync::Arc::new(StrongComponent::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<StrongComponent::NBStrongComponent>) -> Result<bool> + 'static>))?} else {b};
    b = if (b) {UnorderedSet::equal_list(set1.iteration_vars.clone(), set2.iteration_vars.clone(), (std::sync::Arc::new({ let __pe_b1 = (std::sync::Arc::new(BVariable::hash) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>); move |__pe_a0| Slice::hash(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<i32> + 'static>), (std::sync::Arc::new({ let __pe_b2 = (std::sync::Arc::new(BVariable::equalName) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1| Slice::isEqual(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>))?} else {b};
    Ok(b)
}

pub(crate) fn size(mut set: Arc<NBTearing>, mut resize: bool) -> Result<i32> {
    let mut s: i32;
    s = ({
        let mut __acc: i32 = 0;
        for mut eq in (set.residual_eqns.clone()).into_iter().cloned() {
            let __x = Slice::size(eq.clone(), (std::sync::Arc::new({ let __pe_b1 = resize; move |__pe_a0| BEquation::Equation::size(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<i32> + 'static>))?;
            __acc += __x;
        }
        __acc
    });
    s = s + ({
        let mut __acc: i32 = 0;
        for mut eq in (set.innerEquations.clone()).borrow().iter() {
            let __x = StrongComponent::size(eq.clone(), resize)?;
            __acc += __x;
        }
        __acc
    });
    Ok(s)
}

pub(crate) fn toString(mut set: Arc<NBTearing>, mut r#str: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    r#str = (StringUtil::headline_4((r#str).clone())?).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("### Iteration Variables:\n")); __mm_s.push_str(&*Slice::lstToString(set.iteration_vars.clone(), (std::sync::Arc::new(BVariable::pointerToString) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), 10)?); ArcStr::from(__mm_s) }).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n### Residual Equations:\n")); __mm_s.push_str(&*Slice::lstToString(set.residual_eqns.clone(), (std::sync::Arc::new({ let __pe_b1 = (literal!("")).clone(); move |__pe_a0| BEquation::Equation::pointerToString(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<ArcStr> + 'static>), 10)?); ArcStr::from(__mm_s) }).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n### Inner Equations:\n")); __mm_s.push_str(&*Array::toString(set.innerEquations.clone(), (std::sync::Arc::new({ let __pe_b1 = -1; move |__pe_a0| StrongComponent::toString(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("\t")).clone(), (literal!("\n\t")).clone(), (literal!("")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
    if isSome(set.jac.clone()) {
        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*BJacobian::toString(Util::getOption(set.jac.clone())?, (literal!("NLS")).clone())?); ArcStr::from(__mm_s) }).clone();
    }
    Ok(r#str)
}

pub(crate) fn main(mut bdae: Arc<Jacobian::NBackendDAE>, mut kind: Partition::Kind) -> Result<Arc<Jacobian::NBackendDAE>> {
    let mut bdae: Arc<Jacobian::NBackendDAE> = bdae;
    let funcs: Arc<metamodelica::List<Module::tearingInterface>> = getModule()?;
    if Flags::isSet(Flags::TEARING_DUMP.clone())? {
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*Partition::Partition::kindToString(kind)?); __mm_s.push_str(&*literal!("] Tearing")); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    bdae = (::match_deref::match_deref! { match &((kind, bdae.clone())) {
        (Partition::Kind::ODE, Deref @ Jacobian::MAIN { eqData: Deref @ BEquation::EqData::EQ_DATA_SIM { uniqueIndex: eq_index, .. }, .. }) => {
            assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN; ode = tearingTraverser(var_field!((*bdae).ode, Jacobian::NBackendDAE::MAIN).clone(), funcs, var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), eq_index.clone(), kind)?);
            bdae
        },
        (_, Deref @ Jacobian::MAIN { eqData: Deref @ BEquation::EqData::EQ_DATA_SIM { uniqueIndex: eq_index, .. }, .. }) if (Partition::kindIsInitial(kind)) => {
            assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN; init = tearingTraverser(var_field!((*bdae).init, Jacobian::NBackendDAE::MAIN).clone(), funcs.clone(), var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), eq_index.clone(), kind)?);
            if isSome(var_field!((*bdae).init_0, Jacobian::NBackendDAE::MAIN).clone()) {
                assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN; init_0 = Some(tearingTraverser(Util::getOption(var_field!((*bdae).init_0, Jacobian::NBackendDAE::MAIN).clone())?, funcs, var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), eq_index.clone(), kind)?));
            }
            bdae
        },
        (Partition::Kind::DAE, Deref @ Jacobian::MAIN { dae: Some(partitions), eqData: Deref @ BEquation::EqData::EQ_DATA_SIM { uniqueIndex: eq_index, .. }, .. }) => {
            assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN; dae = Some(tearingTraverser(partitions.clone(), funcs, var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), eq_index.clone(), kind)?));
            main(bdae, Partition::Kind::ODE.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(bdae)
}

pub(crate) fn implicit(mut comp: Arc<StrongComponent::NBStrongComponent>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut index: i32, mut kind: Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, i32)> {
    let mut comp: Arc<StrongComponent::NBStrongComponent> = comp;
    let mut index: i32 = index;
    let mut dummy: Arc<Adjacency::Matrix::Matrix> = Arc::new(Adjacency::Matrix::Matrix::EMPTY { st: Adjacency::MatrixStrictness::FULL.clone() });
    let mut new_comp: Arc<StrongComponent::NBStrongComponent> = Arc::new(<StrongComponent::NBStrongComponent as ::std::default::Default>::default());
    let mut homotopy: Pointer::Pointer<bool> = Pointer::create(false);
    (comp, dummy, index) = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::SINGLE_COMPONENT { .. } => {
            BEquation::Equation::map(Pointer::access(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone()), (std::sync::Arc::new({ let __pe_b1 = homotopy.clone(); move |__pe_a0| Initialization::containsHomotopyCall(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            new_comp = Arc::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP { idx: index, strict: singleImplicit(var_field!((*comp).var, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone(), var_field!((*comp).eqn, StrongComponent::NBStrongComponent::SINGLE_COMPONENT).clone()), casual: None, linear: false, mixed: false, homotopy: Pointer::access(homotopy), status: Solve::Status::IMPLICIT.clone() });
            finalize(new_comp, dummy, funcMap, index, BVariable::VariablePointers::empty(BaseHashTable::bigBucketSize.clone(), false), BEquation::EquationPointers::empty(BaseHashTable::bigBucketSize.clone()), Pointer::create(0), kind)?
        },
        Deref @ StrongComponent::MULTI_COMPONENT { .. } => {
            BEquation::Equation::map(Pointer::access(Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone())), (std::sync::Arc::new({ let __pe_b1 = homotopy.clone(); move |__pe_a0| Initialization::containsHomotopyCall(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            new_comp = Arc::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP { idx: index, strict: singleImplicit(Slice::getT(listHead(var_field!((*comp).vars, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone())?), Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::MULTI_COMPONENT).clone())), casual: None, linear: false, mixed: false, homotopy: Pointer::access(homotopy), status: Solve::Status::IMPLICIT.clone() });
            finalize(new_comp, dummy, funcMap, index, BVariable::VariablePointers::empty(BaseHashTable::bigBucketSize.clone(), false), BEquation::EquationPointers::empty(BaseHashTable::bigBucketSize.clone()), Pointer::create(0), kind)?
        },
        Deref @ StrongComponent::RESIZABLE_COMPONENT { .. } => {
            BEquation::Equation::map(Pointer::access(Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone())), (std::sync::Arc::new({ let __pe_b1 = homotopy.clone(); move |__pe_a0| Initialization::containsHomotopyCall(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
            new_comp = Arc::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP { idx: index, strict: singleImplicit(Slice::getT(var_field!((*comp).var, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone()), Slice::getT(var_field!((*comp).eqn, StrongComponent::NBStrongComponent::RESIZABLE_COMPONENT).clone())), casual: None, linear: false, mixed: false, homotopy: Pointer::access(homotopy), status: Solve::Status::IMPLICIT.clone() });
            finalize(new_comp, dummy, funcMap, index, BVariable::VariablePointers::empty(BaseHashTable::bigBucketSize.clone(), false), BEquation::EquationPointers::empty(BaseHashTable::bigBucketSize.clone()), Pointer::create(0), kind)?
        },
        _ => (comp, dummy, index),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((comp, index))
}

pub(crate) fn singleImplicit(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut eqn: Pointer::Pointer<Arc<Equation::Equation>>) -> Arc<NBTearing> {
    let mut tearingSet: Arc<NBTearing> = Arc::new(NBTearing { iteration_vars: list![Arc::new(Slice::NBSlice { t: var.clone(), indices: metamodelica::nil() })], residual_eqns: list![Arc::new(Slice::NBSlice { t: eqn.clone(), indices: metamodelica::nil() })], innerEquations: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), jac: None });
    tearingSet
}

pub(crate) fn getModule() -> Result<Arc<metamodelica::List<Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, i32, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> + 'static>>>> {
    fn isNotGuruVar(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut init: bool) -> Result<bool> {
        let mut b: bool;
        let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
        b = BVariable::hasTearingSelect(var_ptr, NFBackendExtension::TearingSelect::PREFER.clone(), (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?;
        Ok(b)
    }

    let mut funcs: Arc<metamodelica::List<Module::tearingInterface>>;
    let mut flag: ArcStr = Flags::getConfigString(Flags::TEARING_METHOD.clone())?;
    funcs = (::match_deref::match_deref! { match &(flag.clone()) {
        Deref @ "minimalTearing" => list![(std::sync::Arc::new({ let __pe_b8: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, bool) -> Result<bool> + 'static> = (std::sync::Arc::new(BVariable::isDiscontinuous) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, bool) -> Result<bool> + 'static>); let __pe_b9 = (std::sync::Arc::new(fnptr!(BEquation::Equation::isDiscontinuous, Pointer::Pointer<Arc<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7| initialize(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7, __pe_b8.clone(), __pe_b9.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, i32, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(minimal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, i32, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(finalize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, i32, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> + 'static>)],
        Deref @ "cellier" => list![(std::sync::Arc::new({ let __pe_b8: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, bool) -> Result<bool> + 'static> = (std::sync::Arc::new(BVariable::isDiscontinuous) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, bool) -> Result<bool> + 'static>); let __pe_b9 = (std::sync::Arc::new(fnptr!(BEquation::Equation::isDiscontinuous, Pointer::Pointer<Arc<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7| initialize(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7, __pe_b8.clone(), __pe_b9.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, i32, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(minimal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, i32, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(finalize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, i32, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> + 'static>)],
        Deref @ "omcTearing" => list![(std::sync::Arc::new({ let __pe_b8: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, bool) -> Result<bool> + 'static> = (std::sync::Arc::new(BVariable::isDiscontinuous) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, bool) -> Result<bool> + 'static>); let __pe_b9 = (std::sync::Arc::new(fnptr!(BEquation::Equation::isDiscontinuous, Pointer::Pointer<Arc<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7| initialize(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7, __pe_b8.clone(), __pe_b9.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, i32, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(minimal) as std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, i32, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(finalize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, i32, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> + 'static>)],
        Deref @ "guruTearing" => list![(std::sync::Arc::new({ let __pe_b8: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, bool) -> Result<bool> + 'static> = (std::sync::Arc::new(isNotGuruVar) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, bool) -> Result<bool> + 'static>); let __pe_b9 = (std::sync::Arc::new(fnptr!(noFilterEqn, Pointer::Pointer<Arc<Equation::Equation>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7| initialize(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_a4, __pe_a5, __pe_a6, __pe_a7, __pe_b8.clone(), __pe_b9.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, i32, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(guru) as std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, i32, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> + 'static>), (std::sync::Arc::new(finalize) as std::sync::Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, i32, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> + 'static>)],
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(funcs)
}

pub(crate) fn getVariables(mut tearing: Arc<NBTearing>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
    let mut variables: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    variables = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (metamodelica::cons(({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (tearing.iteration_vars.clone()).into_iter().cloned() {
            let __x = Slice::getT(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
        for mut comp in (tearing.innerEquations.clone()).borrow().iter() {
            let __x = StrongComponent::getVariables(comp.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))).into_iter().cloned() {
            let __x = var.clone();
            __acc = __x.append(&__acc);
        }
        __acc
    });
    Ok(variables)
}

pub(crate) fn getResidualVars(mut tearing: Arc<NBTearing>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
    let mut residuals: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut eqn in (tearing.residual_eqns.clone()).into_iter().cloned() {
            let __x = BEquation::Equation::getResidualVar(Slice::getT(eqn.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(residuals)
}

pub(crate) fn getIterationVars(mut tearing: Arc<NBTearing>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> {
    let mut iterationVars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (tearing.iteration_vars.clone()).into_iter().cloned() {
            let __x = Slice::getT(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    iterationVars
}

pub(crate) fn getResidualEqns(mut tearing: Arc<NBTearing>) -> Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> {
    let mut residuals: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut eqn in (tearing.residual_eqns.clone()).into_iter().cloned() {
            let __x = Slice::getT(eqn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    residuals
}

pub(crate) fn setResidualEqns(mut tearing: Arc<NBTearing>, mut residuals: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>) -> Arc<NBTearing> {
    let mut tearing: Arc<NBTearing> = tearing;
    assign_field!(tearing.residual_eqns = residuals);
    tearing
}

fn tearingTraverser(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut funcs: Arc<metamodelica::List<Arc<dyn ::std::ops::Fn(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, i32, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Pointer::Pointer<i32>, Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> + 'static>>>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut eq_index: Pointer::Pointer<i32>, mut kind: Partition::Kind) -> Result<Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>> {
    let mut new_partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>> = metamodelica::nil();
    let mut strongComponents: metamodelica::Array<Arc<StrongComponent::NBStrongComponent>>;
    let mut tmp: Arc<StrongComponent::NBStrongComponent>;
    let mut idx: i32 = 0;
    let mut full: Arc<Adjacency::Matrix::Matrix>;
    for mut part in &*partitions {
        let mut part = part.clone();
        if isSome(part.strongComponents.clone()) && isSome(part.adjacencyMatrix.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(part.strongComponents.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            strongComponents = __pa0.clone();
            let __pa1 = ::match_deref::match_deref! { match &(part.adjacencyMatrix.clone()) {
                Some(__pa1) => __pa1.clone(),
                _ => bail!("pattern mismatch"),
            } };
            full = __pa1.clone();
            for mut i in 1..=metamodelica::arrayLength(strongComponents.clone()) {
                tmp = ({let __elt = strongComponents.borrow()[(i.clone()-1) as usize].clone(); __elt});
                for mut func in &*funcs.clone() {
                    let mut func = func.clone();
                    (tmp, full, idx) = func(tmp.clone(), full.clone(), funcMap.clone(), idx, part.unknowns.clone(), part.equations.clone(), eq_index.clone(), kind)?;
                }
                if !(referenceEq(&*(tmp.clone()),&*(({let __elt = strongComponents.borrow()[(i.clone()-1) as usize].clone(); __elt})))) {
                    metamodelica::arrayUpdate(strongComponents.clone(), i.clone(), tmp.clone())?;
                }
            }
            assign_field!(
                part.strongComponents = Some(strongComponents.clone()),
                part.adjacencyMatrix = Some(full.clone())
            );
        }
        new_partitions = metamodelica::cons(part.clone(), new_partitions.clone());
    }
    new_partitions = new_partitions.reverse();
    Ok(new_partitions)
}

fn noFilterVar(mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut init: bool) -> bool {
    let mut b: bool;
    let mut var: Arc<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    b = true;
    b
}

fn noFilterEqn(mut eqn_ptr: Pointer::Pointer<Arc<Equation::Equation>>) -> bool {
    let mut b: bool;
    b = true;
    b
}

fn initialize(mut comp: Arc<StrongComponent::NBStrongComponent>, mut full: Arc<Adjacency::Matrix::Matrix>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut index: i32, mut variables: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut eq_index: Pointer::Pointer<i32>, mut kind: Partition::Kind, mut varFunc: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, bool) -> Result<bool> + 'static>, mut eqnFunc: Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> {
    pub type checkVarInit = std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, bool) -> Result<bool> + 'static>;

    let mut comp: Arc<StrongComponent::NBStrongComponent> = comp;
    let mut full: Arc<Adjacency::Matrix::Matrix> = full;
    let mut index: i32 = index;
    let mut strict: Arc<NBTearing> = Arc::new(<NBTearing as ::std::default::Default>::default());
    let mut vars_lst: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut eqns_lst: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut vars_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = <Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> as ::std::default::Default>::default();
    let mut v: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> = <Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> as ::std::default::Default>::default();
    let mut e: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> = <Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> as ::std::default::Default>::default();
    let init: bool = Partition::kindIsInitial(kind);
    (comp, full, index) = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::ALGEBRAIC_LOOP { strict: __esc_strict, .. } => {
            strict = (*__esc_strict).clone();
            index = index + 1;
            assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; idx = index);
            vars_lst = ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut var in (strict.iteration_vars.clone()).into_iter().cloned() {
            if !(varFunc(Slice::getT(var.clone()), init)?) { continue; }
            let __x = BVariable::getVarName(Slice::getT(var.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            eqns_lst = ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
            if !(eqnFunc(Slice::getT(eqn.clone()))?) { continue; }
            let __x = BEquation::Equation::getEqnName(Slice::getT(eqn.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            vars_set = UnorderedSet::fromList(vars_lst.clone(), (std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
            v = UnorderedMap::subMap(variables.map.clone(), vars_lst)?;
            e = UnorderedMap::subMap(equations.map.clone(), eqns_lst)?;
            full = Adjacency::Matrix::refine(full, funcMap, v.clone(), e.clone(), variables, equations, vars_set, Partition::kindIsInitial(kind))?;
            assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; linear = checkLinearity(full.clone(), v, e)?);
            (comp, full, index)
        },
        _ => (comp, full, index),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((comp, full, index))
}

fn finalize(mut comp: Arc<StrongComponent::NBStrongComponent>, mut full: Arc<Adjacency::Matrix::Matrix>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut index: i32, mut variables: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut eq_index: Pointer::Pointer<i32>, mut kind: Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> {
    let mut comp: Arc<StrongComponent::NBStrongComponent> = comp;
    let mut full: Arc<Adjacency::Matrix::Matrix> = full;
    let mut index: i32 = index;
    let mut strict: Arc<NBTearing> = Arc::new(<NBTearing as ::std::default::Default>::default());
    let mut acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>>> = metamodelica::nil();
    let mut dummy_set: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>> = UnorderedSet::new((std::sync::Arc::new(BVariable::hash) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(BVariable::equalName) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), 13);
    comp = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::ALGEBRAIC_LOOP { strict: __esc_strict, .. } => {
            strict = (*__esc_strict).clone();
            acc = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>>>> = metamodelica::nil();
        for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
            let __x = Inline::inlineRecordSliceEquation(eqn.clone(), variables.clone(), dummy_set.clone(), eq_index.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            assign_field!(strict.residual_eqns = ({
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
        for mut eqn in (List::flatten(acc)?).into_iter().cloned() {
            let __x = Slice::apply(eqn.clone(), (std::sync::Arc::new({ let __pe_b1 = None; let __pe_b2 = true; let __pe_b3 = false; move |__pe_a0| BEquation::Equation::createResidual(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<Pointer::Pointer<Arc<Equation::Equation>>> + 'static>))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; strict = strict.clone());
            if Flags::isSet(Flags::TEARING_DUMP.clone())? {
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*Partition::Partition::kindToString(kind)?); __mm_s.push_str(&*literal!("] Tearing Result ")); __mm_s.push_str(&*intString(var_field!((*comp).idx, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone())); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            comp
        },
        _ => comp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((comp, full, index))
}

fn minimal(mut comp: Arc<StrongComponent::NBStrongComponent>, mut full: Arc<Adjacency::Matrix::Matrix>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut index: i32, mut variables: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut eq_index: Pointer::Pointer<i32>, mut kind: Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> {
    let mut comp: Arc<StrongComponent::NBStrongComponent> = comp;
    let mut full: Arc<Adjacency::Matrix::Matrix> = full;
    let mut index: i32 = index;
    let mut strict: Arc<NBTearing> = Arc::new(<NBTearing as ::std::default::Default>::default());
    let mut vars_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut cont_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut disc_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut implied_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
    let mut eqns_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut cont_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut disc_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut num_vars: i32 = 0;
    let mut num_eqns: i32 = 0;
    let mut matched_vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
    let mut iteration_vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
    let mut adj: Arc<Adjacency::Matrix::Matrix> = Arc::new(<Adjacency::Matrix::Matrix as ::std::default::Default>::default());
    let mut matching: Arc<Matching::NBMatching> = Arc::new(<Matching::NBMatching as ::std::default::Default>::default());
    let mut inner_comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut v: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> = <Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> as ::std::default::Default>::default();
    let mut e: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> = <Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>> as ::std::default::Default>::default();
    let mut matched_set: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    comp = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ StrongComponent::ALGEBRAIC_LOOP { strict: __esc_strict, .. } => {
            strict = (*__esc_strict).clone();
            vars_lst = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (strict.iteration_vars.clone()).into_iter().cloned() {
            let __x = Slice::getT(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            eqns_lst = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
            let __x = Slice::getT(eqn.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (cont_vars, disc_vars) = filterDiscreteVariables(vars_lst, Partition::kindIsInitial(kind))?;
            (cont_eqns, disc_eqns) = List::splitOnTrue(eqns_lst, (std::sync::Arc::new(BEquation::Equation::isContinousRecordAware) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<bool> + 'static>))?;
            implied_vars = List::flatten(({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
        for mut eqn in (disc_eqns.clone()).into_iter().cloned() {
            let __x = getImpliedInnerVars(eqn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
            disc_vars = UnorderedSet::unique_list(listAppend(disc_vars, implied_vars.clone()), (std::sync::Arc::new(BVariable::hash) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(BVariable::equalName) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?;
            cont_vars = UnorderedSet::difference_list(cont_vars, implied_vars, (std::sync::Arc::new(BVariable::hash) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(BVariable::equalName) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?;
            num_vars = ({
        let mut __acc: i32 = 0;
        for mut var in (disc_vars.clone()).into_iter().cloned() {
            let __x = BVariable::size(var.clone(), false)?;
            __acc += __x;
        }
        __acc
    });
            num_eqns = ({
        let mut __acc: i32 = 0;
        for mut eqn in (disc_eqns.clone()).into_iter().cloned() {
            let __x = BEquation::Equation::size(eqn.clone(), false)?;
            __acc += __x;
        }
        __acc
    });
            if !(disc_eqns.clone().is_empty()) {
                assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; mixed = true);
                v = UnorderedMap::subMap(variables.map.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut var in (disc_vars).into_iter().cloned() {
            let __x = BVariable::getVarName(var.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
                e = UnorderedMap::subMap(equations.map.clone(), ({
        let mut __acc: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut eqn in (disc_eqns).into_iter().cloned() {
            let __x = BEquation::Equation::getEqnName(eqn.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
                adj = Adjacency::Matrix::fullToFinal(full.clone(), v.clone(), e.clone(), equations.clone(), Adjacency::MatrixStrictness::MATCHING.clone(), crate::NBEquation::Iterator::interned_EMPTY())?;
                matching = Matching::regular(Matching::EMPTY_MATCHING().clone(), adj.clone(), true, true, true)?;
                (matched_vars, _, _, _) = Matching::getMatches(matching.clone(), Adjacency::Matrix::getMappingOpt(adj.clone()), variables.clone(), equations.clone())?;
                for mut var in &*matched_vars {
                    let mut var = var.clone();
                    UnorderedSet::add(BVariable::getVarName(Slice::getT(var.clone())), matched_set.clone())?;
                }
                for mut var in &*strict.iteration_vars.clone() {
                    let mut var = var.clone();
                    if !(UnorderedSet::contains(BVariable::getVarName(Slice::getT(var.clone())), matched_set.clone())?) {
                        iteration_vars = metamodelica::cons(var.clone(), iteration_vars.clone());
                    }
                }
                adj = Adjacency::Matrix::upgrade(adj, full.clone(), v, e, equations.clone(), Adjacency::MatrixStrictness::SORTING.clone(), crate::NBEquation::Iterator::interned_EMPTY())?;
                inner_comps = Sorting::tarjan(adj, matching, variables, equations)?;
                assign_field!(
                    strict.innerEquations = metamodelica::arrayFromVec(inner_comps.into_iter().cloned().collect()),
                    strict.residual_eqns = ({
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
        for mut eqn in (cont_eqns).into_iter().cloned() {
            let __x = Arc::new(Slice::NBSlice { t: eqn.clone(), indices: metamodelica::nil() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                    strict.iteration_vars = iteration_vars.reverse()
                );
                assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; strict = strict.clone());
            }
            comp
        },
        _ => comp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((comp, full, index))
}

fn guru(mut comp: Arc<StrongComponent::NBStrongComponent>, mut full: Arc<Adjacency::Matrix::Matrix>, mut funcMap: Arc<UnorderedMap::UnorderedMap<Arc<Path>, Arc<Function::Function>>>, mut index: i32, mut variables: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut eq_index: Pointer::Pointer<i32>, mut kind: Partition::Kind) -> Result<(Arc<StrongComponent::NBStrongComponent>, Arc<Adjacency::Matrix::Matrix>, i32)> {
    let mut comp: Arc<StrongComponent::NBStrongComponent> = comp;
    let mut full: Arc<Adjacency::Matrix::Matrix> = full;
    let mut index: i32 = index;
    let mut inner_comps: Arc<metamodelica::List<Arc<StrongComponent::NBStrongComponent>>> = metamodelica::nil();
    let mut residuals: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = metamodelica::nil();
    let mut strict: Arc<NBTearing> = Arc::new(<NBTearing as ::std::default::Default>::default());
    let mut nEqn: i32 = 0;
    let mut inner_vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
    let mut guru_vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
    let mut failed_vars: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
    let mut unsolved_inner_vars: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = <Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> as ::std::default::Default>::default();
    let mut unsolved_equations: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> = <Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>>> as ::std::default::Default>::default();
    let mut solve_opt: Option<Arc<ComponentRef::NFComponentRef>> = None;
    let mut solve_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut solve_var: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut solve_eqn: Arc<Slice::NBSlice<Pointer::Pointer<Arc<Equation::Equation>>>>;
    let mut success: bool = false;
    let mut var_assigned: bool = false;
    let mut stripped: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let staticAsContinuous: bool = Partition::kindIsInitial(kind);
    comp = (::match_deref::match_deref! { match &((comp.clone(), full.clone())) {
        (Deref @ StrongComponent::ALGEBRAIC_LOOP { strict: __esc_strict, .. }, Deref @ Adjacency::Matrix::FULL { .. }) => {
            strict = (*__esc_strict).clone();
            nEqn = metamodelica::arrayLength(var_field!((*full).equation_names, Adjacency::Matrix::Matrix::FULL).clone());
            (inner_vars, guru_vars) = List::splitOnTrue(strict.iteration_vars.clone(), (std::sync::Arc::new({ let __pe_b1 = (std::sync::Arc::new({ let __pe_b1 = NFBackendExtension::TearingSelect::PREFER.clone(); let __pe_b2: Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static> = (std::sync::Arc::new(fnptr!(intLt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>); move |__pe_a0| BVariable::hasTearingSelect(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>); move |__pe_a0| Slice::check(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
            if guru_vars.clone().is_empty() {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBTearing.guru")); __mm_s.push_str(&*literal!(" failed. No guru variables provided for strong component:\n")); __mm_s.push_str(&*StrongComponent::toString(comp.clone(), -1)?); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            } else {
                failed_vars = ({
        let mut __acc: Arc<metamodelica::List<Arc<Slice::NBSlice<Pointer::Pointer<Arc<Variable::NFVariable>>>>>> = metamodelica::nil();
        for mut var in (guru_vars.clone()).into_iter().cloned() {
            if !(Slice::check(var.clone(), (std::sync::Arc::new({ let __pe_b1 = staticAsContinuous; move |__pe_a0| BVariable::isDiscontinuous(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                if !(failed_vars.clone().is_empty()) {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBTearing.guru")); __mm_s.push_str(&*literal!(" failed. Following variables cannot be chosen as iteration variables because they are discontinuous:\n")); __mm_s.push_str(&*List::toString(failed_vars, (std::sync::Arc::new({ let __pe_b1 = (std::sync::Arc::new(BVariable::pointerToString) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>); let __pe_b2 = 10; move |__pe_a0| Slice::toString(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("\t")).clone(), (literal!("\n\t")).clone(), (literal!("")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone()])?;
                    bail!("fail");
                }
                unsolved_inner_vars = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
                for mut var in &*inner_vars.clone() {
                    let mut var = var.clone();
                    UnorderedMap::add(BVariable::getVarName(Slice::getT(var.clone())), var.clone(), unsolved_inner_vars.clone())?;
                }
                unsolved_equations = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
                for mut eqn in &*strict.residual_eqns.clone() {
                    let mut eqn = eqn.clone();
                    UnorderedMap::add(BEquation::Equation::getEqnName(Slice::getT(eqn.clone()))?, eqn.clone(), unsolved_equations.clone())?;
                }
                while !(UnorderedMap::isEmpty(unsolved_inner_vars.clone())) {
                    for mut i in 1..=nEqn {
                        var_assigned = false;
                        if UnorderedMap::contains(({let __elt = var_field!((*full).equation_names, Adjacency::Matrix::Matrix::FULL).borrow()[(i.clone()-1) as usize].clone(); __elt}), unsolved_equations.clone())? {
                            solve_opt = None;
                            success = false;
                            let __range0 = &*UnorderedSet::toList(({let __elt = var_field!((*full).occurrences, Adjacency::Matrix::Matrix::FULL).borrow()[(i.clone()-1) as usize].clone(); __elt}));
                            for mut cref in __range0 {
                                let mut cref = cref.clone();
                                stripped = ComponentRef::stripSubscriptsAll(cref.clone());
                                if UnorderedMap::contains(stripped.clone(), unsolved_inner_vars.clone())? {
                                    if isNone(solve_opt.clone()) {
                                        success = true;
                                        solve_opt = Some(cref.clone());
                                    } else {
                                        success = false;
                                        break;
                                    }
                                }
                            }
                            let () = (::match_deref::match_deref! { match &((solve_opt.clone(), success)) {
        (Some(__esc_solve_cref), true) => {
            solve_cref = (*__esc_solve_cref).clone();
            stripped = ComponentRef::stripSubscriptsAll(solve_cref.clone());
            solve_var = UnorderedMap::getSafe(stripped.clone(), unsolved_inner_vars.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"))?;
            solve_eqn = UnorderedMap::getSafe(({let __elt = var_field!((*full).equation_names, Adjacency::Matrix::Matrix::FULL).borrow()[(i.clone()-1) as usize].clone(); __elt}), unsolved_equations.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"))?;
            inner_comps = metamodelica::cons(StrongComponent::createSliceOrSingle(solve_cref.clone(), solve_var.clone(), solve_eqn.clone())?, inner_comps.clone());
            UnorderedMap::remove(stripped.clone(), unsolved_inner_vars.clone())?;
            UnorderedMap::remove(({let __elt = var_field!((*full).equation_names, Adjacency::Matrix::Matrix::FULL).borrow()[(i.clone()-1) as usize].clone(); __elt}), unsolved_equations.clone())?;
            var_assigned = true;
            ()
        },
        (Some(__esc_solve_cref), false) => {
            solve_cref = (*__esc_solve_cref).clone();
            ()
        },
        (None, false) => {
            residuals = metamodelica::cons(UnorderedMap::getSafe(({let __elt = var_field!((*full).equation_names, Adjacency::Matrix::Matrix::FULL).borrow()[(i.clone()-1) as usize].clone(); __elt}), unsolved_equations.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"))?, residuals.clone());
            UnorderedMap::remove(({let __elt = var_field!((*full).equation_names, Adjacency::Matrix::Matrix::FULL).borrow()[(i.clone()-1) as usize].clone(); __elt}), unsolved_equations.clone())?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBTearing.guru")); __mm_s.push_str(&*literal!(" failed. Impossible result for equation representative: ")); __mm_s.push_str(&*ComponentRef::toString(({let __elt = var_field!((*full).equation_names, Adjacency::Matrix::Matrix::FULL).borrow()[(i.clone()-1) as usize].clone(); __elt}))?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                        }
                        if var_assigned {
                            break;
                        }
                    }
                    if !(var_assigned) {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBTearing.guru")); __mm_s.push_str(&*literal!(" failed. Following variables could not be solved as inner variables:\n")); __mm_s.push_str(&*List::toString(UnorderedMap::valueList(unsolved_inner_vars.clone()), (std::sync::Arc::new({ let __pe_b1 = (std::sync::Arc::new(BVariable::pointerToString) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>); let __pe_b2 = 10; move |__pe_a0| Slice::toString(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("\t")).clone(), (literal!("\n\t")).clone(), (literal!("")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone()])?;
                        bail!("fail");
                    }
                }
                assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; mixed = List::any(inner_vars, (std::sync::Arc::new({ let __pe_b1 = (std::sync::Arc::new({ let __pe_b1 = staticAsContinuous; move |__pe_a0| BVariable::isDiscontinuous(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>); move |__pe_a0| Slice::check(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?);
                assign_field!(
                    strict.innerEquations = metamodelica::arrayFromVec(inner_comps.reverse().into_iter().cloned().collect()),
                    strict.residual_eqns = listAppend(UnorderedMap::valueList(unsolved_equations), residuals),
                    strict.iteration_vars = guru_vars
                );
                assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; strict = strict.clone());
            }
            comp
        },
        _ => comp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((comp, full, index))
}

fn checkLinearity(mut full: Arc<Adjacency::Matrix::Matrix>, mut v: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut e: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>) -> Result<bool> {
    fn varIsLinear(mut var: Arc<ComponentRef::NFComponentRef>, mut v: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>, mut sol: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>) -> Result<bool> {
        let mut b: bool = !(UnorderedMap::contains(var.clone(), v.clone())? && Adjacency::Solvability::isNonlinearOrImplicit(UnorderedMap::getSafe(var.clone(), sol.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"))?));
        Ok(b)
    }

    fn eqnIsLinear(mut i: i32, mut occ: metamodelica::Array<Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>>, mut sol: metamodelica::Array<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Solvability::Solvability>>>>, mut v: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>) -> Result<bool> {
        let mut b: bool = UnorderedSet::all(({let __elt = occ.borrow()[(i-1) as usize].clone(); __elt}), (std::sync::Arc::new({ let __pe_b1 = v.clone(); let __pe_b2 = ({let __elt = sol.borrow()[(i-1) as usize].clone(); __elt}); move |__pe_a0| varIsLinear(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
        Ok(b)
    }

    let mut linear: bool;
    linear = (::match_deref::match_deref! { match &(full.clone()) {
        Deref @ Adjacency::Matrix::FULL { .. } => UnorderedMap::all(e.clone(), (std::sync::Arc::new({ let __pe_b1 = var_field!((*full).occurrences, Adjacency::Matrix::Matrix::FULL).clone(); let __pe_b2 = var_field!((*full).solvabilities, Adjacency::Matrix::Matrix::FULL).clone(); let __pe_b3 = v; move |__pe_a0| eqnIsLinear(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<bool> + 'static>))?,
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBTearing.checkLinearity")); __mm_s.push_str(&*literal!(" expected type full, got type ")); __mm_s.push_str(&*Adjacency::strictnessString(Adjacency::Matrix::getStrictness(full.clone())?)); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(linear)
}

fn filterDiscreteVariables(mut vars_lst: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut staticAsContinuous: bool) -> Result<(Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>)> {
    fn addDiscreteRecord(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut discrete_records: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
        let () = (match BVariable::getParent(var) {
        Some(mut parent) => {
            UnorderedSet::add(BVariable::getVarName(parent.clone()), discrete_records.clone())?;
            addDiscreteRecord(parent.clone(), discrete_records)?;
            ()
        },
        _ => {
            ()
        },
    });
        Ok(())
    }

    fn checkDiscreteRecord(mut var: Pointer::Pointer<Arc<Variable::NFVariable>>, mut discrete_records: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>>, mut is_parent: bool) -> Result<bool> {
        '__tco: loop {
            match BVariable::getParent(var.clone()) {
        Some(mut parent) => {
            { (var, discrete_records, is_parent) = (parent.clone(), discrete_records, true); continue '__tco; }
        },
        _ => {
            return Ok(is_parent && UnorderedSet::contains(BVariable::getVarName(var), discrete_records)?)
        },
    }
        }
    }

    let mut cont_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut disc_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    let mut discrete_records: Arc<UnorderedSet::UnorderedSet<Arc<ComponentRef::NFComponentRef>>> = UnorderedSet::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
    let mut rec_disc_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    (cont_vars, disc_vars) = List::splitOnTrue(vars_lst.clone(), (std::sync::Arc::new({ let __pe_b1 = staticAsContinuous.clone(); move |__pe_a0| BVariable::isContinuous(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?;
    for mut var in &*disc_vars.clone() {
        let mut var = var.clone();
        addDiscreteRecord(var.clone(), discrete_records.clone())?;
    }
    (rec_disc_vars, cont_vars) = List::splitOnTrue(cont_vars.clone(), (std::sync::Arc::new({ let __pe_b1 = discrete_records; let __pe_b2 = false; move |__pe_a0| checkDiscreteRecord(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>))?;
    disc_vars = listAppend(rec_disc_vars.clone(), disc_vars.clone()).reverse();
    Ok((cont_vars, disc_vars))
}

fn getImpliedInnerVars(mut eqn: Pointer::Pointer<Arc<Equation::Equation>>) -> Result<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> {
    let mut vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>;
    vars = (::match_deref::match_deref! { match &(Pointer::access(eqn)) {
        Deref @ BEquation::Equation::ALGORITHM { alg, .. } => {
            ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut out_cr in (alg.outputs.clone()).into_iter().cloned() {
            let __x = BVariable::getVarPointer(out_cr.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { lhs: tpl @ Deref @ Expression::TUPLE { .. }, .. } => {
            ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut tpl_cr in (UnorderedSet::toList(Expression::extractCrefs(tpl.clone())?)).into_iter().cloned() {
            let __x = BVariable::getVarPointer(tpl_cr.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBTearing.mo"))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { lhs: Deref @ Expression::CREF { .. }, .. } => {
            metamodelica::nil()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(vars)
}


