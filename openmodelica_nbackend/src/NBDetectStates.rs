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

use crate::NBDifferentiate as Differentiate;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NBFunctionAlias as FunctionAlias;
use crate::NBModule as Module;
use crate::NBPartition as BPartition;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_nf_frontend::NFBackendExtension::VariableKind;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction as Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// Old Frontend Imports
// New Frontend Imports
// Backend imports
// Util
// =========================================================================
//                      MAIN ROUTINE, PLEASE DO NOT CHANGE
// =========================================================================
pub fn main(mut bdae: Arc<BackendDAE::NBackendDAE>) -> Result<Arc<BackendDAE::NBackendDAE>> {
    let mut bdae: Arc<BackendDAE::NBackendDAE> = bdae;
    let mut mainFunc: Module::detectStatesInterface;
    let mut contFunc: Module::detectContinuousStatesInterface;
    let mut discFunc: Module::detectDiscreteStatesInterface;
    (mainFunc, contFunc, discFunc) = getModule()?;
    bdae = (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ BackendDAE::MAIN { eqData, varData, .. } => {
            let mut eqData = (*eqData).clone();
            let mut varData = (*varData).clone();
            (varData, eqData) = mainFunc(varData.clone(), eqData.clone(), contFunc.clone(), discFunc.clone())?;
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                varData = varData.clone(),
                eqData = eqData.clone()
            );
            bdae.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDetectStates.main")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bdae)
}

pub fn getModule() -> Result<(Arc<dyn ::std::ops::Fn(Arc<VarData::VarData>, Arc<EqData::EqData>, Arc<dyn ::std::ops::Fn(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>)> + 'static>, Arc<dyn ::std::ops::Fn(Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, ArcStr) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>)> + 'static>) -> Result<(Arc<VarData::VarData>, Arc<EqData::EqData>)> + 'static>, Arc<dyn ::std::ops::Fn(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>)> + 'static>, Arc<dyn ::std::ops::Fn(Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, ArcStr) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>)> + 'static>)> {
    let mut mainFunc: Module::detectStatesInterface;
    let mut contFunc: Module::detectContinuousStatesInterface;
    let mut discFunc: Module::detectDiscreteStatesInterface;
    let mut flag: ArcStr = literal!("default");
    (mainFunc, contFunc, discFunc) = (::match_deref::match_deref! { match &(flag.clone()) {
        Deref @ "default" => ((std::sync::Arc::new(detectStatesDefault) as std::sync::Arc<dyn ::std::ops::Fn(Arc<VarData::VarData>, Arc<EqData::EqData>, Arc<dyn ::std::ops::Fn(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>)> + 'static>, Arc<dyn ::std::ops::Fn(Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, ArcStr) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>)> + 'static>) -> Result<(Arc<VarData::VarData>, Arc<EqData::EqData>)> + 'static>), (std::sync::Arc::new(detectContinuousStatesDefault) as std::sync::Arc<dyn ::std::ops::Fn(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>)> + 'static>), (std::sync::Arc::new(detectDiscreteStatesDefault) as std::sync::Arc<dyn ::std::ops::Fn(Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, ArcStr) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>)> + 'static>)),
        _ => bail!("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((mainFunc, contFunc, discFunc))
}

/* =========================================================================
                              SUB ROUTINES
========================================================================= */
fn detectStatesDefault(mut varData: Arc<VarData::VarData>, mut eqData: Arc<EqData::EqData>, mut continuousFunc: Arc<dyn ::std::ops::Fn(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>)> + 'static>, mut discreteFunc: Arc<dyn ::std::ops::Fn(Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, ArcStr) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>)> + 'static>) -> Result<(Arc<VarData::VarData>, Arc<EqData::EqData>)> {
    let mut varData: Arc<VarData::VarData> = varData;
    let mut eqData: Arc<EqData::EqData> = eqData;
    let mut variables: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut equations: Arc<EquationPointers::EquationPointers> = Arc::new(<EquationPointers::EquationPointers as ::std::default::Default>::default());
    let mut disc_eqns: Arc<EquationPointers::EquationPointers> = Arc::new(<EquationPointers::EquationPointers as ::std::default::Default>::default());
    let mut init_eqns: Arc<EquationPointers::EquationPointers> = Arc::new(<EquationPointers::EquationPointers as ::std::default::Default>::default());
    let mut unknowns: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut knowns: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut initials: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut states: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut derivatives: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut algebraics: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut discretes: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut discrete_states: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut clocked_states: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut previous: Arc<VariablePointers::VariablePointers> = Arc::new(<VariablePointers::VariablePointers as ::std::default::Default>::default());
    let mut aux_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut newEqData: Arc<EqData::EqData> = Arc::new(EqData::EQ_DATA_EMPTY);
    (varData, eqData) = FunctionAlias::introduceSlicedStateAlias(varData.clone(), eqData.clone(), BPartition::Kind::ODE.clone())?;
    (varData, eqData) = (::match_deref::match_deref! { match &((varData.clone(), eqData.clone())) {
        (Deref @ BVariable::VarData::VAR_DATA_SIM { .. }, Deref @ BEquation::EqData::EQ_DATA_SIM { .. }) => {
            (variables, unknowns, knowns, initials, states, derivatives, algebraics, aux_eqns) = continuousFunc(var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).unknowns, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).knowns, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).initials, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).states, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).derivatives, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).algebraics, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*eqData).equations, EqData::EqData::EQ_DATA_SIM).clone())?;
            (variables, disc_eqns, knowns, initials, discretes, discrete_states, clocked_states, previous) = discreteFunc(var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*eqData).discretes, EqData::EqData::EQ_DATA_SIM).clone(), var_field!((*varData).knowns, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).initials, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).discretes, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).discrete_states, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).clocked_states, VarData::VarData::VAR_DATA_SIM).clone(), var_field!((*varData).previous, VarData::VarData::VAR_DATA_SIM).clone(), (literal!("discrete equations")).clone())?;
            (variables, disc_eqns, knowns, initials, discretes, discrete_states, clocked_states, previous) = discreteFunc(variables.clone(), var_field!((*eqData).clocked, EqData::EqData::EQ_DATA_SIM).clone(), knowns.clone(), initials.clone(), discretes.clone(), discrete_states.clone(), clocked_states.clone(), previous.clone(), (literal!("clocked equations")).clone())?;
            (variables, disc_eqns, knowns, initials, discretes, discrete_states, clocked_states, previous) = discreteFunc(variables.clone(), var_field!((*eqData).continuous, EqData::EqData::EQ_DATA_SIM).clone(), knowns.clone(), initials.clone(), discretes.clone(), discrete_states.clone(), clocked_states.clone(), previous.clone(), (literal!("continuous equations")).clone())?;
            (variables, init_eqns, knowns, initials, discretes, discrete_states, clocked_states, previous) = discreteFunc(variables.clone(), var_field!((*eqData).initials, EqData::EqData::EQ_DATA_SIM).clone(), knowns.clone(), initials.clone(), discretes.clone(), discrete_states.clone(), clocked_states.clone(), previous.clone(), (literal!("initial equations")).clone())?;
            assign_variant_field!(varData => VarData::VarData::VAR_DATA_SIM;
                variables = variables.clone(),
                unknowns = unknowns.clone(),
                knowns = knowns.clone(),
                initials = initials.clone(),
                derivatives = derivatives.clone(),
                algebraics = algebraics.clone(),
                discretes = discretes.clone(),
                discrete_states = discrete_states.clone(),
                clocked_states = clocked_states.clone(),
                previous = previous.clone(),
                states = states.clone()
            );
            newEqData = BEquation::EqData::addTypedList(eqData.clone(), aux_eqns.clone(), EqData::EqType::CONTINUOUS.clone(), false)?;
            BEquation::EquationPointers::map(BEquation::EqData::getEquations(newEqData.clone())?, (std::sync::Arc::new({ let __pe_b1 = var_field!((*varData).state_order, VarData::VarData::VAR_DATA_SIM).clone(); move |__pe_a0| stateOrder(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> + 'static>))?;
            if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? && !(UnorderedMap::isEmpty(var_field!((*varData).state_order, VarData::VarData::VAR_DATA_SIM).clone())) {
                metamodelica::print((StringUtil::headline_4((literal!("[stateselection] State Order:")).clone())).clone());
                metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*UnorderedMap::toString(var_field!((*varData).state_order, VarData::VarData::VAR_DATA_SIM).clone(), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(ComponentRef::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<ArcStr> + 'static>), (literal!("\n\t")).clone(), (literal!(" --d/dt--> ")).clone())?); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }).clone());
            }
            (varData.clone(), newEqData.clone())
        },
        _ => (varData.clone(), eqData.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((varData, eqData))
}

fn detectContinuousStatesDefault(mut variables: Arc<VariablePointers::VariablePointers>, mut unknowns: Arc<VariablePointers::VariablePointers>, mut knowns: Arc<VariablePointers::VariablePointers>, mut initials: Arc<VariablePointers::VariablePointers>, mut states: Arc<VariablePointers::VariablePointers>, mut derivatives: Arc<VariablePointers::VariablePointers>, mut algebraics: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>)> {
    let mut variables: Arc<VariablePointers::VariablePointers> = variables;
    let mut unknowns: Arc<VariablePointers::VariablePointers> = unknowns;
    let mut knowns: Arc<VariablePointers::VariablePointers> = knowns;
    let mut initials: Arc<VariablePointers::VariablePointers> = initials;
    let mut states: Arc<VariablePointers::VariablePointers> = states;
    let mut derivatives: Arc<VariablePointers::VariablePointers> = derivatives;
    let mut algebraics: Arc<VariablePointers::VariablePointers> = algebraics;
    let mut aux_eqns: Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>> = metamodelica::nil();
    let mut acc_states: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    let mut acc_derivatives: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    let mut acc_aux_equations: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>> = Pointer::create(metamodelica::nil());
    let mut uniqueIndex: Pointer::Pointer<i32> = Pointer::create(0);
    let mut diffArgs: Arc<Differentiate::DifferentiationArguments::DifferentiationArguments> = Differentiate::DifferentiationArguments::default(Differentiate::DifferentiationType::TIME.clone(), UnorderedMap::new((std::sync::Arc::new(AbsynUtil::pathHash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(AbsynUtil::pathEqual, Arc<Absyn::Path>, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>), 1));
    BEquation::EquationPointers::mapExp(equations.clone(), (std::sync::Arc::new({ let __pe_b1 = acc_states.clone(); let __pe_b2 = acc_derivatives.clone(); let __pe_b3 = variables.scalarized.clone(); move |__pe_a0| collectStatesAndDerivatives(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    BEquation::EquationPointers::mapExp(equations.clone(), (std::sync::Arc::new({ let __pe_b1 = acc_states.clone(); let __pe_b2 = acc_derivatives.clone(); let __pe_b3 = acc_aux_equations.clone(); let __pe_b4 = uniqueIndex.clone(); let __pe_b5 = diffArgs.clone(); move |__pe_a0| resolveGeneralDer(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    (variables, unknowns, knowns, initials, states, derivatives, algebraics) = updateStatesAndDerivatives(variables.clone(), unknowns.clone(), knowns.clone(), initials.clone(), states.clone(), derivatives.clone(), algebraics.clone(), Pointer::access(acc_states.clone()), Pointer::access(acc_derivatives.clone()))?;
    aux_eqns = Pointer::access(acc_aux_equations.clone());
    if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? && !(aux_eqns.clone().is_empty()) {
        metamodelica::print((StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[stateselection] (")); __mm_s.push_str(&*intString((aux_eqns.clone().len() as i32))); __mm_s.push_str(&*literal!(") Created auxiliary equations:")); ArcStr::from(__mm_s) }).clone())).clone());
        metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(aux_eqns.clone(), (std::sync::Arc::new({ let __pe_b1 = (literal!("")).clone(); move |__pe_a0| BEquation::Equation::pointerToString(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Equation::Equation>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("\t")).clone(), (literal!("\n\t")).clone(), (literal!("\n")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok((variables, unknowns, knowns, initials, states, derivatives, algebraics, aux_eqns))
}

fn detectDiscreteStatesDefault(mut variables: Arc<VariablePointers::VariablePointers>, mut equations: Arc<EquationPointers::EquationPointers>, mut knowns: Arc<VariablePointers::VariablePointers>, mut initials: Arc<VariablePointers::VariablePointers>, mut discretes: Arc<VariablePointers::VariablePointers>, mut discrete_states: Arc<VariablePointers::VariablePointers>, mut clocked_states: Arc<VariablePointers::VariablePointers>, mut previous: Arc<VariablePointers::VariablePointers>, mut context: ArcStr) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<EquationPointers::EquationPointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>)> {
    let mut variables: Arc<VariablePointers::VariablePointers> = variables;
    let mut equations: Arc<EquationPointers::EquationPointers> = equations;
    let mut knowns: Arc<VariablePointers::VariablePointers> = knowns;
    let mut initials: Arc<VariablePointers::VariablePointers> = initials;
    let mut discretes: Arc<VariablePointers::VariablePointers> = discretes;
    let mut discrete_states: Arc<VariablePointers::VariablePointers> = discrete_states;
    let mut clocked_states: Arc<VariablePointers::VariablePointers> = clocked_states;
    let mut previous: Arc<VariablePointers::VariablePointers> = previous;
    let mut acc_discrete_states: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    let mut acc_clocked_states: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    let mut acc_previous: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>> = Pointer::create(metamodelica::nil());
    BEquation::EquationPointers::map(equations.clone(), (std::sync::Arc::new({ let __pe_b1 = acc_discrete_states.clone(); let __pe_b2 = acc_previous.clone(); let __pe_b3 = variables.scalarized.clone(); move |__pe_a0| collectDiscreteStatesFromWhen(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::Equation>) -> Result<Arc<Equation::Equation>> + 'static>))?;
    BEquation::EquationPointers::mapExp(equations.clone(), (std::sync::Arc::new({ let __pe_b1 = acc_previous.clone(); let __pe_b2 = acc_clocked_states.clone(); let __pe_b3 = variables.scalarized.clone(); move |__pe_a0| collectPreAndPrevious(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    (variables, knowns, initials, discretes, discrete_states, clocked_states, previous) = updateDiscreteStatesAndPrevious(variables.clone(), knowns.clone(), initials.clone(), discretes.clone(), discrete_states.clone(), clocked_states.clone(), previous.clone(), Pointer::access(acc_discrete_states.clone()), Pointer::access(acc_clocked_states.clone()), Pointer::access(acc_previous.clone()), (context.clone()).clone())?;
    Ok((variables, equations, knowns, initials, discretes, discrete_states, clocked_states, previous))
}

fn collectStatesAndDerivatives(mut exp: Arc<Expression::NFExpression>, mut acc_states: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut acc_derivatives: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut scalarized: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: state_cref, .. }, tail: Deref @ metamodelica::List::Nil }, r#fn: Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, .. } } if (!(BVariable::checkCref(state_cref.clone(), (std::sync::Arc::new(fnptr!(BVariable::isStateDerivative, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<bool> + 'static>), metamodelica::sourceInfo!())?)) => {
            let mut res: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut der_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut state_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut der_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            state_var = BVariable::getVarPointer(state_cref.clone(), metamodelica::sourceInfo!())?;
            if !(BVariable::isContinuous(state_var.clone(), false)?) {
                res = Expression::makeZero(ComponentRef::getSubscriptedType(state_cref.clone(), false)?)?;
            } else {
                if BVariable::hasDerVar(state_var.clone()) {
                    der_cref = BVariable::getPartnerCref(state_cref.clone(), (std::sync::Arc::new(fnptr!(BVariable::getVarDer, Pointer::Pointer<Arc<Variable::NFVariable>>)) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<(Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr)> + 'static>), scalarized.clone())?;
                } else {
                    (der_cref, der_var) = BVariable::makeDerVar(state_cref.clone(), scalarized.clone())?;
                    state_var = BVariable::getVarPointer(state_cref.clone(), metamodelica::sourceInfo!())?;
                    BVariable::setStateDerivativeVar(state_var.clone(), der_var.clone());
                    Pointer::update(acc_states.clone(), metamodelica::cons(state_var.clone(), Pointer::access(acc_states.clone())));
                    Pointer::update(acc_derivatives.clone(), metamodelica::cons(der_var.clone(), Pointer::access(acc_derivatives.clone())));
                }
                res = Expression::fromCref(der_cref.clone(), false)?;
            }
            res.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn resolveGeneralDer(mut exp: Arc<Expression::NFExpression>, mut acc_states: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut acc_derivatives: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut acc_aux_equations: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Equation::Equation>>>>>, mut uniqueIndex: Pointer::Pointer<i32>, mut diffArgs: Arc<Differentiate::DifferentiationArguments::DifferentiationArguments>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::List::Cons { head: arg, tail: Deref @ metamodelica::List::Nil }, r#fn: Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, .. } } => {
            let mut state_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut der_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
            let mut state_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut der_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut returnExp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut aux_equation: Pointer::Pointer<Arc<Equation::Equation>>;
            let mut oDiffArgs: Arc<Differentiate::DifferentiationArguments::DifferentiationArguments> = Arc::new(<Differentiate::DifferentiationArguments::DifferentiationArguments as ::std::default::Default>::default());
            if Expression::fold(arg.clone(), (std::sync::Arc::new(checkAlgebraic) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, i32) -> Result<i32> + 'static>), 0)? > 1 {
                (state_var, state_cref, der_var, der_cref) = BVariable::makeAuxStateVar(Pointer::access(uniqueIndex.clone()), Some(arg.clone()))?;
                aux_equation = BEquation::Equation::makeAssignment(Expression::fromCref(state_cref.clone(), false)?, arg.clone(), uniqueIndex.clone(), (arcstr::literal!(BVariable::AUXILIARY_STR)).clone(), Arc::new(crate::NBEquation::Iterator::EMPTY), BEquation::default(EquationKind::CONTINUOUS.clone(), false, None, None))?;
                returnExp = Expression::fromCref(der_cref.clone(), false)?;
                Pointer::update(acc_states.clone(), metamodelica::cons(state_var.clone(), Pointer::access(acc_states.clone())));
                Pointer::update(acc_derivatives.clone(), metamodelica::cons(der_var.clone(), Pointer::access(acc_derivatives.clone())));
                Pointer::update(acc_aux_equations.clone(), metamodelica::cons(aux_equation.clone(), Pointer::access(acc_aux_equations.clone())));
            } else {
                (returnExp, oDiffArgs) = Differentiate::differentiateExpression(arg.clone(), diffArgs.clone())?;
                returnExp = SimplifyExp::simplifyDump(returnExp.clone(), true, literal!("NBDetectStates.resolveGeneralDer"), (literal!("")).clone())?;
                if List::hasOneElement(oDiffArgs.new_vars.clone()) {
                    der_var = listHead(oDiffArgs.new_vars.clone())?;
                    Pointer::update(acc_derivatives.clone(), metamodelica::cons(der_var.clone(), Pointer::access(acc_derivatives.clone())));
                    Pointer::update(acc_states.clone(), metamodelica::cons(Util::getOption((BVariable::getVarState(der_var.clone())).0)?, Pointer::access(acc_states.clone())));
                } else if List::hasSeveralElements(oDiffArgs.new_vars.clone()) {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDetectStates.resolveGeneralDer")); __mm_s.push_str(&*literal!(" failed because the number of algebraic variables were miscounted! ")); __mm_s.push_str(&*literal!("Expected: 0 or 1, got: ")); __mm_s.push_str(&*intString((oDiffArgs.new_vars.clone().len() as i32))); ArcStr::from(__mm_s) }).clone()])?;
                    bail!("fail");
                }
            }
            returnExp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn checkAlgebraic(mut exp: Arc<Expression::NFExpression>, mut i: i32) -> Result<i32> {
    let mut i: i32 = i;
    i = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { .. } if (BVariable::isStateDerivative(BVariable::getVarPointer(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!())?)) => i.clone() + 2,
        Deref @ Expression::CREF { .. } if (BVariable::isAlgebraic(BVariable::getVarPointer(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!())?)) => i.clone() + 1,
        _ => i.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(i)
}

fn updateStatesAndDerivatives(mut variables: Arc<VariablePointers::VariablePointers>, mut unknowns: Arc<VariablePointers::VariablePointers>, mut knowns: Arc<VariablePointers::VariablePointers>, mut initials: Arc<VariablePointers::VariablePointers>, mut states: Arc<VariablePointers::VariablePointers>, mut derivatives: Arc<VariablePointers::VariablePointers>, mut algebraics: Arc<VariablePointers::VariablePointers>, mut acc_states: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut acc_derivatives: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>)> {
    let mut variables: Arc<VariablePointers::VariablePointers> = variables;
    let mut unknowns: Arc<VariablePointers::VariablePointers> = unknowns;
    let mut knowns: Arc<VariablePointers::VariablePointers> = knowns;
    let mut initials: Arc<VariablePointers::VariablePointers> = initials;
    let mut states: Arc<VariablePointers::VariablePointers> = states;
    let mut derivatives: Arc<VariablePointers::VariablePointers> = derivatives;
    let mut algebraics: Arc<VariablePointers::VariablePointers> = algebraics;
    variables = BVariable::VariablePointers::addList(acc_derivatives.clone(), variables.clone())?;
    unknowns = BVariable::VariablePointers::addList(acc_derivatives.clone(), unknowns.clone())?;
    initials = BVariable::VariablePointers::addList(acc_derivatives.clone(), initials.clone())?;
    derivatives = BVariable::VariablePointers::addList(acc_derivatives.clone(), derivatives.clone())?;
    variables = BVariable::VariablePointers::addList(acc_states.clone(), variables.clone())?;
    states = BVariable::VariablePointers::addList(acc_states.clone(), states.clone())?;
    unknowns = BVariable::VariablePointers::removeList(acc_states.clone(), unknowns.clone())?;
    algebraics = BVariable::VariablePointers::removeList(acc_states.clone(), algebraics.clone())?;
    if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? {
        metamodelica::print((StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[stateselection] (")); __mm_s.push_str(&*intString((acc_states.clone().len() as i32))); __mm_s.push_str(&*literal!(") Natural states before index reduction:")); ArcStr::from(__mm_s) }).clone())).clone());
        if acc_states.clone().is_empty() {
            metamodelica::print((literal!("\t<no states>\n\n")).clone());
        } else {
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(acc_states.clone(), (std::sync::Arc::new(BVariable::pointerToString) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("\t")).clone(), (literal!("\n\t")).clone(), (literal!("\n")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    Ok((variables, unknowns, knowns, initials, states, derivatives, algebraics))
}

fn collectPreAndPrevious(mut exp: Arc<Expression::NFExpression>, mut acc_previous: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut acc_clocked_states: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut scalarized: bool) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::List::Cons { head: Deref @ Expression::BOOLEAN { value: b }, tail: Deref @ metamodelica::List::Nil }, r#fn, .. } } => {
            let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            new_exp = (::match_deref::match_deref! { match &(r#fn.clone()) {
        Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. } => Arc::new(Expression::NFExpression::BOOLEAN { value: b.clone() }),
        Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => Arc::new(Expression::NFExpression::BOOLEAN { value: b.clone() }),
        Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. } => Arc::new(Expression::NFExpression::BOOLEAN { value: false }),
        Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. } => Arc::new(Expression::NFExpression::BOOLEAN { value: false }),
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            new_exp.clone()
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: args, r#fn: Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, .. } } => {
            let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut old_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (new_exp, old_exp) = preFromArgs(args.clone(), acc_previous.clone(), scalarized.clone(), (literal!("previous")).clone())?;
            let () = (::match_deref::match_deref! { match &(old_exp.clone()) {
        Deref @ Expression::CREF { .. } => {
            Pointer::update(acc_clocked_states.clone(), metamodelica::cons(BVariable::getVarPointer(var_field!((*old_exp).cref, Expression::NFExpression::CREF).clone(), metamodelica::sourceInfo!())?, Pointer::access(acc_clocked_states.clone())));
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDetectStates.collectPreAndPrevious")); __mm_s.push_str(&*literal!(" failed because previous() can only contain component references, but contained: ")); __mm_s.push_str(&*Expression::toString(old_exp.clone())?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            new_exp.clone()
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: args, r#fn: Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, .. } } => {
            let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (new_exp, _) = preFromArgs(args.clone(), acc_previous.clone(), scalarized.clone(), (literal!("pre")).clone())?;
            new_exp.clone()
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: args, r#fn: Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. }, .. } } => {
            let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut old_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (new_exp, old_exp) = preFromArgs(args.clone(), acc_previous.clone(), scalarized.clone(), (literal!("edge")).clone())?;
            Arc::new(Expression::NFExpression::LBINARY { exp1: old_exp.clone(), operator: Operator::makeAnd(Expression::typeOf(old_exp.clone())), exp2: Expression::logicNegate(new_exp.clone()) })
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: args, r#fn: Deref @ Function::Function::FUNCTION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. }, .. } } => {
            let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            let mut old_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
            (new_exp, old_exp) = preFromArgs(args.clone(), acc_previous.clone(), scalarized.clone(), (literal!("change")).clone())?;
            Arc::new(Expression::NFExpression::RELATION { exp1: old_exp.clone(), operator: Operator::makeNotEqual(Expression::typeOf(old_exp.clone())), exp2: new_exp.clone(), index: -1 })
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn preFromArgs(mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>, mut acc_previous: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut scalarized: bool, mut context: ArcStr) -> Result<(Arc<Expression::NFExpression>, Arc<Expression::NFExpression>)> {
    let mut new_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut old_exp: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut state_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut pre_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut state_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let mut negated: bool = false;
    (state_var, old_exp, negated) = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: __esc_old_exp @ Deref @ Expression::CREF { cref: state_cref, .. }, tail: Deref @ metamodelica::List::Nil } => {
            old_exp = (*__esc_old_exp).clone();
            (BVariable::getVarPointer(state_cref.clone(), metamodelica::sourceInfo!())?, old_exp.clone(), false)
        },
        Deref @ metamodelica::List::Cons { head: __esc_old_exp @ Deref @ Expression::LUNARY { exp: Deref @ Expression::CREF { cref: state_cref, .. }, .. }, tail: Deref @ metamodelica::List::Nil } => {
            old_exp = (*__esc_old_exp).clone();
            (BVariable::getVarPointer(state_cref.clone(), metamodelica::sourceInfo!())?, old_exp.clone(), true)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBDetectStates.preFromArgs")); __mm_s.push_str(&*literal!(" failed because of unexpected expression ")); __mm_s.push_str(&*context.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*List::toString(args.clone(), (std::sync::Arc::new(Expression::toString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("")).clone(), (literal!(", ")).clone(), (literal!("")).clone(), true, 0)?); __mm_s.push_str(&*literal!(").")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    pre_cref = getPreVar(state_cref.clone(), state_var.clone(), acc_previous.clone(), scalarized.clone())?;
    new_exp = Expression::fromCref(pre_cref.clone(), false)?;
    if negated.clone() {
        new_exp = Expression::logicNegate(new_exp.clone());
    }
    Ok((new_exp, old_exp))
}

fn updateDiscreteStatesAndPrevious(mut variables: Arc<VariablePointers::VariablePointers>, mut knowns: Arc<VariablePointers::VariablePointers>, mut initials: Arc<VariablePointers::VariablePointers>, mut discretes: Arc<VariablePointers::VariablePointers>, mut discrete_states: Arc<VariablePointers::VariablePointers>, mut clocked_states: Arc<VariablePointers::VariablePointers>, mut previous: Arc<VariablePointers::VariablePointers>, mut acc_discrete_states: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut acc_clocked_states: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut acc_previous: Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>, mut context: ArcStr) -> Result<(Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>, Arc<VariablePointers::VariablePointers>)> {
    let mut variables: Arc<VariablePointers::VariablePointers> = variables;
    let mut knowns: Arc<VariablePointers::VariablePointers> = knowns;
    let mut initials: Arc<VariablePointers::VariablePointers> = initials;
    let mut discretes: Arc<VariablePointers::VariablePointers> = discretes;
    let mut discrete_states: Arc<VariablePointers::VariablePointers> = discrete_states;
    let mut clocked_states: Arc<VariablePointers::VariablePointers> = clocked_states;
    let mut previous: Arc<VariablePointers::VariablePointers> = previous;
    variables = BVariable::VariablePointers::addList(acc_previous.clone(), variables.clone())?;
    knowns = BVariable::VariablePointers::addList(acc_previous.clone(), knowns.clone())?;
    initials = BVariable::VariablePointers::addList(acc_previous.clone(), initials.clone())?;
    previous = BVariable::VariablePointers::addList(acc_previous.clone(), previous.clone())?;
    discrete_states = BVariable::VariablePointers::addList(acc_discrete_states.clone(), discrete_states.clone())?;
    clocked_states = BVariable::VariablePointers::addList(acc_clocked_states.clone(), clocked_states.clone())?;
    discretes = BVariable::VariablePointers::removeList(acc_discrete_states.clone(), discretes.clone())?;
    discretes = BVariable::VariablePointers::removeList(acc_clocked_states.clone(), discretes.clone())?;
    discrete_states = BVariable::VariablePointers::removeList(acc_clocked_states.clone(), discrete_states.clone())?;
    if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? {
        if !(acc_discrete_states.clone().is_empty()) {
            metamodelica::print((StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[stateselection] Natural discrete states from ")); __mm_s.push_str(&*context.clone()); __mm_s.push_str(&*literal!(":")); ArcStr::from(__mm_s) }).clone())).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(acc_discrete_states.clone(), (std::sync::Arc::new(BVariable::pointerToString) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("\t")).clone(), (literal!("\n\t")).clone(), (literal!("\n")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        if !(acc_clocked_states.clone().is_empty()) {
            metamodelica::print((StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[stateselection] Natural clocked states from ")); __mm_s.push_str(&*context.clone()); __mm_s.push_str(&*literal!(":")); ArcStr::from(__mm_s) }).clone())).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(acc_clocked_states.clone(), (std::sync::Arc::new(BVariable::pointerToString) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("\t")).clone(), (literal!("\n\t")).clone(), (literal!("\n")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    if Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())? {
        if !(acc_previous.clone().is_empty()) {
            metamodelica::print((StringUtil::headline_4(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[discreteinfo] pre() and previous() variables from ")); __mm_s.push_str(&*context.clone()); __mm_s.push_str(&*literal!(":")); ArcStr::from(__mm_s) }).clone())).clone());
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*List::toString(acc_previous.clone(), (std::sync::Arc::new(BVariable::pointerToString) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<Arc<Variable::NFVariable>>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("\t")).clone(), (literal!("\n\t")).clone(), (literal!("\n")).clone(), true, 0)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
    }
    Ok((variables, knowns, initials, discretes, discrete_states, clocked_states, previous))
}

fn collectDiscreteStatesFromWhen(mut eqn: Arc<Equation::Equation>, mut acc_discrete_states: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut acc_previous: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut scalarized: bool) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let () = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::WHEN_EQUATION { .. } => {
            collectDiscreteStatesFromWhenBody(var_field!((*eqn).body, Equation::Equation::WHEN_EQUATION).clone(), acc_discrete_states.clone(), acc_previous.clone(), scalarized.clone())?;
            ()
        },
        Deref @ BEquation::Equation::FOR_EQUATION { .. } => {
            for mut b_eqn in &*var_field!((*eqn).body, Equation::Equation::FOR_EQUATION).clone() {
                let mut b_eqn = b_eqn.clone();
                collectDiscreteStatesFromWhen(b_eqn.clone(), acc_discrete_states.clone(), acc_previous.clone(), scalarized.clone())?;
            }
            ()
        },
        Deref @ BEquation::Equation::IF_EQUATION { .. } => {
            collectDiscreteStatesFromWhenInIf(var_field!((*eqn).body, Equation::Equation::IF_EQUATION).clone(), acc_discrete_states.clone(), acc_previous.clone(), scalarized.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

fn collectDiscreteStatesFromWhenBody(mut body: Arc<WhenEquationBody::WhenEquationBody>, mut acc_discrete_states: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut acc_previous: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut scalarized: bool) -> Result<()> {
    for mut body_stmt in &*body.when_stmts.clone() {
        let mut body_stmt = body_stmt.clone();
        let () = (::match_deref::match_deref! { match &(body_stmt.clone()) {
        Deref @ BEquation::WhenStatement::ASSIGN { lhs: Deref @ Expression::CREF { cref: state_cref, .. }, .. } => {
            let mut state_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            state_var = BVariable::getVarPointer(state_cref.clone(), metamodelica::sourceInfo!())?;
            BVariable::makeDiscreteStateVar(state_var.clone());
            getPreVar(state_cref.clone(), state_var.clone(), acc_previous.clone(), scalarized.clone())?;
            Pointer::update(acc_discrete_states.clone(), metamodelica::cons(state_var.clone(), Pointer::access(acc_discrete_states.clone())));
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(())
}

fn collectDiscreteStatesFromWhenInIf(mut body: Arc<IfEquationBody::IfEquationBody>, mut acc_discrete_states: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut acc_previous: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut scalarized: bool) -> Result<()> {
    for mut eqn in &*body.then_eqns.clone() {
        let mut eqn = eqn.clone();
        collectDiscreteStatesFromWhen(Pointer::access(eqn.clone()), acc_discrete_states.clone(), acc_previous.clone(), scalarized.clone())?;
    }
    if isSome(body.else_if.clone()) {
        collectDiscreteStatesFromWhenInIf(Util::getOption(body.else_if.clone())?, acc_discrete_states.clone(), acc_previous.clone(), scalarized.clone())?;
    }
    Ok(())
}

fn getPreVar(mut var_cref: Arc<ComponentRef::NFComponentRef>, mut var_ptr: Pointer::Pointer<Arc<Variable::NFVariable>>, mut acc_previous: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut scalarized: bool) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut pre_cref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let (mut pre, _): (Option<Pointer::Pointer<Arc<Variable::NFVariable>>>, ArcStr) = BVariable::getVarPre(var_ptr.clone());
    let mut pre_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
    if isSome(pre.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(pre.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        pre_var = __pa0.clone();
        pre_cref = BVariable::getVarName(pre_var.clone());
        pre_cref = ComponentRef::copySubscripts(var_cref.clone(), pre_cref.clone())?;
    } else {
        if !(scalarized.clone()) {
            (pre_cref, pre_var) = BVariable::makePreVar(ComponentRef::stripSubscriptsAll(var_cref.clone()))?;
            pre_cref = ComponentRef::copySubscripts(var_cref.clone(), pre_cref.clone())?;
        } else {
            (pre_cref, pre_var) = BVariable::makePreVar(var_cref.clone())?;
        }
        Pointer::update(acc_previous.clone(), metamodelica::cons(pre_var.clone(), Pointer::access(acc_previous.clone())));
    }
    Ok(pre_cref)
}

pub fn findDiscreteStatesFromWhenBody(mut body: Arc<WhenEquationBody::WhenEquationBody>, mut acc_discrete_states: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>, mut acc_previous: Pointer::Pointer<Arc<metamodelica::List<Pointer::Pointer<Arc<Variable::NFVariable>>>>>) -> Result<()> {
    for mut body_stmt in &*body.when_stmts.clone() {
        let mut body_stmt = body_stmt.clone();
        let () = (::match_deref::match_deref! { match &(body_stmt.clone()) {
        Deref @ BEquation::WhenStatement::ASSIGN { lhs: Deref @ Expression::CREF { cref: state_cref, .. }, .. } => {
            let mut state_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            let mut pre_var: Pointer::Pointer<Arc<Variable::NFVariable>>;
            state_var = BVariable::getVarPointer(state_cref.clone(), metamodelica::sourceInfo!())?;
            let () = (match (BVariable::getVarPre(state_var.clone())).0 {
        Some(mut pre_var) => {
            Pointer::update(acc_previous.clone(), metamodelica::cons(pre_var.clone(), Pointer::access(acc_previous.clone())));
            ()
        },
        _ => (),
    });
            Pointer::update(acc_discrete_states.clone(), metamodelica::cons(state_var.clone(), Pointer::access(acc_discrete_states.clone())));
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(())
}

pub fn stateOrder(mut eqn: Arc<Equation::Equation>, mut state_order: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<Equation::Equation>> {
    let mut eqn: Arc<Equation::Equation> = eqn;
    let mut lhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut rhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let () = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::SCALAR_EQUATION { rhs: __esc_rhs @ Deref @ Expression::CREF { .. }, lhs: __esc_lhs @ Deref @ Expression::CREF { .. }, .. } => {
            rhs = (*__esc_rhs).clone();
            lhs = (*__esc_lhs).clone();
            updateStateOrder(var_field!((*lhs).cref, Expression::NFExpression::CREF).clone(), var_field!((*rhs).cref, Expression::NFExpression::CREF).clone(), state_order.clone())?;
            ()
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { rhs: __esc_rhs @ Deref @ Expression::CREF { .. }, lhs: __esc_lhs @ Deref @ Expression::CREF { .. }, .. } => {
            lhs = (*__esc_lhs).clone();
            rhs = (*__esc_rhs).clone();
            updateStateOrder(var_field!((*lhs).cref, Expression::NFExpression::CREF).clone(), var_field!((*rhs).cref, Expression::NFExpression::CREF).clone(), state_order.clone())?;
            ()
        },
        Deref @ BEquation::Equation::FOR_EQUATION { .. } => {
            for mut b in &*var_field!((*eqn).body, Equation::Equation::FOR_EQUATION).clone() {
                let mut b = b.clone();
                stateOrder(b.clone(), state_order.clone())?;
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}

pub fn updateStateOrder(mut lhs: Arc<ComponentRef::NFComponentRef>, mut rhs: Arc<ComponentRef::NFComponentRef>, mut state_order: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>>>) -> Result<()> {
    let mut state: Pointer::Pointer<Arc<Variable::NFVariable>>;
    let () = (::match_deref::match_deref! { match &((BVariable::getVarKind(BVariable::getVarPointer(lhs.clone(), metamodelica::sourceInfo!())?), BVariable::getVarKind(BVariable::getVarPointer(rhs.clone(), metamodelica::sourceInfo!())?))) {
        (_, Deref @ VariableKind::STATE_DER { state, .. }) => {
            UnorderedMap::add(BVariable::getVarName(state.clone()), ComponentRef::stripSubscriptsAll(lhs.clone()), state_order.clone())?;
            ()
        },
        (Deref @ VariableKind::STATE_DER { state, .. }, _) => {
            UnorderedMap::add(BVariable::getVarName(state.clone()), ComponentRef::stripSubscriptsAll(rhs.clone()), state_order.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

