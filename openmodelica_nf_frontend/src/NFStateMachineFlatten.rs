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

use crate::NFAttributes as Attributes;
use crate::NFBackendExtension;
use crate::NFBinding as Binding;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFClockKind;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFEquation::ScalarizeMode;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::List;

// ============================================================
// Internal data types
// ============================================================
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Transition {
    pub from: i32,
    pub to: i32,
    pub condition: Arc<Expression::NFExpression>,
    pub immediate: bool,
    pub reset: bool,
    pub synchronize: bool,
    pub priority: i32,
}

impl metamodelica::gc::MMTrace for Transition {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.from, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.to, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.condition, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.immediate, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.reset, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.synchronize, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.priority, __mmv)?;
        Ok(())
    }
}
impl Default for Transition {
    fn default() -> Self {
        Self {
            from: Default::default(),
            to: Default::default(),
            condition: Default::default(),
            immediate: Default::default(),
            reset: Default::default(),
            synchronize: Default::default(),
            priority: Default::default(),
        }
    }
}

pub type TRANSITION = Transition;


#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FlatSmSemantics {
    /// Cref of the initial state (used as prefix for smOf vars)
    pub initStateRef: Arc<ComponentRef::NFComponentRef>,
    /// State crefs; index 1 = initial state
    pub smComps: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>,
    /// Transitions sorted by priority
    pub t: Arc<metamodelica::List<Transition>>,
    /// Conditions sorted by priority
    pub c: Arc<metamodelica::List<Arc<Expression::NFExpression>>>,
    /// SMS discrete variables
    pub vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>,
    /// SMS parameters/constants
    pub knowns: Arc<metamodelica::List<Arc<Variable::NFVariable>>>,
    /// SMS equations
    pub eqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>,
    /// Propagation variables
    pub pvars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>,
    /// Propagation equations
    pub peqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>,
    /// Enclosing state if hierarchical SM
    pub enclosingState: Option<Arc<ComponentRef::NFComponentRef>>,
}

impl metamodelica::gc::MMTrace for FlatSmSemantics {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.initStateRef, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.smComps, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.t, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.c, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.vars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.knowns, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.eqs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.pvars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.peqs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.enclosingState, __mmv)?;
        Ok(())
    }
}
impl Default for FlatSmSemantics {
    fn default() -> Self {
        Self {
            initStateRef: Default::default(),
            smComps: Default::default(),
            t: Default::default(),
            c: Default::default(),
            vars: Default::default(),
            knowns: Default::default(),
            eqs: Default::default(),
            pvars: Default::default(),
            peqs: Default::default(),
            enclosingState: Default::default(),
        }
    }
}

pub type FLAT_SM_SEMANTICS = FlatSmSemantics;


pub(crate) const SMS_PRE: &'static str = "smOf";

// ============================================================
// Public entry point
// ============================================================
pub(crate) fn flatten(mut flatModel: Arc<FlatModel::NFFlatModel>) -> Result<Arc<FlatModel::NFFlatModel>> {
    pub(crate) type OuterVarList = Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>;

    let mut flatModel: Arc<FlatModel::NFFlatModel> = flatModel;
    let mut initStates: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut smGroups: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>;
    let mut smEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    let mut otherEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    let mut resultEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    let mut smVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>;
    let mut resultVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>;
    let mut allStateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>;
    let mut stateToSem: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, FlatSmSemantics>>;
    let mut smGroupPairs: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>>;
    let mut smGroupsSorted: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)>>;
    let mut sem: FlatSmSemantics;
    let mut initState: Arc<ComponentRef::NFComponentRef>;
    let mut parentPrefix: Arc<ComponentRef::NFComponentRef>;
    let mut stateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut enclosingStateCrefOpt: Option<Arc<ComponentRef::NFComponentRef>>;
    let mut enclosingSmSemOpt: Option<FlatSmSemantics>;
    if !(List::any(flatModel.equations.clone(), (std::sync::Arc::new(isTransitionOrInitialState) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<bool> + 'static>))?) && !(List::any(flatModel.initialEquations.clone(), (std::sync::Arc::new(isTransitionOrInitialState) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<bool> + 'static>))?) {
        return Ok(flatModel.clone());
    }
    (initStates, smGroups) = groupStateMachines(flatModel.equations.clone(), flatModel.initialEquations.clone())?;
    if initStates.clone().is_empty() {
        return Ok(flatModel.clone());
    }
    allStateCrefs = List::flatten(smGroups.clone())?;
    otherEqs = List::filterOnFalse(flatModel.equations.clone(), (std::sync::Arc::new(isTransitionOrInitialState) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<bool> + 'static>))?;
    otherEqs = List::filterOnFalse(otherEqs, (std::sync::Arc::new({ let __pe_b1 = allStateCrefs; move |__pe_a0| isOuterStateEquation(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<bool> + 'static>))?;
    outerVarMap = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    smGroupPairs = List::zip(initStates, smGroups);
    smGroupsSorted = List::sort(smGroupPairs, (std::sync::Arc::new(fnptr!(smGroupDepthLt, (Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>), (Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>))) as std::sync::Arc<dyn ::std::ops::Fn((Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>), (Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)) -> Result<bool> + 'static>))?;
    stateToSem = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    smVars = metamodelica::nil();
    smEqs = metamodelica::nil();
    for mut smPair in &*smGroupsSorted {
        let mut smPair = smPair.clone();
        (initState, stateCrefs) = smPair.clone();
        parentPrefix = ComponentRef::rest(initState.clone())?;
        if ComponentRef::isEmpty(parentPrefix.clone()) {
            enclosingStateCrefOpt = None;
            enclosingSmSemOpt = None;
        } else {
            enclosingSmSemOpt = UnorderedMap::get(parentPrefix.clone(), stateToSem.clone())?;
            enclosingStateCrefOpt = if (isSome(enclosingSmSemOpt.clone())) {Some(parentPrefix.clone())} else {None};
        }
        (smEqs, smVars, sem) = flatSmToDataFlow(initState.clone(), stateCrefs.clone(), flatModel.equations.clone(), flatModel.variables.clone(), enclosingStateCrefOpt.clone(), enclosingSmSemOpt.clone(), smEqs.clone(), smVars.clone(), outerVarMap.clone())?;
        for mut sc in &*stateCrefs.clone() {
            let mut sc = sc.clone();
            UnorderedMap::addUnique(sc.clone(), sem.clone(), stateToSem.clone())?;
        }
    }
    for mut outerVarCref in &*UnorderedMap::keyList(outerVarMap.clone()) {
        let mut outerVarCref = outerVarCref.clone();
        (smEqs, smVars) = generateMergeEquation(outerVarCref.clone(), outerVarMap.clone(), flatModel.variables.clone(), smEqs.clone(), smVars.clone())?;
    }
    resultEqs = listAppend(({
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eq in (smEqs).into_iter().cloned() {
            let __x = subsActiveStateInEq(eq.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), ({
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eq in (otherEqs).into_iter().cloned() {
            let __x = subsActiveStateInEq(eq.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
    resultVars = listAppend(smVars, flatModel.variables.clone());
    assign_field!(
        flatModel.equations = resultEqs,
        flatModel.initialEquations = List::filterOnFalse(flatModel.initialEquations.clone(), (std::sync::Arc::new(isTransitionOrInitialState) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<bool> + 'static>))?,
        flatModel.variables = resultVars
    );
    execStat(literal!("NFStateMachineFlatten.flatten"))?;
    Ok(flatModel)
}

// ============================================================
// SM group detection
// ============================================================
fn groupStateMachines(mut equations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut initialEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<(Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>>)> {
    let mut initStates: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut smGroups: Arc<metamodelica::List<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>>> = metamodelica::nil();
    let mut allFroms: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut allTos: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut allInits: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut cr1: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut cr2: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut group: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    for mut eq in &*listAppend(equations, initialEquations) {
        let mut eq = eq.clone();
        let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: eqCall }, .. } => {
            let mut fname: ArcStr;
            fname = (Call::functionNameLast(eqCall.clone())?).clone();
            if stringEq((fname.clone()).clone(), (literal!("transition")).clone()) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::firstN(Call::arguments(eqCall.clone())?, 2)?) {
                    Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __pa0, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __pa1, .. }, tail: Deref @ metamodelica::List::Nil } } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                cr1 = __pa0.clone();
                cr2 = __pa1.clone();
                allFroms = metamodelica::cons(cr1.clone(), allFroms.clone());
                allTos = metamodelica::cons(cr2.clone(), allTos.clone());
            } else if stringEq((fname.clone()).clone(), (literal!("initialState")).clone()) {
                let __pa3 = ::match_deref::match_deref! { match &(List::firstN(Call::arguments(eqCall.clone())?, 1)?) {
                    Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __pa3, .. }, tail: Deref @ metamodelica::List::Nil } => __pa3.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                cr1 = __pa3.clone();
                allInits = metamodelica::cons(cr1.clone(), allInits.clone());
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    for mut initCref in &*allInits {
        let mut initCref = initCref.clone();
        group = collectReachableStates(initCref.clone(), allFroms.clone(), allTos.clone())?;
        initStates = metamodelica::cons(initCref.clone(), initStates.clone());
        smGroups = metamodelica::cons(group.clone(), smGroups.clone());
    }
    initStates = initStates.reverse();
    smGroups = smGroups.reverse();
    Ok((initStates, smGroups))
}

fn collectReachableStates(mut initCref: Arc<ComponentRef::NFComponentRef>, mut froms: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut tos: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>> {
    let mut states: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut queue: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = list![initCref.clone()];
    let mut visited: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut cur: Arc<ComponentRef::NFComponentRef>;
    states = metamodelica::nil();
    while !(queue.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(queue.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cur = __pa0.clone();
        queue = __pa1.clone();
        if !(List::isMemberOnTrue(cur.clone(), visited.clone(), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?) {
            visited = metamodelica::cons(cur.clone(), visited.clone());
            states = metamodelica::cons(cur.clone(), states.clone());
            for mut i in 1..=(froms.clone().len() as i32) {
                if ComponentRef::isEqual((froms.clone()).get(i.clone())?, cur.clone())? {
                    queue = metamodelica::cons((tos.clone()).get(i.clone())?, queue.clone());
                }
                if ComponentRef::isEqual((tos.clone()).get(i.clone())?, cur.clone())? {
                    queue = metamodelica::cons((froms.clone()).get(i.clone())?, queue.clone());
                }
            }
        }
    }
    states = List::sort(states, (std::sync::Arc::new({ let __pe_b2 = initCref; move |__pe_a0, __pe_a1| statePriorityGt(__pe_a0, __pe_a1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
    Ok(states)
}

fn statePriorityGt(mut cr1: Arc<ComponentRef::NFComponentRef>, mut cr2: Arc<ComponentRef::NFComponentRef>, mut initCref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut gt: bool;
    if ComponentRef::isEqual(cr2.clone(), initCref.clone())? {
        gt = true;
    } else if ComponentRef::isEqual(cr1.clone(), initCref)? {
        gt = false;
    } else {
        gt = ComponentRef::toString(cr1)? > ComponentRef::toString(cr2)?;
    }
    Ok(gt)
}

// ============================================================
// Flat SM to data-flow transformation
// ============================================================
fn smGroupDepthLt(mut g1: (Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>), mut g2: (Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>)) -> bool {
    let mut lt: bool;
    let mut c1: Arc<ComponentRef::NFComponentRef>;
    let mut c2: Arc<ComponentRef::NFComponentRef>;
    (c1, _) = g1;
    (c2, _) = g2;
    lt = ComponentRef::depth(c1) < ComponentRef::depth(c2);
    lt
}

fn flatSmToDataFlow(mut initStateCref: Arc<ComponentRef::NFComponentRef>, mut stateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut allEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut allVariables: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut enclosingStateCrefOpt: Option<Arc<ComponentRef::NFComponentRef>>, mut enclosingSmSemOpt: Option<FlatSmSemantics>, mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>, FlatSmSemantics)> {
    let mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = accEqs;
    let mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = accVars;
    let mut outSem: FlatSmSemantics;
    let mut transitionEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    let mut initialStateEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    let mut sem: FlatSmSemantics;
    let mut semWithProp: FlatSmSemantics;
    let mut semFinal: FlatSmSemantics;
    let mut parentPrefix: Arc<ComponentRef::NFComponentRef>;
    let mut varCrefStrings: Arc<metamodelica::List<ArcStr>>;
    transitionEqs = List::filterOnTrue(allEquations.clone(), (std::sync::Arc::new({ let __pe_b1 = stateCrefs.clone(); move |__pe_a0| isTransitionForGroup(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<bool> + 'static>))?;
    initialStateEqs = List::filterOnTrue(allEquations.clone(), (std::sync::Arc::new({ let __pe_b1 = initStateCref.clone(); move |__pe_a0| isInitialStateForGroup(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<bool> + 'static>))?;
    sem = basicFlatSmSemantics(initStateCref, stateCrefs.clone(), transitionEqs)?;
    semWithProp = addPropagationEquations(sem, enclosingStateCrefOpt.clone(), enclosingSmSemOpt)?;
    semFinal = elabXInStateOps(semWithProp, enclosingStateCrefOpt)?;
    parentPrefix = ComponentRef::rest(listHead(stateCrefs.clone())?)?;
    if !(ComponentRef::isEmpty(parentPrefix.clone())) {
        varCrefStrings = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut v in (allVariables.clone()).into_iter().cloned() {
            let __x = ComponentRef::toString(v.name.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        semFinal.eqs = List::map(semFinal.eqs.clone(), (std::sync::Arc::new({ let __pe_b1 = (std::sync::Arc::new({ let __pe_b1 = parentPrefix; let __pe_b2 = varCrefStrings; move |__pe_a0| qualifyOuterVarExpr(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>); move |__pe_a0| Equation::mapExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<Arc<Equation::NFEquation>> + 'static>))?;
    }
    accVars = List::flatten(list![accVars, semFinal.vars.clone(), semFinal.knowns.clone(), semFinal.pvars.clone()])?;
    accEqs = List::flatten(list![accEqs, semFinal.eqs.clone(), semFinal.peqs.clone()])?;
    for mut stateCref in &*stateCrefs {
        let mut stateCref = stateCref.clone();
        (accEqs, accVars) = smCompToDataFlow(stateCref.clone(), semFinal.clone(), allEquations.clone(), allVariables.clone(), accEqs.clone(), accVars.clone(), outerVarMap.clone())?;
    }
    outSem = semFinal;
    Ok((accEqs, accVars, outSem))
}

fn qualifyOuterVarExpr(mut e: Arc<Expression::NFExpression>, mut parentPrefix: Arc<ComponentRef::NFComponentRef>, mut varCrefStrings: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<Expression::NFExpression>> {
    let mut e: Arc<Expression::NFExpression> = e;
    e = Expression::map(e, (std::sync::Arc::new({ let __pe_b1 = parentPrefix; let __pe_b2 = varCrefStrings; move |__pe_a0| qualifyOuterVarCref(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(e)
}

fn qualifyOuterVarCref(mut e: Arc<Expression::NFExpression>, mut parentPrefix: Arc<ComponentRef::NFComponentRef>, mut varCrefStrings: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<Expression::NFExpression>> {
    let mut e: Arc<Expression::NFExpression> = e;
    let mut qualCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let () = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Expression::CREF { .. } if (ComponentRef::isSimple(var_field!((*e).cref, Expression::NFExpression::CREF).clone())) => {
            qualCref = ComponentRef::append(var_field!((*e).cref, Expression::NFExpression::CREF).clone(), parentPrefix)?;
            if listMember((ComponentRef::toString(qualCref.clone())?).clone(), varCrefStrings) {
                e = Arc::new(Expression::NFExpression::CREF { ty: var_field!((*e).ty, Expression::NFExpression::CREF).clone(), cref: qualCref });
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(e)
}

// ============================================================
// State machine component to data-flow
// ============================================================
fn smCompToDataFlow(mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut allEquations: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut allVariables: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = accEqs;
    let mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = accVars;
    let mut stateEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    let mut stateVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>;
    let mut crToStart: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>;
    let mut transformedEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    let mut extraVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>;
    stateEqs = List::filterOnTrue(allEquations, (std::sync::Arc::new({ let __pe_b1 = stateCref.clone(); move |__pe_a0| isEquationOfState(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<bool> + 'static>))?;
    stateVars = List::filterOnTrue(allVariables.clone(), (std::sync::Arc::new({ let __pe_b1 = stateCref.clone(); move |__pe_a0| isVariableOfState(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Variable::NFVariable>) -> Result<bool> + 'static>))?;
    crToStart = UnorderedMap::new((std::sync::Arc::new(ComponentRef::hash) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(ComponentRef::isEqual) as std::sync::Arc<dyn ::std::ops::Fn(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
    for mut v in &*stateVars {
        let mut v = v.clone();
        if List::any(stateEqs.clone(), (std::sync::Arc::new({ let __pe_b1 = v.name.clone(); move |__pe_a0| equationHasPrevious(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<bool> + 'static>))? {
            UnorderedMap::addUnique(v.name.clone(), getStartValue(v.clone())?, crToStart.clone())?;
        }
    }
    transformedEqs = metamodelica::nil();
    extraVars = metamodelica::nil();
    for mut eq in &*stateEqs {
        let mut eq = eq.clone();
        (transformedEqs, extraVars) = addStateActivationAndReset(eq.clone(), stateCref.clone(), sem.clone(), crToStart.clone(), transformedEqs.clone(), extraVars.clone(), outerVarMap.clone())?;
    }
    accEqs = listAppend(transformedEqs.reverse(), accEqs);
    accVars = listAppend(extraVars.reverse(), accVars);
    addHierarchicalPassThroughs(stateCref, sem, allVariables, outerVarMap)?;
    Ok((accEqs, accVars))
}

fn addHierarchicalPassThroughs(mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut allVariables: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>) -> Result<()> {
    let mut stateStr: ArcStr;
    let mut leafName: ArcStr;
    let mut activeRef: Arc<ComponentRef::NFComponentRef>;
    let mut topVarCref: Arc<ComponentRef::NFComponentRef>;
    let mut topVar: Arc<Variable::NFVariable>;
    stateStr = (ComponentRef::toString(stateCref.clone())?).clone();
    activeRef = qCref((literal!("active")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), stateCref)?;
    for mut v in &*allVariables.clone() {
        let mut v = v.clone();
        if !(ComponentRef::isSimple(v.name.clone())) && stringEqual((ComponentRef::toString(ComponentRef::rest(v.name.clone())?)?).clone(), (stateStr.clone()).clone()) {
            leafName = (ComponentRef::firstName(v.name.clone(), false)?).clone();
            if '__try0: {
                topVar = unwrap_break_err!(List::find(allVariables.clone(), (std::sync::Arc::new({ let __pe_b1 = (leafName.clone()).clone(); move |__pe_a0| isSimpleVarNamed(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Variable::NFVariable>) -> Result<bool> + 'static>)), '__try0);
                topVarCref = topVar.name.clone();
                if !(unwrap_break_err!(UnorderedMap::contains(topVarCref.clone(), outerVarMap.clone()), '__try0)) {
                    unwrap_break_err!(UnorderedMap::add(topVarCref.clone(), list![(activeRef.clone(), v.name.clone())], outerVarMap.clone()), '__try0);
                }
                Ok::<(), anyhow::Error>(())
            }.is_err() {
            }
        }
    }
    Ok(())
}

fn isSimpleVarNamed(mut v: Arc<Variable::NFVariable>, mut name: ArcStr) -> Result<bool> {
    let mut res: bool;
    res = ComponentRef::isSimple(v.name.clone()) && stringEqual((ComponentRef::firstName(v.name.clone(), false)?).clone(), (name).clone());
    Ok(res)
}

// ============================================================
// addStateActivationAndReset
// ============================================================
fn addStateActivationAndReset(mut inEq: Arc<Equation::NFEquation>, mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut crToStart: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = accEqs;
    let mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = accVars;
    let () = (::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ Equation::EQUALITY { .. } => {
            (accEqs, accVars) = addStateActivationAndReset1(inEq, stateCref, sem, crToStart, accEqs, accVars, outerVarMap)?;
            ()
        },
        Deref @ Equation::WHEN { .. } => {
            (accEqs, accVars) = transformWhenBranchesAndAccumulate(inEq, stateCref, sem, crToStart, outerVarMap, accEqs, accVars)?;
            ()
        },
        _ => {
            accEqs = metamodelica::cons(inEq, accEqs);
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((accEqs, accVars))
}

fn transformWhenBranchesAndAccumulate(mut whenEq: Arc<Equation::NFEquation>, mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut crToStart: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>, mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = accEqs;
    let mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = accVars;
    let mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>;
    let mut firstBranch: Arc<Equation::Branch::Branch>;
    let mut branchCond: Arc<Expression::NFExpression>;
    let mut outEq: Arc<Equation::NFEquation>;
    let mut extraVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>;
    let mut innerEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>;
    let mut innerVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>;
    let __pa0 = ::match_deref::match_deref! { match &(whenEq.clone()) {
        Deref @ Equation::WHEN { branches: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    branches = __pa0.clone();
    firstBranch = listHead(branches)?;
    let __pa1 = ::match_deref::match_deref! { match &(firstBranch) {
        Deref @ Equation::Branch::BRANCH { condition: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    branchCond = __pa1.clone();
    if Type::isClock(Expression::typeOf(branchCond))? {
        (innerEqs, innerVars) = transformWhenInnerAsPlain(whenEq, stateCref, sem, crToStart, outerVarMap)?;
        accEqs = listAppend(innerEqs, accEqs);
        accVars = listAppend(innerVars, accVars);
    } else {
        (outEq, extraVars) = transformWhenBranches(whenEq, stateCref, sem, crToStart, outerVarMap)?;
        accEqs = metamodelica::cons(outEq, accEqs);
        accVars = listAppend(extraVars, accVars);
    }
    Ok((accEqs, accVars))
}

fn transformWhenInnerAsPlain(mut whenEq: Arc<Equation::NFEquation>, mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut crToStart: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut outEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut outVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>;
    let mut branchBody: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut transformedBody: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut branchVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(whenEq) {
        Deref @ Equation::WHEN { branches: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    branches = __pa0.clone();
    for mut branch in &*branches {
        let mut branch = branch.clone();
        let () = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Equation::Branch::BRANCH { body: __esc_branchBody, .. } => {
            branchBody = (*__esc_branchBody).clone();
            transformedBody = metamodelica::nil();
            branchVars = metamodelica::nil();
            for mut eq in &*branchBody.clone() {
                let mut eq = eq.clone();
                (transformedBody, branchVars) = addStateActivationAndReset(eq.clone(), stateCref.clone(), sem.clone(), crToStart.clone(), transformedBody.clone(), branchVars.clone(), outerVarMap.clone())?;
            }
            outEqs = listAppend(transformedBody.clone().reverse(), outEqs.clone());
            outVars = listAppend(branchVars.clone(), outVars.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok((outEqs, outVars))
}

fn transformWhenBranches(mut whenEq: Arc<Equation::NFEquation>, mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut crToStart: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>) -> Result<(Arc<Equation::NFEquation>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut outEq: Arc<Equation::NFEquation>;
    let mut extraVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut branches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>;
    let mut newBranches: Arc<metamodelica::List<Arc<Equation::Branch::Branch>>>;
    let mut transformedBody: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut branchVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut whenScope: Arc<InstNode::InstNode>;
    let mut whenSource: Arc<DAE::ElementSource>;
    let mut branchCond: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    let mut branchCondVar: Variability = Variability::CONSTANT;
    let mut branchBody: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(whenEq) {
        Deref @ Equation::WHEN { branches: __pa0, scope: __pa1, source: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    branches = __pa0.clone();
    whenScope = __pa1.clone();
    whenSource = __pa2.clone();
    newBranches = metamodelica::nil();
    for mut branch in &*branches {
        let mut branch = branch.clone();
        branch = (::match_deref::match_deref! { match &(branch.clone()) {
        Deref @ Equation::Branch::BRANCH { condition: __esc_branchCond, conditionVar: __esc_branchCondVar, body: __esc_branchBody } => {
            branchCond = (*__esc_branchCond).clone();
            branchCondVar = (*__esc_branchCondVar).clone();
            branchBody = (*__esc_branchBody).clone();
            transformedBody = metamodelica::nil();
            branchVars = metamodelica::nil();
            for mut eq in &*branchBody.clone() {
                let mut eq = eq.clone();
                (transformedBody, branchVars) = addStateActivationAndReset(eq.clone(), stateCref.clone(), sem.clone(), crToStart.clone(), transformedBody.clone(), branchVars.clone(), outerVarMap.clone())?;
            }
            extraVars = listAppend(branchVars.clone(), extraVars.clone());
            Arc::new(Equation::Branch::Branch::BRANCH { condition: branchCond.clone(), conditionVar: branchCondVar.clone(), body: transformedBody.clone().reverse() })
        },
        _ => branch.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        newBranches = metamodelica::cons(branch.clone(), newBranches.clone());
    }
    outEq = Arc::new(Equation::NFEquation::WHEN { branches: newBranches.reverse(), scope: whenScope, source: whenSource });
    Ok((outEq, extraVars))
}

fn addStateActivationAndReset1(mut inEq: Arc<Equation::NFEquation>, mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut crToStart: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>, mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = accEqs;
    let mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = accVars;
    let mut lhs: Arc<Expression::NFExpression>;
    let mut rhs: Arc<Expression::NFExpression>;
    let mut lhsCref: Arc<ComponentRef::NFComponentRef>;
    let mut perStateVarCref: Arc<ComponentRef::NFComponentRef>;
    let mut stateActiveCref: Arc<ComponentRef::NFComponentRef>;
    let mut lhsTy: Arc<Type::NFType>;
    let mut eqScope: Arc<InstNode::InstNode>;
    let mut eqSource: Arc<DAE::ElementSource>;
    let mut stateVarCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>;
    let mut hasStateVarOnLHS: bool;
    let mut isOuterOutput: bool;
    let mut newRhs: Arc<Expression::NFExpression>;
    let mut perStateVarExp: Arc<Expression::NFExpression>;
    let mut eq1: Arc<Equation::NFEquation>;
    let mut eq2: Arc<Equation::NFEquation>;
    let mut perStateVar: Arc<Variable::NFVariable>;
    let mut prevList: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ Equation::EQUALITY { lhs: __pa0, rhs: __pa1, ty: __pa2, scope: __pa3, source: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    lhs = __pa0.clone();
    rhs = __pa1.clone();
    lhsTy = __pa2.clone();
    eqScope = __pa3.clone();
    eqSource = __pa4.clone();
    stateVarCrefs = UnorderedMap::keyList(crToStart.clone());
    match '__try5: {
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(lhs.clone()) {
            Deref @ Expression::CREF { ty: __pa6, cref: __pa7 } => (__pa6.clone(), __pa7.clone()),
            _ => break '__try5 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        lhsTy = __pa6.clone();
        lhsCref = __pa7.clone();
        (newRhs, _) = unwrap_break_err!(Expression::mapFold(rhs.clone(), (std::sync::Arc::new({ let __pe_b1 = stateVarCrefs.clone(); move |__pe_a0, __pe_a2| Ok(subsPreviousCrefs(__pe_a0, __pe_b1.clone(), __pe_a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, bool) -> Result<(Arc<Expression::NFExpression>, bool)> + 'static>), false), '__try5);
        eq1 = Arc::new(Equation::NFEquation::EQUALITY { lhs: lhs.clone(), rhs: newRhs.clone(), ty: lhsTy.clone(), scope: eqScope.clone(), source: eqSource.clone(), scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone() });
        isOuterOutput = !(unwrap_break_err!(crefHasPrefix(stateCref.clone(), lhsCref.clone()), '__try5)) && stringEqual((unwrap_break_err!(InstNode::name(eqScope.clone()), '__try5)).clone(), (unwrap_break_err!(ComponentRef::firstName(stateCref.clone(), false), '__try5)).clone());
        if isOuterOutput {
            perStateVarCref = ComponentRef::prefixCref(Arc::new(InstNode::InstNode::NAME_NODE { name: (unwrap_break_err!(ComponentRef::firstName(lhsCref.clone(), false), '__try5)).clone() }), lhsTy.clone(), metamodelica::nil(), stateCref.clone());
            perStateVar = makeVarWithStart(perStateVarCref.clone(), lhsTy.clone(), Variability::DISCRETE.clone(), getDefaultStart(lhsTy.clone()));
            perStateVarExp = makeCrefExp(perStateVarCref.clone(), lhsTy.clone());
            eq1 = Arc::new(Equation::NFEquation::EQUALITY { lhs: perStateVarExp.clone(), rhs: newRhs.clone(), ty: lhsTy.clone(), scope: eqScope.clone(), source: eqSource.clone(), scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone() });
            eq1 = unwrap_break_err!(wrapInStateActivationConditional(eq1.clone(), stateCref.clone(), false), '__try5);
            accEqs = metamodelica::cons(eq1.clone(), accEqs.clone());
            accVars = metamodelica::cons(perStateVar.clone(), accVars.clone());
            stateActiveCref = unwrap_break_err!(qCref((literal!("active")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), stateCref.clone()), '__try5);
            prevList = unwrap_break_err!(UnorderedMap::getOrDefault(lhsCref.clone(), outerVarMap.clone(), metamodelica::nil()), '__try5);
            unwrap_break_err!(UnorderedMap::add(lhsCref.clone(), metamodelica::cons((stateActiveCref.clone(), perStateVarCref.clone()), prevList.clone()), outerVarMap.clone()), '__try5);
        } else {
            hasStateVarOnLHS = false;
            for mut svc in &*stateVarCrefs.clone() {
                let mut svc = svc.clone();
                hasStateVarOnLHS = unwrap_break_err!(ComponentRef::isEqual(svc.clone(), lhsCref.clone()), '__try5);
                if hasStateVarOnLHS {
                    break;
                }
            }
            if hasStateVarOnLHS {
                eq1 = unwrap_break_err!(wrapInStateActivationConditional(eq1.clone(), stateCref.clone(), true), '__try5);
                eq2 = unwrap_break_err!(createResetEquation(lhsCref.clone(), lhsTy.clone(), stateCref.clone(), sem.clone(), crToStart.clone()), '__try5);
                accEqs = metamodelica::cons(eq1.clone(), metamodelica::cons(eq2.clone(), accEqs.clone()));
                accVars = metamodelica::cons(makeVar(ComponentRef::prefixCref(Arc::new(InstNode::InstNode::NAME_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::firstName(lhsCref.clone(), false), '__try5)); __mm_s.push_str(&*literal!("_previous")); ArcStr::from(__mm_s) }).clone() }), lhsTy.clone(), metamodelica::nil(), unwrap_break_err!(ComponentRef::rest(lhsCref.clone()), '__try5)), lhsTy.clone(), Variability::CONTINUOUS.clone()), accVars.clone());
            } else {
                accEqs = metamodelica::cons(unwrap_break_err!(wrapInStateActivationConditional(eq1.clone(), stateCref.clone(), false), '__try5), accEqs.clone());
            }
        }
        Ok::<_, anyhow::Error>((accEqs.clone(),))
    } {
        Ok((__try5_o0,)) => {
            accEqs = __try5_o0;
        }
        Err(_) => {
            accEqs = metamodelica::cons(inEq.clone(), accEqs.clone());
        }
    }
    Ok((accEqs, accVars))
}

fn equationHasPrevious(mut eq: Arc<Equation::NFEquation>, mut varCref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut found: bool;
    found = Equation::containsExp(eq, (std::sync::Arc::new({ let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static> = (std::sync::Arc::new({ let __pe_b1 = varCref; move |__pe_a0| isPreviousOfCref(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>); move |__pe_a0| Expression::contains(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<bool> + 'static>))?;
    Ok(found)
}

fn isPreviousOfCref(mut e: Arc<Expression::NFExpression>, mut varCref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut res: bool = false;
    let mut expCall: Arc<Call::NFCall> = Arc::new(<Call::NFCall as ::std::default::Default>::default());
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut argCref: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    res = (::match_deref::match_deref! { match &(e) {
        Deref @ Expression::CALL { call: expCall } if (stringEq((Call::functionNameLast(expCall.clone())?).clone(), (literal!("previous")).clone())) => {
            args = Call::arguments(expCall.clone())?;
            res = false;
            if (args.clone().len() as i32) == 1 {
                res = (::match_deref::match_deref! { match &(listHead(args)?) {
        Deref @ Expression::CREF { cref: __esc_argCref, .. } => {
            argCref = (*__esc_argCref).clone();
            ComponentRef::isEqual(argCref.clone(), varCref)?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            res
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn getDefaultStart(mut ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(ty) {
        Deref @ Type::INTEGER => Arc::new(Expression::NFExpression::INTEGER { value: 0 }),
        Deref @ Type::REAL => Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }),
        Deref @ Type::BOOLEAN => Arc::new(Expression::NFExpression::BOOLEAN { value: false }),
        Deref @ Type::STRING => Arc::new(Expression::NFExpression::STRING { value: (literal!("")).clone() }),
        _ => Arc::new(Expression::NFExpression::INTEGER { value: 0 }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

// ============================================================
// basicFlatSmSemantics
// ============================================================
fn basicFlatSmSemantics(mut initStateCref: Arc<ComponentRef::NFComponentRef>, mut stateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut transitionEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<FlatSmSemantics> {
    let mut sem: FlatSmSemantics;
    let mut preRef: Arc<ComponentRef::NFComponentRef>;
    let mut nStates: i32;
    let mut nTransitions: i32;
    let mut i: i32;
    let mut t: Arc<metamodelica::List<Transition>>;
    let mut cExps: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut vars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut knowns: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut eqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut nStatesRef: Arc<ComponentRef::NFComponentRef>;
    let mut activeRef: Arc<ComponentRef::NFComponentRef>;
    let mut resetRef: Arc<ComponentRef::NFComponentRef>;
    let mut selectedStateRef: Arc<ComponentRef::NFComponentRef>;
    let mut selectedResetRef: Arc<ComponentRef::NFComponentRef>;
    let mut firedRef: Arc<ComponentRef::NFComponentRef>;
    let mut activeStateRef: Arc<ComponentRef::NFComponentRef>;
    let mut activeResetRef: Arc<ComponentRef::NFComponentRef>;
    let mut nextStateRef: Arc<ComponentRef::NFComponentRef>;
    let mut nextResetRef: Arc<ComponentRef::NFComponentRef>;
    let mut stateMachineInFinalStateRef: Arc<ComponentRef::NFComponentRef>;
    let mut tArrayBool: Arc<Type::NFType>;
    let mut tArrayInt: Arc<Type::NFType>;
    let mut activeResetStatesRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut nextResetStatesRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut finalStatesRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut cRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut cImmediateRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut tTArrayBool: Arc<Type::NFType>;
    let mut tTArrayInt: Arc<Type::NFType>;
    let mut tFromRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut tToRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut tImmediateRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut tResetRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut tSynchronizeRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut tPriorityRefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>> = metamodelica::nil();
    let mut rhs: Arc<Expression::NFExpression>;
    let mut expCond: Arc<Expression::NFExpression>;
    let mut expThen: Arc<Expression::NFExpression>;
    let mut expElse: Arc<Expression::NFExpression>;
    let mut exp1: Arc<Expression::NFExpression>;
    let mut exp2: Arc<Expression::NFExpression>;
    let mut expIf: Arc<Expression::NFExpression>;
    let mut expLst: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut immediateVal: bool;
    let mut tDim: Arc<Dimension::NFDimension>;
    let mut nStatesDim: Arc<Dimension::NFDimension>;
    preRef = makeSMSPrefix(initStateCref.clone())?;
    (t, cExps) = createTandC(stateCrefs.clone(), transitionEqs)?;
    nStates = (stateCrefs.clone().len() as i32);
    nTransitions = (t.clone().len() as i32);
    tDim = Arc::new(Dimension::NFDimension::INTEGER { size: nTransitions, var: Variability::STRUCTURAL_PARAMETER.clone() });
    nStatesDim = Arc::new(Dimension::NFDimension::INTEGER { size: nStates, var: Variability::STRUCTURAL_PARAMETER.clone() });
    tTArrayBool = Arc::new(Type::NFType::ARRAY { elementType: crate::NFType::interned_BOOLEAN(), dimensions: list![tDim.clone()] });
    tTArrayInt = Arc::new(Type::NFType::ARRAY { elementType: crate::NFType::interned_INTEGER(), dimensions: list![tDim] });
    tArrayBool = Arc::new(Type::NFType::ARRAY { elementType: crate::NFType::interned_BOOLEAN(), dimensions: list![nStatesDim.clone()] });
    tArrayInt = Arc::new(Type::NFType::ARRAY { elementType: crate::NFType::interned_INTEGER(), dimensions: list![nStatesDim] });
    nStatesRef = qCref((literal!("nState")).clone(), crate::NFType::interned_INTEGER(), metamodelica::nil(), preRef.clone())?;
    knowns = metamodelica::cons(makeVarWithBinding(nStatesRef, crate::NFType::interned_INTEGER(), Variability::STRUCTURAL_PARAMETER.clone(), Arc::new(Expression::NFExpression::INTEGER { value: nStates })), knowns);
    i = 0;
    for mut tr in &*t.clone() {
        let mut tr = tr.clone();
        i = i + 1;
        tFromRefs = metamodelica::cons(qCref((literal!("tFrom")).clone(), tTArrayInt.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i }) })], preRef.clone())?, tFromRefs.clone());
        knowns = metamodelica::cons(makeVarWithBinding(listHead(tFromRefs.clone())?, crate::NFType::interned_INTEGER(), Variability::STRUCTURAL_PARAMETER.clone(), Arc::new(Expression::NFExpression::INTEGER { value: tr.from.clone() })), knowns.clone());
        tToRefs = metamodelica::cons(qCref((literal!("tTo")).clone(), tTArrayInt.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i }) })], preRef.clone())?, tToRefs.clone());
        knowns = metamodelica::cons(makeVarWithBinding(listHead(tToRefs.clone())?, crate::NFType::interned_INTEGER(), Variability::STRUCTURAL_PARAMETER.clone(), Arc::new(Expression::NFExpression::INTEGER { value: tr.to.clone() })), knowns.clone());
        tImmediateRefs = metamodelica::cons(qCref((literal!("tImmediate")).clone(), tTArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i }) })], preRef.clone())?, tImmediateRefs.clone());
        knowns = metamodelica::cons(makeVarWithBinding(listHead(tImmediateRefs.clone())?, crate::NFType::interned_BOOLEAN(), Variability::STRUCTURAL_PARAMETER.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: tr.immediate.clone() })), knowns.clone());
        tResetRefs = metamodelica::cons(qCref((literal!("tReset")).clone(), tTArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i }) })], preRef.clone())?, tResetRefs.clone());
        knowns = metamodelica::cons(makeVarWithBinding(listHead(tResetRefs.clone())?, crate::NFType::interned_BOOLEAN(), Variability::STRUCTURAL_PARAMETER.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: tr.reset.clone() })), knowns.clone());
        tSynchronizeRefs = metamodelica::cons(qCref((literal!("tSynchronize")).clone(), tTArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i }) })], preRef.clone())?, tSynchronizeRefs.clone());
        knowns = metamodelica::cons(makeVarWithBinding(listHead(tSynchronizeRefs.clone())?, crate::NFType::interned_BOOLEAN(), Variability::STRUCTURAL_PARAMETER.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: tr.synchronize.clone() })), knowns.clone());
        tPriorityRefs = metamodelica::cons(qCref((literal!("tPriority")).clone(), tTArrayInt.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i }) })], preRef.clone())?, tPriorityRefs.clone());
        knowns = metamodelica::cons(makeVarWithBinding(listHead(tPriorityRefs.clone())?, crate::NFType::interned_INTEGER(), Variability::STRUCTURAL_PARAMETER.clone(), Arc::new(Expression::NFExpression::INTEGER { value: tr.priority.clone() })), knowns.clone());
    }
    tFromRefs = tFromRefs.reverse();
    tToRefs = tToRefs.reverse();
    tImmediateRefs = tImmediateRefs.reverse();
    tResetRefs = tResetRefs.reverse();
    tSynchronizeRefs = tSynchronizeRefs.reverse();
    tPriorityRefs = tPriorityRefs.reverse();
    i = 0;
    for mut cExp in &*cExps.clone() {
        let mut cExp = cExp.clone();
        i = i + 1;
        cImmediateRefs = metamodelica::cons(qCref((literal!("cImmediate")).clone(), tTArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i }) })], preRef.clone())?, cImmediateRefs.clone());
        cRefs = metamodelica::cons(qCref((literal!("c")).clone(), tTArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i }) })], preRef.clone())?, cRefs.clone());
        vars = metamodelica::cons(makeVarWithStart(listHead(cImmediateRefs.clone())?, crate::NFType::interned_BOOLEAN(), Variability::DISCRETE.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: false })), vars.clone());
        vars = metamodelica::cons(makeVar(listHead(cRefs.clone())?, crate::NFType::interned_BOOLEAN(), Variability::DISCRETE.clone()), vars.clone());
    }
    cImmediateRefs = cImmediateRefs.reverse();
    cRefs = cRefs.reverse();
    activeRef = qCref((literal!("active")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), preRef.clone())?;
    vars = metamodelica::cons(makeVar(activeRef.clone(), crate::NFType::interned_BOOLEAN(), Variability::DISCRETE.clone()), vars);
    resetRef = qCref((literal!("reset")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), preRef.clone())?;
    vars = metamodelica::cons(makeVar(resetRef.clone(), crate::NFType::interned_BOOLEAN(), Variability::DISCRETE.clone()), vars);
    selectedStateRef = qCref((literal!("selectedState")).clone(), crate::NFType::interned_INTEGER(), metamodelica::nil(), preRef.clone())?;
    vars = metamodelica::cons(makeVar(selectedStateRef.clone(), crate::NFType::interned_INTEGER(), Variability::DISCRETE.clone()), vars);
    selectedResetRef = qCref((literal!("selectedReset")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), preRef.clone())?;
    vars = metamodelica::cons(makeVar(selectedResetRef.clone(), crate::NFType::interned_BOOLEAN(), Variability::DISCRETE.clone()), vars);
    firedRef = qCref((literal!("fired")).clone(), crate::NFType::interned_INTEGER(), metamodelica::nil(), preRef.clone())?;
    vars = metamodelica::cons(makeVar(firedRef.clone(), crate::NFType::interned_INTEGER(), Variability::DISCRETE.clone()), vars);
    activeStateRef = qCref((literal!("activeState")).clone(), crate::NFType::interned_INTEGER(), metamodelica::nil(), preRef.clone())?;
    vars = metamodelica::cons(makeVar(activeStateRef.clone(), crate::NFType::interned_INTEGER(), Variability::DISCRETE.clone()), vars);
    activeResetRef = qCref((literal!("activeReset")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), preRef.clone())?;
    vars = metamodelica::cons(makeVar(activeResetRef.clone(), crate::NFType::interned_BOOLEAN(), Variability::DISCRETE.clone()), vars);
    nextStateRef = qCref((literal!("nextState")).clone(), crate::NFType::interned_INTEGER(), metamodelica::nil(), preRef.clone())?;
    vars = metamodelica::cons(makeVarWithStart(nextStateRef.clone(), crate::NFType::interned_INTEGER(), Variability::DISCRETE.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 0 })), vars);
    nextResetRef = qCref((literal!("nextReset")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), preRef.clone())?;
    vars = metamodelica::cons(makeVarWithStart(nextResetRef.clone(), crate::NFType::interned_BOOLEAN(), Variability::DISCRETE.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: false })), vars);
    for mut j in 1..=nStates {
        activeResetStatesRefs = metamodelica::cons(qCref((literal!("activeResetStates")).clone(), tArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: j.clone() }) })], preRef.clone())?, activeResetStatesRefs.clone());
        vars = metamodelica::cons(makeVar(listHead(activeResetStatesRefs.clone())?, crate::NFType::interned_BOOLEAN(), Variability::DISCRETE.clone()), vars.clone());
        nextResetStatesRefs = metamodelica::cons(qCref((literal!("nextResetStates")).clone(), tArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: j.clone() }) })], preRef.clone())?, nextResetStatesRefs.clone());
        vars = metamodelica::cons(makeVarWithStart(listHead(nextResetStatesRefs.clone())?, crate::NFType::interned_BOOLEAN(), Variability::DISCRETE.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: false })), vars.clone());
        finalStatesRefs = metamodelica::cons(qCref((literal!("finalStates")).clone(), tArrayBool.clone(), list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: j.clone() }) })], preRef.clone())?, finalStatesRefs.clone());
        vars = metamodelica::cons(makeVar(listHead(finalStatesRefs.clone())?, crate::NFType::interned_BOOLEAN(), Variability::DISCRETE.clone()), vars.clone());
    }
    activeResetStatesRefs = activeResetStatesRefs.reverse();
    nextResetStatesRefs = nextResetStatesRefs.reverse();
    finalStatesRefs = finalStatesRefs.reverse();
    stateMachineInFinalStateRef = qCref((literal!("stateMachineInFinalState")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), preRef.clone())?;
    vars = metamodelica::cons(makeVar(stateMachineInFinalStateRef.clone(), crate::NFType::interned_BOOLEAN(), Variability::DISCRETE.clone()), vars);
    i = 0;
    for mut cExp in &*cExps.clone() {
        let mut cExp = cExp.clone();
        i = i + 1;
        eqs = metamodelica::cons(makeEq(makeCrefExp((cImmediateRefs.clone()).get(i)?, crate::NFType::interned_BOOLEAN()), cExp.clone(), crate::NFType::interned_BOOLEAN()), eqs.clone());
        let Transition { immediate: __pa0, .. } = ((t.clone()).get(i)?) else { bail!("pattern mismatch") };
        immediateVal = __pa0.clone();
        rhs = if (immediateVal) {makeCrefExp((cImmediateRefs.clone()).get(i)?, crate::NFType::interned_BOOLEAN())} else {makePreviousCall(makeCrefExp((cImmediateRefs.clone()).get(i)?, crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN())};
        eqs = metamodelica::cons(makeEq(makeCrefExp((cRefs.clone()).get(i)?, crate::NFType::interned_BOOLEAN()), rhs.clone(), crate::NFType::interned_BOOLEAN()), eqs.clone());
    }
    eqs = metamodelica::cons(makeEq(makeCrefExp(selectedStateRef.clone(), crate::NFType::interned_INTEGER()), makeIfExp(makeCrefExp(resetRef.clone(), crate::NFType::interned_BOOLEAN()), Arc::new(Expression::NFExpression::INTEGER { value: 1 }), makePreviousCall(makeCrefExp(nextStateRef.clone(), crate::NFType::interned_INTEGER()), crate::NFType::interned_INTEGER()), crate::NFType::interned_INTEGER()), crate::NFType::interned_INTEGER()), eqs);
    eqs = metamodelica::cons(makeEq(makeCrefExp(selectedResetRef.clone(), crate::NFType::interned_BOOLEAN()), makeIfExp(makeCrefExp(resetRef.clone(), crate::NFType::interned_BOOLEAN()), Arc::new(Expression::NFExpression::BOOLEAN { value: true }), makePreviousCall(makeCrefExp(nextResetRef.clone(), crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN()), eqs);
    expLst = metamodelica::nil();
    for mut j in 1..=nTransitions {
        expCond = makeRelationEq(makeCrefExp((tFromRefs.clone()).get(j.clone())?, crate::NFType::interned_INTEGER()), makeCrefExp(selectedStateRef.clone(), crate::NFType::interned_INTEGER()), crate::NFType::interned_INTEGER());
        expIf = makeIfExp(expCond.clone(), makeCrefExp((cRefs.clone()).get(j.clone())?, crate::NFType::interned_BOOLEAN()), Arc::new(Expression::NFExpression::BOOLEAN { value: false }), crate::NFType::interned_BOOLEAN());
        expLst = metamodelica::cons(makeIfExp(expIf.clone(), Arc::new(Expression::NFExpression::INTEGER { value: j.clone() }), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), crate::NFType::interned_INTEGER()), expLst.clone());
    }
    expLst = expLst.reverse();
    rhs = if ((expLst.clone().len() as i32) > 1) {makeMaxIntArrCall(expLst.clone())} else if ((expLst.clone().len() as i32) == 1) {listHead(expLst.clone())?} else {Arc::new(Expression::NFExpression::INTEGER { value: 0 })};
    eqs = metamodelica::cons(makeEq(makeCrefExp(firedRef.clone(), crate::NFType::interned_INTEGER()), rhs.clone(), crate::NFType::interned_INTEGER()), eqs);
    exp1 = makeRelationGt(makeCrefExp(firedRef.clone(), crate::NFType::interned_INTEGER()), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), crate::NFType::interned_INTEGER());
    exp2 = makeCrefExp(qCref((literal!("tTo")).clone(), tTArrayInt, list![Arc::new(Subscript::NFSubscript::INDEX { index: makeCrefExp(firedRef.clone(), crate::NFType::interned_INTEGER()) })], preRef.clone())?, crate::NFType::interned_INTEGER());
    expElse = makeIfExp(exp1, exp2, makeCrefExp(selectedStateRef, crate::NFType::interned_INTEGER()), crate::NFType::interned_INTEGER());
    eqs = metamodelica::cons(makeEq(makeCrefExp(activeStateRef.clone(), crate::NFType::interned_INTEGER()), makeIfExp(makeCrefExp(resetRef.clone(), crate::NFType::interned_BOOLEAN()), Arc::new(Expression::NFExpression::INTEGER { value: 1 }), expElse, crate::NFType::interned_INTEGER()), crate::NFType::interned_INTEGER()), eqs);
    exp1 = makeRelationGt(makeCrefExp(firedRef.clone(), crate::NFType::interned_INTEGER()), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), crate::NFType::interned_INTEGER());
    exp2 = makeCrefExp(qCref((literal!("tReset")).clone(), tTArrayBool, list![Arc::new(Subscript::NFSubscript::INDEX { index: makeCrefExp(firedRef, crate::NFType::interned_INTEGER()) })], preRef.clone())?, crate::NFType::interned_BOOLEAN());
    expElse = makeIfExp(exp1.clone(), exp2, makeCrefExp(selectedResetRef, crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN());
    eqs = metamodelica::cons(makeEq(makeCrefExp(activeResetRef, crate::NFType::interned_BOOLEAN()), makeIfExp(makeCrefExp(resetRef.clone(), crate::NFType::interned_BOOLEAN()), Arc::new(Expression::NFExpression::BOOLEAN { value: true }), expElse.clone(), crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN()), eqs);
    eqs = metamodelica::cons(makeEq(makeCrefExp(nextStateRef.clone(), crate::NFType::interned_INTEGER()), makeIfExp(makeCrefExp(activeRef.clone(), crate::NFType::interned_BOOLEAN()), makeCrefExp(activeStateRef.clone(), crate::NFType::interned_INTEGER()), makePreviousCall(makeCrefExp(nextStateRef, crate::NFType::interned_INTEGER()), crate::NFType::interned_INTEGER()), crate::NFType::interned_INTEGER()), crate::NFType::interned_INTEGER()), eqs);
    eqs = metamodelica::cons(makeEq(makeCrefExp(nextResetRef.clone(), crate::NFType::interned_BOOLEAN()), makeIfExp(makeCrefExp(activeRef.clone(), crate::NFType::interned_BOOLEAN()), Arc::new(Expression::NFExpression::BOOLEAN { value: false }), makePreviousCall(makeCrefExp(nextResetRef, crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN()), eqs);
    for mut j in 1..=nStates {
        eqs = metamodelica::cons(makeEq(makeCrefExp((activeResetStatesRefs.clone()).get(j.clone())?, crate::NFType::interned_BOOLEAN()), makeIfExp(makeCrefExp(resetRef.clone(), crate::NFType::interned_BOOLEAN()), Arc::new(Expression::NFExpression::BOOLEAN { value: true }), makePreviousCall(makeCrefExp((nextResetStatesRefs.clone()).get(j.clone())?, crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN()), eqs.clone());
    }
    for mut j in 1..=nStates {
        exp1 = makeRelationEq(makeCrefExp(activeStateRef.clone(), crate::NFType::interned_INTEGER()), Arc::new(Expression::NFExpression::INTEGER { value: j.clone() }), crate::NFType::interned_INTEGER());
        expThen = makeIfExp(exp1.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: false }), makeCrefExp((activeResetStatesRefs.clone()).get(j.clone())?, crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN());
        expElse = makePreviousCall(makeCrefExp((nextResetStatesRefs.clone()).get(j.clone())?, crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN());
        eqs = metamodelica::cons(makeEq(makeCrefExp((nextResetStatesRefs.clone()).get(j.clone())?, crate::NFType::interned_BOOLEAN()), makeIfExp(makeCrefExp(activeRef.clone(), crate::NFType::interned_BOOLEAN()), expThen.clone(), expElse.clone(), crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN()), eqs.clone());
    }
    for mut j in 1..=nStates {
        expLst = metamodelica::nil();
        for mut k in 1..=nTransitions {
            expCond = makeRelationEq(makeCrefExp((tFromRefs.clone()).get(k.clone())?, crate::NFType::interned_INTEGER()), Arc::new(Expression::NFExpression::INTEGER { value: j.clone() }), crate::NFType::interned_INTEGER());
            expLst = metamodelica::cons(makeIfExp(expCond.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 1 }), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), crate::NFType::interned_INTEGER()), expLst.clone());
        }
        expLst = expLst.clone().reverse();
        rhs = if ((expLst.clone().len() as i32) > 1) {makeRelationEq(makeMaxIntArrCall(expLst.clone()), Arc::new(Expression::NFExpression::INTEGER { value: 0 }), crate::NFType::interned_INTEGER())} else if ((expLst.clone().len() as i32) == 1) {makeRelationEq(listHead(expLst.clone())?, Arc::new(Expression::NFExpression::INTEGER { value: 0 }), crate::NFType::interned_INTEGER())} else {Arc::new(Expression::NFExpression::BOOLEAN { value: true })};
        eqs = metamodelica::cons(makeEq(makeCrefExp((finalStatesRefs.clone()).get(j.clone())?, crate::NFType::interned_BOOLEAN()), rhs.clone(), crate::NFType::interned_BOOLEAN()), eqs.clone());
    }
    eqs = metamodelica::cons(makeEq(makeCrefExp(stateMachineInFinalStateRef, crate::NFType::interned_BOOLEAN()), makeCrefExp(qCref((literal!("finalStates")).clone(), tArrayBool, list![Arc::new(Subscript::NFSubscript::INDEX { index: makeCrefExp(activeStateRef, crate::NFType::interned_INTEGER()) })], preRef)?, crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN()), eqs);
    sem = FlatSmSemantics { initStateRef: initStateCref, smComps: metamodelica::arrayFromVec(stateCrefs.into_iter().cloned().collect()), t: t, c: cExps, vars: vars, knowns: knowns, eqs: eqs, pvars: metamodelica::nil(), peqs: metamodelica::nil(), enclosingState: None };
    Ok(sem)
}

// ============================================================
// addPropagationEquations
// ============================================================
fn addPropagationEquations(mut inSem: FlatSmSemantics, mut enclosingStateCrefOpt: Option<Arc<ComponentRef::NFComponentRef>>, mut enclosingSmSemOpt: Option<FlatSmSemantics>) -> Result<FlatSmSemantics> {
    let mut outSem: FlatSmSemantics = inSem.clone();
    let mut preRef: Arc<ComponentRef::NFComponentRef>;
    let mut initStateRef: Arc<ComponentRef::NFComponentRef>;
    let mut activeRef: Arc<ComponentRef::NFComponentRef>;
    let mut resetRef: Arc<ComponentRef::NFComponentRef>;
    let mut initRef: Arc<ComponentRef::NFComponentRef>;
    let mut pvars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = metamodelica::nil();
    let mut peqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
    let mut nStates: i32;
    let mut posOfEnclosing: i32;
    let mut tArrayBool: Arc<Type::NFType>;
    let mut enclosingStateCref: Arc<ComponentRef::NFComponentRef>;
    let mut enclosingPreRef: Arc<ComponentRef::NFComponentRef>;
    let mut enclosingActiveResetStateRef: Arc<ComponentRef::NFComponentRef>;
    let mut enclosingActiveResetRef: Arc<ComponentRef::NFComponentRef>;
    let mut enclosingActiveStateRef: Arc<ComponentRef::NFComponentRef>;
    let mut enclosingInitStateRef: Arc<ComponentRef::NFComponentRef>;
    let mut enclosingSem: FlatSmSemantics;
    let mut enclosingComps: metamodelica::Array<Arc<ComponentRef::NFComponentRef>>;
    let mut stateRef: Arc<ComponentRef::NFComponentRef>;
    let mut activePlotRef: Arc<ComponentRef::NFComponentRef>;
    let mut activePlotVar: Arc<Variable::NFVariable>;
    let mut ticksVar: Arc<Variable::NFVariable>;
    let mut timeEnteredVar: Arc<Variable::NFVariable>;
    let mut timeInVar: Arc<Variable::NFVariable>;
    let mut activePlotEq: Arc<Equation::NFEquation>;
    let mut ticksEq: Arc<Equation::NFEquation>;
    let mut timeEnteredEq: Arc<Equation::NFEquation>;
    let mut timeInEq: Arc<Equation::NFEquation>;
    initStateRef = inSem.initStateRef.clone();
    preRef = makeSMSPrefix(initStateRef)?;
    activeRef = qCref((literal!("active")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), preRef.clone())?;
    resetRef = qCref((literal!("reset")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), preRef.clone())?;
    nStates = metamodelica::arrayLength(inSem.smComps.clone());
    tArrayBool = Arc::new(Type::NFType::ARRAY { elementType: crate::NFType::interned_BOOLEAN(), dimensions: list![Arc::new(Dimension::NFDimension::INTEGER { size: nStates, var: Variability::STRUCTURAL_PARAMETER.clone() })] });
    if isNone(enclosingSmSemOpt.clone()) {
        initRef = qCref((literal!("init")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), preRef.clone())?;
        pvars = metamodelica::cons(makeVarWithStart(initRef.clone(), crate::NFType::interned_BOOLEAN(), Variability::DISCRETE.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: true })), pvars);
        peqs = metamodelica::cons(makeEq(makeCrefExp(initRef.clone(), crate::NFType::interned_BOOLEAN()), Arc::new(Expression::NFExpression::BOOLEAN { value: false }), crate::NFType::interned_BOOLEAN()), peqs);
        peqs = metamodelica::cons(makeEq(makeCrefExp(resetRef, crate::NFType::interned_BOOLEAN()), makePreviousCall(makeCrefExp(initRef, crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN()), crate::NFType::interned_BOOLEAN()), peqs);
        peqs = metamodelica::cons(makeEq(makeCrefExp(activeRef, crate::NFType::interned_BOOLEAN()), Arc::new(Expression::NFExpression::BOOLEAN { value: true }), crate::NFType::interned_BOOLEAN()), peqs);
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(enclosingStateCrefOpt.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        enclosingStateCref = __pa0.clone();
        let __pa1 = ::match_deref::match_deref! { match &(enclosingSmSemOpt) {
            Some(__pa1) => __pa1.clone(),
            _ => bail!("pattern mismatch"),
        } };
        enclosingSem = __pa1.clone();
        enclosingComps = enclosingSem.smComps.clone();
        enclosingInitStateRef = metamodelica::arrayGet(enclosingComps.clone(), 1)?;
        enclosingPreRef = makeSMSPrefix(enclosingInitStateRef)?;
        posOfEnclosing = 1;
        let __range2 = &*Arc::new(enclosingComps.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
        for mut sc in __range2 {
            let mut sc = sc.clone();
            if ComponentRef::isEqual(sc.clone(), enclosingStateCref.clone())? {
                break;
            }
            posOfEnclosing = posOfEnclosing + 1;
        }
        enclosingActiveStateRef = qCref((literal!("activeState")).clone(), crate::NFType::interned_INTEGER(), metamodelica::nil(), enclosingPreRef.clone())?;
        enclosingActiveResetRef = qCref((literal!("activeReset")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), enclosingPreRef.clone())?;
        enclosingActiveResetStateRef = qCref((literal!("activeResetStates")).clone(), tArrayBool, list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: posOfEnclosing }) })], enclosingPreRef)?;
        peqs = metamodelica::cons(makeEq(makeCrefExp(resetRef, crate::NFType::interned_BOOLEAN()), Arc::new(Expression::NFExpression::LBINARY { exp1: makeCrefExp(enclosingActiveResetStateRef, crate::NFType::interned_BOOLEAN()), operator: Operator::makeOr(crate::NFType::interned_BOOLEAN()), exp2: Arc::new(Expression::NFExpression::LBINARY { exp1: makeCrefExp(enclosingActiveResetRef, crate::NFType::interned_BOOLEAN()), operator: Operator::makeAnd(crate::NFType::interned_BOOLEAN()), exp2: makeRelationEq(makeCrefExp(enclosingActiveStateRef.clone(), crate::NFType::interned_INTEGER()), Arc::new(Expression::NFExpression::INTEGER { value: posOfEnclosing }), crate::NFType::interned_INTEGER()) }) }), crate::NFType::interned_BOOLEAN()), peqs);
        peqs = metamodelica::cons(makeEq(makeCrefExp(activeRef, crate::NFType::interned_BOOLEAN()), makeRelationEq(makeCrefExp(enclosingActiveStateRef, crate::NFType::interned_INTEGER()), Arc::new(Expression::NFExpression::INTEGER { value: posOfEnclosing }), crate::NFType::interned_INTEGER()), crate::NFType::interned_BOOLEAN()), peqs);
    }
    for mut j in 1..=nStates {
        stateRef = metamodelica::arrayGet(inSem.smComps.clone(), j.clone())?;
        (activePlotVar, activePlotEq) = createActiveIndicator(stateRef.clone(), preRef.clone(), j.clone())?;
        pvars = metamodelica::cons(activePlotVar.clone(), pvars.clone());
        peqs = metamodelica::cons(activePlotEq.clone(), peqs.clone());
        activePlotRef = activePlotVar.name.clone();
        (ticksVar, ticksEq) = createTicksInStateIndicator(stateRef.clone(), activePlotRef.clone())?;
        pvars = metamodelica::cons(ticksVar.clone(), pvars.clone());
        peqs = metamodelica::cons(ticksEq.clone(), peqs.clone());
        (timeEnteredVar, timeEnteredEq) = createTimeEnteredStateIndicator(stateRef.clone(), activePlotRef.clone())?;
        (timeInVar, timeInEq) = createTimeInStateIndicator(stateRef.clone(), activePlotRef.clone(), timeEnteredVar.clone())?;
        pvars = metamodelica::cons(timeEnteredVar.clone(), metamodelica::cons(timeInVar.clone(), pvars.clone()));
        peqs = metamodelica::cons(timeEnteredEq.clone(), metamodelica::cons(timeInEq.clone(), peqs.clone()));
    }
    outSem.pvars = pvars;
    outSem.peqs = peqs;
    outSem.enclosingState = enclosingStateCrefOpt;
    Ok(outSem)
}

// ============================================================
// elabXInStateOps
// ============================================================
fn elabXInStateOps(mut sem: FlatSmSemantics, mut enclosingStateCrefOpt: Option<Arc<ComponentRef::NFComponentRef>>) -> Result<FlatSmSemantics> {
    let mut sem: FlatSmSemantics = sem;
    let mut tElab: Arc<metamodelica::List<Transition>> = metamodelica::nil();
    let mut cElab: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
    let mut i: i32;
    let mut stateRef: Arc<ComponentRef::NFComponentRef>;
    let mut substTickExp: Arc<Expression::NFExpression>;
    let mut substTimeExp: Arc<Expression::NFExpression>;
    let mut c3: Arc<Expression::NFExpression>;
    let mut c4: Arc<Expression::NFExpression>;
    let mut found: bool;
    let mut curT: Transition;
    let mut curFrom: i32;
    let mut curTo: i32;
    let mut curPriority: i32;
    let mut curImmediate: bool;
    let mut curReset: bool;
    let mut curSynchronize: bool;
    i = 0;
    for mut tc in &*List::zip(sem.t.clone(), sem.c.clone()) {
        let mut tc = tc.clone();
        i = i + 1;
        (_, c3) = tc.clone();
        curT = (sem.t.clone()).get(i)?;
        let Transition { from: __pa0, to: __pa1, immediate: __pa2, reset: __pa3, synchronize: __pa4, priority: __pa5, .. } = (curT.clone()) else { bail!("pattern mismatch") };
        curFrom = __pa0.clone();
        curTo = __pa1.clone();
        curImmediate = __pa2.clone();
        curReset = __pa3.clone();
        curSynchronize = __pa4.clone();
        curPriority = __pa5.clone();
        stateRef = metamodelica::arrayGet(sem.smComps.clone(), curFrom)?;
        substTickExp = makeCrefExp(qCref((literal!("$ticksInState")).clone(), crate::NFType::interned_INTEGER(), metamodelica::nil(), stateRef.clone())?, crate::NFType::interned_INTEGER());
        (c4, found) = subsXInState(c3.clone(), (literal!("ticksInState")).clone(), substTickExp.clone())?;
        if found && isSome(enclosingStateCrefOpt.clone()) {
            Error::addCompilerError((literal!("Found 'ticksInState()' within a state of a hierarchical state machine.")).clone())?;
            bail!("fail");
        }
        if found {
            sem.eqs = ({
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eq in (sem.eqs.clone()).into_iter().cloned() {
            let __x = smeqsSubsXInState(eq.clone(), metamodelica::arrayGet(sem.smComps.clone(), 1)?, i, (sem.t.clone().len() as i32), substTickExp.clone(), (literal!("ticksInState")).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        }
        substTimeExp = makeCrefExp(qCref((literal!("$timeInState")).clone(), crate::NFType::interned_REAL(), metamodelica::nil(), stateRef.clone())?, crate::NFType::interned_REAL());
        (c4, found) = subsXInState(c4.clone(), (literal!("timeInState")).clone(), substTimeExp.clone())?;
        if found && isSome(enclosingStateCrefOpt.clone()) {
            Error::addCompilerError((literal!("Found 'timeInState()' within a state of a hierarchical state machine.")).clone())?;
            bail!("fail");
        }
        if found {
            sem.eqs = ({
        let mut __acc: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = metamodelica::nil();
        for mut eq in (sem.eqs.clone()).into_iter().cloned() {
            let __x = smeqsSubsXInState(eq.clone(), metamodelica::arrayGet(sem.smComps.clone(), 1)?, i, (sem.t.clone().len() as i32), substTimeExp.clone(), (literal!("timeInState")).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
        }
        tElab = metamodelica::cons(Transition { from: curFrom, to: curTo, condition: c4.clone(), immediate: curImmediate, reset: curReset, synchronize: curSynchronize, priority: curPriority }, tElab.clone());
        cElab = metamodelica::cons(c4.clone(), cElab.clone());
    }
    sem.t = tElab.reverse();
    sem.c = cElab.reverse();
    Ok(sem)
}

fn subsXInState(mut inExp: Arc<Expression::NFExpression>, mut funcName: ArcStr, mut substExp: Arc<Expression::NFExpression>) -> Result<(Arc<Expression::NFExpression>, bool)> {
    let mut outExp: Arc<Expression::NFExpression>;
    let mut found: bool = false;
    (outExp, found) = Expression::mapFold(inExp, (std::sync::Arc::new({ let __pe_b1 = (funcName).clone(); let __pe_b2 = substExp; move |__pe_a0, __pe_a3| Ok(subsXInStateHelper(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>, bool) -> Result<(Arc<Expression::NFExpression>, bool)> + 'static>), false)?;
    Ok((outExp, found))
}

fn subsXInStateHelper(mut exp: Arc<Expression::NFExpression>, mut funcName: ArcStr, mut substExp: Arc<Expression::NFExpression>, mut found: bool) -> (Arc<Expression::NFExpression>, bool) {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut found: bool = found;
    let mut expCall: Arc<Call::NFCall>;
    if '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::CALL { call: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        expCall = __pa1.clone();
        if !(stringEq((unwrap_break_err!(Call::functionNameLast(expCall.clone()), '__try0)).clone(), (funcName.clone()).clone())) {
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
        }
        if !(unwrap_break_err!(Call::arguments(expCall.clone()), '__try0).is_empty()) {
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
        }
        exp = substExp.clone();
        found = true;
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    (exp, found)
}

fn smeqsSubsXInState(mut eq: Arc<Equation::NFEquation>, mut initStateComp: Arc<ComponentRef::NFComponentRef>, mut i: i32, mut nTransitions: i32, mut substExp: Arc<Expression::NFExpression>, mut xInState: ArcStr) -> Result<Arc<Equation::NFEquation>> {
    let mut outEq: Arc<Equation::NFEquation> = eq.clone();
    let mut preRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut lhsRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut cRef: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let mut tArrayBool: Arc<Type::NFType> = Arc::new(Type::ANY);
    let mut lhs: Arc<Expression::NFExpression>;
    let mut rhs: Arc<Expression::NFExpression>;
    let mut newRhs: Arc<Expression::NFExpression> = Arc::new(Expression::END);
    outEq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { .. } => {
            preRef = makeSMSPrefix(initStateComp)?;
            tArrayBool = Arc::new(Type::NFType::ARRAY { elementType: crate::NFType::interned_BOOLEAN(), dimensions: list![Arc::new(Dimension::NFDimension::INTEGER { size: nTransitions, var: Variability::STRUCTURAL_PARAMETER.clone() })] });
            cRef = qCref((literal!("cImmediate")).clone(), tArrayBool, list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i }) })], preRef)?;
            let __pa0 = ::match_deref::match_deref! { match &(var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone()) {
                Deref @ Expression::CREF { cref: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            lhsRef = __pa0.clone();
            if ComponentRef::isEqual(cRef, lhsRef)? {
                (newRhs, _) = subsXInState(var_field!((*eq).rhs, Equation::NFEquation::EQUALITY).clone(), (xInState).clone(), substExp)?;
                outEq = Arc::new(Equation::NFEquation::EQUALITY { lhs: var_field!((*eq).lhs, Equation::NFEquation::EQUALITY).clone(), rhs: newRhs, ty: var_field!((*eq).ty, Equation::NFEquation::EQUALITY).clone(), scope: var_field!((*eq).scope, Equation::NFEquation::EQUALITY).clone(), source: var_field!((*eq).source, Equation::NFEquation::EQUALITY).clone(), scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone() });
            }
            outEq
        },
        _ => eq,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEq)
}

// ============================================================
// State indicator helpers
// ============================================================
fn createActiveIndicator(mut stateRef: Arc<ComponentRef::NFComponentRef>, mut preRef: Arc<ComponentRef::NFComponentRef>, mut i: i32) -> Result<(Arc<Variable::NFVariable>, Arc<Equation::NFEquation>)> {
    let mut activePlotVar: Arc<Variable::NFVariable>;
    let mut eqn: Arc<Equation::NFEquation>;
    let mut activePlotRef: Arc<ComponentRef::NFComponentRef>;
    let mut activeRef: Arc<ComponentRef::NFComponentRef>;
    let mut activeStateRef: Arc<ComponentRef::NFComponentRef>;
    let mut andExp: Arc<Expression::NFExpression>;
    let mut eqExp: Arc<Expression::NFExpression>;
    activePlotRef = qCref((literal!("active")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), stateRef)?;
    activePlotVar = makeVarWithStart(activePlotRef.clone(), crate::NFType::interned_BOOLEAN(), Variability::DISCRETE.clone(), Arc::new(Expression::NFExpression::BOOLEAN { value: false }));
    activeRef = qCref((literal!("active")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), preRef.clone())?;
    activeStateRef = qCref((literal!("activeState")).clone(), crate::NFType::interned_INTEGER(), metamodelica::nil(), preRef)?;
    eqExp = makeRelationEq(makeCrefExp(activeStateRef, crate::NFType::interned_INTEGER()), Arc::new(Expression::NFExpression::INTEGER { value: i }), crate::NFType::interned_INTEGER());
    andExp = Arc::new(Expression::NFExpression::LBINARY { exp1: makeCrefExp(activeRef, crate::NFType::interned_BOOLEAN()), operator: Operator::makeAnd(crate::NFType::interned_BOOLEAN()), exp2: eqExp });
    eqn = makeEq(makeCrefExp(activePlotRef, crate::NFType::interned_BOOLEAN()), andExp, crate::NFType::interned_BOOLEAN());
    Ok((activePlotVar, eqn))
}

fn createTicksInStateIndicator(mut stateRef: Arc<ComponentRef::NFComponentRef>, mut stateActiveRef: Arc<ComponentRef::NFComponentRef>) -> Result<(Arc<Variable::NFVariable>, Arc<Equation::NFEquation>)> {
    let mut ticksVar: Arc<Variable::NFVariable>;
    let mut ticksEq: Arc<Equation::NFEquation>;
    let mut ticksRef: Arc<ComponentRef::NFComponentRef>;
    let mut ticksExp: Arc<Expression::NFExpression>;
    let mut expCond: Arc<Expression::NFExpression>;
    let mut expThen: Arc<Expression::NFExpression>;
    let mut expElse: Arc<Expression::NFExpression>;
    ticksRef = qCref((literal!("$ticksInState")).clone(), crate::NFType::interned_INTEGER(), metamodelica::nil(), stateRef)?;
    ticksVar = makeVarWithStart(ticksRef.clone(), crate::NFType::interned_INTEGER(), Variability::DISCRETE.clone(), Arc::new(Expression::NFExpression::INTEGER { value: 0 }));
    ticksExp = makeCrefExp(ticksRef, crate::NFType::interned_INTEGER());
    expCond = Arc::new(Expression::NFExpression::LUNARY { operator: Operator::makeNot(crate::NFType::interned_BOOLEAN()), exp: makeCrefExp(stateActiveRef, crate::NFType::interned_BOOLEAN()) });
    expThen = Arc::new(Expression::NFExpression::INTEGER { value: 0 });
    expElse = Arc::new(Expression::NFExpression::BINARY { exp1: makePreviousCall(ticksExp.clone(), crate::NFType::interned_INTEGER()), operator: Operator::makeAdd(crate::NFType::interned_INTEGER()), exp2: Arc::new(Expression::NFExpression::INTEGER { value: 1 }) });
    ticksEq = makeEq(ticksExp, makeIfExp(expCond, expThen, expElse, crate::NFType::interned_INTEGER()), crate::NFType::interned_INTEGER());
    Ok((ticksVar, ticksEq))
}

fn createTimeEnteredStateIndicator(mut stateRef: Arc<ComponentRef::NFComponentRef>, mut stateActiveRef: Arc<ComponentRef::NFComponentRef>) -> Result<(Arc<Variable::NFVariable>, Arc<Equation::NFEquation>)> {
    let mut timeEnteredVar: Arc<Variable::NFVariable>;
    let mut timeEnteredEq: Arc<Equation::NFEquation>;
    let mut timeEnteredRef: Arc<ComponentRef::NFComponentRef>;
    let mut timeEnteredExp: Arc<Expression::NFExpression>;
    let mut expCond: Arc<Expression::NFExpression>;
    let mut expThen: Arc<Expression::NFExpression>;
    let mut expElse: Arc<Expression::NFExpression>;
    let mut activeExp: Arc<Expression::NFExpression>;
    timeEnteredRef = qCref((literal!("$timeEnteredState")).clone(), crate::NFType::interned_REAL(), metamodelica::nil(), stateRef)?;
    timeEnteredVar = makeVarWithStart(timeEnteredRef.clone(), crate::NFType::interned_REAL(), Variability::CONTINUOUS.clone(), Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }));
    timeEnteredExp = makeCrefExp(timeEnteredRef, crate::NFType::interned_REAL());
    activeExp = makeCrefExp(stateActiveRef, crate::NFType::interned_BOOLEAN());
    expCond = Arc::new(Expression::NFExpression::LBINARY { exp1: makeRelationEq(makePreviousCall(activeExp.clone(), crate::NFType::interned_BOOLEAN()), Arc::new(Expression::NFExpression::BOOLEAN { value: false }), crate::NFType::interned_BOOLEAN()), operator: Operator::makeAnd(crate::NFType::interned_BOOLEAN()), exp2: makeRelationEq(activeExp, Arc::new(Expression::NFExpression::BOOLEAN { value: true }), crate::NFType::interned_BOOLEAN()) });
    expThen = makeSampleTimeCall();
    expElse = makePreviousCall(timeEnteredExp.clone(), crate::NFType::interned_REAL());
    timeEnteredEq = makeEq(timeEnteredExp, makeIfExp(expCond, expThen, expElse, crate::NFType::interned_REAL()), crate::NFType::interned_REAL());
    Ok((timeEnteredVar, timeEnteredEq))
}

fn createTimeInStateIndicator(mut stateRef: Arc<ComponentRef::NFComponentRef>, mut stateActiveRef: Arc<ComponentRef::NFComponentRef>, mut timeEnteredVar: Arc<Variable::NFVariable>) -> Result<(Arc<Variable::NFVariable>, Arc<Equation::NFEquation>)> {
    let mut timeInVar: Arc<Variable::NFVariable>;
    let mut timeInEq: Arc<Equation::NFEquation>;
    let mut timeInRef: Arc<ComponentRef::NFComponentRef>;
    let mut timeInExp: Arc<Expression::NFExpression>;
    let mut expCond: Arc<Expression::NFExpression>;
    let mut expThen: Arc<Expression::NFExpression>;
    let mut expElse: Arc<Expression::NFExpression>;
    let mut timeEnteredExp: Arc<Expression::NFExpression>;
    timeInRef = qCref((literal!("$timeInState")).clone(), crate::NFType::interned_REAL(), metamodelica::nil(), stateRef)?;
    timeInVar = makeVarWithStart(timeInRef.clone(), crate::NFType::interned_REAL(), Variability::CONTINUOUS.clone(), Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }));
    timeInExp = makeCrefExp(timeInRef, crate::NFType::interned_REAL());
    timeEnteredExp = makeCrefExp(timeEnteredVar.name.clone(), crate::NFType::interned_REAL());
    expCond = makeCrefExp(stateActiveRef, crate::NFType::interned_BOOLEAN());
    expThen = Arc::new(Expression::NFExpression::BINARY { exp1: makeSampleTimeCall(), operator: Operator::makeSub(crate::NFType::interned_REAL()), exp2: timeEnteredExp });
    expElse = Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) });
    timeInEq = makeEq(timeInExp, makeIfExp(expCond, expThen, expElse, crate::NFType::interned_REAL()), crate::NFType::interned_REAL());
    Ok((timeInVar, timeInEq))
}

// ============================================================
// Reset and activation wrapping
// ============================================================
fn wrapInStateActivationConditional(mut inEq: Arc<Equation::NFEquation>, mut stateCref: Arc<ComponentRef::NFComponentRef>, mut isResetEquation: bool) -> Result<Arc<Equation::NFEquation>> {
    let mut outEq: Arc<Equation::NFEquation>;
    let mut lhs: Arc<Expression::NFExpression>;
    let mut rhs: Arc<Expression::NFExpression>;
    let mut activeRef: Arc<Expression::NFExpression>;
    let mut expElse: Arc<Expression::NFExpression>;
    let mut lhsCref: Arc<ComponentRef::NFComponentRef>;
    let mut ty: Arc<Type::NFType>;
    let mut eqScope: Arc<InstNode::InstNode>;
    let mut eqSource: Arc<DAE::ElementSource>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(inEq) {
        Deref @ Equation::EQUALITY { lhs: __pa0, rhs: __pa1, ty: __pa2, scope: __pa3, source: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    lhs = __pa0.clone();
    rhs = __pa1.clone();
    ty = __pa2.clone();
    eqScope = __pa3.clone();
    eqSource = __pa4.clone();
    let (__pa5, __pa6) = ::match_deref::match_deref! { match &(lhs.clone()) {
        Deref @ Expression::CREF { ty: __pa5, cref: __pa6 } => (__pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ty = __pa5.clone();
    lhsCref = __pa6.clone();
    activeRef = makeCrefExp(qCref((literal!("active")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), stateCref)?, crate::NFType::interned_BOOLEAN());
    if isResetEquation {
        expElse = makeCrefExp(ComponentRef::prefixCref(Arc::new(InstNode::InstNode::NAME_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::firstName(lhsCref.clone(), false)?); __mm_s.push_str(&*literal!("_previous")); ArcStr::from(__mm_s) }).clone() }), ty.clone(), metamodelica::nil(), ComponentRef::rest(lhsCref)?), ty.clone());
    } else {
        expElse = makePreviousCall(lhs.clone(), ty.clone());
    }
    outEq = Arc::new(Equation::NFEquation::EQUALITY { lhs: lhs, rhs: makeIfExp(activeRef, rhs, expElse, ty.clone()), ty: ty, scope: eqScope, source: eqSource, scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone() });
    Ok(outEq)
}

fn createResetEquation(mut lhsCref: Arc<ComponentRef::NFComponentRef>, mut lhsTy: Arc<Type::NFType>, mut stateCref: Arc<ComponentRef::NFComponentRef>, mut sem: FlatSmSemantics, mut crToStart: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<Expression::NFExpression>>>) -> Result<Arc<Equation::NFEquation>> {
    let mut outEq: Arc<Equation::NFEquation>;
    let mut preRef: Arc<ComponentRef::NFComponentRef>;
    let mut initStateRef: Arc<ComponentRef::NFComponentRef>;
    let mut activeExp: Arc<Expression::NFExpression>;
    let mut activeResetExp: Arc<Expression::NFExpression>;
    let mut activeResetStatesExp: Arc<Expression::NFExpression>;
    let mut orExp: Arc<Expression::NFExpression>;
    let mut andExp: Arc<Expression::NFExpression>;
    let mut prevExp: Arc<Expression::NFExpression>;
    let mut startExp: Arc<Expression::NFExpression>;
    let mut ifExp: Arc<Expression::NFExpression>;
    let mut lhsPrevExp: Arc<Expression::NFExpression>;
    let mut i: i32;
    let mut nStates: i32;
    let mut tArrayBool: Arc<Type::NFType>;
    initStateRef = metamodelica::arrayGet(sem.smComps.clone(), 1)?;
    preRef = makeSMSPrefix(initStateRef)?;
    i = 1;
    let __range0 = &*Arc::new(sem.smComps.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    for mut sc in __range0 {
        let mut sc = sc.clone();
        if ComponentRef::isEqual(sc.clone(), stateCref.clone())? {
            break;
        }
        i = i + 1;
    }
    nStates = metamodelica::arrayLength(sem.smComps.clone());
    tArrayBool = Arc::new(Type::NFType::ARRAY { elementType: crate::NFType::interned_BOOLEAN(), dimensions: list![Arc::new(Dimension::NFDimension::INTEGER { size: nStates, var: Variability::STRUCTURAL_PARAMETER.clone() })] });
    activeResetExp = makeCrefExp(qCref((literal!("activeReset")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), preRef.clone())?, crate::NFType::interned_BOOLEAN());
    activeResetStatesExp = makeCrefExp(qCref((literal!("activeResetStates")).clone(), tArrayBool, list![Arc::new(Subscript::NFSubscript::INDEX { index: Arc::new(Expression::NFExpression::INTEGER { value: i }) })], preRef)?, crate::NFType::interned_BOOLEAN());
    orExp = Arc::new(Expression::NFExpression::LBINARY { exp1: activeResetExp, operator: Operator::makeOr(crate::NFType::interned_BOOLEAN()), exp2: activeResetStatesExp });
    activeExp = makeCrefExp(qCref((literal!("active")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), stateCref)?, crate::NFType::interned_BOOLEAN());
    andExp = Arc::new(Expression::NFExpression::LBINARY { exp1: activeExp, operator: Operator::makeAnd(crate::NFType::interned_BOOLEAN()), exp2: orExp });
    prevExp = makePreviousCall(makeCrefExp(lhsCref.clone(), lhsTy.clone()), lhsTy.clone());
    startExp = UnorderedMap::getOrDefault(lhsCref.clone(), crToStart, Arc::new(Expression::NFExpression::INTEGER { value: 0 }))?;
    ifExp = makeIfExp(andExp, startExp, prevExp, lhsTy.clone());
    lhsPrevExp = makeCrefExp(ComponentRef::prefixCref(Arc::new(InstNode::InstNode::NAME_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ComponentRef::firstName(lhsCref.clone(), false)?); __mm_s.push_str(&*literal!("_previous")); ArcStr::from(__mm_s) }).clone() }), lhsTy.clone(), metamodelica::nil(), ComponentRef::rest(lhsCref)?), lhsTy.clone());
    outEq = makeEq(lhsPrevExp, ifExp, lhsTy);
    Ok(outEq)
}

// ============================================================
// Expression substitution helpers
// ============================================================
fn subsActiveStateInEq(mut eq: Arc<Equation::NFEquation>) -> Result<Arc<Equation::NFEquation>> {
    let mut eq: Arc<Equation::NFEquation> = eq;
    eq = Equation::mapExp(eq, (std::sync::Arc::new(subsActiveStateInExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(eq)
}

fn subsActiveStateInExp(mut exp: Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    exp = Expression::map(exp, (std::sync::Arc::new(fnptr!(subsActiveStateHelper, Arc<Expression::NFExpression>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Expression::NFExpression>) -> Result<Arc<Expression::NFExpression>> + 'static>))?;
    Ok(exp)
}

fn subsActiveStateHelper(mut exp: Arc<Expression::NFExpression>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut expCall: Arc<Call::NFCall>;
    let mut argCref: Arc<ComponentRef::NFComponentRef>;
    let mut newExp: Arc<Expression::NFExpression>;
    if '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::CALL { call: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        expCall = __pa1.clone();
        if !(stringEq((unwrap_break_err!(Call::functionNameLast(expCall.clone()), '__try0)).clone(), (literal!("activeState")).clone())) {
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
        }
        let __pa2 = ::match_deref::match_deref! { match &(unwrap_break_err!(Call::arguments(expCall.clone()), '__try0)) {
            Deref @ metamodelica::List::Cons { head: Deref @ Expression::CREF { cref: __pa2, .. }, tail: Deref @ metamodelica::List::Nil } => __pa2.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        argCref = __pa2.clone();
        newExp = makeCrefExp(unwrap_break_err!(qCref((literal!("active")).clone(), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), argCref.clone()), '__try0), crate::NFType::interned_BOOLEAN());
        exp = newExp.clone();
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    exp
}

fn subsPreviousCrefs(mut exp: Arc<Expression::NFExpression>, mut stateVarCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut found: bool) -> (Arc<Expression::NFExpression>, bool) {
    let mut exp: Arc<Expression::NFExpression> = exp;
    let mut found: bool = found;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut arg1: Arc<Expression::NFExpression>;
    let mut argTy: Arc<Type::NFType>;
    let mut argCref: Arc<ComponentRef::NFComponentRef>;
    let mut expCall: Arc<Call::NFCall>;
    let mut newExp: Arc<Expression::NFExpression>;
    if '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::CALL { call: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        expCall = __pa1.clone();
        if !(stringEq((unwrap_break_err!(Call::functionNameLast(expCall.clone()), '__try0)).clone(), (literal!("previous")).clone())) {
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
        }
        args = unwrap_break_err!(Call::arguments(expCall.clone()), '__try0);
        if (args.clone().len() as i32) != 1 {
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
        }
        arg1 = unwrap_break_err!(listHead(args.clone()), '__try0);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(arg1.clone()) {
            Deref @ Expression::CREF { ty: __pa2, cref: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        argTy = __pa2.clone();
        argCref = __pa3.clone();
        for mut svc in &*stateVarCrefs.clone() {
            let mut svc = svc.clone();
            if unwrap_break_err!(ComponentRef::isEqual(svc.clone(), argCref.clone()), '__try0) {
                newExp = makeCrefExp(ComponentRef::prefixCref(Arc::new(InstNode::InstNode::NAME_NODE { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::firstName(argCref.clone(), false), '__try0)); __mm_s.push_str(&*literal!("_previous")); ArcStr::from(__mm_s) }).clone() }), argTy.clone(), metamodelica::nil(), unwrap_break_err!(ComponentRef::rest(argCref.clone()), '__try0)), argTy.clone());
                exp = newExp.clone();
                found = true;
                break;
            }
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    (exp, found)
}

// ============================================================
// createTandC
// ============================================================
fn createTandC(mut stateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>, mut transitionEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>) -> Result<(Arc<metamodelica::List<Transition>>, Arc<metamodelica::List<Arc<Expression::NFExpression>>>)> {
    let mut t: Arc<metamodelica::List<Transition>>;
    let mut c: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut transitions: Arc<metamodelica::List<Transition>>;
    transitions = List::filterMap(transitionEqs, (std::sync::Arc::new({ let __pe_b1 = stateCrefs; move |__pe_a0| extractTransition(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Equation::NFEquation>) -> Result<Transition> + 'static>));
    t = List::sort(transitions, (std::sync::Arc::new(fnptr!(priorityGt, Transition, Transition)) as std::sync::Arc<dyn ::std::ops::Fn(Transition, Transition) -> Result<bool> + 'static>))?;
    c = ({
        let mut __acc: Arc<metamodelica::List<Arc<Expression::NFExpression>>> = metamodelica::nil();
        for mut tr in (t.clone()).into_iter().cloned() {
            let __x = tr.condition.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((t, c))
}

fn extractTransition(mut eq: Arc<Equation::NFEquation>, mut stateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<Transition> {
    let mut trans: Transition;
    let mut crFrom: Arc<ComponentRef::NFComponentRef>;
    let mut crTo: Arc<ComponentRef::NFComponentRef>;
    let mut cond: Arc<Expression::NFExpression>;
    let mut imm: bool = true;
    let mut rst: bool = true;
    let mut syn: bool = false;
    let mut prio: i32 = 1;
    let mut from: i32;
    let mut to: i32;
    let mut args: Arc<metamodelica::List<Arc<Expression::NFExpression>>>;
    let mut eqCall: Arc<Call::NFCall>;
    let __pa0 = ::match_deref::match_deref! { match &(eq) {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: __pa0 }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqCall = __pa0.clone();
    if !(stringEq((Call::functionNameLast(eqCall.clone())?).clone(), (literal!("transition")).clone())) {
        bail!("fail");
    }
    args = Call::arguments(eqCall)?;
    let __pa2 = ::match_deref::match_deref! { match &((args.clone()).get(1)?) {
        Deref @ Expression::CREF { cref: __pa2, .. } => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    crFrom = __pa2.clone();
    let __pa3 = ::match_deref::match_deref! { match &((args.clone()).get(2)?) {
        Deref @ Expression::CREF { cref: __pa3, .. } => __pa3.clone(),
        _ => bail!("pattern mismatch"),
    } };
    crTo = __pa3.clone();
    cond = (args.clone()).get(3)?;
    if (args.clone().len() as i32) >= 4 {
        let __pa4 = ::match_deref::match_deref! { match &((args.clone()).get(4)?) {
            Deref @ Expression::BOOLEAN { value: __pa4 } => __pa4.clone(),
            _ => bail!("pattern mismatch"),
        } };
        imm = __pa4.clone();
    }
    if (args.clone().len() as i32) >= 5 {
        let __pa5 = ::match_deref::match_deref! { match &((args.clone()).get(5)?) {
            Deref @ Expression::BOOLEAN { value: __pa5 } => __pa5.clone(),
            _ => bail!("pattern mismatch"),
        } };
        rst = __pa5.clone();
    }
    if (args.clone().len() as i32) >= 6 {
        let __pa6 = ::match_deref::match_deref! { match &((args.clone()).get(6)?) {
            Deref @ Expression::BOOLEAN { value: __pa6 } => __pa6.clone(),
            _ => bail!("pattern mismatch"),
        } };
        syn = __pa6.clone();
    }
    if (args.clone().len() as i32) >= 7 {
        let __pa7 = ::match_deref::match_deref! { match &((args).get(7)?) {
            Deref @ Expression::INTEGER { value: __pa7 } => __pa7.clone(),
            _ => bail!("pattern mismatch"),
        } };
        prio = __pa7.clone();
    }
    from = 1;
    for mut sc in &*stateCrefs.clone() {
        let mut sc = sc.clone();
        if ComponentRef::isEqual(sc.clone(), crFrom.clone())? {
            break;
        }
        from = from + 1;
    }
    to = 1;
    for mut sc in &*stateCrefs {
        let mut sc = sc.clone();
        if ComponentRef::isEqual(sc.clone(), crTo.clone())? {
            break;
        }
        to = to + 1;
    }
    trans = Transition { from: from, to: to, condition: cond, immediate: imm, reset: rst, synchronize: syn, priority: prio };
    Ok(trans)
}

fn priorityGt(mut t1: Transition, mut t2: Transition) -> bool {
    let mut gt: bool;
    gt = t1.priority.clone() > t2.priority.clone();
    gt
}

// ============================================================
// Predicate helpers
// ============================================================
fn isTransitionOrInitialState(mut eq: Arc<Equation::NFEquation>) -> Result<bool> {
    let mut res: bool = false;
    let () = (::match_deref::match_deref! { match &(eq) {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: eqCall }, .. } => {
            res = (::match_deref::match_deref! { match &(Call::functionNameLast(eqCall.clone())?) {
        Deref @ "transition" => true,
        Deref @ "initialState" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn isTransitionForGroup(mut eq: Arc<Equation::NFEquation>, mut stateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<bool> {
    let mut res: bool = false;
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let () = (::match_deref::match_deref! { match &(eq) {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: eqCall }, .. } if (stringEq((Call::functionNameLast(eqCall.clone())?).clone(), (literal!("transition")).clone())) => {
            let __pa0 = ::match_deref::match_deref! { match &(listHead(Call::arguments(eqCall.clone())?)?) {
                Deref @ Expression::CREF { cref: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            for mut sc in &*stateCrefs {
                let mut sc = sc.clone();
                if ComponentRef::isEqual(cr.clone(), sc.clone())? {
                    res = true;
                    break;
                }
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn isInitialStateForGroup(mut eq: Arc<Equation::NFEquation>, mut initStateCref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut res: bool = false;
    let mut cr: Arc<ComponentRef::NFComponentRef> = Arc::new(ComponentRef::EMPTY);
    let () = (::match_deref::match_deref! { match &(eq) {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: eqCall }, .. } if (stringEq((Call::functionNameLast(eqCall.clone())?).clone(), (literal!("initialState")).clone())) => {
            let __pa0 = ::match_deref::match_deref! { match &(listHead(Call::arguments(eqCall.clone())?)?) {
                Deref @ Expression::CREF { cref: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            res = ComponentRef::isEqual(cr, initStateCref)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn isEquationOfState(mut eq: Arc<Equation::NFEquation>, mut stateCref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut res: bool = false;
    let mut eqScope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut stateName: ArcStr;
    stateName = (ComponentRef::firstName(stateCref, false)?).clone();
    let () = (::match_deref::match_deref! { match &(eq) {
        Deref @ Equation::EQUALITY { scope: __esc_eqScope, .. } => {
            eqScope = (*__esc_eqScope).clone();
            res = stringEqual((InstNode::name(eqScope.clone())?).clone(), (stateName).clone());
            ()
        },
        Deref @ Equation::WHEN { scope: __esc_eqScope, .. } => {
            eqScope = (*__esc_eqScope).clone();
            res = stringEqual((InstNode::name(eqScope.clone())?).clone(), (stateName).clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn isVariableOfState(mut var: Arc<Variable::NFVariable>, mut stateCref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    let mut res: bool;
    res = crefHasPrefix(stateCref, var.name.clone())?;
    Ok(res)
}

fn isOuterStateEquation(mut eq: Arc<Equation::NFEquation>, mut stateCrefs: Arc<metamodelica::List<Arc<ComponentRef::NFComponentRef>>>) -> Result<bool> {
    let mut res: bool = false;
    let mut eqScope: Arc<InstNode::InstNode> = Arc::new(InstNode::EMPTY_NODE);
    let mut scopeName: ArcStr = arcstr::literal!("");
    let () = (::match_deref::match_deref! { match &(eq) {
        Deref @ Equation::EQUALITY { scope: __esc_eqScope, .. } => {
            eqScope = (*__esc_eqScope).clone();
            scopeName = (InstNode::name(eqScope.clone())?).clone();
            for mut stateCref in &*stateCrefs {
                let mut stateCref = stateCref.clone();
                if stringEqual((scopeName.clone()).clone(), (ComponentRef::firstName(stateCref.clone(), false)?).clone()) {
                    res = true;
                    return Ok(res.clone());
                }
            }
            ()
        },
        Deref @ Equation::WHEN { scope: __esc_eqScope, .. } => {
            eqScope = (*__esc_eqScope).clone();
            scopeName = (InstNode::name(eqScope.clone())?).clone();
            for mut stateCref in &*stateCrefs {
                let mut stateCref = stateCref.clone();
                if stringEqual((scopeName.clone()).clone(), (ComponentRef::firstName(stateCref.clone(), false)?).clone()) {
                    res = true;
                    return Ok(res.clone());
                }
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn generateMergeEquation(mut outerVarCref: Arc<ComponentRef::NFComponentRef>, mut outerVarMap: Arc<UnorderedMap::UnorderedMap<Arc<ComponentRef::NFComponentRef>, Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>>>, mut allVariables: Arc<metamodelica::List<Arc<Variable::NFVariable>>>, mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>>, mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>>) -> Result<(Arc<metamodelica::List<Arc<Equation::NFEquation>>>, Arc<metamodelica::List<Arc<Variable::NFVariable>>>)> {
    let mut accEqs: Arc<metamodelica::List<Arc<Equation::NFEquation>>> = accEqs;
    let mut accVars: Arc<metamodelica::List<Arc<Variable::NFVariable>>> = accVars;
    let mut stateEntries: Arc<metamodelica::List<(Arc<ComponentRef::NFComponentRef>, Arc<ComponentRef::NFComponentRef>)>>;
    let mut mergeRhs: Arc<Expression::NFExpression>;
    let mut outerVarExp: Arc<Expression::NFExpression>;
    let mut ty: Arc<Type::NFType>;
    let mut activeRef: Arc<ComponentRef::NFComponentRef>;
    let mut perStateVarRef: Arc<ComponentRef::NFComponentRef>;
    let mut src: Arc<DAE::ElementSource>;
    stateEntries = UnorderedMap::getOrDefault(outerVarCref.clone(), outerVarMap, metamodelica::nil())?;
    if stateEntries.clone().is_empty() {
        return Ok((accEqs.clone(), accVars.clone()));
    }
    ty = crate::NFType::interned_INTEGER();
    for mut v in &*allVariables {
        let mut v = v.clone();
        if ComponentRef::isEqual(v.name.clone(), outerVarCref.clone())? {
            ty = v.ty.clone();
            break;
        }
    }
    outerVarExp = makeCrefExp(outerVarCref, ty.clone());
    mergeRhs = makePreviousCall(outerVarExp.clone(), ty.clone());
    for mut entry in &*stateEntries {
        let mut entry = entry.clone();
        (activeRef, perStateVarRef) = entry.clone();
        mergeRhs = makeIfExp(makeCrefExp(activeRef.clone(), crate::NFType::interned_BOOLEAN()), makeCrefExp(perStateVarRef.clone(), ty.clone()), mergeRhs.clone(), ty.clone());
    }
    src = ElementSource::createElementSource(Absyn::dummyInfo.clone(), None, openmodelica_frontend_types::DAE::Prefix::NOPRE, (DAE::emptyCref().clone(), DAE::emptyCref().clone()))?;
    accEqs = metamodelica::cons(Arc::new(Equation::NFEquation::EQUALITY { lhs: outerVarExp, rhs: mergeRhs, ty: ty, scope: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), source: src, scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone() }), accEqs);
    Ok((accEqs, accVars))
}

// ============================================================
// ComponentRef utilities
// ============================================================
fn qCref(mut name: ArcStr, mut ty: Arc<Type::NFType>, mut subs: Arc<metamodelica::List<Arc<Subscript::NFSubscript>>>, mut prefixCr: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut cref: Arc<ComponentRef::NFComponentRef>;
    cref = ComponentRef::fromNode(Arc::new(InstNode::InstNode::NAME_NODE { name: (name).clone() }), ty, subs, ComponentRef::Origin::CREF.clone());
    cref = ComponentRef::prepend(prefixCr, cref)?;
    Ok(cref)
}

fn makeSMSPrefix(mut initStateCref: Arc<ComponentRef::NFComponentRef>) -> Result<Arc<ComponentRef::NFComponentRef>> {
    let mut preRef: Arc<ComponentRef::NFComponentRef>;
    preRef = ComponentRef::fromNode(Arc::new(InstNode::InstNode::NAME_NODE { name: (arcstr::literal!(SMS_PRE)).clone() }), crate::NFType::interned_UNKNOWN(), metamodelica::nil(), ComponentRef::Origin::CREF.clone());
    preRef = ComponentRef::append(initStateCref, preRef)?;
    Ok(preRef)
}

// ============================================================
// Variable creation helpers
// ============================================================
fn makeVar(mut name: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>, mut var: Variability) -> Arc<Variable::NFVariable> {
    let mut v: Arc<Variable::NFVariable>;
    let mut attr: Arc<Attributes::NFAttributes>;
    attr = Attributes::DEFAULT_ATTR().clone();
    assign_field!(attr.variability = var);
    v = Arc::new(Variable::NFVariable { name: name, ty: ty, binding: Binding::EMPTY_BINDING().clone(), visibility: Visibility::PUBLIC.clone(), attributes: attr, typeAttributes: metamodelica::nil(), children: metamodelica::nil(), comment: Arc::new(SCode::Comment { annotation_: None, comment: None }), info: Absyn::dummyInfo.clone(), backendinfo: NFBackendExtension::DUMMY_BACKEND_INFO().clone() });
    v
}

fn makeVarWithStart(mut name: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>, mut var: Variability, mut startExp: Arc<Expression::NFExpression>) -> Arc<Variable::NFVariable> {
    let mut v: Arc<Variable::NFVariable>;
    v = makeVar(name, ty, var);
    assign_field!(v.typeAttributes = list![(literal!("start"), Binding::makeFlat(startExp, Variability::CONSTANT.clone(), Binding::Source::GENERATED.clone(), Binding::NO_CONFIDENCE.clone())), (literal!("fixed"), Binding::makeFlat(Arc::new(Expression::NFExpression::BOOLEAN { value: true }), Variability::CONSTANT.clone(), Binding::Source::GENERATED.clone(), Binding::NO_CONFIDENCE.clone()))]);
    v
}

fn makeVarWithBinding(mut name: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>, mut var: Variability, mut bindExp: Arc<Expression::NFExpression>) -> Arc<Variable::NFVariable> {
    let mut v: Arc<Variable::NFVariable>;
    v = makeVar(name, ty, var);
    assign_field!(v.binding = Binding::makeFlat(bindExp, var, Binding::Source::GENERATED.clone(), Binding::NO_CONFIDENCE.clone()));
    v
}

// ============================================================
// Equation creation helpers
// ============================================================
fn makeEq(mut lhs: Arc<Expression::NFExpression>, mut rhs: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>) -> Arc<Equation::NFEquation> {
    let mut eq: Arc<Equation::NFEquation>;
    eq = Arc::new(Equation::NFEquation::EQUALITY { lhs: lhs, rhs: rhs, ty: ty, scope: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), source: DAE::emptyElementSource().clone(), scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone() });
    eq
}

// ============================================================
// Expression creation helpers
// ============================================================
fn makeCrefExp(mut cref: Arc<ComponentRef::NFComponentRef>, mut ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression>;
    exp = Arc::new(Expression::NFExpression::CREF { ty: ty, cref: cref });
    exp
}

fn makeIfExp(mut cond: Arc<Expression::NFExpression>, mut thenExp: Arc<Expression::NFExpression>, mut elseExp: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    let mut exp: Arc<Expression::NFExpression>;
    exp = Arc::new(Expression::NFExpression::IF { ty: ty, condition: cond, trueBranch: thenExp, falseBranch: elseExp });
    exp
}

fn makePreviousCall(mut exp: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression>;
    result = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::PREVIOUS().clone(), list![exp], Variability::DISCRETE.clone(), Purity::IMPURE.clone(), ty) });
    result
}

fn makeInitialCall() -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression>;
    result = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::INITIAL().clone(), metamodelica::nil(), Variability::DISCRETE.clone(), Purity::IMPURE.clone(), crate::NFType::interned_BOOLEAN()) });
    result
}

fn makeMaxIntArrCall(mut exps: Arc<metamodelica::List<Arc<Expression::NFExpression>>>) -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression>;
    let mut arrTy: Arc<Type::NFType>;
    arrTy = Arc::new(Type::NFType::ARRAY { elementType: crate::NFType::interned_INTEGER(), dimensions: list![Arc::new(Dimension::NFDimension::INTEGER { size: (exps.clone().len() as i32), var: Variability::STRUCTURAL_PARAMETER.clone() })] });
    result = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::MAX_INT_ARR().clone(), list![Arc::new(Expression::NFExpression::ARRAY { ty: arrTy, elements: metamodelica::arrayFromVec(exps.into_iter().cloned().collect()), literal: true })], Variability::DISCRETE.clone(), Purity::PURE.clone(), crate::NFType::interned_INTEGER()) });
    result
}

fn makeSampleTimeCall() -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression>;
    let mut timeExp: Arc<Expression::NFExpression>;
    let mut clockExp: Arc<Expression::NFExpression>;
    let mut ty: Arc<Type::NFType>;
    ty = crate::NFType::interned_REAL();
    timeExp = Arc::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: ComponentRef::prefixCref(Arc::new(InstNode::InstNode::NAME_NODE { name: (literal!("time")).clone() }), ty.clone(), metamodelica::nil(), crate::NFComponentRef::interned_EMPTY()) });
    clockExp = Arc::new(Expression::NFExpression::CLKCONST { clk: Arc::new(NFClockKind::NFClockKind::INFERRED_CLOCK { idx: System::tmpTickIndex(Global::inferredClock_index.clone()) }) });
    result = Arc::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::SAMPLE_CLOCKED().clone(), list![timeExp, clockExp], Variability::CONTINUOUS.clone(), Purity::IMPURE.clone(), ty) });
    result
}

fn makeRelationEq(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression>;
    result = Arc::new(Expression::NFExpression::RELATION { exp1: exp1, operator: Operator::makeEqual(ty), exp2: exp2, index: 0 });
    result
}

fn makeRelationGt(mut exp1: Arc<Expression::NFExpression>, mut exp2: Arc<Expression::NFExpression>, mut ty: Arc<Type::NFType>) -> Arc<Expression::NFExpression> {
    let mut result: Arc<Expression::NFExpression>;
    result = Arc::new(Expression::NFExpression::RELATION { exp1: exp1, operator: Operator::makeGreater(ty), exp2: exp2, index: 0 });
    result
}

// ============================================================
// Start value helpers
// ============================================================
fn getStartValue(mut var: Arc<Variable::NFVariable>) -> Result<Arc<Expression::NFExpression>> {
    let mut startExp: Arc<Expression::NFExpression>;
    let mut attrName: ArcStr;
    let mut attrBinding: Arc<Binding::NFBinding>;
    let mut startOpt: Option<Arc<Expression::NFExpression>>;
    let mut ty: Arc<Type::NFType>;
    for mut attr in &*var.typeAttributes.clone() {
        let mut attr = attr.clone();
        (attrName, attrBinding) = attr.clone();
        if attrName.clone() == literal!("start") {
            startOpt = Binding::typedExp(attrBinding.clone());
            if isSome(startOpt.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(startOpt.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                startExp = __pa0.clone();
                return Ok(startExp.clone());
            }
        }
    }
    ty = var.ty.clone();
    startExp = (::match_deref::match_deref! { match &(ty) {
        Deref @ Type::INTEGER => Arc::new(Expression::NFExpression::INTEGER { value: 0 }),
        Deref @ Type::REAL => Arc::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(0.0_f64) }),
        Deref @ Type::BOOLEAN => Arc::new(Expression::NFExpression::BOOLEAN { value: false }),
        Deref @ Type::STRING => Arc::new(Expression::NFExpression::STRING { value: (literal!("")).clone() }),
        _ => Arc::new(Expression::NFExpression::INTEGER { value: 0 }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(startExp)
}

// ============================================================
// ComponentRef prefix check
// ============================================================
fn crefHasPrefix(mut prefix: Arc<ComponentRef::NFComponentRef>, mut cref: Arc<ComponentRef::NFComponentRef>) -> Result<bool> {
    '__tco: loop {
        if ComponentRef::isEqual(prefix.clone(), cref.clone())? {
            return Ok(true)
        } else if ComponentRef::isEmpty(cref.clone()) {
            return Ok(false)
        } else {
            { (prefix, cref) = (prefix, ComponentRef::rest(cref)?); continue '__tco; }
        }
    }
}

