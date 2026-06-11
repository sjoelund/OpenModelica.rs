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

use crate::HashTableCrToExpOption;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

///
/// Properties of a transition
#[derive(Clone, Debug, Eq, Hash, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Transition {
    pub from: i32,
    pub to: i32,
    pub condition: Arc<DAE::Exp>,
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


///
/// Structure that combines states of flat state machine in
/// canonical order with governing semantic equations.
#[derive(Clone, Debug, Eq, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FlatSmSemantics {
    pub ident: ArcStr,
    /// First element is the initial state
    pub smComps: metamodelica::Array<Arc<DAE::Element>>,
    /// List/Array of transition data sorted in priority
    pub t: Arc<metamodelica::List<Transition>>,
    /// Transition conditions sorted in priority
    pub c: Arc<metamodelica::List<Arc<DAE::Exp>>>,
    /// SMS veriables
    pub vars: Arc<metamodelica::List<Arc<DAE::Element>>>,
    /// SMS constants/parameters
    pub knowns: Arc<metamodelica::List<Arc<DAE::Element>>>,
    /// SMS equations
    pub eqs: Arc<metamodelica::List<Arc<DAE::Element>>>,
    /// Propagation related variables
    pub pvars: Arc<metamodelica::List<Arc<DAE::Element>>>,
    /// Propagation equations
    pub peqs: Arc<metamodelica::List<Arc<DAE::Element>>>,
    /// Cref to enclosing state if any
    pub enclosingState: Option<Arc<DAE::ComponentRef>>,
}

impl metamodelica::gc::MMTrace for FlatSmSemantics {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.ident, __mmv)?;
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
            ident: Default::default(),
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

pub fn stateMachineToDataFlow(mut cache: FCore::Cache, mut env: FCore::Graph, mut inDAElist: DAE::DAElist) -> Result<DAE::DAElist> {
    let mut outDAElist: DAE::DAElist;
    let mut elementLst: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut flatSmLst: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut otherLst: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut elementLst2: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut elementLst3: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut nOfSubstitutions: i32;
    let mut ident: ArcStr;
    let mut dAElist: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut source: Arc<DAE::ElementSource>;
    let mut comment: Option<Arc<SCode::Comment>>;
    let DAE::DAE { elementLst: __pa0 } = (inDAElist.clone()) else { bail!("pattern mismatch") };
    elementLst = __pa0.clone();
    assert!((elementLst.clone().len() as i32) == 1, "{}", &*(literal!("Internal compiler error: Handling of elementLst != 1 not supported\n")).clone());
    let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(listHead(elementLst)?) {
        Deref @ DAE::Element::COMP { ident: __pa1, dAElist: __pa2, source: __pa3, comment: __pa4 } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ident = __pa1.clone();
    dAElist = __pa2.clone();
    source = __pa3.clone();
    comment = __pa4.clone();
    if !(List::any(dAElist.clone(), (std::sync::Arc::new(fnptr!(isFlatSm, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?) {
        outDAElist = inDAElist;
        return Ok(outDAElist.clone());
    }
    (flatSmLst, otherLst) = List::extractOnTrue(dAElist, (std::sync::Arc::new(fnptr!(isFlatSm, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
    elementLst2 = List::fold2(flatSmLst, (std::sync::Arc::new(flatSmToDataFlow) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Option<Arc<DAE::ComponentRef>>, Option<FlatSmSemantics>, Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> + 'static>), None, None, metamodelica::nil())?;
    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
        elementLst2 = wrapHack(cache.clone(), elementLst2)?;
    }
    elementLst3 = listAppend(otherLst, elementLst2);
    outDAElist = DAE::DAElist { elementLst: list![Arc::new(DAE::Element::COMP { ident: (ident).clone(), dAElist: elementLst3, source: source, comment: comment })] };
    let (__pa5, _, (_, __pa6)) = DAEUtil::traverseDAE(outDAElist, FCore::getFunctionTree(cache.clone()), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(traversingSubsActiveState) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, i32) -> Result<(Arc<DAE::Exp>, i32)> + 'static>), 0))?;
    outDAElist = __pa5.clone();
    nOfSubstitutions = __pa6.clone();
    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
        let (__pa7, _, (_, __pa8)) = DAEUtil::traverseDAE(outDAElist, FCore::getFunctionTree(cache), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(traversingSubsPreForPrevious, Arc<DAE::Exp>, i32)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, i32) -> Result<(Arc<DAE::Exp>, i32)> + 'static>), 0))?;
        outDAElist = __pa7.clone();
        nOfSubstitutions = __pa8.clone();
    }
    Ok(outDAElist)
}

fn traversingSubsActiveState(mut inExp: Arc<DAE::Exp>, mut inHitCount: i32) -> Result<(Arc<DAE::Exp>, i32)> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outHitCount: i32;
    (outExp, outHitCount) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "activeState" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => {
            (Arc::new(DAE::Exp::CREF { componentRef: ComponentReference::crefPrependIdent(componentRef.clone(), (literal!("active")).clone(), metamodelica::nil(), DAE::T_BOOL_DEFAULT().clone())?, ty: DAE::T_BOOL_DEFAULT().clone() }), inHitCount + 1)
        },
        _ => {
            (inExp, inHitCount)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outHitCount))
}

fn flatSmToDataFlow(mut inFlatSm: Arc<DAE::Element>, mut inEnclosingStateCrefOption: Option<Arc<DAE::ComponentRef>>, mut inEnclosingFlatSmSemanticsOption: Option<FlatSmSemantics>, mut accElems: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outElems: Arc<metamodelica::List<Arc<DAE::Element>>> = accElems.clone();
    let mut ident: ArcStr;
    let mut dAElist: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut smCompsLst: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut otherLst1: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut transitionLst: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut otherLst2: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut otherLst3: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut eqnLst: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut otherLst4: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut smCompsLst2: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut initialStateOp: Arc<DAE::Element>;
    let mut initialStateComp: Arc<DAE::Element>;
    let mut crefInitialState: Arc<DAE::ComponentRef>;
    let mut flatSmSemanticsBasics: FlatSmSemantics;
    let mut flatSmSemanticsWithPropagation: FlatSmSemantics;
    let mut flatSmSemantics: FlatSmSemantics;
    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut knowns: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut eqs: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut pvars: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut peqs: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inFlatSm) {
        Deref @ DAE::Element::FLAT_SM { ident: __pa0, dAElist: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ident = __pa0.clone();
    dAElist = __pa1.clone();
    (smCompsLst, otherLst1) = List::extractOnTrue(dAElist, (std::sync::Arc::new(fnptr!(isSMComp, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
    (transitionLst, otherLst2) = List::extractOnTrue(otherLst1, (std::sync::Arc::new(fnptr!(isTransition, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(List::extractOnTrue(otherLst2, (std::sync::Arc::new(fnptr!(isInitialState, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?) {
        (Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil }, __pa3) => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    initialStateOp = __pa2.clone();
    otherLst3 = __pa3.clone();
    (eqnLst, otherLst4) = List::extractOnTrue(otherLst3, (std::sync::Arc::new(fnptr!(isEquation, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
    assert!(otherLst4.is_empty(), "{}", &*(literal!("Internal compiler error. Unexpected elements in flat state machine.")).clone());
    let __pa5 = ::match_deref::match_deref! { match &(initialStateOp) {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initialState" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __pa5, .. }, tail: Deref @ metamodelica::List::Nil }, .. }, .. } => __pa5.clone(),
        _ => bail!("pattern mismatch"),
    } };
    crefInitialState = __pa5.clone();
    let (__pa7, __pa8) = ::match_deref::match_deref! { match &(List::extract1OnTrue(smCompsLst.clone(), (std::sync::Arc::new(sMCompEqualsRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), crefInitialState)?) {
        (Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Nil }, __pa8) => (__pa7.clone(), __pa8.clone()),
        _ => bail!("pattern mismatch"),
    } };
    initialStateComp = __pa7.clone();
    smCompsLst2 = __pa8.clone();
    flatSmSemanticsBasics = basicFlatSmSemantics((ident).clone(), metamodelica::cons(initialStateComp, smCompsLst2), transitionLst)?;
    flatSmSemanticsWithPropagation = addPropagationEquations(flatSmSemanticsBasics, inEnclosingStateCrefOption.clone(), inEnclosingFlatSmSemanticsOption)?;
    flatSmSemantics = elabXInStateOps(flatSmSemanticsWithPropagation, inEnclosingStateCrefOption)?;
    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
        smCompsLst = List::map(smCompsLst, (std::sync::Arc::new(elabXInStateOps_CT) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<DAE::Element>> + 'static>))?;
    }
    let FlatSmSemantics { vars: __pa10, knowns: __pa11, eqs: __pa12, pvars: __pa13, peqs: __pa14, .. } = (flatSmSemantics.clone()) else { bail!("pattern mismatch") };
    vars = __pa10.clone();
    knowns = __pa11.clone();
    eqs = __pa12.clone();
    pvars = __pa13.clone();
    peqs = __pa14.clone();
    outElems = List::flatten(list![outElems, eqnLst, vars, knowns, eqs, pvars, peqs])?;
    outElems = List::fold1(smCompsLst, (std::sync::Arc::new(smCompToDataFlow) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, FlatSmSemantics, Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> + 'static>), flatSmSemantics, outElems)?;
    Ok(outElems)
}

fn elabXInStateOps_CT(mut inSmComp: Arc<DAE::Element>) -> Result<Arc<DAE::Element>> {
    let mut outSmComp: Arc<DAE::Element>;
    let mut nOfHits: i32 = 0;
    let mut componentRef: Arc<DAE::ComponentRef>;
    let mut dAElist1: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut dAElist2: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut emptyTree: Arc<AvlTreePathFunction::Tree>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inSmComp) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa0, dAElist: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    componentRef = __pa0.clone();
    dAElist1 = __pa1.clone();
    emptyTree = openmodelica_frontend_dump::AvlTreePathFunction::Tree::interned_EMPTY();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(DAEUtil::traverseDAE(DAE::DAElist { elementLst: dAElist1 }, emptyTree, (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(traversingSubsTicksInState) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, i32)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, i32))> + 'static>), (componentRef.clone(), 0)))?) {
        (DAE::DAElist { elementLst: __pa2 }, _, (_, (_, __pa3))) => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    dAElist2 = __pa2.clone();
    nOfHits = __pa3.clone();
    outSmComp = Arc::new(DAE::Element::SM_COMP { componentRef: componentRef, dAElist: dAElist2 });
    Ok(outSmComp)
}

fn traversingSubsTicksInState(mut inExp: Arc<DAE::Exp>, mut inCref_HitCount: (Arc<DAE::ComponentRef>, i32)) -> Result<(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, i32))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut outCref_HitCount: (Arc<DAE::ComponentRef>, i32);
    let mut cref: Arc<DAE::ComponentRef>;
    let mut hitCount: i32;
    (cref, hitCount) = inCref_HitCount.clone();
    (outExp, outCref_HitCount) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ticksInState" }, expLst: Deref @ metamodelica::List::Nil, attr: Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut crefTicksInState: Arc<DAE::ComponentRef>;
            crefTicksInState = ComponentReference::joinCrefs(cref.clone(), Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("$ticksInState")).clone(), identType: ty.clone(), subscriptLst: metamodelica::nil() }))?;
            (Arc::new(DAE::Exp::CREF { componentRef: crefTicksInState.clone(), ty: ty.clone() }), (cref, hitCount + 1))
        },
        _ => {
            (inExp, inCref_HitCount)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outCref_HitCount))
}

fn elabXInStateOps(mut inFlatSmSemantics: FlatSmSemantics, mut inEnclosingStateCrefOption: Option<Arc<DAE::ComponentRef>>) -> Result<FlatSmSemantics> {
    let mut outFlatSmSemantics: FlatSmSemantics;
    let mut i: i32;
    let mut found: bool;
    let mut c2: Arc<DAE::Exp>;
    let mut c3: Arc<DAE::Exp>;
    let mut c4: Arc<DAE::Exp>;
    let mut substTickExp: Arc<DAE::Exp>;
    let mut substTimeExp: Arc<DAE::Exp>;
    let mut stateRef: Arc<DAE::ComponentRef>;
    let mut t2: Transition;
    let mut tElab: Arc<metamodelica::List<Transition>> = metamodelica::nil();
    let mut cElab: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut smeqsElab: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut ident: ArcStr;
    let mut smComps: metamodelica::Array<Arc<DAE::Element>>;
    let mut t: Arc<metamodelica::List<Transition>>;
    let mut c: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut smvars: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut smknowns: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut smeqs: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut pvars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut peqs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut enclosingStateOption: Option<Arc<DAE::ComponentRef>>;
    let mut from: i32;
    let mut to: i32;
    let mut condition: Arc<DAE::Exp>;
    let mut immediate: bool;
    let mut reset: bool;
    let mut synchronize: bool;
    let mut priority: i32;
    let FlatSmSemantics { ident: __pa0, smComps: __pa1, t: __pa2, c: __pa3, vars: __pa4, knowns: __pa5, eqs: __pa6, pvars: __pa7, peqs: __pa8, enclosingState: __pa9 } = (inFlatSmSemantics) else { bail!("pattern mismatch") };
    ident = __pa0.clone();
    smComps = __pa1.clone();
    t = __pa2.clone();
    c = __pa3.clone();
    smvars = __pa4.clone();
    smknowns = __pa5.clone();
    smeqs = __pa6.clone();
    pvars = __pa7.clone();
    peqs = __pa8.clone();
    enclosingStateOption = __pa9.clone();
    i = 0;
    for mut tc in &*List::zip(t.clone(), c) {
        let mut tc = tc.clone();
        i = i + 1;
        (t2, c2) = tc.clone();
        let Transition { from: __pa10, to: __pa11, condition: __pa12, immediate: __pa13, reset: __pa14, synchronize: __pa15, priority: __pa16 } = (t2.clone()) else { bail!("pattern mismatch") };
        from = __pa10.clone();
        to = __pa11.clone();
        condition = __pa12.clone();
        immediate = __pa13.clone();
        reset = __pa14.clone();
        synchronize = __pa15.clone();
        priority = __pa16.clone();
        let __pa17 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(smComps.clone(), from)?) {
            Deref @ DAE::Element::SM_COMP { componentRef: __pa17, .. } => __pa17.clone(),
            _ => bail!("pattern mismatch"),
        } };
        stateRef = __pa17.clone();
        substTickExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("$ticksInState")).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil(), stateRef.clone())?, ty: DAE::T_INTEGER_DEFAULT().clone() });
        let (__pa18, (_, _, __pa19)) = Expression::traverseExpTopDown(c2.clone(), (std::sync::Arc::new(fnptr!(traversingSubsXInState, Arc<DAE::Exp>, (ArcStr, Arc<DAE::Exp>, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (ArcStr, Arc<DAE::Exp>, bool)) -> Result<(Arc<DAE::Exp>, bool, (ArcStr, Arc<DAE::Exp>, bool))> + 'static>), (literal!("ticksInState"), substTickExp.clone(), false))?;
        c3 = __pa18.clone();
        found = __pa19.clone();
        if found && isSome(inEnclosingStateCrefOption.clone()) {
            Error::addCompilerError((literal!("Found 'ticksInState()' within a state of an hierarchical state machine.")).clone())?;
            bail!("fail");
        }
        smeqsElab = if (found) {List::map5(smeqs.clone(), (std::sync::Arc::new(smeqsSubsXInState) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::Element>, i32, i32, Arc<DAE::Exp>, ArcStr) -> Result<Arc<DAE::Element>> + 'static>), metamodelica::arrayGet(smComps.clone(), 1)?, i, (t.clone().len() as i32), substTickExp.clone(), (literal!("ticksInState")).clone())?} else {smeqs.clone()};
        smeqs = smeqsElab.clone();
        substTimeExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("$timeInState")).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil(), stateRef.clone())?, ty: DAE::T_REAL_DEFAULT().clone() });
        let (__pa20, (_, _, __pa21)) = Expression::traverseExpTopDown(c2.clone(), (std::sync::Arc::new(fnptr!(traversingSubsXInState, Arc<DAE::Exp>, (ArcStr, Arc<DAE::Exp>, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (ArcStr, Arc<DAE::Exp>, bool)) -> Result<(Arc<DAE::Exp>, bool, (ArcStr, Arc<DAE::Exp>, bool))> + 'static>), (literal!("timeInState"), substTimeExp.clone(), false))?;
        c4 = __pa20.clone();
        found = __pa21.clone();
        if found && isSome(inEnclosingStateCrefOption.clone()) {
            Error::addCompilerError((literal!("Found 'timeInState()' within a state of an hierarchical state machine.")).clone())?;
            bail!("fail");
        }
        smeqsElab = if (found) {List::map5(smeqs.clone(), (std::sync::Arc::new(smeqsSubsXInState) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::Element>, i32, i32, Arc<DAE::Exp>, ArcStr) -> Result<Arc<DAE::Element>> + 'static>), metamodelica::arrayGet(smComps.clone(), 1)?, i, (t.clone().len() as i32), substTimeExp.clone(), (literal!("timeInState")).clone())?} else {smeqs.clone()};
        smeqs = smeqsElab.clone();
        tElab = metamodelica::cons(Transition { from: from, to: to, condition: c4.clone(), immediate: immediate, reset: reset, synchronize: synchronize, priority: priority }, tElab.clone());
        cElab = metamodelica::cons(c4.clone(), cElab.clone());
    }
    outFlatSmSemantics = FlatSmSemantics { ident: (ident).clone(), smComps: smComps.clone(), t: tElab.reverse(), c: cElab.reverse(), vars: smvars, knowns: smknowns, eqs: smeqsElab, pvars: pvars, peqs: peqs, enclosingState: enclosingStateOption };
    Ok(outFlatSmSemantics)
}

fn smeqsSubsXInState(mut inSmeqs: Arc<DAE::Element>, mut initialStateComp: Arc<DAE::Element>, mut i: i32, mut nTransitions: i32, mut substExp: Arc<DAE::Exp>, mut xInState: ArcStr) -> Result<Arc<DAE::Element>> {
    let mut outSmeqs: Arc<DAE::Element>;
    let mut preRef: Arc<DAE::ComponentRef>;
    let mut cref: Arc<DAE::ComponentRef>;
    let mut lhsRef: Arc<DAE::ComponentRef>;
    let mut crefInitialState: Arc<DAE::ComponentRef>;
    let mut tArrayBool: Arc<DAE::Type>;
    let mut elemSource: Arc<DAE::ElementSource>;
    let mut lhsExp: Arc<DAE::Exp>;
    let mut rhsExp: Arc<DAE::Exp>;
    let mut rhsExp2: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type>;
    let __pa0 = ::match_deref::match_deref! { match &(initialStateComp) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    crefInitialState = __pa0.clone();
    preRef = ComponentReference::crefPrefixString((arcstr::literal!(SMS_PRE)).clone(), crefInitialState);
    tArrayBool = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nTransitions })] });
    cref = qCref((literal!("cImmediate")).clone(), tArrayBool, list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i }) })], preRef)?;
    let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inSmeqs) {
        Deref @ DAE::Element::EQUATION { exp: __pa1, scalar: __pa2, source: __pa3 } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    lhsExp = __pa1.clone();
    rhsExp = __pa2.clone();
    elemSource = __pa3.clone();
    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(lhsExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: __pa4, ty: __pa5 } => (__pa4.clone(), __pa5.clone()),
        _ => bail!("pattern mismatch"),
    } };
    lhsRef = __pa4.clone();
    ty = __pa5.clone();
    if ComponentReferenceBasics::crefEqual(cref, lhsRef)? {
        (rhsExp2, _) = Expression::traverseExpTopDown(rhsExp, (std::sync::Arc::new(fnptr!(traversingSubsXInState, Arc<DAE::Exp>, (ArcStr, Arc<DAE::Exp>, bool))) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (ArcStr, Arc<DAE::Exp>, bool)) -> Result<(Arc<DAE::Exp>, bool, (ArcStr, Arc<DAE::Exp>, bool))> + 'static>), (xInState, substExp, false))?;
    } else {
        rhsExp2 = rhsExp;
    }
    outSmeqs = Arc::new(DAE::Element::EQUATION { exp: lhsExp, scalar: rhsExp2, source: elemSource });
    Ok(outSmeqs)
}

fn traversingSubsXInState(mut inExp: Arc<DAE::Exp>, mut inXSubstHit: (ArcStr, Arc<DAE::Exp>, bool)) -> (Arc<DAE::Exp>, bool, (ArcStr, Arc<DAE::Exp>, bool)) {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = true;
    let mut outXSubstHit: (ArcStr, Arc<DAE::Exp>, bool);
    (outExp, outXSubstHit) = (::match_deref::match_deref! { match &((inExp.clone(), inXSubstHit.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, .. }, (xInState, subsExp, _)) if (name.clone() == xInState.clone()) => {
            (subsExp.clone(), (xInState.clone(), subsExp.clone(), true))
        },
        _ => {
            (inExp, inXSubstHit)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outXSubstHit)
}

fn smCompToDataFlow(mut inSMComp: Arc<DAE::Element>, mut inEnclosingFlatSmSemantics: FlatSmSemantics, mut accElems: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outElems: Arc<metamodelica::List<Arc<DAE::Element>>> = accElems.clone();
    let mut varLst1: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut varLst2: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut assignedVarLst: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut stateVarLst: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut otherLst1: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut equationLst1: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut equationLst2: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut otherLst2: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut flatSmLst: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut otherLst3: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut componentRef: Arc<DAE::ComponentRef>;
    let mut stateVarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut startValuesOpt: Arc<metamodelica::List<Option<Arc<DAE::Exp>>>>;
    let mut varCrefStartVal: Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>)>>;
    let mut dAElist: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut crToExpOpt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>)>>), i32, (HashTableCrToExpOption::FuncHashCref, HashTableCrToExpOption::FuncCrefEqual, HashTableCrToExpOption::FuncCrefStr, HashTableCrToExpOption::FuncExpStr));
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inSMComp.clone()) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa0, dAElist: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    componentRef = __pa0.clone();
    dAElist = __pa1.clone();
    (varLst1, otherLst1) = List::extractOnTrue(dAElist, (std::sync::Arc::new(fnptr!(isVar, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
    (equationLst1, otherLst2) = List::extractOnTrue(otherLst1, (std::sync::Arc::new(fnptr!(isEquationOrWhenEquation, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
    assignedVarLst = List::filterOnTrue(varLst1.clone(), (std::sync::Arc::new({ let __pe_b0 = equationLst1.clone(); let __pe_b1: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(isVarAtLHS) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::Element>) -> Result<bool> + 'static>); move |__pe_a2| List::exist1(__pe_b0.clone(), __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static>))?;
    stateVarLst = List::filterOnTrue(varLst1.clone(), (std::sync::Arc::new({ let __pe_b0 = equationLst1.clone(); let __pe_b1: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(isPreviousAppliedToVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::Element>) -> Result<bool> + 'static>); move |__pe_a2| List::exist1(__pe_b0.clone(), __pe_b1.clone(), __pe_a2) }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static>))?;
    stateVarCrefs = List::map(stateVarLst.clone(), (std::sync::Arc::new(DAEUtil::varCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Arc<DAE::ComponentRef>> + 'static>))?;
    startValuesOpt = List::map(stateVarLst, (std::sync::Arc::new(getStartAttrOption) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<Option<Arc<DAE::Exp>>> + 'static>))?;
    varCrefStartVal = List::zip(stateVarCrefs, startValuesOpt);
    crToExpOpt = HashTableCrToExpOption::emptyHashTableSized((varCrefStartVal.clone().len() as i32) + 1);
    crToExpOpt = List::fold(varCrefStartVal, (std::sync::Arc::new(BaseHashTable::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), crToExpOpt)?;
    (equationLst2, varLst2) = List::fold3(equationLst1, (std::sync::Arc::new(addStateActivationAndReset) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::Element>, FlatSmSemantics, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Option<Arc<DAE::Exp>>) -> Result<ArcStr> + 'static>)), (Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)> + 'static>), inSMComp, inEnclosingFlatSmSemantics.clone(), crToExpOpt, (metamodelica::nil(), metamodelica::nil()))?;
    (flatSmLst, otherLst3) = List::extractOnTrue(otherLst2, (std::sync::Arc::new(fnptr!(isFlatSm, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
    outElems = List::flatten(list![outElems, varLst1, varLst2, equationLst2, otherLst3])?;
    outElems = List::fold2(flatSmLst, (std::sync::Arc::new(flatSmToDataFlow) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Option<Arc<DAE::ComponentRef>>, Option<FlatSmSemantics>, Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> + 'static>), Some(componentRef), Some(inEnclosingFlatSmSemantics), outElems)?;
    Ok(outElems)
}

fn addStateActivationAndReset(mut inEqn: Arc<DAE::Element>, mut inEnclosingSMComp: Arc<DAE::Element>, mut inEnclosingFlatSmSemantics: FlatSmSemantics, mut crToExpOpt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Option<Arc<DAE::Exp>>) -> Result<ArcStr> + 'static>)), mut accEqnsVars: (Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut outEqnsVars: (Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>);
    let mut equations1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut vars1: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut condition: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut equations: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut source: Arc<DAE::ElementSource> = Arc::new(<DAE::ElementSource as ::std::default::Default>::default());
    outEqnsVars = (::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ DAE::Element::EQUATION { .. } => addStateActivationAndReset1(inEqn, inEnclosingSMComp, inEnclosingFlatSmSemantics, crToExpOpt, accEqnsVars)?,
        Deref @ DAE::Element::WHEN_EQUATION { condition: __esc_condition, equations: __esc_equations, elsewhen_: None, source: __esc_source } => {
            condition = (*__esc_condition).clone();
            equations = (*__esc_equations).clone();
            source = (*__esc_source).clone();
            (equations1, vars1) = List::fold3(equations.clone(), (std::sync::Arc::new(addStateActivationAndReset) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::Element>, FlatSmSemantics, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Option<Arc<DAE::Exp>>) -> Result<ArcStr> + 'static>)), (Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)> + 'static>), inEnclosingSMComp, inEnclosingFlatSmSemantics, crToExpOpt, (metamodelica::nil(), metamodelica::nil()))?;
            (metamodelica::cons(Arc::new(DAE::Element::WHEN_EQUATION { condition: condition.clone(), equations: equations1, elsewhen_: None, source: source.clone() }), Util::tuple21(accEqnsVars.clone())), listAppend(vars1, Util::tuple22(accEqnsVars)))
        },
        Deref @ DAE::Element::WHEN_EQUATION { elsewhen_: Some(_), .. } => {
            Error::addCompilerError((literal!("Encountered elsewhen part in a when clause of a clocked state machine.\n")).clone())?;
            bail!("fail")
        },
        _ => {
            Error::addCompilerError((literal!("Internal compiler error: StateMachineFlatten.addStateActivationAndReset(..) called with unexpected argument.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqnsVars)
}

fn addStateActivationAndReset1(mut inEqn: Arc<DAE::Element>, mut inEnclosingSMComp: Arc<DAE::Element>, mut inEnclosingFlatSmSemantics: FlatSmSemantics, mut crToExpOpt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Option<Arc<DAE::Exp>>) -> Result<ArcStr> + 'static>)), mut accEqnsVars: (Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)) -> Result<(Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>)> {
    let mut outEqnsVars: (Arc<metamodelica::List<Arc<DAE::Element>>>, Arc<metamodelica::List<Arc<DAE::Element>>>);
    let mut stateVarCrefs: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>;
    let mut crefLHS: Arc<DAE::ComponentRef>;
    let mut enclosingStateRef: Arc<DAE::ComponentRef>;
    let mut cref2: Arc<DAE::ComponentRef>;
    let mut found: bool;
    let mut tyLHS: Arc<DAE::Type>;
    let mut eqn: Arc<DAE::Element>;
    let mut eqn1: Arc<DAE::Element>;
    let mut eqn2: Arc<DAE::Element>;
    let mut var2: Arc<DAE::Element>;
    let mut varDecl: Arc<DAE::Element>;
    let mut attr: Arc<DAE::CallAttributes>;
    let mut dAElist: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut isOuterVar: bool;
    let mut exp: Arc<DAE::Exp>;
    let mut scalar: Arc<DAE::Exp>;
    let mut scalarNew: Arc<DAE::Exp>;
    let mut source: Arc<DAE::ElementSource>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inEqn.clone()) {
        Deref @ DAE::Element::EQUATION { exp: __pa0, scalar: __pa1, source: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp = __pa0.clone();
    scalar = __pa1.clone();
    source = __pa2.clone();
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(inEnclosingSMComp) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa3, dAElist: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    enclosingStateRef = __pa3.clone();
    dAElist = __pa4.clone();
    stateVarCrefs = BaseHashTable::hashTableKeyList(crToExpOpt.clone())?;
    match '__try5: {
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ DAE::Exp::CREF { componentRef: __pa6, ty: __pa7 } => (__pa6.clone(), __pa7.clone()),
            _ => break '__try5 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        crefLHS = __pa6.clone();
        tyLHS = __pa7.clone();
        let (__pa8, (_, __pa9)) = unwrap_break_err!(Expression::traverseExpTopDown(scalar.clone(), (std::sync::Arc::new(traversingSubsPreviousCrefs) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, bool))> + 'static>), (stateVarCrefs.clone(), false)), '__try5);
        scalarNew = __pa8.clone();
        found = __pa9.clone();
        eqn = Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: scalarNew.clone(), source: source.clone() });
        if unwrap_break_err!(List::any(stateVarCrefs.clone(), (std::sync::Arc::new({ let __pe_b0 = crefLHS.clone(); move |__pe_a1| ComponentReferenceBasics::crefEqual(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<bool> + 'static>)), '__try5) {
            eqn1 = unwrap_break_err!(wrapInStateActivationConditional(eqn.clone(), enclosingStateRef.clone(), true), '__try5);
            var2 = createVarWithDefaults(unwrap_break_err!(ComponentReference::appendStringLastIdent((literal!("_previous")).clone(), crefLHS.clone()), '__try5), openmodelica_frontend_types::DAE::VarKind::DISCRETE, tyLHS.clone(), metamodelica::nil());
            eqn2 = unwrap_break_err!(createResetEquation(crefLHS.clone(), tyLHS.clone(), enclosingStateRef.clone(), inEnclosingFlatSmSemantics.clone(), crToExpOpt.clone()), '__try5);
            outEqnsVars = (metamodelica::cons(eqn1.clone(), metamodelica::cons(eqn2.clone(), Util::tuple21(accEqnsVars.clone()))), metamodelica::cons(var2.clone(), Util::tuple22(accEqnsVars.clone())));
        } else {
            outEqnsVars = (metamodelica::cons(unwrap_break_err!(wrapInStateActivationConditional(eqn.clone(), enclosingStateRef.clone(), false), '__try5), Util::tuple21(accEqnsVars.clone())), Util::tuple22(accEqnsVars.clone()));
        }
        Ok::<_, anyhow::Error>((crefLHS.clone(), outEqnsVars.clone(), tyLHS.clone()))
    } {
        Ok((__try5_o0, __try5_o1, __try5_o2)) => {
            crefLHS = __try5_o0;
            outEqnsVars = __try5_o1;
            tyLHS = __try5_o2;
        }
        Err(_) => {
            match '__try10: {
                if unwrap_break_err!(Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone()), '__try10) {
                    let (__pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(exp.clone()) {
                        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __pa11, ty: __pa12 }, tail: Deref @ metamodelica::List::Nil }, attr: __pa13 } => (__pa11.clone(), __pa12.clone(), __pa13.clone()),
                        _ => break '__try10 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                    } };
                    crefLHS = __pa11.clone();
                    tyLHS = __pa12.clone();
                    attr = __pa13.clone();
                    if let Ok(__iflet16) = List::find1(dAElist.clone(), (std::sync::Arc::new(isCrefInVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), crefLHS.clone()) {
                        varDecl = __iflet16;
                    } else {
                        unwrap_break_err!(Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Couldn't find variable declaration matching to cref ")); __mm_s.push_str(&*unwrap_break_err!(ComponentReference::crefStr(crefLHS.clone()), '__try10)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone()), '__try10);
                        break '__try10 Err::<_, _>(anyhow::anyhow!("fail"));
                    }
                    isOuterVar = DAEUtil::isOuterVar(varDecl.clone());
                    if isOuterVar {
                        cref2 = unwrap_break_err!(ComponentReference::appendStringLastIdent((literal!("_der$")).clone(), crefLHS.clone()), '__try10);
                        var2 = createVarWithDefaults(cref2.clone(), openmodelica_frontend_types::DAE::VarKind::VARIABLE, tyLHS.clone(), metamodelica::nil());
                        eqn1 = Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: cref2.clone(), ty: tyLHS.clone() }), scalar: scalar.clone(), source: source.clone() });
                        outEqnsVars = (metamodelica::cons(eqn1.clone(), Util::tuple21(accEqnsVars.clone())), metamodelica::cons(var2.clone(), Util::tuple22(accEqnsVars.clone())));
                    } else {
                        eqn1 = unwrap_break_err!(wrapInStateActivationConditionalCT(inEqn.clone(), enclosingStateRef.clone()), '__try10);
                        eqn2 = unwrap_break_err!(createResetEquationCT(crefLHS.clone(), tyLHS.clone(), enclosingStateRef.clone(), inEnclosingFlatSmSemantics.clone(), crToExpOpt.clone()), '__try10);
                        outEqnsVars = (metamodelica::cons(eqn1.clone(), metamodelica::cons(eqn2.clone(), Util::tuple21(accEqnsVars.clone()))), Util::tuple22(accEqnsVars.clone()));
                    }
                } else {
                    break '__try10 Err::<_, _>(anyhow::anyhow!("fail"));
                }
                Ok::<_, anyhow::Error>((attr.clone(), crefLHS.clone(), eqn1.clone(), isOuterVar.clone(), outEqnsVars.clone(), tyLHS.clone(), varDecl.clone()))
            } {
                Ok((__try10_o0, __try10_o1, __try10_o2, __try10_o3, __try10_o4, __try10_o5, __try10_o6)) => {
                    attr = __try10_o0;
                    crefLHS = __try10_o1;
                    eqn1 = __try10_o2;
                    isOuterVar = __try10_o3;
                    outEqnsVars = __try10_o4;
                    tyLHS = __try10_o5;
                    varDecl = __try10_o6;
                }
                Err(__try10_err) => {
                    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
                        Error::addCompilerError((literal!("Currently, only equations in state machines with a LHS component reference, e.g., x=.., or its derivative, e.g., der(x)=.., are supported")).clone())?;
                    } else {
                        Error::addCompilerError((literal!("Currently, only equations in state machines with a LHS component reference, e.g., x=.., are supported")).clone())?;
                    }
                    return Err(__try10_err);
                }
            }
        }
    }
    Ok(outEqnsVars)
}

fn isVarAtLHS(mut eqn: Arc<DAE::Element>, mut var: Arc<DAE::Element>) -> Result<bool> {
    let mut res: bool = false;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut crefLHS: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut tyLHS: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut exp: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut equations: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut elsewhen_: Option<Arc<DAE::Element>>;
    res = (::match_deref::match_deref! { match &(eqn) {
        Deref @ DAE::Element::EQUATION { exp: __esc_exp, scalar: _, source: _ } => {
            exp = (*__esc_exp).clone();
            cref = DAEUtil::varCref(var)?;
            match '__try0: {
                let (__pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
                    Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: __pa2 } => (__pa1.clone(), __pa2.clone()),
                    _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                crefLHS = __pa1.clone();
                tyLHS = __pa2.clone();
                res = unwrap_break_err!(ComponentReferenceBasics::crefEqual(crefLHS.clone(), cref.clone()), '__try0);
                Ok::<_, anyhow::Error>((res.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    res = __try0_o0;
                }
                Err(_) => {
                    res = false;
                }
            }
            res
        },
        Deref @ DAE::Element::WHEN_EQUATION { equations: __esc_equations, elsewhen_: None, .. } => {
            equations = (*__esc_equations).clone();
            List::exist1(equations.clone(), (std::sync::Arc::new(isVarAtLHS) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::Element>) -> Result<bool> + 'static>), var)?
        },
        Deref @ DAE::Element::WHEN_EQUATION { elsewhen_: Some(_), .. } => {
            Error::addCompilerError((literal!("Encountered elsewhen part in a when clause of a clocked state machine.\n")).clone())?;
            bail!("fail")
        },
        _ => {
            Error::addCompilerError((literal!("Internal compiler error: StateMachineFlatten.isVarAtLHS(..) called with unexpected argument.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn isPreviousAppliedToVar(mut eqn: Arc<DAE::Element>, mut var: Arc<DAE::Element>) -> Result<bool> {
    let mut found: bool = false;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    let mut scalar: Arc<DAE::Exp> = Arc::new(<DAE::Exp as ::std::default::Default>::default());
    let mut equations: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut elsewhen_: Option<Arc<DAE::Element>>;
    found = (::match_deref::match_deref! { match &(eqn) {
        Deref @ DAE::Element::EQUATION { exp: _, scalar: __esc_scalar, source: _ } => {
            scalar = (*__esc_scalar).clone();
            cref = DAEUtil::varCref(var)?;
            let (_, (_, __pa0)) = Expression::traverseExpTopDown(scalar.clone(), (std::sync::Arc::new(traversingFindPreviousCref) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool))> + 'static>), (cref, false))?;
            found = __pa0.clone();
            found
        },
        Deref @ DAE::Element::WHEN_EQUATION { equations: __esc_equations, elsewhen_: None, .. } => {
            equations = (*__esc_equations).clone();
            List::exist1(equations.clone(), (std::sync::Arc::new(isPreviousAppliedToVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::Element>) -> Result<bool> + 'static>), var)?
        },
        Deref @ DAE::Element::WHEN_EQUATION { elsewhen_: Some(_), .. } => {
            Error::addCompilerError((literal!("Encountered elsewhen part in a when clause of a clocked state machine.\n")).clone())?;
            bail!("fail")
        },
        _ => {
            Error::addCompilerError((literal!("Internal compiler error: StateMachineFlatten.isPreviousAppliedToVar(..) called with unexpected argument.\n")).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(found)
}

fn traversingFindPreviousCref(mut inExp: Arc<DAE::Exp>, mut inCrefHit: (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = true;
    let mut outCrefHit: (Arc<DAE::ComponentRef>, bool);
    (outExp, outCrefHit) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefHit.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, tail: Deref @ metamodelica::List::Nil }, attr: _ }, (cref, _)) if (ComponentReferenceBasics::crefEqual(cr.clone(), cref.clone())?) => {
            (inExp, (cref.clone(), true))
        },
        _ => {
            (inExp, inCrefHit)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outCrefHit))
}

fn createResetEquationCT(mut inLHSCref: Arc<DAE::ComponentRef>, mut inLHSty: Arc<DAE::Type>, mut inStateCref: Arc<DAE::ComponentRef>, mut inEnclosingFlatSmSemantics: FlatSmSemantics, mut crToExpOpt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Option<Arc<DAE::Exp>>) -> Result<ArcStr> + 'static>))) -> Result<Arc<DAE::Element>> {
    let mut outEqn: Arc<DAE::Element>;
    let mut activeExp: Arc<DAE::Exp>;
    let mut activeResetExp: Arc<DAE::Exp>;
    let mut activeResetStatesExp: Arc<DAE::Exp>;
    let mut orExp: Arc<DAE::Exp>;
    let mut andExp: Arc<DAE::Exp>;
    let mut startValueExp: Arc<DAE::Exp>;
    let mut reinitElem: Arc<DAE::Element>;
    let mut startValueOpt: Option<Arc<DAE::Exp>>;
    let mut initStateRef: Arc<DAE::ComponentRef>;
    let mut preRef: Arc<DAE::ComponentRef>;
    let mut i: i32;
    let mut nStates: i32;
    let mut enclosingFlatSMComps: metamodelica::Array<Arc<DAE::Element>>;
    let mut tArrayBool: Arc<DAE::Type>;
    let FlatSmSemantics { smComps: __pa0, .. } = (inEnclosingFlatSmSemantics) else { bail!("pattern mismatch") };
    enclosingFlatSMComps = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(enclosingFlatSMComps.clone(), 1)?) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    initStateRef = __pa1.clone();
    preRef = ComponentReference::crefPrefixString((arcstr::literal!(SMS_PRE)).clone(), initStateRef);
    i = List::position1OnTrue(Arc::new(enclosingFlatSMComps.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), (std::sync::Arc::new(sMCompEqualsRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), inStateCref.clone())?;
    activeResetExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("activeReset")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?, ty: DAE::T_BOOL_DEFAULT().clone() });
    nStates = metamodelica::arrayLength(enclosingFlatSMComps.clone());
    tArrayBool = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nStates })] });
    activeResetStatesExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("activeResetStates")).clone(), tArrayBool, list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i }) })], preRef)?, ty: DAE::T_BOOL_DEFAULT().clone() });
    orExp = Arc::new(DAE::Exp::LBINARY { exp1: activeResetExp, operator: DAE::Operator::OR { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: activeResetStatesExp });
    activeExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), inStateCref)?, ty: DAE::T_BOOL_DEFAULT().clone() });
    andExp = Arc::new(DAE::Exp::LBINARY { exp1: activeExp, operator: DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: orExp });
    startValueOpt = BaseHashTable::get(inLHSCref.clone(), crToExpOpt)?;
    if isSome(startValueOpt.clone()) {
        startValueExp = Util::getOption(startValueOpt)?;
    } else {
        startValueExp = (::match_deref::match_deref! { match &(inLHSty.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=0.\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::ICONST { integer: 0 })
        },
        Deref @ DAE::Type::T_REAL { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=0.\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) })
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=false.\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::BCONST { bool: false })
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=\"\".\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() })
        },
        Deref @ DAE::Type::T_ENUMERATION { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=\"\".\n")); ArcStr::from(__mm_s) }).clone())?;
            Types::getNthEnumLiteral(inLHSty, 1)?
        },
        _ => {
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value.\n")); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    reinitElem = Arc::new(DAE::Element::REINIT { componentRef: inLHSCref, exp: startValueExp, source: DAE::emptyElementSource().clone() });
    outEqn = Arc::new(DAE::Element::WHEN_EQUATION { condition: andExp, equations: list![reinitElem], elsewhen_: None, source: DAE::emptyElementSource().clone() });
    Ok(outEqn)
}

fn isCrefInVar(mut inElement: Arc<DAE::Element>, mut inCref: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut result: bool;
    result = (::match_deref::match_deref! { match &(inElement) {
        Deref @ DAE::Element::VAR { componentRef: cref, .. } if (ComponentReferenceBasics::crefEqual(cref.clone(), inCref.clone())?) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn createResetEquation(mut inLHSCref: Arc<DAE::ComponentRef>, mut inLHSty: Arc<DAE::Type>, mut inStateCref: Arc<DAE::ComponentRef>, mut inEnclosingFlatSmSemantics: FlatSmSemantics, mut crToExpOpt: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Option<Arc<DAE::Exp>>)>>), i32, (Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Option<Arc<DAE::Exp>>) -> Result<ArcStr> + 'static>))) -> Result<Arc<DAE::Element>> {
    let mut outEqn: Arc<DAE::Element>;
    let mut activeExp: Arc<DAE::Exp>;
    let mut lhsExp: Arc<DAE::Exp>;
    let mut activeResetExp: Arc<DAE::Exp>;
    let mut activeResetStatesExp: Arc<DAE::Exp>;
    let mut orExp: Arc<DAE::Exp>;
    let mut andExp: Arc<DAE::Exp>;
    let mut previousExp: Arc<DAE::Exp>;
    let mut startValueExp: Arc<DAE::Exp>;
    let mut ifExp: Arc<DAE::Exp>;
    let mut startValueOpt: Option<Arc<DAE::Exp>>;
    let mut initStateRef: Arc<DAE::ComponentRef>;
    let mut preRef: Arc<DAE::ComponentRef>;
    let mut i: i32;
    let mut nStates: i32;
    let mut enclosingFlatSMComps: metamodelica::Array<Arc<DAE::Element>>;
    let mut tArrayBool: Arc<DAE::Type>;
    let mut callAttributes: Arc<DAE::CallAttributes>;
    let FlatSmSemantics { smComps: __pa0, .. } = (inEnclosingFlatSmSemantics) else { bail!("pattern mismatch") };
    enclosingFlatSMComps = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(enclosingFlatSMComps.clone(), 1)?) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    initStateRef = __pa1.clone();
    preRef = ComponentReference::crefPrefixString((arcstr::literal!(SMS_PRE)).clone(), initStateRef);
    i = List::position1OnTrue(Arc::new(enclosingFlatSMComps.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), (std::sync::Arc::new(sMCompEqualsRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), inStateCref.clone())?;
    activeResetExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("activeReset")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?, ty: DAE::T_BOOL_DEFAULT().clone() });
    nStates = metamodelica::arrayLength(enclosingFlatSMComps.clone());
    tArrayBool = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nStates })] });
    activeResetStatesExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("activeResetStates")).clone(), tArrayBool, list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i }) })], preRef)?, ty: DAE::T_BOOL_DEFAULT().clone() });
    orExp = Arc::new(DAE::Exp::LBINARY { exp1: activeResetExp, operator: DAE::Operator::OR { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: activeResetStatesExp });
    activeExp = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), inStateCref)?, ty: DAE::T_BOOL_DEFAULT().clone() });
    andExp = Arc::new(DAE::Exp::LBINARY { exp1: activeExp, operator: DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: orExp });
    callAttributes = Arc::new(DAE::CallAttributes { ty: inLHSty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL });
    previousExp = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: inLHSCref.clone(), ty: inLHSty.clone() })], attr: callAttributes });
    startValueOpt = BaseHashTable::get(inLHSCref.clone(), crToExpOpt)?;
    if isSome(startValueOpt.clone()) {
        startValueExp = Util::getOption(startValueOpt)?;
    } else {
        startValueExp = (::match_deref::match_deref! { match &(inLHSty.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=0.\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::ICONST { integer: 0 })
        },
        Deref @ DAE::Type::T_REAL { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=0.\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) })
        },
        Deref @ DAE::Type::T_BOOL { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=false.\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::BCONST { bool: false })
        },
        Deref @ DAE::Type::T_STRING { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=\"\".\n")); ArcStr::from(__mm_s) }).clone())?;
            Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() })
        },
        Deref @ DAE::Type::T_ENUMERATION { .. } => {
            Error::addCompilerWarning(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=\"\".\n")); ArcStr::from(__mm_s) }).clone())?;
            Types::getNthEnumLiteral(inLHSty.clone(), 1)?
        },
        _ => {
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable ")); __mm_s.push_str(&*ComponentReference::crefStr(inLHSCref.clone())?); __mm_s.push_str(&*literal!(" lacks start value.\n")); ArcStr::from(__mm_s) }).clone())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    ifExp = Arc::new(DAE::Exp::IFEXP { expCond: andExp, expThen: startValueExp, expElse: previousExp });
    lhsExp = Arc::new(DAE::Exp::CREF { componentRef: ComponentReference::appendStringLastIdent((literal!("_previous")).clone(), inLHSCref)?, ty: inLHSty });
    outEqn = Arc::new(DAE::Element::EQUATION { exp: lhsExp, scalar: ifExp, source: DAE::emptyElementSource().clone() });
    Ok(outEqn)
}

fn wrapInStateActivationConditional(mut inEqn: Arc<DAE::Element>, mut inStateCref: Arc<DAE::ComponentRef>, mut isResetEquation: bool) -> Result<Arc<DAE::Element>> {
    let mut outEqn: Arc<DAE::Element>;
    let mut exp: Arc<DAE::Exp>;
    let mut scalar: Arc<DAE::Exp>;
    let mut scalar1: Arc<DAE::Exp>;
    let mut activeRef: Arc<DAE::Exp>;
    let mut expElse: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type>;
    let mut callAttributes: Arc<DAE::CallAttributes>;
    let mut source: Arc<DAE::ElementSource>;
    let mut cref: Arc<DAE::ComponentRef>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inEqn) {
        Deref @ DAE::Element::EQUATION { exp: __pa0, scalar: __pa1, source: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp = __pa0.clone();
    scalar = __pa1.clone();
    source = __pa2.clone();
    match '__try3: {
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ DAE::Exp::CREF { componentRef: __pa4, ty: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => break '__try3 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        cref = __pa4.clone();
        ty = __pa5.clone();
        Ok::<_, anyhow::Error>((cref.clone(), ty.clone()))
    } {
        Ok((__try3_o0, __try3_o1)) => {
            cref = __try3_o0;
            ty = __try3_o1;
        }
        Err(__try3_err) => {
            Error::addCompilerError((literal!("The LHS of equations in state machines needs to be a component reference")).clone())?;
            return Err(__try3_err);
        }
    }
    activeRef = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), inStateCref)?, ty: DAE::T_BOOL_DEFAULT().clone() });
    callAttributes = Arc::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL });
    if isResetEquation {
        expElse = Arc::new(DAE::Exp::CREF { componentRef: ComponentReference::appendStringLastIdent((literal!("_previous")).clone(), cref)?, ty: ty });
    } else {
        expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![exp.clone()], attr: callAttributes });
    }
    scalar1 = Arc::new(DAE::Exp::IFEXP { expCond: activeRef, expThen: scalar, expElse: expElse });
    outEqn = Arc::new(DAE::Element::EQUATION { exp: exp, scalar: scalar1, source: source });
    Ok(outEqn)
}

fn wrapInStateActivationConditionalCT(mut inEqn: Arc<DAE::Element>, mut inStateCref: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::Element>> {
    let mut outEqn: Arc<DAE::Element>;
    let mut exp: Arc<DAE::Exp>;
    let mut scalar: Arc<DAE::Exp>;
    let mut scalar1: Arc<DAE::Exp>;
    let mut activeRef: Arc<DAE::Exp>;
    let mut expElse: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type>;
    let mut callAttributes: Arc<DAE::CallAttributes>;
    let mut source: Arc<DAE::ElementSource>;
    let mut cref: Arc<DAE::ComponentRef>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inEqn) {
        Deref @ DAE::Element::EQUATION { exp: __pa0, scalar: __pa1, source: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    exp = __pa0.clone();
    scalar = __pa1.clone();
    source = __pa2.clone();
    match '__try3: {
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __pa4, ty: __pa5 }, tail: Deref @ metamodelica::List::Nil }, attr: _ } => (__pa4.clone(), __pa5.clone()),
            _ => break '__try3 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        cref = __pa4.clone();
        ty = __pa5.clone();
        Ok::<_, anyhow::Error>((cref.clone(), ty.clone()))
    } {
        Ok((__try3_o0, __try3_o1)) => {
            cref = __try3_o0;
            ty = __try3_o1;
        }
        Err(__try3_err) => {
            Error::addCompilerError((literal!("The LHS of equations in state machines needs to be a component reference, e.g., x = .., or its derivative, e.g., der(x) = ..")).clone())?;
            return Err(__try3_err);
        }
    }
    activeRef = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), inStateCref)?, ty: DAE::T_BOOL_DEFAULT().clone() });
    callAttributes = Arc::new(DAE::CallAttributes { ty: ty, tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL });
    expElse = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) });
    scalar1 = Arc::new(DAE::Exp::IFEXP { expCond: activeRef, expThen: scalar, expElse: expElse });
    outEqn = Arc::new(DAE::Element::EQUATION { exp: exp, scalar: scalar1, source: source });
    Ok(outEqn)
}

fn traversingSubsPreviousCref(mut inExp: Arc<DAE::Exp>, mut inCrefHit: (Arc<DAE::ComponentRef>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<DAE::ComponentRef>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = true;
    let mut outCrefHit: (Arc<DAE::ComponentRef>, bool);
    (outExp, outCrefHit) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefHit.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty }, tail: Deref @ metamodelica::List::Nil }, attr: _ }, (cref, _)) if (ComponentReferenceBasics::crefEqual(cr.clone(), cref.clone())?) => {
            let mut substituteRef: Arc<DAE::ComponentRef>;
            metamodelica::print(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("StateMachineFlatten.traversingSubsPreviousCref: cr: ")); __mm_s.push_str(&*ComponentReference::crefStr(cr.clone())?); __mm_s.push_str(&*literal!(", cref: ")); __mm_s.push_str(&*ComponentReference::crefStr(cref.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            substituteRef = ComponentReference::appendStringLastIdent((literal!("_previous")).clone(), cref.clone())?;
            (Arc::new(DAE::Exp::CREF { componentRef: substituteRef.clone(), ty: ty.clone() }), (cref.clone(), true))
        },
        _ => {
            (inExp, inCrefHit)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outCrefHit))
}

fn traversingSubsPreviousCrefs(mut inExp: Arc<DAE::Exp>, mut inCrefsHit: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, bool)) -> Result<(Arc<DAE::Exp>, bool, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, bool))> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cont: bool = true;
    let mut outCrefsHit: (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, bool);
    (outExp, outCrefsHit) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefsHit.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty }, tail: Deref @ metamodelica::List::Nil }, attr: _ }, (crefs, _)) if (List::any(crefs.clone(), (std::sync::Arc::new({ let __pe_b0 = cr.clone(); move |__pe_a1| ComponentReferenceBasics::crefEqual(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>) -> Result<bool> + 'static>))?) => {
            let mut substituteRef: Arc<DAE::ComponentRef>;
            substituteRef = ComponentReference::appendStringLastIdent((literal!("_previous")).clone(), cr.clone())?;
            (Arc::new(DAE::Exp::CREF { componentRef: substituteRef.clone(), ty: ty.clone() }), (crefs.clone(), true))
        },
        _ => {
            (inExp, inCrefsHit)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outCrefsHit))
}

fn getStartAttrOption(mut inElt: Arc<DAE::Element>) -> Result<Option<Arc<DAE::Exp>>> {
    let mut outExpOpt: Option<Arc<DAE::Exp>>;
    let mut start: Arc<DAE::Exp>;
    let mut ty: Arc<DAE::Type>;
    let mut varAttrOpt: Option<Arc<DAE::VariableAttributes>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inElt) {
        Deref @ DAE::Element::VAR { variableAttributesOption: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    varAttrOpt = __pa0.clone();
    ty = __pa1.clone();
    if isSome(varAttrOpt.clone()) {
        start = DAEUtil::getStartAttr(varAttrOpt, ty)?;
        outExpOpt = Some(start);
    } else {
        outExpOpt = None;
    }
    Ok(outExpOpt)
}

fn addPropagationEquations(mut inFlatSmSemantics: FlatSmSemantics, mut inEnclosingStateCrefOption: Option<Arc<DAE::ComponentRef>>, mut inEnclosingFlatSmSemanticsOption: Option<FlatSmSemantics>) -> Result<FlatSmSemantics> {
    let mut outFlatSmSemantics: FlatSmSemantics;
    let mut preRef: Arc<DAE::ComponentRef>;
    let mut initStateRef: Arc<DAE::ComponentRef>;
    let mut initRef: Arc<DAE::ComponentRef>;
    let mut resetRef: Arc<DAE::ComponentRef>;
    let mut activeRef: Arc<DAE::ComponentRef>;
    let mut stateRef: Arc<DAE::ComponentRef>;
    let mut activePlotIndicatorRef: Arc<DAE::ComponentRef>;
    let mut initVar: Arc<DAE::Element>;
    let mut activePlotIndicatorVar: Arc<DAE::Element>;
    let mut ticksInStateVar: Arc<DAE::Element>;
    let mut timeEnteredStateVar: Arc<DAE::Element>;
    let mut timeInStateVar: Arc<DAE::Element>;
    let mut activePlotIndicatorEqn: Arc<DAE::Element>;
    let mut ticksInStateEqn: Arc<DAE::Element>;
    let mut timeEnteredStateEqn: Arc<DAE::Element>;
    let mut timeInStateEqn: Arc<DAE::Element>;
    let mut rhs: Arc<DAE::Exp>;
    let mut andExp: Arc<DAE::Exp>;
    let mut eqExp: Arc<DAE::Exp>;
    let mut tArrayBool: Arc<DAE::Type>;
    let mut tArrayInteger: Arc<DAE::Type>;
    let mut ident: ArcStr;
    let mut smComps: metamodelica::Array<Arc<DAE::Element>>;
    let mut t: Arc<metamodelica::List<Transition>>;
    let mut c: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut smvars: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut smknowns: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut smeqs: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut pvars: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut peqs: Arc<metamodelica::List<Arc<DAE::Element>>> = metamodelica::nil();
    let mut enclosingStateCref: Arc<DAE::ComponentRef>;
    let mut enclosingPreRef: Arc<DAE::ComponentRef>;
    let mut enclosingActiveResetStateRef: Arc<DAE::ComponentRef>;
    let mut enclosingActiveResetRef: Arc<DAE::ComponentRef>;
    let mut enclosingActiveStateRef: Arc<DAE::ComponentRef>;
    let mut enclosingFlatSMSemantics: FlatSmSemantics;
    let mut enclosingFlatSMComps: metamodelica::Array<Arc<DAE::Element>>;
    let mut enclosingFlatSMInitStateRef: Arc<DAE::ComponentRef>;
    let mut posOfEnclosingSMComp: i32;
    let mut nStates: i32;
    let FlatSmSemantics { ident: __pa0, smComps: __pa1, t: __pa2, c: __pa3, vars: __pa4, knowns: __pa5, eqs: __pa6, .. } = (inFlatSmSemantics) else { bail!("pattern mismatch") };
    ident = __pa0.clone();
    smComps = __pa1.clone();
    t = __pa2.clone();
    c = __pa3.clone();
    smvars = __pa4.clone();
    smknowns = __pa5.clone();
    smeqs = __pa6.clone();
    let __pa7 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(smComps.clone(), 1)?) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa7, .. } => __pa7.clone(),
        _ => bail!("pattern mismatch"),
    } };
    initStateRef = __pa7.clone();
    preRef = ComponentReference::crefPrefixString((arcstr::literal!(SMS_PRE)).clone(), initStateRef);
    activeRef = qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?;
    resetRef = qCref((literal!("reset")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?;
    if isNone(inEnclosingFlatSmSemanticsOption.clone()) {
        initRef = qCref((literal!("init")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?;
        initVar = createVarWithDefaults(initRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil());
        initVar = setVarFixedStartValue(initVar, Arc::new(DAE::Exp::BCONST { bool: true }))?;
        pvars = metamodelica::cons(initVar, pvars);
        peqs = metamodelica::cons(Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: initRef.clone(), ty: DAE::T_BOOL_DEFAULT().clone() }), scalar: Arc::new(DAE::Exp::BCONST { bool: false }), source: DAE::emptyElementSource().clone() }), peqs);
        rhs = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: initRef, ty: DAE::T_BOOL_DEFAULT().clone() })], attr: DAE::callAttrBuiltinImpureBool().clone() });
        peqs = metamodelica::cons(Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: resetRef, ty: DAE::T_BOOL_DEFAULT().clone() }), scalar: rhs, source: DAE::emptyElementSource().clone() }), peqs);
        peqs = metamodelica::cons(Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: activeRef, ty: DAE::T_BOOL_DEFAULT().clone() }), scalar: Arc::new(DAE::Exp::BCONST { bool: true }), source: DAE::emptyElementSource().clone() }), peqs);
    } else {
        enclosingStateCref = Util::getOption(inEnclosingStateCrefOption.clone())?;
        enclosingFlatSMSemantics = Util::getOption(inEnclosingFlatSmSemanticsOption)?;
        let FlatSmSemantics { smComps: __pa8, .. } = (enclosingFlatSMSemantics) else { bail!("pattern mismatch") };
        enclosingFlatSMComps = __pa8.clone();
        let __pa9 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(enclosingFlatSMComps.clone(), 1)?) {
            Deref @ DAE::Element::SM_COMP { componentRef: __pa9, .. } => __pa9.clone(),
            _ => bail!("pattern mismatch"),
        } };
        enclosingFlatSMInitStateRef = __pa9.clone();
        enclosingPreRef = ComponentReference::crefPrefixString((arcstr::literal!(SMS_PRE)).clone(), enclosingFlatSMInitStateRef);
        posOfEnclosingSMComp = List::position1OnTrue(Arc::new(enclosingFlatSMComps.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()), (std::sync::Arc::new(sMCompEqualsRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), enclosingStateCref)?;
        nStates = metamodelica::arrayLength(enclosingFlatSMComps.clone());
        tArrayBool = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nStates })] });
        tArrayInteger = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nStates })] });
        enclosingActiveResetStateRef = qCref((literal!("activeResetStates")).clone(), tArrayBool, list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: posOfEnclosingSMComp }) })], enclosingPreRef.clone())?;
        enclosingActiveResetRef = qCref((literal!("activeReset")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), enclosingPreRef.clone())?;
        enclosingActiveStateRef = qCref((literal!("activeState")).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil(), enclosingPreRef)?;
        eqExp = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: enclosingActiveStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT().clone() }), operator: DAE::Operator::EQUAL { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: posOfEnclosingSMComp }), index: -1, optionExpisASUB: None });
        andExp = Arc::new(DAE::Exp::LBINARY { exp1: Arc::new(DAE::Exp::CREF { componentRef: enclosingActiveResetRef, ty: DAE::T_BOOL_DEFAULT().clone() }), operator: DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: eqExp });
        rhs = Arc::new(DAE::Exp::LBINARY { exp1: Arc::new(DAE::Exp::CREF { componentRef: enclosingActiveResetStateRef, ty: DAE::T_BOOL_DEFAULT().clone() }), operator: DAE::Operator::OR { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: andExp });
        peqs = metamodelica::cons(Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: resetRef, ty: DAE::T_BOOL_DEFAULT().clone() }), scalar: rhs, source: DAE::emptyElementSource().clone() }), peqs);
        rhs = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: enclosingActiveStateRef, ty: DAE::T_INTEGER_DEFAULT().clone() }), operator: DAE::Operator::EQUAL { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: posOfEnclosingSMComp }), index: -1, optionExpisASUB: None });
        peqs = metamodelica::cons(Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: activeRef, ty: DAE::T_BOOL_DEFAULT().clone() }), scalar: rhs, source: DAE::emptyElementSource().clone() }), peqs);
    }
    for mut i in 1..=metamodelica::arrayLength(smComps.clone()) {
        let __pa10 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(smComps.clone(), i.clone())?) {
            Deref @ DAE::Element::SM_COMP { componentRef: __pa10, .. } => __pa10.clone(),
            _ => bail!("pattern mismatch"),
        } };
        stateRef = __pa10.clone();
        (activePlotIndicatorVar, activePlotIndicatorEqn) = createActiveIndicator(stateRef.clone(), preRef.clone(), i.clone())?;
        pvars = metamodelica::cons(activePlotIndicatorVar.clone(), pvars.clone());
        peqs = metamodelica::cons(activePlotIndicatorEqn.clone(), peqs.clone());
        let __pa11 = ::match_deref::match_deref! { match &(activePlotIndicatorVar.clone()) {
            Deref @ DAE::Element::VAR { componentRef: __pa11, .. } => __pa11.clone(),
            _ => bail!("pattern mismatch"),
        } };
        activePlotIndicatorRef = __pa11.clone();
        (ticksInStateVar, ticksInStateEqn) = createTicksInStateIndicator(stateRef.clone(), activePlotIndicatorRef.clone())?;
        pvars = metamodelica::cons(ticksInStateVar.clone(), pvars.clone());
        peqs = metamodelica::cons(ticksInStateEqn.clone(), peqs.clone());
        (timeEnteredStateVar, timeEnteredStateEqn) = createTimeEnteredStateIndicator(stateRef.clone(), activePlotIndicatorRef.clone())?;
        (timeInStateVar, timeInStateEqn) = createTimeInStateIndicator(stateRef.clone(), activePlotIndicatorRef.clone(), timeEnteredStateVar.clone())?;
        pvars = metamodelica::cons(timeEnteredStateVar.clone(), metamodelica::cons(timeInStateVar.clone(), pvars.clone()));
        peqs = metamodelica::cons(timeEnteredStateEqn.clone(), metamodelica::cons(timeInStateEqn.clone(), peqs.clone()));
    }
    outFlatSmSemantics = FlatSmSemantics { ident: (ident).clone(), smComps: smComps.clone(), t: t, c: c, vars: smvars, knowns: smknowns, eqs: smeqs, pvars: pvars, peqs: peqs, enclosingState: inEnclosingStateCrefOption };
    Ok(outFlatSmSemantics)
}

fn createTimeInStateIndicator(mut stateRef: Arc<DAE::ComponentRef>, mut stateActiveRef: Arc<DAE::ComponentRef>, mut timeEnteredStateVar: Arc<DAE::Element>) -> Result<(Arc<DAE::Element>, Arc<DAE::Element>)> {
    let mut timeInStateVar: Arc<DAE::Element>;
    let mut timeInStateEqn: Arc<DAE::Element>;
    let mut timeInStateRef: Arc<DAE::ComponentRef>;
    let mut timeEnteredStateRef: Arc<DAE::ComponentRef>;
    let mut ty: Arc<DAE::Type>;
    let mut timeInStateExp: Arc<DAE::Exp>;
    let mut timeEnteredStateExp: Arc<DAE::Exp>;
    let mut stateActiveExp: Arc<DAE::Exp>;
    let mut expCond: Arc<DAE::Exp>;
    let mut expSampleTime: Arc<DAE::Exp>;
    let mut expThen: Arc<DAE::Exp>;
    let mut expElse: Arc<DAE::Exp>;
    timeInStateRef = qCref((literal!("$timeInState")).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil(), stateRef)?;
    timeInStateVar = createVarWithDefaults(timeInStateRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
    timeInStateVar = setVarFixedStartValue(timeInStateVar, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) }))?;
    timeInStateExp = Arc::new(DAE::Exp::CREF { componentRef: timeInStateRef, ty: DAE::T_REAL_DEFAULT().clone() });
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(timeEnteredStateVar) {
        Deref @ DAE::Element::VAR { componentRef: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    timeEnteredStateRef = __pa0.clone();
    ty = __pa1.clone();
    timeEnteredStateExp = Arc::new(DAE::Exp::CREF { componentRef: timeEnteredStateRef, ty: ty });
    stateActiveExp = Expression::crefExp(stateActiveRef.clone())?;
    expCond = Expression::crefExp(stateActiveRef)?;
    expSampleTime = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("time")).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_REAL_DEFAULT().clone() }), Arc::new(DAE::Exp::CLKCONST { clk: openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK() })], attr: DAE::callAttrBuiltinImpureReal().clone() });
    expThen = Arc::new(DAE::Exp::BINARY { exp1: expSampleTime, operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: timeEnteredStateExp });
    expElse = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) });
    timeInStateEqn = Arc::new(DAE::Element::EQUATION { exp: timeInStateExp, scalar: Arc::new(DAE::Exp::IFEXP { expCond: expCond, expThen: expThen, expElse: expElse }), source: DAE::emptyElementSource().clone() });
    Ok((timeInStateVar, timeInStateEqn))
}

fn createTimeEnteredStateIndicator(mut stateRef: Arc<DAE::ComponentRef>, mut stateActiveRef: Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::Element>, Arc<DAE::Element>)> {
    let mut timeEnteredStateVar: Arc<DAE::Element>;
    let mut timeEnteredStateEqn: Arc<DAE::Element>;
    let mut timeEnteredStateRef: Arc<DAE::ComponentRef>;
    let mut timeEnteredStateExp: Arc<DAE::Exp>;
    let mut stateActiveExp: Arc<DAE::Exp>;
    let mut expCond: Arc<DAE::Exp>;
    let mut expThen: Arc<DAE::Exp>;
    let mut expElse: Arc<DAE::Exp>;
    timeEnteredStateRef = qCref((literal!("$timeEnteredState")).clone(), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil(), stateRef)?;
    timeEnteredStateVar = createVarWithDefaults(timeEnteredStateRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
    timeEnteredStateVar = setVarFixedStartValue(timeEnteredStateVar, Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat((0) as f64) }))?;
    timeEnteredStateExp = Arc::new(DAE::Exp::CREF { componentRef: timeEnteredStateRef, ty: DAE::T_REAL_DEFAULT().clone() });
    stateActiveExp = Expression::crefExp(stateActiveRef)?;
    expCond = Arc::new(DAE::Exp::LBINARY { exp1: Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![stateActiveExp.clone()], attr: DAE::callAttrBuiltinImpureBool().clone() }), operator: DAE::Operator::EQUAL { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::BCONST { bool: false }), index: -1, optionExpisASUB: None }), operator: DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::RELATION { exp1: stateActiveExp, operator: DAE::Operator::EQUAL { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::BCONST { bool: true }), index: -1, optionExpisASUB: None }) });
    expThen = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("time")).clone(), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_REAL_DEFAULT().clone() }), Arc::new(DAE::Exp::CLKCONST { clk: openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK() })], attr: DAE::callAttrBuiltinImpureReal().clone() });
    expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![timeEnteredStateExp.clone()], attr: DAE::callAttrBuiltinImpureReal().clone() });
    timeEnteredStateEqn = Arc::new(DAE::Element::EQUATION { exp: timeEnteredStateExp, scalar: Arc::new(DAE::Exp::IFEXP { expCond: expCond, expThen: expThen, expElse: expElse }), source: DAE::emptyElementSource().clone() });
    Ok((timeEnteredStateVar, timeEnteredStateEqn))
}

fn createTicksInStateIndicator(mut stateRef: Arc<DAE::ComponentRef>, mut stateActiveRef: Arc<DAE::ComponentRef>) -> Result<(Arc<DAE::Element>, Arc<DAE::Element>)> {
    let mut ticksInStateVar: Arc<DAE::Element>;
    let mut ticksInStateEqn: Arc<DAE::Element>;
    let mut ticksInStateRef: Arc<DAE::ComponentRef>;
    let mut ticksInStateExp: Arc<DAE::Exp>;
    let mut expCond: Arc<DAE::Exp>;
    let mut expThen: Arc<DAE::Exp>;
    let mut expElse: Arc<DAE::Exp>;
    ticksInStateRef = qCref((literal!("$ticksInState")).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil(), stateRef)?;
    ticksInStateVar = createVarWithDefaults(ticksInStateRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil());
    ticksInStateVar = setVarFixedStartValue(ticksInStateVar, Arc::new(DAE::Exp::ICONST { integer: 0 }))?;
    ticksInStateExp = Arc::new(DAE::Exp::CREF { componentRef: ticksInStateRef, ty: DAE::T_INTEGER_DEFAULT().clone() });
    expCond = Expression::crefExp(stateActiveRef)?;
    expThen = Arc::new(DAE::Exp::BINARY { exp1: Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![ticksInStateExp.clone()], attr: DAE::callAttrBuiltinImpureInteger().clone() }), operator: DAE::Operator::ADD { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: 1 }) });
    expElse = Arc::new(DAE::Exp::ICONST { integer: 0 });
    ticksInStateEqn = Arc::new(DAE::Element::EQUATION { exp: ticksInStateExp, scalar: Arc::new(DAE::Exp::IFEXP { expCond: expCond, expThen: expThen, expElse: expElse }), source: DAE::emptyElementSource().clone() });
    Ok((ticksInStateVar, ticksInStateEqn))
}

fn createActiveIndicator(mut stateRef: Arc<DAE::ComponentRef>, mut preRef: Arc<DAE::ComponentRef>, mut i: i32) -> Result<(Arc<DAE::Element>, Arc<DAE::Element>)> {
    let mut activePlotIndicatorVar: Arc<DAE::Element>;
    let mut eqn: Arc<DAE::Element>;
    let mut activeRef: Arc<DAE::ComponentRef>;
    let mut activePlotIndicatorRef: Arc<DAE::ComponentRef>;
    let mut activeStateRef: Arc<DAE::ComponentRef>;
    let mut andExp: Arc<DAE::Exp>;
    let mut eqExp: Arc<DAE::Exp>;
    activePlotIndicatorRef = qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), stateRef)?;
    activePlotIndicatorVar = createVarWithStartValue(activePlotIndicatorRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT().clone(), Arc::new(DAE::Exp::BCONST { bool: false }), metamodelica::nil())?;
    activeRef = qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?;
    activeStateRef = qCref((literal!("activeState")).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil(), preRef)?;
    eqExp = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: activeStateRef, ty: DAE::T_INTEGER_DEFAULT().clone() }), operator: DAE::Operator::EQUAL { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: i }), index: -1, optionExpisASUB: None });
    andExp = Arc::new(DAE::Exp::LBINARY { exp1: Arc::new(DAE::Exp::CREF { componentRef: activeRef, ty: DAE::T_BOOL_DEFAULT().clone() }), operator: DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: eqExp });
    eqn = Arc::new(DAE::Element::EQUATION { exp: Arc::new(DAE::Exp::CREF { componentRef: activePlotIndicatorRef, ty: DAE::T_BOOL_DEFAULT().clone() }), scalar: andExp, source: DAE::emptyElementSource().clone() });
    Ok((activePlotIndicatorVar, eqn))
}

fn setVarFixedStartValue(mut inVar: Arc<DAE::Element>, mut inExp: Arc<DAE::Exp>) -> Result<Arc<DAE::Element>> {
    let mut outVar: Arc<DAE::Element>;
    let mut vao: Option<Arc<DAE::VariableAttributes>>;
    let __pa0 = ::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Element::VAR { variableAttributesOption: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    vao = __pa0.clone();
    vao = DAEUtil::setStartAttrOption(vao, Some(inExp))?;
    vao = DAEUtil::setFixedAttr(vao, Some(Arc::new(DAE::Exp::BCONST { bool: true })))?;
    outVar = DAEUtil::setVariableAttributes(inVar, vao)?;
    Ok(outVar)
}

fn basicFlatSmSemantics(mut ident: ArcStr, mut q: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inTransitions: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<FlatSmSemantics> {
    let mut flatSmSemantics: FlatSmSemantics;
    let mut crefInitialState: Arc<DAE::ComponentRef>;
    let mut preRef: Arc<DAE::ComponentRef>;
    let mut defaultIntVar: Arc<DAE::Element>;
    let mut defaultBoolVar: Arc<DAE::Element>;
    let mut vars: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut knowns: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut i: i32;
    let mut preRef: Arc<DAE::ComponentRef>;
    let mut nStatesRef: Arc<DAE::ComponentRef>;
    let mut activeRef: Arc<DAE::ComponentRef>;
    let mut resetRef: Arc<DAE::ComponentRef>;
    let mut selectedStateRef: Arc<DAE::ComponentRef>;
    let mut selectedResetRef: Arc<DAE::ComponentRef>;
    let mut firedRef: Arc<DAE::ComponentRef>;
    let mut activeStateRef: Arc<DAE::ComponentRef>;
    let mut activeResetRef: Arc<DAE::ComponentRef>;
    let mut nextStateRef: Arc<DAE::ComponentRef>;
    let mut nextResetRef: Arc<DAE::ComponentRef>;
    let mut stateMachineInFinalStateRef: Arc<DAE::ComponentRef>;
    let mut nStatesVar: Arc<DAE::Element>;
    let mut activeVar: Arc<DAE::Element>;
    let mut resetVar: Arc<DAE::Element>;
    let mut selectedStateVar: Arc<DAE::Element>;
    let mut selectedResetVar: Arc<DAE::Element>;
    let mut firedVar: Arc<DAE::Element>;
    let mut activeStateVar: Arc<DAE::Element>;
    let mut activeResetVar: Arc<DAE::Element>;
    let mut nextStateVar: Arc<DAE::Element>;
    let mut nextResetVar: Arc<DAE::Element>;
    let mut stateMachineInFinalStateVar: Arc<DAE::Element>;
    let mut nStates: i32;
    let mut nStatesDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    let mut nStatesArrayBool: Arc<DAE::Type>;
    let mut activeResetStatesRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut nextResetStatesRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut finalStatesRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut activeResetStatesVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut nextResetStatesVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut finalStatesVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut t: Arc<metamodelica::List<Transition>>;
    let mut nTransitions: i32;
    let mut tDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>;
    let mut tArrayInteger: Arc<DAE::Type>;
    let mut tArrayBool: Arc<DAE::Type>;
    let mut tFromRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut tToRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut tImmediateRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut tResetRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut tSynchronizeRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut tPriorityRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut tFromVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut tToVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut tImmediateVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut tResetVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut tSynchronizeVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut tPriorityVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut from: i32;
    let mut to: i32;
    let mut immediate: bool;
    let mut reset: bool;
    let mut synchronize: bool;
    let mut priority: i32;
    let mut cExps: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut cRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut cImmediateRefs: metamodelica::Array<Arc<DAE::ComponentRef>>;
    let mut cVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut cImmediateVars: metamodelica::Array<Arc<DAE::Element>>;
    let mut eqs: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut selectedStateEqn: Arc<DAE::Element>;
    let mut selectedResetEqn: Arc<DAE::Element>;
    let mut firedEqn: Arc<DAE::Element>;
    let mut activeStateEqn: Arc<DAE::Element>;
    let mut activeResetEqn: Arc<DAE::Element>;
    let mut nextStateEqn: Arc<DAE::Element>;
    let mut nextResetEqn: Arc<DAE::Element>;
    let mut exp: Arc<DAE::Exp>;
    let mut rhs: Arc<DAE::Exp>;
    let mut expCond: Arc<DAE::Exp>;
    let mut expThen: Arc<DAE::Exp>;
    let mut expElse: Arc<DAE::Exp>;
    let mut exp1: Arc<DAE::Exp>;
    let mut exp2: Arc<DAE::Exp>;
    let mut expIf: Arc<DAE::Exp>;
    let mut expLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut bindExp: Option<Arc<DAE::Exp>>;
    let __pa0 = ::match_deref::match_deref! { match &(listHead(q.clone())?) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    crefInitialState = __pa0.clone();
    preRef = ComponentReference::crefPrefixString((arcstr::literal!(SMS_PRE)).clone(), crefInitialState);
    (t, cExps) = createTandC(q.clone(), inTransitions)?;
    defaultIntVar = createVarWithDefaults(ComponentReference::makeDummyCref(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil());
    defaultBoolVar = createVarWithDefaults(ComponentReference::makeDummyCref(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil());
    knowns = metamodelica::nil();
    vars = metamodelica::nil();
    nStates = (q.clone().len() as i32);
    nStatesRef = qCref((literal!("nState")).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?;
    nStatesVar = createVarWithDefaults(nStatesRef, openmodelica_frontend_types::DAE::VarKind::PARAM, DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil());
    nStatesVar = DAEUtil::setElementVarBinding(nStatesVar, Some(Arc::new(DAE::Exp::ICONST { integer: nStates })));
    knowns = metamodelica::cons(nStatesVar, knowns);
    nTransitions = (t.clone().len() as i32);
    tDims = list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nTransitions })];
    tArrayInteger = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: tDims.clone() });
    tArrayBool = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: tDims.clone() });
    tFromRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    tToRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    tImmediateRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    tResetRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    tSynchronizeRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    tPriorityRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    tFromVars = arrayCreate(nTransitions, defaultIntVar.clone());
    tToVars = arrayCreate(nTransitions, defaultIntVar.clone());
    tImmediateVars = arrayCreate(nTransitions, defaultBoolVar.clone());
    tResetVars = arrayCreate(nTransitions, defaultBoolVar.clone());
    tSynchronizeVars = arrayCreate(nTransitions, defaultBoolVar.clone());
    tPriorityVars = arrayCreate(nTransitions, defaultIntVar);
    i = 0;
    for mut t1 in &*t.clone() {
        let mut t1 = t1.clone();
        i = i + 1;
        let Transition { from: __pa1, to: __pa2, condition: _, immediate: __pa3, reset: __pa4, synchronize: __pa5, priority: __pa6 } = (t1.clone()) else { bail!("pattern mismatch") };
        from = __pa1.clone();
        to = __pa2.clone();
        immediate = __pa3.clone();
        reset = __pa4.clone();
        synchronize = __pa5.clone();
        priority = __pa6.clone();
        tFromRefs = metamodelica::arrayUpdate(tFromRefs.clone(), i, qCref((literal!("tFrom")).clone(), tArrayInteger.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i }) })], preRef.clone())?)?;
        tFromVars = metamodelica::arrayUpdate(tFromVars.clone(), i, createVarWithDefaults(metamodelica::arrayGet(tFromRefs.clone(), i)?, openmodelica_frontend_types::DAE::VarKind::PARAM, DAE::T_INTEGER_DEFAULT().clone(), tDims.clone()))?;
        tFromVars = metamodelica::arrayUpdate(tFromVars.clone(), i, DAEUtil::setElementVarBinding(metamodelica::arrayGet(tFromVars.clone(), i)?, Some(Arc::new(DAE::Exp::ICONST { integer: from }))))?;
        knowns = metamodelica::cons(metamodelica::arrayGet(tFromVars.clone(), i)?, knowns.clone());
        tToRefs = metamodelica::arrayUpdate(tToRefs.clone(), i, qCref((literal!("tTo")).clone(), tArrayInteger.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i }) })], preRef.clone())?)?;
        tToVars = metamodelica::arrayUpdate(tToVars.clone(), i, createVarWithDefaults(metamodelica::arrayGet(tToRefs.clone(), i)?, openmodelica_frontend_types::DAE::VarKind::PARAM, DAE::T_INTEGER_DEFAULT().clone(), tDims.clone()))?;
        tToVars = metamodelica::arrayUpdate(tToVars.clone(), i, DAEUtil::setElementVarBinding(metamodelica::arrayGet(tToVars.clone(), i)?, Some(Arc::new(DAE::Exp::ICONST { integer: to }))))?;
        knowns = metamodelica::cons(metamodelica::arrayGet(tToVars.clone(), i)?, knowns.clone());
        tImmediateRefs = metamodelica::arrayUpdate(tImmediateRefs.clone(), i, qCref((literal!("tImmediate")).clone(), tArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i }) })], preRef.clone())?)?;
        tImmediateVars = metamodelica::arrayUpdate(tImmediateVars.clone(), i, createVarWithDefaults(metamodelica::arrayGet(tImmediateRefs.clone(), i)?, openmodelica_frontend_types::DAE::VarKind::PARAM, DAE::T_BOOL_DEFAULT().clone(), tDims.clone()))?;
        tImmediateVars = metamodelica::arrayUpdate(tImmediateVars.clone(), i, DAEUtil::setElementVarBinding(metamodelica::arrayGet(tImmediateVars.clone(), i)?, Some(Arc::new(DAE::Exp::BCONST { bool: immediate }))))?;
        knowns = metamodelica::cons(metamodelica::arrayGet(tImmediateVars.clone(), i)?, knowns.clone());
        tResetRefs = metamodelica::arrayUpdate(tResetRefs.clone(), i, qCref((literal!("tReset")).clone(), tArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i }) })], preRef.clone())?)?;
        tResetVars = metamodelica::arrayUpdate(tResetVars.clone(), i, createVarWithDefaults(metamodelica::arrayGet(tResetRefs.clone(), i)?, openmodelica_frontend_types::DAE::VarKind::PARAM, DAE::T_BOOL_DEFAULT().clone(), tDims.clone()))?;
        tResetVars = metamodelica::arrayUpdate(tResetVars.clone(), i, DAEUtil::setElementVarBinding(metamodelica::arrayGet(tResetVars.clone(), i)?, Some(Arc::new(DAE::Exp::BCONST { bool: reset }))))?;
        knowns = metamodelica::cons(metamodelica::arrayGet(tResetVars.clone(), i)?, knowns.clone());
        tSynchronizeRefs = metamodelica::arrayUpdate(tSynchronizeRefs.clone(), i, qCref((literal!("tSynchronize")).clone(), tArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i }) })], preRef.clone())?)?;
        tSynchronizeVars = metamodelica::arrayUpdate(tSynchronizeVars.clone(), i, createVarWithDefaults(metamodelica::arrayGet(tSynchronizeRefs.clone(), i)?, openmodelica_frontend_types::DAE::VarKind::PARAM, DAE::T_BOOL_DEFAULT().clone(), tDims.clone()))?;
        tSynchronizeVars = metamodelica::arrayUpdate(tSynchronizeVars.clone(), i, DAEUtil::setElementVarBinding(metamodelica::arrayGet(tSynchronizeVars.clone(), i)?, Some(Arc::new(DAE::Exp::BCONST { bool: synchronize }))))?;
        knowns = metamodelica::cons(metamodelica::arrayGet(tSynchronizeVars.clone(), i)?, knowns.clone());
        tPriorityRefs = metamodelica::arrayUpdate(tPriorityRefs.clone(), i, qCref((literal!("tPriority")).clone(), tArrayInteger.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i }) })], preRef.clone())?)?;
        tPriorityVars = metamodelica::arrayUpdate(tPriorityVars.clone(), i, createVarWithDefaults(metamodelica::arrayGet(tPriorityRefs.clone(), i)?, openmodelica_frontend_types::DAE::VarKind::PARAM, DAE::T_INTEGER_DEFAULT().clone(), tDims.clone()))?;
        tPriorityVars = metamodelica::arrayUpdate(tPriorityVars.clone(), i, DAEUtil::setElementVarBinding(metamodelica::arrayGet(tPriorityVars.clone(), i)?, Some(Arc::new(DAE::Exp::ICONST { integer: priority }))))?;
        knowns = metamodelica::cons(metamodelica::arrayGet(tPriorityVars.clone(), i)?, knowns.clone());
    }
    cRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    cImmediateRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    cVars = arrayCreate(nTransitions, defaultBoolVar.clone());
    cImmediateVars = arrayCreate(nTransitions, defaultBoolVar.clone());
    i = 0;
    for mut exp in &*cExps.clone() {
        let mut exp = exp.clone();
        i = i + 1;
        cRefs = metamodelica::arrayUpdate(cRefs.clone(), i, qCref((literal!("c")).clone(), tArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i }) })], preRef.clone())?)?;
        cImmediateRefs = metamodelica::arrayUpdate(cImmediateRefs.clone(), i, qCref((literal!("cImmediate")).clone(), tArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i }) })], preRef.clone())?)?;
        cVars = metamodelica::arrayUpdate(cVars.clone(), i, createVarWithDefaults(metamodelica::arrayGet(cRefs.clone(), i)?, openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT().clone(), tDims.clone()))?;
        cImmediateVars = metamodelica::arrayUpdate(cImmediateVars.clone(), i, createVarWithStartValue(metamodelica::arrayGet(cImmediateRefs.clone(), i)?, openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT().clone(), Arc::new(DAE::Exp::BCONST { bool: false }), tDims.clone())?)?;
        vars = metamodelica::cons(metamodelica::arrayGet(cVars.clone(), i)?, vars.clone());
        vars = metamodelica::cons(metamodelica::arrayGet(cImmediateVars.clone(), i)?, vars.clone());
    }
    activeRef = qCref((literal!("active")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?;
    activeVar = createVarWithDefaults(activeRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil());
    vars = metamodelica::cons(activeVar, vars);
    resetRef = qCref((literal!("reset")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?;
    resetVar = createVarWithDefaults(resetRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil());
    vars = metamodelica::cons(resetVar, vars);
    selectedStateRef = qCref((literal!("selectedState")).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?;
    selectedStateVar = createVarWithDefaults(selectedStateRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil());
    vars = metamodelica::cons(selectedStateVar, vars);
    selectedResetRef = qCref((literal!("selectedReset")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?;
    selectedResetVar = createVarWithDefaults(selectedResetRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil());
    vars = metamodelica::cons(selectedResetVar, vars);
    firedRef = qCref((literal!("fired")).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?;
    firedVar = createVarWithDefaults(firedRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil());
    vars = metamodelica::cons(firedVar, vars);
    activeStateRef = qCref((literal!("activeState")).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?;
    activeStateVar = createVarWithDefaults(activeStateRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil());
    vars = metamodelica::cons(activeStateVar, vars);
    activeResetRef = qCref((literal!("activeReset")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?;
    activeResetVar = createVarWithDefaults(activeResetRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil());
    vars = metamodelica::cons(activeResetVar, vars);
    nextStateRef = qCref((literal!("nextState")).clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?;
    nextStateVar = createVarWithStartValue(nextStateRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_INTEGER_DEFAULT().clone(), Arc::new(DAE::Exp::ICONST { integer: 0 }), metamodelica::nil())?;
    vars = metamodelica::cons(nextStateVar, vars);
    nextResetRef = qCref((literal!("nextReset")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?;
    nextResetVar = createVarWithStartValue(nextResetRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT().clone(), Arc::new(DAE::Exp::BCONST { bool: false }), metamodelica::nil())?;
    vars = metamodelica::cons(nextResetVar, vars);
    nStatesDims = list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: nStates })];
    nStatesArrayBool = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: nStatesDims.clone() });
    activeResetStatesRefs = arrayCreate(nStates, ComponentReference::makeDummyCref());
    activeResetStatesVars = arrayCreate(nStates, defaultBoolVar.clone());
    for mut i in 1..=nStates {
        activeResetStatesRefs = metamodelica::arrayUpdate(activeResetStatesRefs.clone(), i, qCref((literal!("activeResetStates")).clone(), nStatesArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i }) })], preRef.clone())?)?;
        activeResetStatesVars = metamodelica::arrayUpdate(activeResetStatesVars.clone(), i, createVarWithDefaults(metamodelica::arrayGet(activeResetStatesRefs.clone(), i)?, openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT().clone(), nStatesDims.clone()))?;
        vars = metamodelica::cons(metamodelica::arrayGet(activeResetStatesVars.clone(), i)?, vars.clone());
    }
    nextResetStatesRefs = arrayCreate(nStates, ComponentReference::makeDummyCref());
    nextResetStatesVars = arrayCreate(nStates, defaultBoolVar.clone());
    for mut i in 1..=nStates {
        nextResetStatesRefs = metamodelica::arrayUpdate(nextResetStatesRefs.clone(), i, qCref((literal!("nextResetStates")).clone(), nStatesArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i }) })], preRef.clone())?)?;
        nextResetStatesVars = metamodelica::arrayUpdate(nextResetStatesVars.clone(), i, createVarWithStartValue(metamodelica::arrayGet(nextResetStatesRefs.clone(), i)?, openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT().clone(), Arc::new(DAE::Exp::BCONST { bool: false }), nStatesDims.clone())?)?;
        vars = metamodelica::cons(metamodelica::arrayGet(nextResetStatesVars.clone(), i)?, vars.clone());
    }
    finalStatesRefs = arrayCreate(nStates, ComponentReference::makeDummyCref());
    finalStatesVars = arrayCreate(nStates, defaultBoolVar);
    for mut i in 1..=nStates {
        finalStatesRefs = metamodelica::arrayUpdate(finalStatesRefs.clone(), i, qCref((literal!("finalStates")).clone(), nStatesArrayBool.clone(), list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::ICONST { integer: i }) })], preRef.clone())?)?;
        finalStatesVars = metamodelica::arrayUpdate(finalStatesVars.clone(), i, createVarWithDefaults(metamodelica::arrayGet(finalStatesRefs.clone(), i)?, openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT().clone(), nStatesDims.clone()))?;
        vars = metamodelica::cons(metamodelica::arrayGet(finalStatesVars.clone(), i)?, vars.clone());
    }
    stateMachineInFinalStateRef = qCref((literal!("stateMachineInFinalState")).clone(), DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil(), preRef.clone())?;
    stateMachineInFinalStateVar = createVarWithDefaults(stateMachineInFinalStateRef.clone(), openmodelica_frontend_types::DAE::VarKind::DISCRETE, DAE::T_BOOL_DEFAULT().clone(), metamodelica::nil());
    vars = metamodelica::cons(stateMachineInFinalStateVar, vars);
    eqs = metamodelica::nil();
    i = 0;
    for mut cExp in &*cExps.clone() {
        let mut cExp = cExp.clone();
        i = i + 1;
        exp = Arc::new(DAE::Exp::CREF { componentRef: metamodelica::arrayGet(cImmediateRefs.clone(), i)?, ty: DAE::T_BOOL_DEFAULT().clone() });
        eqs = metamodelica::cons(Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: cExp.clone(), source: DAE::emptyElementSource().clone() }), eqs.clone());
        exp1 = Arc::new(DAE::Exp::CREF { componentRef: metamodelica::arrayGet(cRefs.clone(), i)?, ty: DAE::T_BOOL_DEFAULT().clone() });
        let __pa7 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(tImmediateVars.clone(), i)?) {
            Deref @ DAE::Element::VAR { binding: __pa7, .. } => __pa7.clone(),
            _ => bail!("pattern mismatch"),
        } };
        bindExp = __pa7.clone();
        rhs = if (Util::applyOptionOrDefault(bindExp.clone(), (std::sync::Arc::new({ let __pe_b0 = Arc::new(DAE::Exp::BCONST { bool: true }); move |__pe_a1| ExpressionBasics::expEqual(__pe_b0.clone(), __pe_a1) }) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<bool> + 'static>), false)?) {exp.clone()} else {Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![exp.clone()], attr: DAE::callAttrBuiltinImpureBool().clone() })};
        eqs = metamodelica::cons(Arc::new(DAE::Element::EQUATION { exp: exp1.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone() }), eqs.clone());
    }
    exp = Arc::new(DAE::Exp::CREF { componentRef: selectedStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT().clone() });
    expCond = Arc::new(DAE::Exp::CREF { componentRef: resetRef.clone(), ty: DAE::T_BOOL_DEFAULT().clone() });
    expThen = Arc::new(DAE::Exp::ICONST { integer: 1 });
    expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: nextStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT().clone() })], attr: DAE::callAttrBuiltinImpureInteger().clone() });
    rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond, expThen: expThen, expElse: expElse });
    selectedStateEqn = Arc::new(DAE::Element::EQUATION { exp: exp, scalar: rhs, source: DAE::emptyElementSource().clone() });
    eqs = metamodelica::cons(selectedStateEqn, eqs);
    exp = Arc::new(DAE::Exp::CREF { componentRef: selectedResetRef.clone(), ty: DAE::T_BOOL_DEFAULT().clone() });
    expCond = Arc::new(DAE::Exp::CREF { componentRef: resetRef.clone(), ty: DAE::T_BOOL_DEFAULT().clone() });
    expThen = Arc::new(DAE::Exp::BCONST { bool: true });
    expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: nextResetRef.clone(), ty: DAE::T_BOOL_DEFAULT().clone() })], attr: DAE::callAttrBuiltinImpureBool().clone() });
    rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() });
    selectedResetEqn = Arc::new(DAE::Element::EQUATION { exp: exp, scalar: rhs, source: DAE::emptyElementSource().clone() });
    eqs = metamodelica::cons(selectedResetEqn, eqs);
    exp = Arc::new(DAE::Exp::CREF { componentRef: firedRef.clone(), ty: DAE::T_INTEGER_DEFAULT().clone() });
    expLst = metamodelica::nil();
    for mut i in 1..=nTransitions {
        expCond = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: metamodelica::arrayGet(tFromRefs.clone(), i)?, ty: DAE::T_INTEGER_DEFAULT().clone() }), operator: DAE::Operator::EQUAL { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::CREF { componentRef: selectedStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT().clone() }), index: -1, optionExpisASUB: None });
        expThen = Arc::new(DAE::Exp::CREF { componentRef: metamodelica::arrayGet(cRefs.clone(), i)?, ty: DAE::T_BOOL_DEFAULT().clone() });
        expElse = Arc::new(DAE::Exp::BCONST { bool: false });
        expIf = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() });
        expLst = metamodelica::cons(Arc::new(DAE::Exp::IFEXP { expCond: expIf.clone(), expThen: Arc::new(DAE::Exp::ICONST { integer: i }), expElse: Arc::new(DAE::Exp::ICONST { integer: 0 }) }), expLst.clone());
    }
    rhs = if ((expLst.clone().len() as i32) > 1) {Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("max")).clone() }), expLst: list![Expression::makeScalarArray(expLst.clone(), DAE::T_INTEGER_DEFAULT().clone())], attr: DAE::callAttrBuiltinInteger().clone() })} else {listHead(expLst.clone())?};
    firedEqn = Arc::new(DAE::Element::EQUATION { exp: exp, scalar: rhs, source: DAE::emptyElementSource().clone() });
    eqs = metamodelica::cons(firedEqn, eqs);
    exp = Arc::new(DAE::Exp::CREF { componentRef: activeStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT().clone() });
    expCond = Arc::new(DAE::Exp::CREF { componentRef: resetRef.clone(), ty: DAE::T_BOOL_DEFAULT().clone() });
    expThen = Arc::new(DAE::Exp::ICONST { integer: 1 });
    exp1 = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: firedRef.clone(), ty: DAE::T_INTEGER_DEFAULT().clone() }), operator: DAE::Operator::GREATER { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: 0 }), index: -1, optionExpisASUB: None });
    exp2 = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("tTo")).clone(), tArrayInteger, list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::CREF { componentRef: firedRef.clone(), ty: DAE::T_INTEGER_DEFAULT().clone() }) })], preRef.clone())?, ty: DAE::T_INTEGER_DEFAULT().clone() });
    expElse = Arc::new(DAE::Exp::IFEXP { expCond: exp1, expThen: exp2, expElse: Arc::new(DAE::Exp::CREF { componentRef: selectedStateRef, ty: DAE::T_INTEGER_DEFAULT().clone() }) });
    rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond, expThen: expThen, expElse: expElse });
    activeStateEqn = Arc::new(DAE::Element::EQUATION { exp: exp, scalar: rhs, source: DAE::emptyElementSource().clone() });
    eqs = metamodelica::cons(activeStateEqn, eqs);
    exp = Arc::new(DAE::Exp::CREF { componentRef: activeResetRef, ty: DAE::T_BOOL_DEFAULT().clone() });
    expCond = Arc::new(DAE::Exp::CREF { componentRef: resetRef.clone(), ty: DAE::T_BOOL_DEFAULT().clone() });
    expThen = Arc::new(DAE::Exp::BCONST { bool: true });
    exp1 = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: firedRef.clone(), ty: DAE::T_INTEGER_DEFAULT().clone() }), operator: DAE::Operator::GREATER { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: 0 }), index: -1, optionExpisASUB: None });
    exp2 = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("tReset")).clone(), tArrayBool, list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::CREF { componentRef: firedRef, ty: DAE::T_INTEGER_DEFAULT().clone() }) })], preRef.clone())?, ty: DAE::T_INTEGER_DEFAULT().clone() });
    expElse = Arc::new(DAE::Exp::IFEXP { expCond: exp1.clone(), expThen: exp2, expElse: Arc::new(DAE::Exp::CREF { componentRef: selectedResetRef, ty: DAE::T_BOOL_DEFAULT().clone() }) });
    rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond, expThen: expThen, expElse: expElse });
    activeResetEqn = Arc::new(DAE::Element::EQUATION { exp: exp, scalar: rhs, source: DAE::emptyElementSource().clone() });
    eqs = metamodelica::cons(activeResetEqn, eqs);
    exp = Arc::new(DAE::Exp::CREF { componentRef: nextStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT().clone() });
    expCond = Arc::new(DAE::Exp::CREF { componentRef: activeRef.clone(), ty: DAE::T_BOOL_DEFAULT().clone() });
    expThen = Arc::new(DAE::Exp::CREF { componentRef: activeStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT().clone() });
    expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: nextStateRef, ty: DAE::T_INTEGER_DEFAULT().clone() })], attr: DAE::callAttrBuiltinImpureInteger().clone() });
    rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond, expThen: expThen, expElse: expElse });
    nextStateEqn = Arc::new(DAE::Element::EQUATION { exp: exp, scalar: rhs, source: DAE::emptyElementSource().clone() });
    eqs = metamodelica::cons(nextStateEqn, eqs);
    exp = Arc::new(DAE::Exp::CREF { componentRef: nextResetRef.clone(), ty: DAE::T_BOOL_DEFAULT().clone() });
    expCond = Arc::new(DAE::Exp::CREF { componentRef: activeRef.clone(), ty: DAE::T_BOOL_DEFAULT().clone() });
    expThen = Arc::new(DAE::Exp::BCONST { bool: false });
    expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: nextResetRef, ty: DAE::T_BOOL_DEFAULT().clone() })], attr: DAE::callAttrBuiltinImpureBool().clone() });
    rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() });
    nextResetEqn = Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone() });
    eqs = metamodelica::cons(nextResetEqn, eqs);
    for mut i in 1..=nStates {
        exp = Arc::new(DAE::Exp::CREF { componentRef: metamodelica::arrayGet(activeResetStatesRefs.clone(), i)?, ty: DAE::T_BOOL_DEFAULT().clone() });
        expCond = Arc::new(DAE::Exp::CREF { componentRef: resetRef.clone(), ty: DAE::T_BOOL_DEFAULT().clone() });
        expThen = Arc::new(DAE::Exp::BCONST { bool: true });
        expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: metamodelica::arrayGet(nextResetStatesRefs.clone(), i)?, ty: DAE::T_BOOL_DEFAULT().clone() })], attr: DAE::callAttrBuiltinImpureBool().clone() });
        rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() });
        eqs = metamodelica::cons(Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone() }), eqs.clone());
    }
    for mut i in 1..=nStates {
        exp = Arc::new(DAE::Exp::CREF { componentRef: metamodelica::arrayGet(nextResetStatesRefs.clone(), i)?, ty: DAE::T_BOOL_DEFAULT().clone() });
        expCond = Arc::new(DAE::Exp::CREF { componentRef: activeRef.clone(), ty: DAE::T_BOOL_DEFAULT().clone() });
        exp1 = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: activeStateRef.clone(), ty: DAE::T_INTEGER_DEFAULT().clone() }), operator: DAE::Operator::EQUAL { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: i }), index: -1, optionExpisASUB: None });
        expThen = Arc::new(DAE::Exp::IFEXP { expCond: exp1.clone(), expThen: Arc::new(DAE::Exp::BCONST { bool: false }), expElse: Arc::new(DAE::Exp::CREF { componentRef: metamodelica::arrayGet(activeResetStatesRefs.clone(), i)?, ty: DAE::T_BOOL_DEFAULT().clone() }) });
        expElse = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("previous")).clone() }), expLst: list![Arc::new(DAE::Exp::CREF { componentRef: metamodelica::arrayGet(nextResetStatesRefs.clone(), i)?, ty: DAE::T_BOOL_DEFAULT().clone() })], attr: DAE::callAttrBuiltinImpureBool().clone() });
        rhs = Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: expThen.clone(), expElse: expElse.clone() });
        eqs = metamodelica::cons(Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone() }), eqs.clone());
    }
    for mut i in 1..=nStates {
        exp = Arc::new(DAE::Exp::CREF { componentRef: metamodelica::arrayGet(finalStatesRefs.clone(), i)?, ty: DAE::T_BOOL_DEFAULT().clone() });
        expLst = metamodelica::nil();
        for mut j in 1..=nTransitions {
            expCond = Arc::new(DAE::Exp::RELATION { exp1: Arc::new(DAE::Exp::CREF { componentRef: metamodelica::arrayGet(tFromRefs.clone(), j.clone())?, ty: DAE::T_INTEGER_DEFAULT().clone() }), operator: DAE::Operator::EQUAL { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: i }), index: -1, optionExpisASUB: None });
            expLst = metamodelica::cons(Arc::new(DAE::Exp::IFEXP { expCond: expCond.clone(), expThen: Arc::new(DAE::Exp::ICONST { integer: 1 }), expElse: Arc::new(DAE::Exp::ICONST { integer: 0 }) }), expLst.clone());
        }
        exp1 = if ((expLst.clone().len() as i32) > 1) {Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("max")).clone() }), expLst: list![Expression::makeScalarArray(expLst.clone(), DAE::T_INTEGER_DEFAULT().clone())], attr: DAE::callAttrBuiltinInteger().clone() })} else {listHead(expLst.clone())?};
        rhs = Arc::new(DAE::Exp::RELATION { exp1: exp1.clone(), operator: DAE::Operator::EQUAL { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: Arc::new(DAE::Exp::ICONST { integer: 0 }), index: -1, optionExpisASUB: None });
        eqs = metamodelica::cons(Arc::new(DAE::Element::EQUATION { exp: exp.clone(), scalar: rhs.clone(), source: DAE::emptyElementSource().clone() }), eqs.clone());
    }
    exp = Arc::new(DAE::Exp::CREF { componentRef: stateMachineInFinalStateRef, ty: DAE::T_BOOL_DEFAULT().clone() });
    rhs = Arc::new(DAE::Exp::CREF { componentRef: qCref((literal!("finalStates")).clone(), nStatesArrayBool, list![Arc::new(DAE::Subscript::INDEX { exp: Arc::new(DAE::Exp::CREF { componentRef: activeStateRef, ty: DAE::T_INTEGER_DEFAULT().clone() }) })], preRef)?, ty: DAE::T_BOOL_DEFAULT().clone() });
    eqs = metamodelica::cons(Arc::new(DAE::Element::EQUATION { exp: exp, scalar: rhs, source: DAE::emptyElementSource().clone() }), eqs);
    flatSmSemantics = FlatSmSemantics { ident: (ident).clone(), smComps: metamodelica::arrayFromVec(q.into_iter().cloned().collect()), t: t, c: cExps, vars: vars, knowns: knowns, eqs: eqs, pvars: metamodelica::nil(), peqs: metamodelica::nil(), enclosingState: None };
    Ok(flatSmSemantics)
}

fn qCref(mut ident: ArcStr, mut identType: Arc<DAE::Type>, mut subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut componentRef: Arc<DAE::ComponentRef>) -> Result<Arc<DAE::ComponentRef>> {
    let mut outQual: Arc<DAE::ComponentRef>;
    outQual = ComponentReference::joinCrefs(componentRef, Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (ident).clone(), identType: identType, subscriptLst: subscriptLst }))?;
    Ok(outQual)
}

fn createVarWithDefaults(mut componentRef: Arc<DAE::ComponentRef>, mut kind: DAE::VarKind, mut ty: Arc<DAE::Type>, mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Arc<DAE::Element> {
    let mut var: Arc<DAE::Element>;
    var = Arc::new(DAE::Element::VAR { componentRef: componentRef, kind: kind, direction: openmodelica_frontend_types::DAE::VarDirection::BIDIR, parallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, protection: openmodelica_frontend_types::DAE::VarVisibility::PUBLIC, ty: ty, binding: None, dims: dims, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), source: DAE::emptyElementSource().clone(), variableAttributesOption: None, comment: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, encrypted: false });
    var
}

fn createVarWithStartValue(mut componentRef: Arc<DAE::ComponentRef>, mut kind: DAE::VarKind, mut ty: Arc<DAE::Type>, mut startExp: Arc<DAE::Exp>, mut dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Arc<DAE::Element>> {
    let mut outVar: Arc<DAE::Element>;
    let mut var: Arc<DAE::Element>;
    var = Arc::new(DAE::Element::VAR { componentRef: componentRef, kind: kind, direction: openmodelica_frontend_types::DAE::VarDirection::BIDIR, parallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, protection: openmodelica_frontend_types::DAE::VarVisibility::PUBLIC, ty: ty, binding: None, dims: dims, connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(), source: DAE::emptyElementSource().clone(), variableAttributesOption: None, comment: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, encrypted: false });
    outVar = setVarFixedStartValue(var, startExp)?;
    Ok(outVar)
}

fn createTandC(mut inSMComps: Arc<metamodelica::List<Arc<DAE::Element>>>, mut inTransitions: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<(Arc<metamodelica::List<Transition>>, Arc<metamodelica::List<Arc<DAE::Exp>>>)> {
    let mut t: Arc<metamodelica::List<Transition>>;
    let mut c: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut transitions: Arc<metamodelica::List<Transition>>;
    transitions = List::map1(inTransitions, (std::sync::Arc::new(createTransition) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Transition> + 'static>), inSMComps)?;
    t = List::sort(transitions, (std::sync::Arc::new(priorityLt) as std::sync::Arc<dyn ::std::ops::Fn(Transition, Transition) -> Result<bool> + 'static>))?;
    c = List::map(t.clone(), (std::sync::Arc::new(extractCondtionFromTransition) as std::sync::Arc<dyn ::std::ops::Fn(Transition) -> Result<Arc<DAE::Exp>> + 'static>))?;
    Ok((t, c))
}

fn extractCondtionFromTransition(mut trans: Transition) -> Result<Arc<DAE::Exp>> {
    let mut condition: Arc<DAE::Exp>;
    let Transition { condition: __pa0, .. } = (trans) else { bail!("pattern mismatch") };
    condition = __pa0.clone();
    Ok(condition)
}

fn priorityLt(mut inTrans1: Transition, mut inTrans2: Transition) -> Result<bool> {
    let mut res: bool;
    let mut priority1: i32;
    let mut priority2: i32;
    let Transition { priority: __pa0, .. } = (inTrans1) else { bail!("pattern mismatch") };
    priority1 = __pa0.clone();
    let Transition { priority: __pa1, .. } = (inTrans2) else { bail!("pattern mismatch") };
    priority2 = __pa1.clone();
    res = intLt(priority1, priority2);
    Ok(res)
}

fn createTransition(mut transitionElem: Arc<DAE::Element>, mut states: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Transition> {
    let mut trans: Transition;
    let mut crefFrom: Arc<DAE::ComponentRef>;
    let mut crefTo: Arc<DAE::ComponentRef>;
    let mut from: i32;
    let mut to: i32;
    let mut condition: Arc<DAE::Exp>;
    let mut immediate: bool = true;
    let mut reset: bool = true;
    let mut synchronize: bool = false;
    let mut priority: i32 = 1;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(transitionElem) {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "transition" }, expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __pa0, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __pa1, .. }, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: __pa3 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: __pa4 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::BCONST { bool: __pa5 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: __pa6 }, tail: Deref @ metamodelica::List::Nil } } } } } } }, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    crefFrom = __pa0.clone();
    crefTo = __pa1.clone();
    condition = __pa2.clone();
    immediate = __pa3.clone();
    reset = __pa4.clone();
    synchronize = __pa5.clone();
    priority = __pa6.clone();
    from = List::position1OnTrue(states.clone(), (std::sync::Arc::new(sMCompEqualsRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), crefFrom)?;
    to = List::position1OnTrue(states, (std::sync::Arc::new(sMCompEqualsRef) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, Arc<DAE::ComponentRef>) -> Result<bool> + 'static>), crefTo)?;
    trans = Transition { from: from, to: to, condition: condition, immediate: immediate, reset: reset, synchronize: synchronize, priority: priority };
    Ok(trans)
}

fn isFlatSm(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outResult: bool;
    outResult = (::match_deref::match_deref! { match &(inElement) {
        Deref @ DAE::Element::FLAT_SM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outResult
}

fn isSMComp(mut inElement: Arc<DAE::Element>) -> bool {
    let mut outResult: bool;
    outResult = (::match_deref::match_deref! { match &(inElement) {
        Deref @ DAE::Element::SM_COMP { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outResult
}

fn isTransition(mut inElement: Arc<DAE::Element>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match &(inElement) {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "transition" }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

fn isInitialState(mut inElement: Arc<DAE::Element>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match &(inElement) {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initialState" }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

fn isEquation(mut inElement: Arc<DAE::Element>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match &(inElement) {
        Deref @ DAE::Element::EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

fn isEquationOrWhenEquation(mut inElement: Arc<DAE::Element>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match &(inElement) {
        Deref @ DAE::Element::EQUATION { .. } => true,
        Deref @ DAE::Element::WHEN_EQUATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

fn isPreOrPreviousEquation(mut inElement: Arc<DAE::Element>) -> Result<bool> {
    let mut result: bool;
    result = (::match_deref::match_deref! { match &(inElement) {
        Deref @ DAE::Element::EQUATION { exp, scalar, source: _ } => {
            Expression::expHasPre(exp.clone())? || Expression::expHasPre(scalar.clone())? || Expression::expHasPrevious(exp.clone())? || Expression::expHasPrevious(scalar.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

fn isVar(mut inElement: Arc<DAE::Element>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match &(inElement) {
        Deref @ DAE::Element::VAR { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

fn sMCompEqualsRef(mut inElement: Arc<DAE::Element>, mut inCref: Arc<DAE::ComponentRef>) -> Result<bool> {
    let mut result: bool;
    result = (::match_deref::match_deref! { match &(inElement) {
        Deref @ DAE::Element::SM_COMP { componentRef: cref, .. } if (ComponentReferenceBasics::crefEqual(cref.clone(), inCref.clone())?) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn dumpTransitionStr(mut transition: Transition) -> Result<ArcStr> {
    let mut transitionStr: ArcStr;
    let mut from: i32;
    let mut to: i32;
    let mut condition: Arc<DAE::Exp>;
    let mut immediate: bool;
    let mut reset: bool;
    let mut synchronize: bool;
    let mut priority: i32;
    let Transition { from: __pa0, to: __pa1, condition: __pa2, immediate: __pa3, reset: __pa4, synchronize: __pa5, priority: __pa6 } = (transition) else { bail!("pattern mismatch") };
    from = __pa0.clone();
    to = __pa1.clone();
    condition = __pa2.clone();
    immediate = __pa3.clone();
    reset = __pa4.clone();
    synchronize = __pa5.clone();
    priority = __pa6.clone();
    transitionStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TRANSITION(from=")); __mm_s.push_str(&*intString(from)); __mm_s.push_str(&*literal!(", to=")); __mm_s.push_str(&*intString(to)); __mm_s.push_str(&*literal!(", condition=")); __mm_s.push_str(&*ExpressionBasics::printExpStr(condition)?); __mm_s.push_str(&*literal!(", immediate=")); __mm_s.push_str(&*boolString(immediate)); __mm_s.push_str(&*literal!(", reset=")); __mm_s.push_str(&*boolString(reset)); __mm_s.push_str(&*literal!(", synchronize=")); __mm_s.push_str(&*boolString(synchronize)); __mm_s.push_str(&*literal!(", priority=")); __mm_s.push_str(&*intString(priority)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    Ok(transitionStr)
}

fn wrapHack(mut cache: FCore::Cache, mut inElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Arc<metamodelica::List<Arc<DAE::Element>>>> {
    let mut outElementLst: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut eqnLst: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut otherLst: Arc<metamodelica::List<Arc<DAE::Element>>>;
    let mut whenEq: Arc<DAE::Element>;
    let mut cond1: Arc<DAE::Exp>;
    let mut cond2: Arc<DAE::Exp>;
    let mut condition: Arc<DAE::Exp>;
    let mut condLst: Arc<metamodelica::List<Arc<DAE::Exp>>>;
    let mut tArrayBool: Arc<DAE::Type>;
    cond1 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("initial")).clone() }), expLst: metamodelica::nil(), attr: DAE::callAttrBuiltinImpureBool().clone() });
    cond2 = Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("sample")).clone() }), expLst: list![Arc::new(DAE::Exp::RCONST { real: Flags::getConfigReal(Flags::DEFAULT_CLOCK_PERIOD.clone())? }), Arc::new(DAE::Exp::RCONST { real: Flags::getConfigReal(Flags::DEFAULT_CLOCK_PERIOD.clone())? })], attr: DAE::callAttrBuiltinImpureBool().clone() });
    tArrayBool = Arc::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![Arc::new(DAE::Dimension::DIM_INTEGER { integer: 2 })] });
    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
        condLst = List::filterMap1(inElementLst.clone(), (std::sync::Arc::new(extractSmOfExps) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>, ArcStr) -> Result<Arc<DAE::Exp>> + 'static>), (literal!("cImmediate")).clone());
        (eqnLst, otherLst) = List::extractOnTrue(inElementLst, (std::sync::Arc::new(isPreOrPreviousEquation) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
        condition = Arc::new(DAE::Exp::ARRAY { ty: tArrayBool, scalar: true, array: metamodelica::cons(cond1, condLst) });
    } else {
        (eqnLst, otherLst) = List::extractOnTrue(inElementLst, (std::sync::Arc::new(fnptr!(isEquation, Arc<DAE::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Element>) -> Result<bool> + 'static>))?;
        condition = Arc::new(DAE::Exp::ARRAY { ty: tArrayBool, scalar: true, array: list![cond1, cond2] });
    }
    whenEq = Arc::new(DAE::Element::WHEN_EQUATION { condition: condition, equations: eqnLst, elsewhen_: None, source: DAE::emptyElementSource().clone() });
    outElementLst = listAppend(otherLst, list![whenEq]);
    Ok(outElementLst)
}

fn extractSmOfExps(mut inElem: Arc<DAE::Element>, mut inLastIdent: ArcStr) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inElem) {
        Deref @ DAE::Element::EQUATION { exp, .. } => {
            let mut cref: Arc<DAE::ComponentRef>;
            let mut firstIdent: ArcStr;
            let mut lastIdent: ArcStr;
            let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cref = __pa0.clone();
            firstIdent = (ComponentReferenceBasics::crefFirstIdent(cref.clone())?).clone();
            let true = (firstIdent.clone() == literal!("smOf")) else { bail!("pattern mismatch") };
            lastIdent = (ComponentReferenceBasics::crefLastIdent(cref.clone())?).clone();
            let true = (lastIdent.clone() == inLastIdent) else { bail!("pattern mismatch") };
            exp.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

fn traversingSubsPreForPrevious(mut inExp: Arc<DAE::Exp>, mut inHitCount: i32) -> (Arc<DAE::Exp>, i32) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outHitCount: i32;
    (outExp, outHitCount) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst, attr } => {
            (Arc::new(DAE::Exp::CALL { path: Arc::new(Absyn::Path::IDENT { name: (literal!("pre")).clone() }), expLst: expLst.clone(), attr: attr.clone() }), inHitCount + 1)
        },
        _ => {
            (inExp, inHitCount)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outHitCount)
}

fn traversingSubsXForSampleX(mut inExp: Arc<DAE::Exp>, mut inHitCount: i32) -> (Arc<DAE::Exp>, i32) {
    let mut outExp: Arc<DAE::Exp>;
    let mut outHitCount: i32;
    (outExp, outHitCount) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, expLst: Deref @ metamodelica::List::Cons { head: expX, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::INFERRED_CLOCK { .. } }, tail: Deref @ metamodelica::List::Nil } }, attr: _ } => {
            (expX.clone(), inHitCount + 1)
        },
        _ => {
            (inExp, inHitCount)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outHitCount)
}

