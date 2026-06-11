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
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBJacobian as Jacobian;
use crate::NBPartition as Partition;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use crate::NSimCode as SimCode;
use crate::NSimCode::Identifier;
use crate::NSimCodeUtil as SimCodeUtil;
use crate::NSimGenericCall as SimGenericCall;
use crate::NSimStrongComponent as SimStrongComponent;
use crate::NSimVar::SimVar;
use crate::NSimVar::SimVars;
use crate::NSimVar::VarType;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable;
use openmodelica_simcode_types::SimCode as OldSimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// NF imports
// Backend imports
// SimCode imports
// Old SimCode imports
// Util imports
pub type SparsityPattern = Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>;

pub type SparsityColoring = Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;

pub mod SimJacobian {
    use super::*;
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct SimJacobian {
        /// unique matrix name
        pub name: ArcStr,
        /// unique jacobian index
        pub jacobianIndex: i32,
        /// index of partition it belongs to
        pub partitionIndex: i32,
        /// corresponds to the number of rows
        pub numberOfResultVars: i32,
        /// column equations equals in size to column vars
        pub columnEqns: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// List of constant equations independent of seed variables
        pub constantEqns: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>,
        /// all column vars, none results vars index -1, the other corresponding to rows index
        pub columnVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        /// corresponds to the number of columns
        pub seedVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>,
        /// sparsity pattern in index form
        pub sparsity: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>,
        /// transposed sparsity pattern
        pub sparsityT: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>,
        /// coloring columns in index form (column coloring)
        pub coloring: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>,
        /// coloring rows in index form (row coloring)
        pub rowColoring: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>,
        /// number of colors
        pub numColors: i32,
        /// Generic for-loop and array calls
        pub generic_loop_calls: Arc<metamodelica::List<Arc<SimGenericCall::NSimGenericCall>>>,
        /// hash table for cref -> simVar
        pub jac_map: Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>>,
        /// indicates if this is an adjoint jacobian
        pub isAdjoint: bool,
    }

    impl metamodelica::gc::MMTrace for SimJacobian {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.jacobianIndex, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.partitionIndex, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numberOfResultVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.columnEqns, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.constantEqns, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.columnVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.seedVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.sparsity, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.sparsityT, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.coloring, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.rowColoring, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numColors, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.generic_loop_calls, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.jac_map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.isAdjoint, __mmv)?;
            Ok(())
        }
    }
    impl Default for SimJacobian {
        fn default() -> Self {
            Self {
                name: Default::default(),
                jacobianIndex: Default::default(),
                partitionIndex: Default::default(),
                numberOfResultVars: Default::default(),
                columnEqns: Default::default(),
                constantEqns: Default::default(),
                columnVars: Default::default(),
                seedVars: Default::default(),
                sparsity: Default::default(),
                sparsityT: Default::default(),
                coloring: Default::default(),
                rowColoring: Default::default(),
                numColors: Default::default(),
                generic_loop_calls: Default::default(),
                jac_map: Default::default(),
                isAdjoint: Default::default(),
            }
        }
    }

    pub type SIM_JAC = SimJacobian;

    pub(crate) fn toString(mut simJac: Arc<SimJacobian>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        let mut idx: i32 = 0;
        let mut dependencies: Arc<metamodelica::List<i32>> = metamodelica::nil();
        r#str = ((::match_deref::match_deref! { match &(simJac.clone()) {
        Deref @ SimJacobian { .. } => {
            if isEmpty(simJac.clone()) {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[EMPTY] SimCode Jacobian ")); __mm_s.push_str(&*simJac.name.clone()); __mm_s.push_str(&*literal!("(idx = ")); __mm_s.push_str(&*intString(simJac.jacobianIndex.clone())); __mm_s.push_str(&*literal!(", partition = ")); __mm_s.push_str(&*intString(simJac.partitionIndex.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            } else {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_2(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SimCode Jacobian ")); __mm_s.push_str(&*simJac.name.clone()); __mm_s.push_str(&*literal!("(idx = ")); __mm_s.push_str(&*intString(simJac.jacobianIndex.clone())); __mm_s.push_str(&*literal!(", partition = ")); __mm_s.push_str(&*intString(simJac.jacobianIndex.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SeedVars (size = ")); __mm_s.push_str(&*intString((simJac.seedVars.clone().len() as i32))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())); ArcStr::from(__mm_s) }).clone();
                for mut var in &*simJac.seedVars.clone() {
                    let mut var = var.clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimVar::toString(var.clone(), (literal!("  ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                }
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TmpVars (size = ")); __mm_s.push_str(&*intString((simJac.columnVars.clone().len() as i32))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())); ArcStr::from(__mm_s) }).clone();
                for mut var in &*simJac.columnVars.clone() {
                    let mut var = var.clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimVar::toString(var.clone(), (literal!("  ")).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                }
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ResultVars (size = ")); __mm_s.push_str(&*intString(simJac.numberOfResultVars.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())); ArcStr::from(__mm_s) }).clone();
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_3(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Column Equations (size = ")); __mm_s.push_str(&*intString((simJac.columnEqns.clone().len() as i32))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())); ArcStr::from(__mm_s) }).clone();
                for mut eq in &*simJac.columnEqns.clone() {
                    let mut eq = eq.clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimStrongComponent::Block::toString(eq.clone(), (literal!("  ")).clone())?); ArcStr::from(__mm_s) }).clone();
                }
                if !(simJac.constantEqns.clone().is_empty()) {
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*StringUtil::headline_3((literal!("Constant Equations")).clone())); ArcStr::from(__mm_s) }).clone();
                    for mut eq in &*simJac.constantEqns.clone() {
                        let mut eq = eq.clone();
                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*SimStrongComponent::Block::toString(eq.clone(), (literal!("  ")).clone())?); ArcStr::from(__mm_s) }).clone();
                    }
                }
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4((literal!("Sparsity Pattern Cols")).clone())); ArcStr::from(__mm_s) }).clone();
                if !(simJac.sparsityT.clone().is_empty()) {
                    for mut tpl in &*simJac.sparsityT.clone() {
                        let mut tpl = tpl.clone();
                        (idx, dependencies) = tpl.clone();
                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*intString(idx)); __mm_s.push_str(&*literal!(":\t")); __mm_s.push_str(&*List::toString(dependencies.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    }
                }
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4((literal!("Sparsity Pattern Rows")).clone())); ArcStr::from(__mm_s) }).clone();
                if !(simJac.sparsity.clone().is_empty()) {
                    for mut tpl in &*simJac.sparsity.clone() {
                        let mut tpl = tpl.clone();
                        (idx, dependencies) = tpl.clone();
                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*intString(idx)); __mm_s.push_str(&*literal!(":\t")); __mm_s.push_str(&*List::toString(dependencies.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    }
                }
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4((literal!("Sparsity Coloring Columns")).clone())); ArcStr::from(__mm_s) }).clone();
                if !(simJac.coloring.clone().is_empty()) {
                    for mut lst in &*simJac.coloring.clone() {
                        let mut lst = lst.clone();
                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*List::toString(lst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    }
                }
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*StringUtil::headline_4((literal!("Sparsity Coloring Rows")).clone())); ArcStr::from(__mm_s) }).clone();
                if !(simJac.rowColoring.clone().is_empty()) {
                    for mut lst in &*simJac.rowColoring.clone() {
                        let mut lst = lst.clone();
                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("  ")); __mm_s.push_str(&*List::toString(lst.clone(), (std::sync::Arc::new(fnptr!(intString, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("{")).clone(), (literal!(", ")).clone(), (literal!("}")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    }
                }
                if !(simJac.generic_loop_calls.clone().is_empty()) {
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*StringUtil::headline_3((literal!("Generic Calls")).clone())); ArcStr::from(__mm_s) }).clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*List::toString(simJac.generic_loop_calls.clone(), (std::sync::Arc::new(SimGenericCall::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimGenericCall::NSimGenericCall>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("  ")).clone(), (literal!("\n  ")).clone(), (literal!("\n")).clone(), true, 0)?); ArcStr::from(__mm_s) }).clone();
                }
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
            }
            r#str
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.toString")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        Ok(r#str)
    }

    pub(crate) fn isEmpty(mut simJac: Arc<SimJacobian>) -> bool {
        let mut b: bool;
        b = (::match_deref::match_deref! { match &(simJac.clone()) {
        Deref @ SimJacobian { .. } => simJac.numberOfResultVars.clone() == 0,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        b
    }

    pub(crate) fn create(mut jacobian: Arc<BackendDAE::NBackendDAE>, mut indices: SimCode::SimCodeIndices, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>) -> Result<(Option<Arc<SimJacobian>>, SimCode::SimCodeIndices)> {
        let mut simJacobian: Option<Arc<SimJacobian>> = None;
        let mut indices: SimCode::SimCodeIndices = indices;
        simJacobian = ({
        let mut dummy_sim_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>> = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        let mut dummy_eqn_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimStrongComponent::Block::Block>>> = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
        let mut columnEqns: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
        let mut seedVars_ptr: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
        let mut resVars_ptr: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
        let mut tmpVars_ptr: Pointer::Pointer<Arc<metamodelica::List<Arc<SimVar::SimVar>>>> = Pointer::create(metamodelica::nil());
        (::match_deref::match_deref! { match &(jacobian.clone()) {
        Deref @ BackendDAE::JACOBIAN { varData: varData @ Deref @ BVariable::VarData::VAR_DATA_JAC { .. }, .. } => {
            let mut columnEqn: Arc<SimStrongComponent::Block::Block>;
            let mut seed_vec: Arc<VariablePointers::VariablePointers>;
            let mut res_vec: Arc<VariablePointers::VariablePointers>;
            let mut tmp_vec: Arc<VariablePointers::VariablePointers>;
            let mut seedVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>;
            let mut resVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>;
            let mut tmpVars: Arc<metamodelica::List<Arc<SimVar::SimVar>>>;
            let mut jac_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>;
            let mut local_idx_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>;
            let mut cref: Arc<ComponentRef::NFComponentRef>;
            let mut sparsity: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>;
            let mut sparsityT: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>;
            let mut coloring: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut rowColoring: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
            let mut jac: Arc<SimJacobian>;
            let mut sim_map: Arc<UnorderedMap::UnorderedMap<Arc<Identifier::Identifier>, i32>>;
            let mut generic_loop_calls: Arc<metamodelica::List<Arc<SimGenericCall::NSimGenericCall>>>;
            sim_map = indices.generic_call_map.clone();
            indices.generic_call_map = UnorderedMap::new((std::sync::Arc::new(SimCode::Identifier::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Identifier::Identifier>) -> Result<i32> + 'static>), (std::sync::Arc::new(SimCode::Identifier::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Identifier::Identifier>, Arc<Identifier::Identifier>) -> Result<bool> + 'static>), 1);
            for mut i in ({let __s=metamodelica::arrayLength(var_field!((*jacobian).comps, BackendDAE::NBackendDAE::JACOBIAN).clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
                (columnEqn, indices, _) = SimStrongComponent::Block::fromStrongComponent(({let __elt = var_field!((*jacobian).comps, BackendDAE::NBackendDAE::JACOBIAN).borrow()[(i.clone()-1) as usize].clone(); __elt}), indices.clone(), Partition::Kind::JAC.clone(), dummy_sim_map.clone(), dummy_eqn_map.clone())?;
                columnEqns = metamodelica::cons(columnEqn.clone(), columnEqns.clone());
            }
            generic_loop_calls = ({
        let mut __acc: Arc<metamodelica::List<Arc<SimGenericCall::NSimGenericCall>>> = metamodelica::nil();
        for mut tpl in (UnorderedMap::toList(indices.generic_call_map.clone())).into_iter().cloned() {
            let __x = SimGenericCall::fromIdentifier(tpl.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            indices.generic_call_map = sim_map.clone();
            if Flags::getConfigBool(Flags::SIM_CODE_SCALARIZE.clone())? {
                seed_vec = BVariable::VariablePointers::scalarize(var_field!((**varData).seedVars, VarData::VarData::VAR_DATA_JAC).clone())?;
                res_vec = BVariable::VariablePointers::scalarize(var_field!((**varData).resultVars, VarData::VarData::VAR_DATA_JAC).clone())?;
                tmp_vec = BVariable::VariablePointers::scalarize(var_field!((**varData).tmpVars, VarData::VarData::VAR_DATA_JAC).clone())?;
            } else {
                seed_vec = var_field!((**varData).seedVars, VarData::VarData::VAR_DATA_JAC).clone();
                res_vec = var_field!((**varData).resultVars, VarData::VarData::VAR_DATA_JAC).clone();
                tmp_vec = var_field!((**varData).tmpVars, VarData::VarData::VAR_DATA_JAC).clone();
            }
            BVariable::VariablePointers::map(seed_vec.clone(), (std::sync::Arc::new({ let __pe_b1 = seedVars_ptr.clone(); let __pe_b2 = Pointer::create(SimCode::EMPTY_SIM_CODE_INDICES()); let __pe_b3 = VarType::SIMULATION.clone(); move |__pe_a0| SimVar::traverseCreate(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFVariable::NFVariable>) -> Result<Arc<NFVariable::NFVariable>> + 'static>))?;
            BVariable::VariablePointers::map(res_vec.clone(), (std::sync::Arc::new({ let __pe_b1 = resVars_ptr.clone(); let __pe_b2 = Pointer::create(SimCode::EMPTY_SIM_CODE_INDICES()); let __pe_b3 = VarType::SIMULATION.clone(); move |__pe_a0| SimVar::traverseCreate(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFVariable::NFVariable>) -> Result<Arc<NFVariable::NFVariable>> + 'static>))?;
            BVariable::VariablePointers::map(tmp_vec.clone(), (std::sync::Arc::new({ let __pe_b1 = tmpVars_ptr.clone(); let __pe_b2 = Pointer::create(SimCode::EMPTY_SIM_CODE_INDICES()); let __pe_b3 = VarType::SIMULATION.clone(); move |__pe_a0| SimVar::traverseCreate(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<NFVariable::NFVariable>) -> Result<Arc<NFVariable::NFVariable>> + 'static>))?;
            seedVars = Pointer::access(seedVars_ptr.clone()).reverse();
            resVars = Pointer::access(resVars_ptr.clone()).reverse();
            tmpVars = Pointer::access(tmpVars_ptr.clone()).reverse();
            jac_map = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), (seedVars.clone().len() as i32) + (resVars.clone().len() as i32) + (tmpVars.clone().len() as i32));
            SimCodeUtil::addListSimCodeMap(seedVars.clone(), jac_map.clone())?;
            SimCodeUtil::addListSimCodeMap(resVars.clone(), jac_map.clone())?;
            SimCodeUtil::addListSimCodeMap(tmpVars.clone(), jac_map.clone())?;
            match '__try0: {
                local_idx_map = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), (seedVars.clone().len() as i32) + (resVars.clone().len() as i32));
                for mut var in &*seedVars.clone() {
                    let mut var = var.clone();
                    cref = SimVar::getName(var.clone());
                    if unwrap_break_err!(BVariable::checkCref(cref.clone(), (std::sync::Arc::new(fnptr!(BVariable::isSeed, Pointer::Pointer<Arc<NFVariable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<NFVariable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!("NSimCode/NSimJacobian.mo")), '__try0) {
                        cref = unwrap_break_err!(BVariable::getPartnerCref(cref.clone(), (std::sync::Arc::new(fnptr!(BVariable::getVarSeed, Pointer::Pointer<Arc<NFVariable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<NFVariable::NFVariable>>) -> Result<(Option<Pointer::Pointer<Arc<NFVariable::NFVariable>>>, ArcStr)> + 'static>), false), '__try0);
                    }
                    unwrap_break_err!(UnorderedMap::add(cref.clone(), var.index.clone(), local_idx_map.clone()), '__try0);
                }
                for mut var in &*resVars.clone() {
                    let mut var = var.clone();
                    cref = SimVar::getName(var.clone());
                    if unwrap_break_err!(BVariable::checkCref(cref.clone(), (std::sync::Arc::new(fnptr!(BVariable::isPDer, Pointer::Pointer<Arc<NFVariable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<NFVariable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!("NSimCode/NSimJacobian.mo")), '__try0) {
                        cref = unwrap_break_err!(BVariable::getPartnerCref(cref.clone(), (std::sync::Arc::new({ let __pe_b1 = false; move |__pe_a0| Ok(BVariable::getVarPDer(__pe_a0, __pe_b1.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<NFVariable::NFVariable>>) -> Result<(Option<Pointer::Pointer<Arc<NFVariable::NFVariable>>>, ArcStr)> + 'static>), false), '__try0);
                    }
                    unwrap_break_err!(UnorderedMap::add(cref.clone(), var.index.clone(), local_idx_map.clone()), '__try0);
                }
                (sparsity, sparsityT, coloring, rowColoring) = unwrap_break_err!(createSparsity(jacobian.clone(), local_idx_map.clone()), '__try0);
                jac = Arc::new(SimJacobian { name: (var_field!((*jacobian).name, BackendDAE::NBackendDAE::JACOBIAN).clone()).clone(), jacobianIndex: indices.jacobianIndex.clone(), partitionIndex: 0, numberOfResultVars: (resVars.clone().len() as i32), columnEqns: columnEqns.clone(), constantEqns: metamodelica::nil(), columnVars: tmpVars.clone(), seedVars: seedVars.clone(), sparsity: sparsity.clone(), sparsityT: sparsityT.clone(), coloring: coloring.clone(), rowColoring: rowColoring.clone(), numColors: (coloring.clone().len() as i32), generic_loop_calls: generic_loop_calls.clone(), jac_map: Some(jac_map.clone()), isAdjoint: var_field!((*jacobian).isAdjoint, BackendDAE::NBackendDAE::JACOBIAN).clone() });
                indices.jacobianIndex = indices.jacobianIndex.clone() + 1;
                simJacobian = Some(jac.clone());
                Ok::<_, anyhow::Error>((simJacobian.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    simJacobian = __try0_o0;
                }
                Err(_) => {
                    simJacobian = None;
                    Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.create")); __mm_s.push_str(&*literal!(" could not generate sparsity pattern of Jacobian ")); __mm_s.push_str(&*Jacobian::jacobianTypeString(var_field!((*jacobian).jacType, BackendDAE::NBackendDAE::JACOBIAN).clone())); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone())?;
                }
            }
            simJacobian
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.create")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
        Ok((simJacobian, indices))
    }

    pub(crate) fn createSimulationJacobian(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut simCodeIndices: SimCode::SimCodeIndices, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>) -> Result<(Arc<SimJacobian>, Arc<SimJacobian>, SimCode::SimCodeIndices)> {
        let mut simJac: Arc<SimJacobian>;
        let mut simJacAdjoint: Arc<SimJacobian>;
        let mut simCodeIndices: SimCode::SimCodeIndices = simCodeIndices;
        let mut jacobians: Arc<metamodelica::List<Arc<BackendDAE::NBackendDAE>>> = metamodelica::nil();
        let mut jacobiansAdjoint: Arc<metamodelica::List<Arc<BackendDAE::NBackendDAE>>> = metamodelica::nil();
        let mut simJacobian: Arc<BackendDAE::NBackendDAE>;
        let mut simJacobianAdjoint: Arc<BackendDAE::NBackendDAE>;
        let mut simJac_opt: Option<Arc<SimJacobian>>;
        let mut simJacAdj_opt: Option<Arc<SimJacobian>>;
        let mut jacobian: Option<Arc<BackendDAE::NBackendDAE>>;
        let mut jacobianAdjoint: Option<Arc<BackendDAE::NBackendDAE>>;
        for mut partition in &*partitions {
            let mut partition = partition.clone();
            jacobian = Partition::Partition::getJacobian(partition.clone());
            if isSome(jacobian.clone()) {
                jacobians = metamodelica::cons(Util::getOption(jacobian.clone())?, jacobians.clone());
            }
            jacobianAdjoint = Partition::Partition::getJacobianAdjoint(partition.clone());
            if isSome(jacobianAdjoint.clone()) {
                jacobiansAdjoint = metamodelica::cons(Util::getOption(jacobianAdjoint.clone())?, jacobiansAdjoint.clone());
            }
        }
        if jacobians.clone().is_empty() {
            (simJac, simCodeIndices) = empty((literal!("A")).clone(), simCodeIndices)?;
        } else {
            simJacobian = Jacobian::combine(jacobians, (literal!("A")).clone())?;
            (simJac_opt, simCodeIndices) = create(simJacobian, simCodeIndices, simcode_map.clone())?;
            if isSome(simJac_opt.clone()) {
                simJac = Util::getOption(simJac_opt)?;
            } else {
                (simJac, simCodeIndices) = empty((literal!("A")).clone(), simCodeIndices)?;
            }
        }
        if jacobiansAdjoint.clone().is_empty() {
            (simJacAdjoint, simCodeIndices) = empty((literal!("ADJ")).clone(), simCodeIndices)?;
        } else {
            simJacobianAdjoint = Jacobian::combine(jacobiansAdjoint, (literal!("ADJ")).clone())?;
            (simJacAdj_opt, simCodeIndices) = create(simJacobianAdjoint, simCodeIndices, simcode_map)?;
            if isSome(simJacAdj_opt.clone()) {
                simJacAdjoint = Util::getOption(simJacAdj_opt)?;
            } else {
                (simJacAdjoint, simCodeIndices) = empty((literal!("ADJ")).clone(), simCodeIndices)?;
            }
        }
        Ok((simJac, simJacAdjoint, simCodeIndices))
    }

    pub(crate) fn createOptimizationJacobian(mut partitions: Arc<metamodelica::List<Arc<Partition::Partition::Partition>>>, mut simCodeIndices: SimCode::SimCodeIndices, mut simcode_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>) -> Result<(Arc<SimJacobian>, Arc<SimJacobian>, Arc<SimJacobian>, SimCode::SimCodeIndices)> {
        let mut simJacLfg: Arc<SimJacobian>;
        let mut simJacMrf: Arc<SimJacobian>;
        let mut simJacR0: Arc<SimJacobian>;
        let mut simCodeIndices: SimCode::SimCodeIndices = simCodeIndices;
        let mut jacobiansLfg: Arc<metamodelica::List<Arc<BackendDAE::NBackendDAE>>> = metamodelica::nil();
        let mut jacobiansMrf: Arc<metamodelica::List<Arc<BackendDAE::NBackendDAE>>> = metamodelica::nil();
        let mut jacobiansR0: Arc<metamodelica::List<Arc<BackendDAE::NBackendDAE>>> = metamodelica::nil();
        let mut simJacobianLfg: Arc<BackendDAE::NBackendDAE>;
        let mut simJacobianMrf: Arc<BackendDAE::NBackendDAE>;
        let mut simJacobianR0: Arc<BackendDAE::NBackendDAE>;
        let mut simJacLfg_opt: Option<Arc<SimJacobian>>;
        let mut simJacMrf_opt: Option<Arc<SimJacobian>>;
        let mut simJacR0_opt: Option<Arc<SimJacobian>>;
        let mut jacobianLfg: Option<Arc<BackendDAE::NBackendDAE>>;
        let mut jacobianMrf: Option<Arc<BackendDAE::NBackendDAE>>;
        let mut jacobianR0: Option<Arc<BackendDAE::NBackendDAE>>;
        for mut partition in &*partitions {
            let mut partition = partition.clone();
            jacobianLfg = Partition::Partition::getJacobianLfg(partition.clone());
            if isSome(jacobianLfg.clone()) {
                jacobiansLfg = metamodelica::cons(Util::getOption(jacobianLfg.clone())?, jacobiansLfg.clone());
            }
            jacobianMrf = Partition::Partition::getJacobianMrf(partition.clone());
            if isSome(jacobianMrf.clone()) {
                jacobiansMrf = metamodelica::cons(Util::getOption(jacobianMrf.clone())?, jacobiansMrf.clone());
            }
            jacobianR0 = Partition::Partition::getJacobianR0(partition.clone());
            if isSome(jacobianR0.clone()) {
                jacobiansR0 = metamodelica::cons(Util::getOption(jacobianR0.clone())?, jacobiansR0.clone());
            }
        }
        if jacobiansLfg.clone().is_empty() {
            (simJacLfg, simCodeIndices) = empty((literal!("OPT_LFG")).clone(), simCodeIndices)?;
        } else {
            simJacobianLfg = Jacobian::combine(jacobiansLfg, (literal!("OPT_LFG")).clone())?;
            (simJacLfg_opt, simCodeIndices) = create(simJacobianLfg, simCodeIndices, simcode_map.clone())?;
            if isSome(simJacLfg_opt.clone()) {
                simJacLfg = Util::getOption(simJacLfg_opt)?;
            } else {
                (simJacLfg, simCodeIndices) = empty((literal!("OPT_LFG")).clone(), simCodeIndices)?;
            }
        }
        if jacobiansMrf.clone().is_empty() {
            (simJacMrf, simCodeIndices) = empty((literal!("OPT_MRF")).clone(), simCodeIndices)?;
        } else {
            simJacobianMrf = Jacobian::combine(jacobiansMrf, (literal!("OPT_MRF")).clone())?;
            (simJacMrf_opt, simCodeIndices) = create(simJacobianMrf, simCodeIndices, simcode_map.clone())?;
            if isSome(simJacMrf_opt.clone()) {
                simJacMrf = Util::getOption(simJacMrf_opt)?;
            } else {
                (simJacMrf, simCodeIndices) = empty((literal!("OPT_MRF")).clone(), simCodeIndices)?;
            }
        }
        if jacobiansR0.clone().is_empty() {
            (simJacR0, simCodeIndices) = empty((literal!("OPT_R0")).clone(), simCodeIndices)?;
        } else {
            simJacobianR0 = Jacobian::combine(jacobiansR0, (literal!("OPT_R0")).clone())?;
            (simJacR0_opt, simCodeIndices) = create(simJacobianR0, simCodeIndices, simcode_map)?;
            if isSome(simJacR0_opt.clone()) {
                simJacR0 = Util::getOption(simJacR0_opt)?;
            } else {
                (simJacR0, simCodeIndices) = empty((literal!("OPT_R0")).clone(), simCodeIndices)?;
            }
        }
        Ok((simJacLfg, simJacMrf, simJacR0, simCodeIndices))
    }

    pub(crate) fn createSparsity(mut jacobian: Arc<BackendDAE::NBackendDAE>, mut local_idx_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>) -> Result<(Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>, Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
        let mut sparsity: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
        let mut sparsityT: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
        let mut coloring: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        let mut rowColoring: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
        (sparsity, sparsityT, coloring, rowColoring) = (::match_deref::match_deref! { match &(jacobian.clone()) {
        Deref @ BackendDAE::JACOBIAN { sparsityPattern: Bpattern, .. } => {
            sparsity = createSparsityPattern(Bpattern.col_wise_pattern.clone(), local_idx_map.clone())?;
            sparsityT = createSparsityPattern(Bpattern.row_wise_pattern.clone(), local_idx_map.clone())?;
            (coloring, rowColoring) = createSparsityColoring(var_field!((*jacobian).sparsityColoring, BackendDAE::NBackendDAE::JACOBIAN).clone(), local_idx_map)?;
            (sparsity, sparsityT, coloring, rowColoring)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.createSparsity")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((sparsity, sparsityT, coloring, rowColoring))
    }

    pub(crate) fn createSparsityPattern(mut cols: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>>, mut local_idx_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>) -> Result<Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>>> {
        let mut simPattern: Arc<metamodelica::List<(i32, Arc<metamodelica::List<i32>>)>> = metamodelica::nil();
        let mut cref: Arc<ComponentRef::NFComponentRef>;
        let mut dependencies: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
        let mut dep_indices: Arc<metamodelica::List<i32>>;
        for mut col in &*cols {
            let mut col = col.clone();
            (cref, dependencies) = col.clone();
            if !(UnorderedMap::contains(cref.clone(), local_idx_map.clone())?) {
                Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.createSparsityPattern")); __mm_s.push_str(&*literal!(": column cref not found in Jacobian local_idx_map: ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!("\n\tAvailable keys: ")); __mm_s.push_str(&*stringDelimitList(List::map(UnorderedMap::keyList(local_idx_map.clone()), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone())?;
                bail!("fail");
            }
            dep_indices = metamodelica::nil();
            for mut dep in &*dependencies.clone() {
                let mut dep = dep.clone();
                if !(UnorderedMap::contains(dep.clone(), local_idx_map.clone())?) {
                    Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.createSparsityPattern")); __mm_s.push_str(&*literal!(": dependency cref not found in Jacobian local_idx_map: ")); __mm_s.push_str(&*ComponentRef::toString(dep.clone())?); __mm_s.push_str(&*literal!("\n\tWhile processing column: ")); __mm_s.push_str(&*ComponentRef::toString(cref.clone())?); __mm_s.push_str(&*literal!("\n\tAvailable keys: ")); __mm_s.push_str(&*stringDelimitList(List::map(UnorderedMap::keyList(local_idx_map.clone()), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>))?, (literal!(", ")).clone())); ArcStr::from(__mm_s) }).clone())?;
                    bail!("fail");
                }
                dep_indices = metamodelica::cons(UnorderedMap::getOrFail(dep.clone(), local_idx_map.clone())?, dep_indices.clone());
            }
            simPattern = metamodelica::cons((UnorderedMap::getOrFail(cref.clone(), local_idx_map.clone())?, List::sort(dep_indices.clone(), (std::sync::Arc::new(fnptr!(intGt, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>))?), simPattern.clone());
        }
        simPattern = List::sort(simPattern, std::sync::Arc::new(fnptr!(Util::compareTupleIntGt, _, _)))?;
        Ok(simPattern)
    }

    pub(crate) fn createSparsityColoring(mut coloring: Arc<Jacobian::SparsityColoring::SparsityColoring>, mut idx_map: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, i32>>) -> Result<(Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>)> {
        let mut simColoringCols: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
        let mut simColoringRows: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>;
        simColoringCols = ({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut group in (coloring.cols.clone()).borrow().iter() {
            let __x = List::map(group.clone(), (std::sync::Arc::new({ let __pe_b1 = idx_map.clone(); move |__pe_a0| UnorderedMap::getOrFail(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        simColoringRows = ({
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut group in (coloring.rows.clone()).borrow().iter() {
            let __x = List::map(group.clone(), (std::sync::Arc::new({ let __pe_b1 = idx_map.clone(); move |__pe_a0| UnorderedMap::getOrFail(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<_> + 'static>))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        Ok((simColoringCols, simColoringRows))
    }

    pub(crate) fn empty(mut name: ArcStr, mut indices: SimCode::SimCodeIndices) -> Result<(Arc<SimJacobian>, SimCode::SimCodeIndices)> {
        let mut emptyJac: Arc<SimJacobian> = EMPTY_SIM_JAC().clone();
        let mut indices: SimCode::SimCodeIndices = indices;
        emptyJac = (::match_deref::match_deref! { match &(emptyJac.clone()) {
        Deref @ SimJacobian { .. } => {
            assign_field!(
                emptyJac.name = name,
                emptyJac.jacobianIndex = indices.jacobianIndex.clone()
            );
            indices.jacobianIndex = indices.jacobianIndex.clone() + 1;
            emptyJac
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.empty")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok((emptyJac, indices))
    }

    pub(crate) fn getJacobianBlocks(mut jacobian: Arc<SimJacobian>) -> Result<Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>> {
        let mut blcks: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>;
        blcks = (::match_deref::match_deref! { match &(jacobian.clone()) {
        Deref @ SimJacobian { .. } => listAppend(jacobian.constantEqns.clone(), jacobian.columnEqns.clone()),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.getJacobianBlocks")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(blcks)
    }

    pub(crate) fn getJacobiansBlocks(mut jacobians: Arc<metamodelica::List<Arc<SimJacobian>>>) -> Result<Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>>> {
        let mut blcks: Arc<metamodelica::List<Arc<SimStrongComponent::Block::Block>>> = metamodelica::nil();
        for mut jacobian in &*jacobians {
            let mut jacobian = jacobian.clone();
            blcks = listAppend(getJacobianBlocks(jacobian.clone())?, blcks.clone());
        }
        Ok(blcks)
    }

    pub(crate) fn getJacobianHT(mut jacobian: Arc<SimJacobian>) -> Result<Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>>> {
        let mut jac_map: Option<Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>>;
        jac_map = (::match_deref::match_deref! { match &(jacobian.clone()) {
        Deref @ SimJacobian { .. } => jacobian.jac_map.clone(),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.getJacobianHT")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(jac_map)
    }

    pub(crate) fn convert(mut simJac: Arc<SimJacobian>) -> Result<Arc<OldSimCode::JacobianMatrix>> {
        let mut oldJac: Arc<OldSimCode::JacobianMatrix> = Arc::new(<OldSimCode::JacobianMatrix as ::std::default::Default>::default());
        let mut oldJacCol: Arc<OldSimCode::JacobianColumn> = Arc::new(<OldSimCode::JacobianColumn as ::std::default::Default>::default());
        oldJac = (::match_deref::match_deref! { match &(simJac.clone()) {
        Deref @ SimJacobian { .. } => {
            oldJacCol = Arc::new(OldSimCode::JacobianColumn { columnEqns: ({
        let mut __acc: Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>> = metamodelica::nil();
        for mut blck in (simJac.columnEqns.clone()).into_iter().cloned() {
            let __x = SimStrongComponent::Block::convert(blck.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), columnVars: ({
        let mut __acc: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
        for mut var in (simJac.columnVars.clone()).into_iter().cloned() {
            let __x = SimVar::convert(var.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), numberOfResultVars: simJac.numberOfResultVars.clone(), constantEqns: ({
        let mut __acc: Arc<metamodelica::List<Arc<OldSimCode::SimEqSystem>>> = metamodelica::nil();
        for mut blck in (simJac.constantEqns.clone()).into_iter().cloned() {
            let __x = SimStrongComponent::Block::convert(blck.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) });
            oldJac = Arc::new(OldSimCode::JacobianMatrix { columns: list![oldJacCol], seedVars: SimVar::convertList(simJac.seedVars.clone())?, matrixName: (simJac.name.clone()).clone(), sparsity: simJac.sparsity.clone(), sparsityT: simJac.sparsityT.clone(), nonlinear: metamodelica::nil(), nonlinearT: metamodelica::nil(), coloredCols: simJac.coloring.clone(), coloredRows: simJac.rowColoring.clone(), maxColorCols: simJac.numColors.clone(), jacobianIndex: simJac.jacobianIndex.clone(), partitionIndex: simJac.partitionIndex.clone(), generic_loop_calls: ({
        let mut __acc: Arc<metamodelica::List<OldSimCode::SimGenericCall>> = metamodelica::nil();
        for mut gc in (simJac.generic_loop_calls.clone()).into_iter().cloned() {
            let __x = SimGenericCall::convert(gc.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), crefsHT: Util::applyOption(simJac.jac_map.clone(), (std::sync::Arc::new(SimCodeUtil::convertSimCodeMap) as std::sync::Arc<dyn ::std::ops::Fn(Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<SimVar::SimVar>>>) -> Result<(metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(SimCodeVar::SimVar) -> Result<ArcStr> + 'static>))> + 'static>))?, isAdjoint: simJac.isAdjoint.clone() });
            oldJac
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.convert")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(oldJac)
    }

}

thread_local! { static __EMPTY_SIM_JAC_TLS: Arc<SimJacobian::SimJacobian> = Arc::new(SimJacobian::SimJacobian { name: (literal!("")).clone(), jacobianIndex: 0, partitionIndex: 0, numberOfResultVars: 0, columnEqns: metamodelica::nil(), constantEqns: metamodelica::nil(), columnVars: metamodelica::nil(), seedVars: metamodelica::nil(), sparsity: metamodelica::nil(), sparsityT: metamodelica::nil(), coloring: metamodelica::nil(), rowColoring: metamodelica::nil(), numColors: 0, generic_loop_calls: metamodelica::nil(), jac_map: None, isAdjoint: false }); }
pub(crate) fn EMPTY_SIM_JAC() -> Arc<SimJacobian::SimJacobian> { __EMPTY_SIM_JAC_TLS.with(|__t| __t.clone()) }

