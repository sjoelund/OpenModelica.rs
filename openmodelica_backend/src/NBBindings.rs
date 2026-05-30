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

use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointers;
use crate::NBModule as Module;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedSet;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// Util
pub fn main(mut bdae: Arc<BackendDAE::NBackendDAE>) -> Result<Arc<BackendDAE::NBackendDAE>> {
    let mut bdae: Arc<BackendDAE::NBackendDAE> = bdae;
    bdae = ({
        let mut binding_cont: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        let mut binding_clck: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        let mut binding_disc: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        let mut binding_rec: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
        let mut new_iters: Arc<UnorderedSet::UnorderedSet<Pointer::Pointer<Arc<Variable::NFVariable>>>> = UnorderedSet::new((std::sync::Arc::new(fnptr!(BVariable::hash, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(BVariable::equalName, Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>, Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), 13);
        (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ BackendDAE::MAIN { eqData: eqData @ Deref @ EqData::EQ_DATA_SIM { .. }, varData: varData @ Deref @ BVariable::VarData::VAR_DATA_SIM { .. }, .. } => {
            let mut bind_eqn: Pointer::Pointer<Arc<Equation::Equation>>;
            let mut bound_vars: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
            let mut bound_clocks: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
            let mut parent: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut skip_record_element: bool = false;
            let mut eqData = (*eqData).clone();
            bound_vars = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (BVariable::VariablePointers::toList(var_field!((**varData).unknowns, VarData::VarData::VAR_DATA_SIM).clone())?).into_iter().cloned() {
            if !(BVariable::isBound(var.clone())) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            for mut var in &*bound_vars.clone() {
                let mut var = var.clone();
                skip_record_element = (match BVariable::getParent(var.clone()) {
        Some(mut parent) => BVariable::isBound(parent.clone()) && BVariable::isUnknownRecord(parent.clone()),
        _ => false,
    });
                if !(skip_record_element.clone()) {
                    bind_eqn = Equation::generateBindingEquation(var.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), false, new_iters.clone())?;
                    if BVariable::isContinuous(var.clone(), false)? {
                        binding_cont = cons(bind_eqn.clone(), binding_cont.clone());
                    } else {
                        binding_disc = cons(bind_eqn.clone(), binding_disc.clone());
                    }
                }
            }
            bound_vars = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (BVariable::VariablePointers::toList(var_field!((**varData).records, VarData::VarData::VAR_DATA_SIM).clone())?).into_iter().cloned() {
            if !(BVariable::isBound(var.clone()) && BVariable::isUnknownRecord(var.clone())) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            for mut var in &*bound_vars.clone() {
                let mut var = var.clone();
                binding_rec = cons(Equation::generateBindingEquation(var.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), false, new_iters.clone())?, binding_rec.clone());
            }
            bound_clocks = ({
        let mut __acc: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>> = metamodelica::nil();
        for mut var in (BVariable::VariablePointers::toList(var_field!((**varData).clocks, VarData::VarData::VAR_DATA_SIM).clone())?).into_iter().cloned() {
            if !(BVariable::isBound(var.clone())) { continue; }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            for mut var in &*bound_clocks.clone() {
                let mut var = var.clone();
                binding_clck = cons(Equation::generateBindingEquation(var.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), false, new_iters.clone())?, binding_clck.clone());
            }
            assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM;
                equations = EquationPointers::addList(binding_cont.clone(), var_field!((*eqData).equations, EqData::EqData::EQ_DATA_SIM).clone()),
                simulation = EquationPointers::addList(binding_cont.clone(), var_field!((*eqData).simulation, EqData::EqData::EQ_DATA_SIM).clone()),
                continuous = EquationPointers::addList(binding_cont.clone(), var_field!((*eqData).continuous, EqData::EqData::EQ_DATA_SIM).clone()),
                equations = EquationPointers::addList(binding_disc.clone(), var_field!((*eqData).equations, EqData::EqData::EQ_DATA_SIM).clone()),
                simulation = EquationPointers::addList(binding_disc.clone(), var_field!((*eqData).simulation, EqData::EqData::EQ_DATA_SIM).clone()),
                discretes = EquationPointers::addList(binding_disc.clone(), var_field!((*eqData).discretes, EqData::EqData::EQ_DATA_SIM).clone()),
                equations = EquationPointers::addList(binding_rec.clone(), var_field!((*eqData).equations, EqData::EqData::EQ_DATA_SIM).clone()),
                simulation = EquationPointers::addList(binding_rec.clone(), var_field!((*eqData).simulation, EqData::EqData::EQ_DATA_SIM).clone()),
                continuous = EquationPointers::addList(binding_rec.clone(), var_field!((*eqData).continuous, EqData::EqData::EQ_DATA_SIM).clone()),
                clocked = EquationPointers::addList(binding_clck.clone(), var_field!((*eqData).clocked, EqData::EqData::EQ_DATA_SIM).clone())
            );
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                eqData = eqData.clone(),
                varData = BVariable::VarData::addTypedList(var_field!((*bdae).varData, BackendDAE::NBackendDAE::MAIN).clone(), UnorderedSet::toList(new_iters.clone()), BVariable::VarData::VarType::ITERATOR.clone())?
            );
            if Flags::isSet(Flags::DUMP_BACKENDDAE_INFO.clone())? {
                Error::addSourceMessage(Error::BACKENDDAEINFO_LOWER.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(EqData::scalarSize(eqData.clone(), false)?)); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(EqData::size(eqData.clone())?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(BVariable::VarData::scalarSize(varData.clone(), false)?)); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BVariable::VarData::size(varData.clone())?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone()], Absyn::dummyInfo.clone())?;
            }
            if Flags::isSet(Flags::DUMP_BINDINGS.clone())? {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(binding_cont.clone(), Arc::new({ let __pe_b1 = (literal!("\t")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Created Continuous Binding Equations (")); __mm_s.push_str(&*intString((binding_cont.clone().len() as i32))); __mm_s.push_str(&*literal!("):")); ArcStr::from(__mm_s) }).clone())).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), false, 0)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(binding_clck.clone(), Arc::new({ let __pe_b1 = (literal!("\t")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Created Clocked Binding Equations (")); __mm_s.push_str(&*intString((binding_clck.clone().len() as i32))); __mm_s.push_str(&*literal!("):")); ArcStr::from(__mm_s) }).clone())).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), false, 0)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(binding_disc.clone(), Arc::new({ let __pe_b1 = (literal!("\t")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Created Discrete Binding Equations (")); __mm_s.push_str(&*intString((binding_disc.clone().len() as i32))); __mm_s.push_str(&*literal!("):")); ArcStr::from(__mm_s) }).clone())).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), false, 0)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(binding_rec.clone(), Arc::new({ let __pe_b1 = (literal!("\t")).clone(); move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), (StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Created Record Binding Equations (")); __mm_s.push_str(&*intString((binding_rec.clone().len() as i32))); __mm_s.push_str(&*literal!("):")); ArcStr::from(__mm_s) }).clone())).clone(), (literal!("")).clone(), (literal!("\n")).clone(), (literal!("")).clone(), false, 0)?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            bdae.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBBindings.main")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
    });
    Ok(bdae)
}

